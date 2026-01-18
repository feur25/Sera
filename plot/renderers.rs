use std::collections::HashMap;

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
        self.points.push(ChartPoint {
            label,
            value,
            hover_data,
            visible: true,
        });
    }

    pub fn visible_points(&self) -> Vec<&ChartPoint> {
        self.points.iter().filter(|p| p.visible).collect()
    }

    pub fn max_value(&self) -> f64 {
        self.points.iter()
            .filter(|p| p.visible)
            .fold(0.0_f64, |a, p| a.max(p.value))
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
        Self {
            config: ChartConfig::new(title),
        }
    }

    pub fn zoom(mut self, zoom: f32) -> Self {
        self.config.zoom = zoom;
        self
    }

    pub fn orientation(mut self, vertical: bool) -> Self {
        self.config.orientation = vertical;
        self
    }

    pub fn tooltip_colors(mut self, bg: (u8, u8, u8, u8), text: (u8, u8, u8, u8)) -> Self {
        self.config.tooltip_bg = bg;
        self.config.tooltip_text = text;
        self
    }

    pub fn add_point(mut self, label: String, value: f64, hover_data: HashMap<String, String>) -> Self {
        self.config.add_point(label, value, hover_data);
        self
    }

    pub fn build(self) -> ChartConfig {
        self.config
    }
}

pub trait ChartRenderer {
    fn render_vertical(&self, ctx: &egui::Context, ui: &mut egui::Ui);
    fn render_horizontal(&self, ctx: &egui::Context, ui: &mut egui::Ui);
}

pub struct BarChart {
    config: ChartConfig,
    hovered_idx: Option<usize>,
}

impl BarChart {
    pub fn new(config: ChartConfig) -> Self {
        Self {
            config,
            hovered_idx: None,
        }
    }

    pub fn with_hovered(mut self, idx: Option<usize>) -> Self {
        self.hovered_idx = idx;
        self
    }
}

pub struct ScatterChart {
    config: ChartConfig,
    hovered_idx: Option<usize>,
}

impl ScatterChart {
    pub fn new(config: ChartConfig) -> Self {
        Self {
            config,
            hovered_idx: None,
        }
    }

    pub fn with_hovered(mut self, idx: Option<usize>) -> Self {
        self.hovered_idx = idx;
        self
    }
}

pub struct LineChart {
    config: ChartConfig,
    hovered_idx: Option<usize>,
}

impl LineChart {
    pub fn new(config: ChartConfig) -> Self {
        Self {
            config,
            hovered_idx: None,
        }
    }

    pub fn with_hovered(mut self, idx: Option<usize>) -> Self {
        self.hovered_idx = idx;
        self
    }
}

pub enum ChartKind {
    Line,
    Scatter,
    Bar,
}

impl ChartKind {
    pub fn from_u8(kind: u8) -> Option<Self> {
        match kind {
            0 => Some(ChartKind::Line),
            1 => Some(ChartKind::Scatter),
            2 => Some(ChartKind::Bar),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            ChartKind::Line => 0,
            ChartKind::Scatter => 1,
            ChartKind::Bar => 2,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ChartKind::Line => "Line",
            ChartKind::Scatter => "Scatter",
            ChartKind::Bar => "Bar",
        }
    }

    pub fn is_valid(&self) -> bool {
        true
    }
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
