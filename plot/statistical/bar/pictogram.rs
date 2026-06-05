use super::config::BarConfig;
use crate::plot::statistical::common::{
    escape_xml, palette_color, push_b, push_f2, push_hex, push_i, Frame,
};

#[crate::chart_demo("labels=[\"Bikes\",\"Cars\",\"Buses\",\"Trains\"], values=[24,38,17,42], unit_description=\"units\", units_per_icon=2.0, icon_size=24, max_icons_per_column=10")]

pub fn render(cfg: &BarConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return String::new();
    }
    let upi = cfg.units_per_icon.max(1.0);
    let max_per_col = cfg.max_icons_per_column.max(1);
    let icon_size = cfg.icon_size.max(8);
    let icon_pad = (icon_size as f64 * 0.18) as i32;
    let col_w = icon_size + icon_pad;
    let col_h = (max_per_col * (icon_size + icon_pad) + icon_pad).max(80);

    let cols: Vec<i32> = (0..n)
        .map(|i| {
            let count = (cfg.values[i] / upi).round() as i32;
            ((count + max_per_col - 1) / max_per_col).max(1)
        })
        .collect();
    let counts: Vec<i32> = (0..n)
        .map(|i| (cfg.values[i] / upi).round() as i32)
        .collect();

    let cat_pad = col_w;
    let total_cols: i32 = cols.iter().sum::<i32>() + (n as i32 - 1).max(0);
    let plot_w = total_cols * col_w + cat_pad;
    let pl = 60;
    let pt = 60;
    let pb = 60;
    let pr = 60;
    let w = (plot_w + pl + pr).max(cfg.width);
    let h = (col_h + pt + pb).max(cfg.height);

    let mut f = Frame::new_html(cfg.title, w, h, pl, pt, pb, pr, n as usize * 800 + 4096);
    f.open(cfg.title, true);

    if !cfg.unit_description.is_empty() {
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, pl - 10);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, pt - 12);
        push_b(
            &mut f.buf,
            b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">Each icon = ",
        );
        push_f2(&mut f.buf, upi);
        push_b(&mut f.buf, b" ");
        escape_xml(&mut f.buf, cfg.unit_description);
        push_b(&mut f.buf, b"</text>");
    }

    let mut x_acc = pl + cat_pad / 2;
    let baseline = pt + col_h - icon_pad;
    for i in 0..n {
        let count = counts[i];
        let n_cols = cols[i];
        let color = if cfg.color_hex != 0 {
            cfg.color_hex
        } else {
            palette_color(cfg.palette, i)
        };

        let mut placed = 0;
        for ci in 0..n_cols {
            let in_col = (count - placed).min(max_per_col);
            let cx = x_acc + ci * col_w + icon_size / 2;
            for k in 0..in_col {
                let cy = baseline - k * (icon_size + icon_pad) - icon_size / 2;
                push_b(&mut f.buf, b"<rect x=\"");
                push_i(&mut f.buf, cx - icon_size / 2);
                push_b(&mut f.buf, b"\" y=\"");
                push_i(&mut f.buf, cy - icon_size / 2);
                push_b(&mut f.buf, b"\" width=\"");
                push_i(&mut f.buf, icon_size);
                push_b(&mut f.buf, b"\" height=\"");
                push_i(&mut f.buf, icon_size);
                push_b(&mut f.buf, b"\" rx=\"3\" fill=\"");
                push_hex(&mut f.buf, color);
                push_b(&mut f.buf, b"\"/>");
            }
            placed += in_col;
        }

        let center_x = x_acc + (n_cols * col_w) / 2;
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, center_x);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, pt - 4);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#1f2937\">");
        push_f2(&mut f.buf, cfg.values[i]);
        push_b(&mut f.buf, b"</text>");

        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, center_x);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, pt + col_h + 18);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        escape_xml(&mut f.buf, &cfg.labels[i]);
        push_b(&mut f.buf, b"</text>");

        x_acc += n_cols * col_w + cat_pad;
    }

    f.html("[]")
}
