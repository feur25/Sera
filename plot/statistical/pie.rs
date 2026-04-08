use super::common::{palette_color, push, push_b, hex6, escape_xml, truncate};
use crate::html::hover::{HoverSlot, slots_to_json, build_chart_html};

pub struct Pie;

pub struct PieConfig<'a> {
    pub title: &'a str,
    pub labels: &'a [String],
    pub values: &'a [f64],
    pub width: i32,
    pub height: i32,
    pub donut: f64,
    pub show_pct: bool,
    pub min_label_frac: f64,
    pub palette: &'a [u32],
    pub hover: &'a [HoverSlot],
}

impl<'a> Default for PieConfig<'a> {
    fn default() -> Self {
        Self {
            title: "",
            labels: &[],
            values: &[],
            width: 720,
            height: 440,
            donut: 0.0,
            show_pct: true,
            min_label_frac: 0.04,
            palette: &[],
            hover: &[],
        }
    }
}

pub fn render_pie_html(cfg: &PieConfig) -> String {
    use std::f64::consts::PI;
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 { return String::new(); }
    let total: f64 = cfg.values[..n].iter().sum();
    if total <= 0.0 { return String::new(); }
    let w = cfg.width;
    let h = cfg.height;
    let chart_w = w as f64 * 0.62;
    let cx = chart_w * 0.50;
    let cy = h as f64 * 0.52;
    let r = (cx.min(cy * 0.90) * 0.84).max(1.0);
    let r_inner = if cfg.donut > 0.0 { r * cfg.donut.clamp(0.0, 0.90) } else { 0.0 };
    let mut buf = Vec::<u8>::with_capacity(n * 380 + 1024);
    push(&mut buf, &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">",
    ));
    push_b(&mut buf, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !cfg.title.is_empty() {
        push(&mut buf, &format!(
            "<text x=\"{tx}\" y=\"22\" text-anchor=\"middle\" \
             font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" \
             font-weight=\"700\" fill=\"#1a202c\">",
            tx = w / 2,
        ));
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }
    let auto_hover = cfg.hover.is_empty();
    let mut auto_slots: Vec<HoverSlot> = if auto_hover { Vec::with_capacity(n) } else { Vec::new() };
    let mut angle = -PI / 2.0;
    for i in 0..n {
        let frac = cfg.values[i] / total;
        let sweep = frac * 2.0 * PI;
        let end_angle = angle + sweep;
        let c = palette_color(cfg.palette, i);
        let hx = hex6(c);
        let hx_str = unsafe { std::str::from_utf8_unchecked(&hx) };
        let large_arc: u8 = if sweep > PI { 1 } else { 0 };
        let x1 = cx + r * angle.cos();
        let y1 = cy + r * angle.sin();
        let x2 = cx + r * end_angle.cos();
        let y2 = cy + r * end_angle.sin();
        if r_inner > 0.0 {
            let xi1 = cx + r_inner * angle.cos();
            let yi1 = cy + r_inner * angle.sin();
            let xi2 = cx + r_inner * end_angle.cos();
            let yi2 = cy + r_inner * end_angle.sin();
            push(&mut buf, &format!(
                "<path data-idx=\"{i}\" \
                 d=\"M{x1:.2},{y1:.2} A{r:.2},{r:.2} 0 {la},1 {x2:.2},{y2:.2} \
                 L{xi2:.2},{yi2:.2} A{ri:.2},{ri:.2} 0 {la},0 {xi1:.2},{yi1:.2} Z\" \
                 fill=\"#{hx}\" stroke=\"#fff\" stroke-width=\"1.5\"/>",
                i=i, x1=x1, y1=y1, r=r, la=large_arc, x2=x2, y2=y2,
                xi2=xi2, yi2=yi2, ri=r_inner, xi1=xi1, yi1=yi1, hx=hx_str,
            ));
        } else {
            push(&mut buf, &format!(
                "<path data-idx=\"{i}\" \
                 d=\"M{cx:.2},{cy:.2} L{x1:.2},{y1:.2} A{r:.2},{r:.2} 0 {la},1 {x2:.2},{y2:.2} Z\" \
                 fill=\"#{hx}\" stroke=\"#fff\" stroke-width=\"1.8\"/>",
                i=i, cx=cx, cy=cy, x1=x1, y1=y1, r=r, la=large_arc, x2=x2, y2=y2, hx=hx_str,
            ));
        }
        if cfg.show_pct && frac >= cfg.min_label_frac {
            let mid = angle + sweep / 2.0;
            let lr = if r_inner > 0.0 { (r + r_inner) / 2.0 } else { r * 0.66 };
            push(&mut buf, &format!(
                "<text x=\"{lx:.1}\" y=\"{ly:.1}\" text-anchor=\"middle\" \
                 dominant-baseline=\"central\" font-family=\"Arial,sans-serif\" \
                 font-size=\"11\" font-weight=\"700\" fill=\"#fff\">{pct:.0}%</text>",
                lx = cx + lr * mid.cos(), ly = cy + lr * mid.sin(), pct = frac * 100.0,
            ));
        }
        if auto_hover {
            auto_slots.push(
                HoverSlot::new(cfg.labels[i].clone())
                    .kv("Valeur", format!("{:.1}", cfg.values[i]))
                    .kv("Part", format!("{:.1}%", frac * 100.0))
            );
        }
        angle = end_angle;
    }
    if r_inner > 0.0 {
        push(&mut buf, &format!(
            "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{ri:.1}\" fill=\"#fff\"/>",
            cx=cx, cy=cy, ri=r_inner - 1.0,
        ));
    }
    let leg_x = (w as f64 * 0.66) as i32;
    let leg_top = ((h as f64 - n as f64 * 22.0) / 2.0).max(30.0) as i32;
    for i in 0..n {
        let frac = cfg.values[i] / total;
        let c = palette_color(cfg.palette, i);
        let hx = hex6(c);
        let hx_str = unsafe { std::str::from_utf8_unchecked(&hx) };
        let ly = leg_top + i as i32 * 22;
        push(&mut buf, &format!(
            "<rect x=\"{lx}\" y=\"{ly}\" width=\"13\" height=\"13\" rx=\"3\" fill=\"#{hx}\"/>",
            lx=leg_x, ly=ly, hx=hx_str,
        ));
        push(&mut buf, &format!(
            "<text x=\"{tx}\" y=\"{ty}\" font-family=\"Arial,sans-serif\" \
             font-size=\"11\" fill=\"#374151\">",
            tx=leg_x+17, ty=ly+11,
        ));
        escape_xml(&mut buf, truncate(&cfg.labels[i], 22));
        push(&mut buf, &format!(" ({:.1}%)</text>", frac * 100.0));
    }
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots = if auto_hover { &auto_slots } else { cfg.hover };
    build_chart_html(cfg.title, &svg, &slots_to_json(slots))
}
