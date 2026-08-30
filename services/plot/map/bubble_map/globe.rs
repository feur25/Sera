use super::common::{push_projected_outlines, visible_shapes, PALETTE};
use super::config::BubbleMapConfig;
use crate::plot::map::projections::{self, Projection};

#[crate::chart_demo(
    "labels=[\"US\",\"CN\",\"IN\",\"ID\",\"PK\",\"BR\",\"NG\",\"BD\",\"RU\",\"MX\",\"JP\",\"ET\",\"PH\",\"EG\",\"VN\",\"DE\",\"TR\",\"IR\",\"TH\",\"GB\",\"FR\",\"IT\",\"ZA\",\"TZ\",\"MM\",\"KR\",\"CO\",\"KE\",\"ES\",\"AR\",\"DZ\",\"UA\",\"UG\",\"IQ\",\"PL\",\"CA\",\"MA\",\"SA\",\"UZ\",\"PE\"], values=[331.9,1412.0,1380.0,273.5,220.9,213.3,206.1,164.7,144.1,128.9,125.7,114.9,109.6,104.3,97.3,83.2,84.3,84.0,69.8,67.9,67.4,60.4,59.9,59.7,54.4,51.8,50.9,53.8,47.4,45.4,43.9,44.1,45.7,40.2,38.4,38.2,36.9,34.8,33.5,33.0], title=\"Population by Country (millions)\", center_lat=15, center_lon=10"
)]

pub fn render(cfg: &BubbleMapConfig) -> String {
    let Some(to_latlon) = cfg.region.to_latlon else {
        return super::filled::render(cfg);
    };
    let n = cfg.values.len().min(cfg.labels.len());
    if n == 0 {
        return String::new();
    }
    let lat = cfg.center_lat.unwrap_or(15.0);
    let lon = cfg.center_lon.unwrap_or(10.0);
    let projection = Projection::Orthographic;
    let shapes = visible_shapes(cfg);
    let raw = projections::project_shapes(&shapes, to_latlon, projection, lat, lon);
    let (transform, projected) = projections::project_and_fit(&raw, cfg.width, cfg.height, 0.92);
    let disc = transform.disc();

    let mut svg = super::common::svg_open(cfg.width, cfg.height);
    svg.push_str(&format!(
        "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"#0d1424\" stroke=\"#2a3a6e\" stroke-width=\"1\"/>",
        disc.cx, disc.cy, disc.radius
    ));
    push_projected_outlines(&mut svg, &projected);

    let max_abs = cfg.values.iter().take(n).fold(0.0_f64, |m, v| m.max(v.abs())).max(1e-9);
    for i in 0..n {
        let Some(shape) = (cfg.region.lookup)(&cfg.labels[i]) else {
            continue;
        };
        if !shapes.iter().any(|s| s.id == shape.id) {
            continue;
        }
        let centroid = crate::plot::map::world_data::shape_centroid(shape);
        let (clat, clon) = to_latlon(centroid[0], centroid[1]);
        let Some((px, py)) = projection.project(clat, clon, lat, lon) else {
            continue;
        };
        let [sx, sy] = transform.apply(px, py);
        let t = (cfg.values[i].abs() / max_abs).sqrt();
        let radius = cfg.min_bubble_size + t * (cfg.max_bubble_size - cfg.min_bubble_size);
        let (r, g, b) = PALETTE[i % PALETTE.len()];
        svg.push_str(&format!(
            "<circle cx=\"{sx:.1}\" cy=\"{sy:.1}\" r=\"{radius:.1}\" fill=\"rgb({r},{g},{b})\" fill-opacity=\"0.75\" stroke=\"rgb({r},{g},{b})\" stroke-width=\"1\" data-index=\"{i}\"/>"
        ));
        svg.push_str(&format!(
            "<text x=\"{sx:.0}\" y=\"{sy:.0}\" fill=\"white\" font-size=\"8\" text-anchor=\"middle\" dominant-baseline=\"middle\">{}</text>",
            shape.id
        ));
    }

    super::common::to_html(cfg, svg)
}
