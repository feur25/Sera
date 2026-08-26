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

pub fn invoke(entry: &FnEntry, json: &str) -> String {
    let html = (entry.invoke)(json);
    if entry.input != InputKind::Json || entry.output != OutputKind::Html {
        return html;
    }
    let parsed: serde_json::Value = match serde_json::from_str(json) {
        Ok(v) => v,
        Err(_) => return html,
    };
    let obj = match parsed.as_object() {
        Some(o) => o,
        None => return html,
    };
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