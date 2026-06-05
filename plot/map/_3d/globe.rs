use crate::core::math::{heat_color, spherical_to_cartesian};
use crate::plot::camera::Point3D;
use crate::plot::containers_3d::render_3d_grid;
use crate::plot::containers_3d::CameraController;
use crate::plot::containers_3d::Cube3DContainer;
use crate::plot::map::world_data;
use crate::plot::scale_renderer::render_scale_labels;
use crate::plot::{apply_bg3d, parse_all};

const GLOBE_RADIUS: f32 = 0.45;

pub struct Globe3DRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub camera_controller: &'a CameraController,
    pub labels: &'a [String],
}

pub fn render_globe_3d(ctx: Globe3DRenderContext) {
    let max_val = ctx.max_val.max(1.0);
    let cube = Cube3DContainer::new(Point3D::new(0.0, 0.0, 0.0), 0.5);

    render_3d_grid(ctx.painter, &cube, ctx.camera_controller, ctx.plot_rect);
    render_scale_labels(ctx.painter, ctx.plot_rect, max_val);

    let center = ctx.plot_rect.center();
    let half_w = ctx.plot_rect.width() / 2.0;
    let half_h = ctx.plot_rect.height() / 2.0;

    let mut label_map: std::collections::HashMap<String, (usize, f64)> =
        std::collections::HashMap::new();
    for &actual_idx in ctx.visible_indices.iter() {
        if actual_idx >= ctx.labels.len() {
            continue;
        }
        let key = ctx.labels[actual_idx].to_uppercase();
        label_map.insert(key, (actual_idx, ctx.values[actual_idx]));
    }

    let base_fill = egui::Color32::from_rgba_premultiplied(26, 26, 46, 200);
    let border_color = egui::Color32::from_rgba_premultiplied(60, 60, 100, 120);

    struct CountryRender {
        projected: Vec<egui::Pos2>,
        fill: egui::Color32,
        stroke: egui::Stroke,
        z_order: f32,
        _idx: Option<usize>,
    }

    let mut all_polys: Vec<CountryRender> = Vec::new();

    for shape in world_data::all_countries() {
        let entry = label_map
            .get(&shape.id)
            .or_else(|| label_map.get(&shape.name.to_uppercase()));

        let (fill, stroke) = if let Some(&(idx, value)) = entry {
            let is_hov = ctx.hovered_idx.map(|h| h == idx).unwrap_or(false);
            if is_hov {
                (
                    egui::Color32::from_rgb(255, 220, 50),
                    egui::Stroke::new(1.5, egui::Color32::WHITE),
                )
            } else {
                let (r, g, b) = heat_color(value, max_val);
                (
                    egui::Color32::from_rgb(r, g, b),
                    egui::Stroke::new(
                        0.5,
                        egui::Color32::from_rgba_premultiplied(255, 255, 255, 60),
                    ),
                )
            }
        } else {
            (base_fill, egui::Stroke::new(0.3, border_color))
        };

        for poly in &shape.polygons {
            if poly.len() < 3 {
                continue;
            }

            let mut projected = Vec::with_capacity(poly.len());
            let mut z_sum: f32 = 0.0;
            let mut all_visible = true;

            for pt in poly.iter() {
                let (lat, lon) = world_data::svg_to_latlon(pt[0], pt[1]);
                let (x, y, z) = spherical_to_cartesian(lat, lon);
                let p3 = Point3D::new(
                    x as f32 * GLOBE_RADIUS,
                    z as f32 * GLOBE_RADIUS,
                    y as f32 * GLOBE_RADIUS,
                );
                if let Some(proj) = ctx.camera_controller.camera.project(p3) {
                    let sp = egui::pos2(center.x + proj.x * half_w, center.y - proj.y * half_h);
                    projected.push(sp);
                    z_sum += p3.z;
                } else {
                    all_visible = false;
                    break;
                }
            }

            if !all_visible || projected.len() < 3 {
                continue;
            }

            let z_avg = z_sum / projected.len() as f32;
            let data_idx = entry.map(|e| e.0);

            all_polys.push(CountryRender {
                projected,
                fill,
                stroke,
                z_order: z_avg,
                _idx: data_idx,
            });
        }
    }

    all_polys.sort_by(|a, b| {
        a.z_order
            .partial_cmp(&b.z_order)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for cr in &all_polys {
        let mut path = egui::epaint::PathShape::closed_line(cr.projected.clone(), cr.stroke);
        path.fill = cr.fill;
        ctx.painter.add(egui::Shape::Path(path));
    }
}

pub fn get_globe_3d_positions(
    _values: &[f64],
    _max_val: f64,
    visible_indices: &[usize],
    camera_controller: &CameraController,
    plot_rect: egui::Rect,
) -> Vec<(egui::Pos2, usize)> {
    let n = visible_indices.len();
    if n == 0 {
        return Vec::new();
    }

    let center = plot_rect.center();
    let half_w = plot_rect.width() / 2.0;
    let half_h = plot_rect.height() / 2.0;

    let mut positions = Vec::with_capacity(n);

    for &actual_idx in visible_indices.iter() {
        let (lat, lon) = index_to_globe_latlon(actual_idx);
        let (x, y, z) = spherical_to_cartesian(lat, lon);
        let p3 = Point3D::new(
            x as f32 * GLOBE_RADIUS,
            z as f32 * GLOBE_RADIUS,
            y as f32 * GLOBE_RADIUS,
        );

        if let Some(proj) = camera_controller.camera.project(p3) {
            let sp = egui::pos2(center.x + proj.x * half_w, center.y - proj.y * half_h);
            positions.push((sp, actual_idx));
        }
    }

    positions
}

pub fn get_globe_3d_positions_with_labels(
    _values: &[f64],
    _max_val: f64,
    visible_indices: &[usize],
    labels: &[String],
    camera_controller: &CameraController,
    plot_rect: egui::Rect,
) -> Vec<(egui::Pos2, usize)> {
    let n = visible_indices.len();
    if n == 0 {
        return Vec::new();
    }

    let center = plot_rect.center();
    let half_w = plot_rect.width() / 2.0;
    let half_h = plot_rect.height() / 2.0;

    let mut positions = Vec::with_capacity(n);

    for &actual_idx in visible_indices.iter() {
        if actual_idx >= labels.len() {
            continue;
        }
        let label = &labels[actual_idx];

        let (lat, lon) = if let Some(shape) = world_data::lookup_country(label) {
            let c = world_data::shape_centroid(shape);
            world_data::svg_to_latlon(c[0], c[1])
        } else {
            index_to_globe_latlon(actual_idx)
        };

        let (x, y, z) = spherical_to_cartesian(lat, lon);
        let p3 = Point3D::new(
            x as f32 * GLOBE_RADIUS,
            z as f32 * GLOBE_RADIUS,
            y as f32 * GLOBE_RADIUS,
        );

        if let Some(proj) = camera_controller.camera.project(p3) {
            let sp = egui::pos2(center.x + proj.x * half_w, center.y - proj.y * half_h);
            positions.push((sp, actual_idx));
        }
    }

    positions
}

fn index_to_globe_latlon(idx: usize) -> (f64, f64) {
    let lat = -60.0 + (idx as f64 * 3.7) % 120.0;
    let lon = -180.0 + (idx as f64 * 7.3) % 360.0;
    (lat, lon)
}
#[crate::sera_builder]
#[crate::sera_alias("globe3d", "globe_3d", "globe3d_chart", "globe")]
pub fn build_globe3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let latitudes = a.lats.unwrap_or_default();
    let longitudes = a.lons.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let n = latitudes.len().min(longitudes.len()).min(values.len());
    let lbl = o.point_labels.clone().unwrap_or_default();
    let cl = if lbl.is_empty() {
        (0..n).map(|i| format!("Point {}", i + 1)).collect()
    } else {
        lbl
    };
    let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let bg_str = o.bg_str();
    apply_bg3d(
        crate::plot::map::_3d::render_globe3d_html(
            title,
            &longitudes[..n],
            &latitudes[..n],
            &values[..n],
            ("Longitude", "Latitude", "Value"),
            &cv,
            &cl,
            o.w(800),
            o.h(600),
            bg_str.as_deref(),
        ),
        &o,
    )
}
