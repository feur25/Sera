use crate::html::hover::{html_id, html_prefix, html_suffix};
use crate::plot::statistical::common::{push_b, push_i, push_f2, hex6, palette_color};
use crate::bindings::utils::simd_ops::find_minmax;
use base64::Engine as _;
use std::collections::HashSet;

crate::chart_config!(KMeansConfig, 1000, 580;
    struct {
        pub x_values: &'a [f64],
        pub y_values: &'a [f64],
        pub k: usize,
        pub max_iter: usize,
        pub tol: f64,
        pub mini_batch: bool,
        pub batch_size: usize,
        pub palette: &'a [u32],
    }
    defaults {
        x_values: &[],
        y_values: &[],
        k: 3,
        max_iter: 300,
        tol: 1e-4,
        mini_batch: false,
        batch_size: 1000,
        palette: &[],
    }
);

#[inline(always)]
fn sq_dist_2d(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    let dx = ax - bx; let dy = ay - by; dx * dx + dy * dy
}

#[inline(always)]
pub fn sq_dist_nd(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len()); let mut d = 0.0f64; let mut i = 0;
    while i + 4 <= n {
        let (d0, d1) = (a[i]-b[i], a[i+1]-b[i+1]);
        let (d2, d3) = (a[i+2]-b[i+2], a[i+3]-b[i+3]);
        d += d0*d0 + d1*d1 + d2*d2 + d3*d3; i += 4;
    }
    while i < n { let di = a[i]-b[i]; d += di*di; i += 1; }
    d
}

fn xorshift64(s: &mut u64) -> u64 {
    *s ^= *s << 13; *s ^= *s >> 7; *s ^= *s << 17; *s
}

fn kmeans_pp_seed(s: &[f64], k: usize, seed: u64) -> Vec<f64> {
    let n = s.len(); let k = k.min(n);
    let mut rng = seed.wrapping_add(n as u64).wrapping_mul(0xDEADBEEF_u64);
    let first = (xorshift64(&mut rng) as usize) % n;
    let mut cx = vec![s[first]];
    let mut dists = vec![f64::INFINITY; n];
    for _ in 1..k {
        let lc = *cx.last().unwrap();
        let mut total = 0.0f64;
        for i in 0..n { let dx = s[i]-lc; let d = dx*dx; if d < dists[i] { dists[i]=d; } total += dists[i]; }
        if total <= 0.0 { break; }
        let mut target = (xorshift64(&mut rng) as f64 / u64::MAX as f64) * total;
        let mut chosen = n-1;
        for (i,&d) in dists.iter().enumerate() { target -= d; if target <= 0.0 { chosen=i; break; } }
        cx.push(s[chosen]);
    }
    cx
}

fn kmeans_pp_2d(x: &[f64], y: &[f64], k: usize, seed: u64) -> (Vec<f64>, Vec<f64>) {
    let n = x.len(); let k = k.min(n);
    let mut rng = seed.wrapping_add(n as u64).wrapping_mul(0xDEADBEEF_u64);
    let first = (xorshift64(&mut rng) as usize) % n;
    let mut cx = vec![x[first]]; let mut cy = vec![y[first]];
    let mut dists = vec![f64::INFINITY; n];
    for _ in 1..k {
        let (lx, ly) = (*cx.last().unwrap(), *cy.last().unwrap());
        let mut total = 0.0f64;
        for i in 0..n { let d = sq_dist_2d(x[i],y[i],lx,ly); if d < dists[i] { dists[i]=d; } total += dists[i]; }
        if total <= 0.0 { break; }
        let mut target = (xorshift64(&mut rng) as f64 / u64::MAX as f64) * total;
        let mut chosen = n-1;
        for (i,&d) in dists.iter().enumerate() { target -= d; if target <= 0.0 { chosen=i; break; } }
        cx.push(x[chosen]); cy.push(y[chosen]);
    }
    (cx, cy)
}

fn kmeans_pp_nd(data: &[Vec<f64>], k: usize, seed: u64) -> Vec<Vec<f64>> {
    let n = data.len(); let k = k.min(n);
    if k == 0 { return Vec::new(); }
    let mut rng = seed.wrapping_add(n as u64).wrapping_mul(0xCAFEBABE_u64);
    let first = (xorshift64(&mut rng) as usize) % n;
    let mut centroids = vec![data[first].clone()];
    let mut dists = vec![f64::INFINITY; n];
    for _ in 1..k {
        let last = centroids.last().unwrap().clone();
        let mut total = 0.0f64;
        for i in 0..n { let d = sq_dist_nd(&data[i],&last); if d < dists[i] { dists[i]=d; } total += dists[i]; }
        if total <= 0.0 { break; }
        let mut target = (xorshift64(&mut rng) as f64 / u64::MAX as f64) * total;
        let mut chosen = n-1;
        for (i,&d) in dists.iter().enumerate() { target -= d; if target <= 0.0 { chosen=i; break; } }
        centroids.push(data[chosen].clone());
    }
    centroids
}

#[inline]
fn assign_2d(x: &[f64], y: &[f64], labels: &mut [i32], cx: &[f64], cy: &[f64], chunk: usize) -> f64 {
    let inertia = std::sync::Mutex::new(0.0f64);
    std::thread::scope(|s| {
        labels.chunks_mut(chunk).zip(x.chunks(chunk).zip(y.chunks(chunk))).for_each(|(lc,(xc,yc))| {
            let (cx,cy,inertia) = (&cx,&cy,&inertia);
            s.spawn(move || {
                let mut loc = 0.0f64;
                for ((xi,yi),lbl) in xc.iter().zip(yc.iter()).zip(lc.iter_mut()) {
                    let (mut bc,mut bd) = (0,f64::INFINITY);
                    for ci in 0..cx.len() { let d=sq_dist_2d(*xi,*yi,cx[ci],cy[ci]); if d<bd { bd=d; bc=ci; } }
                    *lbl=bc as i32; loc+=bd;
                }
                *inertia.lock().unwrap()+=loc;
            });
        });
    });
    inertia.into_inner().unwrap()
}

pub fn kmeans_core_2d(x: &[f64], y: &[f64], k: usize, max_iter: usize, tol: f64) -> (Vec<i32>, Vec<f64>, Vec<f64>, f64) {
    let n = x.len().min(y.len());
    if n == 0 || k == 0 { return (Vec::new(), Vec::new(), Vec::new(), 0.0); }
    let k = k.min(n);
    let chunk = (n + std::thread::available_parallelism().map(|v| v.get()).unwrap_or(4) - 1)
        / std::thread::available_parallelism().map(|v| v.get()).unwrap_or(4);
    let (mut cx, mut cy) = kmeans_pp_2d(x, y, k, 0xABCD1234_u64);
    let mut labels = vec![0i32; n];
    let mut prev = f64::INFINITY;
    for _ in 0..max_iter {
        let inertia = assign_2d(x, y, &mut labels, &cx, &cy, chunk);
        let (mut sx, mut sy, mut cnt) = (vec![0.0f64;k], vec![0.0f64;k], vec![0u32;k]);
        for (i,&ci) in labels.iter().enumerate() { let ci=ci as usize; sx[ci]+=x[i]; sy[ci]+=y[i]; cnt[ci]+=1; }
        for ci in 0..k {
            if cnt[ci]>0 { let c=cnt[ci] as f64; cx[ci]=sx[ci]/c; cy[ci]=sy[ci]/c; }
            else { cx[ci]=x[ci%n]; cy[ci]=y[ci%n]; }
        }
        if (prev-inertia).abs()<tol { break; } prev=inertia;
    }
    (labels, cx, cy, prev)
}

pub fn minibatch_kmeans_core_2d(x: &[f64], y: &[f64], k: usize, max_iter: usize, batch_size: usize) -> (Vec<i32>, Vec<f64>, Vec<f64>, f64) {
    let n = x.len().min(y.len());
    if n == 0 || k == 0 { return (Vec::new(), Vec::new(), Vec::new(), 0.0); }
    let k = k.min(n); let batch = batch_size.min(n);
    let (mut cx, mut cy) = kmeans_pp_2d(x, y, k, 0xDEAD5678_u64);
    let mut counts = vec![1u32; k];
    let mut rng = 0xFEEDFACE_u64.wrapping_add(n as u64);
    for _ in 0..max_iter {
        let s = (xorshift64(&mut rng) as usize) % (n-batch+1).max(1);
        for (xi,yi) in x[s..(s+batch).min(n)].iter().zip(y[s..(s+batch).min(n)].iter()) {
            let (mut bc,mut bd) = (0,f64::INFINITY);
            for ci in 0..k { let d=sq_dist_2d(*xi,*yi,cx[ci],cy[ci]); if d<bd { bd=d; bc=ci; } }
            counts[bc]+=1; let lr=1.0/counts[bc] as f64;
            cx[bc]+=lr*(xi-cx[bc]); cy[bc]+=lr*(yi-cy[bc]);
        }
    }
    let mut labels=vec![0i32;n]; let mut inertia=0.0f64;
    for i in 0..n {
        let (mut bc,mut bd)=(0,f64::INFINITY);
        for ci in 0..k { let d=sq_dist_2d(x[i],y[i],cx[ci],cy[ci]); if d<bd { bd=d; bc=ci; } }
        labels[i]=bc as i32; inertia+=bd;
    }
    (labels, cx, cy, inertia)
}

pub fn kmeans_core_nd(data: &[Vec<f64>], k: usize, max_iter: usize, tol: f64) -> (Vec<i32>, Vec<Vec<f64>>, f64) {
    let n = data.len();
    if n == 0 || k == 0 { return (Vec::new(), Vec::new(), 0.0); }
    let k = k.min(n); let dims = data[0].len();
    let ncpu = std::thread::available_parallelism().map(|v| v.get()).unwrap_or(4);
    let chunk = (n + ncpu - 1) / ncpu;
    let mut centroids = kmeans_pp_nd(data, k, 0xABCD5678_u64);
    let mut labels = vec![0i32; n]; let mut prev = f64::INFINITY;
    for _ in 0..max_iter {
        let inertia = std::sync::Mutex::new(0.0f64);
        std::thread::scope(|s| {
            labels.chunks_mut(chunk).zip(data.chunks(chunk)).for_each(|(lc,dc)| {
                let (centroids,inertia) = (&centroids,&inertia);
                s.spawn(move || {
                    let mut loc=0.0f64;
                    for (pt,lbl) in dc.iter().zip(lc.iter_mut()) {
                        let (mut bc,mut bd)=(0,f64::INFINITY);
                        for (ci,c) in centroids.iter().enumerate() { let d=sq_dist_nd(pt,c); if d<bd { bd=d; bc=ci; } }
                        *lbl=bc as i32; loc+=bd;
                    }
                    *inertia.lock().unwrap()+=loc;
                });
            });
        });
        let inertia = *inertia.lock().unwrap();
        let mut sums = vec![vec![0.0f64;dims];k]; let mut cnt = vec![0u32;k];
        for (i,&ci) in labels.iter().enumerate() { let ci=ci as usize; cnt[ci]+=1; for d in 0..dims { sums[ci][d]+=data[i][d]; } }
        for ci in 0..k {
            if cnt[ci]>0 { let c=cnt[ci] as f64; for d in 0..dims { centroids[ci][d]=sums[ci][d]/c; } }
            else { centroids[ci]=data[ci%n].clone(); }
        }
        if (prev-inertia).abs()<tol { break; } prev=inertia;
    }
    (labels, centroids, prev)
}

pub fn minibatch_kmeans_core_nd(data: &[Vec<f64>], k: usize, max_iter: usize, batch_size: usize) -> (Vec<i32>, Vec<Vec<f64>>, f64) {
    let n = data.len();
    if n == 0 || k == 0 { return (Vec::new(), Vec::new(), 0.0); }
    let k = k.min(n); let dims = data[0].len(); let batch = batch_size.min(n);
    let mut centroids = kmeans_pp_nd(data, k, 0xCAFE1234_u64);
    let mut counts = vec![1u32; k];
    let mut rng = 0xFACEDEAD_u64.wrapping_add(n as u64);
    for _ in 0..max_iter {
        let s = (xorshift64(&mut rng) as usize) % (n-batch+1).max(1);
        for pt in data[s..(s+batch).min(n)].iter() {
            let (mut bc,mut bd)=(0,f64::INFINITY);
            for (ci,c) in centroids.iter().enumerate() { let d=sq_dist_nd(pt,c); if d<bd { bd=d; bc=ci; } }
            counts[bc]+=1; let lr=1.0/counts[bc] as f64;
            for d in 0..dims { centroids[bc][d]+=lr*(pt[d]-centroids[bc][d]); }
        }
    }
    let mut labels=vec![0i32;n]; let mut inertia=0.0f64;
    for (i,pt) in data.iter().enumerate() {
        let (mut bc,mut bd)=(0,f64::INFINITY);
        for (ci,c) in centroids.iter().enumerate() { let d=sq_dist_nd(pt,c); if d<bd { bd=d; bc=ci; } }
        labels[i]=bc as i32; inertia+=bd;
    }
    (labels, centroids, inertia)
}

fn push_js_str(buf: &mut Vec<u8>, s: &str) {
    for b in s.bytes() {
        match b { b'\'' => { buf.push(b'\\'); buf.push(b'\''); } b'\\' => { buf.push(b'\\'); buf.push(b'\\'); } b'\n' => { buf.push(b'\\'); buf.push(b'n'); } _ => buf.push(b) }
    }
}

fn b64enc(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}

pub fn render_kmeans_html(cfg: &KMeansConfig) -> String {
    let n = cfg.x_values.len().min(cfg.y_values.len());
    if n == 0 || cfg.k == 0 { return String::new(); }

    let (labels, cx, cy, inertia) = if cfg.mini_batch || n > 100_000 {
        minibatch_kmeans_core_2d(cfg.x_values, cfg.y_values, cfg.k, cfg.max_iter, cfg.batch_size)
    } else {
        kmeans_core_2d(cfg.x_values, cfg.y_values, cfg.k, cfg.max_iter, cfg.tol)
    };
    let k = cx.len();

    let (min_x, max_x) = find_minmax(cfg.x_values);
    let (min_y, max_y) = find_minmax(cfg.y_values);
    let (rx, ry) = ((max_x-min_x).max(1e-12), (max_y-min_y).max(1e-12));
    let (pad_l, pad_t, pad_b, leg_w) = (56i32, 36i32, 48i32, 170i32);
    let (pw, ph) = (cfg.width - pad_l - 20 - leg_w, cfg.height - pad_t - pad_b);
    let (irx, iry) = (pw as f64 / rx, ph as f64 / ry);

    let colors: Vec<String> = (0..k).map(|i| {
        let hx = hex6(palette_color(cfg.palette, i));
        format!("#{}{}{}{}{}{}", hx[0] as char, hx[1] as char, hx[2] as char, hx[3] as char, hx[4] as char, hx[5] as char)
    }).collect();

    let dedup = n > 50_000;
    let mut seen: HashSet<u64> = if dedup { HashSet::with_capacity((pw*ph) as usize) } else { HashSet::new() };
    let mut raw: Vec<Vec<(i16,i16)>> = vec![Vec::new(); k];
    for i in 0..n {
        let gi = (labels[i] as usize).min(k.saturating_sub(1));
        let px = ((cfg.x_values[i]-min_x)*irx).clamp(0.0,(pw-1)as f64) as i16;
        let py = (ph as f64-(cfg.y_values[i]-min_y)*iry).clamp(0.0,(ph-1)as f64) as i16;
        if dedup { let key=((gi as u64)<<40)|((px as u64&0xFFFFF)<<20)|(py as u64&0xFFFFF); if !seen.insert(key) { continue; } }
        raw[gi].push((px,py));
    }
    let gd: Vec<String> = raw.iter().map(|pts| {
        let mut v=Vec::with_capacity(pts.len()*4);
        for &(px,py) in pts { v.extend_from_slice(&px.to_le_bytes()); v.extend_from_slice(&py.to_le_bytes()); }
        b64enc(&v)
    }).collect();
    let cpx: Vec<i32> = cx.iter().map(|&v| pad_l+((v-min_x)*irx).clamp(0.0,(pw-1)as f64) as i32).collect();
    let cpy: Vec<i32> = cy.iter().map(|&v| pad_t+(ph as f64-(v-min_y)*iry).clamp(0.0,(ph-1)as f64) as i32).collect();

    let hid = html_id();
    let cv = format!("spcv{hid}"); let tip = format!("sptip{hid}");
    let mut buf = Vec::<u8>::with_capacity(n/4 + 14_000);
    let lg = pad_l + pw + 12; let lt = pad_t + 8;

    html_prefix(&mut buf, cfg.title, hid);
    push_b(&mut buf, b"<div style=\"position:relative;display:inline-block\">");
    push_b(&mut buf, b"<canvas id=\""); buf.extend_from_slice(cv.as_bytes());
    push_b(&mut buf, b"\" width=\""); push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\""); push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\" style=\"display:block\"></canvas><div id=\""); buf.extend_from_slice(tip.as_bytes());
    push_b(&mut buf, b"\" style=\"position:absolute;pointer-events:none;opacity:0;transition:opacity .15s;background:#0b0e18;color:#f1f5f9;font:12px -apple-system,Arial,sans-serif;border-radius:8px;padding:6px 10px;white-space:nowrap;box-shadow:0 4px 16px rgba(0,0,0,.4);z-index:10\"></div></div><script>(function(){");
    push_b(&mut buf, b"var cv=document.getElementById('"); buf.extend_from_slice(cv.as_bytes()); push_b(&mut buf, b"'),ctx=cv.getContext('2d'),tip=document.getElementById('"); buf.extend_from_slice(tip.as_bytes()); push_b(&mut buf, b"');");
    push_b(&mut buf, b"var pL="); push_i(&mut buf, pad_l); push_b(&mut buf, b",pT="); push_i(&mut buf, pad_t);
    push_b(&mut buf, b",pW="); push_i(&mut buf, pw); push_b(&mut buf, b",pH="); push_i(&mut buf, ph);
    push_b(&mut buf, b",W="); push_i(&mut buf, cfg.width); push_b(&mut buf, b",H="); push_i(&mut buf, cfg.height);
    push_b(&mut buf, b",minX="); push_f2(&mut buf, min_x); push_b(&mut buf, b",minY="); push_f2(&mut buf, min_y);
    push_b(&mut buf, b",rX="); push_f2(&mut buf, rx); push_b(&mut buf, b",rY="); push_f2(&mut buf, ry);
    push_b(&mut buf, b",GL="); buf.extend_from_slice(if cfg.gridlines { b"1" } else { b"0" }); push_b(&mut buf, b";");
    push_b(&mut buf, b"var hidden={};");
    push_b(&mut buf, b"function b64(s){var b=atob(s),n=b.length,a=new Int16Array(n/2);for(var i=0;i<n;i+=2)a[i/2]=b.charCodeAt(i)|(b.charCodeAt(i+1)<<8);return a;}");
    push_b(&mut buf, b"var GD=[");
    for (i,d) in gd.iter().enumerate() { if i>0{buf.push(b',');} buf.push(b'\''); buf.extend_from_slice(d.as_bytes()); buf.push(b'\''); }
    push_b(&mut buf, b"],GC=[");
    for (i,c) in colors.iter().enumerate() { if i>0{buf.push(b',');} buf.push(b'\''); push_js_str(&mut buf, c); buf.push(b'\''); }
    push_b(&mut buf, b"],GN=[");
    for i in 0..k { if i>0{buf.push(b',');} buf.push(b'\''); push_js_str(&mut buf, &format!("Cluster {}",i+1)); buf.push(b'\''); }
    push_b(&mut buf, b"],CPX=[");
    for (i,&px) in cpx.iter().enumerate() { if i>0{buf.push(b',');} push_i(&mut buf, px); }
    push_b(&mut buf, b"],CPY=[");
    for (i,&py) in cpy.iter().enumerate() { if i>0{buf.push(b',');} push_i(&mut buf, py); }
    push_b(&mut buf, b"];");
    push_b(&mut buf, b"function draw(){ctx.fillStyle='#fff';ctx.fillRect(0,0,W,H);");
    push_b(&mut buf, b"if(GL){ctx.strokeStyle='#e2e8f0';ctx.lineWidth=0.5;for(var i=1;i<=5;i++){var gy=pT+Math.round((1-i/5)*pH);ctx.beginPath();ctx.moveTo(pL,gy);ctx.lineTo(pL+pW,gy);ctx.stroke();}}");
    push_b(&mut buf, b"ctx.strokeStyle='#cbd5e1';ctx.lineWidth=1;ctx.beginPath();ctx.moveTo(pL,pT);ctx.lineTo(pL,pT+pH);ctx.stroke();ctx.beginPath();ctx.moveTo(pL,pT+pH);ctx.lineTo(pL+pW,pT+pH);ctx.stroke();");
    push_b(&mut buf, b"ctx.fillStyle='#9ca3af';ctx.font='9px Arial';ctx.textAlign='end';for(var i=0;i<=5;i++){var f=i/5,yp=pT+Math.round((1-f)*pH),yv=minY+f*rY;ctx.fillText(yv>=1000?Math.round(yv)+'':yv.toFixed(2),pL-4,yp+3);}");
    push_b(&mut buf, b"ctx.textAlign='center';for(var i=0;i<=5;i++){var f=i/5,xp=pL+Math.round(f*pW),xv=minX+f*rX;ctx.fillText(xv>=1000?Math.round(xv)+'':xv.toFixed(2),xp,pT+pH+14);}");
    if !cfg.y_label.is_empty() { push_b(&mut buf, b"ctx.save();ctx.translate(14,pT+pH/2);ctx.rotate(-Math.PI/2);ctx.font='11px Arial';ctx.fillStyle='#374151';ctx.textAlign='center';ctx.fillText('"); push_js_str(&mut buf, cfg.y_label); push_b(&mut buf, b"',0,0);ctx.restore();"); }
    if !cfg.x_label.is_empty() { push_b(&mut buf, b"ctx.font='11px Arial';ctx.fillStyle='#374151';ctx.textAlign='center';ctx.fillText('"); push_js_str(&mut buf, cfg.x_label); push_b(&mut buf, b"',pL+pW/2,H-4);"); }
    if !cfg.title.is_empty() { push_b(&mut buf, b"ctx.font='700 14px -apple-system,Arial,sans-serif';ctx.fillStyle='#1a202c';ctx.textAlign='center';ctx.fillText('"); push_js_str(&mut buf, cfg.title); push_b(&mut buf, b"',W/2,22);"); }
    push_b(&mut buf, b"for(var gi=0;gi<GD.length;gi++){if(hidden[gi])continue;var a=b64(GD[gi]);ctx.fillStyle=GC[gi]||GC[0];for(var i=0;i<a.length;i+=2)ctx.fillRect(pL+a[i],pT+a[i+1],2,2);}");
    push_b(&mut buf, b"for(var ci=0;ci<CPX.length;ci++){if(hidden[ci])continue;var px=CPX[ci],py=CPY[ci],cl=GC[ci]||'#333',s=9;ctx.strokeStyle='#fff';ctx.lineWidth=3.5;ctx.beginPath();ctx.moveTo(px-s,py);ctx.lineTo(px+s,py);ctx.moveTo(px,py-s);ctx.lineTo(px,py+s);ctx.stroke();ctx.strokeStyle=cl;ctx.lineWidth=2;ctx.beginPath();ctx.moveTo(px-s,py);ctx.lineTo(px+s,py);ctx.moveTo(px,py-s);ctx.lineTo(px,py+s);ctx.stroke();}");
    push_b(&mut buf, b"ctx.textAlign='left';ctx.font='11px Arial';");
    for gi in 0..k {
        let ly = lt + gi as i32 * 22;
        push_b(&mut buf, b"if(!hidden["); push_i(&mut buf, gi as i32); push_b(&mut buf, b"]){ctx.globalAlpha=1;}else{ctx.globalAlpha=0.28;}");
        push_b(&mut buf, b"ctx.fillStyle='"); push_js_str(&mut buf, &colors[gi]); push_b(&mut buf, b"';ctx.beginPath();ctx.arc("); push_i(&mut buf, lg+6); buf.push(b','); push_i(&mut buf, ly+6); push_b(&mut buf, b",6,0,2*Math.PI);ctx.fill();ctx.globalAlpha=1;ctx.fillStyle='#374151';ctx.fillText(GN["); push_i(&mut buf, gi as i32); push_b(&mut buf, b"]||'',"); push_i(&mut buf, lg+17); buf.push(b','); push_i(&mut buf, ly+11); push_b(&mut buf, b");");
    }
    push_b(&mut buf, b"ctx.globalAlpha=1;ctx.fillStyle='#9ca3af';ctx.font='10px Arial';ctx.textAlign='left';ctx.fillText('");
    push_js_str(&mut buf, &format!("inertia: {inertia:.0}"));
    push_b(&mut buf, b"',pL+4,pT+pH-8);}draw();");
    push_b(&mut buf, b"cv.addEventListener('click',function(e){var r=cv.getBoundingClientRect(),mx=e.clientX-r.left,my=e.clientY-r.top;");
    push_b(&mut buf, b"for(var gi=0;gi<"); push_i(&mut buf, k as i32); push_b(&mut buf, b";gi++){var ly="); push_i(&mut buf, lt); push_b(&mut buf, b"+gi*22;if(mx>="); push_i(&mut buf, lg); push_b(&mut buf, b"&&mx<="); push_i(&mut buf, lg+150); push_b(&mut buf, b"&&my>=ly&&my<=ly+18){hidden[gi]=!hidden[gi];draw();return;}}});");
    push_b(&mut buf, b"var allPts=[];for(var gi=0;gi<GD.length;gi++){var a=b64(GD[gi]);for(var i=0;i<a.length;i+=2)allPts.push([gi,pL+a[i],pT+a[i+1]]);}");
    push_b(&mut buf, b"cv.addEventListener('mousemove',function(e){var r=cv.getBoundingClientRect(),mx=e.clientX-r.left,my=e.clientY-r.top;if(mx<pL||mx>pL+pW||my<pT||my>pT+pH){tip.style.opacity=0;return;}var best=null,bd=1e9;for(var i=0;i<allPts.length;i++){var p=allPts[i];if(hidden[p[0]])continue;var dx=p[1]-mx,dy=p[2]-my,d=dx*dx+dy*dy;if(d<bd){bd=d;best=p;}}if(!best||bd>400){tip.style.opacity=0;return;}var xv=(best[1]-pL)/pW*rX+minX,yv=(1-(best[2]-pT)/pH)*rY+minY;var gn=GN[best[0]]?'<br><span style=\"color:#94a3b8\">'+GN[best[0]]+'</span>':'';tip.innerHTML='<b>x:</b> '+xv.toFixed(2)+'&nbsp;&nbsp;<b>y:</b> '+yv.toFixed(2)+gn;var tx=best[1]+12,ty=best[2]-28;if(tx+160>W)tx=best[1]-170;if(ty<0)ty=best[2]+8;tip.style.left=tx+'px';tip.style.top=ty+'px';tip.style.opacity=1;});");
    push_b(&mut buf, b"cv.addEventListener('mouseleave',function(){tip.style.opacity=0;});})();</script>");
    html_suffix(&mut buf, hid, "[]");
    unsafe { String::from_utf8_unchecked(buf) }
}
