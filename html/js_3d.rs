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

const ENGINE_3D: &[u8] = br##"
var N=X.length,uc=C.length>=N;
var xmn=1e18,xmx=-1e18,ymn=1e18,ymx=-1e18,zmn=1e18,zmx=-1e18;
for(var i=0;i<N;i++){if(X[i]<xmn)xmn=X[i];if(X[i]>xmx)xmx=X[i];if(Y[i]<ymn)ymn=Y[i];if(Y[i]>ymx)ymx=Y[i];if(Z[i]<zmn)zmn=Z[i];if(Z[i]>zmx)zmx=Z[i];}
var xr=xmx-xmn||1,yr=ymx-ymn||1,zr=zmx-zmn||1;
var yaw=0.785,pitch=0.6,zoom=1.0,TAU=6.2832,fov=0.8;
var dg=false,lx=0,ly=0,mv=false,dwX=0,dwY=0;
var cv=document.getElementById(cid+'c'),g=cv.getContext('2d'),wrap=document.getElementById(cid),tip=document.getElementById(cid+'t');
var dpr=window.devicePixelRatio||1;cv.width=W*dpr;cv.height=H*dpr;g.scale(dpr,dpr);
var pin=false,piI=-1,pp=[];
var AX='#f472b6',AY='#22d3ee',AZ='#fbbf24';
function hx2rgb(h){return[parseInt(h.slice(1,3),16),parseInt(h.slice(3,5),16),parseInt(h.slice(5,7),16)];}
function mix(a,b,t){return Math.round(a+(b-a)*t);}
function pj(px,py,pz){var ex=zoom*Math.cos(yaw)*Math.cos(pitch),ey=zoom*Math.sin(yaw)*Math.cos(pitch),ez=zoom*Math.sin(pitch);var fx=-ex,fy=-ey,fz=-ez,fl=Math.sqrt(fx*fx+fy*fy+fz*fz);fx/=fl;fy/=fl;fz/=fl;var rx=-fy,ry=fx,rz=0,rl=Math.sqrt(rx*rx+ry*ry)||1e-6;rx/=rl;ry/=rl;var u2x=fy*rz-fz*ry,u2y=fz*rx-fx*rz,u2z=fx*ry-fy*rx;var dx=px-ex,dy=py-ey,dz=pz-ez;var dp=dx*fx+dy*fy+dz*fz;if(dp<0.001)return null;var cx2=dx*rx+dy*ry+dz*rz,cy2=dx*u2x+dy*u2y+dz*u2z;var th=Math.tan(fov/2),asp=W/H;return{x:cx2/(dp*th*asp),y:cy2/(dp*th),d:dp};}
var cV=[[-0.5,-0.5,-0.5],[0.5,-0.5,-0.5],[0.5,0.5,-0.5],[-0.5,0.5,-0.5],[-0.5,-0.5,0.5],[0.5,-0.5,0.5],[0.5,0.5,0.5],[-0.5,0.5,0.5]];
var cE=[[0,1],[1,2],[2,3],[3,0],[4,5],[5,6],[6,7],[7,4],[0,4],[1,5],[2,6],[3,7]];
var isDark=BG==='transparent'||BG==='#0e1117'||(BG.charAt(0)==='#'&&parseInt(BG.slice(1,3),16)<60);
function drawBG(){
  g.clearRect(0,0,W,H);
  if(BG==='transparent')return;
  if(isDark){
    var gr=g.createRadialGradient(W*0.5,H*0.42,0,W*0.5,H*0.5,Math.max(W,H)*0.75);
    gr.addColorStop(0,'#131c2e');gr.addColorStop(0.55,'#0d1117');gr.addColorStop(1,'#060810');
    g.fillStyle=gr;g.fillRect(0,0,W,H);
    var n1=g.createRadialGradient(W*0.1,H*0.88,0,W*0.1,H*0.88,W*0.48);
    n1.addColorStop(0,'rgba(99,102,241,0.04)');n1.addColorStop(1,'rgba(0,0,0,0)');
    g.fillStyle=n1;g.fillRect(0,0,W,H);
    var n2=g.createRadialGradient(W*0.9,H*0.1,0,W*0.9,H*0.1,W*0.4);
    n2.addColorStop(0,'rgba(20,184,166,0.03)');n2.addColorStop(1,'rgba(0,0,0,0)');
    g.fillStyle=n2;g.fillRect(0,0,W,H);
  } else {
    g.fillStyle=BG;g.fillRect(0,0,W,H);
  }
}
function drawFloor(mx,my,sc){
  for(var q=0;q<=5;q++){var f=q/5-0.5;
    var p0=pj(f,-0.5,-0.5),p1=pj(f,0.5,-0.5);
    if(p0&&p1){g.strokeStyle=isDark?'rgba(99,102,241,0.055)':'rgba(0,0,0,0.04)';g.lineWidth=0.5;g.beginPath();g.moveTo(mx+p0.x*sc,my-p0.y*sc);g.lineTo(mx+p1.x*sc,my-p1.y*sc);g.stroke();}
    var q0=pj(-0.5,f,-0.5),q1=pj(0.5,f,-0.5);
    if(q0&&q1){g.strokeStyle=isDark?'rgba(99,102,241,0.055)':'rgba(0,0,0,0.04)';g.lineWidth=0.5;g.beginPath();g.moveTo(mx+q0.x*sc,my-q0.y*sc);g.lineTo(mx+q1.x*sc,my-q1.y*sc);g.stroke();}
  }
}
function arw(p0,p1,col){
  if(!p0||!p1)return;
  var sx0=mx+p0.x*sc,sy0=my-p0.y*sc,sx1=mx+p1.x*sc,sy1=my-p1.y*sc;
  g.strokeStyle=col;g.lineWidth=1.1;g.globalAlpha=0.55;
  g.beginPath();g.moveTo(sx0,sy0);g.lineTo(sx1,sy1);g.stroke();
  var an=Math.atan2(sy1-sy0,sx1-sx0),al=7;
  g.fillStyle=col;g.globalAlpha=0.7;g.beginPath();
  g.moveTo(sx1,sy1);g.lineTo(sx1-al*Math.cos(an-0.42),sy1-al*Math.sin(an-0.42));
  g.lineTo(sx1-al*Math.cos(an+0.42),sy1-al*Math.sin(an+0.42));
  g.closePath();g.fill();g.globalAlpha=1;
}
var mx,my,sc;
function R(){
  drawBG();
  mx=W/2;my=H/2;sc=Math.min(W,H)*0.34;
  if(ttl){
    g.textAlign='center';g.textBaseline='top';
    g.font='600 15px -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif';
    if(isDark){
      var tg=g.createLinearGradient(W/2-120,0,W/2+120,0);
      tg.addColorStop(0,'#818cf8');tg.addColorStop(0.5,'#e2e8f0');tg.addColorStop(1,'#38bdf8');
      g.fillStyle=tg;
    } else { g.fillStyle='#1a202c'; }
    g.fillText(ttl,W/2,10);
  }
  drawFloor(mx,my,sc);
  g.save();
  for(var e=0;e<12;e++){
    var a=cV[cE[e][0]],b=cV[cE[e][1]];
    var pa=pj(a[0],a[1],a[2]),pb=pj(b[0],b[1],b[2]);
    if(!pa||!pb)continue;
    var avgD=(pa.d+pb.d)/2;
    var al2=isDark?Math.max(0.05,0.26-avgD*0.07):Math.max(0.04,0.13-avgD*0.02);
    g.strokeStyle=isDark?'rgba(148,163,184,'+al2+')':'rgba(0,0,0,'+al2+')';
    g.lineWidth=0.7;g.beginPath();g.moveTo(mx+pa.x*sc,my-pa.y*sc);g.lineTo(mx+pb.x*sc,my-pb.y*sc);g.stroke();
  }
  g.restore();
  var or=pj(-0.5,-0.5,-0.5);
  arw(or,pj(0.56,-0.5,-0.5),AX);arw(or,pj(-0.5,0.56,-0.5),AY);arw(or,pj(-0.5,-0.5,0.56),AZ);
  g.font='8.5px -apple-system,sans-serif';
  for(var k=0;k<=4;k++){var f=k/4;
    var ax0=pj(f-0.5,-0.5,-0.5);if(ax0){g.textAlign='center';g.textBaseline='top';g.fillStyle=isDark?'rgba(244,114,182,0.55)':'#9ca3af';g.fillText((xmn+xr*f).toFixed(1),mx+ax0.x*sc,my-ax0.y*sc+5);}
    var ay0=pj(0.5,f-0.5,-0.5);if(ay0){g.textAlign='center';g.textBaseline='top';g.fillStyle=isDark?'rgba(34,211,238,0.55)':'#9ca3af';g.fillText((ymn+yr*f).toFixed(1),mx+ay0.x*sc,my-ay0.y*sc+5);}
    var az0=pj(0.5,-0.5,f-0.5);if(az0){g.textAlign='left';g.textBaseline='middle';g.fillStyle=isDark?'rgba(251,191,36,0.55)':'#9ca3af';g.fillText((zmn+zr*f).toFixed(1),mx+az0.x*sc+6,my-az0.y*sc);}
  }
  g.font='700 10.5px -apple-system,sans-serif';
  var lp=pj(0,-0.5,-0.5);if(lp){g.textAlign='center';g.textBaseline='top';g.fillStyle=AX;g.fillText(xl,mx+lp.x*sc,my-lp.y*sc+17);}
  lp=pj(0.5,0,-0.5);if(lp){g.textAlign='center';g.textBaseline='top';g.fillStyle=AY;g.fillText(yl,mx+lp.x*sc,my-lp.y*sc+17);}
  lp=pj(-0.5,-0.5,0);if(lp){g.textAlign='right';g.textBaseline='middle';g.fillStyle=AZ;g.fillText(zl,mx+lp.x*sc-10,my-lp.y*sc);}
  pp=[];
  if(M===0)rS(mx,my,sc);else if(M===1)rB(mx,my,sc);else rL(mx,my,sc);
  drawLgd();
  g.font='9.5px -apple-system,sans-serif';g.fillStyle=isDark?'rgba(100,116,139,0.4)':'rgba(0,0,0,0.18)';
  g.textAlign='center';g.textBaseline='bottom';
  g.fillText('drag to rotate  \xb7  scroll to zoom  \xb7  dblclick to reset',W/2,H-5);
}
function drawLgd(){
  if(CL.length===0)return;
  var lx2=W-150,ly2=36,lh=CL.length*26+18,lw=140;
  g.save();
  var bgrd=g.createLinearGradient(lx2-6,ly2-8,lx2-6+lw,ly2-8+lh);
  if(isDark){bgrd.addColorStop(0,'rgba(17,22,38,0.88)');bgrd.addColorStop(1,'rgba(11,14,24,0.92)');}
  else{bgrd.addColorStop(0,'rgba(249,250,251,0.94)');bgrd.addColorStop(1,'rgba(241,245,249,0.94)');}
  g.fillStyle=bgrd;
  var lr=10;g.beginPath();
  g.moveTo(lx2-6+lr,ly2-8);g.arcTo(lx2-6+lw,ly2-8,lx2-6+lw,ly2-8+lh,lr);
  g.arcTo(lx2-6+lw,ly2-8+lh,lx2-6,ly2-8+lh,lr);g.arcTo(lx2-6,ly2-8+lh,lx2-6,ly2-8,lr);
  g.arcTo(lx2-6,ly2-8,lx2-6+lw,ly2-8,lr);g.closePath();g.fill();
  g.strokeStyle=isDark?'rgba(255,255,255,0.07)':'rgba(0,0,0,0.06)';g.lineWidth=1;g.stroke();
  g.restore();
  g.font='11.5px -apple-system,sans-serif';g.textBaseline='middle';g.textAlign='left';
  for(var li=0;li<CL.length;li++){
    var cy2=ly2+9+li*26,col=PAL[li%PAL.length],rgb=hx2rgb(col);
    var dg2=g.createRadialGradient(lx2+9,cy2,0,lx2+9,cy2,10);
    dg2.addColorStop(0,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.45)');
    dg2.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0)');
    g.fillStyle=dg2;g.beginPath();g.arc(lx2+9,cy2,10,0,TAU);g.fill();
    g.fillStyle=col;g.beginPath();g.arc(lx2+9,cy2,5.5,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.38)';g.beginPath();g.arc(lx2+7,cy2-2,2.2,0,TAU);g.fill();
    g.fillStyle=isDark?'#e2e8f0':'#374151';g.fillText(CL[li],lx2+22,cy2);
  }
}
function rS(mx,my,sc){
  var pts=[];
  for(var i=0;i<N;i++){
    var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;
    var p=pj(nx,ny,nz);if(!p)continue;
    pts.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d,ci:uc?C[i]%PAL.length:i%PAL.length,i:i});
  }
  pts.sort(function(a,b){return a.d-b.d;});
  var dlo=pts.length?pts[0].d:1,dhi=pts.length?pts[pts.length-1].d:2,dr=dhi-dlo||1;
  for(var j=0;j<pts.length;j++){
    var p=pts[j],dn=(p.d-dlo)/dr,r=Math.max(3.5,Math.min(11,9.5-dn*5));
    var col=PAL[p.ci],rgb=hx2rgb(col);
    var br=Math.max(0.45,1-dn*0.38);
    var og=g.createRadialGradient(p.sx,p.sy,0,p.sx,p.sy,r*3.5);
    og.addColorStop(0,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+','+(0.13*br)+')');
    og.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0)');
    g.fillStyle=og;g.beginPath();g.arc(p.sx,p.sy,r*3.5,0,TAU);g.fill();
    var ig=g.createRadialGradient(p.sx,p.sy,r*0.3,p.sx,p.sy,r*1.8);
    ig.addColorStop(0,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+','+(0.5*br)+')');
    ig.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0)');
    g.fillStyle=ig;g.beginPath();g.arc(p.sx,p.sy,r*1.8,0,TAU);g.fill();
    var lr2=Math.min(255,rgb[0]+70),lg2=Math.min(255,rgb[1]+70),lb2=Math.min(255,rgb[2]+70);
    var cg=g.createRadialGradient(p.sx-r*0.3,p.sy-r*0.3,0,p.sx,p.sy,r);
    cg.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');
    cg.addColorStop(0.65,col);
    cg.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.55)');
    g.fillStyle=cg;g.beginPath();g.arc(p.sx,p.sy,r,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.6)';g.beginPath();g.arc(p.sx-r*0.3,p.sy-r*0.3,r*0.28,0,TAU);g.fill();
    pp.push({sx:p.sx,sy:p.sy,i:p.i,r:r});
  }
}
function rB(mx,my,sc){
  var bars=[];
  for(var i=0;i<N;i++){
    var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;
    var pb=pj(nx,ny,-0.5),pt=pj(nx,ny,nz);
    if(!pb||!pt)continue;
    bars.push({bx:mx+pb.x*sc,by:my-pb.y*sc,tx:mx+pt.x*sc,ty:my-pt.y*sc,d:pt.d,ci:uc?C[i]%PAL.length:i%PAL.length,i:i});
  }
  bars.sort(function(a,b){return a.d-b.d;});
  var dlo=bars.length?bars[0].d:1,dhi=bars.length?bars[bars.length-1].d:2,dr=dhi-dlo||1;
  for(var j=0;j<bars.length;j++){
    var b=bars[j],col=PAL[b.ci],rgb=hx2rgb(col);
    var dn=(b.d-dlo)/dr,bw=Math.max(2.5,Math.min(8.5,7-dn*3.5));
    g.globalAlpha=isDark?0.2:0.1;
    g.fillStyle=col;g.beginPath();g.ellipse(b.bx,b.by,bw*1.7,bw*0.52,0,0,TAU);g.fill();
    g.globalAlpha=1;
    var lr2=Math.min(255,rgb[0]+55),lg2=Math.min(255,rgb[1]+55),lb2=Math.min(255,rgb[2]+55);
    var sg=g.createLinearGradient(b.bx,b.by,b.tx,b.ty);
    sg.addColorStop(0,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.25)');
    sg.addColorStop(0.38,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.65)');
    sg.addColorStop(0.82,col);
    sg.addColorStop(1,'rgb('+lr2+','+lg2+','+lb2+')');
    g.strokeStyle=sg;g.lineWidth=bw;g.lineCap='round';
    g.shadowColor=col;g.shadowBlur=isDark?10:5;
    g.beginPath();g.moveTo(b.bx,b.by);g.lineTo(b.tx,b.ty);g.stroke();g.shadowBlur=0;
    var ox=2.5,oy=2;
    g.strokeStyle='rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.18)';g.lineWidth=bw*0.55;
    g.beginPath();g.moveTo(b.bx+ox,b.by+oy);g.lineTo(b.tx+ox,b.ty+oy);g.stroke();
    var tg2=g.createRadialGradient(b.tx,b.ty,0,b.tx,b.ty,bw*2.2);
    tg2.addColorStop(0,'rgba('+lr2+','+lg2+','+lb2+',0.95)');
    tg2.addColorStop(0.45,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.45)');
    tg2.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0)');
    g.fillStyle=tg2;g.beginPath();g.arc(b.tx,b.ty,bw*2.2,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.65)';g.beginPath();g.arc(b.tx-bw*0.22,b.ty-bw*0.22,bw*0.38,0,TAU);g.fill();
    pp.push({sx:b.tx,sy:b.ty,i:b.i,r:bw+2});
  }
}
function rL(mx,my,sc){
  var pts=[];
  for(var i=0;i<N;i++){
    var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;
    var p=pj(nx,ny,nz);if(!p)continue;
    pts.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d,ci:uc?C[i]%PAL.length:0,i:i});
  }
  if(pts.length<2){
    if(pts.length===1)pp.push({sx:pts[0].sx,sy:pts[0].sy,i:pts[0].i,r:6});
    return;
  }
  for(var pass=0;pass<3;pass++){
    var lw2=[14,5,1.8][pass],al=[0.1,0.42,1][pass];
    g.save();g.globalAlpha=al;g.lineJoin='round';g.lineCap='round';g.lineWidth=lw2;
    for(var j=1;j<pts.length;j++){
      var p0=pts[j-1],p1=pts[j];
      var c0=PAL[p0.ci],c1=PAL[p1.ci];
      var r0=hx2rgb(c0),r1=hx2rgb(c1);
      var bo=pass===2?55:0;
      var lg2=g.createLinearGradient(p0.sx,p0.sy,p1.sx,p1.sy);
      lg2.addColorStop(0,'rgba('+Math.min(255,r0[0]+bo)+','+Math.min(255,r0[1]+bo)+','+Math.min(255,r0[2]+bo)+',1)');
      lg2.addColorStop(1,'rgba('+Math.min(255,r1[0]+bo)+','+Math.min(255,r1[1]+bo)+','+Math.min(255,r1[2]+bo)+',1)');
      g.strokeStyle=lg2;
      g.beginPath();g.moveTo(p0.sx,p0.sy);g.lineTo(p1.sx,p1.sy);g.stroke();
    }
    g.restore();
  }
  for(var j=0;j<pts.length;j++){
    var p=pts[j],col=PAL[p.ci],rgb=hx2rgb(col),r=5.5;
    var lr2=Math.min(255,rgb[0]+80),lg3=Math.min(255,rgb[1]+80),lb2=Math.min(255,rgb[2]+80);
    var ng=g.createRadialGradient(p.sx,p.sy,0,p.sx,p.sy,r*2.5);
    ng.addColorStop(0,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.32)');
    ng.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0)');
    g.fillStyle=ng;g.beginPath();g.arc(p.sx,p.sy,r*2.5,0,TAU);g.fill();
    var cg=g.createRadialGradient(p.sx-r*0.32,p.sy-r*0.32,0,p.sx,p.sy,r);
    cg.addColorStop(0,'rgb('+lr2+','+lg3+','+lb2+')');cg.addColorStop(1,col);
    g.fillStyle=cg;g.beginPath();g.arc(p.sx,p.sy,r,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.58)';g.beginPath();g.arc(p.sx-r*0.3,p.sy-r*0.3,r*0.3,0,TAU);g.fill();
    pp.push({sx:p.sx,sy:p.sy,i:p.i,r:r});
  }
}
function ht(ex,ey){var bi=-1,bd=900;for(var i=pp.length-1;i>=0;i--){var dx=pp[i].sx-ex,dy=pp[i].sy-ey,d2=dx*dx+dy*dy,hr=(pp[i].r+10)*(pp[i].r+10);if(d2<bd&&d2<hr){bd=d2;bi=pp[i].i;}}return bi;}
function sT(idx,ex,ey){
  var lbl=CL.length>0&&uc?CL[C[idx]%CL.length]:'Point '+(idx+1);
  var h='<b>'+lbl+'</b>';
  h+='<span>'+xl+':</span> <span class="tv">'+X[idx].toFixed(3)+'</span><br>';
  h+='<span>'+yl+':</span> <span class="tv">'+Y[idx].toFixed(3)+'</span><br>';
  h+='<span>'+zl+':</span> <span class="tv">'+Z[idx].toFixed(3)+'</span>';
  tip.innerHTML=h;tip.className='c3t v'+(pin?' p':'');
  var bx=wrap.getBoundingClientRect();
  var tx=ex-bx.left+16,ty=ey-bx.top-14;
  if(tx+175>W)tx=ex-bx.left-182;if(ty<0)ty=ey-bx.top+22;
  tip.style.left=tx+'px';tip.style.top=ty+'px';
}
function hT(){if(!pin){tip.className='c3t';}}
R();
wrap.addEventListener('mousedown',function(e){dg=true;mv=false;lx=e.clientX;ly=e.clientY;dwX=e.clientX;dwY=e.clientY;e.preventDefault();});
window.addEventListener('mousemove',function(e){if(dg){var dx=e.clientX-lx,dy=e.clientY-ly;if(Math.abs(e.clientX-dwX)>3||Math.abs(e.clientY-dwY)>3)mv=true;yaw+=dx*0.008;pitch=Math.max(-1.47,Math.min(1.47,pitch+dy*0.008));lx=e.clientX;ly=e.clientY;R();return;}if(pin)return;var bx=wrap.getBoundingClientRect(),ex=e.clientX-bx.left,ey=e.clientY-bx.top;if(ex<0||ey<0||ex>W||ey>H){hT();return;}var idx=ht(ex,ey);if(idx>=0)sT(idx,e.clientX,e.clientY);else hT();});
window.addEventListener('mouseup',function(e){if(!dg)return;dg=false;if(!mv){var bx=wrap.getBoundingClientRect(),ex=e.clientX-bx.left,ey=e.clientY-bx.top;var idx=ht(ex,ey);if(idx>=0){pin=true;piI=idx;sT(idx,e.clientX,e.clientY);}else{pin=false;piI=-1;tip.className='c3t';}}});
wrap.addEventListener('wheel',function(e){zoom=Math.max(0.3,Math.min(5,zoom*(e.deltaY>0?1.08:0.93)));R();e.preventDefault();},{passive:false});
wrap.addEventListener('dblclick',function(){yaw=0.785;pitch=0.6;zoom=1.0;pin=false;piI=-1;tip.className='c3t';R();});
wrap.addEventListener('mouseleave',function(){if(!pin)hT();});
document.addEventListener('keydown',function(e){if(e.key==='Escape'&&pin){pin=false;piI=-1;tip.className='c3t';}});
wrap.addEventListener('touchstart',function(e){if(e.touches.length===1){dg=true;mv=false;lx=e.touches[0].clientX;ly=e.touches[0].clientY;dwX=lx;dwY=ly;e.preventDefault();}},{passive:false});
wrap.addEventListener('touchmove',function(e){if(!dg||e.touches.length!==1)return;var dx=e.touches[0].clientX-lx,dy=e.touches[0].clientY-ly;if(Math.abs(e.touches[0].clientX-dwX)>3||Math.abs(e.touches[0].clientY-dwY)>3)mv=true;yaw+=dx*0.008;pitch=Math.max(-1.47,Math.min(1.47,pitch+dy*0.008));lx=e.touches[0].clientX;ly=e.touches[0].clientY;R();e.preventDefault();},{passive:false});
wrap.addEventListener('touchend',function(){dg=false;if(!mv){var bx=wrap.getBoundingClientRect(),ex=dwX-bx.left,ey=dwY-bx.top;var idx=ht(ex,ey);if(idx>=0){pin=true;piI=idx;sT(idx,dwX,dwY);}else{pin=false;piI=-1;tip.className='c3t';}}});
"##;
