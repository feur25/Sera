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
#[pyclass(module = "seraplot")]
pub struct Chart {
    html: String,
}

#[cfg(feature = "python")]
#[pymethods]
impl Chart {
    #[getter]
    fn html(&self) -> &str {
        &self.html
    }

    fn _repr_html_(&self) -> &str {
        &self.html
    }

    fn __str__(&self) -> &str {
        &self.html
    }

    fn __repr__(&self) -> String {
        format!("SeraPlot.Chart({} bytes)", self.html.len())
    }

    fn __len__(&self) -> usize {
        self.html.len()
    }

    fn __bool__(&self) -> bool {
        !self.html.is_empty()
    }

    fn show(&self, py: Python<'_>) -> PyResult<()> {
        let ipython = py.import("IPython.display")?;
        let html_cls = ipython.getattr("HTML")?;
        let display_fn = ipython.getattr("display")?;
        let html_obj = html_cls.call1((self.html.as_str(),))?;
        display_fn.call1((html_obj,))?;
        Ok(())
    }

    #[pyo3(signature = (path))]
    fn save(&self, path: &str) -> PyResult<()> {
        std::fs::write(path, &self.html)?;
        Ok(())
    }

    #[pyo3(signature = (color=None))]
    fn set_bg(&self, color: Option<&str>) -> Chart {
        Chart {
            html: crate::html::hover::apply_bg(self.html.clone(), color),
        }
    }

    #[pyo3(signature = (css))]
    fn inject_css(&self, css: &str) -> Chart {
        Chart {
            html: self.html.replacen("</head>", &format!("<style>{css}</style></head>"), 1),
        }
    }

    #[pyo3(signature = (js))]
    fn inject_js(&self, js: &str) -> Chart {
        Chart {
            html: self.html.replacen("</body>", &format!("<script>{js}</script></body>"), 1),
        }
    }

    fn no_x_axis(&self) -> Chart {
        Chart {
            html: self.html.replacen("</head>", "<style>.sp-ax-x,.sp-xt,.sp-xl{display:none}</style></head>", 1),
        }
    }

    fn no_y_axis(&self) -> Chart {
        Chart {
            html: self.html.replacen("</head>", "<style>.sp-ax-y,.sp-yt,.sp-yl{display:none}</style></head>", 1),
        }
    }

    fn no_axes(&self) -> Chart {
        Chart {
            html: self.html.replacen("</head>", "<style>.sp-ax-x,.sp-ax-y,.sp-xt,.sp-yt,.sp-xl,.sp-yl{display:none}</style></head>", 1),
        }
    }

    fn show_grid(&self) -> Chart {
        Chart {
            html: self.html.replacen("</head>", "<style>.sp-gl{display:block!important;opacity:1!important}</style></head>", 1),
        }
    }

    fn hide_grid(&self) -> Chart {
        Chart {
            html: self.html.replacen("</head>", "<style>.sp-gl{display:none!important}</style></head>", 1),
        }
    }
}

#[cfg(feature = "python")]
impl Chart {
    fn new(html: String) -> Self {
        Self { html }
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn seraplot(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Chart>()?;
    m.add("__version__", VERSION)?;
    m.add("__doc__", "SeraPlot - Rust-Powered Data Visualization Framework\n\nSubmodules:\n  seraplot.charts   - bar, line, scatter, hbar\n  seraplot.stats    - histogram, grouped_bar, violin, heatmap, pie, ...\n  seraplot.geo      - choropleth, bubble_map\n  seraplot.three_d  - scatter3d, bar3d, line3d\n  seraplot.engine   - show_chart_value, bench, set_bg, ...")?;
    bindings::python::python_registry::register_submodules(py, m)?;
    Ok(())
}
