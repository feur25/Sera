use super::common::{color_hex, finalize, label_in_rect, node_data_attrs, node_rect, open_svg, prepare};
use super::config::IcicleConfig;
use crate::plot::statistical::common::push_b;

#[crate::chart_demo("labels=[\"Root\",\"A\",\"B\",\"A1\",\"A2\",\"B1\",\"B2\"], parents=[\"\",\"Root\",\"Root\",\"A\",\"A\",\"B\",\"B\"], values=[0,40,30,20,20,15,15]")]

pub fn render(cfg: &IcicleConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.n * 240 + 1536);
    open_svg(&mut b, cfg);
    let order = p.bfs_order.clone();
    for i in order {
        let r = node_rect(&p, i);
        if r.w < 0.5 {
            continue;
        }
        let hx = color_hex(&p, i);
        let opacity: &[u8] = match p.depth[i] {
            0 => b"0.92",
            1 => b"0.80",
            2 => b"0.70",
            _ => b"0.62",
        };
        push_b(&mut b, b"<rect");
        node_data_attrs(&mut b, &p, i);
        super::common::rect_attrs(&mut b, r);
        push_b(&mut b, b" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" opacity=\"");
        b.extend_from_slice(opacity);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1\"/>");
        label_in_rect(&mut b, &p, i, r, true);
    }
    finalize(b, cfg)
}
