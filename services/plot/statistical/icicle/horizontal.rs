use super::common::{finalize, node_data_attrs, node_rect_horizontal, open_svg, prepare, shaded_color_hex};
use super::config::IcicleConfig;
use crate::plot::statistical::common::{escape_xml, push_b, push_f2, truncate};

#[crate::chart_demo(
    "labels=[\"Company\",\"Engineering\",\"Sales\",\"Marketing\",\"Operations\",\"Backend\",\"Frontend\",\"Data\",\"Enterprise\",\"SMB\",\"Content\",\"Growth\",\"HR\",\"Finance\",\"API\",\"Infra\",\"Web\",\"Mobile\",\"ML\",\"Analytics\",\"NA\",\"EMEA\"], parents=[\"\",\"Company\",\"Company\",\"Company\",\"Company\",\"Engineering\",\"Engineering\",\"Engineering\",\"Sales\",\"Sales\",\"Marketing\",\"Marketing\",\"Operations\",\"Operations\",\"Backend\",\"Backend\",\"Frontend\",\"Frontend\",\"Data\",\"Data\",\"Enterprise\",\"Enterprise\"], values=[0,0,0,0,0,18,14,8,15,10,7,11,5,7,10,8,9,5,5,3,9,6], variant=\"horizontal\""
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
        let r = node_rect_horizontal(&p, i);
        if r.h < 0.5 {
            continue;
        }
        let hx = shaded_color_hex(&p, i);
        let text_fill: &[u8] = if p.depth[i] < 2 { b"#fff" } else { b"#1f2937" };
        push_b(&mut b, b"<rect");
        node_data_attrs(&mut b, &p, i);
        super::common::rect_attrs(&mut b, r);
        push_b(&mut b, b" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1\"/>");
        if r.w > 22.0 && r.h > 14.0 {
            let max_chars = ((r.w / 6.5) as usize).max(1);
            let label = truncate(&p.labels[i], max_chars);
            push_b(&mut b, b"<text x=\"");
            push_f2(&mut b, r.x + r.w / 2.0);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, r.y + r.h / 2.0 + 4.0);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"");
            b.extend_from_slice(text_fill);
            push_b(&mut b, b"\" pointer-events=\"none\">");
            escape_xml(&mut b, label);
            push_b(&mut b, b"</text>");
        }
    }
    finalize(b, cfg)
}
