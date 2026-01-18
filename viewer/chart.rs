use std::sync::{Arc, Mutex};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::collections::HashMap;
use super::image_loader::ImageLoader;

struct ChartData {
    labels: Vec<String>,
    values: Vec<f64>,
    title: String,
    hover_data: Vec<HashMap<String, String>>,
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
    fn map_point(&self, idx: usize, value: f64, max_val: f64, point_count: usize, plot_rect: egui::Rect) -> egui::Pos2;
    fn detect_hover(&self, rel_pos: egui::Vec2, plot_rect: egui::Rect, point_count: usize) -> Option<usize>;
}

struct GenericPlotRenderer {
    vertical: bool,
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

    fn map_point(&self, idx: usize, value: f64, max_val: f64, point_count: usize, plot_rect: egui::Rect) -> egui::Pos2 {
        let norm_val = value / max_val.max(1.0);
        if self.vertical {
            let x = plot_rect.left() + (plot_rect.width() / (point_count as f32 - 1.0).max(1.0)) * idx as f32;
            let y = plot_rect.bottom() - norm_val as f32 * plot_rect.height();
            egui::pos2(x, y)
        } else {
            let x = plot_rect.left() + norm_val as f32 * plot_rect.width();
            let y = plot_rect.top() + (plot_rect.height() / (point_count as f32 - 1.0).max(1.0)) * idx as f32;
            egui::pos2(x, y)
        }
    }

    fn detect_hover(&self, rel_pos: egui::Vec2, plot_rect: egui::Rect, point_count: usize) -> Option<usize> {
        if rel_pos.x < 0.0 || rel_pos.x > plot_rect.width() || rel_pos.y < 0.0 || rel_pos.y > plot_rect.height() {
            return None;
        }
        let norm = if self.vertical { rel_pos.x / plot_rect.width() } else { rel_pos.y / plot_rect.height() };
        Some(((norm * (point_count as f32 - 1.0)).round() as usize).min(point_count.saturating_sub(1)))
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
    #[allow(dead_code)]
    processor_mode: u8,
    filter_threshold: f64,
    show_stats: bool,
    show_processor_menu: bool,
    show_transform_menu: bool,
    aggregation_results: HashMap<String, f64>,
    limit_value: usize,
}

impl eframe::App for ChartApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(sort) = CHART_SORT.lock() {
            self.sort_mode = *sort;
        }
        
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("🔍 Zoom In").clicked() {
                    self.zoom *= 1.2;
                }
                if ui.button("🔍 Zoom Out").clicked() {
                    self.zoom /= 1.2;
                }
                if ui.button("⬜ Fit All").clicked() {
                    self.zoom = 1.0;
                    self.pan_x = 0.0;
                }
                if ui.button("📋 Elements").clicked() {
                    self.show_list = !self.show_list;
                }
                if ui.button(if self.orientation { "📊 Vertical" } else { "📈 Horizontal" }).clicked() {
                    self.orientation = !self.orientation;
                }
                let sort_label = match self.sort_mode {
                    1 => "↑ Asc",
                    2 => "↓ Desc",
                    _ => "⊙ None",
                };
                if ui.button(sort_label).clicked() {
                    self.sort_mode = (self.sort_mode + 1) % 3;
                    if let Ok(mut sort) = CHART_SORT.lock() {
                        *sort = self.sort_mode;
                    }
                }
                ui.separator();
                if ui.button("⚙ Processor").clicked() {
                    self.show_processor_menu = !self.show_processor_menu;
                }
                if ui.button("🔄 Transform").clicked() {
                    self.show_transform_menu = !self.show_transform_menu;
                }
                ui.label(format!("Zoom: {:.1}x", self.zoom));
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
            
            if let Some(mut d) = d_clone {
                let mut indices: Vec<usize> = (0..d.values.len()).collect();
                
                if self.sort_mode == 1 {
                    indices.sort_by(|&a, &b| {
                        d.values[a].partial_cmp(&d.values[b]).unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else if self.sort_mode == 2 {
                    indices.sort_by(|&a, &b| {
                        d.values[b].partial_cmp(&d.values[a]).unwrap_or(std::cmp::Ordering::Equal)
                    });
                }
                
                let sorted_labels: Vec<_> = indices.iter().map(|&i| d.labels[i].clone()).collect();
                let sorted_values: Vec<_> = indices.iter().map(|&i| d.values[i]).collect();
                let sorted_hover_data: Vec<_> = indices.iter().map(|&i| d.hover_data[i].clone()).collect();
                
                d.labels = sorted_labels;
                d.values = sorted_values;
                d.hover_data = sorted_hover_data;
                
                ui.vertical_centered(|ui| {
                    ui.heading(&d.title);
                });
                ui.separator();
                
                if d.values.is_empty() {
                    ui.label("No data");
                } else {
                    match self.current_chart_kind {
                        0 => {
                            if self.orientation {
                                self.render_line_vertical(ctx, ui, &d);
                            } else {
                                self.render_line_horizontal(ctx, ui, &d);
                            }
                        }
                        1 => {
                            if self.orientation {
                                self.render_scatter_vertical(ctx, ui, &d);
                            } else {
                                self.render_scatter_horizontal(ctx, ui, &d);
                            }
                        }
                        _ => {
                            if self.orientation {
                                self.render_bar_vertical(ctx, ui, &d);
                            } else {
                                self.render_bar_horizontal(ctx, ui, &d);
                            }
                        }
                    }
                }
            } else {
                ui.label("Waiting for data...");
            }
            
            ctx.request_repaint();
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
                self.compute_statistics();
            }
            if reset_all {
                let data = self.data.lock().unwrap();
                if let Some(d) = data.as_ref() {
                    self.visible_elements = vec![true; d.labels.len()];
                }
            }
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
            let chart_types = [
                (0u8, "Line"),
                (1u8, "Scatter"),
                (2u8, "Bar"),
            ];
            
            egui::Window::new("🔄 Transform")
                .open(&mut self.show_transform_menu)
                .default_width(250.0)
                .show(ctx, |ui| {
                    ui.label("Select Chart Type");
                    ui.separator();
                    
                    for (kind, name) in &chart_types {
                        let is_selected = self.current_chart_kind == *kind;
                        let button_text = if is_selected {
                            format!("✓ {}", name)
                        } else {
                            name.to_string()
                        };
                        
                        if ui.button(&button_text).clicked() {
                            self.current_chart_kind = *kind;
                        }
                    }
                });
        }
    }
}

impl ChartApp {
    fn render_bar_vertical(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData) {
        self.render_plot(ctx, ui, d, true, 2);
    }

    fn render_bar_horizontal(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData) {
        self.render_plot(ctx, ui, d, false, 2);
    }

    fn render_scatter_vertical(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData) {
        self.render_plot(ctx, ui, d, true, 1);
    }

    fn render_scatter_horizontal(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData) {
        self.render_plot(ctx, ui, d, false, 1);
    }

    fn render_line_vertical(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData) {
        self.render_plot(ctx, ui, d, true, 0);
    }

    fn render_line_horizontal(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData) {
        self.render_plot(ctx, ui, d, false, 0);
    }

    fn render_plot(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData, vertical: bool, _chart_type: u8) {
        let renderer = GenericPlotRenderer { vertical };
        let metrics = if vertical { PlotMetrics::vertical(self.zoom) } else { PlotMetrics::horizontal(self.zoom) };
        let max_val = d.values.iter().fold(0.0_f64, |a, &b| a.max(b));
        
        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(metrics.allocate_size().x, metrics.allocate_size().y),
            egui::Sense::hover(),
        );
        
        let plot_rect = metrics.with_rect(response.rect.min);
        renderer.render_axes(&painter, plot_rect, max_val);
        
        for (idx, value) in d.values.iter().enumerate() {
            if idx >= self.visible_elements.len() || !self.visible_elements[idx] { continue; }
            let pos = renderer.map_point(idx, *value, max_val, d.values.len(), plot_rect);
            let is_hovered = self.hovered_idx.map(|h| h == idx).unwrap_or(false);
            let (radius, color) = if is_hovered { (6.0, egui::Color32::from_rgb(255, 200, 0)) } else { (4.0, egui::Color32::from_rgb(100, 150, 255)) };
            painter.circle_filled(pos, radius, color);
        }
        
        if response.hovered() {
            if let Some(pos) = ctx.pointer_latest_pos() {
                let rel_pos = pos - plot_rect.min;
                self.hovered_idx = renderer.detect_hover(rel_pos, plot_rect, d.values.len());
            }
        }
    }

    fn apply_processor_filter(&mut self) {
        let data = self.data.lock().unwrap();
        if let Some(d) = data.as_ref() {
            let threshold = self.filter_threshold.max(0.0).min(100.0) / 100.0;
            let max_val = d.values.iter().copied().fold(0.0_f64, f64::max);
            let cutoff = max_val * threshold;
            for (idx, &value) in d.values.iter().enumerate() {
                if idx < self.visible_elements.len() {
                    self.visible_elements[idx] = value >= cutoff;
                }
            }
        }
    }

    fn apply_processor_limit(&mut self) {
        let data = self.data.lock().unwrap();
        if let Some(d) = data.as_ref() {
            let mut indices: Vec<usize> = (0..d.values.len()).collect();
            indices.sort_by(|&a, &b| d.values[b].partial_cmp(&d.values[a]).unwrap_or(std::cmp::Ordering::Equal));
            for (idx, &i) in indices.iter().enumerate() {
                if i < self.visible_elements.len() {
                    self.visible_elements[i] = idx < self.limit_value;
                }
            }
        }
    }

    fn compute_statistics(&mut self) {
        let data = self.data.lock().unwrap();
        if let Some(d) = data.as_ref() {
            let visible_values: Vec<f64> = d.values.iter().enumerate()
                .filter(|(i, _)| *i < self.visible_elements.len() && self.visible_elements[*i])
                .map(|(_, &v)| v)
                .collect();
            
            self.aggregation_results.clear();
            if !visible_values.is_empty() {
                let sum: f64 = visible_values.iter().sum();
                let mean = sum / visible_values.len() as f64;
                let max = visible_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let min = visible_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                
                self.aggregation_results.insert("Sum".to_string(), sum);
                self.aggregation_results.insert("Mean".to_string(), mean);
                self.aggregation_results.insert("Max".to_string(), max);
                self.aggregation_results.insert("Min".to_string(), min);
                self.aggregation_results.insert("Count".to_string(), visible_values.len() as f64);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_show_chart_data_with_hover(
    labels: *const *const c_char,
    values: *const f64,
    images: *const *const c_char,
    descriptions: *const *const c_char,
    count: u32,
    title: *const c_char,
) -> bool {
    sera_show_chart_data_with_hover_colors(labels, values, images, descriptions, count, title, 0, 0, 0, 0, 0, 0, 0, 255)
}

#[no_mangle]
pub extern "C" fn sera_show_chart_data_with_hover_colors(
    labels: *const *const c_char,
    values: *const f64,
    images: *const *const c_char,
    descriptions: *const *const c_char,
    count: u32,
    title: *const c_char,
    bg_r: u8,
    bg_g: u8,
    bg_b: u8,
    bg_a: u8,
    txt_r: u8,
    txt_g: u8,
    txt_b: u8,
    txt_a: u8,
) -> bool {
    if labels.is_null() || values.is_null() || title.is_null() {
        return false;
    }

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
        
        let mut hover = HashMap::new();
        
        if !images.is_null() {
            let img_ptr = unsafe { *(images.add(i)) };
            if !img_ptr.is_null() {
                let img = unsafe { CStr::from_ptr(img_ptr).to_string_lossy().into_owned() };
                if !img.is_empty() {
                    hover.insert("image".to_string(), img);
                }
            }
        }
        
        if !descriptions.is_null() {
            let desc_ptr = unsafe { *(descriptions.add(i)) };
            if !desc_ptr.is_null() {
                let desc = unsafe { CStr::from_ptr(desc_ptr).to_string_lossy().into_owned() };
                if !desc.is_empty() {
                    hover.insert("description".to_string(), desc);
                }
            }
        }
        
        hover_data_vec.push(hover);
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
        processor_mode: 0,
        filter_threshold: 0.0,
        show_stats: false,
        show_processor_menu: false,
        show_transform_menu: false,
        aggregation_results: HashMap::new(),
        limit_value: 50,
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

static CHART_ORIENTATION: Mutex<bool> = Mutex::new(true);

#[no_mangle]
pub extern "C" fn sera_set_chart_orientation(vertical: bool) {
    if let Ok(mut orientation) = CHART_ORIENTATION.lock() {
        *orientation = vertical;
    }
}

#[no_mangle]
pub extern "C" fn sera_get_chart_orientation() -> bool {
    CHART_ORIENTATION.lock().map(|o| *o).unwrap_or(true)
}

static CHART_SORT: Mutex<i32> = Mutex::new(0);

#[no_mangle]
pub extern "C" fn sera_set_chart_sort(mode: i32) {
    if let Ok(mut sort) = CHART_SORT.lock() {
        *sort = mode.clamp(0, 2);
    }
}

#[no_mangle]
pub extern "C" fn sera_get_chart_sort() -> i32 {
    CHART_SORT.lock().map(|s| *s).unwrap_or(0)
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
    CHART_KIND.lock().map(|k| *k).unwrap_or(1)
}

#[no_mangle]
pub extern "C" fn sera_add_chart_variant(kind: u8, title: *const c_char) -> bool {
    if title.is_null() {
        return false;
    }
    let title_str = unsafe { CStr::from_ptr(title).to_string_lossy().to_string() };
    if let Ok(mut registry) = VARIANT_REGISTRY.lock() {
        registry.insert(kind, title_str);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn sera_enable_variant_selector(enable: bool) {
    if let Ok(mut sel) = VARIANT_SELECTOR_ENABLED.lock() {
        *sel = enable;
    }
}

#[no_mangle]
pub extern "C" fn sera_is_variant_selector_enabled() -> bool {
    VARIANT_SELECTOR_ENABLED.lock().map(|sel| *sel).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn sera_show_with_variants(enable_transform: bool, default_kind: u8) -> bool {
    sera_enable_variant_selector(enable_transform);
    sera_set_current_chart_kind(default_kind);
    true
}