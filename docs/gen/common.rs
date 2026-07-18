use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            if p.file_name().and_then(|s| s.to_str()) == Some("docgen") {
                continue;
            }
            walk(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
}

pub(crate) fn family_variant(file: &Path, plot_root: &Path) -> Option<(String, String)> {
    let rel = file.strip_prefix(plot_root).ok()?;
    let comps: Vec<String> = rel
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();
    if comps.len() < 2 {
        return None;
    }
    let last = comps.last().unwrap();
    let variant = last.strip_suffix(".rs")?.to_string();
    let parent = comps[comps.len() - 2].clone();
    if parent == "_3d" {
        return Some((variant, "basic".to_string()));
    }
    Some((parent, variant))
}

pub(crate) fn extract_kwargs(src: &str) -> Option<String> {
    let needle = "chart_demo(";
    let after = src.find(needle)? + needle.len();
    let rest = src[after..].trim_start().strip_prefix('"')?;
    let mut end = 0;
    let mut esc = false;
    for (i, c) in rest.char_indices() {
        if esc {
            esc = false;
            continue;
        }
        if c == '\\' {
            esc = true;
            continue;
        }
        if c == '"' {
            end = i;
            break;
        }
    }
    Some(rest[..end].to_string())
}

pub(crate) fn extract_params_required(src: &str) -> Option<Vec<String>> {
    let needle = "params(paramsList";
    let start = src.find(needle)? + needle.len();
    let rest = &src[start..];
    let lb = rest.find('[')?;
    let after_lb = &rest[lb + 1..];
    let rb = after_lb.find(']')?;
    let inner = &after_lb[..rb];
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut in_str = false;
    for c in inner.chars() {
        if c == '"' {
            in_str = !in_str;
            continue;
        }
        if !in_str {
            if c == ',' {
                let s = buf.trim().to_string();
                if !s.is_empty() {
                    out.push(s);
                }
                buf.clear();
            }
            continue;
        }
        buf.push(c);
    }
    let s = buf.trim().to_string();
    if !s.is_empty() {
        out.push(s);
    }
    Some(out)
}

pub(crate) fn extract_sera_aliases(src: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let needle = "sera_alias(";
    let mut cur = 0;
    while let Some(pos) = src[cur..].find(needle) {
        let start = cur + pos + needle.len();
        let rest = &src[start..];
        let mut end = 0;
        let mut depth = 1;
        let mut in_str = false;
        let mut esc = false;
        for (i, c) in rest.char_indices() {
            if esc {
                esc = false;
                continue;
            }
            if c == '\\' {
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
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                depth -= 1;
                if depth == 0 {
                    end = i;
                    break;
                }
            }
        }
        let inner = &rest[..end];
        let mut buf = String::new();
        let mut in_s = false;
        for c in inner.chars() {
            if c == '"' {
                if in_s {
                    let s = buf.trim().to_string();
                    if !s.is_empty() {
                        out.push(s);
                    }
                    buf.clear();
                }
                in_s = !in_s;
                continue;
            }
            if in_s {
                buf.push(c);
            }
        }
        cur = start + end;
    }
    out
}

fn auto_cfg_fields(src: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let bytes = src.as_bytes();
    let n = bytes.len();
    let needle = b"cfg.";
    let mut i = 0;
    while i + 4 <= n {
        if &bytes[i..i + 4] == needle {
            if i > 0 {
                let prev = bytes[i - 1];
                let alnum = prev.is_ascii_alphanumeric() || prev == b'_';
                if alnum {
                    i += 4;
                    continue;
                }
            }
            let mut j = i + 4;
            while j < n {
                let c = bytes[j];
                let id = c.is_ascii_alphanumeric() || c == b'_';
                if id {
                    j += 1;
                } else {
                    break;
                }
            }
            if j > i + 4 {
                if let Ok(name) = std::str::from_utf8(&bytes[i + 4..j]) {
                    let s = name.to_string();
                    if !out.contains(&s) {
                        out.push(s);
                    }
                }
            }
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

pub(crate) fn module_basename(file: &Path) -> Option<String> {
    file.file_stem()?.to_str().map(|s| s.to_string())
}

pub(crate) fn snake_to_camel(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut up = false;
    for c in s.chars() {
        if c == '_' {
            up = true;
            continue;
        }
        if up {
            out.extend(c.to_uppercase());
            up = false;
        } else {
            out.push(c);
        }
    }
    out
}

fn extract_attr_string_arg(attr_src: &str, marker: &str) -> Option<String> {
    let pos = attr_src.find(marker)? + marker.len();
    let tail = attr_src[pos..].trim_start();
    if !tail.starts_with('(') {
        return None;
    }
    let start_q = tail.find('"')? + 1;
    let rest = &tail[start_q..];
    let end_q = rest.find('"')?;
    let value = rest[..end_q].trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn extract_builder_target_between(src: &str) -> Option<String> {
    let pos = src.find("sera_builder")?;
    let tail = &src[pos..];
    let end = tail.find(']')? + 1;
    extract_attr_string_arg(&tail[..end], "sera_builder")
}

pub(crate) fn extract_alias_fn_pairs(src: &str) -> Vec<(Vec<String>, String)> {
    let mut out: Vec<(Vec<String>, String)> = Vec::new();
    let needle = "sera_alias(";
    let mut cur = 0;
    while let Some(pos) = src[cur..].find(needle) {
        let attr_start = cur + pos;
        let start = attr_start + needle.len();
        let rest = &src[start..];
        let mut end = 0;
        let mut depth = 1;
        let mut in_str = false;
        let mut esc = false;
        for (i, c) in rest.char_indices() {
            if esc {
                esc = false;
                continue;
            }
            if c == '\\' {
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
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                depth -= 1;
                if depth == 0 {
                    end = i;
                    break;
                }
            }
        }
        let inner = &rest[..end];
        let mut aliases: Vec<String> = Vec::new();
        let mut buf = String::new();
        let mut in_s = false;
        for c in inner.chars() {
            if c == '"' {
                if in_s {
                    let s = buf.trim().to_string();
                    if !s.is_empty() {
                        aliases.push(s);
                    }
                    buf.clear();
                }
                in_s = !in_s;
                continue;
            }
            if in_s {
                buf.push(c);
            }
        }
        let after = &src[start + end..];
        let fn_pos = after.find("pub fn ");
        let use_pos = after.find("pub use ");
        match (fn_pos, use_pos) {
            (Some(fp), Some(up)) if up < fp => {
                let use_start = start + end + up + 8;
                let tail = &src[use_start..];
                let stmt = tail.split(';').next().unwrap_or_default().trim();
                let name = if let Some(pos) = stmt.rfind(" as ") {
                    stmt[pos + 4..].trim().to_string()
                } else {
                    stmt.rsplit("::")
                        .next()
                        .unwrap_or_default()
                        .trim()
                        .to_string()
                };
                if !aliases.is_empty() && !name.is_empty() {
                    out.push((aliases, name));
                }
            }
            (Some(fp), _) => {
                let builder_target = extract_builder_target_between(&after[..fp]);
                let fn_start = start + end + fp + 7;
                let tail = &src[fn_start..];
                let mut j = 0;
                for (i, c) in tail.char_indices() {
                    let id = c.is_ascii_alphanumeric() || c == '_';
                    if id {
                        j = i + c.len_utf8();
                    } else {
                        break;
                    }
                }
                if j > 0 {
                    let name = builder_target.unwrap_or_else(|| tail[..j].to_string());
                    if !aliases.is_empty() {
                        out.push((aliases, name));
                    }
                }
            }
            (None, Some(up)) => {
                let use_start = start + end + up + 8;
                let tail = &src[use_start..];
                let stmt = tail.split(';').next().unwrap_or_default().trim();
                let name = if let Some(pos) = stmt.rfind(" as ") {
                    stmt[pos + 4..].trim().to_string()
                } else {
                    stmt.rsplit("::")
                        .next()
                        .unwrap_or_default()
                        .trim()
                        .to_string()
                };
                if !aliases.is_empty() && !name.is_empty() {
                    out.push((aliases, name));
                }
            }
            (None, None) => {}
        }
        cur = start + end;
    }
    out
}

pub(crate) fn js_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

pub(crate) fn extract_marker_fn_names(src: &str, marker: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = 0;
    while let Some(pos) = src[cur..].find(marker) {
        let attr_start = cur + pos;
        let rest = &src[attr_start..];
        let Some(bracket_end) = rest.find(']') else {
            break;
        };
        let after_attr = attr_start + bracket_end + 1;
        if let Some(target) = extract_attr_string_arg(&rest[..bracket_end + 1], marker) {
            out.push(target);
            cur = after_attr;
            continue;
        }
        let tail = &src[after_attr..];
        let fn_pos = tail.find("pub fn ");
        let use_pos = tail.find("pub use ");
        match (fn_pos, use_pos) {
            (Some(fp), Some(up)) if up < fp => {
                let use_start = after_attr + up + 8;
                let stmt = src[use_start..]
                    .split(';')
                    .next()
                    .unwrap_or_default()
                    .trim();
                let name = if let Some(pos) = stmt.rfind(" as ") {
                    stmt[pos + 4..].trim().to_string()
                } else {
                    stmt.rsplit("::")
                        .next()
                        .unwrap_or_default()
                        .trim()
                        .to_string()
                };
                if !name.is_empty() {
                    out.push(name);
                }
            }
            (Some(fp), _) => {
                let fn_start = after_attr + fp + 7;
                let tail = &src[fn_start..];
                let mut j = 0;
                for (i, c) in tail.char_indices() {
                    let id = c.is_ascii_alphanumeric() || c == '_';
                    if id {
                        j = i + c.len_utf8();
                    } else {
                        break;
                    }
                }
                if j > 0 {
                    out.push(tail[..j].to_string());
                }
            }
            (None, Some(up)) => {
                let use_start = after_attr + up + 8;
                let stmt = src[use_start..]
                    .split(';')
                    .next()
                    .unwrap_or_default()
                    .trim();
                let name = if let Some(pos) = stmt.rfind(" as ") {
                    stmt[pos + 4..].trim().to_string()
                } else {
                    stmt.rsplit("::")
                        .next()
                        .unwrap_or_default()
                        .trim()
                        .to_string()
                };
                if !name.is_empty() {
                    out.push(name);
                }
            }
            (None, None) => {}
        }
        cur = after_attr;
    }
    out.sort();
    out.dedup();
    out
}

#[allow(dead_code)]
pub(crate) fn extract_pyclass_names(src: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let needle = "#[pyclass";
    let mut cur = 0;
    while let Some(pos) = src[cur..].find(needle) {
        let attr_start = cur + pos;
        let rest = &src[attr_start..];
        let Some(bracket_end) = rest.find(']') else {
            break;
        };
        let after_attr = attr_start + bracket_end + 1;
        let tail = &src[after_attr..];
        let Some(struct_pos) = tail.find("pub struct ").or_else(|| tail.find("struct ")) else {
            cur = after_attr;
            continue;
        };
        let struct_start = after_attr
            + struct_pos
            + if tail[struct_pos..].starts_with("pub struct ") {
                11
            } else {
                7
            };
        let name_tail = &src[struct_start..];
        let mut end = 0;
        for (i, c) in name_tail.char_indices() {
            if c.is_ascii_alphanumeric() || c == '_' {
                end = i + c.len_utf8();
            } else {
                break;
            }
        }
        if end > 0 {
            out.push(name_tail[..end].to_string());
        }
        cur = after_attr;
    }
    out.sort();
    out.dedup();
    out
}

#[allow(dead_code)]
pub(crate) fn extract_pyfunction_names(src: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let needle = "#[pyfunction]";
    let mut cur = 0;
    while let Some(pos) = src[cur..].find(needle) {
        let attr_start = cur + pos;
        let rest = &src[attr_start..];
        let Some(bracket_end) = rest.find(']') else {
            break;
        };
        let after_attr = attr_start + bracket_end + 1;
        let tail = &src[after_attr..];
        let Some(fn_pos) = tail.find("fn ").or_else(|| tail.find("pub fn ")) else {
            cur = after_attr;
            continue;
        };
        let fn_start = after_attr
            + fn_pos
            + if tail[fn_pos..].starts_with("pub fn ") {
                7
            } else {
                3
            };
        let name_tail = &src[fn_start..];
        let mut end = 0;
        for (i, c) in name_tail.char_indices() {
            if c.is_ascii_alphanumeric() || c == '_' {
                end = i + c.len_utf8();
            } else {
                break;
            }
        }
        if end > 0 {
            out.push(name_tail[..end].to_string());
        }
        cur = after_attr;
    }
    out.sort();
    out.dedup();
    out
}

pub(crate) fn struct_field_names(src: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let needle = "pub ";
    let mut cur = 0;
    while let Some(pos) = src[cur..].find(needle) {
        let start = cur + pos + needle.len();
        let rest = &src[start..];
        let mut end = 0;
        for (i, c) in rest.char_indices() {
            if c.is_ascii_alphanumeric() || c == '_' {
                end = i + c.len_utf8();
            } else {
                break;
            }
        }
        cur = start + end.max(1);
        if end == 0 {
            continue;
        }
        let name = &rest[..end];
        let after = rest[end..].trim_start();
        if after.starts_with(':') && !after.starts_with("::") {
            out.push(name.to_string());
        }
    }
    out.sort();
    out.dedup();
    out
}

pub(crate) fn config_field_names(dir: &Path) -> Vec<String> {
    fs::read_to_string(dir.join("config.rs"))
        .map(|s| struct_field_names(&s))
        .unwrap_or_default()
}

pub(crate) fn filtered_auto_fields(src: &str, allowed: &[String]) -> Vec<String> {
    auto_cfg_fields(src)
        .into_iter()
        .filter(|s| allowed.contains(s))
        .collect()
}

