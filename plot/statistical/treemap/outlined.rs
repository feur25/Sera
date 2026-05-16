use super::common::{prepare, open_svg, finalize, leaf_color, label_inside, tile_data_attrs, rect_attrs};
use super::config::TreemapConfig;
use crate::plot::statistical::common::{push_b, hex6};

pub const DEMO_KWARGS: &str = "labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\",\"G\"], values=[40,25,20,10,5,8,12]";
pub fn render(cfg: &TreemapConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.leaf_indices.len() * 220 + 2048);
    open_svg(&mut b, cfg);
    for ri in 0..p.leaf_indices.len() {
        let r = p.rects[ri];
        if r.w < 0.5 || r.h < 0.5 { continue; }
        let hx = hex6(leaf_color(&p, ri));
        push_b(&mut b, b"<rect");
        tile_data_attrs(&mut b, &p, ri);
        rect_attrs(&mut b, r);
        push_b(&mut b, b" rx=\"3\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" fill-opacity=\"0.18\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2\"/>");
        label_inside(&mut b, &p, ri, b"#1f2937");
    }
    finalize(b, cfg)
}


