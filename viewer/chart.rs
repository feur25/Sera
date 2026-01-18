use std::sync::{Arc, Mutex};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::collections::HashMap;
use super::image_loader::ImageLoader;

#[derive(Clone)]
struct PlotVariant {
    kind: u8,
    title: String,
}

struct ChartData {
    labels: Vec<String>,
    values: Vec<f64>,
    title: String,
    hover_data: Vec<HashMap<String, String>>,
    tooltip_bg_color: (u8, u8, u8, u8),
    tooltip_text_color: (u8, u8, u8, u8),
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
    variants: Vec<PlotVariant>,
    show_variant_selector: bool,
    show_transform_menu: bool,
}

impl eframe::App for ChartApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sort_mode = crate::bindings::c_data::sera_get_plot_sort();
        self.orientation = crate::bindings::c_data::sera_get_plot_orientation();
        self.zoom = crate::bindings::c_data::sera_get_plot_zoom();
        self.pan_x = crate::bindings::c_data::sera_get_plot_pan_x();
        self.current_chart_kind = crate::bindings::c_data::sera_get_plot_chart_kind();
        
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("🔍 Zoom In").clicked() {
                    self.zoom *= 1.2;
                    crate::bindings::c_data::sera_set_plot_zoom(self.zoom);
                }
                if ui.button("🔍 Zoom Out").clicked() {
                    self.zoom /= 1.2;
                    crate::bindings::c_data::sera_set_plot_zoom(self.zoom);
                }
                if ui.button("⬜ Fit All").clicked() {
                    self.zoom = 1.0;
                    self.pan_x = 0.0;
                    crate::bindings::c_data::sera_set_plot_zoom(1.0);
                    crate::bindings::c_data::sera_set_plot_pan_x(0.0);
                }
                if ui.button("📋 Elements").clicked() {
                    self.show_list = !self.show_list;
                }
                
                if self.show_variant_selector && self.variants.len() > 1 {
                    if ui.button("📊 Transform").clicked() {
                        self.show_transform_menu = !self.show_transform_menu;
                    }
                }
                
                if ui.button(if self.orientation { "📊 Vertical" } else { "📈 Horizontal" }).clicked() {
                    self.orientation = !self.orientation;
                    crate::bindings::c_data::sera_set_plot_orientation(self.orientation);
                }
                let sort_label = match self.sort_mode {
                    1 => "↑ Asc",
                    2 => "↓ Desc",
                    _ => "⊙ None",
                };
                if ui.button(sort_label).clicked() {
                    self.sort_mode = (self.sort_mode + 1) % 3;
                    crate::bindings::c_data::sera_set_plot_sort(self.sort_mode);
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

        if self.show_transform_menu {
            let mut should_close = false;
            let current_kind = self.current_chart_kind;
            let variants = self.variants.clone();
            
            egui::Window::new("Transform Chart")
                .open(&mut self.show_transform_menu)
                .show(ctx, |ui| {
                    ui.heading("Available transformations:");
                    for variant in &variants {
                        let is_current = variant.kind == current_kind;
                        let label = if is_current {
                            format!("✓ {}", variant.title)
                        } else {
                            variant.title.clone()
                        };
                        
                        if !is_current && ui.button(&label).clicked() {
                            self.current_chart_kind = variant.kind;
                            crate::bindings::c_data::sera_set_plot_chart_kind(variant.kind);
                            should_close = true;
                        } else if is_current {
                            ui.label(&label);
                        }
                    }
                });
            
            if should_close {
                self.show_transform_menu = false;
            }
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
                    self.render_chart(ctx, ui, &d);
                }
            } else {
                ui.label("Waiting for data...");
            }
            
            ctx.request_repaint();
        });
    }
}

impl ChartApp {
    fn render_chart(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData) {
        match self.current_chart_kind {
            1 => {
                if self.orientation {
                    self.render_scatter_vertical(ctx, ui, d);
                } else {
                    self.render_scatter_horizontal(ctx, ui, d);
                }
            }
            2 => {
                if self.orientation {
                    self.render_vertical_bars(ctx, ui, d);
                } else {
                    self.render_horizontal_bars(ctx, ui, d);
                }
            }
            _ => {
                if self.orientation {
                    self.render_vertical_bars(ctx, ui, d);
                } else {
                    self.render_horizontal_bars(ctx, ui, d);
                }
            }
        }
    }

    fn render_scatter_vertical(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData) {
        let (tooltip_bg_color, tooltip_text_color) = (d.tooltip_bg_color, d.tooltip_text_color);
        let max_val = d.values.iter().fold(0.0_f64, |a, &b| a.max(b));
        
        let plot_width = 1400.0_f32 * self.zoom;
        let plot_height = 600.0_f32 * self.zoom;
        let padding_left = 80.0_f32;
        let padding_bottom = 100.0_f32;
        let padding_top = 20.0_f32;
        let padding_right = 20.0_f32;
        
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let (response, mut painter) = ui.allocate_painter(
                    egui::Vec2::new(plot_width + padding_left + padding_right, plot_height + padding_top + padding_bottom),
                    egui::Sense::hover()
                );
                
                let plot_rect = egui::Rect::from_min_size(
                    response.rect.min + egui::Vec2::new(padding_left, padding_top),
                    egui::Vec2::new(plot_width, plot_height)
                );
                
                painter.line_segment(
                    [egui::pos2(plot_rect.left(), plot_rect.bottom()), egui::pos2(plot_rect.right(), plot_rect.bottom())],
                    egui::Stroke::new(1.5, egui::Color32::from_gray(200))
                );
                painter.line_segment(
                    [egui::pos2(plot_rect.left(), plot_rect.top()), egui::pos2(plot_rect.left(), plot_rect.bottom())],
                    egui::Stroke::new(1.5, egui::Color32::from_gray(200))
                );
                
                let font_size = if self.zoom < 0.8 { 9.0 } else if self.zoom > 1.5 { 12.0 } else { 11.0 };
                
                for i in 0..=5 {
                    let y = plot_rect.bottom() - (plot_height / 5.0) * i as f32;
                    let val = (max_val / 5.0) * i as f64;
                    painter.text(
                        egui::pos2(plot_rect.left() - 15.0, y),
                        egui::Align2::RIGHT_CENTER,
                        &format!("{:.1}", val),
                        egui::FontId::proportional(font_size),
                        egui::Color32::from_gray(100)
                    );
                    painter.line_segment(
                        [egui::pos2(plot_rect.left() - 5.0, y), egui::pos2(plot_rect.left(), y)],
                        egui::Stroke::new(0.8, egui::Color32::from_gray(180))
                    );
                }
                
                for (i, label) in d.labels.iter().enumerate() {
                    let x = plot_rect.left() + (plot_width / (d.labels.len() as f32 - 1.0).max(1.0)) * i as f32;
                    
                    painter.text(
                        egui::pos2(x, plot_rect.bottom() + 25.0),
                        egui::Align2::CENTER_TOP,
                        label,
                        egui::FontId::proportional(font_size * 0.85),
                        egui::Color32::from_gray(70)
                    );
                }
                
                for (idx, (x, y)) in d.labels.iter().zip(d.values.iter()).enumerate() {
                    if idx >= self.visible_elements.len() || !self.visible_elements[idx] {
                        continue;
                    }
                    
                    let norm_x = if d.labels.len() > 1 { idx as f64 / (d.labels.len() as f64 - 1.0) } else { 0.5 };
                    let norm_y = if max_val > 0.0 { y / max_val } else { 0.0 };
                    
                    let screen_x = plot_rect.left() + (plot_width as f64 * norm_x) as f32;
                    let screen_y = plot_rect.bottom() - (plot_height as f64 * norm_y) as f32;
                    
                    let point = egui::pos2(screen_x, screen_y);
                    let hue = (idx as f32 * 0.1) % 1.0;
                    let color = hsv_to_rgb(hue, 0.75, 0.85);
                    
                    let mouse_pos = ctx.input(|i| i.pointer.latest_pos().unwrap_or(egui::pos2(-10000.0, -10000.0)));
                    let is_hovered = response.hovered() && 
                        ((point.x - mouse_pos.x).abs() < 12.0 &&
                        (point.y - mouse_pos.y).abs() < 12.0);
                    
                    let point_color = if is_hovered { hsv_to_rgb(hue, 0.95, 1.0) } else { color };
                    let point_size = if is_hovered { 7.0 } else { 5.5 };
                    
                    painter.circle_filled(point, point_size, point_color);
                    
                    if is_hovered {
                        painter.circle_stroke(point, point_size + 2.0, egui::Stroke::new(1.5, egui::Color32::WHITE));
                    }
                    
                    if is_hovered {
                        self.hovered_idx = Some(idx);
                        
                        let mut tooltip_lines = vec![x.clone()];
                        if idx < d.hover_data.len() {
                            for (key, val) in d.hover_data[idx].iter() {
                                if key != "image" {
                                    tooltip_lines.push(format!("{}: {}", key, val));
                                }
                            }
                        }
                        
                        let tooltip_width = 200.0 * self.zoom;
                        let tooltip_height = (tooltip_lines.len() as f32 * 22.0) * self.zoom;
                        let tooltip_x = if screen_x + 120.0 > plot_rect.right() { screen_x - tooltip_width - 15.0 } else { screen_x + 15.0 };
                        let tooltip_y = if screen_y - tooltip_height < plot_rect.top() { screen_y + 15.0 } else { screen_y - tooltip_height };
                        
                        let tooltip_rect = egui::Rect::from_min_size(
                            egui::pos2(tooltip_x, tooltip_y),
                            egui::vec2(tooltip_width, tooltip_height)
                        );
                        
                        painter.rect_filled(tooltip_rect, 6.0, egui::Color32::from_rgba_unmultiplied(
                            tooltip_bg_color.0, tooltip_bg_color.1, tooltip_bg_color.2, tooltip_bg_color.3
                        ));
                        painter.rect_stroke(tooltip_rect, 1.5, egui::Stroke::new(1.0, egui::Color32::WHITE));
                        
                        for (line_idx, line) in tooltip_lines.iter().enumerate() {
                            painter.text(
                                egui::pos2(tooltip_x + 10.0, tooltip_y + 6.0 + (line_idx as f32 * 22.0)),
                                egui::Align2::LEFT_TOP,
                                line,
                                egui::FontId::proportional(font_size),
                                egui::Color32::from_rgb(
                                    tooltip_text_color.0, tooltip_text_color.1, tooltip_text_color.2
                                )
                            );
                        }
                    }
                }
            });
    }

    fn render_scatter_horizontal(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData) {
        let (tooltip_bg_color, tooltip_text_color) = (d.tooltip_bg_color, d.tooltip_text_color);
        let max_val = d.values.iter().fold(0.0_f64, |a, &b| a.max(b));
        
        let plot_width = 600.0_f32 * self.zoom;
        let plot_height = 1400.0_f32 * self.zoom;
        let padding_left = 80.0_f32;
        let padding_bottom = 80.0_f32;
        let padding_top = 20.0_f32;
        let padding_right = 20.0_f32;
        
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let (response, mut painter) = ui.allocate_painter(
                    egui::Vec2::new(plot_width + padding_left + padding_right, plot_height + padding_top + padding_bottom),
                    egui::Sense::hover()
                );
                
                let plot_rect = egui::Rect::from_min_size(
                    response.rect.min + egui::Vec2::new(padding_left, padding_top),
                    egui::Vec2::new(plot_width, plot_height)
                );
                
                painter.line_segment(
                    [egui::pos2(plot_rect.left(), plot_rect.bottom()), egui::pos2(plot_rect.right(), plot_rect.bottom())],
                    egui::Stroke::new(1.5, egui::Color32::from_gray(200))
                );
                painter.line_segment(
                    [egui::pos2(plot_rect.left(), plot_rect.top()), egui::pos2(plot_rect.left(), plot_rect.bottom())],
                    egui::Stroke::new(1.5, egui::Color32::from_gray(200))
                );
                
                let font_size = if self.zoom < 0.8 { 9.0 } else if self.zoom > 1.5 { 12.0 } else { 11.0 };
                
                for i in 0..=5 {
                    let x = plot_rect.left() + (plot_width / 5.0) * i as f32;
                    let val = (max_val / 5.0) * i as f64;
                    painter.text(
                        egui::pos2(x, plot_rect.bottom() + 20.0),
                        egui::Align2::CENTER_TOP,
                        &format!("{:.1}", val),
                        egui::FontId::proportional(font_size),
                        egui::Color32::from_gray(100)
                    );
                    painter.line_segment(
                        [egui::pos2(x, plot_rect.bottom()), egui::pos2(x, plot_rect.bottom() + 5.0)],
                        egui::Stroke::new(0.8, egui::Color32::from_gray(180))
                    );
                }
                
                for (i, label) in d.labels.iter().enumerate() {
                    let y = plot_rect.top() + (plot_height / (d.labels.len() as f32 - 1.0).max(1.0)) * i as f32;
                    
                    painter.text(
                        egui::pos2(plot_rect.left() - 15.0, y),
                        egui::Align2::RIGHT_CENTER,
                        label,
                        egui::FontId::proportional(font_size * 0.85),
                        egui::Color32::from_gray(70)
                    );
                }
                
                for (idx, (x, y)) in d.labels.iter().zip(d.values.iter()).enumerate() {
                    if idx >= self.visible_elements.len() || !self.visible_elements[idx] {
                        continue;
                    }
                    
                    let norm_x = if max_val > 0.0 { y / max_val } else { 0.0 };
                    let norm_y = if d.labels.len() > 1 { idx as f64 / (d.labels.len() as f64 - 1.0) } else { 0.5 };
                    
                    let screen_x = plot_rect.left() + (plot_width as f64 * norm_x) as f32;
                    let screen_y = plot_rect.top() + (plot_height as f64 * norm_y) as f32;
                    
                    let point = egui::pos2(screen_x, screen_y);
                    let hue = (idx as f32 * 0.1) % 1.0;
                    let color = hsv_to_rgb(hue, 0.75, 0.85);
                    
                    let mouse_pos = ctx.input(|i| i.pointer.latest_pos().unwrap_or(egui::pos2(-10000.0, -10000.0)));
                    let is_hovered = response.hovered() && 
                        ((point.x - mouse_pos.x).abs() < 12.0 &&
                        (point.y - mouse_pos.y).abs() < 12.0);
                    
                    let point_color = if is_hovered { hsv_to_rgb(hue, 0.95, 1.0) } else { color };
                    let point_size = if is_hovered { 7.0 } else { 5.5 };
                    
                    painter.circle_filled(point, point_size, point_color);
                    
                    if is_hovered {
                        painter.circle_stroke(point, point_size + 2.0, egui::Stroke::new(1.5, egui::Color32::WHITE));
                    }
                    
                    if is_hovered {
                        self.hovered_idx = Some(idx);
                        
                        let mut tooltip_lines = vec![x.clone()];
                        if idx < d.hover_data.len() {
                            for (key, val) in d.hover_data[idx].iter() {
                                if key != "image" {
                                    tooltip_lines.push(format!("{}: {}", key, val));
                                }
                            }
                        }
                        
                        let tooltip_width = 200.0 * self.zoom;
                        let tooltip_height = (tooltip_lines.len() as f32 * 22.0) * self.zoom;
                        let tooltip_x = if screen_x - tooltip_width < plot_rect.left() { screen_x + 15.0 } else { screen_x - tooltip_width - 15.0 };
                        let tooltip_y = if screen_y - tooltip_height < plot_rect.top() { screen_y + 15.0 } else { screen_y - tooltip_height };
                        
                        let tooltip_rect = egui::Rect::from_min_size(
                            egui::pos2(tooltip_x, tooltip_y),
                            egui::vec2(tooltip_width, tooltip_height)
                        );
                        
                        painter.rect_filled(tooltip_rect, 6.0, egui::Color32::from_rgba_unmultiplied(
                            tooltip_bg_color.0, tooltip_bg_color.1, tooltip_bg_color.2, tooltip_bg_color.3
                        ));
                        painter.rect_stroke(tooltip_rect, 1.5, egui::Stroke::new(1.0, egui::Color32::WHITE));
                        
                        for (line_idx, line) in tooltip_lines.iter().enumerate() {
                            painter.text(
                                egui::pos2(tooltip_x + 10.0, tooltip_y + 6.0 + (line_idx as f32 * 22.0)),
                                egui::Align2::LEFT_TOP,
                                line,
                                egui::FontId::proportional(font_size),
                                egui::Color32::from_rgb(
                                    tooltip_text_color.0, tooltip_text_color.1, tooltip_text_color.2
                                )
                            );
                        }
                    }
                }
            });
    }

    fn render_vertical_bars(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData) {
        let (tooltip_bg_color, tooltip_text_color) = (d.tooltip_bg_color, d.tooltip_text_color);
        let max_val = d.values.iter().fold(0.0_f64, |a, &b| a.max(b));
        let bar_height = 300.0_f32;
        
        let visible_count = d.labels.iter().enumerate()
            .filter(|(idx, _)| idx < &self.visible_elements.len() && self.visible_elements[*idx])
            .count() as f32;
        
        let available_width = 1200.0_f32 - 30.0;
        let item_width = if visible_count > 0.0 {
            available_width / visible_count
        } else {
            50.0
        };
        
        let font_size = if self.zoom < 0.8 { 10.0 } else if self.zoom > 1.5 { 13.0 } else { 12.0 };
        let font_id = egui::FontId::proportional(font_size);
        
        let mut max_content_width: f32 = 100.0;
        for hover in &d.hover_data {
            for (key, val) in hover.iter() {
                if key != "image" {
                    let display = format!("{}: {}", key, val);
                    let galley = ctx.fonts(|f| f.layout_no_wrap(display, font_id.clone(), egui::Color32::WHITE));
                    max_content_width = max_content_width.max(galley.rect.width());
                }
            }
        }
        
        egui::ScrollArea::horizontal()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(1.0);
                    for (idx, (label, value)) in d.labels.iter().zip(d.values.iter()).enumerate() {
                        if idx >= self.visible_elements.len() || !self.visible_elements[idx] {
                            continue;
                        }
                        
                        ui.vertical(|ui| {
                            ui.set_width(item_width);
                            
                            let h = if max_val > 0.0 { (*value / max_val) * (bar_height as f64) } else { 0.0 } as f32;
                            let scaled_h = h * self.zoom;
                            
                            let bar_width = (item_width * 0.55).max(3.0);
                            
                            let field_count = if idx < d.hover_data.len() { d.hover_data[idx].iter().filter(|(k, _)| k.as_str() != "image").count() as f32 } else { 0.0 };
                            let has_image = if idx < d.hover_data.len() { d.hover_data[idx].contains_key("image") } else { false };
                            
                            let actual_image_height = if has_image { 120.0 * self.zoom } else { 0.0 };
                            let field_height = 32.0 * self.zoom;
                            let padding_internal = 30.0 * self.zoom;
                            let actual_content_height = actual_image_height + (field_count * field_height) + padding_internal;
                            let painter_height = (bar_height + actual_content_height + 100.0) * self.zoom;
                            
                            let (response, mut painter) = ui.allocate_painter(
                                egui::Vec2::new(item_width, painter_height),
                                egui::Sense::hover()
                            );
                            
                            let hue = (idx as f32 * 0.1) % 1.0;
                            let color = hsv_to_rgb(hue, 0.8, 0.9);
                            let hover_color = hsv_to_rgb(hue, 1.0, 1.0);
                            
                            let is_hovered = response.hovered();
                            if is_hovered {
                                self.hovered_idx = Some(idx);
                            }
                            
                            let bar_color = if is_hovered { hover_color } else { color };
                            
                            let bar_rect = egui::Rect::from_min_size(
                                response.rect.center_bottom() - egui::Vec2::new(bar_width * self.zoom / 2.0, scaled_h),
                                egui::Vec2::new(bar_width * self.zoom, scaled_h)
                            );
                            
                            painter.rect_filled(bar_rect, 4.0, bar_color);
                            
                            if is_hovered {
                                painter.rect_stroke(bar_rect, 2.0, egui::Stroke::new(2.0, egui::Color32::WHITE));
                            }
                            
                            let label_font = if self.zoom < 0.8 { 8.0 } else { 9.0 };
                            painter.text(
                                response.rect.center_bottom() + egui::Vec2::new(0.0, 12.0 * self.zoom),
                                egui::Align2::CENTER_TOP,
                                label,
                                egui::FontId::proportional(label_font),
                                egui::Color32::DARK_GRAY
                            );
                            
                            if is_hovered {
                                let actual_image_height = if has_image { 120.0 * self.zoom } else { 0.0 };
                                let field_height = 32.0 * self.zoom;
                                let padding_internal = 30.0 * self.zoom;
                                let actual_content_height = actual_image_height + (field_count * field_height) + padding_internal;
                                
                                let mut max_text_width: f32 = 120.0;
                                if idx < d.hover_data.len() {
                                    for (key, val) in d.hover_data[idx].iter() {
                                        if key != "image" {
                                            let display = format!("{}: {}", key, val);
                                            let galley = ctx.fonts(|f| f.layout_no_wrap(display, egui::FontId::proportional(font_size), egui::Color32::WHITE));
                                            max_text_width = max_text_width.max(galley.rect.width());
                                        }
                                    }
                                }
                                
                                let tooltip_w = (max_text_width * 1.5 + 60.0 * self.zoom).max(140.0 * self.zoom);
                                let tooltip_h = actual_content_height.max(180.0 * self.zoom);
                                let tooltip_y = bar_rect.top() - 20.0 * self.zoom - tooltip_h;
                                
                                let tooltip_rect = egui::Rect::from_min_size(
                                    egui::pos2(bar_rect.center().x - tooltip_w / 2.0, tooltip_y),
                                    egui::vec2(tooltip_w, tooltip_h)
                                );
                                painter.set_clip_rect(tooltip_rect);
                                
                                let mut field_offset = if has_image { 130.0 * self.zoom } else { 25.0 * self.zoom };
                                
                                if has_image && idx < d.hover_data.len() {
                                    if let Some(img_path) = d.hover_data[idx].get("image") {
                                        if let Some(color_img) = self.image_loader.load_image(img_path) {
                                            let img_w = 120.0 * self.zoom;
                                            let img_h = 120.0 * self.zoom;
                                            let img_x = bar_rect.center().x - img_w / 2.0;
                                            let img_y = tooltip_y + 15.0 * self.zoom;
                                            
                                            let texture = ctx.load_texture("hover_img", color_img, Default::default());
                                            painter.image(
                                                texture.id(),
                                                egui::Rect::from_min_size(egui::pos2(img_x, img_y), egui::vec2(img_w, img_h)),
                                                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                                                egui::Color32::WHITE
                                            );
                                        } else {
                                            painter.text(
                                                egui::pos2(bar_rect.center().x, tooltip_y + 50.0 * self.zoom),
                                                egui::Align2::CENTER_TOP,
                                                "🖼",
                                                egui::FontId::proportional(font_size * 3.0),
                                                egui::Color32::WHITE
                                            );
                                        }
                                    }
                                }
                                
                                if idx < d.hover_data.len() {
                                    let (txt_r, txt_g, txt_b, txt_a) = tooltip_text_color;
                                    let text_color = egui::Color32::from_rgba_unmultiplied(txt_r, txt_g, txt_b, txt_a);
                                    for (key, val) in d.hover_data[idx].iter() {
                                        if key != "image" {
                                            let display = format!("{}: {}", key, val);
                                            painter.text(
                                                egui::pos2(bar_rect.center().x, tooltip_y + field_offset),
                                                egui::Align2::CENTER_TOP,
                                                display,
                                                egui::FontId::proportional(font_size),
                                                text_color
                                            );
                                            field_offset += field_height + 8.0 * self.zoom;
                                        }
                                    }
                                }
                                
                                if idx < d.hover_data.len() {
                                    let (txt_r, txt_g, txt_b, txt_a) = tooltip_text_color;
                                    let text_color = egui::Color32::from_rgba_unmultiplied(txt_r, txt_g, txt_b, txt_a);
                                    painter.text(
                                        egui::pos2(bar_rect.center().x, tooltip_y + tooltip_h - 42.0),
                                        egui::Align2::CENTER_TOP,
                                        label,
                                        egui::FontId::proportional(font_size * 1.2),
                                        text_color
                                    );
                                    
                                    let value_text = format!("{}", *value as i32);
                                    painter.text(
                                        egui::pos2(bar_rect.center().x, tooltip_y + tooltip_h - 15.0),
                                        egui::Align2::CENTER_TOP,
                                        value_text,
                                        egui::FontId::proportional(font_size * 1.4),
                                        text_color
                                    );
                                }
                            }
                        });
                        ui.add_space(0.0);
                    }
                    ui.add_space(1.0);
                });
            });
    }

    fn render_horizontal_bars(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, d: &ChartData) {
        let (tooltip_bg_color, tooltip_text_color) = (d.tooltip_bg_color, d.tooltip_text_color);
        let max_val = d.values.iter().fold(0.0_f64, |a, &b| a.max(b));
        let bar_max_width = 350.0_f32;
        
        let visible_count = d.labels.iter().enumerate()
            .filter(|(idx, _)| idx < &self.visible_elements.len() && self.visible_elements[*idx])
            .count() as f32;
        
        let available_height = 500.0_f32;
        let item_height = if visible_count > 0.0 {
            (available_height / visible_count).max(45.0)
        } else {
            50.0
        };
        
        let font_size = if self.zoom < 0.8 { 10.0 } else if self.zoom > 1.5 { 13.0 } else { 12.0 };
        
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for (idx, (label, value)) in d.labels.iter().zip(d.values.iter()).enumerate() {
                        if idx >= self.visible_elements.len() || !self.visible_elements[idx] {
                            continue;
                        }
                        
                        let w = if max_val > 0.0 { (*value / max_val) * (bar_max_width as f64) } else { 0.0 } as f32;
                        let scaled_w = w * self.zoom;
                        let bar_h = (item_height * 0.5).max(4.0);
                        
                        let label_width = 110.0;
                        let bar_area_width = bar_max_width * self.zoom + 150.0;
                        
                        let (rect, _response) = ui.allocate_exact_size(
                            egui::Vec2::new(label_width + bar_area_width, item_height),
                            egui::Sense::hover()
                        );
                        
                        let painter = ui.painter_at(rect);
                        
                        let hue = (idx as f32 * 0.1) % 1.0;
                        let color = hsv_to_rgb(hue, 0.8, 0.9);
                        let hover_color = hsv_to_rgb(hue, 1.0, 1.0);
                        
                        let is_hovered = rect.contains(ctx.input(|i| i.pointer.hover_pos()).unwrap_or_default());
                        if is_hovered {
                            self.hovered_idx = Some(idx);
                        }
                        
                        let bar_color = if is_hovered { hover_color } else { color };
                        
                        let label_rect = egui::Rect::from_min_size(
                            rect.left_top() + egui::Vec2::new(5.0, 0.0),
                            egui::Vec2::new(label_width - 10.0, item_height)
                        );
                        
                        painter.text(
                            label_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            label,
                            egui::FontId::proportional(font_size * 0.9),
                            egui::Color32::DARK_GRAY
                        );
                        
                        let bar_x = rect.left() + label_width + 10.0;
                        let bar_y = rect.center().y - bar_h * self.zoom / 2.0;
                        
                        let bar_rect = egui::Rect::from_min_size(
                            egui::pos2(bar_x, bar_y),
                            egui::vec2(scaled_w, bar_h * self.zoom)
                        );
                        
                        painter.rect_filled(bar_rect, 3.0, bar_color);
                        
                        if is_hovered {
                            painter.rect_stroke(bar_rect, 2.0, egui::Stroke::new(2.0, egui::Color32::WHITE));
                        }
                        
                        let value_text = format!("{}", *value as i32);
                        painter.text(
                            bar_rect.right_center() + egui::Vec2::new(8.0, 0.0),
                            egui::Align2::LEFT_CENTER,
                            &value_text,
                            egui::FontId::proportional(font_size),
                            egui::Color32::DARK_GRAY
                        );
                        
                        if is_hovered {
                            let tooltip_w = 200.0 * self.zoom;
                            let tooltip_h = 140.0 * self.zoom;
                            let tooltip_x = bar_rect.right() + 40.0;
                            let tooltip_y = bar_rect.center().y - tooltip_h / 2.0;
                            
                            let tooltip_rect = egui::Rect::from_min_size(
                                egui::pos2(tooltip_x.max(0.0), tooltip_y),
                                egui::vec2(tooltip_w, tooltip_h)
                            );
                            
                            let mut field_offset = 12.0 * self.zoom;
                            
                            if idx < d.hover_data.len() {
                                let (txt_r, txt_g, txt_b, txt_a) = tooltip_text_color;
                                let text_color = egui::Color32::from_rgba_unmultiplied(txt_r, txt_g, txt_b, txt_a);
                                
                                for (key, val) in d.hover_data[idx].iter() {
                                    if key != "image" {
                                        let display = format!("{}: {}", key, val);
                                        painter.text(
                                            egui::pos2(tooltip_x + 8.0, tooltip_y + field_offset),
                                            egui::Align2::LEFT_TOP,
                                            display,
                                            egui::FontId::proportional(font_size * 0.9),
                                            text_color
                                        );
                                        field_offset += 26.0;
                                    }
                                }
                            }
                        }
                    }
                });
            });
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> egui::Color32 {
    let c = v * s;
    let h_prime = (h * 6.0) % 6.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    
    let (r, g, b) = match h_prime as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    
    let m = v - c;
    egui::Color32::from_rgb(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
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

    let variants = {
        let chart_variants = crate::bindings::c_data::get_chart_variants_internal();
        chart_variants.iter().map(|(kind, title)| PlotVariant {
            kind: *kind,
            title: title.clone(),
        }).collect::<Vec<_>>()
    };

    let show_selector = variants.len() > 1;

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
        current_chart_kind: if !variants.is_empty() { variants[0].kind } else { 2 },
        variants,
        show_variant_selector: show_selector,
        show_transform_menu: false,
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
pub extern "C" fn sera_set_chart_orientation(vertical: bool) {
    crate::bindings::c_data::sera_set_plot_orientation(vertical);
}

#[no_mangle]
pub extern "C" fn sera_get_chart_orientation() -> bool {
    crate::bindings::c_data::sera_get_plot_orientation()
}

#[no_mangle]
pub extern "C" fn sera_set_chart_sort(mode: i32) {
    crate::bindings::c_data::sera_set_plot_sort(mode.clamp(0, 2));
}

#[no_mangle]
pub extern "C" fn sera_get_chart_sort() -> i32 {
    crate::bindings::c_data::sera_get_plot_sort()
}

#[no_mangle]
pub extern "C" fn sera_add_chart_variant(kind: u8, title: *const c_char) -> bool {
    if title.is_null() {
        return false;
    }
    crate::bindings::c_data::sera_add_plot_variant(kind, title);
    true
}

#[no_mangle]
pub extern "C" fn sera_set_current_chart_kind(kind: u8) {
    crate::bindings::c_data::sera_set_plot_chart_kind(kind);
}

#[no_mangle]
pub extern "C" fn sera_get_current_chart_kind() -> u8 {
    crate::bindings::c_data::sera_get_plot_chart_kind()
}

#[no_mangle]
pub extern "C" fn sera_enable_variant_selector(enable: bool) {
    crate::bindings::c_data::sera_set_plot_show_selector(enable);
}

#[no_mangle]
pub extern "C" fn sera_is_variant_selector_enabled() -> bool {
    crate::bindings::c_data::sera_get_plot_show_selector()
}

#[no_mangle]
pub extern "C" fn sera_show_with_variants(
    enable_variants: bool,
    default_kind: u8,
) -> bool {
    crate::bindings::c_data::sera_set_plot_show_selector(enable_variants);
    crate::bindings::c_data::sera_set_plot_chart_kind(default_kind);

    let variants = {
        let chart_variants = crate::bindings::c_data::get_chart_variants_internal();
        chart_variants.iter().map(|(kind, title)| PlotVariant {
            kind: *kind,
            title: title.clone(),
        }).collect::<Vec<_>>()
    };

    let app = ChartApp {
        data: Arc::new(Mutex::new(None)),
        hovered_idx: None,
        zoom: 1.0,
        pan_x: 0.0,
        visible_elements: vec![],
        show_list: false,
        image_loader: ImageLoader::new(),
        orientation: true,
        sort_mode: 0,
        current_chart_kind: default_kind,
        variants,
        show_variant_selector: enable_variants,
        show_transform_menu: false,
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


