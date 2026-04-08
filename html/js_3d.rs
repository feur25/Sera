use crate::plot::statistical::common::{push_b, push_i, push_f2, PALETTE};
use std::sync::atomic::{AtomicU32, Ordering};

static ID3D: AtomicU32 = AtomicU32::new(0);

fn js_esc(buf: &mut Vec<u8>, s: &str) {
    for ch in s.chars() {
        match ch {
            '\'' => push_b(buf, b"\\'"),
            '\\' => push_b(buf, b"\\\\"),
            '\n' => push_b(buf, b"\\n"),
            c => {
                let mut tmp = [0u8; 4];
                buf.extend_from_slice(c.encode_utf8(&mut tmp).as_bytes());
            }
        }
    }
}

pub fn render_3d_html(
    mode: u8,
    title: &str,
    x: &[f64],
    y: &[f64],
    z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
) -> String {
    let n = x.len().min(y.len()).min(z.len());
    if n == 0 {
        return String::new();
    }

    let uid = ID3D.fetch_add(1, Ordering::Relaxed);
    let cid = format!("c3d{uid}");
    let uc = !colors.is_empty() && colors.len() >= n;
    let (xl, yl, zl) = axis_labels;

    let cap = n * 40 + 24000;
    let mut buf: Vec<u8> = Vec::with_capacity(cap);

    let bg = match bg_color {
        Some(c) if !c.is_empty() && c != "transparent" && c != "none" => c,
        Some(_) => "transparent",
        None => "#0e1117",
    };

    push_b(&mut buf, b"<!DOCTYPE html><html><head><meta charset=\"utf-8\"><style>");
    push_b(&mut buf, b"body{margin:0;background:");
    buf.extend_from_slice(bg.as_bytes());
    push_b(&mut buf, b";display:flex;justify-content:center;padding:16px 0}");
    push_b(&mut buf, b".c3w{position:relative;display:inline-block;user-select:none;cursor:grab;border-radius:12px;overflow:hidden;box-shadow:0 8px 32px rgba(0,0,0,.5),0 0 0 1px rgba(255,255,255,.06)}");
    push_b(&mut buf, b".c3w:active{cursor:grabbing}");
    push_b(&mut buf, b".c3t{position:absolute;z-index:99;pointer-events:none;opacity:0;");
    push_b(&mut buf, b"transition:opacity .15s,transform .15s;transform:translateY(4px) scale(.97);");
    push_b(&mut buf, b"background:rgba(11,14,24,.92);color:#f1f5f9;backdrop-filter:blur(8px);");
    push_b(&mut buf, b"font:12px -apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;border-radius:10px;");
    push_b(&mut buf, b"padding:10px 14px;min-width:140px;box-shadow:0 8px 24px rgba(0,0,0,.5),0 0 0 1px rgba(255,255,255,.08)}");
    push_b(&mut buf, b".c3t.v{opacity:1;transform:translateY(0) scale(1)}.c3t.p{pointer-events:auto;cursor:default}");
    push_b(&mut buf, b".c3t b{font-size:13px;display:block;margin-bottom:6px;color:#e2e8f0}");
    push_b(&mut buf, b".c3t span{color:#64748b;margin-right:6px;font-size:11px}");
    push_b(&mut buf, b".c3t .tv{color:#f8fafc;font-weight:600}");
    push_b(&mut buf, b"</style></head><body>");

    push_b(&mut buf, b"<div id=\"");
    buf.extend_from_slice(cid.as_bytes());
    push_b(&mut buf, b"\" class=\"c3w\" style=\"width:");
    push_i(&mut buf, w);
    push_b(&mut buf, b"px;height:");
    push_i(&mut buf, h);
    push_b(&mut buf, b"px\">");

    push_b(&mut buf, b"<canvas id=\"");
    buf.extend_from_slice(cid.as_bytes());
    push_b(&mut buf, b"c\" style=\"width:");
    push_i(&mut buf, w);
    push_b(&mut buf, b"px;height:");
    push_i(&mut buf, h);
    push_b(&mut buf, b"px\"></canvas>");

    push_b(&mut buf, b"<div id=\"");
    buf.extend_from_slice(cid.as_bytes());
    push_b(&mut buf, b"t\" class=\"c3t\"></div></div>");

    push_b(&mut buf, b"<script>(function(){var W=");
    push_i(&mut buf, w);
    push_b(&mut buf, b",H=");
    push_i(&mut buf, h);
    push_b(&mut buf, b",cid='");
    buf.extend_from_slice(cid.as_bytes());
    push_b(&mut buf, b"',M=");
    buf.push(mode + b'0');
    push_b(&mut buf, b",BG='");
    buf.extend_from_slice(bg.as_bytes());
    push_b(&mut buf, b"';var X=[");
    for i in 0..n {
        if i > 0 { buf.push(b','); }
        push_f2(&mut buf, x[i]);
    }
    push_b(&mut buf, b"],Y=[");
    for i in 0..n {
        if i > 0 { buf.push(b','); }
        push_f2(&mut buf, y[i]);
    }
    push_b(&mut buf, b"],Z=[");
    for i in 0..n {
        if i > 0 { buf.push(b','); }
        push_f2(&mut buf, z[i]);
    }
    push_b(&mut buf, b"],C=[");
    if uc {
        for i in 0..n {
            if i > 0 { buf.push(b','); }
            push_i(&mut buf, colors[i].round() as i32);
        }
    }
    push_b(&mut buf, b"];");

    push_b(&mut buf, b"var PAL=[");
    let hx = b"0123456789abcdef";
    for (i, &c) in PALETTE.iter().enumerate() {
        if i > 0 { buf.push(b','); }
        push_b(&mut buf, b"'#");
        buf.extend_from_slice(&[
            hx[((c >> 20) & 0xf) as usize],
            hx[((c >> 16) & 0xf) as usize],
            hx[((c >> 12) & 0xf) as usize],
            hx[((c >> 8) & 0xf) as usize],
            hx[((c >> 4) & 0xf) as usize],
            hx[(c & 0xf) as usize],
        ]);
        buf.push(b'\'');
    }
    push_b(&mut buf, b"];");

    push_b(&mut buf, b"var CL=[");
    for (i, lbl) in color_labels.iter().enumerate() {
        if i > 0 { buf.push(b','); }
        buf.push(b'\'');
        js_esc(&mut buf, lbl);
        buf.push(b'\'');
    }
    push_b(&mut buf, b"];");

    push_b(&mut buf, b"var xl='");
    js_esc(&mut buf, xl);
    push_b(&mut buf, b"',yl='");
    js_esc(&mut buf, yl);
    push_b(&mut buf, b"',zl='");
    js_esc(&mut buf, zl);
    push_b(&mut buf, b"',ttl='");
    js_esc(&mut buf, title);
    push_b(&mut buf, b"';");

    push_b(&mut buf, ENGINE_3D);

    push_b(&mut buf, b"})();</script></body></html>");

    unsafe { String::from_utf8_unchecked(buf) }
}

const ENGINE_3D: &[u8] = br#"
var N=X.length,uc=C.length>=N;
var xmn=1e18,xmx=-1e18,ymn=1e18,ymx=-1e18,zmn=1e18,zmx=-1e18;
for(var i=0;i<N;i++){if(X[i]<xmn)xmn=X[i];if(X[i]>xmx)xmx=X[i];if(Y[i]<ymn)ymn=Y[i];if(Y[i]>ymx)ymx=Y[i];if(Z[i]<zmn)zmn=Z[i];if(Z[i]>zmx)zmx=Z[i];}
var xr=xmx-xmn||1,yr=ymx-ymn||1,zr=zmx-zmn||1;
var yaw=0.785,pitch=0.6,zoom=1.0,TAU=6.2832,fov=0.7854;
var dg=false,lx=0,ly=0,mv=false,dwX=0,dwY=0;
var cv=document.getElementById(cid+'c'),g=cv.getContext('2d'),wrap=document.getElementById(cid),tip=document.getElementById(cid+'t');
var dpr=window.devicePixelRatio||1;cv.width=W*dpr;cv.height=H*dpr;g.scale(dpr,dpr);
var pin=false,piI=-1,piX=0,piY=0,pp=[];
function hx2rgb(h){var r=parseInt(h.slice(1,3),16),g=parseInt(h.slice(3,5),16),b=parseInt(h.slice(5,7),16);return [r,g,b];}
function pj(px,py,pz){var ex=zoom*Math.cos(yaw)*Math.cos(pitch),ey=zoom*Math.sin(yaw)*Math.cos(pitch),ez=zoom*Math.sin(pitch);var fx=-ex,fy=-ey,fz=-ez,fl=Math.sqrt(fx*fx+fy*fy+fz*fz);fx/=fl;fy/=fl;fz/=fl;var rx=-fy,ry=fx,rz=0,rl=Math.sqrt(rx*rx+ry*ry)||1e-6;rx/=rl;ry/=rl;var u2x=fy*rz-fz*ry,u2y=fz*rx-fx*rz,u2z=fx*ry-fy*rx;var dx=px-ex,dy=py-ey,dz=pz-ez;var dp=dx*fx+dy*fy+dz*fz;if(dp<0.001)return null;var cx2=dx*rx+dy*ry+dz*rz,cy2=dx*u2x+dy*u2y+dz*u2z;var th=Math.tan(fov/2),asp=W/H;return{x:cx2/(dp*th*asp),y:cy2/(dp*th),d:dp};}
var cV=[[-0.5,-0.5,-0.5],[0.5,-0.5,-0.5],[0.5,0.5,-0.5],[-0.5,0.5,-0.5],[-0.5,-0.5,0.5],[0.5,-0.5,0.5],[0.5,0.5,0.5],[-0.5,0.5,0.5]];
var cE=[[0,1],[1,2],[2,3],[3,0],[4,5],[5,6],[6,7],[7,4],[0,4],[1,5],[2,6],[3,7]];
var isDark=BG==='transparent'||BG==='#0e1117'||BG.match(/^#[0-3]/);
var bgFill=BG==='transparent'?'rgba(0,0,0,0)':BG;
var gridC=isDark?'rgba(148,163,184,.12)':'rgba(0,0,0,.08)';
var edgeC=isDark?'rgba(148,163,184,.22)':'rgba(0,0,0,.12)';
var txtC=isDark?'#94a3b8':'#64748b';
var lblC=isDark?'#e2e8f0':'#1a202c';
var axC=isDark?'#cbd5e1':'#374151';
function R(){
g.clearRect(0,0,W,H);
if(BG!=='transparent'){
var grd=g.createLinearGradient(0,0,0,H);
if(isDark){grd.addColorStop(0,'#0e1117');grd.addColorStop(0.5,'#131820');grd.addColorStop(1,'#0a0d12');}
else{grd.addColorStop(0,BG);grd.addColorStop(1,BG);}
g.fillStyle=grd;g.fillRect(0,0,W,H);}
var mx=W/2,my=H/2,sc=Math.min(W,H)*0.36;
if(ttl){g.font='600 15px -apple-system,BlinkMacSystemFont,sans-serif';g.fillStyle=lblC;g.textAlign='center';g.textBaseline='top';g.fillText(ttl,W/2,10);}
g.lineWidth=0.5;g.strokeStyle=edgeC;
for(var e=0;e<12;e++){var a=cV[cE[e][0]],b=cV[cE[e][1]];var pa=pj(a[0],a[1],a[2]),pb=pj(b[0],b[1],b[2]);if(!pa||!pb)continue;g.beginPath();g.moveTo(mx+pa.x*sc,my-pa.y*sc);g.lineTo(mx+pb.x*sc,my-pb.y*sc);g.stroke();}
g.strokeStyle=gridC;g.lineWidth=0.3;
for(var q=1;q<=4;q++){var t=q/5-0.5;
var a1=pj(t,-0.5,-0.5),a2=pj(t,0.5,-0.5);if(a1&&a2){g.beginPath();g.moveTo(mx+a1.x*sc,my-a1.y*sc);g.lineTo(mx+a2.x*sc,my-a2.y*sc);g.stroke();}
var b1=pj(-0.5,t,-0.5),b2=pj(0.5,t,-0.5);if(b1&&b2){g.beginPath();g.moveTo(mx+b1.x*sc,my-b1.y*sc);g.lineTo(mx+b2.x*sc,my-b2.y*sc);g.stroke();}
var c1=pj(-0.5,-0.5,t),c2=pj(-0.5,0.5,t);if(c1&&c2){g.beginPath();g.moveTo(mx+c1.x*sc,my-c1.y*sc);g.lineTo(mx+c2.x*sc,my-c2.y*sc);g.stroke();}
var d1=pj(-0.5,-0.5,t),d2=pj(0.5,-0.5,t);if(d1&&d2){g.beginPath();g.moveTo(mx+d1.x*sc,my-d1.y*sc);g.lineTo(mx+d2.x*sc,my-d2.y*sc);g.stroke();}}
g.font='8px -apple-system,sans-serif';g.fillStyle=txtC;
for(var k=0;k<=4;k++){var f=k/4;
var ax0=pj(f-0.5,-0.5,-0.5);if(ax0){g.textAlign='center';g.textBaseline='top';g.fillText((xmn+xr*f).toFixed(1),mx+ax0.x*sc,my-ax0.y*sc+5);}
var ay0=pj(0.5,f-0.5,-0.5);if(ay0){g.textAlign='center';g.textBaseline='top';g.fillText((ymn+yr*f).toFixed(1),mx+ay0.x*sc,my-ay0.y*sc+5);}
var az0=pj(0.5,-0.5,f-0.5);if(az0){g.textAlign='left';g.textBaseline='middle';g.fillText((zmn+zr*f).toFixed(1),mx+az0.x*sc+6,my-az0.y*sc);}}
g.font='600 10px -apple-system,sans-serif';g.fillStyle=axC;
var lp=pj(0,-0.5,-0.5);if(lp){g.textAlign='center';g.textBaseline='top';g.fillText(xl,mx+lp.x*sc,my-lp.y*sc+16);}
lp=pj(0.5,0,-0.5);if(lp){g.textAlign='center';g.textBaseline='top';g.fillText(yl,mx+lp.x*sc,my-lp.y*sc+16);}
lp=pj(-0.5,-0.5,0);if(lp){g.textAlign='right';g.textBaseline='middle';g.fillText(zl,mx+lp.x*sc-10,my-lp.y*sc);}
pp=[];
if(M===0)rS(mx,my,sc);else if(M===1)rB(mx,my,sc);else rL(mx,my,sc);
if(CL.length>0){var lx2=W-140,ly2=36,lh=CL.length*22+12;
g.save();g.globalAlpha=isDark?0.7:0.85;g.fillStyle=isDark?'#1a1f2e':'#f9fafb';g.strokeStyle=isDark?'rgba(255,255,255,.08)':'#e5e7eb';g.lineWidth=1;
var lr=8;g.beginPath();g.moveTo(lx2-4+lr,ly2-6);g.arcTo(lx2-4+128,ly2-6,lx2-4+128,ly2-6+lh,lr);g.arcTo(lx2-4+128,ly2-6+lh,lx2-4,ly2-6+lh,lr);g.arcTo(lx2-4,ly2-6+lh,lx2-4,ly2-6,lr);g.arcTo(lx2-4,ly2-6,lx2-4+128,ly2-6,lr);g.closePath();g.fill();g.stroke();g.restore();
g.font='11px -apple-system,sans-serif';g.textBaseline='middle';g.textAlign='left';
for(var li=0;li<CL.length;li++){var cy2=ly2+6+li*22;g.fillStyle=PAL[li%PAL.length];g.shadowColor=PAL[li%PAL.length];g.shadowBlur=4;g.beginPath();g.arc(lx2+8,cy2,5,0,TAU);g.fill();g.shadowBlur=0;g.fillStyle=isDark?'#e2e8f0':'#374151';g.fillText(CL[li],lx2+18,cy2);}}
}
function rS(mx,my,sc){var pts=[];for(var i=0;i<N;i++){var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;var p=pj(nx,ny,nz);if(!p)continue;pts.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d,ci:uc?C[i]%PAL.length:i%PAL.length,i:i});}
pts.sort(function(a,b){return a.d-b.d;});
for(var j=0;j<pts.length;j++){var p=pts[j],r=Math.max(3,Math.min(9,6-p.d*0.8));
var col=PAL[p.ci],rgb=hx2rgb(col);
var grd=g.createRadialGradient(p.sx-r*0.3,p.sy-r*0.3,0,p.sx,p.sy,r*1.8);
grd.addColorStop(0,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',1)');
grd.addColorStop(0.5,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.7)');
grd.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0)');
g.fillStyle=grd;g.beginPath();g.arc(p.sx,p.sy,r*1.8,0,TAU);g.fill();
g.globalAlpha=0.92;g.fillStyle=col;g.shadowColor=col;g.shadowBlur=6;g.beginPath();g.arc(p.sx,p.sy,r,0,TAU);g.fill();g.shadowBlur=0;
g.globalAlpha=0.4;g.fillStyle='#fff';g.beginPath();g.arc(p.sx-r*0.25,p.sy-r*0.25,r*0.35,0,TAU);g.fill();
g.globalAlpha=1;pp.push({sx:p.sx,sy:p.sy,i:p.i,r:r});}}
function rB(mx,my,sc){var bars=[];for(var i=0;i<N;i++){var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;var pb=pj(nx,ny,-0.5),pt=pj(nx,ny,nz);if(!pb||!pt)continue;bars.push({bx:mx+pb.x*sc,by:my-pb.y*sc,tx:mx+pt.x*sc,ty:my-pt.y*sc,d:pt.d,ci:uc?C[i]%PAL.length:i%PAL.length,i:i,h:Math.abs(nz+0.5)});}
bars.sort(function(a,b){return a.d-b.d;});
for(var j=0;j<bars.length;j++){var b=bars[j],col=PAL[b.ci],rgb=hx2rgb(col);
var bw=Math.max(3,Math.min(8,5.5-b.d*0.3));
g.globalAlpha=0.12;g.fillStyle=col;g.beginPath();g.ellipse(b.bx,b.by,bw*1.2,bw*0.5,0,0,TAU);g.fill();g.globalAlpha=1;
var grd=g.createLinearGradient(b.bx,b.by,b.tx,b.ty);
grd.addColorStop(0,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.5)');
grd.addColorStop(0.6,col);
grd.addColorStop(1,'rgba('+Math.min(255,rgb[0]+40)+','+Math.min(255,rgb[1]+40)+','+Math.min(255,rgb[2]+40)+',1)');
g.strokeStyle=grd;g.lineWidth=bw;g.lineCap='round';g.shadowColor=col;g.shadowBlur=6;
g.beginPath();g.moveTo(b.bx,b.by);g.lineTo(b.tx,b.ty);g.stroke();g.shadowBlur=0;
g.fillStyle='rgba(255,255,255,0.35)';g.beginPath();g.arc(b.tx,b.ty,bw*0.55,0,TAU);g.fill();
pp.push({sx:b.tx,sy:b.ty,i:b.i,r:bw});}}
function rL(mx,my,sc){var pts=[];for(var i=0;i<N;i++){var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;var p=pj(nx,ny,nz);if(!p)continue;pts.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d,ci:uc?C[i]%PAL.length:0,i:i});}
if(pts.length>1){
g.save();g.globalAlpha=0.15;g.strokeStyle=PAL[0];g.lineWidth=8;g.lineJoin='round';g.lineCap='round';g.shadowColor=PAL[0];g.shadowBlur=12;g.beginPath();g.moveTo(pts[0].sx,pts[0].sy);for(var j=1;j<pts.length;j++)g.lineTo(pts[j].sx,pts[j].sy);g.stroke();g.restore();
g.strokeStyle=PAL[0];g.lineWidth=2.5;g.lineJoin='round';g.lineCap='round';g.shadowColor=PAL[0];g.shadowBlur=4;g.beginPath();g.moveTo(pts[0].sx,pts[0].sy);for(var j=1;j<pts.length;j++)g.lineTo(pts[j].sx,pts[j].sy);g.stroke();g.shadowBlur=0;}
for(var j=0;j<pts.length;j++){var p=pts[j],col=PAL[p.ci];
g.fillStyle=col;g.shadowColor=col;g.shadowBlur=6;g.beginPath();g.arc(p.sx,p.sy,4.5,0,TAU);g.fill();g.shadowBlur=0;
g.fillStyle='rgba(255,255,255,0.5)';g.beginPath();g.arc(p.sx-1.2,p.sy-1.2,1.8,0,TAU);g.fill();
g.strokeStyle=isDark?'rgba(255,255,255,.25)':'rgba(0,0,0,.15)';g.lineWidth=1;g.beginPath();g.arc(p.sx,p.sy,4.5,0,TAU);g.stroke();
pp.push({sx:p.sx,sy:p.sy,i:p.i,r:5});}}
function ht(ex,ey){var bi=-1,bd=400;for(var i=pp.length-1;i>=0;i--){var dx=pp[i].sx-ex,dy=pp[i].sy-ey,d=dx*dx+dy*dy;if(d<bd){bd=d;bi=pp[i].i;}}return bi;}
function sT(idx,ex,ey){var h='<b>'+(CL.length>0&&uc?CL[C[idx]%CL.length]:'Point '+(idx+1))+'</b><span>'+xl+':</span> <span class="tv">'+X[idx].toFixed(2)+'</span><br><span>'+yl+':</span> <span class="tv">'+Y[idx].toFixed(2)+'</span><br><span>'+zl+':</span> <span class="tv">'+Z[idx].toFixed(2)+'</span>';tip.innerHTML=h;tip.className='c3t v'+(pin?' p':'');var bx=wrap.getBoundingClientRect();var tx=ex-bx.left+14,ty=ey-bx.top-10;if(tx+160>W)tx=ex-bx.left-170;if(ty<0)ty=ey-bx.top+20;tip.style.left=tx+'px';tip.style.top=ty+'px';}
function hT(){if(!pin){tip.className='c3t';}}
R();
wrap.addEventListener('mousedown',function(e){dg=true;mv=false;lx=e.clientX;ly=e.clientY;dwX=e.clientX;dwY=e.clientY;e.preventDefault();});
window.addEventListener('mousemove',function(e){if(dg){var dx=e.clientX-lx,dy=e.clientY-ly;if(Math.abs(e.clientX-dwX)>3||Math.abs(e.clientY-dwY)>3)mv=true;yaw+=dx*0.008;pitch=Math.max(-1.47,Math.min(1.47,pitch+dy*0.008));lx=e.clientX;ly=e.clientY;R();return;}if(pin)return;var bx=wrap.getBoundingClientRect(),ex=e.clientX-bx.left,ey=e.clientY-bx.top;if(ex<0||ey<0||ex>W||ey>H){hT();return;}var idx=ht(ex,ey);if(idx>=0)sT(idx,e.clientX,e.clientY);else hT();});
window.addEventListener('mouseup',function(e){if(!dg)return;dg=false;if(!mv){var bx=wrap.getBoundingClientRect(),ex=e.clientX-bx.left,ey=e.clientY-bx.top;var idx=ht(ex,ey);if(idx>=0){pin=true;piI=idx;piX=e.clientX;piY=e.clientY;sT(idx,e.clientX,e.clientY);}else{pin=false;piI=-1;tip.className='c3t';}}});
wrap.addEventListener('wheel',function(e){zoom=Math.max(0.3,Math.min(5,zoom*(e.deltaY>0?1.08:0.93)));R();e.preventDefault();},{passive:false});
wrap.addEventListener('dblclick',function(){yaw=0.785;pitch=0.6;zoom=1.0;pin=false;piI=-1;tip.className='c3t';R();});
wrap.addEventListener('mouseleave',function(){if(!pin)hT();});
document.addEventListener('keydown',function(e){if(e.key==='Escape'&&pin){pin=false;piI=-1;tip.className='c3t';}});
wrap.addEventListener('touchstart',function(e){if(e.touches.length===1){dg=true;mv=false;lx=e.touches[0].clientX;ly=e.touches[0].clientY;dwX=lx;dwY=ly;e.preventDefault();}},{passive:false});
wrap.addEventListener('touchmove',function(e){if(!dg||e.touches.length!==1)return;var dx=e.touches[0].clientX-lx,dy=e.touches[0].clientY-ly;if(Math.abs(e.touches[0].clientX-dwX)>3||Math.abs(e.touches[0].clientY-dwY)>3)mv=true;yaw+=dx*0.008;pitch=Math.max(-1.47,Math.min(1.47,pitch+dy*0.008));lx=e.touches[0].clientX;ly=e.touches[0].clientY;R();e.preventDefault();},{passive:false});
wrap.addEventListener('touchend',function(){dg=false;if(!mv){var bx=wrap.getBoundingClientRect(),ex=dwX-bx.left,ey=dwY-bx.top;var idx=ht(ex,ey);if(idx>=0){pin=true;piI=idx;sT(idx,dwX,dwY);}else{pin=false;piI=-1;tip.className='c3t';}}});
"#;
