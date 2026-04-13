use std::fs;
use std::path::{Path, PathBuf};

fn slug(path: &Path) -> String {
    path.file_stem().unwrap_or_default().to_string_lossy().to_string()
}

fn parse_md(path: &Path) -> (String, String) {
    let raw = fs::read_to_string(path).unwrap_or_default();
    let content = raw.strip_prefix('\u{feff}').unwrap_or(&raw);

    let mut title = String::new();
    let mut fn_name = String::new();
    let mut in_sig = false;
    let mut in_code = false;

    for line in content.lines() {
        let t = line.trim();
        if title.is_empty() && t.starts_with("# ") {
            title = t.trim_start_matches('#').trim().to_string();
        }
        if t == "## Signature" {
            in_sig = true;
            continue;
        }
        if in_sig && t.starts_with("```python") {
            in_code = true;
            continue;
        }
        if in_sig && in_code {
            if let Some(rest) = t.strip_prefix("sp.") {
                let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                if !name.is_empty() {
                    fn_name = name;
                }
            }
            break;
        }
    }

    if fn_name.is_empty() {
        fn_name = slug(path);
    }
    (fn_name, title)
}

fn scan(docs: &Path, rel_dir: &str) -> Vec<(String, String)> {
    let d = docs.join(rel_dir);
    if !d.is_dir() {
        return vec![];
    }
    let mut files: Vec<PathBuf> = fs::read_dir(&d)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "md").unwrap_or(false))
        .filter(|p| p.file_name().map(|x| x != "index.md").unwrap_or(true))
        .collect();
    files.sort();

    files.iter().map(|f| {
        let (fn_name, title) = parse_md(f);
        let title_short = title.split('/').next().unwrap_or(&title).trim().to_string();
        let link = format!("[`{fn_name}`](../{rel_dir}/{}.md)", slug(f));
        (link, title_short)
    }).collect()
}

fn table(rows: &[(String, String)]) -> String {
    let mut lines = vec![
        "| Function | Description |".to_string(),
        "|----------|-------------|".to_string(),
    ];
    for (link, desc) in rows {
        lines.push(format!("| {link} | {desc} |"));
    }
    lines.join("\n")
}

fn main() {
    let docs = Path::new("docs");

    let charts_2d  = scan(docs, "charts/2d");
    let charts_3d  = scan(docs, "charts/3d");
    let charts_map = scan(docs, "charts/map");
    let ml         = scan(docs, "ml");
    let config     = scan(docs, "config");

    let output = format!(
        "# API Reference\n\nComplete alphabetical index of every public symbol exported by `seraplot`.\n\n---\n\n## Module: `seraplot`\n\n```python\nimport seraplot as sp\n```\n\n---\n\n## Chart Functions — 2D\n\n{}\n\n---\n\n## Chart Functions — 3D\n\n{}\n\n---\n\n## Chart Functions — Map\n\n{}\n\n---\n\n## Machine Learning\n\n{}\n\n---\n\n## Configuration\n\n{}\n",
        table(&charts_2d),
        table(&charts_3d),
        table(&charts_map),
        table(&ml),
        table(&config),
    );

    let out_path = docs.join("api/index.md");
    fs::create_dir_all(out_path.parent().unwrap()).unwrap();
    fs::write(&out_path, output).unwrap();
    println!("Generated {}", out_path.display());
}
