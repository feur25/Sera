use crate::plot::statistical::chart_registry::dispatch;
use crate::plot::statistical::common::escape_xml_s;
use serde_json::{Map, Value};

pub fn axis_values(obj: &Map<String, Value>, keys: &[&str]) -> Vec<f64> {
    for k in keys {
        if let Some(arr) = obj.get(*k).and_then(Value::as_array) {
            return arr.iter().filter_map(Value::as_f64).collect();
        }
    }
    Vec::new()
}

#[allow(clippy::too_many_arguments)]
pub fn build_region(
    family: &str,
    variant: &str,
    values: &[f64],
    width: i64,
    height: i64,
    colorscale: &str,
    palette: &Value,
    bins: i64,
    color_hex: &Value,
) -> String {
    let Some(builder) = dispatch(family) else {
        return String::new();
    };
    let mut o = Map::new();
    o.insert("title".to_string(), Value::String(String::new()));
    o.insert("values".to_string(), Value::from(values.to_vec()));
    o.insert("x".to_string(), Value::from(values.to_vec()));
    let labels: Vec<String> = (1..=values.len()).map(|i| i.to_string()).collect();
    o.insert("labels".to_string(), Value::from(labels));
    o.insert("width".to_string(), Value::from(width));
    o.insert("height".to_string(), Value::from(height));
    o.insert("colorscale".to_string(), Value::String(colorscale.to_string()));
    o.insert("palette".to_string(), palette.clone());
    o.insert("bins".to_string(), Value::from(bins));
    o.insert("color_hex".to_string(), color_hex.clone());
    if !variant.is_empty() {
        o.insert("variant".to_string(), Value::String(variant.to_string()));
    }
    builder(&Value::Object(o).to_string())
}

pub fn cell_iframe(width: i64, height: i64, html: &str) -> String {
    format!(
        "<iframe style=\"width:{w}px;height:{h}px;border:0;background:transparent;display:block\" \
loading=\"lazy\" srcdoc=\"{doc}\"></iframe>",
        w = width,
        h = height,
        doc = escape_xml_s(html),
    )
}

pub fn cell_iframe_rotated(target_w: i64, target_h: i64, html: &str) -> String {
    const COUNTER_ROTATE_CSS: &str =
        "<style>svg text{transform-box:fill-box;transform-origin:center;transform:rotate(90deg) !important}</style></head>";
    let rotated_html = html.replacen("</head>", COUNTER_ROTATE_CSS, 1);
    format!(
        "<div style=\"width:{tw}px;height:{th}px;overflow:hidden;position:relative\">\
<iframe style=\"width:{sw}px;height:{sh}px;border:0;background:transparent;display:block;\
position:absolute;top:50%;left:50%;transform:translate(-50%,-50%) rotate(-90deg)\" \
loading=\"lazy\" srcdoc=\"{doc}\"></iframe></div>",
        tw = target_w,
        th = target_h,
        sw = target_h,
        sh = target_w,
        doc = escape_xml_s(&rotated_html),
    )
}

#[allow(clippy::too_many_arguments)]
pub fn compose_page(title: &str, panel_w: i64, panel_h: i64, margin: i64, top: &str, right: &str, panel: &str) -> String {
    let heading = if title.is_empty() {
        String::new()
    } else {
        format!("<p class=\"sp-joint-title\">{}</p>", escape_xml_s(title))
    };
    let total_w = panel_w + 6 + margin + 32;
    let total_h = margin + 6 + panel_h + 32 + if title.is_empty() { 0 } else { 27 };
    format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>{title}</title>\
<meta name=\"sp-content-size\" content=\"{total_w}x{total_h}\">\
<style>\
*{{box-sizing:border-box}}\
body{{margin:0;padding:16px;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;background:transparent}}\
.sp-joint-title{{font-size:15px;font-weight:600;margin:0 0 12px;color:inherit}}\
.sp-joint-grid{{display:grid;grid-template-columns:{pw}px {mg}px;grid-template-rows:{mg}px {ph}px;gap:6px}}\
.sp-joint-top{{grid-column:1;grid-row:1;overflow:hidden}}\
.sp-joint-corner{{grid-column:2;grid-row:1}}\
.sp-joint-panel{{grid-column:1;grid-row:2;overflow:hidden}}\
.sp-joint-right{{grid-column:2;grid-row:2;overflow:hidden}}\
</style></head><body>\
{heading}\
<div class=\"sp-joint-grid\">\
<div class=\"sp-joint-top\">{top}</div>\
<div class=\"sp-joint-corner\"></div>\
<div class=\"sp-joint-panel\">{panel}</div>\
<div class=\"sp-joint-right\">{right}</div>\
</div>\
</body></html>",
        title = escape_xml_s(title),
        pw = panel_w,
        ph = panel_h,
        mg = margin,
        heading = heading,
        top = top,
        right = right,
        panel = panel,
    )
}
