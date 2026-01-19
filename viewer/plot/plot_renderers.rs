use std::collections::HashMap;
use egui::{Painter, Rect, Color32, Pos2};
use super::super::image_loader::ImageLoader;
use super::plot_template::{PlotConfig, hsv_to_rgb};

pub trait AxisRenderStrategy {
    fn render_axis(
        &self,
        painter: &Painter,
        plot_rect: Rect,
        max_val: f64,
        font_size: f32,
        _config: &PlotConfig,
    );
}

pub struct DefaultAxisRenderer {
    pub gridline_count: usize,
    pub show_gridlines: bool,
}

impl DefaultAxisRenderer {
    pub fn new() -> Self {
        Self {
            gridline_count: 5,
            show_gridlines: true,
        }
    }
}

impl AxisRenderStrategy for DefaultAxisRenderer {
    fn render_axis(
        &self,
        painter: &Painter,
        plot_rect: Rect,
        max_val: f64,
        font_size: f32,
        _config: &PlotConfig,
    ) {
        if self.show_gridlines {
            for i in 0..=self.gridline_count {
                let y = plot_rect.bottom() - (plot_rect.height() / self.gridline_count as f32) * i as f32;
                let val = (max_val / self.gridline_count as f64) * i as f64;
                
                painter.text(
                    Pos2::new(plot_rect.left() - 15.0, y),
                    egui::Align2::RIGHT_CENTER,
                    &format!("{:.1}", val),
                    egui::FontId::proportional(font_size),
                    Color32::from_gray(100),
                );
                
                painter.line_segment(
                    [Pos2::new(plot_rect.left() - 5.0, y), Pos2::new(plot_rect.left(), y)],
                    egui::Stroke::new(0.8, Color32::from_gray(180)),
                );
            }
        }
    }
}

pub struct AxisRenderer {
    strategy: Box<dyn AxisRenderStrategy>,
}

impl AxisRenderer {
    pub fn new() -> Self {
        Self {
            strategy: Box::new(DefaultAxisRenderer::new()),
        }
    }

    pub fn with_strategy<S: AxisRenderStrategy + 'static>(strategy: S) -> Self {
        Self {
            strategy: Box::new(strategy),
        }
    }

    pub fn render_y_axis_vertical(
        &self,
        painter: &Painter,
        plot_rect: Rect,
        max_val: f64,
        font_size: f32,
        config: &PlotConfig,
    ) {
        self.strategy.render_axis(painter, plot_rect, max_val, font_size, config);
    }

    pub fn render_x_axis_horizontal(
        &self,
        painter: &Painter,
        plot_rect: Rect,
        max_val: f64,
        font_size: f32,
        _config: &PlotConfig,
    ) {
        for i in 0..=5 {
            let x = plot_rect.left() + (plot_rect.width() / 5.0) * i as f32;
            let val = (max_val / 5.0) * i as f64;
            
            painter.text(
                Pos2::new(x, plot_rect.bottom() + 20.0),
                egui::Align2::CENTER_TOP,
                &format!("{:.1}", val),
                egui::FontId::proportional(font_size),
                Color32::from_gray(100),
            );
            
            painter.line_segment(
                [Pos2::new(x, plot_rect.bottom()), Pos2::new(x, plot_rect.bottom() + 5.0)],
                egui::Stroke::new(0.8, Color32::from_gray(180)),
            );
        }
    }
}

pub struct LabelRenderer {
    pub angle: f32,
}

impl LabelRenderer {
    pub fn new() -> Self {
        Self { angle: 0.0 }
    }

    pub fn render_x_labels_horizontal(
        &self,
        painter: &Painter,
        labels: &[String],
        plot_rect: Rect,
        font_size: f32,
    ) {
        for (i, label) in labels.iter().enumerate() {
            let x = plot_rect.left() + (plot_rect.width() / (labels.len().saturating_sub(1).max(1) as f32)) * i as f32;
            
            painter.text(
                Pos2::new(x, plot_rect.bottom() + 25.0),
                egui::Align2::CENTER_TOP,
                label,
                egui::FontId::proportional(font_size * 0.85),
                Color32::from_gray(70),
            );
        }
    }

    pub fn render_y_labels_horizontal(
        &self,
        painter: &Painter,
        labels: &[String],
        plot_rect: Rect,
        font_size: f32,
    ) {
        for (i, label) in labels.iter().enumerate() {
            let y = plot_rect.top() + (plot_rect.height() / (labels.len().saturating_sub(1).max(1) as f32)) * i as f32;
            
            painter.text(
                Pos2::new(plot_rect.left() - 15.0, y),
                egui::Align2::RIGHT_CENTER,
                label,
                egui::FontId::proportional(font_size * 0.85),
                Color32::from_gray(70),
            );
        }
    }
}

pub struct TooltipRenderer;

impl TooltipRenderer {
    pub fn render(
        ctx: &egui::Context,
        painter: &Painter,
        tooltip_x: f32,
        tooltip_y: f32,
        label: &str,
        hover_data: &HashMap<String, String>,
        bg_color: (u8, u8, u8, u8),
        text_color: (u8, u8, u8, u8),
        font_size: f32,
        zoom: f32,
        image_loader: &ImageLoader,
    ) {
        let field_count = hover_data.iter().filter(|(k, _)| k.as_str() != "image").count() as f32;
        let has_image = hover_data.contains_key("image");
        let tooltip_width = 200.0 * zoom;
        let base_height = if has_image { 120.0 } else { 85.0 };
        let tooltip_height = base_height + (field_count * 35.0) + 65.0;
        
        let tooltip_rect = Rect::from_min_size(
            Pos2::new(tooltip_x, tooltip_y),
            egui::vec2(tooltip_width, tooltip_height * zoom),
        );
        
        painter.rect_filled(
            tooltip_rect,
            6.0,
            Color32::from_rgba_unmultiplied(bg_color.0, bg_color.1, bg_color.2, bg_color.3),
        );
        painter.rect_stroke(tooltip_rect, 1.5, egui::Stroke::new(1.0, Color32::WHITE));
        
        let mut hover_offset = 15.0;
        
        if has_image {
            if let Some(img_url) = hover_data.get("image") {
                if let Some(color_img) = image_loader.load_image(img_url) {
                    let texture_handle = ctx.load_texture("image", color_img, egui::TextureOptions::default());
                    let img_rect = Rect::from_min_size(
                        Pos2::new(tooltip_x + tooltip_width / 2.0 - 45.0, tooltip_y + 15.0),
                        egui::Vec2::new(90.0, 90.0),
                    );
                    painter.image(
                        texture_handle.id(),
                        img_rect,
                        Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }
            }
            hover_offset = 110.0;
        }
        
        for (key, val) in hover_data.iter() {
            if key != "image" {
                let text_val = if val.len() > 55 {
                    format!("{}...", &val[..52])
                } else {
                    val.clone()
                };
                painter.text(
                    Pos2::new(tooltip_x + 10.0, tooltip_y + hover_offset),
                    egui::Align2::LEFT_TOP,
                    &format!("{}: {}", key, text_val),
                    egui::FontId::proportional(font_size * 0.9),
                    Color32::from_rgb(text_color.0, text_color.1, text_color.2),
                );
                hover_offset += 35.0 * zoom;
            }
        }
        
        painter.text(
            Pos2::new(tooltip_x + tooltip_width / 2.0, tooltip_y + tooltip_height * zoom - 38.0),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(font_size),
            Color32::from_rgb(text_color.0, text_color.1, text_color.2),
        );
    }
}

pub struct PointRenderer {
    pub radius: f32,
    pub hover_radius: f32,
}

impl PointRenderer {
    pub fn new(radius: f32, hover_radius: f32) -> Self {
        Self { radius, hover_radius }
    }

    pub fn render_point(
        &self,
        painter: &Painter,
        point: Pos2,
        idx: usize,
        is_hovered: bool,
    ) {
        let hue = (idx as f32 * 0.1) % 1.0;
        let color = if is_hovered { 
            hsv_to_rgb(hue, 0.95, 1.0) 
        } else { 
            hsv_to_rgb(hue, 0.75, 0.85) 
        };
        
        let point_size = if is_hovered { self.hover_radius } else { self.radius };
        
        painter.circle_filled(point, point_size, color);
        
        if is_hovered {
            painter.circle_stroke(point, point_size + 2.0, egui::Stroke::new(1.5, Color32::WHITE));
        }
    }
}

pub struct BarRenderer {
    pub width_factor: f32,
}

impl BarRenderer {
    pub fn new(width_factor: f32) -> Self {
        Self { width_factor }
    }

    pub fn render_bar(
        &self,
        painter: &Painter,
        bar_rect: Rect,
        idx: usize,
        is_hovered: bool,
    ) {
        let hue = (idx as f32 * 0.1) % 1.0;
        let color = if is_hovered { 
            hsv_to_rgb(hue, 1.0, 1.0) 
        } else { 
            hsv_to_rgb(hue, 0.8, 0.9) 
        };
        
        painter.rect_filled(bar_rect, 4.0, color);
        
        if is_hovered {
            painter.rect_stroke(bar_rect, 2.0, egui::Stroke::new(2.0, Color32::WHITE));
        }
    }
}
