#[inline(always)]
fn sq_dist_2d(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    let dx = ax - bx;
    let dy = ay - by;
    dx * dx + dy * dy
}

#[inline(always)]
pub fn sq_dist_nd(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    let mut d = 0.0f64;
    let mut i = 0usize;
    while i + 4 <= n {
        let d0 = a[i] - b[i];
        let d1 = a[i + 1] - b[i + 1];
        let d2 = a[i + 2] - b[i + 2];
        let d3 = a[i + 3] - b[i + 3];
        d += d0 * d0 + d1 * d1 + d2 * d2 + d3 * d3;
        i += 4;
    }
    while i < n {
        let di = a[i] - b[i];
        d += di * di;
        i += 1;
    }
    d
}

fn xorshift64(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn kmeans_pp_2d(x: &[f64], y: &[f64], k: usize, seed: u64) -> (Vec<f64>, Vec<f64>) {
    let n = x.len();
    let k = k.min(n);
    let mut rng = seed.wrapping_add(n as u64).wrapping_mul(0xDEADBEEF_u64);
    let first = (xorshift64(&mut rng) as usize) % n;
    let mut cx = vec![x[first]];
    let mut cy = vec![y[first]];
    let mut dists = vec![f64::INFINITY; n];
    for _ in 1..k {
        let lx = *cx.last().unwrap();
        let ly = *cy.last().unwrap();
        let mut total = 0.0f64;
        for i in 0..n {
            let d = sq_dist_2d(x[i], y[i], lx, ly);
            if d < dists[i] { dists[i] = d; }
            total += dists[i];
        }
        if total <= 0.0 { break; }
        let mut target = (xorshift64(&mut rng) as f64 / u64::MAX as f64) * total;
        let mut chosen = n - 1;
        for (i, &d) in dists.iter().enumerate() {
            target -= d;
            if target <= 0.0 { chosen = i; break; }
        }
        cx.push(x[chosen]);
        cy.push(y[chosen]);
    }
    (cx, cy)
}

fn kmeans_pp_nd(data: &[Vec<f64>], k: usize, seed: u64) -> Vec<Vec<f64>> {
    let n = data.len();
    let k = k.min(n);
    if k == 0 { return Vec::new(); }
    let mut rng = seed.wrapping_add(n as u64).wrapping_mul(0xCAFEBABE_u64);
    let first = (xorshift64(&mut rng) as usize) % n;
    let mut centroids = vec![data[first].clone()];
    let mut dists = vec![f64::INFINITY; n];
    for _ in 1..k {
        let last = centroids.last().unwrap().clone();
        let mut total = 0.0f64;
        for i in 0..n {
            let d = sq_dist_nd(&data[i], &last);
            if d < dists[i] { dists[i] = d; }
            total += dists[i];
        }
        if total <= 0.0 { break; }
        let mut target = (xorshift64(&mut rng) as f64 / u64::MAX as f64) * total;
        let mut chosen = n - 1;
        for (i, &d) in dists.iter().enumerate() {
            target -= d;
            if target <= 0.0 { chosen = i; break; }
        }
        centroids.push(data[chosen].clone());
    }
    centroids
}

pub fn kmeans_core_2d(
    x: &[f64],
    y: &[f64],
    k: usize,
    max_iter: usize,
    tol: f64,
) -> (Vec<i32>, Vec<f64>, Vec<f64>, f64) {
    let n = x.len().min(y.len());
    if n == 0 || k == 0 { return (Vec::new(), Vec::new(), Vec::new(), 0.0); }
    let k = k.min(n);
    let ncpu = std::thread::available_parallelism().map(|v| v.get()).unwrap_or(4);
    let chunk = (n + ncpu - 1) / ncpu;

    let (mut cx, mut cy) = kmeans_pp_2d(x, y, k, 0xABCD1234_u64);
    let mut labels = vec![0i32; n];
    let mut prev_inertia = f64::INFINITY;

    for _iter in 0..max_iter {
        let inertia = std::sync::Mutex::new(0.0f64);
        std::thread::scope(|s| {
            labels
                .chunks_mut(chunk)
                .zip(x.chunks(chunk).zip(y.chunks(chunk)))
                .for_each(|(lc, (xc, yc))| {
                    let (cx, cy, inertia) = (&cx, &cy, &inertia);
                    s.spawn(move || {
                        let mut local = 0.0f64;
                        for ((xi, yi), lbl) in xc.iter().zip(yc.iter()).zip(lc.iter_mut()) {
                            let mut best_c = 0usize;
                            let mut best_d = f64::INFINITY;
                            for ci in 0..cx.len() {
                                let d = sq_dist_2d(*xi, *yi, cx[ci], cy[ci]);
                                if d < best_d { best_d = d; best_c = ci; }
                            }
                            *lbl = best_c as i32;
                            local += best_d;
                        }
                        *inertia.lock().unwrap() += local;
                    });
                });
        });
        let inertia = *inertia.lock().unwrap();

        let mut sx = vec![0.0f64; k];
        let mut sy = vec![0.0f64; k];
        let mut cnt = vec![0u32; k];
        for (i, &ci) in labels.iter().enumerate() {
            let ci = ci as usize;
            sx[ci] += x[i];
            sy[ci] += y[i];
            cnt[ci] += 1;
        }
        for ci in 0..k {
            if cnt[ci] > 0 {
                let c = cnt[ci] as f64;
                cx[ci] = sx[ci] / c;
                cy[ci] = sy[ci] / c;
            } else {
                cx[ci] = x[ci % n];
                cy[ci] = y[ci % n];
            }
        }

        if (prev_inertia - inertia).abs() < tol { break; }
        prev_inertia = inertia;
    }

    (labels, cx, cy, prev_inertia)
}

pub fn minibatch_kmeans_core_2d(
    x: &[f64],
    y: &[f64],
    k: usize,
    max_iter: usize,
    batch_size: usize,
) -> (Vec<i32>, Vec<f64>, Vec<f64>, f64) {
    let n = x.len().min(y.len());
    if n == 0 || k == 0 { return (Vec::new(), Vec::new(), Vec::new(), 0.0); }
    let k = k.min(n);
    let batch = batch_size.min(n);

    let (mut cx, mut cy) = kmeans_pp_2d(x, y, k, 0xDEAD5678_u64);
    let mut counts = vec![1u32; k];
    let mut rng = 0xFEEDFACE_u64.wrapping_add(n as u64);

    for _iter in 0..max_iter {
        let start = (xorshift64(&mut rng) as usize) % (n - batch + 1).max(1);
        let xe = &x[start..(start + batch).min(n)];
        let ye = &y[start..(start + batch).min(n)];
        for (xi, yi) in xe.iter().zip(ye.iter()) {
            let mut best_c = 0usize;
            let mut best_d = f64::INFINITY;
            for ci in 0..k {
                let d = sq_dist_2d(*xi, *yi, cx[ci], cy[ci]);
                if d < best_d { best_d = d; best_c = ci; }
            }
            counts[best_c] += 1;
            let lr = 1.0 / counts[best_c] as f64;
            cx[best_c] += lr * (xi - cx[best_c]);
            cy[best_c] += lr * (yi - cy[best_c]);
        }
    }

    let mut labels = vec![0i32; n];
    let mut inertia = 0.0f64;
    for i in 0..n {
        let mut best_c = 0usize;
        let mut best_d = f64::INFINITY;
        for ci in 0..k {
            let d = sq_dist_2d(x[i], y[i], cx[ci], cy[ci]);
            if d < best_d { best_d = d; best_c = ci; }
        }
        labels[i] = best_c as i32;
        inertia += best_d;
    }

    (labels, cx, cy, inertia)
}

pub fn kmeans_core_nd(
    data: &[Vec<f64>],
    k: usize,
    max_iter: usize,
    tol: f64,
) -> (Vec<i32>, Vec<Vec<f64>>, f64) {
    let n = data.len();
    if n == 0 || k == 0 { return (Vec::new(), Vec::new(), 0.0); }
    let k = k.min(n);
    let dims = data[0].len();
    let ncpu = std::thread::available_parallelism().map(|v| v.get()).unwrap_or(4);
    let chunk = (n + ncpu - 1) / ncpu;

    let mut centroids = kmeans_pp_nd(data, k, 0xABCD5678_u64);
    let mut labels = vec![0i32; n];
    let mut prev_inertia = f64::INFINITY;

    for _iter in 0..max_iter {
        let inertia = std::sync::Mutex::new(0.0f64);
        std::thread::scope(|s| {
            labels
                .chunks_mut(chunk)
                .zip(data.chunks(chunk))
                .for_each(|(lc, dc)| {
                    let (centroids, inertia) = (&centroids, &inertia);
                    s.spawn(move || {
                        let mut local = 0.0f64;
                        for (pt, lbl) in dc.iter().zip(lc.iter_mut()) {
                            let mut best_c = 0usize;
                            let mut best_d = f64::INFINITY;
                            for (ci, cent) in centroids.iter().enumerate() {
                                let d = sq_dist_nd(pt, cent);
                                if d < best_d { best_d = d; best_c = ci; }
                            }
                            *lbl = best_c as i32;
                            local += best_d;
                        }
                        *inertia.lock().unwrap() += local;
                    });
                });
        });
        let inertia = *inertia.lock().unwrap();

        let mut sums = vec![vec![0.0f64; dims]; k];
        let mut cnt = vec![0u32; k];
        for (i, &ci) in labels.iter().enumerate() {
            let ci = ci as usize;
            cnt[ci] += 1;
            for d in 0..dims { sums[ci][d] += data[i][d]; }
        }
        for ci in 0..k {
            if cnt[ci] > 0 {
                let c = cnt[ci] as f64;
                for d in 0..dims { centroids[ci][d] = sums[ci][d] / c; }
            } else {
                centroids[ci] = data[ci % n].clone();
            }
        }

        if (prev_inertia - inertia).abs() < tol { break; }
        prev_inertia = inertia;
    }

    (labels, centroids, prev_inertia)
}

pub fn minibatch_kmeans_core_nd(
    data: &[Vec<f64>],
    k: usize,
    max_iter: usize,
    batch_size: usize,
) -> (Vec<i32>, Vec<Vec<f64>>, f64) {
    let n = data.len();
    if n == 0 || k == 0 { return (Vec::new(), Vec::new(), 0.0); }
    let k = k.min(n);
    let dims = data[0].len();
    let batch = batch_size.min(n);

    let mut centroids = kmeans_pp_nd(data, k, 0xCAFE1234_u64);
    let mut counts = vec![1u32; k];
    let mut rng = 0xFACEDEAD_u64.wrapping_add(n as u64);

    for _iter in 0..max_iter {
        let start = (xorshift64(&mut rng) as usize) % (n - batch + 1).max(1);
        let slice = &data[start..(start + batch).min(n)];
        for pt in slice.iter() {
            let mut best_c = 0usize;
            let mut best_d = f64::INFINITY;
            for (ci, cent) in centroids.iter().enumerate() {
                let d = sq_dist_nd(pt, cent);
                if d < best_d { best_d = d; best_c = ci; }
            }
            counts[best_c] += 1;
            let lr = 1.0 / counts[best_c] as f64;
            for d in 0..dims { centroids[best_c][d] += lr * (pt[d] - centroids[best_c][d]); }
        }
    }

    let mut labels = vec![0i32; n];
    let mut inertia = 0.0f64;
    for (i, pt) in data.iter().enumerate() {
        let mut best_c = 0usize;
        let mut best_d = f64::INFINITY;
        for (ci, cent) in centroids.iter().enumerate() {
            let d = sq_dist_nd(pt, cent);
            if d < best_d { best_d = d; best_c = ci; }
        }
        labels[i] = best_c as i32;
        inertia += best_d;
    }

    (labels, centroids, inertia)
}

fn push_js_str(buf: &mut Vec<u8>, s: &str) {
    for b in s.bytes() {
        match b {
            b'\'' => { buf.push(b'\\'); buf.push(b'\''); }
            b'\\' => { buf.push(b'\\'); buf.push(b'\\'); }
            b'\n' => { buf.push(b'\\'); buf.push(b'n'); }
            _ => buf.push(b),
        }
    }
}

fn b64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}

pub fn render_kmeans_html(
    title: &str,
    x_values: &[f64],
    y_values: &[f64],
    labels: &[i32],
    centroids_x: &[f64],
    centroids_y: &[f64],
    n_clusters: usize,
    inertia: f64,
    palette: &[u32],
    x_label: &str,
    y_label: &str,
    width: i32,
    height: i32,
    gridlines: bool,
) -> String {
    use crate::html::hover::{html_id, html_prefix, html_suffix};
    use crate::plot::statistical::common::{push_b, push_i, push_f2, hex6, palette_color};

    let n = x_values.len().min(y_values.len()).min(labels.len());
    if n == 0 { return String::new(); }

    let (min_x, max_x) = crate::bindings::utils::simd_ops::find_minmax(x_values);
    let (min_y, max_y) = crate::bindings::utils::simd_ops::find_minmax(y_values);
    let range_x = (max_x - min_x).max(1e-12);
    let range_y = (max_y - min_y).max(1e-12);

    let legend_w = 170i32;
    let pad_l = 56i32; let pad_t = 36i32; let pad_b = 48i32;
    let pad_r_actual = 20i32 + legend_w;
    let plot_w = width - pad_l - pad_r_actual;
    let plot_h = height - pad_t - pad_b;

    let group_names: Vec<String> = (0..n_clusters).map(|i| format!("Cluster {}", i + 1)).collect();
    let group_colors: Vec<String> = (0..n_clusters).map(|i| {
        let hx = hex6(palette_color(palette, i));
        format!("#{}{}{}{}{}{}", hx[0] as char, hx[1] as char, hx[2] as char, hx[3] as char, hx[4] as char, hx[5] as char)
    }).collect();

    let inv_rx = plot_w as f64 / range_x;
    let inv_ry = plot_h as f64 / range_y;

    let dedup = n > 50_000;
    let mut seen: std::collections::HashSet<u64> = if dedup {
        std::collections::HashSet::with_capacity((plot_w * plot_h) as usize)
    } else {
        std::collections::HashSet::new()
    };

    let mut group_raw: Vec<Vec<(i16, i16)>> = vec![Vec::new(); n_clusters];
    for i in 0..n {
        let gi = (labels[i] as usize).min(n_clusters.saturating_sub(1));
        let px = ((x_values[i] - min_x) * inv_rx).clamp(0.0, (plot_w - 1) as f64) as i16;
        let py = (plot_h as f64 - (y_values[i] - min_y) * inv_ry).clamp(0.0, (plot_h - 1) as f64) as i16;
        if dedup {
            let key = ((gi as u64) << 40) | ((px as u64 & 0xFFFFF) << 20) | (py as u64 & 0xFFFFF);
            if !seen.insert(key) { continue; }
        }
        group_raw[gi].push((px, py));
    }

    let group_b64: Vec<String> = group_raw.iter().map(|pts| {
        let mut raw = Vec::with_capacity(pts.len() * 4);
        for &(px, py) in pts {
            raw.extend_from_slice(&px.to_le_bytes());
            raw.extend_from_slice(&py.to_le_bytes());
        }
        b64_encode(&raw)
    }).collect();

    let centroid_px: Vec<i32> = centroids_x.iter().map(|&v| {
        pad_l + ((v - min_x) * inv_rx).clamp(0.0, (plot_w - 1) as f64) as i32
    }).collect();
    let centroid_py: Vec<i32> = centroids_y.iter().map(|&v| {
        pad_t + (plot_h as f64 - (v - min_y) * inv_ry).clamp(0.0, (plot_h - 1) as f64) as i32
    }).collect();

    let hid = html_id();
    let cv_id = format!("spcv{}", hid);
    let tip_id = format!("sptip{}", hid);
    let mut buf = Vec::<u8>::with_capacity(n / 4 + 14_000);

    html_prefix(&mut buf, title, hid);
    push_b(&mut buf, b"<div style=\"position:relative;display:inline-block\">");
    push_b(&mut buf, b"<canvas id=\""); buf.extend_from_slice(cv_id.as_bytes());
    push_b(&mut buf, b"\" width=\""); push_i(&mut buf, width);
    push_b(&mut buf, b"\" height=\""); push_i(&mut buf, height);
    push_b(&mut buf, b"\" style=\"display:block\"></canvas>");
    push_b(&mut buf, b"<div id=\""); buf.extend_from_slice(tip_id.as_bytes());
    push_b(&mut buf, b"\" style=\"position:absolute;pointer-events:none;opacity:0;transition:opacity .15s;background:#0b0e18;color:#f1f5f9;font:12px -apple-system,Arial,sans-serif;border-radius:8px;padding:6px 10px;white-space:nowrap;box-shadow:0 4px 16px rgba(0,0,0,.4);z-index:10\"></div>");
    push_b(&mut buf, b"</div><script>(function(){");
    push_b(&mut buf, b"var cv=document.getElementById('"); buf.extend_from_slice(cv_id.as_bytes()); push_b(&mut buf, b"'),ctx=cv.getContext('2d');");
    push_b(&mut buf, b"var tip=document.getElementById('"); buf.extend_from_slice(tip_id.as_bytes()); push_b(&mut buf, b"');");
    push_b(&mut buf, b"var pL="); push_i(&mut buf, pad_l); push_b(&mut buf, b",pT="); push_i(&mut buf, pad_t);
    push_b(&mut buf, b",pW="); push_i(&mut buf, plot_w); push_b(&mut buf, b",pH="); push_i(&mut buf, plot_h);
    push_b(&mut buf, b",W="); push_i(&mut buf, width); push_b(&mut buf, b",H="); push_i(&mut buf, height);
    push_b(&mut buf, b",minX="); push_f2(&mut buf, min_x);
    push_b(&mut buf, b",minY="); push_f2(&mut buf, min_y);
    push_b(&mut buf, b",rX="); push_f2(&mut buf, range_x);
    push_b(&mut buf, b",rY="); push_f2(&mut buf, range_y);
    push_b(&mut buf, b",N="); buf.extend_from_slice(format!("{}", n).as_bytes()); push_b(&mut buf, b";");
    push_b(&mut buf, b"var hidden={};");
    push_b(&mut buf, b"function b64(s){var b=atob(s),n=b.length,a=new Int16Array(n/2);for(var i=0;i<n;i+=2)a[i/2]=b.charCodeAt(i)|(b.charCodeAt(i+1)<<8);return a;}");
    push_b(&mut buf, b"var GD=[");
    for (gi, b64s) in group_b64.iter().enumerate() {
        if gi > 0 { buf.push(b','); }
        buf.push(b'\''); buf.extend_from_slice(b64s.as_bytes()); buf.push(b'\'');
    }
    push_b(&mut buf, b"];");
    push_b(&mut buf, b"var GC=[");
    for (gi, c) in group_colors.iter().enumerate() {
        if gi > 0 { buf.push(b','); }
        buf.push(b'\''); push_js_str(&mut buf, c); buf.push(b'\'');
    }
    push_b(&mut buf, b"];");
    push_b(&mut buf, b"var GN=[");
    for (gi, nm) in group_names.iter().enumerate() {
        if gi > 0 { buf.push(b','); }
        buf.push(b'\''); push_js_str(&mut buf, nm); buf.push(b'\'');
    }
    push_b(&mut buf, b"];");
    push_b(&mut buf, b"var CPX=[");
    for (i, &px) in centroid_px.iter().enumerate() {
        if i > 0 { buf.push(b','); }
        push_i(&mut buf, px);
    }
    push_b(&mut buf, b"],CPY=[");
    for (i, &py) in centroid_py.iter().enumerate() {
        if i > 0 { buf.push(b','); }
        push_i(&mut buf, py);
    }
    push_b(&mut buf, b"];");
    let gl = if gridlines { b"1" as &[u8] } else { b"0" };
    push_b(&mut buf, b"var GL="); buf.extend_from_slice(gl); push_b(&mut buf, b";");

    push_b(&mut buf, b"function draw(){");
    push_b(&mut buf, b"ctx.fillStyle='#fff';ctx.fillRect(0,0,W,H);");
    push_b(&mut buf, b"if(GL){ctx.strokeStyle='#e2e8f0';ctx.lineWidth=0.5;for(var i=1;i<=5;i++){var gy=pT+Math.round((1-i/5)*pH);ctx.beginPath();ctx.moveTo(pL,gy);ctx.lineTo(pL+pW,gy);ctx.stroke();}}");
    push_b(&mut buf, b"ctx.strokeStyle='#cbd5e1';ctx.lineWidth=1;");
    push_b(&mut buf, b"ctx.beginPath();ctx.moveTo(pL,pT);ctx.lineTo(pL,pT+pH);ctx.stroke();");
    push_b(&mut buf, b"ctx.beginPath();ctx.moveTo(pL,pT+pH);ctx.lineTo(pL+pW,pT+pH);ctx.stroke();");
    push_b(&mut buf, b"ctx.fillStyle='#9ca3af';ctx.font='9px Arial';ctx.textAlign='end';");
    push_b(&mut buf, b"for(var i=0;i<=5;i++){var f=i/5,yp=pT+Math.round((1-f)*pH),yv=minY+f*rY;ctx.fillText(yv>=1000?Math.round(yv)+'':yv.toFixed(2),pL-4,yp+3);}");
    push_b(&mut buf, b"ctx.textAlign='center';");
    push_b(&mut buf, b"for(var i=0;i<=5;i++){var f=i/5,xp=pL+Math.round(f*pW),xv=minX+f*rX;ctx.fillText(xv>=1000?Math.round(xv)+'':xv.toFixed(2),xp,pT+pH+14);}");
    if !y_label.is_empty() {
        push_b(&mut buf, b"ctx.save();ctx.translate(14,pT+pH/2);ctx.rotate(-Math.PI/2);ctx.font='11px Arial';ctx.fillStyle='#374151';ctx.textAlign='center';ctx.fillText('");
        push_js_str(&mut buf, y_label); push_b(&mut buf, b"',0,0);ctx.restore();");
    }
    if !x_label.is_empty() {
        push_b(&mut buf, b"ctx.font='11px Arial';ctx.fillStyle='#374151';ctx.textAlign='center';ctx.fillText('");
        push_js_str(&mut buf, x_label); push_b(&mut buf, b"',pL+pW/2,H-4);");
    }
    if !title.is_empty() {
        push_b(&mut buf, b"ctx.font='700 14px -apple-system,Arial,sans-serif';ctx.fillStyle='#1a202c';ctx.textAlign='center';ctx.fillText('");
        push_js_str(&mut buf, title); push_b(&mut buf, b"',W/2,22);");
    }
    push_b(&mut buf, b"for(var gi=0;gi<GD.length;gi++){if(hidden[gi])continue;var a=b64(GD[gi]);ctx.fillStyle=GC[gi]||GC[0];for(var i=0;i<a.length;i+=2)ctx.fillRect(pL+a[i],pT+a[i+1],2,2);}");
    push_b(&mut buf, b"for(var ci=0;ci<CPX.length;ci++){if(hidden[ci])continue;var px=CPX[ci],py=CPY[ci],cl=GC[ci]||'#333',s=9;");
    push_b(&mut buf, b"ctx.strokeStyle='#fff';ctx.lineWidth=3.5;ctx.beginPath();ctx.moveTo(px-s,py);ctx.lineTo(px+s,py);ctx.moveTo(px,py-s);ctx.lineTo(px,py+s);ctx.stroke();");
    push_b(&mut buf, b"ctx.strokeStyle=cl;ctx.lineWidth=2;ctx.beginPath();ctx.moveTo(px-s,py);ctx.lineTo(px+s,py);ctx.moveTo(px,py-s);ctx.lineTo(px,py+s);ctx.stroke();}");
    let leg_x = pad_l + plot_w + 12;
    let leg_top = pad_t + 8;
    push_b(&mut buf, b"ctx.textAlign='left';ctx.font='11px Arial';");
    for gi in 0..n_clusters {
        let ly = leg_top + gi as i32 * 22;
        push_b(&mut buf, b"if(!hidden["); push_i(&mut buf, gi as i32); push_b(&mut buf, b"]){ctx.globalAlpha=1;}else{ctx.globalAlpha=0.28;}");
        push_b(&mut buf, b"ctx.fillStyle='"); push_js_str(&mut buf, &group_colors[gi]); push_b(&mut buf, b"';");
        push_b(&mut buf, b"ctx.beginPath();ctx.arc("); push_i(&mut buf, leg_x + 6); buf.push(b','); push_i(&mut buf, ly + 6);
        push_b(&mut buf, b",6,0,2*Math.PI);ctx.fill();");
        push_b(&mut buf, b"ctx.globalAlpha=1;ctx.fillStyle='#374151';ctx.fillText(GN["); push_i(&mut buf, gi as i32); push_b(&mut buf, b"]||'',");
        push_i(&mut buf, leg_x + 17); buf.push(b','); push_i(&mut buf, ly + 11); push_b(&mut buf, b");");
    }
    push_b(&mut buf, b"ctx.globalAlpha=1;");
    let inertia_str = format!("inertia: {:.0}", inertia);
    push_b(&mut buf, b"ctx.fillStyle='#9ca3af';ctx.font='10px Arial';ctx.textAlign='left';ctx.fillText('");
    push_js_str(&mut buf, &inertia_str);
    push_b(&mut buf, b"',pL+4,pT+pH-8);}");
    push_b(&mut buf, b"draw();");
    push_b(&mut buf, b"cv.addEventListener('click',function(e){");
    push_b(&mut buf, b"var r=cv.getBoundingClientRect(),mx=e.clientX-r.left,my=e.clientY-r.top;");
    push_b(&mut buf, b"var lx="); push_i(&mut buf, leg_x); push_b(&mut buf, b",lt="); push_i(&mut buf, leg_top); push_b(&mut buf, b",ng="); push_i(&mut buf, n_clusters as i32); push_b(&mut buf, b";");
    push_b(&mut buf, b"for(var gi=0;gi<ng;gi++){var ly=lt+gi*22;if(mx>=lx&&mx<=lx+150&&my>=ly&&my<=ly+18){hidden[gi]=!hidden[gi];draw();return;}}");
    push_b(&mut buf, b"});");
    push_b(&mut buf, b"var allPts=[];for(var gi=0;gi<GD.length;gi++){var a=b64(GD[gi]);for(var i=0;i<a.length;i+=2)allPts.push([gi,pL+a[i],pT+a[i+1]]);}");
    push_b(&mut buf, b"cv.addEventListener('mousemove',function(e){");
    push_b(&mut buf, b"var r=cv.getBoundingClientRect(),mx=e.clientX-r.left,my=e.clientY-r.top;");
    push_b(&mut buf, b"if(mx<pL||mx>pL+pW||my<pT||my>pT+pH){tip.style.opacity=0;return;}");
    push_b(&mut buf, b"var best=null,bd=1e9;");
    push_b(&mut buf, b"for(var i=0;i<allPts.length;i++){var p=allPts[i];if(hidden[p[0]])continue;var dx=p[1]-mx,dy=p[2]-my,d=dx*dx+dy*dy;if(d<bd){bd=d;best=p;}}");
    push_b(&mut buf, b"if(!best||bd>400){tip.style.opacity=0;return;}");
    push_b(&mut buf, b"var xv=(best[1]-pL)/pW*rX+minX,yv=(1-(best[2]-pT)/pH)*rY+minY;");
    push_b(&mut buf, b"var gname=GN[best[0]]?'<br><span style=\"color:#94a3b8\">'+GN[best[0]]+'</span>':'';");
    push_b(&mut buf, b"tip.innerHTML='<b>x:</b> '+xv.toFixed(2)+'&nbsp;&nbsp;<b>y:</b> '+yv.toFixed(2)+gname;");
    push_b(&mut buf, b"var tx=best[1]+12,ty=best[2]-28;if(tx+160>W)tx=best[1]-170;if(ty<0)ty=best[2]+8;");
    push_b(&mut buf, b"tip.style.left=tx+'px';tip.style.top=ty+'px';tip.style.opacity=1;});");
    push_b(&mut buf, b"cv.addEventListener('mouseleave',function(){tip.style.opacity=0;});");
    push_b(&mut buf, b"})();</script>");
    html_suffix(&mut buf, hid, "[]");
    unsafe { String::from_utf8_unchecked(buf) }
}
