pub mod core;
pub mod data;
pub mod plot;
pub mod ml;
pub mod cloud;
pub mod telemetry;
#[path = "bindings/doc_registry.rs"]
pub mod doc_registry;
#[path = "bindings/model_registry.rs"]
pub mod model_registry;

pub use data::{Dataset, DataPoint, DatasetStats};
pub use crate::core::hw_profile::HwProfile;

pub use seraplot_macros::{chart_demo, ml_doc, model, params, sera_alias, sera_bind, sera_class, sera_doc, sera_doc_impl, sera_impl, sera_sig};

include!(concat!(env!("OUT_DIR"), "/demo_registry.rs"));
include!(concat!(env!("OUT_DIR"), "/params_registry.rs"));
include!(concat!(env!("OUT_DIR"), "/sera_aliases.rs"));
include!(concat!(env!("OUT_DIR"), "/chart_alias_registry.rs"));

pub fn demo_kwargs(family: &str, variant: &str) -> Option<&'static str> {
    DEMO_REGISTRY.iter().find(|(f, v, _)| *f == family && *v == variant).map(|(_, _, k)| *k)
}

pub fn required_params_for(family: &str, variant: &str) -> Option<&'static [&'static str]> {
    if let Some(e) = PARAMS_REGISTRY.iter().find(|(f, v, _)| *f == family && *v == variant) {
        return Some(e.2);
    }
    PARAMS_REGISTRY.iter().find(|(f, v, _)| *f == family && *v == "basic").map(|e| e.2)
}

pub fn sera_aliases_for(key: &str) -> Option<&'static [&'static str]> {
    SERA_ALIASES.iter().find(|(k, _)| *k == key).map(|e| e.1)
}

pub fn demo_snippet(family: &str, variant: &str) -> Option<String> {
    let k = demo_kwargs(family, variant)?;
    let mut c = variant.chars();
    let title = c.next().map(|f| f.to_uppercase().chain(c).collect::<String>()).unwrap_or_default();
    let suffix = if variant == "basic" || variant == "default" { String::new() }
                 else { format!(",\n    variant=\"{}\"", variant) };
    Some(format!("import seraplot as sp\n\nc = sp.{}(\n    \"{} demo\",\n    {}{}\n)\n", family, title, k, suffix))
}

#[cfg(any(feature = "python", feature = "gui"))]

pub mod viewer;
pub mod bindings;
pub mod wiki;
pub mod html;

pub use core::math::{self, mean, median, std_dev};
pub use data::loader;
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
static GLOBAL_ANIMATION_DURATION: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(300);
static GLOBAL_CROSSHAIR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static GLOBAL_ZOOM: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static GLOBAL_TOOLTIP: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_LOCALE: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_THOUSANDS_SEP: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_MARGIN: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static GLOBAL_EXPORT_BTN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static GLOBAL_TEXT_AUTO: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_TEXT_POSITION: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static GLOBAL_TEXT_ANGLE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(i32::MIN);
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

#[sera_class]
pub struct Chart {
    html: String,
    doc_str: &'static str,
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
    s.push_str("else{ov.style.right='4px';ov.style.top='50%';ov.style.transform='translateY(-50%)';}");
    s.push_str("var dis=[];");
    s.push_str("var rb=document.createElement('span');rb.textContent='\\u21BA';rb.title='Show all';");
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

fn inject_labels(html: &str, pos: &str, labels: &[String], colors: &[String]) -> String {
    if html.contains("window.__SL__") {
        return html.to_string();
    }
    let forced = if labels.is_empty() { String::new() } else { encode_forced(labels, colors) };
    html.replacen("</body>", &build_labels_js(pos, &forced), 1)
}

const SP_CROSSHAIR_JS: &str = "(function(){if(window.__spc__)return;window.__spc__=1;var svg=document.querySelector('svg');if(!svg)return;var ns='http://www.w3.org/2000/svg';var vl=document.createElementNS(ns,'line');var hl=document.createElementNS(ns,'line');[vl,hl].forEach(function(l){l.setAttribute('stroke','#6366f1');l.setAttribute('stroke-width','1');l.setAttribute('stroke-dasharray','4,4');l.setAttribute('opacity','0.5');l.style.display='none';l.style.pointerEvents='none';svg.appendChild(l);});svg.addEventListener('mousemove',function(e){var r=svg.getBoundingClientRect();var x=e.clientX-r.left;var y=e.clientY-r.top;vl.setAttribute('x1',x);vl.setAttribute('x2',x);vl.setAttribute('y1',0);vl.setAttribute('y2',r.height);hl.setAttribute('x1',0);hl.setAttribute('x2',r.width);hl.setAttribute('y1',y);hl.setAttribute('y2',y);vl.style.display='';hl.style.display='';});svg.addEventListener('mouseleave',function(){vl.style.display='none';hl.style.display='none';});})()";

const SP_ZOOM_JS: &str = "(function(){if(window.__spz__)return;window.__spz__=1;var svg=document.querySelector('svg');if(!svg)return;var s=1,tx=0,ty=0,dr=false,sx,sy;svg.style.cursor='grab';svg.addEventListener('wheel',function(e){e.preventDefault();var z=e.deltaY<0?1.1:0.9;s=Math.min(Math.max(s*z,0.5),10);svg.style.transform='scale('+s+') translate('+tx+'px,'+ty+'px)';svg.style.transformOrigin='center center';},{passive:false});svg.addEventListener('mousedown',function(e){dr=true;sx=e.clientX-tx;sy=e.clientY-ty;svg.style.cursor='grabbing';});window.addEventListener('mouseup',function(){dr=false;if(svg)svg.style.cursor='grab';});svg.addEventListener('mousemove',function(e){if(!dr)return;tx=e.clientX-sx;ty=e.clientY-sy;svg.style.transform='scale('+s+') translate('+tx+'px,'+ty+'px)';});svg.addEventListener('dblclick',function(){s=1;tx=0;ty=0;svg.style.transform='';});})()";

const SP_FLIP_JS: &str = "(function(){if(window.__spfl__)return;window.__spfl__=1;var svg=document.querySelector('svg');if(!svg)return;var m=svg.getAttribute('data-sp');if(!m)return;var p=m.split(',').map(Number),pL=p[0],pT=p[1],pW=p[2],pH=p[3];var rects=svg.querySelectorAll('rect[data-idx][data-v]');if(!rects.length)return;var n=rects.length,vals=[],ymax=0,ymin=0;for(var i=0;i<n;i++){var v=parseFloat(rects[i].getAttribute('data-v'));vals.push(v);if(v>ymax)ymax=v;if(v<ymin)ymin=v;}var rg=ymax-ymin;if(rg<1e-12)rg=1;var slotH=pH/n,barH=Math.max(2,slotH*0.7);for(var i=0;i<n;i++){var v=vals[i];var newW=Math.max(1,(v-ymin)/rg*pW);var ny=pT+i*slotH+(slotH-barH)/2;rects[i].setAttribute('x',pL);rects[i].setAttribute('y',ny);rects[i].setAttribute('width',newW);rects[i].setAttribute('height',barH);}var xts=svg.querySelectorAll('.sp-xt');for(var k=0;k<xts.length&&k<n;k++){xts[k].setAttribute('y',pT+k*slotH+slotH/2+4);xts[k].setAttribute('x',pL-8);xts[k].setAttribute('text-anchor','end');}var yts=svg.querySelectorAll('.sp-yt'),nT=yts.length;for(var j=0;j<nT;j++){var f=nT>1?j/(nT-1):0;var nx=pL+f*pW;var v2=ymin+f*rg;yts[j].setAttribute('x',nx);yts[j].setAttribute('y',pT+pH+16);yts[j].setAttribute('text-anchor','middle');yts[j].textContent=Math.abs(v2)>=1000?Math.round(v2).toString():(+v2).toFixed(2);}var gls=svg.querySelectorAll('.sp-gl');for(var g=0;g<gls.length;g++){var f=gls.length>1?(g+1)/(gls.length+1):0.5;var nx=pL+f*pW;gls[g].setAttribute('x1',nx);gls[g].setAttribute('x2',nx);gls[g].setAttribute('y1',pT);gls[g].setAttribute('y2',pT+pH);}})()";

pub(crate) const SP_SORT_JS: &str = "(function(){if(window.__spso__)return;window.__spso__=1;var ord=window.__sp_sort__||'desc';if(ord==='none')return;var svg=document.querySelector('svg');if(!svg)return;var rects=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));if(rects.length<2)return;var items=rects.map(function(r){return{r:r,v:parseFloat(r.getAttribute('data-v'))||0,lb:r.getAttribute('data-lbl')||'',x:parseFloat(r.getAttribute('x'))||0,y:parseFloat(r.getAttribute('y'))||0,h:parseFloat(r.getAttribute('height'))||0,w:parseFloat(r.getAttribute('width'))||0};});var horizontal=items[0].h<items[0].w*0.5&&items[0].x<100;var cmp;if(ord==='asc')cmp=function(a,b){return a.v-b.v;};else if(ord==='desc')cmp=function(a,b){return b.v-a.v;};else if(ord==='alpha')cmp=function(a,b){return a.lb.localeCompare(b.lb);};else if(ord==='alpha_desc')cmp=function(a,b){return b.lb.localeCompare(a.lb);};else return;var sorted=items.slice().sort(cmp);var slots=horizontal?items.map(function(it){return it.y;}).sort(function(a,b){return a-b;}):items.map(function(it){return it.x;}).sort(function(a,b){return a-b;});var labels=items.map(function(it){return it.lb;});var newOrder=sorted.map(function(it){return it.lb;});for(var k=0;k<sorted.length;k++){if(horizontal)sorted[k].r.setAttribute('y',slots[k]);else sorted[k].r.setAttribute('x',slots[k]);}var ts=svg.querySelectorAll(horizontal?'.sp-yt':'.sp-xt');var labTs=[];ts.forEach(function(t){var tt=t.textContent.trim();if(labels.indexOf(tt)>=0)labTs.push(t);});if(labTs.length===newOrder.length){for(var i=0;i<newOrder.length;i++){labTs[i].textContent=newOrder[i];}}})()";

const SP_MARGIN_JS: &str = "(function(){if(window.__spmg__)return;window.__spmg__=1;var m=+window.__sp_margin_px__||0;if(m<=0)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp');if(!d)return;var p=d.split(',').map(Number),pL=p[0],pT=p[1],pW=p[2],pH=p[3];if(m*2>=pW||m*2>=pH)return;var nL=pL+m,nT=pT+m,nW=pW-2*m,nH=pH-2*m,sx=nW/pW,sy=nH/pH;var rx=function(x){return nL+(x-pL)*sx;};var ry=function(y){return nT+(y-pT)*sy;};var els=svg.querySelectorAll('[data-idx]');els.forEach(function(e){var tg=e.tagName;if(tg==='rect'){var x=parseFloat(e.getAttribute('x'))||0,y=parseFloat(e.getAttribute('y'))||0,w=parseFloat(e.getAttribute('width'))||0,h=parseFloat(e.getAttribute('height'))||0;e.setAttribute('x',rx(x));e.setAttribute('y',ry(y));e.setAttribute('width',w*sx);e.setAttribute('height',h*sy);}else if(tg==='circle'){var cx=parseFloat(e.getAttribute('cx'))||0,cy=parseFloat(e.getAttribute('cy'))||0;e.setAttribute('cx',rx(cx));e.setAttribute('cy',ry(cy));}else if(tg==='line'){e.setAttribute('x1',rx(parseFloat(e.getAttribute('x1'))||0));e.setAttribute('x2',rx(parseFloat(e.getAttribute('x2'))||0));e.setAttribute('y1',ry(parseFloat(e.getAttribute('y1'))||0));e.setAttribute('y2',ry(parseFloat(e.getAttribute('y2'))||0));}});svg.querySelectorAll('.sp-xt').forEach(function(t){var x=parseFloat(t.getAttribute('x'))||0,y=parseFloat(t.getAttribute('y'))||0;t.setAttribute('x',rx(x));if(y>pT+pH-2)t.setAttribute('y',ry(pT+pH)+8);});svg.querySelectorAll('.sp-yt').forEach(function(t){var x=parseFloat(t.getAttribute('x'))||0,y=parseFloat(t.getAttribute('y'))||0;t.setAttribute('y',ry(y));if(x<pL+2)t.setAttribute('x',rx(pL)-6);});svg.querySelectorAll('.sp-gl').forEach(function(g){g.setAttribute('x1',rx(parseFloat(g.getAttribute('x1'))||0));g.setAttribute('x2',rx(parseFloat(g.getAttribute('x2'))||0));g.setAttribute('y1',ry(parseFloat(g.getAttribute('y1'))||0));g.setAttribute('y2',ry(parseFloat(g.getAttribute('y2'))||0));});svg.querySelectorAll('.sp-ax-x,.sp-ax-y').forEach(function(a){var x1=a.getAttribute('x1'),x2=a.getAttribute('x2'),y1=a.getAttribute('y1'),y2=a.getAttribute('y2');if(x1!=null)a.setAttribute('x1',rx(parseFloat(x1)));if(x2!=null)a.setAttribute('x2',rx(parseFloat(x2)));if(y1!=null)a.setAttribute('y1',ry(parseFloat(y1)));if(y2!=null)a.setAttribute('y2',ry(parseFloat(y2)));});svg.setAttribute('data-sp',[nL,nT,nW,nH].join(','));})()";

pub(crate) const SP_LEGEND_JS: &str = "(function(){if(window.__spleg__)return;window.__spleg__=1;var pos=window.__sp_legend_pos__||'right';var svg=document.querySelector('svg');if(!svg)return;var ns='http://www.w3.org/2000/svg';var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var stl=document.createElementNS(ns,'style');stl.textContent='svg rect[data-idx],svg circle[data-idx],svg line[data-idx],svg path[data-idx]{transition:x .35s cubic-bezier(.22,1,.36,1),y .35s cubic-bezier(.22,1,.36,1),width .35s cubic-bezier(.22,1,.36,1),height .35s cubic-bezier(.22,1,.36,1),cx .35s cubic-bezier(.22,1,.36,1),cy .35s cubic-bezier(.22,1,.36,1),opacity .25s ease}svg .sp-xt,svg .sp-yt,svg .sp-gl{transition:x .35s cubic-bezier(.22,1,.36,1),y .35s cubic-bezier(.22,1,.36,1),x1 .35s cubic-bezier(.22,1,.36,1),x2 .35s cubic-bezier(.22,1,.36,1),y1 .35s cubic-bezier(.22,1,.36,1),y2 .35s cubic-bezier(.22,1,.36,1),opacity .25s ease}g[data-leg-se]{transition:opacity .2s ease}svg [data-idx][style*=\"display: none\"]{opacity:0}';svg.insertBefore(stl,svg.firstChild);var legs=svg.querySelectorAll('g[data-legend]');var items=[];if(legs.length){legs.forEach(function(lg){var rc=lg.querySelector('rect'),tx=lg.querySelector('text');items.push({lb:tx?tx.textContent:'',co:rc?rc.getAttribute('fill'):'#888',se:lg.getAttribute('data-series')});lg.style.display='none';});}else{var bars=svg.querySelectorAll('[data-idx][data-lbl]');var seen={};bars.forEach(function(b){var lb=b.getAttribute('data-lbl')||'';if(!lb||seen[lb])return;seen[lb]=1;items.push({lb:lb,co:b.getAttribute('fill')||b.getAttribute('stroke')||'#888',se:lb});});}if(!items.length)return;var allRInit=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));var nTot=allRInit.length;var fillRX=0.7,fillRY=0.7,isHBar=false;if(nTot>0){var ri0=allRInit[0];var ri0w=parseFloat(ri0.getAttribute('width'))||1;var ri0h=parseFloat(ri0.getAttribute('height'))||1;isHBar=ri0w>ri0h*1.5;if(!isHBar){fillRX=Math.min(0.95,ri0w/(pW/nTot));}else{fillRY=Math.min(0.95,ri0h/(pH/nTot));}}var w=parseFloat(svg.getAttribute('width'))||900;var h=parseFloat(svg.getAttribute('height'))||480;var IH=22,GAP=10,PAD=10,SW=10;var isH=pos==='top'||pos==='bottom';var extra=isH?(IH+PAD*2):0;var extraW=isH?0:items.reduce(function(a,it){return Math.max(a,it.lb.length*8+SW+PAD+24);},70);var childs=[];for(var ci=0;ci<svg.childNodes.length;ci++){var nd=svg.childNodes[ci];if(nd!==stl)childs.push(nd);}var wrap=document.createElementNS(ns,'g');childs.forEach(function(n){wrap.appendChild(n);});if(pos==='top')wrap.setAttribute('transform','translate(0,'+extra+')');if(pos==='left')wrap.setAttribute('transform','translate('+extraW+',0)');svg.appendChild(wrap);if(isH){svg.setAttribute('height',h+extra);svg.setAttribute('viewBox','0 0 '+w+' '+(h+extra));}else{svg.setAttribute('width',w+extraW);svg.setAttribute('viewBox','0 0 '+(w+extraW)+' '+h);}var g=document.createElementNS(ns,'g');g.setAttribute('class','sp-leg-grp');var ix=isH?pL:(pos==='right'?w+PAD:PAD);var iy=pos==='top'?(PAD+IH/2+4):(pos==='bottom'?(h+extra-PAD-IH/2):(pT+IH));items.forEach(function(it){var gg=document.createElementNS(ns,'g');gg.setAttribute('data-leg-se',it.se!=null?String(it.se):it.lb);gg.style.cursor='pointer';var r=document.createElementNS(ns,'rect');r.setAttribute('width',SW);r.setAttribute('height',SW);r.setAttribute('rx','2');r.setAttribute('fill',it.co);r.setAttribute('class','sp-leg-sw');var tx=document.createElementNS(ns,'text');tx.setAttribute('font-family','-apple-system,Arial,sans-serif');tx.setAttribute('font-size','11');tx.setAttribute('fill','#374151');tx.setAttribute('class','sp-leg-tx');tx.textContent=it.lb;if(isH){r.setAttribute('x',ix);r.setAttribute('y',iy-SW/2);tx.setAttribute('x',ix+SW+4);tx.setAttribute('y',iy+4);ix+=SW+4+it.lb.length*8+GAP;}else{r.setAttribute('x',ix);r.setAttribute('y',iy-SW/2);tx.setAttribute('x',ix+SW+4);tx.setAttribute('y',iy+4);iy+=IH+GAP;}gg.appendChild(r);gg.appendChild(tx);g.appendChild(gg);});var hidden={};function rescale(){var allR=Array.prototype.slice.call(svg.querySelectorAll('rect[data-idx][data-v]'));if(!allR.length)return;var visR=allR.filter(function(el){return el.style.display!=='none';});if(!visR.length)return;var nV=visR.length;var maxV=0;visR.forEach(function(el){var v=parseFloat(el.getAttribute('data-v'))||0;if(v>maxV)maxV=v;});if(maxV<=0)return;if(!isHBar){var nSW=pW/nV;var nBW=Math.max(1,nSW*fillRX);visR.forEach(function(el,i){var v=parseFloat(el.getAttribute('data-v'))||0;var nH=(v/maxV)*pH;el.setAttribute('x',pL+i*nSW+(nSW-nBW)/2);el.setAttribute('width',nBW);el.setAttribute('y',pT+pH-nH);el.setAttribute('height',nH);});var visXt=Array.prototype.slice.call(svg.querySelectorAll('.sp-xt')).filter(function(t){return t.style.display!=='none';});visXt.forEach(function(t,i){t.setAttribute('x',pL+i*nSW+nSW/2);});var yts=svg.querySelectorAll('.sp-yt');var nT=yts.length;for(var j=0;j<nT;j++){var fj=nT>1?j/(nT-1):0;var vj=fj*maxV;yts[j].setAttribute('y',pT+pH-(fj*pH)+4);yts[j].textContent=vj>=1000?Math.round(vj).toString():(+vj).toFixed(2);}var gls=svg.querySelectorAll('.sp-gl');for(var gi=0;gi<gls.length;gi++){var fg=gls.length>1?gi/(gls.length-1):0;var gy=pT+pH-(fg*pH);gls[gi].setAttribute('y1',gy);gls[gi].setAttribute('y2',gy);}}else{var nSH=pH/nV;var nBH=Math.max(1,nSH*fillRY);visR.forEach(function(el,i){var v=parseFloat(el.getAttribute('data-v'))||0;var nW=(v/maxV)*pW;el.setAttribute('y',pT+i*nSH+(nSH-nBH)/2);el.setAttribute('height',nBH);el.setAttribute('x',pL);el.setAttribute('width',nW);});var visYt=Array.prototype.slice.call(svg.querySelectorAll('.sp-yt')).filter(function(t){return t.style.display!=='none';});visYt.forEach(function(t,i){t.setAttribute('y',pT+i*nSH+nSH/2+4);});var xts2=svg.querySelectorAll('.sp-xt');var nX=xts2.length;for(var k=0;k<nX;k++){var fk=nX>1?k/(nX-1):0;var vk=fk*maxV;xts2[k].setAttribute('x',pL+fk*pW);xts2[k].textContent=vk>=1000?Math.round(vk).toString():(+vk).toFixed(2);}}}g.querySelectorAll('[data-leg-se]').forEach(function(grp){var se=grp.getAttribute('data-leg-se');grp.addEventListener('click',function(){var isHiding=!hidden[se];hidden[se]=isHiding;svg.querySelectorAll('[data-lbl=\"'+se+'\"],[data-series=\"'+se+'\"]').forEach(function(el){if(el.classList.contains('sp-leg-sw')||el.classList.contains('sp-leg-tx'))return;if(isHiding){el.style.opacity='0';setTimeout(function(){el.style.display='none';},250);}else{el.style.display='';requestAnimationFrame(function(){el.style.opacity='';});}});svg.querySelectorAll('.sp-xt,.sp-yt').forEach(function(t){if(t.textContent.trim()===se){if(isHiding){t.style.opacity='0';setTimeout(function(){t.style.display='none';},250);}else{t.style.display='';requestAnimationFrame(function(){t.style.opacity='';});}}});grp.style.opacity=isHiding?'0.35':'';if(isHiding){setTimeout(rescale,260);}else{requestAnimationFrame(rescale);}});});svg.appendChild(g);})()";



const SP_AUTOCLASS_JS: &str = "(function(){if(window.__spac__)return;window.__spac__=1;var svgs=document.querySelectorAll('svg');svgs.forEach(function(svg){var d=svg.getAttribute('data-sp');if(!d)return;var p=d.split(',').map(Number),pL=p[0]||0,pT=p[1]||0,pW=p[2]||0,pH=p[3]||0;if(pW<=0||pH<=0)return;var bX=pT+pH;var lX=pL;svg.querySelectorAll('text').forEach(function(t){if(t.getAttribute('class'))return;if(t.hasAttribute('data-idx'))return;if(t.hasAttribute('data-series'))return;var tx=parseFloat(t.getAttribute('x'));var ty=parseFloat(t.getAttribute('y'));if(!isFinite(tx)||!isFinite(ty))return;var ta=t.getAttribute('text-anchor')||'';var inXBand=ty>=bX-2&&ty<=bX+30;var inYBand=ty>=pT-4&&ty<=pT+pH+4&&tx<=lX+2&&tx>=lX-80;if(inXBand&&tx>=pL-5&&tx<=pL+pW+5){t.setAttribute('class','sp-xt');}else if(inYBand||(ta==='end'&&tx<lX&&ty>=pT-2&&ty<=pT+pH+12)){t.setAttribute('class','sp-yt');}});});})()";

const SP_BAR_GAP_JS: &str = "(function(){if(window.__spbg__)return;window.__spbg__=1;var gap=window.__sp_bar_gap__;if(gap==null)return;var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||50,pT=sp[1]||36,pW=sp[2]||700,pH=sp[3]||360;var rects=svg.querySelectorAll('rect[data-idx][data-v]');if(!rects.length)return;var isHoriz=parseFloat(rects[0].getAttribute('width'))>parseFloat(rects[0].getAttribute('height'));var n=rects.length;var maxV=0;rects.forEach(function(r){var v=parseFloat(r.getAttribute('data-v'))||0;if(v>maxV)maxV=v;});if(maxV<=0)return;if(isHoriz){var slotH=pH/n;var barH=slotH*(1-gap);rects.forEach(function(r,i){var v=parseFloat(r.getAttribute('data-v'))||0;var bw=(v/maxV)*pW;var by=pT+i*slotH+(slotH-barH)/2;r.setAttribute('y',by);r.setAttribute('height',barH);r.setAttribute('x',pL);r.setAttribute('width',Math.max(1,bw));});var xts=svg.querySelectorAll('.sp-xt');xts.forEach(function(t,i){if(i<n)t.setAttribute('y',pT+i*(pH/n)+(pH/n)/2+4);});}else{var slotW=pW/n;var barW=slotW*(1-gap);rects.forEach(function(r,i){var v=parseFloat(r.getAttribute('data-v'))||0;var bh=(v/maxV)*pH;var bx=pL+i*slotW+(slotW-barW)/2;var by=pT+pH-bh;r.setAttribute('x',bx);r.setAttribute('width',Math.max(1,barW));r.setAttribute('y',by);r.setAttribute('height',bh);});var xts=svg.querySelectorAll('.sp-xt');xts.forEach(function(t,i){if(i<n){var cx=pL+i*slotW+slotW/2;t.setAttribute('x',cx);}});}})()";

pub(crate) const SP_TEXT_JS: &str = "(function(){var o=window.__sp_text__||{};if(window.__spt__)return;window.__spt__=1;var fmt=function(v,f){if(f==null||f===true||f==='true'||f==='')return (Math.round(v*1000)/1000).toString();var m=/^\\.(\\d+)([fs%eg])$/.exec(f);if(!m)return String(v);var d=+m[1],t=m[2];if(t==='f')return (+v).toFixed(d);if(t==='%')return ((+v)*100).toFixed(d)+'%';if(t==='e')return (+v).toExponential(d);if(t==='g')return (+v).toPrecision(d);if(t==='s'){var u=['','K','M','B','T'],a=Math.abs(+v),i=0;while(a>=1000&&i<u.length-1){a/=1000;i++;}var sn=(+v)<0?-a:a;return sn.toFixed(d)+u[i];}return String(v);};var ns='http://www.w3.org/2000/svg';var pos=o.position||'auto',ang=o.angle==null?0:+o.angle,fs=o.font_size||11,col=o.color||'#1f2937',ff=o.font_family||'system-ui,Arial,sans-serif',fmtS=o.format,umin=o.uniform_min||0,umode=o.uniform_mode||'';document.querySelectorAll('svg [data-v]').forEach(function(el){if(el.tagName==='text')return;if(el.getAttribute('data-sp-text')==='1')return;el.setAttribute('data-sp-text','1');var v=parseFloat(el.getAttribute('data-v'));if(!isFinite(v))return;var svg=el.ownerSVGElement;if(!svg)return;var bb;try{bb=el.getBBox();}catch(e){return;}var cx=bb.x+bb.width/2,tx,ty,ta='middle',pp=pos;if(pp==='auto')pp=(el.tagName==='rect'&&bb.height>fs*1.6)?'inside':'outside';if(el.tagName==='rect'){var isHoriz=bb.width>bb.height*1.5&&bb.x>50;if(pp==='inside'){tx=cx;ty=bb.y+bb.height/2+fs/3;}else if(pp==='outside'){tx=cx;ty=bb.y-4;}else{tx=cx;ty=bb.y-4;}if(isHoriz&&pp==='outside'){tx=bb.x+bb.width+6;ty=bb.y+bb.height/2+fs/3;ta='start';}}else{tx=cx;ty=bb.y-6;}var t=document.createElementNS(ns,'text');t.setAttribute('x',tx);t.setAttribute('y',ty);t.setAttribute('text-anchor',ta);t.setAttribute('font-family',ff);t.setAttribute('font-size',fs);t.setAttribute('fill',col);t.setAttribute('pointer-events','none');t.setAttribute('class','sp-vt');if(ang)t.setAttribute('transform','rotate('+ang+' '+tx+' '+ty+')');t.textContent=fmt(v,fmtS);el.parentNode.appendChild(t);if(umin>0){var rect=t.getBBox();if(rect.width>bb.width&&umode==='hide')t.style.display='none';}});})()";

const SP_BAR_RADIUS_JS: &str = "(function(){var r=window.__sp_bar_r__;if(r==null||window.__spbr__)return;window.__spbr__=1;document.querySelectorAll('svg rect[data-idx]').forEach(function(el){var v=r;if(typeof r==='string'&&r.charAt(r.length-1)==='%'){var bb;try{bb=el.getBBox();}catch(e){return;}var p=parseFloat(r)/100;v=Math.min(bb.width,bb.height)*p;}el.setAttribute('rx',v);el.setAttribute('ry',v);});})()";

const SP_EXPORT_JS: &str = "(function(){if(window.__spe__)return;window.__spe__=1;var c=document.querySelector('.chart-container')||document.querySelector('.c3w')||document.body;if(getComputedStyle(c).position==='static')c.style.position='relative';var b=document.createElement('button');b.textContent='\u{2B07}';b.title='Download chart';b.style.cssText='position:absolute;top:8px;right:8px;z-index:300;background:#6366f1;color:#fff;border:none;border-radius:6px;width:32px;height:32px;font-size:16px;cursor:pointer;opacity:0.6;transition:opacity .2s';b.onmouseenter=function(){b.style.opacity='1';};b.onmouseleave=function(){b.style.opacity='0.6';};b.onclick=function(ev){ev.preventDefault();ev.stopPropagation();try{var html='<!DOCTYPE html>\\n'+document.documentElement.outerHTML;var bl=new Blob([html],{type:'text/html;charset=utf-8'});var url=URL.createObjectURL(bl);var a=document.createElement('a');a.href=url;a.download='chart.html';a.rel='noopener';a.style.display='none';document.body.appendChild(a);a.click();setTimeout(function(){try{document.body.removeChild(a);URL.revokeObjectURL(url);}catch(e){}},100);}catch(e){try{var w=window.open('','_blank');if(w){w.document.write(document.documentElement.outerHTML);w.document.close();}}catch(_){}}};c.appendChild(b);})()";

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
    let text_auto_v = GLOBAL_TEXT_AUTO.lock().ok().and_then(|g| g.clone());
    let text_pos_v = GLOBAL_TEXT_POSITION.lock().ok().and_then(|g| g.clone());
    let text_angle_v = GLOBAL_TEXT_ANGLE.load(Relaxed);
    let text_fs_v = GLOBAL_TEXT_FONT_SIZE.load(Relaxed);
    let text_col_v = GLOBAL_TEXT_FONT_COLOR.lock().ok().and_then(|g| g.clone());
    let utext_min = GLOBAL_UNIFORM_TEXT_MIN.load(Relaxed);
    let utext_mode = GLOBAL_UNIFORM_TEXT_MODE.lock().ok().and_then(|g| g.clone());
    if text_auto_v.is_some() || text_pos_v.is_some() || text_angle_v != i32::MIN || text_fs_v > 0 || text_col_v.is_some() {
        let mut opts = String::from("window.__sp_text__={");
        if let Some(ref t) = text_auto_v { opts.push_str(&format!("format:{},", json_str(t))); }
        if let Some(ref p) = text_pos_v { opts.push_str(&format!("position:{},", json_str(p))); }
        if text_angle_v != i32::MIN { opts.push_str(&format!("angle:{},", text_angle_v)); }
        if text_fs_v > 0 { opts.push_str(&format!("font_size:{},", text_fs_v)); }
        if let Some(ref c) = text_col_v { opts.push_str(&format!("color:{},", json_str(c))); }
        if utext_min > 0 { opts.push_str(&format!("uniform_min:{},", utext_min)); }
        if let Some(ref m) = utext_mode { opts.push_str(&format!("uniform_mode:{},", json_str(m))); }
        opts.push_str("};");
        js.push_str(&opts);
        js.push_str(SP_TEXT_JS);
        js.push(';');
    }
    let bar_r = GLOBAL_BAR_CORNER_RADIUS.lock().ok().and_then(|g| g.clone());
    if let Some(r) = bar_r {
        js.push_str(&format!("window.__sp_bar_r__={};", if r.ends_with('%') { json_str(&r) } else { r.parse::<f64>().map(|v| v.to_string()).unwrap_or_else(|_| json_str(&r)) }));
        js.push_str(SP_BAR_RADIUS_JS);
        js.push(';');
    }
    js.push_str(SP_AUTOCLASS_JS);
    js.push(';');
    let mut out = html;
    if !css.is_empty() { out = out.replacen("</head>", &format!("<style>{}</style></head>", css), 1); }
    out = out.replacen("</body>", &format!("<script>{}</script></body>", js), 1);
    out
}

impl Chart {
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

    #[sera_doc(category = "chart_method", file = "charts/export.md", en = "Saves the chart HTML to a file at the given path.", fr = "Enregistre le HTML du graphique dans un fichier au chemin indiqué.", param(name = "path", ty = "str", en = "Destination file path (e.g. 'chart.html').", fr = "Chemin du fichier de destination (ex: 'chart.html')."))]
    #[sera_sig(path)]
    #[sera_wasm_skip]
    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, &self.html)
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Sets the background color of the chart. Pass None to remove the background.", fr = "Définit la couleur d'arrière-plan du graphique. Passez None pour supprimer l'arrière-plan.", param(name = "color", ty = "str | None", en = "CSS color string (hex, rgb, named). None removes the background.", fr = "Couleur CSS (hex, rgb, nommée). None supprime l'arrière-plan."))]
    #[sera_sig(color=None)]
    pub fn set_bg(&self, color: Option<&str>) -> Chart {
        self.propagate(crate::html::hover::apply_bg(self.html.clone(), color))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Injects a raw CSS string into the chart's <head> element.", fr = "Injecte une chaîne CSS brute dans l'élément <head> du graphique.", param(name = "css", ty = "str", en = "Raw CSS rules to inject.", fr = "Règles CSS brutes à injecter."))]
    #[sera_sig(css)]
    pub fn inject_css(&self, css: &str) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>{css}</style></head>"), 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Injects a raw JavaScript string into the chart's <body> element.", fr = "Injecte une chaîne JavaScript brute dans l'élément <body> du graphique.", param(name = "js", ty = "str", en = "Raw JavaScript code to inject.", fr = "Code JavaScript brut à injecter."))]
    #[sera_sig(js)]
    pub fn inject_js(&self, js: &str) -> Chart {
        self.propagate(self.html.replacen("</body>", &format!("<script>{js}</script></body>"), 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Hides the X axis, its ticks, and its label.", fr = "Masque l'axe X, ses graduations et son étiquette.")]
    pub fn no_x_axis(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-ax-x,.sp-xt,.sp-xl{display:none}</style></head>", 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Disables the hover tooltip and removes hover highlighting on data elements.", fr = "Désactive l'infobulle au survol et supprime le surlignage des éléments au survol.")]
    pub fn no_hover(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>#sp-tip{display:none!important}[data-idx]{pointer-events:none!important}[data-idx]:hover{filter:none!important}</style></head>", 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Hides the Y axis, its ticks, and its label.", fr = "Masque l'axe Y, ses graduations et son étiquette.")]
    pub fn no_y_axis(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-ax-y,.sp-yt,.sp-yl{display:none}</style></head>", 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Hides both X and Y axes along with their ticks and labels.", fr = "Masque les axes X et Y ainsi que leurs graduations et étiquettes.")]
    pub fn no_axes(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-ax-x,.sp-ax-y,.sp-xt,.sp-yt,.sp-xl,.sp-yl{display:none}</style></head>", 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Shows horizontal and vertical grid lines on the chart background.", fr = "Affiche les lignes de grille horizontales et verticales en arrière-plan du graphique.")]
    pub fn show_grid(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-gl{display:block!important;opacity:1!important}</style></head>", 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Hides the grid lines if they were previously enabled.", fr = "Masque les lignes de grille si elles étaient précédemment activées.")]
    pub fn hide_grid(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-gl{display:none!important}</style></head>", 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Overrides all SVG text elements to the specified font size in pixels.", fr = "Remplace la taille de police de tous les éléments texte SVG par la valeur spécifiée en pixels.", param(name = "px", ty = "int", en = "Font size in pixels.", fr = "Taille de police en pixels."))]
    #[sera_sig(px)]
    pub fn set_font_size(&self, px: u32) -> Chart {
        let style = format!("<style>svg text{{font-size:{}px!important}}</style></head>", px);
        self.propagate(self.html.replacen("</head>", &style, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Scales the entire SVG by a given factor from the top-left origin.", fr = "Met à l'échelle l'intégralité du SVG par un facteur donné depuis le coin supérieur gauche.", param(name = "factor", ty = "float", en = "Scale multiplier (e.g. 1.5 for 150%).", fr = "Multiplicateur d'échelle (ex: 1.5 pour 150%)."))]
    #[sera_sig(factor)]
    pub fn scale(&self, factor: f64) -> Chart {
        let style = format!("<style>svg{{transform:scale({});transform-origin:top left}}</style></head>", factor);
        self.propagate(self.html.replacen("</head>", &style, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Sets the background color of the SVG/canvas frame. Use 'transparent' or None to remove it.", fr = "Définit la couleur d'arrière-plan du cadre SVG/canvas. Utilisez 'transparent' ou None pour le supprimer.", param(name = "color", ty = "str | None", en = "CSS color for the frame background.", fr = "Couleur CSS pour l'arrière-plan du cadre."))]
    #[sera_sig(color=None)]
    pub fn set_frame(&self, color: Option<&str>) -> Chart {
        let bg = match color {
            None | Some("none") | Some("transparent") | Some("") => "transparent".to_string(),
            Some(c) => c.to_string(),
        };
        let style = format!("<style>svg{{background:{bg}!important}}.c3w canvas{{background:{bg}!important}}.c3w{{background:{bg}!important}}</style></head>");
        self.propagate(self.html.replacen("</head>", &style, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Adds an interactive series filter overlay with clickable pill-shaped labels.", fr = "Ajoute une superposition de filtre de séries interactif avec des étiquettes en forme de pilule cliquables.", param(name = "position", ty = "str", en = "Position of the overlay: 'top', 'bottom', 'left', 'right'.", fr = "Position de la superposition: 'top', 'bottom', 'left', 'right'."), param(name = "labels", ty = "list[str] | None", en = "Custom label names. Auto-detected if None.", fr = "Noms d'étiquettes personnalisés. Détection automatique si None."), param(name = "colors", ty = "list[str] | None", en = "Custom color hex strings matching labels.", fr = "Couleurs hex personnalisées correspondant aux étiquettes."))]
    #[sera_sig(position="bottom", labels=None, colors=None)]
    pub fn show_labels(&self, position: &str, labels: Option<Vec<String>>, colors: Option<Vec<String>>) -> Chart {
        let lb = labels.unwrap_or_default();
        let co = colors.unwrap_or_default();
        self.propagate(inject_labels(&self.html, position, &lb, &co))
    }

    #[sera_doc(category = "chart_method", file = "charts/export.md", en = "Extracts and returns the raw SVG string from the chart HTML, or None if not present.", fr = "Extrait et retourne la chaîne SVG brute depuis le HTML du graphique, ou None si absente.")]
    pub fn to_svg(&self) -> Option<String> {
        let h = &self.html;
        let start = h.find("<svg")?;
        let end = h.rfind("</svg>")? + 6;
        Some(h[start..end].to_string())
    }

    #[sera_doc(category = "chart_method", file = "charts/export.md", en = "Saves the chart's SVG to a file.", fr = "Enregistre le SVG du graphique dans un fichier.", param(name = "path", ty = "str", en = "Destination .svg file path.", fr = "Chemin du fichier .svg de destination."))]
    #[sera_sig(path)]
    #[sera_wasm_skip]
    pub fn export_svg(&self, path: &str) -> Result<(), std::io::Error> {
        let svg = self.to_svg().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "No SVG in chart"))?;
        std::fs::write(path, svg)
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Sets the font family for all SVG text and body text in the chart.", fr = "Définit la famille de polices pour tous les textes SVG et les textes du corps du graphique.", param(name = "name", ty = "str", en = "Font family name (e.g. 'Roboto', 'Inter').", fr = "Nom de la famille de polices (ex: 'Roboto', 'Inter')."))]
    #[sera_sig(name)]
    pub fn font(&self, name: &str) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>svg text,body{{font-family:'{}',system-ui,sans-serif!important}}</style></head>", name), 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Sets the font size of the chart title in pixels.", fr = "Définit la taille de police du titre du graphique en pixels.", param(name = "px", ty = "int", en = "Title font size in pixels.", fr = "Taille de police du titre en pixels."))]
    #[sera_sig(px)]
    pub fn title_size(&self, px: i32) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>.sp-ttl{{font-size:{}px!important}}</style></head>", px), 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Adds an interactive crosshair that follows the mouse cursor across the SVG.", fr = "Ajoute un réticule interactif qui suit le curseur de la souris sur le SVG.")]
    pub fn crosshair(&self) -> Chart {
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", SP_CROSSHAIR_JS), 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Enables mouse-wheel zoom and click-drag panning on the chart. Double-click to reset.", fr = "Active le zoom à la molette et le déplacement par glisser-cliquer. Double-clic pour réinitialiser.")]
    pub fn zoom(&self) -> Chart {
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", SP_ZOOM_JS), 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Flips a vertical bar chart into a horizontal bar chart by recalculating bar positions.", fr = "Transforme un graphique à barres verticales en graphique à barres horizontales en recalculant les positions.")]
    pub fn flip(&self) -> Chart {
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", SP_FLIP_JS), 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Alias for flip(). Renders the chart with horizontal bars.", fr = "Alias de flip(). Affiche le graphique avec des barres horizontales.")]
    pub fn horizontal(&self) -> Chart {
        self.flip()
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Rotates the entire chart by a snapped angle (0, 90, 180 or 270 degrees).", fr = "Fait pivoter l'intégralité du graphique selon un angle arrondi (0, 90, 180 ou 270 degrés).", param(name = "deg", ty = "int", en = "Rotation in degrees, snapped to nearest 90°. Default: 90.", fr = "Rotation en degrés, arrondie au 90° le plus proche. Défaut: 90."))]
    #[sera_sig(deg=90)]
    pub fn rotate(&self, deg: i32) -> Chart {
        let d = ((deg % 360) + 360) % 360;
        let snapped = match d { 0..=44 | 316..=359 => 0, 45..=134 => 90, 135..=224 => 180, _ => 270 };
        self.propagate(crate::html::hover::apply_rotation(self.html.clone(), snapped))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Sorts chart bars by value or label using a client-side JavaScript re-render.", fr = "Trie les barres du graphique par valeur ou étiquette via un rendu JavaScript côté client.", param(name = "order", ty = "str", en = "Sort order: 'desc' (default), 'asc', 'alpha', 'alpha_desc', 'none'.", fr = "Ordre de tri: 'desc' (défaut), 'asc', 'alpha', 'alpha_desc', 'none'."))]
    #[sera_sig(order="desc")]
    pub fn sort_by(&self, order: &str) -> Chart {
        let ord = match order { "asc" | "desc" | "alpha" | "alpha_desc" | "none" => order, _ => "desc" };
        let snippet = format!("<script>window.__sp_sort__={};{}</script></body>", json_str(ord), SP_SORT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Repositions the chart legend and enables interactive series toggling by click.", fr = "Repositionne la légende du graphique et active la bascule interactive des séries au clic.", param(name = "position", ty = "str", en = "Legend position: 'right' (default), 'left', 'top', 'bottom', 'none'.", fr = "Position de la légende: 'right' (défaut), 'left', 'top', 'bottom', 'none'."))]
    #[sera_sig(position="right")]
    pub fn legend(&self, position: &str) -> Chart {
        let pos = match position { "right" | "left" | "top" | "bottom" | "none" => position, _ => "right" };
        if pos == "none" { return self.no_legend(); }
        let snippet = format!("<script>window.__sp_legend_pos__={};{}</script></body>", json_str(pos), SP_LEGEND_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Rotates X axis tick labels by the specified angle in degrees.", fr = "Fait pivoter les étiquettes de graduation de l'axe X de l'angle spécifié en degrés.", param(name = "angle", ty = "int", en = "Rotation angle in degrees (e.g. -45 for diagonal labels).", fr = "Angle de rotation en degrés (ex: -45 pour des étiquettes diagonales)."))]
    #[sera_sig(angle)]
    pub fn rotate_labels(&self, angle: i32) -> Chart {
        let css = format!("<style>.sp-xt{{transform-box:fill-box;transform-origin:center;transform:rotate({}deg)}}</style></head>", angle);
        self.propagate(self.html.replacen("</head>", &css, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Makes the SVG width 100% of its container while keeping proportional height.", fr = "Rend la largeur du SVG égale à 100% de son conteneur tout en conservant une hauteur proportionnelle.")]
    pub fn responsive(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>svg{width:100%!important;height:auto!important}</style></head>", 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Adds a staggered entry animation to data elements (bars, circles, areas).", fr = "Ajoute une animation d'entrée décalée aux éléments de données (barres, cercles, zones).", param(name = "duration", ty = "int", en = "Animation duration in milliseconds. Default: 300.", fr = "Durée de l'animation en millisecondes. Défaut: 300."))]
    #[sera_sig(duration=300)]
    pub fn animate(&self, duration: i32) -> Chart {
        let css = format!("<style>@keyframes sp-in{{from{{opacity:0;transform:translateY(8px)}}to{{opacity:1;transform:none}}}}svg rect[data-idx],svg circle[data-idx],svg path.sp-area{{animation:sp-in {}ms ease-out both}}</style></head>", duration);
        let js = "<script>(function(){if(window.__spa__)return;window.__spa__=1;var els=document.querySelectorAll('svg [data-idx]');for(var i=0;i<els.length;i++)els[i].style.animationDelay=i*30+'ms';})();</script></body>";
        self.propagate(self.html.replacen("</head>", &css, 1).replacen("</body>", js, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Applies a CSS border-radius to the chart container element.", fr = "Applique un border-radius CSS à l'élément conteneur du graphique.", param(name = "px", ty = "int", en = "Corner radius in pixels.", fr = "Rayon des coins en pixels."))]
    #[sera_sig(px)]
    pub fn border_radius(&self, px: i32) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>[id^='spp'],.c3w{{border-radius:{}px!important;overflow:hidden}}</style></head>", px), 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Sets the opacity of all data elements (bars, circles, areas) in the chart.", fr = "Définit l'opacité de tous les éléments de données (barres, cercles, zones) du graphique.", param(name = "value", ty = "float", en = "Opacity between 0.0 (invisible) and 1.0 (fully opaque).", fr = "Opacité entre 0.0 (invisible) et 1.0 (totalement opaque)."))]
    #[sera_sig(value)]
    pub fn set_opacity(&self, value: f64) -> Chart {
        let v = value.clamp(0.0, 1.0);
        self.propagate(self.html.replacen("</head>", &format!("<style>svg rect[data-idx],svg circle[data-idx],svg path.sp-area{{opacity:{}!important}}</style></head>", v), 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Adds internal padding to the chart and adjusts data element positions accordingly.", fr = "Ajoute un espacement interne au graphique et ajuste en conséquence les positions des éléments de données.", param(name = "px", ty = "int", en = "Margin in pixels applied to all four sides.", fr = "Marge en pixels appliquée aux quatre côtés."))]
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

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Adjusts the gap ratio between bars. Higher values create thinner bars with more space.", fr = "Ajuste le ratio d'espacement entre les barres. Des valeurs plus élevées créent des barres plus fines.", param(name = "ratio", ty = "float", en = "Gap ratio between 0.0 (no gap) and 0.95 (almost no bar). Default: 0.3.", fr = "Ratio d'espacement entre 0.0 (sans espacement) et 0.95 (presque sans barre). Défaut: 0.3."))]
    #[sera_sig(ratio=0.3)]
    pub fn bar_gap(&self, ratio: f64) -> Chart {
        let r = ratio.clamp(0.0, 0.95);
        let snippet = format!("<script>window.__sp_bar_gap__={:.4};{}</script></body>", r, SP_BAR_GAP_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Applies CSS padding to the chart container element.", fr = "Applique un padding CSS à l'élément conteneur du graphique.", param(name = "px", ty = "int", en = "Padding in pixels applied to all four sides.", fr = "Padding en pixels appliqué aux quatre côtés."))]
    #[sera_sig(px)]
    pub fn set_padding(&self, px: i32) -> Chart {
        let css = format!("<style>[id^='spp'],.c3w{{padding:{px}px!important;box-sizing:border-box}}</style></head>");
        self.propagate(self.html.replacen("</head>", &css, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Rotates X and/or Y axis tick labels independently.", fr = "Fait pivoter indépendamment les étiquettes de graduation des axes X et/ou Y.", param(name = "x_angle", ty = "int | None", en = "Rotation angle for X axis labels in degrees.", fr = "Angle de rotation des étiquettes de l'axe X en degrés."), param(name = "y_angle", ty = "int | None", en = "Rotation angle for Y axis labels in degrees.", fr = "Angle de rotation des étiquettes de l'axe Y en degrés."))]
    #[sera_sig(x_angle=None, y_angle=None)]
    pub fn axis_label_angle(&self, x_angle: Option<i32>, y_angle: Option<i32>) -> Chart {
        let mut css = String::from("<style>");
        if let Some(a) = x_angle { css.push_str(&format!(".sp-xt{{transform-box:fill-box;transform-origin:center;transform:rotate({}deg)}}", a)); }
        if let Some(a) = y_angle { css.push_str(&format!(".sp-yt{{transform-box:fill-box;transform-origin:center;transform:rotate({}deg)}}", a)); }
        css.push_str("</style></head>");
        self.propagate(self.html.replacen("</head>", &css, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Hides the chart legend.", fr = "Masque la légende du graphique.")]
    pub fn no_legend(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>g[data-legend]{display:none!important}</style></head>", 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Hides the chart title.", fr = "Masque le titre du graphique.")]
    pub fn no_title(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-ttl{display:none!important}</style></head>", 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Forces the chart title to be visible with a contrast stroke for readability.", fr = "Force le titre du graphique à être visible avec un contour de contraste pour la lisibilité.")]
    pub fn show_title(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-ttl{display:block!important;visibility:visible!important;opacity:1!important;fill:#e2e8f0!important;paint-order:stroke;stroke:rgba(0,0,0,.6);stroke-width:.6px}</style></head>", 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Forces the chart legend to be visible even if it was hidden.", fr = "Force la légende du graphique à être visible même si elle était masquée.")]
    pub fn show_legend(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>g[data-legend]{display:block!important;visibility:visible!important}</style></head>", 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Alias for legend(position). Repositions the chart legend.", fr = "Alias de legend(position). Repositionne la légende du graphique.", param(name = "position", ty = "str", en = "Legend position: 'right', 'left', 'top', 'bottom', 'none'.", fr = "Position de la légende: 'right', 'left', 'top', 'bottom', 'none'."))]
    #[sera_sig(position="right")]
    pub fn legend_position(&self, position: &str) -> Chart {
        self.legend(position)
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Alias for show_labels(position). Adds an interactive legend overlay at the given position.", fr = "Alias de show_labels(position). Ajoute une superposition de légende interactive à la position donnée.", param(name = "position", ty = "str", en = "Overlay position: 'top', 'bottom', 'left', 'right'.", fr = "Position de la superposition: 'top', 'bottom', 'left', 'right'."))]
    #[sera_sig(position="bottom")]
    pub fn label_position(&self, position: &str) -> Chart {
        self.show_labels(position, None, None)
    }

    #[sera_doc(category = "chart_method", file = "charts/export.md", en = "Adds a floating download button (?) to the chart that saves the full HTML on click.", fr = "Ajoute un bouton de téléchargement flottant (?) au graphique qui sauvegarde le HTML complet au clic.")]
    pub fn export_button(&self) -> Chart {
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", SP_EXPORT_JS), 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Overlays data value labels on all chart elements. Supports format strings and positioning.", fr = "Superpose des étiquettes de valeurs de données sur tous les éléments du graphique. Supporte les chaînes de format et le positionnement.", param(name = "format", ty = "str | None", en = "Format string (e.g. '.2f', '.0%') or empty string for auto. None disables.", fr = "Chaîne de format (ex: '.2f', '.0%') ou chaîne vide pour automatique. None désactive."), param(name = "position", ty = "str | None", en = "Label position: 'auto', 'inside', 'outside'.", fr = "Position de l'étiquette: 'auto', 'inside', 'outside'."), param(name = "angle", ty = "int | None", en = "Label rotation angle in degrees.", fr = "Angle de rotation des étiquettes en degrés."), param(name = "font_size", ty = "int | None", en = "Font size of the data labels in pixels.", fr = "Taille de police des étiquettes de données en pixels."), param(name = "color", ty = "str | None", en = "Color of the data labels.", fr = "Couleur des étiquettes de données."))]
    #[sera_sig(format=None, position=None, angle=None, font_size=None, color=None)]
    pub fn text_auto(&self, format: Option<&str>, position: Option<&str>, angle: Option<i32>, font_size: Option<i32>, color: Option<&str>) -> Chart {
        let mut opts = String::from("window.__sp_text__={");
        if let Some(f) = format { opts.push_str(&format!("format:{},", json_str(f))); }
        if let Some(p) = position { opts.push_str(&format!("position:{},", json_str(p))); }
        if let Some(a) = angle { opts.push_str(&format!("angle:{},", a)); }
        if let Some(s) = font_size { opts.push_str(&format!("font_size:{},", s)); }
        if let Some(c) = color { opts.push_str(&format!("color:{},", json_str(c))); }
        opts.push_str("};");
        let snippet = format!("<script>{}{}</script></body>", opts, SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Sets the position for data value labels on chart elements.", fr = "Définit la position des étiquettes de valeurs de données sur les éléments du graphique.", param(name = "position", ty = "str", en = "Position: 'auto', 'inside', 'outside'.", fr = "Position: 'auto', 'inside', 'outside'."))]
    #[sera_sig(position)]
    pub fn text_position(&self, position: &str) -> Chart {
        let snippet = format!("<script>window.__sp_text__=Object.assign(window.__sp_text__||{{}},{{position:{}}});{}</script></body>", json_str(position), SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Sets the rotation angle for data value labels.", fr = "Définit l'angle de rotation des étiquettes de valeurs de données.", param(name = "degrees", ty = "int", en = "Rotation angle in degrees.", fr = "Angle de rotation en degrés."))]
    #[sera_sig(degrees)]
    pub fn text_angle(&self, degrees: i32) -> Chart {
        let snippet = format!("<script>window.__sp_text__=Object.assign(window.__sp_text__||{{}},{{angle:{}}});{}</script></body>", degrees, SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Sets font family, size, and color for data value labels.", fr = "Définit la famille de polices, la taille et la couleur des étiquettes de valeurs de données.", param(name = "family", ty = "str | None", en = "Font family name.", fr = "Nom de la famille de polices."), param(name = "size", ty = "int | None", en = "Font size in pixels.", fr = "Taille de police en pixels."), param(name = "color", ty = "str | None", en = "Label text color.", fr = "Couleur du texte des étiquettes."))]
    #[sera_sig(family=None, size=None, color=None)]
    pub fn text_font(&self, family: Option<&str>, size: Option<i32>, color: Option<&str>) -> Chart {
        let mut opts = String::from("window.__sp_text__=Object.assign(window.__sp_text__||{},{");
        if let Some(f) = family { opts.push_str(&format!("font_family:{},", json_str(f))); }
        if let Some(s) = size { opts.push_str(&format!("font_size:{},", s)); }
        if let Some(c) = color { opts.push_str(&format!("color:{},", json_str(c))); }
        opts.push_str("});");
        let snippet = format!("<script>{}{}</script></body>", opts, SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Enforces a minimum font size for data labels; hides or shows labels that don't fit.", fr = "Impose une taille de police minimale pour les étiquettes de données; masque ou affiche les étiquettes qui ne tiennent pas.", param(name = "min_size", ty = "int", en = "Minimum font size in pixels.", fr = "Taille de police minimale en pixels."), param(name = "mode", ty = "str", en = "Behaviour when label doesn't fit: 'hide' or 'show'.", fr = "Comportement quand l'étiquette ne tient pas: 'hide' ou 'show'."))]
    #[sera_sig(min_size=8, mode="hide")]
    pub fn uniform_text(&self, min_size: i32, mode: &str) -> Chart {
        let snippet = format!("<script>window.__sp_text__=Object.assign(window.__sp_text__||{{}},{{uniform_min:{},uniform_mode:{}}});{}</script></body>", min_size, json_str(mode), SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Applies a corner radius to all bar rectangles in the chart.", fr = "Applique un rayon de coin à tous les rectangles de barres du graphique.", param(name = "radius", ty = "str", en = "Radius in pixels as string or percentage (e.g. '8' or '50%').", fr = "Rayon en pixels sous forme de chaîne ou pourcentage (ex: '8' ou '50%')."))]
    #[sera_sig(radius)]
    pub fn corner_radius_bars(&self, radius: &str) -> Chart {
        let val = if radius.ends_with('%') { json_str(radius) } else { radius.parse::<f64>().map(|v| v.to_string()).unwrap_or_else(|_| json_str(radius)) };
        let snippet = format!("<script>window.__sp_bar_r__={};{}</script></body>", val, SP_BAR_RADIUS_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Removes inline event handlers to make the chart compatible with strict Content-Security-Policy environments.", fr = "Supprime les gestionnaires d'événements inline pour rendre le graphique compatible avec les environnements à politique de sécurité de contenu stricte.")]
    pub fn csp_safe(&self) -> Chart {
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
                        Some(j) => { blob.push_str(&after[..j]); blob.push_str(";\n"); rest = &after[j + 9..]; }
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

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Adds ARIA accessibility attributes (title and description) to the SVG element.", fr = "Ajoute des attributs d'accessibilité ARIA (titre et description) à l'élément SVG.", param(name = "title", ty = "str", en = "Accessible title for screen readers.", fr = "Titre accessible pour les lecteurs d'écran."), param(name = "desc", ty = "str", en = "Accessible description for screen readers.", fr = "Description accessible pour les lecteurs d'écran."))]
    #[sera_sig(title="", desc="")]
    pub fn a11y(&self, title: &str, desc: &str) -> Chart {
        let snippet = format!(
            "<svg role=\"img\" aria-label=\"{}\"><title>{}</title><desc>{}</desc>",
            title.replace('"', "&quot;"),
            title.replace('<', "&lt;"),
            desc.replace('<', "&lt;"),
        );
        self.propagate(self.html.replacen("<svg", &snippet, 1))
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Returns a textual diff between this chart's HTML and another chart's HTML.", fr = "Retourne un diff textuel entre le HTML de ce graphique et celui d'un autre graphique.", param(name = "other", ty = "Chart", en = "The other Chart instance to compare against.", fr = "L'autre instance Chart à comparer."))]
    pub fn diff(&self, other: &Chart) -> String {
        crate::bindings::commands::charts::chart_diff(
            &serde_json::json!({"a": self.html, "b": other.html}).to_string()
        )
    }

    #[sera_doc(category = "chart_method", file = "charts/chart.md", en = "Downsamples line chart data using the LTTB algorithm to reduce visual clutter.", fr = "Réduit les données du graphique en courbes via l'algorithme LTTB pour diminuer l'encombrement visuel.", param(name = "n", ty = "int", en = "Target number of data points after downsampling.", fr = "Nombre cible de points de données après réduction."), param(name = "method", ty = "str", en = "Downsampling method. Currently only 'lttb' is supported.", fr = "Méthode de réduction. Seul 'lttb' est actuellement supporté."))]
    #[sera_sig(n=2000, method="lttb")]
    pub fn downsample(&self, n: usize, method: &str) -> Chart {
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

#[cfg(feature = "ffi")]
mod chart_ffi {
    use super::*;
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;

    #[no_mangle]
    pub extern "C" fn sera_chart_html(chart: *const Chart) -> *mut c_char {
        if chart.is_null() { return std::ptr::null_mut(); }
        let h = &unsafe { &*chart }.html;
        CString::new(h.as_str()).map(|s| s.into_raw()).unwrap_or(std::ptr::null_mut())
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_free_string(s: *mut c_char) {
        if !s.is_null() { unsafe { drop(CString::from_raw(s)); } }
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_free(chart: *mut Chart) {
        if !chart.is_null() { unsafe { drop(Box::from_raw(chart)); } }
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_save(chart: *const Chart, path: *const c_char) -> i32 {
        if chart.is_null() || path.is_null() { return -1; }
        let h = &unsafe { &*chart }.html;
        let p = match unsafe { CStr::from_ptr(path) }.to_str() { Ok(s) => s, Err(_) => return -1 };
        match std::fs::write(p, h.as_str()) { Ok(()) => 0, Err(_) => -1 }
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_to_svg(chart: *const Chart) -> *mut c_char {
        if chart.is_null() { return std::ptr::null_mut(); }
        let h = &unsafe { &*chart }.html;
        let result = h.find("<svg").and_then(|start| h.rfind("</svg>").map(|end| h[start..end + 6].to_string()));
        result.and_then(|s| CString::new(s).ok()).map(|s| s.into_raw()).unwrap_or(std::ptr::null_mut())
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_inject_css(chart: *const Chart, css: *const c_char) -> *mut Chart {
        if chart.is_null() || css.is_null() { return std::ptr::null_mut(); }
        let c = unsafe { &*chart };
        let s = match unsafe { CStr::from_ptr(css) }.to_str() { Ok(s) => s, Err(_) => return std::ptr::null_mut() };
        Box::into_raw(Box::new(c.inject_css(s)))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_inject_js(chart: *const Chart, js: *const c_char) -> *mut Chart {
        if chart.is_null() || js.is_null() { return std::ptr::null_mut(); }
        let c = unsafe { &*chart };
        let s = match unsafe { CStr::from_ptr(js) }.to_str() { Ok(s) => s, Err(_) => return std::ptr::null_mut() };
        Box::into_raw(Box::new(c.inject_js(s)))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_set_bg(chart: *const Chart, color: *const c_char) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        let c = unsafe { &*chart };
        let col = if color.is_null() { None } else { unsafe { CStr::from_ptr(color) }.to_str().ok() };
        Box::into_raw(Box::new(c.set_bg(col)))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_no_x_axis(chart: *const Chart) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        Box::into_raw(Box::new(unsafe { &*chart }.no_x_axis()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_no_y_axis(chart: *const Chart) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        Box::into_raw(Box::new(unsafe { &*chart }.no_y_axis()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_no_axes(chart: *const Chart) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        Box::into_raw(Box::new(unsafe { &*chart }.no_axes()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_show_grid(chart: *const Chart) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        Box::into_raw(Box::new(unsafe { &*chart }.show_grid()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_hide_grid(chart: *const Chart) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        Box::into_raw(Box::new(unsafe { &*chart }.hide_grid()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_responsive(chart: *const Chart) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        Box::into_raw(Box::new(unsafe { &*chart }.responsive()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_flip(chart: *const Chart) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        Box::into_raw(Box::new(unsafe { &*chart }.flip()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_crosshair(chart: *const Chart) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        Box::into_raw(Box::new(unsafe { &*chart }.crosshair()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_zoom(chart: *const Chart) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        Box::into_raw(Box::new(unsafe { &*chart }.zoom()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_no_legend(chart: *const Chart) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        Box::into_raw(Box::new(unsafe { &*chart }.no_legend()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_no_title(chart: *const Chart) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        Box::into_raw(Box::new(unsafe { &*chart }.no_title()))
    }

    #[no_mangle]
    pub extern "C" fn sera_chart_export_button(chart: *const Chart) -> *mut Chart {
        if chart.is_null() { return std::ptr::null_mut(); }
        Box::into_raw(Box::new(unsafe { &*chart }.export_button()))
    }
}


#[sera_doc(category = "config", file = "config/global.md", en = "Sets a global background color applied to all charts created after this call.", fr = "DÃ©finit une couleur d'arriÃ¨re-plan globale appliquÃ©e Ã  tous les graphiques crÃ©Ã©s aprÃ¨s cet appel.", param(name = "color", ty = "str", en = "CSS color string (hex, rgb, named). Empty string removes it.", fr = "Couleur CSS (hex, rgb, nommÃ©e). ChaÃ®ne vide pour la supprimer."))]
#[sera_bind]
pub fn set_global_background(color: &str) {
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() {
        *bg = Some(color.to_string());
    }
}

#[sera_doc(category = "config", file = "config/global.md", en = "Clears the global background color so charts use their default background.", fr = "Efface la couleur d'arriÃ¨re-plan globale afin que les graphiques utilisent leur arriÃ¨re-plan par dÃ©faut.")]
#[sera_bind]
pub fn reset_global_background() {
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() {
        *bg = None;
    }
}

#[sera_doc(category = "config", file = "config/global.md", en = "Enables or disables automatic display of charts in Jupyter notebooks upon creation.", fr = "Active ou dÃ©sactive l'affichage automatique des graphiques dans les notebooks Jupyter Ã  la crÃ©ation.", param(name = "enabled", ty = "bool", en = "True to auto-display charts in Jupyter; False to suppress auto-display.", fr = "True pour afficher automatiquement les graphiques dans Jupyter; False pour supprimer l'affichage automatique."))]
#[sera_bind]
pub fn set_auto_display(enabled: bool) {
    AUTO_DISPLAY.store(enabled, std::sync::atomic::Ordering::Relaxed);
}

struct ThemePreset {
    bg: Option<&'static str>,
    palette: &'static [u32],
    gridlines: bool,
}

const THEME_DARK: ThemePreset = ThemePreset {
    bg: Some("#0f172a"),
    palette: &[0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA, 0x22D3EE, 0xF472B6, 0xA3E635, 0xF87171, 0x2DD4BF],
    gridlines: true,
};

const THEME_LIGHT: ThemePreset = ThemePreset {
    bg: None,
    palette: &[0x6366F1, 0xF43F5E, 0x10B981, 0xF59E0B, 0x8B5CF6, 0x06B6D4, 0xEC4899, 0x84CC16, 0xEF4444, 0x14B8A6],
    gridlines: false,
};

const THEME_SCIENTIFIC: ThemePreset = ThemePreset {
    bg: Some("#fafafa"),
    palette: &[0x1F77B4, 0xFF7F0E, 0x2CA02C, 0xD62728, 0x9467BD, 0x8C564B, 0xE377C2, 0x7F7F7F, 0xBCBD22, 0x17BECF],
    gridlines: true,
};

const THEME_APPLE: ThemePreset = ThemePreset {
    bg: Some("#000000"),
    palette: &[0x0A84FF, 0x30D158, 0xFF453A, 0xFFD60A, 0xBF5AF2, 0x64D2FF, 0xFF9F0A, 0xFF375F, 0xAC8E68, 0x63E6E2],
    gridlines: false,
};

const THEME_NOTION: ThemePreset = ThemePreset {
    bg: Some("#191919"),
    palette: &[0x529CCA, 0xD08B65, 0x6C9B7D, 0xCB7C7A, 0x9A6DD7, 0x868686, 0xCCAA55, 0x75B5AA, 0xD477A8, 0x507AA6],
    gridlines: false,
};

const THEME_MINIMAL: ThemePreset = ThemePreset {
    bg: None,
    palette: &[0x374151, 0x6B7280, 0x9CA3AF, 0xD1D5DB, 0x111827, 0x4B5563, 0x1F2937, 0xE5E7EB, 0x030712, 0x6B7280],
    gridlines: false,
};

const THEME_NEON: ThemePreset = ThemePreset {
    bg: Some("#0a0a0a"),
    palette: &[0x00FF87, 0xFF006E, 0x00B4D8, 0xFFBE0B, 0xE500A4, 0x8338EC, 0x3A86FF, 0xFB5607, 0xFF006E, 0x06D6A0],
    gridlines: false,
};

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
#[sera_doc(category = "theme", file = "theme/theme.md", en = "Applies a named color theme to all subsequent chart renders.", fr = "Applique un thÃ¨me de couleurs nommÃ© Ã  tous les rendus de graphiques suivants.", param(name = "name", ty = "str", en = "Theme name (e.g. 'dark', 'light', 'ocean'). Use sp.themes() to list all.", fr = "Nom du thÃ¨me (ex: 'dark', 'light', 'ocean'). Utilisez sp.themes() pour lister tous les thÃ¨mes."))]
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

#[sera_doc(category = "theme", file = "theme/theme.md", en = "Resets the active theme back to the framework default.", fr = "RÃ©initialise le thÃ¨me actif vers le thÃ¨me par dÃ©faut du framework.")]
#[sera_bind]
pub fn reset_theme() {
    if let Ok(mut bg) = GLOBAL_BACKGROUND.lock() { *bg = None; }
    if let Ok(mut pal) = GLOBAL_PALETTE.lock() { *pal = None; }
    GLOBAL_GRIDLINES.store(false, std::sync::atomic::Ordering::Relaxed);
    if let Ok(mut tn) = GLOBAL_THEME_NAME.lock() { *tn = None; }
}

#[sera_doc(category = "theme", file = "theme/theme.md", en = "Returns a list of all available theme names.", fr = "Retourne la liste de tous les noms de thÃ¨mes disponibles.")]
#[sera_bind]
pub fn themes() -> Vec<String> {
    vec!["dark", "light", "scientific", "apple", "notion", "minimal", "neon"]
        .into_iter().map(String::from).collect()
}


#[sera_doc(category = "utility", file = "api/reference.md", en = "Returns a list of all available chart family names that have demo snippets.", fr = "Retourne la liste de tous les noms de familles de graphiques ayant des extraits de dÃ©monstration.")]
#[sera_bind]
pub fn demos() -> Vec<&'static str> {
    let mut out: Vec<&'static str> = crate::DEMO_REGISTRY.iter().map(|(f, _, _)| *f).collect();
    out.sort(); out.dedup(); out
}

#[sera_doc(category = "utility", file = "api/reference.md", en = "Returns a Python code snippet demonstrating how to create the specified chart type, or None if unknown.", fr = "Retourne un extrait de code Python illustrant comment crÃ©er le type de graphique spÃ©cifiÃ©, ou None si inconnu.", param(name = "chart", ty = "str", en = "Chart family name (e.g. 'bar', 'line', 'scatter').", fr = "Nom de la famille de graphique (ex: 'bar', 'line', 'scatter')."), param(name = "variant", ty = "str | None", en = "Variant name. Defaults to 'basic' if None.", fr = "Nom de la variante. Par dÃ©faut 'basic' si None."))]
#[sera_sig(chart, variant=None)]
#[sera_bind]
pub fn demo(chart: &str, variant: Option<&str>) -> Option<String> {
    crate::demo_snippet(chart, variant.unwrap_or("basic"))
}

#[sera_doc(category = "utility", file = "api/reference.md", en = "Returns the optional keyword arguments accepted by the specified chart type.", fr = "Retourne les arguments nommÃ©s optionnels acceptÃ©s par le type de graphique spÃ©cifiÃ©.", param(name = "chart", ty = "str | None", en = "Chart family name. Returns all charts if None.", fr = "Nom de la famille de graphique. Retourne tous les graphiques si None."), param(name = "variant", ty = "str | None", en = "Variant name for variant-specific params.", fr = "Nom de la variante pour les paramÃ¨tres spÃ©cifiques Ã  la variante."))]
#[sera_sig(chart=None, variant=None)]
#[sera_bind(serde)]
pub fn params(chart: Option<&str>, variant: Option<&str>) -> serde_json::Value {
    use serde_json::{Map, Value};
    if let (Some(c), Some(v)) = (chart, variant) {
        for (f, vv, k) in crate::DEMO_REGISTRY.iter() {
            if *f == c && *vv == v { return Value::String((*k).to_string()); }
        }
        return Value::Null;
    }
    if let Some(c) = chart {
        let mut m = Map::new();
        for (f, v, k) in crate::DEMO_REGISTRY.iter() {
            if *f == c { m.insert((*v).to_string(), Value::String((*k).to_string())); }
        }
        if m.is_empty() { return Value::Null; }
        return Value::Object(m);
    }
    let mut root = Map::new();
    for (f, v, k) in crate::DEMO_REGISTRY.iter() {
        let entry = root.entry((*f).to_string()).or_insert_with(|| Value::Object(Map::new()));
        if let Value::Object(inner) = entry {
            inner.insert((*v).to_string(), Value::String((*k).to_string()));
        }
    }
    Value::Object(root)
}

#[sera_doc(category = "utility", file = "api/reference.md", en = "Returns the required positional argument names for the specified chart type.", fr = "Retourne les noms d'arguments positionnels requis pour le type de graphique spÃ©cifiÃ©.", param(name = "chart", ty = "str | None", en = "Chart family name.", fr = "Nom de la famille de graphique."), param(name = "variant", ty = "str | None", en = "Variant name.", fr = "Nom de la variante."))]
#[sera_sig(chart=None, variant=None)]
#[sera_bind(serde)]
pub fn required_params(chart: Option<&str>, variant: Option<&str>) -> serde_json::Value {
    use serde_json::{Map, Value};
    if let (Some(c), Some(v)) = (chart, variant) {
        let list = crate::required_params_for(c, v).unwrap_or(&[]);
        return Value::Array(list.iter().map(|s| Value::String((*s).to_string())).collect());
    }
    if let Some(c) = chart {
        let mut m = Map::new();
        for (f, v, p) in crate::PARAMS_REGISTRY.iter() {
            if *f == c {
                m.insert((*v).to_string(), Value::Array(p.iter().map(|s| Value::String((*s).to_string())).collect()));
            }
        }
        return Value::Object(m);
    }
    let mut root = Map::new();
    for (f, v, p) in crate::PARAMS_REGISTRY.iter() {
        let entry = root.entry((*f).to_string()).or_insert_with(|| Value::Object(Map::new()));
        if let Value::Object(inner) = entry {
            inner.insert((*v).to_string(), Value::Array(p.iter().map(|s| Value::String((*s).to_string())).collect()));
        }
    }
    Value::Object(root)
}

#[sera_doc(category = "utility", file = "api/reference.md", en = "Returns a mapping of each chart family name to its list of available variant names and aliases.", fr = "Retourne une association de chaque nom de famille de graphique Ã  sa liste de variantes disponibles et alias.")]
#[sera_bind(serde)]
pub fn chart_variants() -> serde_json::Value {
    use serde_json::{Map, Value};
    use crate::plot::statistical::{
        BarVariant, LineVariant, HeatmapVariant, HistogramVariant,
        PieVariant, BoxplotVariant, ViolinVariant, KdeVariant, RidgelineVariant, RadarVariant,
        SlopeVariant, FunnelVariant, SunburstVariant, WaterfallVariant, TreemapVariant, CandlestickVariant,
        DumbbellVariant, BulletVariant, GaugeVariant, LollipopVariant, ParallelVariant, WordCloudVariant,
    };
    use crate::plot::statistical::scatter::ScatterVariant;
    use crate::plot::statistical::bubble::BubbleVariant;

    fn build(keys: &'static [(&'static str, &'static [&'static str])], default_key: &'static str) -> Value {
        let mut outer = Map::new();
        outer.insert("default".to_string(), Value::String(default_key.to_string()));
        let arr: Vec<Value> = keys.iter().map(|(k, aliases)| {
            let mut item = Map::new();
            item.insert("key".to_string(), Value::String((*k).to_string()));
            item.insert("aliases".to_string(), Value::Array(aliases.iter().map(|a| Value::String((*a).to_string())).collect()));
            Value::Object(item)
        }).collect();
        outer.insert("variants".to_string(), Value::Array(arr));
        Value::Object(outer)
    }

    let mut out = Map::new();
    out.insert("bar".to_string(), build(BarVariant::keys_and_aliases(), BarVariant::default_key()));
    out.insert("line".to_string(), build(LineVariant::keys_and_aliases(), LineVariant::default_key()));
    out.insert("scatter".to_string(), build(ScatterVariant::keys_and_aliases(), ScatterVariant::default_key()));
    out.insert("bubble".to_string(), build(BubbleVariant::keys_and_aliases(), BubbleVariant::default_key()));
    out.insert("histogram".to_string(), build(HistogramVariant::keys_and_aliases(), HistogramVariant::default_key()));
    out.insert("heatmap".to_string(), build(HeatmapVariant::keys_and_aliases(), HeatmapVariant::default_key()));
    out.insert("pie".to_string(), build(PieVariant::keys_and_aliases(), PieVariant::default_key()));
    out.insert("boxplot".to_string(), build(BoxplotVariant::keys_and_aliases(), BoxplotVariant::default_key()));
    out.insert("violin".to_string(), build(ViolinVariant::keys_and_aliases(), ViolinVariant::default_key()));
    out.insert("kde".to_string(), build(KdeVariant::keys_and_aliases(), KdeVariant::default_key()));
    out.insert("ridgeline".to_string(), build(RidgelineVariant::keys_and_aliases(), RidgelineVariant::default_key()));
    out.insert("radar".to_string(), build(RadarVariant::keys_and_aliases(), RadarVariant::default_key()));
    out.insert("slope".to_string(), build(SlopeVariant::keys_and_aliases(), SlopeVariant::default_key()));
    out.insert("funnel".to_string(), build(FunnelVariant::keys_and_aliases(), FunnelVariant::default_key()));
    out.insert("sunburst".to_string(), build(SunburstVariant::keys_and_aliases(), SunburstVariant::default_key()));
    out.insert("waterfall".to_string(), build(WaterfallVariant::keys_and_aliases(), WaterfallVariant::default_key()));
    out.insert("treemap".to_string(), build(TreemapVariant::keys_and_aliases(), TreemapVariant::default_key()));
    out.insert("candlestick".to_string(), build(CandlestickVariant::keys_and_aliases(), CandlestickVariant::default_key()));
    out.insert("dumbbell".to_string(), build(DumbbellVariant::keys_and_aliases(), DumbbellVariant::default_key()));
    out.insert("bullet".to_string(), build(BulletVariant::keys_and_aliases(), BulletVariant::default_key()));
    out.insert("gauge".to_string(), build(GaugeVariant::keys_and_aliases(), GaugeVariant::default_key()));
    out.insert("lollipop".to_string(), build(LollipopVariant::keys_and_aliases(), LollipopVariant::default_key()));
    out.insert("parallel".to_string(), build(ParallelVariant::keys_and_aliases(), ParallelVariant::default_key()));
    out.insert("wordcloud".to_string(), build(WordCloudVariant::keys_and_aliases(), WordCloudVariant::default_key()));
    Value::Object(out)
}

#[cfg(feature = "python")]
#[sera_doc(category = "config", file = "config/config.md", en = "Configures global chart rendering defaults: font, sizes, animation, tooltips, and more.", fr = "Configure les paramÃ¨tres de rendu globaux des graphiques: police, tailles, animation, infobulles, etc.", param(name = "font", ty = "str | None", en = "Global font family name.", fr = "Nom de la famille de polices globale."), param(name = "font_size", ty = "int | None", en = "Global font size in pixels.", fr = "Taille de police globale en pixels."), param(name = "title_size", ty = "int | None", en = "Title font size in pixels.", fr = "Taille de police du titre en pixels."), param(name = "border_radius", ty = "int | None", en = "Corner radius for chart containers in pixels.", fr = "Rayon de coin pour les conteneurs de graphiques en pixels."), param(name = "opacity", ty = "float | None", en = "Default data element opacity (0.0-1.0).", fr = "OpacitÃ© par dÃ©faut des Ã©lÃ©ments de donnÃ©es (0.0-1.0)."), param(name = "responsive", ty = "bool | None", en = "If True, SVG fills container width.", fr = "Si True, le SVG remplit la largeur du conteneur."), param(name = "animation", ty = "bool | None", en = "If True, enables entry animation for data elements.", fr = "Si True, active l'animation d'entrÃ©e pour les Ã©lÃ©ments de donnÃ©es."), param(name = "animation_duration", ty = "int | None", en = "Animation duration in milliseconds.", fr = "DurÃ©e de l'animation en millisecondes."), param(name = "crosshair", ty = "bool | None", en = "If True, adds a crosshair to all charts.", fr = "Si True, ajoute un rÃ©ticule Ã  tous les graphiques."), param(name = "zoom", ty = "bool | None", en = "If True, enables zoom on all charts.", fr = "Si True, active le zoom sur tous les graphiques."), param(name = "margin", ty = "int | None", en = "Global chart margin in pixels.", fr = "Marge globale des graphiques en pixels."), param(name = "export_button", ty = "bool | None", en = "If True, adds a download button to all charts.", fr = "Si True, ajoute un bouton de tÃ©lÃ©chargement Ã  tous les graphiques."), param(name = "background", ty = "str | None", en = "Global chart background color.", fr = "Couleur d'arriÃ¨re-plan globale des graphiques."), param(name = "gridlines", ty = "bool | None", en = "If True, shows grid lines on all charts.", fr = "Si True, affiche les lignes de grille sur tous les graphiques."), param(name = "text_auto", ty = "bool | str | None", en = "Auto data label format. True for default, string for custom format.", fr = "Format d'Ã©tiquette de donnÃ©es automatique. True pour dÃ©faut, chaÃ®ne pour format personnalisÃ©."))]
#[pyfunction]
#[pyo3(signature = (*, font=None, font_size=None, title_size=None, border_radius=None, opacity=None, responsive=None, animation=None, animation_duration=None, crosshair=None, zoom=None, tooltip=None, locale=None, thousands_sep=None, margin=None, export_button=None, palette=None, background=None, gridlines=None, text_auto=None, text_position=None, text_angle=None, text_font_size=None, text_font_color=None, uniform_text_min_size=None, uniform_text_mode=None, bar_corner_radius=None))]
pub fn config(font: Option<&str>, font_size: Option<i32>, title_size: Option<i32>, border_radius: Option<i32>, opacity: Option<f64>, responsive: Option<bool>, animation: Option<bool>, animation_duration: Option<i32>, crosshair: Option<bool>, zoom: Option<bool>, tooltip: Option<&str>, locale: Option<&str>, thousands_sep: Option<&str>, margin: Option<i32>, export_button: Option<bool>, palette: Option<Vec<u32>>, background: Option<&str>, gridlines: Option<bool>, text_auto: Option<&PyAny>, text_position: Option<&str>, text_angle: Option<i32>, text_font_size: Option<i32>, text_font_color: Option<&str>, uniform_text_min_size: Option<i32>, uniform_text_mode: Option<&str>, bar_corner_radius: Option<&PyAny>) {
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
    if let Some(v) = text_auto {
        let s = if let Ok(b) = v.extract::<bool>() { if b { String::from("") } else { return reset_text_auto(); } }
            else if let Ok(s) = v.extract::<String>() { s }
            else { return; };
        if let Ok(mut g) = GLOBAL_TEXT_AUTO.lock() { *g = Some(s); }
    }
    if let Some(v) = text_position { if let Ok(mut g) = GLOBAL_TEXT_POSITION.lock() { *g = Some(v.to_string()); } }
    if let Some(v) = text_angle { GLOBAL_TEXT_ANGLE.store(v, Relaxed); }
    if let Some(v) = text_font_size { GLOBAL_TEXT_FONT_SIZE.store(v, Relaxed); }
    if let Some(v) = text_font_color { if let Ok(mut g) = GLOBAL_TEXT_FONT_COLOR.lock() { *g = Some(v.to_string()); } }
    if let Some(v) = uniform_text_min_size { GLOBAL_UNIFORM_TEXT_MIN.store(v, Relaxed); }
    if let Some(v) = uniform_text_mode { if let Ok(mut g) = GLOBAL_UNIFORM_TEXT_MODE.lock() { *g = Some(v.to_string()); } }
    if let Some(v) = bar_corner_radius {
        let s = if let Ok(i) = v.extract::<i32>() { i.to_string() }
            else if let Ok(f) = v.extract::<f64>() { f.to_string() }
            else if let Ok(s) = v.extract::<String>() { s }
            else { return; };
        if let Ok(mut g) = GLOBAL_BAR_CORNER_RADIUS.lock() { *g = Some(s); }
    }
}

#[cfg(feature = "python")]
fn reset_text_auto() {
    if let Ok(mut g) = GLOBAL_TEXT_AUTO.lock() { *g = None; }
}

#[sera_doc(category = "config", file = "config/config.md", en = "Resets all global config settings applied via config() back to their defaults.", fr = "RÃ©initialise tous les paramÃ¨tres de configuration globale dÃ©finis via config() Ã  leurs valeurs par dÃ©faut.")]
#[sera_bind]
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
    if let Ok(mut g) = GLOBAL_TEXT_AUTO.lock() { *g = None; }
    if let Ok(mut g) = GLOBAL_TEXT_POSITION.lock() { *g = None; }
    GLOBAL_TEXT_ANGLE.store(i32::MIN, Relaxed);
    GLOBAL_TEXT_FONT_SIZE.store(0, Relaxed);
    if let Ok(mut g) = GLOBAL_TEXT_FONT_COLOR.lock() { *g = None; }
    GLOBAL_UNIFORM_TEXT_MIN.store(0, Relaxed);
    if let Ok(mut g) = GLOBAL_UNIFORM_TEXT_MODE.lock() { *g = None; }
    if let Ok(mut g) = GLOBAL_BAR_CORNER_RADIUS.lock() { *g = None; }
    if let Ok(mut g) = GLOBAL_HOVER_INFO.lock() { *g = None; }
    if let Ok(mut g) = GLOBAL_PATTERN_SHAPE.lock() { *g = None; }
}

#[sera_doc(category = "performance", file = "config/performance.md", en = "Returns hardware profile information: CPU thread count, parallelism threshold, and L2 cache chunk size.", fr = "Retourne les informations du profil matÃ©riel: nombre de threads CPU, seuil de parallÃ©lisme et taille de chunk du cache L2.")]
#[sera_bind(serde)]
pub fn hw() -> crate::core::hw_profile::HwProfile {
    *crate::core::hw_profile::hw()
}

#[cfg(feature = "python")]
#[pyclass(name = "DatasetStats", module = "seraplot")]
pub struct PyDatasetStats {
    #[pyo3(get)] pub min: f64,
    #[pyo3(get)] pub max: f64,
    #[pyo3(get)] pub mean: f64,
    #[pyo3(get)] pub std_dev: f64,
    #[pyo3(get)] pub sum: f64,
    #[pyo3(get)] pub count: usize,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyDatasetStats {
    fn __repr__(&self) -> String {
        format!("DatasetStats(min={:.4}, max={:.4}, mean={:.4}, std_dev={:.4}, count={})",
            self.min, self.max, self.mean, self.std_dev, self.count)
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
    #[sera_doc(category = "data", file = "api/dataset.md", en = "Creates a Dataset from a Python list of float values.", fr = "CrÃ©e un Dataset Ã  partir d'une liste Python de valeurs float.", param(name = "values", ty = "list[float]", en = "List of numeric values.", fr = "Liste de valeurs numÃ©riques."))]
    #[pyo3(signature = (values, labels=None))]
    fn from_list(values: Vec<f64>, labels: Option<Vec<String>>) -> Self {
        let mut ds = crate::data::Dataset::with_capacity("dataset", values.len());
        for (i, v) in values.iter().enumerate() {
            let lbl = labels.as_ref().and_then(|l| l.get(i)).map(|s| s.as_str()).unwrap_or("").to_string();
            ds.push(*v, lbl);
        }
        PyDataset { inner: ds }
    }

    #[sera_doc(category = "data", file = "api/dataset.md", en = "Computes descriptive statistics in parallel: min, max, mean, variance, std_dev, sum, count.", fr = "Calcule des statistiques descriptives en parallÃ¨le: min, max, mean, variance, std_dev, sum, count.")]
    fn par_stats(&self) -> PyDatasetStats {
        let s = self.inner.par_stats();
        PyDatasetStats { min: s.min, max: s.max, mean: s.mean, std_dev: s.std_dev, sum: s.sum, count: s.count }
    }

    #[sera_doc(category = "data", file = "api/dataset.md", en = "Splits the dataset into n equal-sized chunks and returns them as a list of Dataset objects.", fr = "Divise le dataset en n morceaux de taille Ã©gale et les retourne sous forme de liste d'objets Dataset.", param(name = "n", ty = "int", en = "Number of chunks to split into.", fr = "Nombre de morceaux en lesquels diviser."))]
    fn into_chunks(&self, n: usize) -> Vec<PyDataset> {
        let vals: Vec<f64> = self.inner.values().collect();
        let labels: Vec<String> = self.inner.labels().map(|s| s.to_string()).collect();
        if n == 0 || vals.is_empty() { return vec![]; }
        let chunk_size = (vals.len() + n - 1) / n;
        vals.chunks(chunk_size).enumerate().map(|(ci, chunk)| {
            let lbl_slice = &labels[ci * chunk_size..ci * chunk_size + chunk.len()];
            let mut ds = crate::data::Dataset::with_capacity(&format!("chunk_{ci}"), chunk.len());
            for (v, l) in chunk.iter().zip(lbl_slice.iter()) { ds.push(*v, l.as_str()); }
            PyDataset { inner: ds }
        }).collect()
    }

    fn values(&self) -> Vec<f64> {
        self.inner.values().collect()
    }

    fn labels(&self) -> Vec<String> {
        self.inner.labels().map(|s| s.to_string()).collect()
    }

    fn __len__(&self) -> usize { self.inner.len() }

    fn __repr__(&self) -> String {
        format!("Dataset(name={:?}, len={})", self.inner.name, self.inner.len())
    }
}

#[sera_doc(category = "performance", file = "config/performance.md", en = "Enables or disables the adaptive retry system. When enabled, operations that panic will auto-degrade chunk sizes and retry.", fr = "Active ou dÃ©sactive le systÃ¨me de rÃ©essai adaptatif. Quand activÃ©, les opÃ©rations qui paniquent rÃ©duisent automatiquement les tailles de chunks et rÃ©essaient.", param(name = "on", ty = "bool", en = "True to enable adaptive retry (default). False to disable.", fr = "True pour activer le rÃ©essai adaptatif (dÃ©faut). False pour dÃ©sactiver."))]
#[sera_bind]
pub fn set_adaptive_retry(on: bool) {
    crate::core::adaptive_exec::set_adaptive_retry(on);
}

#[sera_doc(category = "performance", file = "config/performance.md", en = "Resets the degradation level back to 0 (full-speed operation).", fr = "RÃ©initialise le niveau de dÃ©gradation Ã  0 (opÃ©ration Ã  pleine vitesse).")]
#[sera_bind]
pub fn reset_perf_state() {
    crate::core::adaptive_exec::reset_perf_state();
}

#[sera_doc(category = "performance", file = "config/performance.md", en = "Returns the current degradation level (0 = full speed, 4 = maximum degradation).", fr = "Retourne le niveau de dÃ©gradation actuel (0 = pleine vitesse, 4 = dÃ©gradation maximale).")]
#[sera_bind]
pub fn adaptive_degrade_level() -> usize {
    crate::core::adaptive_exec::degrade_level()
}

#[cfg(feature = "python")]
pub(crate) fn extract_f64_vec(_py: Python<'_>, obj: &PyAny) -> PyResult<Vec<f64>> {
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
pub(crate) fn extract_str_vec(_py: Python<'_>, obj: &PyAny) -> PyResult<Vec<String>> {
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
#[sera_doc(category = "layout", file = "charts/layout.md", en = "Arranges multiple Chart objects in a responsive grid layout and returns a single Chart.", fr = "Organise plusieurs objets Chart dans une mise en page grille responsive et retourne un seul Chart.", param(name = "charts", ty = "list[Chart]", en = "List of Chart objects to arrange in the grid.", fr = "Liste d'objets Chart Ã  organiser dans la grille."), param(name = "cols", ty = "int", en = "Number of columns in the grid. Default: 3.", fr = "Nombre de colonnes dans la grille. DÃ©faut: 3."), param(name = "gap", ty = "int", en = "Gap between cells in pixels. Default: 16.", fr = "Espacement entre les cellules en pixels. DÃ©faut: 16."), param(name = "bg_color", ty = "str", en = "Background color for the grid container. Default: '#0a0f1c'.", fr = "Couleur d'arriÃ¨re-plan du conteneur de grille. DÃ©faut: '#0a0f1c'."), param(name = "title", ty = "str", en = "Optional title displayed above the grid.", fr = "Titre optionnel affichÃ© au-dessus de la grille."), param(name = "cell_height", ty = "int | None", en = "Fixed height for each cell in pixels. Auto-detected if None.", fr = "Hauteur fixe de chaque cellule en pixels. DÃ©tection automatique si None."))]
#[pyfunction]
#[pyo3(signature = (charts, cols=3, gap=16, bg_color="#0a0f1c", title="", cell_height=None))]
fn build_grid(
    py: Python<'_>,
    charts: Vec<PyObject>,
    cols: usize,
    gap: i32,
    bg_color: &str,
    title: &str,
    cell_height: Option<i32>,
) -> PyResult<Chart> {
    let cols = if cols < 1 { 1 } else { cols };
    let mut cells = String::new();
    for obj in &charts {
        let chart_ref: PyRef<Chart> = obj.extract(py)?;
        let h = if let Some(ch) = cell_height {
            ch as u32
        } else {
            let mut found = 480u32;
            let s = &chart_ref.html;
            let mut start = 0usize;
            loop {
                match s[start..].find("height=\"") {
                    None => break,
                    Some(rel) => {
                        let abs = start + rel + 8;
                        if let Some(end) = s[abs..].find('"') {
                            if let Ok(v) = s[abs..abs + end].parse::<u32>() {
                                if v >= 150 && v <= 1600 {
                                    found = v;
                                    break;
                                }
                            }
                        }
                        start += rel + 1;
                    }
                }
            }
            found
        };
        let esc = chart_ref.html.replace('&', "&amp;").replace('"', "&quot;");
        cells.push_str(&format!(
            r#"<div class="sp-gc"><iframe srcdoc="{esc}" style="width:100%;height:{h}px;border:none;display:block" frameborder="0" loading="lazy"></iframe></div>"#
        ));
    }
    let title_html = if title.is_empty() {
        String::new()
    } else {
        let t = title.replace('<', "&lt;").replace('>', "&gt;");
        format!(r#"<h1 class="sp-gtitle">{t}</h1>"#)
    };
    let html = format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><style>*{{box-sizing:border-box}}html,body{{margin:0;padding:0;background:{bg_color};font-family:system-ui,-apple-system,Arial,sans-serif}}body{{padding:16px}}.sp-gtitle{{color:#e2e8f0;font-size:22px;font-weight:700;text-align:center;margin:0 0 16px;letter-spacing:-.01em;border:none}}.sp-grid{{display:grid;grid-template-columns:repeat({cols},1fr);gap:{gap}px}}.sp-gc{{border-radius:10px;overflow:hidden;background:#0d1117;box-shadow:0 4px 20px rgba(0,0,0,.5)}}</style></head><body>{title_html}<div class="sp-grid">{cells}</div></body></html>"#
    );
    Ok(Chart::new(html))
}

#[cfg(feature = "python")]
#[sera_doc(category = "layout", file = "charts/layout.md", en = "Alias for build_grid(). Arranges multiple Chart objects in a responsive grid.", fr = "Alias de build_grid(). Organise plusieurs objets Chart dans une grille responsive.", param(name = "charts", ty = "list[Chart]", en = "List of Chart objects to arrange.", fr = "Liste d'objets Chart Ã  organiser."), param(name = "cols", ty = "int", en = "Number of grid columns. Default: 3.", fr = "Nombre de colonnes de la grille. DÃ©faut: 3."), param(name = "gap", ty = "int", en = "Gap in pixels. Default: 16.", fr = "Espacement en pixels. DÃ©faut: 16."), param(name = "bg_color", ty = "str", en = "Grid background color. Default: '#0a0f1c'.", fr = "Couleur d'arriÃ¨re-plan de la grille. DÃ©faut: '#0a0f1c'."), param(name = "title", ty = "str", en = "Optional grid title.", fr = "Titre optionnel de la grille."), param(name = "cell_height", ty = "int | None", en = "Fixed cell height in pixels or None for auto.", fr = "Hauteur fixe des cellules en pixels ou None pour automatique."))]
#[pyfunction]
#[pyo3(signature = (charts, cols=3, gap=16, bg_color="#0a0f1c", title="", cell_height=None))]
fn grid(
    py: Python<'_>,
    charts: Vec<PyObject>,
    cols: usize,
    gap: i32,
    bg_color: &str,
    title: &str,
    cell_height: Option<i32>,
) -> PyResult<Chart> {
    build_grid(py, charts, cols, gap, bg_color, title, cell_height)
}

#[cfg(feature = "python")]
#[sera_doc(category = "layout", file = "charts/layout.md", en = "Builds a live system monitor dashboard showing CPU, memory, disk usage, and system info.", fr = "Construit un tableau de bord de surveillance systÃ¨me en direct montrant l'utilisation du CPU, de la mÃ©moire, des disques et les informations systÃ¨me.", param(name = "bg_color", ty = "str", en = "Dashboard background color. Default: '#0a0f1c'.", fr = "Couleur d'arriÃ¨re-plan du tableau de bord. DÃ©faut: '#0a0f1c'."), param(name = "update_interval_ms", ty = "int", en = "Simulated refresh interval in milliseconds. Default: 2000.", fr = "Intervalle de rafraÃ®chissement simulÃ© en millisecondes. DÃ©faut: 2000."))]
#[pyfunction]
#[pyo3(signature = (bg_color="#0a0f1c", update_interval_ms=2000u32))]
fn build_sysmon(bg_color: &str, update_interval_ms: u32) -> Chart {
    use sysinfo::{System, Disks};
    let mut sys = System::new_all();
    sys.refresh_all();
    std::thread::sleep(std::time::Duration::from_millis(120));
    sys.refresh_all();

    let cpu_pct = sys.global_cpu_info().cpu_usage();
    let cpu_count = sys.cpus().len();
    let core_usage: Vec<f32> = sys.cpus().iter().map(|c| c.cpu_usage()).collect();
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let mem_pct = if total_mem > 0 {
        used_mem as f64 / total_mem as f64 * 100.0
    } else {
        0.0
    };
    let procs = sys.processes().len();
    let uptime_s = System::uptime();
    let hostname = System::host_name().unwrap_or_else(|| String::from("localhost"));
    let os_name = System::long_os_version()
        .or_else(|| System::os_version())
        .unwrap_or_else(|| String::from("Unknown OS"));

    let disks = Disks::new_with_refreshed_list();
    let disk_parts: Vec<String> = disks
        .list()
        .iter()
        .take(6)
        .filter_map(|d| {
            let total = d.total_space();
            if total == 0 {
                return None;
            }
            let name = d.name().to_string_lossy().to_string();
            let name = if name.is_empty() {
                String::from("Disk")
            } else {
                name
            };
            let avail = d.available_space();
            let used = total.saturating_sub(avail);
            let pct = (used as f64 / total as f64 * 100.0) as u32;
            let name_esc = name.replace('"', "\\\"");
            Some(format!(
                "{{\"name\":\"{name_esc}\",\"used\":{used},\"total\":{total},\"pct\":{pct}}}"
            ))
        })
        .collect();

    let core_parts: Vec<String> = core_usage
        .iter()
        .take(8)
        .map(|v| format!("{:.1}", v))
        .collect();

    let data_json = format!(
        "{{\"cpu_pct\":{cpu_pct:.1},\"cpu_count\":{cpu_count},\"cpu_cores\":[{cores}],\"mem_pct\":{mem_pct:.1},\"used_mem\":{used_mem},\"total_mem\":{total_mem},\"disks\":[{disks}],\"processes\":{procs},\"uptime_s\":{uptime_s}}}",
        cores = core_parts.join(","),
        disks = disk_parts.join(","),
    );

    let host_display: String = hostname.chars().take(28).collect();
    let os_display: String = os_name.chars().take(44).collect();

    let css = format!(
        "*{{box-sizing:border-box;margin:0;padding:0}}body{{background:{bg_color};color:#e2e8f0;font-family:system-ui,-apple-system,Arial,sans-serif;padding:20px;min-height:100vh}}.sm-header{{text-align:center;margin-bottom:24px}}.sm-title{{font-size:22px;font-weight:800;color:#f1f5f9;letter-spacing:-.02em}}.sm-sub{{font-size:13px;color:#475569;margin-top:5px}}.sm-grid{{display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:16px;margin-bottom:16px}}.sm-card{{background:#0d1117;border:1px solid #1e293b;border-radius:12px;padding:20px}}.sm-card-title{{font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.08em;color:#475569;margin-bottom:14px}}.sm-value{{font-size:32px;font-weight:800;line-height:1}}.sm-unit{{font-size:13px;font-weight:400;color:#94a3b8;margin-left:4px}}.sm-bar-bg{{width:100%;height:8px;background:#1e293b;border-radius:4px;margin-top:12px;overflow:hidden}}.sm-bar-fill{{height:100%;border-radius:4px;transition:width .6s ease}}.sm-disk-row{{display:flex;align-items:center;gap:10px;margin-bottom:8px;font-size:12px}}.sm-disk-name{{color:#94a3b8;width:80px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;flex-shrink:0}}.sm-disk-bar-bg{{flex:1;height:6px;background:#1e293b;border-radius:3px;overflow:hidden}}.sm-disk-bar{{height:100%;border-radius:3px;transition:width .6s ease}}.sm-disk-pct{{color:#e2e8f0;width:36px;text-align:right;flex-shrink:0}}.sm-stat-row{{display:flex;gap:12px;margin-top:12px}}.sm-stat{{flex:1;background:#0a0f1c;border-radius:8px;padding:10px;text-align:center}}.sm-stat-val{{font-size:18px;font-weight:700;color:#f1f5f9}}.sm-stat-lbl{{font-size:10px;color:#475569;text-transform:uppercase;letter-spacing:.06em;margin-top:2px}}.sm-gauge-wrap{{display:flex;align-items:center;justify-content:center}}.sm-ts{{font-size:10px;color:#334155;text-align:center;margin-top:12px}}"
    );

    let js_tpl = r##"const DATA=__DATA__;
let T=JSON.parse(JSON.stringify(DATA));
function clr(p){return p<50?'#22c55e':p<75?'#f59e0b':'#ef4444'}
function fmt(b){var u=['B','KB','MB','GB','TB'],i=0,v=b;while(v>=1024&&i<4){v/=1024;i++}return v.toFixed(i>0?1:0)+' '+u[i]}
function gauge(p,c,sz){var r=sz/2-10,cx=sz/2,cy=sz/2,circ=2*Math.PI*r,d=p/100*circ*.75;return'<svg width="'+sz+'" height="'+sz+'" viewBox="0 0 '+sz+' '+sz+'"><circle cx="'+cx+'" cy="'+cy+'" r="'+r+'" fill="none" stroke="#1e293b" stroke-width="14" stroke-dasharray="'+(circ*.75)+' '+(circ*.25)+'" stroke-linecap="round" transform="rotate(-225 '+cx+' '+cy+')"/><circle cx="'+cx+'" cy="'+cy+'" r="'+r+'" fill="none" stroke="'+c+'" stroke-width="14" stroke-dasharray="'+d+' '+(circ-d)+'" stroke-linecap="round" transform="rotate(-225 '+cx+' '+cy+')" style="transition:stroke-dasharray .6s ease"/><text x="'+cx+'" y="'+(cy+7)+'" text-anchor="middle" fill="#f1f5f9" font-size="22" font-weight="800" font-family="system-ui">'+Math.round(p)+'%</text></svg>'}
function render(d){
  var g=document.getElementById('sm-grid'),h='',cc=clr(d.cpu_pct);
  h+='<div class="sm-card"><div class="sm-card-title">CPU</div><div class="sm-gauge-wrap">'+gauge(d.cpu_pct,cc,150)+'</div><div class="sm-stat-row">';
  d.cpu_cores.slice(0,4).forEach(function(v,i){h+='<div class="sm-stat"><div class="sm-stat-val" style="color:'+clr(v)+'">'+Math.round(v)+'%</div><div class="sm-stat-lbl">Core '+(i+1)+'</div></div>'});
  h+='</div></div>';
  var mc=clr(d.mem_pct);
  h+='<div class="sm-card"><div class="sm-card-title">Memory</div><div class="sm-value" style="color:'+mc+'">'+Math.round(d.mem_pct)+'<span class="sm-unit">%</span></div><div class="sm-bar-bg"><div class="sm-bar-fill" style="width:'+d.mem_pct+'%;background:'+mc+'"></div></div><div class="sm-stat-row"><div class="sm-stat"><div class="sm-stat-val">'+fmt(d.used_mem)+'</div><div class="sm-stat-lbl">Used</div></div><div class="sm-stat"><div class="sm-stat-val">'+fmt(d.total_mem)+'</div><div class="sm-stat-lbl">Total</div></div></div></div>';
  h+='<div class="sm-card"><div class="sm-card-title">Disks</div>';
  d.disks.forEach(function(dk){var c=clr(dk.pct);h+='<div class="sm-disk-row"><div class="sm-disk-name">'+dk.name+'</div><div class="sm-disk-bar-bg"><div class="sm-disk-bar" style="width:'+dk.pct+'%;background:'+c+'"></div></div><div class="sm-disk-pct" style="color:'+c+'">'+dk.pct+'%</div></div>'});
  h+='</div>';
  var up=d.uptime_s?Math.floor(d.uptime_s/3600)+'h '+Math.floor(d.uptime_s%3600/60)+'m':'N/A';
  h+='<div class="sm-card"><div class="sm-card-title">System</div><div class="sm-stat-row"><div class="sm-stat"><div class="sm-stat-val">'+d.cpu_count+'</div><div class="sm-stat-lbl">Cores</div></div><div class="sm-stat"><div class="sm-stat-val">'+d.processes+'</div><div class="sm-stat-lbl">Processes</div></div></div><div class="sm-stat-row" style="margin-top:8px"><div class="sm-stat"><div class="sm-stat-val">'+fmt(d.total_mem)+'</div><div class="sm-stat-lbl">Total RAM</div></div><div class="sm-stat"><div class="sm-stat-val">'+up+'</div><div class="sm-stat-lbl">Uptime</div></div></div></div>';
  g.innerHTML=h}
render(T);
setInterval(function(){T=Object.assign({},T,{cpu_pct:Math.min(100,Math.max(0,T.cpu_pct+(Math.random()-.5)*4)),mem_pct:Math.min(100,Math.max(0,T.mem_pct+(Math.random()-.5)*.8)),cpu_cores:T.cpu_cores.map(function(v){return Math.min(100,Math.max(0,v+(Math.random()-.5)*6))})});render(T);document.getElementById('sm-ts').textContent='Updated: '+new Date().toLocaleTimeString()},__INTERVAL__);"##;

    let js = js_tpl
        .replace("__DATA__", &data_json)
        .replace("__INTERVAL__", &update_interval_ms.to_string());

    let html = format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SeraPlot Sysmon</title><style>{css}</style></head><body><div class="sm-header"><div class="sm-title">System Monitor</div><div class="sm-sub">{host_display} &middot; {os_display} &middot; {cpu_count} cores</div></div><div class="sm-grid" id="sm-grid"></div><div class="sm-ts" id="sm-ts">Snapshot taken</div><script>{js}</script></body></html>"#
    );

    Chart::new(html)
}

#[cfg(feature = "python")]
#[sera_doc(category = "layout", file = "charts/layout.md", en = "Alias for build_sysmon(). Returns a live system monitor dashboard as a Chart.", fr = "Alias de build_sysmon(). Retourne un tableau de bord de surveillance systÃ¨me en direct sous forme de Chart.", param(name = "bg_color", ty = "str", en = "Background color. Default: '#0a0f1c'.", fr = "Couleur d'arriÃ¨re-plan. DÃ©faut: '#0a0f1c'."), param(name = "update_interval_ms", ty = "int", en = "Refresh interval in milliseconds. Default: 2000.", fr = "Intervalle de rafraÃ®chissement en millisecondes. DÃ©faut: 2000."))]
#[pyfunction]
#[pyo3(signature = (bg_color="#0a0f1c", update_interval_ms=2000u32))]
fn sysmon(bg_color: &str, update_interval_ms: u32) -> Chart {
    build_sysmon(bg_color, update_interval_ms)
}

#[sera_doc(category = "telemetry", file = "about/telemetry.md", en = "Enables or disables usage telemetry collection. Disabled by default.", fr = "Active ou dÃ©sactive la collecte de tÃ©lÃ©mÃ©trie d'utilisation. DÃ©sactivÃ© par dÃ©faut.", param(name = "enabled", ty = "bool", en = "True to enable telemetry, False to disable.", fr = "True pour activer la tÃ©lÃ©mÃ©trie, False pour dÃ©sactiver."))]
#[sera_bind]
fn telemetry_consent(enabled: bool) {
    crate::telemetry::set_consent(enabled);
}

#[sera_doc(category = "telemetry", file = "about/telemetry.md", en = "Returns the filesystem path where telemetry data is stored.", fr = "Retourne le chemin du systÃ¨me de fichiers oÃ¹ les donnÃ©es de tÃ©lÃ©mÃ©trie sont stockÃ©es.")]
#[sera_bind]
fn telemetry_path() -> String {
    crate::telemetry::telemetry_file_path()
}

#[sera_doc(category = "telemetry", file = "about/telemetry.md", en = "Returns a JSON string with aggregated usage metrics summary.", fr = "Retourne une chaÃ®ne JSON avec un rÃ©sumÃ© des mÃ©triques d'utilisation agrÃ©gÃ©es.")]
#[sera_bind]
fn get_metrics() -> String {
    serde_json::to_string(&crate::telemetry::get_metrics_summary()).unwrap_or_default()
}

#[cfg(feature = "python")]
#[sera_doc(category = "telemetry", file = "about/telemetry.md", en = "Uploads pending telemetry events to the specified endpoint with an auth token.", fr = "TÃ©lÃ©verse les Ã©vÃ©nements de tÃ©lÃ©mÃ©trie en attente vers l'endpoint spÃ©cifiÃ© avec un jeton d'authentification.", param(name = "endpoint", ty = "str", en = "HTTPS URL of the telemetry collection endpoint.", fr = "URL HTTPS de l'endpoint de collecte de tÃ©lÃ©mÃ©trie."), param(name = "token", ty = "str", en = "Authentication token for the endpoint.", fr = "Jeton d'authentification pour l'endpoint."))]
#[pyfunction]
#[pyo3(signature = (endpoint, token))]
fn push_telemetry(py: Python<'_>, endpoint: &str, token: &str) -> PyResult<usize> {
    let events = crate::telemetry::read_pending();
    let count = events.len();
    if count == 0 {
        return Ok(0);
    }
    
    let summary = crate::telemetry::get_metrics_summary();
    let system = summary.get("system").cloned().unwrap_or_else(|| serde_json::json!({}));
    
    let payload = serde_json::json!({
        "secret": token,
        "events": events,
        "system": system
    });
    let body = serde_json::to_string(&payload)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let result = py.allow_threads(|| -> Result<u16, String> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| e.to_string())?;
        rt.block_on(async {
            let client = reqwest::Client::new();
            let resp = client
                .post(endpoint)
                .header("Content-Type", "application/json")
                .header("User-Agent", format!("seraplot/{VERSION}"))
                .body(body)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok::<u16, String>(resp.status().as_u16())
        })
    });
    match result {
        Ok(status) if status < 300 => {
            crate::telemetry::clear_pending();
            Ok(count)
        }
        Ok(status) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "HTTP {status}"
        ))),
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
    }
}

#[sera_doc(category = "utility", file = "api/reference.md", en = "Returns all documented functions with their signatures, parameters, and bilingual descriptions.", fr = "Retourne toutes les fonctions documentÃ©es avec leurs signatures, paramÃ¨tres et descriptions bilingues.")]
#[sera_bind(serde)]
pub fn docs() -> Vec<&'static crate::doc_registry::FnDoc> {
    crate::doc_registry::all_docs()
}

#[sera_doc(category = "utility", file = "api/reference.md", en = "Returns documentation for a single function by name, or null if not found.", fr = "Retourne la documentation d'une seule fonction par nom, ou null si introuvable.", param(name = "name", ty = "str", en = "Function name.", fr = "Nom de la fonction."))]
#[sera_bind(serde)]
pub fn doc(name: &str) -> Option<&'static crate::doc_registry::FnDoc> {
    crate::doc_registry::doc_for(name)
}

#[sera_doc(category = "utility", file = "api/reference.md", en = "Returns all registered ML and plot model structs with their fields and categories.", fr = "Retourne tous les modèles ML et plot enregistrés avec leurs champs et catégories.")]
#[sera_bind(serde)]
pub fn models() -> Vec<&'static crate::model_registry::ModelInfo> {
    crate::model_registry::all_models()
}

#[sera_doc(category = "utility", file = "api/reference.md", en = "Returns all models for a given category.", fr = "Retourne tous les modèles pour une catégorie donnée.", param(name = "category", ty = "str", en = "Category name.", fr = "Nom de la catégorie."))]
#[sera_bind(serde)]
pub fn models_for_category(category: &str) -> Vec<&'static crate::model_registry::ModelInfo> {
    crate::model_registry::models_by_category(category)
}

#[sera_doc(category = "utility", file = "api/reference.md", en = "Returns all models for a given domain (ml or plot).", fr = "Retourne tous les modèles pour un domaine donné (ml ou plot).", param(name = "domain", ty = "str", en = "Domain: 'ml' or 'plot'.", fr = "Domaine: 'ml' ou 'plot'."))]
#[sera_bind(serde)]
pub fn models_for_domain(domain: &str) -> Vec<&'static crate::model_registry::ModelInfo> {
    crate::model_registry::models_by_domain(domain)
}

#[cfg(feature = "python")]
#[pymodule]
fn seraplot(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    if let Ok(sys) = py.import("sys") {
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
    m.add_class::<Chart>()?;
    m.add_class::<PyDatasetStats>()?;
    m.add_class::<PyDataset>()?;
    m.add("__version__", VERSION)?;

    m.add_function(wrap_pyfunction!(__sera_py_hw, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_set_adaptive_retry, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_reset_perf_state, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_adaptive_degrade_level, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_set_global_background, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_reset_global_background, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_set_auto_display, m)?)?;
    m.add_function(wrap_pyfunction!(theme, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_reset_theme, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_themes, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_demos, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_demo, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_chart_variants, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_params, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_required_params, m)?)?;
    m.add_function(wrap_pyfunction!(config, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_reset_config, m)?)?;

    bindings::commands::registry::register_submodules(py, m)?;

    m.add_function(wrap_pyfunction!(build_grid, m)?)?;
    m.add_function(wrap_pyfunction!(grid, m)?)?;
    m.add_function(wrap_pyfunction!(build_sysmon, m)?)?;
    m.add_function(wrap_pyfunction!(sysmon, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_telemetry_consent, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_telemetry_path, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_get_metrics, m)?)?;
    m.add_function(wrap_pyfunction!(push_telemetry, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_docs, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_doc, m)?)?;

    Ok(())
}



