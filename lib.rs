pub mod core;
pub mod data;
pub mod plot;
pub mod viewer;
pub mod bindings;
pub mod wiki;
pub mod html;

pub use core::math::{self, mean, median, std_dev};
pub use data::loader;
pub use data::processor;
pub use data::conversion;
pub use data::index;
pub use plot::canvas::Canvas;
pub use viewer::gui;
pub use wiki::{WikiExport, MethodDoc, ModuleDoc, WikiExtractor};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct SeraPlot;

impl SeraPlot {
    pub fn version() -> &'static str {
        VERSION
    }

    pub fn new_canvas(width: f32, height: f32, labels: Vec<String>, values: Vec<f64>, type_id: u8) -> Canvas {
        Canvas::new(width, height, labels, values, type_id)
    }

    pub fn load_csv<P: AsRef<std::path::Path>>(path: P) -> Result<crate::data::loader::CsvData, Box<dyn std::error::Error>> {
        crate::data::loader::CsvData::load(path).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn seraplot(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", VERSION)?;
    m.add("__doc__", r#"SeraPlot - Rust-Powered Data Visualization Framework

SeraPlot is a framework developed in Rust, meticulously crafted with care. It is a modern 
alternative to Plotly, designed specifically for data visualization. This library is distributed 
across multiple programming languages (Python, C#, C++, JavaScript), regularly maintained and 
updated, offering superior speed and significantly lower memory consumption compared to competitors.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 COVER: src/asset/cover.png
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Why Choose SeraPlot
  • More fast than plotly; generation across all dataset sizes
  • Minimal memory footprint - ideal for resource-constrained environments
  • Production-ready with enterprise-grade stability
  • Multi-language support (Python, C#, C++, JavaScript, and more if necessary send me a message)
  • Regularly updated with new plots - new features and improvements
  • Perfect for real-time dashboards and batch processing

Install Command
Seraplot may be installed using pip
    >>> pip install seraplot

or you can also install in conda
    >>> conda install -c conda-forge seraplot

Simple Usage 
  >>> import seraplot, json
  >>> seraplot.show_chart_value(json.dumps({
  ...     'title': 'My Chart',
  ...     'labels': ['A', 'B', 'C', 'D'],
  ...     'values': [45.2, 38.9, 52.1, 41.7],
  ...     'hover': [{'index': i} for i in range(4)],
  ...     'group': 'default'
  ... }))
"#)?;
    m.add_function(wrap_pyfunction!(show_chart_value, m)?)?;
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction]
fn show_chart_value(chart_json: &str) -> bool {
    let chart_json_c = std::ffi::CString::new(chart_json).unwrap_or_default();
    unsafe {
        crate::viewer::chart::sera_show_chart_value(chart_json_c.as_ptr())
    }
}
