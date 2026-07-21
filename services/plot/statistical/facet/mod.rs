use crate::plot::chart_input::sanitize_non_finite_json;
use serde_json::{Map, Value};

type BuilderFn = fn(&str) -> String;

fn dispatch(family: &str) -> Option<BuilderFn> {
    use crate::plot::statistical as st;
    Some(match family {
        "area" => st::area::build_area_chart,
        "bar" => st::bar::build_bar,
        "boxplot" | "box" => st::boxplot::build_boxplot,
        "bubble" => st::bubble::build_bubble,
        "bullet" => st::bullet::build_bullet,
        "candlestick" => st::candlestick::build_candlestick,
        "dumbbell" => st::dumbbell::build_dumbbell,
        "eventplot" => st::eventplot::build_eventplot,
        "funnel" => st::funnel::build_funnel,
        "gantt" => st::gantt::build_gantt,
        "gauge" => st::gauge::build_gauge,
        "heatmap" => st::heatmap::build_heatmap,
        "hexbin" => st::hexbin::build_hexbin,
        "histogram" | "hist" => st::histogram::build_histogram,
        "icicle" => st::icicle::build_icicle,
        "joint" | "jointplot" | "bivariate" => st::joint::build_joint,
        "kde" | "density" => st::kde::build_kde_chart,
        "line" | "lineplot" => st::line::build_line,
        "lollipop" => st::lollipop::build_lollipop_chart,
        "parallel" => st::parallel::build_parallel,
        "pie" => st::pie::build_pie,
        "radar" | "polar" => st::radar::build_radar_chart,
        "ridgeline" => st::ridgeline::build_ridgeline_chart,
        "scatter" => st::scatter::build_scatter_chart,
        "slope" => st::slope::build_slope,
        "splom" => st::splom::build_splom,
        "stackplot" => st::stackplot::build_stackplot,
        "sunburst" => st::sunburst::build_sunburst,
        "treemap" => st::treemap::build_treemap,
        "violin" => st::violin::build_violin,
        "waterfall" => st::waterfall::build_waterfall,
        "wordcloud" => st::wordcloud::build_wordcloud,
        _ => return None,
    })
}

fn value_to_key(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => "?".to_string(),
    }
}

fn html_attr_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
    out
}

fn html_text_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
    out
}

fn cell_input(obj: &Map<String, Value>, facet_key: &[String], n: usize, group: &str, cell_w: i64, cell_h: i64, title: &str) -> String {
    let mut cell = Map::new();
    for (k, v) in obj.iter() {
        if k == "family" || k == "facet_by" || k == "cols" || k == "cell_width" || k == "cell_height" || k == "layout" {
            continue;
        }
        if let Value::Array(arr) = v {
            if arr.len() == n {
                let filtered: Vec<Value> = arr
                    .iter()
                    .zip(facet_key.iter())
                    .filter(|(_, fk)| fk.as_str() == group)
                    .map(|(item, _)| item.clone())
                    .collect();
                cell.insert(k.clone(), Value::Array(filtered));
                continue;
            }
        }
        cell.insert(k.clone(), v.clone());
    }
    cell.insert("width".to_string(), Value::from(cell_w));
    cell.insert("height".to_string(), Value::from(cell_h));
    let cell_title = if title.is_empty() { group.to_string() } else { format!("{title} \u{2014} {group}") };
    cell.insert("title".to_string(), Value::from(cell_title));
    Value::Object(cell).to_string()
}

fn facet_cell_html(group: &str, cell_w: i64, cell_h: i64, cell_html: &str) -> String {
    format!(
        "<div class=\"sp-facet-cell\" style=\"width:{cw}px\">\
<iframe class=\"sp-facet-frame\" style=\"width:{cw}px;height:{ch}px;border:0;background:transparent\" \
sandbox=\"allow-scripts\" srcdoc=\"{doc}\"></iframe>\
<div class=\"sp-facet-label\">{label}</div></div>",
        cw = cell_w,
        ch = cell_h,
        doc = html_attr_escape(cell_html),
        label = html_text_escape(group),
    )
}

fn facet_page_html(title: &str, cols: usize, cell_w: i64, cells: &str) -> String {
    format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>{title}</title>\
<style>\
*{{box-sizing:border-box}}\
body{{margin:0;padding:16px;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;background:transparent}}\
.sp-facet-title{{font-size:15px;font-weight:600;margin:0 0 12px;color:inherit}}\
.sp-facet-grid{{display:grid;grid-template-columns:repeat({cols},minmax({cw}px,1fr));gap:14px}}\
.sp-facet-cell{{display:flex;flex-direction:column;align-items:center}}\
.sp-facet-label{{margin-top:4px;font-size:12px;font-weight:500;opacity:.75;text-align:center}}\
</style></head><body>\
{heading}\
<div class=\"sp-facet-grid\">{cells}</div>\
</body></html>",
        title = html_text_escape(title),
        cols = cols.max(1),
        cw = cell_w,
        heading = if title.is_empty() { String::new() } else { format!("<p class=\"sp-facet-title\">{}</p>", html_text_escape(title)) },
        cells = cells,
    )
}

pub use build as build_facet;

#[crate::sera_alias("facet", "facet_grid", "facetgrid", "small_multiples")]
#[crate::sera_builder("build_facet")]
pub fn build(input: &str) -> String {
    let sanitized = sanitize_non_finite_json(input);
    let root: Value = serde_json::from_str(&sanitized).unwrap_or(Value::Null);
    let obj = match root.as_object() {
        Some(o) => o,
        None => return String::new(),
    };
    let family = obj.get("family").and_then(Value::as_str).unwrap_or("histogram");
    let builder = match dispatch(family) {
        Some(b) => b,
        None => return String::new(),
    };
    let facet_key: Vec<String> = obj
        .get("facet_by")
        .and_then(Value::as_array)
        .map(|a| a.iter().map(value_to_key).collect())
        .unwrap_or_default();
    let n = facet_key.len();
    let title = obj.get("title").and_then(Value::as_str).unwrap_or("").to_string();
    let cols = obj.get("cols").and_then(Value::as_u64).unwrap_or(3) as usize;
    let cell_w = obj.get("cell_width").and_then(Value::as_i64).unwrap_or(320);
    let cell_h = obj.get("cell_height").and_then(Value::as_i64).unwrap_or(280);

    if n == 0 {
        return builder(input);
    }

    let mut groups: Vec<String> = Vec::new();
    for k in &facet_key {
        if !groups.contains(k) {
            groups.push(k.clone());
        }
    }

    let mut cells = String::new();
    for g in &groups {
        let cell = cell_input(obj, &facet_key, n, g, cell_w, cell_h, &title);
        let cell_html = builder(&cell);
        cells.push_str(&facet_cell_html(g, cell_w, cell_h, &cell_html));
    }
    facet_page_html(&title, cols, cell_w, &cells)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_by_facet_key_and_dispatches_to_parent_family() {
        let mut values = Vec::new();
        let mut facet_by = Vec::new();
        for i in 0..400 {
            let bucket = i % 4;
            let mean: f64 = [20.0, 12.0, 2.0, 5.0][bucket];
            let label = ["sun", "wind", "drizzle", "rain"][bucket];
            values.push(mean + ((i as f64) * 0.037).sin() * mean.max(1.0) * 0.6);
            facet_by.push(label.to_string());
        }
        let input = serde_json::json!({
            "family": "histogram",
            "title": "weather",
            "values": values,
            "facet_by": facet_by,
            "cols": 2,
        })
        .to_string();
        let html = build(&input);
        assert!(html.contains("sp-facet-grid"));
        assert_eq!(html.matches("class=\"sp-facet-cell\"").count(), 4);
        assert!(html.contains("weather \u{2014} sun"));
    }

    #[test]
    fn returns_plain_chart_when_no_facet_key_present() {
        let input = serde_json::json!({
            "family": "histogram",
            "values": [1.0, 2.0, 3.0, 4.0],
        })
        .to_string();
        let html = build(&input);
        assert!(!html.contains("sp-facet-grid"));
        assert!(!html.is_empty());
    }

    #[test]
    fn unknown_family_returns_empty() {
        let input = serde_json::json!({
            "family": "not_a_real_family",
            "values": [1.0, 2.0],
            "facet_by": ["a", "b"],
        })
        .to_string();
        assert!(build(&input).is_empty());
    }
}
