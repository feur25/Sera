use std::fs;
use std::path::{Path, PathBuf};

use crate::build_common::{snake_to_camel, walk};

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
fn read_file(path: &Path) -> String {
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
    wasm_body.push_str("    serde_json::to_string(&crate::chart_variants()).unwrap_or_default()\n");
    wasm_body.push_str("}\n");
    wasm_body.push_str("#[wasm_bindgen(js_name = \"params\")]\n");
    wasm_body.push_str("pub fn wasm_params_json(input: &str) -> String {\n");
    wasm_body.push_str("    #[derive(serde::Deserialize, Default)] struct In { chart: Option<String>, family: Option<String>, variant: Option<String> }\n");
    wasm_body.push_str("    let i: In = serde_json::from_str(input).unwrap_or_default();\n");
    wasm_body.push_str("    let chart = i.chart.or(i.family);\n");
    wasm_body.push_str("    serde_json::to_string(&crate::params(chart.as_deref(), i.variant.as_deref())).unwrap_or_default()\n");
    wasm_body.push_str("}\n");
    wasm_body.push_str("#[wasm_bindgen(js_name = \"requiredParams\")]\n");
    wasm_body.push_str("pub fn wasm_required_params_json(input: &str) -> String {\n");
    wasm_body.push_str("    #[derive(serde::Deserialize, Default)] struct In { chart: Option<String>, family: Option<String>, variant: Option<String> }\n");
    wasm_body.push_str("    let i: In = serde_json::from_str(input).unwrap_or_default();\n");
    wasm_body.push_str("    let chart = i.chart.or(i.family);\n");
    wasm_body.push_str("    serde_json::to_string(&crate::required_params(chart.as_deref(), i.variant.as_deref())).unwrap_or_default()\n");
    wasm_body.push_str("}\n");
    wasm_body.push_str("#[wasm_bindgen(js_name = \"chartThemes\")]\n");
    wasm_body.push_str("pub fn wasm_chart_themes_json() -> String {\n");
    wasm_body.push_str("    serde_json::to_string(&crate::chart_themes()).unwrap_or_default()\n");
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

fn emit_macro(name: &str, fns: &[String]) -> String {
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
}

fn emit_class_macro(name: &str, classes: &[String]) -> String {
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
}

pub(crate) fn write_all(
    manifest: &Path,
    bindings_root: &Path,
    out_dir: &Path,
    builder_fns: &[String],
) {
    let ml_pyclasses: Vec<String> = Vec::new();
    let ml_pyfunctions: Vec<String> = Vec::new();

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
    for n in builder_fns {
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
