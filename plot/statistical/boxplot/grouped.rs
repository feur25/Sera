use super::common::{
    compute_box, draw_box_vertical, draw_cat_label, draw_outliers_vertical, finish_frame,
    global_range, make_frame, open_axes, BoxStats,
};
use super::config::BoxplotConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{palette_color, push_b, push_i, svg_legend_item};

pub fn render(cfg: &BoxplotConfig) -> String {
    let cats: Vec<String> = if cfg.category_labels.is_empty() {
        (0..cfg.series.len()).map(|i| format!("G{}", i + 1)).collect()
    } else {
        cfg.category_labels.to_vec()
    };
    let n_cats = cats.len();
    if n_cats == 0 || cfg.series.is_empty() {
        return super::basic::render(cfg);
    }
    let n_series = cfg.series.len();

    let mut all_stats: Vec<Vec<BoxStats>> = Vec::with_capacity(n_series);
    for s in cfg.series {
        let parts: Vec<&[f64]> = split_series(s, n_cats);
        let mut row = Vec::with_capacity(n_cats);
        for p in parts {
            row.push(compute_box(p));
        }
        all_stats.push(row);
    }

    let flat: Vec<BoxStats> = all_stats.iter().flatten().cloned().collect();
    let gr = global_range(&flat);

    let legend_w: i32 = 140;
    let mut f = make_frame(cfg, n_cats, legend_w);
    open_axes(&mut f, cfg.title, cfg.gridlines, gr.y_min, gr.y_max);

    let plot_w = f.pw;
    let slot_w = plot_w as f64 / n_cats as f64;
    let group_w = slot_w * 0.78;
    let sub_w = group_w / n_series as f64;
    let box_hw = (sub_w * 0.36) as i32;

    for ci in 0..n_cats {
        let center = f.pl + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        for si in 0..n_series {
            let st = &all_stats[si][ci];
            let cx = center - (group_w as i32) / 2 + ((si as f64 + 0.5) * sub_w) as i32;
            let color = palette_color(cfg.palette, si);
            let label = if si < cfg.series_names.len() { &cfg.series_names[si] } else { "" };
            push_b(&mut f.buf, b"<g data-series=\"");
            push_i(&mut f.buf, si as i32);
            push_b(&mut f.buf, b"\">");
            draw_box_vertical(&mut f, cx, box_hw, st, color, cfg.fill_opacity, cfg.stroke_width, cfg.notch, label, si as i32, gr.y_min, gr.range_y);
            draw_outliers_vertical(&mut f, cx, st, color, 2.5, gr.y_min, gr.range_y);
            push_b(&mut f.buf, b"</g>");
        }
        draw_cat_label(&mut f, center, &cats[ci]);
    }

    let lx = f.w - legend_w + 12;
    for si in 0..n_series {
        let name = if si < cfg.series_names.len() { cfg.series_names[si].as_str() } else { "" };
        svg_legend_item(&mut f.buf, si as i32, name, palette_color(cfg.palette, si), lx, f.pt + si as i32 * 22, 16);
    }

    finish_frame(&mut f, &[], cfg.palette, cfg.x_label, cfg.y_label, 0);
    f.html(&slots_to_json(cfg.hover))
}

fn split_series<'a>(s: &'a [f64], n_cats: usize) -> Vec<&'a [f64]> {
    if n_cats == 0 || s.is_empty() {
        return Vec::new();
    }
    let chunk = (s.len() + n_cats - 1) / n_cats;
    (0..n_cats)
        .map(|i| {
            let start = i * chunk;
            let end = ((i + 1) * chunk).min(s.len());
            if start >= s.len() { &[][..] } else { &s[start..end] }
        })
        .collect()
}


