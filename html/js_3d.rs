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
var dg=false,lx=0,ly=0,mv=false,dwX=0,dwY=0,raf=0;
var cv=document.getElementById(cid+'c'),g=cv.getContext('2d'),wrap=document.getElementById(cid),tip=document.getElementById(cid+'t');
var dpr=window.devicePixelRatio||1;cv.width=W*dpr;cv.height=H*dpr;g.scale(dpr,dpr);
var pin=false,piI=-1,pp=[];
var AX='#f472b6',AY='#22d3ee',AZ='#fbbf24';
var autoR=false,velY=0,velP=0,panX=0,panY=0,keys={};
var _rc={};
function hx2rgb(h){if(_rc[h])return _rc[h];var r=[parseInt(h.slice(1,3),16),parseInt(h.slice(3,5),16),parseInt(h.slice(5,7),16)];_rc[h]=r;return r;}
function pj(px,py,pz){var ex=zoom*Math.cos(yaw)*Math.cos(pitch),ey=zoom*Math.sin(yaw)*Math.cos(pitch),ez=zoom*Math.sin(pitch);var fx=-ex,fy=-ey,fz=-ez,fl=Math.sqrt(fx*fx+fy*fy+fz*fz);fx/=fl;fy/=fl;fz/=fl;var rx=-fy,ry=fx,rz=0,rl=Math.sqrt(rx*rx+ry*ry)||1e-6;rx/=rl;ry/=rl;var u2x=fy*rz-fz*ry,u2y=fz*rx-fx*rz,u2z=fx*ry-fy*rx;var dx=px-ex,dy=py-ey,dz=pz-ez;var dp=dx*fx+dy*fy+dz*fz;if(dp<0.001)return null;var cx2=dx*rx+dy*ry+dz*rz,cy2=dx*u2x+dy*u2y+dz*u2z;var th=Math.tan(fov/2),asp=W/H;return{x:cx2/(dp*th*asp)+panX/sc,y:cy2/(dp*th)+panY/sc,d:dp};}
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
  g.strokeStyle=isDark?'rgba(99,102,241,0.055)':'rgba(0,0,0,0.04)';g.lineWidth=0.5;
  for(var q=0;q<=5;q++){var f=q/5-0.5;
    var p0=pj(f,-0.5,-0.5),p1=pj(f,0.5,-0.5);
    if(p0&&p1){g.beginPath();g.moveTo(mx+p0.x*sc,my-p0.y*sc);g.lineTo(mx+p1.x*sc,my-p1.y*sc);g.stroke();}
    var q0=pj(-0.5,f,-0.5),q1=pj(0.5,f,-0.5);
    if(q0&&q1){g.beginPath();g.moveTo(mx+q0.x*sc,my-q0.y*sc);g.lineTo(mx+q1.x*sc,my-q1.y*sc);g.stroke();}
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
  if(M===0)rS(mx,my,sc);else if(M===1)rB(mx,my,sc);else if(M===2)rL(mx,my,sc);else if(M===3)rRdr(mx,my,sc);else if(M===4)rLol(mx,my,sc);else if(M===5)rKde(mx,my,sc);else rRdg(mx,my,sc);
  drawLgd();
  g.font='9.5px -apple-system,sans-serif';g.fillStyle=isDark?'rgba(100,116,139,0.4)':'rgba(0,0,0,0.18)';
  g.textAlign='center';g.textBaseline='bottom';
  g.fillText('drag \xb7 scroll \xb7 dblclick reset \xb7 arrows \xb7 A=auto-rotate \xb7 shift+drag=pan',W/2,H-5);
  if(autoR){g.fillStyle='#6366F1';g.globalAlpha=0.55;g.fillText('\u27f3 auto',W-45,H-5);g.globalAlpha=1;}
}
function drawLgd(){
  if(CL.length===0)return;
  var lx2=W-150,ly2=36,lh=CL.length*22+16,lw=140;
  g.save();
  g.fillStyle=isDark?'rgba(14,18,30,0.88)':'rgba(249,250,251,0.94)';
  var lr=8;g.beginPath();
  g.moveTo(lx2-6+lr,ly2-6);g.arcTo(lx2-6+lw,ly2-6,lx2-6+lw,ly2-6+lh,lr);
  g.arcTo(lx2-6+lw,ly2-6+lh,lx2-6,ly2-6+lh,lr);g.arcTo(lx2-6,ly2-6+lh,lx2-6,ly2-6,lr);
  g.arcTo(lx2-6,ly2-6,lx2-6+lw,ly2-6,lr);g.closePath();g.fill();
  g.strokeStyle=isDark?'rgba(255,255,255,0.07)':'rgba(0,0,0,0.06)';g.lineWidth=1;g.stroke();
  g.restore();
  g.font='11px -apple-system,sans-serif';g.textBaseline='middle';g.textAlign='left';
  for(var li=0;li<CL.length;li++){
    var cy2=ly2+7+li*22,col=PAL[li%PAL.length];
    g.fillStyle=col;g.beginPath();g.arc(lx2+9,cy2,5,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.5)';g.beginPath();g.arc(lx2+7.5,cy2-1.5,1.8,0,TAU);g.fill();
    g.fillStyle=isDark?'#e2e8f0':'#374151';g.fillText(CL[li],lx2+20,cy2);
  }
}
function drawHalo(sx,sy,r,col){
  var rgb=hx2rgb(col);
  var og=g.createRadialGradient(sx,sy,r*0.8,sx,sy,r*3.2);
  og.addColorStop(0,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.35)');
  og.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0)');
  g.fillStyle=og;g.beginPath();g.arc(sx,sy,r*3.2,0,TAU);g.fill();
  g.strokeStyle='rgba(255,255,255,0.75)';g.lineWidth=1.5;
  g.beginPath();g.arc(sx,sy,r+2.5,0,TAU);g.stroke();
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
  var selSx=0,selSy=0,selR=0,selCol='';
  for(var j=0;j<pts.length;j++){
    var p=pts[j],dn=(p.d-dlo)/dr,r=Math.max(3,Math.min(8,7.5-dn*4));
    var col=PAL[p.ci],rgb=hx2rgb(col);
    var lr2=Math.min(255,rgb[0]+60),lg2=Math.min(255,rgb[1]+60),lb2=Math.min(255,rgb[2]+60);
    var cg=g.createRadialGradient(p.sx-r*0.28,p.sy-r*0.28,0,p.sx,p.sy,r);
    cg.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');
    cg.addColorStop(0.7,col);
    cg.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.6)');
    g.fillStyle=cg;g.beginPath();g.arc(p.sx,p.sy,r,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.55)';g.beginPath();g.arc(p.sx-r*0.28,p.sy-r*0.28,r*0.25,0,TAU);g.fill();
    pp.push({sx:p.sx,sy:p.sy,i:p.i,r:r});
    if(p.i===piI){selSx=p.sx;selSy=p.sy;selR=r;selCol=col;}
  }
  if(piI>=0&&selCol)drawHalo(selSx,selSy,selR,selCol);
}
function rB(mx,my,sc){
  var bars=[];
  for(var i=0;i<N;i++){
    var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;
    var pb=pj(nx,ny,-0.5),pt=pj(nx,ny,nz);
    if(!pb||!pt)continue;
    bars.push({bx:mx+pb.x*sc,by:my-pb.y*sc,tx:mx+pt.x*sc,ty:my-pt.y*sc,d:pt.d,ci:uc?C[i]%PAL.length:i%PAL.length,i:i,nz:nz});
  }
  bars.sort(function(a,b){return a.d-b.d;});
  var dlo=bars.length?bars[0].d:1,dhi=bars.length?bars[bars.length-1].d:2,dr=dhi-dlo||1;
  var selB=null;
  for(var j=0;j<bars.length;j++){
    var b=bars[j],col=PAL[b.ci],rgb=hx2rgb(col);
    var dn=(b.d-dlo)/dr,bw=Math.max(3,Math.min(9,8-dn*4));
    g.globalAlpha=isDark?0.18:0.1;
    g.fillStyle=col;g.beginPath();g.ellipse(b.bx,b.by,bw*2,bw*0.6,0,0,TAU);g.fill();
    g.globalAlpha=1;
    var lr2=Math.min(255,rgb[0]+50),lg2=Math.min(255,rgb[1]+50),lb2=Math.min(255,rgb[2]+50);
    var dr2=Math.max(0,rgb[0]-30),dg2=Math.max(0,rgb[1]-30),db2=Math.max(0,rgb[2]-30);
    var offX=bw*0.3,offY=0;
    g.strokeStyle='rgba('+dr2+','+dg2+','+db2+',0.5)';g.lineWidth=bw+2;g.lineCap='round';
    g.beginPath();g.moveTo(b.bx+offX,b.by+offY);g.lineTo(b.tx+offX,b.ty+offY);g.stroke();
    var sg=g.createLinearGradient(b.bx-bw,b.by,b.tx+bw,b.ty);
    sg.addColorStop(0,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.4)');
    sg.addColorStop(0.4,col);
    sg.addColorStop(0.8,'rgb('+lr2+','+lg2+','+lb2+')');
    sg.addColorStop(1,'rgb('+Math.min(255,rgb[0]+80)+','+Math.min(255,rgb[1]+80)+','+Math.min(255,rgb[2]+80)+')');
    g.strokeStyle=sg;g.lineWidth=bw;g.lineCap='round';
    g.beginPath();g.moveTo(b.bx,b.by);g.lineTo(b.tx,b.ty);g.stroke();
    g.fillStyle='rgb('+lr2+','+lg2+','+lb2+')';g.beginPath();g.arc(b.tx,b.ty,bw*0.6,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.65)';g.beginPath();g.arc(b.tx-bw*0.2,b.ty-bw*0.2,bw*0.3,0,TAU);g.fill();
    pp.push({sx:b.tx,sy:b.ty,i:b.i,r:bw+2});
    if(b.i===piI)selB=b;
  }
  if(piI>=0&&selB)drawHalo(selB.tx,selB.ty,4,PAL[selB.ci]);
}
function rL(mx,my,sc){
  var pts=[];
  for(var i=0;i<N;i++){
    var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;
    var p=pj(nx,ny,nz);if(!p)continue;
    pts.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d,ci:uc?C[i]%PAL.length:0,i:i});
  }
  if(pts.length<2){
    if(pts.length===1)pp.push({sx:pts[0].sx,sy:pts[0].sy,i:pts[0].i,r:5});
    return;
  }
  for(var pass=0;pass<2;pass++){
    var lw2=pass===0?5:2,al=pass===0?0.18:1;
    g.save();g.globalAlpha=al;g.lineJoin='round';g.lineCap='round';g.lineWidth=lw2;
    for(var j=1;j<pts.length;j++){
      var p0=pts[j-1],p1=pts[j];
      var r0=hx2rgb(PAL[p0.ci]),r1=hx2rgb(PAL[p1.ci]);
      var bo=pass===1?45:0;
      var lg2=g.createLinearGradient(p0.sx,p0.sy,p1.sx,p1.sy);
      lg2.addColorStop(0,'rgba('+Math.min(255,r0[0]+bo)+','+Math.min(255,r0[1]+bo)+','+Math.min(255,r0[2]+bo)+',1)');
      lg2.addColorStop(1,'rgba('+Math.min(255,r1[0]+bo)+','+Math.min(255,r1[1]+bo)+','+Math.min(255,r1[2]+bo)+',1)');
      g.strokeStyle=lg2;g.beginPath();g.moveTo(p0.sx,p0.sy);g.lineTo(p1.sx,p1.sy);g.stroke();
    }
    g.restore();
  }
  var selP=null;
  for(var j=0;j<pts.length;j++){
    var p=pts[j],col=PAL[p.ci],rgb=hx2rgb(col),r=4.5;
    var lr2=Math.min(255,rgb[0]+70),lg3=Math.min(255,rgb[1]+70),lb2=Math.min(255,rgb[2]+70);
    g.fillStyle='rgb('+lr2+','+lg3+','+lb2+')';g.beginPath();g.arc(p.sx,p.sy,r,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.52)';g.beginPath();g.arc(p.sx-r*0.28,p.sy-r*0.28,r*0.28,0,TAU);g.fill();
    pp.push({sx:p.sx,sy:p.sy,i:p.i,r:r});
    if(p.i===piI){selP=p;}
  }
  if(piI>=0&&selP)drawHalo(selP.sx,selP.sy,4.5,PAL[selP.ci]);
}
function rRdr(mx,my,sc){
  var groups={};
  for(var i=0;i<N;i++){var ci=uc?C[i]%PAL.length:0;if(!groups[ci])groups[ci]=[];groups[ci].push(i);}
  var gkeys=Object.keys(groups).sort(function(a,b){return parseInt(a)-parseInt(b);});
  for(var gi=0;gi<gkeys.length;gi++){
    var ci=parseInt(gkeys[gi]),idxs=groups[ci];
    idxs.sort(function(a,b){return Math.atan2(Z[a],X[a])-Math.atan2(Z[b],X[b]);});
    var col=PAL[ci%PAL.length],rgb=hx2rgb(col);
    var pts2d=[],bpts=[];
    for(var k=0;k<idxs.length;k++){
      var ii=idxs[k];
      var nx=(X[ii]-xmn)/xr-0.5,ny=(Y[ii]-ymn)/yr-0.5,nz=(Z[ii]-zmn)/zr-0.5;
      var p=pj(nx,ny,nz);if(p)pts2d.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d,ii:ii});
      var pb=pj(nx,-0.5,nz);if(pb)bpts.push({sx:mx+pb.x*sc,sy:my-pb.y*sc});
    }
    if(pts2d.length<3)continue;
    if(bpts.length===pts2d.length){
      g.globalAlpha=0.06;g.beginPath();g.moveTo(bpts[0].sx,bpts[0].sy);
      for(var k=1;k<bpts.length;k++)g.lineTo(bpts[k].sx,bpts[k].sy);
      g.closePath();g.fillStyle=col;g.fill();g.globalAlpha=1;
      for(var k=0;k<pts2d.length;k++){
        g.strokeStyle='rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.18)';g.lineWidth=1;
        g.beginPath();g.moveTo(bpts[k].sx,bpts[k].sy);g.lineTo(pts2d[k].sx,pts2d[k].sy);g.stroke();
      }
    }
    var lr2=Math.min(255,rgb[0]+40),lg2=Math.min(255,rgb[1]+40),lb2=Math.min(255,rgb[2]+40);
    g.beginPath();g.moveTo(pts2d[0].sx,pts2d[0].sy);
    for(var k=1;k<pts2d.length;k++)g.lineTo(pts2d[k].sx,pts2d[k].sy);
    g.closePath();
    var minY2=1e9,maxY2=-1e9;for(var k=0;k<pts2d.length;k++){if(pts2d[k].sy<minY2)minY2=pts2d[k].sy;if(pts2d[k].sy>maxY2)maxY2=pts2d[k].sy;}
    var fg=g.createLinearGradient(0,minY2,0,maxY2);
    fg.addColorStop(0,'rgba('+lr2+','+lg2+','+lb2+',0.38)');
    fg.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.12)');
    g.fillStyle=fg;g.fill();
    g.save();g.globalAlpha=0.15;g.strokeStyle=col;g.lineWidth=8;g.stroke();g.restore();
    g.strokeStyle=col;g.lineWidth=2.5;g.stroke();
    g.strokeStyle='rgba('+Math.min(255,rgb[0]+80)+','+Math.min(255,rgb[1]+80)+','+Math.min(255,rgb[2]+80)+',0.5)';
    g.lineWidth=1;g.stroke();
    for(var k=0;k<pts2d.length;k++){
      var sr=5;
      var cg=g.createRadialGradient(pts2d[k].sx-sr*0.25,pts2d[k].sy-sr*0.25,0,pts2d[k].sx,pts2d[k].sy,sr);
      cg.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');cg.addColorStop(0.7,col);
      cg.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.6)');
      g.fillStyle=cg;g.beginPath();g.arc(pts2d[k].sx,pts2d[k].sy,sr,0,TAU);g.fill();
      g.fillStyle='rgba(255,255,255,0.5)';g.beginPath();g.arc(pts2d[k].sx-sr*0.25,pts2d[k].sy-sr*0.25,sr*0.25,0,TAU);g.fill();
      pp.push({sx:pts2d[k].sx,sy:pts2d[k].sy,i:idxs[k],r:sr+2});
    }
  }
}
function rLol(mx,my,sc){
  var lols=[];
  for(var i=0;i<N;i++){
    var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;
    var pb=pj(nx,ny,-0.5),pt=pj(nx,ny,nz);
    if(!pb||!pt)continue;
    lols.push({bx:mx+pb.x*sc,by:my-pb.y*sc,tx:mx+pt.x*sc,ty:my-pt.y*sc,d:pt.d,ci:uc?C[i]%PAL.length:i%PAL.length,i:i});
  }
  lols.sort(function(a,b){return a.d-b.d;});
  var selL=null;
  for(var j=0;j<lols.length;j++){
    var l=lols[j],col=PAL[l.ci],rgb=hx2rgb(col);
    var lr2=Math.min(255,rgb[0]+60),lg2=Math.min(255,rgb[1]+60),lb2=Math.min(255,rgb[2]+60);
    g.strokeStyle='rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.2)';g.lineWidth=4;g.lineCap='round';
    g.beginPath();g.moveTo(l.bx,l.by);g.lineTo(l.tx,l.ty);g.stroke();
    g.strokeStyle=col;g.lineWidth=1.8;g.lineCap='round';
    g.beginPath();g.moveTo(l.bx,l.by);g.lineTo(l.tx,l.ty);g.stroke();
    var sr=8;
    var cg=g.createRadialGradient(l.tx-sr*0.3,l.ty-sr*0.3,0,l.tx,l.ty,sr);
    cg.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');cg.addColorStop(0.65,col);cg.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.5)');
    g.fillStyle=cg;g.beginPath();g.arc(l.tx,l.ty,sr,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.55)';g.beginPath();g.arc(l.tx-sr*0.3,l.ty-sr*0.3,sr*0.3,0,TAU);g.fill();
    pp.push({sx:l.tx,sy:l.ty,i:l.i,r:sr+2});
    if(l.i===piI)selL=l;
  }
  if(piI>=0&&selL)drawHalo(selL.tx,selL.ty,8,PAL[selL.ci]);
}
function rKde(mx,my,sc){
  var ymap={};
  for(var i=0;i<N;i++){var yk=Math.round(Y[i]*1000)/1000;if(!ymap[yk])ymap[yk]=[];ymap[yk].push(i);}
  var ykeys=Object.keys(ymap).sort(function(a,b){return parseFloat(a)-parseFloat(b);});
  var strips=[];
  for(var gi=0;gi<ykeys.length;gi++){
    var idxs=ymap[ykeys[gi]];idxs.sort(function(a,b){return X[a]-X[b];});
    var col=PAL[gi%PAL.length],rgb=hx2rgb(col);
    var pts2d=[],pBase=[];
    for(var k=0;k<idxs.length;k++){
      var ii=idxs[k];
      var nx=(X[ii]-xmn)/xr-0.5,ny=(Y[ii]-ymn)/yr-0.5,nz=(Z[ii]-zmn)/zr-0.5;
      var p=pj(nx,ny,nz);if(p)pts2d.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d});
      var pb2=pj(nx,ny,-0.5);if(pb2)pBase.push({sx:mx+pb2.x*sc,sy:my-pb2.y*sc,d:pb2.d});
    }
    if(pts2d.length<2)continue;
    var avgD=0;for(var k=0;k<pts2d.length;k++)avgD+=pts2d[k].d;avgD/=pts2d.length;
    strips.push({pts:pts2d,base:pBase,col:col,rgb:rgb,gi:gi,avgD:avgD});
  }
  strips.sort(function(a,b){return a.avgD-b.avgD;});
  for(var si=0;si<strips.length;si++){
    var s=strips[si],pts2d=s.pts,pBase=s.base,col=s.col,rgb=s.rgb;
    if(pBase.length===pts2d.length){
      g.beginPath();g.moveTo(pBase[0].sx,pBase[0].sy);g.lineTo(pts2d[0].sx,pts2d[0].sy);
      g.lineTo(pts2d[0].sx,pts2d[0].sy);g.lineTo(pBase[0].sx,pBase[0].sy);
      g.closePath();g.fillStyle='rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.15)';g.fill();
      var last=pts2d.length-1;
      g.beginPath();g.moveTo(pBase[last].sx,pBase[last].sy);g.lineTo(pts2d[last].sx,pts2d[last].sy);
      g.lineTo(pts2d[last].sx,pts2d[last].sy);g.lineTo(pBase[last].sx,pBase[last].sy);
      g.closePath();g.fillStyle='rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.12)';g.fill();
      g.beginPath();g.moveTo(pBase[0].sx,pBase[0].sy);
      for(var k=1;k<pBase.length;k++)g.lineTo(pBase[k].sx,pBase[k].sy);
      for(var k=pts2d.length-1;k>=0;k--)g.lineTo(pts2d[k].sx,pts2d[k].sy);
      g.closePath();
      var minY2=1e9,maxY2=-1e9;
      for(var k=0;k<pts2d.length;k++){if(pts2d[k].sy<minY2)minY2=pts2d[k].sy;if(pts2d[k].sy>maxY2)maxY2=pts2d[k].sy;}
      for(var k=0;k<pBase.length;k++){if(pBase[k].sy>maxY2)maxY2=pBase[k].sy;}
      var fg=g.createLinearGradient(0,minY2,0,maxY2);
      var lr2=Math.min(255,rgb[0]+50),lg2=Math.min(255,rgb[1]+50),lb2=Math.min(255,rgb[2]+50);
      fg.addColorStop(0,'rgba('+lr2+','+lg2+','+lb2+',0.5)');
      fg.addColorStop(0.5,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.3)');
      fg.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.08)');
      g.fillStyle=fg;g.fill();
    }
    g.save();g.globalAlpha=0.12;g.strokeStyle=col;g.lineWidth=7;g.lineCap='round';g.lineJoin='round';
    g.beginPath();g.moveTo(pts2d[0].sx,pts2d[0].sy);
    for(var k=1;k<pts2d.length;k++)g.lineTo(pts2d[k].sx,pts2d[k].sy);g.stroke();g.restore();
    g.strokeStyle=col;g.lineWidth=3;g.lineCap='round';g.lineJoin='round';
    g.beginPath();g.moveTo(pts2d[0].sx,pts2d[0].sy);
    for(var k=1;k<pts2d.length;k++)g.lineTo(pts2d[k].sx,pts2d[k].sy);g.stroke();
    var hr2=Math.min(255,rgb[0]+90),hg2=Math.min(255,rgb[1]+90),hb2=Math.min(255,rgb[2]+90);
    g.strokeStyle='rgba('+hr2+','+hg2+','+hb2+',0.55)';g.lineWidth=1;
    g.beginPath();g.moveTo(pts2d[0].sx,pts2d[0].sy);
    for(var k=1;k<pts2d.length;k++)g.lineTo(pts2d[k].sx,pts2d[k].sy);g.stroke();
  }
}
function rRdg(mx,my,sc){
  var ymap={};
  for(var i=0;i<N;i++){var yk=Math.round(Y[i]*1000)/1000;if(!ymap[yk])ymap[yk]=[];ymap[yk].push(i);}
  var ykeys=Object.keys(ymap).sort(function(a,b){return parseFloat(a)-parseFloat(b);});
  var strips=[];
  for(var gi=0;gi<ykeys.length;gi++){
    var idxs=ymap[ykeys[gi]];idxs.sort(function(a,b){return X[a]-X[b];});
    var col=PAL[gi%PAL.length],rgb=hx2rgb(col);
    var pts2d=[],pBase=[];
    for(var k=0;k<idxs.length;k++){
      var ii=idxs[k];
      var nx=(X[ii]-xmn)/xr-0.5,ny=(Y[ii]-ymn)/yr-0.5,nz=(Z[ii]-zmn)/zr-0.5;
      var p=pj(nx,ny,nz);if(p)pts2d.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d});
      var pb2=pj(nx,ny,-0.5);if(pb2)pBase.push({sx:mx+pb2.x*sc,sy:my-pb2.y*sc,d:pb2.d});
    }
    if(pts2d.length<2)continue;
    var avgD=0;for(var k=0;k<pts2d.length;k++)avgD+=pts2d[k].d;avgD/=pts2d.length;
    strips.push({pts:pts2d,base:pBase,col:col,rgb:rgb,gi:gi,avgD:avgD});
  }
  strips.sort(function(a,b){return a.avgD-b.avgD;});
  for(var si=0;si<strips.length;si++){
    var s=strips[si],pts2d=s.pts,pBase=s.base,col=s.col,rgb=s.rgb;
    if(pBase.length===pts2d.length){
      g.beginPath();g.moveTo(pBase[0].sx,pBase[0].sy);
      for(var k=1;k<pBase.length;k++)g.lineTo(pBase[k].sx,pBase[k].sy);
      for(var k=pts2d.length-1;k>=0;k--)g.lineTo(pts2d[k].sx,pts2d[k].sy);
      g.closePath();
      g.fillStyle=isDark?'rgba(13,17,23,0.88)':'rgba(248,250,252,0.88)';g.fill();
      g.beginPath();g.moveTo(pBase[0].sx,pBase[0].sy);
      for(var k=1;k<pBase.length;k++)g.lineTo(pBase[k].sx,pBase[k].sy);
      for(var k=pts2d.length-1;k>=0;k--)g.lineTo(pts2d[k].sx,pts2d[k].sy);
      g.closePath();
      var minY2=1e9,maxY2=-1e9;
      for(var k=0;k<pts2d.length;k++){if(pts2d[k].sy<minY2)minY2=pts2d[k].sy;if(pts2d[k].sy>maxY2)maxY2=pts2d[k].sy;}
      for(var k=0;k<pBase.length;k++){if(pBase[k].sy>maxY2)maxY2=pBase[k].sy;}
      var fg=g.createLinearGradient(0,minY2,0,maxY2);
      var lr2=Math.min(255,rgb[0]+60),lg2=Math.min(255,rgb[1]+60),lb2=Math.min(255,rgb[2]+60);
      fg.addColorStop(0,'rgba('+lr2+','+lg2+','+lb2+',0.48)');
      fg.addColorStop(0.4,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.35)');
      fg.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.1)');
      g.fillStyle=fg;g.fill();
      g.beginPath();g.moveTo(pBase[0].sx,pBase[0].sy);g.lineTo(pts2d[0].sx,pts2d[0].sy);
      g.moveTo(pBase[pBase.length-1].sx,pBase[pBase.length-1].sy);g.lineTo(pts2d[pts2d.length-1].sx,pts2d[pts2d.length-1].sy);
      g.strokeStyle='rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.25)';g.lineWidth=1.5;g.stroke();
    }
    g.save();g.globalAlpha=0.1;g.strokeStyle=col;g.lineWidth=8;g.lineCap='round';g.lineJoin='round';
    g.beginPath();g.moveTo(pts2d[0].sx,pts2d[0].sy);
    for(var k=1;k<pts2d.length;k++)g.lineTo(pts2d[k].sx,pts2d[k].sy);g.stroke();g.restore();
    g.strokeStyle=col;g.lineWidth=3;g.lineCap='round';g.lineJoin='round';
    g.beginPath();g.moveTo(pts2d[0].sx,pts2d[0].sy);
    for(var k=1;k<pts2d.length;k++)g.lineTo(pts2d[k].sx,pts2d[k].sy);g.stroke();
    var hr2=Math.min(255,rgb[0]+90),hg2=Math.min(255,rgb[1]+90),hb2=Math.min(255,rgb[2]+90);
    g.strokeStyle='rgba('+hr2+','+hg2+','+hb2+',0.5)';g.lineWidth=1;
    g.beginPath();g.moveTo(pts2d[0].sx,pts2d[0].sy);
    for(var k=1;k<pts2d.length;k++)g.lineTo(pts2d[k].sx,pts2d[k].sy);g.stroke();
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
var shiftDrag=false;
function tick(){
  var dirty=false;
  if(autoR&&!dg){yaw+=0.004;dirty=true;}
  if(Math.abs(velY)>0.0003||Math.abs(velP)>0.0003){
    yaw+=velY;pitch=Math.max(-1.47,Math.min(1.47,pitch+velP));
    velY*=0.95;velP*=0.95;dirty=true;
  }
  if(keys.ArrowLeft){yaw-=0.025;dirty=true;}
  if(keys.ArrowRight){yaw+=0.025;dirty=true;}
  if(keys.ArrowUp){pitch=Math.min(1.47,pitch+0.025);dirty=true;}
  if(keys.ArrowDown){pitch=Math.max(-1.47,pitch-0.025);dirty=true;}
  if(dirty)R();
  requestAnimationFrame(tick);
}
R();requestAnimationFrame(tick);
wrap.addEventListener('mousedown',function(e){
  if(e.shiftKey){shiftDrag=true;dg=true;mv=false;lx=e.clientX;ly=e.clientY;e.preventDefault();return;}
  dg=true;mv=false;lx=e.clientX;ly=e.clientY;dwX=e.clientX;dwY=e.clientY;velY=0;velP=0;e.preventDefault();
});
window.addEventListener('mousemove',function(e){
  if(dg&&shiftDrag){
    panX+=(e.clientX-lx);panY+=(e.clientY-ly);lx=e.clientX;ly=e.clientY;R();return;
  }
  if(dg){
    var dx=e.clientX-lx,dy=e.clientY-ly;
    if(Math.abs(e.clientX-dwX)>3||Math.abs(e.clientY-dwY)>3)mv=true;
    velY=dx*0.008;velP=dy*0.008;
    yaw+=velY;pitch=Math.max(-1.47,Math.min(1.47,pitch+velP));
    lx=e.clientX;ly=e.clientY;
    R();return;
  }
  if(pin)return;
  var bx=wrap.getBoundingClientRect(),ex=e.clientX-bx.left,ey=e.clientY-bx.top;
  if(ex<0||ey<0||ex>W||ey>H){hT();return;}
  var idx=ht(ex,ey);if(idx>=0)sT(idx,e.clientX,e.clientY);else hT();
});
window.addEventListener('mouseup',function(e){
  if(!dg)return;var wasSh=shiftDrag;dg=false;shiftDrag=false;
  if(wasSh)return;
  if(!mv){var bx=wrap.getBoundingClientRect(),ex=e.clientX-bx.left,ey=e.clientY-bx.top;var idx=ht(ex,ey);if(idx>=0){pin=true;piI=idx;sT(idx,e.clientX,e.clientY);R();}else{pin=false;piI=-1;tip.className='c3t';R();}}
});
wrap.addEventListener('wheel',function(e){zoom=Math.max(0.3,Math.min(5,zoom*(e.deltaY>0?1.08:0.93)));R();e.preventDefault();},{passive:false});
wrap.addEventListener('dblclick',function(){yaw=0.785;pitch=0.6;zoom=1.0;panX=0;panY=0;pin=false;piI=-1;tip.className='c3t';velY=0;velP=0;R();});
wrap.addEventListener('mouseleave',function(){if(!pin)hT();});
document.addEventListener('keydown',function(e){
  keys[e.key]=true;
  if(e.key==='Escape'){if(pin){pin=false;piI=-1;tip.className='c3t';R();}}
  if(e.key==='a'||e.key==='A'){autoR=!autoR;R();}
  if(e.key==='='||e.key==='+'){zoom=Math.min(5,zoom*1.1);R();}
  if(e.key==='-'){zoom=Math.max(0.3,zoom*0.9);R();}
  if(e.key==='r'||e.key==='R'){yaw=0.785;pitch=0.6;zoom=1.0;panX=0;panY=0;velY=0;velP=0;R();}
});
document.addEventListener('keyup',function(e){keys[e.key]=false;});
var t0d=0;
wrap.addEventListener('touchstart',function(e){
  if(e.touches.length===2){t0d=Math.hypot(e.touches[1].clientX-e.touches[0].clientX,e.touches[1].clientY-e.touches[0].clientY);e.preventDefault();return;}
  if(e.touches.length===1){dg=true;mv=false;lx=e.touches[0].clientX;ly=e.touches[0].clientY;dwX=lx;dwY=ly;velY=0;velP=0;e.preventDefault();}
},{passive:false});
wrap.addEventListener('touchmove',function(e){
  if(e.touches.length===2){var nd=Math.hypot(e.touches[1].clientX-e.touches[0].clientX,e.touches[1].clientY-e.touches[0].clientY);if(t0d>0){zoom=Math.max(0.3,Math.min(5,zoom*(nd/t0d)));t0d=nd;R();}e.preventDefault();return;}
  if(!dg||e.touches.length!==1)return;var dx=e.touches[0].clientX-lx,dy=e.touches[0].clientY-ly;if(Math.abs(e.touches[0].clientX-dwX)>3||Math.abs(e.touches[0].clientY-dwY)>3)mv=true;velY=dx*0.008;velP=dy*0.008;yaw+=velY;pitch=Math.max(-1.47,Math.min(1.47,pitch+velP));lx=e.touches[0].clientX;ly=e.touches[0].clientY;R();e.preventDefault();
},{passive:false});
wrap.addEventListener('touchend',function(e){if(e.touches.length===0){dg=false;t0d=0;if(!mv){var bx=wrap.getBoundingClientRect(),ex=dwX-bx.left,ey=dwY-bx.top;var idx=ht(ex,ey);if(idx>=0){pin=true;piI=idx;sT(idx,dwX,dwY);R();}else{pin=false;piI=-1;tip.className='c3t';}}}});
"##;

pub fn render_radar3d_html(
    title: &str,
    x: &[f64], y: &[f64], z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32, h: i32,
    bg_color: Option<&str>,
) -> String {
    render_3d_html(3, title, x, y, z, axis_labels, colors, color_labels, w, h, bg_color)
}

pub fn render_lollipop3d_html(
    title: &str,
    x: &[f64], y: &[f64], z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32, h: i32,
    bg_color: Option<&str>,
) -> String {
    render_3d_html(4, title, x, y, z, axis_labels, colors, color_labels, w, h, bg_color)
}

pub fn render_kde3d_html(
    title: &str,
    x: &[f64], y: &[f64], z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32, h: i32,
    bg_color: Option<&str>,
) -> String {
    render_3d_html(5, title, x, y, z, axis_labels, colors, color_labels, w, h, bg_color)
}

pub fn render_ridgeline3d_html(
    title: &str,
    x: &[f64], y: &[f64], z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32, h: i32,
    bg_color: Option<&str>,
) -> String {
    render_3d_html(6, title, x, y, z, axis_labels, colors, color_labels, w, h, bg_color)
}
