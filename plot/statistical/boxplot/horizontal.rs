use super::common::{compute_box, global_range, group_values, sorted_groups};
use super::config::BoxplotConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i,
    svg_axis_lines, svg_legend_item, svg_x_label, svg_y_label, Frame,
};

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
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 110, 44, 54, legend_w, n_cats * 600 + 4096);
    f.open(cfg.title, false);
    f.x_grid(5, gr.y_min, gr.y_max, cfg.gridlines);
    svg_axis_lines(&mut f.buf, f.pl, f.pt, f.pw, f.ph);

    let slot_h = f.ph as f64 / n_cats as f64;
    let box_hh = (slot_h * 0.28) as i32;

    for (ci, (cat, st)) in cats.iter().zip(stats.iter()).enumerate() {
        let cy = f.pt + (ci as f64 * slot_h + slot_h / 2.0) as i32;
        let color = palette_color(cfg.palette, ci);
        let hx = hex6(color);
        let xv = |v: f64| f.pl + ((v - gr.y_min) / gr.range_y * f.pw as f64) as i32;
        let x_q1 = xv(st.q1);
        let x_med = xv(st.median);
        let x_q3 = xv(st.q3);
        let x_wlo = xv(st.whisker_lo);
        let x_whi = xv(st.whisker_hi);
        let box_left = x_q1.min(x_q3);
        let box_right = x_q1.max(x_q3);

        push_b(&mut f.buf, b"<g data-series=\""); push_i(&mut f.buf, ci as i32); push_b(&mut f.buf, b"\">");

        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, x_wlo);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, box_left);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.4\"/>");
        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, box_right);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, x_whi);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.4\"/>");
        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, x_wlo);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, cy - box_hh / 2);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, x_wlo);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, cy + box_hh / 2);
        push_b(&mut f.buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.4\"/>");
        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, x_whi);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, cy - box_hh / 2);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, x_whi);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, cy + box_hh / 2);
        push_b(&mut f.buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.4\"/>");

        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, cat);
        push_b(&mut f.buf, b"\" data-kv-Median=\""); push_f2(&mut f.buf, st.median);
        push_b(&mut f.buf, b"\" data-kv-Q1=\""); push_f2(&mut f.buf, st.q1);
        push_b(&mut f.buf, b"\" data-kv-Q3=\""); push_f2(&mut f.buf, st.q3);
        push_b(&mut f.buf, b"\" data-kv-N=\""); push_i(&mut f.buf, st.n as i32);
        push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, box_left);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, cy - box_hh);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, (box_right - box_left).max(2));
        push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, box_hh * 2);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\""); push_f2(&mut f.buf, cfg.fill_opacity);
        push_b(&mut f.buf, b"\" rx=\"3\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, cfg.stroke_width);
        push_b(&mut f.buf, b"\"/>");

        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, x_med);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, cy - box_hh);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, x_med);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, cy + box_hh);
        push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"2.4\"/>");

        for &ov in &st.outliers {
            let ox = f.pl + ((ov - gr.y_min) / gr.range_y * f.pw as f64) as i32;
            push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, ox);
            push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
            push_b(&mut f.buf, b"\" r=\"3\" fill=\"#"); f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" fill-opacity=\"0.75\" stroke=\"#fff\" stroke-width=\"0.8\"/>");
        }

        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, f.pl - 8);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, cy + 4);
        push_b(&mut f.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
        let trimmed = if cat.len() <= 18 { cat.as_str() } else { &cat[..18] };
        escape_xml(&mut f.buf, trimmed);
        push_b(&mut f.buf, b"</text>");

        push_b(&mut f.buf, b"</g>");
    }

    if n_cats > 1 {
        let lx = f.w - legend_w + 12;
        for (ci, cat) in cats.iter().enumerate() {
            svg_legend_item(&mut f.buf, ci as i32, cat, palette_color(cfg.palette, ci), lx, f.pt + ci as i32 * 22, 14);
        }
    }
    svg_x_label(&mut f.buf, cfg.x_label, f.pl + f.pw / 2, f.h - 4);
    svg_y_label(&mut f.buf, cfg.y_label, 14, f.pt, f.ph);
    f.html(&slots_to_json(cfg.hover))
}


