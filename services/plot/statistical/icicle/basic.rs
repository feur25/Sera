use super::common::{finalize, label_in_rect, node_data_attrs, node_rect, open_svg, prepare, shaded_color_hex};
use super::config::IcicleConfig;
use crate::plot::statistical::common::push_b;

#[crate::chart_demo(
    "labels=[\"Company\",\"Engineering\",\"Sales\",\"Marketing\",\"Operations\",\"Backend\",\"Frontend\",\"Data\",\"Enterprise\",\"SMB\",\"Content\",\"Growth\",\"HR\",\"Finance\",\"API\",\"Infra\",\"Web\",\"Mobile\",\"ML\",\"Analytics\",\"NA\",\"EMEA\"], parents=[\"\",\"Company\",\"Company\",\"Company\",\"Company\",\"Engineering\",\"Engineering\",\"Engineering\",\"Sales\",\"Sales\",\"Marketing\",\"Marketing\",\"Operations\",\"Operations\",\"Backend\",\"Backend\",\"Frontend\",\"Frontend\",\"Data\",\"Data\",\"Enterprise\",\"Enterprise\"], values=[0,0,0,0,0,18,14,8,15,10,7,11,5,7,10,8,9,5,5,3,9,6]"
)]

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
        let hx = shaded_color_hex(&p, i);
        push_b(&mut b, b"<rect");
        node_data_attrs(&mut b, &p, i);
        super::common::rect_attrs(&mut b, r);
        push_b(&mut b, b" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1\"/>");
        label_in_rect(&mut b, &p, i, r, p.depth[i] < 2);
    }
    finalize(b, cfg)
}
