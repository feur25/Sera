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
    module: String,
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
    let mut out = Vec::new();
    let Some(pos) = src.find(&format!("pub fn {fn_name}")) else {
        return out;
    };
    let body = fn_body(src, pos).unwrap_or(&src[pos..]);
    if body.contains("ml_parse(input)") {
        for name in ["data", "target", "test_data"] {
            push_key(&mut out, name);
        }
    }
    if body.contains("yf(&v)") || body.contains("yi(&v)") {
        push_key(&mut out, "target");
    }
    if body.contains("parse_max_features(&v)") {
        push_key(&mut out, "max_features");
    }
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
    let mut cur = 0;
    while let Some(p) = body[cur..].find("v.get(\"") {
        let start = cur + p + "v.get(\"".len();
        let rest = &body[start..];
        if let Some(end) = rest.find('"') {
            push_key(&mut out, &rest[..end]);
        }
        cur = start;
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

fn fn_attr_block(src: &str, start: usize, end: usize) -> Option<String> {
    let window = &src[start.min(src.len())..end.min(src.len())];
    let pos = window.rfind("sera_doc(")?;
    let tail = &window[pos..];
    let end = tail.find(")]")?;
    Some(tail[..end + 2].to_string())
}

fn slugify(s: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            if dash && !out.is_empty() {
                out.push('-');
            }
            out.push(c.to_ascii_lowercase());
            dash = false;
        } else {
            dash = true;
        }
    }
    out
}

fn inferred_ml_file(source: &Path, name: &str, fallback: &str) -> String {
    if !fallback.is_empty() {
        return fallback.to_string();
    }
    let stem = name.strip_prefix("ml_").unwrap_or(name);
    let base = stem
        .strip_suffix("_classifier")
        .or_else(|| stem.strip_suffix("_regressor"))
        .or_else(|| stem.strip_suffix("_fit_predict"))
        .unwrap_or(stem);
    let slug = slugify(base);
    if !slug.is_empty() {
        return format!("{slug}.md");
    }
    let module = source.file_stem().and_then(|s| s.to_str()).unwrap_or("ml");
    format!("{}.md", slugify(module))
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
        while let Some(pos_rel) = src[cur..].find("pub fn ml_") {
            let pos = cur + pos_rel;
            let attr = fn_attr_block(&src, cur, pos);
            let current_category = attr
                .as_deref()
                .map(|a| attr_value(a, "category"))
                .unwrap_or_default();
            let current_file = attr
                .as_deref()
                .map(|a| attr_value(a, "file"))
                .unwrap_or_default();
            let current_en = attr
                .as_deref()
                .map(|a| attr_value(a, "en"))
                .unwrap_or_default();
            let current_fr = attr
                .as_deref()
                .map(|a| attr_value(a, "fr"))
                .unwrap_or_default();
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
            let file = inferred_ml_file(&f, &name, &current_file);
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
                module: String::new(),
            });
            cur = name_start + name.len();
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

fn extract_fn_params(src: &str, sig_start: usize) -> Vec<String> {
    let rest = &src[sig_start..];
    let Some(open_rel) = rest.find('(') else {
        return Vec::new();
    };
    let open = sig_start + open_rel;
    let mut depth = 0i32;
    let mut close = open;
    for (i, c) in src[open..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    close = open + i;
                    break;
                }
            }
            _ => {}
        }
    }
    let inner = &src[open + 1..close];
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut part_start = 0usize;
    let bytes = inner.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' | b'<' | b'[' => depth += 1,
            b')' | b'>' | b']' => depth -= 1,
            b',' if depth == 0 => {
                out.push(inner[part_start..i].to_string());
                part_start = i + 1;
            }
            _ => {}
        }
    }
    if part_start < inner.len() {
        out.push(inner[part_start..].to_string());
    }
    out.into_iter()
        .filter_map(|p| {
            let p = p.trim();
            if p.is_empty() || p.ends_with("self") {
                return None;
            }
            let name = p.split(':').next().unwrap_or("").trim();
            if name.is_empty() {
                None
            } else {
                Some(name.to_string())
            }
        })
        .collect()
}

fn extract_data_docs(data_root: &Path) -> Vec<MlDocEntry> {
    let mut files = Vec::new();
    crate::build_common::walk(data_root, &mut files);
    files.sort();
    let mut out = Vec::new();
    for f in files {
        let Ok(src) = fs::read_to_string(&f) else {
            continue;
        };
        let mut cur = 0;
        while let Some(pos_rel) = src[cur..].find("sera_doc(") {
            let pos = cur + pos_rel;
            let Some(attr_end) = src[pos..].find(")]") else {
                break;
            };
            let attr = &src[pos..pos + attr_end + 2];
            let after = &src[pos + attr_end + 2..];
            let Some(fn_rel) = after.find("fn ") else {
                cur = pos + attr_end + 2;
                continue;
            };
            let name_start = pos + attr_end + 2 + fn_rel + "fn ".len();
            let tail = &src[name_start..];
            let name: String = tail
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if name.is_empty() {
                cur = name_start;
                continue;
            }
            let file = attr_value(attr, "file");
            if file.is_empty() {
                cur = name_start + name.len();
                continue;
            }
            out.push(MlDocEntry {
                name: name.clone(),
                category: attr_value(attr, "category"),
                file,
                en: attr_value(attr, "en"),
                fr: attr_value(attr, "fr"),
                aliases: attr_block_before(&src, "sera_alias(", pos)
                    .map(|a| crate::build_common::extract_sera_aliases(&a))
                    .unwrap_or_default(),
                params: extract_fn_params(&src, name_start + name.len()),
                returns: Vec::new(),
                module: f.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string(),
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

pub fn write_registry(src_root: &Path, ml_root: &Path, data_root: &Path) {
    let mut docs = extract_ml_docs(ml_root);
    docs.extend(extract_data_docs(data_root));
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
        js.push_str("],module:\"");
        js.push_str(&crate::build_common::js_escape(&d.module));
        js.push_str("\"}");
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
    let path = src_root.join("docs").join("theme").join("ml-registry.js");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(path, js).expect("write ml-registry.js");
}
