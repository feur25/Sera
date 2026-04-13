#!/usr/bin/env python3
"""
Generate live chart previews and embed them into each chart documentation page.

For each chart .md file, this script:
  1. Extracts the first Python code example from the ## Examples section
  2. Runs it to produce a SeraPlot Chart object
  3. Grabs the .html property (fully self-contained HTML+JS)
  4. Injects it as a collapsible <details> block right after the example

SeraPlot charts are self-contained HTML — tooltips, WebGL 3D rotation, and all
interactivity work without any server or CDN.

This script is run during CI, after `pip install seraplot`, before `mdbook build`.
It modifies the .md files in-place; the changes are never committed to the repo.
"""
import os
import re
import subprocess
import sys
import tempfile

DOCS = os.path.join(os.path.dirname(__file__), "..", "docs")


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def extract_first_example(content: str) -> str | None:
    """Return the source of the first ```python block after ## Examples."""
    lines = content.splitlines()
    in_examples = False
    in_code = False
    code_lines: list[str] = []

    for line in lines:
        if re.match(r"^## Examples?\b", line):
            in_examples = True
            continue
        if not in_examples:
            continue
        if line.strip().startswith("```python"):
            in_code = True
            code_lines = []
            continue
        if in_code and line.strip() == "```":
            if code_lines:
                return "\n".join(code_lines)
            in_code = False
            continue
        if in_code:
            code_lines.append(line)

    return None


def find_chart_var(code: str) -> str:
    """Return the first variable assigned from an sp.* call."""
    m = re.search(r"^(\w+)\s*=\s*sp\.", code, re.MULTILINE)
    return m.group(1) if m else "chart"


def run_example(code: str, chart_var: str) -> str | None:
    """Execute code and return the chart HTML string, or None on failure."""
    wrapper = (
        "import seraplot as _sp\n"
        "_sp.set_auto_display(False)\n"
        + code
        + f"\nimport sys\nsys.stdout.buffer.write({chart_var}.html.encode('utf-8'))\n"
    )
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".py", delete=False, encoding="utf-8"
    ) as f:
        f.write(wrapper)
        tmp = f.name

    try:
        result = subprocess.run(
            [sys.executable, tmp],
            capture_output=True,
            timeout=30,
        )
        if result.returncode == 0 and result.stdout:
            return result.stdout.decode("utf-8")
        if result.stderr:
            snippet = result.stderr.decode("utf-8", errors="replace")[:300]
            print(f"    stderr: {snippet}")
    except subprocess.TimeoutExpired:
        print("    Timeout")
    except Exception as exc:
        print(f"    Error: {exc}")
    finally:
        try:
            os.unlink(tmp)
        except OSError:
            pass

    return None


def inject_preview(content: str, chart_html: str) -> str:
    """Insert a <details> preview block right after the first example code block."""
    preview = (
        "\n\n<details open>\n"
        '<summary style="cursor:pointer;font-weight:600;padding:4px 0">'
        "&#9654; Live Preview</summary>\n\n"
        '<div style="width:100%;overflow:auto;border-radius:6px;margin:8px 0">\n'
        + chart_html.strip()
        + "\n</div>\n\n</details>\n"
    )
    pattern = r"(## Examples?\b.*?```python.*?```)"
    m = re.search(pattern, content, re.DOTALL)
    if m:
        pos = m.end()
        return content[:pos] + preview + content[pos:]
    return content


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def process_dir(rel_dir: str) -> None:
    d = os.path.join(DOCS, rel_dir)
    if not os.path.isdir(d):
        return

    for fname in sorted(os.listdir(d)):
        if not fname.endswith(".md") or fname == "index.md":
            continue

        path = os.path.join(d, fname)
        with open(path, encoding="utf-8-sig") as f:
            content = f.read()

        if "<details" in content:
            print(f"  skip (preview exists): {rel_dir}/{fname}")
            continue

        code = extract_first_example(content)
        if not code:
            print(f"  skip (no example): {rel_dir}/{fname}")
            continue

        chart_var = find_chart_var(code)
        print(f"  {rel_dir}/{fname} ...", end=" ", flush=True)
        html = run_example(code, chart_var)
        if html:
            new_content = inject_preview(content, html)
            with open(path, "w", encoding="utf-8", newline="\n") as f:
                f.write(new_content)
            print(f"OK ({len(html):,} chars)")
        else:
            print("FAILED (skipped)")


if __name__ == "__main__":
    try:
        import seraplot  # noqa: F401
    except ImportError:
        print("seraplot not installed — skipping preview generation")
        sys.exit(0)

    print("Generating chart previews...")
    for d in ["charts/2d", "charts/3d", "charts/map", "ml"]:
        process_dir(d)
    print("Done.")
