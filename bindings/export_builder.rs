use crate::plot::controller::chart_controller::get_group_registry;
use crate::html::HtmlExporter;
use crate::html::{HtmlExportConfig, HtmlTheme};

pub struct ExportBuilder {
    width: i32,
    height: i32,
    title: String,
    theme: HtmlTheme,
    svg_content: String,
    labels: Vec<String>,
    values: Vec<f64>,
    hover_data: serde_json::Value,
    chart_type: u8,
    color: u32,
}

impl ExportBuilder {
    pub fn new() -> Self {
        Self {
            width: 1200,
            height: 600,
            title: String::from("SeraPlot Export"),
            theme: HtmlTheme::Light,
            svg_content: String::new(),
            labels: Vec::new(),
            values: Vec::new(),
            hover_data: serde_json::json!({}),
            chart_type: 0,
            color: 0x1f77b4,
        }
    }

    pub fn width(mut self, w: i32) -> Self {
        self.width = w;
        self
    }

    pub fn height(mut self, h: i32) -> Self {
        self.height = h;
        self
    }

    pub fn title(mut self, t: &str) -> Self {
        self.title = t.to_string();
        self
    }

    pub fn color(mut self, c: u32) -> Self {
        self.color = c;
        self
    }

    pub fn chart_type(mut self, ct: u8) -> Self {
        self.chart_type = ct;
        self
    }

    pub fn theme(mut self, th: HtmlTheme) -> Self {
        self.theme = th;
        self
    }

    pub fn svg(mut self, s: String) -> Self {
        self.svg_content = s;
        self
    }

    pub fn data(mut self, labels: Vec<String>, values: Vec<f64>) -> Self {
        self.labels = labels;
        self.values = values;
        self
    }

    pub fn hover(mut self, h: serde_json::Value) -> Self {
        self.hover_data = h;
        self
    }

    pub fn build(self) -> HtmlExporter {
        let config = HtmlExportConfig {
            width: self.width,
            height: self.height,
            title: self.title.clone(),
            theme: self.theme,
        };

        HtmlExporter::new(config)
            .with_svg(self.svg_content)
            .with_data(self.labels, self.values)
            .with_hover(self.hover_data)
            .title(&self.title)
    }
}

impl Default for ExportBuilder {
    fn default() -> Self {
        Self::new()
    }
}
