use std::sync::{Arc, Mutex};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::collections::HashMap;

struct ChartData {
    labels: Vec<String>,
    values: Vec<f64>,
    title: String,
    hover_data: Vec<HashMap<String, String>>,
}

struct ChartApp {
    data: Arc<Mutex<Option<ChartData>>>,
    hovered_idx: Option<usize>,
    zoom: f32,
    pan_x: f32,
    visible_elements: Vec<bool>,
    show_list: bool,
}

impl eframe::App for ChartApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            if let Some(d) = data.as_ref() {
                ui.vertical_centered(|ui| {
                    ui.heading(&d.title);
                });
                ui.separator();
                
                if d.values.is_empty() {
                    ui.label("No data");
                } else {
                    let max_val = d.values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    let bar_height = 300.0_f32;
                    let bar_width = 40.0_f32;
                    let item_width = (bar_width + 25.0) * self.zoom;
                    
                    egui::ScrollArea::horizontal()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(10.0);
                                for (idx, (label, value)) in d.labels.iter().zip(d.values.iter()).enumerate() {
                                    if idx >= self.visible_elements.len() || !self.visible_elements[idx] {
                                        continue;
                                    }
                                    
                                    ui.vertical(|ui| {
                                        ui.set_width(item_width);
                                        
                                        let h = if max_val > 0.0 { (*value / max_val) * bar_height as f64 } else { 0.0 } as f32;
                                        let scaled_h = h * self.zoom;
                                        
                                        let (response, painter) = ui.allocate_painter(
                                            egui::Vec2::new(item_width, (bar_height + 140.0) * self.zoom),
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
                                            response.rect.left_bottom() - egui::Vec2::new(0.0, scaled_h),
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
                                            let hover_count = if idx < d.hover_data.len() { d.hover_data[idx].len() } else { 0 };
                                            let base_height = 50.0;
                                            let field_height = 18.0;
                                            let tooltip_h = (base_height + (hover_count as f32 * field_height)) * self.zoom;
                                            let tooltip_w = if hover_count > 0 { 200.0 } else { 130.0 } * self.zoom;
                                            let tooltip_y = bar_rect.top() - 10.0 * self.zoom - tooltip_h;
                                            
                                            painter.rect_filled(
                                                egui::Rect::from_min_size(
                                                    egui::pos2(bar_rect.center().x - tooltip_w / 2.0, tooltip_y),
                                                    egui::vec2(tooltip_w, tooltip_h)
                                                ),
                                                4.0,
                                                egui::Color32::from_rgba_unmultiplied(44, 62, 80, 240)
                                            );
                                            
                                            let font_size = if self.zoom < 0.8 { 8.0 } else if self.zoom > 1.5 { 11.0 } else { 9.0 };
                                            let line_height = font_size * 1.3;
                                            let mut field_offset = 0.0;
                                            
                                            if idx < d.hover_data.len() {
                                                for (key, val) in d.hover_data[idx].iter() {
                                                    if key == "image" {
                                                        painter.text(
                                                            egui::pos2(bar_rect.center().x - tooltip_w / 2.0 + 5.0, tooltip_y + 5.0 + field_offset),
                                                            egui::Align2::LEFT_TOP,
                                                            "[img]",
                                                            egui::FontId::proportional(font_size * 0.85),
                                                            egui::Color32::LIGHT_GRAY
                                                        );
                                                    } else {
                                                        let max_len = if self.zoom > 1.2 { 22 } else { 18 };
                                                        let val_short = if val.len() > max_len {
                                                            format!("{}...", &val[..max_len.min(val.len() - 1)])
                                                        } else {
                                                            val.clone()
                                                        };
                                                        let display = format!("{}: {}", key, val_short);
                                                        painter.text(
                                                            egui::pos2(bar_rect.center().x, tooltip_y + 8.0 + field_offset),
                                                            egui::Align2::CENTER_TOP,
                                                            display,
                                                            egui::FontId::proportional(font_size * 0.8),
                                                            egui::Color32::LIGHT_GRAY
                                                        );
                                                    }
                                                    field_offset += field_height;
                                                }
                                            }
                                            
                                            painter.text(
                                                egui::pos2(bar_rect.center().x, tooltip_y + 8.0 + field_offset + line_height * 0.5),
                                                egui::Align2::CENTER_TOP,
                                                label,
                                                egui::FontId::proportional(font_size * 1.1),
                                                egui::Color32::WHITE
                                            );
                                            
                                            let value_text = format!("{}", *value as i32);
                                            painter.text(
                                                egui::pos2(bar_rect.center().x, tooltip_y + 8.0 + field_offset + line_height * 1.8),
                                                egui::Align2::CENTER_TOP,
                                                value_text,
                                                egui::FontId::proportional(font_size * 1.2),
                                                egui::Color32::from_rgb(26, 188, 156)
                                            );
                                        }
                                    });
                                    ui.add_space(5.0);
                                }
                                ui.add_space(10.0);
                            });
                        });
                }
            } else {
                ui.label("Waiting for data...");
            }
            
            ctx.request_repaint();
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

static _CHART_DATA: Mutex<Option<ChartData>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn sera_show_chart_data(
    labels: *const *const c_char,
    values: *const f64,
    count: u32,
    title: *const c_char,
) -> bool {
    sera_show_chart_data_with_hover(labels, values, std::ptr::null(), std::ptr::null(), count, title)
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
    };

    let app = ChartApp {
        data: Arc::new(Mutex::new(Some(data))),
        hovered_idx: None,
        zoom: 1.0,
        pan_x: 0.0,
        visible_elements: vec![true; num_elements],
        show_list: false,
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
