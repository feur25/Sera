use super::common::{prepare, SB_PALETTE};
use super::config::SunburstConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::hex6;

fn darken(c: u32, f: f64) -> u32 {
    let r = (((c >> 16) & 0xFF) as f64 * f).min(255.0) as u32;
    let g = (((c >> 8) & 0xFF) as f64 * f).min(255.0) as u32;
    let b = ((c & 0xFF) as f64 * f).min(255.0) as u32;
    (r << 16) | (g << 8) | b
}

fn json_f(v: f64) -> String {
    format!("{:.5}", v)
}

const JS: &str = r#"(function(){
var HOLE=__HOLE__,RW=__RW__,CX=__CX__,CY=__CY__,RMAX=__RMAX__;
var X0=__X0__,X1=__X1__,Y=__Y__,COL=__COL__,LAB=__LAB__,HC=__HC__;
var N=X0.length;
var HPI=-Math.PI/2,PI2=2*Math.PI;
var DUR=680;

var state={a0:HPI,a1:HPI+PI2,vd:0};
var hist=[];
var animId=null,animT0=null,from_s=null,to_s=null;

function lerp(a,b,t){return a+(b-a)*t;}
function ease(t){return t<.5?2*t*t:(4-2*t)*t-1;}

function display(i,s){
    var span=s.a1-s.a0;
    if(span<1e-9)return null;
    var t0=(X0[i]-s.a0)/span,t1=(X1[i]-s.a0)/span;
    var x0=HPI+Math.max(0,Math.min(1,t0))*PI2;
    var x1=HPI+Math.max(0,Math.min(1,t1))*PI2;
    var ny=Math.max(0,Y[i]-s.vd);
    var r0=HOLE+ny*RW,r1=HOLE+(ny+1)*RW-1;
    return{x0:x0,x1:x1,r0:r0,r1:r1};
}

function arcPath(x0,x1,r0,r1){
    if(x1-x0<5e-4||r1<=HOLE)return'';
    var lg=x1-x0>Math.PI?1:0;
    var ax=Math.cos(x0),ay=Math.sin(x0);
    var bx=Math.cos(x1),by=Math.sin(x1);
    return'M '+(r0*ax).toFixed(2)+' '+(r0*ay).toFixed(2)+
           ' A '+r0+' '+r0+' 0 '+lg+' 1 '+(r0*bx).toFixed(2)+' '+(r0*by).toFixed(2)+
           ' L '+(r1*bx).toFixed(2)+' '+(r1*by).toFixed(2)+
           ' A '+r1+' '+r1+' 0 '+lg+' 0 '+(r1*ax).toFixed(2)+' '+(r1*ay).toFixed(2)+' Z';
}

function lblTx(d){
    var mx=(d.x0+d.x1)/2,my=(d.r0+d.r1)/2;
    var deg=mx*180/Math.PI;
    var flip=mx>Math.PI/2?180:0;
    return'rotate('+deg+') translate('+my+',0) rotate('+flip+')';
}

function lblFs(d){
    var arw=d.r1-d.r0;
    return Math.min(11,Math.max(7,arw*0.38)).toFixed(1);
}

function isVis(d){return d&&d.x1-d.x0>5e-4&&d.r1>HOLE;}
function isLbl(d){
    if(!isVis(d))return false;
    var arw=d.r1-d.r0;
    var arclen=(d.r0+d.r1)/2*(d.x1-d.x0);
    return arw>=12&&arclen>=40;
}

var svg=document.getElementById('sp-z');
var g=document.createElementNS('http://www.w3.org/2000/svg','g');
g.setAttribute('transform','translate('+CX+','+CY+')');
svg.appendChild(g);

var cBg=document.createElementNS('http://www.w3.org/2000/svg','circle');
cBg.setAttribute('r',String(HOLE-1));
cBg.setAttribute('fill','#ffffff');
cBg.setAttribute('pointer-events','none');
g.appendChild(cBg);

var paths=[],texts=[];
for(var i=0;i<N;i++){
    var p=document.createElementNS('http://www.w3.org/2000/svg','path');
    p.setAttribute('fill',COL[i]);
    p.setAttribute('fill-opacity','0.88');
    p.setAttribute('stroke','rgba(255,255,255,0.5)');
    p.setAttribute('stroke-width','0.8');
    p.style.cursor=Number(HC[i])?'pointer':'default';
    (function(idx){
        p.addEventListener('mouseenter',function(){doHover(idx);});
        p.addEventListener('mouseleave',clearHover);
        if(Number(HC[idx])){
            p.addEventListener('click',function(){doZoom(idx);});
        }
    })(i);
    g.appendChild(p);
    paths.push(p);

    var t=document.createElementNS('http://www.w3.org/2000/svg','text');
    t.setAttribute('text-anchor','middle');
    t.setAttribute('dy','0.35em');
    t.setAttribute('font-family','-apple-system,Arial,sans-serif');
    t.setAttribute('font-size','10');
    t.setAttribute('fill','rgba(255,255,255,0.92)');
    t.setAttribute('pointer-events','none');
    t.textContent=LAB[i];
    g.appendChild(t);
    texts.push(t);
}

var upCirc=document.createElementNS('http://www.w3.org/2000/svg','circle');
upCirc.setAttribute('r',String(HOLE-2));
upCirc.setAttribute('fill','rgba(0,0,0,0.04)');
upCirc.setAttribute('pointer-events','all');
upCirc.style.cursor='pointer';
upCirc.addEventListener('click',doUp);
g.appendChild(upCirc);

var upTxt=document.createElementNS('http://www.w3.org/2000/svg','text');
upTxt.setAttribute('text-anchor','middle');
upTxt.setAttribute('dy','0.35em');
upTxt.setAttribute('font-size','18');
upTxt.setAttribute('fill','rgba(80,80,80,0.5)');
upTxt.setAttribute('pointer-events','none');
upTxt.textContent='';
g.appendChild(upTxt);

function drawS(s){
    for(var i=0;i<N;i++){
        var d=display(i,s);
        var v=isVis(d);
        if(v){
            paths[i].setAttribute('d',arcPath(d.x0,d.x1,d.r0,d.r1));
            paths[i].setAttribute('fill-opacity','0.88');
            paths[i].style.pointerEvents='auto';
        }else{
            paths[i].setAttribute('d','');
            paths[i].setAttribute('fill-opacity','0');
            paths[i].style.pointerEvents='none';
        }
        var lv=isLbl(d);
        if(lv){
            texts[i].setAttribute('transform',lblTx(d));
            texts[i].setAttribute('font-size',lblFs(d));
            texts[i].setAttribute('fill','rgba(255,255,255,0.92)');
        }else{
            texts[i].setAttribute('fill','rgba(0,0,0,0)');
        }
    }
}

function interpS(p){
    return{
        a0:lerp(from_s.a0,to_s.a0,p),
        a1:lerp(from_s.a1,to_s.a1,p),
        vd:lerp(from_s.vd,to_s.vd,p)
    };
}

function startAnim(){
    if(animId)cancelAnimationFrame(animId);
    animT0=null;
    function frame(ts){
        if(!animT0)animT0=ts;
        var p=ease(Math.min((ts-animT0)/DUR,1));
        drawS(interpS(p));
        if(p<1){animId=requestAnimationFrame(frame);}
        else{state=to_s;animId=null;}
    }
    animId=requestAnimationFrame(frame);
}

function doZoom(i){
    hist.push({a0:state.a0,a1:state.a1,vd:state.vd});
    from_s={a0:state.a0,a1:state.a1,vd:state.vd};
    to_s={a0:X0[i],a1:X1[i],vd:Y[i]};
    state=from_s;
    startAnim();
    upTxt.textContent='⬆';
}

function doUp(){
    if(!hist.length)return;
    var prev=hist.pop();
    from_s={a0:state.a0,a1:state.a1,vd:state.vd};
    to_s=prev;
    state=from_s;
    startAnim();
    upTxt.textContent=hist.length?'⬆':'';
}

function doHover(i){
    for(var j=0;j<N;j++){
        if(paths[j].style.pointerEvents==='none')continue;
        var isAnc=X0[j]<=X0[i]+1e-6&&X1[j]>=X1[i]-1e-6&&Y[j]<Y[i];
        var isDes=X0[j]>=X0[i]-1e-6&&X1[j]<=X1[i]+1e-6&&Y[j]>Y[i];
        paths[j].setAttribute('fill-opacity',
            (j===i||isAnc||isDes)?'1':'0.18');
    }
}

function clearHover(){
    drawS(state);
}

state={a0:HPI,a1:HPI+PI2,vd:0};
from_s=to_s={a0:HPI,a1:HPI+PI2,vd:0};
drawS(state);
})();"#;

pub fn render(cfg: &SunburstConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };

    let palette: &[u32] = if cfg.palette.is_empty() { SB_PALETTE } else { cfg.palette };

    let mut has_children = vec![false; p.n];
    {
        let mut label_idx = std::collections::HashMap::with_capacity(p.n);
        for (i, l) in p.labels.iter().enumerate() {
            label_idx.insert(l.as_str(), i);
        }
        for i in 0..p.n {
            if i < cfg.parents.len() {
                if let Some(&pi) = label_idx.get(cfg.parents[i].as_str()) {
                    has_children[pi] = true;
                }
            }
        }
    }

    let darken_factors = [1.0f64, 1.0, 0.78, 0.62, 0.50];

    let mut x0_arr = Vec::with_capacity(p.n);
    let mut x1_arr = Vec::with_capacity(p.n);
    let mut y_arr  = Vec::with_capacity(p.n);
    let mut col_arr = Vec::with_capacity(p.n);
    let mut lab_arr = Vec::with_capacity(p.n);
    let mut hc_arr  = Vec::with_capacity(p.n);

    for &i in &p.bfs_order {
        let (a0, a1) = p.ang[i];
        if a1 - a0 < 1e-6 { continue; }
        let depth = p.depth[i];
        let base = palette[p.cidx[i] % palette.len()];
        let factor = darken_factors[depth.min(darken_factors.len() - 1)];
        let col = darken(base, factor);
        let hx = hex6(col);
        let color = format!("\"#{}\"", std::str::from_utf8(&hx).unwrap_or("636EFA"));
        let label = p.labels[i]
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        x0_arr.push(json_f(a0));
        x1_arr.push(json_f(a1));
        y_arr.push(depth.to_string());
        col_arr.push(color);
        lab_arr.push(format!("\"{}\"", label));
        hc_arr.push(if has_children[i] { "1" } else { "0" });
    }

    let cx = p.layout.cx;
    let cy = p.layout.cy;
    let hole = p.layout.r_hole;
    let rw = p.layout.ring_w;

    let js = JS
        .replace("__HOLE__", &hole.to_string())
        .replace("__RW__", &rw.to_string())
        .replace("__CX__", &cx.to_string())
        .replace("__CY__", &cy.to_string())
        .replace("__X0__", &format!("[{}]", x0_arr.join(",")))
        .replace("__X1__", &format!("[{}]", x1_arr.join(",")))
        .replace("__Y__", &format!("[{}]", y_arr.join(",")))
        .replace("__COL__", &format!("[{}]", col_arr.join(",")))
        .replace("__LAB__", &format!("[{}]", lab_arr.join(",")))
        .replace("__RMAX__", &p.layout.r_max.to_string())
        .replace("__HC__", &format!("[{}]", hc_arr.join(",")));

    let svg = format!(
        "<svg id=\"sp-z\" xmlns=\"http://www.w3.org/2000/svg\" \
         width=\"{}\" height=\"{}\" style=\"display:block;overflow:visible\">\
         <rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>\
         </svg><script>{}</script>",
        cfg.width, cfg.height, js
    );

    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
