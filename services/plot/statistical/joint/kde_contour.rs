use super::common::{draw_main_axes, finalize, kde_heat_contour, layout, open, Bounds};
use super::config::JointConfig;
use crate::plot::statistical::common::palette_color;

#[crate::chart_demo(
    "group_series=[(\"a\",[1.2,2.4,2.1,3.6,3.1,3.9,2.2,2.6],[1.1,2.3,3.2,2.4,3.6,4.1,2.1,3.1]),\
(\"b\",[4.2,4.6,4.4,4.9,5.5,5.1,5.8,5.2],[3.3,4.7,5.2,3.9,4.4,5.6,6.1,4.5])]"
)]
pub fn render(cfg: &JointConfig) -> String {
    if cfg.group_series.is_empty() {
        return String::new();
    }
    let (mut xmin, mut xmax) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut ymin, mut ymax) = (f64::INFINITY, f64::NEG_INFINITY);
    for (_, xs, ys) in cfg.group_series {
        for &v in xs {
            xmin = xmin.min(v);
            xmax = xmax.max(v);
        }
        for &v in ys {
            ymin = ymin.min(v);
            ymax = ymax.max(v);
        }
    }
    if !xmin.is_finite() || !ymin.is_finite() {
        return String::new();
    }
    let xpad = (xmax - xmin).max(1e-9) * 0.08;
    let ypad = (ymax - ymin).max(1e-9) * 0.08;
    let bounds = Bounds {
        xmin: xmin - xpad,
        xmax: xmax + xpad,
        ymin: ymin - ypad,
        ymax: ymax + ypad,
    };

    let mut f = open(cfg);
    let l = layout(cfg);
    draw_main_axes(&mut f, cfg, &l);

    for (i, (_, xs, ys)) in cfg.group_series.iter().enumerate() {
        let color = palette_color(cfg.palette, i);
        kde_heat_contour(&mut f, &l, &bounds, xs, ys, color, 40, 0.16);
    }

    let names: Vec<&str> = cfg.group_series.iter().map(|(n, _, _)| n.as_str()).collect();
    f.legend_pos(&names, cfg.palette, "top-right");

    finalize(f, cfg)
}
