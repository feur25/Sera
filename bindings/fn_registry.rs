#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputKind {
    Json,
    ChartHtml,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutputKind {
    Html,
    Json,
    Bool,
}

pub struct FnEntry {
    pub name: &'static str,
    pub input: InputKind,
    pub output: OutputKind,
    pub invoke: fn(&str) -> String,
}

inventory::collect!(FnEntry);

pub fn iter_entries() -> impl Iterator<Item = &'static FnEntry> {
    inventory::iter::<FnEntry>()
}

static INDEX: std::sync::OnceLock<std::collections::HashMap<&'static str, &'static FnEntry>> =
    std::sync::OnceLock::new();

fn build_index() -> std::collections::HashMap<&'static str, &'static FnEntry> {
    let mut map: std::collections::HashMap<&'static str, &'static FnEntry> =
        iter_entries().map(|e| (e.name, e)).collect();
    for doc in crate::doc_registry::all_docs() {
        if let Some(entry) = map.get(doc.name).copied() {
            for alias in doc.aliases {
                map.entry(alias).or_insert(entry);
            }
        }
    }
    map
}

pub fn find(name: &str) -> Option<&'static FnEntry> {
    let idx = INDEX.get_or_init(build_index);
    if let Some(entry) = idx.get(name) {
        return Some(*entry);
    }
    let snake = crate::bindings::name_norm::to_snake_case(name);
    if snake != name {
        idx.get(snake.as_str()).copied()
    } else {
        None
    }
}

fn method_args_for(key: &str, value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Object(_) => Some(value.to_string()),
        serde_json::Value::Bool(on) => {
            if *on {
                Some("{}".to_string())
            } else {
                None
            }
        }
        other => {
            let param_name = crate::doc_registry::doc_for(key)
                .and_then(|d| d.params.first())
                .map(|p| p.name)
                .unwrap_or(key);
            Some(serde_json::json!({ param_name: other }).to_string())
        }
    }
}

fn json_f64_array(obj: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<Vec<f64>> {
    obj.get(key)?.as_array().map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
}

fn json_str_array(obj: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<Vec<String>> {
    obj.get(key)?.as_array().map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
}

fn json_i32(obj: &serde_json::Map<String, serde_json::Value>, key: &str, default: i32) -> i32 {
    obj.get(key).and_then(|v| v.as_i64()).map(|n| n as i32).unwrap_or(default)
}

fn json_u32(obj: &serde_json::Map<String, serde_json::Value>, key: &str, default: u32) -> u32 {
    obj.get(key).and_then(|v| v.as_u64()).map(|n| n as u32).unwrap_or(default)
}

fn json_bool(obj: &serde_json::Map<String, serde_json::Value>, key: &str, default: bool) -> bool {
    obj.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
}

fn json_string(obj: &serde_json::Map<String, serde_json::Value>, key: &str, default: &str) -> String {
    obj.get(key).and_then(|v| v.as_str()).map(str::to_string).unwrap_or_else(|| default.to_string())
}

struct RawNativeOpts {
    width: i32,
    height: i32,
    color_hex: u32,
    gridlines: bool,
    x_label: String,
    y_label: String,
    show_regression: bool,
    regression_type: String,
    cols: i32,
    categories: Vec<String>,
    variant: String,
}

impl RawNativeOpts {
    fn as_opts(&self) -> crate::plot::canvas_points::NativeChartOpts<'_> {
        crate::plot::canvas_points::NativeChartOpts {
            width: self.width,
            height: self.height,
            color_hex: self.color_hex,
            gridlines: self.gridlines,
            x_label: &self.x_label,
            y_label: &self.y_label,
            show_regression: self.show_regression,
            regression_type: &self.regression_type,
            cols: self.cols,
            categories: &self.categories,
            variant: &self.variant,
        }
    }
}

fn raw_native_opts(obj: &serde_json::Map<String, serde_json::Value>, default_w: i32, default_h: i32) -> RawNativeOpts {
    RawNativeOpts {
        width: json_i32(obj, "width", default_w),
        height: json_i32(obj, "height", default_h),
        color_hex: json_u32(obj, "color_hex", 0),
        gridlines: json_bool(obj, "gridlines", true),
        x_label: json_string(obj, "x_label", ""),
        y_label: json_string(obj, "y_label", ""),
        show_regression: json_bool(obj, "show_regression", false),
        regression_type: json_string(obj, "regression_type", "linear"),
        cols: obj.get("col_labels").and_then(|v| v.as_array()).map(|a| a.len() as i32).unwrap_or(0),
        categories: json_str_array(obj, "categories").unwrap_or_default(),
        variant: json_string(obj, "variant", ""),
    }
}

#[allow(unused_variables)]
fn try_native_fast_path(target: &str, obj: &serde_json::Map<String, serde_json::Value>) -> Option<String> {
    for entry in inventory::iter::<crate::plot::canvas_points::NativeChartEntry>() {
        if entry.name != target {
            continue;
        }
        let y = json_f64_array(obj, "values").or_else(|| json_f64_array(obj, "y"))?;
        if y.len() <= entry.threshold {
            return None;
        }
        let x = json_f64_array(obj, "x").unwrap_or_else(|| (0..y.len()).map(|i| i as f64).collect());
        let raw = raw_native_opts(obj, 900, 500);
        let opts = raw.as_opts();
        let (html, hid) = (entry.render)(&json_string(obj, "title", ""), &x, &y, &opts);
        #[cfg(feature = "sera-pulse")]
        {
            let meta = crate::plot::push_registry::PushMeta::from_xy(&x, &y, opts.width, opts.height);
            crate::plot::push_registry::register(hid, meta);
            crate::plot::chart_source_registry::register(hid, entry.name, serde_json::Value::Object(obj.clone()).to_string());
        }
        let _ = hid;
        return Some(html);
    }
    for entry in inventory::iter::<crate::plot::canvas_points::LabeledChartEntry>() {
        if entry.name != target {
            continue;
        }
        let values = json_f64_array(obj, "values")?;
        if values.len() <= entry.threshold {
            return None;
        }
        let labels = json_str_array(obj, "labels")?;
        let raw = raw_native_opts(obj, 900, 500);
        let opts = raw.as_opts();
        let shape_ok = if opts.cols > 0 { (labels.len() as i64) * (opts.cols as i64) >= values.len() as i64 } else { labels.len() >= values.len() };
        if !shape_ok {
            return None;
        }
        let (html, hid) = (entry.render)(&json_string(obj, "title", ""), &labels, &values, &opts);
        #[cfg(feature = "sera-pulse")]
        {
            let axis_px = if opts.cols > 0 { 1000 } else { crate::plot::default::bar::bar_plot_h(opts.height) };
            let meta = crate::plot::push_registry::PushMeta::from_values(&values, axis_px);
            crate::plot::push_registry::register(hid, meta);
            crate::plot::chart_source_registry::register(hid, entry.name, serde_json::Value::Object(obj.clone()).to_string());
        }
        let _ = hid;
        return Some(html);
    }
    for entry in inventory::iter::<crate::plot::canvas_points::OhlcChartEntry>() {
        if entry.name != target {
            continue;
        }
        let open = json_f64_array(obj, "open")?;
        let high = json_f64_array(obj, "high")?;
        let low = json_f64_array(obj, "low")?;
        let close = json_f64_array(obj, "close")?;
        if close.len() <= entry.threshold {
            return None;
        }
        let labels = json_str_array(obj, "labels").unwrap_or_default();
        let raw = raw_native_opts(obj, 1100, 500);
        let opts = raw.as_opts();
        let (html, hid) = (entry.render)(&json_string(obj, "title", ""), &labels, &open, &high, &low, &close, &opts);
        #[cfg(feature = "sera-pulse")]
        {
            let plot_h = (opts.height - 36 - 48).max(10);
            let meta = crate::plot::push_registry::PushMeta::from_vector_shared_scale(&[&open, &high, &low, &close], plot_h);
            crate::plot::push_registry::register(hid, meta);
            crate::plot::chart_source_registry::register(hid, entry.name, serde_json::Value::Object(obj.clone()).to_string());
        }
        let _ = hid;
        return Some(html);
    }
    None
}

pub fn invoke(entry: &FnEntry, json: &str) -> String {
    let parsed: serde_json::Value = match serde_json::from_str(json) {
        Ok(v) => v,
        Err(_) => return (entry.invoke)(json),
    };
    let obj = match parsed.as_object() {
        Some(o) => o,
        None => return (entry.invoke)(json),
    };
    if entry.input == InputKind::Json && entry.output == OutputKind::Html {
        if let Some(native_html) = try_native_fast_path(entry.name, obj) {
            return native_html;
        }
    }
    let html = (entry.invoke)(json);
    if entry.input != InputKind::Json || entry.output != OutputKind::Html {
        return html;
    }
    let mut out = html;
    for (key, value) in obj {
        if crate::bindings::method_registry::find(key).is_none() {
            continue;
        }
        let args_json = match method_args_for(key, value) {
            Some(a) => a,
            None => continue,
        };
        if let Some(applied) = crate::bindings::method_registry::apply_by_name(&out, key, &args_json) {
            out = applied;
        }
    }
    out
}

#[cfg(test)]
mod native_fast_path_tests {
    use super::{find, invoke};

    #[test]
    fn a_small_bar_call_stays_on_the_plain_svg_builder() {
        let entry = find("build_bar").expect("build_bar is registered");
        let labels: Vec<String> = (0..5).map(|i| i.to_string()).collect();
        let values: Vec<f64> = (0..5).map(|i| i as f64).collect();
        let json = serde_json::json!({ "title": "t", "labels": labels, "values": values }).to_string();
        let html = invoke(entry, &json);
        assert!(html.contains("<svg"));
        assert!(!html.contains("webgl2"));
    }

    #[test]
    fn a_large_bar_call_switches_to_the_native_canvas_renderer_with_a_push_capable_marker() {
        let entry = find("build_bar").expect("build_bar is registered");
        let n = 600;
        let labels: Vec<String> = (0..n).map(|i| i.to_string()).collect();
        let values: Vec<f64> = (0..n).map(|i| (i % 30) as f64).collect();
        let json = serde_json::json!({ "title": "t", "labels": labels, "values": values }).to_string();
        let html = invoke(entry, &json);
        assert!(html.contains("id=\"spbarsvg"));
        assert!(html.contains("sp_apply_"));
    }

    #[test]
    fn a_large_scatter_call_switches_to_the_native_canvas_renderer() {
        let entry = find("build_scatter_chart").expect("build_scatter_chart is registered");
        let n = 3200;
        let x: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let y: Vec<f64> = (0..n).map(|i| (i as f64) * 1.1).collect();
        let json = serde_json::json!({ "title": "t", "x": x, "y": y }).to_string();
        let html = invoke(entry, &json);
        assert!(html.contains("sp_apply_"));
    }

    #[test]
    fn a_large_line_call_switches_to_the_native_canvas_renderer() {
        let entry = find("build_line").expect("build_line is registered");
        let n = 3200;
        let x: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let y: Vec<f64> = (0..n).map(|i| (i as f64).sin()).collect();
        let json = serde_json::json!({ "title": "t", "x": x, "y": y }).to_string();
        let html = invoke(entry, &json);
        assert!(html.contains("sp_apply_"));
    }

    #[test]
    fn a_large_candlestick_call_switches_to_the_native_renderer() {
        let entry = find("build_candlestick").expect("build_candlestick is registered");
        let n = 600;
        let o: Vec<f64> = (0..n).map(|i| 100.0 + i as f64 * 0.1).collect();
        let h: Vec<f64> = o.iter().map(|v| v + 1.0).collect();
        let l: Vec<f64> = o.iter().map(|v| v - 1.0).collect();
        let c: Vec<f64> = o.iter().map(|v| v + 0.5).collect();
        let labels: Vec<String> = (0..n).map(|i| i.to_string()).collect();
        let json = serde_json::json!({ "title": "t", "labels": labels, "open": o, "high": h, "low": l, "close": c }).to_string();
        let html = invoke(entry, &json);
        assert!(html.contains("sp_apply_"));
    }

    #[test]
    fn an_unrelated_function_name_never_takes_the_native_fast_path_even_with_a_big_values_array() {
        let entry = find("build_heatmap3d_chart").expect("build_heatmap3d_chart is registered");
        let n = 5000;
        let labels: Vec<String> = (0..n).map(|i| i.to_string()).collect();
        let values: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let json = serde_json::json!({ "title": "t", "labels": labels, "values": values }).to_string();
        let html = invoke(entry, &json);
        assert!(!html.contains("sp_apply_"));
    }
}

#[cfg(test)]
mod alias_dispatch_tests {
    use super::find;

    #[test]
    fn every_declared_fn_alias_resolves_to_its_canonical_entry() {
        let mut checked = 0;
        for doc in crate::doc_registry::all_docs() {
            if doc.aliases.is_empty() {
                continue;
            }
            let canonical = match find(doc.name) {
                Some(e) => e,
                None => continue,
            };
            for alias in doc.aliases {
                let via_alias = find(alias).unwrap_or_else(|| {
                    panic!("alias '{alias}' of fn '{}' does not resolve -- declared in #[sera_doc(aliases(...))] but never dispatchable", doc.name)
                });
                assert!(
                    std::ptr::eq(canonical, via_alias),
                    "alias '{alias}' resolved to a different FnEntry than '{}' itself",
                    doc.name
                );
                checked += 1;
            }
        }
        assert!(checked > 0, "no fn_registry aliases were found to check -- doc_registry wiring may be broken");
    }

    #[test]
    fn pascal_case_fn_names_resolve_the_same_entry_as_their_snake_case_form() {
        let snake = find("build_bar").expect("build_bar is a registered fn");
        let pascal = find("BuildBar").expect("'BuildBar' (PascalCase, as a C# caller would naturally write it) must resolve");
        assert!(std::ptr::eq(snake, pascal));
    }

    #[test]
    fn a_bool_kwarg_matching_a_chart_method_name_applies_that_method() {
        let entry = find("build_bar").expect("build_bar is a registered fn");
        let plain = super::invoke(entry, r#"{"title":"T","labels":["a","b"],"values":[1,2]}"#);
        let with_grid = super::invoke(entry, r#"{"title":"T","labels":["a","b"],"values":[1,2],"grid":true}"#);
        assert_ne!(plain, with_grid, "a 'grid: true' kwarg alongside chart data must apply the grid method during construction");
        assert!(with_grid.contains(".sp-gl{display:block"));
    }

    #[test]
    fn a_false_bool_kwarg_matching_a_chart_method_name_is_a_no_op() {
        let entry = find("build_bar").expect("build_bar is a registered fn");
        let with_grid_off = super::invoke(entry, r#"{"title":"T","labels":["a","b"],"values":[1,2],"grid":false}"#);
        assert!(!with_grid_off.contains(".sp-gl{display:block"));
    }

    #[test]
    fn an_object_kwarg_matching_a_chart_method_name_forwards_its_fields_as_that_methods_args() {
        let entry = find("build_bar").expect("build_bar is a registered fn");
        let with_bg = super::invoke(entry, r##"{"title":"T","labels":["a","b"],"values":[1,2],"set_bg":{"color":"#111111"}}"##);
        assert!(with_bg.contains("#111111"));
    }

    #[test]
    fn ordinary_data_kwargs_are_left_alone_when_they_do_not_match_any_method_name() {
        let entry = find("build_bar").expect("build_bar is a registered fn");
        let html = super::invoke(entry, r#"{"title":"T","labels":["a","b"],"values":[1,2]}"#);
        assert!(html.contains("<svg"));
    }
}