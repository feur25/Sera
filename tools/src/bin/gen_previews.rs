use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

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
        // Match "chart = sp.build_..." OR "chart = ("  (parenthesized chain starting on next line)
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
            // e.g. "chart = ("
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

fn python_bin() -> &'static str {
    if Command::new("python3").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
        "python3"
    } else {
        "python"
    }
}

fn run_example(code: &str, chart_var: &str) -> Option<String> {
    // Dark-theme CSS injected so charts blend with the navy docs inside the iframe.
    let dark_css = concat!(
        ".sp-bg{{fill:transparent!important}}",
        ".sp-ttl{{fill:#e2e8f0!important}}",
        "svg text{{fill:#cbd5e1!important}}",
        ".sp-ax-x,.sp-ax-y{{stroke:#475569!important}}",
        ".sp-gl{{stroke:#2d3748!important}}",
        ".sp-xl,.sp-yl{{fill:#94a3b8!important}}",
        "[id^='spp']{{box-shadow:none!important;border-radius:0!important}}"
    );
    let logo = "https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png";
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
        // Match docs dark background inside the iframe
        let html = html.replace("background:#fff;", "background:#0d1117;");
        // Inject SeraPlot logo into hover tooltip entries
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

/// Adds a logo image to every hover tooltip entry in `var data=[...]`.
/// If the array is empty, builds entries from `data-lbl` attributes on SVG elements.
fn inject_logo_into_hover(html: String, logo: &str) -> String {
    let marker = "var data=[";
    let Some(s) = html.find(marker) else { return html; };
    let b = s + marker.len() - 1; // points to '['

    // Walk balanced brackets to find the end of the array
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
        // No hover data — synthesise entries from SVG data-lbl attributes.
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
        // Existing entries — add/replace "image" field in each object.
        // Simple string manipulation: insert `,"image":"<logo>"` before each object's `}`.
        // We walk depth-1 objects (top-level array items).
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
                        // End of a top-level object — inject image before closing `}`
                        // Only add if "image" key not already present in this object slice
                        let obj_slice = &result[obj_start..];
                        if !obj_slice.contains("\"image\"") {
                            result.push_str(&image_kv);
                        } else {
                            // Replace existing image value
                            // Find last "image":" in result from obj_start
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

/// Strips the outer `<!DOCTYPE>/<html>/<head>/<body>` wrapper from chart HTML.
/// Keeps all `<style>` tags from `<head>` and the full `<body>` content.
/// This prevents the closing `</body></html>` from disrupting the mdBook page's
/// DOM structure, which was causing nav arrows to be mispositioned.
fn extract_body_content(html: String) -> String {
    // Collect all <style>...</style> blocks from the <head> section
    let head_end = html.find("</head>").unwrap_or(0);
    let head = &html[..head_end];
    let mut out = String::new();
    let mut hpos = 0;
    while let Some(rel) = head[hpos..].find("<style") {
        let abs = hpos + rel;
        if let Some(end_rel) = head[abs..].find("</style>") {
            let abs_end = abs + end_rel + 8; // 8 = "</style>".len()
            out.push_str(&head[abs..abs_end]);
            hpos = abs_end;
        } else {
            break;
        }
    }

    // Extract content inside <body>...</body>
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

/// Injects an `<iframe>` live-preview block after the first code fence in the Examples section.
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

fn strip_preview(content: &str) -> String {
    // Remove everything from "\n\n<details" up to and including the closing "</details>\n"
    let mut out = String::with_capacity(content.len());
    let mut rest = content;
    while let Some(start) = rest.find("\n\n<details") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..]; // skip the two leading newlines
        if let Some(end) = after.find("</details>") {
            rest = &after[end + 10..]; // skip "</details>"
            if rest.starts_with('\n') {
                rest = &rest[1..];
            }
        } else {
            // malformed — keep remainder as-is
            out.push_str(&rest[start..]);
            return out;
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

    // Create the previews output directory
    let previews_dir = docs.join("previews");
    fs::create_dir_all(&previews_dir).ok();

    // Compute relative path from the chart page back to /previews/
    // e.g. "charts/2d" (depth 2) → "../../previews/"
    //      "ml"         (depth 1) → "../previews/"
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
        if !force && content.contains("<details") {
            println!("  skip (preview exists): {}", path.file_name().unwrap_or_default().to_string_lossy());
            continue;
        }

        // Strip existing preview block before regenerating.
        let content = if content.contains("<details") {
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
            // Write standalone chart HTML into docs/previews/{name}.html
            let preview_file = previews_dir.join(format!("{}.html", chart_name));
            let _ = fs::write(&preview_file, html.as_bytes());

            // Inject iframe reference into the markdown
            let iframe_src = format!("{}{}.html", preview_rel, chart_name);
            let new_content = inject_preview(&content, &iframe_src);
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
