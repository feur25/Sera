use std::collections::HashMap;

pub struct RendererConfig {
    pub colors: Vec<u32>,
    pub use_fast_path: bool,
}

#[derive(Clone)]
pub struct ChartPoint {
    pub label: String,
    pub value: f64,
    pub hover_data: HashMap<String, String>,
    pub visible: bool,
}

pub struct ChartConfig {
    pub title: String,
    pub points: Vec<ChartPoint>,
    pub zoom: f32,
    pub orientation: bool,
    pub tooltip_bg: (u8, u8, u8, u8),
    pub tooltip_text: (u8, u8, u8, u8),
}

impl ChartConfig {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            points: Vec::new(),
            zoom: 1.0,
            orientation: true,
            tooltip_bg: (0, 0, 0, 0),
            tooltip_text: (0, 0, 0, 255),
        }
    }

    pub fn add_point(&mut self, label: String, value: f64, hover_data: HashMap<String, String>) {
        self.points.push(ChartPoint { label, value, hover_data, visible: true });
    }

    pub fn visible_points(&self) -> Vec<&ChartPoint> {
        self.points.iter().filter(|p| p.visible).collect()
    }

    pub fn max_value(&self) -> f64 {
        self.points.iter().filter(|p| p.visible).fold(0.0_f64, |a, p| a.max(p.value))
    }

    pub fn visible_count(&self) -> usize {
        self.points.iter().filter(|p| p.visible).count()
    }
}

pub struct ChartConfigBuilder {
    config: ChartConfig,
}

impl ChartConfigBuilder {
    pub fn new(title: impl Into<String>) -> Self {
        Self { config: ChartConfig::new(title) }
    }

    #[inline]
    pub fn zoom(mut self, zoom: f32) -> Self {
        self.config.zoom = zoom; self
    }

    #[inline]
    pub fn orientation(mut self, vertical: bool) -> Self {
        self.config.orientation = vertical; self
    }

    #[inline]
    pub fn tooltip_colors(mut self, bg: (u8, u8, u8, u8), text: (u8, u8, u8, u8)) -> Self {
        self.config.tooltip_bg = bg;
        self.config.tooltip_text = text;
        self
    }

    #[inline]
    pub fn add_point(mut self, label: String, value: f64, hover_data: HashMap<String, String>) -> Self {
        self.config.add_point(label, value, hover_data); self
    }

    #[inline]
    pub fn build(self) -> ChartConfig {
        self.config
    }
}

pub struct GenericChart {
    config: ChartConfig,
    hovered_idx: Option<usize>,
}

impl GenericChart {
    pub fn new(config: ChartConfig) -> Self {
        Self { config, hovered_idx: None }
    }

    #[inline]
    pub fn with_hovered(mut self, idx: Option<usize>) -> Self {
        self.hovered_idx = idx; self
    }

    #[inline]
    pub fn config(&self) -> &ChartConfig { &self.config }
    
    #[inline]
    pub fn hovered(&self) -> Option<usize> { self.hovered_idx }
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> egui::Color32 {
    let c = v * s;
    let h_p = (h * 6.0) % 6.0;
    let x = c * (1.0 - ((h_p % 2.0) - 1.0).abs());
    let (r, g, b) = match h_p as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    egui::Color32::from_rgb(((r + m) * 255.0) as u8, ((g + m) * 255.0) as u8, ((b + m) * 255.0) as u8)
}


