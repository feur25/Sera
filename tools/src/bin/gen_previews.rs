use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

fn snake_to_camel(s: &str) -> String {
    let mut out = String::new();
    let mut cap = false;
    for c in s.chars() {
        if c == '_' { cap = true; } else if cap { out.push(c.to_ascii_uppercase()); cap = false; } else { out.push(c); }
    }
    out
}

fn positional_count(fn_name: &str) -> usize {
    match fn_name {
        "build_histogram" | "build_kde_chart" | "build_kde3d_chart" |
        "build_violin3d_chart" | "build_gauge" => 1,
        "build_bar_chart" | "build_hbar" | "build_line_chart" |
        "build_histogram_overlay" | "build_grouped_bar" | "build_stacked_bar" |
        "build_heatmap" | "build_pie_chart" | "build_donut_chart" |
        "build_boxplot" | "build_violin" | "build_funnel" | "build_treemap" |
        "build_multiline_chart" | "build_area_chart" | "build_waterfall" |
        "build_bullet" | "build_bubble_map" | "build_choropleth" |
        "build_radar_chart" | "build_radar3d_chart" | "build_lollipop_chart" |
        "build_ridgeline_chart" | "build_ridgeline3d_chart" | "build_pie3d_chart" |
        "build_stacked_bar3d_chart" | "build_parallel" | "build_dbscan_chart" |
        "build_wordcloud" | "build_scatter_chart" => 2,
        "build_slope" | "build_sunburst" | "build_sunburst3d_chart" |
        "build_scatter3d_chart" | "build_bar3d_chart" | "build_line3d_chart" |
        "build_lollipop3d_chart" | "build_heatmap3d_chart" |
        "build_dumbbell3d_chart" | "build_dumbbell" | "build_globe3d_chart" |
        "build_dbscan_chart_3d" | "build_bubble" => 3,
        "build_bubble3d_chart" => 4,
        "build_candlestick" | "build_candlestick3d_chart" => 5,
        _ => 2,
    }
}

fn split_args(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    let mut in_str = false;
    let mut str_ch = '"';
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if in_str {
            cur.push(c);
            if c == '\\' { if let Some(nc) = chars.next() { cur.push(nc); } }
            else if c == str_ch { in_str = false; }
        } else {
            match c {
                '"' | '\'' => { in_str = true; str_ch = c; cur.push(c); }
                '(' | '[' | '{' => { depth += 1; cur.push(c); }
                ')' | ']' | '}' => { depth -= 1; if depth < 0 { break; } cur.push(c); }
                ',' if depth == 0 => { args.push(cur.trim().to_string()); cur = String::new(); }
                _ => { cur.push(c); }
            }
        }
    }
    let t = cur.trim().to_string();
    if !t.is_empty() { args.push(t); }
    args
}

fn find_kwarg_eq(s: &str) -> Option<usize> {
    let mut depth = 0i32;
    let mut in_str = false;
    let mut str_ch = '"';
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if in_str {
            if c == '\\' { i += 1; }
            else if c == str_ch { in_str = false; }
        } else {
            match c {
                '"' | '\'' => { in_str = true; str_ch = c; }
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth -= 1,
                '=' if depth == 0 => {
                    if i + 1 < bytes.len() && bytes[i + 1] != b'=' && (i == 0 || bytes[i - 1] != b'!') {
                        let key = s[..i].trim();
                        if key.chars().all(|c| c.is_alphanumeric() || c == '_') && !key.is_empty() {
                            return Some(i);
                        }
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    None
}

fn py_val_to_js(v: &str) -> String {
    let v = v.trim();
    if v == "True" { return "true".to_string(); }
    if v == "False" { return "false".to_string(); }
    if v == "None" { return "null".to_string(); }
    if v.starts_with('[') {
        let inner = &v[1..v.len().saturating_sub(1)];
        let elems: Vec<String> = split_args(inner).iter().map(|e| py_val_to_js(e)).collect();
        return format!("[{}]", elems.join(", "));
    }
    v.to_string()
}

fn translate_build_call_body(fn_name: &str, args_text: &str) -> String {
    let n_pos = positional_count(fn_name);
    let raw_args = split_args(args_text);
    let mut positional: Vec<String> = Vec::new();
    let mut kwargs: Vec<(String, String)> = Vec::new();
    let mut pos_seen = 0usize;

    for arg in &raw_args {
        let t = arg.trim().trim_end_matches(',').trim();
        if t.is_empty() { continue; }
        if let Some(eq) = find_kwarg_eq(t) {
            let key = t[..eq].trim();
            let val = py_val_to_js(t[eq + 1..].trim());
            if pos_seen < n_pos && val == key {
                positional.push(val);
                pos_seen += 1;
            } else if pos_seen < n_pos && positional.len() < n_pos + 1 {
                positional.push(val);
                pos_seen += 1;
            } else {
                kwargs.push((key.to_string(), val));
            }
        } else {
            positional.push(py_val_to_js(t));
            pos_seen += 1;
        }
    }

    let mut parts: Vec<String> = positional;
    if !kwargs.is_empty() {
        let kv_lines: Vec<String> = kwargs.iter()
            .map(|(k, v)| format!("    {}: {}", k, v))
            .collect();
        parts.push(format!("{{\n{}\n}}", kv_lines.join(",\n")));
    }
    parts.join(",\n")
}

fn convert_sp_fn_name_js(name: &str) -> String {
    format!("sp.{}", name)
}

fn py_to_js(py: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = py.chars().collect();
    let len = chars.len();

    while i < len {
        let rest: String = chars[i..].iter().collect();

        if rest.starts_with("import seraplot as sp") {
            result.push_str("const sp = require('seraplot');");
            i += "import seraplot as sp".len();
            continue;
        }

        if rest.starts_with("True") && (i == 0 || !chars[i-1].is_alphanumeric() && chars[i-1] != '_') {
            let after = if i + 4 < len { chars[i+4] } else { ' ' };
            if !after.is_alphanumeric() && after != '_' {
                result.push_str("true"); i += 4; continue;
            }
        }
        if rest.starts_with("False") && (i == 0 || !chars[i-1].is_alphanumeric() && chars[i-1] != '_') {
            let after = if i + 5 < len { chars[i+5] } else { ' ' };
            if !after.is_alphanumeric() && after != '_' {
                result.push_str("false"); i += 5; continue;
            }
        }
        if rest.starts_with("None") && (i == 0 || !chars[i-1].is_alphanumeric() && chars[i-1] != '_') {
            let after = if i + 4 < len { chars[i+4] } else { ' ' };
            if !after.is_alphanumeric() && after != '_' {
                result.push_str("null"); i += 4; continue;
            }
        }

        if rest.starts_with("sp.build_") || rest.starts_with("sp.build_hover_json") {
            let fn_start = i + 3;
            let mut fn_end = fn_start;
            while fn_end < len && (chars[fn_end].is_alphanumeric() || chars[fn_end] == '_') {
                fn_end += 1;
            }
            let fn_name: String = chars[fn_start..fn_end].iter().collect();
            let js_fn = convert_sp_fn_name_js(&fn_name);

            if fn_end < len && chars[fn_end] == '(' {
                let call_start = fn_end + 1;
                let mut depth = 1i32;
                let mut call_end = call_start;
                let mut in_str = false;
                let mut str_ch = '"';
                while call_end < len && depth > 0 {
                    let c = chars[call_end];
                    if in_str {
                        if c == '\\' { call_end += 1; }
                        else if c == str_ch { in_str = false; }
                    } else {
                        match c {
                            '"' | '\'' => { in_str = true; str_ch = c; }
                            '(' | '[' | '{' => depth += 1,
                            ')' | ']' | '}' => { depth -= 1; if depth == 0 { break; } }
                            _ => {}
                        }
                    }
                    call_end += 1;
                }
                let args_text: String = chars[call_start..call_end].iter().collect();
                let translated = translate_build_call_body(&fn_name, &args_text);
                result.push_str(&js_fn);
                result.push('(');
                result.push_str(&translated);
                result.push(')');
                i = call_end + 1;
                continue;
            } else {
                result.push_str(&js_fn);
                i = fn_end;
                continue;
            }
        }

        if rest.starts_with(".save(") { result.push_str("// save("); i += 6; continue; }

        if chars[i] == '\n' {
            result.push('\n');
            i += 1;
            let line_start = i;
            let mut word_end = i;
            while word_end < len && (chars[word_end].is_alphanumeric() || chars[word_end] == '_') {
                word_end += 1;
            }
            if word_end > line_start {
                let mut eq_pos = word_end;
                while eq_pos < len && chars[eq_pos] == ' ' { eq_pos += 1; }
                if eq_pos < len && chars[eq_pos] == '=' {
                    let next = if eq_pos + 1 < len { chars[eq_pos + 1] } else { ' ' };
                    if next != '=' {
                        let var_name: String = chars[line_start..word_end].iter().collect();
                        if !var_name.starts_with("import") {
                            result.push_str("const ");
                        }
                    }
                }
            }
            continue;
        }

        if chars[i] == '#' {
            result.push('/'); result.push('/');
            i += 1;
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    let lines: Vec<String> = result.lines().map(|l| {
        let t = l.trim_start();
        if t.starts_with("const (") || (t.starts_with("const ") && l.trim_end().ends_with('(') && !l.contains('=')) {
            l.replacen("const ", "", 1)
        } else {
            l.to_string()
        }
    }).collect();
    let joined = lines.join("\n");
    translate_call_kwargs_pass(&joined)
}

fn translate_call_kwargs_pass(code: &str) -> String {
    let chars: Vec<char> = code.chars().collect();
    let len = chars.len();
    let mut out = String::with_capacity(len);
    let mut i = 0;

    while i < len {
        if chars[i] == '(' {
            out.push('(');
            i += 1;
            let args_start = i;
            let mut depth = 1i32;
            let mut end = i;
            let mut in_str = false;
            let mut str_ch = '"';
            while end < len && depth > 0 {
                let c = chars[end];
                if in_str {
                    if c == '\\' { end += 1; } else if c == str_ch { in_str = false; }
                } else {
                    match c {
                        '"' | '\'' => { in_str = true; str_ch = c; }
                        '(' | '[' | '{' => depth += 1,
                        ')' | ']' | '}' => { depth -= 1; if depth == 0 { break; } }
                        _ => {}
                    }
                }
                end += 1;
            }
            let args_text: String = chars[args_start..end].iter().collect();
            let converted = convert_kwargs_in_args(&args_text);
            out.push_str(&converted);
            if end < len { out.push(chars[end]); i = end + 1; }
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn convert_kwargs_in_args(args: &str) -> String {
    let raw = split_args(args);
    if raw.is_empty() { return args.to_string(); }
    let has_kwarg = raw.iter().any(|a| find_kwarg_eq(a.trim().trim_end_matches(',').trim()).is_some());
    if !has_kwarg { return args.to_string(); }
    let mut positional: Vec<String> = Vec::new();
    let mut kwargs: Vec<(String, String)> = Vec::new();
    for arg in &raw {
        let t = arg.trim().trim_end_matches(',').trim();
        if t.is_empty() { continue; }
        if let Some(eq) = find_kwarg_eq(t) {
            let key = t[..eq].trim();
            let val = py_val_to_js(t[eq + 1..].trim());
            kwargs.push((key.to_string(), val));
        } else {
            positional.push(py_val_to_js(t));
        }
    }
    let mut parts = positional;
    if !kwargs.is_empty() {
        let kv: Vec<String> = kwargs.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
        parts.push(format!("{{{}}}", kv.join(", ")));
    }
    parts.join(", ")
}

fn py_to_ts(py: &str) -> String {
    let js = py_to_js(py);
    let mut out_lines = Vec::new();
    for line in js.lines() {
        let t = line.trim_start();
        if t.starts_with("const sp = require") {
            out_lines.push("import * as sp from 'seraplot';".to_string());
        } else if t.starts_with("const ") && t.contains(" = [") {
            let rest = &t["const ".len()..];
            if let Some(eq) = rest.find(" = [") {
                let var = rest[..eq].trim();
                let val = rest[eq + 3..].trim();
                let indent = &line[..line.len() - t.len()];
                if val.contains('"') || val.contains('\'') {
                    out_lines.push(format!("{}const {}: string[] = {}", indent, var, val));
                } else {
                    out_lines.push(format!("{}const {}: number[] = {}", indent, var, val));
                }
            } else {
                out_lines.push(line.to_string());
            }
        } else if t.starts_with("const ") && !t.contains("sp.") {
            let rest = &t["const ".len()..];
            if let Some(eq) = rest.find(" = ") {
                let var = rest[..eq].trim();
                let val = rest[eq + 3..].trim();
                let indent = &line[..line.len() - t.len()];
                if val.starts_with('"') || val.starts_with('\'') {
                    out_lines.push(format!("{}const {}: string = {}", indent, var, val));
                } else if val.parse::<f64>().is_ok() {
                    out_lines.push(format!("{}const {}: number = {}", indent, var, val));
                } else {
                    out_lines.push(line.to_string());
                }
            } else {
                out_lines.push(line.to_string());
            }
        } else {
            out_lines.push(line.to_string());
        }
    }
    out_lines.join("\n")
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

static TAB_CSS: &str = r#"<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155}
.sp-tb{padding:9px 22px;border:none;background:none;color:#64748b;cursor:pointer;font-size:13px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){hljs.highlightElement(c)})}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc code').forEach(function(c){hljs.highlightElement(c)})});
</script>"#;

fn make_code_tabs(group_id: &str, py: &str, js: &str, ts: &str) -> String {
    format!(
        "\n{css}\n<div class=\"sp-tabs\" id=\"{g}\">\n<div class=\"sp-tab-btns\">\
<button class=\"sp-tb sp-act\" onclick=\"spTab('{g}','{g}-py',this)\">Python</button>\
<button class=\"sp-tb\" onclick=\"spTab('{g}','{g}-js',this)\">JavaScript</button>\
<button class=\"sp-tb\" onclick=\"spTab('{g}','{g}-ts',this)\">TypeScript</button>\
</div>\n<div id=\"{g}-py\" class=\"sp-tc sp-on\"><pre style=\"margin:0;border-radius:0\"><code class=\"language-python\">{py_e}</code></pre></div>\
\n<div id=\"{g}-js\" class=\"sp-tc\"><pre style=\"margin:0;border-radius:0\"><code class=\"language-javascript\">{js_e}</code></pre></div>\
\n<div id=\"{g}-ts\" class=\"sp-tc\"><pre style=\"margin:0;border-radius:0\"><code class=\"language-typescript\">{ts_e}</code></pre></div>\
\n</div>\n",
        css = TAB_CSS,
        g = group_id,
        py_e = html_escape(py),
        js_e = html_escape(js),
        ts_e = html_escape(ts),
    )
}

fn inject_tabs_and_preview(content: &str, iframe_src: &str, py_code: &str, group_id: &str) -> String {
    let js_code = py_to_js(py_code);
    let ts_code = py_to_ts(py_code);
    let tabs_block = make_code_tabs(group_id, py_code, &js_code, &ts_code);

    let preview = format!(
        "\n\n<details open>\n<summary style=\"cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8\">&#9654;&nbsp;Live Preview</summary>\n\n<iframe src=\"{iframe_src}\" style=\"width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117\" loading=\"lazy\"></iframe>\n\n</details>\n"
    );

    let mut in_examples = false;
    let mut in_code = false;
    let mut code_start: Option<usize> = None;
    let mut code_end: Option<usize> = None;
    let mut pos = 0usize;

    for line in content.lines() {
        let byte_len = line.len() + 1;
        if line.starts_with("## Example") {
            in_examples = true;
        }
        if in_examples && line.trim().starts_with("```python") {
            in_code = true;
            code_start = Some(pos);
        }
        if in_examples && in_code && line.trim() == "```" {
            code_end = Some(pos + byte_len);
            break;
        }
        pos += byte_len;
    }

    if let (Some(start), Some(end)) = (code_start, code_end) {
        let mut out = content[..start].to_string();
        out.push_str(&tabs_block);
        out.push_str(&preview);
        out.push_str(&content[end..]);
        out
    } else {
        content.to_string()
    }
}

fn extract_first_example(content: &str) -> Option<String> {
    let mut in_examples = false;
    let mut in_code = false;
    let mut code_lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        let t = line.trim();
        if line.starts_with("## Example") {
            in_examples = true;
            continue;
        }
        if !in_examples {
            continue;
        }
        if t.starts_with("```python") {
            in_code = true;
            code_lines.clear();
            continue;
        }
        if in_code && t == "```" {
            if !code_lines.is_empty() {
                return Some(code_lines.join("\n"));
            }
            in_code = false;
            continue;
        }
        if in_code {
            code_lines.push(line);
        }
    }
    None
}

fn find_chart_var(code: &str) -> String {
    let mut best: Option<String> = None;
    for line in code.lines() {
        let trimmed = line.trim();
        let is_paren = trimmed.ends_with("= (") || trimmed.ends_with("=(");
        if let Some(idx) = line.find("= sp.") {
            if line.contains("build_hover_json") {
                continue;
            }
            let before = line[..idx].trim();
            if let Some(var) = before.split_whitespace().last() {
                let v: String = var.chars().filter(|c| c.is_alphanumeric() || *c == '_').collect();
                if !v.is_empty() {
                    best = Some(v);
                }
            }
        } else if is_paren {
            let lhs = trimmed.trim_end_matches(|c| c == '(' || c == ' ' || c == '=');
            let var = lhs.split_whitespace().last().unwrap_or("").to_string();
            let v: String = var.chars().filter(|c| c.is_alphanumeric() || *c == '_').collect();
            if !v.is_empty() {
                best = Some(v);
            }
        }
    }
    best.unwrap_or_else(|| "chart".to_string())
}

fn python_bin() -> String {
    let candidates = [
        "../../.venv/Scripts/python.exe",
        "../../.venv/bin/python3",
        "../../.venv/bin/python",
        "../../../.venv/Scripts/python.exe",
        "../../../.venv/bin/python3",
        "../../../.venv/bin/python",
    ];
    for c in &candidates {
        if std::path::Path::new(c).exists() {
            return c.to_string();
        }
    }
    if Command::new("python3").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
        "python3".to_string()
    } else {
        "python".to_string()
    }
}

fn run_example(code: &str, chart_var: &str) -> Option<String> {
    let dark_css = concat!(
        ".sp-bg{fill:transparent!important}",
        ".sp-ttl{fill:#e2e8f0!important}",
        "svg text{fill:#cbd5e1!important}",
        ".sp-ax-x,.sp-ax-y{stroke:#475569!important}",
        ".sp-gl{stroke:#2d3748!important}",
        ".sp-xl,.sp-yl{fill:#94a3b8!important}",
        "[id^='spp']{box-shadow:none!important;border-radius:0!important}"
    );
    let logo = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png";
    let wrapper = format!(
        "import seraplot as _sp\n_sp.set_auto_display(False)\n{code}\nimport sys\n_c={chart_var}.set_bg(None).inject_css(\"{dark_css}\")\nsys.stdout.buffer.write(_c.html.encode('utf-8'))\n"
    );

    let tmp = std::env::temp_dir().join("seraplot_preview.py");
    fs::write(&tmp, wrapper).ok()?;

    let result = Command::new(python_bin()).arg(&tmp).output();
    let _ = fs::remove_file(&tmp);

    let output = result.ok()?;
    if output.status.success() && !output.stdout.is_empty() {
        let html = String::from_utf8(output.stdout).ok()?;
        let html = html.replace("background:#fff;", "background:#0d1117;");
        let html = html.replace("loading=\"lazy\"", "loading=\"eager\"");
        let html = inject_logo_into_hover(html, logo);
        Some(html)
    } else {
        if !output.stderr.is_empty() {
            let err = String::from_utf8_lossy(&output.stderr);
            eprintln!("    stderr: {}", &err[..err.len().min(300)]);
        }
        None
    }
}

fn inject_logo_into_hover(html: String, logo: &str) -> String {
    let marker = "var data=[";
    let Some(s) = html.find(marker) else { return html; };
    let b = s + marker.len() - 1;
    let bytes = html.as_bytes();
    let mut depth = 0i32;
    let mut end = b;
    for (i, &ch) in bytes[b..].iter().enumerate() {
        match ch {
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    end = b + i + 1;
                    break;
                }
            }
            _ => {}
        }
    }

    let old_array = &html[b..end];
    let esc = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"");

    let new_array = if old_array == "[]" {
        let mut max_idx: i32 = -1;
        let mut pos = 0;
        while let Some(p) = html[pos..].find("data-idx=\"") {
            let start = pos + p + 10;
            if let Some(q) = html[start..].find('"') {
                let end_n = start + q;
                if let Ok(idx) = html[start..end_n].parse::<i32>() {
                    max_idx = max_idx.max(idx);
                }
                pos = end_n + 1;
            } else {
                break;
            }
        }
        if max_idx < 0 {
            return html;
        }
        let mut entries = Vec::new();
        for idx in 0..=(max_idx as usize) {
            let needle = format!("data-idx=\"{}\"", idx);
            let lbl = html.find(&needle).and_then(|p| {
                let win = &html[p..html.len().min(p + 300)];
                let lp = win.find("data-lbl=\"")?;
                let ls = p + lp + 10;
                let le = html[ls..].find('"')? + ls;
                Some(html[ls..le].to_string())
            });
            let title = lbl.unwrap_or_else(|| (idx + 1).to_string());
            entries.push(format!(
                "{{\"title\":\"{}\",\"kv\":[],\"image\":\"{}\"}}",
                esc(&title),
                esc(logo)
            ));
        }
        format!("[{}]", entries.join(","))
    } else {
        let mut result = String::with_capacity(old_array.len() + 64 * 20);
        let ab = old_array.as_bytes();
        let mut depth2 = 0i32;
        let mut obj_start = 0usize;
        let image_kv = format!(",\"image\":\"{}\"", esc(logo));
        for (i, &ch) in ab.iter().enumerate() {
            match ch {
                b'[' | b'{' => {
                    if depth2 == 0 { result.push(ch as char); }
                    else if depth2 == 1 && ch == b'{' { obj_start = result.len(); result.push(ch as char); }
                    else { result.push(ch as char); }
                    depth2 += 1;
                }
                b']' | b'}' => {
                    depth2 -= 1;
                    if depth2 == 1 && ch == b'}' {
                        let obj_slice = &result[obj_start..];
                        if !obj_slice.contains("\"image\"") {
                            result.push_str(&image_kv);
                        } else {
                            if let Some(ip) = result[obj_start..].rfind("\"image\":\"") {
                                let vs = obj_start + ip + 9;
                                if let Some(ve) = result[vs..].find('"') {
                                    let ve = vs + ve;
                                    result.replace_range(vs..ve, &esc(logo));
                                }
                            }
                        }
                        result.push('}');
                    } else {
                        result.push(ch as char);
                    }
                }
                b',' if depth2 == 1 => result.push(','),
                _ if depth2 >= 1 => result.push(ch as char),
                _ => {}
            }
        }
        result
    };

    let mut out = String::with_capacity(html.len() + new_array.len());
    out.push_str(&html[..b]);
    out.push_str(&new_array);
    out.push_str(&html[end..]);
    out
}

fn extract_body_content(html: String) -> String {
    let head_end = html.find("</head>").unwrap_or(0);
    let head = &html[..head_end];
    let mut out = String::new();
    let mut hpos = 0;
    while let Some(rel) = head[hpos..].find("<style") {
        let abs = hpos + rel;
        if let Some(end_rel) = head[abs..].find("</style>") {
            let abs_end = abs + end_rel + 8;
            out.push_str(&head[abs..abs_end]);
            hpos = abs_end;
        } else {
            break;
        }
    }

    if let Some(body_open) = html.find("<body") {
        let after_tag = &html[body_open..];
        if let Some(gt) = after_tag.find('>') {
            let body_start = body_open + gt + 1;
            let body_end = html[body_start..]
                .rfind("</body>")
                .map(|p| body_start + p)
                .unwrap_or(html.len());
            out.push_str(&html[body_start..body_end]);
        }
    }

    if out.is_empty() { html } else { out }
}

fn inject_preview(content: &str, iframe_src: &str) -> String {
    let preview = format!(
        "\n\n<details open>\n<summary style=\"cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8\">&#9654;&nbsp;Live Preview</summary>\n\n<iframe src=\"{iframe_src}\" style=\"width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117\" loading=\"lazy\"></iframe>\n\n</details>\n"
    );

    let mut in_examples = false;
    let mut in_code = false;
    let mut insert_at: Option<usize> = None;
    let mut pos = 0usize;

    for line in content.lines() {
        let byte_len = line.len() + 1;
        if line.starts_with("## Example") {
            in_examples = true;
        }
        if in_examples && line.trim().starts_with("```python") {
            in_code = true;
        }
        if in_examples && in_code && line.trim() == "```" {
            insert_at = Some(pos + byte_len);
            break;
        }
        pos += byte_len;
    }

    if let Some(i) = insert_at {
        let mut out = content[..i].to_string();
        out.push_str(&preview);
        out.push_str(&content[i..]);
        out
    } else {
        content.to_string()
    }
}

fn html_unescape(s: &str) -> String {
    s.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"")
}

fn strip_preview(content: &str) -> String {
    let tab_marker = "\n<style>\n.sp-tabs{";
    // new format markers
    let py_code_start_new = "class=\"language-python\">";
    let code_end_new = "</code>";
    // old format markers
    let py_pane_open = "data-lang=\"py\">";
    let code_block_start = "\n```python\n";
    let code_block_end = "\n```\n";

    let mut out = String::with_capacity(content.len());
    let mut rest = content;

    loop {
        let Some(tp) = rest.find(tab_marker) else { break };
        out.push_str(&rest[..tp]);

        let suffix = &rest[tp..];

        // Try new format: <code class="language-python">HTML-ESCAPED</code>
        let py_code: Option<String> = suffix.find(py_code_start_new)
            .and_then(|pp| {
                let after = &suffix[pp + py_code_start_new.len()..];
                after.find(code_end_new).map(|end| {
                    html_unescape(&after[..end])
                })
            })
            // fallback: old format with data-lang="py" + markdown fences
            .or_else(|| {
                suffix.find(py_pane_open)
                    .and_then(|pp| {
                        let after_pane = &suffix[pp + py_pane_open.len()..];
                        after_pane.find(code_block_start).and_then(|cs| {
                            let code_start = cs + code_block_start.len();
                            after_pane[cs..].find(code_block_end).map(|ce| {
                                after_pane[code_start..cs + ce].to_string()
                            })
                        })
                    })
            });

        if let Some(code) = py_code {
            out.push_str("\n```python\n");
            out.push_str(&code);
            out.push_str("\n```\n");
        }

        let end_marker = "</details>";
        if let Some(end) = suffix.find(end_marker) {
            rest = &rest[tp + end + end_marker.len()..];
            if rest.starts_with('\n') { rest = &rest[1..]; }
        } else {
            let div_end = "</div>\n";
            if let Some(end) = suffix.find(div_end) {
                rest = &rest[tp + end + div_end.len()..];
            } else {
                break;
            }
        }
    }
    out.push_str(rest);
    out
}

fn process_dir(docs: &Path, rel_dir: &str) {
    let d = docs.join(rel_dir);
    if !d.is_dir() {
        return;
    }

    let previews_dir = docs.join("previews");
    fs::create_dir_all(&previews_dir).ok();
    let depth = rel_dir.split('/').count();
    let preview_rel: String = "../".repeat(depth) + "previews/";

    let mut files: Vec<PathBuf> = fs::read_dir(&d)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "md").unwrap_or(false))
        .filter(|p| p.file_name().map(|x| x != "index.md").unwrap_or(true))
        .collect();
    files.sort();

    for path in &files {
        let raw = fs::read_to_string(path).unwrap_or_default();
        let content = raw.strip_prefix('\u{feff}').unwrap_or(&raw).to_string();

        let force = std::env::args().any(|a| a == "--force");
        if !force && (content.contains("<details") || content.contains("sp-tabs")) {
            println!("  skip (preview exists): {}", path.file_name().unwrap_or_default().to_string_lossy());
            continue;
        }

        let content = if content.contains("<details") || content.contains("sp-tabs") {
            strip_preview(&content)
        } else {
            content
        };

        let Some(code) = extract_first_example(&content) else {
            println!("  skip (no example): {}", path.file_name().unwrap_or_default().to_string_lossy());
            continue;
        };

        let chart_var = find_chart_var(&code);
        let chart_name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
        print!("  {} ...", path.file_name().unwrap_or_default().to_string_lossy());
        let _ = std::io::stdout().flush();

        if let Some(html) = run_example(&code, &chart_var) {
            let preview_file = previews_dir.join(format!("{}.html", chart_name));
            let _ = fs::write(&preview_file, html.as_bytes());
            let iframe_src = format!("{}{}.html", preview_rel, chart_name);
            let new_content = inject_tabs_and_preview(&content, &iframe_src, &code, &chart_name);
            let _ = fs::write(path, new_content.as_bytes());
            println!(" OK ({} chars)", html.len());
        } else {
            println!(" FAILED (skipped)");
        }
    }
}

fn main() {
    let check = Command::new(python_bin())
        .args(["-c", "import seraplot"])
        .output();
    if check.map(|o| !o.status.success()).unwrap_or(true) {
        println!("seraplot not installed — skipping preview generation");
        return;
    }

    println!("Generating chart previews...");
    let docs = Path::new("docs");
    for dir in &["charts/2d", "charts/3d", "charts/map", "ml"] {
        process_dir(docs, dir);
    }
    println!("Done.");
}


