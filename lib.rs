pub mod core;
pub mod data;
pub mod plot;
pub mod render;
pub mod viewer;
pub mod bindings;
pub mod wiki;

pub use core::types::{self, Layout};
pub use core::math::{self, mean, median, std_dev};
pub use data::loader;
pub use data::processor;
pub use data::conversion;
pub use data::index;
pub use plot::builder;
pub use plot::plot as plot_module;
pub use plot::canvas::Canvas;
pub use render::svg;
pub use render::json;
pub use render::interactive;
pub use render::dim3d;
pub use viewer::chart;
pub use viewer::gui;
pub use bindings::c;
pub use wiki::{WikiExport, MethodDoc, ModuleDoc, WikiExtractor};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct SeraPlot;

impl SeraPlot {
    pub fn version() -> &'static str {
        VERSION
    }

    pub fn new_canvas(width: f32, height: f32) -> Canvas {
        Canvas::new(width, height, Layout::default())
    }

    pub fn load_csv<P: AsRef<std::path::Path>>(path: P) -> Result<crate::data::loader::CsvData, Box<dyn std::error::Error>> {
        crate::data::loader::CsvData::load(path).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}
