use std::sync::{Arc, Mutex};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::collections::HashMap;
use super::image_loader::ImageLoader;
use super::cache::{RenderCache, ColorCache, CacheKey};
use super::viewer_3d::AdvancedViewer3D;
use super::wiki_viewer::WikiViewer;
use super::manager::button_manager::{ButtonManager, ButtonId};
use super::render::{AdvancedBatchRenderer, AdvancedBatchRendererBuilder, RenderState, DataCache, PointComputeBuilder, ChunkRenderBuilder, RenderPipeline, VisibilityOptimizer};
use crate::plot::default::{PlotRenderContext, render_plot_by_type};
use crate::plot::controller::plot_3d_controller::{Plot3DRenderContext, render_by_type as render_3d_by_type};
use crate::plot::CameraController;
use crate::bindings::{HtmlExporter, HtmlExportConfig, HtmlTheme, ChartStateBuilder};

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
            threshold: 15.0,
        }
    }
    
    fn detect(&self, mouse_pos: egui::Pos2) -> Option<usize> {
        let mut closest_idx = None;
        let mut closest_dist = self.threshold;
        
        for &(screen_pos, actual_idx) in &self.positions {
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
    visible_elements: Vec<bool>,
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
                    if let Ok(data) = self.data.lock() {
                        if let Some(d) = data.as_ref() {
                            self.visible_elements = vec![true; d.labels.len()];
                            self.selection_start = None;
                            self.selection_end = None;
                            self.selection_active = false;
                        }
                    }
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
                    if let Ok(data) = self.data.lock() {
                        if let Some(d) = data.as_ref() {
                            let d_clone = ChartData {
                                labels: d.labels.clone(),
                                values: d.values.clone(),
                                title: d.title.clone(),
                                hover_data: d.hover_data.clone(),
                                tooltip_bg_color: d.tooltip_bg_color,
                                tooltip_text_color: d.tooltip_text_color,
                            };
                            drop(data);
                            self.export_html(&d_clone);
                        }
                    }

                }
                ui.label(&zoom_label);
            });
        });

        if self.show_list {
            egui::SidePanel::left("element_list")
                .resizable(true)
                .default_width(180.0)
                .show(ctx, |ui| {
                    ui.heading("Elements");
                    ui.separator();
                    
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            let data = self.data.lock().unwrap();
                            if let Some(d) = data.as_ref() {
                                for (idx, label) in d.labels.iter().enumerate() {
                                    if idx < self.visible_elements.len() {
                                        ui.checkbox(&mut self.visible_elements[idx], label);
                                    }
                                }
                            }
                        });
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let data = self.data.lock().unwrap();
            let d_clone = data.as_ref().map(|d| ChartData {
                labels: d.labels.clone(),
                values: d.values.clone(),
                title: d.title.clone(),
                hover_data: d.hover_data.clone(),
                tooltip_bg_color: d.tooltip_bg_color,
                tooltip_text_color: d.tooltip_text_color,
            });
            drop(data);
            
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
                    let data = self.data.lock().unwrap();
                    if let Some(d) = data.as_ref() {
                        self.visible_elements = vec![true; d.labels.len()];
                    }
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
        let mut visible = (0..d.values.len())
            .filter(|i| i < &self.visible_elements.len() && self.visible_elements[*i])
            .collect::<Vec<_>>();

        match self.sort_mode {
            1 => visible.sort_by(|&a, &b| d.values[a].partial_cmp(&d.values[b]).unwrap_or(std::cmp::Ordering::Equal)),
            2 => visible.sort_by(|&a, &b| d.values[b].partial_cmp(&d.values[a]).unwrap_or(std::cmp::Ordering::Equal)),
            _ => {}
        }
        visible
    }

    fn render_plot(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData, vertical: bool, chart_type: u8) {
        let renderer = GenericPlotRenderer { vertical };
        let metrics = if vertical { PlotMetrics::vertical(self.zoom) } else { PlotMetrics::horizontal(self.zoom) };
        let max_val = d.values.iter().fold(0.0_f64, |a, &b| a.max(b)).max(1.0);
        
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
        
        // let render_pipeline = RenderPipeline::builder().build();
        // let chunk_renderer = ChunkRenderBuilder::new()
        //     .with_chunk_size(render_pipeline.get_optimal_batch_size(visible_count))
        //     .build();
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
                        let min_x = start.x.min(end.x);
                        let max_x = start.x.max(end.x);
                        let min_y = start.y.min(end.y);
                        let max_y = start.y.max(end.y);
                        
                        let threshold = 5.0;
                        if (max_x - min_x) > threshold && (max_y - min_y) > threshold {
                            for (i, &point) in points.iter().enumerate() {
                                if let Some(&actual_idx) = visible_indices.get(i) {
                                    let tolerance = 15.0;
                                    let intersects = point.x >= min_x - tolerance && point.x <= max_x + tolerance && 
                                                   point.y >= min_y - tolerance && point.y <= max_y + tolerance;
                                    self.visible_elements[actual_idx] = intersects;
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
                            let rel_pos = pos - plot_rect.min;
                            let visible_values: Vec<f64> = visible_indices.iter().map(|&i| d.values.get(i).copied().unwrap_or(0.0)).collect();
                            if let Some(vis_idx) = renderer.detect_hover(rel_pos, plot_rect, visible_count, &visible_values, max_val) {
                                if let Some(&actual_idx) = visible_indices.get(vis_idx) {
                                    self.hovered_idx = Some(actual_idx);
                                }
                            } else {
                                self.hovered_idx = None;
                            }
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
                        self.current_chart_kind,
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
        if let Ok(data) = self.data.lock() {
            if let Some(d) = data.as_ref() {
                let cutoff = d.values.iter().copied().fold(0.0_f64, f64::max) * self.filter_threshold.clamp(0.0, 100.0) / 100.0;
                self.visible_elements = d.values.iter().enumerate()
                    .map(|(idx, &value)| idx >= self.visible_elements.len() || value >= cutoff)
                    .collect();
            }
        }
    }

    fn apply_processor_limit(&mut self) {
        if let Ok(data) = self.data.lock() {
            if let Some(d) = data.as_ref() {
                let sorted_indices = self.get_sorted_visible_indices(d);
                let mut new_visible = vec![false; d.values.len()];
                
                for (count, &idx) in sorted_indices.iter().enumerate() {
                    if count < self.limit_value {
                        new_visible[idx] = true;
                    }
                }
                
                self.visible_elements = new_visible;
            }
        }
    }

    fn compute_statistics(&mut self) {
        let visible = {
            if let Ok(data) = self.data.lock() {
                if let Some(d) = data.as_ref() {
                    d.values.iter().enumerate()
                        .filter(|(i, _)| *i < self.visible_elements.len() && self.visible_elements[*i])
                        .map(|(_, &v)| v)
                        .collect::<Vec<f64>>()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        };
        
        self.aggregation_results.clear();
        if !visible.is_empty() {
            let sum: f64 = visible.iter().sum();
            let len = visible.len() as f64;
            [("Sum", sum),
             ("Mean", sum / len),
             ("Max", visible.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))),
             ("Min", visible.iter().fold(f64::INFINITY, |a, &b| a.min(b))),
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

    fn export_html(&self, d: &ChartData) {
        use crate::bindings::ImageProcessor;
        
        let svg = self.generate_svg(d);
        
        let mut images_base64 = Vec::new();
        for hover in &d.hover_data {
            let mut image_data_url = String::new();
            for (k, v) in hover {
                if k == "image" {
                    if let Some(data_url) = ImageProcessor::to_data_url(v) {
                        image_data_url = data_url;
                    }
                    break;
                }
            }
            images_base64.push(image_data_url);
        }
        
        let hover_as_maps: Vec<HashMap<String, String>> = d.hover_data.iter()
            .map(|pairs| {
                let mut map = HashMap::new();
                for (k, v) in pairs {
                    map.insert(k.clone(), v.clone());
                }
                map
            })
            .collect();
        
        let state = ChartStateBuilder::new()
            .labels(d.labels.clone())
            .values(d.values.clone())
            .title(d.title.clone())
            .hover_data(hover_as_maps)
            .tooltip_colors(d.tooltip_bg_color, d.tooltip_text_color)
            .orientation(self.orientation)
            .sort_mode(self.sort_mode)
            .chart_kind(self.current_chart_kind)
            .is_3d(self.is_3d_mode)
            .zoom(self.zoom)
            .filter_threshold(self.filter_threshold)
            .visible_elements(self.visible_elements.clone())
            .limit_value(self.limit_value)
            .images_base64(images_base64)
            .build();

        let config = HtmlExportConfig::default()
            .with_title(&d.title)
            .with_theme(HtmlTheme::Professional)
            .with_state_export(true)
            .with_controls(true);

        let exporter = HtmlExporter::new(config)
            .with_state(state)
            .with_data(d.labels.clone(), d.values.clone())
            .with_svg(svg);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let filepath = format!("seraplot_export_{}.html", timestamp);
        exporter.export_to_file_background(filepath);
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
    let mut label_vec = Vec::new();
    let mut value_vec = Vec::new();
    let mut hover_data_vec = Vec::new();

    let mut hover_map: std::collections::HashMap<u32, Vec<(String, String)>> = std::collections::HashMap::new();
    
    if !hover_items.is_null() {
        let mut j = 0;
        loop {
            let item = unsafe { *hover_items.add(j) };
            if item.index == u32::MAX {
                break;
            }
            if item.index >= count {
                j += 1;
                continue;
            }
            let key = if item.key.is_null() {
                String::new()
            } else {
                unsafe { CStr::from_ptr(item.key).to_string_lossy().into_owned() }
            };
            let value = if item.value.is_null() {
                String::new()
            } else {
                unsafe { CStr::from_ptr(item.value).to_string_lossy().into_owned() }
            };
            
            if !key.is_empty() && !value.is_empty() {
                hover_map.entry(item.index).or_insert_with(Vec::new).push((key, value));
            }
            
            j += 1;
        }
    }

    for i in 0..count as usize {
        let label_ptr = unsafe { *(labels.add(i)) };
        if !label_ptr.is_null() {
            label_vec.push(unsafe { CStr::from_ptr(label_ptr).to_string_lossy().into_owned() });
        }
        value_vec.push(unsafe { *(values.add(i)) });
        
        let hover_pairs = hover_map.remove(&(i as u32)).unwrap_or_default();
        hover_data_vec.push(hover_pairs);
    }

    let num_elements = label_vec.len();
    
    let data = ChartData {
        labels: label_vec,
        values: value_vec,
        title: title_str,
        hover_data: hover_data_vec,
        tooltip_bg_color: (bg_r, bg_g, bg_b, bg_a),
        tooltip_text_color: (txt_r, txt_g, txt_b, txt_a),
    };

    let app = ChartApp {
        data: Arc::new(Mutex::new(Some(data))),
        hovered_idx: None,
        zoom: 1.0,
        pan_x: 0.0,
        visible_elements: vec![true; num_elements],
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
    };

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 600.0]),
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "SeraPlot",
        native_options,
        Box::new(|_| Box::new(app)),
    );

    true
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

    let num_elements = label_vec.len();
    
    let data = ChartData {
        labels: label_vec,
        values: value_vec,
        title: title_str,
        hover_data: hover_data_vec,
        tooltip_bg_color: (255, 255, 255, 255),
        tooltip_text_color: (0, 0, 0, 255),
    };

    let app = ChartApp {
        data: Arc::new(Mutex::new(Some(data))),
        hovered_idx: None,
        zoom: 1.0,
        pan_x: 0.0,
        visible_elements: vec![true; num_elements],
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
    };

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 600.0]),
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "SeraPlot",
        native_options,
        Box::new(|_| Box::new(app)),
    );

    true
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
                        for (key, val) in item_obj.iter() {
                            if key != "index" {
                                if let Some(v_str) = val.as_str() {
                                    hover_map.entry(index)
                                        .or_insert_with(Vec::new)
                                        .push((key.clone(), v_str.to_string()));
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

    let num_elements = label_vec.len();
    
    let data = ChartData {
        labels: label_vec,
        values: value_vec,
        title: title_str,
        hover_data: hover_data_vec,
        tooltip_bg_color: (255, 255, 255, 255),
        tooltip_text_color: (0, 0, 0, 255),
    };

    let app = ChartApp {
        data: Arc::new(Mutex::new(Some(data))),
        hovered_idx: None,
        zoom: 1.0,
        pan_x: 0.0,
        visible_elements: vec![true; num_elements],
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
    };

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 600.0]),
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "SeraPlot",
        native_options,
        Box::new(|_| Box::new(app)),
    );

    true
}

static CHART_ORIENTATION: std::sync::OnceLock<Mutex<bool>> = std::sync::OnceLock::new();
static CHART_SORT: std::sync::OnceLock<Mutex<i32>> = std::sync::OnceLock::new();

fn get_orientation() -> &'static Mutex<bool> {
    CHART_ORIENTATION.get_or_init(|| Mutex::new(true))
}

fn get_sort() -> &'static Mutex<i32> {
    CHART_SORT.get_or_init(|| Mutex::new(0))
}

#[no_mangle]
pub extern "C" fn sera_set_chart_orientation(vertical: bool) {
    if let Ok(mut o) = get_orientation().lock() {
        *o = vertical;
    }
}

#[no_mangle]
pub extern "C" fn sera_get_chart_orientation() -> bool {
    get_orientation().lock().ok().map(|o| *o).unwrap_or(true)
}

#[no_mangle]
pub extern "C" fn sera_set_chart_sort(mode: i32) {
    if let Ok(mut s) = get_sort().lock() {
        *s = mode.clamp(0, 2);
    }
}

#[no_mangle]
pub extern "C" fn sera_get_chart_sort() -> i32 {
    get_sort().lock().ok().map(|s| *s).unwrap_or(0)
}

static CHART_KIND: std::sync::Mutex<u8> = std::sync::Mutex::new(1);
static VARIANT_REGISTRY: std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<u8, String>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));
static VARIANT_SELECTOR_ENABLED: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

#[no_mangle]
pub extern "C" fn sera_set_current_chart_kind(kind: u8) {
    if let Ok(mut k) = CHART_KIND.lock() {
        *k = kind;
    }
}

#[no_mangle]
pub extern "C" fn sera_get_current_chart_kind() -> u8 {
    CHART_KIND.lock().ok().map(|k| *k).unwrap_or(1)
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
    if let Ok(mut sel) = VARIANT_SELECTOR_ENABLED.lock() {
        *sel = enable;
    }
}

#[no_mangle]
pub extern "C" fn sera_is_variant_selector_enabled() -> bool {
    VARIANT_SELECTOR_ENABLED.lock().ok().map(|sel| *sel).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn sera_show_with_variants(enable_transform: bool, default_kind: u8) -> bool {
    sera_enable_variant_selector(enable_transform);
    sera_set_current_chart_kind(default_kind);
    true
}