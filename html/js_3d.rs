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
) -> String {
    let n = x.len().min(y.len()).min(z.len());
    if n == 0 {
        return String::new();
    }

    let uid = ID3D.fetch_add(1, Ordering::Relaxed);
    let cid = format!("c3d{uid}");
    let uc = !colors.is_empty() && colors.len() >= n;
    let (xl, yl, zl) = axis_labels;

    let cap = n * 40 + 16000;
    let mut buf: Vec<u8> = Vec::with_capacity(cap);

    push_b(&mut buf, b"<!DOCTYPE html><html><head><meta charset=\"utf-8\"><style>");
    push_b(&mut buf, b"body{margin:0;background:#fff}");
    push_b(&mut buf, b".c3w{position:relative;display:inline-block;user-select:none;cursor:grab}");
    push_b(&mut buf, b".c3w:active{cursor:grabbing}");
    push_b(&mut buf, b".c3t{position:absolute;z-index:99;pointer-events:none;opacity:0;");
    push_b(&mut buf, b"transition:opacity .12s;background:#0b0e18;color:#f1f5f9;");
    push_b(&mut buf, b"font:12px -apple-system,Arial,sans-serif;border-radius:8px;");
    push_b(&mut buf, b"padding:8px 12px;min-width:120px;box-shadow:0 4px 16px rgba(0,0,0,.4)}");
    push_b(&mut buf, b".c3t.v{opacity:1}.c3t.p{pointer-events:auto;cursor:default}");
    push_b(&mut buf, b".c3t b{font-size:13px;display:block;margin-bottom:4px}");
    push_b(&mut buf, b".c3t span{color:#94a3b8;margin-right:6px}");
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
    push_b(&mut buf, b";var X=[");
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
function pj(px,py,pz){var ex=zoom*Math.cos(yaw)*Math.cos(pitch),ey=zoom*Math.sin(yaw)*Math.cos(pitch),ez=zoom*Math.sin(pitch);var fx=-ex,fy=-ey,fz=-ez,fl=Math.sqrt(fx*fx+fy*fy+fz*fz);fx/=fl;fy/=fl;fz/=fl;var rx=-fy,ry=fx,rz=0,rl=Math.sqrt(rx*rx+ry*ry)||1e-6;rx/=rl;ry/=rl;var u2x=fy*rz-fz*ry,u2y=fz*rx-fx*rz,u2z=fx*ry-fy*rx;var dx=px-ex,dy=py-ey,dz=pz-ez;var dp=dx*fx+dy*fy+dz*fz;if(dp<0.001)return null;var cx2=dx*rx+dy*ry+dz*rz,cy2=dx*u2x+dy*u2y+dz*u2z;var th=Math.tan(fov/2),asp=W/H;return{x:cx2/(dp*th*asp),y:cy2/(dp*th),d:dp};}
var cV=[[-0.5,-0.5,-0.5],[0.5,-0.5,-0.5],[0.5,0.5,-0.5],[-0.5,0.5,-0.5],[-0.5,-0.5,0.5],[0.5,-0.5,0.5],[0.5,0.5,0.5],[-0.5,0.5,0.5]];
var cE=[[0,1],[1,2],[2,3],[3,0],[4,5],[5,6],[6,7],[7,4],[0,4],[1,5],[2,6],[3,7]];
function R(){
g.clearRect(0,0,W,H);g.fillStyle='#fff';g.fillRect(0,0,W,H);
var mx=W/2,my=H/2,sc=Math.min(W,H)*0.38;
if(ttl){g.font='bold 14px -apple-system,Arial,sans-serif';g.fillStyle='#1a202c';g.textAlign='center';g.textBaseline='top';g.fillText(ttl,W/2,6);}
g.strokeStyle='#e2e8f0';g.lineWidth=0.7;
for(var e=0;e<12;e++){var a=cV[cE[e][0]],b=cV[cE[e][1]];var pa=pj(a[0],a[1],a[2]),pb=pj(b[0],b[1],b[2]);if(!pa||!pb)continue;g.beginPath();g.moveTo(mx+pa.x*sc,my-pa.y*sc);g.lineTo(mx+pb.x*sc,my-pb.y*sc);g.stroke();}
g.strokeStyle='#e5e7eb';g.lineWidth=0.4;
for(var q=1;q<=4;q++){var t=q/5-0.5;var a1=pj(t,-0.5,-0.5),a2=pj(t,0.5,-0.5);if(a1&&a2){g.beginPath();g.moveTo(mx+a1.x*sc,my-a1.y*sc);g.lineTo(mx+a2.x*sc,my-a2.y*sc);g.stroke();}var b1=pj(-0.5,t,-0.5),b2=pj(0.5,t,-0.5);if(b1&&b2){g.beginPath();g.moveTo(mx+b1.x*sc,my-b1.y*sc);g.lineTo(mx+b2.x*sc,my-b2.y*sc);g.stroke();}}
g.font='8px Arial';g.fillStyle='#9ca3af';
for(var k=0;k<=4;k++){var f=k/4;
var ax=pj(f-0.5,-0.5,-0.5);if(ax){g.textAlign='center';g.textBaseline='top';g.fillText((xmn+xr*f).toFixed(1),mx+ax.x*sc,my-ax.y*sc+4);}
var ay=pj(0.5,f-0.5,-0.5);if(ay){g.textAlign='center';g.textBaseline='top';g.fillText((ymn+yr*f).toFixed(1),mx+ay.x*sc,my-ay.y*sc+4);}
var az=pj(0.5,-0.5,f-0.5);if(az){g.textAlign='left';g.textBaseline='middle';g.fillText((zmn+zr*f).toFixed(1),mx+az.x*sc+6,my-az.y*sc);}}
g.font='bold 10px Arial';g.fillStyle='#4b5563';
var lp=pj(0,-0.5,-0.5);if(lp){g.textAlign='center';g.textBaseline='top';g.fillText(xl,mx+lp.x*sc,my-lp.y*sc+16);}
lp=pj(0.5,0,-0.5);if(lp){g.textAlign='center';g.textBaseline='top';g.fillText(yl,mx+lp.x*sc,my-lp.y*sc+16);}
lp=pj(-0.5,-0.5,0);if(lp){g.textAlign='right';g.textBaseline='middle';g.fillText(zl,mx+lp.x*sc-8,my-lp.y*sc);}
pp=[];
if(M===0)rS(mx,my,sc);else if(M===1)rB(mx,my,sc);else rL(mx,my,sc);
if(CL.length>0){var lx2=W-130,ly2=36,lh=CL.length*18+8;g.fillStyle='#f9fafb';g.strokeStyle='#e5e7eb';g.lineWidth=0.8;g.beginPath();g.rect(lx2-6,ly2-4,120,lh);g.fill();g.stroke();g.font='10px Arial';g.textBaseline='middle';g.textAlign='left';for(var li=0;li<CL.length;li++){g.fillStyle=PAL[li%PAL.length];g.beginPath();g.arc(lx2+5,ly2+4+li*18,5,0,TAU);g.fill();g.fillStyle='#374151';g.fillText(CL[li],lx2+14,ly2+4+li*18);}}
}
function rS(mx,my,sc){var pts=[];for(var i=0;i<N;i++){var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;var p=pj(nx,ny,nz);if(!p)continue;pts.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d,ci:uc?C[i]%PAL.length:i%PAL.length,i:i});}
pts.sort(function(a,b){return a.d-b.d;});
for(var j=0;j<pts.length;j++){var p=pts[j],r=Math.max(3,Math.min(8,5+p.d*0.6));g.globalAlpha=0.82;g.fillStyle=PAL[p.ci];g.beginPath();g.arc(p.sx,p.sy,r,0,TAU);g.fill();g.globalAlpha=1;pp.push({sx:p.sx,sy:p.sy,i:p.i,r:r});}}
function rB(mx,my,sc){var bars=[];for(var i=0;i<N;i++){var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;var pb=pj(nx,ny,-0.5),pt=pj(nx,ny,nz);if(!pb||!pt)continue;bars.push({bx:mx+pb.x*sc,by:my-pb.y*sc,tx:mx+pt.x*sc,ty:my-pt.y*sc,d:pt.d,ci:uc?C[i]%PAL.length:i%PAL.length,i:i});}
bars.sort(function(a,b){return a.d-b.d;});
g.lineCap='round';for(var j=0;j<bars.length;j++){var b=bars[j];g.strokeStyle=PAL[b.ci];g.lineWidth=5;g.beginPath();g.moveTo(b.bx,b.by);g.lineTo(b.tx,b.ty);g.stroke();g.fillStyle=PAL[b.ci];g.beginPath();g.arc(b.tx,b.ty,4.5,0,TAU);g.fill();pp.push({sx:b.tx,sy:b.ty,i:b.i,r:5});}}
function rL(mx,my,sc){var pts=[];for(var i=0;i<N;i++){var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;var p=pj(nx,ny,nz);if(!p)continue;pts.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d,ci:uc?C[i]%PAL.length:0,i:i});}
if(pts.length>1){g.strokeStyle=PAL[0];g.lineWidth=2;g.lineJoin='round';g.beginPath();g.moveTo(pts[0].sx,pts[0].sy);for(var j=1;j<pts.length;j++)g.lineTo(pts[j].sx,pts[j].sy);g.stroke();}
for(var j=0;j<pts.length;j++){var p=pts[j];g.fillStyle=PAL[p.ci];g.beginPath();g.arc(p.sx,p.sy,5,0,TAU);g.fill();g.strokeStyle='#fff';g.lineWidth=1.5;g.stroke();pp.push({sx:p.sx,sy:p.sy,i:p.i,r:5});}}
function ht(ex,ey){var bi=-1,bd=400;for(var i=pp.length-1;i>=0;i--){var dx=pp[i].sx-ex,dy=pp[i].sy-ey,d=dx*dx+dy*dy;if(d<bd){bd=d;bi=pp[i].i;}}return bi;}
function sT(idx,ex,ey){var h='<b>'+(CL.length>0&&uc?CL[C[idx]%CL.length]:'Point '+(idx+1))+'</b><span>'+xl+':</span>'+X[idx].toFixed(2)+'<br><span>'+yl+':</span>'+Y[idx].toFixed(2)+'<br><span>'+zl+':</span>'+Z[idx].toFixed(2);tip.innerHTML=h;tip.className='c3t v'+(pin?' p':'');var bx=wrap.getBoundingClientRect();var tx=ex-bx.left+14,ty=ey-bx.top-10;if(tx+160>W)tx=ex-bx.left-170;if(ty<0)ty=ey-bx.top+20;tip.style.left=tx+'px';tip.style.top=ty+'px';}
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
