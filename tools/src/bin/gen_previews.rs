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
    // Dark-theme CSS injected into every preview so charts blend with the navy docs.
    let dark_css = concat!(
        ".sp-bg{{fill:transparent!important}}",
        ".sp-ttl{{fill:#e2e8f0!important}}",
        "svg text{{fill:#cbd5e1!important}}",
        ".sp-ax-x,.sp-ax-y{{stroke:#475569!important}}",
        ".sp-gl{{stroke:#2d3748!important}}",
        ".sp-xl,.sp-yl{{fill:#94a3b8!important}}",
        "[id^='spp']{{box-shadow:none!important;border-radius:0!important}}"
    );
    let wrapper = format!(
        "import seraplot as _sp\n_sp.set_auto_display(False)\n{code}\nimport sys\n_c={chart_var}.set_bg(None).inject_css(\"{dark_css}\")\nsys.stdout.buffer.write(_c.html.encode('utf-8'))\n"
    );

    let tmp = std::env::temp_dir().join("seraplot_preview.py");
    fs::write(&tmp, wrapper).ok()?;

    let result = Command::new(python_bin()).arg(&tmp).output();
    let _ = fs::remove_file(&tmp);

    let output = result.ok()?;
    if output.status.success() && !output.stdout.is_empty() {
        String::from_utf8(output.stdout).ok()
    } else {
        if !output.stderr.is_empty() {
            let err = String::from_utf8_lossy(&output.stderr);
            eprintln!("    stderr: {}", &err[..err.len().min(300)]);
        }
        None
    }
}

fn inject_preview(content: &str, chart_html: &str) -> String {
    let preview = format!(
        "\n\n<details open>\n<summary style=\"cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8\">&#9654;&nbsp;Live Preview</summary>\n\n<div style=\"width:100%;overflow:auto;border-radius:8px;margin:12px 0;background:#0d1117\">\n{}\n</div>\n\n</details>\n",
        chart_html.trim()
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
        print!("  {} ...", path.file_name().unwrap_or_default().to_string_lossy());
        let _ = std::io::stdout().flush();

        if let Some(html) = run_example(&code, &chart_var) {
            let new_content = inject_preview(&content, &html);
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
