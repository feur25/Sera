use crate::html::js_3d::render_3d_html_impl;
use crate::plot::{apply_bg3d, parse_all};

const RING_GAP_CONTROL_JS: &str = "(function(){var wrap=document.getElementById(cid);if(!wrap)return;var outer=document.createElement('div');outer.style.cssText='display:flex;flex-direction:column;align-items:center;gap:10px;max-width:100%';wrap.parentElement.insertBefore(outer,wrap);outer.appendChild(wrap);var ctrl=document.createElement('div');ctrl.style.cssText='width:'+W+'px;max-width:100%;box-sizing:border-box;display:flex;align-items:center;gap:10px;font:11px -apple-system,BlinkMacSystemFont,\"Segoe UI\",Roboto,sans-serif;color:#cbd5e1;background:rgba(15,23,42,.6);padding:7px 12px;border-radius:9px';var lbl=document.createElement('span');lbl.textContent='ring gap';lbl.style.cssText='white-space:nowrap;opacity:.75';var sl=document.createElement('input');sl.type='range';sl.min='0';sl.max='200';sl.step='1';sl.value=String(Math.round(RG*200));sl.style.cssText='flex:1;cursor:pointer';var val=document.createElement('span');val.textContent=RG.toFixed(2);val.style.cssText='width:32px;text-align:right;font-weight:700;color:#f1f5f9';sl.addEventListener('input',function(){RG=parseInt(sl.value,10)/200;val.textContent=RG.toFixed(2);R();});ctrl.appendChild(lbl);ctrl.appendChild(sl);ctrl.appendChild(val);outer.appendChild(ctrl);})();";

pub fn render_radar3d_html(
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
    scene: &str,
    ring_gap: f64,
) -> String {
    let extra_js = format!("var RG={:.4};{}", ring_gap, RING_GAP_CONTROL_JS);
    let html = render_3d_html_impl(
        3,
        title,
        x,
        y,
        z,
        axis_labels,
        colors,
        color_labels,
        w,
        h,
        bg_color,
        scene,
        extra_js.as_bytes(),
    );
    html.replacen(
        "<body>",
        &format!("<body><div height=\"{}\" style=\"display:none\"></div>", h + 60),
        1,
    )
}

#[crate::chart_demo("axes=[\"Python\",\"Rust\",\"SQL\",\"ML\",\"DevOps\"], series=[[9,7,8,8,6],[5,10,6,4,9]], series_names=[\"Alice\",\"Bob\"]")]
#[crate::params(paramsList[
    "title", "axes", "series", "series_names", "palette", "bg_color", "scene", "orientation3d",
    "width", "height", "max_val", "fill_opacity", "ring_gap"
])]
#[crate::sera_alias("radar3d", "radar_3d", "radar3d_chart", "radar3d_family")]
#[crate::sera_builder]
pub fn build_radar3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
    let n_axes = axes.len();
    if n_axes == 0 {
        return String::new();
    }
    let names: Vec<String> = o
        .series_names
        .clone()
        .unwrap_or_else(|| (0..series_flat.len()).map(|_| String::new()).collect());
    let n_series = names.len().min(series_flat.len());
    let ring_gap = o.ring_gap.unwrap_or(1.0).clamp(0.0, 1.0);
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    for si in 0..n_series {
        let vals = &series_flat[si];
        let max_val = vals.iter().cloned().fold(0.0f64, f64::max).max(1e-9);
        for ai in 0..n_axes.min(vals.len()) {
            let angle = std::f64::consts::TAU * ai as f64 / n_axes as f64;
            let r = vals[ai] / max_val;
            xv.push(angle.cos() * r);
            yv.push(si as f64);
            zv.push(angle.sin() * r);
            cv.push(si as f64);
        }
    }
    let bg_str = o.bg_str();
    apply_bg3d(
        crate::plot::statistical::_3d::render_radar3d_html(
            title,
            &xv,
            &yv,
            &zv,
            (&o.xl(), &o.yl(), &o.zl()),
            &cv,
            &names,
            o.w(900),
            o.h(560),
            bg_str.as_deref(),
            &o.scene3d(),
            ring_gap,
        ),
        &o,
    )
}
