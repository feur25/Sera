use super::config::EventplotConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, push_b, push_f2, push_i, truncate, Frame};
use std::collections::HashMap;

pub struct Prepared {
    pub row_names: Vec<String>,
    pub row_events: Vec<Vec<f64>>,
    pub xmin: f64,
    pub xmax: f64,
}

pub fn prepare(cfg: &EventplotConfig) -> Option<Prepared> {
    let n = cfg.x_values.len().min(cfg.categories.len());
    if n == 0 {
        return None;
    }
    let mut row_index: HashMap<&str, usize> = HashMap::new();
    let mut row_names: Vec<String> = Vec::new();
    let mut row_events: Vec<Vec<f64>> = Vec::new();
    for i in 0..n {
        let cat = cfg.categories[i].as_str();
        let ri = *row_index.entry(cat).or_insert_with(|| {
            row_names.push(cat.to_string());
            row_events.push(Vec::new());
            row_names.len() - 1
        });
        row_events[ri].push(cfg.x_values[i]);
    }

    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    for &v in cfg.x_values {
        if v < xmin {
            xmin = v;
        }
        if v > xmax {
            xmax = v;
        }
    }
    let pad = (xmax - xmin).max(1e-9) * 0.05;

    Some(Prepared {
        row_names,
        row_events,
        xmin: xmin - pad,
        xmax: xmax + pad,
    })
}

pub fn open_frame(cfg: &EventplotConfig, p: &Prepared) -> Frame {
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        100,
        38,
        52,
        20,
        cfg.x_values.len() * 40 + 2048,
    );
    f.open(cfg.title, true);
    f.x_grid(6, p.xmin, p.xmax, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    f
}

pub fn x_at(f: &Frame, p: &Prepared, v: f64) -> f64 {
    f.pl as f64 + (v - p.xmin) / (p.xmax - p.xmin).max(1e-9) * f.pw as f64
}

pub fn label_left(f: &mut Frame, name: &str, cy: f64) {
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, f.pl - 6);
    push_b(&mut f.buf, b"\" y=\"");
    push_f2(&mut f.buf, cy + 3.0);
    push_b(
        &mut f.buf,
        b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">",
    );
    escape_xml(&mut f.buf, truncate(name, 18));
    push_b(&mut f.buf, b"</text>");
}

pub fn finalize(f: Frame, cfg: &EventplotConfig) -> String {
    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    f.html(json)
}
