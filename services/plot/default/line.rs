use crate::plot::{apply, parse_all};
pub struct Line;

pub fn render_lines_fast(values: &[f64], labels: &[String], width: i32, height: i32) -> String {
    let n = values.len().min(labels.len());
    if n < 2 {
        return String::new();
    }

    let (_, max_val) = crate::bindings::utils::simd_ops::find_minmax(values);
    let max_val = max_val.max(1.0);
    let scale_x = width as f64 / n as f64;
    let scale_y = height as f64 / max_val;

    let mut svg = String::with_capacity(n * 180 + 512);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&width.to_string());
    svg.push(' ');
    svg.push_str(&height.to_string());
    svg.push_str(
        "\"><defs><style>.l{stroke-width:2;fill:none}.p{fill:#fff;stroke-width:1}</style></defs>",
    );

    let color = 0x636EFA;
    svg.push_str("<polyline class=\"l\" stroke=\"#");
    svg.push_str(&format!("{:06x}", color));
    svg.push_str("\" points=\"");

    for i in 0..n {
        let x = (i as f64 * scale_x) as i32;
        let y = height - (values[i] * scale_y) as i32;

        if i > 0 {
            svg.push(' ');
        }
        svg.push_str(&x.to_string());
        svg.push(',');
        svg.push_str(&y.to_string());
    }

    svg.push_str("\"/>");

    let colors = [0x636EFA, 0xEF553B, 0x00CC96, 0xAB63FA];
    for i in 0..n {
        let x = (i as f64 * scale_x) as i32;
        let y = height - (values[i] * scale_y) as i32;
        let hex = format!("{:06x}", colors[i % colors.len()]);

        svg.push_str("<circle cx=\"");
        svg.push_str(&x.to_string());
        svg.push_str("\" cy=\"");
        svg.push_str(&y.to_string());
        svg.push_str("\" r=\"3\" fill=\"#");
        svg.push_str(&hex);
        svg.push_str("\" data-index=\"");
        svg.push_str(&i.to_string());
        svg.push_str("\"/>");
    }

    svg.push_str("</svg>");
    svg
}

pub fn render_lines(ctx: super::PlotRenderContext) {
    if ctx.visible_indices.len() < 2 {
        return;
    }

    let visible_count = ctx.visible_indices.len();
    let mut prev_pos: Option<egui::Pos2> = None;

    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = value / ctx.max_val.max(1.0);

        let pos = if ctx.vertical {
            let x = ctx.plot_rect.left()
                + (ctx.plot_rect.width() / (visible_count as f32 - 1.0).max(1.0)) * vis_idx as f32;
            let y = ctx.plot_rect.bottom() - norm_val as f32 * ctx.plot_rect.height();
            egui::pos2(x, y)
        } else {
            let x = ctx.plot_rect.left() + norm_val as f32 * ctx.plot_rect.width();
            let y = ctx.plot_rect.top()
                + (ctx.plot_rect.height() / (visible_count as f32 - 1.0).max(1.0)) * vis_idx as f32;
            egui::pos2(x, y)
        };

        let color = ctx.colors[actual_idx % ctx.colors.len()];
        if let Some(p) = prev_pos {
            ctx.painter
                .line_segment([p, pos], egui::Stroke::new(2.0, color));
        }

        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let (radius, display_color) = if is_hovered {
            (6.0, egui::Color32::from_rgb(255, 200, 0))
        } else {
            (4.0, color)
        };
        ctx.painter.circle_filled(pos, radius, display_color);
        prev_pos = Some(pos);
    }
}

pub fn render_svg_lines(
    svg: &mut String,
    values: &[f64],
    colors: &[&'static str],
    pad: i32,
    plot_width: i32,
    plot_height: i32,
    max_val: f64,
    vertical: bool,
) {
    if values.len() < 2 {
        return;
    }

    let visible_count = values.len();
    let mut prev_pos: Option<(i32, i32)> = None;

    for (vis_idx, &val) in values.iter().enumerate() {
        let norm_val = val / max_val.max(1.0);
        let (x, y) = if vertical {
            let x_pos =
                pad as f64 + (plot_width as f64 / (visible_count - 1) as f64) * vis_idx as f64;
            let y_pos = pad + plot_height - (norm_val * plot_height as f64) as i32;
            (x_pos as i32, y_pos)
        } else {
            let x_pos = pad as i32 + (norm_val * plot_width as f64) as i32;
            let y_pos =
                pad as f64 + (plot_height as f64 / (visible_count - 1) as f64) * vis_idx as f64;
            (x_pos, y_pos as i32)
        };
        let color = colors[vis_idx % colors.len()];

        if let Some((px, py)) = prev_pos {
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\" class=\"interactive-line\"/>",
                px, py, x, y, color
            ));
        }

        svg.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\" class=\"interactive-point\" data-index=\"{}\"/>",
            x, y, color, vis_idx
        ));

        prev_pos = Some((x, y));
    }
}

pub fn render_lines_html(
    title: &str,
    labels: &[String],
    values: &[f64],
    width: i32,
    height: i32,
    hover: &[crate::html::hover::HoverSlot],
    color_hex: u32,
    x_label: &str,
    y_label: &str,
    gridlines: bool,
    show_points: bool,
    sort_order: &str,
) -> String {
    use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
    use crate::plot::statistical::common::{
        apply_sort, escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate,
    };
    let n = values.len().min(labels.len());
    if n < 2 {
        return String::new();
    }
    let (labels, values) = apply_sort(&labels[..n], &values[..n], sort_order);
    let labels = &labels[..];
    let values = &values[..];
    let (_, max_val) = crate::bindings::utils::simd_ops::find_minmax(values);
    let max_val = max_val.max(1.0);
    let pad_l = 52i32;
    let pad_t = 36i32;
    let pad_b = 48i32;
    let pad_r = 20i32;
    let plot_w = width - pad_l - pad_r;
    let plot_h = height - pad_t - pad_b;
    let step_x = plot_w as f64 / (n - 1).max(1) as f64;
    let line_color = if color_hex != 0 { color_hex } else { 0x636EFA };
    let auto = hover.is_empty();
    let hid = html_id();
    let mut b = Vec::<u8>::with_capacity(n * 80 + 24_000);
    html_prefix(&mut b, title, hid);
    push_b(
        &mut b,
        b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"",
    );
    push_i(&mut b, width);
    push_b(&mut b, b"\" height=\"");
    push_i(&mut b, height);
    push_b(&mut b, b"\" viewBox=\"0 0 ");
    push_i(&mut b, width);
    push_b(&mut b, b" ");
    push_i(&mut b, height);
    push_b(&mut b, b"\">");
    push_b(
        &mut b,
        b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>",
    );
    push_b(&mut b, b"<title>");
    escape_xml(&mut b, if title.is_empty() { "Chart" } else { title });
    push_b(&mut b, b"</title>");
    if !title.is_empty() {
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, width / 2);
        push_b(&mut b, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut b, title);
        push_b(&mut b, b"</text>");
    }
    for i in 0..=5 {
        let frac = i as f64 / 5.0;
        let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        let val = frac * max_val;
        if gridlines && i > 0 {
            push_b(&mut b, b"<line x1=\"");
            push_i(&mut b, pad_l);
            push_b(&mut b, b"\" y1=\"");
            push_i(&mut b, y);
            push_b(&mut b, b"\" x2=\"");
            push_i(&mut b, pad_l + plot_w);
            push_b(&mut b, b"\" y2=\"");
            push_i(&mut b, y);
            push_b(
                &mut b,
                b"\" stroke=\"#e2e8f0\" stroke-width=\"0.5\" class=\"sp-gl\"/>",
            );
        }
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, pad_l - 4);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, y + 3);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-yt\">");
        if val >= 1000.0 {
            push_i(&mut b, val as i32);
        } else {
            push_f2(&mut b, val);
        }
        push_b(&mut b, b"</text>");
    }
    push_b(&mut b, b"<line x1=\"");
    push_i(&mut b, pad_l);
    push_b(&mut b, b"\" y1=\"");
    push_i(&mut b, pad_t);
    push_b(&mut b, b"\" x2=\"");
    push_i(&mut b, pad_l);
    push_b(&mut b, b"\" y2=\"");
    push_i(&mut b, pad_t + plot_h);
    push_b(
        &mut b,
        b"\" stroke=\"#6b7280\" stroke-width=\"1\" class=\"sp-ax-y\"/>",
    );
    push_b(&mut b, b"<line x1=\"");
    push_i(&mut b, pad_l);
    push_b(&mut b, b"\" y1=\"");
    push_i(&mut b, pad_t + plot_h);
    push_b(&mut b, b"\" x2=\"");
    push_i(&mut b, pad_l + plot_w);
    push_b(&mut b, b"\" y2=\"");
    push_i(&mut b, pad_t + plot_h);
    push_b(
        &mut b,
        b"\" stroke=\"#6b7280\" stroke-width=\"1\" class=\"sp-ax-x\"/>",
    );
    if !y_label.is_empty() {
        let ym = pad_t + plot_h / 2;
        push_b(&mut b, b"<text x=\"14\" y=\"");
        push_i(&mut b, ym);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\" transform=\"rotate(-90,14,");
        push_i(&mut b, ym);
        push_b(&mut b, b")\" class=\"sp-yl\">");
        escape_xml(&mut b, y_label);
        push_b(&mut b, b"</text>");
    }
    if !x_label.is_empty() {
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, pad_l + plot_w / 2);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, height - 4);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\" class=\"sp-xl\">");
        escape_xml(&mut b, x_label);
        push_b(&mut b, b"</text>");
    }
    let hx = hex6(line_color);
    push_b(&mut b, b"<polyline data-idx=\"0\" data-pts=\"");
    for i in 0..n {
        if i > 0 {
            b.push(b',');
        }
        push_f2(&mut b, values[i]);
    }
    push_b(&mut b, b"\" fill=\"none\" stroke=\"#");
    b.extend_from_slice(&hx);
    push_b(
        &mut b,
        b"\" stroke-width=\"2\" stroke-linecap=\"round\" points=\"",
    );
    for i in 0..n {
        let x = pad_l + (i as f64 * step_x) as i32;
        let y = pad_t + plot_h - ((values[i] / max_val) * plot_h as f64) as i32;
        if i > 0 {
            b.push(b' ');
        }
        push_i(&mut b, x);
        b.push(b',');
        push_i(&mut b, y);
    }
    push_b(&mut b, b"\"/>");
    if show_points {
        let circ_step = ((n as f64 / 30.0).ceil() as usize).max(1);
        for i in (0..n).step_by(circ_step) {
            let x = pad_l + (i as f64 * step_x) as i32;
            let y = pad_t + plot_h - ((values[i] / max_val) * plot_h as f64) as i32;
            let color = palette_color(&[], i);
            let chx = hex6(color);
            push_b(&mut b, b"<circle data-idx=\"");
            push_i(&mut b, i as i32);
            push_b(&mut b, b"\" data-y=\"");
            push_f2(&mut b, values[i]);
            push_b(&mut b, b"\" data-lbl=\"");
            escape_xml(&mut b, &labels[i]);
            push_b(&mut b, b"\" cx=\"");
            push_i(&mut b, x);
            push_b(&mut b, b"\" cy=\"");
            push_i(&mut b, y);
            push_b(&mut b, b"\" r=\"4\" fill=\"#");
            b.extend_from_slice(&chx);
            push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1.5\"/>");
        }
    }
    let tick_step = ((n as f64 / 12.0).ceil() as usize).max(1);
    for i in (0..n).step_by(tick_step) {
        let x = pad_l + (i as f64 * step_x) as i32;
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, x);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, pad_t + plot_h + 14);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        escape_xml(&mut b, truncate(&labels[i], 10));
        push_b(&mut b, b"</text>");
    }
    push_b(&mut b, b"</svg>");
    let slots_json;
    let json: &str = if auto {
        "[]"
    } else {
        slots_json = slots_to_json(hover);
        &slots_json
    };
    html_suffix(&mut b, hid, json);
    unsafe { String::from_utf8_unchecked(b) }
}

#[allow(dead_code)]
#[inline]
fn line_xml_esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
#[allow(dead_code)]
#[inline]
fn line_trunc(s: &str, max: usize) -> &str {
    if s.len() <= max {
        s
    } else {
        &s[..max]
    }
}

#[crate::sera_alias("line_chart")]
#[crate::sera_builder]
pub fn build_line_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let hover = o.hj();
    let html = crate::plot::default::render_lines_html(
        title,
        &labels,
        &values,
        o.w(900),
        o.h(480),
        &hover,
        o.color_hex.unwrap_or(0x636EFA),
        &o.xl(),
        &o.yl(),
        o.grid(),
        o.show_points.unwrap_or(true),
        &o.srt(),
    );
    apply(html, &o)
}
