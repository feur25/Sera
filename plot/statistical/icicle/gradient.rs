use super::common::{finalize, label_in_rect, node_data_attrs, node_rect, open_svg, prepare};
use super::config::IcicleConfig;
use crate::plot::statistical::common::{heat_color, hex6, push_b};

#[crate::chart_demo("labels=[\"Root\",\"A\",\"B\",\"A1\",\"A2\",\"B1\",\"B2\"], parents=[\"\",\"Root\",\"Root\",\"A\",\"A\",\"B\",\"B\"], values=[0,40,30,20,20,15,15]")]

pub fn render(cfg: &IcicleConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.n * 240 + 1536);
    open_svg(&mut b, cfg);
    let total = p.grand_total.max(1.0);
    let order = p.bfs_order.clone();
    for i in order {
        let r = node_rect(&p, i);
        if r.w < 0.5 {
            continue;
        }
        let t = (p.values_eff[i] / total).clamp(0.0, 1.0);
        let hx = hex6(heat_color(t));
        push_b(&mut b, b"<rect");
        node_data_attrs(&mut b, &p, i);
        super::common::rect_attrs(&mut b, r);
        push_b(&mut b, b" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1\"/>");
        label_in_rect(&mut b, &p, i, r, true);
    }
    finalize(b, cfg)
}
