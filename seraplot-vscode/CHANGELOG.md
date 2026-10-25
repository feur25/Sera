# Changelog

## 0.10.0 - 2026-10-25

- **Fix — Chart Studio's live preview was empty for several chart families**: heatmap, candlestick/OHLC, sankey, chord and wordcloud's network variant always rendered 0 bytes, and treemap/choropleth rendered but silently without their real hierarchy/map data. The Studio always called `sp.<chart>(title, labels=[...], values=[...])` regardless of what the chart actually needed. It now pulls each chart's own documentation demo data (`sp.demo(id, variant)`) and uses that instead — real per-family data (`col_labels`+matrix for heatmap, `open/high/low/close` for candlestick, `edges_i/edges_j/edges_w` for sankey, etc.). If you've typed real data into the Chart data panel (or it was detected from your open file), that still takes priority over the demo data.
- **Theme Studio — Extension Appearance, much more explicit**: went from 4 color pickers to 11, grouped into "Sidebar & panels", "Buttons" and "Accent & text", each with a one-line hint saying exactly what it recolors. The live preview mockup now shows a full mini sidebar (title, detected-language card, section label, primary button, regular button, a hovered-looking button, and a hint box) so every token's effect is visible at a glance instead of two generic rows.
- **Home sidebar — rename sections**: every section, built-in or custom, now has a rename control next to delete. Deleting the last remaining section is blocked instead of leaving an empty sidebar.
- **Chart Studio — chart-only list**: ML models/functions/transformers no longer appear in Chart Studio's chart list (they never had a live preview there, which made clicking them feel broken). They're still reachable from the Gallery panel's ML cards, which now insert a ready-to-edit snippet directly instead of redirecting into an empty Chart Studio.
- **Smoother interactions throughout Chart Studio**: the chart-list panel and mini-preview now ease open/closed (width/height transition) instead of snapping instantly, and config-form sections collapse with a real height animation instead of popping away. Dragging a resize handle correctly suspends the easing so it doesn't fight your mouse. Favorite stars, folder icons, method chips and the "+ New section" button all got hover/press polish.

## 0.9.1 - 2026-10-25

- **Home sidebar — rename sections**: every section (built-in or custom) now has a rename (pencil) control next to its name, not just delete. Deleting the last remaining section is blocked rather than leaving you with an empty sidebar.
- **Chart Studio — the same favorites & folders as the Gallery**: the chart list on the left of Chart Studio now has its own ★/⊞ controls per chart, a ★ Favorites group, folder groups and a "+ Folder" button — backed by the exact same `globalState` data as the Gallery tree view, so starring or filing a chart from either place shows up instantly in the other. Persisted forever, across every future update.
- **Live Preview — drag to reorder charts**: when a file produces more than one chart, a row of numbered chips now sits under the toolbar; drag them to change which chart is "1st", "2nd", etc. instead of only stepping through with Prev/Next. The currently-viewed chart follows its content when you reorder, rather than jumping to whatever ends up in that slot.

## 0.9.0 - 2026-10-25

- **Fix — wrong color palette**: the Home sidebar, Live Preview toolbar and every other panel were recolored to an indigo/purple palette that was never the right reference. They now match the documentation site's actual warm brown/amber identity (`#2b211d`/`#3b2e2a` surfaces, `#e69f67`/`#d98a4a` accent) throughout.
- **Gallery — favorites & custom folders**: star any chart or a specific sub-variant to pin it under a new ★ Favorites group at the top of the Gallery tree. Create named folders (right-click a chart → Add to folder…, or the folder icon in the Gallery title bar) to organize charts into your own structure; rename/delete folders and remove items from the tree's context menu. A new search box (magnifying-glass icon) filters the whole catalog live. Everything is stored in `globalState`, so it survives extension updates and reinstalls.
- **Chart Studio — favorite chainable methods**: every method chip (`.set_bg`, `.crosshair`, …) now has a small ★ toggle; favorited methods sort to the front of the grid so the ones you use constantly aren't buried in the full method list. Persisted the same way as chart favorites.
- **Theme Studio — recolor the extension itself**: a new "Extension Appearance" section lets you pick the Home sidebar's background, surface, accent and text colors with a live mini-preview, independent from chart palettes. Saved forever via `globalState`; "Reset to default" restores the shipped warm palette.
- **Home sidebar — drag to reorder, create your own sections**: every button has a drag handle and can be dragged to reorder within its section or moved into a different one, including custom sections you create via "+ New section…" at the bottom of the sidebar. Delete a custom section (× next to its name) and its buttons fall back to a default section rather than disappearing. Layout is persisted via `globalState`, so it's there after every reload and every future update of the extension.

## 0.8.1 - 2026-10-25

- **Fix — Live Preview crashed on unsaved/untitled files** (`FileNotFoundError: ... Untitled-1`): the file path handed to Python/Node/dotnet was always the editor's on-disk `fileName`, which doesn't exist for an untitled buffer and doesn't reflect unsaved edits for a saved one either (despite `seraplot.liveType`'s own description promising "no need to save"). Live Preview (Python and JavaScript/TypeScript) now always mirrors the live editor buffer to a throwaway file right before rendering, so untitled files work and "live as you type" actually reflects what you're typing. C# still requires a saved file (it compiles the whole project via `dotnet run`, so a buffer copy can't safely sit in the same project without colliding on top-level `Main`).
- **Fix — dead `SeraPlot: Check Installation` command**: declared in the command palette since forever, never had a handler, clicking it errored "command not found". Implemented it for real — checks Python, Node and .NET/dotnet availability for all three preview languages and reports back.
- **Fix — stale `feur25.github.io/seraplot` / `github.com/feur25/seraplot` links**: the repo renamed to `Sera`; "Open Documentation" and the various package.json/README URLs now point at the real ones.
- **Design**: the Home sidebar and Live Preview toolbar no longer defer to the user's VS Code theme variables, which is why they looked inconsistent with the rest of the extension before. Also toned down "primary" button usage to one per section. (Superseded by 0.9.0 — the palette landed here was the wrong reference; see below.)
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
