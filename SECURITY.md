# Security Policy

SeraPlot is a Rust core (compiled to a Python extension via PyO3, and to
WebAssembly for the docs playground and embeddable charts) plus thin FFI/C#
bindings. It renders untrusted-looking data into HTML/SVG that end users may
open in a browser, so the main attack surface to care about is: can
attacker-controlled chart data (labels, values, hover content, custom
CSS/JS passed via `style()`/`script()`/`custom_css`/`custom_js`) escape the
chart's intended sandbox and execute arbitrary script in the host page, or
can a malformed input cause a panic/crash/resource exhaustion in the Rust
core.

## Supported Versions

Only the latest published release on [PyPI](https://pypi.org/project/seraplot/)
is supported with security fixes. There is no long-term support branch —
`pip install --upgrade seraplot` is the fix for any reported issue once a
patch ships.

| Version         | Supported |
| ---------------- | --------- |
| latest (`pip install seraplot`) | :white_check_mark: |
| anything older    | :x: |

## Reporting a Vulnerability

Please **do not open a public GitHub issue** for a suspected security
vulnerability (XSS via chart output, a Rust panic/UB reachable from Python
input, path traversal in `save()`/`export_*`/`canvas_save_named()`, etc.).

Instead, open a [GitHub Security Advisory](https://github.com/feur25/Sera/security/advisories/new)
on this repository (Security tab → "Report a vulnerability"), or email the
maintainer directly — contact details are on the
[Support & Contact](https://feur25.github.io/Sera/about/support.html) page.
Include:

- SeraPlot version (`sp.__version__`) and how it was installed (PyPI wheel,
  built from source, WASM/docs playground, VS Code extension, Power BI
  visual).
- A minimal reproduction: the exact `sp.<chart>(...)` call or `sp.Canvas`
  script and input data that triggers the issue.
- What you observed vs. what you expected (e.g. "arbitrary `<script>`
  content from a `labels` string ends up unescaped in the output HTML").

You should get an acknowledgement within a few days. Since this is a
single-maintainer project developed in spare time, please allow reasonable
time for a fix before any public disclosure — I'll keep you updated on
progress and credit you in the fix's changelog/release notes unless you'd
rather stay anonymous.

## Scope notes

A few things that are **by design** and not considered vulnerabilities on
their own:

- `Canvas.script()`, `Canvas.style()`, `custom_css`/`custom_js` chart
  parameters, and the Power BI/VS Code integrations all let *you* inject
  arbitrary HTML/CSS/JS into *your own* chart output on purpose — that's
  the intended escape hatch for advanced customization. Don't pass
  untrusted third-party input into these unless you trust it, the same way
  you wouldn't pass it to `eval()`.
- Rendered chart HTML is meant to be viewed by the person who generated it
  (a notebook, a local file, an embedded iframe you control) — it is not
  designed to defend against a malicious *viewer* of the page; it's
  designed to not let malicious *input data* (labels/values coming from
  elsewhere, e.g. a CSV you didn't write) escape into script execution
  when you didn't ask for that via the explicit custom CSS/JS hooks above.

Reports about *those* specific, documented escape hatches doing exactly
what they're documented to do aren't actionable — but if you find a way to
get script execution through plain chart *data* fields (labels, tooltip
text, category names, etc.) without touching `script()`/`custom_js`, that's
exactly the kind of report this policy is for.
