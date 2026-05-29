#[macro_use]
pub mod registry_macro;
pub mod doc_registry;
pub mod model_registry;
pub mod fast_render;
pub mod utils;
pub mod chart_types;
pub mod fast_export_c;
pub mod unified_config;
pub mod commands;

pub use fast_render::*;
pub use utils::*;
pub use commands::*;
pub use crate::html::{FastHtmlExporter, HtmlExporter, HtmlExportConfig, HtmlTheme};
pub use chart_types::*;
pub use fast_export_c::*;
pub use unified_config::{ChartConfig, ChartConfigBuilder, ChartPoint};

pub use crate::plot::default::bar::render_bars_fast;
pub use crate::plot::default::scatter::render_scatter_fast;
pub use crate::plot::default::line::render_lines_fast;

