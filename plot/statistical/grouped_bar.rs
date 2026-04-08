use super::common::{palette_color, push, push_b, escape_xml, truncate};
use crate::html::hover::{HoverSlot, slots_to_json, build_chart_html};

pub struct GroupedBar;

pub struct GroupedBarConfig<'a> {
    pub title: &'a str,
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub category_labels: &'a [String],
    pub series: &'a [(String, Vec<f64>)],
    pub palette: &'a [u32],
    pub width: i32,
    pub height: i32,
    pub stacked: bool,
    pub show_values: bool,
    pub value_min_height: i32,
    pub gridlines: bool,
    pub hover: &'a [HoverSlot],
}

impl<'a> Default for GroupedBarConfig<'a> {
    fn default() -> Self {
        Self {
            title: "",
            x_label: "",
            y_label: "",
            category_labels: &[],
            series: &[],
            palette: &[],
            width: 1100,
            height: 480,
            stacked: false,
            show_values: false,
            value_min_height: 16,
            gridlines: true,
            hover: &[],
        }
    }
}

pub fn render_grouped_bar_html(cfg: &GroupedBarConfig) -> String {
    let n_cats = cfg.category_labels.len();
    let n_ser = cfg.series.len();
    if n_cats == 0 || n_ser == 0 { return String::new(); }
    let max_val = if cfg.stacked {
        (0..n_cats).map(|ci| {
            cfg.series.iter()
                .filter_map(|(_, vals)| vals.get(ci).copied())
                .filter(|v| v.is_finite())
                .sum::<f64>()
        }).fold(0.0_f64, f64::max)
    } else {
        cfg.series.iter()
            .flat_map(|(_, vals)| vals.iter().copied())
            .filter(|v| v.is_finite())
            .fold(0.0_f64, f64::max)
    }.max(1.0);
    let legend_w: i32 = 160;
    let pad_l: i32 = 56;
    let pad_t: i32 = 42;
    let pad_b: i32 = 52;
    let pad_r: i32 = legend_w;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;
    let group_w = plot_w as f64 / n_cats as f64;
    let bar_w = if cfg.stacked {
        group_w * 0.62
    } else {
        group_w / (n_ser as f64 + 0.8) * 0.88
    };
    let auto_hover = cfg.hover.is_empty();
    let n_total = n_cats * n_ser;
    let mut auto_slots: Vec<HoverSlot> = if auto_hover { Vec::with_capacity(n_total) } else { Vec::new() };
    let mut buf = Vec::<u8>::with_capacity(n_total * 260 + 4096);
    push(&mut buf, &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">",
        w=cfg.width, h=cfg.height,
    ));
    push_b(&mut buf, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !cfg.title.is_empty() {
        push(&mut buf, &format!(
            "<text x=\"{tx}\" y=\"26\" text-anchor=\"middle\" \
             font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" \
             font-weight=\"700\" fill=\"#1a202c\">",
            tx = (cfg.width - legend_w) / 2 + pad_l,
        ));
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }
    let n_yticks: i32 = 6;
    for i in 0..=n_yticks {
        let frac = i as f64 / n_yticks as f64;
        let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        let vval = frac * max_val;
        if cfg.gridlines && i > 0 {
            push(&mut buf, &format!(
                "<line x1=\"{x1}\" y1=\"{y}\" x2=\"{x2}\" y2=\"{y}\" \
                 stroke=\"#e5e7eb\" stroke-width=\"0.6\" stroke-dasharray=\"3,3\"/>",
                x1=pad_l, x2=pad_l+plot_w, y=y,
            ));
        }
        let label = if vval >= 1000.0 { format!("{:.0}", vval) }
            else if vval >= 1.0 { format!("{:.1}", vval) }
            else { format!("{:.2}", vval) };
        push(&mut buf, &format!(
            "<text x=\"{x}\" y=\"{ty}\" text-anchor=\"end\" \
             font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">{v}</text>",
            x=pad_l-4, ty=y+3, v=label,
        ));
    }
    if !cfg.y_label.is_empty() {
        push(&mut buf, &format!(
            "<text x=\"14\" y=\"{y}\" text-anchor=\"middle\" \
             font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\" \
             transform=\"rotate(-90,14,{y})\">",
            y = pad_t + plot_h / 2,
        ));
        escape_xml(&mut buf, cfg.y_label);
        push_b(&mut buf, b"</text>");
    }
    push(&mut buf, &format!(
        "<line x1=\"{x}\" y1=\"{y1}\" x2=\"{x}\" y2=\"{y2}\" stroke=\"#9ca3af\" stroke-width=\"1.2\"/>",
        x=pad_l, y1=pad_t, y2=pad_t+plot_h,
    ));
    push(&mut buf, &format!(
        "<line x1=\"{x1}\" y1=\"{y}\" x2=\"{x2}\" y2=\"{y}\" stroke=\"#9ca3af\" stroke-width=\"1.2\"/>",
        x1=pad_l, x2=pad_l+plot_w, y=pad_t+plot_h,
    ));
    for (ci, cat) in cfg.category_labels.iter().enumerate() {
        let gx = pad_l as f64 + ci as f64 * group_w;
        let base_y = (pad_t + plot_h) as f64;
        let label_x = gx + group_w / 2.0;
        push(&mut buf, &format!(
            "<text x=\"{tx:.1}\" y=\"{ty}\" text-anchor=\"middle\" \
             font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">",
            tx=label_x, ty=pad_t+plot_h+16,
        ));
        escape_xml(&mut buf, truncate(cat, 16));
        push_b(&mut buf, b"</text>");
        if cfg.stacked {
            let mut stack_y = base_y;
            for (si, (sname, svals)) in cfg.series.iter().enumerate() {
                let val = svals.get(ci).copied().unwrap_or(0.0).max(0.0);
                let bh = (val / max_val * plot_h as f64).max(0.0);
                let bx = gx + (group_w - bar_w) / 2.0;
                let by = stack_y - bh;
                let color = palette_color(cfg.palette, si);
                let hx = {
                    let h = super::common::hex6(color);
                    unsafe { std::str::from_utf8_unchecked(&h).to_string() }
                };
                let hover_idx = ci * n_ser + si;
                push(&mut buf, &format!(
                    "<rect data-idx=\"{idx}\" x=\"{bx:.1}\" y=\"{by:.1}\" width=\"{bw:.1}\" height=\"{bh:.1}\" \
                     fill=\"#{hx}\" stroke=\"#fff\" stroke-width=\"0.5\"/>",
                    idx=hover_idx, bx=bx, by=by, bw=bar_w, bh=bh, hx=hx,
                ));
                if auto_hover {
                    auto_slots.push(
                        HoverSlot::new(format!("{} \u{2014} {}", cat, sname))
                            .kv("Valeur", format!("{:.2}", val))
                            .kv("Cat\u{e9}gorie", cat.clone())
                            .kv("S\u{e9}rie", sname.clone())
                    );
                }
                stack_y -= bh;
            }
        } else {
            let offset_start = (group_w - bar_w * n_ser as f64) / 2.0;
            for (si, (sname, svals)) in cfg.series.iter().enumerate() {
                let val = svals.get(ci).copied().unwrap_or(0.0).max(0.0);
                let bh = (val / max_val * plot_h as f64).max(0.0);
                let bx = gx + offset_start + si as f64 * bar_w;
                let by = base_y - bh;
                let color = palette_color(cfg.palette, si);
                let hx = {
                    let h = super::common::hex6(color);
                    unsafe { std::str::from_utf8_unchecked(&h).to_string() }
                };
                let hover_idx = ci * n_ser + si;
                push(&mut buf, &format!(
                    "<rect data-idx=\"{idx}\" x=\"{bx:.1}\" y=\"{by:.1}\" width=\"{bw:.1}\" height=\"{bh:.1}\" \
                     fill=\"#{hx}\" fill-opacity=\"0.88\" stroke=\"#fff\" stroke-width=\"0.4\"/>",
                    idx=hover_idx, bx=bx, by=by, bw=bar_w.max(1.0), bh=bh, hx=hx,
                ));
                if cfg.show_values && bh as i32 >= cfg.value_min_height {
                    let vlabel = if val >= 1.0 { format!("{:.1}", val) } else { format!("{:.2}", val) };
                    push(&mut buf, &format!(
                        "<text x=\"{tx:.1}\" y=\"{ty:.1}\" text-anchor=\"middle\" \
                         font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#1a202c\">{v}</text>",
                        tx = bx + bar_w / 2.0, ty = by - 2.0, v = vlabel,
                    ));
                }
                if auto_hover {
                    auto_slots.push(
                        HoverSlot::new(format!("{} \u{2014} {}", cat, sname))
                            .kv("Valeur", format!("{:.2}", val))
                            .kv("Cat\u{e9}gorie", cat.clone())
                            .kv("S\u{e9}rie", sname.clone())
                    );
                }
            }
        }
    }
    let leg_x = pad_l + plot_w + 12;
    let leg_top = pad_t + 8;
    for (si, (sname, _)) in cfg.series.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = {
            let h = super::common::hex6(color);
            unsafe { std::str::from_utf8_unchecked(&h).to_string() }
        };
        let ly = leg_top + si as i32 * 22;
        push(&mut buf, &format!(
            "<rect x=\"{lx}\" y=\"{ly}\" width=\"13\" height=\"13\" rx=\"3\" fill=\"#{hx}\"/>",
            lx=leg_x, ly=ly, hx=hx,
        ));
        push(&mut buf, &format!(
            "<text x=\"{tx}\" y=\"{ty}\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">",
            tx=leg_x+17, ty=ly+11,
        ));
        escape_xml(&mut buf, truncate(sname, 20));
        push_b(&mut buf, b"</text>");
    }
    if !cfg.x_label.is_empty() {
        push(&mut buf, &format!(
            "<text x=\"{tx}\" y=\"{ty}\" text-anchor=\"middle\" \
             font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">",
            tx = pad_l + plot_w / 2, ty = cfg.height - 6,
        ));
        escape_xml(&mut buf, cfg.x_label);
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots: &[HoverSlot] = if auto_hover { &auto_slots } else { cfg.hover };
    build_chart_html(cfg.title, &svg, &slots_to_json(slots))
}
