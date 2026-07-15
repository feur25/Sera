fn sniff_attr(s: &str, attr: &str, lo: u32, hi: u32, default: u32) -> u32 {
    let needle = format!("{}=\"", attr);
    let mut start = 0usize;
    loop {
        match s[start..].find(needle.as_str()) {
            None => return default,
            Some(rel) => {
                let abs = start + rel + needle.len();
                if let Some(end) = s[abs..].find('"') {
                    if let Ok(v) = s[abs..abs + end].parse::<u32>() {
                        if v >= lo && v <= hi {
                            return v;
                        }
                    }
                }
                start += rel + 1;
            }
        }
    }
}

fn escaped_attr(s: &str) -> String {
    s.replace('&', "&amp;").replace('"', "&quot;")
}

fn cleaned_html(html: &str) -> String {
    let html = html.replace(
        "border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)",
        "border-radius:0;overflow:hidden",
    );
    html.replacen(
        "</head>",
        "<style>html,body{margin:0!important;padding:0!important;overflow:hidden!important;background:transparent}body>div{max-width:100%!important;min-width:0!important}svg{max-width:100%!important;width:100%!important;height:auto!important;display:block}canvas,.c3w,.sp-wrap,.chart-container{max-width:100%!important;overflow:hidden!important}</style></head>",
        1,
    )
}

fn sniff_meta_size(s: &str) -> Option<(u32, u32)> {
    let needle = "name=\"sp-size\" content=\"";
    let start = s.find(needle)? + needle.len();
    let end = start + s[start..].find('"')?;
    let raw = &s[start..end];
    let (w, h) = raw.split_once('x')?;
    let w = w.parse::<u32>().ok()?;
    let h = h.parse::<u32>().ok()?;
    if (150..=8000).contains(&w) && (150..=8000).contains(&h) {
        Some((w, h))
    } else {
        None
    }
}

fn frame_height(html: &str) -> u32 {
    let base = sniff_attr(html, "height", 150, 2200, 560);
    let mut extra = 80;
    if html.contains("sp-caption") || html.contains("__sp_caption__") {
        extra += 36;
    }
    if html.contains("__sprs__") || html.contains("__sp_frames__") {
        extra += 52;
    }
    (base + extra).clamp(360, 1240)
}

fn frame_dims(html: &str) -> (u32, u32) {
    sniff_meta_size(html).unwrap_or_else(|| {
        (
            sniff_attr(html, "width", 150, 2400, 900),
            frame_height(html),
        )
    })
}

pub(crate) fn chart_iframe(html: &str) -> String {
    let (w, h) = frame_dims(html);
    let clean = cleaned_html(html);
    let esc = escaped_attr(&clean);
    format!(
        r#"<iframe srcdoc="{esc}" scrolling="no" style="width:100%;max-width:100%;aspect-ratio:{w}/{h};border:none;display:block;border-radius:8px;overflow:hidden" frameborder="0" loading="lazy"></iframe>"#
    )
}

#[cfg(feature = "python")]
pub(crate) fn cmp_score(html: &str, metric: &str) -> f64 {
    let mut vals: Vec<f64> = Vec::new();
    for attr in &["data-v=\"", "data-y=\""] {
        let mut pos = 0;
        while let Some(i) = html[pos..].find(attr) {
            let start = pos + i + attr.len();
            if let Some(end) = html[start..].find('"') {
                if let Ok(v) = html[start..start + end].parse::<f64>() {
                    vals.push(v);
                }
            }
            pos = start + 1;
        }
    }
    if vals.is_empty() {
        return 0.0;
    }
    match metric {
        "mean" | "avg" => vals.iter().sum::<f64>() / vals.len() as f64,
        "max" => vals.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        "min" => vals.iter().copied().fold(f64::INFINITY, f64::min),
        "count" => vals.len() as f64,
        _ => vals.iter().sum(),
    }
}

#[cfg(feature = "python")]
fn cmp_title(html: &str) -> String {
    if let Some(i) = html.find("sp-ttl") {
        if let Some(gt) = html[i..].find('>') {
            let start = i + gt + 1;
            if let Some(lt) = html[start..].find('<') {
                let t = html[start..start + lt].trim();
                if !t.is_empty() {
                    return t.to_string();
                }
            }
        }
    }
    "Chart".to_string()
}

#[cfg(feature = "python")]
fn apply_visual_scale(html: String, target_max: f64) -> String {
    if target_max <= 0.0 {
        return html;
    }
    let snippet = format!(
        "<script>(function(){{var svg=document.querySelector('svg');if(!svg)return;var sp=(svg.getAttribute('data-sp')||'').split(',').map(Number);if(sp.length<4)return;var pl=sp[0],pt=sp[1],pw=sp[2],ph=sp[3];var tmax={};svg.querySelectorAll('[data-idx][data-v]').forEach(function(e){{var v=parseFloat(e.getAttribute('data-v'));if(isNaN(v))return;var nh=(v/tmax)*ph;var ny=pt+ph-nh;var tag=e.tagName.toLowerCase();if(tag==='rect'){{e.setAttribute('height',Math.max(0,nh));e.setAttribute('y',ny);}}else if(tag==='circle'){{e.setAttribute('cy',ny);}}}});}})();</script></body>",
        target_max
    );
    html.replacen("</body>", &snippet, 1)
}

#[cfg(feature = "python")]
pub(crate) fn build_compare_page(
    htmls: &[String],
    metric: &str,
    arrows: bool,
    scale_target: Option<f64>,
) -> String {
    let rank_colors = ["#fbbf24", "#94a3b8", "#cd7c2f", "#64748b"];
    let metric_lbl = match metric {
        "mean" | "avg" => "avg",
        "max" => "max",
        "min" => "min",
        "count" => "n",
        _ => "sum",
    };
    let mut entries: Vec<(String, f64, String)> = htmls
        .iter()
        .map(|html| {
            let display = match scale_target {
                Some(t) if t > 0.0 => apply_visual_scale(html.clone(), t),
                _ => html.clone(),
            };
            (
                cmp_title(html),
                cmp_score(html, metric),
                chart_iframe(&display),
            )
        })
        .collect();
    entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let top = entries.first().map(|e| e.1).unwrap_or(0.0);
    let n = entries.len();
    let grid_cols = if arrows {
        (0..n)
            .map(|i| {
                if i == 0 {
                    "1fr".to_string()
                } else {
                    "36px 1fr".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        format!("repeat({n}, minmax(0, 1fr))")
    };
    let mut cells: Vec<String> = Vec::new();
    for (i, (title, score, iframe)) in entries.iter().enumerate() {
        if i > 0 && arrows {
            let prev = entries[i - 1].1;
            let diff = if top.abs() > 1e-9 {
                (prev - score).abs() / top.abs()
            } else {
                0.0
            };
            let arr = if diff > 0.5 {
                ">>>"
            } else if diff > 0.2 {
                ">>"
            } else {
                ">"
            };
            cells.push(format!(
                "<div style='display:flex;justify-content:center;align-items:flex-start;padding-top:36px;color:#475569;font:700 11px/1 system-ui'>{arr}</div>"
            ));
        }
        let color = rank_colors.get(i).copied().unwrap_or("#64748b");
        let rank = match i {
            0 => "#1",
            1 => "#2",
            2 => "#3",
            _ => "#?",
        };
        let delta = if i > 0 {
            format!(" - {:.2}", top - score)
        } else {
            String::new()
        };
        cells.push(format!(
            "<div style='display:flex;flex-direction:column;gap:7px;min-width:0'>\
<div style='display:flex;align-items:center;gap:6px;background:rgba(15,23,42,.9);padding:6px 14px;border-radius:18px;border:2px solid {color};font:600 11px/1.2 system-ui,sans-serif;max-width:100%;box-shadow:0 2px 10px rgba(0,0,0,.4);flex-wrap:wrap'>\
<span style='color:{color};flex-shrink:0'>{rank}</span>\
<span style='color:#f1f5f9'>{title}</span>\
<span style='color:#94a3b8;font-weight:400;flex-shrink:0'>{metric_lbl} {score:.2}{delta}</span>\
</div>{iframe}</div>"
        ));
    }
    format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\">\
<style>*{{box-sizing:border-box;margin:0;padding:0}}body{{background:transparent;font-family:system-ui,sans-serif;padding:4px 0}}</style>\
</head><body>\
<div style=\"display:grid;grid-template-columns:{grid_cols};gap:12px;align-items:start\">{cells}</div>\
</body></html>",
        grid_cols = grid_cols,
        cells = cells.join("")
    )
}

pub(crate) fn build_grid_page(
    htmls: &[String],
    cols: usize,
    title: Option<&str>,
    gap: u32,
    background: &str,
) -> String {
    let n = htmls.len();
    let c = if n == 0 { 1 } else { cols.clamp(1, n) };
    let dims: Vec<(u32, u32)> = htmls.iter().map(|html| frame_dims(html)).collect();
    let cell_w = dims.iter().map(|(w, _)| *w).max().unwrap_or(900);
    let mut page_h = if title.filter(|t| !t.is_empty()).is_some() {
        34
    } else {
        8
    };
    for row in dims.chunks(c) {
        let row_h = row
            .iter()
            .map(|(w, h)| ((*h as f64) * (cell_w as f64) / (*w as f64).max(1.0)).ceil() as u32)
            .max()
            .unwrap_or(0);
        page_h += row_h;
    }
    let rows = if n == 0 { 0 } else { (n + c - 1) / c };
    if rows > 1 {
        page_h += gap.saturating_mul((rows - 1) as u32);
    }
    let page_w = (cell_w * c as u32).saturating_add(gap.saturating_mul(c.saturating_sub(1) as u32));
    let grid_label = title.filter(|t| !t.is_empty()).unwrap_or("Chart grid");
    let title_html = title
        .map(|t| {
            format!(
                "<div style=\"text-align:center;margin-bottom:10px;font:600 13px/1.2 system-ui,sans-serif;color:#64748b\">{}</div>",
                t.replace('&', "&amp;").replace('<', "&lt;")
            )
        })
        .unwrap_or_default();
    let cells: String = htmls
        .iter()
        .enumerate()
        .map(|(i, html)| {
            format!(
                "<div style='min-width:0' role=\"group\" aria-label=\"Chart {}\">{}</div>",
                i + 1,
                chart_iframe(html)
            )
        })
        .collect::<Vec<_>>()
        .join("");
    format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\">\
<meta name=\"sp-size\" content=\"{page_w}x{page_h}\">\
<style>*{{box-sizing:border-box;margin:0;padding:0}}html,body{{background:{background};font-family:system-ui,sans-serif;padding:0;overflow:hidden}}body{{padding:4px 0}}iframe{{overflow:hidden}}</style>\
</head><body>{title_html}<div role=\"region\" aria-label=\"{grid_label}\" style=\"display:grid;grid-template-columns:repeat({c},minmax(0,1fr));gap:{gap}px;align-items:start\">{cells}</div></body></html>",
        page_w = page_w,
        page_h = page_h,
        title_html = title_html,
        c = c,
        gap = gap,
        background = background,
        cells = cells
    )
}
