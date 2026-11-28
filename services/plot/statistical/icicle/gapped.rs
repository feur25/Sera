use super::common::{color_hex, finalize, label_in_rect, node_data_attrs, node_rect, open_svg, prepare, Rect};
use super::config::IcicleConfig;
use crate::plot::statistical::common::push_b;

#[crate::chart_demo(
    "labels=[\"Company\",\"Engineering\",\"Sales\",\"Marketing\",\"Operations\",\"Backend\",\"Frontend\",\"Data\",\"Enterprise\",\"SMB\",\"Content\",\"Growth\",\"HR\",\"Finance\",\"API\",\"Infra\",\"Web\",\"Mobile\",\"ML\",\"Analytics\",\"NA\",\"EMEA\"], parents=[\"\",\"Company\",\"Company\",\"Company\",\"Company\",\"Engineering\",\"Engineering\",\"Engineering\",\"Sales\",\"Sales\",\"Marketing\",\"Marketing\",\"Operations\",\"Operations\",\"Backend\",\"Backend\",\"Frontend\",\"Frontend\",\"Data\",\"Data\",\"Enterprise\",\"Enterprise\"], values=[0,0,0,0,0,18,14,8,15,10,7,11,5,7,10,8,9,5,5,3,9,6], variant=\"gapped\""
)]

pub fn render(cfg: &IcicleConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.n * 240 + 1536);
    open_svg(&mut b, cfg, &p.layout);
    let gap = 2.0;
    let order = p.bfs_order.clone();
    for i in order {
        let raw = node_rect(&p, i);
        if raw.w < 1.0 + gap * 2.0 {
            continue;
        }
        let r = Rect {
            x: raw.x + gap,
            y: raw.y + gap / 2.0,
            w: (raw.w - gap * 2.0).max(0.0),
            h: (raw.h - gap).max(0.0),
        };
        let hx = color_hex(&p, i);
        push_b(&mut b, b"<rect");
        node_data_attrs(&mut b, &p, i);
        super::common::rect_attrs(&mut b, r);
        push_b(&mut b, b" rx=\"3\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\"/>");
        label_in_rect(&mut b, &p, i, r, true);
    }
    finalize(b, cfg)
}
