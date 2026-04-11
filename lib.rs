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
fn label_btn_script(ap: Option<&str>) -> String {
    let ap_val = match ap {
        Some(p) => format!("'{}'", p),
        None => "null".to_string(),
    };
    let mut s = String::with_capacity(4096);
    s.push_str("<script>(function(){");
    s.push_str("if(window.__SLB__)return;window.__SLB__=1;");
    s.push_str(&format!("var AP={};", ap_val));
    s.push_str("var ST='idle',DIS=[],OV=null,MB,PK;");
    s.push_str("var CRN={'top-left':{top:'8px',right:'auto',bottom:'auto',left:'8px'},'top-right':{top:'8px',right:'8px',bottom:'auto',left:'auto'},'bottom-left':{top:'auto',right:'auto',bottom:'8px',left:'8px'},'bottom-right':{top:'auto',right:'8px',bottom:'8px',left:'auto'}};");
    s.push_str("function esc(s){return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;');}");
    s.push_str("function gl2d(svg){var arr=[];svg.querySelectorAll('[data-legend]').forEach(function(lg){var rc=lg.querySelector('rect');var tx=lg.querySelector('text');var lbl=tx?tx.textContent:'';if(!lbl)return;arr.push({lb:lbl,co:rc?rc.getAttribute('fill'):'',se:lg.getAttribute('data-series')});});return arr;}");
    s.push_str("function gl3d(){var sc=document.querySelectorAll('script'),cl=null,pl=[];for(var i=0;i<sc.length;i++){var m=sc[i].textContent.match(/var CL=\\[([\\s\\S]*?)\\];/);if(m){try{cl=JSON.parse('['+m[1]+']');}catch(e){var a=m[1].match(/'([^']*)'/g)||[];cl=a.map(function(x){return x.slice(1,-1);});}break;}}if(!cl||!cl.length)return[];for(var j=0;j<sc.length;j++){var pm=sc[j].textContent.match(/var PAL=\\[([\\s\\S]*?)\\];/);if(pm){try{pl=JSON.parse('['+pm[1]+']');}catch(e){}break;}}var seen={},arr=[];cl.forEach(function(l){if(!seen[l]){seen[l]=1;arr.push({lb:l,co:pl[arr.length%pl.length]||'',se:null});}});return arr;}");
    s.push_str("function mkOv(pos,cont,items,svg){if(OV){try{OV.parentNode.removeChild(OV);}catch(e){}OV=null;}DIS=[];var c=CRN[pos]||CRN['bottom-right'];OV=document.createElement('div');OV.style.cssText='position:absolute;z-index:200;display:flex;flex-wrap:wrap;gap:6px;padding:6px;';Object.keys(c).forEach(function(k){OV.style[k]=c[k];});");
    s.push_str("items.forEach(function(it){var b=document.createElement('span');b.style.cssText='display:inline-flex;align-items:center;gap:5px;padding:3px 8px;border-radius:999px;font-size:11px;font-weight:600;cursor:pointer;user-select:none;transition:opacity .2s;background:rgba(0,0,0,.55);color:#f1f5f9;border:1px solid rgba(255,255,255,.15);backdrop-filter:blur(4px);';");
    s.push_str("if(it.co){var d=document.createElement('span');d.style.cssText='width:8px;height:8px;border-radius:50%;background:'+it.co+';flex-shrink:0;';b.appendChild(d);}b.appendChild(document.createTextNode(esc(it.lb)));b.title='Click to dismiss';");
    s.push_str("b.onclick=function(){b.style.opacity='0';setTimeout(function(){b.style.display='none';},200);DIS.push({b:b,se:it.se,sv:svg});ubtn();};OV.appendChild(b);});cont.appendChild(OV);}");
    s.push_str("function rst(){DIS.forEach(function(x){x.b.style.display='';setTimeout(function(){x.b.style.opacity='1';},10);if(x.se!=null&&x.sv){x.sv.querySelectorAll('[data-series=\"'+x.se+'\"]:not([data-legend])').forEach(function(e){e.style.display='';});}});DIS=[];ubtn();}");
    s.push_str("function clOv(){if(OV){try{OV.parentNode.removeChild(OV);}catch(e){}OV=null;}DIS=[];ST='idle';ubtn();}");
    s.push_str("function ubtn(){if(ST==='idle'){MB.textContent='\u{1F3F7}';MB.title='Show series labels';}else if(ST==='pick'){MB.textContent='\u{D7}';MB.title='Cancel';}else{MB.textContent=DIS.length>0?'\u{21BA}':'\u{D7}';MB.title=DIS.length>0?'Reset dismissed':'Close labels';}PK.style.display=ST==='pick'?'flex':'none';}");
    s.push_str("document.addEventListener('DOMContentLoaded',function(){");
    s.push_str("var cont=document.querySelector('.chart-container')||document.querySelector('.c3w');if(!cont)return;");
    s.push_str("if(getComputedStyle(cont).position==='static')cont.style.position='relative';");
    s.push_str("var svg=cont.querySelector('svg');var items=svg?gl2d(svg):gl3d();if(!items.length)return;");
    s.push_str("var wrap=document.createElement('div');wrap.style.cssText='position:absolute;top:10px;left:20px;z-index:300;opacity:0;pointer-events:none;transition:opacity .2s;';");
    s.push_str("var col=document.createElement('div');col.style.cssText='display:flex;flex-direction:column;align-items:flex-start;';");
    s.push_str("var row=document.createElement('div');row.style.cssText='display:flex;gap:4px;align-items:center;';");
    s.push_str("MB=document.createElement('button');MB.style.cssText='background:rgba(255,255,255,.92);border:1px solid rgba(0,0,0,.15);border-radius:6px;width:28px;height:28px;cursor:pointer;font-size:14px;line-height:1;box-shadow:0 2px 6px rgba(0,0,0,.12);display:flex;align-items:center;justify-content:center;padding:0;';");
    s.push_str("MB.addEventListener('click',function(){if(ST==='idle'){ST='pick';ubtn();}else if(ST==='pick'){ST='idle';ubtn();}else{if(DIS.length>0)rst();else clOv();}});");
    s.push_str("PK=document.createElement('div');PK.style.cssText='display:none;gap:4px;margin-top:4px;flex-wrap:wrap;';");
    s.push_str("[['\u{2196}','top-left'],['\u{2197}','top-right'],['\u{2199}','bottom-left'],['\u{2198}','bottom-right']].forEach(function(p){var pb=document.createElement('button');pb.textContent=p[0];pb.title=p[1];pb.style.cssText='background:rgba(255,255,255,.92);border:1px solid rgba(0,0,0,.15);border-radius:6px;width:28px;height:28px;cursor:pointer;font-size:14px;box-shadow:0 2px 6px rgba(0,0,0,.12);padding:0;';pb.addEventListener('click',function(){mkOv(p[1],cont,items,svg);ST='on';ubtn();});PK.appendChild(pb);});");
    s.push_str("row.appendChild(MB);col.appendChild(row);col.appendChild(PK);wrap.appendChild(col);cont.appendChild(wrap);");
    s.push_str("cont.addEventListener('mouseenter',function(){wrap.style.opacity='1';wrap.style.pointerEvents='auto';});");
    s.push_str("cont.addEventListener('mouseleave',function(){if(ST!=='on'){wrap.style.opacity='0';wrap.style.pointerEvents='none';}});");
    s.push_str(&format!("if(AP){{mkOv(AP,cont,items,svg);ST='on';wrap.style.opacity='1';wrap.style.pointerEvents='auto';}}"));
    s.push_str("ubtn();});})();</script></body>");
    s
}

#[cfg(feature = "python")]
fn inject_label_btn(html: &str, ap: Option<&str>) -> String {
    if html.contains("window.__SLB__") {
        return html.to_string();
    }
    let script = label_btn_script(ap);
    html.replacen("</body>", &script, 1)
}

#[cfg(feature = "python")]
#[pymethods]
impl Chart {
    #[getter]
    fn html(&self) -> &str {
        &self.html
    }

    fn _repr_html_(&self) -> String {
        inject_label_btn(&self.html, None)
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
        Chart { html: inject_label_btn(&self.html, Some(position)) }
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
