use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
}

fn family_variant(file: &Path, plot_root: &Path) -> Option<(String, String)> {
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
    let family = comps[comps.len() - 2].clone();
    Some((family, variant))
}

fn extract_kwargs(src: &str) -> Option<String> {
    let needle = "chart_demo(\"";
    let start = src.find(needle)? + needle.len();
    let rest = &src[start..];
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

fn extract_params_required(src: &str) -> Option<Vec<String>> {
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

fn extract_sera_aliases(src: &str) -> Vec<String> {
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
                let alnum = (prev >= b'a' && prev <= b'z')
                    || (prev >= b'A' && prev <= b'Z')
                    || (prev >= b'0' && prev <= b'9')
                    || prev == b'_';
                if alnum {
                    i += 4;
                    continue;
                }
            }
            let mut j = i + 4;
            while j < n {
                let c = bytes[j];
                let id = (c >= b'a' && c <= b'z')
                    || (c >= b'A' && c <= b'Z')
                    || (c >= b'0' && c <= b'9')
                    || c == b'_';
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

fn module_basename(file: &Path) -> Option<String> {
    file.file_stem()?.to_str().map(|s| s.to_string())
}

fn snake_to_camel(s: &str) -> String {
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

fn extract_alias_fn_pairs(src: &str) -> Vec<(Vec<String>, String)> {
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

fn extract_marker_fn_names(src: &str, marker: &str) -> Vec<String> {
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
fn extract_pyclass_names(src: &str) -> Vec<String> {
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
fn extract_pyfunction_names(src: &str) -> Vec<String> {
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

const PARAMS_BLACKLIST: &[&str] = &[
    "buf",
    "pl",
    "pt",
    "pb",
    "pw",
    "ph",
    "len",
    "is_empty",
    "iter",
    "as_str",
    "to_vec",
    "to_string",
    "max",
    "min",
    "clone",
    "into",
];

const PUB_USE_FN_ALLOWLIST: &[&str] = &[
    "plot_chart",
    "themes",
    "build_slideshow",
    "build_hover_json",
    "chart_append",
    "export_svg",
    "export_data_url",
    "export_html_file",
    "chart_info",
    "validate_input",
    "downsample_lttb",
    "chart_diff",
    "drift_ks",
    "scale_plan",
    "system_profile",
    "csv_count_rows",
    "csv_chunk_read",
];

fn keep_pub_use_fn(name: &str) -> bool {
    name.starts_with("build_") || PUB_USE_FN_ALLOWLIST.contains(&name)
}

fn split_top_level_csv(input: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0usize;
    let mut depth = 0i32;
    for (idx, ch) in input.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => depth -= 1,
            ',' if depth == 0 => {
                out.push(input[start..idx].trim());
                start = idx + ch.len_utf8();
            }
            _ => {}
        }
    }
    out.push(input[start..].trim());
    out.into_iter().filter(|part| !part.is_empty()).collect()
}

fn collect_pub_use_names(stmt: &str, out: &mut Vec<String>) {
    let stmt = stmt.trim();
    if stmt.is_empty() {
        return;
    }
    if let (Some(lb), Some(rb)) = (stmt.find('{'), stmt.rfind('}')) {
        let inner = &stmt[lb + 1..rb];
        for part in split_top_level_csv(inner) {
            collect_pub_use_names(part, out);
        }
        return;
    }
    let name = if let Some(pos) = stmt.rfind(" as ") {
        stmt[pos + 4..].trim()
    } else {
        stmt.rsplit("::").next().unwrap_or_default().trim()
    };
    if !name.is_empty()
        && keep_pub_use_fn(name)
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        out.push(name.to_string());
    }
}

fn extract_input_str_fns(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in src.lines() {
        let t = line.trim_start();
        if t.starts_with("pub use ") {
            let stmt = t[8..].trim();
            let stmt = stmt.split(';').next().unwrap_or_default().trim();
            collect_pub_use_names(stmt, &mut out);
            continue;
        }
        if !t.starts_with("pub fn ") {
            continue;
        }
        let after = &t[7..];
        let Some(paren) = after.find('(') else {
            continue;
        };
        let name = &after[..paren];
        if name.is_empty() {
            continue;
        }
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            continue;
        }
        let rest = &after[paren..];
        let has_str_input = rest.contains(": &str)")
            || rest.contains(": &str,")
            || rest.contains("_: &str)")
            || rest.contains("_: &str,");
        if !has_str_input {
            continue;
        }
        if !rest.contains("-> String") {
            continue;
        }
        out.push(name.to_string());
    }
    out.sort();
    out.dedup();
    out
}

const PYTHON_CUSTOM_WRAPPED: &[&str] = &[
    "build_grid",
    "grid",
    "build_sysmon",
    "sysmon",
    "build_slideshow",
    "build_hover_json",
    "plot_chart",
    "plot",
    "set_bg",
    "show_chart_value",
    "bench_chart_value",
    "set_chart_kind",
    "set_chart_orientation",
    "bench_pure_rust",
    "push_telemetry",
    "export_svg",
    "export_data_url",
    "chart_info",
];

fn filtered_auto_fields(src: &str) -> Vec<String> {
    let raw = auto_cfg_fields(src);
    raw.into_iter()
        .filter(|s| {
            !PARAMS_BLACKLIST.contains(&s.as_str())
                && !s.is_empty()
                && !s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(true)
        })
        .collect()
}

#[allow(dead_code)]
struct FnSpec<'a> {
    name: &'a str,
    module: &'a str,
    is_chart: bool,
}

#[allow(dead_code)]
fn gen_python_chart(f: &str) -> String {
    let w = format!("__gen_py_{}", f);
    let r = "crate::_py::PyFnEntry";
    let mut s = String::new();
    s.push_str(&format!("#[pyo3::pyfunction(name = \"{}\")]\n", f));
    s.push_str(
        "#[pyo3(signature = (title=\"\", labels=None, values=None, theme=None, **kwargs))]\n",
    );
    s.push_str(&format!("pub fn {}(\n", w));
    s.push_str("    title: &str,\n");
    s.push_str("    labels: Option<&pyo3::Bound<'_, pyo3::types::PyAny>>,\n");
    s.push_str("    values: Option<&pyo3::Bound<'_, pyo3::types::PyAny>>,\n");
    s.push_str("    theme: Option<&str>,\n");
    s.push_str("    kwargs: Option<&pyo3::Bound<'_, pyo3::types::PyDict>>,\n");
    s.push_str(") -> crate::Chart {\n");
    s.push_str("    let __t = std::time::Instant::now();\n");
    s.push_str(
        "    let __p = crate::_py::python_py_args_to_json(title, labels, values, theme, kwargs);\n",
    );
    s.push_str(&format!(
        "    let __h = crate::bindings::commands::charts::{}(&__p);\n",
        f
    ));
    s.push_str("    let __dc = labels.and_then(|l| l.len().ok()).unwrap_or(0) as u64;\n");
    s.push_str(&format!("    let mut __ev = crate::telemetry::TelemetryEvent::new(\"{}\", __t.elapsed().as_secs_f64() * 1000.0);\n", f));
    s.push_str("    if __dc > 0 { __ev = __ev.with_data(__dc, 0.0); }\n");
    s.push_str("    crate::telemetry::record(__ev);\n");
    s.push_str("    crate::_py::python_chart_from_html(__h, \"\")\n");
    s.push_str("}\n");
    s.push_str(&format!("inventory::submit! {{ {} {{ register: |m| Ok(m.add_function(pyo3::wrap_pyfunction!({}, m)?)?) }} }}\n\n", r, w));
    s
}

#[allow(dead_code)]
fn gen_python_str(f: &str, module: &str) -> String {
    let w = format!("__gen_py_{}", f);
    let r = "crate::_py::PyFnEntry";
    let mut s = String::new();
    s.push_str(&format!("#[pyo3::pyfunction(name = \"{}\")]\n", f));
    s.push_str(&format!(
        "pub fn {}(input: &str) -> String {{ crate::{}::{}(input) }}\n",
        w, module, f
    ));
    s.push_str(&format!("inventory::submit! {{ {} {{ register: |m| Ok(m.add_function(pyo3::wrap_pyfunction!({}, m)?)?) }} }}\n\n", r, w));
    s
}

fn gen_wasm(f: &str, js: &str, module: &str) -> String {
    let mut s = String::new();
    s.push_str("#[cfg(all(feature = \"js\", target_arch = \"wasm32\"))]\n");
    s.push_str(&format!(
        "#[wasm_bindgen::prelude::wasm_bindgen(js_name = \"{}\")]\n",
        js
    ));
    s.push_str(&format!(
        "pub fn __wasm_{}(input: String) -> String {{ crate::{}::{}(&input) }}\n\n",
        f, module, f
    ));
    s
}

fn gen_ffi(f: &str, module: &str) -> String {
    let mut s = String::new();
    s.push_str("#[cfg(feature = \"ffi\")]\n#[no_mangle]\n");
    s.push_str(&format!("pub unsafe extern \"C\" fn sera_{}(input: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {{\n", f));
    s.push_str(
        "    let s = unsafe { std::ffi::CStr::from_ptr(input).to_str().unwrap_or(\"\") };\n",
    );
    s.push_str(&format!(
        "    std::ffi::CString::new(crate::{}::{}(s)).unwrap_or_default().into_raw()\n}}\n\n",
        module, f
    ));
    s
}

fn should_emit_ffi(name: &str) -> bool {
    !matches!(
        name,
        "reset_global_background"
            | "set_global_background"
            | "reset_theme"
            | "set_theme"
            | "themes"
    )
}

fn pascal_case(s: &str) -> String {
    s.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().to_string() + c.as_str(),
            }
        })
        .collect()
}

#[allow(dead_code)]
fn gen_csharp(fns: &[FnSpec<'_>]) -> String {
    let mut s = String::new();
    s.push_str("using System.Runtime.InteropServices;\n");
    s.push_str("namespace SeraPlot {\n");
    s.push_str("    public static partial class Api {\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"seraplot_free\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern void Free(IntPtr ptr);\n\n");
    s.push_str("        private static string Call(IntPtr ptr) {\n");
    s.push_str("            var r = Marshal.PtrToStringUTF8(ptr) ?? string.Empty;\n");
    s.push_str("            Free(ptr); return r;\n");
    s.push_str("        }\n\n");
    for f in fns {
        let p = pascal_case(f.name);
        s.push_str(&format!("        [DllImport(\"seraplot\", EntryPoint = \"sera_{}\", CallingConvention = CallingConvention.Cdecl)]\n", f.name));
        s.push_str(&format!("        private static extern IntPtr _{}([MarshalAs(UnmanagedType.LPUTF8Str)] string input);\n", f.name));
        s.push_str(&format!(
            "        public static string {}(string input) => Call(_{}(input));\n\n",
            p, f.name
        ));
    }
    s.push_str("    }\n}\n");
    s
}

#[allow(dead_code)]
fn gen_typescript(fns: &[FnSpec<'_>]) -> String {
    let mut s = String::new();
    s.push_str("export interface SeraPlot {\n");
    for f in fns {
        s.push_str(&format!(
            "  {}(input: string): string;\n",
            snake_to_camel(f.name)
        ));
    }
    s.push_str("}\n");
    s.push_str("export declare const seraplot: SeraPlot;\n");
    s
}

#[allow(dead_code)]
fn gen_go(fns: &[FnSpec<'_>]) -> String {
    let mut s = String::new();
    s.push_str("package seraplot\n\n");
    s.push_str("/*\n#cgo LDFLAGS: -lseraplot\n#include <stdlib.h>\n");
    s.push_str("extern void seraplot_free(char*);\n");
    for f in fns {
        s.push_str(&format!(
            "extern char* sera_{}(const char* input);\n",
            f.name
        ));
    }
    s.push_str("*/\nimport \"C\"\nimport \"unsafe\"\n\n");
    s.push_str("func freeStr(p *C.char) { C.seraplot_free(p) }\n\n");
    for f in fns {
        let g = pascal_case(f.name);
        s.push_str(&format!("func {}(input string) string {{\n", g));
        s.push_str("    cs := C.CString(input); defer C.free(unsafe.Pointer(cs))\n");
        s.push_str(&format!(
            "    r := C.sera_{}(cs); defer freeStr(r)\n",
            f.name
        ));
        s.push_str("    return C.GoString(r)\n}\n\n");
    }
    s
}

#[allow(dead_code)]
fn gen_cpp_header(fns: &[FnSpec<'_>]) -> String {
    let mut s = String::new();
    s.push_str("#pragma once\n#include <string>\nextern \"C\" {\n");
    s.push_str("    void seraplot_free(char* ptr);\n");
    for f in fns {
        s.push_str(&format!("    char* sera_{}(const char* input);\n", f.name));
    }
    s.push_str("}\nnamespace seraplot {\n");
    s.push_str("    inline void free_str(char* p) { seraplot_free(p); }\n");
    for f in fns {
        let c = snake_to_camel(f.name);
        s.push_str(&format!(
            "    inline std::string {}(const std::string& input) {{\n",
            c
        ));
        s.push_str(&format!(
            "        char* r = sera_{}(input.c_str());\n",
            f.name
        ));
        s.push_str("        std::string out(r); free_str(r); return out;\n    }\n");
    }
    s.push_str("}\n");
    s
}

#[allow(dead_code)]
fn read_file(path: &std::path::Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

fn generate_adapters(chart_fns: &[String], ml_fns: &[String], util_fns: &[String]) -> String {
    let charts = "bindings::commands::charts";
    let ml = "bindings::commands::ml";

    let mut wasm_body = String::new();
    wasm_body.push_str("use wasm_bindgen::prelude::*;\n");
    for n in chart_fns {
        wasm_body.push_str(&gen_wasm(n, &snake_to_camel(n), charts));
    }
    for n in util_fns {
        wasm_body.push_str(&gen_wasm(n, &snake_to_camel(n), charts));
    }
    for n in ml_fns {
        wasm_body.push_str(&gen_wasm(n, &snake_to_camel(n), ml));
    }
    wasm_body.push_str("#[wasm_bindgen(js_name = \"demo\")]\n");
    wasm_body.push_str("pub fn wasm_demo(input: &str) -> String {\n");
    wasm_body.push_str("    #[derive(serde::Deserialize, Default)] struct In { family: Option<String>, variant: Option<String> }\n");
    wasm_body.push_str("    let i: In = serde_json::from_str(input).unwrap_or_default();\n");
    wasm_body.push_str("    crate::demo_snippet(&i.family.unwrap_or_default(), &i.variant.unwrap_or_else(|| \"basic\".to_string())).unwrap_or_default()\n");
    wasm_body.push_str("}\n");
    wasm_body.push_str("#[wasm_bindgen(js_name = \"chartVariants\")]\n");
    wasm_body.push_str("pub fn wasm_chart_variants_json() -> String {\n");
    wasm_body.push_str("    let mut out: std::collections::BTreeMap<&str, Vec<&str>> = Default::default();\n");
    wasm_body.push_str("    for (family, variant, _) in crate::DEMO_REGISTRY.iter() {\n");
    wasm_body.push_str("        out.entry(family).or_default().push(variant);\n");
    wasm_body.push_str("    }\n");
    wasm_body.push_str("    serde_json::to_string(&out).unwrap_or_default()\n");
    wasm_body.push_str("}\n");

    let mut ffi_body = String::new();
    for n in chart_fns {
        if should_emit_ffi(n) {
            ffi_body.push_str(&gen_ffi(n, charts));
        }
    }
    for n in util_fns {
        if should_emit_ffi(n) {
            ffi_body.push_str(&gen_ffi(n, charts));
        }
    }
    for n in ml_fns {
        if should_emit_ffi(n) {
            ffi_body.push_str(&gen_ffi(n, ml));
        }
    }
    ffi_body.push_str("#[no_mangle]\npub unsafe extern \"C\" fn seraplot_free(ptr: *mut std::os::raw::c_char) {\n");
    ffi_body.push_str("    if !ptr.is_null() { drop(std::ffi::CString::from_raw(ptr)); }\n}\n");

    let mut out = String::new();
    out.push_str("#[cfg(all(feature = \"js\", target_arch = \"wasm32\"))]\npub mod _wasm {\n");
    for line in wasm_body.lines() {
        out.push_str("    ");
        out.push_str(line);
        out.push('\n');
    }
    out.push_str("}\n\n");

    out.push_str("#[cfg(feature = \"ffi\")]\npub mod _ffi {\n");
    for line in ffi_body.lines() {
        out.push_str("    ");
        out.push_str(line);
        out.push('\n');
    }
    out.push_str("}\n");

    out
}

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let plot_root = manifest.join("src").join("plot");
    let bindings_root = manifest.join("src").join("bindings");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=src/plot");
    println!("cargo:rerun-if-changed=src/bindings");

    let mut files: Vec<PathBuf> = Vec::new();
    walk(&plot_root, &mut files);
    files.sort();

    let mut bindings_files: Vec<PathBuf> = Vec::new();
    walk(&bindings_root, &mut bindings_files);
    bindings_files.sort();

    let mut demo_entries: Vec<(String, String, String)> = Vec::new();
    let mut param_entries: Vec<(String, String, Vec<String>)> = Vec::new();
    let mut alias_entries: Vec<(String, Vec<String>)> = Vec::new();

    for f in &files {
        let Ok(src) = fs::read_to_string(f) else {
            continue;
        };
        let Some((family, variant)) = family_variant(f, &plot_root) else {
            continue;
        };
        if variant == "mod" || variant == "common" || variant == "config" || variant == "shared" {
            continue;
        }

        if let Some(kw) = extract_kwargs(&src) {
            demo_entries.push((family.clone(), variant.clone(), kw));
        }

        let params = if let Some(explicit) = extract_params_required(&src) {
            explicit
        } else {
            filtered_auto_fields(&src)
        };
        if !params.is_empty() {
            param_entries.push((family.clone(), variant.clone(), params));
        }

        let aliases = extract_sera_aliases(&src);
        if !aliases.is_empty() {
            if let Some(base) = module_basename(f) {
                alias_entries.push((format!("{}::{}", family, base), aliases));
            }
        }
    }

    demo_entries.sort();
    param_entries.sort();
    alias_entries.sort();

    let mut chart_alias_pairs: Vec<(String, String)> = Vec::new();
    let mut builder_fns: Vec<String> = Vec::new();
    for f in files.iter().chain(bindings_files.iter()) {
        let Ok(src) = fs::read_to_string(f) else {
            continue;
        };
        if f.starts_with(&plot_root) {
            builder_fns.extend(extract_marker_fn_names(&src, "sera_builder"));
        }
        for (aliases, fn_name) in extract_alias_fn_pairs(&src) {
            for a in aliases {
                chart_alias_pairs.push((a, fn_name.clone()));
            }
        }
    }
    chart_alias_pairs.sort();
    chart_alias_pairs.dedup();
    builder_fns.sort();
    builder_fns.dedup();

    let ml_pyclasses: Vec<String> = Vec::new();
    let ml_pyfunctions: Vec<String> = Vec::new();

    let mut body = String::from("pub static DEMO_REGISTRY: &[(&str, &str, &str)] = &[\n");
    for (f, v, k) in &demo_entries {
        body.push_str("    (\"");
        body.push_str(f);
        body.push_str("\", \"");
        body.push_str(v);
        body.push_str("\", \"");
        body.push_str(k);
        body.push_str("\"),\n");
    }
    body.push_str("];\n");
    fs::write(out_dir.join("demo_registry.rs"), body).expect("write demo_registry.rs");

    let mut pbody = String::from("pub static PARAMS_REGISTRY: &[(&str, &str, &[&str])] = &[\n");
    for (f, v, ps) in &param_entries {
        pbody.push_str("    (\"");
        pbody.push_str(f);
        pbody.push_str("\", \"");
        pbody.push_str(v);
        pbody.push_str("\", &[");
        for (i, p) in ps.iter().enumerate() {
            if i > 0 {
                pbody.push_str(", ");
            }
            pbody.push('"');
            pbody.push_str(p);
            pbody.push('"');
        }
        pbody.push_str("]),\n");
    }
    pbody.push_str("];\n");
    fs::write(out_dir.join("params_registry.rs"), pbody).expect("write params_registry.rs");

    let mut abody = String::from("pub static SERA_ALIASES: &[(&str, &[&str])] = &[\n");
    for (k, al) in &alias_entries {
        abody.push_str("    (\"");
        abody.push_str(k);
        abody.push_str("\", &[");
        for (i, a) in al.iter().enumerate() {
            if i > 0 {
                abody.push_str(", ");
            }
            abody.push('"');
            abody.push_str(a);
            abody.push('"');
        }
        abody.push_str("]),\n");
    }
    abody.push_str("];\n");
    fs::write(out_dir.join("sera_aliases.rs"), abody).expect("write sera_aliases.rs");

    let mut cbody = String::from("pub static CHART_ALIAS_REGISTRY: &[(&str, &str)] = &[\n");
    for (a, fname) in &chart_alias_pairs {
        cbody.push_str("    (\"");
        cbody.push_str(a);
        cbody.push_str("\", \"");
        cbody.push_str(fname);
        cbody.push_str("\"),\n");
    }
    cbody.push_str("];\n");
    fs::write(out_dir.join("chart_alias_registry.rs"), cbody)
        .expect("write chart_alias_registry.rs");

    let charts_path = bindings_root.join("commands").join("charts.rs");
    let charts_src = if charts_path.exists() {
        fs::read_to_string(&charts_path).unwrap_or_default()
    } else {
        let charts_dir = bindings_root.join("commands").join("charts");
        if charts_dir.is_dir() {
            println!("cargo:rerun-if-changed=src/bindings/commands/charts");
            let mut src = String::new();
            let mut chart_files: Vec<PathBuf> = Vec::new();
            walk(&charts_dir, &mut chart_files);
            chart_files.sort();
            for f in &chart_files {
                if let Ok(s) = fs::read_to_string(f) {
                    src.push_str(&s);
                    src.push('\n');
                }
            }
            src
        } else {
            String::new()
        }
    };
    let ml_dir = bindings_root.join("commands").join("ml");
    let ml_src = if ml_dir.is_dir() {
        let mut src = String::new();
        let mut ml_files: Vec<PathBuf> = Vec::new();
        walk(&ml_dir, &mut ml_files);
        ml_files.sort();
        for f in &ml_files {
            if let Ok(s) = fs::read_to_string(f) {
                src.push_str(&s);
                src.push('\n');
            }
        }
        src
    } else {
        let ml_path = bindings_root.join("commands").join("ml.rs");
        fs::read_to_string(&ml_path).unwrap_or_default()
    };
    let all_fns = extract_input_str_fns(&charts_src);
    let plot_utils_path = manifest.join("src").join("plot").join("utils.rs");
    let plot_utils_src = fs::read_to_string(&plot_utils_path).unwrap_or_default();
    println!("cargo:rerun-if-changed=src/plot/utils.rs");
    let plot_util_fns = extract_input_str_fns(&plot_utils_src);
    let ml_fns_vec = extract_input_str_fns(&ml_src);
    let mut chart_fns: Vec<String> = Vec::new();
    let ml_fns: Vec<String> = ml_fns_vec;
    let mut util_fns: Vec<String> = Vec::new();
    let mut auto_util_fns: Vec<String> = Vec::new();
    for n in &builder_fns {
        if PYTHON_CUSTOM_WRAPPED.contains(&n.as_str()) {
            util_fns.push(n.clone());
        } else {
            chart_fns.push(n.clone());
        }
    }
    for n in &all_fns {
        if PYTHON_CUSTOM_WRAPPED.contains(&n.as_str()) {
            if !util_fns.contains(n) {
                util_fns.push(n.clone());
            }
        } else {
            if !util_fns.contains(n) {
                util_fns.push(n.clone());
            }
            if !auto_util_fns.contains(n) {
                auto_util_fns.push(n.clone());
            }
        }
    }
    for n in &plot_util_fns {
        if !util_fns.contains(n) {
            if PYTHON_CUSTOM_WRAPPED.contains(&n.as_str()) {
                util_fns.push(n.clone());
            } else {
                util_fns.push(n.clone());
                auto_util_fns.push(n.clone());
            }
        }
    }

    let emit_macro = |name: &str, fns: &[String]| -> String {
        let mut s = format!(
            "#[allow(unused_macros)]\nmacro_rules! {} {{\n    ($mac:ident) => {{\n",
            name
        );
        for n in fns {
            s.push_str(&format!(
                "        $mac!({}, \"{}\");\n",
                n,
                snake_to_camel(n)
            ));
        }
        s.push_str("    };\n}\n#[allow(unused_imports)]\npub(crate) use ");
        s.push_str(name);
        s.push_str(";\n");
        s
    };

    let emit_class_macro = |name: &str, classes: &[String]| -> String {
        let mut s = format!(
            "#[allow(unused_macros)]\nmacro_rules! {} {{\n    ($mac:ident) => {{\n",
            name
        );
        for n in classes {
            s.push_str(&format!("        $mac!({});\n", n));
        }
        s.push_str("    };\n}\n#[allow(unused_imports)]\npub(crate) use ");
        s.push_str(name);
        s.push_str(";\n");
        s
    };

    let adapters = generate_adapters(&chart_fns, &ml_fns, &auto_util_fns);
    fs::write(out_dir.join("adapters.rs"), adapters).expect("write adapters.rs");

    let all_fns: Vec<FnSpec<'_>> = chart_fns
        .iter()
        .filter(|n| should_emit_ffi(n))
        .map(|n| FnSpec {
            name: n.as_str(),
            module: "bindings::commands::charts",
            is_chart: true,
        })
        .chain(
            auto_util_fns
                .iter()
                .filter(|n| should_emit_ffi(n))
                .map(|n| FnSpec {
                    name: n.as_str(),
                    module: "bindings::commands::charts",
                    is_chart: false,
                }),
        )
        .chain(
            ml_fns
                .iter()
                .filter(|n| should_emit_ffi(n))
                .map(|n| FnSpec {
                    name: n.as_str(),
                    module: "bindings::commands::ml",
                    is_chart: false,
                }),
        )
        .collect();
    let _ = all_fns;

    let chart_py_wrappers: Vec<String> = chart_fns
        .iter()
        .map(|n| format!("__sera_py_{}", n))
        .collect();

    fs::write(
        out_dir.join("chart_fn_macro.rs"),
        emit_macro("for_each_json_chart_fn", &chart_fns),
    )
    .expect("write chart_fn_macro.rs");
    fs::write(
        out_dir.join("chart_py_wrapper_macro.rs"),
        emit_class_macro("for_each_json_chart_py_wrapper_fn", &chart_py_wrappers),
    )
    .expect("write chart_py_wrapper_macro.rs");
    fs::write(
        out_dir.join("ml_fn_macro.rs"),
        emit_macro("for_each_ml_oneshot_fn", &ml_fns),
    )
    .expect("write ml_fn_macro.rs");
    fs::write(
        out_dir.join("ml_pyclass_macro.rs"),
        emit_class_macro("for_each_ml_pyclass", &ml_pyclasses),
    )
    .expect("write ml_pyclass_macro.rs");
    fs::write(
        out_dir.join("ml_pyfn_macro.rs"),
        emit_macro("for_each_ml_python_fn", &ml_pyfunctions),
    )
    .expect("write ml_pyfn_macro.rs");
    fs::write(
        out_dir.join("util_fn_macro.rs"),
        emit_macro("for_each_util_fn", &util_fns),
    )
    .expect("write util_fn_macro.rs");
    fs::write(
        out_dir.join("auto_util_fn_macro.rs"),
        emit_macro("for_each_auto_util_fn", &auto_util_fns),
    )
    .expect("write auto_util_fn_macro.rs");
}
