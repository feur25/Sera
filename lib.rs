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
static GLOBAL_BACKGROUND: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

#[cfg(feature = "python")]
static AUTO_DISPLAY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

#[cfg(feature = "python")]
pub fn get_global_background() -> Option<String> {
    GLOBAL_BACKGROUND.lock().ok().and_then(|g| g.clone())
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot")]
pub struct Chart {
    html: String,
}

#[cfg(feature = "python")]
fn build_labels_js(pos: &str, forced: &str) -> String {
    let p = match pos {
        "top" | "left" | "right" | "bottom" => pos,
        _ => "bottom",
    };
    let mut s = String::with_capacity(4096);
    s.push_str("<script>(function(){");
    s.push_str("if(window.__SL__)return;window.__SL__=1;");
    s.push_str("function esc(t){return String(t).replace(/&/g,'&amp;').replace(/</g,'&lt;');}");
    s.push_str("function gl2d(svg){var a=[];svg.querySelectorAll('[data-legend]').forEach(function(lg){var rc=lg.querySelector('rect');var tx=lg.querySelector('text');var l=tx?tx.textContent:'';if(!l)return;a.push({lb:l,co:rc?rc.getAttribute('fill'):'',se:lg.getAttribute('data-series')});});return a;}");
    s.push_str("function gl3d(){var sc=document.querySelectorAll('script'),cl=null,pl=[];for(var i=0;i<sc.length;i++){var m=sc[i].textContent.match(/var CL=\\[([\\s\\S]*?)\\];/);if(m){try{cl=JSON.parse('['+m[1]+']');}catch(e){var a=m[1].match(/'([^']*)'/g)||[];cl=a.map(function(x){return x.slice(1,-1);});}break;}}if(!cl||!cl.length)return[];for(var j=0;j<sc.length;j++){var pm=sc[j].textContent.match(/var PAL=\\[([\\s\\S]*?)\\];/);if(pm){try{pl=JSON.parse('['+pm[1]+']');}catch(e){}break;}}var seen={},arr=[];cl.forEach(function(l){if(!seen[l]){seen[l]=1;arr.push({lb:l,co:pl[arr.length%pl.length]||'',se:null});}});return arr;}");
    s.push_str("function glColors(svg){var pal=[];if(!svg)return pal;var seen={};svg.querySelectorAll('rect[fill],path[fill],circle[fill]').forEach(function(e){var f=e.getAttribute('fill');if(f&&f!=='none'&&f!=='#fff'&&f!=='#ffffff'&&!seen[f]){seen[f]=1;pal.push(f);}});return pal;}");
    s.push_str(&format!("var POS='{}';", p));
    if !forced.is_empty() {
        s.push_str("var FRC=");
        s.push_str(forced);
        s.push_str(";");
    } else {
        s.push_str("var FRC=null;");
    }
    s.push_str("document.addEventListener('DOMContentLoaded',function(){");
    s.push_str("var cont=document.querySelector('.chart-container')||document.querySelector('.c3w');if(!cont)return;");
    s.push_str("if(getComputedStyle(cont).position==='static')cont.style.position='relative';");
    s.push_str("var svg=cont.querySelector('svg');");
    s.push_str("var items;if(FRC){var ac=glColors(svg);items=FRC.map(function(f,i){return{lb:f.l,co:f.c||(ac[i%ac.length]||''),se:f.s};});}else{items=svg?gl2d(svg):gl3d();}");
    s.push_str("if(!items.length)return;");
    s.push_str("var ov=document.createElement('div');");
    s.push_str("var isH=POS==='top'||POS==='bottom';");
    s.push_str("ov.style.cssText='position:absolute;z-index:200;display:flex;gap:6px;padding:6px 10px;pointer-events:auto;align-items:center;'+(isH?'flex-direction:row;flex-wrap:wrap;justify-content:center;':'flex-direction:column;');");
    s.push_str("if(POS==='top'){ov.style.top='4px';ov.style.left='50%';ov.style.transform='translateX(-50%)';}");
    s.push_str("else if(POS==='bottom'){ov.style.bottom='4px';ov.style.left='50%';ov.style.transform='translateX(-50%)';}");
    s.push_str("else if(POS==='left'){ov.style.left='4px';ov.style.top='50%';ov.style.transform='translateY(-50%)';}");
    s.push_str("else{ov.style.right='4px';ov.style.top='50%';ov.style.transform='translateY(-50%)';}");
    s.push_str("var dis=[];");
    s.push_str("var rb=document.createElement('span');rb.textContent='\\u21BA';rb.title='Show all';");
    s.push_str("rb.style.cssText='display:none;width:22px;height:22px;border-radius:50%;background:rgba(0,0,0,.6);color:#f1f5f9;font-size:13px;cursor:pointer;border:1px solid rgba(255,255,255,.2);align-items:center;justify-content:center;flex-shrink:0;backdrop-filter:blur(4px);';");
    s.push_str("rb.addEventListener('click',function(){dis.forEach(function(d){d.b.style.display='';setTimeout(function(){d.b.style.opacity='1';},10);if(d.se!=null&&svg){svg.querySelectorAll('[data-series=\"'+d.se+'\"]:not([data-legend])').forEach(function(e){e.style.display='';});}});dis=[];rb.style.display='none';});");
    s.push_str("items.forEach(function(it){");
    s.push_str("var b=document.createElement('span');");
    s.push_str("b.style.cssText='display:inline-flex;align-items:center;gap:5px;padding:3px 10px;border-radius:999px;font-size:11px;font-weight:600;cursor:pointer;user-select:none;transition:opacity .2s;background:rgba(0,0,0,.55);color:#f1f5f9;border:1px solid rgba(255,255,255,.15);backdrop-filter:blur(4px);white-space:nowrap;';");
    s.push_str("if(it.co){var d=document.createElement('span');d.style.cssText='width:8px;height:8px;border-radius:50%;flex-shrink:0;background:'+it.co+';';b.appendChild(d);}");
    s.push_str("b.appendChild(document.createTextNode(esc(it.lb)));");
    s.push_str("b.addEventListener('click',function(){b.style.opacity='0';setTimeout(function(){b.style.display='none';},200);dis.push({b:b,se:it.se});rb.style.display='inline-flex';if(it.se!=null&&svg){svg.querySelectorAll('[data-series=\"'+it.se+'\"]:not([data-legend])').forEach(function(e){e.style.display='none';});}});");
    s.push_str("ov.appendChild(b);});");
    s.push_str("ov.appendChild(rb);cont.appendChild(ov);");
    s.push_str("});})();</script></body>");
    s
}

#[cfg(feature = "python")]
fn encode_forced(labels: &[String], colors: &[String]) -> String {
    let mut j = String::from("[");
    for (i, lb) in labels.iter().enumerate() {
        if i > 0 { j.push(','); }
        j.push_str("{l:'");
        for ch in lb.chars() {
            match ch { '\'' => j.push_str("\\'"), '\\' => j.push_str("\\\\"), _ => j.push(ch) }
        }
        j.push_str("',c:'");
        if let Some(c) = colors.get(i) {
            for ch in c.chars() {
                match ch { '\'' => j.push_str("\\'"), '\\' => j.push_str("\\\\"), _ => j.push(ch) }
            }
        }
        j.push_str("',s:");
        j.push_str(&i.to_string());
        j.push('}');
    }
    j.push(']');
    j
}

#[cfg(feature = "python")]
fn inject_labels(html: &str, pos: &str, labels: &[String], colors: &[String]) -> String {
    if html.contains("window.__SL__") {
        return html.to_string();
    }
    let forced = if labels.is_empty() { String::new() } else { encode_forced(labels, colors) };
    html.replacen("</body>", &build_labels_js(pos, &forced), 1)
}

#[cfg(feature = "python")]
#[pymethods]
impl Chart {
    #[getter]
    fn html(&self) -> &str {
        &self.html
    }

    fn _repr_html_(&self) -> String {
        self.chart_iframe()
    }

    #[pyo3(signature = (**kwargs))]
    fn _ipython_display_(&self, py: Python<'_>, kwargs: Option<&pyo3::types::PyDict>) -> PyResult<()> {
        let _ = kwargs;
        let ipython = py.import("IPython.display")?;
        let html_cls = ipython.getattr("HTML")?;
        let display_fn = ipython.getattr("display")?;
        let html_obj = html_cls.call1((self.chart_iframe().as_str(),))?;
        display_fn.call1((html_obj,))?;
        Ok(())
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
        let html_obj = html_cls.call1((self.chart_iframe().as_str(),))?;
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

    fn no_hover(&self) -> Chart {
        Chart {
            html: self.html.replacen("</head>", "<style>#sp-tip{display:none!important}[data-idx]{pointer-events:none!important}[data-idx]:hover{filter:none!important}</style></head>", 1),
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

    #[pyo3(signature = (position="bottom", labels=None, colors=None))]
    fn show_labels(&self, position: &str, labels: Option<Vec<String>>, colors: Option<Vec<String>>) -> Chart {
        let lb = labels.unwrap_or_default();
        let co = colors.unwrap_or_default();
        Chart { html: inject_labels(&self.html, position, &lb, &co) }
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
        // Apply global background if one was set via sp.set_global_background(...)
        let html = if let Some(bg) = get_global_background() {
            crate::html::hover::apply_bg(html, Some(&bg))
        } else {
            html
        };
        let chart = Self { html };
        // Auto-display in Jupyter when the flag is enabled (default: true)
        if AUTO_DISPLAY.load(std::sync::atomic::Ordering::Relaxed) {
            Python::with_gil(|py| auto_show_in_jupyter(py, &chart));
        }
        chart
    }

    fn chart_iframe(&self) -> String {
        let h: u32 = {
            let mut found = 560u32;
            let s = &self.html;
            let mut start = 0usize;
            loop {
                match s[start..].find("height=\"") {
                    None => break,
                    Some(rel) => {
                        let abs = start + rel + 8;
                        if let Some(end) = s[abs..].find('"') {
                            if let Ok(v) = s[abs..abs+end].parse::<u32>() {
                                if v >= 150 && v <= 1600 { found = v; break; }
                            }
                        }
                        start += rel + 1;
                    }
                }
            }
            found + 24
        };
        let clean = self.html.replace(
            "border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)",
            "border-radius:0;overflow:hidden",
        );
        let esc = clean.replace('&', "&amp;").replace('"', "&quot;");
        format!(r#"<iframe srcdoc="{esc}" style="width:100%;height:{h}px;border:none;display:block;border-radius:8px;overflow:hidden" frameborder="0"></iframe>"#)
    }
}

#[cfg(feature = "python")]
fn auto_show_in_jupyter(py: Python<'_>, chart: &Chart) {
    let _ = (|| -> PyResult<()> {
        let ipython = py.import("IPython")?;
        let ip = ipython.getattr("get_ipython")?.call0()?;
        if ip.is_none() { return Ok(()); }
        let display_mod = py.import("IPython.display")?;
        let html_cls = display_mod.getattr("HTML")?;
        let display_fn = display_mod.getattr("display")?;
        let html_obj = html_cls.call1((chart.chart_iframe().as_str(),))?;
        display_fn.call1((html_obj,))?;
        Ok(())
    })();
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn set_global_background(color: &str) {
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() {
        *bg = Some(color.to_string());
    }
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn reset_global_background() {
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() {
        *bg = None;
    }
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn set_auto_display(enabled: bool) {
    AUTO_DISPLAY.store(enabled, std::sync::atomic::Ordering::Relaxed);
}

#[cfg(feature = "python")]
#[pymodule]
fn seraplot(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Chart>()?;
    m.add("__version__", VERSION)?;
    m.add("__doc__", "SeraPlot - Rust-Powered Data Visualization Framework\n\nSubmodules:\n  seraplot.charts   - bar, line, scatter, hbar\n  seraplot.stats    - histogram, grouped_bar, violin, heatmap, pie, ...\n  seraplot.geo      - choropleth, bubble_map\n  seraplot.three_d  - scatter3d, bar3d, line3d\n  seraplot.engine   - show_chart_value, bench, set_bg, ...")?;
    m.add_function(wrap_pyfunction!(set_global_background, m)?)?;
    m.add_function(wrap_pyfunction!(reset_global_background, m)?)?;
    m.add_function(wrap_pyfunction!(set_auto_display, m)?)?;
    bindings::python::python_registry::register_submodules(py, m)?;
    Ok(())
}
