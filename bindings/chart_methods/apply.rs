use crate::json_str;
use super::js::*;

pub(crate) fn apply_despine(html: String) -> String {
    html.replacen(
        "</head>",
        "<style>.sp-ax-x,.sp-ax-y{display:none!important}</style></head>",
        1,
    )
}

pub(crate) fn apply_watermark(html: String, text: &str, opacity: f64) -> String {
    let o = opacity.clamp(0.0, 1.0);
    let css = format!(
        "<style>.sp-watermark{{position:fixed;top:50%;left:50%;transform:translate(-50%,-50%) rotate(-28deg);font-size:42px;font-weight:800;letter-spacing:.08em;color:rgba(148,163,184,{o});pointer-events:none;z-index:1;white-space:nowrap;user-select:none;font-family:-apple-system,Arial,sans-serif}}</style></head>",
    );
    let html = html.replacen("</head>", &css, 1);
    let snippet = format!(
        "<div class=\"sp-watermark\">{}</div></body>",
        text.replace('&', "&amp;").replace('<', "&lt;")
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_caption(html: String, text: &str) -> String {
    let css = "<style>.sp-caption{text-align:center;font-size:11px;color:#94a3b8;margin:6px 0 0;font-family:-apple-system,Arial,sans-serif}</style></head>";
    let html = html.replacen("</head>", css, 1);
    let snippet = format!(
        "<script>window.__sp_caption__={};{}</script></body>",
        json_str(text),
        SP_CAPTION_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_glow(html: String, color: &str) -> String {
    let mut html = html;
    if let Some(svg_pos) = html.find("<svg") {
        if let Some(rel_end) = html[svg_pos..].find('>') {
            let insert_at = svg_pos + rel_end + 1;
            let defs = format!(
                "<defs><filter id=\"sp-glow-f\" x=\"-60%\" y=\"-60%\" width=\"220%\" height=\"160%\"><feDropShadow dx=\"0\" dy=\"0\" stdDeviation=\"4\" flood-color=\"{}\" flood-opacity=\"0.85\"/></filter></defs>",
                color
            );
            html.insert_str(insert_at, &defs);
        }
    }
    html.replacen(
        "</head>",
        "<style>svg [data-idx]{filter:url(#sp-glow-f)}</style></head>",
        1,
    )
}

pub(crate) fn apply_neon_bloom(html: String, color: &str) -> String {
    let mut html = html;
    if let Some(svg_pos) = html.find("<svg") {
        if let Some(rel_end) = html[svg_pos..].find('>') {
            let insert_at = svg_pos + rel_end + 1;
            let defs = format!(
                "<defs><filter id=\"sp-bloom\" x=\"-180%\" y=\"-180%\" width=\"560%\" height=\"560%\" color-interpolation-filters=\"sRGB\">\
                <feFlood flood-color=\"{c}\" flood-opacity=\"1\" result=\"fc\"/>\
                <feComposite in=\"fc\" in2=\"SourceGraphic\" operator=\"in\" result=\"col\"/>\
                <feGaussianBlur in=\"col\" stdDeviation=\"16\" result=\"o3\"/>\
                <feGaussianBlur in=\"col\" stdDeviation=\"7\" result=\"o2\"/>\
                <feGaussianBlur in=\"col\" stdDeviation=\"2.5\" result=\"o1\"/>\
                <feMerge>\
                <feMergeNode in=\"o3\"/><feMergeNode in=\"o3\"/>\
                <feMergeNode in=\"o2\"/><feMergeNode in=\"o1\"/>\
                <feMergeNode in=\"SourceGraphic\"/>\
                </feMerge></filter></defs>",
                c = color
            );
            html.insert_str(insert_at, &defs);
        }
    }
    html.replacen(
        "</head>",
        "<style>svg [data-idx],svg path[stroke]:not([stroke=\"none\"]),svg circle[fill]:not([fill=\"none\"]),svg line[stroke]{filter:url(#sp-bloom)}</style></head>",
        1,
    )
}

pub(crate) fn apply_void(html: String, bg_color: &str) -> String {
    let css = format!(
        "<style>\
        body,html{{background:{bg}!important}}\
        [id^='spp'],.c3w{{background:{bg}!important;border-radius:8px}}\
        svg text:not([data-idx]):not([fill^='#e']):not([fill^='#f']):not([fill^='#a']):not([fill^='#c']){{fill:#94a3b8!important}}\
        svg line[stroke='#e2e8f0'],svg line[stroke='#cbd5e1'],svg line[stroke='#f1f5f9']{{stroke:#1e293b!important}}\
        </style></head>",
        bg = bg_color
    );
    let bg_json = json_str(bg_color);
    let js = format!(
        "<script>(function(){{\
        var svg=document.querySelector('svg');\
        if(!svg)return;\
        var vb=svg.viewBox.baseVal;\
        var sw=vb&&vb.width>0?vb.width:+(svg.getAttribute('width')||900);\
        var sh=vb&&vb.height>0?vb.height:+(svg.getAttribute('height')||500);\
        var ns='http://www.w3.org/2000/svg';\
        var bg=document.createElementNS(ns,'rect');\
        bg.setAttribute('width','100%');bg.setAttribute('height','100%');\
        bg.setAttribute('fill',{bg});\
        var first=svg.firstChild;\
        if(first&&first.nodeName==='rect'){{first.setAttribute('fill',{bg});}}else{{svg.insertBefore(bg,first);}}\
        var n=Math.ceil(sw*sh/3800);\
        var seed=2654435761;\
        function rand(){{seed=(seed^(seed<<13))>>>0;seed=(seed^(seed>>7))>>>0;seed=(seed^(seed<<17))>>>0;return seed/4294967296;}}\
        var sg=svg.insertBefore(document.createElementNS(ns,'g'),svg.firstChild.nextSibling||svg.firstChild);\
        sg.setAttribute('pointer-events','none');\
        for(var i=0;i<n;i++){{\
        var c=document.createElementNS(ns,'circle');\
        c.setAttribute('cx',(rand()*sw).toFixed(1));\
        c.setAttribute('cy',(rand()*sh).toFixed(1));\
        c.setAttribute('r',(rand()*1.1+0.2).toFixed(2));\
        c.setAttribute('fill','#ffffff');\
        c.setAttribute('fill-opacity',(rand()*0.32+0.04).toFixed(2));\
        sg.appendChild(c);\
        }}\
        }})();</script></body>",
        bg = bg_json
    );
    html.replacen("</head>", &css, 1)
        .replacen("</body>", &js, 1)
}

fn extract_bracket_range(html: &str, marker: &str) -> Option<(f64, f64)> {
    let start = html.find(marker)? + marker.len();
    let end = html[start..].find(']')? + start;
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for tok in html[start..end].split(',') {
        if let Ok(v) = tok.trim().parse::<f64>() {
            if v < lo {
                lo = v;
            }
            if v > hi {
                hi = v;
            }
        }
    }
    if lo.is_finite() && hi.is_finite() {
        Some((lo, hi))
    } else {
        None
    }
}

fn extract_attr_range(html: &str, attr: &str) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    let mut from = 0usize;
    while let Some(rel) = html[from..].find(attr) {
        let start = from + rel + attr.len();
        let end = start + html[start..].find('"')?;
        if let Ok(v) = html[start..end].parse::<f64>() {
            if v < lo {
                lo = v;
            }
            if v > hi {
                hi = v;
            }
        }
        from = end;
    }
    if lo.is_finite() && hi.is_finite() {
        Some((lo, hi))
    } else {
        None
    }
}

pub(crate) fn apply_colorbar(html: String, position: &str) -> String {
    if html.contains("box-shadow:0 2px 8px rgba(0,0,0,.25),0 0 0 1px rgba(255,255,255,.15)") {
        return html;
    }
    let pos = match position {
        "left" | "right" | "top" | "bottom" => position,
        _ => "right",
    };
    let is_3d = html.contains("class=\"c3w\"");
    let range = if is_3d {
        extract_bracket_range(&html, ",Z=[")
    } else {
        extract_attr_range(&html, "data-v=\"").or_else(|| extract_attr_range(&html, "data-y=\""))
    };
    let (lo, hi) = range.unwrap_or((0.0, 1.0));
    let horizontal = pos == "top" || pos == "bottom";
    let gradient = if horizontal {
        "linear-gradient(to right,#00008f,#00ffff,#00ff00,#ffff00,#ff0000)"
    } else {
        "linear-gradient(to top,#00008f,#00ffff,#00ff00,#ffff00,#ff0000)"
    };
    let (css_pos, w, h) = match pos {
        "left" => ("top:50%;left:10px;transform:translateY(-50%)", 14, 150),
        "top" => ("top:10px;left:50%;transform:translateX(-50%)", 150, 14),
        "bottom" => ("bottom:10px;left:50%;transform:translateX(-50%)", 150, 14),
        _ => ("top:50%;right:10px;transform:translateY(-50%)", 14, 150),
    };
    let label_css = if horizontal {
        "position:absolute;top:16px;font-size:10px;color:#94a3b8;font-family:-apple-system,Arial,sans-serif"
    } else {
        "position:absolute;left:18px;font-size:10px;color:#94a3b8;font-family:-apple-system,Arial,sans-serif"
    };
    let (lo_pos, hi_pos) = if horizontal {
        ("left:0", "right:0")
    } else {
        ("bottom:0", "top:0")
    };
    let bar = format!(
        "<div style=\"position:absolute;{};width:{}px;height:{}px;background:{};border-radius:4px;box-shadow:0 2px 8px rgba(0,0,0,.25),0 0 0 1px rgba(255,255,255,.15);z-index:50\"><span style=\"{};{}\">{:.2}</span><span style=\"{};{}\">{:.2}</span></div>",
        css_pos, w, h, gradient, label_css, lo_pos, lo, label_css, hi_pos, hi
    );
    if is_3d {
        html.replacen("</button></div>", &format!("</button>{}</div>", bar), 1)
    } else {
        let insert_at = html
            .find("</body>")
            .and_then(|body_pos| html[..body_pos].rfind("</div>"));
        match insert_at {
            Some(at) => {
                let mut out = html;
                out.insert_str(at, &bar);
                out
            }
            None => html,
        }
    }
}

pub(crate) fn apply_heatify(html: String, position: &str) -> String {
    let already_gauged =
        html.contains("box-shadow:0 2px 8px rgba(0,0,0,.25),0 0 0 1px rgba(255,255,255,.15)");
    let pos = match position {
        "left" | "right" | "top" | "bottom" => position,
        _ => "right",
    };
    let is_3d = html.contains("class=\"c3w\"");
    let range = if is_3d {
        extract_bracket_range(&html, ",Z=[")
    } else {
        extract_attr_range(&html, "data-v=\"").or_else(|| extract_attr_range(&html, "data-y=\""))
    };
    let (lo, hi) = range.unwrap_or((0.0, 1.0));
    let horizontal = pos == "top" || pos == "bottom";
    let gradient = if horizontal {
        "linear-gradient(to right,#6366f1,#06b6d4,#22c55e,#fbbf24,#ef4444)"
    } else {
        "linear-gradient(to top,#6366f1,#06b6d4,#22c55e,#fbbf24,#ef4444)"
    };
    let (css_pos, w, h) = match pos {
        "left" => ("top:50%;left:10px;transform:translateY(-50%)", 14, 150),
        "top" => ("top:10px;left:50%;transform:translateX(-50%)", 150, 14),
        "bottom" => ("bottom:10px;left:50%;transform:translateX(-50%)", 150, 14),
        _ => ("top:50%;right:10px;transform:translateY(-50%)", 14, 150),
    };
    let label_css = if horizontal {
        "position:absolute;top:16px;font-size:10px;color:#94a3b8;font-family:-apple-system,Arial,sans-serif"
    } else {
        "position:absolute;left:18px;font-size:10px;color:#94a3b8;font-family:-apple-system,Arial,sans-serif"
    };
    let (lo_pos, hi_pos) = if horizontal {
        ("left:0", "right:0")
    } else {
        ("bottom:0", "top:0")
    };
    let bar = format!(
        "<div style=\"position:absolute;{};width:{}px;height:{}px;background:{};border-radius:4px;box-shadow:0 2px 8px rgba(0,0,0,.25),0 0 0 1px rgba(255,255,255,.15);z-index:50\"><span style=\"{};{}\">{:.2}</span><span style=\"{};{}\">{:.2}</span></div>",
        css_pos, w, h, gradient, label_css, lo_pos, lo, label_css, hi_pos, hi
    );
    let recolored = html.replacen(
        "</body>",
        &format!("<script>{}</script></body>", SP_COLOR_DENSITY_JS),
        1,
    );
    if already_gauged {
        return recolored;
    }
    if is_3d {
        recolored.replacen("</button></div>", &format!("</button>{}</div>", bar), 1)
    } else {
        let insert_at = recolored
            .find("</body>")
            .and_then(|body_pos| recolored[..body_pos].rfind("</div>"));
        match insert_at {
            Some(at) => {
                let mut out = recolored;
                out.insert_str(at, &bar);
                out
            }
            None => recolored,
        }
    }
}

pub(crate) fn apply_orient3d(html: String, mode: &str) -> String {
    let (yaw, pitch) = crate::plot::scene3d::Orientation3D::from_str(mode).angles();
    let marker = "var yaw=";
    if let Some(start) = html.find(marker) {
        if let Some(rel) = html[start..].find(",zoom=") {
            let end = start + rel;
            let mut out = String::with_capacity(html.len() + 16);
            out.push_str(&html[..start]);
            out.push_str(&format!("var yaw={:.4},pitch={:.4}", yaw, pitch));
            out.push_str(&html[end..]);
            return out;
        }
    }
    html
}

pub(crate) fn apply_highlight(html: String, index: usize, color: &str) -> String {
    let css = format!(
        "<style>svg [data-idx=\"{}\"]{{fill:{}!important;stroke:{}!important;filter:drop-shadow(0 0 6px {})}}</style></head>",
        index, color, color, color
    );
    html.replacen("</head>", &css, 1)
}

pub(crate) fn apply_hline(html: String, value: f64, color: &str, label: Option<&str>) -> String {
    let cfg = format!(
        "{{\"v\":{},\"c\":{},\"lbl\":{}}}",
        value,
        json_str(color),
        label.map(json_str).unwrap_or_else(|| "null".to_string())
    );
    let snippet = format!("<script>window.__sp_hline__={};{}</script></body>", cfg, SP_HLINE_JS);
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_trendline(html: String, color: &str, width: f64) -> String {
    let cfg = format!("{{\"c\":{},\"w\":{}}}", json_str(color), width);
    let snippet = format!(
        "<script>window.__sp_trendline__={};{}</script></body>",
        cfg, SP_TRENDLINE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_annotate_extreme(
    html: String,
    mode: &str,
    color: &str,
    label: Option<&str>,
) -> String {
    let cfg = format!(
        "{{\"mode\":{},\"c\":{},\"lbl\":{}}}",
        json_str(mode),
        json_str(color),
        label.map(json_str).unwrap_or_else(|| "null".to_string())
    );
    let snippet = format!(
        "<script>window.__sp_annotate_extreme__={};{}</script></body>",
        cfg, SP_ANNOTATE_EXTREME_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_reference_band(html: String, low: f64, high: f64, color: &str, opacity: f64) -> String {
    let cfg = format!(
        "{{\"lo\":{},\"hi\":{},\"c\":{},\"op\":{}}}",
        low,
        high,
        json_str(color),
        opacity.clamp(0.0, 1.0)
    );
    let snippet = format!(
        "<script>window.__sp_ref_band__={};{}</script></body>",
        cfg, SP_REFERENCE_BAND_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_responsive(html: String) -> String {
    html.replacen(
        "</head>",
        "<style>html,body{max-width:100%;overflow:hidden}body>div{max-width:100%;min-width:0}svg{max-width:100%;width:100%;height:auto;display:block}canvas,.c3w,.sp-wrap,.chart-container{max-width:100%;overflow:hidden}</style></head>",
        1,
    )
}

pub(crate) fn apply_value_labels(html: String, decimals: i32, color: &str) -> String {
    let cfg = format!("{{\"d\":{},\"c\":{}}}", decimals.max(0), json_str(color));
    let snippet = format!(
        "<script>{}window.__sp_value_labels__={};{}</script></body>",
        SP_STACK_INIT_JS, cfg, SP_VALUE_LABELS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_error_bars(html: String, margin: f64, color: &str) -> String {
    let cfg = format!("{{\"m\":{},\"c\":{}}}", margin.abs(), json_str(color));
    let snippet = format!(
        "<script>{}window.__sp_error_bars__={};{}</script></body>",
        SP_STACK_INIT_JS, cfg, SP_ERROR_BARS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_delta_labels(html: String, pos_color: &str, neg_color: &str) -> String {
    let cfg = format!(
        "{{\"pc\":{},\"nc\":{}}}",
        json_str(pos_color),
        json_str(neg_color)
    );
    let snippet = format!(
        "<script>{}window.__sp_delta_labels__={};{}</script></body>",
        SP_STACK_INIT_JS, cfg, SP_DELTA_LABELS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_cumulative_line(html: String, color: &str) -> String {
    let cfg = format!("{{\"c\":{}}}", json_str(color));
    let snippet = format!(
        "<script>window.__sp_cumulative_line__={};{}</script></body>",
        cfg, SP_CUMULATIVE_LINE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_rank_badges(html: String, top_n: usize, color: &str) -> String {
    let cfg = format!("{{\"n\":{},\"c\":{}}}", top_n.max(1), json_str(color));
    let snippet = format!(
        "<script>{}window.__sp_rank_badges__={};{}</script></body>",
        SP_STACK_INIT_JS, cfg, SP_RANK_BADGES_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_log_scale(html: String) -> String {
    let snippet = format!("<script>{}</script></body>", SP_LOG_SCALE_JS);
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_moving_average(html: String, window: usize, color: &str) -> String {
    let cfg = format!("{{\"w\":{},\"c\":{}}}", window.max(1), json_str(color));
    let snippet = format!(
        "<script>window.__sp_moving_avg__={};{}</script></body>",
        cfg, SP_MOVING_AVG_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_outliers(html: String, threshold_std: f64, color: &str) -> String {
    let cfg = format!("{{\"t\":{},\"c\":{}}}", threshold_std.max(0.1), json_str(color));
    let snippet = format!(
        "<script>window.__sp_outliers__={};{}</script></body>",
        cfg, SP_OUTLIERS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_rolling_median(html: String, window: usize, color: &str) -> String {
    let cfg = format!("{{\"w\":{},\"c\":{}}}", window.max(1), json_str(color));
    let snippet = format!(
        "<script>window.__sp_rolling_median__={};{}</script></body>",
        cfg, SP_ROLLING_MEDIAN_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_change_points(html: String, threshold_std: f64, color: &str) -> String {
    let cfg = format!("{{\"t\":{},\"c\":{}}}", threshold_std.max(0.1), json_str(color));
    let snippet = format!(
        "<script>window.__sp_change_points__={};{}</script></body>",
        cfg, SP_CHANGE_POINTS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_rank_labels(html: String, ascending: bool) -> String {
    let cfg = format!("{{\"asc\":{}}}", ascending);
    let snippet = format!(
        "<script>window.__sp_rank_labels__={};{}</script></body>",
        cfg, SP_RANK_LABELS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_fill_between(html: String, color: &str, opacity: f64) -> String {
    let cfg = format!("{{\"c\":{},\"op\":{}}}", json_str(color), opacity.clamp(0.0, 1.0));
    let snippet = format!(
        "<script>window.__sp_fill_between__={};{}</script></body>",
        cfg, SP_FILL_BETWEEN_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_box_annotate(html: String, color: &str) -> String {
    let cfg = format!("{{\"c\":{}}}", json_str(color));
    let snippet = format!(
        "<script>window.__sp_box_annotate__={};{}</script></body>",
        cfg, SP_BOX_ANNOTATE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_pct_of_total(html: String, decimals: i32, color: &str) -> String {
    let cfg = format!(
        "{{\"d\":{},\"c\":{}}}",
        decimals.max(0),
        json_str(color)
    );
    let snippet = format!(
        "<script>{}window.__sp_pct_of_total__={};{}</script></body>",
        SP_STACK_INIT_JS, cfg, SP_PCT_OF_TOTAL_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_correlation_badge(html: String, color: &str) -> String {
    let cfg = format!("{{\"c\":{}}}", json_str(color));
    let snippet = format!(
        "<script>window.__sp_correlation_badge__={};{}</script></body>",
        cfg, SP_CORRELATION_BADGE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_highlight_range(html: String, low: usize, high: usize, color: &str, opacity: f64) -> String {
    let cfg = format!(
        "{{\"lo\":{},\"hi\":{},\"c\":{},\"op\":{}}}",
        low,
        high,
        json_str(color),
        opacity.clamp(0.0, 1.0)
    );
    let snippet = format!(
        "<script>window.__sp_highlight_range__={};{}</script></body>",
        cfg, SP_HIGHLIGHT_RANGE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_iqr_band(html: String, color: &str, opacity: f64) -> String {
    let cfg = format!("{{\"c\":{},\"op\":{}}}", json_str(color), opacity.clamp(0.0, 1.0));
    let snippet = format!(
        "<script>window.__sp_iqr_band__={};{}</script></body>",
        cfg, SP_IQR_BAND_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_growth_badge(html: String, color: &str) -> String {
    let cfg = format!("{{\"c\":{}}}", json_str(color));
    let snippet = format!(
        "<script>window.__sp_growth_badge__={};{}</script></body>",
        cfg, SP_GROWTH_BADGE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_zscore_heat(html: String) -> String {
    let snippet = format!(
        "<script>window.__sp_zscore_heat__=true;{}</script></body>",
        SP_ZSCORE_HEAT_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_pareto_marker(html: String, threshold_pct: f64, color: &str) -> String {
    let cfg = format!(
        "{{\"t\":{},\"c\":{}}}",
        threshold_pct.clamp(1.0, 99.0),
        json_str(color)
    );
    let snippet = format!(
        "<script>window.__sp_pareto_marker__={};{}</script></body>",
        cfg, SP_PARETO_MARKER_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_diff_from_mean(html: String, pos_color: &str, neg_color: &str) -> String {
    let cfg = format!(
        "{{\"pc\":{},\"nc\":{}}}",
        json_str(pos_color),
        json_str(neg_color)
    );
    let snippet = format!(
        "<script>{}window.__sp_diff_from_mean__={};{}</script></body>",
        SP_STACK_INIT_JS, cfg, SP_DIFF_FROM_MEAN_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_rolling_std_band(html: String, window: usize, color: &str, opacity: f64) -> String {
    let cfg = format!(
        "{{\"w\":{},\"c\":{},\"op\":{}}}",
        window.max(1),
        json_str(color),
        opacity.clamp(0.0, 1.0)
    );
    let snippet = format!(
        "<script>window.__sp_rolling_std_band__={};{}</script></body>",
        cfg, SP_ROLLING_STD_BAND_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_forecast_line(html: String, periods: usize, color: &str) -> String {
    let cfg = format!("{{\"n\":{},\"c\":{}}}", periods.max(1), json_str(color));
    let snippet = format!(
        "<script>window.__sp_forecast_line__={};{}</script></body>",
        cfg, SP_FORECAST_LINE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_percentile_band(html: String, low_pct: f64, high_pct: f64, color: &str, opacity: f64) -> String {
    let cfg = format!(
        "{{\"lo\":{},\"hi\":{},\"c\":{},\"op\":{}}}",
        low_pct.clamp(0.0, 100.0),
        high_pct.clamp(0.0, 100.0),
        json_str(color),
        opacity.clamp(0.0, 1.0)
    );
    let snippet = format!(
        "<script>window.__sp_percentile_band__={};{}</script></body>",
        cfg, SP_PERCENTILE_BAND_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_scatter_regression(html: String, color: &str, width: f64) -> String {
    let cfg = format!("{{\"c\":{},\"w\":{}}}", json_str(color), width.max(0.5));
    let snippet = format!(
        "<script>window.__sp_scatter_regression__={};{}</script></body>",
        cfg, SP_SCATTER_REGRESSION_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_cluster(html: String, eps: f64, min_samples: usize) -> String {
    let cfg = format!("{{\"eps\":{},\"m\":{}}}", eps.max(0.001), min_samples.max(1));
    let snippet = format!(
        "<script>window.__sp_cluster__={};{}</script></body>",
        cfg, SP_CLUSTER_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_subtitle(html: String, text: &str) -> String {
    let snippet = format!(
        "<script>(function(){{var t=document.querySelector('.sp-ttl');if(!t)return;var ns='http://www.w3.org/2000/svg';var s=document.createElementNS(ns,'text');s.setAttribute('x',t.getAttribute('x')||'0');s.setAttribute('y',(parseFloat(t.getAttribute('y'))||0)+16);s.setAttribute('text-anchor',t.getAttribute('text-anchor')||'middle');s.setAttribute('font-family','-apple-system,Arial,sans-serif');s.setAttribute('font-size','12');s.setAttribute('fill','#94a3b8');s.setAttribute('class','sp-subtitle');s.textContent={};t.parentNode.insertBefore(s,t.nextSibling);}})();</script></body>",
        json_str(text)
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_shadow(html: String, blur: i32, color: &str) -> String {
    let css = format!(
        "<style>body>:first-child{{box-shadow:0 {}px {}px -8px {} !important}}</style></head>",
        blur / 2,
        blur,
        color
    );
    html.replacen("</head>", &css, 1)
}

pub(crate) fn apply_pulse(
    html: String,
    duration: f64,
    indices: Option<&[usize]>,
    above: Option<f64>,
    color: Option<&str>,
) -> String {
    let d = duration.max(0.1);
    let keyframes = "@keyframes sp-pulse{0%,100%{opacity:1}50%{opacity:.55}}";
    if let Some(c) = color {
        let css = format!(
            "<style>{}@keyframes sp-pulse-c{{0%,100%{{fill:var(--sp-c0,currentColor)}}50%{{fill:var(--sp-c1,currentColor)}}}}</style></head>",
            keyframes
        );
        let html = html.replacen("</head>", &css, 1);
        let selector = match indices {
            Some(idxs) if !idxs.is_empty() => idxs
                .iter()
                .map(|i| format!("svg [data-idx=\"{}\"]", i))
                .collect::<Vec<_>>()
                .join(","),
            _ if above.is_some() => "svg [data-v]".to_string(),
            _ => "svg [data-idx]".to_string(),
        };
        let thr = above.map(|t| t.to_string()).unwrap_or_else(|| "null".to_string());
        let snippet = format!(
            "<script>(function(){{var sel={},c={},d={},thr={};document.querySelectorAll(sel).forEach(function(e){{if(thr!==null){{var v=parseFloat(e.getAttribute('data-v'));if(!(v>thr))return;}}var f=e.getAttribute('fill')||getComputedStyle(e).fill||'currentColor';e.style.setProperty('--sp-c0',f);e.style.setProperty('--sp-c1',c);e.style.setProperty('animation','sp-pulse '+d+'s ease-in-out infinite, sp-pulse-c '+d+'s ease-in-out infinite','important');}});}})();</script></body>",
            json_str(&selector), json_str(c), d, thr
        );
        return html.replacen("</body>", &snippet, 1);
    }
    if let Some(thr) = above {
        let css = format!("<style>{}</style></head>", keyframes);
        let html = html.replacen("</head>", &css, 1);
        let snippet = format!(
            "<script>(function(){{var thr={},d={};document.querySelectorAll('svg [data-v]').forEach(function(e){{if(parseFloat(e.getAttribute('data-v'))>thr){{e.style.setProperty('animation','sp-pulse '+d+'s ease-in-out infinite','important');}}}});}})();</script></body>",
            thr, d
        );
        return html.replacen("</body>", &snippet, 1);
    }
    let selector = match indices {
        Some(idxs) if !idxs.is_empty() => idxs
            .iter()
            .map(|i| format!("svg [data-idx=\"{}\"]", i))
            .collect::<Vec<_>>()
            .join(","),
        _ => "svg [data-idx]".to_string(),
    };
    let css = format!(
        "<style>{}{}{{animation:sp-pulse {}s ease-in-out infinite !important}}</style></head>",
        keyframes, selector, d
    );
    html.replacen("</head>", &css, 1)
}

pub(crate) fn apply_outline(html: String, color: &str, width: f64) -> String {
    let css = format!(
        "<style>svg [data-idx]{{stroke:{} !important;stroke-width:{}px !important}}</style></head>",
        color, width
    );
    html.replacen("</head>", &css, 1)
}

pub(crate) fn apply_turning_points(html: String, peak_color: &str, trough_color: &str) -> String {
    let cfg = format!(
        "{{\"pc\":{},\"tc\":{}}}",
        json_str(peak_color),
        json_str(trough_color)
    );
    let snippet = format!(
        "<script>window.__sp_turning_points__={};{}</script></body>",
        cfg, SP_TURNING_POINTS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_sigma_bands(html: String, n: f64, color: &str, opacity: f64) -> String {
    let cfg = format!(
        "{{\"n\":{},\"c\":{},\"op\":{}}}",
        n,
        json_str(color),
        opacity.clamp(0.0, 1.0)
    );
    let snippet = format!(
        "<script>window.__sp_sigma_bands__={};{}</script></body>",
        cfg, SP_SIGMA_BANDS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_annotate_gap(html: String, idx1: usize, idx2: usize, color: &str) -> String {
    let cfg = format!("{{\"i1\":{},\"i2\":{},\"c\":{}}}", idx1, idx2, json_str(color));
    let snippet = format!(
        "<script>window.__sp_annotate_gap__={};{}</script></body>",
        cfg, SP_ANNOTATE_GAP_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_callout(html: String, index: usize, text: &str, color: &str) -> String {
    let cfg = format!(
        "{{\"i\":{},\"t\":{},\"c\":{}}}",
        index,
        json_str(text),
        json_str(color)
    );
    let snippet = format!(
        "<script>window.__sp_callout__={};{}</script></body>",
        cfg, SP_CALLOUT_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_threshold_crossings(html: String, threshold: f64, up_color: &str, down_color: &str) -> String {
    let cfg = format!(
        "{{\"t\":{},\"up\":{},\"dn\":{}}}",
        threshold,
        json_str(up_color),
        json_str(down_color)
    );
    let snippet = format!(
        "<script>window.__sp_threshold_crossings__={};{}</script></body>",
        cfg, SP_THRESHOLD_CROSSINGS_JS
    );
    html.replacen("</body>", &snippet, 1)
}

pub(crate) fn apply_stats_badge(html: String, color: &str) -> String {
    let cfg = format!("{{\"c\":{}}}", json_str(color));
    let snippet = format!(
        "<script>window.__sp_stats_badge__={};{}</script></body>",
        cfg, SP_STATS_BADGE_JS
    );
    html.replacen("</body>", &snippet, 1)
}

fn texture_pattern_body(pattern: &str) -> (&'static str, u32, u32, &'static str) {
    match pattern {
        "dots" | "dotted" => (
            "<circle cx=\"5\" cy=\"5\" r=\"1.1\" fill=\"#000\"/>",
            10,
            10,
            "",
        ),
        "diagonal" | "diag" => (
            "<rect width=\"12\" height=\"1.4\" fill=\"#000\"/>",
            12,
            12,
            " patternTransform=\"rotate(45)\"",
        ),
        "crosshatch" | "hatch" | "hatching" | "cross" => (
            "<path d=\"M0,0L12,12M12,0L0,12\" stroke=\"#000\" stroke-width=\"0.6\" fill=\"none\"/>",
            12,
            12,
            "",
        ),
        "grid" | "waffle" => (
            "<path d=\"M12,0L0,0L0,12\" stroke=\"#000\" stroke-width=\"0.6\" fill=\"none\"/>",
            12,
            12,
            "",
        ),
        "noise" | "static" | "grain" => (
            "<circle cx=\"2\" cy=\"2\" r=\"0.5\" fill=\"#000\"/><circle cx=\"7\" cy=\"4.5\" r=\"0.45\" fill=\"#000\"/><circle cx=\"4\" cy=\"8\" r=\"0.4\" fill=\"#000\"/>",
            10,
            10,
            "",
        ),
        _ => (
            "<rect width=\"12\" height=\"1.4\" fill=\"#000\"/>",
            12,
            12,
            "",
        ),
    }
}

fn escape_text(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn parse_plot_rect(html: &str) -> Option<(f64, f64, f64, f64)> {
    let pos = html.find("data-sp=\"")?;
    let start = pos + "data-sp=\"".len();
    let end = html[start..].find('"')? + start;
    let mut parts = html[start..end].split(',');
    let l: f64 = parts.next()?.trim().parse().ok()?;
    let t: f64 = parts.next()?.trim().parse().ok()?;
    let w: f64 = parts.next()?.trim().parse().ok()?;
    let h: f64 = parts.next()?.trim().parse().ok()?;
    Some((l, t, w, h))
}

fn replace_text_class(html: &str, class: &str, new_text: &str) -> Option<String> {
    let marker = format!("class=\"{}\">", class);
    let pos = html.find(&marker)?;
    let start = pos + marker.len();
    let end_rel = html[start..].find("</text>")?;
    let mut out = String::with_capacity(html.len());
    out.push_str(&html[..start]);
    out.push_str(&escape_text(new_text));
    out.push_str(&html[start + end_rel..]);
    Some(out)
}

fn replace_attr(tag: &str, attr: &str, value: &str) -> String {
    let marker = format!("{}=\"", attr);
    if let Some(pos) = tag.find(&marker) {
        let start = pos + marker.len();
        if let Some(end_rel) = tag[start..].find('"') {
            let mut out = String::with_capacity(tag.len());
            out.push_str(&tag[..start]);
            out.push_str(value);
            out.push_str(&tag[start + end_rel..]);
            return out;
        }
    }
    tag.to_string()
}

pub(crate) fn apply_set_title(html: String, text: &str) -> String {
    if let Some(out) = replace_text_class(&html, "sp-ttl", text) {
        return out;
    }
    let Some((l, _t, w, _h)) = parse_plot_rect(&html) else {
        return html;
    };
    let cx = l + w / 2.0;
    let snippet = format!(
        "<text x=\"{cx}\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">{}</text>",
        escape_text(text)
    );
    let Some(pos) = html.find("</svg>") else {
        return html;
    };
    let mut out = html.clone();
    out.insert_str(pos, &snippet);
    out
}

pub(crate) fn apply_set_x_label(html: String, text: &str) -> String {
    if let Some(out) = replace_text_class(&html, "sp-xl", text) {
        return out;
    }
    let Some((l, t, w, h)) = parse_plot_rect(&html) else {
        return html;
    };
    let cx = l + w / 2.0;
    let y = t + h + 42.0;
    let snippet = format!(
        "<text x=\"{cx}\" y=\"{y}\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\" class=\"sp-xl\">{}</text>",
        escape_text(text)
    );
    let Some(pos) = html.find("</svg>") else {
        return html;
    };
    let mut out = html.clone();
    out.insert_str(pos, &snippet);
    out
}

pub(crate) fn apply_set_y_label(html: String, text: &str) -> String {
    if let Some(out) = replace_text_class(&html, "sp-yl", text) {
        return out;
    }
    let Some((l, t, _w, h)) = parse_plot_rect(&html) else {
        return html;
    };
    let cy = t + h / 2.0;
    let x: f64 = (l - 54.0).max(14.0);
    let snippet = format!(
        "<text x=\"{x}\" y=\"{cy}\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\" transform=\"rotate(-90,{x},{cy})\" class=\"sp-yl\">{}</text>",
        escape_text(text)
    );
    let Some(pos) = html.find("</svg>") else {
        return html;
    };
    let mut out = html.clone();
    out.insert_str(pos, &snippet);
    out
}

pub(crate) fn apply_set_size(html: String, w: Option<i32>, h: Option<i32>) -> String {
    let Some(svg_pos) = html.find("<svg") else {
        return html;
    };
    let Some(rel_end) = html[svg_pos..].find('>') else {
        return html;
    };
    let tag_end = svg_pos + rel_end;
    let mut tag = html[svg_pos..tag_end].to_string();
    if let Some(neww) = w {
        tag = replace_attr(&tag, "width", &neww.to_string());
    }
    if let Some(newh) = h {
        tag = replace_attr(&tag, "height", &newh.to_string());
    }
    let mut out = String::with_capacity(html.len());
    out.push_str(&html[..svg_pos]);
    out.push_str(&tag);
    out.push_str(&html[tag_end..]);
    out
}

fn collect_series_colors(html: &str) -> Vec<(usize, String)> {
    let mut pairs: Vec<(usize, String)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut idx = 0usize;
    let needle = "data-series=\"";
    while let Some(rel) = html[idx..].find(needle) {
        let pos = idx + rel + needle.len();
        let Some(end_rel) = html[pos..].find('"') else {
            break;
        };
        let series_str = &html[pos..pos + end_rel];
        if let Ok(series_idx) = series_str.parse::<usize>() {
            let tag_start = html[..pos].rfind('<').unwrap_or(pos);
            let tag_end = html[pos..].find('>').map(|e| pos + e).unwrap_or(html.len());
            let tag = &html[tag_start..tag_end];
            for attr in ["fill=\"#", "stroke=\"#"] {
                if let Some(frel) = tag.find(attr) {
                    let fpos = tag_start + frel + attr.len() - 1;
                    if let Some(fend) = html[fpos..].find('"') {
                        let color = html[fpos..fpos + fend].to_string();
                        if seen.insert((series_idx, color.clone())) {
                            pairs.push((series_idx, color));
                        }
                    }
                }
            }
        }
        idx = pos + end_rel;
    }
    pairs.sort_by_key(|(i, _)| *i);
    pairs
}

pub(crate) fn apply_palette(html: String, colors: &[u32]) -> String {
    if colors.is_empty() {
        return html;
    }
    let pairs = collect_series_colors(&html);
    let mut out = html;
    for (i, (_, old_color)) in pairs.iter().enumerate() {
        let token = format!("@@SPPAL{}@@", i);
        out = out.replace(old_color.as_str(), &token);
    }
    for (i, (series_idx, _)) in pairs.iter().enumerate() {
        let token = format!("@@SPPAL{}@@", i);
        let new_color = format!("#{:06x}", colors[series_idx % colors.len()] & 0xFFFFFF);
        out = out.replace(&token, &new_color);
    }
    out
}

pub(crate) fn apply_color_hex(html: String, color: &str) -> String {
    let pairs = collect_series_colors(&html);
    let mut out = html;
    for (_, old_color) in pairs {
        out = out.replace(old_color.as_str(), color);
    }
    out
}

pub(crate) fn apply_gridlines(html: String, on: bool) -> String {
    if on {
        html.replacen(
            "</head>",
            "<style>.sp-gl{display:block!important;opacity:1!important}</style></head>",
            1,
        )
    } else {
        html.replacen(
            "</head>",
            "<style>.sp-gl{display:none!important}</style></head>",
            1,
        )
    }
}

pub(crate) fn apply_hover_toggle(html: String, on: bool) -> String {
    if on {
        return html;
    }
    html.replacen(
        "</head>",
        "<style>#sp-tip{display:none!important}[data-idx]{pointer-events:none!important}[data-idx]:hover{filter:none!important}</style></head>",
        1,
    )
}

pub(crate) fn apply_texture(html: String, pattern: &str, opacity: f64) -> String {
    let mut html = html;
    let (body, w, h, transform) = texture_pattern_body(pattern);
    if let Some(svg_pos) = html.find("<svg") {
        if let Some(rel_end) = html[svg_pos..].find('>') {
            let insert_at = svg_pos + rel_end + 1;
            let defs = format!(
                "<defs><pattern id=\"sp-texture-pat\" width=\"{w}\" height=\"{h}\" patternUnits=\"userSpaceOnUse\"{transform}>{body}</pattern></defs>"
            );
            html.insert_str(insert_at, &defs);
        }
    }
    let cfg = format!("{{\"op\":{}}}", opacity.clamp(0.0, 1.0));
    let snippet = format!(
        "<script>window.__sp_texture__={};{}</script></body>",
        cfg, SP_TEXTURE_JS
    );
    html.replacen("</body>", &snippet, 1)
}
