#[macro_use]
pub mod registry_macro;
pub mod chart_types;
pub mod commands;
pub mod doc_registry;
pub mod exports;
pub mod fast_export_c;
pub mod fast_render;
pub mod fn_registry;
pub mod model_registry;
pub mod unified_config;
pub mod utils;

pub use crate::html::{FastHtmlExporter, HtmlExportConfig, HtmlExporter, HtmlTheme};
pub use chart_types::*;
pub use commands::*;
pub use fast_export_c::*;
pub use fast_render::*;
pub use unified_config::{ChartConfig, ChartConfigBuilder, ChartPoint};
pub use utils::*;

pub use crate::plot::default::bar::render_bars_fast;
pub use crate::plot::default::line::render_lines_fast;
pub use crate::plot::default::scatter::render_scatter_fast;
