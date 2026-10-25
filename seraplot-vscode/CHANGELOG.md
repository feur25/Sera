# Changelog

## 0.8.1 - 2026-10-25

- **Fix — Live Preview crashed on unsaved/untitled files** (`FileNotFoundError: ... Untitled-1`): the file path handed to Python/Node/dotnet was always the editor's on-disk `fileName`, which doesn't exist for an untitled buffer and doesn't reflect unsaved edits for a saved one either (despite `seraplot.liveType`'s own description promising "no need to save"). Live Preview (Python and JavaScript/TypeScript) now always mirrors the live editor buffer to a throwaway file right before rendering, so untitled files work and "live as you type" actually reflects what you're typing. C# still requires a saved file (it compiles the whole project via `dotnet run`, so a buffer copy can't safely sit in the same project without colliding on top-level `Main`).
- **Fix — dead `SeraPlot: Check Installation` command**: declared in the command palette since forever, never had a handler, clicking it errored "command not found". Implemented it for real — checks Python, Node and .NET/dotnet availability for all three preview languages and reports back.
- **Fix — stale `feur25.github.io/seraplot` / `github.com/feur25/seraplot` links**: the repo renamed to `Sera`; "Open Documentation" and the various package.json/README URLs now point at the real ones.
- **Design**: the Home sidebar and Live Preview toolbar now match Chart Studio's actual hardcoded dark palette (`#0b1018`/`#30363d`/`#94a3b8`/`#a78bfa`/`#6366f1→#8b5cf6`) instead of deferring to the user's VS Code theme variables, which is why they looked inconsistent with the rest of the extension before. Also toned down "primary" button usage to one per section, matching Chart Studio's own restraint.
- `seraplot.autoInstall`, `seraplot.uninstall` and the new `seraplot.checkInstall` are now in the command palette (they worked from the Home sidebar before, just weren't reachable via Ctrl+Shift+P).

## 0.8.0 - 2026-10-25

- **Cross-language Live Preview**: detects and renders Python, C#, JavaScript and TypeScript files, not just Python. Live Preview, Gallery and the status bar all recognize the three new languages.
  - JavaScript/TypeScript: spawns `node`, requires the file fresh on every run, captures `module.exports = chart` or any `chart.save(path)` call.
  - C#: runs `dotnet run` against the nearest `.csproj` to the open file and picks up any `chart.Save("*.html")` written during the run.
  - New `seraplot.nodePath` and `seraplot.dotnetPath` settings, alongside the existing `seraplot.pythonPath`.
- **Redesign**: Home sidebar and Live Preview toolbar now share a dark design system (gradient accents, status dots, icon set) matching the docs site, plus a live language-detection badge.
- **Fix**: the "Insert Chart…" and "Insert sp.config(…)" command titles were corrupted mojibake in the command palette.

## 0.6.9 - 2026-06-22

- **Fix — Live Preview render failed with `AttributeError: module 'seraplot' has no attribute 'chart_info'`**: `chart_info` was implemented in Rust (`plot::utils::chart_info`) and listed as a "custom Python wrapper" function in the build-time codegen allowlist, but the actual `#[pyfunction]` wrapper converting a `Chart` object into the JSON payload it expects was never written. Added it and wired it into the module.
- **Fix — `bubble` (and any chart relying on shared per-family `common.rs` layout helpers) rendered an empty/0-byte chart even when called with every field listed in `required_params()`**: the build-time parameter scanner only scanned each variant's own file (e.g. `bubble/basic.rs`) for `cfg.<field>` usages; fields only referenced from the family's shared `common.rs` (like `sizes`, used by `compute_layout`) were silently missing from the registry. The scanner now also merges fields referenced in a sibling `common.rs`/`shared.rs`.
- Requires seraplot >= 2.7.10 (rebuilt wheel).

## 0.6.8 - 2026-06-22

- **Fix — Chart Studio loaded with an empty chart list**: `chart_variants`, `params`, `required_params` and `chart_themes` are computed at Rust build time from the real per-variant parameter registry (auto-extracted from `src/plot/**`), but were never wired into the PyO3 module (`#[sera_bind(serde)]` only emits FFI/WASM bindings, not Python). `introspect.py` was therefore probing the generic `(*args, **kwargs)` Python wrappers for real signatures and always failing. Added explicit Python bindings for these registries and rewired chart catalog discovery in `introspect.py` to read from them instead of `inspect.signature`. Requires seraplot >= 2.7.10 (rebuilt wheel).
- ML model/function catalog (`ml_models`, `ml_functions`) is still empty — same class of issue but no equivalent params registry exists yet on the ML side; tracked separately.

## 0.6.7 - 2026-06-22

- **Fix — Chart Studio failed to load (`ImportError: cannot import name 'PyDataset'`)**: the Rust `Dataset`/`DatasetStats` PyO3 classes are exposed to Python under those names (via `#[pyclass(name = "Dataset")]`), but `seraplot/__init__.py` was importing them as `PyDataset`/`PyDatasetStats`, which never existed on the compiled module. Any seraplot install hit this on import, which broke the whole catalog/introspection pipeline. Requires seraplot >= 2.7.10 (rebuilt wheel).

## 0.6.0 - 2026-05-02

- **Chart Studio UI overhaul**: every panel is now resizable with the mouse and collapsible.
  - Drag the vertical handle between the chart list and the configuration column to resize the sidebar.
  - Drag the horizontal handle below the live mini-preview to grow / shrink it.
  - Click the arrow buttons in each panel header to collapse / expand the chart list and the live preview.
  - Click any section heading inside the configuration form to fold / unfold that section.
- **Light & dark themes**: toggle between the classic dark Studio and a clean white theme via the `☾ Dark` / `☀ Light` button in the header.
- **Persistent state**: all of the above plus the currently selected chart, kwargs, chain methods, theme, title, labels and values are persisted via the webview state, so the Studio reopens exactly where you left it.
- Requires seraplot >= 2.4.25 for the universal `orientation_option` rotation across **all** chart families (boxplot violin variant, etc.).

## 0.5.9 - 2026-05-03

- **Orientation params surfaced**: every chart now exposes two new kwargs in completions / Studio:
  - `orientation="horizontal"` — native flipped-axis layout for `bar`, `grouped_bar`, `stacked_bar`, `boxplot`, `violin`, `histogram`, `lollipop`, `waterfall` (requires seraplot >= 2.4.23).
  - `orientation_option=1|2|3|4` — universal CSS rotation applicable to every inherited chart (1=0°, 2=90°, 3=180°, 4=270°). Requires seraplot >= 2.4.24.
- **New snippets**: `spbarh` (horizontal bar), `spbarrot` (4-way rotation), `sprotate` (chain `.rotate(deg)`).
- Common kwargs catalog updated so Studio shows orientation options without re-introspection.

## 0.5.6 - 2026-05-02

- **Fix — Variant enum now actually rendered**: the Studio variant `<select>` is built from real multiline JS instead of a malformed inline string, so picking **Bar** shows the full enum (`basic`, `horizontal`, `grouped`, `stacked`, `relative`, `grouped_stacked`, `marimekko`, `pictogram`, `multicategory`) and likewise for `bubble`, `scatter`, `histogram`, `heatmap`, `line`. Changing the variant updates the live preview and the inserted snippet (`variant="…"`).
- **Fix — Insert at cursor now detects identifiers**: `labels`, `values`, `df["x"]`, `top_brands.values.tolist()` and other expressions are recognised as data sources. The redundant `labels = […]` / `values = […]` prelude is only emitted per-variable, only when the current file does not already define it.
- **Fix — `export_button()` actually downloads**: the framework's injected JS now appends the `<a>` to `document.body` before clicking and revokes the object URL afterward; clicking the arrow now reliably saves `chart.html` in Chrome / Edge / Firefox / VS Code's webview. Falls back to opening the chart in a new tab if the browser blocks the download.

## 0.5.3 - 2026-05-01

- **Variants vs aliases**: Studio now shows two distinct selectors. **Alias** picks an alternative spelling of the same callable (`bar`, `bars`, `bar_unified`, …). **Variant** picks a sub-type within the chart family — `bar` exposes `basic`, `horizontal`, `grouped`, `stacked`, `relative`, `grouped_stacked`, `marimekko`, `pictogram`, `multicategory`; `bubble`, `scatter`, `histogram`, `heatmap`, `line` likewise. Selecting a variant injects `variant="…"` into the call.
- **Fix — Insert at cursor opens new file**: focus moving to the Studio webview no longer makes the extension fall through to a brand-new untitled doc. The last active Python editor is tracked and re-focused before the snippet is inserted.
- **Fix — Mini-preview not centered**: removed conflict between flexbox centering and absolute iframe positioning; scaling is recomputed via `requestAnimationFrame`, two follow-up ticks, and a `ResizeObserver`, so the chart stays centered after layout settles or the panel resizes.
- **Floating window mode**: third `seraplot.previewMode` value `"window"` opens the preview in its own OS window via `workbench.action.moveEditorToNewWindow`. Toolbar **Float** button performs the move on demand.
- **Studio layout**: 2-column grid; mini-preview lives at the top of the configuration column (no more right rail). Chart families with a single iframe scale to fit instead of overflowing.

## 0.5.0 - 2026-05-01

- **Chart Studio rework**: per-chart kwargs + all chain methods (`.set_margin`, `.border_radius`, `.scale`, …) discovered from wheel; theme picker; alias/variant chips shown under chart name; code preview updates live.
- **Live Preview panel mode**: preview now renders in the VS Code bottom panel by default (draggable, resizable, dockable anywhere). Switch to editor-tab mode at any time via `SeraPlot: Switch Preview Mode` or the **Mode** button inside the preview toolbar.
- **Toggle preview**: new `SeraPlot: Toggle Live Preview` command (eye icon) in the editor title bar — shows or hides the preview without navigating away.
- **Fix — NameError `labels`/`values`**: Studio render and Insert at cursor now detect non-literal expressions and substitute safe defaults; inserted blocks include a `labels = …` / `values = …` prelude when the file doesn't define them.
- **Alias / variant display**: Studio shows all aliases of the selected chart (e.g. `bar_family`, `bars_unified`, `build_bar` for `bar`) as clickable chips.
- **Settings**: new `seraplot.previewMode` (`"panel"` | `"tab"`, default `"panel"`); `seraplot.previewColumn` now only applies in tab mode.

## 0.1.1 - 2026-04-23

- Added repository, license, homepage and bugs metadata for Marketplace listing.
- Bundled MIT LICENSE file.

## 0.1.0 - 2026-04-22

- Initial release.
- Commands: `SeraPlot: Live Preview`, `SeraPlot: Open Theme Studio`, `SeraPlot: Open Gallery`.
- Snippets: import, bar, scatter, dashboard, automl, drift.
- Settings: `seraplot.pythonPath`, `seraplot.autoReload`.
