use super::common::{
    fill_hex, finalize, label_inside, leaf_color, open_svg, prepare_with_pad, rect_attrs,
    tile_data_attrs,
};
use super::config::TreemapConfig;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2};

#[crate::chart_demo("labels=[\"Root\",\"A\",\"B\",\"A1\",\"A2\",\"B1\"], parents=[\"\",\"Root\",\"Root\",\"A\",\"A\",\"B\"], values=[0,40,30,20,20,30]")]

pub fn render(cfg: &TreemapConfig) -> String {
    let p = match prepare_with_pad(cfg, 14.0) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.leaf_indices.len() * 240 + 2048);
    open_svg(&mut b, cfg);
    if p.has_parents {
        for (name, gr) in p.parent_groups.iter() {
            let mut frame = *gr;
            frame.x -= 8.0;
            frame.y -= 22.0;
            frame.w += 16.0;
            frame.h += 30.0;
            if frame.w < 4.0 || frame.h < 4.0 {
                continue;
            }
            let pi = p.parent_color.get(name).copied().unwrap_or(p.palette[0]);
            let hx = hex6(pi);
            push_b(&mut b, b"<rect");
            rect_attrs(&mut b, frame);
            push_b(&mut b, b" rx=\"6\" fill=\"#");
            b.extend_from_slice(&hx);
            push_b(&mut b, b"\" fill-opacity=\"0.10\" stroke=\"#");
            b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke-width=\"1.4\"/>");
            push_b(&mut b, b"<text x=\"");
            push_f2(&mut b, frame.x + 10.0);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, frame.y + 14.0);
            push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#");
            b.extend_from_slice(&hx);
            push_b(&mut b, b"\" pointer-events=\"none\">");
            escape_xml(&mut b, name);
            push_b(&mut b, b"</text>");
        }
    }
    for ri in 0..p.leaf_indices.len() {
        let r = p.rects[ri];
        if r.w < 0.5 || r.h < 0.5 {
            continue;
        }
        push_b(&mut b, b"<rect");
        tile_data_attrs(&mut b, &p, ri);
        rect_attrs(&mut b, r);
        push_b(&mut b, b" rx=\"3");
        fill_hex(&mut b, leaf_color(&p, ri));
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1.5\"/>");
        label_inside(&mut b, &p, ri, b"#fff");
    }
    finalize(b, cfg)
}
