use super::common::{prepare, open_svg, finalize, label_inside, tile_data_attrs, rect_attrs};
use super::config::TreemapConfig;
use crate::plot::statistical::common::{push_b, push_f2, hex6};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\",\"G\"], values=[40,25,20,10,5,8,12]")]

pub fn render(cfg: &TreemapConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let n = p.leaf_indices.len();
    let mut b = Vec::<u8>::with_capacity(n * 200 + 2048);
    open_svg(&mut b, cfg);
    let base: u32 = if p.palette.is_empty() { 0x6366F1 } else { p.palette[0] };
    let hx = hex6(base);
    for ri in 0..n {
        let r = p.rects[ri];
        if r.w < 0.5 || r.h < 0.5 { continue; }
        let frac = if n > 1 { (n - 1 - ri) as f64 / (n - 1) as f64 } else { 1.0 };
        let opacity = 0.35 + 0.6 * frac;
        push_b(&mut b, b"<rect");
        tile_data_attrs(&mut b, &p, ri);
        rect_attrs(&mut b, r);
        push_b(&mut b, b" rx=\"3\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" fill-opacity=\""); push_f2(&mut b, opacity);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1.5\"/>");
        label_inside(&mut b, &p, ri, b"#fff");
    }
    finalize(b, cfg)
}

