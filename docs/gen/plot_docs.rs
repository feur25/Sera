use std::fs;
use std::path::Path;

struct MethodDocParam {
    name: String,
    ty: String,
    en: String,
    fr: String,
}

struct MethodDocEntry {
    name: String,
    category: String,
    file: String,
    en: String,
    fr: String,
    aliases: Vec<String>,
    params: Vec<MethodDocParam>,
}

fn extract_quoted_string(s: &str) -> Option<(String, usize)> {
    if !s.starts_with('"') {
        return None;
    }
    let mut out = String::new();
    let mut esc = false;
    for (i, c) in s[1..].char_indices() {
        if esc {
            out.push('\\');
            out.push(c);
            esc = false;
        } else if c == '\\' {
            esc = true;
        } else if c == '"' {
            return Some((out, i + 2));
        } else {
            out.push(c);
        }
    }
    None
}

fn extract_paren_body(src: &str, at: usize) -> Option<(String, usize)> {
    if src.as_bytes().get(at) != Some(&b'(') {
        return None;
    }
    let mut depth = 0i32;
    let mut in_str = false;
    let mut esc = false;
    let body_start = at + 1;
    for (i, c) in src[at..].char_indices() {
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
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some((src[body_start..at + i].to_string(), at + i + 1));
                }
            }
            _ => {}
        }
    }
    None
}

fn kv_str(body: &str, key: &str) -> String {
    let pattern = format!("{} = \"", key);
    let Some(pos) = body.find(&pattern) else {
        return String::new();
    };
    let s = &body[pos + pattern.len() - 1..];
    extract_quoted_string(s).map(|(v, _)| v).unwrap_or_default()
}

fn parse_aliases_group(body: &str) -> Vec<String> {
    let Some(pos) = body.find("aliases(") else {
        return Vec::new();
    };
    let paren_pos = pos + "aliases".len();
    let Some((content, _)) = extract_paren_body(body, paren_pos) else {
        return Vec::new();
    };
    let mut result = Vec::new();
    let mut rest = content.as_str();
    loop {
        let Some(q) = rest.find('"') else { break };
        rest = &rest[q..];
        let Some((s, consumed)) = extract_quoted_string(rest) else {
            break;
        };
        result.push(s);
        rest = &rest[consumed..];
    }
    result
}

fn parse_params_group(body: &str) -> Vec<MethodDocParam> {
    let mut result = Vec::new();
    let mut pos = 0;
    while let Some(rel) = body[pos..].find("param(") {
        let abs_paren = pos + rel + "param".len();
        let Some((content, end)) = extract_paren_body(body, abs_paren) else {
            pos += rel + 1;
            continue;
        };
        let p = MethodDocParam {
            name: kv_str(&content, "name"),
            ty: kv_str(&content, "ty"),
            en: kv_str(&content, "en"),
            fr: kv_str(&content, "fr"),
        };
        if !p.name.is_empty() {
            result.push(p);
        }
        pos = end;
    }
    result
}

fn skip_bracket_attr(s: &str) -> Option<usize> {
    if !s.starts_with("#[") {
        return None;
    }
    let mut depth = 0i32;
    let mut in_str = false;
    let mut esc = false;
    for (i, c) in s[1..].char_indices() {
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
        match c {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i + 2);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_fn_name_after(src: &str, from: usize) -> String {
    let mut pos = from;
    loop {
        let trimmed = src[pos..].trim_start();
        let ws = src[pos..].len() - trimmed.len();
        pos += ws;
        if trimmed.starts_with("#[") {
            if let Some(skip) = skip_bracket_attr(trimmed) {
                pos += skip;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    let s = src[pos..].trim_start();
    for prefix in &["pub async fn ", "pub fn ", "fn "] {
        if let Some(rest) = s.strip_prefix(prefix) {
            let end = rest
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .unwrap_or(rest.len());
            return rest[..end].to_string();
        }
    }
    String::new()
}

fn parse_method_docs(src: &str) -> Vec<MethodDocEntry> {
    let mut entries = Vec::new();
    let mut pos = 0;
    while let Some(rel) = src[pos..].find("#[sera_doc(") {
        let attr_start = pos + rel;
        let paren_start = attr_start + "#[sera_doc".len();
        let Some((body, paren_end)) = extract_paren_body(src, paren_start) else {
            pos = attr_start + 1;
            continue;
        };
        let after_attr = paren_end + 1;
        let fn_name = find_fn_name_after(src, after_attr);
        if !fn_name.is_empty() {
            entries.push(MethodDocEntry {
                name: fn_name,
                category: kv_str(&body, "category"),
                file: kv_str(&body, "file"),
                en: kv_str(&body, "en"),
                fr: kv_str(&body, "fr"),
                aliases: parse_aliases_group(&body),
                params: parse_params_group(&body),
            });
        }
        pos = paren_end;
    }
    entries
}

fn doc_js_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

pub struct PlotDocData {
    pub demo_entries: Vec<(String, String, String)>,
    pub param_entries: Vec<(String, String, Vec<String>)>,
    pub required_entries: Vec<(String, String, Vec<String>)>,
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
    required_entries: &mut Vec<(String, String, Vec<String>)>,
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
                    demo_entries.push((family.to_string(), variant.clone(), demo));
                }
            }
            if !required_entries
                .iter()
                .any(|(f, v, _)| f == family && v == &variant)
            {
                if let Some((_, _, required)) = required_entries
                    .iter()
                    .find(|(f, v, _)| f == family && v == module)
                    .cloned()
                {
                    required_entries.push((family.to_string(), variant, required));
                }
            }
        }
    }
    param_entries.sort();
    param_entries.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    demo_entries.sort();
    demo_entries.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    required_entries.sort();
    required_entries.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
}

pub fn collect(plot_root: &Path) -> PlotDocData {
    let mut files = Vec::new();
    crate::build_common::walk(plot_root, &mut files);
    files.sort();
    let mut demo_entries = Vec::new();
    let mut param_entries = Vec::new();
    let mut required_entries = Vec::new();
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
            let dir = f.parent();
            let allowed = dir.map(crate::build_common::config_field_names).unwrap_or_default();
            let mut fields = crate::build_common::filtered_auto_fields(&src, &allowed);
            if let Some(dir) = dir {
                for shared_name in ["common.rs", "shared.rs"] {
                    let shared_path = dir.join(shared_name);
                    if let Ok(shared_src) = fs::read_to_string(&shared_path) {
                        for s in crate::build_common::filtered_auto_fields(&shared_src, &allowed) {
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
            let dir = f.parent();
            let true_required = dir.map(crate::build_common::required_data_fields).unwrap_or_default();
            let required: Vec<String> = params
                .iter()
                .filter(|p| true_required.contains(p))
                .cloned()
                .collect();
            if !required.is_empty() {
                required_entries.push((family.clone(), variant.clone(), required));
            }
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
    required_entries.sort();
    inherit_dispatch_params(plot_root, &mut param_entries, &mut demo_entries, &mut required_entries);
    alias_entries.sort();
    PlotDocData {
        demo_entries,
        param_entries,
        required_entries,
        alias_entries,
    }
}

pub fn write_registry(
    src_root: &Path,
    plot_root: &Path,
    demo_entries: &[(String, String, String)],
    param_entries: &[(String, String, Vec<String>)],
    required_entries: &[(String, String, Vec<String>)],
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
    js.push_str("},trueRequired:{");
    for (i, (f, v, ps)) in required_entries.iter().enumerate() {
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
    js.push_str("]},docs:[");
    let lib_src = fs::read_to_string(src_root.join("lib.rs")).unwrap_or_default();
    let method_docs = parse_method_docs(&lib_src);
    for (i, entry) in method_docs.iter().enumerate() {
        if i > 0 {
            js.push(',');
        }
        js.push_str("{name:\"");
        js.push_str(&doc_js_str(&entry.name));
        js.push_str("\",category:\"");
        js.push_str(&doc_js_str(&entry.category));
        js.push_str("\",file:\"");
        js.push_str(&doc_js_str(&entry.file));
        js.push_str("\",en:\"");
        js.push_str(&doc_js_str(&entry.en));
        js.push_str("\",fr:\"");
        js.push_str(&doc_js_str(&entry.fr));
        js.push_str("\",aliases:[");
        for (ai, a) in entry.aliases.iter().enumerate() {
            if ai > 0 {
                js.push(',');
            }
            js.push('"');
            js.push_str(&doc_js_str(a));
            js.push('"');
        }
        js.push_str("],params:[");
        for (pi, p) in entry.params.iter().enumerate() {
            if pi > 0 {
                js.push(',');
            }
            js.push_str("{name:\"");
            js.push_str(&doc_js_str(&p.name));
            js.push_str("\",ty:\"");
            js.push_str(&doc_js_str(&p.ty));
            js.push_str("\",en:\"");
            js.push_str(&doc_js_str(&p.en));
            js.push_str("\",fr:\"");
            js.push_str(&doc_js_str(&p.fr));
            js.push_str("\"}");
        }
        js.push_str("]}");
    }
    js.push_str("]};\n");
    let path = src_root.join("docs").join("theme").join("doc-registry.js");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(path, js).expect("write doc-registry.js");
}
