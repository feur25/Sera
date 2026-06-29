use super::common::{
    data_value_rect, finalize, label_left, max_for, open_svg, prepare, value_text,
};
use super::config::BulletConfig;
use crate::plot::statistical::common::{hex6, push_b, push_i};

#[crate::chart_demo("labels=[\"Revenue\",\"Profit\",\"CSAT\"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5], comparisons=[70,55,3.8]")]

pub fn render(cfg: &BulletConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.n * 340 + 1024);
    open_svg(&mut b, cfg, &p);
    let bar_h = (p.row_h as f64 * 0.32) as i32;
    let ghost_h = (p.row_h as f64 * 0.5) as i32;
    let hx_track = hex6(0xE5E7EB);
    let hx_ghost = hex6(0xCBD5E1);
    for i in 0..p.n {
        let max_v = max_for(&p, i);
        let ry = p.pad_t + i as i32 * p.row_h + (p.row_h - ghost_h) / 2;
        let by = p.pad_t + i as i32 * p.row_h + (p.row_h - bar_h) / 2;
        push_b(&mut b, b"<rect x=\"");
        push_i(&mut b, p.pad_l);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, ry);
        push_b(&mut b, b"\" width=\"");
        push_i(&mut b, p.plot_w);
        push_b(&mut b, b"\" height=\"");
        push_i(&mut b, ghost_h);
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&hx_track);
        push_b(&mut b, b"\" rx=\"3\"/>");
        let prior = if p.comparisons[i] > 0.0 {
            p.comparisons[i]
        } else if p.ranges[i] > 0.0 {
            p.ranges[i]
        } else {
            0.0
        };
        if prior > 0.0 {
            let pw = ((prior / max_v).min(1.0) * p.plot_w as f64) as i32;
            push_b(&mut b, b"<rect x=\"");
            push_i(&mut b, p.pad_l);
            push_b(&mut b, b"\" y=\"");
            push_i(&mut b, ry);
            push_b(&mut b, b"\" width=\"");
            push_i(&mut b, pw);
            push_b(&mut b, b"\" height=\"");
            push_i(&mut b, ghost_h);
            push_b(&mut b, b"\" fill=\"#");
            b.extend_from_slice(&hx_ghost);
            push_b(&mut b, b"\" rx=\"3\"/>");
        }
        let value_w = ((p.values[i] / max_v).min(1.0) * p.plot_w as f64) as i32;
        data_value_rect(
            &mut b, &p, i, p.pad_l, by, value_w, bar_h, 0x636EFA, 2, 0.95,
        );
        if p.targets[i] > 0.0 {
            let tx = p.pad_l + ((p.targets[i] / max_v).min(1.0) * p.plot_w as f64) as i32;
            push_b(&mut b, b"<line x1=\"");
            push_i(&mut b, tx);
            push_b(&mut b, b"\" y1=\"");
            push_i(&mut b, ry - 2);
            push_b(&mut b, b"\" x2=\"");
            push_i(&mut b, tx);
            push_b(&mut b, b"\" y2=\"");
            push_i(&mut b, ry + ghost_h + 2);
            push_b(&mut b, b"\" stroke=\"#dc2626\" stroke-width=\"2.4\"/>");
        }
        label_left(&mut b, &p, i, by, bar_h);
        if cfg.show_text {
            value_text(
                &mut b,
                p.values[i],
                p.pad_l + value_w + 5,
                by + bar_h / 2 + 4,
            );
        }
    }
    finalize(b, cfg)
}
