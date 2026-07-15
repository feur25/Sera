use super::config::GanttConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{
    escape_xml, palette_color, push_b, push_f2, push_i, sort_indices, sorted, svg_legend_item, truncate, Frame,
};
use std::collections::HashMap;

pub struct Prepared {
    pub n: usize,
    pub labels: Vec<String>,
    pub start: Vec<f64>,
    pub end: Vec<f64>,
    pub categories: Vec<String>,
    pub progress: Vec<f64>,
    pub vmin: f64,
    pub vrange: f64,
    pub cat_order: Vec<String>,
    pub cat_color: HashMap<String, u32>,
}

pub fn prepare(cfg: &GanttConfig) -> Option<Prepared> {
    let n = cfg
        .labels
        .len()
        .min(cfg.values_start.len())
        .min(cfg.values_end.len());
    if n == 0 {
        return None;
    }
    let idx = sort_indices(n, cfg.values_start, cfg.labels, cfg.sort_order);
    let labels = sorted(&idx, cfg.labels);
    let start = sorted(&idx, cfg.values_start);
    let end = sorted(&idx, cfg.values_end);
    let categories: Vec<String> = if cfg.categories.len() >= n {
        idx.iter().map(|&i| cfg.categories[i].clone()).collect()
    } else {
        vec![String::new(); n]
    };
    let progress: Vec<f64> = if cfg.progress.len() >= n {
        idx.iter().map(|&i| cfg.progress[i].clamp(0.0, 1.0)).collect()
    } else {
        vec![0.0; n]
    };

    let mut vmin = f64::INFINITY;
    let mut vmax = f64::NEG_INFINITY;
    for i in 0..n {
        let lo = start[i].min(end[i]);
        let hi = start[i].max(end[i]);
        if lo < vmin {
            vmin = lo;
        }
        if hi > vmax {
            vmax = hi;
        }
    }
    let vrange = (vmax - vmin).max(1.0);

    let mut cat_order: Vec<String> = Vec::new();
    let mut cat_color: HashMap<String, u32> = HashMap::new();
    for c in &categories {
        if !c.is_empty() && !cat_color.contains_key(c) {
            cat_color.insert(c.clone(), palette_color(cfg.palette, cat_order.len()));
            cat_order.push(c.clone());
        }
    }

    Some(Prepared {
        n,
        labels,
        start,
        end,
        categories,
        progress,
        vmin,
        vrange,
        cat_order,
        cat_color,
    })
}

pub fn row_color(p: &Prepared, cfg: &GanttConfig, i: usize) -> u32 {
    match p.categories.get(i).and_then(|c| p.cat_color.get(c)) {
        Some(&c) => c,
        None => palette_color(cfg.palette, i),
    }
}

pub fn open_frame(cfg: &GanttConfig, p: &Prepared) -> Frame {
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        132,
        38,
        52,
        20,
        p.n * 220 + 2048,
    );
    f.open(cfg.title, true);
    f.x_grid(6, p.vmin, p.vmin + p.vrange, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    f
}

pub fn x_at(f: &Frame, p: &Prepared, v: f64) -> i32 {
    f.pl + (((v - p.vmin) / p.vrange) * f.pw as f64) as i32
}

pub fn label_left(f: &mut Frame, p: &Prepared, i: usize, cy: i32) {
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, f.pl - 6);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, cy + 3);
    push_b(
        &mut f.buf,
        b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">",
    );
    escape_xml(&mut f.buf, truncate(&p.labels[i], 18));
    push_b(&mut f.buf, b"</text>");
}

pub fn bar_data_attrs(f: &mut Frame, p: &Prepared, i: usize) {
    push_b(&mut f.buf, b" data-idx=\"");
    push_i(&mut f.buf, i as i32);
    push_b(&mut f.buf, b"\" data-y=\"");
    push_f2(&mut f.buf, p.end[i] - p.start[i]);
    push_b(&mut f.buf, b"\" data-lbl=\"");
    escape_xml(&mut f.buf, &p.labels[i]);
    push_b(&mut f.buf, b"\"");
}

pub fn legend(f: &mut Frame, p: &Prepared) {
    for (i, name) in p.cat_order.iter().enumerate() {
        let color = p.cat_color[name];
        svg_legend_item(&mut f.buf, i as i32, name, color, f.w - 132, f.pt + i as i32 * 20, 18);
    }
}

pub fn finalize(f: Frame, cfg: &GanttConfig) -> String {
    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    f.html(json)
}
