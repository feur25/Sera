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

fn scan_top_level_pairs(src: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut chars = src.char_indices().peekable();
    while let Some((start, c)) = chars.next() {
        if !(c.is_ascii_alphabetic() || c == '_') {
            continue;
        }
        let mut end = start + c.len_utf8();
        while let Some(&(i, c2)) = chars.peek() {
            if c2.is_ascii_alphanumeric() || c2 == '_' {
                end = i + c2.len_utf8();
                chars.next();
            } else {
                break;
            }
        }
        let name = &src[start..end];
        let after = src[end..].trim_start();
        if !after.starts_with(':') || after.starts_with("::") {
            continue;
        }
        let val_start = end + (src[end..].len() - after.len()) + 1;
        let mut depth = 0i32;
        let mut in_str = false;
        let mut esc = false;
        let mut val_end = src.len();
        let bytes = src.as_bytes();
        let mut j = val_start;
        while j < bytes.len() {
            let b = bytes[j];
            if esc {
                esc = false;
                j += 1;
                continue;
            }
            match b {
                b'\\' if in_str => esc = true,
                b'"' => in_str = !in_str,
                b'(' | b'[' | b'{' | b'<' if !in_str => depth += 1,
                b')' | b']' | b'}' | b'>' if !in_str => {
                    if depth == 0 {
                        val_end = j;
                        break;
                    }
                    depth -= 1;
                }
                b',' if !in_str && depth == 0 => {
                    val_end = j;
                    break;
                }
                _ => {}
            }
            j += 1;
        }
        if val_end > src.len() {
            val_end = src.len();
        }
        out.push((name.to_string(), src[val_start..val_end].trim().to_string()));
        while chars.peek().map(|&(i, _)| i < val_end).unwrap_or(false) {
            chars.next();
        }
    }
    out
}

fn is_collection_type(ty: &str) -> bool {
    let t = ty.trim();
    t.contains('[') || t.starts_with("Vec<") || t.starts_with("Vec ")
}

fn is_empty_default(expr: &str) -> bool {
    matches!(expr.trim(), "&[]" | "[]" | "Vec::new()" | "vec![]" | "&Vec::new()")
}

const NON_DATA_COLLECTION_FIELDS: &[&str] = &[
    "hover",
    "palette",
    "color_groups",
    "thresholds",
    "pull",
    "x_widths",
    "y_heights",
    "row_totals",
    "col_totals",
    "secondary_matrix",
    "mask",
    "points_x",
    "points_y",
    "point_clusters",
    "cluster_labels",
    "edges_i",
    "edges_j",
    "edges_w",
    "ranges",
    "comparisons",
];

fn braced_block_after(src: &str, marker_pos: usize) -> Option<(usize, usize)> {
    let brace_pos = marker_pos + src[marker_pos..].find('{')?;
    let end = matching_brace(src, brace_pos)?;
    Some((brace_pos + 1, end))
}

fn required_from_type_default_pairs(
    types: &std::collections::HashMap<String, String>,
    defaults: &[(String, String)],
) -> Vec<String> {
    let mut out = Vec::new();
    for (name, default_expr) in defaults {
        if NON_DATA_COLLECTION_FIELDS.contains(&name.as_str()) {
            continue;
        }
        let Some(ty) = types.get(name) else { continue };
        if is_collection_type(ty) && is_empty_default(default_expr) {
            out.push(name.clone());
        }
    }
    out.sort();
    out
}

fn required_data_fields_macro_form(src: &str) -> Option<Vec<String>> {
    let macro_pos = src.find("chart_config!")?;
    let struct_kw = macro_pos + src[macro_pos..].find("struct")?;
    let (struct_start, struct_end) = braced_block_after(src, struct_kw)?;
    let types: std::collections::HashMap<String, String> =
        scan_top_level_pairs(&src[struct_start..struct_end]).into_iter().collect();

    let defaults_kw = struct_end + src[struct_end..].find("defaults")?;
    let (def_start, def_end) = braced_block_after(src, defaults_kw)?;
    let defaults = scan_top_level_pairs(&src[def_start..def_end]);

    Some(required_from_type_default_pairs(&types, &defaults))
}

fn required_data_fields_verbose_form(src: &str) -> Option<Vec<String>> {
    let struct_start = src.find("struct ")?;
    let (struct_body_start, struct_body_end) = braced_block_after(src, struct_start)?;
    let types: std::collections::HashMap<String, String> =
        scan_top_level_pairs(&src[struct_body_start..struct_body_end]).into_iter().collect();

    let default_fn = src.find("fn default()")?;
    let self_kw = default_fn + src[default_fn..].find("Self")?;
    let (self_body_start, self_body_end) = braced_block_after(src, self_kw)?;
    let defaults = scan_top_level_pairs(&src[self_body_start..self_body_end]);

    Some(required_from_type_default_pairs(&types, &defaults))
}

pub(crate) fn required_data_fields(dir: &Path) -> Vec<String> {
    let Ok(src) = fs::read_to_string(dir.join("config.rs")) else {
        return Vec::new();
    };
    required_data_fields_macro_form(&src)
        .or_else(|| required_data_fields_verbose_form(&src))
        .unwrap_or_default()
}

fn matching_brace(src: &str, open_pos: usize) -> Option<usize> {
    let bytes = src.as_bytes();
    if bytes.get(open_pos) != Some(&b'{') {
        return None;
    }
    let mut depth = 0i32;
    let mut in_str = false;
    let mut esc = false;
    let mut i = open_pos;
    while i < bytes.len() {
        let b = bytes[i];
        if esc {
            esc = false;
            i += 1;
            continue;
        }
        match b {
            b'\\' if in_str => esc = true,
            b'"' => in_str = !in_str,
            b'{' if !in_str => depth += 1,
            b'}' if !in_str => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

