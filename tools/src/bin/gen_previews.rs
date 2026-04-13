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
    for line in code.lines() {
        if let Some(idx) = line.find("= sp.") {
            let before = line[..idx].trim();
            if let Some(var) = before.split_whitespace().last() {
                let v: String = var.chars().filter(|c| c.is_alphanumeric() || *c == '_').collect();
                if !v.is_empty() {
                    return v;
                }
            }
        }
    }
    "chart".to_string()
}

fn python_bin() -> &'static str {
    if Command::new("python3").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
        "python3"
    } else {
        "python"
    }
}

fn run_example(code: &str, chart_var: &str) -> Option<String> {
    let wrapper = format!(
        "import seraplot as _sp\n_sp.set_auto_display(False)\n{code}\nimport sys\nsys.stdout.buffer.write({chart_var}.html.encode('utf-8'))\n"
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
        "\n\n<details open>\n<summary style=\"cursor:pointer;font-weight:600;padding:4px 0\">&#9654; Live Preview</summary>\n\n<div style=\"width:100%;overflow:auto;border-radius:6px;margin:8px 0\">\n{}\n</div>\n\n</details>\n",
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

        if content.contains("<details") {
            println!("  skip (preview exists): {}", path.file_name().unwrap_or_default().to_string_lossy());
            continue;
        }

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
