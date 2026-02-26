use std::sync::{Arc, Mutex};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::collections::HashMap;
use image::ImageDecoder;
use super::image_loader::ImageLoader;
use super::cache::{RenderCache, ColorCache, CacheKey};
use super::viewer_3d::AdvancedViewer3D;
use super::wiki_viewer::WikiViewer;
use super::manager::button_manager::{ButtonManager, ButtonId};
use super::render::{AdvancedBatchRenderer, AdvancedBatchRendererBuilder, RenderState, PointComputeBuilder, VisibilityOptimizer};
use crate::plot::default::{PlotRenderContext, render_plot_by_type};
use crate::plot::controller::plot_3d_controller::{Plot3DRenderContext, render_by_type as render_3d_by_type};
use crate::plot::CameraController;
use crate::bindings::{HtmlExporter, HtmlExportConfig, HtmlTheme};
use crate::html::HtmlTemplate;
use crate::bindings::utils::{BitSet, DataProcessor, simd_ops};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HoverItem {
    pub index: u32,
    pub key: *const c_char,
    pub value: *const c_char,
}

struct Hover3DDetector {
    positions: Vec<(egui::Pos2, usize)>,
    threshold: f32,
}

impl Hover3DDetector {
    fn new(positions: Vec<(egui::Pos2, usize)>) -> Self {
        Self {
            positions,
            threshold: 25.0,
        }
    }
    
    fn detect(&self, mouse_pos: egui::Pos2) -> Option<usize> {
        if !mouse_pos.is_finite() {
            return None;
        }
        
        let mut closest_idx = None;
        let mut closest_dist = self.threshold;
        
        for &(screen_pos, actual_idx) in &self.positions {
            if !screen_pos.is_finite() {
                continue;
            }
            
            let delta = mouse_pos - screen_pos;
            let dist = (delta.x * delta.x + delta.y * delta.y).sqrt();
            
            if dist < closest_dist {
                closest_dist = dist;
                closest_idx = Some(actual_idx);
            }
        }
        
        closest_idx
    }
}

struct ChartData {
    labels: Vec<String>,
    values: Vec<f64>,
    title: String,
    hover_data: Vec<Vec<(String, String)>>,
    tooltip_bg_color: (u8, u8, u8, u8),
    tooltip_text_color: (u8, u8, u8, u8),
}

struct PlotMetrics {
    width: f32,
    height: f32,
    pad_x: f32,
    pad_y: f32,
}

impl PlotMetrics {
    fn vertical(zoom: f32) -> Self {
        Self {
            width: 600.0 * zoom,
            height: 400.0 * zoom,
            pad_x: 40.0,
            pad_y: 20.0,
        }
    }

    fn horizontal(zoom: f32) -> Self {
        Self {
            width: 400.0 * zoom,
            height: 600.0 * zoom,
            pad_x: 80.0,
            pad_y: 20.0,
        }
    }

    fn allocate_size(&self) -> egui::Vec2 {
        egui::Vec2::new(self.width + self.pad_x * 2.0, self.height + self.pad_y * 2.0)
    }

    fn with_rect(&self, base_min: egui::Pos2) -> egui::Rect {
        egui::Rect::from_min_size(
            base_min + egui::Vec2::new(self.pad_x, self.pad_y),
            egui::Vec2::new(self.width, self.height),
        )
    }
}

trait PlotRenderer {
    fn render_axes(&self, painter: &egui::Painter, plot_rect: egui::Rect, max_val: f64);
    fn detect_hover(&self, rel_pos: egui::Vec2, plot_rect: egui::Rect, point_count: usize, values: &[f64], max_val: f64) -> Option<usize>;
}

struct GenericPlotRenderer {
    vertical: bool,
}

impl GenericPlotRenderer {
    fn map_point(&self, idx: usize, value: f64, max_val: f64, point_count: usize, plot_rect: egui::Rect) -> egui::Pos2 {
        let norm_val = value / max_val.max(1.0);
        let (mut x, mut y);
        
        if self.vertical {
            x = plot_rect.left() + (plot_rect.width() / (point_count as f32 - 1.0).max(1.0)) * idx as f32;
            y = plot_rect.bottom() - norm_val as f32 * plot_rect.height();
        } else {
            x = plot_rect.left() + norm_val as f32 * plot_rect.width();
            y = plot_rect.top() + (plot_rect.height() / (point_count as f32 - 1.0).max(1.0)) * idx as f32;
        }

        egui::pos2(x, y)
    }
}

impl PlotRenderer for GenericPlotRenderer {
    fn render_axes(&self, painter: &egui::Painter, plot_rect: egui::Rect, max_val: f64) {
        painter.line_segment(
            [egui::pos2(plot_rect.left(), plot_rect.bottom()), egui::pos2(plot_rect.right(), plot_rect.bottom())],
            egui::Stroke::new(1.5, egui::Color32::from_gray(200)),
        );
        painter.line_segment(
            [egui::pos2(plot_rect.left(), plot_rect.top()), egui::pos2(plot_rect.left(), plot_rect.bottom())],
            egui::Stroke::new(1.5, egui::Color32::from_gray(200)),
        );
        
        let font_size = 11.0;
        for i in 0..=5 {
            let val = (max_val / 5.0) * i as f64;
            let (pos, align) = if self.vertical {
                let y = plot_rect.bottom() - (plot_rect.height() / 5.0) * i as f32;
                (egui::pos2(plot_rect.left() - 15.0, y), egui::Align2::RIGHT_CENTER)
            } else {
                let x = plot_rect.left() + (plot_rect.width() / 5.0) * i as f32;
                (egui::pos2(x, plot_rect.bottom() + 15.0), egui::Align2::CENTER_TOP)
            };
            painter.text(pos, align, &format!("{:.1}", val), egui::FontId::proportional(font_size), egui::Color32::from_gray(100));
        }
    }

    fn detect_hover(&self, rel_pos: egui::Vec2, plot_rect: egui::Rect, point_count: usize, values: &[f64], max_val: f64) -> Option<usize> {
        if rel_pos.x < 0.0 || rel_pos.x > plot_rect.width() || rel_pos.y < 0.0 || rel_pos.y > plot_rect.height() {
            return None;
        }
        
        let threshold = 12.0;
        let mut closest_idx = None;
        let mut closest_dist = threshold;
        
        for idx in 0..point_count {
            let point = self.map_point(idx, values.get(idx).copied().unwrap_or(0.0), max_val, point_count, plot_rect);
            let delta = egui::Vec2::new(point.x - (plot_rect.left() + rel_pos.x), point.y - (plot_rect.top() + rel_pos.y));
            let dist = (delta.x * delta.x + delta.y * delta.y).sqrt();
            
            if dist < closest_dist {
                closest_dist = dist;
                closest_idx = Some(idx);
            }
        }
        
        closest_idx
    }
}

struct ChartApp {
    data: Arc<Mutex<Option<ChartData>>>,
    hovered_idx: Option<usize>,
    zoom: f32,
    pan_x: f32,
    visible_elements: BitSet,
    selected_elements: BitSet,
    show_list: bool,
    image_loader: ImageLoader,
    orientation: bool,
    sort_mode: i32,
    current_chart_kind: u8,
    is_3d_mode: bool,
    camera_controller: CameraController,
    advanced_viewer_3d: AdvancedViewer3D,
    render_cache: RenderCache,
    color_cache: ColorCache,
    last_data_hash: u64,
    last_render_state: (bool, u8, i32, bool),
    filter_threshold: f64,
    show_stats: bool,
    show_processor_menu: bool,
    show_transform_menu: bool,
    show_wiki: bool,
    show_info: bool,
    wiki_viewer: Option<WikiViewer>,
    button_manager: ButtonManager,
    aggregation_results: HashMap<String, f64>,
    limit_value: usize,
    batch_renderer: AdvancedBatchRenderer,
    render_state: RenderState,
    selection_start: Option<egui::Pos2>,
    selection_end: Option<egui::Pos2>,
    selection_active: bool,
}

struct ChartAppBuilder {
    data: Arc<Mutex<Option<ChartData>>>,
    num_elements: usize,
}

impl ChartAppBuilder {
    fn new(data: ChartData) -> Self {
        let num_elements = data.labels.len();
        Self {
            data: Arc::new(Mutex::new(Some(data))),
            num_elements,
        }
    }

    fn build(self) -> ChartApp {
        ChartApp {
            data: self.data,
            hovered_idx: None,
            zoom: 1.0,
            pan_x: 0.0,
            visible_elements: BitSet::new(self.num_elements),
            selected_elements: BitSet::new(self.num_elements),
            show_list: false,
            image_loader: ImageLoader::new(),
            orientation: true,
            sort_mode: 0,
            current_chart_kind: 1,
            is_3d_mode: false,
            camera_controller: CameraController::default(),
            advanced_viewer_3d: AdvancedViewer3D::new(),
            filter_threshold: 0.0,
            show_stats: false,
            show_processor_menu: false,
            show_transform_menu: false,
            show_wiki: false,
            show_info: false,
            wiki_viewer: None,
            button_manager: ButtonManager::new(),
            aggregation_results: HashMap::new(),
            limit_value: 50,
            render_cache: RenderCache::new(),
            color_cache: ColorCache::new(),
            last_data_hash: 0,
            last_render_state: (true, 1, 0, false),
            batch_renderer: AdvancedBatchRendererBuilder::new().with_capacity(100000).build(),
            render_state: RenderState::new(1200.0, 600.0),
            selection_start: None,
            selection_end: None,
            selection_active: false,
        }
    }
}

fn launch_chart_app(app: ChartApp) -> bool {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([1200.0, 600.0]);
    
    let icon_paths = vec![
        "src/asset/logo.png",
        "../src/asset/logo.png",
        "../../v2/src/asset/logo.png",
    ];
    
    for path in icon_paths {
        if let Ok(img) = image::open(path) {
            let rgba = img.to_rgba8();
            let (w, h) = rgba.dimensions();
            viewport = viewport.with_icon(egui::IconData {
                rgba: rgba.into_raw(),
                width: w,
                height: h,
            });
            break;
        }
    }
    
    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "SeraPlot",
        native_options,
        Box::new(|_| Box::new(app)),
    );

    true
}

fn render_svg_by_type(chart_type: u8, _labels: &[String], values: &[f64], colors: &[u32], indices: &[usize], orientation: bool, pad_left: f32, _pad_top: f32, _pad_right: f32, _pad_bottom: f32, plot_w: f32, plot_h: f32, scale_max: f32, _width: f32, _height: f32, svg: &mut String) {
    let hex_colors: Vec<&'static str> = colors.iter().map(|&c| {
        let r = ((c >> 16) & 0xFF) as u8;
        let g = ((c >> 8) & 0xFF) as u8;
        let b = (c & 0xFF) as u8;
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        Box::leak(hex.into_boxed_str()) as &'static str
    }).collect();
    
    if let Some(renderer) = crate::plot::controller::chart_controller::get_svg_renderer(chart_type) {
        renderer(svg, values, &hex_colors, pad_left as i32, plot_w as i32, plot_h as i32, scale_max as f64, orientation);
    }
}

impl eframe::App for ChartApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_3d_mode {
            if let Ok(kind) = CHART_KIND.lock() {
                self.current_chart_kind = *kind;
            }
        }
        
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let zoom_label = if self.zoom >= 1.0 {
                    format!("🔍 {:.1}x", self.zoom)
                } else {
                    format!("🔍 1/{:.1}", 1.0 / self.zoom)
                };

                if ui.button("➕").clicked() {
                    self.zoom *= 1.2;
                }
                if ui.button("➖").clicked() {
                    self.zoom /= 1.2;
                }
                if ui.button("⬜").clicked() {
                    self.zoom = 1.0;
                    self.pan_x = 0.0;
                }
                
                if ui.button("🔄 Reset Selection").clicked() {
                    self.visible_elements.set_all();
                    self.selected_elements.set_all();
                    self.selection_start = None;
                    self.selection_end = None;
                    self.selection_active = false;
                }

                ui.separator();

                let clicked = self.button_manager.render_with_descriptions(ui);

                if clicked.contains_key(&ButtonId::Elements) {
                    self.show_list = !self.show_list;
                }
                if clicked.contains_key(&ButtonId::Processor) {
                    self.show_processor_menu = !self.show_processor_menu;
                }
                if clicked.contains_key(&ButtonId::Transform) {
                    self.show_transform_menu = !self.show_transform_menu;
                }
                // if clicked.contains_key(&ButtonId::Stats) {
                //     self.show_stats = !self.show_stats;
                // }
                if clicked.contains_key(&ButtonId::Info) {
                    self.show_info = !self.show_info;
                    self.button_manager.show_info(self.show_info);
                }
                if clicked.contains_key(&ButtonId::Orientation) {
                    self.orientation = !self.orientation;
                }
                if clicked.contains_key(&ButtonId::Sort) {
                    self.sort_mode = (self.sort_mode + 1) % 3;
                }
                if clicked.contains_key(&ButtonId::Mode3D) {
                    self.is_3d_mode = !self.is_3d_mode;
                }

                if clicked.contains_key(&ButtonId::Wiki) {
                    self.show_wiki = !self.show_wiki;
                    if self.show_wiki && self.wiki_viewer.is_none() {
                        let wiki = crate::wiki::generate_seraplot_docs();
                        self.wiki_viewer = Some(WikiViewer::builder().with_wiki(wiki).build());
                    }
                }

                if clicked.contains_key(&ButtonId::Html) {
                    let (labels, values, title, hover_data, orientation, sort_mode, chart_type, visible) = {
                        let data = self.data.lock().unwrap();
                        if let Some(d) = data.as_ref() {
                            let mut vis = vec![true; d.labels.len()];
                            for i in 0..d.labels.len() {
                                vis[i] = self.visible_elements.get(i);
                            }
                            (d.labels.clone(), d.values.clone(), d.title.clone(), d.hover_data.clone(), self.orientation, self.sort_mode, self.current_chart_kind, vis)
                        } else {
                            return;
                        }
                    };

                    std::thread::spawn(move || {
                        use crate::bindings::HtmlExporter;
                        use crate::bindings::utils::simd_ops;

                        let mut indexed_data: Vec<(usize, f64, String, Vec<(String, String)>)> = values.iter().enumerate()
                            .map(|(i, &v)| (i, v, labels.get(i).cloned().unwrap_or_default(), hover_data.get(i).cloned().unwrap_or_default()))
                            .collect();

                        indexed_data.retain(|(idx, _, _, _)| visible.get(*idx).copied().unwrap_or(true));

                        if sort_mode == 1 {
                            indexed_data.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
                        } else if sort_mode == 2 {
                            indexed_data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                        }

                        let filtered_labels: Vec<String> = indexed_data.iter().map(|(_, _, l, _)| l.clone()).collect();
                        let filtered_values: Vec<f64> = indexed_data.iter().map(|(_, v, _, _)| *v).collect();
                        let filtered_hover: Vec<Vec<(String, String)>> = indexed_data.iter().map(|(_, _, _, h)| h.clone()).collect();
                        let original_indices: Vec<usize> = indexed_data.iter().map(|(idx, _, _, _)| *idx).collect();

                        let (_, max_val) = simd_ops::find_minmax(&filtered_values);
                        let max_val = max_val.max(1.0);
                        let scale_max = ((max_val as i32 + 20) / 10) as f32 * 10.0;

                        let (width, height) = if orientation { (1200.0f32, 700.0f32) } else { (1200.0f32, 700.0f32) };
                        let (pad_left, pad_right, pad_top, pad_bottom) = (80.0, 40.0, 40.0, 80.0);

                        let plot_w = width - pad_left - pad_right;
                        let plot_h = height - pad_top - pad_bottom;

                        let mut svg = String::with_capacity(filtered_values.len() * 250 + 4096);
                        svg.push_str(&format!("<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" style=\"background:white\">", width as i32, height as i32, width as i32, height as i32));

                        let colors = [0x1f77b4u32, 0xff7f0e, 0x2ca02c, 0xd62728, 0x9467bd, 0x8c564b, 0xe377c2, 0x7f7f7f, 0xbcbd22, 0x17becf];
                        
                        render_svg_by_type(chart_type, &filtered_labels, &filtered_values, &colors, &original_indices, orientation, pad_left, pad_top, pad_right, pad_bottom, plot_w, plot_h, scale_max, width, height, &mut svg);

                        svg.push_str(&format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#000\" stroke-width=\"2\"/>", pad_left, pad_top + plot_h, pad_left, pad_top));
                        svg.push_str(&format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#000\" stroke-width=\"2\"/>", pad_left, pad_top + plot_h, width - pad_right, pad_top + plot_h));

                        let tick_count = 5i32;
                        let tick_step = scale_max / tick_count as f32;
                        
                        for i in 0..=tick_count {
                            let val = i as f32 * tick_step;
                            let y = pad_top + plot_h - (val * (plot_h / scale_max));
                            svg.push_str(&format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#ccc\" stroke-width=\"1\"/>", pad_left - 5.0, y, width - pad_right, y));
                            svg.push_str(&format!("<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" font-size=\"12\" fill=\"#666\">{:.0}</text>", pad_left - 10.0, y + 4.0, val));
                        }

                        for (i, label) in filtered_labels.iter().enumerate() {
                            let x_step = plot_w / (filtered_labels.len().max(1) as f32 - 1.0).max(1.0);
                            let x = pad_left + (i as f32 * x_step);
                            svg.push_str(&format!("<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"10\" fill=\"#666\" transform=\"rotate(-45 {:.1} {:.1})\">{}</text>", x, pad_top + plot_h + 20.0, x, pad_top + plot_h + 20.0, label));
                        }

                        svg.push_str("</svg>");

                        let mut hover_json = serde_json::json!({});
                        for (new_idx, pairs) in filtered_hover.iter().enumerate() {
                            let fields: Vec<Vec<String>> = pairs.iter().map(|(k, v)| vec![k.clone(), v.clone()]).collect();
                            hover_json[new_idx.to_string()] = serde_json::json!({"fields": fields});
                        }

                        let state_json = serde_json::json!({
                            "labels": filtered_labels,
                            "values": filtered_values,
                            "hover_data": hover_json,
                            "visible_count": filtered_labels.len()
                        });

                        let exporter = HtmlExporter::new(crate::bindings::HtmlExportConfig {
                            width: width as i32,
                            height: height as i32,
                            title: title.clone(),
                            theme: crate::bindings::HtmlTheme::Light,
                        }).with_svg(svg).with_hover(state_json).title(&title);

                        let html = exporter.build_html();

                        let timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_secs())
                            .unwrap_or(0);
                        let filepath = format!("seraplot_export_{}.html", timestamp);
                        let _ = std::fs::write(&filepath, html);
                    });
                }
                ui.label(&zoom_label);
            });
        });

        if self.show_list {
            let elements_data = {
                let data = self.data.lock().unwrap();
                data.as_ref().map(|d| d.labels.clone()).unwrap_or_default()
            };

            egui::SidePanel::left("element_list")
                .resizable(true)
                .default_width(180.0)
                .show(ctx, |ui| {
                    ui.heading("Elements");
                    ui.separator();
                    
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            for (idx, label) in elements_data.iter().enumerate() {
                                if idx < self.visible_elements.capacity() {
                                    let mut checked = self.visible_elements.get(idx);
                                    if ui.checkbox(&mut checked, label).changed() {
                                        if checked {
                                            self.visible_elements.set(idx);
                                        } else {
                                            self.visible_elements.clear(idx);
                                        }
                                    }
                                }
                            }
                        });
                });
        }

        let d_clone = {
            let data = self.data.lock().unwrap();
            data.as_ref().map(|d| ChartData {
                labels: d.labels.clone(),
                values: d.values.clone(),
                title: d.title.clone(),
                hover_data: d.hover_data.clone(),
                tooltip_bg_color: d.tooltip_bg_color,
                tooltip_text_color: d.tooltip_text_color,
            })
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            
            if let Some(ref d) = d_clone {
                let mut data_hash = 0u64;
                for &val in &d.values {
                    data_hash = data_hash.wrapping_mul(31).wrapping_add(val.to_bits() as u64);
                }
                
                let render_state = (self.orientation, self.current_chart_kind, self.sort_mode, self.is_3d_mode);
                
                if data_hash != self.last_data_hash || render_state != self.last_render_state {
                    self.last_data_hash = data_hash;
                    self.last_render_state = render_state;
                    let new_cache_key = CacheKey {
                        chart_type: self.current_chart_kind,
                        is_3d: false,
                        vertical: self.orientation,
                        sort_mode: self.sort_mode,
                    };
                    self.render_cache.invalidate_except(new_cache_key);
                }
            }
            
            if let Some(d) = d_clone {
                ui.vertical_centered(|ui| {
                    ui.heading(&d.title);
                });
                ui.separator();
                
                if d.values.is_empty() {
                    ui.label("No data");
                } else if self.is_3d_mode {
                    self.render_3d_viewer(ctx, ui, &d);
                } else {
                    self.render_plot(ctx, ui, &d, self.orientation, self.current_chart_kind);
                }
            } else {
                ui.label("Waiting for data...");
            }
            
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        });

        if self.show_processor_menu {
            let mut filter_threshold = self.filter_threshold;
            let mut limit_value = self.limit_value;
            let mut apply_filter = false;
            let mut apply_limit = false;
            let mut compute_stats = false;
            let mut reset_all = false;
            
            egui::Window::new("⚙ Processor")
                .open(&mut self.show_processor_menu)
                .default_width(350.0)
                .show(ctx, |ui| {
                    ui.label("Data Processing Operations");
                    ui.separator();
                    
                    ui.label("Filter");
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(&mut filter_threshold, 0.0..=100.0).text("Threshold %"));
                        if ui.button("Apply").clicked() {
                            apply_filter = true;
                        }
                    });
                    
                    ui.separator();
                    ui.label("Limit");
                    ui.horizontal(|ui| {
                        let mut limit_str = format!("{}", limit_value);
                        if ui.text_edit_singleline(&mut limit_str).changed() {
                            if let Ok(val) = limit_str.parse::<usize>() {
                                limit_value = val;
                            }
                        }
                        if ui.button("Apply").clicked() {
                            apply_limit = true;
                        }
                    });
                    
                    ui.separator();
                    ui.label("Statistics");
                    if ui.button("Compute").clicked() {
                        compute_stats = true;
                    }
                    
                    ui.separator();
                    if ui.button("Reset All").clicked() {
                        reset_all = true;
                    }
                });
            
            self.filter_threshold = filter_threshold;
            self.limit_value = limit_value;
            
            if apply_filter {
                self.apply_processor_filter();
            }
            if apply_limit {
                self.apply_processor_limit();
            }
            if compute_stats {
                self.show_stats = true;
            }
            if reset_all {
                {
                    self.visible_elements.set_all();
                    self.selected_elements.set_all();
                }
            }
            self.compute_statistics();
        }

        if self.show_stats {
            egui::Window::new("📊 Statistics")
                .open(&mut self.show_stats)
                .default_width(300.0)
                .show(ctx, |ui| {
                    if self.aggregation_results.is_empty() {
                        ui.label("No statistics computed");
                    } else {
                        for (key, value) in &self.aggregation_results {
                            ui.label(format!("{}: {:.2}", key, value));
                        }
                    }
                });
        }

        if self.show_transform_menu {
            let chart_types = crate::plot::default::get_current_group_types();
            
            egui::Window::new("Transform")
                .open(&mut self.show_transform_menu)
                .default_width(200.0)
                .show(ctx, |ui| {
                    ui.label("Chart Types");
                    ui.separator();
                    
                    for (kind, name) in &chart_types {
                        let is_selected = self.current_chart_kind == *kind;
                        let button_text = if is_selected {
                            format!("[{}]", name)
                        } else {
                            name.to_string()
                        };
                        
                        if ui.button(&button_text).clicked() {
                            self.current_chart_kind = *kind;
                            sera_set_current_chart_kind(*kind);
                            self.last_render_state = (false, 255, -1, false);
                            ctx.request_repaint();
                        }
                    }
                });
        }

        if self.show_wiki {
            egui::Window::new("📚 SeraPlot Wiki - API Documentation")
                .open(&mut self.show_wiki)
                .default_width(400.0)
                .default_height(600.0)
                .default_pos([1200.0 - 400.0 - 10.0, 50.0])
                .show(ctx, |ui| {
                    if let Some(ref mut wiki_viewer) = self.wiki_viewer {
                        wiki_viewer.render(ui);
                    }
                });
        }
    }
}

impl ChartApp {
    fn render_tooltip(&mut self, ctx: &egui::Context, painter: &egui::Painter, pos: egui::Pos2, rect: egui::Rect, actual_idx: usize, d: &ChartData) {
        let hover = &d.hover_data[actual_idx];
        
        if hover.is_empty() {
            return;
        }
        
        let img_size = 60.0;
        let line_height = 15.0;
        let padding = 8.0;
        let mut tooltip_height = 0.0;
        
        for (k, _) in hover {
            if k == "image" {
                tooltip_height += img_size + padding;
            } else {
                tooltip_height += line_height;
            }
        }

        let max_line_width = hover.iter()
            .filter(|(k, _)| k != "image")
            .map(|(_, v)| v.len() as f32 * 6.5)
            .fold(0.0, f32::max)
            .max(img_size);
        
        let mut tooltip_x = pos.x + 15.0;
        let mut tooltip_y = pos.y - 80.0;
        
        if tooltip_x + max_line_width > rect.right() {
            tooltip_x = (pos.x - max_line_width - 10.0).max(rect.left());
        }
        
        if tooltip_y + tooltip_height > rect.bottom() {
            tooltip_y = (pos.y + 20.0).min(rect.bottom() - tooltip_height - 5.0);
        }
        
        if tooltip_y < rect.top() {
            tooltip_y = rect.top() + 5.0;
        }
        
        let mut current_y = tooltip_y;
        
        let text_color = egui::Color32::from_rgba_unmultiplied(
            d.tooltip_text_color.0, 
            d.tooltip_text_color.1, 
            d.tooltip_text_color.2, 
            d.tooltip_text_color.3
        );
        
        for (k, v) in hover {
            if k == "image" {
                if let Some(color_img) = self.image_loader.load_image(v) {
                    let img_rect = egui::Rect::from_min_size(
                        egui::pos2(tooltip_x, current_y),
                        egui::vec2(img_size, img_size),
                    );
                    let texture = ctx.load_texture("tooltip_img", color_img, egui::TextureOptions::LINEAR);
                    painter.image(texture.id(), img_rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
                    current_y += img_size + padding;
                }
            } else {
                let line = format!("{}: {}", k, v);
                painter.text(
                    egui::pos2(tooltip_x, current_y),
                    egui::Align2::LEFT_TOP,
                    &line,
                    egui::FontId::proportional(11.0),
                    text_color,
                );
                current_y += line_height;
            }
        }
    }

    fn get_sorted_visible_indices(&self, d: &ChartData) -> Vec<usize> {
        let mut visible = Vec::with_capacity(d.values.len());
        
        for i in self.visible_elements.iter_set() {
            if i < d.values.len() && self.selected_elements.get(i) {
                visible.push(i);
            }
        }

        match self.sort_mode {
            1 => visible.sort_by(|&a, &b| d.values[a].partial_cmp(&d.values[b]).unwrap_or(std::cmp::Ordering::Equal)),
            2 => visible.sort_by(|&a, &b| d.values[b].partial_cmp(&d.values[a]).unwrap_or(std::cmp::Ordering::Equal)),
            _ => {}
        }
        visible
    }

    fn get_bar_bounds(&self, vis_idx: usize, value: f64, max_val: f64, visible_count: usize, plot_rect: egui::Rect, vertical: bool) -> (f32, f32, f32, f32) {
        let element_size = if visible_count > 1 {
            if vertical {
                plot_rect.width() / visible_count as f32
            } else {
                plot_rect.height() / visible_count as f32
            }
        } else {
            if vertical { plot_rect.width() } else { plot_rect.height() }
        };

        let norm_val = (value as f32 / max_val as f32).min(1.0).max(0.0);
        let margin_x = element_size * 0.15;
        let margin_y = element_size * 0.05;

        if vertical {
            let bar_left = plot_rect.left() + (element_size * vis_idx as f32);
            let bar_right = bar_left + element_size;
            let bar_top = plot_rect.bottom() - norm_val * plot_rect.height();
            let bar_bottom = plot_rect.bottom();

            (bar_left - margin_x, bar_right + margin_x, bar_top - margin_y, bar_bottom + margin_y)
        } else {
            let bar_top = plot_rect.top() + (element_size * vis_idx as f32);
            let bar_bottom = bar_top + element_size;
            let bar_left = plot_rect.left();
            let bar_right = plot_rect.left() + norm_val * plot_rect.width();

            (bar_left - margin_x, bar_right + margin_x, bar_top - margin_y, bar_bottom + margin_y)
        }
    }

    fn render_plot(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData, vertical: bool, chart_type: u8) {
        let renderer = GenericPlotRenderer { vertical };
        let metrics = if vertical { PlotMetrics::vertical(self.zoom) } else { PlotMetrics::horizontal(self.zoom) };
        let (_, max_val) = simd_ops::find_minmax(&d.values);
        let max_val = max_val.max(1.0);
        
        let cache_key = CacheKey {
            chart_type,
            is_3d: self.is_3d_mode,
            vertical,
            sort_mode: self.sort_mode,
        };
        self.render_cache.update_active(cache_key);
        
        let visible_indices = self.get_sorted_visible_indices(d);
        let visible_count = visible_indices.len();
        
        if visible_count == 0 {
            ui.label("No visible data");
            return;
        }
        
        let mut vis_optimizer = VisibilityOptimizer::new();
        
        let scroll_id = egui::Id::new((chart_type, vertical, self.sort_mode));
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .id_source(scroll_id)
            .show(ui, |ui| {
                let (response, painter) = ui.allocate_painter(
                    egui::Vec2::new(metrics.allocate_size().x, metrics.allocate_size().y),
                    egui::Sense::click_and_drag(),
                );
                
                self.render_state.update_viewport(response.rect.width(), response.rect.height());
                self.render_state.update_zoom(self.zoom);
                
                let plot_rect = metrics.with_rect(response.rect.min);
                renderer.render_axes(&painter, plot_rect, max_val as f64);
                
                let colors = self.color_cache.colors();
                let point_computer = PointComputeBuilder::new().build();
                let points = point_computer.compute_points(
                    &d.values,
                    &visible_indices,
                    max_val,
                    plot_rect,
                    vertical,
                );
                
                vis_optimizer.set_padding(20.0);
                let _visible_point_indices = vis_optimizer.filter_visible(&points, plot_rect);
                
                self.batch_renderer.clear();
                
                let element_width = if visible_count > 1 {
                    plot_rect.width() / visible_count as f32
                } else {
                    plot_rect.width()
                };
                
                if response.drag_started() {
                    let pos = response.interact_pointer_pos().unwrap_or(plot_rect.center());
                    self.selection_start = Some(pos);
                    self.selection_end = Some(pos);
                    self.selection_active = true;
                }
                
                if self.selection_active {
                    if let Some(pos) = response.interact_pointer_pos() {
                        self.selection_end = Some(pos);
                    }
                }
                
                if response.drag_stopped() {
                    if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
                        let sel_min_x = start.x.min(end.x);
                        let sel_max_x = start.x.max(end.x);
                        let sel_min_y = start.y.min(end.y);
                        let sel_max_y = start.y.max(end.y);
                        
                        let threshold = 5.0;
                        if (sel_max_x - sel_min_x) > threshold && (sel_max_y - sel_min_y) > threshold {
                            self.selected_elements.clear_all();
                            for (vis_idx, &actual_idx) in visible_indices.iter().enumerate() {
                                let value = d.values.get(actual_idx).copied().unwrap_or(0.0);
                                let (bar_min_x, bar_max_x, bar_min_y, bar_max_y) = self.get_bar_bounds(vis_idx, value, max_val, visible_count, plot_rect, vertical);
                                
                                let intersects = sel_min_x < bar_max_x && sel_max_x > bar_min_x && 
                                               sel_min_y < bar_max_y && sel_max_y > bar_min_y;
                                
                                if intersects {
                                    self.selected_elements.set(actual_idx);
                                }
                            }
                        }
                    }
                    self.selection_start = None;
                    self.selection_end = None;
                    self.selection_active = false;
                }
                
                let plot_ctx = PlotRenderContext {
                    painter: &painter,
                    plot_rect,
                    colors,
                    hovered_idx: self.hovered_idx,
                    values: &d.values,
                    max_val,
                    visible_indices: &visible_indices,
                    vertical,
                    labels: &d.labels,
                };
                
                render_plot_by_type(chart_type, plot_ctx);
                
                if self.selection_active {
                    if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
                        let min_x = start.x.min(end.x);
                        let max_x = start.x.max(end.x);
                        let min_y = start.y.min(end.y);
                        let max_y = start.y.max(end.y);
                        
                        let rect = egui::Rect::from_min_max(
                            egui::pos2(min_x, min_y),
                            egui::pos2(max_x, max_y),
                        );
                        
                        painter.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, egui::Color32::from_rgb(31, 119, 180)));
                        painter.rect_filled(rect, 0.0, egui::Color32::from_rgba_premultiplied(31, 119, 180, 20));
                    }
                }
                
                for (i, &point) in points.iter().enumerate() {
                    if let Some(&actual_idx) = visible_indices.get(i) {
                        if self.hovered_idx.map(|h| h == actual_idx).unwrap_or(false) {
                            self.render_tooltip(ctx, &painter, point, plot_rect, actual_idx, d);
                        }
                    }
                }
                
                if response.hovered() && !self.selection_active {
                    if let Some(pos) = ctx.pointer_latest_pos() {
                        if plot_rect.contains(pos) {
                            let mut hovered = None;
                            for (vis_idx, &actual_idx) in visible_indices.iter().enumerate() {
                                let value = d.values.get(actual_idx).copied().unwrap_or(0.0);
                                let (bar_min_x, bar_max_x, bar_min_y, bar_max_y) = self.get_bar_bounds(vis_idx, value, max_val, visible_count, plot_rect, vertical);
                                
                                if pos.x >= bar_min_x && pos.x <= bar_max_x && 
                                   pos.y >= bar_min_y && pos.y <= bar_max_y {
                                    hovered = Some(actual_idx);
                                    break;
                                }
                            }
                            self.hovered_idx = hovered;
                        } else {
                            self.hovered_idx = None;
                        }
                    }
                } else {
                    self.hovered_idx = None;
                }
            });
    }

    fn render_3d_viewer(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData) {
        self.advanced_viewer_3d.render_controls(ui);
        
        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(800.0, 600.0),
            egui::Sense::click_and_drag(),
        );
        
        if response.drag_started() {
            self.advanced_viewer_3d.is_dragging = true;
            self.advanced_viewer_3d.last_mouse_pos = response.interact_pointer_pos();
        }
        
        if self.advanced_viewer_3d.is_dragging {
            if let Some(curr_pos) = response.interact_pointer_pos() {
                if let Some(last_pos) = self.advanced_viewer_3d.last_mouse_pos {
                    let delta_x = (curr_pos.x - last_pos.x) * 0.01;
                    let delta_y = (curr_pos.y - last_pos.y) * 0.01;
                    self.advanced_viewer_3d.camera_controller.rotate_orbit(delta_x, -delta_y);
                }
                self.advanced_viewer_3d.last_mouse_pos = Some(curr_pos);
            }
        }
        
        if response.drag_stopped() {
            self.advanced_viewer_3d.is_dragging = false;
            self.advanced_viewer_3d.last_mouse_pos = None;
        }
        
        let scroll = ui.input(|i| i.raw_scroll_delta.y);
        if scroll != 0.0 {
            let zoom_factor = if scroll > 0.0 { 0.85 } else { 1.15 };
            self.advanced_viewer_3d.camera_controller.zoom(zoom_factor);
        }
        
        let rect_width = response.rect.width();
        let rect_height = response.rect.height();
        self.advanced_viewer_3d.camera_controller.set_viewport(rect_width, rect_height);
        
        let visible_indices = self.get_sorted_visible_indices(d);
        
        let max_val = d.values.iter().copied().fold(0.0_f64, f64::max).max(1.0);
        let colors = self.color_cache.colors();
        
        let chart_type = self.current_chart_kind;
        let chart_id = chart_type + 3;
        let ctx_3d = Plot3DRenderContext {
            painter: &painter,
            plot_rect: response.rect,
            colors,
            hovered_idx: self.hovered_idx,
            values: &d.values,
            max_val,
            visible_indices: &visible_indices,
            camera_controller: &self.advanced_viewer_3d.camera_controller,
        };
        render_3d_by_type(chart_id, ctx_3d);
        
        for &actual_idx in &visible_indices {
            if self.hovered_idx.map(|h| h == actual_idx).unwrap_or(false) {
                if let Some(pos) = ctx.pointer_latest_pos() {
                    self.render_tooltip(ctx, &painter, pos, response.rect, actual_idx, d);
                }
            }
        }
        
        if response.hovered() {
            if let Some(mouse_pos) = ctx.pointer_latest_pos() {
                if response.rect.contains(mouse_pos) {
                    let positions = crate::plot::default::_3d::get_3d_positions(
                        chart_id,
                        &d.values,
                        max_val,
                        &visible_indices,
                        &self.advanced_viewer_3d.camera_controller,
                        response.rect,
                    );
                    let detector = Hover3DDetector::new(positions);
                    self.hovered_idx = detector.detect(mouse_pos);
                } else {
                    self.hovered_idx = None;
                }
            }
        } else {
            self.hovered_idx = None;
        }
    }

    fn apply_processor_filter(&mut self) {
        let (values, count) = {
            if let Ok(data) = self.data.lock() {
                if let Some(d) = data.as_ref() {
                    (d.values.clone(), d.values.len())
                } else {
                    return;
                }
            } else {
                return;
            }
        };

        let (_, max_val) = simd_ops::find_minmax(&values);
        let cutoff = max_val * self.filter_threshold.clamp(0.0, 100.0) / 100.0;
        
        self.visible_elements.clear_all();
        for (idx, &value) in values.iter().enumerate() {
            if value >= cutoff {
                self.visible_elements.set(idx);
            }
        }
    }

    fn apply_processor_limit(&mut self) {
        let (labels, values) = {
            if let Ok(data) = self.data.lock() {
                if let Some(d) = data.as_ref() {
                    (d.labels.clone(), d.values.clone())
                } else {
                    return;
                }
            } else {
                return;
            }
        };

        let temp_d = ChartData {
            labels: labels,
            values: values,
            title: String::new(),
            hover_data: Vec::new(),
            tooltip_bg_color: (0, 0, 0, 0),
            tooltip_text_color: (0, 0, 0, 0),
        };

        let sorted_indices = self.get_sorted_visible_indices(&temp_d);
        
        self.visible_elements.clear_all();
        for (count, &idx) in sorted_indices.iter().enumerate() {
            if count < self.limit_value {
                self.visible_elements.set(idx);
            }
        }
    }

    fn compute_statistics(&mut self) {
        let values = {
            let data = self.data.lock().unwrap();
            if let Some(d) = data.as_ref() {
                let mut v = Vec::with_capacity(d.values.len());
                for idx in self.visible_elements.iter_set() {
                    if idx < d.values.len() {
                        v.push(d.values[idx]);
                    }
                }
                v
            } else {
                Vec::new()
            }
        };
        
        self.aggregation_results.clear();
        if !values.is_empty() {
            let sum: f64 = values.iter().sum();
            let len = values.len() as f64;
            [("Sum", sum),
             ("Mean", sum / len),
             ("Max", values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))),
             ("Min", values.iter().fold(f64::INFINITY, |a, &b| a.min(b))),
             ("Count", len)]
                .into_iter()
                .for_each(|(k, v)| { self.aggregation_results.insert(k.to_string(), v); });
        }
    }

    fn generate_svg(&self, d: &ChartData) -> String {
        use crate::bindings::FastChartRenderer;
        use crate::bindings::FastChartConfig;

        let visible_indices = self.get_sorted_visible_indices(d);
        
        let filtered_labels: Vec<String> = visible_indices.iter()
            .map(|&idx| d.labels.get(idx).cloned().unwrap_or_default())
            .collect();
        
        let filtered_values: Vec<f64> = visible_indices.iter()
            .map(|&idx| d.values.get(idx).copied().unwrap_or(0.0))
            .collect();

        let chart_type = self.current_chart_kind;
        let vertical = self.orientation;
        
        let (width, height, padding) = if vertical {
            (800.0, 600.0, 60.0)
        } else {
            (1000.0, 800.0, 300.0)
        };
        
        let config = FastChartConfig {
            chart_type,
            width,
            height,
            padding,
            vertical,
        };

        let renderer = FastChartRenderer::new(config, &d.title)
            .with_data(filtered_labels, filtered_values);
        
        renderer.render_svg()
    }
}

#[no_mangle]
pub extern "C" fn sera_show_chart_data(
    labels: *const *const c_char,
    values: *const f64,
    count: u32,
    title: *const c_char,
    group_name: *const c_char,
    hover_items: *const HoverItem,
) -> bool {
    sera_show_chart_data_full(
        labels,
        values,
        count,
        title,
        group_name,
        hover_items,
        255,
        255,
        255,
        255,
        0,
        0,
        0,
        255,
    )
}

#[no_mangle]
pub extern "C" fn sera_show_chart_data_full(
    labels: *const *const c_char,
    values: *const f64,
    count: u32,
    title: *const c_char,
    group_name: *const c_char,
    hover_items: *const HoverItem,
    bg_r: u8,
    bg_g: u8,
    bg_b: u8,
    bg_a: u8,
    txt_r: u8,
    txt_g: u8,
    txt_b: u8,
    txt_a: u8,
) -> bool {
    crate::bindings::init_chart_types();
    
    if labels.is_null() || values.is_null() || title.is_null() {
        return false;
    }

    let group = if group_name.is_null() {
        "default".to_string()
    } else {
        unsafe { CStr::from_ptr(group_name).to_string_lossy().into_owned() }
    };
    
    crate::bindings::load_group(&group);
    crate::plot::controller::set_current_chart_group(&group);

    let title_str = unsafe { CStr::from_ptr(title).to_string_lossy().into_owned() };
    let interner = crate::bindings::utils::get_interner();
    
    let mut label_vec = Vec::with_capacity(count as usize);
    let mut value_vec = Vec::with_capacity(count as usize);
    let mut hover_data_vec = Vec::with_capacity(count as usize);
    let mut hover_map: std::collections::HashMap<u32, Vec<(String, String)>> = std::collections::HashMap::new();
    
    if !hover_items.is_null() {
        let mut j = 0;
        loop {
            let item = unsafe { *hover_items.add(j) };
            if item.index == u32::MAX {
                break;
            }
            if item.index < count {
                let key = if !item.key.is_null() {
                    unsafe { CStr::from_ptr(item.key).to_string_lossy().into_owned() }
                } else {
                    String::new()
                };
                let value = if !item.value.is_null() {
                    unsafe { CStr::from_ptr(item.value).to_string_lossy().into_owned() }
                } else {
                    String::new()
                };
                
                if !key.is_empty() && !value.is_empty() {
                    hover_map.entry(item.index).or_insert_with(Vec::new).push((key, value));
                }
            }
            j += 1;
        }
    }

    for i in 0..count as usize {
        let label_ptr = unsafe { *(labels.add(i)) };
        let label_str = if !label_ptr.is_null() {
            unsafe { CStr::from_ptr(label_ptr).to_string_lossy() }
        } else {
            std::borrow::Cow::Borrowed("")
        };
        label_vec.push(interner.intern(&label_str).to_string());
        value_vec.push(unsafe { *(values.add(i)) });
        hover_data_vec.push(hover_map.remove(&(i as u32)).unwrap_or_default());
    }

    let data = ChartData {
        labels: label_vec,
        values: value_vec,
        title: title_str,
        hover_data: hover_data_vec,
        tooltip_bg_color: (bg_r, bg_g, bg_b, bg_a),
        tooltip_text_color: (txt_r, txt_g, txt_b, txt_a),
    };

    let app = ChartAppBuilder::new(data).build();
    launch_chart_app(app)
}

#[no_mangle]
pub extern "C" fn sera_show_chart(_svg: *const c_char, _title: *const c_char, _width: u32, _height: u32) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn sera_show_chart_data_json(
    labels: *const *const c_char,
    values: *const f64,
    count: u32,
    title: *const c_char,
    group_name: *const c_char,
    hover_json: *const c_char,
) -> bool {
    crate::bindings::init_chart_types();
    
    if labels.is_null() || values.is_null() || title.is_null() {
        return false;
    }

    let group = if group_name.is_null() {
        "default".to_string()
    } else {
        unsafe { CStr::from_ptr(group_name).to_string_lossy().into_owned() }
    };
    
    crate::bindings::load_group(&group);
    crate::plot::controller::set_current_chart_group(&group);

    let title_str = unsafe { CStr::from_ptr(title).to_string_lossy().into_owned() };
    let mut label_vec = Vec::new();
    let mut value_vec = Vec::new();
    let mut hover_data_vec = Vec::new();

    for i in 0..count as usize {
        let label_ptr = unsafe { *(labels.add(i)) };
        if !label_ptr.is_null() {
            label_vec.push(unsafe { CStr::from_ptr(label_ptr).to_string_lossy().into_owned() });
        }
        value_vec.push(unsafe { *(values.add(i)) });
    }

    let mut hover_map: std::collections::HashMap<u32, Vec<(String, String)>> = std::collections::HashMap::new();
    
    if !hover_json.is_null() {
        let json_str = unsafe { CStr::from_ptr(hover_json).to_string_lossy().into_owned() };
        if let Ok(serde_json::Value::Array(items)) = serde_json::from_str::<serde_json::Value>(&json_str) {
            for item in items {
                if let serde_json::Value::Object(obj) = item {
                    if let Some(serde_json::Value::Number(idx)) = obj.get("index") {
                        if let Some(idx_u64) = idx.as_u64() {
                            let index = idx_u64 as u32;
                            if index >= count {
                                continue;
                            }
                            for (k, v) in obj.iter() {
                                if k != "index" {
                                    if let serde_json::Value::String(val) = v {
                                        hover_map.entry(index).or_insert_with(Vec::new).push((k.clone(), val.clone()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for i in 0..count as usize {
        let hover_pairs = hover_map.remove(&(i as u32)).unwrap_or_default();
        hover_data_vec.push(hover_pairs);
    }

    let data = ChartData {
        labels: label_vec,
        values: value_vec,
        title: title_str,
        hover_data: hover_data_vec,
        tooltip_bg_color: (255, 255, 255, 255),
        tooltip_text_color: (0, 0, 0, 255),
    };

    let app = ChartAppBuilder::new(data).build();
    launch_chart_app(app)
}

#[no_mangle]
pub extern "C" fn sera_show_chart_value(chart_json: *const c_char) -> bool {
    crate::bindings::init_chart_types();
    
    if chart_json.is_null() {
        return false;
    }

    let json_str = unsafe { CStr::from_ptr(chart_json).to_string_lossy().into_owned() };
    let Ok(root) = serde_json::from_str::<serde_json::Value>(&json_str) else {
        return false;
    };
    
    let obj = match root {
        serde_json::Value::Object(o) => o,
        _ => return false,
    };

    let title_str = obj.get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Chart")
        .to_string();
    
    let group = obj.get("group")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    crate::bindings::load_group(&group);
    crate::plot::controller::set_current_chart_group(&group);

    let mut label_vec = Vec::new();
    let mut value_vec = Vec::new();
    let mut hover_data_vec = Vec::new();

    if let Some(serde_json::Value::Array(labels)) = obj.get("labels") {
        for label in labels {
            if let Some(s) = label.as_str() {
                label_vec.push(s.to_string());
            }
        }
    }

    if let Some(serde_json::Value::Array(values)) = obj.get("values") {
        for val in values {
            if let Some(n) = val.as_f64() {
                value_vec.push(n);
            }
        }
    }

    let mut hover_map: std::collections::HashMap<u32, Vec<(String, String)>> = std::collections::HashMap::new();
    
    if let Some(serde_json::Value::Array(hover_items)) = obj.get("hover") {
        for item in hover_items {
            if let serde_json::Value::Object(item_obj) = item {
                if let Some(serde_json::Value::Number(idx_num)) = item_obj.get("index") {
                    if let Some(idx) = idx_num.as_u64() {
                        let index = idx as u32;
                        if let Some(serde_json::Value::Array(fields)) = item_obj.get("fields") {
                            for field in fields {
                                if let serde_json::Value::Array(pair) = field {
                                    if pair.len() == 2 {
                                        if let (Some(serde_json::Value::String(key)), Some(serde_json::Value::String(val))) = (pair.get(0), pair.get(1)) {
                                            hover_map.entry(index)
                                                .or_insert_with(Vec::new)
                                                .push((key.clone(), val.clone()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for i in 0..label_vec.len() {
        let hover_pairs = hover_map.remove(&(i as u32)).unwrap_or_default();
        hover_data_vec.push(hover_pairs);
    }

    let data = ChartData {
        labels: label_vec,
        values: value_vec,
        title: title_str,
        hover_data: hover_data_vec,
        tooltip_bg_color: (255, 255, 255, 255),
        tooltip_text_color: (0, 0, 0, 255),
    };

    let app = ChartAppBuilder::new(data).build();
    launch_chart_app(app)
}

static CHART_ORIENTATION: std::sync::OnceLock<Mutex<bool>> = std::sync::OnceLock::new();
static CHART_SORT: std::sync::OnceLock<Mutex<i32>> = std::sync::OnceLock::new();
static CHART_KIND: std::sync::Mutex<u8> = std::sync::Mutex::new(1);
static VARIANT_REGISTRY: std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<u8, String>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));
static VARIANT_SELECTOR_ENABLED: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

fn get_orientation() -> &'static Mutex<bool> {
    CHART_ORIENTATION.get_or_init(|| Mutex::new(true))
}

fn get_sort() -> &'static Mutex<i32> {
    CHART_SORT.get_or_init(|| Mutex::new(0))
}

macro_rules! mutex_setter {
    ($val:expr, $lock:expr) => {
        if let Ok(mut guard) = $lock {
            *guard = $val;
        }
    };
}

macro_rules! mutex_getter {
    ($lock:expr, $default:expr) => {
        $lock.ok().map(|g| *g).unwrap_or($default)
    };
}

#[no_mangle]
pub extern "C" fn sera_set_chart_orientation(vertical: bool) {
    mutex_setter!(vertical, get_orientation().lock());
}

#[no_mangle]
pub extern "C" fn sera_get_chart_orientation() -> bool {
    mutex_getter!(get_orientation().lock(), true)
}

#[no_mangle]
pub extern "C" fn sera_set_chart_sort(mode: i32) {
    mutex_setter!(mode.clamp(0, 2), get_sort().lock());
}

#[no_mangle]
pub extern "C" fn sera_get_chart_sort() -> i32 {
    mutex_getter!(get_sort().lock(), 0)
}

#[no_mangle]
pub extern "C" fn sera_set_current_chart_kind(kind: u8) {
    mutex_setter!(kind, CHART_KIND.lock());
}

#[no_mangle]
pub extern "C" fn sera_get_current_chart_kind() -> u8 {
    mutex_getter!(CHART_KIND.lock(), 1)
}

#[no_mangle]
pub extern "C" fn sera_add_chart_variant(kind: u8, title: *const c_char) -> bool {
    if title.is_null() {
        return false;
    }
    let title_str = unsafe { CStr::from_ptr(title).to_string_lossy().to_string() };
    VARIANT_REGISTRY.lock().ok().map_or(false, |mut registry| {
        registry.insert(kind, title_str);
        true
    })
}

#[no_mangle]
pub extern "C" fn sera_enable_variant_selector(enable: bool) {
    mutex_setter!(enable, VARIANT_SELECTOR_ENABLED.lock());
}

#[no_mangle]
pub extern "C" fn sera_is_variant_selector_enabled() -> bool {
    mutex_getter!(VARIANT_SELECTOR_ENABLED.lock(), false)
}

#[no_mangle]
pub extern "C" fn sera_show_with_variants(enable_transform: bool, default_kind: u8) -> bool {
    sera_enable_variant_selector(enable_transform);
    sera_set_current_chart_kind(default_kind);
    true
}