use crate::html::hover::{html_id, html_prefix, html_suffix};
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};

pub struct NativeChartOpts<'a> {
    pub width: i32,
    pub height: i32,
    pub color_hex: u32,
    pub gridlines: bool,
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub show_regression: bool,
    pub regression_type: &'a str,
    pub cols: i32,
    pub categories: &'a [String],
    pub variant: &'a str,
}

pub struct NativeChartEntry {
    pub name: &'static str,
    pub threshold: usize,
    pub render: fn(&str, &[f64], &[f64], &NativeChartOpts) -> (String, u64),
}
inventory::collect!(NativeChartEntry);

pub struct LabeledChartEntry {
    pub name: &'static str,
    pub threshold: usize,
    pub render: fn(&str, &[String], &[f64], &NativeChartOpts) -> (String, u64),
}
inventory::collect!(LabeledChartEntry);

inventory::submit! { LabeledChartEntry { name: "build_bar", threshold: 500, render: crate::plot::default::bar::render_bar_family_native } }
inventory::submit! { LabeledChartEntry { name: "build_heatmap", threshold: 2_000, render: crate::plot::statistical::heatmap::render_heatmap_family_native } }

pub struct OhlcChartEntry {
    pub name: &'static str,
    pub threshold: usize,
    #[allow(clippy::type_complexity)]
    pub render: fn(&str, &[String], &[f64], &[f64], &[f64], &[f64], &NativeChartOpts) -> (String, u64),
}
inventory::collect!(OhlcChartEntry);

inventory::submit! { OhlcChartEntry { name: "build_candlestick", threshold: 500, render: crate::plot::statistical::candlestick::render_candlestick_family_native } }

pub struct HierarchyChartEntry {
    pub name: &'static str,
    pub threshold: usize,
    #[allow(clippy::type_complexity)]
    pub render: fn(&str, &[String], &[String], &[f64], &NativeChartOpts) -> (String, u64, f64, f64),
}
inventory::collect!(HierarchyChartEntry);

inventory::submit! { HierarchyChartEntry { name: "build_icicle", threshold: 500, render: crate::plot::statistical::icicle::render_icicle_family_native } }

pub struct SeriesChartEntry {
    pub name: &'static str,
    pub threshold: usize,
}
inventory::collect!(SeriesChartEntry);

inventory::submit! { SeriesChartEntry { name: "build_line", threshold: 20 } }
inventory::submit! { SeriesChartEntry { name: "build_line_chart", threshold: 20 } }

pub fn render_line_family_native(
    title: &str,
    x_values: &[f64],
    y_values: &[f64],
    opts: &NativeChartOpts,
) -> String {
    render_line_family_native_id(title, x_values, y_values, opts).0
}

pub fn render_line_family_native_id(
    title: &str,
    x_values: &[f64],
    y_values: &[f64],
    opts: &NativeChartOpts,
) -> (String, u64) {
    let spec = CanvasPlotSpec {
        title,
        width: opts.width,
        height: opts.height,
        x_label: opts.x_label,
        y_label: opts.y_label,
        gridlines: opts.gridlines,
        mode: MODE_LINE,
        color_hex: if opts.color_hex != 0 { opts.color_hex } else { 0x636EFA },
    };
    render_canvas_points_html_id(&spec, x_values, y_values)
}

inventory::submit! { NativeChartEntry { name: "build_line", threshold: 3_000, render: render_line_family_native_id } }
inventory::submit! { NativeChartEntry { name: "build_line_chart", threshold: 3_000, render: render_line_family_native_id } }
inventory::submit! { NativeChartEntry { name: "build_area_chart", threshold: 3_000, render: render_line_family_native_id } }
inventory::submit! { NativeChartEntry { name: "build_scatter_chart", threshold: 3_000, render: crate::plot::default::scatter::render_scatter_family_native_id } }
inventory::submit! { NativeChartEntry { name: "build_bubble", threshold: 3_000, render: crate::plot::default::scatter::render_scatter_family_native_id } }

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

const PAR_B64_THRESHOLD: usize = 196_608;
const PAR_PACK_THRESHOLD: usize = 50_000;

pub fn b64_encode(data: &[u8]) -> String {
    use base64::Engine;
    if data.len() < PAR_B64_THRESHOLD {
        return base64::engine::general_purpose::STANDARD.encode(data);
    }
    use rayon::prelude::*;
    const CHUNK: usize = 3 * 65_536;
    data.par_chunks(CHUNK)
        .map(|c| base64::engine::general_purpose::STANDARD.encode(c))
        .collect::<Vec<String>>()
        .concat()
}

pub fn pack_indices_u32_raw(indices: &[u32]) -> Vec<u8> {
    let mut raw = vec![0u8; indices.len() * 4];
    for (i, v) in indices.iter().enumerate() {
        raw[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
    raw
}

pub fn pack_indices_u32(indices: &[u32]) -> String {
    b64_encode(&pack_indices_u32_raw(indices))
}

pub fn push_delta_patch_js(buf: &mut Vec<u8>, hid: u64, target_expr: &[u8]) {
    push_b(buf, b"function b64u32(s){var b=atob(s),n=b.length,a=new Uint32Array(n/4);for(var i=0;i<n;i+=4)a[i/4]=b.charCodeAt(i)|(b.charCodeAt(i+1)<<8)|(b.charCodeAt(i+2)<<16)|(b.charCodeAt(i+3)<<24);return a;}");
    push_b(buf, b"window['sp_apply_");
    buf.extend_from_slice(hid.to_string().as_bytes());
    push_b(buf, b"']=function(idx,pts){");
    push_b(buf, b"var tgt=");
    buf.extend_from_slice(target_expr);
    push_b(buf, b";");
    push_b(buf, b"for(var i=0;i<idx.length;i++){tgt[idx[i]*2]=pts[i*2];tgt[idx[i]*2+1]=pts[i*2+1];}");
    push_b(buf, b"GDIRTY=1;draw();};");
    push_b(buf, b"window['sp_push_");
    buf.extend_from_slice(hid.to_string().as_bytes());
    push_b(buf, b"']=function(idxB64,ptsB64){window['sp_apply_");
    buf.extend_from_slice(hid.to_string().as_bytes());
    push_b(buf, b"'](b64u32(idxB64),b64(ptsB64));};");
}

#[allow(clippy::too_many_arguments)]
pub fn nearest_grid_js(
    buf: &mut Vec<u8>,
    n_expr: &[u8],
    x_expr: &[u8],
    y_expr: &[u8],
    bx0: &[u8],
    by0: &[u8],
    bw_expr: &[u8],
    bh_expr: &[u8],
    skip_expr: &[u8],
    rebuild_stmt: &[u8],
) {
    push_b(buf, b"var GC=Math.min(1024,Math.max(1,Math.ceil(Math.sqrt((");
    buf.extend_from_slice(n_expr);
    push_b(buf, b")/2)))),GcW=(");
    buf.extend_from_slice(bw_expr);
    push_b(buf, b")/GC,GcH=(");
    buf.extend_from_slice(bh_expr);
    push_b(buf, b")/GC,GH=null,GS=null,GDIRTY=1;");

    push_b(buf, b"function buildGrid(){");
    buf.extend_from_slice(rebuild_stmt);
    push_b(buf, b"var n=");
    buf.extend_from_slice(n_expr);
    push_b(buf, b";var cnt=new Int32Array(GC*GC+1);");
    push_b(buf, b"function cellOf(i){var px=(");
    buf.extend_from_slice(x_expr);
    push_b(buf, b")-(");
    buf.extend_from_slice(bx0);
    push_b(buf, b"),py=(");
    buf.extend_from_slice(y_expr);
    push_b(buf, b")-(");
    buf.extend_from_slice(by0);
    push_b(buf, b");var gx=px/GcW|0;if(gx<0)gx=0;if(gx>=GC)gx=GC-1;var gy=py/GcH|0;if(gy<0)gy=0;if(gy>=GC)gy=GC-1;return gy*GC+gx;}");
    push_b(buf, b"for(var i=0;i<n;i++)cnt[cellOf(i)+1]++;");
    push_b(buf, b"for(var c=0;c<GC*GC;c++)cnt[c+1]+=cnt[c];var pos=cnt.slice(),idx=new Int32Array(n);");
    push_b(buf, b"for(var i=0;i<n;i++){var c=cellOf(i);idx[pos[c]++]=i;}GH=cnt;GS=idx;GDIRTY=0;}");

    push_b(buf, b"function nearest(qx,qy){var n=");
    buf.extend_from_slice(n_expr);
    push_b(buf, b";if(n<=0)return -1;if(!GS||GDIRTY)buildGrid();");
    push_b(buf, b"var lx=qx-(");
    buf.extend_from_slice(bx0);
    push_b(buf, b"),ly=qy-(");
    buf.extend_from_slice(by0);
    push_b(buf, b");var gx=lx/GcW|0;if(gx<0)gx=0;if(gx>=GC)gx=GC-1;var gy=ly/GcH|0;if(gy<0)gy=0;if(gy>=GC)gy=GC-1;");
    push_b(buf, b"var bd=1e18,bi=-1;");
    push_b(buf, b"for(var ring=0;ring<=GC;ring++){var x0=gx-ring;if(x0<0)x0=0;var x1=gx+ring;if(x1>=GC)x1=GC-1;var y0=gy-ring;if(y0<0)y0=0;var y1=gy+ring;if(y1>=GC)y1=GC-1;");
    push_b(buf, b"for(var ny=y0;ny<=y1;ny++)for(var nx=x0;nx<=x1;nx++){var c=ny*GC+nx,s=GH[c],e=GH[c+1];");
    push_b(buf, b"for(var k=s;k<e;k++){var i=GS[k];");
    if !skip_expr.is_empty() {
        push_b(buf, b"if(");
        buf.extend_from_slice(skip_expr);
        push_b(buf, b")continue;");
    }
    push_b(buf, b"var dx=(");
    buf.extend_from_slice(x_expr);
    push_b(buf, b")-qx,dy=(");
    buf.extend_from_slice(y_expr);
    push_b(buf, b")-qy,d=dx*dx+dy*dy;if(d<bd){bd=d;bi=i;}}}");
    push_b(buf, b"if(x0===0&&y0===0&&x1===GC-1&&y1===GC-1)break;");
    push_b(buf, b"if(bi>=0){var eL=x0>0?lx-x0*GcW:1e18,eR=x1<GC-1?(x1+1)*GcW-lx:1e18,eT=y0>0?ly-y0*GcH:1e18,eB=y1<GC-1?(y1+1)*GcH-ly:1e18;");
    push_b(buf, b"var em=Math.min(eL,eR,eT,eB);if(em*em>=bd)break;}}");
    push_b(buf, b"return bi;}");
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
    b64_encode(&pack_points_i16_raw(
        x_values, y_values, min_x, range_x, min_y, range_y, plot_w, plot_h,
    ))
}

pub fn pack_points_i16_raw(
    x_values: &[f64],
    y_values: &[f64],
    min_x: f64,
    range_x: f64,
    min_y: f64,
    range_y: f64,
    plot_w: i32,
    plot_h: i32,
) -> Vec<u8> {
    let n = x_values.len().min(y_values.len());
    let inv_rx = plot_w as f64 / range_x.max(1e-12);
    let inv_ry = plot_h as f64 / range_y.max(1e-12);
    let plot_h_f = plot_h as f64;
    let mut raw = vec![0u8; n * 4];
    if n < PAR_PACK_THRESHOLD {
        for i in 0..n {
            let px = ((x_values[i] - min_x) * inv_rx) as i16;
            let py = (plot_h_f - (y_values[i] - min_y) * inv_ry) as i16;
            raw[i * 4..i * 4 + 2].copy_from_slice(&px.to_le_bytes());
            raw[i * 4 + 2..i * 4 + 4].copy_from_slice(&py.to_le_bytes());
        }
    } else {
        use rayon::prelude::*;
        raw.par_chunks_mut(4).enumerate().for_each(|(i, out)| {
            let px = ((x_values[i] - min_x) * inv_rx) as i16;
            let py = (plot_h_f - (y_values[i] - min_y) * inv_ry) as i16;
            out[0..2].copy_from_slice(&px.to_le_bytes());
            out[2..4].copy_from_slice(&py.to_le_bytes());
        });
    }
    raw
}

pub fn pack_scalar_i16_raw(values: &[f64], min_val: f64, range_val: f64, axis_px: i32) -> Vec<u8> {
    let n = values.len();
    let inv_r = axis_px as f64 / range_val.max(1e-12);
    let mut raw = vec![0u8; n * 2];
    if n < PAR_PACK_THRESHOLD {
        for i in 0..n {
            let v = ((values[i] - min_val) * inv_r) as i16;
            raw[i * 2..i * 2 + 2].copy_from_slice(&v.to_le_bytes());
        }
    } else {
        use rayon::prelude::*;
        raw.par_chunks_mut(2).enumerate().for_each(|(i, out)| {
            let v = ((values[i] - min_val) * inv_r) as i16;
            out.copy_from_slice(&v.to_le_bytes());
        });
    }
    raw
}

pub fn pack_scalar_i16(values: &[f64], min_val: f64, range_val: f64, axis_px: i32) -> String {
    b64_encode(&pack_scalar_i16_raw(values, min_val, range_val, axis_px))
}

pub fn pack_vector_i16_raw(components: &[&[f64]], min_val: f64, range_val: f64, axis_px: i32) -> Vec<u8> {
    let vpp = components.len();
    let n = components.first().map(|c| c.len()).unwrap_or(0);
    let inv_r = axis_px as f64 / range_val.max(1e-12);
    let mut raw = vec![0u8; n * vpp * 2];
    for i in 0..n {
        for (k, comp) in components.iter().enumerate() {
            let v = ((comp[i] - min_val) * inv_r) as i16;
            let off = (i * vpp + k) * 2;
            raw[off..off + 2].copy_from_slice(&v.to_le_bytes());
        }
    }
    raw
}

pub fn pack_vector_i16(components: &[&[f64]], min_val: f64, range_val: f64, axis_px: i32) -> String {
    b64_encode(&pack_vector_i16_raw(components, min_val, range_val, axis_px))
}

pub fn shared_minmax(arrays: &[&[f64]]) -> (f64, f64) {
    let mut min_val = f64::INFINITY;
    let mut max_val = f64::NEG_INFINITY;
    for arr in arrays {
        let (lo, hi) = crate::bindings::utils::simd_ops::find_minmax(arr);
        min_val = min_val.min(lo);
        max_val = max_val.max(hi);
    }
    if !min_val.is_finite() || !max_val.is_finite() {
        min_val = 0.0;
        max_val = 0.0;
    }
    (min_val, (max_val - min_val).max(1e-12))
}

pub fn push_scalar_patch_js(buf: &mut Vec<u8>, hid: u64, bars_expr: &[u8], axis_px: i32) {
    push_scalar_patch_js_oriented(buf, hid, bars_expr, axis_px, true);
}

pub fn push_scalar_patch_js_oriented(buf: &mut Vec<u8>, hid: u64, bars_expr: &[u8], axis_px: i32, vertical: bool) {
    push_b(buf, b"window['sp_apply_");
    buf.extend_from_slice(hid.to_string().as_bytes());
    push_b(buf, b"']=function(idx,vals){");
    push_b(buf, b"var bars=");
    buf.extend_from_slice(bars_expr);
    push_b(buf, b",axisPx=");
    buf.extend_from_slice(axis_px.to_string().as_bytes());
    push_b(buf, b";");
    push_b(buf, b"for(var i=0;i<idx.length;i++){var el=bars[idx[i]];if(!el)continue;var h=vals[i];");
    if vertical {
        push_b(buf, b"el.setAttribute('height',h);el.setAttribute('y',axisPx-h);}");
    } else {
        push_b(buf, b"el.setAttribute('width',h);}");
    }
    push_b(buf, b"};");
}

pub fn lerp_color(low: u32, high: u32, t: f64) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let lr = ((low >> 16) & 0xFF) as f64;
    let lg = ((low >> 8) & 0xFF) as f64;
    let lb = (low & 0xFF) as f64;
    let hr = ((high >> 16) & 0xFF) as f64;
    let hg = ((high >> 8) & 0xFF) as f64;
    let hb = (high & 0xFF) as f64;
    let r = (lr + (hr - lr) * t).round() as u32;
    let g = (lg + (hg - lg) * t).round() as u32;
    let b = (lb + (hb - lb) * t).round() as u32;
    (r << 16) | (g << 8) | b
}

pub fn push_color_patch_js(buf: &mut Vec<u8>, hid: u64, cells_expr: &[u8], scale_max: i32, color_low: u32, color_high: u32) {
    push_b(buf, b"window['sp_apply_");
    buf.extend_from_slice(hid.to_string().as_bytes());
    push_b(buf, b"']=function(idx,vals){");
    push_b(buf, b"var cells=");
    buf.extend_from_slice(cells_expr);
    push_b(buf, b",lr=");
    buf.extend_from_slice(((color_low >> 16) & 0xFF).to_string().as_bytes());
    push_b(buf, b",lg=");
    buf.extend_from_slice(((color_low >> 8) & 0xFF).to_string().as_bytes());
    push_b(buf, b",lb=");
    buf.extend_from_slice((color_low & 0xFF).to_string().as_bytes());
    push_b(buf, b",hr=");
    buf.extend_from_slice(((color_high >> 16) & 0xFF).to_string().as_bytes());
    push_b(buf, b",hg=");
    buf.extend_from_slice(((color_high >> 8) & 0xFF).to_string().as_bytes());
    push_b(buf, b",hb=");
    buf.extend_from_slice((color_high & 0xFF).to_string().as_bytes());
    push_b(buf, b",sm=");
    buf.extend_from_slice(scale_max.to_string().as_bytes());
    push_b(buf, b";");
    push_b(buf, b"function lp(a,b,t){return Math.round(a+(b-a)*t);}");
    push_b(buf, b"function colHex(t){var r=lp(lr,hr,t),g=lp(lg,hg,t),b=lp(lb,hb,t);return '#'+((1<<24)+(r<<16)+(g<<8)+b).toString(16).slice(1);}");
    push_b(buf, b"for(var i=0;i<idx.length;i++){var el=cells[idx[i]];if(!el)continue;el.setAttribute('fill',colHex(vals[i]/sm));}");
    push_b(buf, b"};");
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
    render_canvas_points_html_id(spec, x_values, y_values).0
}

pub fn render_canvas_points_html_id(spec: &CanvasPlotSpec, x_values: &[f64], y_values: &[f64]) -> (String, u64) {
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
    push_delta_patch_js(&mut buf, hid, b"PTS");

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

    nearest_grid_js(&mut buf, b"N", b"PTS[i*2]", b"PTS[i*2+1]", b"0", b"0", b"pW", b"pH", b"", b"");

    push_b(&mut buf, b"cv.addEventListener('wheel',function(e){e.preventDefault();var rc=cv.getBoundingClientRect();var mx=e.clientX-rc.left,my=e.clientY-rc.top;");
    push_b(&mut buf, b"var lx=(mx-pL-offX)/scale,ly=(my-pT-offY)/scale;var f=e.deltaY<0?1.18:1/1.18;");
    push_b(&mut buf, b"scale=Math.min(maxScale,Math.max(1,scale*f));offX=mx-pL-lx*scale;offY=my-pT-ly*scale;draw();},{passive:false});");
    push_b(&mut buf, b"var dragging=false,lastX=0,lastY=0;");
    push_b(&mut buf, b"cv.addEventListener('mousedown',function(e){dragging=true;lastX=e.clientX;lastY=e.clientY;cv.style.cursor='grabbing';});");
    push_b(&mut buf, b"window.addEventListener('mouseup',function(){dragging=false;cv.style.cursor='grab';});");
    push_b(&mut buf, b"cv.addEventListener('mousemove',function(e){var rc=cv.getBoundingClientRect();var mx=e.clientX-rc.left,my=e.clientY-rc.top;");
    push_b(&mut buf, b"if(dragging){offX+=e.clientX-lastX;offY+=e.clientY-lastY;lastX=e.clientX;lastY=e.clientY;draw();tip.style.opacity=0;return;}");
    push_b(&mut buf, b"if(mx<pL||mx>pL+pW||my<pT||my>pT+pH){tip.style.opacity=0;return;}");
    push_b(&mut buf, b"var lx=(mx-pL-offX)/scale,ly=(my-pT-offY)/scale;var bi=nearest(lx,ly);if(bi<0){tip.style.opacity=0;return;}");
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
    push_b(&mut buf, b"var lx=(mx-pL-offX)/scale,ly=(my-pT-offY)/scale;var bi=nearest(lx,ly);if(bi<0){tip.style.opacity=0;return;}");
    push_b(&mut buf, b"var dx=minX+(PTS[bi*2]/pW)*rX,dy=maxY-(PTS[bi*2+1]/pH)*rY;");
    push_b(&mut buf, b"tip.innerHTML='<b>'+fmt(dx)+'</b>, '+fmt(dy);tip.style.left=(mx+12)+'px';tip.style.top=(my-8)+'px';tip.style.opacity=1;");
    push_b(&mut buf, b"setTimeout(function(){tip.style.opacity=0;},2200);}});");

    push_b(&mut buf, b"draw();");
    push_b(&mut buf, b"})();</script>");

    html_suffix(&mut buf, hid, "[]");
    (unsafe { String::from_utf8_unchecked(buf) }, hid)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i16_at(raw: &[u8], i: usize) -> i16 {
        i16::from_le_bytes([raw[i * 2], raw[i * 2 + 1]])
    }

    #[test]
    fn pack_scalar_i16_raw_does_not_clamp_a_value_pushed_above_the_originally_established_range() {
        let raw = pack_scalar_i16_raw(&[500.0], 0.0, 100.0, 300);
        let v = i16_at(&raw, 0);
        assert!(v as f64 > 299.0, "a value 5x past the established range must encode past the old axis boundary, not clamp to it, got {v}");
    }

    #[test]
    fn pack_scalar_i16_raw_does_not_clamp_a_value_pushed_below_the_originally_established_range() {
        let raw = pack_scalar_i16_raw(&[-200.0], 0.0, 100.0, 300);
        let v = i16_at(&raw, 0);
        assert!(v < 0, "a value below min_val must encode to a negative pixel, not clamp to 0, got {v}");
    }

    #[test]
    fn pack_points_i16_raw_does_not_clamp_a_y_value_pushed_above_the_originally_established_range() {
        let raw = pack_points_i16_raw(&[50.0], &[500.0], 0.0, 100.0, 0.0, 100.0, 400, 300);
        let py = i16_at(&raw, 1);
        assert!(py < 0, "svg y-pixel space is inverted, so a y value far above the established range must encode to a negative pixel, not clamp to 0, got {py}");
    }

    #[test]
    fn pack_points_i16_raw_does_not_clamp_an_x_value_pushed_below_the_originally_established_range() {
        let raw = pack_points_i16_raw(&[-500.0], &[50.0], 0.0, 100.0, 0.0, 100.0, 400, 300);
        let px = i16_at(&raw, 0);
        assert!(px < 0, "an x value below min_x must encode to a negative pixel, not clamp to 0, got {px}");
    }

    #[test]
    fn pack_vector_i16_raw_does_not_clamp_a_component_pushed_above_the_originally_established_range() {
        let raw = pack_vector_i16_raw(&[&[900.0]], 0.0, 100.0, 300);
        let v = i16_at(&raw, 0);
        assert!(v as f64 > 299.0, "an ohlc component far past the established range must encode past the old axis boundary, not clamp to it, got {v}");
    }

    #[test]
    fn pack_scalar_i16_raw_round_trips_back_through_a_manual_inversion_for_an_extrapolated_value() {
        let min_val = 10.0;
        let range_val = 90.0;
        let axis_px = 300;
        let value = 260.0;
        let raw = pack_scalar_i16_raw(&[value], min_val, range_val, axis_px);
        let px = i16_at(&raw, 0);
        let back = min_val + (px as f64 / axis_px as f64) * range_val;
        assert!((back - value).abs() < 1.0, "encoding then inverting a far out-of-range value should recover ~{value}, got {back}");
    }
}
