use std::fs;
use std::path::Path;

pub struct PlotDocData {
    pub demo_entries: Vec<(String, String, String)>,
    pub param_entries: Vec<(String, String, Vec<String>)>,
    pub alias_entries: Vec<(String, Vec<String>)>,
}

fn parse_plot_family(src: &str) -> Vec<(String, Vec<String>)> {
    let Some(start) = src.find("plot_family!") else {
        return Vec::new();
    };
    let Some(open_rel) = src[start..].find('{') else {
        return Vec::new();
    };
    let open = start + open_rel;
    let mut depth = 0i32;
    let mut close = src.len();
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
                close = open + i;
                break;
            }
        }
    }
    if close <= open {
        return Vec::new();
    }
    let body = &src[open + 1..close];
    let mut out = Vec::new();
    for line in body.lines() {
        let Some(pos) = line.find("=>") else {
            continue;
        };
        let rhs = &line[pos + 2..];
        let mut vals = Vec::new();
        let mut buf = String::new();
        let mut in_str = false;
        let mut esc = false;
        for c in rhs.chars() {
            if esc {
                if in_str {
                    buf.push(c);
                }
                esc = false;
                continue;
            }
            if c == '\\' {
                esc = true;
                continue;
            }
            if c == '"' {
                if in_str {
                    vals.push(buf.clone());
                    buf.clear();
                }
                in_str = !in_str;
                continue;
            }
            if in_str {
                buf.push(c);
            }
        }
        if let Some((key, aliases)) = vals.split_first() {
            out.push((key.clone(), aliases.to_vec()));
        }
    }
    out
}

fn pascal_to_snake(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn inherit_dispatch_params(
    plot_root: &Path,
    param_entries: &mut Vec<(String, String, Vec<String>)>,
    demo_entries: &mut Vec<(String, String, String)>,
) {
    let mut files = Vec::new();
    crate::build_common::walk(plot_root, &mut files);
    files.sort();
    for f in files {
        if f.file_name().and_then(|n| n.to_str()) != Some("mod.rs") {
            continue;
        }
        let Some(family) = f
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
        else {
            continue;
        };
        let Ok(src) = fs::read_to_string(&f) else {
            continue;
        };
        let valid_variants = f
            .parent()
            .map(|p| p.join("variant.rs"))
            .and_then(|p| fs::read_to_string(p).ok())
            .map(|s| parse_plot_family(&s))
            .unwrap_or_default();
        for line in src.lines() {
            let Some(pos) = line.find("=>") else {
                continue;
            };
            let left = line[..pos].trim().trim_start_matches('|').trim();
            let Some(left_token) = left.split_whitespace().next().map(|s| s.trim_matches(','))
            else {
                continue;
            };
            if !left_token.chars().all(|c| c.is_ascii_alphanumeric()) {
                continue;
            }
            let variant = pascal_to_snake(left_token);
            if !valid_variants.iter().any(|(v, _)| v == &variant) {
                continue;
            }
            let right = &line[pos + 2..];
            let Some(module_end) = right.find("::render") else {
                continue;
            };
            let module = right[..module_end]
                .rsplit(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .next()
                .unwrap_or_default()
                .trim();
            if module.is_empty() || module == variant {
                continue;
            }
            if !param_entries
                .iter()
                .any(|(f, v, _)| f == family && v == &variant)
            {
                if let Some((_, _, params)) = param_entries
                    .iter()
                    .find(|(f, v, _)| f == family && v == module)
                    .cloned()
                {
                    param_entries.push((family.to_string(), variant.clone(), params));
                }
            }
            if !demo_entries
                .iter()
                .any(|(f, v, _)| f == family && v == &variant)
            {
                if let Some((_, _, demo)) = demo_entries
                    .iter()
                    .find(|(f, v, _)| f == family && v == module)
                    .cloned()
                {
                    demo_entries.push((family.to_string(), variant, demo));
                }
            }
        }
    }
    param_entries.sort();
    param_entries.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    demo_entries.sort();
    demo_entries.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
}

pub fn collect(plot_root: &Path) -> PlotDocData {
    let mut files = Vec::new();
    crate::build_common::walk(plot_root, &mut files);
    files.sort();
    let mut demo_entries = Vec::new();
    let mut param_entries = Vec::new();
    let mut alias_entries = Vec::new();
    for f in &files {
        let Ok(src) = fs::read_to_string(f) else {
            continue;
        };
        let Some((family, variant)) = crate::build_common::family_variant(f, plot_root) else {
            continue;
        };
        if variant == "mod"
            || variant == "common"
            || variant == "config"
            || variant == "shared"
            || variant == "variant"
        {
            continue;
        }
        if let Some(kw) = crate::build_common::extract_kwargs(&src) {
            demo_entries.push((family.clone(), variant.clone(), kw));
        }
        let params = if let Some(explicit) = crate::build_common::extract_params_required(&src) {
            explicit
        } else {
            let mut fields = crate::build_common::filtered_auto_fields(&src);
            if let Some(dir) = f.parent() {
                for shared_name in ["common.rs", "shared.rs"] {
                    let shared_path = dir.join(shared_name);
                    if let Ok(shared_src) = fs::read_to_string(&shared_path) {
                        for s in crate::build_common::filtered_auto_fields(&shared_src) {
                            if !fields.contains(&s) {
                                fields.push(s);
                            }
                        }
                    }
                }
            }
            fields
        };
        if !params.is_empty() {
            param_entries.push((family.clone(), variant.clone(), params));
        }
        let aliases = crate::build_common::extract_sera_aliases(&src);
        if !aliases.is_empty() {
            if let Some(base) = crate::build_common::module_basename(f) {
                alias_entries.push((format!("{}::{}", family, base), aliases));
            }
        }
    }
    demo_entries.sort();
    param_entries.sort();
    inherit_dispatch_params(plot_root, &mut param_entries, &mut demo_entries);
    alias_entries.sort();
    PlotDocData {
        demo_entries,
        param_entries,
        alias_entries,
    }
}

pub fn write_registry(
    manifest: &Path,
    plot_root: &Path,
    demo_entries: &[(String, String, String)],
    param_entries: &[(String, String, Vec<String>)],
) {
    let mut files = Vec::new();
    crate::build_common::walk(plot_root, &mut files);
    files.sort();
    let mut families: Vec<(String, Vec<(String, Vec<String>)>)> = Vec::new();
    let mut themes: Vec<(String, Vec<String>)> = Vec::new();
    for f in files {
        let Ok(src) = fs::read_to_string(&f) else {
            continue;
        };
        let parsed = parse_plot_family(&src);
        if parsed.is_empty() {
            continue;
        }
        if f.file_name().and_then(|n| n.to_str()) == Some("theme.rs") {
            themes = parsed;
            continue;
        }
        if f.file_name().and_then(|n| n.to_str()) == Some("variant.rs") {
            if let Some(parent) = f
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
            {
                families.push((parent.to_string(), parsed));
            }
        }
    }
    families.sort_by(|a, b| a.0.cmp(&b.0));
    let mut js = String::from("window.SeraPlotDocRegistry={variants:{");
    for (fi, (family, variants)) in families.iter().enumerate() {
        if fi > 0 {
            js.push(',');
        }
        js.push('"');
        js.push_str(&crate::build_common::js_escape(family));
        js.push_str("\":{default:\"");
        if let Some((key, _)) = variants.first() {
            js.push_str(&crate::build_common::js_escape(key));
        }
        js.push_str("\",variants:[");
        for (vi, (key, aliases)) in variants.iter().enumerate() {
            if vi > 0 {
                js.push(',');
            }
            js.push_str("{key:\"");
            js.push_str(&crate::build_common::js_escape(key));
            js.push_str("\",aliases:[");
            for (ai, alias) in aliases.iter().enumerate() {
                if ai > 0 {
                    js.push(',');
                }
                js.push('"');
                js.push_str(&crate::build_common::js_escape(alias));
                js.push('"');
            }
            js.push_str("]}");
        }
        js.push_str("]}");
    }
    js.push_str("},params:{");
    for (i, (f, v, k)) in demo_entries.iter().enumerate() {
        if i > 0 {
            js.push(',');
        }
        js.push('"');
        js.push_str(&crate::build_common::js_escape(&format!("{f}:{v}")));
        js.push_str("\":\"");
        js.push_str(&crate::build_common::js_escape(k));
        js.push('"');
    }
    js.push_str("},required:{");
    for (i, (f, v, ps)) in param_entries.iter().enumerate() {
        if i > 0 {
            js.push(',');
        }
        js.push('"');
        js.push_str(&crate::build_common::js_escape(&format!("{f}:{v}")));
        js.push_str("\":[");
        for (pi, p) in ps.iter().enumerate() {
            if pi > 0 {
                js.push(',');
            }
            js.push('"');
            js.push_str(&crate::build_common::js_escape(p));
            js.push('"');
        }
        js.push(']');
    }
    js.push_str("},themes:{default:\"none\",themes:[");
    for (i, (key, aliases)) in themes.iter().enumerate() {
        if i > 0 {
            js.push(',');
        }
        js.push_str("{key:\"");
        js.push_str(&crate::build_common::js_escape(key));
        js.push_str("\",aliases:[");
        for (ai, alias) in aliases.iter().enumerate() {
            if ai > 0 {
                js.push(',');
            }
            js.push('"');
            js.push_str(&crate::build_common::js_escape(alias));
            js.push('"');
        }
        js.push_str("]}");
    }
    js.push_str("]}};\n");
    let path = manifest
        .join("src")
        .join("docs")
        .join("theme")
        .join("doc-registry.js");
    fs::write(path, js).expect("write doc-registry.js");
}
