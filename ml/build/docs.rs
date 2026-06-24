use std::fs;
use std::path::Path;

#[derive(Clone)]
struct MlDocEntry {
    name: String,
    category: String,
    file: String,
    en: String,
    fr: String,
    aliases: Vec<String>,
    params: Vec<String>,
    returns: Vec<String>,
}

#[derive(Clone)]
struct MlModelEntry {
    name: String,
    category: String,
    domain: String,
    fields: Vec<(String, String)>,
}

fn attr_value(attr_src: &str, key: &str) -> String {
    let needle = format!("{key} = \"");
    let Some(start) = attr_src.find(&needle) else {
        return String::new();
    };
    let rest = &attr_src[start + needle.len()..];
    let mut out = String::new();
    let mut esc = false;
    for c in rest.chars() {
        if esc {
            out.push(c);
            esc = false;
            continue;
        }
        if c == '\\' {
            esc = true;
            continue;
        }
        if c == '"' {
            break;
        }
        out.push(c);
    }
    out
}

fn attr_block_before(src: &str, marker: &str, end_pos: usize) -> Option<String> {
    let before = &src[..end_pos.min(src.len())];
    let pos = before.rfind(marker)?;
    let tail = &before[pos..];
    let end = tail.find(")]")?;
    Some(tail[..end + 2].to_string())
}

fn fn_body(src: &str, fn_pos: usize) -> Option<&str> {
    let rest = &src[fn_pos..];
    let open_rel = rest.find('{')?;
    let open = fn_pos + open_rel;
    let mut depth = 0i32;
    let mut in_str = false;
    let mut esc = false;
    for (i, c) in src[open..].char_indices() {
        if esc {
            esc = false;
            continue;
        }
        if c == '\\' && in_str {
            esc = true;
            continue;
        }
        if c == '"' {
            in_str = !in_str;
            continue;
        }
        if in_str {
            continue;
        }
        if c == '{' {
            depth += 1;
        } else if c == '}' {
            depth -= 1;
            if depth == 0 {
                return Some(&src[open..open + i + 1]);
            }
        }
    }
    None
}

fn extract_ml_params(src: &str, fn_name: &str) -> Vec<String> {
    let mut out = vec![
        "data".to_string(),
        "target".to_string(),
        "test_data".to_string(),
    ];
    let Some(pos) = src.find(&format!("pub fn {fn_name}")) else {
        return out;
    };
    let body = fn_body(src, pos).unwrap_or(&src[pos..]);
    for helper in ["jf", "jb", "ju", "js"] {
        let needle = format!("{helper}(&v, \"");
        let mut cur = 0;
        while let Some(p) = body[cur..].find(&needle) {
            let start = cur + p + needle.len();
            let rest = &body[start..];
            if let Some(end) = rest.find('"') {
                let name = rest[..end].to_string();
                if !out.contains(&name) {
                    out.push(name);
                }
            }
            cur = start;
        }
    }
    for form in ["struct I", "struct Input"] {
        if let Some(spos) = body.find(form) {
            let rest = &body[spos..];
            if let Some(open) = rest.find('{') {
                if let Some(close) = rest[open + 1..].find('}') {
                    let inner = &rest[open + 1..open + 1 + close];
                    for line in inner.lines() {
                        let line = line.trim();
                        let Some(colon) = line.find(':') else {
                            continue;
                        };
                        let name = line[..colon].trim().trim_start_matches("pub ").to_string();
                        if !name.is_empty()
                            && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                            && !out.contains(&name)
                        {
                            out.push(name);
                        }
                    }
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

fn push_key(out: &mut Vec<String>, key: &str) {
    if !key.is_empty()
        && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !out.iter().any(|x| x == key)
    {
        out.push(key.to_string());
    }
}

fn scan_json_text(text: &str, out: &mut Vec<String>) {
    let mut i = 0;
    let bytes = text.as_bytes();
    while i < bytes.len() {
        if bytes[i] != b'"' {
            i += 1;
            continue;
        }
        let start = i + 1;
        let mut j = start;
        let mut esc = false;
        while j < bytes.len() {
            let c = bytes[j];
            if esc {
                esc = false;
                j += 1;
                continue;
            }
            if c == b'\\' {
                esc = true;
                j += 1;
                continue;
            }
            if c == b'"' {
                break;
            }
            j += 1;
        }
        if j >= bytes.len() {
            break;
        }
        let key = &text[start..j];
        let mut k = j + 1;
        while k < bytes.len() && bytes[k].is_ascii_whitespace() {
            k += 1;
        }
        if k < bytes.len() && bytes[k] == b':' {
            push_key(out, key);
        }
        i = j + 1;
    }
}

fn colon_after(src: &str, pos: usize) -> bool {
    let bytes = src.as_bytes();
    let mut i = pos;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    i < bytes.len() && bytes[i] == b':'
}

fn extract_json_keys(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'r' {
            let mut h = i + 1;
            while h < bytes.len() && bytes[h] == b'#' {
                h += 1;
            }
            if h < bytes.len() && bytes[h] == b'"' {
                let hashes = h - i - 1;
                let content_start = h + 1;
                let mut j = content_start;
                while j < bytes.len() {
                    if bytes[j] == b'"'
                        && j + hashes < bytes.len()
                        && bytes[j + 1..j + 1 + hashes].iter().all(|b| *b == b'#')
                    {
                        let content = &src[content_start..j];
                        scan_json_text(content, &mut out);
                        if colon_after(src, j + 1 + hashes) {
                            push_key(&mut out, content);
                        }
                        i = j + 1 + hashes;
                        break;
                    }
                    j += 1;
                }
                if j >= bytes.len() {
                    break;
                }
                continue;
            }
        }
        if bytes[i] == b'"' {
            let start = i + 1;
            let mut j = start;
            let mut content = String::new();
            while j < bytes.len() {
                let c = bytes[j];
                if c == b'\\' && j + 1 < bytes.len() {
                    let next = bytes[j + 1] as char;
                    content.push(next);
                    j += 2;
                    continue;
                }
                if c == b'"' {
                    break;
                }
                content.push(c as char);
                j += 1;
            }
            if j >= bytes.len() {
                break;
            }
            scan_json_text(&content, &mut out);
            if colon_after(src, j + 1) {
                push_key(&mut out, &content);
            }
            i = j + 1;
            continue;
        }
        i += 1;
    }
    out
}

fn extract_ml_returns(src: &str, fn_name: &str) -> Vec<String> {
    let Some(pos) = src.find(&format!("pub fn {fn_name}")) else {
        return Vec::new();
    };
    let body = fn_body(src, pos).unwrap_or(&src[pos..]);
    extract_json_keys(body)
}

fn inferred_ml_file(name: &str, fallback: &str) -> String {
    match name {
        "ml_linear_regression" => "linear-regression.md",
        "ml_ridge" | "ml_ridge_classifier" => "ridge.md",
        "ml_lasso" => "lasso.md",
        "ml_elastic_net" => "elastic-net.md",
        "ml_logistic_regression" => "logistic-regression.md",
        "ml_sgd_classifier" | "ml_sgd_regressor" => "sgd.md",
        "ml_decision_tree_classifier" | "ml_decision_tree_regressor" => "decision-tree.md",
        "ml_random_forest_classifier" | "ml_random_forest_regressor" => "random-forest.md",
        "ml_gradient_boosting_classifier" | "ml_gradient_boosting_regressor" => {
            "gradient-boosting.md"
        }
        "ml_adaboost_classifier" | "ml_adaboost_regressor" => "adaboost.md",
        "ml_knn_classifier" | "ml_knn_regressor" | "ml_nearest_centroid" => "knn.md",
        "ml_gaussian_nb" | "ml_multinomial_nb" | "ml_bernoulli_nb" => "naive-bayes.md",
        "ml_linear_svc" | "ml_linear_svr" => "svm.md",
        "ml_pca" | "ml_truncated_svd" => "decomposition.md",
        "ml_dbscan_fit_predict" => "dbscan.md",
        "ml_kmeans_fit_predict" => "kmeans.md",
        "ml_kfold_split" | "ml_cross_val_score" => "cv-splitters.md",
        "ml_grid_search_cv" => "grid-search.md",
        "ml_permutation_importance" => "permutation-importance.md",
        "ml_isolation_forest" => "isolation-forest.md",
        "ml_metric_score" | "ml_metric_curve" => "metrics.md",
        "ml_save_model" | "ml_load_model" => "registry.md",
        "ml_standard_scaler" | "ml_minmax_scaler" | "ml_robust_scaler" | "ml_fit_transform" => {
            "preprocessing.md"
        }
        _ => fallback,
    }
    .to_string()
}

fn extract_ml_docs(ml_root: &Path) -> Vec<MlDocEntry> {
    let bindings_root = ml_root.join("bindings");
    let mut files = Vec::new();
    crate::build_common::walk(&bindings_root, &mut files);
    files.sort();
    let mut out = Vec::new();
    for f in files {
        let Ok(src) = fs::read_to_string(&f) else {
            continue;
        };
        let mut cur = 0;
        let mut current_category = String::new();
        let mut current_file = String::new();
        let mut current_en = String::new();
        let mut current_fr = String::new();
        while let Some(pos_rel) = src[cur..].find("pub fn ml_") {
            let pos = cur + pos_rel;
            if let Some(attr) = attr_block_before(&src, "sera_doc(", pos) {
                let cat = attr_value(&attr, "category");
                if !cat.is_empty() {
                    current_category = cat;
                }
                let file = attr_value(&attr, "file");
                if !file.is_empty() {
                    current_file = file;
                }
                let en = attr_value(&attr, "en");
                if !en.is_empty() {
                    current_en = en;
                }
                let fr = attr_value(&attr, "fr");
                if !fr.is_empty() {
                    current_fr = fr;
                }
            }
            let name_start = pos + "pub fn ".len();
            let tail = &src[name_start..];
            let name: String = tail
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if name == "ml_parse" {
                cur = name_start + name.len();
                continue;
            }
            if name.is_empty() {
                cur = pos + 1;
                continue;
            }
            let file = inferred_ml_file(&name, &current_file);
            let aliases = attr_block_before(&src, "sera_alias(", pos)
                .map(|a| crate::build_common::extract_sera_aliases(&a))
                .unwrap_or_default();
            out.push(MlDocEntry {
                name: name.clone(),
                category: current_category.clone(),
                file,
                en: current_en.clone(),
                fr: current_fr.clone(),
                aliases,
                params: extract_ml_params(&src, &name),
                returns: extract_ml_returns(&src, &name),
            });
            cur = name_start + name.len();
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

fn extract_ml_models(ml_root: &Path) -> Vec<MlModelEntry> {
    let mut files = Vec::new();
    crate::build_common::walk(ml_root, &mut files);
    files.sort();
    let mut out = Vec::new();
    for f in files {
        if f.components().any(|c| c.as_os_str() == "bindings") {
            continue;
        }
        let Ok(src) = fs::read_to_string(&f) else {
            continue;
        };
        let mut cur = 0;
        while let Some(pos_rel) = src[cur..].find("#[crate::model(") {
            let pos = cur + pos_rel;
            let rest = &src[pos..];
            let Some(attr_end) = rest.find(")]") else {
                break;
            };
            let attr = &rest[..attr_end + 2];
            let after = &rest[attr_end + 2..];
            let Some(struct_pos) = after.find("pub struct ") else {
                cur = pos + attr_end + 2;
                continue;
            };
            let name_start = struct_pos + "pub struct ".len();
            let name_tail = &after[name_start..];
            let name: String = name_tail
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            let mut fields = Vec::new();
            if let Some(open) = name_tail.find('{') {
                if let Some(close) = name_tail[open + 1..].find('}') {
                    let inner = &name_tail[open + 1..open + 1 + close];
                    for line in inner.lines() {
                        let line = line.trim();
                        if !line.starts_with("pub ") {
                            continue;
                        }
                        let Some(colon) = line.find(':') else {
                            continue;
                        };
                        let fname = line[4..colon].trim().to_string();
                        let ty = line[colon + 1..].trim().trim_end_matches(',').to_string();
                        if !fname.is_empty() {
                            fields.push((fname, ty));
                        }
                    }
                }
            }
            if !name.is_empty() {
                out.push(MlModelEntry {
                    name,
                    category: attr_value(attr, "category"),
                    domain: attr_value(attr, "domain"),
                    fields,
                });
            }
            cur = pos + attr_end + 2;
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

pub fn write_registry(manifest: &Path, ml_root: &Path) {
    let docs = extract_ml_docs(ml_root);
    let models = extract_ml_models(ml_root);
    let mut js = String::from("window.SeraPlotMlRegistry={docs:[");
    for (i, d) in docs.iter().enumerate() {
        if i > 0 {
            js.push(',');
        }
        js.push_str("{name:\"");
        js.push_str(&crate::build_common::js_escape(&d.name));
        js.push_str("\",category:\"");
        js.push_str(&crate::build_common::js_escape(&d.category));
        js.push_str("\",file:\"");
        js.push_str(&crate::build_common::js_escape(&d.file));
        js.push_str("\",en:\"");
        js.push_str(&crate::build_common::js_escape(&d.en));
        js.push_str("\",fr:\"");
        js.push_str(&crate::build_common::js_escape(&d.fr));
        js.push_str("\",aliases:[");
        for (ai, a) in d.aliases.iter().enumerate() {
            if ai > 0 {
                js.push(',');
            }
            js.push('"');
            js.push_str(&crate::build_common::js_escape(a));
            js.push('"');
        }
        js.push_str("],params:[");
        for (pi, p) in d.params.iter().enumerate() {
            if pi > 0 {
                js.push(',');
            }
            js.push('"');
            js.push_str(&crate::build_common::js_escape(p));
            js.push('"');
        }
        js.push_str("],returns:[");
        for (ri, r) in d.returns.iter().enumerate() {
            if ri > 0 {
                js.push(',');
            }
            js.push('"');
            js.push_str(&crate::build_common::js_escape(r));
            js.push('"');
        }
        js.push_str("]}");
    }
    js.push_str("],models:[");
    for (i, m) in models.iter().enumerate() {
        if i > 0 {
            js.push(',');
        }
        js.push_str("{name:\"");
        js.push_str(&crate::build_common::js_escape(&m.name));
        js.push_str("\",category:\"");
        js.push_str(&crate::build_common::js_escape(&m.category));
        js.push_str("\",domain:\"");
        js.push_str(&crate::build_common::js_escape(&m.domain));
        js.push_str("\",fields:[");
        for (fi, (name, ty)) in m.fields.iter().enumerate() {
            if fi > 0 {
                js.push(',');
            }
            js.push_str("{name:\"");
            js.push_str(&crate::build_common::js_escape(name));
            js.push_str("\",ty:\"");
            js.push_str(&crate::build_common::js_escape(ty));
            js.push_str("\"}");
        }
        js.push_str("]}");
    }
    js.push_str("]};\n");
    let path = manifest
        .join("src")
        .join("docs")
        .join("theme")
        .join("ml-registry.js");
    fs::write(path, js).expect("write ml-registry.js");
}
