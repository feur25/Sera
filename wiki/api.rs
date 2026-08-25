use crate::wiki::{CodeExample, MethodDoc, ModuleDoc, ParamDoc, WikiExport};
use std::collections::BTreeMap;

fn title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(f) => f.to_uppercase().collect::<String>() + &chars.as_str().replace('_', " "),
        None => String::new(),
    }
}

fn method_from_fndoc(doc: &crate::doc_registry::FnDoc) -> MethodDoc {
    let parameters: Vec<ParamDoc> = doc
        .params
        .iter()
        .map(|p| ParamDoc {
            name: p.name.to_string(),
            param_type: p.ty.to_string(),
            description: p.en.to_string(),
        })
        .collect();
    let sig_params: String = doc
        .params
        .iter()
        .map(|p| p.name.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let description = if doc.aliases.is_empty() {
        doc.en.to_string()
    } else {
        format!("{} (aliases: {})", doc.en, doc.aliases.join(", "))
    };
    MethodDoc {
        name: doc.name.to_string(),
        module: doc.module.to_string(),
        description,
        parameters,
        returns: None,
        examples: Vec::new(),
        since_version: None,
        deprecated: false,
        python_signature: format!("sp.{}({})", doc.name, sig_params),
        csharp_signature: String::new(),
        cpp_signature: String::new(),
        rust_signature: String::new(),
    }
}

fn chart_catalog_module() -> ModuleDoc {
    use crate::plot::chart_demo_registry::{demo_payload, iter_entries};

    let mut seen: BTreeMap<String, MethodDoc> = BTreeMap::new();
    for entry in iter_entries() {
        let Some(payload) = demo_payload(entry) else {
            continue;
        };
        if seen.contains_key(&payload.family) {
            continue;
        }
        let aliases = crate::sera_aliases_for(&payload.family)
            .map(|a| a.join(", "))
            .unwrap_or_default();
        let example = crate::demo_snippet(&payload.family, &payload.variant).unwrap_or_default();
        let description = if aliases.is_empty() {
            format!("Chart family '{}'.", payload.family)
        } else {
            format!("Chart family '{}' — aliases: {}", payload.family, aliases)
        };
        seen.insert(
            payload.family.clone(),
            MethodDoc {
                name: payload.builder.clone(),
                module: "Chart Catalog".to_string(),
                description,
                parameters: Vec::new(),
                returns: Some("Chart".to_string()),
                examples: if example.is_empty() {
                    Vec::new()
                } else {
                    vec![CodeExample::new(&example, "", "", "")]
                },
                since_version: None,
                deprecated: false,
                python_signature: format!(
                    "sp.{}(title, ..., variant=\"{}\", **kwargs) -> Chart",
                    payload.family, payload.variant
                ),
                csharp_signature: String::new(),
                cpp_signature: String::new(),
                rust_signature: String::new(),
            },
        );
    }

    ModuleDoc {
        name: "Chart Catalog".to_string(),
        description: "Every chart family currently registered in SeraPlot, one representative entry per family with a real runnable example, sourced live from the same registry that feeds the web documentation.".to_string(),
        methods: seen.into_values().collect(),
    }
}

pub fn generate_seraplot_docs() -> WikiExport {
    let mut export = WikiExport::new("SeraPlot", env!("CARGO_PKG_VERSION"));
    export.add_module(chart_catalog_module());

    let mut by_category: BTreeMap<String, Vec<MethodDoc>> = BTreeMap::new();
    for doc in crate::doc_registry::all_docs() {
        by_category
            .entry(doc.category.to_string())
            .or_default()
            .push(method_from_fndoc(doc));
    }

    for (category, methods) in by_category {
        let name = title_case(&category);
        export.add_module(ModuleDoc {
            name: name.clone(),
            description: format!("{name} functions and methods."),
            methods,
        });
    }

    export
}
