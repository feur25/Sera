use crate::html::hover::{html_id, html_prefix, html_suffix};
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};

pub fn push_js_str(buf: &mut Vec<u8>, s: &str) {
    for b in s.bytes() {
        match b {
            b'\'' => {
                buf.push(b'\\');
                buf.push(b'\'');
            }
            b'\\' => {
                buf.push(b'\\');
                buf.push(b'\\');
            }
            b'\n' => {
                buf.push(b'\\');
                buf.push(b'n');
            }
            _ => buf.push(b),
        }
    }
}

pub fn b64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}

pub fn pack_points_i16(
    x_values: &[f64],
    y_values: &[f64],
    min_x: f64,
    range_x: f64,
    min_y: f64,
    range_y: f64,
    plot_w: i32,
    plot_h: i32,
) -> String {
    let n = x_values.len().min(y_values.len());
    let inv_rx = plot_w as f64 / range_x.max(1e-12);
    let inv_ry = plot_h as f64 / range_y.max(1e-12);
    let mut raw = Vec::with_capacity(n * 4);
    for i in 0..n {
        let px = ((x_values[i] - min_x) * inv_rx).clamp(0.0, (plot_w - 1) as f64) as i16;
        let py = (plot_h as f64 - (y_values[i] - min_y) * inv_ry).clamp(0.0, (plot_h - 1) as f64) as i16;
        raw.extend_from_slice(&px.to_le_bytes());
        raw.extend_from_slice(&py.to_le_bytes());
    }
    b64_encode(&raw)
}

pub const MODE_POINTS: i32 = 0;
pub const MODE_LINE: i32 = 1;

pub struct CanvasPlotSpec<'a> {
    pub title: &'a str,
    pub width: i32,
    pub height: i32,
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub gridlines: bool,
    pub mode: i32,
    pub color_hex: u32,
}

pub fn render_canvas_points_html(spec: &CanvasPlotSpec, x_values: &[f64], y_values: &[f64]) -> String {
    let n = x_values.len().min(y_values.len());
    let (min_x, max_x) = crate::bindings::utils::simd_ops::find_minmax(x_values);
    let (min_y, max_y) = crate::bindings::utils::simd_ops::find_minmax(y_values);
    let range_x = (max_x - min_x).max(1e-9);
    let range_y = (max_y - min_y).max(1e-9);
    let pad_l = 56i32;
    let pad_t = 36i32;
    let pad_b = 48i32;
    let pad_r = 20i32;
    let plot_w = (spec.width - pad_l - pad_r).max(10);
    let plot_h = (spec.height - pad_t - pad_b).max(10);

    let packed = pack_points_i16(x_values, y_values, min_x, range_x, min_y, range_y, plot_w, plot_h);
    let color = hex6(spec.color_hex);

    let hid = html_id();
    let cv_id = format!("spcv{}", hid);
    let tip_id = format!("sptip{}", hid);
    let mut buf = Vec::<u8>::with_capacity(packed.len() + 8192);
    html_prefix(&mut buf, spec.title, hid);
    push_b(&mut buf, b"<div style=\"position:relative;display:inline-block\">");
    push_b(&mut buf, b"<canvas id=\"");
    buf.extend_from_slice(cv_id.as_bytes());
    push_b(&mut buf, b"\" role=\"img\" aria-label=\"");
    crate::plot::statistical::common::escape_xml(
        &mut buf,
        if spec.title.is_empty() { "2D chart" } else { spec.title },
    );
    push_b(&mut buf, b"\" width=\"");
    push_i(&mut buf, spec.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, spec.height);
    push_b(&mut buf, b"\" style=\"display:block;cursor:grab;touch-action:none\"></canvas>");
    push_b(&mut buf, b"<div id=\"");
    buf.extend_from_slice(tip_id.as_bytes());
    push_b(&mut buf, b"\" style=\"position:absolute;pointer-events:none;opacity:0;transition:opacity .12s;background:#0b0e18;color:#f1f5f9;font:12px -apple-system,Arial,sans-serif;border-radius:8px;padding:6px 10px;white-space:nowrap;box-shadow:0 4px 16px rgba(0,0,0,.4);z-index:10\"></div>");
    push_b(&mut buf, b"</div><script>(function(){");
    push_b(&mut buf, b"var cv=document.getElementById('");
    buf.extend_from_slice(cv_id.as_bytes());
    push_b(&mut buf, b"'),ctx=cv.getContext('2d');");
    push_b(&mut buf, b"var tip=document.getElementById('");
    buf.extend_from_slice(tip_id.as_bytes());
    push_b(&mut buf, b"');");

    push_b(&mut buf, b"var pL=");
    push_i(&mut buf, pad_l);
    push_b(&mut buf, b",pT=");
    push_i(&mut buf, pad_t);
    push_b(&mut buf, b",pW=");
    push_i(&mut buf, plot_w);
    push_b(&mut buf, b",pH=");
    push_i(&mut buf, plot_h);
    push_b(&mut buf, b",W=");
    push_i(&mut buf, spec.width);
    push_b(&mut buf, b",H=");
    push_i(&mut buf, spec.height);
    push_b(&mut buf, b",minX=");
    push_f2(&mut buf, min_x);
    push_b(&mut buf, b",maxX=");
    push_f2(&mut buf, max_x);
    push_b(&mut buf, b",rX=");
    push_f2(&mut buf, range_x);
    push_b(&mut buf, b",minY=");
    push_f2(&mut buf, min_y);
    push_b(&mut buf, b",maxY=");
    push_f2(&mut buf, max_y);
    push_b(&mut buf, b",rY=");
    push_f2(&mut buf, range_y);
    push_b(&mut buf, b",N=");
    buf.extend_from_slice(n.to_string().as_bytes());
    push_b(&mut buf, b",MODE=");
    buf.extend_from_slice(spec.mode.to_string().as_bytes());
    push_b(&mut buf, b",GL=");
    push_b(&mut buf, if spec.gridlines { b"1" } else { b"0" });
    push_b(&mut buf, b",COLOR='#");
    buf.extend_from_slice(&color);
    push_b(&mut buf, b"',XL='");
    push_js_str(&mut buf, spec.x_label);
    push_b(&mut buf, b"',YL='");
    push_js_str(&mut buf, spec.y_label);
    push_b(&mut buf, b"';");

    push_b(&mut buf, b"function b64(s){var b=atob(s),n=b.length,a=new Int16Array(n/2);for(var i=0;i<n;i+=2)a[i/2]=b.charCodeAt(i)|(b.charCodeAt(i+1)<<8);return a;}");
    push_b(&mut buf, b"var PTS=b64('");
    buf.extend_from_slice(packed.as_bytes());
    push_b(&mut buf, b"');");

    push_b(&mut buf, b"var scale=1,offX=0,offY=0,maxScale=200;");
    push_b(&mut buf, b"function fmt(v){return Math.abs(v)>=1000?Math.round(v).toLocaleString():(Math.round(v*100)/100).toString();}");

    push_b(&mut buf, b"function visRange(){");
    push_b(&mut buf, b"var lx0=Math.max(0,(0-offX)/scale),lx1=Math.min(pW,(pW-offX)/scale);");
    push_b(&mut buf, b"var ly0=Math.max(0,(0-offY)/scale),ly1=Math.min(pH,(pH-offY)/scale);");
    push_b(&mut buf, b"var vx0=minX+(lx0/pW)*rX,vx1=minX+(lx1/pW)*rX;");
    push_b(&mut buf, b"var vy1=maxY-(ly0/pH)*rY,vy0=maxY-(ly1/pH)*rY;");
    push_b(&mut buf, b"return [vx0,vx1,vy0,vy1];}");

    push_b(&mut buf, b"function draw(){");
    push_b(&mut buf, b"ctx.setTransform(1,0,0,1,0,0);ctx.clearRect(0,0,W,H);");
    push_b(&mut buf, b"ctx.fillStyle='#ffffff';ctx.fillRect(0,0,W,H);");
    push_b(&mut buf, b"ctx.font='11px Arial,sans-serif';ctx.fillStyle='#1a202c';ctx.textAlign='center';");
    if !spec.title.is_empty() {
        push_b(&mut buf, b"ctx.font='700 15px -apple-system,Arial,sans-serif';ctx.fillText('");
        push_js_str(&mut buf, spec.title);
        push_b(&mut buf, b"',W/2,22);");
    }
    push_b(&mut buf, b"var r=visRange();");
    push_b(&mut buf, b"ctx.strokeStyle='#e2e8f0';ctx.fillStyle='#6b7280';ctx.font='9px Arial,sans-serif';");
    push_b(&mut buf, b"for(var i=0;i<=5;i++){var f=i/5,dx=r[0]+f*(r[1]-r[0]),sx=pL+(dx-minX)/rX*pW*scale+offX;");
    push_b(&mut buf, b"if(sx<pL-1||sx>pL+pW+1)continue;");
    push_b(&mut buf, b"if(GL){ctx.beginPath();ctx.moveTo(sx,pT);ctx.lineTo(sx,pT+pH);ctx.stroke();}");
    push_b(&mut buf, b"ctx.textAlign='center';ctx.fillText(fmt(dx),sx,pT+pH+14);}");
    push_b(&mut buf, b"ctx.textAlign='right';");
    push_b(&mut buf, b"for(var i=0;i<=5;i++){var f=i/5,dy=r[2]+f*(r[3]-r[2]),sy=pT+pH-(dy-minY)/rY*pH*scale-offY;");
    push_b(&mut buf, b"if(sy<pT-1||sy>pT+pH+1)continue;");
    push_b(&mut buf, b"if(GL){ctx.beginPath();ctx.moveTo(pL,sy);ctx.lineTo(pL+pW,sy);ctx.stroke();}");
    push_b(&mut buf, b"ctx.fillText(fmt(dy),pL-6,sy+3);}");
    push_b(&mut buf, b"ctx.strokeStyle='#64748b';ctx.beginPath();ctx.moveTo(pL,pT);ctx.lineTo(pL,pT+pH);ctx.lineTo(pL+pW,pT+pH);ctx.stroke();");
    if !spec.x_label.is_empty() {
        push_b(&mut buf, b"ctx.textAlign='center';ctx.fillStyle='#374151';ctx.fillText(XL,pL+pW/2,H-6);");
    }
    if !spec.y_label.is_empty() {
        push_b(&mut buf, b"ctx.save();ctx.translate(14,pT+pH/2);ctx.rotate(-Math.PI/2);ctx.textAlign='center';ctx.fillStyle='#374151';ctx.fillText(YL,0,0);ctx.restore();");
    }
    push_b(&mut buf, b"ctx.save();ctx.beginPath();ctx.rect(pL,pT,pW,pH);ctx.clip();");
    push_b(&mut buf, b"ctx.translate(pL+offX,pT+offY);ctx.scale(scale,scale);");
    push_b(&mut buf, b"if(MODE===1){ctx.strokeStyle=COLOR;ctx.lineWidth=Math.max(0.6,1.6/scale);ctx.beginPath();");
    push_b(&mut buf, b"for(var i=0;i<N;i++){var x=PTS[i*2],y=PTS[i*2+1];if(i===0)ctx.moveTo(x,y);else ctx.lineTo(x,y);}ctx.stroke();}");
    push_b(&mut buf, b"else{ctx.fillStyle=COLOR;var r2=Math.max(0.8,2.2/scale);");
    push_b(&mut buf, b"for(var i=0;i<N;i++){var x=PTS[i*2],y=PTS[i*2+1];ctx.beginPath();ctx.arc(x,y,r2,0,6.2832);ctx.fill();}}");
    push_b(&mut buf, b"ctx.restore();}");

    push_b(&mut buf, b"function nearest(mx,my){var lx=(mx-pL-offX)/scale,ly=(my-pT-offY)/scale;var bd=1e18,bi=-1;");
    push_b(&mut buf, b"for(var i=0;i<N;i++){var dx=PTS[i*2]-lx,dy=PTS[i*2+1]-ly,d=dx*dx+dy*dy;if(d<bd){bd=d;bi=i;}}");
    push_b(&mut buf, b"return bi;}");

    push_b(&mut buf, b"cv.addEventListener('wheel',function(e){e.preventDefault();var rc=cv.getBoundingClientRect();var mx=e.clientX-rc.left,my=e.clientY-rc.top;");
    push_b(&mut buf, b"var lx=(mx-pL-offX)/scale,ly=(my-pT-offY)/scale;var f=e.deltaY<0?1.18:1/1.18;");
    push_b(&mut buf, b"scale=Math.min(maxScale,Math.max(1,scale*f));offX=mx-pL-lx*scale;offY=my-pT-ly*scale;draw();},{passive:false});");
    push_b(&mut buf, b"var dragging=false,lastX=0,lastY=0;");
    push_b(&mut buf, b"cv.addEventListener('mousedown',function(e){dragging=true;lastX=e.clientX;lastY=e.clientY;cv.style.cursor='grabbing';});");
    push_b(&mut buf, b"window.addEventListener('mouseup',function(){dragging=false;cv.style.cursor='grab';});");
    push_b(&mut buf, b"cv.addEventListener('mousemove',function(e){var rc=cv.getBoundingClientRect();var mx=e.clientX-rc.left,my=e.clientY-rc.top;");
    push_b(&mut buf, b"if(dragging){offX+=e.clientX-lastX;offY+=e.clientY-lastY;lastX=e.clientX;lastY=e.clientY;draw();tip.style.opacity=0;return;}");
    push_b(&mut buf, b"if(mx<pL||mx>pL+pW||my<pT||my>pT+pH){tip.style.opacity=0;return;}");
    push_b(&mut buf, b"var bi=nearest(mx,my);if(bi<0){tip.style.opacity=0;return;}");
    push_b(&mut buf, b"var dx=minX+(PTS[bi*2]/pW)*rX,dy=maxY-(PTS[bi*2+1]/pH)*rY;");
    push_b(&mut buf, b"tip.innerHTML='<b>'+fmt(dx)+'</b>, '+fmt(dy);tip.style.left=(mx+12)+'px';tip.style.top=(my-8)+'px';tip.style.opacity=1;});");
    push_b(&mut buf, b"cv.addEventListener('mouseleave',function(){tip.style.opacity=0;});");
    push_b(&mut buf, b"cv.addEventListener('dblclick',function(){scale=1;offX=0;offY=0;draw();});");

    push_b(&mut buf, b"var touches={},tDist=0,tLastTap=0,tMoved=false;");
    push_b(&mut buf, b"function touchPos(t){var rc=cv.getBoundingClientRect();return [t.clientX-rc.left,t.clientY-rc.top];}");
    push_b(&mut buf, b"function touchDist(a,b){var dx=a.clientX-b.clientX,dy=a.clientY-b.clientY;return Math.sqrt(dx*dx+dy*dy);}");
    push_b(&mut buf, b"cv.addEventListener('touchstart',function(e){e.preventDefault();tMoved=false;");
    push_b(&mut buf, b"if(e.touches.length===1){var p=touchPos(e.touches[0]);dragging=true;lastX=e.touches[0].clientX;lastY=e.touches[0].clientY;}");
    push_b(&mut buf, b"else if(e.touches.length===2){dragging=false;tDist=touchDist(e.touches[0],e.touches[1]);}},{passive:false});");
    push_b(&mut buf, b"cv.addEventListener('touchmove',function(e){e.preventDefault();tMoved=true;");
    push_b(&mut buf, b"if(e.touches.length===1&&dragging){offX+=e.touches[0].clientX-lastX;offY+=e.touches[0].clientY-lastY;lastX=e.touches[0].clientX;lastY=e.touches[0].clientY;draw();tip.style.opacity=0;}");
    push_b(&mut buf, b"else if(e.touches.length===2){var nd=touchDist(e.touches[0],e.touches[1]);if(tDist>0){var rc=cv.getBoundingClientRect();");
    push_b(&mut buf, b"var mx=(e.touches[0].clientX+e.touches[1].clientX)/2-rc.left,my=(e.touches[0].clientY+e.touches[1].clientY)/2-rc.top;");
    push_b(&mut buf, b"var lx=(mx-pL-offX)/scale,ly=(my-pT-offY)/scale;var f=nd/tDist;");
    push_b(&mut buf, b"scale=Math.min(maxScale,Math.max(1,scale*f));offX=mx-pL-lx*scale;offY=my-pT-ly*scale;draw();}tDist=nd;}},{passive:false});");
    push_b(&mut buf, b"cv.addEventListener('touchend',function(e){dragging=false;tDist=0;");
    push_b(&mut buf, b"if(!tMoved&&e.changedTouches.length===1){var p=touchPos(e.changedTouches[0]);var mx=p[0],my=p[1];");
    push_b(&mut buf, b"var now=Date.now();if(now-tLastTap<320){scale=1;offX=0;offY=0;draw();tLastTap=0;e.preventDefault();return;}tLastTap=now;");
    push_b(&mut buf, b"if(mx<pL||mx>pL+pW||my<pT||my>pT+pH){tip.style.opacity=0;return;}");
    push_b(&mut buf, b"var bi=nearest(mx,my);if(bi<0){tip.style.opacity=0;return;}");
    push_b(&mut buf, b"var dx=minX+(PTS[bi*2]/pW)*rX,dy=maxY-(PTS[bi*2+1]/pH)*rY;");
    push_b(&mut buf, b"tip.innerHTML='<b>'+fmt(dx)+'</b>, '+fmt(dy);tip.style.left=(mx+12)+'px';tip.style.top=(my-8)+'px';tip.style.opacity=1;");
    push_b(&mut buf, b"setTimeout(function(){tip.style.opacity=0;},2200);}});");

    push_b(&mut buf, b"draw();");
    push_b(&mut buf, b"})();</script>");

    html_suffix(&mut buf, hid, "[]");
    unsafe { String::from_utf8_unchecked(buf) }
}
