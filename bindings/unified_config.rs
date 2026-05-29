use std::collections::HashMap;

#[crate::model(category = "Chart Config", domain = "plot")]
#[derive(Clone)]
pub struct ChartConfig {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub bg_color: u32,
    pub axis_color: u32,
    pub grid_color: u32,
    pub padding: i32,
    pub zoom: f32,
    pub points: Vec<ChartPoint>,
    pub orientation: bool,
    pub tooltip_bg: (u8, u8, u8, u8),
    pub tooltip_text: (u8, u8, u8, u8),
}

#[crate::model(category = "Chart Config", domain = "plot")]
#[derive(Clone)]
pub struct ChartPoint {
    pub label: String,
    pub value: f64,
    pub hover_data: HashMap<String, String>,
    pub visible: bool,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            width: 1200,
            height: 600,
            bg_color: 0xFFFFFF,
            axis_color: 0x000000,
            grid_color: 0xEEEEEE,
            padding: 50,
            zoom: 1.0,
            points: Vec::new(),
            orientation: true,
            tooltip_bg: (0, 0, 0, 0),
            tooltip_text: (0, 0, 0, 255),
        }
    }
}

impl ChartConfig {
    #[inline]
    pub fn new<S: Into<String>>(title: S) -> Self {
        let mut config = Self::default();
        config.title = title.into();
        config
    }

    #[inline]
    pub fn with_dimensions(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    #[inline]
    pub fn with_colors(mut self, bg: u32, axis: u32, grid: u32) -> Self {
        self.bg_color = bg;
        self.axis_color = axis;
        self.grid_color = grid;
        self
    }

    #[inline]
    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }

    #[inline]
    pub fn with_orientation(mut self, vertical: bool) -> Self {
        self.orientation = vertical;
        self
    }

    #[inline]
    pub fn add_point(&mut self, label: String, value: f64, hover_data: HashMap<String, String>) {
        self.points.push(ChartPoint { label, value, hover_data, visible: true });
    }

    #[inline]
    pub fn visible_points(&self) -> Vec<&ChartPoint> {
        self.points.iter().filter(|p| p.visible).collect()
    }

    #[inline]
    pub fn max_value(&self) -> f64 {
        self.visible_points().iter().fold(0.0_f64, |a, p| a.max(p.value))
    }

    #[inline]
    pub fn visible_count(&self) -> usize {
        self.points.iter().filter(|p| p.visible).count()
    }
}

pub struct ChartConfigBuilder {
    config: ChartConfig,
}

impl ChartConfigBuilder {
    #[inline]
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            config: ChartConfig::new(title),
        }
    }

    #[inline]
    pub fn dimensions(mut self, width: i32, height: i32) -> Self {
        self.config = self.config.with_dimensions(width, height);
        self
    }

    #[inline]
    pub fn colors(mut self, bg: u32, axis: u32, grid: u32) -> Self {
        self.config = self.config.with_colors(bg, axis, grid);
        self
    }

    #[inline]
    pub fn zoom(mut self, zoom: f32) -> Self {
        self.config = self.config.with_zoom(zoom);
        self
    }

    #[inline]
    pub fn orientation(mut self, vertical: bool) -> Self {
        self.config = self.config.with_orientation(vertical);
        self
    }

    #[inline]
    pub fn add_point(mut self, label: String, value: f64, hover_data: HashMap<String, String>) -> Self {
        self.config.add_point(label, value, hover_data);
        self
    }

    #[inline]
    pub fn build(self) -> ChartConfig {
        self.config
    }
}


