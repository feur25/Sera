use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use egui::Painter;

pub type PropertyMap = Arc<Mutex<HashMap<String, String>>>;
pub type VisibilitySet = Arc<Mutex<HashSet<usize>>>;

#[derive(Clone, Debug)]
pub struct PlotConfig {
    pub width: f32,
    pub height: f32,
    pub padding_left: f32,
    pub padding_right: f32,
    pub padding_top: f32,
    pub padding_bottom: f32,
    pub font_size: f32,
    pub zoom: f32,
    pub custom_props: Arc<Mutex<HashMap<String, String>>>,
}

impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            width: 1400.0,
            height: 600.0,
            padding_left: 80.0,
            padding_right: 20.0,
            padding_top: 20.0,
            padding_bottom: 100.0,
            font_size: 11.0,
            zoom: 1.0,
            custom_props: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub struct PlotConfigBuilder {
    config: PlotConfig,
}

impl PlotConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: PlotConfig::default(),
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.config.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.config.height = height;
        self
    }

    pub fn padding(mut self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        self.config.padding_left = left;
        self.config.padding_right = right;
        self.config.padding_top = top;
        self.config.padding_bottom = bottom;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.config.font_size = size;
        self
    }

    pub fn zoom(mut self, zoom: f32) -> Self {
        self.config.zoom = zoom;
        self
    }

    pub fn with_property(self, key: &str, value: &str) -> Self {
        if let Ok(mut props) = self.config.custom_props.lock() {
            props.insert(key.to_string(), value.to_string());
        }
        self
    }

    pub fn build(self) -> PlotConfig {
        self.config
    }
}

#[derive(Clone, Debug)]
pub struct TooltipStyle {
    pub bg_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub width: f32,
    pub height: f32,
}

impl Default for TooltipStyle {
    fn default() -> Self {
        Self {
            bg_color: (44, 62, 80, 245),
            text_color: (255, 255, 255, 255),
            width: 200.0,
            height: 100.0,
        }
    }
}

pub struct TooltipStyleBuilder {
    style: TooltipStyle,
}

impl TooltipStyleBuilder {
    pub fn new() -> Self {
        Self {
            style: TooltipStyle::default(),
        }
    }

    pub fn bg_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.style.bg_color = (r, g, b, a);
        self
    }

    pub fn text_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.style.text_color = (r, g, b, a);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.style.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.style.height = height;
        self
    }

    pub fn build(self) -> TooltipStyle {
        self.style
    }
}

pub struct RenderContext<'a> {
    pub painter: &'a Painter,
    pub ctx: &'a egui::Context,
    pub config: PlotConfig,
    pub tooltip_style: TooltipStyle,
}

pub struct PlotData {
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub title: String,
    pub hover_data: Vec<HashMap<String, String>>,
}

pub trait PlotRenderer {
    fn render(&mut self, ctx: &RenderContext, data: &PlotData, plot_rect: egui::Rect);
}

#[derive(Clone, Copy)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

pub struct AxisConfig {
    pub orientation: Orientation,
    pub show_gridlines: bool,
    pub gridline_count: usize,
    pub label_angle: f32,
}

impl Default for AxisConfig {
    fn default() -> Self {
        Self {
            orientation: Orientation::Vertical,
            show_gridlines: true,
            gridline_count: 5,
            label_angle: 0.0,
        }
    }
}

pub struct PointStyle {
    pub radius: f32,
    pub stroke_width: f32,
    pub hover_radius: f32,
}

impl Default for PointStyle {
    fn default() -> Self {
        Self {
            radius: 5.5,
            stroke_width: 1.5,
            hover_radius: 7.0,
        }
    }
}

pub struct PointStyleBuilder {
    style: PointStyle,
}

impl PointStyleBuilder {
    pub fn new() -> Self {
        Self {
            style: PointStyle::default(),
        }
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.style.radius = radius;
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.style.stroke_width = width;
        self
    }

    pub fn hover_radius(mut self, radius: f32) -> Self {
        self.style.hover_radius = radius;
        self
    }

    pub fn build(self) -> PointStyle {
        self.style
    }
}

pub struct BarStyleBuilder {
    pub width_factor: f32,
    pub corner_radius: f32,
    pub spacing: f32,
}

impl BarStyleBuilder {
    pub fn new() -> Self {
        Self {
            width_factor: 0.8,
            corner_radius: 4.0,
            spacing: 5.0,
        }
    }

    pub fn width_factor(mut self, factor: f32) -> Self {
        self.width_factor = factor;
        self
    }

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn build(self) -> BarStyleBuilder {
        self
    }
}

pub struct LegendBuilder {
    pub show: bool,
    pub position: LegendPosition,
    pub font_size: f32,
}

pub enum LegendPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl LegendBuilder {
    pub fn new() -> Self {
        Self {
            show: false,
            position: LegendPosition::TopRight,
            font_size: 12.0,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = show;
        self
    }

    pub fn position(mut self, pos: LegendPosition) -> Self {
        self.position = pos;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn build(self) -> LegendBuilder {
        self
    }
}

pub fn create_property_map() -> PropertyMap {
    Arc::new(Mutex::new(HashMap::new()))
}

pub fn create_visibility_set() -> VisibilitySet {
    Arc::new(Mutex::new(HashSet::new()))
}

pub fn normalize_value(value: f64, min: f64, max: f64) -> f64 {
    if max <= min {
        return 0.5;
    }
    (value - min) / (max - min)
}

pub fn screen_x(norm_x: f64, plot_rect: egui::Rect, width: f32) -> f32 {
    plot_rect.left() + (width * norm_x as f32)
}

pub fn screen_y(norm_y: f64, plot_rect: egui::Rect, height: f32) -> f32 {
    plot_rect.bottom() - (height * norm_y as f32)
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> egui::Color32 {
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

pub fn point_color(idx: usize, is_hovered: bool) -> egui::Color32 {
    let hue = (idx as f32 * 0.1) % 1.0;
    if is_hovered {
        hsv_to_rgb(hue, 0.95, 1.0)
    } else {
        hsv_to_rgb(hue, 0.75, 0.85)
    }
}

pub fn layout_bounds(total_width: f32, total_height: f32, config: &PlotConfig) -> (f32, f32, f32, f32) {
    let plot_width = total_width - config.padding_left - config.padding_right;
    let plot_height = total_height - config.padding_top - config.padding_bottom;
    (config.padding_left, config.padding_top, plot_width, plot_height)
}

pub struct ConfigStore {
    configs: Arc<Mutex<HashMap<String, PlotConfig>>>,
    defaults: Arc<Mutex<HashMap<String, PlotConfig>>>,
}

impl ConfigStore {
    pub fn new() -> Self {
        Self {
            configs: Arc::new(Mutex::new(HashMap::new())),
            defaults: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_default(self, name: impl Into<String>, config: PlotConfig) -> Self {
        if let Ok(mut defs) = self.defaults.lock() {
            defs.insert(name.into(), config);
        }
        self
    }

    pub fn get(&self, name: &str) -> Option<PlotConfig> {
        if let Ok(configs) = self.configs.lock() {
            if let Some(cfg) = configs.get(name) {
                return Some(cfg.clone());
            }
        }
        if let Ok(defs) = self.defaults.lock() {
            defs.get(name).cloned()
        } else {
            None
        }
    }

    pub fn set(&self, name: impl Into<String>, config: PlotConfig) {
        if let Ok(mut configs) = self.configs.lock() {
            configs.insert(name.into(), config);
        }
    }

    pub fn reset(&self, name: &str) {
        if let Ok(mut configs) = self.configs.lock() {
            configs.remove(name);
        }
    }

    pub fn list_configs(&self) -> Vec<String> {
        let mut names = Vec::new();
        if let Ok(configs) = self.configs.lock() {
            names.extend(configs.keys().cloned());
        }
        if let Ok(defs) = self.defaults.lock() {
            names.extend(defs.keys().cloned());
        }
        names.sort();
        names.dedup();
        names
    }
}

pub fn generic_normalize<T: Into<f64>>(value: T, min: f64, max: f64) -> f64 {
    let val = value.into();
    normalize_value(val, min, max)
}

pub fn apply_zoom<T: Into<f32>>(value: T, zoom: f32) -> f32 {
    value.into() * zoom
}

pub fn blend_colors(c1: egui::Color32, c2: egui::Color32, alpha: f32) -> egui::Color32 {
    let alpha = alpha.clamp(0.0, 1.0);
    let r = (c1.r() as f32 * (1.0 - alpha) + c2.r() as f32 * alpha) as u8;
    let g = (c1.g() as f32 * (1.0 - alpha) + c2.g() as f32 * alpha) as u8;
    let b = (c1.b() as f32 * (1.0 - alpha) + c2.b() as f32 * alpha) as u8;
    egui::Color32::from_rgb(r, g, b)
}

pub fn scale_rect(rect: egui::Rect, factor: f32) -> egui::Rect {
    let center = rect.center();
    let width = (rect.width() * factor) / 2.0;
    let height = (rect.height() * factor) / 2.0;
    egui::Rect::from_center_size(center, egui::vec2(width * 2.0, height * 2.0))
}

