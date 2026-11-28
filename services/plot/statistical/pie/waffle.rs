use super::common::{open_svg, write_title};
use super::config::PieConfig;
use crate::plot::statistical::common::{
    apply_sort, escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate,
};

const ROWS: usize = 10;
const COLS: usize = 10;

#[crate::chart_demo(
    "labels=[\"Apple\",\"Banana\",\"Cherry\",\"Date\",\"Fig\"], values=[40,25,20,10,5], variant=\"waffle\""
)]

pub fn render(cfg: &PieConfig) -> String {
    let (labels, values) = apply_sort(cfg.labels, cfg.values, cfg.sort_order);
    let n = labels.len().min(values.len());
    if n == 0 {
        return String::new();
    }
    let total: f64 = values[..n].iter().sum();
    if total <= 0.0 {
        return String::new();
    }
    let n_cells = ROWS * COLS;

    let raw: Vec<f64> = values[..n].iter().map(|&v| v.max(0.0) / total * n_cells as f64).collect();
    let mut counts: Vec<usize> = raw.iter().map(|&r| r.floor() as usize).collect();
    let assigned: usize = counts.iter().sum();
    let mut remainders: Vec<(usize, f64)> = raw
        .iter()
        .enumerate()
        .map(|(i, &r)| (i, r - r.floor()))
        .collect();
    remainders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut leftover = n_cells.saturating_sub(assigned);
    let mut ri = 0;
    while leftover > 0 && ri < remainders.len() {
        counts[remainders[ri].0] += 1;
        leftover -= 1;
        ri += 1;
    }

    let mut cell_cat: Vec<usize> = Vec::with_capacity(n_cells);
    for (i, &c) in counts.iter().enumerate() {
        for _ in 0..c {
            cell_cat.push(i);
        }
    }
    while cell_cat.len() < n_cells {
        cell_cat.push(n - 1);
    }

    let w = cfg.width;
    let h = cfg.height;
    let title_pad = if cfg.title.is_empty() { 8.0 } else { 40.0 };
    let legend_w = 190.0;
    let grid_area_w = w as f64 - legend_w - 24.0;
    let grid_area_h = h as f64 - title_pad - 16.0;
    let cell_size = (grid_area_w / COLS as f64).min(grid_area_h / ROWS as f64);
    let gap = (cell_size * 0.12).max(1.0);
    let cell = cell_size - gap;
    let grid_x = 16.0;
    let grid_y = title_pad;

    let mut buf = Vec::<u8>::with_capacity(n_cells * 180 + n * 120 + 1024);
    open_svg(&mut buf, w, h);
    write_title(&mut buf, w, cfg.title);

    for r in 0..ROWS {
        for c in 0..COLS {
            let idx = r * COLS + c;
            let cat = cell_cat[idx];
            let color = palette_color(cfg.palette, cat);
            let hx = hex6(color);
            let x = grid_x + c as f64 * cell_size;
            let y = grid_y + r as f64 * cell_size;
            push_b(&mut buf, b"<rect data-idx=\"");
            push_i(&mut buf, cat as i32);
            push_b(&mut buf, b"\" data-lbl=\"");
            escape_xml(&mut buf, &labels[cat]);
            push_b(&mut buf, b"\" data-v=\"");
            push_f2(&mut buf, values[cat]);
            push_b(&mut buf, b"\" data-kv-pct=\"");
            push_f2(&mut buf, values[cat] / total * 100.0);
            push_b(&mut buf, b"%\" x=\"");
            push_f2(&mut buf, x);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, y);
            push_b(&mut buf, b"\" width=\"");
            push_f2(&mut buf, cell);
            push_b(&mut buf, b"\" height=\"");
            push_f2(&mut buf, cell);
            push_b(&mut buf, b"\" rx=\"2\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\"/>");
        }
    }

    let leg_x = grid_x + COLS as f64 * cell_size + 24.0;
    let mut leg_y = grid_y + 4.0;
    for i in 0..n {
        let color = palette_color(cfg.palette, i);
        let hx = hex6(color);
        push_b(&mut buf, b"<rect x=\"");
        push_f2(&mut buf, leg_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y);
        push_b(&mut buf, b"\" width=\"12\" height=\"12\" rx=\"2\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, leg_x + 18.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y + 10.0);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#1f2937\">");
        escape_xml(&mut buf, truncate(&labels[i], 16));
        push_b(&mut buf, b" (");
        push_f2(&mut buf, values[i] / total * 100.0);
        push_b(&mut buf, b"%)</text>");
        leg_y += 22.0;
    }

    push_b(&mut buf, b"</svg>");
    unsafe { String::from_utf8_unchecked(buf) }
}
