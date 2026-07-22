use super::{cell_input, dispatch, facet_cell_html, facet_page_html, value_to_key};
use crate::plot::chart_input::sanitize_non_finite_json;
use serde_json::Value;

pub use build as build_facet;

#[crate::chart_demo(
    "family=\"histogram\", title=\"Precipitation\", cols=2, \
values=[0.2,1.1,0.5,2.3,0.8,3.1,1.4,0.3,2.0,1.7,0.6,4.2,1.9,0.4,2.8,1.2,0.9,3.6,1.5,0.7,5.1,2.4,0.3,3.9,1.8,0.6,2.6,1.1,4.4,2.0], \
facet_by=[\"sun\",\"sun\",\"sun\",\"sun\",\"sun\",\"sun\",\"sun\",\"rain\",\"rain\",\"rain\",\"rain\",\"rain\",\"rain\",\"rain\",\"drizzle\",\"drizzle\",\"drizzle\",\"drizzle\",\"drizzle\",\"drizzle\",\"drizzle\",\"fog\",\"fog\",\"fog\",\"fog\",\"fog\",\"fog\",\"fog\",\"fog\",\"fog\"]"
)]
#[crate::params(paramsList["family", "variant", "facet_by", "title", "cols", "cell_width", "cell_height"])]
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
