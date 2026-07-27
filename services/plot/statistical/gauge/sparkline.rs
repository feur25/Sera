use super::common::{color_for, finalize, open_svg, prepare};
use super::config::GaugeConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2};

#[crate::chart_demo(
    "value=72, min_val=0, max_val=100, label=\"Score\", history=[55,58,60,57,63,66,64,68,70,72], variant=\"sparkline\""
)]

pub fn render(cfg: &GaugeConfig) -> String {
    let p = prepare(cfg);
    let mut b = Vec::<u8>::with_capacity(4096);
    open_svg(&mut b, cfg);
    super::basic::draw(&mut b, cfg, &p);

    let n = cfg.history.len();
    if n > 1 {
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for &v in cfg.history {
            if v.is_finite() {
                lo = lo.min(v);
                hi = hi.max(v);
            }
        }
        if hi > lo {
            let spark_w = (cfg.width as f64 * 0.55).min(180.0);
            let spark_h = 26.0;
            let sx0 = p.cx - spark_w / 2.0;
            let sy0 = (cfg.height as f64 - spark_h - 10.0).max(p.cy + 62.0);
            let col = color_for(&p, p.frac);
            let hx = hex6(col);
            let pts: Vec<(f64, f64)> = cfg
                .history
                .iter()
                .enumerate()
                .map(|(i, &v)| {
                    let x = sx0 + (i as f64 / (n - 1) as f64) * spark_w;
                    let y = sy0 + spark_h - ((v - lo) / (hi - lo)) * spark_h;
                    (x, y)
                })
                .collect();
            push_b(&mut b, b"<polyline points=\"");
            for (i, &(x, y)) in pts.iter().enumerate() {
                if i > 0 {
                    b.push(b' ');
                }
                push_f2(&mut b, x);
                b.push(b',');
                push_f2(&mut b, y);
            }
            push_b(&mut b, b"\" fill=\"none\" stroke=\"#");
            b.extend_from_slice(&hx);
            push_b(
                &mut b,
                b"\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\" opacity=\"0.85\"/>",
            );
            let (lx, ly) = pts[pts.len() - 1];
            push_b(&mut b, b"<circle cx=\"");
            push_f2(&mut b, lx);
            push_b(&mut b, b"\" cy=\"");
            push_f2(&mut b, ly);
            push_b(&mut b, b"\" r=\"2.6\" fill=\"#");
            b.extend_from_slice(&hx);
            push_b(&mut b, b"\"/>");
        }
    }

    finalize(b, cfg)
}
