use super::common::{fill_hex, finalize, label_inside, leaf_color, open_svg, prepare, rect_attrs, tile_data_attrs};
use super::config::TreemapConfig;
use crate::plot::statistical::common::{push_b, push_f2};

#[crate::chart_demo(
    "labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\",\"G\"], values=[40,25,20,10,5,8,12], comparisons=[34,27,15,12,4,10,9], variant=\"trend\""
)]

pub fn render(cfg: &TreemapConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.leaf_indices.len() * 240 + 2048);
    open_svg(&mut b, cfg);
    for ri in 0..p.leaf_indices.len() {
        let r = p.rects[ri];
        if r.w < 0.5 || r.h < 0.5 {
            continue;
        }
        push_b(&mut b, b"<rect");
        tile_data_attrs(&mut b, &p, ri);
        rect_attrs(&mut b, r);
        push_b(&mut b, b" rx=\"4");
        fill_hex(&mut b, leaf_color(&p, ri));
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"2\"/>");
        label_inside(&mut b, &p, ri, b"#fff", cfg.show_text);

        let oi = p.leaf_indices[ri];
        let prior = cfg.prior_values.get(oi).copied();
        if let Some(prior) = prior {
            if prior.is_finite() && prior != 0.0 && r.w > 46.0 && r.h > 30.0 {
                let cur = p.values[oi];
                let pct = (cur - prior) / prior.abs() * 100.0;
                let up = pct >= 0.0;
                let badge_col: &[u8] = if up { b"#10B981" } else { b"#EF4444" };
                let txt = format!("{}{:.1}%", arrow_prefix(up), pct.abs());
                let bw = (txt.chars().count() as f64 * 5.6 + 10.0).min(r.w - 8.0);
                let bx = r.x + r.w - bw - 4.0;
                let by = r.y + 4.0;
                push_b(&mut b, b"<rect x=\"");
                push_f2(&mut b, bx);
                push_b(&mut b, b"\" y=\"");
                push_f2(&mut b, by);
                push_b(&mut b, b"\" width=\"");
                push_f2(&mut b, bw);
                push_b(&mut b, b"\" height=\"15\" rx=\"7\" fill=\"");
                b.extend_from_slice(badge_col);
                push_b(&mut b, b"\" opacity=\"0.92\"/>");
                push_b(&mut b, b"<text x=\"");
                push_f2(&mut b, bx + bw / 2.0);
                push_b(&mut b, b"\" y=\"");
                push_f2(&mut b, by + 11.0);
                push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" fill=\"#fff\">");
                b.extend_from_slice(txt.as_bytes());
                push_b(&mut b, b"</text>");
            }
        }
    }
    finalize(b, cfg)
}

fn arrow_prefix(up: bool) -> &'static str {
    if up {
        "\u{25B2} "
    } else {
        "\u{25BC} "
    }
}
