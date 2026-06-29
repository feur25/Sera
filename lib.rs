pub mod cloud;
pub mod core;
pub mod data;
#[path = "bindings/doc_registry.rs"]
pub mod doc_registry;
pub mod ml;
#[path = "bindings/model_registry.rs"]
pub mod model_registry;
pub mod plot;
pub mod telemetry;

pub use crate::core::hw_profile::HwProfile;
pub use data::{DataPoint, Dataset, DatasetStats};

#[allow(unused_imports)]
pub(crate) use crate::bindings::registry_macro::{
    for_each_json_chart_fn, for_each_ml_oneshot_fn, for_each_util_fn,
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
pub mod html;
#[cfg(feature = "python")]
pub mod python;
#[cfg(any(feature = "python", feature = "gui"))]
pub mod viewer;
pub mod wiki;
include!(concat!(env!("OUT_DIR"), "/adapters.rs"));

pub use core::math::{self, mean, median, std_dev};
pub use data::loader;
pub use plot::canvas::Canvas;

#[cfg(any(feature = "python", feature = "gui"))]
pub use viewer::gui;

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
    s.push_str("rb.style.cssText='display:none;width:22px;height:22px;border-radius:50%;background:rgba(0,0,0,.6);color:#f1f5f9;font-size:13px;cursor:pointer;border:1px solid rgba(255,255,255,.2);align-items:center;justify-content:center;flex-shrink:0;backdrop-filter:blur(4px);';");
    s.push_str("rb.addEventListener('click',function(){dis.forEach(function(d){d.b.style.display='';setTimeout(function(){d.b.style.opacity='1';},10);if(svg){if(d.se!=null)svg.querySelectorAll('[data-series=\"'+d.se+'\"]:not([data-legend])').forEach(function(e){e.style.display='';});else if(d.ix!=null)svg.querySelectorAll('[data-idx=\"'+d.ix+'\"]').forEach(function(e){e.style.display='';});}});dis=[];rb.style.display='none';});");
    s.push_str("items.forEach(function(it){");
    s.push_str("var b=document.createElement('span');");
    s.push_str("b.style.cssText='display:inline-flex;align-items:center;gap:5px;padding:3px 10px;border-radius:999px;font-size:11px;font-weight:600;cursor:pointer;user-select:none;transition:opacity .2s;background:rgba(0,0,0,.55);color:#f1f5f9;border:1px solid rgba(255,255,255,.15);backdrop-filter:blur(4px);white-space:nowrap;';");
    s.push_str("if(it.co){var d=document.createElement('span');d.style.cssText='width:8px;height:8px;border-radius:50%;flex-shrink:0;background:'+it.co+';';b.appendChild(d);}");
    s.push_str("b.appendChild(document.createTextNode(esc(it.lb)));");
    s.push_str("b.addEventListener('click',function(){b.style.opacity='0';setTimeout(function(){b.style.display='none';},200);dis.push({b:b,se:it.se,ix:it.idx});rb.style.display='inline-flex';if(svg){if(it.se!=null)svg.querySelectorAll('[data-series=\"'+it.se+'\"]:not([data-legend])').forEach(function(e){e.style.display='none';});else if(it.idx!=null)svg.querySelectorAll('[data-idx=\"'+it.idx+'\"]').forEach(function(e){e.style.display='none';});}});");
    s.push_str("ov.appendChild(b);});");
    s.push_str("ov.appendChild(rb);cont.appendChild(ov);");
    s.push_str("});})();</script></body>");
    s
}

fn encode_forced(labels: &[String], colors: &[String]) -> String {
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

fn extract_c3d_id(html: &str) -> Option<&str> {
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

fn inject_labels(html: &str, pos: &str, labels: &[String], colors: &[String]) -> String {
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

const SP_ZOOM_JS: &str = "(function(){if(window.__spz__)return;window.__spz__=1;var svg=document.querySelector('svg');if(!svg)return;var s=1,tx=0,ty=0,dr=false,sx,sy;svg.style.cursor='grab';svg.addEventListener('wheel',function(e){e.preventDefault();var z=e.deltaY<0?1.1:0.9;s=Math.min(Math.max(s*z,0.5),10);svg.style.transform='scale('+s+') translate('+tx+'px,'+ty+'px)';svg.style.transformOrigin='center center';},{passive:false});svg.addEventListener('mousedown',function(e){dr=true;sx=e.clientX-tx;sy=e.clientY-ty;svg.style.cursor='grabbing';});window.addEventListener('mouseup',function(){dr=false;if(svg)svg.style.cursor='grab';});svg.addEventListener('mousemove',function(e){if(!dr)return;tx=e.clientX-sx;ty=e.clientY-sy;svg.style.transform='scale('+s+') translate('+tx+'px,'+ty+'px)';});svg.addEventListener('dblclick',function(){s=1;tx=0;ty=0;svg.style.transform='';});})()";

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

pub(crate) fn apply_despine(html: String) -> String {
    html.replacen(
        "</head>",
        "<style>.sp-ax-x,.sp-ax-y{display:none!important}</style></head>",
        1,
    )
}

pub(crate) fn apply_watermark(html: String, text: &str, opacity: f64) -> String {
    let o = opacity.clamp(0.0, 1.0);
    let css = format!(
        "<style>.sp-watermark{{position:fixed;top:50%;left:50%;transform:translate(-50%,-50%) rotate(-28deg);font-size:42px;font-weight:800;letter-spacing:.08em;color:rgba(148,163,184,{o});pointer-events:none;z-index:1;white-space:nowrap;user-select:none;font-family:-apple-system,Arial,sans-serif}}</style></head>",
    );
    let html = html.replacen("</head>", &css, 1);
    let snippet = format!(
        "<div class=\"sp-watermark\">{}</div></body>",
        text.replace('&', "&amp;").replace('<', "&lt;")
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_caption(html: String, text: &str) -> String {
    let css = "<style>.sp-caption{text-align:center;font-size:11px;color:#94a3b8;margin:6px 0 0;font-family:-apple-system,Arial,sans-serif}</style></head>";
    let html = html.replacen("</head>", css, 1);
    let snippet = format!(
        "<script>window.__sp_caption__={};{}</script></body>",
        json_str(text),
        SP_CAPTION_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_glow(html: String, color: &str) -> String {
    let mut html = html;
    if let Some(svg_pos) = html.find("<svg") {
        if let Some(rel_end) = html[svg_pos..].find('>') {
            let insert_at = svg_pos + rel_end + 1;
            let defs = format!(
                "<defs><filter id=\"sp-glow-f\" x=\"-60%\" y=\"-60%\" width=\"220%\" height=\"160%\"><feDropShadow dx=\"0\" dy=\"0\" stdDeviation=\"4\" flood-color=\"{}\" flood-opacity=\"0.85\"/></filter></defs>",
                color
            );
            html.insert_str(insert_at, &defs);
        }
    }
    html.replacen(
        "</head>",
        "<style>svg [data-idx]{filter:url(#sp-glow-f)}</style></head>",
        1,
    )
}

fn extract_bracket_range(html: &str, marker: &str) -> Option<(f64, f64)> {
    let start = html.find(marker)? + marker.len();
    let end = html[start..].find(']')? + start;
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for tok in html[start..end].split(',') {
        if let Ok(v) = tok.trim().parse::<f64>() {
            if v < lo {
                lo = v;
            }
            if v > hi {
                hi = v;
            }
        }
    }
    if lo.is_finite() && hi.is_finite() {
        Some((lo, hi))
    } else {
        None
    }
}

fn extract_attr_range(html: &str, attr: &str) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    let mut from = 0usize;
    while let Some(rel) = html[from..].find(attr) {
        let start = from + rel + attr.len();
        let end = start + html[start..].find('"')?;
        if let Ok(v) = html[start..end].parse::<f64>() {
            if v < lo {
                lo = v;
            }
            if v > hi {
                hi = v;
            }
        }
        from = end;
    }
    if lo.is_finite() && hi.is_finite() {
        Some((lo, hi))
    } else {
        None
    }
}

pub(crate) fn apply_colorbar(html: String, position: &str) -> String {
    if html.contains("box-shadow:0 2px 8px rgba(0,0,0,.25),0 0 0 1px rgba(255,255,255,.15)") {
        return html;
    }
    let pos = match position {
        "left" | "right" | "top" | "bottom" => position,
        _ => "right",
    };
    let is_3d = html.contains("class=\"c3w\"");
    let range = if is_3d {
        extract_bracket_range(&html, ",Z=[")
    } else {
        extract_attr_range(&html, "data-v=\"").or_else(|| extract_attr_range(&html, "data-y=\""))
    };
    let (lo, hi) = range.unwrap_or((0.0, 1.0));
    let horizontal = pos == "top" || pos == "bottom";
    let gradient = if horizontal {
        "linear-gradient(to right,#00008f,#00ffff,#00ff00,#ffff00,#ff0000)"
    } else {
        "linear-gradient(to top,#00008f,#00ffff,#00ff00,#ffff00,#ff0000)"
    };
    let (css_pos, w, h) = match pos {
        "left" => ("top:50%;left:10px;transform:translateY(-50%)", 14, 150),
        "top" => ("top:10px;left:50%;transform:translateX(-50%)", 150, 14),
        "bottom" => ("bottom:10px;left:50%;transform:translateX(-50%)", 150, 14),
        _ => ("top:50%;right:10px;transform:translateY(-50%)", 14, 150),
    };
    let label_css = if horizontal {
        "position:absolute;top:16px;font-size:10px;color:#94a3b8;font-family:-apple-system,Arial,sans-serif"
    } else {
        "position:absolute;left:18px;font-size:10px;color:#94a3b8;font-family:-apple-system,Arial,sans-serif"
    };
    let (lo_pos, hi_pos) = if horizontal {
        ("left:0", "right:0")
    } else {
        ("bottom:0", "top:0")
    };
    let bar = format!(
        "<div style=\"position:absolute;{};width:{}px;height:{}px;background:{};border-radius:4px;box-shadow:0 2px 8px rgba(0,0,0,.25),0 0 0 1px rgba(255,255,255,.15);z-index:50\"><span style=\"{};{}\">{:.2}</span><span style=\"{};{}\">{:.2}</span></div>",
        css_pos, w, h, gradient, label_css, lo_pos, lo, label_css, hi_pos, hi
    );
    if is_3d {
        html.replacen("</button></div>", &format!("</button>{}</div>", bar), 1)
    } else {
        html.replacen(
            "</div></body></html>",
            &format!("{}</div></body></html>", bar),
            1,
        )
    }
}

pub(crate) fn apply_orient3d(html: String, mode: &str) -> String {
    let (yaw, pitch) = crate::plot::scene3d::Orientation3D::from_str(mode).angles();
    let marker = "var yaw=";
    if let Some(start) = html.find(marker) {
        if let Some(rel) = html[start..].find(",zoom=") {
            let end = start + rel;
            let mut out = String::with_capacity(html.len() + 16);
            out.push_str(&html[..start]);
            out.push_str(&format!("var yaw={:.4},pitch={:.4}", yaw, pitch));
            out.push_str(&html[end..]);
            return out;
        }
    }
    html
}

pub(crate) fn apply_highlight(html: String, index: usize, color: &str) -> String {
    let css = format!(
        "<style>svg [data-idx=\"{}\"]{{fill:{}!important;stroke:{}!important;filter:drop-shadow(0 0 6px {})}}</style></head>",
        index, color, color, color
    );
    html.replacen("</head>", &css, 1)
}

pub(crate) fn apply_hline(html: String, value: f64, color: &str, label: Option<&str>) -> String {
    let cfg = format!(
        "{{\"v\":{},\"c\":{},\"lbl\":{}}}",
        value,
        json_str(color),
        label.map(json_str).unwrap_or_else(|| "null".to_string())
    );
    let snippet = format!("<script>window.__sp_hline__={};{}</script></body>", cfg, SP_HLINE_JS);
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_trendline(html: String, color: &str, width: f64) -> String {
    let cfg = format!("{{\"c\":{},\"w\":{}}}", json_str(color), width);
    let snippet = format!(
        "<script>window.__sp_trendline__={};{}</script></body>",
        cfg, SP_TRENDLINE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_annotate_extreme(
    html: String,
    mode: &str,
    color: &str,
    label: Option<&str>,
) -> String {
    let cfg = format!(
        "{{\"mode\":{},\"c\":{},\"lbl\":{}}}",
        json_str(mode),
        json_str(color),
        label.map(json_str).unwrap_or_else(|| "null".to_string())
    );
    let snippet = format!(
        "<script>window.__sp_annotate_extreme__={};{}</script></body>",
        cfg, SP_ANNOTATE_EXTREME_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_reference_band(html: String, low: f64, high: f64, color: &str, opacity: f64) -> String {
    let cfg = format!(
        "{{\"lo\":{},\"hi\":{},\"c\":{},\"op\":{}}}",
        low,
        high,
        json_str(color),
        opacity.clamp(0.0, 1.0)
    );
    let snippet = format!(
        "<script>window.__sp_ref_band__={};{}</script></body>",
        cfg, SP_REFERENCE_BAND_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_responsive(html: String) -> String {
    html.replacen(
        "</head>",
        "<style>body>div{max-width:100%;min-width:0}svg{max-width:100%;height:auto;display:block}</style></head>",
        1,
    )
}

pub(crate) fn apply_value_labels(html: String, decimals: i32, color: &str) -> String {
    let cfg = format!("{{\"d\":{},\"c\":{}}}", decimals.max(0), json_str(color));
    let snippet = format!(
        "<script>{}window.__sp_value_labels__={};{}</script></body>",
        SP_STACK_INIT_JS, cfg, SP_VALUE_LABELS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_error_bars(html: String, margin: f64, color: &str) -> String {
    let cfg = format!("{{\"m\":{},\"c\":{}}}", margin.abs(), json_str(color));
    let snippet = format!(
        "<script>{}window.__sp_error_bars__={};{}</script></body>",
        SP_STACK_INIT_JS, cfg, SP_ERROR_BARS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_delta_labels(html: String, pos_color: &str, neg_color: &str) -> String {
    let cfg = format!(
        "{{\"pc\":{},\"nc\":{}}}",
        json_str(pos_color),
        json_str(neg_color)
    );
    let snippet = format!(
        "<script>{}window.__sp_delta_labels__={};{}</script></body>",
        SP_STACK_INIT_JS, cfg, SP_DELTA_LABELS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_cumulative_line(html: String, color: &str) -> String {
    let cfg = format!("{{\"c\":{}}}", json_str(color));
    let snippet = format!(
        "<script>window.__sp_cumulative_line__={};{}</script></body>",
        cfg, SP_CUMULATIVE_LINE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_rank_badges(html: String, top_n: usize, color: &str) -> String {
    let cfg = format!("{{\"n\":{},\"c\":{}}}", top_n.max(1), json_str(color));
    let snippet = format!(
        "<script>{}window.__sp_rank_badges__={};{}</script></body>",
        SP_STACK_INIT_JS, cfg, SP_RANK_BADGES_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_log_scale(html: String) -> String {
    let snippet = format!("<script>{}</script></body>", SP_LOG_SCALE_JS);
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_moving_average(html: String, window: usize, color: &str) -> String {
    let cfg = format!("{{\"w\":{},\"c\":{}}}", window.max(1), json_str(color));
    let snippet = format!(
        "<script>window.__sp_moving_avg__={};{}</script></body>",
        cfg, SP_MOVING_AVG_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_outliers(html: String, threshold_std: f64, color: &str) -> String {
    let cfg = format!("{{\"t\":{},\"c\":{}}}", threshold_std.max(0.1), json_str(color));
    let snippet = format!(
        "<script>window.__sp_outliers__={};{}</script></body>",
        cfg, SP_OUTLIERS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_fill_between(html: String, color: &str, opacity: f64) -> String {
    let cfg = format!("{{\"c\":{},\"op\":{}}}", json_str(color), opacity.clamp(0.0, 1.0));
    let snippet = format!(
        "<script>window.__sp_fill_between__={};{}</script></body>",
        cfg, SP_FILL_BETWEEN_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_box_annotate(html: String, color: &str) -> String {
    let cfg = format!("{{\"c\":{}}}", json_str(color));
    let snippet = format!(
        "<script>window.__sp_box_annotate__={};{}</script></body>",
        cfg, SP_BOX_ANNOTATE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_pct_of_total(html: String, decimals: i32, color: &str) -> String {
    let cfg = format!(
        "{{\"d\":{},\"c\":{}}}",
        decimals.max(0),
        json_str(color)
    );
    let snippet = format!(
        "<script>{}window.__sp_pct_of_total__={};{}</script></body>",
        SP_STACK_INIT_JS, cfg, SP_PCT_OF_TOTAL_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_correlation_badge(html: String, color: &str) -> String {
    let cfg = format!("{{\"c\":{}}}", json_str(color));
    let snippet = format!(
        "<script>window.__sp_correlation_badge__={};{}</script></body>",
        cfg, SP_CORRELATION_BADGE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_highlight_range(html: String, low: usize, high: usize, color: &str, opacity: f64) -> String {
    let cfg = format!(
        "{{\"lo\":{},\"hi\":{},\"c\":{},\"op\":{}}}",
        low,
        high,
        json_str(color),
        opacity.clamp(0.0, 1.0)
    );
    let snippet = format!(
        "<script>window.__sp_highlight_range__={};{}</script></body>",
        cfg, SP_HIGHLIGHT_RANGE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_iqr_band(html: String, color: &str, opacity: f64) -> String {
    let cfg = format!("{{\"c\":{},\"op\":{}}}", json_str(color), opacity.clamp(0.0, 1.0));
    let snippet = format!(
        "<script>window.__sp_iqr_band__={};{}</script></body>",
        cfg, SP_IQR_BAND_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_growth_badge(html: String, color: &str) -> String {
    let cfg = format!("{{\"c\":{}}}", json_str(color));
    let snippet = format!(
        "<script>window.__sp_growth_badge__={};{}</script></body>",
        cfg, SP_GROWTH_BADGE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_zscore_heat(html: String) -> String {
    let snippet = format!(
        "<script>window.__sp_zscore_heat__=true;{}</script></body>",
        SP_ZSCORE_HEAT_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_pareto_marker(html: String, threshold_pct: f64, color: &str) -> String {
    let cfg = format!(
        "{{\"t\":{},\"c\":{}}}",
        threshold_pct.clamp(1.0, 99.0),
        json_str(color)
    );
    let snippet = format!(
        "<script>window.__sp_pareto_marker__={};{}</script></body>",
        cfg, SP_PARETO_MARKER_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_diff_from_mean(html: String, pos_color: &str, neg_color: &str) -> String {
    let cfg = format!(
        "{{\"pc\":{},\"nc\":{}}}",
        json_str(pos_color),
        json_str(neg_color)
    );
    let snippet = format!(
        "<script>{}window.__sp_diff_from_mean__={};{}</script></body>",
        SP_STACK_INIT_JS, cfg, SP_DIFF_FROM_MEAN_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_rolling_std_band(html: String, window: usize, color: &str, opacity: f64) -> String {
    let cfg = format!(
        "{{\"w\":{},\"c\":{},\"op\":{}}}",
        window.max(1),
        json_str(color),
        opacity.clamp(0.0, 1.0)
    );
    let snippet = format!(
        "<script>window.__sp_rolling_std_band__={};{}</script></body>",
        cfg, SP_ROLLING_STD_BAND_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_forecast_line(html: String, periods: usize, color: &str) -> String {
    let cfg = format!("{{\"n\":{},\"c\":{}}}", periods.max(1), json_str(color));
    let snippet = format!(
        "<script>window.__sp_forecast_line__={};{}</script></body>",
        cfg, SP_FORECAST_LINE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_percentile_band(html: String, low_pct: f64, high_pct: f64, color: &str, opacity: f64) -> String {
    let cfg = format!(
        "{{\"lo\":{},\"hi\":{},\"c\":{},\"op\":{}}}",
        low_pct.clamp(0.0, 100.0),
        high_pct.clamp(0.0, 100.0),
        json_str(color),
        opacity.clamp(0.0, 1.0)
    );
    let snippet = format!(
        "<script>window.__sp_percentile_band__={};{}</script></body>",
        cfg, SP_PERCENTILE_BAND_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_scatter_regression(html: String, color: &str, width: f64) -> String {
    let cfg = format!("{{\"c\":{},\"w\":{}}}", json_str(color), width.max(0.5));
    let snippet = format!(
        "<script>window.__sp_scatter_regression__={};{}</script></body>",
        cfg, SP_SCATTER_REGRESSION_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_cluster(html: String, eps: f64, min_samples: usize) -> String {
    let cfg = format!("{{\"eps\":{},\"m\":{}}}", eps.max(0.001), min_samples.max(1));
    let snippet = format!(
        "<script>window.__sp_cluster__={};{}</script></body>",
        cfg, SP_CLUSTER_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_subtitle(html: String, text: &str) -> String {
    let snippet = format!(
        "<script>(function(){{var t=document.querySelector('.sp-ttl');if(!t)return;var ns='http://www.w3.org/2000/svg';var s=document.createElementNS(ns,'text');s.setAttribute('x',t.getAttribute('x')||'0');s.setAttribute('y',(parseFloat(t.getAttribute('y'))||0)+16);s.setAttribute('text-anchor',t.getAttribute('text-anchor')||'middle');s.setAttribute('font-family','-apple-system,Arial,sans-serif');s.setAttribute('font-size','12');s.setAttribute('fill','#94a3b8');s.setAttribute('class','sp-subtitle');s.textContent={};t.parentNode.insertBefore(s,t.nextSibling);}})();</script></body>",
        json_str(text)
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_shadow(html: String, blur: i32, color: &str) -> String {
    let css = format!(
        "<style>body>:first-child{{box-shadow:0 {}px {}px -8px {} !important}}</style></head>",
        blur / 2,
        blur,
        color
    );
    html.replacen("</head>", &css, 1)
}

pub(crate) fn apply_pulse(
    html: String,
    duration: f64,
    indices: Option<&[usize]>,
    above: Option<f64>,
) -> String {
    let d = duration.max(0.1);
    let keyframes = "@keyframes sp-pulse{0%,100%{opacity:1}50%{opacity:.55}}";
    if let Some(thr) = above {
        let css = format!("<style>{}</style></head>", keyframes);
        let html = html.replacen("</head>", &css, 1);
        let snippet = format!(
            "<script>(function(){{var thr={},d={};document.querySelectorAll('svg [data-v]').forEach(function(e){{if(parseFloat(e.getAttribute('data-v'))>thr){{e.style.setProperty('animation','sp-pulse '+d+'s ease-in-out infinite','important');}}}});}})();</script></body>",
            thr, d
        );
        return html.replacen("</body>", &snippet, 1);
    }
    let selector = match indices {
        Some(idxs) if !idxs.is_empty() => idxs
            .iter()
            .map(|i| format!("svg [data-idx=\"{}\"]", i))
            .collect::<Vec<_>>()
            .join(","),
        _ => "svg [data-idx]".to_string(),
    };
    let css = format!(
        "<style>{}{}{{animation:sp-pulse {}s ease-in-out infinite !important}}</style></head>",
        keyframes, selector, d
    );
    html.replacen("</head>", &css, 1)
}

pub(crate) fn apply_outline(html: String, color: &str, width: f64) -> String {
    let css = format!(
        "<style>svg [data-idx]{{stroke:{} !important;stroke-width:{}px !important}}</style></head>",
        color, width
    );
    html.replacen("</head>", &css, 1)
}

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
        fn sniff(s: &str, attr: &str, lo: u32, hi: u32, default: u32) -> u32 {
            let needle = format!("{}=\"", attr);
            let mut start = 0usize;
            loop {
                match s[start..].find(needle.as_str()) {
                    None => return default,
                    Some(rel) => {
                        let abs = start + rel + needle.len();
                        if let Some(end) = s[abs..].find('"') {
                            if let Ok(v) = s[abs..abs + end].parse::<u32>() {
                                if v >= lo && v <= hi {
                                    return v;
                                }
                            }
                        }
                        start += rel + 1;
                    }
                }
            }
        }
        let w = sniff(&self.html, "width", 150, 2000, 900);
        let h = sniff(&self.html, "height", 150, 1600, 560) + 24;
        let clean = self.html.replace(
            "border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)",
            "border-radius:0;overflow:hidden",
        );
        let esc = clean.replace('&', "&amp;").replace('"', "&quot;");
        format!(
            r#"<iframe srcdoc="{esc}" style="width:100%;max-width:{w}px;aspect-ratio:{w}/{h};border:none;display:block;border-radius:8px;overflow:hidden" frameborder="0"></iframe>"#
        )
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

#[sera_impl(html_display, pickle, export)]
impl Chart {
    #[sera_python_skip]
    pub fn html_str(&self) -> &str {
        &self.html
    }

    #[sera_python_skip]
    pub fn doc_str_val(&self) -> &'static str {
        self.doc_str
    }

    #[sera_python_skip]
    pub fn iframe(&self) -> String {
        self.chart_iframe()
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/export.md",
        en = "Saves the chart HTML to a file at the given path.",
        fr = "Enregistre le HTML du graphique dans un fichier au chemin indiqué.",
        aliases("save_html", "write", "export_html"),
        param(
            name = "path",
            ty = "str",
            en = "Destination file path (e.g. 'chart.html').",
            fr = "Chemin du fichier de destination (ex: 'chart.html')."
        )
    )]
    #[sera_sig(path)]
    #[sera_wasm_skip]
    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, &self.html)
    }

    #[sera_doc(
        category = "chart_method",
        aliases("bg", "background"),
        file = "charts/chart.md",
        en = "Sets the background color of the chart. Pass None to remove the background.",
        fr = "Définit la couleur d'arrière-plan du graphique. Passez None pour supprimer l'arrière-plan.",
        param(
            name = "color",
            ty = "str | None",
            en = "CSS color string (hex, rgb, named). None removes the background.",
            fr = "Couleur CSS (hex, rgb, nommée). None supprime l'arrière-plan."
        )
    )]
    #[sera_sig(color=None)]
    pub fn set_bg(&self, color: Option<&str>) -> Chart {
        self.propagate(crate::html::hover::apply_bg(self.html.clone(), color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("transparent_bg", "no_bg"),
        file = "charts/chart.md",
        en = "Removes every chart background layer and keeps the output transparent.",
        fr = "Supprime toutes les couches d'arrière-plan du graphique et conserve une sortie transparente."
    )]
    pub fn no_background(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>html,body,.chart-container,.c3w,.sp-wrap,svg,canvas{background:transparent!important}.sp-bg{fill:transparent!important}body>:first-child{box-shadow:none!important}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("css", "custom_css"),
        file = "charts/chart.md",
        en = "Injects a raw CSS string into the chart's <head> element.",
        fr = "Injecte une chaîne CSS brute dans l'élément <head> du graphique.",
        param(
            name = "css",
            ty = "str",
            en = "Raw CSS rules to inject.",
            fr = "Règles CSS brutes à injecter."
        )
    )]
    #[sera_sig(css)]
    pub fn inject_css(&self, css: &str) -> Chart {
        self.propagate(
            self.html
                .replacen("</head>", &format!("<style>{css}</style></head>"), 1),
        )
    }

    #[sera_doc(
        category = "chart_method",
        aliases("js", "custom_js"),
        file = "charts/chart.md",
        en = "Injects a raw JavaScript string into the chart's <body> element.",
        fr = "Injecte une chaîne JavaScript brute dans l'élément <body> du graphique.",
        param(
            name = "js",
            ty = "str",
            en = "Raw JavaScript code to inject.",
            fr = "Code JavaScript brut à injecter."
        )
    )]
    #[sera_sig(js)]
    pub fn inject_js(&self, js: &str) -> Chart {
        self.propagate(
            self.html
                .replacen("</body>", &format!("<script>{js}</script></body>"), 1),
        )
    }

    #[sera_doc(
        category = "chart_method",
        aliases("hide_x"),
        file = "charts/chart.md",
        en = "Hides the X axis, its ticks, and its label.",
        fr = "Masque l'axe X, ses graduations et son étiquette."
    )]
    pub fn no_x_axis(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>.sp-ax-x,.sp-xt,.sp-xl{display:none}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("no_tooltip"),
        file = "charts/chart.md",
        en = "Disables the hover tooltip and removes hover highlighting on data elements.",
        fr = "Désactive l'infobulle au survol et supprime le surlignage des éléments au survol."
    )]
    pub fn no_hover(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>#sp-tip{display:none!important}[data-idx]{pointer-events:none!important}[data-idx]:hover{filter:none!important}</style></head>", 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("hide_y"),
        file = "charts/chart.md",
        en = "Hides the Y axis, its ticks, and its label.",
        fr = "Masque l'axe Y, ses graduations et son étiquette."
    )]
    pub fn no_y_axis(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>.sp-ax-y,.sp-yt,.sp-yl{display:none}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("hide_axes"),
        file = "charts/chart.md",
        en = "Hides both X and Y axes along with their ticks and labels.",
        fr = "Masque les axes X et Y ainsi que leurs graduations et étiquettes."
    )]
    pub fn no_axes(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>.sp-ax-x,.sp-ax-y,.sp-xt,.sp-yt,.sp-xl,.sp-yl{display:none}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("grid"),
        file = "charts/chart.md",
        en = "Shows horizontal and vertical grid lines on the chart background.",
        fr = "Affiche les lignes de grille horizontales et verticales en arrière-plan du graphique."
    )]
    pub fn show_grid(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>.sp-gl{display:block!important;opacity:1!important}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("no_grid"),
        file = "charts/chart.md",
        en = "Hides the grid lines if they were previously enabled.",
        fr = "Masque les lignes de grille si elles étaient précédemment activées."
    )]
    pub fn hide_grid(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>.sp-gl{display:none!important}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("font_size"),
        file = "charts/chart.md",
        en = "Overrides all SVG text elements to the specified font size in pixels.",
        fr = "Remplace la taille de police de tous les éléments texte SVG par la valeur spécifiée en pixels.",
        param(
            name = "px",
            ty = "int",
            en = "Font size in pixels.",
            fr = "Taille de police en pixels."
        )
    )]
    #[sera_sig(px)]
    pub fn set_font_size(&self, px: u32) -> Chart {
        let style = format!(
            "<style>svg text{{font-size:{}px!important}}</style></head>",
            px
        );
        self.propagate(self.html.replacen("</head>", &style, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("resize"),
        file = "charts/chart.md",
        en = "Scales the entire SVG by a given factor from the top-left origin.",
        fr = "Met à l'échelle l'intégralité du SVG par un facteur donné depuis le coin supérieur gauche.",
        param(
            name = "factor",
            ty = "float",
            en = "Scale multiplier (e.g. 1.5 for 150%).",
            fr = "Multiplicateur d'échelle (ex: 1.5 pour 150%)."
        )
    )]
    #[sera_sig(factor)]
    pub fn scale(&self, factor: f64) -> Chart {
        let style = format!(
            "<style>svg{{transform:scale({});transform-origin:top left}}</style></head>",
            factor
        );
        self.propagate(self.html.replacen("</head>", &style, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("frame"),
        file = "charts/chart.md",
        en = "Sets the background color of the SVG/canvas frame. Use 'transparent' or None to remove it.",
        fr = "Définit la couleur d'arrière-plan du cadre SVG/canvas. Utilisez 'transparent' ou None pour le supprimer.",
        param(
            name = "color",
            ty = "str | None",
            en = "CSS color for the frame background.",
            fr = "Couleur CSS pour l'arrière-plan du cadre."
        )
    )]
    #[sera_sig(color=None)]
    pub fn set_frame(&self, color: Option<&str>) -> Chart {
        let bg = match color {
            None | Some("none") | Some("transparent") | Some("") => "transparent".to_string(),
            Some(c) => c.to_string(),
        };
        let style = format!("<style>svg{{background:{bg}!important}}.c3w canvas{{background:{bg}!important}}.c3w{{background:{bg}!important}}</style></head>");
        self.propagate(self.html.replacen("</head>", &style, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("labels", "legend_labels"),
        file = "charts/chart.md",
        en = "Adds an interactive series filter overlay with clickable pill-shaped labels.",
        fr = "Ajoute une superposition de filtre de séries interactif avec des étiquettes en forme de pilule cliquables.",
        param(
            name = "position",
            ty = "str",
            en = "Position of the overlay: 'top', 'bottom', 'left', 'right'.",
            fr = "Position de la superposition: 'top', 'bottom', 'left', 'right'."
        ),
        param(
            name = "labels",
            ty = "list[str] | None",
            en = "Custom label names. Auto-detected if None.",
            fr = "Noms d'étiquettes personnalisés. Détection automatique si None."
        ),
        param(
            name = "colors",
            ty = "list[str] | None",
            en = "Custom color hex strings matching labels.",
            fr = "Couleurs hex personnalisées correspondant aux étiquettes."
        )
    )]
    #[sera_sig(position="bottom", labels=None, colors=None)]
    pub fn show_labels(
        &self,
        position: &str,
        labels: Option<Vec<String>>,
        colors: Option<Vec<String>>,
    ) -> Chart {
        let lb = labels.unwrap_or_default();
        let co = colors.unwrap_or_default();
        self.propagate(inject_labels(&self.html, position, &lb, &co))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/export.md",
        en = "Extracts and returns the raw SVG string from the chart HTML, or None if not present.",
        fr = "Extrait et retourne la chaîne SVG brute depuis le HTML du graphique, ou None si absente.",
        aliases("svg", "as_svg", "get_svg")
    )]
    pub fn to_svg(&self) -> Option<String> {
        let h = &self.html;
        let start = h.find("<svg")?;
        let end = h.rfind("</svg>")? + 6;
        Some(h[start..end].to_string())
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/export.md",
        en = "Saves the chart's SVG to a file.",
        fr = "Enregistre le SVG du graphique dans un fichier.",
        aliases("save_svg", "svg_export", "write_svg"),
        param(
            name = "path",
            ty = "str",
            en = "Destination .svg file path.",
            fr = "Chemin du fichier .svg de destination."
        )
    )]
    #[sera_sig(path)]
    #[sera_wasm_skip]
    pub fn export_svg(&self, path: &str) -> Result<(), std::io::Error> {
        let svg = self
            .to_svg()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "No SVG in chart"))?;
        std::fs::write(path, svg)
    }

    #[sera_doc(
        category = "chart_method",
        aliases("set_font", "font_family"),
        file = "charts/chart.md",
        en = "Sets the font family for all SVG text and body text in the chart.",
        fr = "Définit la famille de polices pour tous les textes SVG et les textes du corps du graphique.",
        param(
            name = "name",
            ty = "str",
            en = "Font family name (e.g. 'Roboto', 'Inter').",
            fr = "Nom de la famille de polices (ex: 'Roboto', 'Inter')."
        )
    )]
    #[sera_sig(name)]
    pub fn font(&self, name: &str) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>svg text,body{{font-family:'{}',system-ui,sans-serif!important}}</style></head>", name), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("ttl_size"),
        file = "charts/chart.md",
        en = "Sets the font size of the chart title in pixels.",
        fr = "Définit la taille de police du titre du graphique en pixels.",
        param(
            name = "px",
            ty = "int",
            en = "Title font size in pixels.",
            fr = "Taille de police du titre en pixels."
        )
    )]
    #[sera_sig(px)]
    pub fn title_size(&self, px: i32) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            &format!(
                "<style>.sp-ttl{{font-size:{}px!important}}</style></head>",
                px
            ),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("xhair"),
        file = "charts/chart.md",
        en = "Adds an interactive crosshair that follows the mouse cursor across the SVG.",
        fr = "Ajoute un réticule interactif qui suit le curseur de la souris sur le SVG."
    )]
    pub fn crosshair(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_CROSSHAIR_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("magnify"),
        file = "charts/chart.md",
        en = "Enables mouse-wheel zoom and click-drag panning on the chart. Double-click to reset.",
        fr = "Active le zoom à la molette et le déplacement par glisser-cliquer. Double-clic pour réinitialiser."
    )]
    pub fn zoom(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_ZOOM_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("swap_axes", "transpose"),
        file = "charts/chart.md",
        en = "Flips a vertical bar chart into a horizontal bar chart by recalculating bar positions.",
        fr = "Transforme un graphique à barres verticales en graphique à barres horizontales en recalculant les positions."
    )]
    pub fn flip(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_FLIP_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("hbar", "flip_h"),
        file = "charts/chart.md",
        en = "Alias for flip(). Renders the chart with horizontal bars.",
        fr = "Alias de flip(). Affiche le graphique avec des barres horizontales."
    )]
    pub fn horizontal(&self) -> Chart {
        self.flip()
    }

    #[sera_doc(
        category = "chart_method",
        aliases("spin"),
        file = "charts/chart.md",
        en = "Rotates the entire chart by a snapped angle (0, 90, 180 or 270 degrees).",
        fr = "Fait pivoter l'intégralité du graphique selon un angle arrondi (0, 90, 180 ou 270 degrés).",
        param(
            name = "deg",
            ty = "int",
            en = "Rotation in degrees, snapped to nearest 90°. Default: 90.",
            fr = "Rotation en degrés, arrondie au 90° le plus proche. Défaut: 90."
        )
    )]
    #[sera_sig(deg = 90)]
    pub fn rotate(&self, deg: i32) -> Chart {
        let d = ((deg % 360) + 360) % 360;
        let snapped = match d {
            0..=44 | 316..=359 => 0,
            45..=134 => 90,
            135..=224 => 180,
            _ => 270,
        };
        self.propagate(crate::html::hover::apply_rotation(
            self.html.clone(),
            snapped,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("sort", "order_by"),
        file = "charts/chart.md",
        en = "Sorts chart bars by value or label using a client-side JavaScript re-render.",
        fr = "Trie les barres du graphique par valeur ou étiquette via un rendu JavaScript côté client.",
        param(
            name = "order",
            ty = "str",
            en = "Sort order: 'desc' (default), 'asc', 'alpha', 'alpha_desc', 'none'.",
            fr = "Ordre de tri: 'desc' (défaut), 'asc', 'alpha', 'alpha_desc', 'none'."
        )
    )]
    #[sera_sig(order = "desc")]
    pub fn sort_by(&self, order: &str) -> Chart {
        let ord = match order {
            "asc" | "desc" | "alpha" | "alpha_desc" | "none" => order,
            _ => "desc",
        };
        let snippet = format!(
            "<script>window.__sp_sort__={};{}</script></body>",
            json_str(ord),
            SP_SORT_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("show_leg"),
        file = "charts/chart.md",
        en = "Repositions the chart legend and enables interactive series toggling by click.",
        fr = "Repositionne la légende du graphique et active la bascule interactive des séries au clic.",
        param(
            name = "position",
            ty = "str",
            en = "Legend position: 'right' (default), 'left', 'top', 'bottom', 'none'.",
            fr = "Position de la légende: 'right' (défaut), 'left', 'top', 'bottom', 'none'."
        )
    )]
    #[sera_sig(position = "right")]
    pub fn legend(&self, position: &str) -> Chart {
        let pos = match position {
            "right" | "left" | "top" | "bottom" | "none" => position,
            _ => "right",
        };
        if pos == "none" {
            return self.no_legend();
        }
        let snippet = format!(
            "<script>window.__sp_legend_pos__={};{}</script></body>",
            json_str(pos),
            SP_LEGEND_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("tilt_labels"),
        file = "charts/chart.md",
        en = "Rotates X axis tick labels by the specified angle in degrees.",
        fr = "Fait pivoter les étiquettes de graduation de l'axe X de l'angle spécifié en degrés.",
        param(
            name = "angle",
            ty = "int",
            en = "Rotation angle in degrees (e.g. -45 for diagonal labels).",
            fr = "Angle de rotation en degrés (ex: -45 pour des étiquettes diagonales)."
        )
    )]
    #[sera_sig(angle)]
    pub fn rotate_labels(&self, angle: i32) -> Chart {
        let css = format!("<style>.sp-xt{{transform-box:fill-box;transform-origin:center;transform:rotate({}deg)}}</style></head>", angle);
        self.propagate(self.html.replacen("</head>", &css, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("fluid", "auto_size"),
        file = "charts/chart.md",
        en = "Makes the SVG width 100% of its container while keeping proportional height.",
        fr = "Rend la largeur du SVG égale à 100% de son conteneur tout en conservant une hauteur proportionnelle."
    )]
    pub fn responsive(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>svg{width:100%!important;height:auto!important}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("animation"),
        file = "charts/chart.md",
        en = "Adds a staggered entry animation to data elements (bars, circles, areas).",
        fr = "Ajoute une animation d'entrée décalée aux éléments de données (barres, cercles, zones).",
        param(
            name = "duration",
            ty = "int",
            en = "Animation duration in milliseconds. Default: 300.",
            fr = "Durée de l'animation en millisecondes. Défaut: 300."
        )
    )]
    #[sera_sig(duration = 300)]
    pub fn animate(&self, duration: i32) -> Chart {
        let css = format!("<style>@keyframes sp-in{{from{{opacity:0;transform:translateY(8px)}}to{{opacity:1;transform:none}}}}svg rect[data-idx],svg circle[data-idx],svg path.sp-area{{animation:sp-in {}ms ease-out both}}</style></head>", duration);
        let js = "<script>(function(){if(window.__spa__)return;window.__spa__=1;var els=document.querySelectorAll('svg [data-idx]');for(var i=0;i<els.length;i++)els[i].style.animationDelay=i*30+'ms';})();</script></body>";
        self.propagate(
            self.html
                .replacen("</head>", &css, 1)
                .replacen("</body>", js, 1),
        )
    }

    #[sera_doc(
        category = "chart_method",
        aliases("radius", "corners"),
        file = "charts/chart.md",
        en = "Applies a CSS border-radius to the chart container element.",
        fr = "Applique un border-radius CSS à l'élément conteneur du graphique.",
        param(
            name = "px",
            ty = "int",
            en = "Corner radius in pixels.",
            fr = "Rayon des coins en pixels."
        )
    )]
    #[sera_sig(px)]
    pub fn border_radius(&self, px: i32) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>[id^='spp'],.c3w{{border-radius:{}px!important;overflow:hidden}}</style></head>", px), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("opacity", "alpha"),
        file = "charts/chart.md",
        en = "Sets the opacity of all data elements (bars, circles, areas) in the chart.",
        fr = "Définit l'opacité de tous les éléments de données (barres, cercles, zones) du graphique.",
        param(
            name = "value",
            ty = "float",
            en = "Opacity between 0.0 (invisible) and 1.0 (fully opaque).",
            fr = "Opacité entre 0.0 (invisible) et 1.0 (totalement opaque)."
        )
    )]
    #[sera_sig(value)]
    pub fn set_opacity(&self, value: f64) -> Chart {
        let v = value.clamp(0.0, 1.0);
        self.propagate(self.html.replacen("</head>", &format!("<style>svg rect[data-idx],svg circle[data-idx],svg path.sp-area{{opacity:{}!important}}</style></head>", v), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("margin"),
        file = "charts/chart.md",
        en = "Adds internal padding to the chart and adjusts data element positions accordingly.",
        fr = "Ajoute un espacement interne au graphique et ajuste en conséquence les positions des éléments de données.",
        param(
            name = "px",
            ty = "int",
            en = "Margin in pixels applied to all four sides.",
            fr = "Marge en pixels appliquée aux quatre côtés."
        )
    )]
    #[sera_sig(px)]
    pub fn set_margin(&self, px: i32) -> Chart {
        let css = format!("<style>body{{padding:{px}px!important;box-sizing:border-box}}[id^='spp'],.c3w{{margin:{px}px!important}}</style></head>");
        let gap_ratio = ((px as f64) / 80.0).clamp(0.0, 0.7);
        let mut snippet = String::new();
        snippet.push_str("<script>window.__sp_margin_px__=");
        snippet.push_str(&px.to_string());
        snippet.push_str(";window.__sp_bar_gap__=");
        snippet.push_str(&format!("{:.4}", gap_ratio));
        snippet.push(';');
        snippet.push_str(SP_MARGIN_JS);
        snippet.push(';');
        snippet.push_str(SP_BAR_GAP_JS);
        snippet.push_str(";</script></body>");
        let h = self.html.replacen("</head>", &css, 1);
        self.propagate(h.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("gap"),
        file = "charts/chart.md",
        en = "Adjusts the gap ratio between bars. Higher values create thinner bars with more space.",
        fr = "Ajuste le ratio d'espacement entre les barres. Des valeurs plus élevées créent des barres plus fines.",
        param(
            name = "ratio",
            ty = "float",
            en = "Gap ratio between 0.0 (no gap) and 0.95 (almost no bar). Default: 0.3.",
            fr = "Ratio d'espacement entre 0.0 (sans espacement) et 0.95 (presque sans barre). Défaut: 0.3."
        )
    )]
    #[sera_sig(ratio = 0.3)]
    pub fn bar_gap(&self, ratio: f64) -> Chart {
        let r = ratio.clamp(0.0, 0.95);
        let snippet = format!(
            "<script>window.__sp_bar_gap__={:.4};{}</script></body>",
            r, SP_BAR_GAP_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("padding"),
        file = "charts/chart.md",
        en = "Applies CSS padding to the chart container element.",
        fr = "Applique un padding CSS à l'élément conteneur du graphique.",
        param(
            name = "px",
            ty = "int",
            en = "Padding in pixels applied to all four sides.",
            fr = "Padding en pixels appliqué aux quatre côtés."
        )
    )]
    #[sera_sig(px)]
    pub fn set_padding(&self, px: i32) -> Chart {
        let css = format!("<style>[id^='spp'],.c3w{{padding:{px}px!important;box-sizing:border-box}}</style></head>");
        self.propagate(self.html.replacen("</head>", &css, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("tick_angle", "label_angle"),
        file = "charts/chart.md",
        en = "Rotates X and/or Y axis tick labels independently.",
        fr = "Fait pivoter indépendamment les étiquettes de graduation des axes X et/ou Y.",
        param(
            name = "x_angle",
            ty = "int | None",
            en = "Rotation angle for X axis labels in degrees.",
            fr = "Angle de rotation des étiquettes de l'axe X en degrés."
        ),
        param(
            name = "y_angle",
            ty = "int | None",
            en = "Rotation angle for Y axis labels in degrees.",
            fr = "Angle de rotation des étiquettes de l'axe Y en degrés."
        )
    )]
    #[sera_sig(x_angle=None, y_angle=None)]
    pub fn axis_label_angle(&self, x_angle: Option<i32>, y_angle: Option<i32>) -> Chart {
        let mut css = String::from("<style>");
        if let Some(a) = x_angle {
            css.push_str(&format!(
                ".sp-xt{{transform-box:fill-box;transform-origin:center;transform:rotate({}deg)}}",
                a
            ));
        }
        if let Some(a) = y_angle {
            css.push_str(&format!(
                ".sp-yt{{transform-box:fill-box;transform-origin:center;transform:rotate({}deg)}}",
                a
            ));
        }
        css.push_str("</style></head>");
        self.propagate(self.html.replacen("</head>", &css, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("hide_legend"),
        file = "charts/chart.md",
        en = "Hides the chart legend.",
        fr = "Masque la légende du graphique."
    )]
    pub fn no_legend(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>g[data-legend]{display:none!important}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("hide_title"),
        file = "charts/chart.md",
        en = "Hides the chart title.",
        fr = "Masque le titre du graphique."
    )]
    pub fn no_title(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>.sp-ttl{display:none!important}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("title_on"),
        file = "charts/chart.md",
        en = "Forces the chart title to be visible with a contrast stroke for readability.",
        fr = "Force le titre du graphique à être visible avec un contour de contraste pour la lisibilité."
    )]
    pub fn show_title(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-ttl{display:block!important;visibility:visible!important;opacity:1!important;fill:#e2e8f0!important;paint-order:stroke;stroke:rgba(0,0,0,.6);stroke-width:.6px}</style></head>", 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("leg"),
        file = "charts/chart.md",
        en = "Forces the chart legend to be visible even if it was hidden.",
        fr = "Force la légende du graphique à être visible même si elle était masquée."
    )]
    pub fn show_legend(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>g[data-legend]{display:block!important;visibility:visible!important}</style></head>", 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("leg_pos"),
        file = "charts/chart.md",
        en = "Alias for legend(position). Repositions the chart legend.",
        fr = "Alias de legend(position). Repositionne la légende du graphique.",
        param(
            name = "position",
            ty = "str",
            en = "Legend position: 'right', 'left', 'top', 'bottom', 'none'.",
            fr = "Position de la légende: 'right', 'left', 'top', 'bottom', 'none'."
        )
    )]
    #[sera_sig(position = "right")]
    pub fn legend_position(&self, position: &str) -> Chart {
        self.legend(position)
    }

    #[sera_doc(
        category = "chart_method",
        aliases("labels_at"),
        file = "charts/chart.md",
        en = "Alias for show_labels(position). Adds an interactive legend overlay at the given position.",
        fr = "Alias de show_labels(position). Ajoute une superposition de légende interactive à la position donnée.",
        param(
            name = "position",
            ty = "str",
            en = "Overlay position: 'top', 'bottom', 'left', 'right'.",
            fr = "Position de la superposition: 'top', 'bottom', 'left', 'right'."
        )
    )]
    #[sera_sig(position = "bottom")]
    pub fn label_position(&self, position: &str) -> Chart {
        self.show_labels(position, None, None)
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/export.md",
        en = "Adds a floating download button to the chart that saves the full HTML on click.",
        fr = "Ajoute un bouton de téléchargement flottant au graphique qui sauvegarde le HTML complet au clic.",
        aliases("download_button", "export_btn", "with_export_button")
    )]
    pub fn export_button(&self) -> Chart {
        if self.html.contains("class=\"c3w\"") {
            return self.propagate(apply_3d_cfg(self.html.clone(), "{\"exportBtn\":true}"));
        }
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_EXPORT_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("ttl_color", "tc"),
        file = "charts/chart.md",
        en = "Sets the title color of a 3D chart (works on 3D canvas charts only). Default: white.",
        fr = "Définit la couleur du titre d'un graphique 3D (fonctionne uniquement sur les graphiques canvas 3D). Défaut : blanc.",
        param(
            name = "color",
            ty = "str",
            en = "CSS color for the title text.",
            fr = "Couleur CSS pour le texte du titre."
        )
    )]
    #[sera_sig(color = "#ffffff")]
    pub fn title_color(&self, color: &str) -> Chart {
        if self.html.contains("class=\"c3w\"") {
            return self.propagate(apply_3d_cfg(
                self.html.clone(),
                &format!("{{\"titleColor\":{}}}", json_str(color)),
            ));
        }
        let css = format!(
            "<style>.sp-ttl{{fill:{}!important}}</style></head>",
            color
        );
        self.propagate(self.html.replacen("</head>", &css, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("auto_text", "data_labels"),
        file = "charts/chart.md",
        en = "Overlays data value labels on all chart elements. Supports format strings and positioning.",
        fr = "Superpose des étiquettes de valeurs de données sur tous les éléments du graphique. Supporte les chaînes de format et le positionnement.",
        param(
            name = "format",
            ty = "str | None",
            en = "Format string (e.g. '.2f', '.0%') or empty string for auto. None disables.",
            fr = "Chaîne de format (ex: '.2f', '.0%') ou chaîne vide pour automatique. None désactive."
        ),
        param(
            name = "position",
            ty = "str | None",
            en = "Label position: 'auto', 'inside', 'outside'.",
            fr = "Position de l'étiquette: 'auto', 'inside', 'outside'."
        ),
        param(
            name = "angle",
            ty = "int | None",
            en = "Label rotation angle in degrees.",
            fr = "Angle de rotation des étiquettes en degrés."
        ),
        param(
            name = "font_size",
            ty = "int | None",
            en = "Font size of the data labels in pixels.",
            fr = "Taille de police des étiquettes de données en pixels."
        ),
        param(
            name = "color",
            ty = "str | None",
            en = "Color of the data labels.",
            fr = "Couleur des étiquettes de données."
        )
    )]
    #[sera_sig(format=None, position=None, angle=None, font_size=None, color=None)]
    pub fn text_auto(
        &self,
        format: Option<&str>,
        position: Option<&str>,
        angle: Option<i32>,
        font_size: Option<i32>,
        color: Option<&str>,
    ) -> Chart {
        let mut opts = String::from("window.__sp_text__={");
        if let Some(f) = format {
            opts.push_str(&format!("format:{},", json_str(f)));
        }
        if let Some(p) = position {
            opts.push_str(&format!("position:{},", json_str(p)));
        }
        if let Some(a) = angle {
            opts.push_str(&format!("angle:{},", a));
        }
        if let Some(s) = font_size {
            opts.push_str(&format!("font_size:{},", s));
        }
        if let Some(c) = color {
            opts.push_str(&format!("color:{},", json_str(c)));
        }
        opts.push_str("};");
        let snippet = format!("<script>{}{}</script></body>", opts, SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("text_pos"),
        file = "charts/chart.md",
        en = "Sets the position for data value labels on chart elements.",
        fr = "Définit la position des étiquettes de valeurs de données sur les éléments du graphique.",
        param(
            name = "position",
            ty = "str",
            en = "Position: 'auto', 'inside', 'outside'.",
            fr = "Position: 'auto', 'inside', 'outside'."
        )
    )]
    #[sera_sig(position)]
    pub fn text_position(&self, position: &str) -> Chart {
        let snippet = format!("<script>window.__sp_text__=Object.assign(window.__sp_text__||{{}},{{position:{}}});{}</script></body>", json_str(position), SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("rotate_text"),
        file = "charts/chart.md",
        en = "Sets the rotation angle for data value labels.",
        fr = "Définit l'angle de rotation des étiquettes de valeurs de données.",
        param(
            name = "degrees",
            ty = "int",
            en = "Rotation angle in degrees.",
            fr = "Angle de rotation en degrés."
        )
    )]
    #[sera_sig(degrees)]
    pub fn text_angle(&self, degrees: i32) -> Chart {
        let snippet = format!("<script>window.__sp_text__=Object.assign(window.__sp_text__||{{}},{{angle:{}}});{}</script></body>", degrees, SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("font_text"),
        file = "charts/chart.md",
        en = "Sets font family, size, and color for data value labels.",
        fr = "Définit la famille de polices, la taille et la couleur des étiquettes de valeurs de données.",
        param(
            name = "family",
            ty = "str | None",
            en = "Font family name.",
            fr = "Nom de la famille de polices."
        ),
        param(
            name = "size",
            ty = "int | None",
            en = "Font size in pixels.",
            fr = "Taille de police en pixels."
        ),
        param(
            name = "color",
            ty = "str | None",
            en = "Label text color.",
            fr = "Couleur du texte des étiquettes."
        )
    )]
    #[sera_sig(family=None, size=None, color=None)]
    pub fn text_font(&self, family: Option<&str>, size: Option<i32>, color: Option<&str>) -> Chart {
        let mut opts = String::from("window.__sp_text__=Object.assign(window.__sp_text__||{},{");
        if let Some(f) = family {
            opts.push_str(&format!("font_family:{},", json_str(f)));
        }
        if let Some(s) = size {
            opts.push_str(&format!("font_size:{},", s));
        }
        if let Some(c) = color {
            opts.push_str(&format!("color:{},", json_str(c)));
        }
        opts.push_str("});");
        let snippet = format!("<script>{}{}</script></body>", opts, SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("uniform"),
        file = "charts/chart.md",
        en = "Enforces a minimum font size for data labels; hides or shows labels that don't fit.",
        fr = "Impose une taille de police minimale pour les étiquettes de données; masque ou affiche les étiquettes qui ne tiennent pas.",
        param(
            name = "min_size",
            ty = "int",
            en = "Minimum font size in pixels.",
            fr = "Taille de police minimale en pixels."
        ),
        param(
            name = "mode",
            ty = "str",
            en = "Behaviour when label doesn't fit: 'hide' or 'show'.",
            fr = "Comportement quand l'étiquette ne tient pas: 'hide' ou 'show'."
        )
    )]
    #[sera_sig(min_size = 8, mode = "hide")]
    pub fn uniform_text(&self, min_size: i32, mode: &str) -> Chart {
        let snippet = format!("<script>window.__sp_text__=Object.assign(window.__sp_text__||{{}},{{uniform_min:{},uniform_mode:{}}});{}</script></body>", min_size, json_str(mode), SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("bar_radius"),
        file = "charts/chart.md",
        en = "Applies a corner radius to all bar rectangles in the chart.",
        fr = "Applique un rayon de coin à tous les rectangles de barres du graphique.",
        param(
            name = "radius",
            ty = "str",
            en = "Radius in pixels as string or percentage (e.g. '8' or '50%').",
            fr = "Rayon en pixels sous forme de chaîne ou pourcentage (ex: '8' ou '50%')."
        )
    )]
    #[sera_sig(radius)]
    pub fn corner_radius_bars(&self, radius: &str) -> Chart {
        let val = if radius.ends_with('%') {
            json_str(radius)
        } else {
            radius
                .parse::<f64>()
                .map(|v| v.to_string())
                .unwrap_or_else(|_| json_str(radius))
        };
        let snippet = format!(
            "<script>window.__sp_bar_r__={};{}</script></body>",
            val, SP_BAR_RADIUS_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("safe_csp"),
        file = "charts/chart.md",
        en = "Removes inline event handlers to make the chart compatible with strict Content-Security-Policy environments.",
        fr = "Supprime les gestionnaires d'événements inline pour rendre le graphique compatible avec les environnements à politique de sécurité de contenu stricte."
    )]
    pub fn csp_safe(&self) -> Chart {
        let mut out = String::with_capacity(self.html.len());
        let mut rest = self.html.as_str();
        let mut blob = String::new();
        loop {
            match rest.find("<script>") {
                None => {
                    out.push_str(rest);
                    break;
                }
                Some(i) => {
                    out.push_str(&rest[..i]);
                    let after = &rest[i + 8..];
                    match after.find("</script>") {
                        None => {
                            out.push_str("<script>");
                            out.push_str(after);
                            break;
                        }
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

    #[sera_doc(
        category = "chart_method",
        aliases("accessibility", "aria"),
        file = "charts/chart.md",
        en = "Adds ARIA accessibility attributes (title and description) to the SVG element.",
        fr = "Ajoute des attributs d'accessibilité ARIA (titre et description) à l'élément SVG.",
        param(
            name = "title",
            ty = "str",
            en = "Accessible title for screen readers.",
            fr = "Titre accessible pour les lecteurs d'écran."
        ),
        param(
            name = "desc",
            ty = "str",
            en = "Accessible description for screen readers.",
            fr = "Description accessible pour les lecteurs d'écran."
        )
    )]
    #[sera_sig(title = "", desc = "")]
    pub fn a11y(&self, title: &str, desc: &str) -> Chart {
        let snippet = format!(
            "<svg role=\"img\" aria-label=\"{}\"><title>{}</title><desc>{}</desc>",
            title.replace('"', "&quot;"),
            title.replace('<', "&lt;"),
            desc.replace('<', "&lt;"),
        );
        self.propagate(self.html.replacen("<svg", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("compare"),
        file = "charts/chart.md",
        en = "Returns a textual diff between this chart's HTML and another chart's HTML.",
        fr = "Retourne un diff textuel entre le HTML de ce graphique et celui d'un autre graphique.",
        param(
            name = "other",
            ty = "Chart",
            en = "The other Chart instance to compare against.",
            fr = "L'autre instance Chart à comparer."
        )
    )]
    pub fn diff(&self, other: &Chart) -> String {
        crate::bindings::commands::charts::chart_diff(
            &serde_json::json!({"a": self.html, "b": other.html}).to_string(),
        )
    }

    #[sera_doc(
        category = "chart_method",
        aliases("decimate", "lod"),
        file = "charts/chart.md",
        en = "Downsamples line chart data using the LTTB algorithm to reduce visual clutter.",
        fr = "Réduit les données du graphique en courbes via l'algorithme LTTB pour diminuer l'encombrement visuel.",
        param(
            name = "n",
            ty = "int",
            en = "Target number of data points after downsampling.",
            fr = "Nombre cible de points de données après réduction."
        ),
        param(
            name = "method",
            ty = "str",
            en = "Downsampling method. Currently only 'lttb' is supported.",
            fr = "Méthode de réduction. Seul 'lttb' est actuellement supporté."
        )
    )]
    #[sera_sig(n = 2000, method = "lttb")]
    pub fn downsample(&self, n: usize, method: &str) -> Chart {
        let _ = method;
        let h = &self.html;
        let mut out = String::with_capacity(h.len());
        let mut rest = h.as_str();
        loop {
            match rest.find("data-x=\"") {
                None => {
                    out.push_str(rest);
                    break;
                }
                Some(i) => {
                    out.push_str(&rest[..i]);
                    let after = &rest[i + 8..];
                    let end = match after.find('"') {
                        Some(e) => e,
                        None => {
                            out.push_str("data-x=\"");
                            out.push_str(after);
                            break;
                        }
                    };
                    let xs_raw = &after[..end];
                    let after2 = &after[end + 1..];
                    let after_y = match after2.find("data-y=\"") {
                        Some(j) => j,
                        None => {
                            out.push_str("data-x=\"");
                            out.push_str(after);
                            break;
                        }
                    };
                    let ys_section = &after2[after_y + 8..];
                    let ys_end = match ys_section.find('"') {
                        Some(e) => e,
                        None => {
                            out.push_str("data-x=\"");
                            out.push_str(after);
                            break;
                        }
                    };
                    let ys_raw = &ys_section[..ys_end];
                    let xs: Vec<f64> = xs_raw.split(',').filter_map(|s| s.parse().ok()).collect();
                    let ys: Vec<f64> = ys_raw.split(',').filter_map(|s| s.parse().ok()).collect();
                    if xs.len() == ys.len() && xs.len() > n && n >= 3 {
                        let payload = serde_json::json!({"x":xs,"y":ys,"threshold":n}).to_string();
                        let res = crate::bindings::commands::charts::downsample_lttb(&payload);
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&res) {
                            if v.get("ok").and_then(|b| b.as_bool()).unwrap_or(false) {
                                let nx: Vec<String> = v
                                    .get("x")
                                    .and_then(|a| a.as_array())
                                    .map(|a| {
                                        a.iter()
                                            .filter_map(|n| n.as_f64().map(|x| x.to_string()))
                                            .collect()
                                    })
                                    .unwrap_or_default();
                                let ny: Vec<String> = v
                                    .get("y")
                                    .and_then(|a| a.as_array())
                                    .map(|a| {
                                        a.iter()
                                            .filter_map(|n| n.as_f64().map(|x| x.to_string()))
                                            .collect()
                                    })
                                    .unwrap_or_default();
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

    #[sera_doc(
        category = "chart_method",
        aliases("tab_group"),
        file = "charts/chart.md",
        en = "Groups this chart with every other chart sharing the same group name into one navigable, clickable, interchangeable tabbed section. Works for any 2D or 3D chart, in a notebook or any HTML page.",
        fr = "Regroupe ce graphique avec tous les autres graphiques partageant le même nom de groupe en une section à onglets navigable, cliquable et interchangeable. Fonctionne pour tout graphique 2D ou 3D, dans un notebook ou n'importe quelle page HTML.",
        param(
            name = "name",
            ty = "str",
            en = "Group identifier shared by every chart that should appear together.",
            fr = "Identifiant de groupe partagé par tous les graphiques qui doivent apparaître ensemble."
        ),
        param(
            name = "position",
            ty = "str",
            en = "Where the tab strip sits relative to the chart: 'top' (default), 'bottom', 'left' or 'right'.",
            fr = "Où se place la barre d'onglets par rapport au graphique : 'top' (défaut), 'bottom', 'left' ou 'right'."
        )
    )]
    #[sera_sig(name, position = "top")]
    pub fn group(&self, name: &str, position: &str) -> Chart {
        let pos = match position {
            "bottom" | "left" | "right" => position,
            _ => "top",
        };
        let snippet = format!(
            "<script>window.__sp_group_name__={};window.__sp_group_pos__={};{}</script></body>",
            json_str(name),
            json_str(pos),
            SP_GROUP_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("no_spines"),
        file = "charts/chart.md",
        en = "Removes the axis spine lines, keeping ticks and labels (seaborn-style despine).",
        fr = "Retire les traits d'axe (spines) tout en gardant les graduations et les libellés (despine façon seaborn)."
    )]
    pub fn despine(&self) -> Chart {
        self.propagate(apply_despine(self.html.clone()))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("wm"),
        file = "charts/chart.md",
        en = "Overlays a large diagonal watermark text across the whole chart, for branding or draft marks.",
        fr = "Superpose un grand texte en filigrane diagonal sur tout le graphique, pour le branding ou les brouillons.",
        param(
            name = "text",
            ty = "str",
            en = "Watermark text.",
            fr = "Texte du filigrane."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Watermark opacity between 0.0 and 1.0. Default: 0.08.",
            fr = "Opacité du filigrane entre 0.0 et 1.0. Défaut : 0.08."
        )
    )]
    #[sera_sig(text, opacity = 0.08)]
    pub fn watermark(&self, text: &str, opacity: f64) -> Chart {
        self.propagate(apply_watermark(self.html.clone(), text, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("footnote"),
        file = "charts/chart.md",
        en = "Adds a small footnote caption centered below the chart.",
        fr = "Ajoute une petite légende de bas de page centrée sous le graphique.",
        param(
            name = "text",
            ty = "str",
            en = "Caption text.",
            fr = "Texte de la légende."
        )
    )]
    #[sera_sig(text)]
    pub fn caption(&self, text: &str) -> Chart {
        self.propagate(apply_caption(self.html.clone(), text))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("orientation3d", "tilt3d", "rotate3d"),
        file = "charts/chart.md",
        en = "Sets the initial camera orientation of a 3D chart: 'iso' (default), 'horizontal' (side-on), 'vertical' (top-down) or 'front'.",
        fr = "Définit l'orientation initiale de la caméra d'un graphique 3D : 'iso' (défaut), 'horizontal' (vue de côté), 'vertical' (vue de dessus) ou 'front'.",
        param(
            name = "mode",
            ty = "str",
            en = "'iso', 'horizontal', 'vertical' or 'front'.",
            fr = "'iso', 'horizontal', 'vertical' ou 'front'."
        )
    )]
    #[sera_sig(mode = "iso")]
    pub fn orient3d(&self, mode: &str) -> Chart {
        self.propagate(apply_orient3d(self.html.clone(), mode))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("color_scale", "heat_legend", "color_legend"),
        file = "charts/chart.md",
        en = "Adds a heat colorbar legend showing the value range used to color the data points, in both 2D and 3D charts. Position can be 'right' (default), 'left', 'top' or 'bottom'.",
        fr = "Ajoute une jauge de couleur (colorbar) montrant la plage de valeurs utilisée pour colorer les points, en 2D comme en 3D. La position peut être 'right' (défaut), 'left', 'top' ou 'bottom'.",
        param(
            name = "position",
            ty = "str",
            en = "'right', 'left', 'top' or 'bottom'.",
            fr = "'right', 'left', 'top' ou 'bottom'."
        )
    )]
    #[sera_sig(position = "right")]
    pub fn colorbar(&self, position: &str) -> Chart {
        self.propagate(apply_colorbar(self.html.clone(), position))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("neon"),
        file = "charts/chart.md",
        en = "Adds a neon drop-shadow glow behind every data element (bars, lines, points).",
        fr = "Ajoute une lueur néon (drop-shadow) derrière chaque élément de données (barres, lignes, points).",
        param(
            name = "color",
            ty = "str | None",
            en = "Glow color. Defaults to the chart's accent color (#6366f1).",
            fr = "Couleur de la lueur. Par défaut, la couleur d'accent du graphique (#6366f1)."
        )
    )]
    #[sera_sig(color = None)]
    pub fn glow(&self, color: Option<&str>) -> Chart {
        self.propagate(apply_glow(self.html.clone(), color.unwrap_or("#6366f1")))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("mark", "spotlight"),
        file = "charts/chart.md",
        en = "Highlights a single data element by index with a distinct color and glow, dimming nothing else.",
        fr = "Met en évidence un seul élément de données par son index avec une couleur distincte et une lueur, sans assombrir le reste.",
        param(
            name = "index",
            ty = "int",
            en = "Zero-based index of the data element to highlight.",
            fr = "Index (à partir de 0) de l'élément de données à mettre en évidence."
        ),
        param(
            name = "color",
            ty = "str | None",
            en = "Highlight color. Default: #f59e0b.",
            fr = "Couleur de mise en évidence. Défaut : #f59e0b."
        )
    )]
    #[sera_sig(index, color = None)]
    pub fn highlight(&self, index: usize, color: Option<&str>) -> Chart {
        self.propagate(apply_highlight(
            self.html.clone(),
            index,
            color.unwrap_or("#f59e0b"),
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("hrule", "target_line"),
        file = "charts/chart.md",
        en = "Draws a dashed horizontal reference/threshold line at the given data value, on value-based bar charts.",
        fr = "Trace une ligne de référence/seuil en pointillés à la valeur donnée, sur les graphiques en barres basés sur des valeurs.",
        param(
            name = "value",
            ty = "float",
            en = "Data value where the line is drawn.",
            fr = "Valeur de donnée où la ligne est tracée."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Line color. Default: #ef4444.",
            fr = "Couleur de la ligne. Défaut : #ef4444."
        ),
        param(
            name = "label",
            ty = "str | None",
            en = "Optional label drawn next to the line.",
            fr = "Étiquette optionnelle affichée près de la ligne."
        )
    )]
    #[sera_sig(value, color = "#ef4444", label = None)]
    pub fn hline(&self, value: f64, color: &str, label: Option<&str>) -> Chart {
        self.propagate(apply_hline(self.html.clone(), value, color, label))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("sub"),
        file = "charts/chart.md",
        en = "Adds a small subtitle directly under the chart title.",
        fr = "Ajoute un petit sous-titre directement sous le titre du graphique.",
        param(
            name = "text",
            ty = "str",
            en = "Subtitle text.",
            fr = "Texte du sous-titre."
        )
    )]
    #[sera_sig(text)]
    pub fn subtitle(&self, text: &str) -> Chart {
        self.propagate(apply_subtitle(self.html.clone(), text))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("drop_shadow"),
        file = "charts/chart.md",
        en = "Adds a soft drop-shadow behind the whole chart container.",
        fr = "Ajoute une ombre douce derrière tout le conteneur du graphique.",
        param(
            name = "blur",
            ty = "int",
            en = "Shadow blur radius in pixels. Default: 24.",
            fr = "Rayon de flou de l'ombre en pixels. Défaut : 24."
        ),
        param(
            name = "color",
            ty = "str | None",
            en = "Shadow color. Defaults to a neutral dark shadow.",
            fr = "Couleur de l'ombre. Par défaut, une ombre neutre sombre."
        )
    )]
    #[sera_sig(blur = 24, color = None)]
    pub fn shadow(&self, blur: i32, color: Option<&str>) -> Chart {
        self.propagate(apply_shadow(
            self.html.clone(),
            blur,
            color.unwrap_or("rgba(0,0,0,.35)"),
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("blink", "beacon"),
        file = "charts/chart.md",
        en = "Makes data elements gently pulse, drawing attention to the chart (e.g. for live dashboards). By default pulses every element; target specific ones with index, or every element above a value with above.",
        fr = "Fait pulser des éléments de données, pour attirer l'attention (ex : tableaux de bord en direct). Par défaut, pulse tous les éléments ; cible des éléments précis avec index, ou tout élément au-dessus d'une valeur avec above.",
        param(
            name = "duration",
            ty = "float",
            en = "Pulse cycle duration in seconds. Default: 2.0.",
            fr = "Durée du cycle de pulsation en secondes. Défaut : 2.0."
        ),
        param(
            name = "index",
            ty = "list[int] | None",
            en = "Zero-based indices of the data elements to pulse (e.g. [0] for the first). Defaults to every element.",
            fr = "Index (à partir de 0) des éléments à faire pulser (ex : [0] pour le premier). Par défaut, tous les éléments."
        ),
        param(
            name = "above",
            ty = "float | None",
            en = "Only pulse data elements whose value exceeds this threshold.",
            fr = "Ne fait pulser que les éléments de données dont la valeur dépasse ce seuil."
        )
    )]
    #[sera_sig(duration = 2.0, index = None, above = None)]
    pub fn pulse(&self, duration: f64, index: Option<Vec<usize>>, above: Option<f64>) -> Chart {
        self.propagate(apply_pulse(
            self.html.clone(),
            duration,
            index.as_deref(),
            above,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("border", "stroke"),
        file = "charts/chart.md",
        en = "Draws a solid colored outline around every data element, without blurring like glow().",
        fr = "Trace un contour coloré et net autour de chaque élément de données, sans flou comme glow().",
        param(
            name = "color",
            ty = "str | None",
            en = "Outline color. Defaults to the chart's accent color (#6366f1).",
            fr = "Couleur du contour. Par défaut, la couleur d'accent du graphique (#6366f1)."
        ),
        param(
            name = "width",
            ty = "float",
            en = "Outline width in pixels. Default: 2.0.",
            fr = "Épaisseur du contour en pixels. Défaut : 2.0."
        )
    )]
    #[sera_sig(color = None, width = 2.0)]
    pub fn outline(&self, color: Option<&str>, width: f64) -> Chart {
        self.propagate(apply_outline(
            self.html.clone(),
            color.unwrap_or("#6366f1"),
            width,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("trend", "regline"),
        file = "charts/chart.md",
        en = "Overlays a linear regression trend line computed from the chart's own values, on value-based bar charts.",
        fr = "Superpose une droite de régression linéaire calculée à partir des valeurs du graphique, sur les graphiques en barres basés sur des valeurs.",
        param(
            name = "color",
            ty = "str",
            en = "Trend line color. Default: #10b981.",
            fr = "Couleur de la droite de tendance. Défaut : #10b981."
        ),
        param(
            name = "width",
            ty = "float",
            en = "Trend line width in pixels. Default: 2.0.",
            fr = "Épaisseur de la droite de tendance en pixels. Défaut : 2.0."
        )
    )]
    #[sera_sig(color = "#10b981", width = 2.0)]
    pub fn trendline(&self, color: &str, width: f64) -> Chart {
        self.propagate(apply_trendline(self.html.clone(), color, width))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("mark_max", "ann_max"),
        file = "charts/chart.md",
        en = "Highlights and labels the data element with the highest value, on value-based bar charts.",
        fr = "Met en évidence et étiquette l'élément de données ayant la valeur la plus élevée, sur les graphiques en barres basés sur des valeurs.",
        param(
            name = "label",
            ty = "str | None",
            en = "Optional custom label text. Defaults to the value itself.",
            fr = "Texte d'étiquette personnalisé optionnel. Par défaut, la valeur elle-même."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Highlight and label color. Default: #22c55e.",
            fr = "Couleur de mise en évidence et de l'étiquette. Défaut : #22c55e."
        )
    )]
    #[sera_sig(label = None, color = "#22c55e")]
    pub fn annotate_max(&self, label: Option<&str>, color: &str) -> Chart {
        self.propagate(apply_annotate_extreme(self.html.clone(), "max", color, label))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("mark_min", "ann_min"),
        file = "charts/chart.md",
        en = "Highlights and labels the data element with the lowest value, on value-based bar charts.",
        fr = "Met en évidence et étiquette l'élément de données ayant la valeur la plus basse, sur les graphiques en barres basés sur des valeurs.",
        param(
            name = "label",
            ty = "str | None",
            en = "Optional custom label text. Defaults to the value itself.",
            fr = "Texte d'étiquette personnalisé optionnel. Par défaut, la valeur elle-même."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Highlight and label color. Default: #ef4444.",
            fr = "Couleur de mise en évidence et de l'étiquette. Défaut : #ef4444."
        )
    )]
    #[sera_sig(label = None, color = "#ef4444")]
    pub fn annotate_min(&self, label: Option<&str>, color: &str) -> Chart {
        self.propagate(apply_annotate_extreme(self.html.clone(), "min", color, label))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("ref_band", "target_zone"),
        file = "charts/chart.md",
        en = "Draws a shaded reference band between two data values, on value-based bar charts (e.g. a target range).",
        fr = "Trace une bande de référence ombrée entre deux valeurs, sur les graphiques en barres basés sur des valeurs (ex : une plage cible).",
        param(
            name = "low",
            ty = "float",
            en = "Lower bound of the band.",
            fr = "Borne basse de la bande."
        ),
        param(
            name = "high",
            ty = "float",
            en = "Upper bound of the band.",
            fr = "Borne haute de la bande."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Band fill color. Default: #f59e0b.",
            fr = "Couleur de remplissage de la bande. Défaut : #f59e0b."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Band opacity between 0.0 and 1.0. Default: 0.12.",
            fr = "Opacité de la bande entre 0.0 et 1.0. Défaut : 0.12."
        )
    )]
    #[sera_sig(low, high, color = "#f59e0b", opacity = 0.12)]
    pub fn reference_band(&self, low: f64, high: f64, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_reference_band(self.html.clone(), low, high, color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("vlabels", "bar_labels"),
        file = "charts/chart.md",
        en = "Prints the numeric value above each bar, like matplotlib's bar_label() or Plotly's textposition='auto'.",
        fr = "Affiche la valeur numérique au-dessus de chaque barre, comme bar_label() de matplotlib ou textposition='auto' de Plotly.",
        param(
            name = "decimals",
            ty = "int",
            en = "Number of decimal places to show. Default: 0.",
            fr = "Nombre de décimales à afficher. Défaut : 0."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Label text color. Default: #475569.",
            fr = "Couleur du texte du label. Défaut : #475569."
        )
    )]
    #[sera_sig(decimals = 0, color = "#475569")]
    pub fn value_labels(&self, decimals: i32, color: &str) -> Chart {
        self.propagate(apply_value_labels(self.html.clone(), decimals, color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Draws symmetric error-bar whiskers above and below each bar's top, like matplotlib's errorbar() or yerr.",
        fr = "Dessine des moustaches symétriques (barres d'erreur) au-dessus et en-dessous du sommet de chaque barre, comme errorbar() ou yerr de matplotlib.",
        aliases("errorbar"),
        param(
            name = "margin",
            ty = "float",
            en = "Error margin, expressed in the same units as the chart's values.",
            fr = "Marge d'erreur, exprimée dans la même unité que les valeurs du graphique."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Whisker color. Default: #475569.",
            fr = "Couleur des moustaches. Défaut : #475569."
        )
    )]
    #[sera_sig(margin, color = "#475569")]
    pub fn error_bars(&self, margin: f64, color: &str) -> Chart {
        self.propagate(apply_error_bars(self.html.clone(), margin, color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Labels each bar with its percentage change versus the previous bar, handy for tracking growth (QoQ, MoM, ...).",
        fr = "Annote chaque barre avec son évolution en pourcentage par rapport à la barre précédente, pratique pour suivre une croissance (trimestre, mois, ...).",
        aliases("deltas", "pct_change"),
        param(
            name = "pos_color",
            ty = "str",
            en = "Color used for positive changes. Default: #22c55e.",
            fr = "Couleur utilisée pour les évolutions positives. Défaut : #22c55e."
        ),
        param(
            name = "neg_color",
            ty = "str",
            en = "Color used for negative changes. Default: #ef4444.",
            fr = "Couleur utilisée pour les évolutions négatives. Défaut : #ef4444."
        )
    )]
    #[sera_sig(pos_color = "#22c55e", neg_color = "#ef4444")]
    pub fn delta_labels(&self, pos_color: &str, neg_color: &str) -> Chart {
        self.propagate(apply_delta_labels(self.html.clone(), pos_color, neg_color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Overlays a cumulative-sum line (running total over the running grand total), the classic Pareto-chart finishing touch.",
        fr = "Superpose une courbe de cumul (somme courante sur le total général), la touche finale classique d'un diagramme de Pareto.",
        aliases("cumline"),
        param(
            name = "color",
            ty = "str",
            en = "Line and marker color. Default: #6366f1.",
            fr = "Couleur de la courbe et des marqueurs. Défaut : #6366f1."
        )
    )]
    #[sera_sig(color = "#6366f1")]
    pub fn cumulative_line(&self, color: &str) -> Chart {
        self.propagate(apply_cumulative_line(self.html.clone(), color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Pins numbered rank badges (1, 2, 3, ...) on the top N highest-value bars, for quick leaderboard-style reads.",
        fr = "Épingle des badges de classement numérotés (1, 2, 3, ...) sur les N barres aux valeurs les plus élevées, pour une lecture rapide façon classement.",
        aliases("ranks"),
        param(
            name = "top_n",
            ty = "int",
            en = "How many of the highest-value bars to badge. Default: 3.",
            fr = "Nombre de barres (les plus élevées) à marquer d'un badge. Défaut : 3."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Badge fill color. Default: #6366f1.",
            fr = "Couleur de remplissage du badge. Défaut : #6366f1."
        )
    )]
    #[sera_sig(top_n = 3, color = "#6366f1")]
    pub fn rank_badges(&self, top_n: usize, color: &str) -> Chart {
        self.propagate(apply_rank_badges(self.html.clone(), top_n, color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Switches the value axis to a logarithmic scale, like matplotlib's plt.yscale('log') — great for data spanning several orders of magnitude.",
        fr = "Passe l'axe des valeurs en échelle logarithmique, comme plt.yscale('log') de matplotlib — idéal pour des données s'étalant sur plusieurs ordres de grandeur.",
        aliases("logy")
    )]
    pub fn log_scale(&self) -> Chart {
        self.propagate(apply_log_scale(self.html.clone()))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Overlays a moving-average line (trailing window), like pandas' .rolling(window).mean() — smooths out noisy series to reveal the underlying trend.",
        fr = "Superpose une courbe de moyenne mobile (fenêtre glissante), comme .rolling(window).mean() de pandas — lisse une série bruitée pour révéler la tendance sous-jacente.",
        aliases("rolling_mean"),
        param(
            name = "window",
            ty = "int",
            en = "Number of trailing points to average over.",
            fr = "Nombre de points (précédents) sur lesquels moyenner."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Line color. Default: #f59e0b.",
            fr = "Couleur de la courbe. Défaut : #f59e0b."
        )
    )]
    #[sera_sig(window = 3, color = "#f59e0b")]
    pub fn moving_average(&self, window: usize, color: &str) -> Chart {
        self.propagate(apply_moving_average(self.html.clone(), window, color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Flags statistical outliers with a warning marker and a highlighted color. Works on any chart: bars/lollipops/etc are flagged by z-score on their value, scatter points by residual distance from their best-fit line.",
        fr = "Signale les valeurs aberrantes statistiques avec un marqueur d'avertissement et une couleur de mise en évidence. Fonctionne sur tout type de graphique : barres/lollipops/etc sont signalés par z-score sur leur valeur, les points de scatter par distance résiduelle à leur droite de régression.",
        aliases("flag_outliers", "anomalies", "scatter_outliers"),
        param(
            name = "threshold_std",
            ty = "float",
            en = "Number of standard deviations beyond which a value is flagged. Default: 2.0.",
            fr = "Nombre d'écarts-types au-delà duquel une valeur est signalée. Défaut : 2.0."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Highlight color for flagged bars. Default: #ef4444.",
            fr = "Couleur de mise en évidence des barres signalées. Défaut : #ef4444."
        )
    )]
    #[sera_sig(threshold_std = 2.0, color = "#ef4444")]
    pub fn outliers(&self, threshold_std: f64, color: &str) -> Chart {
        self.propagate(apply_outliers(self.html.clone(), threshold_std, color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("fill_area", "area_fill"),
        file = "charts/chart.md",
        en = "Shades the area under a line chart's curve down to the baseline, like matplotlib's fill_between(x, y, 0).",
        fr = "Colore la zone sous la courbe d'un line chart jusqu'à la ligne de base, comme fill_between(x, y, 0) de matplotlib.",
        param(
            name = "color",
            ty = "str",
            en = "Fill color. Default: #6366f1.",
            fr = "Couleur de remplissage. Défaut : #6366f1."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Fill opacity between 0.0 and 1.0. Default: 0.15.",
            fr = "Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.15."
        )
    )]
    #[sera_sig(color = "#6366f1", opacity = 0.15)]
    pub fn fill_between(&self, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_fill_between(self.html.clone(), color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("boxplot"),
        file = "charts/chart.md",
        en = "Draws a box-and-whisker summary (min, Q1, median, Q3, max) of the chart's own values as a small inset glyph, like a built-in boxplot companion.",
        fr = "Dessine un résumé boîte-à-moustaches (min, Q1, médiane, Q3, max) des valeurs du graphique sous forme de petit glyphe en marge, comme un boxplot d'accompagnement intégré.",
        param(
            name = "color",
            ty = "str",
            en = "Box, whiskers and median line color. Default: #6366f1.",
            fr = "Couleur de la boîte, des moustaches et de la médiane. Défaut : #6366f1."
        )
    )]
    #[sera_sig(color = "#6366f1")]
    pub fn box_annotate(&self, color: &str) -> Chart {
        self.propagate(apply_box_annotate(self.html.clone(), color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Labels each bar with its share of the total (%) instead of its raw value.",
        fr = "Annote chaque barre avec sa part du total (%) plutôt que sa valeur brute.",
        aliases("pct"),
        param(
            name = "decimals",
            ty = "int",
            en = "Number of decimal places to show. Default: 1.",
            fr = "Nombre de décimales à afficher. Défaut : 1."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Label text color. Default: #475569.",
            fr = "Couleur du texte du label. Défaut : #475569."
        )
    )]
    #[sera_sig(decimals = 1, color = "#475569")]
    pub fn pct_of_total(&self, decimals: i32, color: &str) -> Chart {
        self.propagate(apply_pct_of_total(self.html.clone(), decimals, color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Computes and displays the Pearson correlation coefficient (r) of a scatter chart's x/y values as a small badge.",
        fr = "Calcule et affiche le coefficient de corrélation de Pearson (r) des valeurs x/y d'un scatter sous forme de petit badge.",
        aliases("corr"),
        param(
            name = "color",
            ty = "str",
            en = "Badge color. Default: #6366f1.",
            fr = "Couleur du badge. Défaut : #6366f1."
        )
    )]
    #[sera_sig(color = "#6366f1")]
    pub fn correlation_badge(&self, color: &str) -> Chart {
        self.propagate(apply_correlation_badge(self.html.clone(), color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Shades a contiguous range of bars/points by index, distinct from reference_band which shades by Y value.",
        fr = "Colore une plage contiguë de barres/points par index, à la différence de reference_band qui colore par valeur Y.",
        aliases("hl_range", "idx_band"),
        param(
            name = "low",
            ty = "int",
            en = "Start index of the range (inclusive).",
            fr = "Index de début de la plage (inclus)."
        ),
        param(
            name = "high",
            ty = "int",
            en = "End index of the range (inclusive).",
            fr = "Index de fin de la plage (inclus)."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Highlight fill color. Default: #6366f1.",
            fr = "Couleur de remplissage de la mise en évidence. Défaut : #6366f1."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Fill opacity between 0.0 and 1.0. Default: 0.12.",
            fr = "Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.12."
        )
    )]
    #[sera_sig(low, high, color = "#6366f1", opacity = 0.12)]
    pub fn highlight_range(&self, low: usize, high: usize, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_highlight_range(self.html.clone(), low, high, color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("iqr"),
        file = "charts/chart.md",
        en = "Shades a horizontal band across the interquartile range (Q1 to Q3) of the chart's own values, to see at a glance which bars fall inside the 'normal' middle 50%.",
        fr = "Colore une bande horizontale couvrant l'écart interquartile (Q1 à Q3) des valeurs du graphique, pour voir d'un coup d'œil quelles barres se trouvent dans les 50% centraux 'normaux'.",
        param(
            name = "color",
            ty = "str",
            en = "Band fill color. Default: #6366f1.",
            fr = "Couleur de remplissage de la bande. Défaut : #6366f1."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Fill opacity between 0.0 and 1.0. Default: 0.10.",
            fr = "Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.10."
        )
    )]
    #[sera_sig(color = "#6366f1", opacity = 0.10)]
    pub fn iqr_band(&self, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_iqr_band(self.html.clone(), color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Displays a badge with the total % change between the first and last bar — a quick growth-rate summary.",
        fr = "Affiche un badge avec la variation totale en % entre la première et la dernière barre — un résumé rapide du taux de croissance.",
        aliases("growth"),
        param(
            name = "color",
            ty = "str",
            en = "Badge color. Default: #22c55e.",
            fr = "Couleur du badge. Défaut : #22c55e."
        )
    )]
    #[sera_sig(color = "#22c55e")]
    pub fn growth_badge(&self, color: &str) -> Chart {
        self.propagate(apply_growth_badge(self.html.clone(), color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("zheat", "heatbars"),
        file = "charts/chart.md",
        en = "Recolors every bar on a diverging blue-white-red scale based on its z-score, turning the chart into a heatmap-as-bars view of which values are unusually high or low.",
        fr = "Recolore chaque barre sur une échelle divergente bleu-blanc-rouge selon son z-score, transformant le graphique en vue façon heatmap des valeurs anormalement hautes ou basses."
    )]
    pub fn zscore_heat(&self) -> Chart {
        self.propagate(apply_zscore_heat(self.html.clone()))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Marks the bar at which the cumulative total crosses a threshold percentage (80% by default), the classic Pareto '80/20' cutoff point. Pairs well with cumulative_line().",
        fr = "Marque la barre à partir de laquelle le total cumulé dépasse un pourcentage seuil (80% par défaut), le point de coupure Pareto '80/20' classique. Se combine bien avec cumulative_line().",
        aliases("pareto", "eighty_twenty"),
        param(
            name = "threshold_pct",
            ty = "float",
            en = "Cumulative percentage threshold to mark. Default: 80.0.",
            fr = "Pourcentage cumulé seuil à marquer. Défaut : 80.0."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Marker line and label color. Default: #ef4444.",
            fr = "Couleur du trait et du label du marqueur. Défaut : #ef4444."
        )
    )]
    #[sera_sig(threshold_pct = 80.0, color = "#ef4444")]
    pub fn pareto_marker(&self, threshold_pct: f64, color: &str) -> Chart {
        self.propagate(apply_pareto_marker(self.html.clone(), threshold_pct, color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("mean_diff"),
        file = "charts/chart.md",
        en = "Labels each bar with its absolute deviation from the dataset's mean, instead of percentage change versus the previous bar like delta_labels().",
        fr = "Annote chaque barre avec son écart absolu par rapport à la moyenne du jeu de données, à la différence de delta_labels() qui compare au pourcentage de variation par rapport à la barre précédente.",
        param(
            name = "pos_color",
            ty = "str",
            en = "Color used for above-average bars. Default: #22c55e.",
            fr = "Couleur utilisée pour les barres au-dessus de la moyenne. Défaut : #22c55e."
        ),
        param(
            name = "neg_color",
            ty = "str",
            en = "Color used for below-average bars. Default: #ef4444.",
            fr = "Couleur utilisée pour les barres en-dessous de la moyenne. Défaut : #ef4444."
        )
    )]
    #[sera_sig(pos_color = "#22c55e", neg_color = "#ef4444")]
    pub fn diff_from_mean(&self, pos_color: &str, neg_color: &str) -> Chart {
        self.propagate(apply_diff_from_mean(self.html.clone(), pos_color, neg_color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Overlays a Bollinger-band-style shaded region (rolling mean ± rolling standard deviation) over a trailing window, the classic volatility envelope from financial charting.",
        fr = "Superpose une zone ombrée façon bandes de Bollinger (moyenne mobile ± écart-type mobile) sur une fenêtre glissante, l'enveloppe de volatilité classique des graphiques financiers.",
        aliases("bollinger_band", "volatility_band"),
        param(
            name = "window",
            ty = "int",
            en = "Number of trailing points used for the rolling mean and standard deviation.",
            fr = "Nombre de points (précédents) utilisés pour la moyenne et l'écart-type mobiles."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Band fill color. Default: #6366f1.",
            fr = "Couleur de remplissage de la bande. Défaut : #6366f1."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Fill opacity between 0.0 and 1.0. Default: 0.15.",
            fr = "Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.15."
        )
    )]
    #[sera_sig(window = 5, color = "#6366f1", opacity = 0.15)]
    pub fn rolling_std_band(&self, window: usize, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_rolling_std_band(self.html.clone(), window, color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Extrapolates a linear-regression trend beyond the last data point, with a dashed line and ghost markers for each forecast step.",
        fr = "Extrapole une tendance de régression linéaire au-delà du dernier point de données, avec un trait pointillé et des marqueurs fantômes pour chaque pas de prévision.",
        aliases("forecast", "extrapolate"),
        param(
            name = "periods",
            ty = "int",
            en = "Number of future periods to forecast. Default: 3.",
            fr = "Nombre de périodes futures à prévoir. Défaut : 3."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Forecast line and marker color. Default: #8b5cf6.",
            fr = "Couleur du trait et des marqueurs de prévision. Défaut : #8b5cf6."
        )
    )]
    #[sera_sig(periods = 3, color = "#8b5cf6")]
    pub fn forecast_line(&self, periods: usize, color: &str) -> Chart {
        self.propagate(apply_forecast_line(self.html.clone(), periods, color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Shades a horizontal band between two arbitrary percentiles of the chart's own values, a generalization of iqr_band for any custom range (e.g. 5th-95th).",
        fr = "Colore une bande horizontale entre deux percentiles arbitraires des valeurs du graphique, une généralisation de iqr_band pour toute plage personnalisée (ex : 5e-95e).",
        aliases("pct_band"),
        param(
            name = "low_pct",
            ty = "float",
            en = "Lower percentile (0-100).",
            fr = "Percentile bas (0-100)."
        ),
        param(
            name = "high_pct",
            ty = "float",
            en = "Upper percentile (0-100).",
            fr = "Percentile haut (0-100)."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Band fill color. Default: #6366f1.",
            fr = "Couleur de remplissage de la bande. Défaut : #6366f1."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Fill opacity between 0.0 and 1.0. Default: 0.10.",
            fr = "Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.10."
        )
    )]
    #[sera_sig(low_pct, high_pct, color = "#6366f1", opacity = 0.10)]
    pub fn percentile_band(&self, low_pct: f64, high_pct: f64, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_percentile_band(self.html.clone(), low_pct, high_pct, color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Fits and draws a linear regression line through a scatter chart's points, like seaborn's regplot() or numpy.polyfit(x, y, 1).",
        fr = "Calcule et trace une droite de régression linéaire à travers les points d'un scatter, comme regplot() de seaborn ou numpy.polyfit(x, y, 1).",
        aliases("regression_line", "lmplot"),
        param(
            name = "color",
            ty = "str",
            en = "Line color. Default: #ef4444.",
            fr = "Couleur de la droite. Défaut : #ef4444."
        ),
        param(
            name = "width",
            ty = "float",
            en = "Line stroke width. Default: 2.0.",
            fr = "Épaisseur de la droite. Défaut : 2.0."
        )
    )]
    #[sera_sig(color = "#ef4444", width = 2.0)]
    pub fn scatter_regression(&self, color: &str, width: f64) -> Chart {
        self.propagate(apply_scatter_regression(self.html.clone(), color, width))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Runs DBSCAN density clustering and recolors each element by its cluster, in-browser and dependency-free. Works on any chart: uses x/y for scatter points, or falls back to the single value for bars/lollipops/etc. Noise points (no dense neighborhood) are colored gray.",
        fr = "Exécute un clustering par densité DBSCAN et recolore chaque élément selon son cluster, directement dans le navigateur sans dépendance. Fonctionne sur tout type de graphique : utilise x/y pour un scatter, ou se rabat sur la valeur seule pour les barres/lollipops/etc. Les points de bruit (sans voisinage dense) sont colorés en gris.",
        aliases("dbscan"),
        param(
            name = "eps",
            ty = "float",
            en = "Maximum distance (in data units) between two points for them to be considered neighbors.",
            fr = "Distance maximale (en unités de données) entre deux points pour qu'ils soient considérés voisins."
        ),
        param(
            name = "min_samples",
            ty = "int",
            en = "Minimum neighborhood size (including the point itself) to seed a cluster. Default: 3.",
            fr = "Taille minimale de voisinage (point inclus) pour amorcer un cluster. Défaut : 3."
        )
    )]
    #[sera_sig(eps, min_samples = 3)]
    pub fn cluster(&self, eps: f64, min_samples: usize) -> Chart {
        self.propagate(apply_cluster(self.html.clone(), eps, min_samples))
    }
}

#[cfg(feature = "ffi")]
mod chart_ffi {
    use super::*;
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;

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
    use crate::plot::statistical::bubble::BubbleVariant;
    use crate::plot::statistical::scatter::ScatterVariant;
    use crate::plot::statistical::{
        BarVariant, BoxplotVariant, BulletVariant, CandlestickVariant, DumbbellVariant,
        FunnelVariant, GaugeVariant, HeatmapVariant, HistogramVariant, KdeVariant, LineVariant,
        LollipopVariant, ParallelVariant, PieVariant, RadarVariant, RidgelineVariant, SlopeVariant,
        SunburstVariant, TreemapVariant, ViolinVariant, WaterfallVariant, WordCloudVariant,
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

#[cfg(feature = "python")]
struct ThemePreset {
    bg: Option<&'static str>,
    palette: &'static [u32],
    gridlines: bool,
}

#[cfg(feature = "python")]
const THEME_DARK: ThemePreset = ThemePreset {
    bg: Some("#0f172a"),
    palette: &[
        0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA, 0x22D3EE, 0xF472B6, 0xA3E635, 0xF87171,
        0x2DD4BF,
    ],
    gridlines: true,
};

#[cfg(feature = "python")]
const THEME_LIGHT: ThemePreset = ThemePreset {
    bg: None,
    palette: &[
        0x636EFA, 0xEF553B, 0x00CC96, 0xAB63FA, 0xFFA15A, 0x19D3F3, 0xFF6692, 0xB6E880, 0xFF97FF,
        0xFECB52,
    ],
    gridlines: false,
};

#[cfg(feature = "python")]
const THEME_SCIENTIFIC: ThemePreset = ThemePreset {
    bg: Some("#fafafa"),
    palette: &[
        0x1F77B4, 0xFF7F0E, 0x2CA02C, 0xD62728, 0x9467BD, 0x8C564B, 0xE377C2, 0x7F7F7F, 0xBCBD22,
        0x17BECF,
    ],
    gridlines: true,
};

#[cfg(feature = "python")]
const THEME_APPLE: ThemePreset = ThemePreset {
    bg: Some("#000000"),
    palette: &[
        0x0A84FF, 0x30D158, 0xFF453A, 0xFFD60A, 0xBF5AF2, 0x64D2FF, 0xFF9F0A, 0xFF375F, 0xAC8E68,
        0x63E6E2,
    ],
    gridlines: false,
};

#[cfg(feature = "python")]
const THEME_NOTION: ThemePreset = ThemePreset {
    bg: Some("#191919"),
    palette: &[
        0x529CCA, 0xD08B65, 0x6C9B7D, 0xCB7C7A, 0x9A6DD7, 0x868686, 0xCCAA55, 0x75B5AA, 0xD477A8,
        0x507AA6,
    ],
    gridlines: false,
};

#[cfg(feature = "python")]
const THEME_MINIMAL: ThemePreset = ThemePreset {
    bg: None,
    palette: &[
        0x374151, 0x6B7280, 0x9CA3AF, 0xD1D5DB, 0x111827, 0x4B5563, 0x1F2937, 0xE5E7EB, 0x030712,
        0x6B7280,
    ],
    gridlines: false,
};

#[cfg(feature = "python")]
const THEME_NEON: ThemePreset = ThemePreset {
    bg: Some("#0a0a0a"),
    palette: &[
        0x00FF87, 0xFF006E, 0x00B4D8, 0xFFBE0B, 0xE500A4, 0x8338EC, 0x3A86FF, 0xFB5607, 0xFF006E,
        0x06D6A0,
    ],
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
    }
    if let Some(value) = palette {
        if let Ok(mut field) = GLOBAL_PALETTE.lock() {
            *field = Some(value);
        }
    }
    if let Some(value) = gridlines {
        GLOBAL_GRIDLINES.store(value, Relaxed);
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
