use super::unified_config::ChartConfig;
use super::builder_template::{BuilderOutput, Buildable};
use crate::plot::controller::chart_controller;

#[derive(Clone)]
pub struct ChartDataset {
    pub type_id: u8,
    pub name: String,
    pub color: u32,
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub x_data: Vec<f64>,
    pub y_data: Vec<f64>,
}

impl ChartDataset {
    #[inline]
    pub fn new(type_id: u8, name: String, color: u32) -> Option<Self> {
        let types = chart_controller::list_dataset_types();
        if types.iter().any(|(id, _, _)| *id == type_id) {
            Some(Self {
                type_id,
                name,
                color,
                labels: Vec::new(),
                values: Vec::new(),
                x_data: Vec::new(),
                y_data: Vec::new(),
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn with_color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }

    pub fn render_to_string(
        &self,
        config: &ChartConfig,
        _batch_size: usize,
    ) -> String {
        if let Ok(reg) = chart_controller::get_registry().lock() {
            if let Some(svg_renderer) = reg.get_svg(self.type_id) {
                let mut svg = String::new();
                let colors: Vec<&'static str> = vec!["#4a90e2"];
                svg_renderer(&mut svg, &self.values, &colors, 0, config.width, config.height, 1.0, true);
                return svg;
            }
        }
        String::new()
    }
}

pub struct DatasetBuilder {
    type_id: u8,
    name: String,
    color: u32,
}

impl DatasetBuilder {
    #[inline]
    pub fn new(type_id: u8, name: String) -> Option<Self> {
        let types = chart_controller::list_dataset_types();
        types.iter().find(|(id, _, _)| *id == type_id).map(|(_, _, default_color)| Self {
            type_id,
            name,
            color: *default_color,
        })
    }

    #[inline]
    pub fn color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }

    #[inline]
    pub fn build(self) -> Option<ChartDataset> {
        ChartDataset::new(self.type_id, self.name, self.color)
    }
}

pub struct ChartBuilder {
    config: ChartConfig,
    datasets: Vec<ChartDataset>,
    batch_size: usize,
}

impl BuilderOutput for ChartBuilder {
    type Output = (ChartConfig, Vec<ChartDataset>, usize);

    fn build(self) -> Self::Output {
        (self.config, self.datasets, self.batch_size)
    }
}

impl Buildable for ChartBuilder {
    fn builder() -> Self {
        Self::default()
    }
}

impl ChartBuilder {
    #[inline]
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            config: ChartConfig::new(title),
            datasets: Vec::with_capacity(8),
            batch_size: 1024,
        }
    }

    #[inline]
    pub fn config(mut self, config: ChartConfig) -> Self {
        self.config = config;
        self
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
    pub fn add_dataset(mut self, dataset: ChartDataset) -> Self {
        self.datasets.push(dataset);
        self
    }

    #[inline]
    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    #[inline]
    pub fn dataset_count(&self) -> usize {
        self.datasets.len()
    }

    #[inline]
    pub fn build(self) -> (ChartConfig, Vec<ChartDataset>, usize) {
        <Self as BuilderOutput>::build(self)
    }
}

impl Default for ChartBuilder {
    fn default() -> Self {
        Self::new("Chart")
    }
}

pub struct ChartBuildingPipeline {
    builder: ChartBuilder,
}

impl ChartBuildingPipeline {
    #[inline]
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            builder: ChartBuilder::new(title),
        }
    }

    #[inline]
    pub fn add_dataset_by_type<S: Into<String>>(mut self, type_id: u8, name: S) -> Self {
        if let Some(dataset) = ChartDataset::new(type_id, name.into(), Self::get_color(type_id)) {
            self.builder = self.builder.add_dataset(dataset);
        }
        self
    }

    #[inline]
    fn get_color(type_id: u8) -> u32 {
        chart_controller::list_dataset_types().iter()
            .find(|(id, _, _)| *id == type_id)
            .map(|(_, _, color)| *color)
            .unwrap_or(0x4a90e2)
    }

    #[inline]
    pub fn with_config(mut self, config: ChartConfig) -> Self {
        self.builder = self.builder.config(config);
        self
    }

    #[inline]
    pub fn build(self) -> (ChartConfig, Vec<ChartDataset>, usize) {
        self.builder.build()
    }
}

pub struct FullRenderPipeline {
    pipeline: ChartBuildingPipeline,
}

impl FullRenderPipeline {
    #[inline]
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            pipeline: ChartBuildingPipeline::new(title),
        }
    }

    #[inline]
    pub fn pipeline_mut(&mut self) -> &mut ChartBuildingPipeline {
        &mut self.pipeline
    }

    #[inline]
    pub fn pipeline(&self) -> &ChartBuildingPipeline {
        &self.pipeline
    }

    pub fn render(mut self) -> Vec<String> {
        let (config, datasets, _batch_size) = self.pipeline.build();
        datasets.iter().map(|dataset| dataset.render_to_string(&config, 1024)).collect()
    }

    pub fn render_single(mut self) -> String {
        let (config, datasets, _batch_size) = self.pipeline.build();
        if datasets.is_empty() {
            return String::new();
        }
        datasets[0].render_to_string(&config, 1024)
    }
}

