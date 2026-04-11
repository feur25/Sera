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

    fn no_legend(&self) -> Chart {
        Chart {
            html: self.html.replacen("</head>", "<style>g[data-legend]{display:none!important}</style></head>", 1),
        }
    }

    fn no_title(&self) -> Chart {
        Chart {
            html: self.html.replacen("</head>", "<style>.sp-ttl{display:none!important}</style></head>", 1),
        }
    }

    #[pyo3(signature = (px))]
    fn set_font_size(&self, px: u32) -> Chart {
        let style = format!("<style>svg text{{font-size:{}px!important}}</style></head>", px);
        Chart { html: self.html.replacen("</head>", &style, 1) }
    }

    #[pyo3(signature = (factor))]
    fn scale(&self, factor: f64) -> Chart {
        let style = format!("<style>svg{{transform:scale({});transform-origin:top left}}</style></head>", factor);
        Chart { html: self.html.replacen("</head>", &style, 1) }
    }

    #[pyo3(signature = (color=None))]
    fn set_frame(&self, color: Option<&str>) -> Chart {
        let bg = match color {
            None | Some("none") | Some("transparent") | Some("") => "transparent".to_string(),
            Some(c) => c.to_string(),
        };
        let style = format!("<style>svg{{background:{bg}!important}}.c3w canvas{{background:{bg}!important}}.c3w{{background:{bg}!important}}</style></head>");
        Chart { html: self.html.replacen("</head>", &style, 1) }
    }

    #[pyo3(signature = (position="bottom-right"))]
    fn show_labels(&self, position: &str) -> Chart {
        let (pt, pr, pb, pl) = match position {
            "top-left"     => ("8px", "auto", "auto", "8px"),
            "top-right"    => ("8px", "8px", "auto", "auto"),
            "bottom-left"  => ("auto", "auto", "8px", "8px"),
            _              => ("auto", "8px", "8px", "auto"),
        };
        let js = format!(
            concat!(
                "<script>(function(){{",
                "var OT='{pt}',OR='{pr}',OB='{pb}',OL='{pl}';",
                "function esc(s){{return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;');}}",
                "function mkO(){{var d=document.createElement('div');",
                "d.style.cssText='position:absolute;z-index:200;display:flex;flex-wrap:wrap;gap:6px;padding:6px;pointer-events:auto;';",
                "if(OT!=='auto')d.style.top=OT;if(OR!=='auto')d.style.right=OR;",
                "if(OB!=='auto')d.style.bottom=OB;if(OL!=='auto')d.style.left=OL;",
                "return d;}}",
                "function addB(ov,lbl,clr,fn){{",
                "var b=document.createElement('span');",
                "b.style.cssText='display:inline-flex;align-items:center;gap:5px;padding:3px 8px;border-radius:999px;font-family:-apple-system,Arial,sans-serif;font-size:11px;font-weight:600;cursor:pointer;user-select:none;transition:opacity .2s;background:rgba(0,0,0,.55);color:#f1f5f9;border:1px solid rgba(255,255,255,.15);backdrop-filter:blur(4px);';",
                "if(clr){{var dot=document.createElement('span');",
                "dot.style.cssText='width:8px;height:8px;border-radius:50%;background:'+clr+';flex-shrink:0;';",
                "b.appendChild(dot);}}",
                "b.appendChild(document.createTextNode(esc(lbl)));",
                "b.title='Click to dismiss';",
                "b.addEventListener('click',function(){{b.style.opacity='0';setTimeout(function(){{b.style.display='none';}},200);if(fn)fn();}});",
                "ov.appendChild(b);}}",
                "function i2d(wrap){{",
                "var svg=wrap.querySelector('svg');if(!svg)return;",
                "var legs=svg.querySelectorAll('[data-legend]');if(!legs.length)return;",
                "var ov=mkO();wrap.style.position='relative';wrap.appendChild(ov);",
                "legs.forEach(function(lg){{",
                "var rc=lg.querySelector('rect');var clr=rc?rc.getAttribute('fill'):'';",
                "var tx=lg.querySelector('text');var lbl=tx?tx.textContent:'';if(!lbl)return;",
                "var s=lg.getAttribute('data-series');",
                "addB(ov,lbl,clr,function(){{",
                "if(s!=null){{svg.querySelectorAll('[data-series=\"'+s+'\"]:not([data-legend])').forEach(function(e){{e.style.display='none';}});}}}});",
                "}});}}", 
                "function i3d(wrap){{",
                "var sc=document.querySelectorAll('script'),cl=null;",
                "for(var si=0;si<sc.length;si++){{",
                "var mm=sc[si].textContent.match(/var CL=\\[([\\s\\S]*?)\\];/);",
                "if(mm){{try{{cl=JSON.parse('['+mm[1]+']');}}catch(ee){{",
                "var rr=mm[1],arr=rr.match(/'([^']*)'/g)||[];",
                "cl=arr.map(function(x){{return x.slice(1,-1);}});}}break;}}}}",
                "if(!cl||!cl.length)return;",
                "var pal=[];var ps=document.querySelector('script:not([src])');",
                "if(ps){{var pm=ps.textContent.match(/var PAL=\\[([\\s\\S]*?)\\];/);",
                "if(pm){{try{{pal=JSON.parse('['+pm[1]+']');}}catch(ee){{}}}}}}",
                "var seen={{}},uniq=[];",
                "cl.forEach(function(l){{if(!seen[l]){{seen[l]=1;uniq.push({{lb:l,co:pal[uniq.length%pal.length]||''}});}}}});",
                "var ov=mkO();wrap.style.position='relative';wrap.appendChild(ov);",
                "uniq.forEach(function(it){{addB(ov,it.lb,it.co,null);}});}}",
                "document.addEventListener('DOMContentLoaded',function(){{",
                "var ws=document.querySelectorAll('.sp-wrap,.c3w');",
                "ws.forEach(function(w){{if(w.querySelector('svg'))i2d(w);else if(w.querySelector('canvas'))i3d(w);}});",
                "if(!ws.length){{",
                "var sv=document.querySelector('svg');if(sv&&sv.parentElement)i2d(sv.parentElement);",
                "var cv=document.querySelector('canvas');if(cv&&cv.parentElement)i3d(cv.parentElement);}}}});",
                "}})();</script></body>"
            ),
            pt=pt, pr=pr, pb=pb, pl=pl
        );
        Chart { html: self.html.replacen("</body>", &js, 1) }
    }

    fn to_svg(&self) -> PyResult<String> {
        let h = &self.html;
        let start = h.find("<svg").ok_or_else(|| pyo3::exceptions::PyValueError::new_err("No SVG in chart"))?;
        let end = h.rfind("</svg>").ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Malformed SVG"))? + 6;
        Ok(h[start..end].to_string())
    }

    #[pyo3(signature = (path))]
    fn export_svg(&self, path: &str) -> PyResult<()> {
        let svg = self.to_svg()?;
        std::fs::write(path, svg).map_err(|e| pyo3::exceptions::PyOSError::new_err(e.to_string()))
    }

    #[pyo3(signature = (path, scale=2.0))]
    fn export_png(&self, py: Python<'_>, path: &str, scale: f64) -> PyResult<()> {
        let svg = self.to_svg()?;
        match py.import("cairosvg") {
            Ok(m) => {
                let kwargs = pyo3::types::PyDict::new(py);
                kwargs.set_item("write_to", path)?;
                kwargs.set_item("scale", scale)?;
                m.call_method("svg2png", (svg.as_str(),), Some(kwargs))?;
                Ok(())
            }
            Err(_) => Err(pyo3::exceptions::PyImportError::new_err(
                "PNG export requires cairosvg: pip install cairosvg"
            ))
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
