use super::common::{color_hex, finalize, node_data_attrs, node_rect_horizontal, open_svg, prepare};
use super::config::IcicleConfig;
use crate::plot::statistical::common::{escape_xml, push_b, push_f2, truncate};

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
        let r = node_rect_horizontal(&p, i);
        if r.h < 0.5 {
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
        if r.w > 22.0 && r.h > 14.0 {
            let max_chars = ((r.w / 6.5) as usize).max(1);
            let label = truncate(&p.labels[i], max_chars);
            push_b(&mut b, b"<text x=\"");
            push_f2(&mut b, r.x + r.w / 2.0);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, r.y + r.h / 2.0 + 4.0);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#fff\" pointer-events=\"none\">");
            escape_xml(&mut b, label);
            push_b(&mut b, b"</text>");
        }
    }
    finalize(b, cfg)
}
