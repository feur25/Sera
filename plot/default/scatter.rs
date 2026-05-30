use crate::plot::{parse_all, apply_bg3d};
pub struct Scatter;

pub fn render_scatter_fast(
    values: &[f64],
    labels: &[String],
    width: i32,
    height: i32,
) -> String {
    let n = values.len().min(labels.len());
    if n == 0 { return String::new(); }
    
    let (_, max_val) = crate::bindings::utils::simd_ops::find_minmax(values);
    let max_val = max_val.max(1.0);
    let radius = 4;
    let scale_x = width as f64 / n as f64;
    let scale_y = height as f64 / max_val;
    
    let mut svg = String::with_capacity(n * 100 + 256);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\"><defs><style>.s{fill-opacity:0.8}</style></defs>");
    
    let colors = [0x6366F1, 0xF43F5E, 0x10B981, 0xF59E0B, 0x8B5CF6, 0x06B6D4, 0xEC4899, 0x84CC16];
    
    for i in 0..n {
        let x = (i as f64 * scale_x + scale_x / 2.0) as i32;
        let y = height - (values[i] * scale_y) as i32;
        let hex = format!("{:06x}", colors[i % colors.len()]);
        
        svg.push_str("<circle cx=\"");
        svg.push_str(&x.to_string());
        svg.push_str("\" cy=\"");
        svg.push_str(&y.to_string());
        svg.push_str("\" r=\"");
        svg.push_str(&radius.to_string());
        svg.push_str("\" fill=\"#");
        svg.push_str(&hex);
        svg.push_str("\" data-index=\"");
        svg.push_str(&i.to_string());
        svg.push_str("\"/>");
    }
    
    svg.push_str("</svg>");
    svg
}

pub fn render_points(ctx: super::PlotRenderContext) {
    let visible_count = ctx.visible_indices.len();
    
    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = value / ctx.max_val.max(1.0);
        
        let pos = if ctx.vertical {
            let x = ctx.plot_rect.left() + (ctx.plot_rect.width() / (visible_count as f32 - 1.0).max(1.0)) * vis_idx as f32;
            let y = ctx.plot_rect.bottom() - norm_val as f32 * ctx.plot_rect.height();
            egui::pos2(x, y)
        } else {
            let x = ctx.plot_rect.left() + norm_val as f32 * ctx.plot_rect.width();
            let y = ctx.plot_rect.top() + (ctx.plot_rect.height() / (visible_count as f32 - 1.0).max(1.0)) * vis_idx as f32;
            egui::pos2(x, y)
        };
        
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let color = ctx.colors[actual_idx % ctx.colors.len()];
        let (radius, display_color) = if is_hovered { 
            (6.0, egui::Color32::from_rgb(255, 200, 0)) 
        } else { 
            (4.0, color) 
        };
        ctx.painter.circle_filled(pos, radius, display_color);
    }
}

pub fn render_svg_scatter(
    svg: &mut String,
    values: &[f64],
    colors: &[&'static str],
    pad: i32,
    plot_width: i32,
    plot_height: i32,
    max_val: f64,
    vertical: bool,
) {
    let visible_count = values.len();
    let step = ((visible_count as f64) / (plot_width as f64)).max(1.0) as usize;

    for (vis_idx, &val) in values.iter().enumerate().step_by(step.max(1)) {
        let norm_val = val / max_val.max(1.0);
        let (x, y) = if vertical {
            let x_pos = pad as f64 + (plot_width as f64 / (visible_count as f64 - 1.0).max(1.0)) * vis_idx as f64;
            let y_pos = pad + plot_height - (norm_val * plot_height as f64) as i32;
            (x_pos as i32, y_pos)
        } else {
            let x_pos = pad as i32 + (norm_val * plot_width as f64) as i32;
            let y_pos = pad as f64 + (plot_height as f64 / (visible_count as f64 - 1.0).max(1.0)) * vis_idx as f64;
            (x_pos, y_pos as i32)
        };
        let color = colors[vis_idx % colors.len()];

        svg.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\" class=\"interactive-point\" data-index=\"{}\"/>",
            x, y, color, vis_idx
        ));
    }
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

fn scatter_reg_curve(x_values: &[f64], y_values: &[f64], regression_type: &str, min_x: f64, max_x: f64, min_y: f64, range_y: f64, pad_l: i32, pad_t: i32, plot_w: i32, plot_h: i32) -> Option<Vec<(i32, i32)>> {
    let n = x_values.len().min(y_values.len());
    if n < 2 { return None; }
    let nf = n as f64;
    let steps = 80usize;
    let inv_rx = plot_w as f64 / (max_x - min_x).max(1e-12);
    let inv_ry = plot_h as f64 / range_y.max(1e-12);
    match regression_type {
        "linear" => {
            let (sx, sy, sxx, sxy) = x_values.iter().zip(y_values.iter())
                .fold((0.0f64,0.0,0.0,0.0), |(sx,sy,sxx,sxy),(&xi,&yi)| (sx+xi,sy+yi,sxx+xi*xi,sxy+xi*yi));
            let denom = nf * sxx - sx * sx;
            if denom.abs() < 1e-12 { return None; }
            let slope = (nf * sxy - sx * sy) / denom;
            let intercept = (sy - slope * sx) / nf;
            let pts: Vec<(i32,i32)> = (0..=steps).map(|k| {
                let xv = min_x + k as f64 / steps as f64 * (max_x - min_x);
                let yv = intercept + slope * xv;
                let px = pad_l + ((xv - min_x) * inv_rx) as i32;
                let py = (pad_t + plot_h - ((yv - min_y) * inv_ry) as i32).clamp(0, pad_t + plot_h + 100);
                (px, py)
            }).collect();
            Some(pts)
        }
        "polynomial2" => {
            let (sx, sy, sx2, sx3, sx4, sxy, sx2y) = x_values.iter().zip(y_values.iter())
                .fold((0.0f64,0.0,0.0,0.0,0.0,0.0,0.0), |(sx,sy,sx2,sx3,sx4,sxy,sx2y),(&xi,&yi)| {
                    let x2=xi*xi; (sx+xi,sy+yi,sx2+x2,sx3+x2*xi,sx4+x2*x2,sxy+xi*yi,sx2y+x2*yi)
                });
            let a = [[nf,sx,sx2],[sx,sx2,sx3],[sx2,sx3,sx4]];
            let b = [sy,sxy,sx2y];
            fn det3(m: [[f64;3];3]) -> f64 {
                m[0][0]*(m[1][1]*m[2][2]-m[1][2]*m[2][1])
                -m[0][1]*(m[1][0]*m[2][2]-m[1][2]*m[2][0])
                +m[0][2]*(m[1][0]*m[2][1]-m[1][1]*m[2][0])
            }
            let d = det3(a);
            if d.abs() < 1e-12 { return None; }
            let c0 = det3([[b[0],a[0][1],a[0][2]],[b[1],a[1][1],a[1][2]],[b[2],a[2][1],a[2][2]]]) / d;
            let c1 = det3([[a[0][0],b[0],a[0][2]],[a[1][0],b[1],a[1][2]],[a[2][0],b[2],a[2][2]]]) / d;
            let c2 = det3([[a[0][0],a[0][1],b[0]],[a[1][0],a[1][1],b[1]],[a[2][0],a[2][1],b[2]]]) / d;
            let pts: Vec<(i32,i32)> = (0..=steps).map(|k| {
                let xv = min_x + k as f64 / steps as f64 * (max_x - min_x);
                let yv = c0 + c1 * xv + c2 * xv * xv;
                let px = pad_l + ((xv - min_x) * inv_rx) as i32;
                let py = (pad_t + plot_h - ((yv - min_y) * inv_ry) as i32).clamp(0, pad_t + plot_h + 100);
                (px, py)
            }).collect();
            Some(pts)
        }
        "log" => {
            let valid: Vec<(f64,f64)> = x_values.iter().zip(y_values.iter())
                .filter(|(&xi,_)| xi > 0.0).map(|(&xi,&yi)| (xi.ln(),yi)).collect();
            let m = valid.len(); if m < 2 { return None; }
            let mf = m as f64;
            let (slx, sy, slxx, slxy) = valid.iter()
                .fold((0.0f64,0.0,0.0,0.0), |(a,b,c,d),(lx,y)| (a+lx,b+y,c+lx*lx,d+lx*y));
            let denom = mf * slxx - slx * slx;
            if denom.abs() < 1e-12 { return None; }
            let slope = (mf * slxy - slx * sy) / denom;
            let intercept = (sy - slope * slx) / mf;
            let x0 = x_values.iter().cloned().filter(|&v| v > 0.0).fold(f64::INFINITY, f64::min).max(1e-9);
            let pts: Vec<(i32,i32)> = (0..=steps).map(|k| {
                let xv = x0 + k as f64 / steps as f64 * (max_x - x0).max(1e-12);
                let yv = intercept + slope * xv.ln();
                let px = pad_l + ((xv - min_x) * inv_rx) as i32;
                let py = (pad_t + plot_h - ((yv - min_y) * inv_ry) as i32).clamp(0, pad_t + plot_h + 100);
                (px, py)
            }).collect();
            Some(pts)
        }
        "exp" => {
            let valid: Vec<(f64,f64)> = x_values.iter().zip(y_values.iter())
                .filter(|(_,&yi)| yi > 0.0).map(|(&xi,&yi)| (xi, yi.ln())).collect();
            let m = valid.len(); if m < 2 { return None; }
            let mf = m as f64;
            let (sx2, sly, sxx2, sx2ly) = valid.iter()
                .fold((0.0f64,0.0,0.0,0.0), |(a,b,c,d),(x,ly)| (a+x,b+ly,c+x*x,d+x*ly));
            let denom = mf * sxx2 - sx2 * sx2;
            if denom.abs() < 1e-12 { return None; }
            let b = (mf * sx2ly - sx2 * sly) / denom;
            let a = ((sly - b * sx2) / mf).exp();
            let pts: Vec<(i32,i32)> = (0..=steps).map(|k| {
                let xv = min_x + k as f64 / steps as f64 * (max_x - min_x);
                let yv = a * (b * xv).exp();
                let px = pad_l + ((xv - min_x) * inv_rx) as i32;
                let py = (pad_t + plot_h - ((yv - min_y) * inv_ry) as i32).clamp(0, pad_t + plot_h + 100);
                (px, py)
            }).collect();
            Some(pts)
        }
        _ => None,
    }
}

fn render_scatter_canvas_html(
    title: &str,
    x_values: &[f64],
    y_values: &[f64],
    color_groups: &[String],
    palette: &[u32],
    x_label: &str,
    y_label: &str,
    color_hex: u32,
    width: i32,
    height: i32,
    gridlines: bool,
    show_regression: bool,
    regression_type: &str,
) -> String {
    use crate::html::hover::{html_id, html_prefix, html_suffix};
    use crate::plot::statistical::common::{push_b, push_i, push_f2, hex6, palette_color};
    let n = x_values.len().min(y_values.len());
    let (min_x, max_x) = crate::bindings::utils::simd_ops::find_minmax(x_values);
    let (min_y, max_y) = crate::bindings::utils::simd_ops::find_minmax(y_values);
    let range_x = (max_x - min_x).max(1e-12);
    let range_y = (max_y - min_y).max(1e-12);
    let has_groups = !color_groups.is_empty();
    let legend_w = if has_groups { 170i32 } else { 0i32 };
    let pad_l = 56i32; let pad_t = 36i32; let pad_b = 48i32;
    let pad_r_actual = 20i32 + legend_w;
    let plot_w = width - pad_l - pad_r_actual;
    let plot_h = height - pad_t - pad_b;
    let (group_names, group_colors, group_color_hex, group_map) = if has_groups {
        let mut names: Vec<String> = Vec::new();
        let mut map: Vec<usize> = Vec::with_capacity(n);
        for g in color_groups.iter().take(n) {
            let idx = names.iter().position(|x| x == g).unwrap_or_else(|| { names.push(g.clone()); names.len() - 1 });
            map.push(idx);
        }
        let colors: Vec<String> = names.iter().enumerate().map(|(i,_)| {
            let hx = hex6(palette_color(palette, i));
            format!("#{}{}{}{}{}{}", hx[0] as char, hx[1] as char, hx[2] as char, hx[3] as char, hx[4] as char, hx[5] as char)
        }).collect();
        (names, colors, Vec::<String>::new(), map)
    } else {
        let c = if color_hex != 0 { color_hex } else { palette_color(palette, 0) };
        let hx = hex6(c);
        let cs = format!("#{}{}{}{}{}{}", hx[0] as char, hx[1] as char, hx[2] as char, hx[3] as char, hx[4] as char, hx[5] as char);
        (Vec::new(), Vec::new(), vec![cs], Vec::new())
    };
    let ng = if has_groups { group_names.len().max(1) } else { 1 };
    let inv_rx = plot_w as f64 / range_x;
    let inv_ry = plot_h as f64 / range_y;
    let mut group_raw: Vec<Vec<(i16, i16, f32, f32)>> = vec![Vec::with_capacity(n / ng + 8); ng];
    for i in 0..n {
        let px = ((x_values[i] - min_x) * inv_rx).clamp(0.0, (plot_w - 1) as f64) as i16;
        let py = (plot_h as f64 - (y_values[i] - min_y) * inv_ry).clamp(0.0, (plot_h - 1) as f64) as i16;
        let gi = if has_groups && i < group_map.len() { group_map[i].min(ng - 1) } else { 0 };
        group_raw[gi].push((px, py, x_values[i] as f32, y_values[i] as f32));
    }
    let group_b64: Vec<String> = group_raw.iter().map(|pts| {
        let mut raw = Vec::with_capacity(pts.len() * 4);
        for &(px, py, _, _) in pts {
            raw.extend_from_slice(&px.to_le_bytes());
            raw.extend_from_slice(&py.to_le_bytes());
        }
        b64_encode(&raw)
    }).collect();
    let reg_curve = if show_regression {
        scatter_reg_curve(x_values, y_values, regression_type, min_x, max_x, min_y, range_y, pad_l, pad_t, plot_w, plot_h)
    } else { None };
    let hid = html_id();
    let cv_id = format!("spcv{}", hid);
    let tip_id = format!("sptip{}", hid);
    let mut buf = Vec::<u8>::with_capacity(n / 4 + 12_000);
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
    for (gi, b64) in group_b64.iter().enumerate() {
        if gi > 0 { buf.push(b','); }
        buf.push(b'\'');
        buf.extend_from_slice(b64.as_bytes());
        buf.push(b'\'');
    }
    push_b(&mut buf, b"];");
    push_b(&mut buf, b"var GC=[");
    if has_groups {
        for (gi, c) in group_colors.iter().enumerate() {
            if gi > 0 { buf.push(b','); }
            buf.push(b'\''); push_js_str(&mut buf, c); buf.push(b'\'');
        }
    } else {
        for (gi, c) in group_color_hex.iter().enumerate() {
            if gi > 0 { buf.push(b','); }
            buf.push(b'\''); push_js_str(&mut buf, c); buf.push(b'\'');
        }
    }
    push_b(&mut buf, b"];");
    push_b(&mut buf, b"var GN=[");
    if has_groups {
        for (gi, nm) in group_names.iter().enumerate() {
            if gi > 0 { buf.push(b','); }
            buf.push(b'\''); push_js_str(&mut buf, if nm.len() > 22 { &nm[..22] } else { nm }); buf.push(b'\'');
        }
    }
    push_b(&mut buf, b"];");
    let gridlines_js = if gridlines { b"1" as &[u8] } else { b"0" };
    push_b(&mut buf, b"var GL="); buf.extend_from_slice(gridlines_js); push_b(&mut buf, b";");
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
    if let Some(curve) = &reg_curve {
        push_b(&mut buf, b"ctx.strokeStyle='#ef4444';ctx.lineWidth=2;ctx.setLineDash([8,4]);ctx.beginPath();");
        for (k, &(px, py)) in curve.iter().enumerate() {
            if k == 0 {
                push_b(&mut buf, b"ctx.moveTo("); push_i(&mut buf, px); buf.push(b','); push_i(&mut buf, py); push_b(&mut buf, b");");
            } else {
                push_b(&mut buf, b"ctx.lineTo("); push_i(&mut buf, px); buf.push(b','); push_i(&mut buf, py); push_b(&mut buf, b");");
            }
        }
        push_b(&mut buf, b"ctx.stroke();ctx.setLineDash([]);");
    }
    push_b(&mut buf, b"ctx.fillStyle='#9ca3af';ctx.font='10px Arial';ctx.textAlign='left';ctx.fillText('n=");
    buf.extend_from_slice(format!("{}", n).as_bytes());
    push_b(&mut buf, b"',pL+4,pT+pH-8);");
    if has_groups {
        let leg_x = pad_l + plot_w + 12;
        let leg_top = pad_t + 8;
        push_b(&mut buf, b"ctx.textAlign='left';ctx.font='11px Arial';");
        for gi in 0..group_names.len() {
            let c = &group_colors[gi];
            let ly = leg_top + gi as i32 * 22;
            push_b(&mut buf, b"if(!hidden["); push_i(&mut buf, gi as i32); push_b(&mut buf, b"]){");
            push_b(&mut buf, b"ctx.globalAlpha=1;}else{ctx.globalAlpha=0.28;}");
            push_b(&mut buf, b"ctx.fillStyle='"); push_js_str(&mut buf, c); push_b(&mut buf, b"';");
            push_b(&mut buf, b"ctx.beginPath();ctx.arc("); push_i(&mut buf, leg_x + 6); buf.push(b','); push_i(&mut buf, ly + 6);
            push_b(&mut buf, b",6,0,2*Math.PI);ctx.fill();");
            push_b(&mut buf, b"ctx.globalAlpha=1;ctx.fillStyle='#374151';ctx.fillText(GN["); push_i(&mut buf, gi as i32); push_b(&mut buf, b"]||'',");
            push_i(&mut buf, leg_x + 17); buf.push(b','); push_i(&mut buf, ly + 11); push_b(&mut buf, b");");
        }
        push_b(&mut buf, b"ctx.globalAlpha=1;");
    }
    push_b(&mut buf, b"}");
    push_b(&mut buf, b"draw();");
    if has_groups {
        let leg_x = pad_l + plot_w + 12;
        let leg_top = pad_t + 8;
        let ng_js = group_names.len();
        push_b(&mut buf, b"cv.addEventListener('click',function(e){");
        push_b(&mut buf, b"var r=cv.getBoundingClientRect(),mx=e.clientX-r.left,my=e.clientY-r.top;");
        push_b(&mut buf, b"var lx="); push_i(&mut buf, leg_x); push_b(&mut buf, b",lt="); push_i(&mut buf, leg_top); push_b(&mut buf, b",ng="); push_i(&mut buf, ng_js as i32); push_b(&mut buf, b";");
        push_b(&mut buf, b"for(var gi=0;gi<ng;gi++){var ly=lt+gi*22;if(mx>=lx&&mx<=lx+150&&my>=ly&&my<=ly+18){hidden[gi]=!hidden[gi];draw();return;}}");
        push_b(&mut buf, b"});");
    }
    push_b(&mut buf, b"var _hx=-1,_hy=-1,_htimer=0;");
    push_b(&mut buf, b"var allPts=[];for(var gi=0;gi<GD.length;gi++){if(!GD[gi])continue;var a=b64(GD[gi]);for(var i=0;i<a.length;i+=2)allPts.push([gi,pL+a[i],pT+a[i+1]]);}");
    push_b(&mut buf, b"cv.addEventListener('mousemove',function(e){");
    push_b(&mut buf, b"var r=cv.getBoundingClientRect(),mx=e.clientX-r.left,my=e.clientY-r.top;");
    push_b(&mut buf, b"if(mx<pL||mx>pL+pW||my<pT||my>pT+pH){tip.style.opacity=0;return;}");
    push_b(&mut buf, b"clearTimeout(_htimer);_htimer=setTimeout(function(){");
    push_b(&mut buf, b"var best=null,bd=1e9;");
    push_b(&mut buf, b"for(var i=0;i<allPts.length;i++){var p=allPts[i];if(hidden[p[0]])continue;var dx=p[1]-mx,dy=p[2]-my,d=dx*dx+dy*dy;if(d<bd){bd=d;best=p;}}");
    push_b(&mut buf, b"if(!best||bd>400){tip.style.opacity=0;return;}");
    push_b(&mut buf, b"var xv=(best[1]-pL)/pW*rX+minX,yv=(1-(best[2]-pT)/pH)*rY+minY;");
    push_b(&mut buf, b"var gname=GN[best[0]]?'<br><span style=\"color:#94a3b8\">'+GN[best[0]]+'</span>':'';");
    push_b(&mut buf, b"tip.innerHTML='<b>x:</b> '+xv.toFixed(2)+'&nbsp;&nbsp;<b>y:</b> '+yv.toFixed(2)+gname;");
    push_b(&mut buf, b"var tx=best[1]+12,ty=best[2]-28;");
    push_b(&mut buf, b"if(tx+160>W)tx=best[1]-170;if(ty<0)ty=best[2]+8;");
    push_b(&mut buf, b"tip.style.left=tx+'px';tip.style.top=ty+'px';tip.style.opacity=1;");
    push_b(&mut buf, b"},60);});");
    push_b(&mut buf, b"cv.addEventListener('mouseleave',function(){tip.style.opacity=0;});");
    push_b(&mut buf, b"})();</script>");
    html_suffix(&mut buf, hid, "[]");
    unsafe { String::from_utf8_unchecked(buf) }
}

pub fn render_scatter_html(
    title: &str,
    x_values: &[f64],
    y_values: &[f64],
    labels: &[String],
    width: i32,
    height: i32,
    hover: &[crate::html::hover::HoverSlot],
    sizes: &[f64],
    color_groups: &[String],
    palette: &[u32],
    x_label: &str,
    y_label: &str,
    color_hex: u32,
    gridlines: bool,
    show_text: bool,
    show_regression: bool,
    regression_type: &str,
) -> String {
    use crate::html::hover::{slots_to_json, html_id, html_prefix, html_suffix};
    use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6, palette_color};
    let n = x_values.len().min(y_values.len());
    if n == 0 { return String::new(); }
    if n > 3_000 {
        return render_scatter_canvas_html(
            title, x_values, y_values, color_groups, palette,
            x_label, y_label, color_hex, width, height, gridlines, show_regression, regression_type,
        );
    }
    let (min_x, max_x) = crate::bindings::utils::simd_ops::find_minmax(x_values);
    let (min_y, max_y) = crate::bindings::utils::simd_ops::find_minmax(y_values);
    let range_x = (max_x - min_x).max(1.0);
    let range_y = (max_y - min_y).max(1.0);
    let pad_l = 56i32; let pad_t = 36i32; let pad_b = 48i32; let pad_r = 20i32;
    let has_groups = !color_groups.is_empty();
    let has_sizes = !sizes.is_empty();
    let (legend_w, group_names, group_map) = if has_groups {
        let mut names: Vec<String> = Vec::new();
        let mut map: Vec<usize> = Vec::with_capacity(n);
        for g in color_groups.iter().take(n) {
            let idx = names.iter().position(|x| x == g).unwrap_or_else(|| { names.push(g.clone()); names.len() - 1 });
            map.push(idx);
        }
        (160i32, names, map)
    } else {
        (0i32, Vec::new(), Vec::new())
    };
    let pad_r_actual = pad_r + legend_w;
    let plot_w = width - pad_l - pad_r_actual;
    let plot_h = height - pad_t - pad_b;
    let (size_min, size_max) = if has_sizes {
        let mn = sizes.iter().cloned().fold(f64::INFINITY, f64::min);
        let mx = sizes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (mn, (mx - mn).max(1.0))
    } else { (0.0, 1.0) };
    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 120 + 18_000);
    html_prefix(&mut buf, title, hid);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, width); push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, height); push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, width); push_b(&mut buf, b" ");
    push_i(&mut buf, height); push_b(&mut buf, b"\" data-sp=\"");
    push_i(&mut buf, pad_l); push_b(&mut buf, b",");
    push_i(&mut buf, pad_t); push_b(&mut buf, b",");
    push_i(&mut buf, plot_w); push_b(&mut buf, b",");
    push_i(&mut buf, plot_h); push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<defs><style>.sp{fill-opacity:.75;stroke:#fff;stroke-width:1}</style></defs>");
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    if !title.is_empty() {
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, width / 2);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut buf, title);
        push_b(&mut buf, b"</text>");
    }
    for i in 0..=5 {
        let frac = i as f64 / 5.0;
        let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        let val = min_y + frac * range_y;
        if gridlines && i > 0 {
            push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, pad_l);
            push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, y);
            push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, pad_l + plot_w);
            push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, y);
            push_b(&mut buf, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.5\" class=\"sp-gl\"/>");
        }
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, pad_l - 4);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, y + 3);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\" class=\"sp-yt\">");
        push_f2(&mut buf, val);
        push_b(&mut buf, b"</text>");
        let xval = min_x + frac * range_x;
        let xi = pad_l + (frac * plot_w as f64) as i32;
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, xi);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, pad_t + plot_h + 14);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\" class=\"sp-xt\">");
        push_f2(&mut buf, xval);
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, pad_t);
    push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, pad_t + plot_h);
    push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\" class=\"sp-ax-y\"/>");
    push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, pad_t + plot_h);
    push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, pad_l + plot_w);
    push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, pad_t + plot_h);
    push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\" class=\"sp-ax-x\"/>");
    if !y_label.is_empty() {
        push_b(&mut buf, b"<text x=\"14\" y=\""); push_i(&mut buf, pad_t + plot_h / 2);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\" transform=\"rotate(-90,14,");
        push_i(&mut buf, pad_t + plot_h / 2);
        push_b(&mut buf, b")\" class=\"sp-yl\">");
        escape_xml(&mut buf, y_label);
        push_b(&mut buf, b"</text>");
    }
    if !x_label.is_empty() {
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, pad_l + plot_w / 2);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, height - 4);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\" class=\"sp-xl\">");
        escape_xml(&mut buf, x_label);
        push_b(&mut buf, b"</text>");
    }
    let inv_rx = plot_w as f64 / range_x;
    let inv_ry = plot_h as f64 / range_y;
    let mut di = 0i32;
    for i in 0..n {
        let cx = pad_l + ((x_values[i] - min_x) * inv_rx) as i32;
        let cy = pad_t + plot_h - ((y_values[i] - min_y) * inv_ry) as i32;
        let color = if has_groups && i < group_map.len() {
            palette_color(palette, group_map[i])
        } else if color_hex != 0 {
            color_hex
        } else {
            palette_color(palette, i)
        };
        let hx = hex6(color);
        let r = if has_sizes && i < sizes.len() {
            ((sizes[i] - size_min) / size_max * 18.0 + 3.0) as i32
        } else { 5 };
        push_b(&mut buf, b"<circle data-idx=\""); push_i(&mut buf, di);
        if has_groups { push_b(&mut buf, b"\" data-series=\""); push_i(&mut buf, group_map[i] as i32); }
        push_b(&mut buf, b"\" data-x=\""); push_f2(&mut buf, x_values[i]);
        push_b(&mut buf, b"\" data-y=\""); push_f2(&mut buf, y_values[i]);
        if i < labels.len() && !labels[i].is_empty() {
            push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, &labels[i]);
        }
        push_b(&mut buf, b"\" cx=\""); push_i(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\""); push_i(&mut buf, cy);
        push_b(&mut buf, b"\" r=\""); push_i(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" class=\"sp\"/>");
        di += 1;
        if show_text && i < labels.len() && !labels[i].is_empty() {
            push_b(&mut buf, b"<text x=\""); push_i(&mut buf, cx);
            push_b(&mut buf, b"\" y=\""); push_i(&mut buf, cy - r - 3);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#374151\">");
            escape_xml(&mut buf, if labels[i].len() <= 14 { &labels[i] } else { &labels[i][..14] });
            push_b(&mut buf, b"</text>");
        }
    }
    if show_regression {
        if let Some(curve) = scatter_reg_curve(x_values, y_values, regression_type, min_x, max_x, min_y, range_y, pad_l, pad_t, plot_w, plot_h) {
            push_b(&mut buf, b"<polyline points=\"");
            for (k, &(px, py)) in curve.iter().enumerate() {
                if k > 0 { buf.push(b' '); }
                push_i(&mut buf, px); buf.push(b','); push_i(&mut buf, py);
            }
            push_b(&mut buf, b"\" fill=\"none\" stroke=\"#ef4444\" stroke-width=\"2\" stroke-dasharray=\"8,4\" opacity=\"0.85\"/>");
        }
    }
    if has_groups {
        let leg_x = pad_l + plot_w + 12;
        let leg_top = pad_t + 8;
        for (gi, name) in group_names.iter().enumerate() {
            let hx = hex6(palette_color(palette, gi));
            let ly = leg_top + gi as i32 * 22;
            push_b(&mut buf, b"<g data-legend=\"1\" data-series=\"");
            push_i(&mut buf, gi as i32);
            push_b(&mut buf, b"\">");
            push_b(&mut buf, b"<circle cx=\""); push_i(&mut buf, leg_x + 6);
            push_b(&mut buf, b"\" cy=\""); push_i(&mut buf, ly + 6);
            push_b(&mut buf, b"\" r=\"6\" fill=\"#"); buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\"/>");
            push_b(&mut buf, b"<text x=\""); push_i(&mut buf, leg_x + 17);
            push_b(&mut buf, b"\" y=\""); push_i(&mut buf, ly + 11);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
            escape_xml(&mut buf, if name.len() <= 20 { name } else { &name[..20] });
            push_b(&mut buf, b"</text></g>");
        }
    }
    push_b(&mut buf, b"</svg>");
    let slots_json;
    let json: &str = if hover.is_empty() { "[]" } else { slots_json = slots_to_json(hover); &slots_json };
    html_suffix(&mut buf, hid, json);
    unsafe { String::from_utf8_unchecked(buf) }
}

#[allow(dead_code)]
#[inline] fn scat_xml_esc(s: &str) -> String { s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;") }

fn dbscan_core(x: &[f64], y: &[f64], eps: f64, min_samples: usize) -> (Vec<i32>, usize) {
    use std::sync::atomic::{AtomicU32, Ordering};

    fn uf_find(uf: &[AtomicU32], mut x: usize) -> usize {
        loop {
            let p = uf[x].load(Ordering::Relaxed) as usize;
            if p == x { return x; }
            let gp = uf[p].load(Ordering::Relaxed) as usize;
            let _ = uf[x].compare_exchange_weak(p as u32, gp as u32, Ordering::Relaxed, Ordering::Relaxed);
            x = p;
        }
    }

    fn uf_union(uf: &[AtomicU32], a: usize, b: usize) {
        loop {
            let (ra, rb) = (uf_find(uf, a), uf_find(uf, b));
            if ra == rb { return; }
            let (lo, hi) = if ra < rb { (ra, rb) } else { (rb, ra) };
            if uf[hi].compare_exchange_weak(hi as u32, lo as u32, Ordering::AcqRel, Ordering::Relaxed).is_ok() { return; }
        }
    }

    let n = x.len().min(y.len());
    if n == 0 { return (Vec::new(), 0); }

    let eps_f = eps as f32;
    let eps2 = eps_f * eps_f;
    let cell_sz = eps_f * 0.5f32;
    let inv_cell = 1.0f32 / cell_sz;

    let xf: Vec<f32> = x[..n].iter().map(|&v| v as f32).collect();
    let yf: Vec<f32> = y[..n].iter().map(|&v| v as f32).collect();

    let (mut xmin, mut xmax, mut ymin, mut ymax) = (xf[0], xf[0], yf[0], yf[0]);
    for i in 1..n {
        if xf[i] < xmin { xmin = xf[i]; } if xf[i] > xmax { xmax = xf[i]; }
        if yf[i] < ymin { ymin = yf[i]; } if yf[i] > ymax { ymax = yf[i]; }
    }

    let gw = (((xmax - xmin) * inv_cell).ceil() as usize).max(1) + 1;
    let gh = (((ymax - ymin) * inv_cell).ceil() as usize).max(1) + 1;
    let tc = gw * gh;

    let mut co = vec![0u32; n];
    let mut cc = vec![0u32; tc];
    for i in 0..n {
        let c = (((yf[i] - ymin) * inv_cell) as usize).min(gh - 1) * gw
              + (((xf[i] - xmin) * inv_cell) as usize).min(gw - 1);
        co[i] = c as u32; cc[c] += 1;
    }
    let mut cs = vec![0u32; tc + 1];
    for i in 0..tc { cs[i + 1] = cs[i] + cc[i]; }

    let mut sx = vec![0.0f32; n];
    let mut sy = vec![0.0f32; n];
    let mut si = vec![0u32; n];
    let mut p = cs[..tc].to_vec();
    for i in 0..n {
        let c = co[i] as usize;
        let j = p[c] as usize;
        sx[j] = xf[i]; sy[j] = yf[i]; si[j] = i as u32;
        p[c] += 1;
    }

    let ncpu = std::thread::available_parallelism().map(|v| v.get()).unwrap_or(4).min(16);
    let ms = min_samples as u32;

    // For each cell, precompute the count of points in the cross-shaped 5-cell neighborhood
    // (the cell itself + 4 direct neighbors: up/down/left/right). With cell_sz = eps/2,
    // the maximum distance between any two points in this cross is exactly eps (horizontal/
    // vertical only), so they are guaranteed to be within the eps-ball. This allows us to
    // skip per-point distance checks for clearly dense cells.
    let mut cell_cross_cnt = vec![0u32; tc];
    for cy in 0..gh {
        for cx in 0..gw {
            let c = cy * gw + cx;
            if cc[c] == 0 { continue; }
            let mut sum = cc[c];
            if cy > 0 { sum += cc[(cy - 1) * gw + cx]; }
            if cy + 1 < gh { sum += cc[(cy + 1) * gw + cx]; }
            if cx > 0 { sum += cc[cy * gw + cx - 1]; }
            if cx + 1 < gw { sum += cc[cy * gw + cx + 1]; }
            cell_cross_cnt[c] = sum;
        }
    }

    let is_core: Vec<std::sync::atomic::AtomicU8> = (0..n).map(|_| std::sync::atomic::AtomicU8::new(0)).collect();
    let task = std::sync::atomic::AtomicUsize::new(0);

    std::thread::scope(|scope| {
        for _ in 0..ncpu {
            let (xf, yf, sx, sy, _si, cs, co, is_core, cell_cross_cnt, task) =
                (&xf, &yf, &sx, &sy, &si, &cs, &co, &is_core, &cell_cross_cnt, &task);
            scope.spawn(move || {
                loop {
                    let oi = task.fetch_add(1, Ordering::Relaxed);
                    if oi >= n { break; }
                    let c = co[oi] as usize;
                    if cell_cross_cnt[c] >= ms {
                        is_core[oi].store(1, Ordering::Relaxed);
                        continue;
                    }
                    let (xi, yi) = (xf[oi], yf[oi]);
                    let (gy, gx) = (c / gw, c % gw);
                    let mut cnt = 0u32;
                    'outer: for cy in gy.saturating_sub(2)..=(gy + 2).min(gh - 1) {
                        for cx in gx.saturating_sub(2)..=(gx + 2).min(gw - 1) {
                            let (a, b) = (cs[cy * gw + cx] as usize, cs[cy * gw + cx + 1] as usize);
                            for k in a..b {
                                let ddx = xi - sx[k]; let ddy = yi - sy[k];
                                if ddx * ddx + ddy * ddy <= eps2 {
                                    cnt += 1;
                                    if cnt >= ms {
                                        is_core[oi].store(1, Ordering::Relaxed);
                                        break 'outer;
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    let uf: Vec<AtomicU32> = (0..n).map(|i| AtomicU32::new(i as u32)).collect();

    // Phase 2: connect core points via union-find.
    // Key insight with cell_sz = eps/2:
    //   - Same cell or cross-adjacent (|dx|+|dy|<=1): any two points <= eps apart. Guaranteed.
    //   - Diagonal adjacent (|dx|=|dy|=1): max dist = sqrt(2)*eps > eps. Need exact check.
    //   - Outer ring (max(|dx|,|dy|)=2): max dist up to sqrt(8)*eps. Need exact check.
    //
    // Optimization: for guaranteed pairs, we only need to connect ONE representative per
    // cell (the first core in CSR order). All other cores in the cell are then chained to
    // it in a second O(n) pass. This reduces Phase 2 from O(pts_per_cell^2) to O(n_cells*25).

    // cell_rep[c] = index into CSR of first core point in cell c, or u32::MAX if none.
    let mut cell_rep = vec![u32::MAX; tc];
    for c in 0..tc {
        let (a, b) = (cs[c] as usize, cs[c + 1] as usize);
        for k in a..b {
            let oi = si[k] as usize;
            if is_core[oi].load(Ordering::Relaxed) == 1 {
                cell_rep[c] = k as u32;
                break;
            }
        }
    }

    // Connect all cores within same cell to the cell representative (intra-cell).
    for c in 0..tc {
        let kr = cell_rep[c];
        if kr == u32::MAX { continue; }
        let (a, b) = (cs[c] as usize, cs[c + 1] as usize);
        let or_ = si[kr as usize] as usize;
        for k in a..b {
            let oi = si[k] as usize;
            if oi == or_ { continue; }
            if is_core[oi].load(Ordering::Relaxed) == 1 { uf_union(&uf, or_, oi); }
        }
    }

    // Connect representative cores across neighboring cells, parallelized by column strips.
    // uf_union is atomic so safe to call from multiple threads simultaneously.
    let task3 = std::sync::atomic::AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..ncpu {
            let (sx, sy, si, cs, _co, is_core, uf, cell_rep, task3) =
                (&sx, &sy, &si, &cs, &co, &is_core, &uf, &cell_rep, &task3);
            scope.spawn(move || {
                loop {
                    let cy = task3.fetch_add(1, Ordering::Relaxed);
                    if cy >= gh { break; }
                    for cx in 0..gw {
                        let c = cy * gw + cx;
                        let kr0 = cell_rep[c];
                        if kr0 == u32::MAX { continue; }
                        let oi = si[kr0 as usize] as usize;
                        let (_xi, _yi) = (sx[kr0 as usize], sy[kr0 as usize]);
                        for ny in cy.saturating_sub(2)..=(cy + 2).min(gh - 1) {
                            for nx in cx.saturating_sub(2)..=(cx + 2).min(gw - 1) {
                                let nc = ny * gw + nx;
                                if nc <= c { continue; }
                                let kr1 = cell_rep[nc];
                                if kr1 == u32::MAX { continue; }
                                let oj = si[kr1 as usize] as usize;
                                let dcy = (ny as isize - cy as isize).unsigned_abs();
                                let dcx = (nx as isize - cx as isize).unsigned_abs();
                                if dcy + dcx <= 1 {
                                    uf_union(uf, oi, oj);
                                } else {
                                    let (a0, b0) = (cs[c] as usize, cs[c + 1] as usize);
                                    let (an, bn) = (cs[nc] as usize, cs[nc + 1] as usize);
                                    'outer: for k0 in a0..b0 {
                                        let pi = si[k0] as usize;
                                        if is_core[pi].load(Ordering::Relaxed) == 0 { continue; }
                                        let (pxi, pyi) = (sx[k0], sy[k0]);
                                        for k1 in an..bn {
                                            let pj = si[k1] as usize;
                                            if is_core[pj].load(Ordering::Relaxed) == 0 { continue; }
                                            let ddx = pxi - sx[k1]; let ddy = pyi - sy[k1];
                                            if ddx * ddx + ddy * ddy <= eps2 {
                                                uf_union(uf, pi, pj);
                                                break 'outer;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    let mut labels = vec![-1i32; n];

    for i in 0..n {
        if is_core[i].load(Ordering::Relaxed) == 1 {
            labels[i] = uf_find(&uf, i) as i32;
        }
    }

    for i in 0..n {
        if labels[i] >= 0 { continue; }
        let c = co[i] as usize;
        let (gy, gx) = (c / gw, c % gw);
        let (xi, yi) = (xf[i], yf[i]);
        let mut best = u32::MAX;
        let mut bd = f32::INFINITY;
        for cy in gy.saturating_sub(2)..=(gy + 2).min(gh - 1) {
            for cx in gx.saturating_sub(2)..=(gx + 2).min(gw - 1) {
                let (a, b) = (cs[cy * gw + cx] as usize, cs[cy * gw + cx + 1] as usize);
                for k in a..b {
                    let oj = si[k] as usize;
                    if is_core[oj].load(Ordering::Relaxed) == 0 { continue; }
                    let dx = xi - sx[k]; let dy = yi - sy[k];
                    let d = dx * dx + dy * dy;
                    if d <= eps2 && d < bd { bd = d; best = oj as u32; }
                }
            }
        }
        if best != u32::MAX {
            labels[i] = uf_find(&uf, best as usize) as i32;
        }
    }

    let mut rm = std::collections::HashMap::<i32, i32>::new();
    let mut cid = 0i32;
    for l in labels.iter_mut() {
        if *l >= 0 {
            let e = rm.entry(*l).or_insert_with(|| { let v = cid; cid += 1; v });
            *l = *e;
        }
    }

    (labels, cid as usize)
}

pub fn dbscan_core_nd(data: &[Vec<f64>], eps: f64, min_samples: usize) -> (Vec<i32>, usize) {
    use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};

    let n = data.len();
    if n == 0 { return (Vec::new(), 0); }
    let d = data[0].len();
    if d == 0 { return (vec![-1; n], 0); }

    if d == 2 {
        let x: Vec<f64> = data.iter().map(|p| p[0]).collect();
        let y: Vec<f64> = data.iter().map(|p| p[1]).collect();
        return dbscan_core(&x, &y, eps, min_samples);
    }

    let eps2 = eps * eps;
    let ms = min_samples as u32;
    let ncpu = std::thread::available_parallelism().map(|v| v.get()).unwrap_or(4).min(16);

    fn uf_find(uf: &[AtomicU32], mut x: usize) -> usize {
        loop {
            let p = uf[x].load(Ordering::Relaxed) as usize;
            if p == x { return x; }
            let gp = uf[p].load(Ordering::Relaxed) as usize;
            let _ = uf[x].compare_exchange_weak(p as u32, gp as u32, Ordering::Relaxed, Ordering::Relaxed);
            x = p;
        }
    }
    fn uf_union(uf: &[AtomicU32], a: usize, b: usize) {
        loop {
            let (ra, rb) = (uf_find(uf, a), uf_find(uf, b));
            if ra == rb { return; }
            let (lo, hi) = if ra < rb { (ra, rb) } else { (rb, ra) };
            if uf[hi].compare_exchange_weak(hi as u32, lo as u32, Ordering::AcqRel, Ordering::Relaxed).is_ok() { return; }
        }
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_unstable_by(|&a, &b| data[a][0].partial_cmp(&data[b][0]).unwrap_or(std::cmp::Ordering::Equal));

    let sorted_c0: Vec<f64> = order.iter().map(|&oi| data[oi][0]).collect();

    let mut inv_order = vec![0usize; n];
    for (si, &oi) in order.iter().enumerate() { inv_order[oi] = si; }

    let is_core: Vec<AtomicU8> = (0..n).map(|_| AtomicU8::new(0)).collect();
    let task1 = AtomicU32::new(0);
    std::thread::scope(|s| {
        for _ in 0..ncpu {
            let (data, order, sorted_c0, is_core, task1) =
                (&data, &order, &sorted_c0, &is_core, &task1);
            s.spawn(move || loop {
                let si = task1.fetch_add(1, Ordering::Relaxed) as usize;
                if si >= n { break; }
                let oi = order[si];
                let pt = &data[oi];
                let lo = sorted_c0.partition_point(|&v| v < pt[0] - eps);
                let hi = sorted_c0.partition_point(|&v| v <= pt[0] + eps);
                let mut cnt = 0u32;
                for sj in lo..hi {
                    let oj = order[sj];
                    let d2: f64 = pt.iter().zip(data[oj].iter()).map(|(&a, &b)| (a - b) * (a - b)).sum();
                    if d2 <= eps2 {
                        cnt += 1;
                        if cnt >= ms { is_core[oi].store(1, Ordering::Relaxed); break; }
                    }
                }
            });
        }
    });

    let uf: Vec<AtomicU32> = (0..n).map(|i| AtomicU32::new(i as u32)).collect();
    let task2 = AtomicU32::new(0);
    std::thread::scope(|s| {
        for _ in 0..ncpu {
            let (data, order, sorted_c0, is_core, uf, task2) =
                (&data, &order, &sorted_c0, &is_core, &uf, &task2);
            s.spawn(move || loop {
                let si = task2.fetch_add(1, Ordering::Relaxed) as usize;
                if si >= n { break; }
                let oi = order[si];
                if is_core[oi].load(Ordering::Relaxed) == 0 { continue; }
                let pt = &data[oi];
                let lo = sorted_c0.partition_point(|&v| v < pt[0] - eps);
                let hi = sorted_c0.partition_point(|&v| v <= pt[0] + eps);
                for sj in lo..hi {
                    if sj <= si { continue; } // process each pair once
                    let oj = order[sj];
                    if is_core[oj].load(Ordering::Relaxed) == 0 { continue; }
                    let d2: f64 = pt.iter().zip(data[oj].iter()).map(|(&a, &b)| (a - b) * (a - b)).sum();
                    if d2 <= eps2 { uf_union(&uf, oi, oj); }
                }
            });
        }
    });

    let mut labels = vec![-1i32; n];
    for i in 0..n {
        if is_core[i].load(Ordering::Relaxed) == 1 {
            labels[i] = uf_find(&uf, i) as i32;
        }
    }

    for i in 0..n {
        if labels[i] >= 0 { continue; }
        let pt = &data[i];
        let si = inv_order[i];
        let lo = sorted_c0.partition_point(|&v| v < pt[0] - eps);
        let hi = sorted_c0.partition_point(|&v| v <= pt[0] + eps);
        let mut best_root = u32::MAX;
        let mut best_d = f64::INFINITY;
        for sj in lo..hi {
            if sj == si { continue; }
            let oj = order[sj];
            if is_core[oj].load(Ordering::Relaxed) == 0 { continue; }
            let d2: f64 = pt.iter().zip(data[oj].iter()).map(|(&a, &b)| (a - b) * (a - b)).sum();
            if d2 <= eps2 && d2 < best_d { best_d = d2; best_root = uf_find(&uf, oj) as u32; }
        }
        if best_root != u32::MAX { labels[i] = best_root as i32; }
    }

    let mut rm = std::collections::HashMap::<i32, i32>::new();
    let mut cid = 0i32;
    for l in labels.iter_mut() {
        if *l >= 0 {
            let e = rm.entry(*l).or_insert_with(|| { let v = cid; cid += 1; v });
            *l = *e;
        }
    }

    (labels, cid as usize)
}

#[inline(always)]
fn dist2_flat(a: &[f64], b: &[f64], p: usize) -> f64 {
    let mut s = 0.0;
    for j in 0..p {
        let d = unsafe { *a.get_unchecked(j) - *b.get_unchecked(j) };
        s += d * d;
    }
    s
}

pub fn dbscan_core_nd_flat(data: &[f64], n: usize, p: usize, eps: f64, min_samples: usize) -> (Vec<i32>, usize) {
    use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};
    if n == 0 { return (Vec::new(), 0); }
    if p == 0 { return (vec![-1; n], 0); }

    let eps2 = eps * eps;
    let ms = min_samples as u32;
    let ncpu = std::thread::available_parallelism().map(|v| v.get()).unwrap_or(4).min(16);

    fn uf_find(uf: &[AtomicU32], mut x: usize) -> usize {
        loop {
            let pp = uf[x].load(Ordering::Relaxed) as usize;
            if pp == x { return x; }
            let gp = uf[pp].load(Ordering::Relaxed) as usize;
            let _ = uf[x].compare_exchange_weak(pp as u32, gp as u32, Ordering::Relaxed, Ordering::Relaxed);
            x = pp;
        }
    }
    fn uf_union(uf: &[AtomicU32], a: usize, b: usize) {
        loop {
            let (ra, rb) = (uf_find(uf, a), uf_find(uf, b));
            if ra == rb { return; }
            let (lo, hi) = if ra < rb { (ra, rb) } else { (rb, ra) };
            if uf[hi].compare_exchange_weak(hi as u32, lo as u32, Ordering::AcqRel, Ordering::Relaxed).is_ok() { return; }
        }
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_unstable_by(|&a, &b| data[a * p].partial_cmp(&data[b * p]).unwrap_or(std::cmp::Ordering::Equal));
    let sorted_c0: Vec<f64> = order.iter().map(|&oi| data[oi * p]).collect();
    let mut inv_order = vec![0usize; n];
    for (si, &oi) in order.iter().enumerate() { inv_order[oi] = si; }

    let is_core: Vec<AtomicU8> = (0..n).map(|_| AtomicU8::new(0)).collect();
    let task1 = AtomicU32::new(0);
    std::thread::scope(|s| {
        for _ in 0..ncpu {
            let (data, order, sorted_c0, is_core, task1) = (&data, &order, &sorted_c0, &is_core, &task1);
            s.spawn(move || loop {
                let si = task1.fetch_add(1, Ordering::Relaxed) as usize;
                if si >= n { break; }
                let oi = order[si];
                let pt = &data[oi * p..(oi + 1) * p];
                let c0 = pt[0];
                let lo = sorted_c0.partition_point(|&v| v < c0 - eps);
                let hi = sorted_c0.partition_point(|&v| v <= c0 + eps);
                let mut cnt = 0u32;
                for sj in lo..hi {
                    let oj = order[sj];
                    let q = &data[oj * p..(oj + 1) * p];
                    if dist2_flat(pt, q, p) <= eps2 {
                        cnt += 1;
                        if cnt >= ms { is_core[oi].store(1, Ordering::Relaxed); break; }
                    }
                }
            });
        }
    });

    let uf: Vec<AtomicU32> = (0..n).map(|i| AtomicU32::new(i as u32)).collect();
    let task2 = AtomicU32::new(0);
    std::thread::scope(|s| {
        for _ in 0..ncpu {
            let (data, order, sorted_c0, is_core, uf, task2) = (&data, &order, &sorted_c0, &is_core, &uf, &task2);
            s.spawn(move || loop {
                let si = task2.fetch_add(1, Ordering::Relaxed) as usize;
                if si >= n { break; }
                let oi = order[si];
                if is_core[oi].load(Ordering::Relaxed) == 0 { continue; }
                let pt = &data[oi * p..(oi + 1) * p];
                let c0 = pt[0];
                let lo = sorted_c0.partition_point(|&v| v < c0 - eps);
                let hi = sorted_c0.partition_point(|&v| v <= c0 + eps);
                for sj in lo..hi {
                    if sj <= si { continue; }
                    let oj = order[sj];
                    if is_core[oj].load(Ordering::Relaxed) == 0 { continue; }
                    let q = &data[oj * p..(oj + 1) * p];
                    if dist2_flat(pt, q, p) <= eps2 { uf_union(&uf, oi, oj); }
                }
            });
        }
    });

    let mut labels = vec![-1i32; n];
    for i in 0..n {
        if is_core[i].load(Ordering::Relaxed) == 1 {
            labels[i] = uf_find(&uf, i) as i32;
        }
    }

    for i in 0..n {
        if labels[i] >= 0 { continue; }
        let pt = &data[i * p..(i + 1) * p];
        let si = inv_order[i];
        let c0 = pt[0];
        let lo = sorted_c0.partition_point(|&v| v < c0 - eps);
        let hi = sorted_c0.partition_point(|&v| v <= c0 + eps);
        let mut best_root = u32::MAX;
        let mut best_d = f64::INFINITY;
        for sj in lo..hi {
            if sj == si { continue; }
            let oj = order[sj];
            if is_core[oj].load(Ordering::Relaxed) == 0 { continue; }
            let q = &data[oj * p..(oj + 1) * p];
            let d2 = dist2_flat(pt, q, p);
            if d2 <= eps2 && d2 < best_d { best_d = d2; best_root = uf_find(&uf, oj) as u32; }
        }
        if best_root != u32::MAX { labels[i] = best_root as i32; }
    }

    let mut rm = std::collections::HashMap::<i32, i32>::new();
    let mut cid = 0i32;
    for l in labels.iter_mut() {
        if *l >= 0 {
            let e = rm.entry(*l).or_insert_with(|| { let v = cid; cid += 1; v });
            *l = *e;
        }
    }
    (labels, cid as usize)
}

fn render_dbscan_canvas(
    title: &str,
    x_values: &[f64],
    y_values: &[f64],
    labels: &[i32],
    n_clusters: usize,
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

    let ng = n_clusters + 1;
    let legend_w = 170i32;
    let pad_l = 56i32; let pad_t = 36i32; let pad_b = 48i32;
    let pad_r_actual = 20i32 + legend_w;
    let plot_w = width - pad_l - pad_r_actual;
    let plot_h = height - pad_t - pad_b;

    let mut group_names: Vec<String> = Vec::with_capacity(ng);
    for i in 0..n_clusters { group_names.push(format!("Cluster {}", i)); }
    group_names.push("Bruit (Noise)".into());

    let group_colors: Vec<String> = (0..ng).map(|i| {
        let hx = hex6(palette_color(palette, i));
        format!("#{}{}{}{}{}{}", hx[0] as char, hx[1] as char, hx[2] as char, hx[3] as char, hx[4] as char, hx[5] as char)
    }).collect();

    let inv_rx = plot_w as f64 / range_x;
    let inv_ry = plot_h as f64 / range_y;

    let noise_gi = n_clusters;
    let mut group_raw: Vec<Vec<(i16, i16)>> = vec![Vec::new(); ng];
    for gi in 0..ng {
        let est = n / ng + 64;
        group_raw[gi].reserve(est);
    }
    // Deduplicate at pixel level for large datasets: no visual difference,
    // but drastically reduces HTML size and encoding time.
    let dedup = n > 50_000;
    let mut seen: std::collections::HashSet<u64> = if dedup {
        std::collections::HashSet::with_capacity((plot_w * plot_h) as usize)
    } else {
        std::collections::HashSet::new()
    };
    for i in 0..n {
        let px = ((x_values[i] - min_x) * inv_rx).clamp(0.0, (plot_w - 1) as f64) as u32;
        let py = (plot_h as f64 - (y_values[i] - min_y) * inv_ry).clamp(0.0, (plot_h - 1) as f64) as u32;
        let gi = if labels[i] < 0 { noise_gi } else { labels[i] as usize }.min(ng - 1);
        if dedup {
            let key = ((gi as u64) << 40) | ((px as u64) << 20) | (py as u64);
            if !seen.insert(key) { continue; }
        }
        group_raw[gi].push((px as i16, py as i16));
    }

    let group_b64: Vec<String> = group_raw.iter().map(|pts| {
        let mut raw = Vec::with_capacity(pts.len() * 4);
        for &(px, py) in pts {
            raw.extend_from_slice(&px.to_le_bytes());
            raw.extend_from_slice(&py.to_le_bytes());
        }
        b64_encode(&raw)
    }).collect();

    let hid = html_id();
    let cv_id = format!("spcv{}", hid);
    let tip_id = format!("sptip{}", hid);
    let mut buf = Vec::<u8>::with_capacity(n / 4 + 12_000);
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
    for (gi, b64) in group_b64.iter().enumerate() {
        if gi > 0 { buf.push(b','); }
        buf.push(b'\'');
        buf.extend_from_slice(b64.as_bytes());
        buf.push(b'\'');
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
        buf.push(b'\''); push_js_str(&mut buf, if nm.len() > 22 { &nm[..22] } else { nm }); buf.push(b'\'');
    }
    push_b(&mut buf, b"];");
    let gridlines_js = if gridlines { b"1" as &[u8] } else { b"0" };
    push_b(&mut buf, b"var GL="); buf.extend_from_slice(gridlines_js); push_b(&mut buf, b";");
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
    let leg_x = pad_l + plot_w + 12;
    let leg_top = pad_t + 8;
    push_b(&mut buf, b"ctx.textAlign='left';ctx.font='11px Arial';");
    for gi in 0..group_names.len() {
        let ly = leg_top + gi as i32 * 22;
        push_b(&mut buf, b"if(!hidden["); push_i(&mut buf, gi as i32); push_b(&mut buf, b"]){");
        push_b(&mut buf, b"ctx.globalAlpha=1;}else{ctx.globalAlpha=0.28;}");
        push_b(&mut buf, b"ctx.fillStyle='"); push_js_str(&mut buf, &group_colors[gi]); push_b(&mut buf, b"';");
        push_b(&mut buf, b"ctx.beginPath();ctx.arc("); push_i(&mut buf, leg_x + 6); buf.push(b','); push_i(&mut buf, leg_top + gi as i32 * 22 + 6);
        push_b(&mut buf, b",6,0,2*Math.PI);ctx.fill();");
        push_b(&mut buf, b"ctx.globalAlpha=1;ctx.fillStyle='#374151';ctx.fillText(GN["); push_i(&mut buf, gi as i32); push_b(&mut buf, b"]||'',");
        push_i(&mut buf, leg_x + 17); buf.push(b','); push_i(&mut buf, ly + 11); push_b(&mut buf, b");");
    }
    push_b(&mut buf, b"ctx.globalAlpha=1;");
    push_b(&mut buf, b"}");
    push_b(&mut buf, b"draw();");
    push_b(&mut buf, b"cv.addEventListener('click',function(e){");
    push_b(&mut buf, b"var r=cv.getBoundingClientRect(),mx=e.clientX-r.left,my=e.clientY-r.top;");
    push_b(&mut buf, b"var lx="); push_i(&mut buf, leg_x); push_b(&mut buf, b",lt="); push_i(&mut buf, leg_top); push_b(&mut buf, b",ng="); push_i(&mut buf, ng as i32); push_b(&mut buf, b";");
    push_b(&mut buf, b"for(var gi=0;gi<ng;gi++){var ly=lt+gi*22;if(mx>=lx&&mx<=lx+150&&my>=ly&&my<=ly+18){hidden[gi]=!hidden[gi];draw();return;}}");
    push_b(&mut buf, b"});");
    push_b(&mut buf, b"var allPts=[];for(var gi=0;gi<GD.length;gi++){if(!GD[gi])continue;var a=b64(GD[gi]);for(var i=0;i<a.length;i+=2)allPts.push([gi,pL+a[i],pT+a[i+1]]);}");
    push_b(&mut buf, b"cv.addEventListener('mousemove',function(e){");
    push_b(&mut buf, b"var r=cv.getBoundingClientRect(),mx=e.clientX-r.left,my=e.clientY-r.top;");
    push_b(&mut buf, b"if(mx<pL||mx>pL+pW||my<pT||my>pT+pH){tip.style.opacity=0;return;}");
    push_b(&mut buf, b"var best=null,bd=1e9;");
    push_b(&mut buf, b"for(var i=0;i<allPts.length;i++){var p=allPts[i];if(hidden[p[0]])continue;var dx=p[1]-mx,dy=p[2]-my,d=dx*dx+dy*dy;if(d<bd){bd=d;best=p;}}");
    push_b(&mut buf, b"if(!best||bd>400){tip.style.opacity=0;return;}");
    push_b(&mut buf, b"var xv=(best[1]-pL)/pW*rX+minX,yv=(1-(best[2]-pT)/pH)*rY+minY;");
    push_b(&mut buf, b"var gname=GN[best[0]]?'<br><span style=\"color:#94a3b8\">'+GN[best[0]]+'</span>':'';");
    push_b(&mut buf, b"tip.innerHTML='<b>x:</b> '+xv.toFixed(2)+'&nbsp;&nbsp;<b>y:</b> '+yv.toFixed(2)+gname;");
    push_b(&mut buf, b"var tx=best[1]+12,ty=best[2]-28;");
    push_b(&mut buf, b"if(tx+160>W)tx=best[1]-170;if(ty<0)ty=best[2]+8;");
    push_b(&mut buf, b"tip.style.left=tx+'px';tip.style.top=ty+'px';tip.style.opacity=1;");
    push_b(&mut buf, b"});");
    push_b(&mut buf, b"cv.addEventListener('mouseleave',function(){tip.style.opacity=0;});");
    push_b(&mut buf, b"})();</script>");
    html_suffix(&mut buf, hid, "[]");
    unsafe { String::from_utf8_unchecked(buf) }
}

pub fn render_dbscan_html(
    title: &str,
    x_values: &[f64],
    y_values: &[f64],
    eps: f64,
    min_samples: usize,
    x_label: &str,
    y_label: &str,
    width: i32,
    height: i32,
    gridlines: bool,
    normalize: bool,
    palette: &[u32],
) -> String {
    let n = x_values.len().min(y_values.len());
    if n == 0 { return String::new(); }

    let (xn, yn) = if normalize {
        let (xmin, xmax) = crate::bindings::utils::simd_ops::find_minmax(x_values);
        let (ymin, ymax) = crate::bindings::utils::simd_ops::find_minmax(y_values);
        let rx = (xmax - xmin).max(1e-12);
        let ry = (ymax - ymin).max(1e-12);
        let xn: Vec<f64> = x_values.iter().map(|&v| (v - xmin) / rx).collect();
        let yn: Vec<f64> = y_values.iter().map(|&v| (v - ymin) / ry).collect();
        (xn, yn)
    } else {
        (x_values.to_vec(), y_values.to_vec())
    };

    let (labels, n_clusters) = dbscan_core(&xn, &yn, eps, min_samples);
    let n_noise = labels.iter().filter(|&&l| l < 0).count();

    let full_title = format!("{} \u{2014} {} clusters, {} pts bruit ({n} total)", title, n_clusters, n_noise);

    render_dbscan_canvas(
        &full_title, x_values, y_values, &labels, n_clusters, palette,
        x_label, y_label, width, height, gridlines,
    )
}



#[crate::sera_alias("dbscan", "dbscans", "dbscan_chart", "DBSCAN")]
#[crate::sera_builder]
pub fn build_dbscan_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let eps = o.eps.unwrap_or(0.5);
    let min_samples = o.min_samples.unwrap_or(5);
    let normalize = o.normalize.unwrap_or(false);
    let pal = o.pal();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_dbscan_html(
        title, &x, &y, eps, min_samples, &o.xl(), &o.yl(),
        o.w(900), o.h(540), o.grid(), normalize, &pal,
    );
    crate::html::hover::apply_opts(html, bg_str.as_deref(), !o.no_x(), !o.no_y())
}

#[crate::sera_alias("dbscan3d", "dbscan_3d", "dbscan3d_chart")]
#[crate::sera_builder]
pub fn build_dbscan_chart_3d(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let eps = o.eps.unwrap_or(0.5);
    let min_samples = o.min_samples.unwrap_or(5);
    let normalize = o.normalize.unwrap_or(false);
    let n = x.len().min(y.len()).min(z.len());
    let (xn, yn, zn) = if normalize && n > 0 {
        let norm = |v: &[f64]| {
            let mn = v.iter().cloned().fold(f64::INFINITY, f64::min);
            let mx = v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let r = (mx - mn).max(1e-12);
            v.iter().map(|&val| (val - mn) / r).collect::<Vec<_>>()
        };
        (norm(&x[..n]), norm(&y[..n]), norm(&z[..n]))
    } else {
        (x[..n].to_vec(), y[..n].to_vec(), z[..n].to_vec())
    };
    let data: Vec<Vec<f64>> = (0..n).map(|i| vec![xn[i], yn[i], zn[i]]).collect();
    let (labels, _) = crate::plot::default::scatter::dbscan_core_nd(&data, eps, min_samples);
    let color_labels: Vec<String> = labels.iter().map(|&l| {
        if l < 0 { "Noise".to_string() } else { format!("Cluster {}", l + 1) }
    }).collect();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_scatter3d_html(
        title, &xn, &yn, &zn, (&o.xl(), &o.yl(), &o.zl()), &[], &color_labels,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}
