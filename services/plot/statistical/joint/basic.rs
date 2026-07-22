use super::common::{axis_values, build_region, cell_iframe, compose_page};
use super::variant::resolve_legacy_panel;
use crate::plot::chart_input::sanitize_non_finite_json;
use crate::plot::statistical::chart_registry::dispatch;
use serde_json::Value;

pub use build as build_joint;

#[crate::chart_demo(
    "x=[1.2,2.4,2.1,3.6,3.1,3.9,4.2,4.6,4.4,4.9,5.5,5.1,5.8,2.2,3.3,3.7,4.1,1.8,2.6,3.4,4.3,5.2,3.2,3.8], \
y=[1.1,2.3,3.2,2.4,3.6,4.1,3.3,4.7,5.2,3.9,4.4,5.6,6.1,1.4,2.5,4.2,4.6,2.1,3.1,3.3,5.1,4.5,3.4,5.3], \
variant=\"hexbin\", marginal=\"histogram\""
)]
#[crate::params(paramsList[
    "variant", "panel_variant", "marginal", "marginal_variant", "x", "y", "bins", "colorscale",
    "color_hex", "palette", "width", "height"
])]
#[crate::sera_alias("joint", "jointplot", "joint_plot", "bivariate")]
#[crate::sera_builder("build_joint")]
pub fn build(input: &str) -> String {
    let sanitized = sanitize_non_finite_json(input);
    let root: Value = serde_json::from_str(&sanitized).unwrap_or(Value::Null);
    let obj = match root.as_object() {
        Some(o) => o.clone(),
        None => return String::new(),
    };

    let title = obj.get("title").and_then(Value::as_str).unwrap_or("").to_string();
    let requested = obj.get("variant").and_then(Value::as_str).unwrap_or("hexbin");
    let panel_family = resolve_legacy_panel(requested).unwrap_or(requested).to_string();
    let panel_variant = obj.get("panel_variant").and_then(Value::as_str).unwrap_or("").to_string();
    let marginal_family = obj.get("marginal").and_then(Value::as_str).unwrap_or("histogram").to_string();
    let marginal_variant = obj.get("marginal_variant").and_then(Value::as_str).unwrap_or("").to_string();
    let width = obj.get("width").and_then(Value::as_i64).unwrap_or(760);
    let height = obj.get("height").and_then(Value::as_i64).unwrap_or(760);
    let colorscale = obj.get("colorscale").and_then(Value::as_str).unwrap_or("").to_string();
    let bins = obj.get("bins").and_then(Value::as_i64).unwrap_or(24);
    let palette = obj.get("palette").cloned().unwrap_or(Value::Array(vec![]));
    let color_hex = obj.get("color_hex").cloned().unwrap_or(Value::Null);

    let margin: i64 = 130;
    let gap: i64 = 6;
    let panel_w = (width - margin - gap).max(200);
    let panel_h = (height - margin - gap).max(200);

    let Some(panel_builder) = dispatch(&panel_family) else {
        return String::new();
    };

    let mut panel_obj = obj.clone();
    panel_obj.remove("marginal");
    panel_obj.remove("marginal_variant");
    panel_obj.remove("panel_variant");
    if panel_variant.is_empty() {
        panel_obj.remove("variant");
    } else {
        panel_obj.insert("variant".to_string(), Value::String(panel_variant));
    }
    if !panel_obj.contains_key("values") {
        let x_values = axis_values(&obj, &["x", "x_values"]);
        if !x_values.is_empty() {
            panel_obj.insert("values".to_string(), Value::from(x_values));
        }
    }
    panel_obj.insert("width".to_string(), Value::from(panel_w));
    panel_obj.insert("height".to_string(), Value::from(panel_h));
    panel_obj.insert("title".to_string(), Value::String(String::new()));
    let panel_html = panel_builder(&Value::Object(panel_obj).to_string());
    let panel_frame = cell_iframe(panel_w, panel_h, &panel_html);

    let x_values = axis_values(&obj, &["x", "x_values"]);
    let y_values = axis_values(&obj, &["y", "y_values"]);

    let disabled = marginal_family.is_empty() || marginal_family == "none";
    let (top_frame, right_frame) = if disabled {
        (String::new(), String::new())
    } else {
        let top_html = build_region(
            &marginal_family, &marginal_variant, &x_values, panel_w, margin, &colorscale, &palette, bins, &color_hex, false,
        );
        let right_html = build_region(
            &marginal_family, &marginal_variant, &y_values, margin, panel_h, &colorscale, &palette, bins, &color_hex, true,
        );
        (cell_iframe(panel_w, margin, &top_html), cell_iframe(margin, panel_h, &right_html))
    };

    compose_page(&title, panel_w, panel_h, margin, &top_frame, &right_frame, &panel_frame)
}
