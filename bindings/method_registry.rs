pub struct MethodEntry {
    pub name: &'static str,
    pub apply: fn(&crate::Chart, &str) -> crate::Chart,
}

inventory::collect!(MethodEntry);

pub fn iter_entries() -> impl Iterator<Item = &'static MethodEntry> {
    inventory::iter::<MethodEntry>()
}

static INDEX: std::sync::OnceLock<std::collections::HashMap<&'static str, &'static MethodEntry>> =
    std::sync::OnceLock::new();

fn build_index() -> std::collections::HashMap<&'static str, &'static MethodEntry> {
    let mut map: std::collections::HashMap<&'static str, &'static MethodEntry> =
        iter_entries().map(|e| (e.name, e)).collect();
    for doc in crate::doc_registry::all_docs() {
        if doc.category != "chart_method" {
            continue;
        }
        if let Some(entry) = map.get(doc.name).copied() {
            for alias in doc.aliases {
                map.entry(alias).or_insert(entry);
            }
        }
    }
    map
}

pub fn find(name: &str) -> Option<&'static MethodEntry> {
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

pub fn apply_by_name(html: &str, name: &str, args_json: &str) -> Option<String> {
    let entry = find(name)?;
    let chart = crate::Chart {
        html: html.to_string(),
        doc_str: "",
    };
    Some((entry.apply)(&chart, args_json).html)
}

#[cfg(test)]
mod alias_dispatch_tests {
    use super::find;

    #[test]
    fn grid_and_show_grid_behave_identically_despite_different_closure_pointers() {
        let html = "<html><head></head><body></body></html>";
        let via_canonical = super::apply_by_name(html, "show_grid", "{}").unwrap();
        let via_alias = super::apply_by_name(html, "grid", "{}").unwrap();
        assert_eq!(via_canonical, via_alias, "show_grid and its declared alias 'grid' must render identically");
    }

    #[test]
    fn a11y_and_its_alias_behave_identically_despite_different_closure_pointers() {
        let html = "<html><head></head><body></body></html>";
        let via_canonical = super::apply_by_name(html, "a11y", r#"{"title":"T","desc":"D"}"#).unwrap();
        let via_alias = super::apply_by_name(html, "accessibility", r#"{"title":"T","desc":"D"}"#).unwrap();
        assert_eq!(via_canonical, via_alias, "a11y and its declared alias 'accessibility' must render identically");
    }

    #[test]
    fn every_declared_chart_method_alias_at_least_resolves_to_some_entry() {
        let mut checked = 0;
        let mut missing: Vec<String> = Vec::new();
        for doc in crate::doc_registry::all_docs() {
            if doc.category != "chart_method" || doc.aliases.is_empty() || find(doc.name).is_none() {
                continue;
            }
            for alias in doc.aliases {
                if find(alias).is_none() {
                    missing.push(format!("'{alias}' (declared alias of chart method '{}')", doc.name));
                }
                checked += 1;
            }
        }
        assert!(checked > 100, "expected hundreds of chart_method aliases to check, found {checked} -- doc_registry wiring may be broken");
        assert!(missing.is_empty(), "these declared aliases never resolve to any MethodEntry: {missing:?}");
    }

    #[test]
    fn pascal_case_method_names_resolve_the_same_as_their_snake_case_form() {
        let html = "<html><head></head><body></body></html>";
        let via_snake = super::apply_by_name(html, "show_grid", "{}").unwrap();
        let via_pascal = super::apply_by_name(html, "Grid", "{}").unwrap();
        let via_pascal_canonical = super::apply_by_name(html, "ShowGrid", "{}").unwrap();
        assert_eq!(via_snake, via_pascal, "'Grid' (PascalCase, as a C# caller would naturally write it) must resolve identically to 'show_grid'");
        assert_eq!(via_snake, via_pascal_canonical, "'ShowGrid' (PascalCase canonical name) must resolve identically to 'show_grid'");
    }

    #[test]
    fn no_arg_chain_call_dispatches_show_grid_by_its_canonical_name() {
        let html = "<html><head></head><body></body></html>";
        let out = super::apply_by_name(html, "show_grid", "{}").expect("apply_by_name(show_grid) must succeed");
        assert!(out.contains(".sp-gl{display:block"));
    }

    #[test]
    fn show_legend_embeds_the_universal_legend_builder_script_not_just_a_css_toggle() {
        let html = "<html><head></head><body><svg><rect data-idx=\"0\" data-lbl=\"A\"/></svg></body></html>";
        let out = super::apply_by_name(html, "show_legend", "{}").expect("apply_by_name(show_legend) must succeed");
        assert!(out.contains("sp-leg-grp"), "show_legend must embed the runtime legend builder so charts with no pre-rendered g[data-legend] (plain bar/line/scatter) still get a legend");
    }

    #[test]
    fn leg_alias_behaves_identically_to_show_legend() {
        let html = "<html><head></head><body><svg><rect data-idx=\"0\" data-lbl=\"A\"/></svg></body></html>";
        let via_canonical = super::apply_by_name(html, "show_legend", "{}").unwrap();
        let via_alias = super::apply_by_name(html, "leg", "{}").unwrap();
        assert_eq!(via_canonical, via_alias, "show_legend and its declared alias 'leg' must render identically");
    }

    #[test]
    fn no_legend_hides_both_a_pre_rendered_group_and_a_js_built_one() {
        let html = "<html><head></head><body></body></html>";
        let out = super::apply_by_name(html, "no_legend", "{}").expect("apply_by_name(no_legend) must succeed");
        assert!(out.contains("g[data-legend]") && out.contains("g.sp-leg-grp"), "no_legend must hide both the renderer-emitted group and the runtime-built one from show_legend/legend()");
    }

    #[test]
    fn legends_click_to_toggle_rescale_only_repositions_gridlines_that_were_already_horizontal() {
        let html = "<html><head></head><body></body></html>";
        let out = super::apply_by_name(html, "show_legend", "{}").expect("apply_by_name(show_legend) must succeed");
        assert!(
            out.contains("filter.call(glsAll,function(l){return l.getAttribute('y1')===l.getAttribute('y2');})"),
            "the legend's click-to-toggle rescale must filter .sp-gl to horizontal lines (y1===y2) before repositioning them, or it collapses vertical gridlines (x1===x2) onto the same evenly-distributed y-scale, destroying them"
        );
    }
}
