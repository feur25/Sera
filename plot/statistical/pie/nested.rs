use super::common::{render_pie_svg, PiePiece, open_svg, write_title};
use super::config::PieConfig;
use crate::plot::statistical::common::{push_b, push_i, escape_xml};

pub const DEMO_KWARGS: &str = "labels=[\"Apple\",\"Banana\",\"Cherry\",\"Date\",\"Fig\"], values=[40,25,20,10,5], secondary_labels=[\"X\",\"Y\",\"Z\"], secondary_values=[55,30,15]";
pub fn render(cfg: &PieConfig) -> String {
    let outer_labels: Vec<String> = cfg.labels.to_vec();
    let outer_values: Vec<f64> = cfg.values.to_vec();
    let inner_values: Vec<f64> = cfg.secondary_values.to_vec();
    let inner_labels: Vec<String> = if cfg.secondary_labels.is_empty() {
        outer_labels.clone()
    } else {
        cfg.secondary_labels.to_vec()
    };

    if outer_values.is_empty() && inner_values.is_empty() {
        return super::basic::render(cfg);
    }

    let w = cfg.width.max(420);
    let h = cfg.height.max(320);

    let mut buf = Vec::<u8>::with_capacity(2048);
    open_svg(&mut buf, w, h);
    write_title(&mut buf, w, cfg.title);

    let title_pad = if cfg.title.is_empty() { 0.0 } else { 30.0 };
    let pull_empty: Vec<f64> = Vec::new();

    let outer_donut = if cfg.donut > 0.0 { cfg.donut } else { 0.55 };
    let outer_piece = PiePiece {
        area_x: 0.0,
        area_y: title_pad,
        area_w: w as f64,
        area_h: h as f64 - title_pad,
        donut: outer_donut,
        draw_legend: true,
        ..PiePiece::default()
    };
    if !outer_values.is_empty() {
        render_pie_svg(&mut buf, cfg, &outer_labels, &outer_values, &pull_empty, &outer_piece);
    }

    if !inner_values.is_empty() {
        let inner_radius_factor = (outer_donut.max(0.35) - 0.10).clamp(0.25, 0.75);
        let inner_piece = PiePiece {
            area_x: 0.0,
            area_y: title_pad,
            area_w: w as f64 * 0.62,
            area_h: h as f64 - title_pad,
            donut: 0.4,
            radius_scale: outer_donut,
            draw_legend: false,
            palette_offset: outer_values.len(),
            pattern_id_offset: outer_values.len(),
            center_text: cfg.center_text.to_string(),
            center_subtext: cfg.center_subtext.to_string(),
            ..PiePiece::default()
        };
        render_pie_svg(&mut buf, cfg, &inner_labels, &inner_values, &pull_empty, &inner_piece);
    } else if !cfg.center_text.is_empty() {
        let cx = (w as f64) * 0.31;
        let cy = title_pad + (h as f64 - title_pad) * 0.52;
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, cx as i32);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, cy as i32);
        push_b(&mut buf, b"\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"-apple-system,Arial,sans-serif\" font-weight=\"800\" font-size=\"22\" fill=\"#f1f5f9\">");
        escape_xml(&mut buf, cfg.center_text);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");
    unsafe { String::from_utf8_unchecked(buf) }
}


