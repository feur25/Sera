use super::types::ChartOpts;

fn fmt_pos(v: f64, frac: bool) -> String {
    if frac {
        format!("{:.2}%", v * 100.0)
    } else {
        format!("{}", v)
    }
}

fn ann_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn apply_annotations(html: String, o: &ChartOpts) -> String {
    let anns = match &o.annotations {
        Some(a) if !a.is_empty() => a.clone(),
        _ => return html,
    };
    let mut buf = String::with_capacity(256 * anns.len());
    for a in &anns {
        let frac = a.frac.unwrap_or(true);
        let color = a.color.as_deref().unwrap_or("#ef4444");
        let sw = a.stroke_width.unwrap_or(1.5);
        let dash = a.dash.as_deref().unwrap_or("");
        let dash_attr = if dash.is_empty() {
            String::new()
        } else {
            format!(" stroke-dasharray=\"{}\"", dash)
        };
        let opacity = a.opacity.unwrap_or(1.0);
        let op_attr = if (opacity - 1.0).abs() < 1e-6 {
            String::new()
        } else {
            format!(" opacity=\"{:.2}\"", opacity)
        };
        match a.kind.as_str() {
            "hline" => {
                let y = fmt_pos(a.y.unwrap_or(0.5), frac);
                buf.push_str(&format!("<line x1=\"0\" x2=\"100%\" y1=\"{y}\" y2=\"{y}\" stroke=\"{color}\" stroke-width=\"{sw}\"{dash_attr}{op_attr}/>"));
                if let Some(t) = &a.text {
                    let fs = a.font_size.unwrap_or(11.0);
                    buf.push_str(&format!("<text x=\"6\" y=\"{y}\" dy=\"-3\" font-size=\"{fs}\" fill=\"{color}\" font-family=\"Arial,sans-serif\">{}</text>", ann_escape(t)));
                }
            }
            "vline" => {
                let x = fmt_pos(a.x.unwrap_or(0.5), frac);
                buf.push_str(&format!("<line y1=\"0\" y2=\"100%\" x1=\"{x}\" x2=\"{x}\" stroke=\"{color}\" stroke-width=\"{sw}\"{dash_attr}{op_attr}/>"));
                if let Some(t) = &a.text {
                    let fs = a.font_size.unwrap_or(11.0);
                    buf.push_str(&format!("<text x=\"{x}\" y=\"14\" dx=\"4\" font-size=\"{fs}\" fill=\"{color}\" font-family=\"Arial,sans-serif\">{}</text>", ann_escape(t)));
                }
            }
            "line" | "arrow" => {
                let x1 = fmt_pos(a.x.unwrap_or(0.0), frac);
                let y1 = fmt_pos(a.y.unwrap_or(0.0), frac);
                let x2 = fmt_pos(a.x2.unwrap_or(1.0), frac);
                let y2 = fmt_pos(a.y2.unwrap_or(1.0), frac);
                let arrow_id = format!("sp-ar-{}", buf.len());
                if a.kind == "arrow" {
                    buf.push_str(&format!("<defs><marker id=\"{arrow_id}\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"6\" markerHeight=\"6\" orient=\"auto\"><path d=\"M0,0 L10,5 L0,10 z\" fill=\"{color}\"/></marker></defs>"));
                }
                let m_attr = if a.kind == "arrow" {
                    format!(" marker-end=\"url(#{arrow_id})\"")
                } else {
                    String::new()
                };
                buf.push_str(&format!("<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"{color}\" stroke-width=\"{sw}\"{dash_attr}{op_attr}{m_attr}/>"));
            }
            "rect" => {
                let x = fmt_pos(a.x.unwrap_or(0.0), frac);
                let y = fmt_pos(a.y.unwrap_or(0.0), frac);
                let x2v = a.x2.unwrap_or(1.0);
                let y2v = a.y2.unwrap_or(1.0);
                let w = fmt_pos((x2v - a.x.unwrap_or(0.0)).max(0.0), frac);
                let h = fmt_pos((y2v - a.y.unwrap_or(0.0)).max(0.0), frac);
                let fill = a.fill.as_deref().unwrap_or("none");
                buf.push_str(&format!("<rect x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"{h}\" fill=\"{fill}\" stroke=\"{color}\" stroke-width=\"{sw}\"{dash_attr}{op_attr}/>"));
            }
            "text" => {
                let x = fmt_pos(a.x.unwrap_or(0.5), frac);
                let y = fmt_pos(a.y.unwrap_or(0.5), frac);
                let fs = a.font_size.unwrap_or(13.0);
                let t = a.text.clone().unwrap_or_default();
                buf.push_str(&format!("<text x=\"{x}\" y=\"{y}\" font-size=\"{fs}\" font-family=\"Arial,sans-serif\" fill=\"{color}\"{op_attr}>{}</text>", ann_escape(&t)));
            }
            _ => {}
        }
    }
    if buf.is_empty() {
        return html;
    }
    if let Some(idx) = html.rfind("</svg>") {
        let mut out = String::with_capacity(html.len() + buf.len() + 64);
        out.push_str(&html[..idx]);
        out.push_str("<g class=\"sp-annotations\" pointer-events=\"none\">");
        out.push_str(&buf);
        out.push_str("</g>");
        out.push_str(&html[idx..]);
        out
    } else {
        html
    }
}
