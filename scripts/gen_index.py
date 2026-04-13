import os
import re

DOCS = os.path.join(os.path.dirname(__file__), "..", "docs")

def slug(filename):
    return os.path.splitext(os.path.basename(filename))[0]

def parse_md(path):
    with open(path, encoding="utf-8") as f:
        content = f.read()
    title = ""
    for line in content.splitlines():
        line = line.strip()
        if line.startswith("# "):
            title = re.sub(r"^#+\s*", "", line).strip()
            break
    fn_name = ""
    in_sig = False
    in_code = False
    for line in content.splitlines():
        if "## Signature" in line:
            in_sig = True
            continue
        if in_sig and line.strip().startswith("```python"):
            in_code = True
            continue
        if in_sig and in_code:
            m = re.match(r"sp\.(\w+)\s*[\(\(]", line.strip())
            if m:
                fn_name = m.group(1)
            break
    if not fn_name:
        fn_name = slug(path)
    return fn_name, title

def scan(rel_dir):
    d = os.path.join(DOCS, rel_dir)
    if not os.path.isdir(d):
        return []
    files = sorted(f for f in os.listdir(d) if f.endswith(".md") and f != "index.md")
    rows = []
    for f in files:
        path = os.path.join(d, f)
        fn_name, title = parse_md(path)
        title_short = title.split("/")[0].strip()
        link = f"[`{fn_name}`](../{rel_dir}/{slug(f)}.md)"
        rows.append((link, title_short))
    return rows

def table(rows):
    lines = ["| Function | Description |", "|----------|-------------|"]
    for link, desc in rows:
        lines.append(f"| {link} | {desc} |")
    return "\n".join(lines)

charts_2d   = scan("charts/2d")
charts_3d   = scan("charts/3d")
charts_map  = scan("charts/map")
ml          = scan("ml")
config      = scan("config")

out = f"""\
# API Reference

Complete alphabetical index of every public symbol exported by `seraplot`.

Index alphabétique complet de tous les symboles publics exportés par `seraplot`.

---

## Module: `seraplot`

```python
import seraplot as sp
```

---

## Chart Functions — 2D

{table(charts_2d)}

---

## Chart Functions — 3D

{table(charts_3d)}

---

## Chart Functions — Map

{table(charts_map)}

---

## Machine Learning

{table(ml)}

---

## Configuration

{table(config)}
"""

out_path = os.path.join(DOCS, "api", "index.md")
os.makedirs(os.path.dirname(out_path), exist_ok=True)
with open(out_path, "w", encoding="utf-8") as f:
    f.write(out)
print(f"Generated {out_path}")
