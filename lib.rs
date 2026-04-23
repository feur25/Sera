pub mod core;
pub mod data;
pub mod plot;
pub mod ml;

#[cfg(any(feature = "python", feature = "gui"))]

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

#[cfg(any(feature = "python", feature = "gui"))]
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
static GLOBAL_PALETTE: std::sync::Mutex<Option<Vec<u32>>> = std::sync::Mutex::new(None);

#[cfg(feature = "python")]
static GLOBAL_GRIDLINES: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(feature = "python")]
static GLOBAL_THEME_NAME: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

#[cfg(feature = "python")]
static AUTO_DISPLAY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

#[cfg(feature = "python")]
static GLOBAL_FONT: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
#[cfg(feature = "python")]
static GLOBAL_FONT_SIZE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
#[cfg(feature = "python")]
static GLOBAL_TITLE_SIZE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
#[cfg(feature = "python")]
static GLOBAL_BORDER_RADIUS: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
#[cfg(feature = "python")]
static GLOBAL_OPACITY: std::sync::Mutex<Option<f64>> = std::sync::Mutex::new(None);
#[cfg(feature = "python")]
static GLOBAL_RESPONSIVE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[cfg(feature = "python")]
static GLOBAL_ANIMATION: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[cfg(feature = "python")]
static GLOBAL_ANIMATION_DURATION: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(300);
#[cfg(feature = "python")]
static GLOBAL_CROSSHAIR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[cfg(feature = "python")]
static GLOBAL_ZOOM: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[cfg(feature = "python")]
static GLOBAL_TOOLTIP: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
#[cfg(feature = "python")]
static GLOBAL_LOCALE: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
#[cfg(feature = "python")]
static GLOBAL_THOUSANDS_SEP: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
#[cfg(feature = "python")]
static GLOBAL_MARGIN: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
#[cfg(feature = "python")]
static GLOBAL_EXPORT_BTN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(feature = "python")]
pub fn get_global_background() -> Option<String> {
    GLOBAL_BACKGROUND.lock().ok().and_then(|g| g.clone())
}

#[cfg(feature = "python")]
pub fn get_global_palette() -> Option<Vec<u32>> {
    GLOBAL_PALETTE.lock().ok().and_then(|g| g.clone())
}

#[cfg(feature = "python")]
pub fn get_global_gridlines() -> bool {
    GLOBAL_GRIDLINES.load(std::sync::atomic::Ordering::Relaxed)
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot")]
pub struct Chart {
    html: String,
    doc_str: &'static str,
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

    fn doc(&self) -> &'static str {
        self.doc_str
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
        self.propagate(crate::html::hover::apply_bg(self.html.clone(), color))
    }

    #[pyo3(signature = (css))]
    fn inject_css(&self, css: &str) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>{css}</style></head>"), 1))
    }

    #[pyo3(signature = (js))]
    fn inject_js(&self, js: &str) -> Chart {
        self.propagate(self.html.replacen("</body>", &format!("<script>{js}</script></body>"), 1))
    }

    fn no_x_axis(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-ax-x,.sp-xt,.sp-xl{display:none}</style></head>", 1))
    }

    fn no_hover(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>#sp-tip{display:none!important}[data-idx]{pointer-events:none!important}[data-idx]:hover{filter:none!important}</style></head>", 1))
    }

    fn no_y_axis(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-ax-y,.sp-yt,.sp-yl{display:none}</style></head>", 1))
    }

    fn no_axes(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-ax-x,.sp-ax-y,.sp-xt,.sp-yt,.sp-xl,.sp-yl{display:none}</style></head>", 1))
    }

    fn show_grid(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-gl{display:block!important;opacity:1!important}</style></head>", 1))
    }

    fn hide_grid(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-gl{display:none!important}</style></head>", 1))
    }

    fn no_legend(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>g[data-legend]{display:none!important}</style></head>", 1))
    }

    fn no_title(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-ttl{display:none!important}</style></head>", 1))
    }

    #[pyo3(signature = (px))]
    fn set_font_size(&self, px: u32) -> Chart {
        let style = format!("<style>svg text{{font-size:{}px!important}}</style></head>", px);
        self.propagate(self.html.replacen("</head>", &style, 1))
    }

    #[pyo3(signature = (factor))]
    fn scale(&self, factor: f64) -> Chart {
        let style = format!("<style>svg{{transform:scale({});transform-origin:top left}}</style></head>", factor);
        self.propagate(self.html.replacen("</head>", &style, 1))
    }

    #[pyo3(signature = (color=None))]
    fn set_frame(&self, color: Option<&str>) -> Chart {
        let bg = match color {
            None | Some("none") | Some("transparent") | Some("") => "transparent".to_string(),
            Some(c) => c.to_string(),
        };
        let style = format!("<style>svg{{background:{bg}!important}}.c3w canvas{{background:{bg}!important}}.c3w{{background:{bg}!important}}</style></head>");
        self.propagate(self.html.replacen("</head>", &style, 1))
    }

    #[pyo3(signature = (position="bottom", labels=None, colors=None))]
    fn show_labels(&self, position: &str, labels: Option<Vec<String>>, colors: Option<Vec<String>>) -> Chart {
        let lb = labels.unwrap_or_default();
        let co = colors.unwrap_or_default();
        self.propagate(inject_labels(&self.html, position, &lb, &co))
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

    #[pyo3(signature = (name))]
    fn font(&self, name: &str) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>svg text,body{{font-family:'{}',system-ui,sans-serif!important}}</style></head>", name), 1))
    }

    #[pyo3(signature = (px))]
    fn title_size(&self, px: i32) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>.sp-ttl{{font-size:{}px!important}}</style></head>", px), 1))
    }

    fn crosshair(&self) -> Chart {
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", SP_CROSSHAIR_JS), 1))
    }

    fn zoom(&self) -> Chart {
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", SP_ZOOM_JS), 1))
    }

    fn responsive(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>svg{width:100%!important;height:auto!important}</style></head>", 1))
    }

    #[pyo3(signature = (duration=300))]
    fn animate(&self, duration: i32) -> Chart {
        let css = format!("<style>@keyframes sp-in{{from{{opacity:0;transform:translateY(8px)}}to{{opacity:1;transform:none}}}}svg rect[data-idx],svg circle[data-idx],svg path.sp-area{{animation:sp-in {}ms ease-out both}}</style></head>", duration);
        let js = "<script>(function(){if(window.__spa__)return;window.__spa__=1;var els=document.querySelectorAll('svg [data-idx]');for(var i=0;i<els.length;i++)els[i].style.animationDelay=i*30+'ms';})();</script></body>";
        self.propagate(self.html.replacen("</head>", &css, 1).replacen("</body>", js, 1))
    }

    #[pyo3(signature = (px))]
    fn border_radius(&self, px: i32) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>.chart-container,.c3w{{border-radius:{}px!important;overflow:hidden}}</style></head>", px), 1))
    }

    #[pyo3(signature = (value))]
    fn set_opacity(&self, value: f64) -> Chart {
        let v = value.clamp(0.0, 1.0);
        self.propagate(self.html.replacen("</head>", &format!("<style>svg rect[data-idx],svg circle[data-idx],svg path.sp-area{{opacity:{}!important}}</style></head>", v), 1))
    }

    #[pyo3(signature = (px))]
    fn set_margin(&self, px: i32) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>.chart-container,.c3w{{padding:{}px}}</style></head>", px), 1))
    }

    fn export_button(&self) -> Chart {
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", SP_EXPORT_JS), 1))
    }

    #[new]
    #[pyo3(signature = (html=String::new()))]
    fn py_new(html: String) -> Self {
        Self { html, doc_str: "" }
    }

    fn __getstate__(&self) -> String { self.html.clone() }

    fn __setstate__(&mut self, state: String) -> PyResult<()> {
        self.html = state;
        Ok(())
    }

    fn diff(&self, other: &Chart) -> String {
        crate::bindings::commands::charts::chart_diff(
            &serde_json::json!({"a": self.html, "b": other.html}).to_string()
        )
    }

    fn csp_safe(&self) -> Chart {
        let mut out = String::with_capacity(self.html.len());
        let mut rest = self.html.as_str();
        let mut blob = String::new();
        loop {
            match rest.find("<script>") {
                None => { out.push_str(rest); break; }
                Some(i) => {
                    out.push_str(&rest[..i]);
                    let after = &rest[i + 8..];
                    match after.find("</script>") {
                        None => { out.push_str("<script>"); out.push_str(after); break; }
                        Some(j) => {
                            blob.push_str(&after[..j]);
                            blob.push_str(";\n");
                            rest = &after[j + 9..];
                        }
                    }
                }
            }
        }
        let injected = if blob.is_empty() {
            out
        } else {
            let id = format!("sp-csp-{}", blob.len());
            let tag = format!("<script type=\"application/json\" id=\"{id}\">{}</script><script nonce=\"sp-nonce\">eval(document.getElementById('{id}').textContent)</script></body>", blob.replace("</script>", "<\\/script>"));
            out.replacen("</body>", &tag, 1)
        };
        self.propagate(injected)
    }

    #[pyo3(signature = (title="", desc=""))]
    fn a11y(&self, title: &str, desc: &str) -> Chart {
        let snippet = format!(
            "<svg role=\"img\" aria-label=\"{}\"><title>{}</title><desc>{}</desc>",
            title.replace('"', "&quot;"),
            title.replace('<', "&lt;"),
            desc.replace('<', "&lt;"),
        );
        self.propagate(self.html.replacen("<svg", &snippet, 1))
    }

    #[pyo3(signature = (n=2000, method="lttb"))]
    fn downsample(&self, n: usize, method: &str) -> Chart {
        let _ = method;
        let h = &self.html;
        let mut out = String::with_capacity(h.len());
        let mut rest = h.as_str();
        loop {
            match rest.find("data-x=\"") {
                None => { out.push_str(rest); break; }
                Some(i) => {
                    out.push_str(&rest[..i]);
                    let after = &rest[i + 8..];
                    let end = match after.find('"') { Some(e) => e, None => { out.push_str("data-x=\""); out.push_str(after); break; } };
                    let xs_raw = &after[..end];
                    let after2 = &after[end + 1..];
                    let after_y = match after2.find("data-y=\"") {
                        Some(j) => j,
                        None => { out.push_str("data-x=\""); out.push_str(after); break; }
                    };
                    let ys_section = &after2[after_y + 8..];
                    let ys_end = match ys_section.find('"') { Some(e) => e, None => { out.push_str("data-x=\""); out.push_str(after); break; } };
                    let ys_raw = &ys_section[..ys_end];
                    let xs: Vec<f64> = xs_raw.split(',').filter_map(|s| s.parse().ok()).collect();
                    let ys: Vec<f64> = ys_raw.split(',').filter_map(|s| s.parse().ok()).collect();
                    if xs.len() == ys.len() && xs.len() > n && n >= 3 {
                        let payload = serde_json::json!({"x":xs,"y":ys,"threshold":n}).to_string();
                        let res = crate::bindings::commands::charts::downsample_lttb(&payload);
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&res) {
                            if v.get("ok").and_then(|b| b.as_bool()).unwrap_or(false) {
                                let nx: Vec<String> = v.get("x").and_then(|a| a.as_array()).map(|a| a.iter().filter_map(|n| n.as_f64().map(|x| x.to_string())).collect()).unwrap_or_default();
                                let ny: Vec<String> = v.get("y").and_then(|a| a.as_array()).map(|a| a.iter().filter_map(|n| n.as_f64().map(|x| x.to_string())).collect()).unwrap_or_default();
                                out.push_str(&format!("data-x=\"{}\"", nx.join(",")));
                                out.push_str(&after2[..after_y]);
                                out.push_str(&format!("data-y=\"{}\"", ny.join(",")));
                                rest = &ys_section[ys_end + 1..];
                                continue;
                            }
                        }
                    }
                    out.push_str("data-x=\"");
                    out.push_str(xs_raw);
                    out.push('"');
                    out.push_str(&after2[..after_y]);
                    out.push_str("data-y=\"");
                    out.push_str(ys_raw);
                    out.push('"');
                    rest = &ys_section[ys_end + 1..];
                }
            }
        }
        self.propagate(out)
    }
}

#[cfg(feature = "python")]
const SP_CROSSHAIR_JS: &str = "(function(){if(window.__spc__)return;window.__spc__=1;var svg=document.querySelector('svg');if(!svg)return;var ns='http://www.w3.org/2000/svg';var vl=document.createElementNS(ns,'line');var hl=document.createElementNS(ns,'line');[vl,hl].forEach(function(l){l.setAttribute('stroke','#6366f1');l.setAttribute('stroke-width','1');l.setAttribute('stroke-dasharray','4,4');l.setAttribute('opacity','0.5');l.style.display='none';l.style.pointerEvents='none';svg.appendChild(l);});svg.addEventListener('mousemove',function(e){var r=svg.getBoundingClientRect();var x=e.clientX-r.left;var y=e.clientY-r.top;vl.setAttribute('x1',x);vl.setAttribute('x2',x);vl.setAttribute('y1',0);vl.setAttribute('y2',r.height);hl.setAttribute('x1',0);hl.setAttribute('x2',r.width);hl.setAttribute('y1',y);hl.setAttribute('y2',y);vl.style.display='';hl.style.display='';});svg.addEventListener('mouseleave',function(){vl.style.display='none';hl.style.display='none';});})()";

#[cfg(feature = "python")]
const SP_ZOOM_JS: &str = "(function(){if(window.__spz__)return;window.__spz__=1;var svg=document.querySelector('svg');if(!svg)return;var s=1,tx=0,ty=0,dr=false,sx,sy;svg.style.cursor='grab';svg.addEventListener('wheel',function(e){e.preventDefault();var z=e.deltaY<0?1.1:0.9;s=Math.min(Math.max(s*z,0.5),10);svg.style.transform='scale('+s+') translate('+tx+'px,'+ty+'px)';svg.style.transformOrigin='center center';},{passive:false});svg.addEventListener('mousedown',function(e){dr=true;sx=e.clientX-tx;sy=e.clientY-ty;svg.style.cursor='grabbing';});window.addEventListener('mouseup',function(){dr=false;if(svg)svg.style.cursor='grab';});svg.addEventListener('mousemove',function(e){if(!dr)return;tx=e.clientX-sx;ty=e.clientY-sy;svg.style.transform='scale('+s+') translate('+tx+'px,'+ty+'px)';});svg.addEventListener('dblclick',function(){s=1;tx=0;ty=0;svg.style.transform='';});})()";

#[cfg(feature = "python")]
const SP_EXPORT_JS: &str = "(function(){if(window.__spe__)return;window.__spe__=1;var c=document.querySelector('.chart-container')||document.querySelector('.c3w')||document.body;if(getComputedStyle(c).position==='static')c.style.position='relative';var b=document.createElement('button');b.textContent='\u{2B07}';b.title='Download chart';b.style.cssText='position:absolute;top:8px;right:8px;z-index:300;background:#6366f1;color:#fff;border:none;border-radius:6px;width:32px;height:32px;font-size:16px;cursor:pointer;opacity:0.6;transition:opacity .2s';b.onmouseenter=function(){b.style.opacity='1';};b.onmouseleave=function(){b.style.opacity='0.6';};b.onclick=function(){var bl=new Blob([document.documentElement.outerHTML],{type:'text/html'});var a=document.createElement('a');a.href=URL.createObjectURL(bl);a.download='chart.html';a.click();};c.appendChild(b);})()";

#[cfg(feature = "python")]
fn inject_global_cfg(html: String) -> String {
    use std::sync::atomic::Ordering::Relaxed;
    let mut css = String::new();
    let mut js = String::new();
    if let Ok(f) = GLOBAL_FONT.lock() {
        if let Some(ref font) = *f {
            css.push_str(&format!("svg text,body{{font-family:'{}',system-ui,sans-serif!important}}", font));
        }
    }
    let fs = GLOBAL_FONT_SIZE.load(Relaxed);
    if fs > 0 { css.push_str(&format!("svg text{{font-size:{}px!important}}", fs)); }
    let ts = GLOBAL_TITLE_SIZE.load(Relaxed);
    if ts > 0 { css.push_str(&format!(".sp-ttl{{font-size:{}px!important}}", ts)); }
    let br = GLOBAL_BORDER_RADIUS.load(Relaxed);
    if br > 0 { css.push_str(&format!(".chart-container,.c3w{{border-radius:{}px!important;overflow:hidden}}", br)); }
    if let Ok(op) = GLOBAL_OPACITY.lock() {
        if let Some(o) = *op { if o < 1.0 { css.push_str(&format!("svg rect[data-idx],svg circle[data-idx],svg path.sp-area{{opacity:{}!important}}", o)); } }
    }
    if GLOBAL_RESPONSIVE.load(Relaxed) { css.push_str("svg{width:100%!important;height:auto!important}"); }
    let mg = GLOBAL_MARGIN.load(Relaxed);
    if mg > 0 { css.push_str(&format!(".chart-container,.c3w{{padding:{}px}}", mg)); }
    if GLOBAL_ANIMATION.load(Relaxed) {
        let dur = GLOBAL_ANIMATION_DURATION.load(Relaxed);
        css.push_str(&format!("@keyframes sp-in{{from{{opacity:0;transform:translateY(8px)}}to{{opacity:1;transform:none}}}}svg rect[data-idx],svg circle[data-idx],svg path.sp-area{{animation:sp-in {}ms ease-out both}}", dur));
        js.push_str("(function(){if(window.__spa__)return;window.__spa__=1;var els=document.querySelectorAll('svg [data-idx]');for(var i=0;i<els.length;i++)els[i].style.animationDelay=i*30+'ms';})();");
    }
    if GLOBAL_CROSSHAIR.load(Relaxed) { js.push_str(SP_CROSSHAIR_JS); js.push(';'); }
    if GLOBAL_ZOOM.load(Relaxed) { js.push_str(SP_ZOOM_JS); js.push(';'); }
    if GLOBAL_EXPORT_BTN.load(Relaxed) { js.push_str(SP_EXPORT_JS); js.push(';'); }
    if css.is_empty() && js.is_empty() { return html; }
    let mut out = html;
    if !css.is_empty() { out = out.replacen("</head>", &format!("<style>{}</style></head>", css), 1); }
    if !js.is_empty() { out = out.replacen("</body>", &format!("<script>{}</script></body>", js), 1); }
    out
}

#[cfg(feature = "python")]
impl Chart {
    fn new(html: String) -> Self {
        let html = if let Some(bg) = get_global_background() {
            crate::html::hover::apply_bg(html, Some(&bg))
        } else {
            html
        };
        let html = inject_global_cfg(html);
        let chart = Self { html, doc_str: "" };
        if AUTO_DISPLAY.load(std::sync::atomic::Ordering::Relaxed) {
            Python::with_gil(|py| auto_show_in_jupyter(py, &chart));
        }
        chart
    }

    fn new_doc(html: String, doc: &'static str) -> Self {
        let mut c = Self::new(html);
        c.doc_str = doc;
        c
    }

    fn propagate(&self, html: String) -> Self {
        Self { html, doc_str: self.doc_str }
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
struct ThemePreset {
    bg: Option<&'static str>,
    palette: &'static [u32],
    gridlines: bool,
}

#[cfg(feature = "python")]
const THEME_DARK: ThemePreset = ThemePreset {
    bg: Some("#0f172a"),
    palette: &[0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA, 0x22D3EE, 0xF472B6, 0xA3E635, 0xF87171, 0x2DD4BF],
    gridlines: true,
};

#[cfg(feature = "python")]
const THEME_LIGHT: ThemePreset = ThemePreset {
    bg: None,
    palette: &[0x6366F1, 0xF43F5E, 0x10B981, 0xF59E0B, 0x8B5CF6, 0x06B6D4, 0xEC4899, 0x84CC16, 0xEF4444, 0x14B8A6],
    gridlines: false,
};

#[cfg(feature = "python")]
const THEME_SCIENTIFIC: ThemePreset = ThemePreset {
    bg: Some("#fafafa"),
    palette: &[0x1F77B4, 0xFF7F0E, 0x2CA02C, 0xD62728, 0x9467BD, 0x8C564B, 0xE377C2, 0x7F7F7F, 0xBCBD22, 0x17BECF],
    gridlines: true,
};

#[cfg(feature = "python")]
const THEME_APPLE: ThemePreset = ThemePreset {
    bg: Some("#000000"),
    palette: &[0x0A84FF, 0x30D158, 0xFF453A, 0xFFD60A, 0xBF5AF2, 0x64D2FF, 0xFF9F0A, 0xFF375F, 0xAC8E68, 0x63E6E2],
    gridlines: false,
};

#[cfg(feature = "python")]
const THEME_NOTION: ThemePreset = ThemePreset {
    bg: Some("#191919"),
    palette: &[0x529CCA, 0xD08B65, 0x6C9B7D, 0xCB7C7A, 0x9A6DD7, 0x868686, 0xCCAA55, 0x75B5AA, 0xD477A8, 0x507AA6],
    gridlines: false,
};

#[cfg(feature = "python")]
const THEME_MINIMAL: ThemePreset = ThemePreset {
    bg: None,
    palette: &[0x374151, 0x6B7280, 0x9CA3AF, 0xD1D5DB, 0x111827, 0x4B5563, 0x1F2937, 0xE5E7EB, 0x030712, 0x6B7280],
    gridlines: false,
};

#[cfg(feature = "python")]
const THEME_NEON: ThemePreset = ThemePreset {
    bg: Some("#0a0a0a"),
    palette: &[0x00FF87, 0xFF006E, 0x00B4D8, 0xFFBE0B, 0xE500A4, 0x8338EC, 0x3A86FF, 0xFB5607, 0xFF006E, 0x06D6A0],
    gridlines: false,
};

#[cfg(feature = "python")]
fn resolve_theme(name: &str) -> Option<&'static ThemePreset> {
    match name {
        "dark" => Some(&THEME_DARK),
        "light" => Some(&THEME_LIGHT),
        "scientific" => Some(&THEME_SCIENTIFIC),
        "apple" => Some(&THEME_APPLE),
        "notion" => Some(&THEME_NOTION),
        "minimal" => Some(&THEME_MINIMAL),
        "neon" => Some(&THEME_NEON),
        _ => None,
    }
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (name))]
pub fn theme(name: &str) -> PyResult<()> {
    let preset = resolve_theme(name)
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(
            format!("Unknown theme '{}'. Available: dark, light, scientific, apple, notion, minimal, neon", name)
        ))?;
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() {
        *bg = preset.bg.map(|s| s.to_string());
    }
    if let Ok(mut pal) = GLOBAL_PALETTE.lock() {
        *pal = Some(preset.palette.to_vec());
    }
    GLOBAL_GRIDLINES.store(preset.gridlines, std::sync::atomic::Ordering::Relaxed);
    if let Ok(mut tn) = GLOBAL_THEME_NAME.lock() {
        *tn = Some(name.to_string());
    }
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn reset_theme() {
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() { *bg = None; }
    if let Ok(mut pal) = GLOBAL_PALETTE.lock() { *pal = None; }
    GLOBAL_GRIDLINES.store(false, std::sync::atomic::Ordering::Relaxed);
    if let Ok(mut tn) = GLOBAL_THEME_NAME.lock() { *tn = None; }
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn themes() -> Vec<String> {
    vec!["dark", "light", "scientific", "apple", "notion", "minimal", "neon"]
        .into_iter().map(String::from).collect()
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (*, font=None, font_size=None, title_size=None, border_radius=None, opacity=None, responsive=None, animation=None, animation_duration=None, crosshair=None, zoom=None, tooltip=None, locale=None, thousands_sep=None, margin=None, export_button=None, palette=None, background=None, gridlines=None))]
pub fn config(font: Option<&str>, font_size: Option<i32>, title_size: Option<i32>, border_radius: Option<i32>, opacity: Option<f64>, responsive: Option<bool>, animation: Option<bool>, animation_duration: Option<i32>, crosshair: Option<bool>, zoom: Option<bool>, tooltip: Option<&str>, locale: Option<&str>, thousands_sep: Option<&str>, margin: Option<i32>, export_button: Option<bool>, palette: Option<Vec<u32>>, background: Option<&str>, gridlines: Option<bool>) {
    use std::sync::atomic::Ordering::Relaxed;
    if let Some(v) = font { if let Ok(mut f) = GLOBAL_FONT.lock() { *f = Some(v.to_string()); } }
    if let Some(v) = font_size { GLOBAL_FONT_SIZE.store(v, Relaxed); }
    if let Some(v) = title_size { GLOBAL_TITLE_SIZE.store(v, Relaxed); }
    if let Some(v) = border_radius { GLOBAL_BORDER_RADIUS.store(v, Relaxed); }
    if let Some(v) = opacity { if let Ok(mut o) = GLOBAL_OPACITY.lock() { *o = if v < 1.0 { Some(v) } else { None }; } }
    if let Some(v) = responsive { GLOBAL_RESPONSIVE.store(v, Relaxed); }
    if let Some(v) = animation { GLOBAL_ANIMATION.store(v, Relaxed); }
    if let Some(v) = animation_duration { GLOBAL_ANIMATION_DURATION.store(v, Relaxed); }
    if let Some(v) = crosshair { GLOBAL_CROSSHAIR.store(v, Relaxed); }
    if let Some(v) = zoom { GLOBAL_ZOOM.store(v, Relaxed); }
    if let Some(v) = tooltip { if let Ok(mut t) = GLOBAL_TOOLTIP.lock() { *t = Some(v.to_string()); } }
    if let Some(v) = locale { if let Ok(mut l) = GLOBAL_LOCALE.lock() { *l = Some(v.to_string()); } }
    if let Some(v) = thousands_sep { if let Ok(mut s) = GLOBAL_THOUSANDS_SEP.lock() { *s = Some(v.to_string()); } }
    if let Some(v) = margin { GLOBAL_MARGIN.store(v, Relaxed); }
    if let Some(v) = export_button { GLOBAL_EXPORT_BTN.store(v, Relaxed); }
    if let Some(v) = background { if let Ok(mut b) = GLOBAL_BACKGROUND.lock() { *b = Some(v.to_string()); } }
    if let Some(p) = palette { if let Ok(mut pl) = GLOBAL_PALETTE.lock() { *pl = Some(p); } }
    if let Some(v) = gridlines { GLOBAL_GRIDLINES.store(v, Relaxed); }
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn reset_config() {
    use std::sync::atomic::Ordering::Relaxed;
    if let Ok(mut f) = GLOBAL_FONT.lock() { *f = None; }
    GLOBAL_FONT_SIZE.store(0, Relaxed);
    GLOBAL_TITLE_SIZE.store(0, Relaxed);
    GLOBAL_BORDER_RADIUS.store(0, Relaxed);
    if let Ok(mut o) = GLOBAL_OPACITY.lock() { *o = None; }
    GLOBAL_RESPONSIVE.store(false, Relaxed);
    GLOBAL_ANIMATION.store(false, Relaxed);
    GLOBAL_ANIMATION_DURATION.store(300, Relaxed);
    GLOBAL_CROSSHAIR.store(false, Relaxed);
    GLOBAL_ZOOM.store(false, Relaxed);
    if let Ok(mut t) = GLOBAL_TOOLTIP.lock() { *t = None; }
    if let Ok(mut l) = GLOBAL_LOCALE.lock() { *l = None; }
    if let Ok(mut s) = GLOBAL_THOUSANDS_SEP.lock() { *s = None; }
    GLOBAL_MARGIN.store(0, Relaxed);
    GLOBAL_EXPORT_BTN.store(false, Relaxed);
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() { *bg = None; }
    if let Ok(mut pal) = GLOBAL_PALETTE.lock() { *pal = None; }
    GLOBAL_GRIDLINES.store(false, Relaxed);
    if let Ok(mut tn) = GLOBAL_THEME_NAME.lock() { *tn = None; }
}

#[cfg(feature = "python")]
pub(crate) fn py_to_f64_vec(_py: Python<'_>, obj: &PyAny) -> PyResult<Vec<f64>> {
    if let Ok(list) = obj.extract::<Vec<f64>>() {
        return Ok(list);
    }
    if let Ok(arr) = obj.getattr("tolist") {
        if let Ok(list) = arr.call0()?.extract::<Vec<f64>>() {
            return Ok(list);
        }
    }
    if let Ok(series) = obj.getattr("values") {
        if let Ok(arr) = series.getattr("tolist") {
            if let Ok(list) = arr.call0()?.extract::<Vec<f64>>() {
                return Ok(list);
            }
        }
    }
    Err(pyo3::exceptions::PyTypeError::new_err("Expected list, numpy array, or pandas Series of numbers"))
}

#[cfg(feature = "python")]
pub(crate) fn py_to_str_vec(_py: Python<'_>, obj: &PyAny) -> PyResult<Vec<String>> {
    if let Ok(list) = obj.extract::<Vec<String>>() {
        return Ok(list);
    }
    if let Ok(arr) = obj.getattr("tolist") {
        if let Ok(list) = arr.call0()?.extract::<Vec<String>>() {
            return Ok(list);
        }
    }
    if let Ok(series) = obj.getattr("values") {
        if let Ok(arr) = series.getattr("tolist") {
            if let Ok(list) = arr.call0()?.extract::<Vec<String>>() {
                return Ok(list);
            }
        }
    }
    if let Ok(list) = obj.extract::<Vec<f64>>() {
        return Ok(list.into_iter().map(|v| v.to_string()).collect());
    }
    Err(pyo3::exceptions::PyTypeError::new_err("Expected list, numpy array, or pandas Series of strings"))
}

#[cfg(feature = "python")]
pub(crate) fn merge_global_opts(background: Option<&str>, palette: Option<Vec<u32>>, gridlines: bool) -> (Option<String>, Vec<u32>, bool) {
    let bg = background.map(|s| s.to_string()).or_else(get_global_background);
    let pal = if palette.as_ref().map(|p| !p.is_empty()).unwrap_or(false) {
        palette.unwrap()
    } else {
        get_global_palette().unwrap_or_default()
    };
    let grid = gridlines || get_global_gridlines();
    (bg, pal, grid)
}

#[cfg(feature = "python")]
#[pymodule]
fn seraplot(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Chart>()?;
    m.add("__version__", VERSION)?;

    m.add_function(wrap_pyfunction!(set_global_background, m)?)?;
    m.add_function(wrap_pyfunction!(reset_global_background, m)?)?;
    m.add_function(wrap_pyfunction!(set_auto_display, m)?)?;
    m.add_function(wrap_pyfunction!(theme, m)?)?;
    m.add_function(wrap_pyfunction!(reset_theme, m)?)?;
    m.add_function(wrap_pyfunction!(themes, m)?)?;
    m.add_function(wrap_pyfunction!(config, m)?)?;
    m.add_function(wrap_pyfunction!(reset_config, m)?)?;

    bindings::commands::registry::register_submodules(py, m)?;
    Ok(())
}
