pub mod fast_render;
pub mod utils;
pub mod chart_types;
pub mod fast_export_c;
pub mod memory_pool;
pub mod builder_template;
pub mod unified_config;
pub mod unified_builder;
pub mod export_builder;

pub use fast_render::*;
pub use utils::*;
pub use crate::html::{FastHtmlExporter, HtmlExporter, HtmlExportConfig, HtmlTheme};
pub use chart_types::*;
pub use fast_export_c::*;
pub use memory_pool::{MemoryBlock, RingBuffer, StackBuffer, CompactBuffer};
pub use builder_template::{GenericBuilder, BuilderOutput, Buildable, TypeRegistry};
pub use unified_config::{ChartConfig, ChartConfigBuilder, ChartPoint};
pub use unified_builder::{ChartBuilder, ChartDataset, DatasetBuilder, ChartBuildingPipeline, FullRenderPipeline};
pub use export_builder::ExportBuilder;

pub use crate::plot::default::bar::render_bars_fast;
pub use crate::plot::default::scatter::render_scatter_fast;
pub use crate::plot::default::line::render_lines_fast;