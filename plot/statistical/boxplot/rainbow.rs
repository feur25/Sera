use super::common::{
    compute_box, draw_box_vertical, draw_cat_label, draw_outliers_vertical, finish_frame,
    global_range, group_values, make_frame, open_axes, sorted_groups,
};
use super::config::BoxplotConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{push_b, push_i};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]")]

pub fn render(cfg: &BoxplotConfig) -> String {
    let n = cfg.category_labels.len().min(cfg.values.len());
    if n == 0 {
        return String::new();
    }
    let (cats, groups) = group_values(cfg.category_labels, cfg.values);
    let stats: Vec<_> = groups.iter().map(|g| compute_box(g)).collect();
    let (cats, _groups, stats) = sorted_groups(cats, groups, stats, cfg.sort_order);
    let n_cats = cats.len();
    let gr = global_range(&stats);

    let legend_w: i32 = if n_cats > 1 { 130 } else { 20 };
    let mut f = make_frame(cfg, n_cats, legend_w);
    let plot_w = f.pw;
    let slot_w = plot_w as f64 / n_cats as f64;
    let box_hw = (slot_w * 0.28) as i32;

    open_axes(&mut f, cfg.title, cfg.gridlines, gr.y_min, gr.y_max);

    let stops: [u32; 7] = [
        0xEF4444, 0xF97316, 0xFACC15, 0x22C55E, 0x06B6D4, 0x6366F1, 0xA855F7,
    ];
    for (ci, (cat, st)) in cats.iter().zip(stats.iter()).enumerate() {
        let cx = f.pl + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        let t = if n_cats > 1 {
            ci as f64 / (n_cats - 1) as f64
        } else {
            0.0
        };
        let color = sample_palette(&stops, t);
        push_b(&mut f.buf, b"<g data-series=\"");
        push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\">");
        draw_box_vertical(
            &mut f,
            cx,
            box_hw,
            st,
            color,
            cfg.fill_opacity + 0.18,
            cfg.stroke_width + 0.4,
            cfg.notch,
            cat,
            ci as i32,
            gr.y_min,
            gr.range_y,
        );
        draw_outliers_vertical(&mut f, cx, st, color, 3.5, gr.y_min, gr.range_y);
        draw_cat_label(&mut f, cx, cat);
        push_b(&mut f.buf, b"</g>");
    }

    finish_frame(
        &mut f,
        &cats,
        cfg.palette,
        cfg.x_label,
        cfg.y_label,
        legend_w,
    );
    f.html(&slots_to_json(cfg.hover))
}

fn sample_palette(stops: &[u32], t: f64) -> u32 {
    let t = t.clamp(0.0, 1.0);
    if stops.is_empty() {
        return 0x6366F1;
    }
    if stops.len() == 1 {
        return stops[0];
    }
    let scaled = t * (stops.len() - 1) as f64;
    let lo = scaled.floor() as usize;
    let hi = (lo + 1).min(stops.len() - 1);
    let frac = scaled - lo as f64;
    lerp(stops[lo], stops[hi], frac)
}

fn lerp(a: u32, b: u32, t: f64) -> u32 {
    let ar = ((a >> 16) & 0xFF) as f64;
    let ag = ((a >> 8) & 0xFF) as f64;
    let ab = (a & 0xFF) as f64;
    let br = ((b >> 16) & 0xFF) as f64;
    let bg = ((b >> 8) & 0xFF) as f64;
    let bb = (b & 0xFF) as f64;
    let r = (ar + (br - ar) * t).round() as u32;
    let g = (ag + (bg - ag) * t).round() as u32;
    let bl = (ab + (bb - ab) * t).round() as u32;
    (r << 16) | (g << 8) | bl
}
