use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChartKind {
    Line,
    Scatter,
    Bar,
    Area,
    Histogram,
    Box,
    Violin,
    Heatmap,
    Contour,
    Surface,
    Bubble,
    Candlestick,
    Waterfall,
    Funnel,
    Sunburst,
    Treemap,
    Sankey,
}

#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b, a: 255 } }
    pub fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r, g, b, a } }
    pub fn from_hex(hex: &str) -> Self {
        let h = hex.trim_start_matches('#');
        let val = u32::from_str_radix(h, 16).unwrap_or(0);
        Self { r: ((val >> 16) & 0xFF) as u8, g: ((val >> 8) & 0xFF) as u8, b: (val & 0xFF) as u8, a: if h.len() == 8 { ((val >> 24) & 0xFF) as u8 } else { 255 } }
    }
    pub fn to_hex(&self) -> String { format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a) }
    pub fn transparent() -> Self { Self { r: 0, g: 0, b: 0, a: 0 } }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AggOp {
    Sum,
    Avg,
    Min,
    Max,
    Count,
    Median,
    Std,
    Var,
    Percentile(f64),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

impl Range {
    pub fn new(min: f64, max: f64) -> Self { Self { min, max } }
    pub fn from_slice(values: &[f64]) -> Option<Self> {
        if values.is_empty() { return None; }
        let (min, max) = (values.iter().cloned().fold(f64::INFINITY, f64::min), values.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
        Some(Self { min, max })
    }
    pub fn width(&self) -> f64 { self.max - self.min }
    pub fn center(&self) -> f64 { (self.min + self.max) / 2.0 }
    pub fn pad(&mut self, percent: f64) { let w = self.width() * percent; self.min -= w; self.max += w; }
    pub fn contains(&self, value: f64) -> bool { self.min <= value && value <= self.max }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Axis {
    pub title: String,
    pub kind: AxisKind,
    pub range: Option<Range>,
    pub color: Color,
    pub width: f32,
    pub label_size: u32,
    pub title_size: u32,
    pub show_grid: bool,
    pub grid_color: Color,
    pub grid_width: f32,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisKind {
    Linear,
    Log,
    Date,
    Category,
}

impl Default for Axis {
    fn default() -> Self {
        Self { title: String::new(), kind: AxisKind::Linear, range: None, color: Color::new(0, 0, 0), width: 1.0, label_size: 12, title_size: 14, show_grid: true, grid_color: Color::new(200, 200, 200), grid_width: 0.5 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Marker {
    pub size: u32,
    pub symbol: MarkerSymbol,
    pub color: Color,
    pub opacity: f32,
    pub line_width: f32,
    pub line_color: Color,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkerSymbol {
    Circle,
    Square,
    Diamond,
    Triangle,
    Cross,
    X,
    Star,
    Pentagon,
    Hexagon,
}

impl Default for Marker {
    fn default() -> Self {
        Self { size: 8, symbol: MarkerSymbol::Circle, color: Color::new(31, 119, 180), opacity: 1.0, line_width: 1.0, line_color: Color::new(0, 0, 0) }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Line {
    pub width: f32,
    pub color: Color,
    pub dash: LineDash,
    pub opacity: f32,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineDash {
    Solid,
    Dash,
    Dot,
    DashDot,
}

impl Default for Line {
    fn default() -> Self {
        Self { width: 2.0, color: Color::new(31, 119, 180), dash: LineDash::Solid, opacity: 1.0 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layout {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub x_axis: Axis,
    pub y_axis: Axis,
    pub z_axis: Option<Axis>,
    pub background: Color,
    pub paper_color: Color,
    pub plot_bgcolor: Color,
    pub margin: Margin,
    pub show_legend: bool,
    pub legend_x: f32,
    pub legend_y: f32,
    pub font_family: String,
    pub font_size: u32,
    pub font_color: Color,
    pub hovermode: HoverMode,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HoverMode {
    Closest,
    X,
    Y,
    XUnified,
    YUnified,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Margin {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

impl Default for Margin {
    fn default() -> Self {
        Self { left: 60, right: 40, top: 40, bottom: 60 }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            title: String::new(),
            width: 1200,
            height: 800,
            x_axis: Axis {
                title: "X".to_string(),
                ..Default::default()
            },
            y_axis: Axis {
                title: "Y".to_string(),
                ..Default::default()
            },
            z_axis: None,
            background: Color::new(255, 255, 255),
            paper_color: Color::new(255, 255, 255),
            plot_bgcolor: Color::new(240, 240, 240),
            margin: Margin::default(),
            show_legend: true,
            legend_x: 1.02,
            legend_y: 1.0,
            font_family: "sans-serif".to_string(),
            font_size: 12,
            font_color: Color::new(0, 0, 0),
            hovermode: HoverMode::Closest,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trace {
    pub id: String,
    pub name: String,
    pub kind: ChartKind,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Option<Vec<f64>>,
    pub marker: Marker,
    pub line: Line,
    pub fill: FillMode,
    pub visible: bool,
    pub opacity: f32,
    pub hover_text: Vec<String>,
    pub custom_data: HashMap<String, Vec<String>>,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FillMode {
    None,
    ToZero,
    ToZerox,
    ToNext,
    ToNextx,
    Tonext,
    Tonextx,
}

impl Trace {
    pub fn new<S: Into<String>>(id: S, name: S, kind: ChartKind, x: Vec<f64>, y: Vec<f64>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
            x,
            y,
            z: None,
            marker: Marker::default(),
            line: Line::default(),
            fill: FillMode::None,
            visible: true,
            opacity: 1.0,
            hover_text: Vec::new(),
            custom_data: HashMap::new(),
        }
    }

    pub fn bounds(&self) -> (Range, Range) {
        let x_range = Range::from_slice(&self.x).unwrap_or(Range::new(0.0, 1.0));
        let y_range = Range::from_slice(&self.y).unwrap_or(Range::new(0.0, 1.0));
        (x_range, y_range)
    }

    pub fn count(&self) -> usize {
        self.x.len().min(self.y.len())
    }
}
