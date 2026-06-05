use super::common::{
    fill_hex, finalize, label_inside, lerp_color, open_svg, prepare, rect_attrs, tile_data_attrs,
};
use super::config::TreemapConfig;
use crate::plot::statistical::common::push_b;

#[crate::chart_demo(
    "labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\",\"G\"], values=[40,25,20,10,5,8,12]"
)]

pub fn render(cfg: &TreemapConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.leaf_indices.len() * 200 + 2048);
    open_svg(&mut b, cfg);
    let vmin = p.leaf_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let vmax = p
        .leaf_values
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let span = (vmax - vmin).max(1e-9);
    let cold: u32 = 0x1E3A8A;
    let warm: u32 = 0xDC2626;
    for ri in 0..p.leaf_indices.len() {
        let r = p.rects[ri];
        if r.w < 0.5 || r.h < 0.5 {
            continue;
        }
        let t = (p.leaf_values[ri] - vmin) / span;
        let c = lerp_color(cold, warm, t);
        push_b(&mut b, b"<rect");
        tile_data_attrs(&mut b, &p, ri);
        rect_attrs(&mut b, r);
        push_b(&mut b, b" rx=\"2");
        fill_hex(&mut b, c);
        push_b(&mut b, b"\" stroke=\"#0f172a\" stroke-width=\"0.5\"/>");
        label_inside(&mut b, &p, ri, b"#fff");
    }
    finalize(b, cfg)
}
