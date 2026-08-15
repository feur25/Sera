pub struct ChartDemoEntry {
    pub file: &'static str,
    pub fn_name: &'static str,
    pub kwargs: &'static str,
    pub media: &'static str,
}
inventory::collect!(ChartDemoEntry);

pub fn iter_entries() -> impl Iterator<Item = &'static ChartDemoEntry> {
    inventory::iter::<ChartDemoEntry>()
}

pub fn family_variant(file: &str) -> Option<(String, String)> {
    let comps: Vec<&str> = file.split(['/', '\\']).filter(|s| !s.is_empty()).collect();
    if comps.len() < 2 {
        return None;
    }
    let last = *comps.last()?;
    let variant = last.strip_suffix(".rs")?.to_string();
    let parent = comps[comps.len() - 2];
    if parent == "_3d" || parent == "statistical" {
        return Some((variant, "basic".to_string()));
    }
    Some((parent.to_string(), variant))
}

const SKIP_VARIANTS: &[&str] = &["mod", "common", "config", "shared", "variant"];

fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut in_str = false;
    let mut start = 0usize;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            '"' => in_str = !in_str,
            '[' | '{' | '(' if !in_str => depth += 1,
            ']' | '}' | ')' if !in_str => depth -= 1,
            c2 if c2 == sep && !in_str && depth == 0 => {
                out.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
        i += 1;
    }
    out.push(&s[start..]);
    out
}

fn find_top_level_eq(s: &str) -> Option<usize> {
    let mut depth = 0i32;
    let mut in_str = false;
    for (i, c) in s.char_indices() {
        match c {
            '"' => in_str = !in_str,
            '[' | '{' | '(' if !in_str => depth += 1,
            ']' | '}' | ')' if !in_str => depth -= 1,
            '=' if !in_str && depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

fn python_literals_to_json(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut in_str = false;
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '"' {
            in_str = !in_str;
            out.push(c);
            i += 1;
            continue;
        }
        if !in_str {
            let rest = &value[i..];
            let is_word_start = i == 0 || !(bytes[i - 1] as char).is_ascii_alphanumeric();
            let matched = [("True", "true"), ("False", "false"), ("None", "null")]
                .into_iter()
                .find(|(py, _)| {
                    rest.starts_with(py)
                        && is_word_start
                        && rest[py.len()..]
                            .chars()
                            .next()
                            .map(|c2| !c2.is_ascii_alphanumeric() && c2 != '_')
                            .unwrap_or(true)
                });
            if let Some((py, json)) = matched {
                out.push_str(json);
                i += py.len();
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

pub fn kwargs_to_json(kwargs: &str) -> String {
    let mut out = String::from("{");
    let mut first = true;
    for pair in split_top_level(kwargs, ',') {
        let pair = pair.trim();
        if pair.is_empty() {
            continue;
        }
        let Some(eq) = find_top_level_eq(pair) else {
            continue;
        };
        let key = pair[..eq].trim();
        let value = pair[eq + 1..].trim();
        if key.is_empty() || value.is_empty() {
            continue;
        }
        if !first {
            out.push(',');
        }
        first = false;
        out.push('"');
        out.push_str(key);
        out.push_str("\":");
        out.push_str(&python_literals_to_json(value));
    }
    out.push('}');
    out
}

fn inject_variant(json: &str, variant: &str) -> String {
    let body = json
        .trim()
        .strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
        .unwrap_or("")
        .trim();
    if body.is_empty() {
        format!("{{\"variant\":\"{variant}\"}}")
    } else {
        format!("{{\"variant\":\"{variant}\",{body}}}")
    }
}

pub struct DemoPayload {
    pub builder: String,
    pub family: String,
    pub variant: String,
    pub json: String,
}

fn resolve_builder(entry: &ChartDemoEntry) -> (String, String, String) {
    if entry.fn_name.starts_with("build_") {
        let resolved_fn = crate::bindings::alias_registry::resolve_call_target(entry.fn_name);
        let is_direct_builder = crate::bindings::fn_registry::iter_entries()
            .any(|e| e.name == resolved_fn);
        if is_direct_builder {
            return (resolved_fn, entry.fn_name.to_string(), entry.fn_name.to_string());
        }
    }
    if let Some((family, variant)) = family_variant(entry.file) {
        let builder = crate::bindings::alias_registry::resolve_call_target(&family);
        return (builder, family, variant);
    }
    let resolved_fn = crate::bindings::alias_registry::resolve_call_target(entry.fn_name);
    (resolved_fn, entry.fn_name.to_string(), entry.fn_name.to_string())
}

pub fn demo_payload(entry: &ChartDemoEntry) -> Option<DemoPayload> {
    let (builder, family, variant) = resolve_builder(entry);
    if SKIP_VARIANTS.contains(&variant.as_str()) {
        return None;
    }
    let base = kwargs_to_json(entry.kwargs);
    let json = if entry.kwargs.contains("variant") {
        base
    } else {
        inject_variant(&base, &variant)
    };
    Some(DemoPayload {
        builder,
        family,
        variant,
        json,
    })
}

pub fn apply_media(html: String, media_json: &str) -> String {
    let trimmed = media_json.trim();
    if trimmed.is_empty() {
        return html;
    }
    let Ok(items) = serde_json::from_str::<Vec<serde_json::Value>>(trimmed) else {
        return html;
    };
    let mut out = html;
    for item in &items {
        let kind = item.get("kind").and_then(|v| v.as_str()).unwrap_or("image");
        let Some(src) = item.get("src").and_then(|v| v.as_str()) else {
            continue;
        };
        let x = item.get("x").and_then(|v| v.as_f64()).unwrap_or(0.5);
        let y = item.get("y").and_then(|v| v.as_f64()).unwrap_or(0.5);
        let w = item.get("w").and_then(|v| v.as_f64()).unwrap_or(0.2);
        let h = item.get("h").and_then(|v| v.as_f64()).unwrap_or(0.2);
        let shape = item.get("shape").and_then(|v| v.as_str()).unwrap_or("rect");
        let opacity = item.get("opacity").and_then(|v| v.as_f64()).unwrap_or(1.0);
        out = crate::html::media_overlay::add_media(&out, kind, src, x, y, w, h, shape, opacity);
    }
    out
}

pub fn render_demo_html(entry: &ChartDemoEntry) -> Option<String> {
    let payload = demo_payload(entry)?;
    let fe = crate::bindings::fn_registry::iter_entries().find(|f| f.name == payload.builder)?;
    let html = (fe.invoke)(&payload.json);
    Some(apply_media(html, entry.media))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_variant_reads_family_from_the_parent_directory_and_variant_from_the_file_stem() {
        let (family, variant) =
            family_variant("src/services/plot/statistical/bar/pictogram.rs").unwrap();
        assert_eq!(family, "bar");
        assert_eq!(variant, "pictogram");
    }

    #[test]
    fn family_variant_defaults_the_variant_to_basic_for_a_flat_file_directly_under_statistical() {
        let (family, variant) =
            family_variant("src/services/plot/statistical/histogram.rs").unwrap();
        assert_eq!(family, "histogram");
        assert_eq!(variant, "basic");
    }

    #[test]
    fn family_variant_handles_backslash_paths_the_same_as_forward_slash_paths() {
        let (family, variant) =
            family_variant("src\\services\\plot\\statistical\\boxplot\\violin.rs").unwrap();
        assert_eq!(family, "boxplot");
        assert_eq!(variant, "violin");
    }

    #[test]
    fn kwargs_to_json_converts_simple_key_value_pairs_into_a_json_object() {
        let json = kwargs_to_json(r#"x_labels=["Jan","Feb"], values=[12,19]"#);
        assert_eq!(json, r#"{"x_labels":["Jan","Feb"],"values":[12,19]}"#);
    }

    #[test]
    fn kwargs_to_json_does_not_split_on_commas_inside_nested_arrays_or_strings() {
        let json = kwargs_to_json(r#"labels=["A, B", "C"], sizes=[1,2,3]"#);
        assert_eq!(json, r#"{"labels":["A, B", "C"],"sizes":[1,2,3]}"#);
    }

    #[test]
    fn kwargs_to_json_handles_an_empty_kwargs_string() {
        assert_eq!(kwargs_to_json(""), "{}");
    }

    #[test]
    fn kwargs_to_json_normalizes_python_boolean_and_none_literals_to_their_json_equivalents() {
        let json = kwargs_to_json("show_points=True, show_grid=False, label=None");
        assert_eq!(
            json,
            r#"{"show_points":true,"show_grid":false,"label":null}"#
        );
    }

    #[test]
    fn kwargs_to_json_does_not_touch_the_words_true_false_none_when_they_appear_inside_a_string() {
        let json = kwargs_to_json(r#"note="True story, not False""#);
        assert_eq!(json, r#"{"note":"True story, not False"}"#);
    }

    #[test]
    fn kwargs_to_json_does_not_mangle_a_value_where_true_is_only_a_substring_of_a_larger_word() {
        let json = kwargs_to_json("mode=NotTrueYet");
        assert_eq!(json, r#"{"mode":NotTrueYet}"#);
    }

    #[test]
    fn inject_variant_adds_the_key_to_a_non_empty_object() {
        assert_eq!(
            inject_variant(r#"{"values":[1,2]}"#, "grouped"),
            r#"{"variant":"grouped","values":[1,2]}"#
        );
    }

    #[test]
    fn inject_variant_adds_the_key_to_an_empty_object_without_a_trailing_comma() {
        assert_eq!(inject_variant("{}", "basic"), r#"{"variant":"basic"}"#);
    }

    #[test]
    fn demo_payload_skips_shared_infrastructure_files_that_are_not_real_variants() {
        let entry = ChartDemoEntry {
            file: "src/services/plot/statistical/bar/config.rs",
            fn_name: "render",
            kwargs: "values=[1,2,3]",
            media: "",
        };
        assert!(demo_payload(&entry).is_none());
    }

    #[test]
    fn demo_payload_resolves_a_real_variant_to_its_builder_family_and_injects_the_variant_key() {
        let entry = ChartDemoEntry {
            file: "src/services/plot/statistical/bar/pictogram.rs",
            fn_name: "render",
            kwargs: "labels=[\"A\",\"B\"], values=[1,2]",
            media: "",
        };
        let payload = demo_payload(&entry).unwrap();
        assert_eq!(payload.builder, "build_bar");
        assert_eq!(payload.family, "bar");
        assert_eq!(payload.variant, "pictogram");
        assert!(payload.json.contains("\"variant\":\"pictogram\""));
        assert!(payload.json.contains("\"labels\":[\"A\",\"B\"]"));
    }

    #[test]
    fn demo_payload_disambiguates_two_independent_builders_declared_in_the_same_file_by_their_own_function_name() {
        let mesh = ChartDemoEntry {
            file: "src/services/plot/statistical/_3d/mesh3d.rs",
            fn_name: "build_mesh3d_chart",
            kwargs: "x=[0,1,0,1], y=[0,0,1,1], z=[0,0,0,1], mesh_i=[0], mesh_j=[1], mesh_k=[2]",
            media: "",
        };
        let wire = ChartDemoEntry {
            file: "src/services/plot/statistical/_3d/mesh3d.rs",
            fn_name: "build_wireframe3d_chart",
            kwargs: "x=[0,1,2], y=[0,1,2], matrix=[[0,1,0],[1,2,1],[0,1,0]]",
            media: "",
        };
        let mesh_payload = demo_payload(&mesh).unwrap();
        let wire_payload = demo_payload(&wire).unwrap();
        assert_eq!(mesh_payload.builder, "build_mesh3d_chart");
        assert_eq!(wire_payload.builder, "build_wireframe3d_chart");
        assert_ne!(mesh_payload.builder, wire_payload.builder);
    }

    #[test]
    fn demo_payload_never_trusts_the_generic_per_variant_render_function_name_as_a_direct_builder() {
        let entry = ChartDemoEntry {
            file: "src/services/plot/statistical/boxplot/violin.rs",
            fn_name: "render",
            kwargs: "labels=[\"A\",\"B\"], series=[[1.0,2.0],[3.0,4.0]]",
            media: "",
        };
        let payload = demo_payload(&entry).unwrap();
        assert_eq!(payload.builder, "build_boxplot");
        assert_eq!(payload.variant, "violin");
    }

    #[test]
    fn every_registered_chart_demo_entry_produces_non_empty_html_through_its_resolved_builder() {
        let mut checked = 0usize;
        let mut failures: Vec<String> = Vec::new();
        for entry in iter_entries() {
            let Some(payload) = demo_payload(entry) else {
                continue;
            };
            let Some(fe) = crate::bindings::fn_registry::iter_entries()
                .find(|e| e.name == payload.builder)
            else {
                continue;
            };
            checked += 1;
            let html = (fe.invoke)(&payload.json);
            if html.trim().is_empty() {
                failures.push(format!(
                    "{}::{} ({}) produced empty output for kwargs: {}",
                    payload.family, payload.variant, payload.builder, entry.kwargs
                ));
            }
        }
        assert!(
            checked > 50,
            "expected the chart_demo registry to cover a substantial share of the plot catalog, only resolved {checked} entries against a real fn_registry builder"
        );
        assert!(
            failures.is_empty(),
            "{} of {} chart_demo entries rendered empty output:\n{}",
            failures.len(),
            checked,
            failures.join("\n")
        );
    }
}
