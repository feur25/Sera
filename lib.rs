#[cfg(not(target_arch = "wasm32"))]
#[global_allocator]
static GLOBAL_ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub(crate) use bindings::chart_methods::apply::*;
pub mod core;
#[path = "bindings/doc_registry.rs"]
pub mod doc_registry;
#[path = "bindings/model_registry.rs"]
pub mod model_registry;
pub mod services;
pub use services::{data, ml, plot};
pub mod telemetry;

pub use crate::core::hw_profile::HwProfile;
pub use data::{DataPoint, Dataset, DatasetStats};

#[allow(unused_imports)]
pub(crate) use crate::bindings::registry_macro::{
    for_each_html_util_fn, for_each_json_chart_fn, for_each_ml_oneshot_fn, for_each_util_fn,
};
pub use seraplot_macros::{
    chart_demo, ml_doc, model, params, sera_alias, sera_bind, sera_binding, sera_builder,
    sera_class, sera_doc, sera_doc_impl, sera_impl, sera_register, sera_sig,
};

include!(concat!(env!("OUT_DIR"), "/demo_registry.rs"));
include!(concat!(env!("OUT_DIR"), "/params_registry.rs"));
include!(concat!(env!("OUT_DIR"), "/sera_aliases.rs"));
include!(concat!(env!("OUT_DIR"), "/chart_alias_registry.rs"));

pub fn demo_kwargs(family: &str, variant: &str) -> Option<&'static str> {
    DEMO_REGISTRY
        .iter()
        .find(|(f, v, _)| *f == family && *v == variant)
        .map(|(_, _, k)| *k)
}

pub fn required_params_for(family: &str, variant: &str) -> Option<&'static [&'static str]> {
    if let Some(e) = PARAMS_REGISTRY
        .iter()
        .find(|(f, v, _)| *f == family && *v == variant)
    {
        return Some(e.2);
    }
    PARAMS_REGISTRY
        .iter()
        .find(|(f, v, _)| *f == family && *v == "basic")
        .map(|e| e.2)
}

pub fn sera_aliases_for(key: &str) -> Option<&'static [&'static str]> {
    SERA_ALIASES.iter().find(|(k, _)| *k == key).map(|e| e.1)
}

pub fn demo_snippet(family: &str, variant: &str) -> Option<String> {
    let is_scene = crate::plot::scene3d::Scene3DVariant::keys_and_aliases()
        .iter()
        .any(|(k, aliases)| *k == variant || aliases.contains(&variant));
    let (k, kwarg) = if let Some(k) = demo_kwargs(family, variant) {
        (k, "variant")
    } else if is_scene {
        (demo_kwargs(family, "basic")?, "scene")
    } else {
        return None;
    };
    let mut c = variant.chars();
    let title = c
        .next()
        .map(|f| f.to_uppercase().chain(c).collect::<String>())
        .unwrap_or_default();
    let suffix = if variant == "basic" || variant == "default" {
        String::new()
    } else {
        format!(",\n    {}=\"{}\"", kwarg, variant)
    };
    Some(format!(
        "import seraplot as sp\n\nc = sp.{}(\n    \"{} demo\",\n    {}{}\n)\n",
        family, title, k, suffix
    ))
}

#[cfg(feature = "python")]
pub mod _py;
pub mod bindings;
pub use services::plot::html;
#[cfg(any(feature = "python", feature = "gui"))]
pub mod viewer;
#[cfg(feature = "webapp")]
pub mod webapp;
pub mod wiki;
include!(concat!(env!("OUT_DIR"), "/adapters.rs"));

pub use core::math::{self, mean, median, std_dev};
pub use data::loader;
pub use plot::canvas::Canvas;

#[cfg(any(feature = "python", feature = "gui"))]
pub use viewer::gui;

#[cfg(feature = "python")]
use crate::bindings::chart_methods::{build_compare_page, build_grid_page, cmp_score};
pub use wiki::{MethodDoc, ModuleDoc, WikiExport, WikiExtractor};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct SeraPlot;

impl SeraPlot {
    pub fn version() -> &'static str {
        VERSION
    }

    pub fn new_canvas(
        width: f32,
        height: f32,
        labels: Vec<String>,
        values: Vec<f64>,
        type_id: u8,
    ) -> Canvas {
        Canvas::new(width, height, labels, values, type_id)
    }

    pub fn load_csv<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<crate::data::loader::CsvData, Box<dyn std::error::Error>> {
        Ok(crate::data::loader::CsvData::load(path)?)
    }
}

#[cfg(feature = "python")]
use pyo3::prelude::*;

static GLOBAL_BACKGROUND: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

static GLOBAL_PALETTE: std::sync::Mutex<Option<Vec<u32>>> = std::sync::Mutex::new(None);

static GLOBAL_GRIDLINES: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static GLOBAL_THEME_NAME: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

static AUTO_DISPLAY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

static GLOBAL_FONT: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_FONT_SIZE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static GLOBAL_TITLE_SIZE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static GLOBAL_BORDER_RADIUS: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static GLOBAL_OPACITY: std::sync::Mutex<Option<f64>> = std::sync::Mutex::new(None);
static GLOBAL_RESPONSIVE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static GLOBAL_ANIMATION: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static GLOBAL_ANIMATION_DURATION: std::sync::atomic::AtomicI32 =
    std::sync::atomic::AtomicI32::new(300);
static GLOBAL_CROSSHAIR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static GLOBAL_ZOOM: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static GLOBAL_TOOLTIP: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_LOCALE: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_THOUSANDS_SEP: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_MARGIN: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static GLOBAL_EXPORT_BTN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static GLOBAL_TEXT_AUTO: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

static GLOBAL_TEXT_POSITION: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_TEXT_ANGLE: std::sync::atomic::AtomicI32 =
    std::sync::atomic::AtomicI32::new(i32::MIN);
static GLOBAL_TEXT_FONT_SIZE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static GLOBAL_TEXT_FONT_COLOR: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_UNIFORM_TEXT_MIN: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static GLOBAL_UNIFORM_TEXT_MODE: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_BAR_CORNER_RADIUS: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_HOVER_INFO: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_PATTERN_SHAPE: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
pub(crate) static GLOBAL_COLOR_BINDINGS: std::sync::Mutex<Vec<(String, u32)>> = std::sync::Mutex::new(Vec::new());

pub fn get_global_background() -> Option<String> {
    GLOBAL_BACKGROUND.lock().ok().and_then(|g| g.clone())
}

pub fn get_global_palette() -> Option<Vec<u32>> {
    GLOBAL_PALETTE.lock().ok().and_then(|g| g.clone())
}

pub fn get_global_gridlines() -> bool {
    GLOBAL_GRIDLINES.load(std::sync::atomic::Ordering::Relaxed)
}

#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(name = "Chart", module = "seraplot")
)]
#[sera_class]
pub struct Chart {
    pub(crate) html: String,
    pub(crate) doc_str: &'static str,
}

pub(crate) fn build_labels_js(pos: &str, forced: &str) -> String {
    let p = match pos {
        "top" | "left" | "right" | "bottom" => pos,
        _ => "bottom",
    };
    let mut s = String::with_capacity(4096);
    s.push_str("<script>(function(){");
    s.push_str("if(window.__SL__)return;window.__SL__=1;");
    s.push_str("function esc(t){return String(t).replace(/&/g,'&amp;').replace(/</g,'&lt;');}");
    s.push_str("function gl2d(svg){var a=[];svg.querySelectorAll('[data-legend]').forEach(function(lg){var rc=lg.querySelector('rect');var tx=lg.querySelector('text');var l=tx?tx.textContent:'';if(!l)return;a.push({lb:l,co:rc?rc.getAttribute('fill'):'',se:lg.getAttribute('data-series')});});return a;}");
    s.push_str("function glCat(svg){var a=[],seen={};svg.querySelectorAll('[data-idx][data-lbl]').forEach(function(el){var lb=el.getAttribute('data-lbl');if(!lb||seen[lb])return;seen[lb]=1;var co=el.getAttribute('fill')||'';a.push({lb:lb,co:co,se:null,idx:el.getAttribute('data-idx')});});return a;}");
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
    s.push_str("var items;if(FRC){var ac=glColors(svg);items=FRC.map(function(f,i){return{lb:f.l,co:f.c||(ac[i%ac.length]||''),se:f.s};});}else{items=svg?gl2d(svg):gl3d();if((!items||items.length<2)&&svg){var cats=glCat(svg);if(cats.length>=2)items=cats;}}");
    s.push_str("if(!items.length)return;");
    s.push_str("var ov=document.createElement('div');");
    s.push_str("var isH=POS==='top'||POS==='bottom';");
    s.push_str("ov.style.cssText='position:absolute;z-index:200;display:flex;gap:6px;padding:6px 10px;pointer-events:auto;align-items:center;'+(isH?'flex-direction:row;flex-wrap:wrap;justify-content:center;':'flex-direction:column;');");
    s.push_str("if(POS==='top'){ov.style.top='4px';ov.style.left='50%';ov.style.transform='translateX(-50%)';}");
    s.push_str("else if(POS==='bottom'){ov.style.bottom='4px';ov.style.left='50%';ov.style.transform='translateX(-50%)';}");
    s.push_str("else if(POS==='left'){ov.style.left='4px';ov.style.top='50%';ov.style.transform='translateY(-50%)';}");
    s.push_str(
        "else{ov.style.right='4px';ov.style.top='50%';ov.style.transform='translateY(-50%)';}",
    );
    s.push_str("var dis=[];");
    s.push_str(
        "var rb=document.createElement('span');rb.textContent='\\u21BA';rb.title='Show all';",
    );
    s.push_str("rb.style.cssText='display:none;width:22px;height:22px;border-radius:50%;background:rgba(0,0,0,.6);color:#f1f5f9;font-size:13px;cursor:pointer;border:1px solid rgba(255,255,255,.2);align-items:center;justify-content:center;flex-shrink:0;-webkit-backdrop-filter:blur(4px);backdrop-filter:blur(4px);';");
    s.push_str("rb.addEventListener('click',function(){dis.forEach(function(d){d.b.style.display='';setTimeout(function(){d.b.style.opacity='1';},10);if(svg){if(d.se!=null)svg.querySelectorAll('[data-series=\"'+d.se+'\"]:not([data-legend])').forEach(function(e){e.style.display='';});else if(d.ix!=null)svg.querySelectorAll('[data-idx=\"'+d.ix+'\"]').forEach(function(e){e.style.display='';});}});dis=[];rb.style.display='none';});");
    s.push_str("items.forEach(function(it){");
    s.push_str("var b=document.createElement('span');");
    s.push_str("b.style.cssText='display:inline-flex;align-items:center;gap:5px;padding:3px 10px;border-radius:999px;font-size:11px;font-weight:600;cursor:pointer;user-select:none;transition:opacity .2s;background:rgba(0,0,0,.55);color:#f1f5f9;border:1px solid rgba(255,255,255,.15);-webkit-backdrop-filter:blur(4px);backdrop-filter:blur(4px);white-space:nowrap;';");
    s.push_str("if(it.co){var d=document.createElement('span');d.style.cssText='width:8px;height:8px;border-radius:50%;flex-shrink:0;background:'+it.co+';';b.appendChild(d);}");
    s.push_str("b.appendChild(document.createTextNode(esc(it.lb)));");
    s.push_str("b.addEventListener('click',function(){b.style.opacity='0';setTimeout(function(){b.style.display='none';},200);dis.push({b:b,se:it.se,ix:it.idx});rb.style.display='inline-flex';if(svg){if(it.se!=null)svg.querySelectorAll('[data-series=\"'+it.se+'\"]:not([data-legend])').forEach(function(e){e.style.display='none';});else if(it.idx!=null)svg.querySelectorAll('[data-idx=\"'+it.idx+'\"]').forEach(function(e){e.style.display='none';});}});");
    s.push_str("ov.appendChild(b);});");
    s.push_str("ov.appendChild(rb);cont.appendChild(ov);");
    s.push_str("});})();</script></body>");
    s
}

pub(crate) fn encode_forced(labels: &[String], colors: &[String]) -> String {
    let mut j = String::from("[");
    for (i, lb) in labels.iter().enumerate() {
        if i > 0 {
            j.push(',');
        }
        j.push_str("{l:'");
        for ch in lb.chars() {
            match ch {
                '\'' => j.push_str("\\'"),
                '\\' => j.push_str("\\\\"),
                _ => j.push(ch),
            }
        }
        j.push_str("',c:'");
        if let Some(c) = colors.get(i) {
            for ch in c.chars() {
                match ch {
                    '\'' => j.push_str("\\'"),
                    '\\' => j.push_str("\\\\"),
                    _ => j.push(ch),
                }
            }
        }
        j.push_str("',s:");
        j.push_str(&i.to_string());
        j.push('}');
    }
    j.push(']');
    j
}

pub(crate) fn extract_c3d_id(html: &str) -> Option<&str> {
    let start = html.find("class=\"c3w\"")?;
    let id_attr = html[..start].rfind("id=\"")? + 4;
    let rest = &html[id_attr..];
    let end = rest.find('"')?;
    Some(&rest[..end])
}

pub(crate) fn apply_3d_cfg(html: String, opts_json: &str) -> String {
    let cid = match extract_c3d_id(&html) {
        Some(c) => c.to_string(),
        None => return html,
    };
    let snippet = format!(
        "<script>(function(){{var f=window['__sp3dCfg_{}'];if(f)f({});}})();</script></body>",
        cid, opts_json
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn inject_labels(html: &str, pos: &str, labels: &[String], colors: &[String]) -> String {
    if html.contains("class=\"c3w\"") {
        return apply_3d_cfg(html.to_string(), &format!("{{\"legend\":true,\"legendPos\":{}}}", json_str(pos)));
    }
    if html.contains("window.__SL__") {
        return html.to_string();
    }
    let forced = if labels.is_empty() {
        String::new()
    } else {
        encode_forced(labels, colors)
    };
    html.replacen("</body>", &build_labels_js(pos, &forced), 1)
}

const SP_CROSSHAIR_JS: &str = "(function(){if(window.__spc__)return;window.__spc__=1;var svg=document.querySelector('svg');if(!svg)return;var ns='http://www.w3.org/2000/svg';var vl=document.createElementNS(ns,'line');var hl=document.createElementNS(ns,'line');[vl,hl].forEach(function(l){l.setAttribute('stroke','#6366f1');l.setAttribute('stroke-width','1');l.setAttribute('stroke-dasharray','4,4');l.setAttribute('opacity','0.5');l.style.display='none';l.style.pointerEvents='none';svg.appendChild(l);});svg.addEventListener('mousemove',function(e){var r=svg.getBoundingClientRect();var x=e.clientX-r.left;var y=e.clientY-r.top;vl.setAttribute('x1',x);vl.setAttribute('x2',x);vl.setAttribute('y1',0);vl.setAttribute('y2',r.height);hl.setAttribute('x1',0);hl.setAttribute('x2',r.width);hl.setAttribute('y1',y);hl.setAttribute('y2',y);vl.style.display='';hl.style.display='';});svg.addEventListener('mouseleave',function(){vl.style.display='none';hl.style.display='none';});})()";

const SP_ZOOM_JS: &str = crate::bindings::chart_methods::js::SP_ZOOM_JS;

const SP_FLIP_JS: &str = "(function(){if(window.__spfl__)return;window.__spfl__=1;var svg=document.querySelector('svg');if(!svg)return;var m=svg.getAttribute('data-sp');if(!m)return;var p=m.split(',').map(Number),pL=p[0],pT=p[1],pW=p[2],pH=p[3];var rects=svg.querySelectorAll('rect[data-idx][data-v]');if(!rects.length)return;var n=rects.length,vals=[],ymax=0,ymin=0;for(var i=0;i<n;i++){var v=parseFloat(rects[i].getAttribute('data-v'));vals.push(v);if(v>ymax)ymax=v;if(v<ymin)ymin=v;}var rg=ymax-ymin;if(rg<1e-12)rg=1;var slotH=pH/n,barH=Math.max(2,slotH*0.7);for(var i=0;i<n;i++){var v=vals[i];var newW=Math.max(1,(v-ymin)/rg*pW);var ny=pT+i*slotH+(slotH-barH)/2;rects[i].setAttribute('x',pL);rects[i].setAttribute('y',ny);rects[i].setAttribute('width',newW);rects[i].setAttribute('height',barH);}var xts=svg.querySelectorAll('.sp-xt');for(var k=0;k<xts.length&&k<n;k++){xts[k].setAttribute('y',pT+k*slotH+slotH/2+4);xts[k].setAttribute('x',pL-8);xts[k].setAttribute('text-anchor','end');}var yts=svg.querySelectorAll('.sp-yt'),nT=yts.length;for(var j=0;j<nT;j++){var f=nT>1?j/(nT-1):0;var nx=pL+f*pW;var v2=ymin+f*rg;yts[j].setAttribute('x',nx);yts[j].setAttribute('y',pT+pH+16);yts[j].setAttribute('text-anchor','middle');yts[j].textContent=Math.abs(v2)>=1000?Math.round(v2).toString():(+v2).toFixed(2);}var gls=svg.querySelectorAll('.sp-gl');for(var g=0;g<gls.length;g++){var f=gls.length>1?(g+1)/(gls.length+1):0.5;var nx=pL+f*pW;gls[g].setAttribute('x1',nx);gls[g].setAttribute('x2',nx);gls[g].setAttribute('y1',pT);gls[g].setAttribute('y2',pT+pH);}})()";

pub(crate) const SP_SORT_JS: &str = "(function(){if(window.__spso__)return;window.__spso__=1;var ord=window.__sp_sort__||'desc';if(ord==='none')return;var svg=document.querySelector('svg');if(!svg)return;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));if(rects.length<2)return;var items=rects.map(function(r){return{r:r,v:parseFloat(r.getAttribute('data-v'))||0,lb:r.getAttribute('data-lbl')||'',x:parseFloat(r.getAttribute('x'))||0,y:parseFloat(r.getAttribute('y'))||0,h:parseFloat(r.getAttribute('height'))||0,w:parseFloat(r.getAttribute('width'))||0};});var horizontal=items[0].h<items[0].w*0.5&&items[0].x<100;var cmp;if(ord==='asc')cmp=function(a,b){return a.v-b.v;};else if(ord==='desc')cmp=function(a,b){return b.v-a.v;};else if(ord==='alpha')cmp=function(a,b){return a.lb.localeCompare(b.lb);};else if(ord==='alpha_desc')cmp=function(a,b){return b.lb.localeCompare(a.lb);};else return;var sorted=items.slice().sort(cmp);var slots=horizontal?items.map(function(it){return it.y;}).sort(function(a,b){return a-b;}):items.map(function(it){return it.x;}).sort(function(a,b){return a-b;});var labels=items.map(function(it){return it.lb;});var newOrder=sorted.map(function(it){return it.lb;});for(var k=0;k<sorted.length;k++){if(horizontal)sorted[k].r.setAttribute('y',slots[k]);else sorted[k].r.setAttribute('x',slots[k]);}var ts=svg.querySelectorAll(horizontal?'.sp-yt':'.sp-xt');var labTs=[];ts.forEach(function(t){var tt=t.textContent.trim();if(labels.indexOf(tt)>=0)labTs.push(t);});if(labTs.length===newOrder.length){for(var i=0;i<newOrder.length;i++){labTs[i].textContent=newOrder[i];}}})()";

const SP_MARGIN_JS: &str = "(function(){if(window.__spmg__)return;window.__spmg__=1;var m=+window.__sp_margin_px__||0;if(m<=0)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp');if(!d)return;var p=d.split(',').map(Number),pL=p[0],pT=p[1],pW=p[2],pH=p[3];if(m*2>=pW||m*2>=pH)return;var nL=pL+m,nT=pT+m,nW=pW-2*m,nH=pH-2*m,sx=nW/pW,sy=nH/pH;var rx=function(x){return nL+(x-pL)*sx;};var ry=function(y){return nT+(y-pT)*sy;};var els=svg.querySelectorAll('[data-idx]');els.forEach(function(e){var tg=e.tagName;if(tg==='rect'){var x=parseFloat(e.getAttribute('x'))||0,y=parseFloat(e.getAttribute('y'))||0,w=parseFloat(e.getAttribute('width'))||0,h=parseFloat(e.getAttribute('height'))||0;e.setAttribute('x',rx(x));e.setAttribute('y',ry(y));e.setAttribute('width',w*sx);e.setAttribute('height',h*sy);}else if(tg==='circle'){var cx=parseFloat(e.getAttribute('cx'))||0,cy=parseFloat(e.getAttribute('cy'))||0;e.setAttribute('cx',rx(cx));e.setAttribute('cy',ry(cy));}else if(tg==='line'){e.setAttribute('x1',rx(parseFloat(e.getAttribute('x1'))||0));e.setAttribute('x2',rx(parseFloat(e.getAttribute('x2'))||0));e.setAttribute('y1',ry(parseFloat(e.getAttribute('y1'))||0));e.setAttribute('y2',ry(parseFloat(e.getAttribute('y2'))||0));}});svg.querySelectorAll('.sp-xt').forEach(function(t){var x=parseFloat(t.getAttribute('x'))||0,y=parseFloat(t.getAttribute('y'))||0;t.setAttribute('x',rx(x));if(y>pT+pH-2)t.setAttribute('y',ry(pT+pH)+8);});svg.querySelectorAll('.sp-yt').forEach(function(t){var x=parseFloat(t.getAttribute('x'))||0,y=parseFloat(t.getAttribute('y'))||0;t.setAttribute('y',ry(y));if(x<pL+2)t.setAttribute('x',rx(pL)-6);});svg.querySelectorAll('.sp-gl').forEach(function(g){g.setAttribute('x1',rx(parseFloat(g.getAttribute('x1'))||0));g.setAttribute('x2',rx(parseFloat(g.getAttribute('x2'))||0));g.setAttribute('y1',ry(parseFloat(g.getAttribute('y1'))||0));g.setAttribute('y2',ry(parseFloat(g.getAttribute('y2'))||0));});svg.querySelectorAll('.sp-ax-x,.sp-ax-y').forEach(function(a){var x1=a.getAttribute('x1'),x2=a.getAttribute('x2'),y1=a.getAttribute('y1'),y2=a.getAttribute('y2');if(x1!=null)a.setAttribute('x1',rx(parseFloat(x1)));if(x2!=null)a.setAttribute('x2',rx(parseFloat(x2)));if(y1!=null)a.setAttribute('y1',ry(parseFloat(y1)));if(y2!=null)a.setAttribute('y2',ry(parseFloat(y2)));});svg.setAttribute('data-sp',[nL,nT,nW,nH].join(','));})()";

pub(crate) const SP_LEGEND_JS: &str = "(function(){if(window.__spleg__)return;window.__spleg__=1;var pos=window.__sp_legend_pos__||'right';var svg=document.querySelector('svg');if(!svg)return;var ns='http://www.w3.org/2000/svg';var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var stl=document.createElementNS(ns,'style');stl.textContent='svg rect[data-idx],svg circle[data-idx],svg line[data-idx],svg path[data-idx]{transition:x .35s cubic-bezier(.22,1,.36,1),y .35s cubic-bezier(.22,1,.36,1),width .35s cubic-bezier(.22,1,.36,1),height .35s cubic-bezier(.22,1,.36,1),cx .35s cubic-bezier(.22,1,.36,1),cy .35s cubic-bezier(.22,1,.36,1),opacity .25s ease}svg .sp-xt,svg .sp-yt,svg .sp-gl{transition:x .35s cubic-bezier(.22,1,.36,1),y .35s cubic-bezier(.22,1,.36,1),x1 .35s cubic-bezier(.22,1,.36,1),x2 .35s cubic-bezier(.22,1,.36,1),y1 .35s cubic-bezier(.22,1,.36,1),y2 .35s cubic-bezier(.22,1,.36,1),opacity .25s ease}g[data-leg-se]{transition:opacity .2s ease}svg [data-idx][style*=\"display: none\"]{opacity:0}';svg.insertBefore(stl,svg.firstChild);var legs=svg.querySelectorAll('g[data-legend]');var items=[];if(legs.length){legs.forEach(function(lg){var rc=lg.querySelector('rect'),tx=lg.querySelector('text');items.push({lb:tx?tx.textContent:'',co:rc?rc.getAttribute('fill'):'#888',se:lg.getAttribute('data-series')});lg.style.display='none';});}else{var bars=svg.querySelectorAll('[data-idx][data-lbl]');var seen={};bars.forEach(function(b){var lb=b.getAttribute('data-lbl')||'';if(!lb||seen[lb])return;seen[lb]=1;items.push({lb:lb,co:b.getAttribute('fill')||b.getAttribute('stroke')||'#888',se:lb});});}if(!items.length)return;var allRInit=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));var nTot=allRInit.length;var fillRX=0.7,fillRY=0.7,isHBar=false;if(nTot>0){var ri0=allRInit[0];var ri0w=parseFloat(ri0.getAttribute('width'))||1;var ri0h=parseFloat(ri0.getAttribute('height'))||1;isHBar=ri0w>ri0h*1.5;if(!isHBar){fillRX=Math.min(0.95,ri0w/(pW/nTot));}else{fillRY=Math.min(0.95,ri0h/(pH/nTot));}}var w=parseFloat(svg.getAttribute('width'))||900;var h=parseFloat(svg.getAttribute('height'))||480;var IH=22,GAP=10,PAD=10,SW=10;var isH=pos==='top'||pos==='bottom';var extra=isH?(IH+PAD*2):0;var extraW=isH?0:items.reduce(function(a,it){return Math.max(a,it.lb.length*8+SW+PAD+24);},70);var childs=[];for(var ci=0;ci<svg.childNodes.length;ci++){var nd=svg.childNodes[ci];if(nd!==stl)childs.push(nd);}var wrap=document.createElementNS(ns,'g');childs.forEach(function(n){wrap.appendChild(n);});if(pos==='top')wrap.setAttribute('transform','translate(0,'+extra+')');if(pos==='left')wrap.setAttribute('transform','translate('+extraW+',0)');svg.appendChild(wrap);if(isH){svg.setAttribute('height',h+extra);svg.setAttribute('viewBox','0 0 '+w+' '+(h+extra));}else{svg.setAttribute('width',w+extraW);svg.setAttribute('viewBox','0 0 '+(w+extraW)+' '+h);}var g=document.createElementNS(ns,'g');g.setAttribute('class','sp-leg-grp');var ix=isH?pL:(pos==='right'?w+PAD:PAD);var iy=pos==='top'?(PAD+IH/2+4):(pos==='bottom'?(h+extra-PAD-IH/2):(pT+IH));items.forEach(function(it){var gg=document.createElementNS(ns,'g');gg.setAttribute('data-leg-se',it.se!=null?String(it.se):it.lb);gg.style.cursor='pointer';var r=document.createElementNS(ns,'rect');r.setAttribute('width',SW);r.setAttribute('height',SW);r.setAttribute('rx','2');r.setAttribute('fill',it.co);r.setAttribute('class','sp-leg-sw');var tx=document.createElementNS(ns,'text');tx.setAttribute('font-family','-apple-system,Arial,sans-serif');tx.setAttribute('font-size','11');tx.setAttribute('fill','#374151');tx.setAttribute('class','sp-leg-tx');tx.textContent=it.lb;if(isH){r.setAttribute('x',ix);r.setAttribute('y',iy-SW/2);tx.setAttribute('x',ix+SW+4);tx.setAttribute('y',iy+4);ix+=SW+4+it.lb.length*8+GAP;}else{r.setAttribute('x',ix);r.setAttribute('y',iy-SW/2);tx.setAttribute('x',ix+SW+4);tx.setAttribute('y',iy+4);iy+=IH+GAP;}gg.appendChild(r);gg.appendChild(tx);g.appendChild(gg);});var hidden={};function rescale(){var allR=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));if(!allR.length)return;var visR=allR.filter(function(el){return el.style.display!=='none';});if(!visR.length)return;var nV=visR.length;var maxV=0;visR.forEach(function(el){var v=parseFloat(el.getAttribute('data-v'))||0;if(v>maxV)maxV=v;});if(maxV<=0)return;if(!isHBar){var nSW=pW/nV;var nBW=Math.max(1,nSW*fillRX);visR.forEach(function(el,i){var v=parseFloat(el.getAttribute('data-v'))||0;var nH=(v/maxV)*pH;el.setAttribute('x',pL+i*nSW+(nSW-nBW)/2);el.setAttribute('width',nBW);el.setAttribute('y',pT+pH-nH);el.setAttribute('height',nH);});var visXt=Array.prototype.slice.call(svg.querySelectorAll('.sp-xt')).filter(function(t){return t.style.display!=='none';});visXt.forEach(function(t,i){t.setAttribute('x',pL+i*nSW+nSW/2);});var yts=svg.querySelectorAll('.sp-yt');var nT=yts.length;for(var j=0;j<nT;j++){var fj=nT>1?j/(nT-1):0;var vj=fj*maxV;yts[j].setAttribute('y',pT+pH-(fj*pH)+4);yts[j].textContent=vj>=1000?Math.round(vj).toString():(+vj).toFixed(2);}var gls=svg.querySelectorAll('.sp-gl');for(var gi=0;gi<gls.length;gi++){var fg=gls.length>1?gi/(gls.length-1):0;var gy=pT+pH-(fg*pH);gls[gi].setAttribute('y1',gy);gls[gi].setAttribute('y2',gy);}}else{var nSH=pH/nV;var nBH=Math.max(1,nSH*fillRY);visR.forEach(function(el,i){var v=parseFloat(el.getAttribute('data-v'))||0;var nW=(v/maxV)*pW;el.setAttribute('y',pT+i*nSH+(nSH-nBH)/2);el.setAttribute('height',nBH);el.setAttribute('x',pL);el.setAttribute('width',nW);});var visYt=Array.prototype.slice.call(svg.querySelectorAll('.sp-yt')).filter(function(t){return t.style.display!=='none';});visYt.forEach(function(t,i){t.setAttribute('y',pT+i*nSH+nSH/2+4);});var xts2=svg.querySelectorAll('.sp-xt');var nX=xts2.length;for(var k=0;k<nX;k++){var fk=nX>1?k/(nX-1):0;var vk=fk*maxV;xts2[k].setAttribute('x',pL+fk*pW);xts2[k].textContent=vk>=1000?Math.round(vk).toString():(+vk).toFixed(2);}}}g.querySelectorAll('[data-leg-se]').forEach(function(grp){var se=grp.getAttribute('data-leg-se');grp.addEventListener('click',function(){var isHiding=!hidden[se];hidden[se]=isHiding;svg.querySelectorAll('[data-lbl=\"'+se+'\"],[data-series=\"'+se+'\"]').forEach(function(el){if(el.classList.contains('sp-leg-sw')||el.classList.contains('sp-leg-tx'))return;if(isHiding){el.style.opacity='0';setTimeout(function(){el.style.display='none';},250);}else{el.style.display='';requestAnimationFrame(function(){el.style.opacity='';});}});svg.querySelectorAll('.sp-xt,.sp-yt').forEach(function(t){if(t.textContent.trim()===se){if(isHiding){t.style.opacity='0';setTimeout(function(){t.style.display='none';},250);}else{t.style.display='';requestAnimationFrame(function(){t.style.opacity='';});}}});grp.style.opacity=isHiding?'0.35':'';if(isHiding){setTimeout(rescale,260);}else{requestAnimationFrame(rescale);}});});svg.appendChild(g);})()";

#[cfg(feature = "python")]
const SP_AUTOCLASS_JS: &str = "(function(){if(window.__spac__)return;window.__spac__=1;var svgs=document.querySelectorAll('svg');svgs.forEach(function(svg){var d=svg.getAttribute('data-sp');if(!d)return;var p=d.split(',').map(Number),pL=p[0]||0,pT=p[1]||0,pW=p[2]||0,pH=p[3]||0;if(pW<=0||pH<=0)return;var bX=pT+pH;var lX=pL;svg.querySelectorAll('text').forEach(function(t){if(t.getAttribute('class'))return;if(t.hasAttribute('data-idx'))return;if(t.hasAttribute('data-series'))return;var tx=parseFloat(t.getAttribute('x'));var ty=parseFloat(t.getAttribute('y'));if(!isFinite(tx)||!isFinite(ty))return;var ta=t.getAttribute('text-anchor')||'';var inXBand=ty>=bX-2&&ty<=bX+30;var inYBand=ty>=pT-4&&ty<=pT+pH+4&&tx<=lX+2&&tx>=lX-80;if(inXBand&&tx>=pL-5&&tx<=pL+pW+5){t.setAttribute('class','sp-xt');}else if(inYBand||(ta==='end'&&tx<lX&&ty>=pT-2&&ty<=pT+pH+12)){t.setAttribute('class','sp-yt');}});});})()";

const SP_BAR_GAP_JS: &str = "(function(){if(window.__spbg__)return;window.__spbg__=1;var gap=window.__sp_bar_gap__;if(gap==null)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var rects=svg.querySelectorAll('rect[data-idx][data-v]');if(!rects.length)return;var isHoriz=parseFloat(rects[0].getAttribute('width'))>parseFloat(rects[0].getAttribute('height'));var n=rects.length;var maxV=0;rects.forEach(function(r){var v=parseFloat(r.getAttribute('data-v'))||0;if(v>maxV)maxV=v;});if(maxV<=0)return;if(isHoriz){var slotH=pH/n;var barH=slotH*(1-gap);rects.forEach(function(r,i){var v=parseFloat(r.getAttribute('data-v'))||0;var bw=(v/maxV)*pW;var by=pT+i*slotH+(slotH-barH)/2;r.setAttribute('y',by);r.setAttribute('height',barH);r.setAttribute('x',pL);r.setAttribute('width',Math.max(1,bw));});var xts=svg.querySelectorAll('.sp-xt');xts.forEach(function(t,i){if(i<n)t.setAttribute('y',pT+i*(pH/n)+(pH/n)/2+4);});}else{var slotW=pW/n;var barW=slotW*(1-gap);rects.forEach(function(r,i){var v=parseFloat(r.getAttribute('data-v'))||0;var bh=(v/maxV)*pH;var bx=pL+i*slotW+(slotW-barW)/2;var by=pT+pH-bh;r.setAttribute('x',bx);r.setAttribute('width',Math.max(1,barW));r.setAttribute('y',by);r.setAttribute('height',bh);});var xts=svg.querySelectorAll('.sp-xt');xts.forEach(function(t,i){if(i<n){var cx=pL+i*slotW+slotW/2;t.setAttribute('x',cx);}});}})()";

pub(crate) const SP_TEXT_JS: &str = "(function(){var o=window.__sp_text__||{};if(window.__spt__)return;window.__spt__=1;var fmt=function(v,f){if(f==null||f===true||f==='true'||f==='')return (Math.round(v*1000)/1000).toString();var m=/^\\.(\\d+)([fs%eg])$/.exec(f);if(!m)return String(v);var d=+m[1],t=m[2];if(t==='f')return (+v).toFixed(d);if(t==='%')return ((+v)*100).toFixed(d)+'%';if(t==='e')return (+v).toExponential(d);if(t==='g')return (+v).toPrecision(d);if(t==='s'){var u=['','K','M','B','T'],a=Math.abs(+v),i=0;while(a>=1000&&i<u.length-1){a/=1000;i++;}var sn=(+v)<0?-a:a;return sn.toFixed(d)+u[i];}return String(v);};var ns='http://www.w3.org/2000/svg';var pos=o.position||'auto',ang=o.angle==null?0:+o.angle,fs=o.font_size||11,col=o.color||'#1f2937',ff=o.font_family||'system-ui,Arial,sans-serif',fmtS=o.format,umin=o.uniform_min||0,umode=o.uniform_mode||'';document.querySelectorAll('svg [data-v]').forEach(function(el){if(el.tagName==='text')return;if(el.getAttribute('data-sp-text')==='1')return;el.setAttribute('data-sp-text','1');var v=parseFloat(el.getAttribute('data-v'));if(!isFinite(v))return;var svg=el.ownerSVGElement;if(!svg)return;var bb;try{bb=el.getBBox();}catch(e){return;}var cx=bb.x+bb.width/2,tx,ty,ta='middle',pp=pos;if(pp==='auto')pp=(el.tagName==='rect'&&bb.height>fs*1.6)?'inside':'outside';if(el.tagName==='rect'){var isHoriz=bb.width>bb.height*1.5&&bb.x>50;if(pp==='inside'){tx=cx;ty=bb.y+bb.height/2+fs/3;}else if(pp==='outside'){tx=cx;ty=bb.y-4;}else{tx=cx;ty=bb.y-4;}if(isHoriz&&pp==='outside'){tx=bb.x+bb.width+6;ty=bb.y+bb.height/2+fs/3;ta='start';}}else{tx=cx;ty=bb.y-6;}var t=document.createElementNS(ns,'text');t.setAttribute('x',tx);t.setAttribute('y',ty);t.setAttribute('text-anchor',ta);t.setAttribute('font-family',ff);t.setAttribute('font-size',fs);t.setAttribute('fill',col);t.setAttribute('pointer-events','none');t.setAttribute('class','sp-vt');if(ang)t.setAttribute('transform','rotate('+ang+' '+tx+' '+ty+')');t.textContent=fmt(v,fmtS);el.parentNode.appendChild(t);if(umin>0){var rect=t.getBBox();if(rect.width>bb.width&&umode==='hide')t.style.display='none';}});})()";

const SP_BAR_RADIUS_JS: &str = "(function(){var r=window.__sp_bar_r__;if(r==null||window.__spbr__)return;window.__spbr__=1;document.querySelectorAll('svg rect[data-idx]').forEach(function(el){var v=r;if(typeof r==='string'&&r.charAt(r.length-1)==='%'){var bb;try{bb=el.getBBox();}catch(e){return;}var p=parseFloat(r)/100;v=Math.min(bb.width,bb.height)*p;}el.setAttribute('rx',v);el.setAttribute('ry',v);});})()";

const SP_EXPORT_JS: &str = "(function(){if(window.__spe__)return;window.__spe__=1;var c=document.querySelector('.chart-container')||document.querySelector('.c3w')||document.body;if(getComputedStyle(c).position==='static')c.style.position='relative';var b=document.createElement('button');b.textContent='\u{2B07}';b.title='Download chart';b.style.cssText='position:absolute;top:8px;right:8px;z-index:300;background:#6366f1;color:#fff;border:none;border-radius:6px;width:32px;height:32px;font-size:16px;cursor:pointer;opacity:0.6;transition:opacity .2s';b.onmouseenter=function(){b.style.opacity='1';};b.onmouseleave=function(){b.style.opacity='0.6';};b.onclick=function(ev){ev.preventDefault();ev.stopPropagation();try{var html='<!DOCTYPE html>\\n'+document.documentElement.outerHTML;var bl=new Blob([html],{type:'text/html;charset=utf-8'});var url=URL.createObjectURL(bl);var a=document.createElement('a');a.href=url;a.download='chart.html';a.rel='noopener';a.style.display='none';document.body.appendChild(a);a.click();setTimeout(function(){try{document.body.removeChild(a);URL.revokeObjectURL(url);}catch(e){}},100);}catch(e){try{var w=window.open('','_blank');if(w){w.document.write(document.documentElement.outerHTML);w.document.close();}}catch(_){}}};c.appendChild(b);})()";

const SP_GROUP_JS: &str = "(function(){var fe=window.frameElement;if(!fe)return;var nm=window.__sp_group_name__;if(!nm)return;var pos=window.__sp_group_pos__||'top';var lbl=(document.querySelector('.sp-ttl')||{}).textContent||'';fe.setAttribute('data-sp-group',nm);fe.setAttribute('data-sp-group-label',lbl);var doc=window.parent.document;if(!doc.getElementById('sp-group-style')){var st=doc.createElement('style');st.id='sp-group-style';st.textContent='.sp-group-wrap{display:flex;border:1px solid #e5e7eb;border-radius:8px;overflow:hidden}.sp-group-wrap.sp-group-col{flex-direction:column}.sp-group-wrap.sp-group-row{flex-direction:row}.sp-group-tabs{display:flex}.sp-group-wrap.sp-group-col .sp-group-tabs{flex-direction:row;flex-wrap:wrap;border-bottom:1px solid #e5e7eb}.sp-group-wrap.sp-group-col.sp-group-rev .sp-group-tabs{order:2;border-bottom:none;border-top:1px solid #e5e7eb}.sp-group-wrap.sp-group-row .sp-group-tabs{flex-direction:column;border-right:1px solid #e5e7eb;min-width:140px}.sp-group-wrap.sp-group-row.sp-group-rev .sp-group-tabs{order:2;border-right:none;border-left:1px solid #e5e7eb}.sp-group-tab{padding:10px 16px;border:none;background:none;color:#6b7280;cursor:pointer;font-size:13px;font-family:system-ui,sans-serif;text-align:left;border-bottom:2px solid transparent}.sp-group-wrap.sp-group-row .sp-group-tab{border-bottom:none;border-left:2px solid transparent}.sp-group-tab:hover{color:#111827}.sp-group-tab.sp-group-act{color:#111827;font-weight:600;border-bottom-color:#111827}.sp-group-wrap.sp-group-row .sp-group-tab.sp-group-act{border-left-color:#111827}.sp-group-body{flex:1;min-width:0}.sp-group-body iframe{width:100%;border:none}';doc.head.appendChild(st);}if(!doc.getElementById('sp-group-script')){var sc=doc.createElement('script');sc.id='sp-group-script';sc.textContent='window.__spGroupShow=function(w,e){w.querySelectorAll(\"iframe\").forEach(function(f){f.style.setProperty(\"display\",\"none\",\"important\");});e.style.setProperty(\"display\",\"block\",\"important\");};window.__spGroupClick=function(btn,wid,eid){var w=document.getElementById(wid),e=document.getElementById(eid);if(!w||!e)return;w.querySelectorAll(\".sp-group-tab\").forEach(function(b){b.classList.remove(\"sp-group-act\");});btn.classList.add(\"sp-group-act\");window.__spGroupShow(w,e);};';doc.head.appendChild(sc);}function findWrap(els){for(var i=0;i<els.length;i++){var p=els[i].parentElement;if(p&&p.classList&&p.classList.contains('sp-group-body'))return p.parentElement;}return null;}function uid(prefix){return prefix+Math.random().toString(36).slice(2)+Date.now().toString(36);}function run(){var els=Array.prototype.slice.call(doc.querySelectorAll('iframe[data-sp-group=\"'+nm+'\"]'));if(els.length<2)return;var wrap=findWrap(els);if(!wrap){var first=els[0];wrap=doc.createElement('div');wrap.id=uid('sp-wrap-');wrap.className='sp-group-wrap '+((pos==='left'||pos==='right')?'sp-group-row':'sp-group-col')+((pos==='right'||pos==='bottom')?' sp-group-rev':'');var tabs=doc.createElement('div');tabs.className='sp-group-tabs';var body=doc.createElement('div');body.className='sp-group-body';wrap.appendChild(tabs);wrap.appendChild(body);first.parentElement.insertBefore(wrap,first);}var tabs2=wrap.querySelector('.sp-group-tabs');var body2=wrap.querySelector('.sp-group-body');els.forEach(function(el,i){if(el.getAttribute('data-sp-grouped'))return;el.setAttribute('data-sp-grouped','1');if(!el.id)el.id=uid('sp-frame-');var idx=tabs2.children.length;var btn=doc.createElement('button');btn.className='sp-group-tab'+(idx===0?' sp-group-act':'');btn.textContent=el.getAttribute('data-sp-group-label')||('Chart '+(idx+1));btn.setAttribute('onclick','window.__spGroupClick(this,\"'+wrap.id+'\",\"'+el.id+'\")');tabs2.appendChild(btn);body2.appendChild(el);el.style.setProperty('display',idx===0?'block':'none','important');});}run();setTimeout(run,80);})()";

const SP_CAPTION_JS: &str = "(function(){var c=document.body.firstElementChild;if(!c)return;var outer=document.createElement('div');outer.style.cssText='display:flex;flex-direction:column;align-items:center;width:100%';c.parentNode.insertBefore(outer,c);outer.appendChild(c);var cap=document.createElement('div');cap.className='sp-caption';cap.textContent=window.__sp_caption__;outer.appendChild(cap);})()";

const SP_TRENDLINE_JS: &str = "(function(){var cfg=window.__sp_trendline__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));if(rects.length<2)return;var pts=rects.map(function(r){return{x:parseFloat(r.getAttribute('x'))+parseFloat(r.getAttribute('width'))/2,v:parseFloat(r.getAttribute('data-v'))||0};});var maxV=0;pts.forEach(function(p){if(p.v>maxV)maxV=p.v;});if(maxV<=0)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pT=sp[1]||36,pH=sp[3]||360;var n=pts.length,sx=0,sy=0,sxy=0,sxx=0;pts.forEach(function(p,i){sx+=i;sy+=p.v;sxy+=i*p.v;sxx+=i*i;});var denom=(n*sxx-sx*sx)||1;var slope=(n*sxy-sx*sy)/denom;var intercept=(sy-slope*sx)/n;var ns='http://www.w3.org/2000/svg';var y0=pT+pH-(intercept/maxV)*pH;var y1=pT+pH-((intercept+slope*(n-1))/maxV)*pH;var ln=document.createElementNS(ns,'line');ln.setAttribute('x1',pts[0].x);ln.setAttribute('y1',y0);ln.setAttribute('x2',pts[n-1].x);ln.setAttribute('y2',y1);ln.setAttribute('stroke',cfg.c);ln.setAttribute('stroke-width',cfg.w);ln.setAttribute('stroke-linecap','round');svg.appendChild(ln);})()";

const SP_ANNOTATE_EXTREME_JS: &str = "(function(){var cfg=window.__sp_annotate_extreme__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));if(!rects.length)return;var best=null;rects.forEach(function(r){var v=parseFloat(r.getAttribute('data-v'));if(best===null||(cfg.mode==='max'?v>best.v:v<best.v)){best={v:v,r:r};}});if(!best)return;best.r.style.setProperty('fill',cfg.c,'important');var ns='http://www.w3.org/2000/svg';var x=parseFloat(best.r.getAttribute('x'))+parseFloat(best.r.getAttribute('width'))/2;var y=parseFloat(best.r.getAttribute('y'))-8;var t=document.createElementNS(ns,'text');t.setAttribute('x',x);t.setAttribute('y',y);t.setAttribute('text-anchor','middle');t.setAttribute('font-size','11');t.setAttribute('font-weight','700');t.setAttribute('fill',cfg.c);t.textContent=cfg.lbl||best.v;svg.appendChild(t);})()";

const SP_REFERENCE_BAND_JS: &str = "(function(){var cfg=window.__sp_ref_band__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var rects=svg.querySelectorAll('rect[data-idx][data-v]');if(!rects.length)return;var maxV=0;rects.forEach(function(r){var v=parseFloat(r.getAttribute('data-v'))||0;if(v>maxV)maxV=v;});if(maxV<=0)return;var ns='http://www.w3.org/2000/svg';var yLow=pT+pH-(cfg.lo/maxV)*pH,yHigh=pT+pH-(cfg.hi/maxV)*pH;var rect=document.createElementNS(ns,'rect');rect.setAttribute('x',pL);rect.setAttribute('width',pW);rect.setAttribute('y',Math.min(yLow,yHigh));rect.setAttribute('height',Math.abs(yLow-yHigh));rect.setAttribute('fill',cfg.c);rect.setAttribute('opacity',cfg.op);var first=svg.querySelector('rect[data-idx]');svg.insertBefore(rect,first);})()";

const SP_HLINE_JS: &str = "(function(){var cfg=window.__sp_hline__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var rects=svg.querySelectorAll('rect[data-idx][data-v]');if(!rects.length)return;var maxV=0;rects.forEach(function(r){var v=parseFloat(r.getAttribute('data-v'))||0;if(v>maxV)maxV=v;});if(maxV<=0)return;var ns='http://www.w3.org/2000/svg';var y=pT+pH-(cfg.v/maxV)*pH;var ln=document.createElementNS(ns,'line');ln.setAttribute('x1',pL);ln.setAttribute('x2',pL+pW);ln.setAttribute('y1',y);ln.setAttribute('y2',y);ln.setAttribute('stroke',cfg.c);ln.setAttribute('stroke-width','1.5');ln.setAttribute('stroke-dasharray','6,4');svg.appendChild(ln);if(cfg.lbl){var tx=document.createElementNS(ns,'text');tx.setAttribute('x',pL+pW-4);tx.setAttribute('y',y-6);tx.setAttribute('text-anchor','end');tx.setAttribute('font-size','11');tx.setAttribute('fill',cfg.c);tx.textContent=cfg.lbl;svg.appendChild(tx);}})()";

const SP_GROUP_HOVER_JS: &str = "(function(){if(window.__spgh__)return;window.__spgh__=1;var dim=window.__sp_group_dim__||0.15;var svg=document.querySelector('svg');if(!svg)return;var all=Array.prototype.slice.call(svg.querySelectorAll('[data-group],[data-lbl]'));if(!all.length)return;function getG(el){return el.getAttribute('data-group')||el.getAttribute('data-lbl')||'';}all.forEach(function(el){el.addEventListener('mouseenter',function(){var g=getG(el);all.forEach(function(e){if(getG(e)!==g){e.style.opacity=String(dim);e.style.transition='opacity .18s ease';}});});el.addEventListener('mouseleave',function(){all.forEach(function(e){e.style.opacity='';});});});})()";

const SP_DESATURATE_JS: &str = "(function(){var cfg=window.__sp_desat__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;svg.querySelectorAll('[data-idx]').forEach(function(el){var idx=parseInt(el.getAttribute('data-idx'));if(cfg.i&&cfg.i.indexOf(idx)<0)return;el.style.filter='saturate('+cfg.f+')';el.style.transition='filter .2s';});})()";
const SP_GRID_Y_ONLY_JS: &str = "(function(){var svg=document.querySelector('svg');if(!svg)return;svg.querySelectorAll('line.sp-gl').forEach(function(l){if(l.getAttribute('x1')!==l.getAttribute('x2'))l.style.display='none';});})()";
const SP_GRID_X_ONLY_JS: &str = "(function(){var svg=document.querySelector('svg');if(!svg)return;svg.querySelectorAll('line.sp-gl').forEach(function(l){if(l.getAttribute('y1')!==l.getAttribute('y2'))l.style.display='none';});})()";
const SP_GRID_AT_JS: &str = "(function(){var cfg=window.__sp_grid_at__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var rects=svg.querySelectorAll('rect[data-idx][data-v]');if(!rects.length)return;var maxV=0;rects.forEach(function(r){var v=parseFloat(r.getAttribute('data-v'))||0;if(v>maxV)maxV=v;});if(maxV<=0)return;var ns='http://www.w3.org/2000/svg';var color=cfg.c||'#64748b';var first=svg.querySelector('rect[data-idx]');cfg.vs.forEach(function(val,i){var y=pT+pH-(val/maxV)*pH;if(y<pT-2||y>pT+pH+2)return;var ln=document.createElementNS(ns,'line');ln.setAttribute('x1',pL);ln.setAttribute('x2',pL+pW);ln.setAttribute('y1',y);ln.setAttribute('y2',y);ln.setAttribute('stroke',color);ln.setAttribute('stroke-width','1');ln.setAttribute('stroke-opacity','0.75');if(first)svg.insertBefore(ln,first);else svg.appendChild(ln);if(cfg.ls&&cfg.ls[i]!=null){var tx=document.createElementNS(ns,'text');tx.setAttribute('x',pL+pW-4);tx.setAttribute('y',y-4);tx.setAttribute('text-anchor','end');tx.setAttribute('font-size','10');tx.setAttribute('fill',color);tx.textContent=cfg.ls[i];svg.appendChild(tx);}});})()";
const SP_CUT_BARS_JS: &str = "(function(){var cfg=window.__sp_cut_bars__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var ns='http://www.w3.org/2000/svg';var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pH=sp[3]||360;var rects=svg.querySelectorAll('rect[data-idx][data-v]');if(!rects.length)return;var stepPx=cfg.sp>0?cfg.sp:Math.max(6,pH/12);var gapPx=cfg.gp>0?cfg.gp:2;var color=cfg.c||'#ffffff';var g=document.createElementNS(ns,'g');g.id='sp-cut-layer';rects.forEach(function(rect){var bX=parseFloat(rect.getAttribute('x'))||0;var bY=parseFloat(rect.getAttribute('y'))||0;var bW=parseFloat(rect.getAttribute('width'))||0;var bH=parseFloat(rect.getAttribute('height'))||0;var y=bY+stepPx;while(y<bY+bH-1){var r2=document.createElementNS(ns,'rect');r2.setAttribute('x',bX);r2.setAttribute('y',y);r2.setAttribute('width',bW);r2.setAttribute('height',gapPx);r2.setAttribute('fill',color);r2.setAttribute('pointer-events','none');g.appendChild(r2);y+=stepPx;}});svg.appendChild(g);})()";
const SP_DRAW_TOOL_JS: &str = r"(function(){if(window.__sp_draw_init__)return;window.__sp_draw_init__=1;var cfg=window.__sp_draw_cfg__||{},color=cfg.color||'#ff0000',ns='http://www.w3.org/2000/svg';var svg=document.querySelector('svg');if(!svg)return;var cont=svg.parentElement||document.body;if(!cont.style.position||cont.style.position==='static')cont.style.position='relative';var dl=document.createElementNS(ns,'g');dl.id='sp-draw-layer';svg.appendChild(dl);var mode='draw',drawing=false,curPath=null,pts=[],selEls=[],selOvs=[],rxStart=null,activeInp=null,SIZES=[1,1.5,2.5,4,6,9],szIdx=2,tbDrag=false,tbDx=0,tbDy=0,collapsed=false,vert=false;function mkBtn(t,tip){var b=document.createElement('button');b.textContent=t;if(tip)b.title=tip;b.style.cssText='cursor:pointer;min-width:26px;height:26px;border:1px solid rgba(255,255,255,.12);border-radius:5px;font-size:12px;background:rgba(255,255,255,.07);color:#e2e8f0;display:inline-flex;align-items:center;justify-content:center;padding:0 4px;flex-shrink:0';return b;}function mkSep(){var d=document.createElement('div');d.className='sp-dr-sep';d.style.cssText='width:1px;height:20px;background:rgba(255,255,255,.1);flex-shrink:0';return d;}var tb=document.createElement('div');tb.style.cssText='position:absolute;top:6px;right:6px;z-index:9999;background:rgba(10,10,18,.93);border:1px solid rgba(255,255,255,.14);border-radius:9px;font-family:system-ui;box-shadow:0 4px 20px rgba(0,0,0,.5)';var hdr=document.createElement('div');hdr.style.cssText='display:flex;align-items:center;justify-content:space-between;padding:4px 8px;cursor:move;border-bottom:1px solid rgba(255,255,255,.07)';var grip=document.createElement('span');grip.textContent='⋮⋮';grip.style.cssText='color:#334155;font-size:11px;letter-spacing:-2px;pointer-events:none';var btnColl=document.createElement('button');btnColl.textContent='◂';btnColl.title='Collapse';btnColl.style.cssText='cursor:pointer;background:none;border:none;color:#64748b;font-size:13px;padding:0 2px;line-height:1';hdr.appendChild(grip);hdr.appendChild(btnColl);var body=document.createElement('div');body.style.cssText='display:flex;flex-wrap:wrap;gap:5px;padding:7px 8px 8px;align-items:center';var btnDraw=mkBtn('✏','Draw (freehand)');var btnText=mkBtn('T','Text');var btnSel=mkBtn('↖','Select / Move');var btnErase=mkBtn('⌫','Eraser');var ci=document.createElement('input');ci.type='color';ci.value=color;ci.title='Color';ci.style.cssText='width:26px;height:26px;border:1px solid rgba(255,255,255,.2);border-radius:5px;cursor:pointer;padding:1px 2px;background:rgba(255,255,255,.05);flex-shrink:0';ci.addEventListener('input',function(){color=ci.value;});var btnSzM=mkBtn('−','Thinner');var szDisp=document.createElement('span');szDisp.style.cssText='color:#94a3b8;font-size:10px;min-width:26px;text-align:center;flex-shrink:0;user-select:none';var btnSzP=mkBtn('+','Thicker');function updateSz(){szDisp.textContent=SIZES[szIdx]+'px';}updateSz();var btnClr=mkBtn('✕','Clear all');var btnSav=mkBtn('⬇','Save SVG');var btnOri=mkBtn('⇕','Toggle layout');[btnDraw,btnText,btnSel,btnErase,mkSep(),ci,btnSzM,szDisp,btnSzP,mkSep(),btnClr,btnSav,mkSep(),btnOri].forEach(function(el){body.appendChild(el);});tb.appendChild(hdr);tb.appendChild(body);cont.appendChild(tb);function getSP(e){var r=svg.getBoundingClientRect(),vb=svg.viewBox&&svg.viewBox.baseVal,sw=vb&&vb.width?vb.width:parseFloat(svg.getAttribute('width'))||r.width,sh=vb&&vb.height?vb.height:parseFloat(svg.getAttribute('height'))||r.height;return{x:(e.clientX-r.left)*sw/r.width,y:(e.clientY-r.top)*sh/r.height};}function clearSel(){selOvs.forEach(function(o){try{o.parentNode.removeChild(o);}catch(ex){}});selOvs=[];selEls=[];}function addSelOv(el){try{var bb=el.getBBox(),pad=3,r=document.createElementNS(ns,'rect');r.setAttribute('x',bb.x-pad);r.setAttribute('y',bb.y-pad);r.setAttribute('width',Math.max(6,bb.width+pad*2));r.setAttribute('height',Math.max(6,bb.height+pad*2));r.setAttribute('fill','rgba(99,102,241,.12)');r.setAttribute('stroke','#818cf8');r.setAttribute('stroke-width','1.2');r.setAttribute('stroke-dasharray','4 3');r.style.pointerEvents='none';dl.appendChild(r);selOvs.push(r);}catch(ex){}}function selectEl(el){if(selEls.indexOf(el)<0){selEls.push(el);addSelOv(el);}}function refreshOvs(){selOvs.forEach(function(ov,i){if(!selEls[i])return;try{var bb=selEls[i].getBBox(),pad=3;ov.setAttribute('x',bb.x-pad);ov.setAttribute('y',bb.y-pad);ov.setAttribute('width',Math.max(6,bb.width+pad*2));ov.setAttribute('height',Math.max(6,bb.height+pad*2));}catch(ex){}});}var rbRect=document.createElementNS(ns,'rect');rbRect.setAttribute('fill','rgba(99,102,241,.08)');rbRect.setAttribute('stroke','#818cf8');rbRect.setAttribute('stroke-width','1');rbRect.setAttribute('stroke-dasharray','4 2');rbRect.style.display='none';rbRect.style.pointerEvents='none';dl.appendChild(rbRect);function dismissInp(){if(activeInp&&document.body.contains(activeInp)){document.body.removeChild(activeInp);activeInp=null;}}function setMode(m){mode=m;var modes=['draw','text','select','erase'],btns=[btnDraw,btnText,btnSel,btnErase];btns.forEach(function(b,i){b.style.background=modes[i]===m?'rgba(99,102,241,.45)':'rgba(255,255,255,.07)';});svg.style.cursor=m==='draw'||m==='erase'?'crosshair':m==='text'?'text':'default';if(m!=='text')dismissInp();if(m!=='select')clearSel();}function moveEl(el,dx,dy){var tr=el.getAttribute('transform')||'',m=tr.match(/translate\(([^,]+),([^)]+)\)/),ox=m?parseFloat(m[1]):0,oy=m?parseFloat(m[2]):0;el.setAttribute('transform','translate('+(ox+dx)+','+(oy+dy)+')');}function finishRB(){var rx=parseFloat(rbRect.getAttribute('x')),ry=parseFloat(rbRect.getAttribute('y')),rw=parseFloat(rbRect.getAttribute('width')),rh=parseFloat(rbRect.getAttribute('height'));rxStart=null;rbRect.style.display='none';if(rw>3||rh>3){Array.prototype.slice.call(dl.querySelectorAll('[data-sp-draw]')).forEach(function(el){try{var bb=el.getBBox();if(bb.x+bb.width>rx&&bb.x<rx+rw&&bb.y+bb.height>ry&&bb.y<ry+rh){selectEl(el);}}catch(ex){}});}}svg.addEventListener('mousedown',function(e){if(e.button!==0)return;var pt=getSP(e);if(mode==='text'){dismissInp();var inp=document.createElement('input');inp.type='text';inp.style.cssText='position:fixed;left:'+e.clientX+'px;top:'+e.clientY+'px;z-index:99999;font-size:13px;border:1px solid #6366f1;border-radius:3px;background:#0a0f1c;color:'+color+';padding:2px 6px;min-width:80px;outline:none;font-family:system-ui';document.body.appendChild(inp);activeInp=inp;inp.focus();var escaped=false;var commitTx=function(){if(inp.value.trim()){var tx=document.createElementNS(ns,'text');tx.setAttribute('x',pt.x);tx.setAttribute('y',pt.y);tx.setAttribute('font-size','13');tx.setAttribute('fill',color);tx.setAttribute('font-weight','600');tx.textContent=inp.value.trim();tx.setAttribute('data-sp-draw','1');dl.appendChild(tx);}dismissInp();};inp.addEventListener('keydown',function(ke){ke.stopPropagation();if(ke.key==='Enter'){commitTx();}else if(ke.key==='Escape'){escaped=true;dismissInp();}});inp.addEventListener('blur',function(){if(!escaped)setTimeout(function(){if(activeInp===inp)commitTx();},180);});e.preventDefault();e.stopPropagation();return;}if(mode==='select'){var allDr=Array.prototype.slice.call(dl.querySelectorAll('[data-sp-draw]')),hit=null;for(var ii=allDr.length-1;ii>=0;ii--){try{var bb2=allDr[ii].getBBox();if(pt.x>=bb2.x-4&&pt.x<=bb2.x+bb2.width+4&&pt.y>=bb2.y-4&&pt.y<=bb2.y+bb2.height+4){hit=allDr[ii];break;}}catch(ex){}}if(hit){if(!e.shiftKey)clearSel();selectEl(hit);var dp=pt,dmov=false,snap=selEls.slice();var onDm=function(me){var mp=getSP(me),dx=mp.x-dp.x,dy=mp.y-dp.y;if(!dmov&&(Math.abs(dx)>1||Math.abs(dy)>1))dmov=true;if(dmov){snap.forEach(function(el){moveEl(el,dx,dy);});dp=mp;refreshOvs();}};var onDu=function(){document.removeEventListener('mousemove',onDm);document.removeEventListener('mouseup',onDu);};document.addEventListener('mousemove',onDm);document.addEventListener('mouseup',onDu);}else{if(!e.shiftKey)clearSel();rxStart=pt;rbRect.setAttribute('x',pt.x);rbRect.setAttribute('y',pt.y);rbRect.setAttribute('width',0);rbRect.setAttribute('height',0);rbRect.style.display='';}e.preventDefault();e.stopPropagation();return;}if(mode==='erase'){var eraseAt=function(p){Array.prototype.slice.call(dl.querySelectorAll('[data-sp-draw]')).forEach(function(el){try{var bb3=el.getBBox();if(p.x>=bb3.x-6&&p.x<=bb3.x+bb3.width+6&&p.y>=bb3.y-6&&p.y<=bb3.y+bb3.height+6){dl.removeChild(el);}}catch(ex){}});};eraseAt(pt);var erasing=true;var onEm=function(me){if(erasing)eraseAt(getSP(me));};var onEu=function(){erasing=false;document.removeEventListener('mousemove',onEm);document.removeEventListener('mouseup',onEu);};document.addEventListener('mousemove',onEm);document.addEventListener('mouseup',onEu);e.preventDefault();e.stopPropagation();return;}drawing=true;pts=[pt];curPath=document.createElementNS(ns,'path');curPath.setAttribute('d','M'+pt.x+','+pt.y);curPath.setAttribute('stroke',color);curPath.setAttribute('stroke-width',SIZES[szIdx]);curPath.setAttribute('fill','none');curPath.setAttribute('stroke-linecap','round');curPath.setAttribute('stroke-linejoin','round');curPath.setAttribute('data-sp-draw','1');dl.appendChild(curPath);e.preventDefault();e.stopPropagation();});svg.addEventListener('mousemove',function(e){if(mode==='select'&&rxStart){var pt2=getSP(e);rbRect.setAttribute('x',Math.min(rxStart.x,pt2.x));rbRect.setAttribute('y',Math.min(rxStart.y,pt2.y));rbRect.setAttribute('width',Math.abs(pt2.x-rxStart.x));rbRect.setAttribute('height',Math.abs(pt2.y-rxStart.y));return;}if(!drawing||!curPath)return;var pt2=getSP(e);pts.push(pt2);var d='M'+pts[0].x+','+pts[0].y;for(var j=1;j<pts.length;j++){var p0=pts[j-1],p1=pts[j];d+=' Q'+p0.x+','+p0.y+' '+((p0.x+p1.x)/2)+','+((p0.y+p1.y)/2);}curPath.setAttribute('d',d);e.preventDefault();});svg.addEventListener('mouseup',function(){if(rxStart){finishRB();return;}drawing=false;curPath=null;pts=[];});svg.addEventListener('mouseleave',function(){if(rxStart)return;drawing=false;curPath=null;pts=[];});document.addEventListener('mouseup',function(){if(rxStart)finishRB();});document.addEventListener('keydown',function(e){if((e.key==='Delete'||e.key==='Backspace')&&selEls.length&&!activeInp){selEls.forEach(function(el){try{el.parentNode.removeChild(el);}catch(ex){}});clearSel();e.preventDefault();}});document.addEventListener('mousedown',function(e){if(activeInp&&e.target!==activeInp&&!tb.contains(e.target)){dismissInp();}},true);btnDraw.addEventListener('click',function(){setMode('draw');});btnText.addEventListener('click',function(){setMode(mode==='text'?'draw':'text');});btnSel.addEventListener('click',function(){setMode('select');});btnErase.addEventListener('click',function(){setMode('erase');});btnSzM.addEventListener('click',function(){if(szIdx>0){szIdx--;updateSz();}});btnSzP.addEventListener('click',function(){if(szIdx<SIZES.length-1){szIdx++;updateSz();}});btnClr.addEventListener('click',function(){Array.prototype.slice.call(dl.querySelectorAll('[data-sp-draw]')).forEach(function(c){try{dl.removeChild(c);}catch(ex){}});clearSel();});btnSav.addEventListener('click',function(){var s=new XMLSerializer().serializeToString(svg);var a=document.createElement('a');a.href=URL.createObjectURL(new Blob([s],{type:'image/svg+xml'}));a.download='chart-annotated.svg';a.click();});btnOri.addEventListener('click',function(){vert=!vert;btnOri.textContent=vert?'⇔':'⇕';body.style.flexDirection=vert?'column':'row';body.style.flexWrap=vert?'nowrap':'wrap';var seps=body.querySelectorAll('.sp-dr-sep');Array.prototype.forEach.call(seps,function(s){s.style.width=vert?'100%':'1px';s.style.height=vert?'1px':'20px';});});btnColl.addEventListener('click',function(){collapsed=!collapsed;body.style.display=collapsed?'none':'flex';btnColl.textContent=collapsed?'▸':'◂';});hdr.addEventListener('mousedown',function(e){if(e.target===btnColl)return;tbDrag=true;var r=tb.getBoundingClientRect();tbDx=e.clientX-r.left;tbDy=e.clientY-r.top;e.preventDefault();});document.addEventListener('mousemove',function(e){if(!tbDrag)return;var pr=cont.getBoundingClientRect();tb.style.left=Math.max(0,e.clientX-pr.left-tbDx)+'px';tb.style.top=Math.max(0,e.clientY-pr.top-tbDy)+'px';tb.style.right='auto';});document.addEventListener('mouseup',function(){tbDrag=false;});setMode('draw');})()";
const SP_HIGHLIGHT_STATIC_JS: &str = "(function(){var cfg=window.__sp_hl_grp__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var all=Array.prototype.slice.call(svg.querySelectorAll('[data-lbl],[data-group]'));if(!all.length)return;all.forEach(function(el){var lbl=el.getAttribute('data-lbl')||el.getAttribute('data-group')||'';if(cfg.g.indexOf(lbl)<0){el.style.opacity=String(cfg.d);el.style.transition='opacity .18s';}});})()";

const SP_STACK_INIT_JS: &str = "window.__spStackTop=window.__spStackTop||{};window.__spStackClaim=window.__spStackClaim||function(i,h){var t=window.__spStackTop,c=t[i]||0;t[i]=c+h;return c;};";

const SP_VALUE_LABELS_JS: &str = "(function(){var cfg=window.__sp_value_labels__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var rects=svg.querySelectorAll('rect[data-idx][data-v]');var ns='http://www.w3.org/2000/svg';rects.forEach(function(r){var v=parseFloat(r.getAttribute('data-v'));if(isNaN(v))return;var idx=r.getAttribute('data-idx');var prior=window.__spStackClaim(idx,16);var x=parseFloat(r.getAttribute('x'))+parseFloat(r.getAttribute('width'))/2;var y=parseFloat(r.getAttribute('y'))-prior-4;var t=document.createElementNS(ns,'text');t.setAttribute('x',x);t.setAttribute('y',y);t.setAttribute('text-anchor','middle');t.setAttribute('font-size','11');t.setAttribute('font-weight','700');t.setAttribute('fill',cfg.c);t.textContent=v.toFixed(cfg.d);svg.appendChild(t);});})()";

const SP_ERROR_BARS_JS: &str = "(function(){var cfg=window.__sp_error_bars__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pT=sp[1]||36,pH=sp[3]||360;var rects=svg.querySelectorAll('rect[data-idx][data-v]');if(!rects.length)return;var maxV=0;rects.forEach(function(r){var v=parseFloat(r.getAttribute('data-v'))||0;if(v>maxV)maxV=v;});if(maxV<=0)return;var mPx=cfg.m/maxV*pH;var ns='http://www.w3.org/2000/svg';rects.forEach(function(r){var idx=r.getAttribute('data-idx');var rectY=parseFloat(r.getAttribute('y'));var claimH=2*mPx+6;var prior=window.__spStackClaim(idx,claimH);var x=parseFloat(r.getAttribute('x'))+parseFloat(r.getAttribute('width'))/2;var y1=Math.min(rectY,rectY-prior-3);var y0=Math.max(pT,y1-2*mPx);if(y0>=y1)y0=Math.max(pT,y1-4);var ln=document.createElementNS(ns,'line');ln.setAttribute('x1',x);ln.setAttribute('x2',x);ln.setAttribute('y1',y0);ln.setAttribute('y2',y1);ln.setAttribute('stroke',cfg.c);ln.setAttribute('stroke-width','1.6');svg.appendChild(ln);[y0,y1].forEach(function(yy){var cap=document.createElementNS(ns,'line');cap.setAttribute('x1',x-5);cap.setAttribute('x2',x+5);cap.setAttribute('y1',yy);cap.setAttribute('y2',yy);cap.setAttribute('stroke',cfg.c);cap.setAttribute('stroke-width','1.6');svg.appendChild(cap);});});})()";

const SP_DELTA_LABELS_JS: &str = "(function(){var cfg=window.__sp_delta_labels__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));rects.sort(function(a,b){return parseInt(a.getAttribute('data-idx'))-parseInt(b.getAttribute('data-idx'));});var ns='http://www.w3.org/2000/svg';for(var i=1;i<rects.length;i++){var v0=parseFloat(rects[i-1].getAttribute('data-v')),v1=parseFloat(rects[i].getAttribute('data-v'));if(!v0)continue;var pct=(v1-v0)/Math.abs(v0)*100;var col=pct>=0?cfg.pc:cfg.nc;var idx=rects[i].getAttribute('data-idx');var prior=window.__spStackClaim(idx,15);var x=parseFloat(rects[i].getAttribute('x'))+parseFloat(rects[i].getAttribute('width'))/2;var y=parseFloat(rects[i].getAttribute('y'))-prior-4;var t=document.createElementNS(ns,'text');t.setAttribute('x',x);t.setAttribute('y',y);t.setAttribute('text-anchor','middle');t.setAttribute('font-size','10.5');t.setAttribute('font-weight','700');t.setAttribute('fill',col);t.textContent=(pct>=0?'+':'')+pct.toFixed(1)+'%';svg.appendChild(t);}})()";

const SP_CUMULATIVE_LINE_JS: &str = "(function(){var cfg=window.__sp_cumulative_line__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pT=sp[1]||36,pH=sp[3]||360;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));rects.sort(function(a,b){return parseInt(a.getAttribute('data-idx'))-parseInt(b.getAttribute('data-idx'));});if(!rects.length)return;var total=0;rects.forEach(function(r){total+=parseFloat(r.getAttribute('data-v'))||0;});if(total<=0)return;var ns='http://www.w3.org/2000/svg';var run=0,pts=[];rects.forEach(function(r){run+=parseFloat(r.getAttribute('data-v'))||0;var x=parseFloat(r.getAttribute('x'))+parseFloat(r.getAttribute('width'))/2;var y=pT+pH-(run/total)*pH;pts.push(x+','+y);});var halo=document.createElementNS(ns,'polyline');halo.setAttribute('points',pts.join(' '));halo.setAttribute('fill','none');halo.setAttribute('stroke','#fff');halo.setAttribute('stroke-width','5');halo.setAttribute('stroke-linecap','round');halo.setAttribute('stroke-linejoin','round');halo.setAttribute('opacity','0.85');svg.appendChild(halo);var pl=document.createElementNS(ns,'polyline');pl.setAttribute('points',pts.join(' '));pl.setAttribute('fill','none');pl.setAttribute('stroke',cfg.c);pl.setAttribute('stroke-width','2');pl.setAttribute('stroke-linecap','round');pl.setAttribute('stroke-linejoin','round');svg.appendChild(pl);pts.forEach(function(p){var xy=p.split(',');var c=document.createElementNS(ns,'circle');c.setAttribute('cx',xy[0]);c.setAttribute('cy',xy[1]);c.setAttribute('r','3.5');c.setAttribute('fill','#fff');svg.appendChild(c);var c2=document.createElementNS(ns,'circle');c2.setAttribute('cx',xy[0]);c2.setAttribute('cy',xy[1]);c2.setAttribute('r','3');c2.setAttribute('fill',cfg.c);svg.appendChild(c2);});})()";

const SP_RANK_BADGES_JS: &str = "(function(){var cfg=window.__sp_rank_badges__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pT=sp[1]||36;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));var top=rects.slice().sort(function(a,b){return parseFloat(b.getAttribute('data-v'))-parseFloat(a.getAttribute('data-v'));}).slice(0,cfg.n);var ns='http://www.w3.org/2000/svg';top.forEach(function(r,i){var idx=r.getAttribute('data-idx');var prior=window.__spStackClaim(idx,26);var x=parseFloat(r.getAttribute('x'))+parseFloat(r.getAttribute('width'))/2;var y=Math.max(12,parseFloat(r.getAttribute('y'))-prior-12);var g=document.createElementNS(ns,'g');var circ=document.createElementNS(ns,'circle');circ.setAttribute('cx',x);circ.setAttribute('cy',y);circ.setAttribute('r','10');circ.setAttribute('fill',cfg.c);circ.setAttribute('stroke','#fff');circ.setAttribute('stroke-width','1.5');g.appendChild(circ);var t=document.createElementNS(ns,'text');t.setAttribute('x',x);t.setAttribute('y',y+1);t.setAttribute('text-anchor','middle');t.setAttribute('dominant-baseline','middle');t.setAttribute('font-size','10');t.setAttribute('font-weight','800');t.setAttribute('fill','#fff');t.textContent=String(i+1);g.appendChild(t);svg.appendChild(g);});})()";

const SP_LOG_SCALE_JS: &str = "(function(){if(window.__splg__)return;window.__splg__=1;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pT=sp[1]||36,pH=sp[3]||360;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));if(!rects.length)return;var minV=Infinity,maxV=0;rects.forEach(function(r){var v=parseFloat(r.getAttribute('data-v'))||0;if(v>0&&v<minV)minV=v;if(v>maxV)maxV=v;});if(!isFinite(minV)||maxV<=0||minV>=maxV)return;var lmin=Math.log10(minV),lmax=Math.log10(maxV),lr=lmax-lmin||1;rects.forEach(function(r){var v=parseFloat(r.getAttribute('data-v'))||0;var lv=v>0?Math.log10(v):lmin;var f=(lv-lmin)/lr;var nh=Math.max(1,f*pH);r.setAttribute('y',pT+pH-nh);r.setAttribute('height',nh);});var yts=svg.querySelectorAll('.sp-yt');var nT=yts.length;for(var j=0;j<nT;j++){var fj=nT>1?j/(nT-1):0;var vj=Math.pow(10,lmin+fj*lr);yts[j].setAttribute('y',pT+pH-fj*pH+4);yts[j].textContent=vj>=1000?Math.round(vj).toString():(+vj).toPrecision(2);}})()";

const SP_MOVING_AVG_JS: &str = "(function(){var cfg=window.__sp_moving_avg__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));rects.sort(function(a,b){return parseInt(a.getAttribute('data-idx'))-parseInt(b.getAttribute('data-idx'));});if(rects.length<2)return;var vals=rects.map(function(r){return parseFloat(r.getAttribute('data-v'))||0;});var w=Math.max(1,cfg.w);var ns='http://www.w3.org/2000/svg';var pts=[];for(var i=0;i<rects.length;i++){var lo=Math.max(0,i-w+1),sum=0,cnt=0;for(var k=lo;k<=i;k++){sum+=vals[k];cnt++;}var avg=sum/cnt;var maxV=0;vals.forEach(function(v){if(v>maxV)maxV=v;});var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pT=sp[1]||36,pH=sp[3]||360;var y=pT+pH-(avg/maxV)*pH;var x=parseFloat(rects[i].getAttribute('x'))+parseFloat(rects[i].getAttribute('width'))/2;pts.push(x+','+y);}var halo=document.createElementNS(ns,'polyline');halo.setAttribute('points',pts.join(' '));halo.setAttribute('fill','none');halo.setAttribute('stroke','#fff');halo.setAttribute('stroke-width','5.5');halo.setAttribute('stroke-linecap','round');halo.setAttribute('stroke-linejoin','round');halo.setAttribute('opacity','0.85');svg.appendChild(halo);var pl=document.createElementNS(ns,'polyline');pl.setAttribute('points',pts.join(' '));pl.setAttribute('fill','none');pl.setAttribute('stroke',cfg.c);pl.setAttribute('stroke-width','2.2');pl.setAttribute('stroke-linecap','round');pl.setAttribute('stroke-linejoin','round');pl.setAttribute('stroke-dasharray','0');svg.appendChild(pl);})()";

const SP_OUTLIERS_JS: &str = "(function(){var cfg=window.__sp_outliers__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var ns='http://www.w3.org/2000/svg';function markerAt(el){var tag=el.tagName.toLowerCase();var x,y;if(tag==='circle'){x=parseFloat(el.getAttribute('cx'));y=parseFloat(el.getAttribute('cy'))-(parseFloat(el.getAttribute('r'))||4)-8;}else if(el.hasAttribute('x')&&el.hasAttribute('width')){x=parseFloat(el.getAttribute('x'))+parseFloat(el.getAttribute('width'))/2;y=parseFloat(el.getAttribute('y'))-8;}else{try{var bb=el.getBBox();x=bb.x+bb.width/2;y=bb.y-8;}catch(ex){return;}}var t=document.createElementNS(ns,'text');t.setAttribute('x',x);t.setAttribute('y',y);t.setAttribute('text-anchor','middle');t.setAttribute('font-size','13');t.setAttribute('font-weight','800');t.setAttribute('fill',cfg.c);t.textContent='\\u26a0';svg.appendChild(t);}function flag(el){el.style.setProperty('fill',cfg.c,'important');el.style.setProperty('stroke',cfg.c,'important');el.style.setProperty('stroke-width','2','important');markerAt(el);}var vEls=Array.prototype.slice.call(svg.querySelectorAll('[data-idx][data-v]'));var xyEls=Array.prototype.slice.call(svg.querySelectorAll('[data-idx][data-x][data-y]'));if(vEls.length>=2){var vals=vEls.map(function(r){return parseFloat(r.getAttribute('data-v'))||0;});var n=vals.length,mean=0;vals.forEach(function(v){mean+=v;});mean/=n;var variance=0;vals.forEach(function(v){variance+=(v-mean)*(v-mean);});variance/=n;var std=Math.sqrt(variance);if(std>0){vEls.forEach(function(el,i){if(Math.abs(vals[i]-mean)/std>cfg.t)flag(el);});}}else if(xyEls.length>=3){var xs=xyEls.map(function(p){return parseFloat(p.getAttribute('data-x'))||0;});var ys=xyEls.map(function(p){return parseFloat(p.getAttribute('data-y'))||0;});var n2=xs.length,sx=0,sy=0;for(var i=0;i<n2;i++){sx+=xs[i];sy+=ys[i];}var mx=sx/n2,my=sy/n2,num=0,den=0;for(var j=0;j<n2;j++){var dx=xs[j]-mx;num+=dx*(ys[j]-my);den+=dx*dx;}var slope=den>0?num/den:0,intercept=my-slope*mx;var resid=xs.map(function(x,i){return ys[i]-(slope*x+intercept);});var rm=0;resid.forEach(function(r){rm+=r;});rm/=n2;var rv=0;resid.forEach(function(r){rv+=(r-rm)*(r-rm);});rv/=n2;var rs=Math.sqrt(rv)||1;xyEls.forEach(function(el,i){if(Math.abs(resid[i]-rm)/rs>cfg.t)flag(el);});}})()";

const SP_FILL_BETWEEN_JS: &str = "(function(){var cfg=window.__sp_fill_between__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var pl=svg.querySelector('polyline');if(!pl)return;var pts=(pl.getAttribute('points')||'').trim();if(!pts)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pT=sp[1]||36,pH=sp[3]||360;var base=pT+pH;var parts=pts.split(/\\s+/);var first=parts[0].split(',')[0],last=parts[parts.length-1].split(',')[0];var ns='http://www.w3.org/2000/svg';var poly=document.createElementNS(ns,'polygon');poly.setAttribute('points',pts+' '+last+','+base+' '+first+','+base);poly.setAttribute('fill',cfg.c);poly.setAttribute('opacity',cfg.op);poly.setAttribute('stroke','none');svg.insertBefore(poly,pl);})()";

const SP_BOX_ANNOTATE_JS: &str = "(function(){var cfg=window.__sp_box_annotate__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var rects=svg.querySelectorAll('rect[data-idx][data-v]');if(!rects.length)return;var vals=[];rects.forEach(function(r){vals.push(parseFloat(r.getAttribute('data-v'))||0);});vals.sort(function(a,b){return a-b;});var maxV=vals[vals.length-1];if(maxV<=0)return;function q(p){var idx=p*(vals.length-1),lo=Math.floor(idx),hi=Math.ceil(idx);if(lo===hi)return vals[lo];return vals[lo]+(vals[hi]-vals[lo])*(idx-lo);}var mn=vals[0],q1=q(0.25),md=q(0.5),q3=q(0.75),mx=vals[vals.length-1];function yOf(v){return pT+pH-(v/maxV)*pH;}var ns='http://www.w3.org/2000/svg';var bx=pL+pW+24,bw=18;var g=document.createElementNS(ns,'g');function ln(x1,y1,x2,y2,w){var l=document.createElementNS(ns,'line');l.setAttribute('x1',x1);l.setAttribute('y1',y1);l.setAttribute('x2',x2);l.setAttribute('y2',y2);l.setAttribute('stroke',cfg.c);l.setAttribute('stroke-width',w);g.appendChild(l);}ln(bx,yOf(mn),bx,yOf(q1),1.5);ln(bx,yOf(q3),bx,yOf(mx),1.5);var box=document.createElementNS(ns,'rect');box.setAttribute('x',bx-bw/2);box.setAttribute('y',yOf(q3));box.setAttribute('width',bw);box.setAttribute('height',Math.max(1,yOf(q1)-yOf(q3)));box.setAttribute('fill',cfg.c);box.setAttribute('opacity','0.18');box.setAttribute('stroke',cfg.c);box.setAttribute('stroke-width','1.5');g.appendChild(box);ln(bx-bw/2,yOf(md),bx+bw/2,yOf(md),2);ln(bx-bw/3,yOf(mn),bx+bw/3,yOf(mn),1.5);ln(bx-bw/3,yOf(mx),bx+bw/3,yOf(mx),1.5);svg.appendChild(g);var vb=(svg.getAttribute('viewBox')||'').split(' ').map(Number);if(vb.length===4&&bx+30>vb[2]){var nw=bx+34;svg.setAttribute('viewBox','0 0 '+nw+' '+vb[3]);svg.setAttribute('width',nw);}})()";

const SP_PCT_OF_TOTAL_JS: &str = "(function(){var cfg=window.__sp_pct_of_total__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var rects=svg.querySelectorAll('rect[data-idx][data-v]');if(!rects.length)return;var total=0;rects.forEach(function(r){total+=parseFloat(r.getAttribute('data-v'))||0;});if(total<=0)return;var ns='http://www.w3.org/2000/svg';rects.forEach(function(r){var v=parseFloat(r.getAttribute('data-v'))||0;var idx=r.getAttribute('data-idx');var prior=window.__spStackClaim(idx,16);var pct=v/total*100;var x=parseFloat(r.getAttribute('x'))+parseFloat(r.getAttribute('width'))/2;var y=parseFloat(r.getAttribute('y'))-prior-4;var t=document.createElementNS(ns,'text');t.setAttribute('x',x);t.setAttribute('y',y);t.setAttribute('text-anchor','middle');t.setAttribute('font-size','11');t.setAttribute('font-weight','700');t.setAttribute('fill',cfg.c);t.textContent=pct.toFixed(cfg.d)+'%';svg.appendChild(t);});})()";

const SP_CORRELATION_BADGE_JS: &str = "(function(){var cfg=window.__sp_correlation_badge__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var pts=Array.prototype.slice.call(svg.querySelectorAll('circle[data-idx][data-x][data-y]'));if(pts.length<2)return;var xs=pts.map(function(p){return parseFloat(p.getAttribute('data-x'))||0;});var ys=pts.map(function(p){return parseFloat(p.getAttribute('data-y'))||0;});var n=xs.length,sx=0,sy=0;for(var i=0;i<n;i++){sx+=xs[i];sy+=ys[i];}var mx=sx/n,my=sy/n,num=0,dxx=0,dyy=0;for(var j=0;j<n;j++){var dx=xs[j]-mx,dy=ys[j]-my;num+=dx*dy;dxx+=dx*dx;dyy+=dy*dy;}var den=Math.sqrt(dxx*dyy);var r=den>0?num/den:0;var ns='http://www.w3.org/2000/svg';var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700;var bw=86,bh=30,bx=pL+pW-bw-6,by=pT+6;var g=document.createElementNS(ns,'g');var box=document.createElementNS(ns,'rect');box.setAttribute('x',bx);box.setAttribute('y',by);box.setAttribute('width',bw);box.setAttribute('height',bh);box.setAttribute('rx','7');box.setAttribute('fill',cfg.c);box.setAttribute('opacity','0.12');box.setAttribute('stroke',cfg.c);box.setAttribute('stroke-width','1.2');g.appendChild(box);var t=document.createElementNS(ns,'text');t.setAttribute('x',bx+bw/2);t.setAttribute('y',by+bh/2+4);t.setAttribute('text-anchor','middle');t.setAttribute('font-size','13');t.setAttribute('font-weight','800');t.setAttribute('fill',cfg.c);t.textContent='r = '+r.toFixed(2);g.appendChild(t);svg.appendChild(g);})()";

const SP_HIGHLIGHT_RANGE_JS: &str = "(function(){var cfg=window.__sp_highlight_range__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pT=sp[1]||36,pH=sp[3]||360;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));if(!rects.length)return;var lo=Math.min(cfg.lo,cfg.hi),hi=Math.max(cfg.lo,cfg.hi);var ns='http://www.w3.org/2000/svg';var minX=Infinity,maxX=-Infinity;rects.forEach(function(r){var idx=parseInt(r.getAttribute('data-idx'));if(idx<lo||idx>hi)return;var x=parseFloat(r.getAttribute('x')),w=parseFloat(r.getAttribute('width'));if(x<minX)minX=x;if(x+w>maxX)maxX=x+w;});if(!isFinite(minX))return;var rect=document.createElementNS(ns,'rect');rect.setAttribute('x',minX-4);rect.setAttribute('y',pT);rect.setAttribute('width',maxX-minX+8);rect.setAttribute('height',pH);rect.setAttribute('fill',cfg.c);rect.setAttribute('opacity',cfg.op);var first=svg.querySelector('rect[data-idx]');svg.insertBefore(rect,first);})()";

const SP_IQR_BAND_JS: &str = "(function(){var cfg=window.__sp_iqr_band__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var rects=svg.querySelectorAll('rect[data-idx][data-v]');if(!rects.length)return;var vals=[];rects.forEach(function(r){vals.push(parseFloat(r.getAttribute('data-v'))||0);});vals.sort(function(a,b){return a-b;});var maxV=vals[vals.length-1];if(maxV<=0)return;function q(p){var idx=p*(vals.length-1),lo=Math.floor(idx),hi=Math.ceil(idx);if(lo===hi)return vals[lo];return vals[lo]+(vals[hi]-vals[lo])*(idx-lo);}var q1=q(0.25),q3=q(0.75);var y1=pT+pH-(q3/maxV)*pH,y2=pT+pH-(q1/maxV)*pH;var ns='http://www.w3.org/2000/svg';var rect=document.createElementNS(ns,'rect');rect.setAttribute('x',pL);rect.setAttribute('width',pW);rect.setAttribute('y',y1);rect.setAttribute('height',Math.max(1,y2-y1));rect.setAttribute('fill',cfg.c);rect.setAttribute('opacity',cfg.op);var first=svg.querySelector('rect[data-idx]');svg.insertBefore(rect,first);})()";

const SP_GROWTH_BADGE_JS: &str = "(function(){var cfg=window.__sp_growth_badge__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));if(rects.length<2)return;rects.sort(function(a,b){return parseInt(a.getAttribute('data-idx'))-parseInt(b.getAttribute('data-idx'));});var v0=parseFloat(rects[0].getAttribute('data-v'))||0,v1=parseFloat(rects[rects.length-1].getAttribute('data-v'))||0;if(v0===0)return;var pct=(v1-v0)/Math.abs(v0)*100;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700;var ns='http://www.w3.org/2000/svg';var bw=110,bh=30,bx=pL+pW-bw-6,by=pT+6;var g=document.createElementNS(ns,'g');var box=document.createElementNS(ns,'rect');box.setAttribute('x',bx);box.setAttribute('y',by);box.setAttribute('width',bw);box.setAttribute('height',bh);box.setAttribute('rx','7');box.setAttribute('fill',cfg.c);box.setAttribute('opacity','0.12');box.setAttribute('stroke',cfg.c);box.setAttribute('stroke-width','1.2');g.appendChild(box);var t=document.createElementNS(ns,'text');t.setAttribute('x',bx+bw/2);t.setAttribute('y',by+bh/2+4);t.setAttribute('text-anchor','middle');t.setAttribute('font-size','13');t.setAttribute('font-weight','800');t.setAttribute('fill',cfg.c);t.textContent=(pct>=0?'+':'')+pct.toFixed(1)+'% total';g.appendChild(t);svg.appendChild(g);})()";

const SP_ZSCORE_HEAT_JS: &str = "(function(){if(!window.__sp_zscore_heat__)return;var svg=document.querySelector('svg');if(!svg)return;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));if(rects.length<2)return;var vals=rects.map(function(r){return parseFloat(r.getAttribute('data-v'))||0;});var n=vals.length,mean=0;vals.forEach(function(v){mean+=v;});mean/=n;var variance=0;vals.forEach(function(v){variance+=(v-mean)*(v-mean);});variance/=n;var std=Math.sqrt(variance)||1;rects.forEach(function(r,i){var z=(vals[i]-mean)/std;var t=Math.max(-2,Math.min(2,z))/2;var col;if(t>=0){var k=Math.round(t*255);col='rgb(255,'+(255-k)+','+(255-k)+')';}else{var k2=Math.round(-t*255);col='rgb('+(255-k2)+','+(255-k2)+',255)';}r.style.setProperty('fill',col,'important');});})()";

const SP_PARETO_MARKER_JS: &str = "(function(){var cfg=window.__sp_pareto_marker__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pT=sp[1]||36,pH=sp[3]||360;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));var sorted=rects.slice().sort(function(a,b){return parseFloat(b.getAttribute('data-v'))-parseFloat(a.getAttribute('data-v'));});if(!sorted.length)return;var total=0;sorted.forEach(function(r){total+=parseFloat(r.getAttribute('data-v'))||0;});if(total<=0)return;var run=0,mark=null;for(var i=0;i<sorted.length;i++){run+=parseFloat(sorted[i].getAttribute('data-v'))||0;if(run/total*100>=cfg.t){mark=sorted[i];break;}}if(!mark)return;var x=parseFloat(mark.getAttribute('x'))+parseFloat(mark.getAttribute('width'));var ns='http://www.w3.org/2000/svg';var ln=document.createElementNS(ns,'line');ln.setAttribute('x1',x);ln.setAttribute('x2',x);ln.setAttribute('y1',pT);ln.setAttribute('y2',pT+pH);ln.setAttribute('stroke',cfg.c);ln.setAttribute('stroke-width','1.5');ln.setAttribute('stroke-dasharray','5,4');svg.appendChild(ln);var t=document.createElementNS(ns,'text');t.setAttribute('x',x+4);t.setAttribute('y',pT+10);t.setAttribute('font-size','10.5');t.setAttribute('font-weight','700');t.setAttribute('fill',cfg.c);t.textContent=cfg.t.toFixed(0)+'%';svg.appendChild(t);})()";

const SP_DIFF_FROM_MEAN_JS: &str = "(function(){var cfg=window.__sp_diff_from_mean__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));if(rects.length<2)return;var vals=rects.map(function(r){return parseFloat(r.getAttribute('data-v'))||0;});var mean=0;vals.forEach(function(v){mean+=v;});mean/=vals.length;var ns='http://www.w3.org/2000/svg';rects.forEach(function(r,i){var idx=r.getAttribute('data-idx');var prior=window.__spStackClaim(idx,15);var diff=vals[i]-mean;var col=diff>=0?cfg.pc:cfg.nc;var x=parseFloat(r.getAttribute('x'))+parseFloat(r.getAttribute('width'))/2;var y=parseFloat(r.getAttribute('y'))-prior-4;var t=document.createElementNS(ns,'text');t.setAttribute('x',x);t.setAttribute('y',y);t.setAttribute('text-anchor','middle');t.setAttribute('font-size','10.5');t.setAttribute('font-weight','700');t.setAttribute('fill',col);t.textContent=(diff>=0?'+':'')+diff.toFixed(1);svg.appendChild(t);});})()";

const SP_ROLLING_STD_BAND_JS: &str = "(function(){var cfg=window.__sp_rolling_std_band__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var els=Array.prototype.slice.call(svg.querySelectorAll('[data-idx][data-v]'));els.sort(function(a,b){return parseInt(a.getAttribute('data-idx'))-parseInt(b.getAttribute('data-idx'));});if(els.length<2)return;var vals=els.map(function(r){return parseFloat(r.getAttribute('data-v'))||0;});var w=Math.max(1,cfg.w);var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pT=sp[1]||36,pH=sp[3]||360;var maxV=0;vals.forEach(function(v){if(v>maxV)maxV=v;});if(maxV<=0)return;function xOf(el){return parseFloat(el.getAttribute('x'))+parseFloat(el.getAttribute('width'))/2;}function yOf(v){return pT+pH-(v/maxV)*pH;}var upPts=[],dnPts=[];for(var i=0;i<els.length;i++){var lo=Math.max(0,i-w+1),sum=0,cnt=0;for(var k=lo;k<=i;k++){sum+=vals[k];cnt++;}var mean=sum/cnt;var sq=0;for(var k2=lo;k2<=i;k2++){sq+=(vals[k2]-mean)*(vals[k2]-mean);}var sd=Math.sqrt(sq/cnt);var x=xOf(els[i]);upPts.push(x+','+yOf(mean+sd));dnPts.push(x+','+yOf(mean-sd));}var pts=upPts.concat(dnPts.reverse());var ns='http://www.w3.org/2000/svg';var poly=document.createElementNS(ns,'polygon');poly.setAttribute('points',pts.join(' '));poly.setAttribute('fill',cfg.c);poly.setAttribute('opacity',cfg.op);var first=svg.querySelector('[data-idx]');svg.insertBefore(poly,first);})()";

const SP_FORECAST_LINE_JS: &str = "(function(){var cfg=window.__sp_forecast_line__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var els=Array.prototype.slice.call(svg.querySelectorAll('[data-idx][data-v]'));els.sort(function(a,b){return parseInt(a.getAttribute('data-idx'))-parseInt(b.getAttribute('data-idx'));});if(els.length<2)return;var vals=els.map(function(r){return parseFloat(r.getAttribute('data-v'))||0;});var n=vals.length,sx=0,sy=0;for(var i=0;i<n;i++){sx+=i;sy+=vals[i];}var mx=sx/n,my=sy/n,num=0,den=0;for(var j=0;j<n;j++){var dx=j-mx;num+=dx*(vals[j]-my);den+=dx*dx;}var slope=den>0?num/den:0,intercept=my-slope*mx;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pT=sp[1]||36,pH=sp[3]||360;var maxV=0;vals.forEach(function(v){if(v>maxV)maxV=v;});if(maxV<=0)return;var fvals=[];for(var p=1;p<=cfg.n;p++)fvals.push(slope*(n-1+p)+intercept);var maxF=Math.max.apply(null,fvals);if(maxF>maxV){var newMax=maxF*1.08;els.forEach(function(el,i){var nh=vals[i]/newMax*pH;el.setAttribute('y',pT+pH-nh);el.setAttribute('height',nh);});var yts=svg.querySelectorAll('.sp-yt');var nT=yts.length;for(var t=0;t<nT;t++){var f=nT>1?t/(nT-1):0;var v2=f*newMax;yts[t].setAttribute('y',pT+pH-(f*pH)+4);yts[t].textContent=v2>=1000?Math.round(v2).toString():(+v2).toFixed(2);}maxV=newMax;}var last=els[n-1];var x0=parseFloat(last.getAttribute('x'))+parseFloat(last.getAttribute('width'))/2;var step=20;if(n>1){var prev=els[n-2];step=x0-(parseFloat(prev.getAttribute('x'))+parseFloat(prev.getAttribute('width'))/2);}var y0=pT+pH-(vals[n-1]/maxV)*pH;var ns='http://www.w3.org/2000/svg';var pts=[x0+','+y0];for(var p2=0;p2<fvals.length;p2++){var x=x0+step*(p2+1);var y=pT+pH-(fvals[p2]/maxV)*pH;pts.push(x+','+y);}var pl=document.createElementNS(ns,'polyline');pl.setAttribute('points',pts.join(' '));pl.setAttribute('fill','none');pl.setAttribute('stroke',cfg.c);pl.setAttribute('stroke-width','2');pl.setAttribute('stroke-dasharray','6,4');pl.setAttribute('stroke-linecap','round');svg.appendChild(pl);for(var k=1;k<pts.length;k++){var xy=pts[k].split(',');var c=document.createElementNS(ns,'circle');c.setAttribute('cx',xy[0]);c.setAttribute('cy',xy[1]);c.setAttribute('r','3');c.setAttribute('fill',cfg.c);c.setAttribute('opacity','0.7');svg.appendChild(c);}})()";

const SP_PERCENTILE_BAND_JS: &str = "(function(){var cfg=window.__sp_percentile_band__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var rects=svg.querySelectorAll('[data-idx][data-v]');if(!rects.length)return;var vals=[];rects.forEach(function(r){vals.push(parseFloat(r.getAttribute('data-v'))||0);});vals.sort(function(a,b){return a-b;});var maxV=vals[vals.length-1];if(maxV<=0)return;function q(p){var idx=p/100*(vals.length-1),lo=Math.floor(idx),hi=Math.ceil(idx);if(lo===hi)return vals[lo];return vals[lo]+(vals[hi]-vals[lo])*(idx-lo);}var lo=q(cfg.lo),hi=q(cfg.hi);var y1=pT+pH-(hi/maxV)*pH,y2=pT+pH-(lo/maxV)*pH;var ns='http://www.w3.org/2000/svg';var rect=document.createElementNS(ns,'rect');rect.setAttribute('x',pL);rect.setAttribute('width',pW);rect.setAttribute('y',y1);rect.setAttribute('height',Math.max(1,y2-y1));rect.setAttribute('fill',cfg.c);rect.setAttribute('opacity',cfg.op);var first=svg.querySelector('[data-idx]');svg.insertBefore(rect,first);})()";

const SP_SCATTER_REGRESSION_JS: &str = "(function(){var cfg=window.__sp_scatter_regression__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var pts=Array.prototype.slice.call(svg.querySelectorAll('circle[data-idx][data-x][data-y]'));if(pts.length<2)return;var xs=pts.map(function(p){return parseFloat(p.getAttribute('data-x'))||0;});var ys=pts.map(function(p){return parseFloat(p.getAttribute('data-y'))||0;});var n=xs.length,sx=0,sy=0;for(var i=0;i<n;i++){sx+=xs[i];sy+=ys[i];}var mx=sx/n,my=sy/n,num=0,den=0;for(var j=0;j<n;j++){var dx=xs[j]-mx;num+=dx*(ys[j]-my);den+=dx*dx;}var slope=den>0?num/den:0,intercept=my-slope*mx;var cxs=pts.map(function(p){return parseFloat(p.getAttribute('cx'))||0;});var cys=pts.map(function(p){return parseFloat(p.getAttribute('cy'))||0;});var xA=xs[0],xB=xs[n-1];if(xA===xB)xB=xs[1];var ax=(cxs[n-1]-cxs[0])/((xB-xA)||1),bx=cxs[0]-ax*xA;var yA=ys[0],yB=ys[n-1];if(yA===yB)yB=ys[1];var ay=(cys[n-1]-cys[0])/((yB-yA)||1),by=cys[0]-ay*yA;var x0=Math.min.apply(null,xs),x1=Math.max.apply(null,xs);var y0=slope*x0+intercept,y1=slope*x1+intercept;var cx0=ax*x0+bx,cy0=ay*y0+by,cx1=ax*x1+bx,cy1=ay*y1+by;var ns='http://www.w3.org/2000/svg';var halo=document.createElementNS(ns,'line');halo.setAttribute('x1',cx0);halo.setAttribute('y1',cy0);halo.setAttribute('x2',cx1);halo.setAttribute('y2',cy1);halo.setAttribute('stroke','#fff');halo.setAttribute('stroke-width',(cfg.w+3));halo.setAttribute('opacity','0.85');svg.appendChild(halo);var ln=document.createElementNS(ns,'line');ln.setAttribute('x1',cx0);ln.setAttribute('y1',cy0);ln.setAttribute('x2',cx1);ln.setAttribute('y2',cy1);ln.setAttribute('stroke',cfg.c);ln.setAttribute('stroke-width',cfg.w);ln.setAttribute('stroke-linecap','round');svg.appendChild(ln);})()";


const SP_CLUSTER_JS: &str = "(function(){var cfg=window.__sp_cluster__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var pts=Array.prototype.slice.call(svg.querySelectorAll('[data-idx][data-x][data-y]'));var oneD=false;if(pts.length<2){pts=Array.prototype.slice.call(svg.querySelectorAll('[data-idx][data-v]'));oneD=true;}if(pts.length<2)return;var P=oneD?pts.map(function(p){return{x:parseFloat(p.getAttribute('data-v'))||0,y:0};}):pts.map(function(p){return{x:parseFloat(p.getAttribute('data-x'))||0,y:parseFloat(p.getAttribute('data-y'))||0};});var n=P.length,labels=new Array(n).fill(-2),visited=new Array(n).fill(false);function neigh(i){var r=[];for(var j=0;j<n;j++){var dx=P[i].x-P[j].x,dy=P[i].y-P[j].y;if(Math.sqrt(dx*dx+dy*dy)<=cfg.eps)r.push(j);}return r;}var cid=0;for(var i=0;i<n;i++){if(visited[i])continue;visited[i]=true;var nb=neigh(i);if(nb.length<cfg.m){labels[i]=-1;continue;}labels[i]=cid;var q=nb.slice();while(q.length){var j=q.shift();if(!visited[j]){visited[j]=true;var nb2=neigh(j);if(nb2.length>=cfg.m)q=q.concat(nb2);}if(labels[j]<0)labels[j]=cid;}cid++;}var pal=['#6366f1','#22c55e','#f59e0b','#ef4444','#06b6d4','#ec4899','#84cc16','#8b5cf6'];pts.forEach(function(p,i){var col=labels[i]===-1?'#94a3b8':pal[labels[i]%pal.length];p.style.setProperty('fill',col,'important');p.style.setProperty('stroke',col,'important');});})()";

const SP_COMPARE_JS: &str = "(function(){if(window.__spcm__)return;window.__spcm__=1;var fe=window.frameElement;if(!fe)return;var doc=window.parent.document,win=window.parent;var svg=document.querySelector('svg');var score=0;if(svg){svg.querySelectorAll('[data-idx][data-v],[data-idx][data-y]').forEach(function(e){var v=parseFloat(e.getAttribute('data-v')||e.getAttribute('data-y'));if(!isNaN(v))score+=v;});}var title=(document.querySelector('.sp-ttl')||{}).textContent||'Chart';fe.setAttribute('data-sp-cmp-score',score);fe.setAttribute('data-sp-cmp-title',title);if(!win.__spCmpInit){win.__spCmpInit=1;var st=doc.createElement('style');st.textContent='#sp-cmp-ticker{position:fixed;top:14px;left:50%;transform:translateX(-50%);background:rgba(15,23,42,.95);color:#f1f5f9;padding:10px 24px;border-radius:40px;font:600 14px/1 system-ui,sans-serif;z-index:99999;display:none;box-shadow:0 4px 24px rgba(0,0,0,.4);white-space:nowrap}.sp-cmp-sel{outline:3px solid #818cf8!important;outline-offset:2px!important}';doc.head.appendChild(st);var tk=doc.createElement('div');tk.id='sp-cmp-ticker';doc.body.appendChild(tk);win.__spCmpUpdate=function(){var sel=Array.prototype.slice.call(doc.querySelectorAll('iframe[data-sp-cmp-sel]'));var tk2=doc.getElementById('sp-cmp-ticker');if(!tk2)return;if(sel.length<2){tk2.style.display='none';return;}sel.sort(function(a,b){return parseFloat(b.getAttribute('data-sp-cmp-score'))-parseFloat(a.getAttribute('data-sp-cmp-score'));});var sc=sel.map(function(f){return parseFloat(f.getAttribute('data-sp-cmp-score'))||0;});var parts=[];for(var i=0;i<sel.length;i++){var t=sel[i].getAttribute('data-sp-cmp-title')||('C'+(i+1));parts.push('<span style=\"color:#a5b4fc\">'+t+'</span>');if(i<sel.length-1){var diff=Math.abs(sc[0])>1e-9?Math.abs(sc[i]-sc[i+1])/Math.abs(sc[0]):0;var arr=diff>0.5?'>>>':diff>0.2?'>>':'>';parts.push(' <span style=\"color:#64748b\">'+arr+'</span> ');}}tk2.innerHTML=parts.join('');tk2.style.display='block';};}document.addEventListener('click',function(e){if(!e.shiftKey)return;e.preventDefault();if(fe.hasAttribute('data-sp-cmp-sel')){fe.removeAttribute('data-sp-cmp-sel');fe.classList.remove('sp-cmp-sel');}else{fe.setAttribute('data-sp-cmp-sel','1');fe.classList.add('sp-cmp-sel');}if(win.__spCmpUpdate)win.__spCmpUpdate();});})()";

const SP_PICK_JS: &str = "(function(){if(window.__sp_pick__)return;window.__sp_pick__=1;var svg=document.querySelector('svg');if(!svg)return;var els=Array.prototype.slice.call(svg.querySelectorAll('[data-idx]'));if(els.length<2)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var sel={};var ns='http://www.w3.org/2000/svg';var bg=document.createElementNS(ns,'g');bg.id='sp-pick-g';svg.appendChild(bg);function gv(el){return parseFloat(el.getAttribute('data-v')||el.getAttribute('data-y')||'0');}function getLabel(el){var idx=parseInt(el.getAttribute('data-idx'));var cx=el.tagName==='rect'?parseFloat(el.getAttribute('x'))+parseFloat(el.getAttribute('width'))/2:parseFloat(el.getAttribute('cx')||el.getAttribute('x')||'0');var mid=pT+pH/2;var best=null,bd=Infinity;svg.querySelectorAll('text').forEach(function(t){var ty=parseFloat(t.getAttribute('y')||'0');if(ty>mid){var tx=parseFloat(t.getAttribute('x')||'0');var dd=Math.abs(tx-cx);if(dd<bd){bd=dd;best=t;}}});return best&&bd<60?best.textContent:'#'+idx;}function render(){var keys=Object.keys(sel);els.forEach(function(el){el.style.opacity=keys.length&&!sel[el.getAttribute('data-idx')]?'0.18':'';el.style.transition='opacity .15s';});var g=document.getElementById('sp-pick-g');while(g.firstChild)g.removeChild(g.firstChild);if(keys.length<2)return;var items=keys.map(function(k){return{label:getLabel(sel[k]),val:gv(sel[k])};});items.sort(function(a,b){return b.val-a.val;});var topV=Math.abs(items[0].val);var parts=[];items.forEach(function(it,i){parts.push(it.label+' ('+it.val.toFixed(2)+')');if(i<items.length-1){var diff=topV>1e-9?(it.val-items[i+1].val)/topV:0;parts.push(diff>0.4?'>>>':diff>0.15?'>>':'>');}});var txt=parts.join(' ');var tw=Math.min(pW*0.92,Math.max(110,txt.length*7+28));var bx=pL+pW/2,by=pT-8;var rct=document.createElementNS(ns,'rect');rct.setAttribute('x',bx-tw/2);rct.setAttribute('y',by-20);rct.setAttribute('width',tw);rct.setAttribute('height',24);rct.setAttribute('rx','12');rct.setAttribute('fill','rgba(15,23,42,.92)');rct.setAttribute('stroke','#818cf8');rct.setAttribute('stroke-width','1.5');g.appendChild(rct);var t=document.createElementNS(ns,'text');t.setAttribute('x',bx);t.setAttribute('y',by);t.setAttribute('text-anchor','middle');t.setAttribute('font-size','11');t.setAttribute('font-weight','600');t.setAttribute('fill','#f1f5f9');t.setAttribute('font-family','system-ui,sans-serif');t.textContent=txt;g.appendChild(t);}els.forEach(function(el){el.style.cursor='pointer';el.addEventListener('click',function(e){if(!e.shiftKey)return;e.preventDefault();e.stopPropagation();var idx=el.getAttribute('data-idx');if(sel[idx]){delete sel[idx];}else{sel[idx]=el;}render();});});})()";

const SP_MEAN_JS: &str = "(function(){var cfg=window.__sp_mean__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var els=svg.querySelectorAll('[data-idx][data-v],[data-idx][data-y]');if(!els.length)return;var vals=[],sum=0;els.forEach(function(e){var v=parseFloat(e.getAttribute('data-v')||e.getAttribute('data-y'));if(!isNaN(v)){vals.push(v);sum+=v;}});if(!vals.length)return;var stat=sum/vals.length;var yts=svg.querySelectorAll('.sp-yt');var tvs=[];yts.forEach(function(t){var v=parseFloat(t.textContent);if(!isNaN(v))tvs.push(v);});var minV=tvs.length>=2?Math.min.apply(null,tvs):0;var maxV=tvs.length>=2?Math.max.apply(null,tvs):Math.max.apply(null,vals);if(maxV<=minV)return;var y=pT+pH-(stat-minV)/(maxV-minV)*pH;if(y<pT-2||y>pT+pH+2)return;var ns='http://www.w3.org/2000/svg';var ln=document.createElementNS(ns,'line');ln.setAttribute('x1',pL);ln.setAttribute('x2',pL+pW);ln.setAttribute('y1',y);ln.setAttribute('y2',y);ln.setAttribute('stroke',cfg.c);ln.setAttribute('stroke-width','1.5');ln.setAttribute('stroke-dasharray','6,4');svg.appendChild(ln);var tx=document.createElementNS(ns,'text');tx.setAttribute('x',pL+pW-4);tx.setAttribute('y',y-5);tx.setAttribute('text-anchor','end');tx.setAttribute('font-size','10.5');tx.setAttribute('fill',cfg.c);tx.textContent='μ '+stat.toFixed(2);svg.appendChild(tx);})()";

const SP_MEDIAN_JS: &str = "(function(){var cfg=window.__sp_median__;if(!cfg)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var els=svg.querySelectorAll('[data-idx][data-v],[data-idx][data-y]');if(!els.length)return;var vals=[];els.forEach(function(e){var v=parseFloat(e.getAttribute('data-v')||e.getAttribute('data-y'));if(!isNaN(v))vals.push(v);});if(!vals.length)return;var sorted=vals.slice().sort(function(a,b){return a-b;});var mid=Math.floor(sorted.length/2);var stat=sorted.length%2?sorted[mid]:(sorted[mid-1]+sorted[mid])/2;var yts=svg.querySelectorAll('.sp-yt');var tvs=[];yts.forEach(function(t){var v=parseFloat(t.textContent);if(!isNaN(v))tvs.push(v);});var minV=tvs.length>=2?Math.min.apply(null,tvs):0;var maxV=tvs.length>=2?Math.max.apply(null,tvs):Math.max.apply(null,vals);if(maxV<=minV)return;var y=pT+pH-(stat-minV)/(maxV-minV)*pH;if(y<pT-2||y>pT+pH+2)return;var ns='http://www.w3.org/2000/svg';var ln=document.createElementNS(ns,'line');ln.setAttribute('x1',pL);ln.setAttribute('x2',pL+pW);ln.setAttribute('y1',y);ln.setAttribute('y2',y);ln.setAttribute('stroke',cfg.c);ln.setAttribute('stroke-width','1.5');ln.setAttribute('stroke-dasharray','4,3');svg.appendChild(ln);var tx=document.createElementNS(ns,'text');tx.setAttribute('x',pL+pW-4);tx.setAttribute('y',y-5);tx.setAttribute('text-anchor','end');tx.setAttribute('font-size','10.5');tx.setAttribute('fill',cfg.c);tx.textContent='Med '+stat.toFixed(2);svg.appendChild(tx);})()";


pub(crate) fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(feature = "python")]
fn inject_global_cfg(html: String) -> String {
    use std::sync::atomic::Ordering::Relaxed;
    let mut css = String::new();
    let mut js = String::new();
    if let Ok(f) = GLOBAL_FONT.lock() {
        if let Some(ref font) = *f {
            css.push_str(&format!(
                "svg text,body{{font-family:'{}',system-ui,sans-serif!important}}",
                font
            ));
        }
    }
    let fs = GLOBAL_FONT_SIZE.load(Relaxed);
    if fs > 0 {
        css.push_str(&format!("svg text{{font-size:{}px!important}}", fs));
    }
    let ts = GLOBAL_TITLE_SIZE.load(Relaxed);
    if ts > 0 {
        css.push_str(&format!(".sp-ttl{{font-size:{}px!important}}", ts));
    }
    let br = GLOBAL_BORDER_RADIUS.load(Relaxed);
    if br > 0 {
        css.push_str(&format!(
            ".chart-container,.c3w{{border-radius:{}px!important;overflow:hidden}}",
            br
        ));
    }
    if let Ok(op) = GLOBAL_OPACITY.lock() {
        if let Some(o) = *op {
            if o < 1.0 {
                css.push_str(&format!("svg rect[data-idx],svg circle[data-idx],svg path.sp-area{{opacity:{}!important}}", o));
            }
        }
    }
    if GLOBAL_RESPONSIVE.load(Relaxed) {
        css.push_str("svg{width:100%!important;height:auto!important}");
    }
    let mg = GLOBAL_MARGIN.load(Relaxed);
    if mg > 0 {
        css.push_str(&format!(".chart-container,.c3w{{padding:{}px}}", mg));
    }
    if GLOBAL_ANIMATION.load(Relaxed) {
        let dur = GLOBAL_ANIMATION_DURATION.load(Relaxed);
        css.push_str(&format!("@keyframes sp-in{{from{{opacity:0;transform:translateY(8px)}}to{{opacity:1;transform:none}}}}svg rect[data-idx],svg circle[data-idx],svg path.sp-area{{animation:sp-in {}ms ease-out both}}", dur));
        js.push_str("(function(){if(window.__spa__)return;window.__spa__=1;var els=document.querySelectorAll('svg [data-idx]');for(var i=0;i<els.length;i++)els[i].style.animationDelay=i*30+'ms';})();");
    }
    if GLOBAL_CROSSHAIR.load(Relaxed) {
        js.push_str(SP_CROSSHAIR_JS);
        js.push(';');
    }
    if GLOBAL_ZOOM.load(Relaxed) {
        js.push_str(SP_ZOOM_JS);
        js.push(';');
    }
    if GLOBAL_EXPORT_BTN.load(Relaxed) {
        js.push_str(SP_EXPORT_JS);
        js.push(';');
    }
    let text_auto_v = GLOBAL_TEXT_AUTO.lock().ok().and_then(|g| g.clone());
    let text_pos_v = GLOBAL_TEXT_POSITION.lock().ok().and_then(|g| g.clone());
    let text_angle_v = GLOBAL_TEXT_ANGLE.load(Relaxed);
    let text_fs_v = GLOBAL_TEXT_FONT_SIZE.load(Relaxed);
    let text_col_v = GLOBAL_TEXT_FONT_COLOR.lock().ok().and_then(|g| g.clone());
    let utext_min = GLOBAL_UNIFORM_TEXT_MIN.load(Relaxed);
    let utext_mode = GLOBAL_UNIFORM_TEXT_MODE.lock().ok().and_then(|g| g.clone());
    if text_auto_v.is_some()
        || text_pos_v.is_some()
        || text_angle_v != i32::MIN
        || text_fs_v > 0
        || text_col_v.is_some()
    {
        let mut opts = String::from("window.__sp_text__={");
        if let Some(ref t) = text_auto_v {
            opts.push_str(&format!("format:{},", json_str(t)));
        }
        if let Some(ref p) = text_pos_v {
            opts.push_str(&format!("position:{},", json_str(p)));
        }
        if text_angle_v != i32::MIN {
            opts.push_str(&format!("angle:{},", text_angle_v));
        }
        if text_fs_v > 0 {
            opts.push_str(&format!("font_size:{},", text_fs_v));
        }
        if let Some(ref c) = text_col_v {
            opts.push_str(&format!("color:{},", json_str(c)));
        }
        if utext_min > 0 {
            opts.push_str(&format!("uniform_min:{},", utext_min));
        }
        if let Some(ref m) = utext_mode {
            opts.push_str(&format!("uniform_mode:{},", json_str(m)));
        }
        opts.push_str("};");
        js.push_str(&opts);
        js.push_str(SP_TEXT_JS);
        js.push(';');
    }
    let bar_r = GLOBAL_BAR_CORNER_RADIUS.lock().ok().and_then(|g| g.clone());
    if let Some(r) = bar_r {
        js.push_str(&format!(
            "window.__sp_bar_r__={};",
            if r.ends_with('%') {
                json_str(&r)
            } else {
                r.parse::<f64>()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|_| json_str(&r))
            }
        ));
        js.push_str(SP_BAR_RADIUS_JS);
        js.push(';');
    }
    js.push_str(SP_AUTOCLASS_JS);
    js.push(';');
    let mut out = html;
    if !css.is_empty() {
        out = out.replacen("</head>", &format!("<style>{}</style></head>", css), 1);
    }
    out = out.replacen("</body>", &format!("<script>{}</script></body>", js), 1);
    out
}

pub(crate) fn hover_dedup_images(slots_json: &str) -> (String, String) {
    use serde_json::Value;
    let Ok(Value::Array(mut slots)) = serde_json::from_str::<Value>(slots_json) else {
        return (slots_json.to_string(), String::new());
    };
    let mut imgs: Vec<String> = Vec::new();
    let mut img_idx: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut any = false;
    for slot in &mut slots {
        if let Value::Object(map) = slot {
            if let Some(Value::String(img)) = map.remove("image") {
                let idx = if let Some(&i) = img_idx.get(&img) {
                    i
                } else {
                    let i = imgs.len();
                    img_idx.insert(img.clone(), i);
                    imgs.push(img);
                    i
                };
                map.insert("imgIdx".to_string(), Value::Number(idx.into()));
                any = true;
            }
        }
    }
    if !any {
        return (slots_json.to_string(), String::new());
    }
    let deduped = serde_json::to_string(&slots).unwrap_or_else(|_| slots_json.to_string());
    let imgs_js = serde_json::to_string(&imgs).unwrap_or_else(|_| "[]".to_string());
    (deduped, imgs_js)
}

impl Chart {
    #[cfg(feature = "python")]
    fn new(html: String) -> Self {
        let html = if let Some(bg) = get_global_background() {
            crate::html::hover::apply_bg(html, Some(&bg))
        } else {
            html
        };
        let html = inject_global_cfg(html);
        let chart = Self { html, doc_str: "" };
        #[cfg(feature = "python")]
        if AUTO_DISPLAY.load(std::sync::atomic::Ordering::Relaxed) {
            Python::with_gil(|py| auto_show_in_jupyter(py, &chart));
        }
        chart
    }

    #[cfg(feature = "python")]
    fn new_doc(html: String, doc: &'static str) -> Self {
        let mut c = Self::new(html);
        c.doc_str = doc;
        c
    }

    fn propagate(&self, html: String) -> Self {
        Self {
            html,
            doc_str: self.doc_str,
        }
    }

    fn chart_iframe(&self) -> String {
        crate::bindings::chart_methods::chart_iframe(&self.html)
    }
}

#[cfg(feature = "python")]
fn auto_show_in_jupyter(py: Python<'_>, chart: &Chart) {
    let _ = (|| -> PyResult<()> {
        let ipython = py.import_bound("IPython")?;
        let ip = ipython.getattr("get_ipython")?.call0()?;
        if ip.is_none() {
            return Ok(());
        }
        let display_mod = py.import_bound("IPython.display")?;
        let html_cls = display_mod.getattr("HTML")?;
        let display_fn = display_mod.getattr("display")?;
        let html_obj = html_cls.call1((chart.chart_iframe().as_str(),))?;
        display_fn.call1((html_obj,))?;
        Ok(())
    })();
}


#[cfg(feature = "python")]
#[pyo3::pymethods]
impl Chart {
    pub fn compare(
        &self,
        others: Vec<pyo3::Bound<'_, Chart>>,
        metric: Option<String>,
        arrows: Option<bool>,
        scale: Option<bool>,
        scale_to: Option<usize>,
    ) -> Chart {
        let mut htmls = vec![self.html.clone()];
        for bound in &others {
            htmls.push(bound.borrow().html.clone());
        }
        let scale_target = if let Some(idx) = scale_to {
            htmls.get(idx).map(|h| cmp_score(h, "max"))
        } else if scale.unwrap_or(false) {
            htmls.iter().map(|h| cmp_score(h, "max")).fold(None, |acc: Option<f64>, v| {
                Some(acc.map_or(v, |a| a.max(v)))
            })
        } else {
            None
        };
        self.propagate(build_compare_page(
            &htmls,
            metric.as_deref().unwrap_or("sum"),
            arrows.unwrap_or(false),
            scale_target,
        ))
    }

    pub fn subplot(
        &self,
        others: Vec<pyo3::Bound<'_, Chart>>,
        cols: Option<usize>,
        title: Option<String>,
    ) -> Chart {
        let mut htmls = vec![self.html.clone()];
        for bound in &others {
            htmls.push(bound.borrow().html.clone());
        }
        let c = cols.unwrap_or(2).max(1);
        self.propagate(build_grid_page(
            &htmls,
            c,
            title.as_deref(),
            12,
            "transparent",
        ))
    }

}

#[cfg(feature = "ffi")]
mod chart_ffi {
    use super::*;
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;

    #[no_mangle]
    pub extern "C" fn sera_chart_from_html(html: *const c_char) -> *mut Chart {
        if html.is_null() {
            return std::ptr::null_mut();
        }
        let html_str = unsafe { CStr::from_ptr(html) }.to_string_lossy().into_owned();
        Box::into_raw(Box::new(Chart {
            html: html_str,
            doc_str: "",
        }))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_html(chart: *const Chart) -> *mut c_char {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        let h = &unsafe { &*chart }.html;
        CString::new(h.as_str())
            .map(|s| s.into_raw())
            .unwrap_or(std::ptr::null_mut())
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_free_string(s: *mut c_char) {
        if !s.is_null() {
            unsafe {
                drop(CString::from_raw(s));
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_free(chart: *mut Chart) {
        if !chart.is_null() {
            unsafe {
                drop(Box::from_raw(chart));
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_save(chart: *const Chart, path: *const c_char) -> i32 {
        if chart.is_null() || path.is_null() {
            return -1;
        }
        let h = &unsafe { &*chart }.html;
        let p = match unsafe { CStr::from_ptr(path) }.to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        };
        match std::fs::write(p, h.as_str()) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_to_svg(chart: *const Chart) -> *mut c_char {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        let h = &unsafe { &*chart }.html;
        let result = h
            .find("<svg")
            .and_then(|start| h.rfind("</svg>").map(|end| h[start..end + 6].to_string()));
        result
            .and_then(|s| CString::new(s).ok())
            .map(|s| s.into_raw())
            .unwrap_or(std::ptr::null_mut())
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_inject_css(chart: *const Chart, css: *const c_char) -> *mut Chart {
        if chart.is_null() || css.is_null() {
            return std::ptr::null_mut();
        }
        let c = unsafe { &*chart };
        let s = match unsafe { CStr::from_ptr(css) }.to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        Box::into_raw(Box::new(c.inject_css(s)))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_inject_js(chart: *const Chart, js: *const c_char) -> *mut Chart {
        if chart.is_null() || js.is_null() {
            return std::ptr::null_mut();
        }
        let c = unsafe { &*chart };
        let s = match unsafe { CStr::from_ptr(js) }.to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        Box::into_raw(Box::new(c.inject_js(s)))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_set_bg(chart: *const Chart, color: *const c_char) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        let c = unsafe { &*chart };
        let col = if color.is_null() {
            None
        } else {
            unsafe { CStr::from_ptr(color) }.to_str().ok()
        };
        Box::into_raw(Box::new(c.set_bg(col)))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_no_background(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.no_background()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_no_x_axis(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.no_x_axis()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_no_y_axis(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.no_y_axis()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_no_axes(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.no_axes()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_show_grid(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.show_grid()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_hide_grid(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.hide_grid()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_responsive(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.responsive()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_flip(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.flip()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_crosshair(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.crosshair()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_zoom(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.zoom()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_no_legend(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.no_legend()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_no_title(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.no_title()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_export_button(chart: *const Chart) -> *mut Chart {
        if chart.is_null() {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(unsafe { &*chart }.export_button()))
    }

    #[no_mangle]
    pub unsafe extern "C" fn sera_call(
        name: *const c_char,
        json: *const c_char,
    ) -> *mut c_char {
        let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("");
        let json = unsafe { CStr::from_ptr(json) }.to_str().unwrap_or("{}");
        let resolved = crate::bindings::alias_registry::resolve(name);
        let target = resolved.as_deref().unwrap_or(name);
        for entry in crate::bindings::fn_registry::iter_entries() {
            if entry.name == target {
                let result = (entry.invoke)(json);
                return CString::new(result).unwrap_or_default().into_raw();
            }
        }
        std::ptr::null_mut()
    }

    #[no_mangle]
    pub extern "C" fn sera_list() -> *mut c_char {
        let names: Vec<&str> = crate::bindings::fn_registry::iter_entries()
            .map(|e| e.name)
            .collect();
        CString::new(serde_json::to_string(&names).unwrap_or_default())
            .unwrap_or_default()
            .into_raw()
    }

    #[no_mangle]
    pub extern "C" fn sera_version() -> *mut c_char {
        CString::new(env!("CARGO_PKG_VERSION"))
            .unwrap_or_default()
            .into_raw()
    }

    #[no_mangle]
    pub unsafe extern "C" fn sera_params_json(
        chart: *const c_char,
        variant: *const c_char,
    ) -> *mut c_char {
        let chart = if chart.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(chart) }.to_str().unwrap_or(""))
        };
        let variant = if variant.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(variant) }.to_str().unwrap_or(""))
        };
        let s = serde_json::to_string(&crate::params(chart, variant)).unwrap_or_default();
        CString::new(s).unwrap_or_default().into_raw()
    }

    #[no_mangle]
    pub unsafe extern "C" fn sera_required_params_json(
        chart: *const c_char,
        variant: *const c_char,
    ) -> *mut c_char {
        let chart = if chart.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(chart) }.to_str().unwrap_or(""))
        };
        let variant = if variant.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(variant) }.to_str().unwrap_or(""))
        };
        let s =
            serde_json::to_string(&crate::required_params(chart, variant)).unwrap_or_default();
        CString::new(s).unwrap_or_default().into_raw()
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_variants_json() -> *mut c_char {
        let s = serde_json::to_string(&crate::chart_variants()).unwrap_or_default();
        CString::new(s).unwrap_or_default().into_raw()
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_themes_json() -> *mut c_char {
        let s = serde_json::to_string(&crate::chart_themes()).unwrap_or_default();
        CString::new(s).unwrap_or_default().into_raw()
    }

    #[no_mangle]
    pub extern "C" fn sera_scenes3d_json() -> *mut c_char {
        let s = serde_json::to_string(&crate::scenes3d()).unwrap_or_default();
        CString::new(s).unwrap_or_default().into_raw()
    }

    #[no_mangle]
    pub extern "C" fn sera_docs_json() -> *mut c_char {
        let s = serde_json::to_string(&crate::doc_registry::all_docs()).unwrap_or_default();
        CString::new(s).unwrap_or_default().into_raw()
    }

    #[no_mangle]
    pub extern "C" fn sera_themes_list() -> *mut c_char {
        let s = serde_json::to_string(&crate::themes()).unwrap_or_default();
        CString::new(s).unwrap_or_default().into_raw()
    }

    #[no_mangle]
    pub unsafe extern "C" fn sera_set_theme(name: *const c_char) {
        let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("");
        if let Some(preset) = crate::resolve_theme(name) {
            if let Ok(mut bg) = crate::GLOBAL_BACKGROUND.lock() {
                *bg = preset.bg.map(|v| v.to_string());
            }
            if let Ok(mut palette) = crate::GLOBAL_PALETTE.lock() {
                *palette = Some(preset.palette.to_vec());
            }
            crate::GLOBAL_GRIDLINES.store(preset.gridlines, std::sync::atomic::Ordering::Relaxed);
            if let Ok(mut tn) = crate::GLOBAL_THEME_NAME.lock() {
                *tn = Some(name.to_string());
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn sera_set_bg(color: *const c_char) {
        let color = unsafe { CStr::from_ptr(color) }.to_str().unwrap_or("");
        crate::set_global_background(color);
    }

    #[no_mangle]
    pub extern "C" fn sera_reset_bg() {
        crate::reset_global_background();
    }

    #[no_mangle]
    pub extern "C" fn sera_demos_list() -> *mut c_char {
        let s = serde_json::to_string(&crate::demos()).unwrap_or_default();
        CString::new(s).unwrap_or_default().into_raw()
    }

    #[no_mangle]
    pub unsafe extern "C" fn sera_demo_code(
        name: *const c_char,
        variant: *const c_char,
    ) -> *mut c_char {
        let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("");
        let variant = if variant.is_null() {
            None
        } else {
            let s = unsafe { CStr::from_ptr(variant) }.to_str().unwrap_or("");
            if s.is_empty() { None } else { Some(s) }
        };
        let result = crate::demo(name, variant).unwrap_or_default();
        CString::new(result).unwrap_or_default().into_raw()
    }
}

#[sera_doc(
    category = "config",
    file = "config/global.md",
    en = "Binds label names to specific colors. Every chart built after this call will automatically apply these color overrides to elements matching those labels.",
    fr = "Associe des noms de labels à des couleurs spécifiques. Tous les graphiques créés après cet appel appliquent automatiquement ces couleurs aux éléments correspondants."
)]
#[sera_bind(ffi)]
pub fn bind_colors(bindings: Vec<(String, u32)>) {
    if let Ok(mut g) = GLOBAL_COLOR_BINDINGS.lock() {
        for (lbl, col) in bindings {
            if let Some(pos) = g.iter().position(|(k, _)| k == &lbl) {
                g[pos].1 = col;
            } else {
                g.push((lbl, col));
            }
        }
    }
}

#[sera_doc(
    category = "config",
    file = "config/global.md",
    en = "Removes all label→color bindings registered via bind_colors().",
    fr = "Supprime toutes les correspondances label→couleur enregistrées via bind_colors()."
)]
#[sera_bind]
pub fn clear_color_bindings() {
    if let Ok(mut g) = GLOBAL_COLOR_BINDINGS.lock() {
        g.clear();
    }
}

pub(crate) fn apply_global_color_bindings(html: String) -> String {
    let bindings = match GLOBAL_COLOR_BINDINGS.lock() {
        Ok(g) => g.clone(),
        Err(_) => return html,
    };
    if bindings.is_empty() {
        return html;
    }
    let entries: String = bindings.iter()
        .map(|(lbl, col)| format!("\"{}\":\"#{:06X}\"", lbl.replace('"', "\\\""), col))
        .collect::<Vec<_>>()
        .join(",");
    let js = format!(
        "(function(){{var m={{{}}};var svg=document.querySelector('svg');if(!svg)return;svg.querySelectorAll('[data-lbl]').forEach(function(el){{var lbl=el.getAttribute('data-lbl');if(m[lbl])el.style.setProperty('fill',m[lbl],'important');}});}})()",
        entries
    );
    html.replacen("</body>", &format!("<script>{}</script></body>", js), 1)
}

#[sera_doc(
    category = "config",
    file = "config/global.md",
    en = "Sets a global background color applied to all charts created after this call.",
    fr = "DÃ©finit une couleur d'arriÃ¨re-plan globale appliquÃ©e Ã  tous les graphiques crÃ©Ã©s aprÃ¨s cet appel.",
    param(
        name = "color",
        ty = "str",
        en = "CSS color string (hex, rgb, named). Empty string removes it.",
        fr = "Couleur CSS (hex, rgb, nommÃ©e). ChaÃ®ne vide pour la supprimer."
    )
)]
#[sera_bind]
pub fn set_global_background(color: &str) {
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() {
        *bg = Some(color.to_string());
    }
    crate::plot::set_global_bg(Some(color.to_string()));
}

#[sera_doc(
    category = "config",
    file = "config/global.md",
    en = "Clears the global background color so charts use their default background.",
    fr = "Efface la couleur d'arriÃ¨re-plan globale afin que les graphiques utilisent leur arriÃ¨re-plan par dÃ©faut."
)]
#[sera_bind]
pub fn reset_global_background() {
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() {
        *bg = None;
    }
    crate::plot::set_global_bg(None);
}

#[sera_doc(
    category = "config",
    file = "config/global.md",
    en = "Enables or disables automatic display of charts in Jupyter notebooks upon creation.",
    fr = "Active ou dÃ©sactive l'affichage automatique des graphiques dans les notebooks Jupyter Ã  la crÃ©ation.",
    param(
        name = "enabled",
        ty = "bool",
        en = "True to auto-display charts in Jupyter; False to suppress auto-display.",
        fr = "True pour afficher automatiquement les graphiques dans Jupyter; False pour supprimer l'affichage automatique."
    )
)]
#[sera_bind]
pub fn set_auto_display(enabled: bool) {
    AUTO_DISPLAY.store(enabled, std::sync::atomic::Ordering::Relaxed);
}

#[sera_doc(
    category = "theme",
    file = "theme/theme.md",
    en = "Resets the active theme back to the framework default.",
    fr = "RÃ©initialise le thÃ¨me actif vers le thÃ¨me par dÃ©faut du framework."
)]
#[sera_bind]
pub fn reset_theme() {
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() {
        *bg = None;
    }
    if let Ok(mut pal) = GLOBAL_PALETTE.lock() {
        *pal = None;
    }
    GLOBAL_GRIDLINES.store(false, std::sync::atomic::Ordering::Relaxed);
    if let Ok(mut tn) = GLOBAL_THEME_NAME.lock() {
        *tn = None;
    }
    crate::plot::set_global_bg(None);
    crate::plot::set_global_pal(Vec::new());
    crate::plot::set_global_grid(false);
}

#[sera_doc(
    category = "theme",
    file = "theme/theme.md",
    en = "Returns a list of all available theme names.",
    fr = "Retourne la liste de tous les noms de thÃ¨mes disponibles."
)]
#[sera_bind]
pub fn themes() -> Vec<String> {
    vec![
        "dark",
        "light",
        "scientific",
        "apple",
        "notion",
        "minimal",
        "neon",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

#[sera_doc(
    category = "utility",
    file = "api/reference.md",
    en = "Returns a list of all available chart family names that have demo snippets.",
    fr = "Retourne la liste de tous les noms de familles de graphiques ayant des extraits de dÃ©monstration."
)]
#[sera_bind]
pub fn demos() -> Vec<&'static str> {
    let mut out: Vec<&'static str> = crate::DEMO_REGISTRY.iter().map(|(f, _, _)| *f).collect();
    out.sort();
    out.dedup();
    out
}

#[sera_doc(
    category = "utility",
    file = "api/reference.md",
    en = "Returns a Python code snippet demonstrating how to create the specified chart type, or None if unknown.",
    fr = "Retourne un extrait de code Python illustrant comment crÃ©er le type de graphique spÃ©cifiÃ©, ou None si inconnu.",
    param(
        name = "chart",
        ty = "str",
        en = "Chart family name (e.g. 'bar', 'line', 'scatter').",
        fr = "Nom de la famille de graphique (ex: 'bar', 'line', 'scatter')."
    ),
    param(
        name = "variant",
        ty = "str | None",
        en = "Variant name. Defaults to 'basic' if None.",
        fr = "Nom de la variante. Par dÃ©faut 'basic' si None."
    )
)]
#[sera_sig(chart, variant=None)]
#[sera_bind]
pub fn demo(chart: &str, variant: Option<&str>) -> Option<String> {
    crate::demo_snippet(chart, variant.unwrap_or("basic"))
}

#[sera_doc(
    category = "utility",
    file = "api/reference.md",
    en = "Returns the optional keyword arguments accepted by the specified chart type.",
    fr = "Retourne les arguments nommÃ©s optionnels acceptÃ©s par le type de graphique spÃ©cifiÃ©.",
    param(
        name = "chart",
        ty = "str | None",
        en = "Chart family name. Returns all charts if None.",
        fr = "Nom de la famille de graphique. Retourne tous les graphiques si None."
    ),
    param(
        name = "variant",
        ty = "str | None",
        en = "Variant name for variant-specific params.",
        fr = "Nom de la variante pour les paramÃ¨tres spÃ©cifiques Ã  la variante."
    )
)]
#[sera_sig(chart=None, variant=None)]
#[sera_bind(serde)]
pub fn params(chart: Option<&str>, variant: Option<&str>) -> serde_json::Value {
    use serde_json::{Map, Value};
    if let (Some(c), Some(v)) = (chart, variant) {
        for (f, vv, k) in crate::DEMO_REGISTRY.iter() {
            if *f == c && *vv == v {
                return Value::String((*k).to_string());
            }
        }
        return Value::Null;
    }
    if let Some(c) = chart {
        let mut m = Map::new();
        for (f, v, k) in crate::DEMO_REGISTRY.iter() {
            if *f == c {
                m.insert((*v).to_string(), Value::String((*k).to_string()));
            }
        }
        if m.is_empty() {
            return Value::Null;
        }
        return Value::Object(m);
    }
    let mut root = Map::new();
    for (f, v, k) in crate::DEMO_REGISTRY.iter() {
        let entry = root
            .entry((*f).to_string())
            .or_insert_with(|| Value::Object(Map::new()));
        if let Value::Object(inner) = entry {
            inner.insert((*v).to_string(), Value::String((*k).to_string()));
        }
    }
    Value::Object(root)
}

#[sera_doc(
    category = "utility",
    file = "api/reference.md",
    en = "Returns the required positional argument names for the specified chart type.",
    fr = "Retourne les noms d'arguments positionnels requis pour le type de graphique spÃ©cifiÃ©.",
    param(
        name = "chart",
        ty = "str | None",
        en = "Chart family name.",
        fr = "Nom de la famille de graphique."
    ),
    param(
        name = "variant",
        ty = "str | None",
        en = "Variant name.",
        fr = "Nom de la variante."
    )
)]
#[sera_sig(chart=None, variant=None)]
#[sera_bind(serde)]
pub fn required_params(chart: Option<&str>, variant: Option<&str>) -> serde_json::Value {
    use serde_json::{Map, Value};
    if let (Some(c), Some(v)) = (chart, variant) {
        let list = crate::required_params_for(c, v).unwrap_or(&[]);
        return Value::Array(
            list.iter()
                .map(|s| Value::String((*s).to_string()))
                .collect(),
        );
    }
    if let Some(c) = chart {
        let mut m = Map::new();
        for (f, v, p) in crate::PARAMS_REGISTRY.iter() {
            if *f == c {
                m.insert(
                    (*v).to_string(),
                    Value::Array(p.iter().map(|s| Value::String((*s).to_string())).collect()),
                );
            }
        }
        return Value::Object(m);
    }
    let mut root = Map::new();
    for (f, v, p) in crate::PARAMS_REGISTRY.iter() {
        let entry = root
            .entry((*f).to_string())
            .or_insert_with(|| Value::Object(Map::new()));
        if let Value::Object(inner) = entry {
            inner.insert(
                (*v).to_string(),
                Value::Array(p.iter().map(|s| Value::String((*s).to_string())).collect()),
            );
        }
    }
    Value::Object(root)
}

#[sera_doc(
    category = "utility",
    file = "api/reference.md",
    en = "Returns a mapping of each chart family name to its list of available variant names and aliases.",
    fr = "Retourne une association de chaque nom de famille de graphique Ã  sa liste de variantes disponibles et alias."
)]
#[sera_bind(serde)]
pub fn chart_variants() -> serde_json::Value {
    use crate::plot::statistical::arc_diagram::ArcDiagramVariant;
    use crate::plot::statistical::bubble::BubbleVariant;
    use crate::plot::statistical::chord::ChordVariant;
    use crate::plot::statistical::circle_pack::CirclePackVariant;
    use crate::plot::statistical::correlogram::CorrelogramVariant;
    use crate::plot::statistical::dendrogram::DendrogramVariant;
    use crate::plot::statistical::hive::HiveVariant;
    use crate::plot::statistical::orbita::OrbitaVariant;
    use crate::plot::statistical::pulse::PulseVariant;
    use crate::plot::statistical::sankey::SankeyVariant;
    use crate::plot::statistical::scatter::ScatterVariant;
    use crate::plot::statistical::venn::VennVariant;
    use crate::plot::statistical::{
        BarVariant, BoxplotVariant, BulletVariant, CandlestickVariant, DumbbellVariant,
        EventplotVariant, FunnelVariant, GanttVariant, GaugeVariant, HeatmapVariant,
        HexbinVariant, HistogramVariant, IcicleVariant, KdeVariant, LineVariant,
        LollipopVariant, ParallelVariant, ParcatsVariant, PieVariant, PlotWebVariant,
        RadarVariant, RidgelineVariant, ScatterTernaryVariant, SlopeVariant, SplomVariant,
        StackplotVariant, SunburstVariant, TreemapVariant, ViolinVariant, WaterfallVariant,
        WordCloudVariant,
    };
    use serde_json::{Map, Value};

    fn build(
        keys: &'static [(&'static str, &'static [&'static str])],
        default_key: &'static str,
    ) -> Value {
        let mut outer = Map::new();
        outer.insert(
            "default".to_string(),
            Value::String(default_key.to_string()),
        );
        let arr: Vec<Value> = keys
            .iter()
            .map(|(k, aliases)| {
                let mut item = Map::new();
                item.insert("key".to_string(), Value::String((*k).to_string()));
                item.insert(
                    "aliases".to_string(),
                    Value::Array(
                        aliases
                            .iter()
                            .map(|a| Value::String((*a).to_string()))
                            .collect(),
                    ),
                );
                Value::Object(item)
            })
            .collect();
        outer.insert("variants".to_string(), Value::Array(arr));
        Value::Object(outer)
    }

    let mut out = Map::new();
    out.insert(
        "bar".to_string(),
        build(BarVariant::keys_and_aliases(), BarVariant::default_key()),
    );
    out.insert(
        "line".to_string(),
        build(LineVariant::keys_and_aliases(), LineVariant::default_key()),
    );
    out.insert(
        "scatter".to_string(),
        build(
            ScatterVariant::keys_and_aliases(),
            ScatterVariant::default_key(),
        ),
    );
    out.insert(
        "bubble".to_string(),
        build(
            BubbleVariant::keys_and_aliases(),
            BubbleVariant::default_key(),
        ),
    );
    out.insert(
        "histogram".to_string(),
        build(
            HistogramVariant::keys_and_aliases(),
            HistogramVariant::default_key(),
        ),
    );
    out.insert(
        "heatmap".to_string(),
        build(
            HeatmapVariant::keys_and_aliases(),
            HeatmapVariant::default_key(),
        ),
    );
    out.insert(
        "pie".to_string(),
        build(PieVariant::keys_and_aliases(), PieVariant::default_key()),
    );
    out.insert(
        "boxplot".to_string(),
        build(
            BoxplotVariant::keys_and_aliases(),
            BoxplotVariant::default_key(),
        ),
    );
    out.insert(
        "violin".to_string(),
        build(
            ViolinVariant::keys_and_aliases(),
            ViolinVariant::default_key(),
        ),
    );
    out.insert(
        "kde".to_string(),
        build(KdeVariant::keys_and_aliases(), KdeVariant::default_key()),
    );
    out.insert(
        "ridgeline".to_string(),
        build(
            RidgelineVariant::keys_and_aliases(),
            RidgelineVariant::default_key(),
        ),
    );
    out.insert(
        "radar".to_string(),
        build(
            RadarVariant::keys_and_aliases(),
            RadarVariant::default_key(),
        ),
    );
    out.insert(
        "slope".to_string(),
        build(
            SlopeVariant::keys_and_aliases(),
            SlopeVariant::default_key(),
        ),
    );
    out.insert(
        "funnel".to_string(),
        build(
            FunnelVariant::keys_and_aliases(),
            FunnelVariant::default_key(),
        ),
    );
    out.insert(
        "sunburst".to_string(),
        build(
            SunburstVariant::keys_and_aliases(),
            SunburstVariant::default_key(),
        ),
    );
    out.insert(
        "waterfall".to_string(),
        build(
            WaterfallVariant::keys_and_aliases(),
            WaterfallVariant::default_key(),
        ),
    );
    out.insert(
        "treemap".to_string(),
        build(
            TreemapVariant::keys_and_aliases(),
            TreemapVariant::default_key(),
        ),
    );
    out.insert(
        "candlestick".to_string(),
        build(
            CandlestickVariant::keys_and_aliases(),
            CandlestickVariant::default_key(),
        ),
    );
    out.insert(
        "dumbbell".to_string(),
        build(
            DumbbellVariant::keys_and_aliases(),
            DumbbellVariant::default_key(),
        ),
    );
    out.insert(
        "bullet".to_string(),
        build(
            BulletVariant::keys_and_aliases(),
            BulletVariant::default_key(),
        ),
    );
    out.insert(
        "gauge".to_string(),
        build(
            GaugeVariant::keys_and_aliases(),
            GaugeVariant::default_key(),
        ),
    );
    out.insert(
        "lollipop".to_string(),
        build(
            LollipopVariant::keys_and_aliases(),
            LollipopVariant::default_key(),
        ),
    );
    out.insert(
        "parallel".to_string(),
        build(
            ParallelVariant::keys_and_aliases(),
            ParallelVariant::default_key(),
        ),
    );
    out.insert(
        "wordcloud".to_string(),
        build(
            WordCloudVariant::keys_and_aliases(),
            WordCloudVariant::default_key(),
        ),
    );
    out.insert(
        "sankey".to_string(),
        build(SankeyVariant::keys_and_aliases(), SankeyVariant::default_key()),
    );
    out.insert(
        "chord".to_string(),
        build(ChordVariant::keys_and_aliases(), ChordVariant::default_key()),
    );
    out.insert(
        "circle_pack".to_string(),
        build(
            CirclePackVariant::keys_and_aliases(),
            CirclePackVariant::default_key(),
        ),
    );
    out.insert(
        "arc_diagram".to_string(),
        build(
            ArcDiagramVariant::keys_and_aliases(),
            ArcDiagramVariant::default_key(),
        ),
    );
    out.insert(
        "dendrogram".to_string(),
        build(
            DendrogramVariant::keys_and_aliases(),
            DendrogramVariant::default_key(),
        ),
    );
    out.insert(
        "venn".to_string(),
        build(VennVariant::keys_and_aliases(), VennVariant::default_key()),
    );
    out.insert(
        "correlogram".to_string(),
        build(
            CorrelogramVariant::keys_and_aliases(),
            CorrelogramVariant::default_key(),
        ),
    );
    out.insert(
        "hive".to_string(),
        build(HiveVariant::keys_and_aliases(), HiveVariant::default_key()),
    );
    out.insert(
        "pulse".to_string(),
        build(PulseVariant::keys_and_aliases(), PulseVariant::default_key()),
    );
    out.insert(
        "orbita".to_string(),
        build(OrbitaVariant::keys_and_aliases(), OrbitaVariant::default_key()),
    );
    out.insert(
        "eventplot".to_string(),
        build(
            EventplotVariant::keys_and_aliases(),
            EventplotVariant::default_key(),
        ),
    );
    out.insert(
        "gantt".to_string(),
        build(GanttVariant::keys_and_aliases(), GanttVariant::default_key()),
    );
    out.insert(
        "hexbin".to_string(),
        build(HexbinVariant::keys_and_aliases(), HexbinVariant::default_key()),
    );
    out.insert(
        "icicle".to_string(),
        build(IcicleVariant::keys_and_aliases(), IcicleVariant::default_key()),
    );
    out.insert(
        "parcats".to_string(),
        build(
            ParcatsVariant::keys_and_aliases(),
            ParcatsVariant::default_key(),
        ),
    );
    out.insert(
        "scatterternary".to_string(),
        build(
            ScatterTernaryVariant::keys_and_aliases(),
            ScatterTernaryVariant::default_key(),
        ),
    );
    out.insert(
        "splom".to_string(),
        build(SplomVariant::keys_and_aliases(), SplomVariant::default_key()),
    );
    out.insert(
        "stackplot".to_string(),
        build(
            StackplotVariant::keys_and_aliases(),
            StackplotVariant::default_key(),
        ),
    );
    out.insert(
        "plot_web".to_string(),
        build(
            PlotWebVariant::keys_and_aliases(),
            PlotWebVariant::default_key(),
        ),
    );
    use crate::plot::scene3d::Scene3DVariant;
    let scene_keys = Scene3DVariant::keys_and_aliases();
    let scene_default = Scene3DVariant::default_key();
    let default_only: &[(&str, &[&str])] = &[("default", &["default", "classic", "categorical"])];
    for family in [
        "bar_3d",
        "line_3d",
        "scatter_3d",
        "candlestick3d",
        "dumbbell3d",
        "heatmap3d",
        "kde3d",
        "lollipop3d",
        "ridgeline3d",
        "stacked_bar3d",
        "violin3d",
        "plot_3d_types",
    ] {
        out.insert(family.to_string(), build(scene_keys, scene_default));
    }
    for family in ["radar3d", "funnel3d", "pie3d", "sunburst3d", "globe"] {
        out.insert(family.to_string(), build(default_only, "default"));
    }
    Value::Object(out)
}

pub fn chart_themes() -> serde_json::Value {
    use crate::plot::statistical::theme::ChartTheme;
    use serde_json::{Map, Value};

    let mut out = Map::new();
    out.insert(
        "default".to_string(),
        Value::String(ChartTheme::default_key().to_string()),
    );
    let arr: Vec<Value> = ChartTheme::keys_and_aliases()
        .iter()
        .map(|(k, aliases)| {
            let mut item = Map::new();
            item.insert("key".to_string(), Value::String((*k).to_string()));
            item.insert(
                "aliases".to_string(),
                Value::Array(
                    aliases
                        .iter()
                        .map(|a| Value::String((*a).to_string()))
                        .collect(),
                ),
            );
            Value::Object(item)
        })
        .collect();
    out.insert("themes".to_string(), Value::Array(arr));
    Value::Object(out)
}

pub fn scenes3d() -> serde_json::Value {
    use crate::plot::scene3d::{iter_entries, Scene3DVariant};
    use serde_json::{Map, Value};

    let mut out = Map::new();
    out.insert(
        "default".to_string(),
        Value::String(Scene3DVariant::default_key().to_string()),
    );
    let arr: Vec<Value> = Scene3DVariant::keys_and_aliases()
        .iter()
        .map(|(k, aliases)| {
            let mut item = Map::new();
            item.insert("key".to_string(), Value::String((*k).to_string()));
            item.insert(
                "aliases".to_string(),
                Value::Array(
                    aliases
                        .iter()
                        .map(|a| Value::String((*a).to_string()))
                        .collect(),
                ),
            );
            if let Some(entry) = iter_entries().find(|e| e.name == *k) {
                item.insert("en".to_string(), Value::String(entry.en.to_string()));
                item.insert("fr".to_string(), Value::String(entry.fr.to_string()));
            }
            Value::Object(item)
        })
        .collect();
    out.insert("scenes".to_string(), Value::Array(arr));
    Value::Object(out)
}

#[sera_doc(
    category = "config",
    file = "config/config.md",
    en = "Resets all global config settings applied via config() back to their defaults.",
    fr = "RÃ©initialise tous les paramÃ¨tres de configuration globale dÃ©finis via config() Ã  leurs valeurs par dÃ©faut."
)]
#[sera_bind]
pub fn reset_config() {
    use std::sync::atomic::Ordering::Relaxed;
    if let Ok(mut f) = GLOBAL_FONT.lock() {
        *f = None;
    }
    GLOBAL_FONT_SIZE.store(0, Relaxed);
    GLOBAL_TITLE_SIZE.store(0, Relaxed);
    GLOBAL_BORDER_RADIUS.store(0, Relaxed);
    if let Ok(mut o) = GLOBAL_OPACITY.lock() {
        *o = None;
    }
    GLOBAL_RESPONSIVE.store(false, Relaxed);
    GLOBAL_ANIMATION.store(false, Relaxed);
    GLOBAL_ANIMATION_DURATION.store(300, Relaxed);
    GLOBAL_CROSSHAIR.store(false, Relaxed);
    GLOBAL_ZOOM.store(false, Relaxed);
    if let Ok(mut t) = GLOBAL_TOOLTIP.lock() {
        *t = None;
    }
    if let Ok(mut l) = GLOBAL_LOCALE.lock() {
        *l = None;
    }
    if let Ok(mut s) = GLOBAL_THOUSANDS_SEP.lock() {
        *s = None;
    }
    GLOBAL_MARGIN.store(0, Relaxed);
    GLOBAL_EXPORT_BTN.store(false, Relaxed);
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() {
        *bg = None;
    }
    if let Ok(mut pal) = GLOBAL_PALETTE.lock() {
        *pal = None;
    }
    GLOBAL_GRIDLINES.store(false, Relaxed);
    if let Ok(mut tn) = GLOBAL_THEME_NAME.lock() {
        *tn = None;
    }
    if let Ok(mut g) = GLOBAL_TEXT_AUTO.lock() {
        *g = None;
    }
    if let Ok(mut g) = GLOBAL_TEXT_POSITION.lock() {
        *g = None;
    }
    GLOBAL_TEXT_ANGLE.store(i32::MIN, Relaxed);
    GLOBAL_TEXT_FONT_SIZE.store(0, Relaxed);
    if let Ok(mut g) = GLOBAL_TEXT_FONT_COLOR.lock() {
        *g = None;
    }
    GLOBAL_UNIFORM_TEXT_MIN.store(0, Relaxed);
    if let Ok(mut g) = GLOBAL_UNIFORM_TEXT_MODE.lock() {
        *g = None;
    }
    if let Ok(mut g) = GLOBAL_BAR_CORNER_RADIUS.lock() {
        *g = None;
    }
    if let Ok(mut g) = GLOBAL_HOVER_INFO.lock() {
        *g = None;
    }
    if let Ok(mut g) = GLOBAL_PATTERN_SHAPE.lock() {
        *g = None;
    }
    crate::plot::utils::set_global_despine(false);
    crate::plot::utils::set_global_watermark(None);
    crate::plot::utils::set_global_shadow(None);
}

#[sera_doc(
    category = "performance",
    file = "config/performance.md",
    en = "Returns hardware profile information: CPU thread count, parallelism threshold, and L2 cache chunk size.",
    fr = "Retourne les informations du profil matÃ©riel: nombre de threads CPU, seuil de parallÃ©lisme et taille de chunk du cache L2."
)]
#[sera_bind(serde)]
pub fn hw() -> crate::core::hw_profile::HwProfile {
    *crate::core::hw_profile::hw()
}

#[cfg(feature = "python")]
#[pyclass(name = "DatasetStats", module = "seraplot")]
pub struct PyDatasetStats {
    #[pyo3(get)]
    pub min: f64,
    #[pyo3(get)]
    pub max: f64,
    #[pyo3(get)]
    pub mean: f64,
    #[pyo3(get)]
    pub std_dev: f64,
    #[pyo3(get)]
    pub sum: f64,
    #[pyo3(get)]
    pub count: usize,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyDatasetStats {
    fn __repr__(&self) -> String {
        format!(
            "DatasetStats(min={:.4}, max={:.4}, mean={:.4}, std_dev={:.4}, count={})",
            self.min, self.max, self.mean, self.std_dev, self.count
        )
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "Dataset", module = "seraplot")]
pub struct PyDataset {
    inner: crate::data::Dataset<f64>,
}

#[cfg(feature = "python")]
#[sera_doc_impl]
#[pymethods]
impl PyDataset {
    #[staticmethod]
    #[sera_doc(
        category = "data",
        file = "api/dataset.md",
        en = "Creates a Dataset from a Python list of float values.",
        fr = "CrÃ©e un Dataset Ã  partir d'une liste Python de valeurs float.",
        param(
            name = "values",
            ty = "list[float]",
            en = "List of numeric values.",
            fr = "Liste de valeurs numÃ©riques."
        )
    )]
    #[pyo3(signature = (values, labels=None))]
    fn from_list(values: Vec<f64>, labels: Option<Vec<String>>) -> Self {
        let mut ds = crate::data::Dataset::with_capacity("dataset", values.len());
        for (i, v) in values.iter().enumerate() {
            let lbl = labels
                .as_ref()
                .and_then(|l| l.get(i))
                .map(|s| s.as_str())
                .unwrap_or("")
                .to_string();
            ds.push(*v, lbl);
        }
        PyDataset { inner: ds }
    }

    #[sera_doc(
        category = "data",
        file = "api/dataset.md",
        en = "Computes descriptive statistics in parallel: min, max, mean, variance, std_dev, sum, count.",
        fr = "Calcule des statistiques descriptives en parallÃ¨le: min, max, mean, variance, std_dev, sum, count."
    )]
    fn par_stats(&self) -> PyDatasetStats {
        let s = self.inner.par_stats();
        PyDatasetStats {
            min: s.min,
            max: s.max,
            mean: s.mean,
            std_dev: s.std_dev,
            sum: s.sum,
            count: s.count,
        }
    }

    #[sera_doc(
        category = "data",
        file = "api/dataset.md",
        en = "Splits the dataset into n equal-sized chunks and returns them as a list of Dataset objects.",
        fr = "Divise le dataset en n morceaux de taille Ã©gale et les retourne sous forme de liste d'objets Dataset.",
        param(
            name = "n",
            ty = "int",
            en = "Number of chunks to split into.",
            fr = "Nombre de morceaux en lesquels diviser."
        )
    )]
    fn into_chunks(&self, n: usize) -> Vec<PyDataset> {
        let vals: Vec<f64> = self.inner.values().collect();
        let labels: Vec<String> = self.inner.labels().map(|s| s.to_string()).collect();
        if n == 0 || vals.is_empty() {
            return vec![];
        }
        let chunk_size = (vals.len() + n - 1) / n;
        vals.chunks(chunk_size)
            .enumerate()
            .map(|(ci, chunk)| {
                let lbl_slice = &labels[ci * chunk_size..ci * chunk_size + chunk.len()];
                let mut ds =
                    crate::data::Dataset::with_capacity(&format!("chunk_{ci}"), chunk.len());
                for (v, l) in chunk.iter().zip(lbl_slice.iter()) {
                    ds.push(*v, l.as_str());
                }
                PyDataset { inner: ds }
            })
            .collect()
    }

    fn values(&self) -> Vec<f64> {
        self.inner.values().collect()
    }

    fn labels(&self) -> Vec<String> {
        self.inner.labels().map(|s| s.to_string()).collect()
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "Dataset(name={:?}, len={})",
            self.inner.name,
            self.inner.len()
        )
    }
}

#[sera_doc(
    category = "performance",
    file = "config/performance.md",
    en = "Enables or disables the adaptive retry system. When enabled, operations that panic will auto-degrade chunk sizes and retry.",
    fr = "Active ou dÃ©sactive le systÃ¨me de rÃ©essai adaptatif. Quand activÃ©, les opÃ©rations qui paniquent rÃ©duisent automatiquement les tailles de chunks et rÃ©essaient.",
    param(
        name = "on",
        ty = "bool",
        en = "True to enable adaptive retry (default). False to disable.",
        fr = "True pour activer le rÃ©essai adaptatif (dÃ©faut). False pour dÃ©sactiver."
    )
)]
#[sera_bind]
pub fn set_adaptive_retry(on: bool) {
    crate::core::adaptive_exec::set_adaptive_retry(on);
}

#[sera_doc(
    category = "performance",
    file = "config/performance.md",
    en = "Resets the degradation level back to 0 (full-speed operation).",
    fr = "RÃ©initialise le niveau de dÃ©gradation Ã  0 (opÃ©ration Ã  pleine vitesse)."
)]
#[sera_bind]
pub fn reset_perf_state() {
    crate::core::adaptive_exec::reset_perf_state();
}

#[sera_doc(
    category = "performance",
    file = "config/performance.md",
    en = "Returns the current degradation level (0 = full speed, 4 = maximum degradation).",
    fr = "Retourne le niveau de dÃ©gradation actuel (0 = pleine vitesse, 4 = dÃ©gradation maximale)."
)]
#[sera_bind]
pub fn adaptive_degrade_level() -> usize {
    crate::core::adaptive_exec::degrade_level()
}

#[sera_doc(
    category = "telemetry",
    file = "about/telemetry.md",
    en = "Enables or disables usage telemetry collection. Disabled by default.",
    fr = "Active ou dÃ©sactive la collecte de tÃ©lÃ©mÃ©trie d'utilisation. DÃ©sactivÃ© par dÃ©faut.",
    param(
        name = "enabled",
        ty = "bool",
        en = "True to enable telemetry, False to disable.",
        fr = "True pour activer la tÃ©lÃ©mÃ©trie, False pour dÃ©sactiver."
    )
)]
#[sera_bind]
pub fn telemetry_consent(enabled: bool) {
    crate::telemetry::set_consent(enabled);
}

#[sera_doc(
    category = "telemetry",
    file = "about/telemetry.md",
    en = "Returns the filesystem path where telemetry data is stored.",
    fr = "Retourne le chemin du systÃ¨me de fichiers oÃ¹ les donnÃ©es de tÃ©lÃ©mÃ©trie sont stockÃ©es."
)]
#[sera_bind]
pub fn telemetry_path() -> String {
    crate::telemetry::telemetry_file_path()
}

#[sera_doc(
    category = "telemetry",
    file = "about/telemetry.md",
    en = "Returns a JSON string with aggregated usage metrics summary.",
    fr = "Retourne une chaÃ®ne JSON avec un rÃ©sumÃ© des mÃ©triques d'utilisation agrÃ©gÃ©es."
)]
#[sera_bind]
pub fn get_metrics() -> String {
    serde_json::to_string(&crate::telemetry::get_metrics_summary()).unwrap_or_default()
}

#[sera_doc(
    category = "utility",
    file = "api/reference.md",
    en = "Returns all documented functions with their signatures, parameters, and bilingual descriptions.",
    fr = "Retourne toutes les fonctions documentÃ©es avec leurs signatures, paramÃ¨tres et descriptions bilingues."
)]
#[sera_bind(serde)]
pub fn docs() -> Vec<&'static crate::doc_registry::FnDoc> {
    crate::doc_registry::all_docs()
}

#[sera_doc(
    category = "utility",
    file = "api/reference.md",
    en = "Returns documentation for a single function by name, or null if not found.",
    fr = "Retourne la documentation d'une seule fonction par nom, ou null si introuvable.",
    param(
        name = "name",
        ty = "str",
        en = "Function name.",
        fr = "Nom de la fonction."
    )
)]
#[sera_bind(serde)]
pub fn doc(name: &str) -> Option<&'static crate::doc_registry::FnDoc> {
    crate::doc_registry::doc_for(name)
}

#[sera_doc(
    category = "utility",
    file = "api/reference.md",
    en = "Returns all registered ML and plot model structs with their fields and categories.",
    fr = "Retourne tous les modèles ML et plot enregistrés avec leurs champs et catégories."
)]
#[sera_bind(serde)]
pub fn models() -> Vec<&'static crate::model_registry::ModelInfo> {
    crate::model_registry::all_models()
}

#[sera_doc(
    category = "utility",
    file = "api/reference.md",
    en = "Returns all models for a given category.",
    fr = "Retourne tous les modèles pour une catégorie donnée.",
    param(
        name = "category",
        ty = "str",
        en = "Category name.",
        fr = "Nom de la catégorie."
    )
)]
#[sera_bind(serde)]
pub fn models_for_category(category: &str) -> Vec<&'static crate::model_registry::ModelInfo> {
    crate::model_registry::models_by_category(category)
}

#[sera_doc(
    category = "utility",
    file = "api/reference.md",
    en = "Returns all models for a given domain (ml or plot).",
    fr = "Retourne tous les modèles pour un domaine donné (ml ou plot).",
    param(
        name = "domain",
        ty = "str",
        en = "Domain: 'ml' or 'plot'.",
        fr = "Domaine: 'ml' ou 'plot'."
    )
)]
#[sera_bind(serde)]
pub fn models_for_domain(domain: &str) -> Vec<&'static crate::model_registry::ModelInfo> {
    crate::model_registry::models_by_domain(domain)
}

#[sera_bind(serde)]
pub fn no_hover_html(html: String) -> String {
    Chart { html, doc_str: "" }.no_hover().html
}

#[sera_bind(serde)]
pub fn no_x_axis_html(html: String) -> String {
    Chart { html, doc_str: "" }.no_x_axis().html
}

#[sera_bind(serde)]
pub fn no_y_axis_html(html: String) -> String {
    Chart { html, doc_str: "" }.no_y_axis().html
}

#[sera_bind(serde)]
pub fn no_axes_html(html: String) -> String {
    Chart { html, doc_str: "" }.no_axes().html
}

#[sera_bind(serde)]
pub fn no_legend_html(html: String) -> String {
    Chart { html, doc_str: "" }.no_legend().html
}

#[sera_bind(serde)]
pub fn no_title_html(html: String) -> String {
    Chart { html, doc_str: "" }.no_title().html
}

#[sera_bind(serde)]
pub fn no_background_html(html: String) -> String {
    Chart { html, doc_str: "" }.no_background().html
}

#[sera_bind(serde)]
pub fn responsive_html(html: String) -> String {
    Chart { html, doc_str: "" }.responsive().html
}

#[sera_bind(serde)]
pub fn show_grid_html(html: String) -> String {
    Chart { html, doc_str: "" }.show_grid().html
}

#[sera_bind(serde)]
pub fn flip_html(html: String) -> String {
    Chart { html, doc_str: "" }.flip().html
}

#[sera_bind(serde)]
pub fn crosshair_html(html: String) -> String {
    Chart { html, doc_str: "" }.crosshair().html
}

#[sera_bind(serde)]
pub fn zoom_html(html: String) -> String {
    Chart { html, doc_str: "" }.zoom().html
}

#[sera_bind(serde)]
pub fn export_button_html(html: String) -> String {
    Chart { html, doc_str: "" }.export_button().html
}

#[sera_bind(serde)]
pub fn inject_css_html(html: String, css: String) -> String {
    Chart { html, doc_str: "" }.inject_css(&css).html
}

#[sera_bind(serde)]
pub fn inject_js_html(html: String, js: String) -> String {
    Chart { html, doc_str: "" }.inject_js(&js).html
}

#[sera_bind(serde)]
pub fn group_hover_opacity_html(html: String, dim: f64) -> String {
    Chart { html, doc_str: "" }.group_hover_opacity(dim).html
}

#[sera_bind(serde)]
pub fn desaturate_html(html: String, indices: Option<Vec<usize>>, factor: f64) -> String {
    Chart { html, doc_str: "" }.desaturate(indices, factor).html
}

#[sera_bind(serde)]
pub fn sparse_grid_html(html: String) -> String {
    Chart { html, doc_str: "" }.sparse_grid().html
}

#[sera_bind(serde)]
pub fn grid_y_html(html: String) -> String {
    Chart { html, doc_str: "" }.grid_y().html
}

#[sera_bind(serde)]
pub fn grid_x_html(html: String) -> String {
    Chart { html, doc_str: "" }.grid_x().html
}

#[sera_bind(serde)]
pub fn color_density_html(html: String) -> String {
    Chart { html, doc_str: "" }.color_density().html
}

#[sera_bind(serde)]
pub fn highlight_group_html(html: String, labels: Vec<String>, dim: f64) -> String {
    Chart { html, doc_str: "" }.highlight_group(labels, dim).html
}

#[sera_bind(serde)]
pub fn apply_color_bindings_html(html: String) -> String {
    Chart { html, doc_str: "" }.apply_color_bindings().html
}

#[sera_bind(serde)]
pub fn grid_at_html(html: String, value: f64, color: String, label: Option<String>) -> String {
    Chart { html, doc_str: "" }.grid_at(value, &color, label.as_deref()).html
}

#[sera_bind(serde)]
pub fn cut_bars_html(html: String, step: Option<f64>, gap: i32, color: Option<String>) -> String {
    Chart { html, doc_str: "" }.cut_bars(step, gap, color.as_deref()).html
}

#[sera_bind(serde)]
pub fn draw_tool_html(html: String, color: String) -> String {
    Chart { html, doc_str: "" }.draw_tool(&color).html
}

#[sera_bind(serde)]
pub fn no_select_html(html: String) -> String {
    Chart { html, doc_str: "" }.no_select().html
}

#[sera_bind(serde)]
pub fn hover_slots_html(html: String, slots_json: String) -> String {
    Chart { html, doc_str: "" }.hover_slots(&slots_json).html
}

#[cfg(any(feature = "python", feature = "ffi"))]
struct ThemePreset {
    bg: Option<&'static str>,
    palette: &'static [u32],
    gridlines: bool,
}

#[cfg(any(feature = "python", feature = "ffi"))]
const THEME_DARK: ThemePreset = ThemePreset {
    bg: Some("#0f172a"),
    palette: &[
        0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA, 0x22D3EE, 0xF472B6, 0xA3E635, 0xF87171,
        0x2DD4BF,
    ],
    gridlines: true,
};

#[cfg(any(feature = "python", feature = "ffi"))]
const THEME_LIGHT: ThemePreset = ThemePreset {
    bg: None,
    palette: &[
        0x636EFA, 0xEF553B, 0x00CC96, 0xAB63FA, 0xFFA15A, 0x19D3F3, 0xFF6692, 0xB6E880, 0xFF97FF,
        0xFECB52,
    ],
    gridlines: false,
};

#[cfg(any(feature = "python", feature = "ffi"))]
const THEME_SCIENTIFIC: ThemePreset = ThemePreset {
    bg: Some("#fafafa"),
    palette: &[
        0x1F77B4, 0xFF7F0E, 0x2CA02C, 0xD62728, 0x9467BD, 0x8C564B, 0xE377C2, 0x7F7F7F, 0xBCBD22,
        0x17BECF,
    ],
    gridlines: true,
};

#[cfg(any(feature = "python", feature = "ffi"))]
const THEME_APPLE: ThemePreset = ThemePreset {
    bg: Some("#000000"),
    palette: &[
        0x0A84FF, 0x30D158, 0xFF453A, 0xFFD60A, 0xBF5AF2, 0x64D2FF, 0xFF9F0A, 0xFF375F, 0xAC8E68,
        0x63E6E2,
    ],
    gridlines: false,
};

#[cfg(any(feature = "python", feature = "ffi"))]
const THEME_NOTION: ThemePreset = ThemePreset {
    bg: Some("#191919"),
    palette: &[
        0x529CCA, 0xD08B65, 0x6C9B7D, 0xCB7C7A, 0x9A6DD7, 0x868686, 0xCCAA55, 0x75B5AA, 0xD477A8,
        0x507AA6,
    ],
    gridlines: false,
};

#[cfg(any(feature = "python", feature = "ffi"))]
const THEME_MINIMAL: ThemePreset = ThemePreset {
    bg: None,
    palette: &[
        0x374151, 0x6B7280, 0x9CA3AF, 0xD1D5DB, 0x111827, 0x4B5563, 0x1F2937, 0xE5E7EB, 0x030712,
        0x6B7280,
    ],
    gridlines: false,
};

#[cfg(any(feature = "python", feature = "ffi"))]
const THEME_NEON: ThemePreset = ThemePreset {
    bg: Some("#0a0a0a"),
    palette: &[
        0x00FF87, 0xFF006E, 0x00B4D8, 0xFFBE0B, 0xE500A4, 0x8338EC, 0x3A86FF, 0xFB5607, 0xFF006E,
        0x06D6A0,
    ],
    gridlines: false,
};

#[cfg(any(feature = "python", feature = "ffi"))]
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
#[sera_doc(
    category = "theme",
    file = "theme/theme.md",
    en = "Applies a named color theme to all subsequent chart renders.",
    fr = "Applique un thème de couleurs nommé à tous les rendus de graphiques suivants.",
    param(
        name = "name",
        ty = "str",
        en = "Theme name (e.g. 'dark', 'light', 'ocean'). Use sp.themes() to list all.",
        fr = "Nom du thème (ex: 'dark', 'light', 'ocean'). Utilisez sp.themes() pour lister tous les thèmes."
    )
)]
#[sera_register]
#[pyfunction]
#[pyo3(signature = (name))]
pub fn theme(name: &str) -> PyResult<()> {
    let preset = resolve_theme(name).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err(format!(
            "Unknown theme '{}'. Available: dark, light, scientific, apple, notion, minimal, neon",
            name
        ))
    })?;
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() {
        *bg = preset.bg.map(|value| value.to_string());
    }
    if let Ok(mut palette) = GLOBAL_PALETTE.lock() {
        *palette = Some(preset.palette.to_vec());
    }
    GLOBAL_GRIDLINES.store(preset.gridlines, std::sync::atomic::Ordering::Relaxed);
    if let Ok(mut theme_name) = GLOBAL_THEME_NAME.lock() {
        *theme_name = Some(name.to_string());
    }
    crate::plot::set_global_bg(preset.bg.map(|value| value.to_string()));
    crate::plot::set_global_pal(preset.palette.to_vec());
    crate::plot::set_global_grid(preset.gridlines);
    Ok(())
}

#[sera_doc(
    category = "config",
    file = "config/config.md",
    en = "Sets global defaults applied to every chart created after this call. Pass only the keys you want to change; all are optional and keep their previous value when omitted.",
    fr = "Définit des valeurs par défaut globales appliquées à tous les graphiques créés après cet appel. Ne passez que les clés à modifier ; toutes sont optionnelles et conservent leur valeur précédente si omises.",
    param(name = "font", ty = "str", en = "Font family for every text element. Default: system font.", fr = "Police pour chaque élément texte. Défaut : police système."),
    param(name = "font_size", ty = "int", en = "Base font size in px. Default: 12.", fr = "Taille de police de base en px. Défaut : 12."),
    param(name = "title_size", ty = "int", en = "Title font size in px. Default: 16.", fr = "Taille de police du titre en px. Défaut : 16."),
    param(name = "border_radius", ty = "int", en = "Container border radius in px. Default: 0.", fr = "Rayon des coins du conteneur en px. Défaut : 0."),
    param(name = "opacity", ty = "float", en = "Element opacity, 0.0-1.0. Default: 1.0.", fr = "Opacité des éléments, 0.0-1.0. Défaut : 1.0."),
    param(name = "responsive", ty = "bool", en = "Auto-resize chart to its container width. Default: False.", fr = "Redimensionnement automatique selon la largeur du conteneur. Défaut : False."),
    param(name = "animation", ty = "bool", en = "Fade-in animation on chart load. Default: False.", fr = "Animation de fondu au chargement. Défaut : False."),
    param(name = "animation_duration", ty = "int", en = "Animation duration in ms. Default: 300.", fr = "Durée de l'animation en ms. Défaut : 300."),
    param(name = "crosshair", ty = "bool", en = "Crosshair lines follow mouse hover. Default: False.", fr = "Lignes de visée suivant la souris. Défaut : False."),
    param(name = "zoom", ty = "bool", en = "Mouse wheel zoom + pan, double-click resets. Default: False.", fr = "Zoom à la molette + pan, double-clic réinitialise. Défaut : False."),
    param(name = "tooltip", ty = "str", en = "Tooltip mode. Default: unset.", fr = "Mode d'infobulle. Défaut : non défini."),
    param(name = "locale", ty = "str", en = "Number formatting locale. Default: unset.", fr = "Locale de formatage des nombres. Défaut : non défini."),
    param(name = "thousands_sep", ty = "str", en = "Thousands separator character. Default: unset.", fr = "Caractère séparateur de milliers. Défaut : non défini."),
    param(name = "margin", ty = "int", en = "Container padding in px. Default: 0.", fr = "Marge interne du conteneur en px. Défaut : 0."),
    param(name = "export_button", ty = "bool", en = "Show a download button on every chart. Default: False.", fr = "Affiche un bouton de téléchargement sur chaque chart. Défaut : False."),
    param(name = "palette", ty = "list[int]", en = "Color palette as hex ints, e.g. [0x6366F1, 0xFB7185]. Default: unset.", fr = "Palette de couleurs en hex, ex. [0x6366F1, 0xFB7185]. Défaut : non défini."),
    param(name = "background", ty = "str", en = "Background color, any CSS color. Default: unset.", fr = "Couleur de fond, toute couleur CSS. Défaut : non défini."),
    param(name = "gridlines", ty = "bool", en = "Show grid lines in chart. Default: False.", fr = "Affiche la grille dans le chart. Défaut : False."),
    param(name = "text_auto", ty = "bool | str", en = "Display values on bars/markers. True for raw values, or a d3 format string like '.2s'. Default: False.", fr = "Affiche les valeurs sur barres/marqueurs. True pour brut, ou un format d3 comme '.2s'. Défaut : False."),
    param(name = "text_position", ty = "str", en = "'auto' / 'inside' / 'outside'. Default: 'auto'.", fr = "'auto' / 'inside' / 'outside'. Défaut : 'auto'."),
    param(name = "text_angle", ty = "int", en = "Value label rotation in degrees. Default: 0.", fr = "Rotation des étiquettes de valeur en degrés. Défaut : 0."),
    param(name = "text_font_size", ty = "int", en = "Max font size for value labels in px. Default: 12.", fr = "Taille de police max des étiquettes de valeur en px. Défaut : 12."),
    param(name = "text_font_color", ty = "str", en = "Value label color. Default: unset.", fr = "Couleur des étiquettes de valeur. Défaut : non défini."),
    param(name = "uniform_text_min_size", ty = "int", en = "Minimum px before a label is hidden. Default: 0.", fr = "Taille px minimale avant masquage de l'étiquette. Défaut : 0."),
    param(name = "uniform_text_mode", ty = "str", en = "'hide' small labels or 'show' overflow. Default: 'hide'.", fr = "'hide' masque les petites étiquettes ou 'show' autorise le débordement. Défaut : 'hide'."),
    param(name = "bar_corner_radius", ty = "int | str", en = "Bar corner radius in px, or a percent string like '20%' of bar width. Default: 0.", fr = "Rayon des coins de barre en px, ou un pourcentage comme '20%' de la largeur. Défaut : 0."),
    param(name = "despine", ty = "bool", en = "Remove the top/right axis spines on every chart. Default: False.", fr = "Retire les axes haut/droite sur chaque chart. Défaut : False."),
    param(name = "watermark_text", ty = "str", en = "Diagonal watermark text drawn across every chart. Default: unset (no watermark).", fr = "Texte de watermark diagonal dessiné sur chaque chart. Défaut : non défini (pas de watermark)."),
    param(name = "watermark_opacity", ty = "float", en = "Watermark text opacity, 0.0-1.0. Default: 0.08.", fr = "Opacité du texte de watermark, 0.0-1.0. Défaut : 0.08."),
    param(name = "shadow_blur", ty = "int", en = "Drop-shadow blur radius in px applied to chart elements. Default: 24.", fr = "Rayon de flou de l'ombre en px appliqué aux éléments du chart. Défaut : 24."),
    param(name = "shadow_color", ty = "str", en = "Drop-shadow color, any CSS color. Default: unset (no shadow).", fr = "Couleur de l'ombre, toute couleur CSS. Défaut : non défini (pas d'ombre).")
)]
#[cfg(feature = "python")]
#[sera_register]
#[pyfunction]
#[pyo3(signature = (*, font=None, font_size=None, title_size=None, border_radius=None, opacity=None, responsive=None, animation=None, animation_duration=None, crosshair=None, zoom=None, tooltip=None, locale=None, thousands_sep=None, margin=None, export_button=None, palette=None, background=None, gridlines=None, text_auto=None, text_position=None, text_angle=None, text_font_size=None, text_font_color=None, uniform_text_min_size=None, uniform_text_mode=None, bar_corner_radius=None, despine=None, watermark_text=None, watermark_opacity=None, shadow_blur=None, shadow_color=None))]
pub fn config(
    font: Option<&str>,
    font_size: Option<i32>,
    title_size: Option<i32>,
    border_radius: Option<i32>,
    opacity: Option<f64>,
    responsive: Option<bool>,
    animation: Option<bool>,
    animation_duration: Option<i32>,
    crosshair: Option<bool>,
    zoom: Option<bool>,
    tooltip: Option<&str>,
    locale: Option<&str>,
    thousands_sep: Option<&str>,
    margin: Option<i32>,
    export_button: Option<bool>,
    palette: Option<Vec<u32>>,
    background: Option<&str>,
    gridlines: Option<bool>,
    text_auto: Option<&Bound<'_, PyAny>>,
    text_position: Option<&str>,
    text_angle: Option<i32>,
    text_font_size: Option<i32>,
    text_font_color: Option<&str>,
    uniform_text_min_size: Option<i32>,
    uniform_text_mode: Option<&str>,
    bar_corner_radius: Option<&Bound<'_, PyAny>>,
    despine: Option<bool>,
    watermark_text: Option<&str>,
    watermark_opacity: Option<f64>,
    shadow_blur: Option<i32>,
    shadow_color: Option<&str>,
) {
    use std::sync::atomic::Ordering::Relaxed;
    if let Some(value) = despine {
        crate::plot::utils::set_global_despine(value);
    }
    if let Some(value) = watermark_text {
        crate::plot::utils::set_global_watermark(Some((
            value.to_string(),
            watermark_opacity.unwrap_or(0.08),
        )));
    }
    if shadow_blur.is_some() || shadow_color.is_some() {
        crate::plot::utils::set_global_shadow(Some((
            shadow_blur.unwrap_or(24),
            shadow_color.map(|s| s.to_string()),
        )));
    }
    if let Some(value) = font {
        if let Ok(mut field) = GLOBAL_FONT.lock() {
            *field = Some(value.to_string());
        }
    }
    if let Some(value) = font_size {
        GLOBAL_FONT_SIZE.store(value, Relaxed);
    }
    if let Some(value) = title_size {
        GLOBAL_TITLE_SIZE.store(value, Relaxed);
    }
    if let Some(value) = border_radius {
        GLOBAL_BORDER_RADIUS.store(value, Relaxed);
    }
    if let Some(value) = opacity {
        if let Ok(mut field) = GLOBAL_OPACITY.lock() {
            *field = if value < 1.0 { Some(value) } else { None };
        }
    }
    if let Some(value) = responsive {
        GLOBAL_RESPONSIVE.store(value, Relaxed);
    }
    if let Some(value) = animation {
        GLOBAL_ANIMATION.store(value, Relaxed);
    }
    if let Some(value) = animation_duration {
        GLOBAL_ANIMATION_DURATION.store(value, Relaxed);
    }
    if let Some(value) = crosshair {
        GLOBAL_CROSSHAIR.store(value, Relaxed);
    }
    if let Some(value) = zoom {
        GLOBAL_ZOOM.store(value, Relaxed);
    }
    if let Some(value) = tooltip {
        if let Ok(mut field) = GLOBAL_TOOLTIP.lock() {
            *field = Some(value.to_string());
        }
    }
    if let Some(value) = locale {
        if let Ok(mut field) = GLOBAL_LOCALE.lock() {
            *field = Some(value.to_string());
        }
    }
    if let Some(value) = thousands_sep {
        if let Ok(mut field) = GLOBAL_THOUSANDS_SEP.lock() {
            *field = Some(value.to_string());
        }
    }
    if let Some(value) = margin {
        GLOBAL_MARGIN.store(value, Relaxed);
    }
    if let Some(value) = export_button {
        GLOBAL_EXPORT_BTN.store(value, Relaxed);
    }
    if let Some(value) = background {
        if let Ok(mut field) = GLOBAL_BACKGROUND.lock() {
            *field = Some(value.to_string());
        }
        crate::plot::set_global_bg(Some(value.to_string()));
    }
    if let Some(value) = palette {
        if let Ok(mut field) = GLOBAL_PALETTE.lock() {
            *field = Some(value.clone());
        }
        crate::plot::set_global_pal(value);
    }
    if let Some(value) = gridlines {
        GLOBAL_GRIDLINES.store(value, Relaxed);
        crate::plot::set_global_grid(value);
    }
    if let Some(value) = text_auto {
        let text = if let Ok(boolean) = value.extract::<bool>() {
            if boolean {
                String::new()
            } else {
                reset_text_auto();
                return;
            }
        } else if let Ok(text) = value.extract::<String>() {
            text
        } else {
            return;
        };
        if let Ok(mut field) = GLOBAL_TEXT_AUTO.lock() {
            *field = Some(text);
        }
    }
    if let Some(value) = text_position {
        if let Ok(mut field) = GLOBAL_TEXT_POSITION.lock() {
            *field = Some(value.to_string());
        }
    }
    if let Some(value) = text_angle {
        GLOBAL_TEXT_ANGLE.store(value, Relaxed);
    }
    if let Some(value) = text_font_size {
        GLOBAL_TEXT_FONT_SIZE.store(value, Relaxed);
    }
    if let Some(value) = text_font_color {
        if let Ok(mut field) = GLOBAL_TEXT_FONT_COLOR.lock() {
            *field = Some(value.to_string());
        }
    }
    if let Some(value) = uniform_text_min_size {
        GLOBAL_UNIFORM_TEXT_MIN.store(value, Relaxed);
    }
    if let Some(value) = uniform_text_mode {
        if let Ok(mut field) = GLOBAL_UNIFORM_TEXT_MODE.lock() {
            *field = Some(value.to_string());
        }
    }
    if let Some(value) = bar_corner_radius {
        let radius = if let Ok(integer) = value.extract::<i32>() {
            integer.to_string()
        } else if let Ok(number) = value.extract::<f64>() {
            number.to_string()
        } else if let Ok(text) = value.extract::<String>() {
            text
        } else {
            return;
        };
        if let Ok(mut field) = GLOBAL_BAR_CORNER_RADIUS.lock() {
            *field = Some(radius);
        }
    }
}

#[cfg(feature = "python")]
fn reset_text_auto() {
    if let Ok(mut field) = GLOBAL_TEXT_AUTO.lock() {
        *field = None;
    }
}

static LIVE_STREAM_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(name = "LiveStream", module = "seraplot")
)]
pub struct LiveStream {
    kind: String,
    title: String,
    xs: Vec<f64>,
    ys: Vec<f64>,
    max_points: usize,
    color_hex: u32,
    width: i32,
    height: i32,
    display_id: String,
    started: bool,
}

impl LiveStream {
    fn render_html(&self) -> String {
        let base = serde_json::json!({
            "title": self.title,
            "x": self.xs,
            "y": self.ys,
            "color_hex": self.color_hex,
            "width": self.width,
            "height": self.height,
            "show_points": true
        });
        match self.kind.as_str() {
            "scatter" => crate::plot::statistical::scatter::build(&base.to_string()),
            _ => crate::plot::default::build_line_chart(
                &serde_json::json!({
                    "title": self.title,
                    "labels": self.xs.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
                    "values": self.ys,
                    "color_hex": self.color_hex,
                    "width": self.width,
                    "height": self.height,
                    "show_points": true
                })
                .to_string(),
            ),
        }
    }

    fn iframe_html(&self, html: &str) -> String {
        let esc = html.replace('&', "&amp;").replace('"', "&quot;");
        format!(
            r#"<iframe id="{}" srcdoc="{}" style="width:100%;max-width:{}px;aspect-ratio:{}/{};border:none;display:block;border-radius:8px;overflow:hidden" frameborder="0"></iframe>"#,
            self.display_id, esc, self.width, self.width, self.height
        )
    }

    fn cap(&mut self) {
        if self.xs.len() > self.max_points {
            let cut = self.xs.len() - self.max_points;
            self.xs.drain(..cut);
            self.ys.drain(..cut);
        }
    }

    #[cfg(feature = "python")]
    fn live_update(&mut self, py: Python<'_>) -> PyResult<()> {
        let wrapped = self.iframe_html(&self.render_html());
        let display_mod = py.import_bound("IPython.display")?;
        let html_obj = display_mod.getattr("HTML")?.call1((wrapped,))?;
        let kwargs = pyo3::types::PyDict::new_bound(py);
        kwargs.set_item("display_id", &self.display_id)?;
        if !self.started {
            display_mod
                .getattr("display")?
                .call((html_obj,), Some(&kwargs))?;
            self.started = true;
        } else {
            display_mod
                .getattr("update_display")?
                .call((html_obj,), Some(&kwargs))?;
        }
        Ok(())
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl LiveStream {
    #[new]
    #[pyo3(signature = (kind = "line", title = "", max_points = 500, color_hex = 0x636EFA, width = 900, height = 420))]
    fn new(kind: &str, title: &str, max_points: usize, color_hex: u32, width: i32, height: i32) -> Self {
        let n = LIVE_STREAM_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        LiveStream {
            kind: kind.to_string(),
            title: title.to_string(),
            xs: Vec::new(),
            ys: Vec::new(),
            max_points: max_points.max(2),
            color_hex,
            width,
            height,
            display_id: format!("sp-live-{}", n),
            started: false,
        }
    }

    fn push(&mut self, py: Python<'_>, x: f64, y: f64) -> PyResult<()> {
        self.xs.push(x);
        self.ys.push(y);
        self.cap();
        self.live_update(py)
    }

    #[pyo3(name = "append")]
    fn push_alias_append(&mut self, py: Python<'_>, x: f64, y: f64) -> PyResult<()> {
        self.push(py, x, y)
    }

    #[pyo3(name = "add_point")]
    fn push_alias_add_point(&mut self, py: Python<'_>, x: f64, y: f64) -> PyResult<()> {
        self.push(py, x, y)
    }

    fn extend(&mut self, py: Python<'_>, xs: Vec<f64>, ys: Vec<f64>) -> PyResult<()> {
        self.xs.extend(xs);
        self.ys.extend(ys);
        self.cap();
        self.live_update(py)
    }

    #[pyo3(name = "add_points")]
    fn extend_alias_add_points(&mut self, py: Python<'_>, xs: Vec<f64>, ys: Vec<f64>) -> PyResult<()> {
        self.extend(py, xs, ys)
    }

    #[pyo3(name = "append_many")]
    fn extend_alias_append_many(&mut self, py: Python<'_>, xs: Vec<f64>, ys: Vec<f64>) -> PyResult<()> {
        self.extend(py, xs, ys)
    }

    fn clear(&mut self, py: Python<'_>) -> PyResult<()> {
        self.xs.clear();
        self.ys.clear();
        self.live_update(py)
    }

    #[pyo3(name = "reset")]
    fn clear_alias_reset(&mut self, py: Python<'_>) -> PyResult<()> {
        self.clear(py)
    }

    fn render(&self) -> crate::Chart {
        crate::Chart {
            html: self.render_html(),
            doc_str: "",
        }
    }

    #[pyo3(name = "snapshot")]
    fn render_alias_snapshot(&self) -> crate::Chart {
        self.render()
    }

    #[pyo3(name = "to_chart")]
    fn render_alias_to_chart(&self) -> crate::Chart {
        self.render()
    }

    #[getter]
    fn n(&self) -> usize {
        self.xs.len()
    }

    #[getter]
    fn html(&self) -> String {
        self.render_html()
    }
}

inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "LiveStream",
        category: "chart_method",
        file: "config/streaming.md",
        en: "A bounded ring-buffer accumulator that turns a series of (x, y) samples arriving over time into a continuously redrawn chart. push()/extend()/clear() repaint the same Jupyter output cell in place via IPython's display_id, so the chart updates smoothly with no flicker or new cells.",
        fr: "Un accumulateur à buffer circulaire borné qui transforme une série d'échantillons (x, y) arrivant dans le temps en un graphique redessiné en continu. push()/extend()/clear() repeignent la même cellule de sortie Jupyter sur place via le display_id d'IPython, donc le graphique se met à jour sans scintillement ni nouvelle cellule.",
        params: &[
            crate::doc_registry::ParamDoc { name: "kind", ty: "str", en: "Chart kind: 'line' or 'scatter'. Default: 'line'.", fr: "Type de graphique : 'line' ou 'scatter'. Défaut : 'line'." },
            crate::doc_registry::ParamDoc { name: "title", ty: "str", en: "Chart title.", fr: "Titre du graphique." },
            crate::doc_registry::ParamDoc { name: "max_points", ty: "int", en: "Ring buffer size; oldest samples are dropped past this. Default: 500.", fr: "Taille du buffer circulaire ; les échantillons les plus anciens sont supprimés au-delà. Défaut : 500." },
            crate::doc_registry::ParamDoc { name: "color_hex", ty: "int", en: "Series color as a 24-bit RGB integer.", fr: "Couleur de la série en entier RGB 24 bits." },
            crate::doc_registry::ParamDoc { name: "width", ty: "int", en: "Canvas width in pixels.", fr: "Largeur du canvas en pixels." },
            crate::doc_registry::ParamDoc { name: "height", ty: "int", en: "Canvas height in pixels.", fr: "Hauteur du canvas en pixels." },
        ],
        aliases: &[],
    }
}
inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "LiveStream.push",
        category: "chart_method",
        file: "config/streaming.md",
        en: "Appends one (x, y) sample, enforces max_points, and redraws the live chart in place.",
        fr: "Ajoute un échantillon (x, y), applique max_points, et redessine le graphique live sur place.",
        params: &[
            crate::doc_registry::ParamDoc { name: "x", ty: "float", en: "X value.", fr: "Valeur X." },
            crate::doc_registry::ParamDoc { name: "y", ty: "float", en: "Y value.", fr: "Valeur Y." },
        ],
        aliases: &["append", "add_point"],
    }
}
inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "LiveStream.extend",
        category: "chart_method",
        file: "config/streaming.md",
        en: "Appends two lists of samples in lock-step, enforces max_points, and redraws the live chart in place.",
        fr: "Ajoute deux listes d'échantillons en parallèle, applique max_points, et redessine le graphique live sur place.",
        params: &[
            crate::doc_registry::ParamDoc { name: "xs", ty: "list[float]", en: "X values.", fr: "Valeurs X." },
            crate::doc_registry::ParamDoc { name: "ys", ty: "list[float]", en: "Y values.", fr: "Valeurs Y." },
        ],
        aliases: &["add_points", "append_many"],
    }
}
inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "LiveStream.clear",
        category: "chart_method",
        file: "config/streaming.md",
        en: "Empties the ring buffer and redraws the (now empty) live chart in place.",
        fr: "Vide le buffer circulaire et redessine le graphique live (désormais vide) sur place.",
        params: &[],
        aliases: &["reset"],
    }
}
inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "LiveStream.render",
        category: "chart_method",
        file: "config/streaming.md",
        en: "Renders the current buffer to a standalone Chart object, without touching the live display.",
        fr: "Rend le buffer courant sous forme d'objet Chart autonome, sans toucher à l'affichage live.",
        params: &[],
        aliases: &["snapshot", "to_chart"],
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn seraplot(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    if let Ok(sys) = py.import_bound("sys") {
        if let Ok(vi) = sys.getattr("version_info") {
            if let (Ok(maj), Ok(min), Ok(mic)) = (
                vi.getattr("major").and_then(|v| v.extract::<u32>()),
                vi.getattr("minor").and_then(|v| v.extract::<u32>()),
                vi.getattr("micro").and_then(|v| v.extract::<u32>()),
            ) {
                crate::telemetry::set_python_version(&format!("{}.{}.{}", maj, min, mic));
            }
        }
    }
    _py::__init(py, m)
}
