use serde_json::{Map, Value};

pub(super) use super::chart_registry::dispatch;

pub(super) fn value_to_key(v: &Value) -> String {
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

pub(super) fn cell_input(obj: &Map<String, Value>, facet_key: &[String], n: usize, group: &str, cell_w: i64, cell_h: i64, title: &str) -> String {
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

pub(super) fn facet_cell_html(group: &str, cell_w: i64, cell_h: i64, cell_html: &str) -> String {
    format!(
        "<div class=\"sp-facet-cell\" style=\"width:{cw}px\">\
<iframe class=\"sp-facet-frame\" style=\"width:{cw}px;height:{ch}px;border:0;background:transparent\" \
loading=\"lazy\" srcdoc=\"{doc}\"></iframe>\
<div class=\"sp-facet-label\">{label}</div></div>",
        cw = cell_w,
        ch = cell_h,
        doc = html_attr_escape(cell_html),
        label = html_text_escape(group),
    )
}

pub(super) fn facet_page_html(title: &str, cols: usize, cell_w: i64, cell_h: i64, n_groups: usize, cells: &str) -> String {
    let cols = cols.max(1);
    let rows = ((n_groups + cols - 1) / cols).max(1);
    let total_w = cols as i64 * cell_w + (cols as i64 - 1).max(0) * 14 + 32;
    let total_h = rows as i64 * (cell_h + 22) + (rows as i64 - 1).max(0) * 14 + 32 + if title.is_empty() { 0 } else { 27 };
    format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>{title}</title>\
<meta name=\"sp-content-size\" content=\"{total_w}x{total_h}\">\
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
        cw = cell_w,
        heading = if title.is_empty() { String::new() } else { format!("<p class=\"sp-facet-title\">{}</p>", html_text_escape(title)) },
        cells = cells,
    )
}

pub mod basic;
pub use basic::build_facet;

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
        let html = build_facet(&input);
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
        let html = build_facet(&input);
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
        assert!(build_facet(&input).is_empty());
    }
}
