# SeraPlot for Power BI

A native Power BI custom visual that renders **SeraPlot** charts (bar, line, pie, area,
histogram, scatter) directly inside Power BI Desktop/Service, using the same Rust/WASM
chart engine that powers [seraplot](https://pypi.org/project/seraplot/) and its
[documentation site](https://feur25.github.io/Sera/).

Drag a category field into **Category / Labels** and a numeric field into **Values**
(or **Values** + **Y (scatter only)** for scatter plots), pick a chart type from the
format pane, and the visual renders it with SeraPlot's own engine — not a Power BI
native chart type re-skinned to look similar.

## How it works

The visual doesn't bundle SeraPlot's ~8.6MB WASM binary into the `.pbiviz` package.
Instead, on first render it loads `seraplot_bg.wasm` and its JS glue from the SeraPlot
docs site (`https://feur25.github.io/Sera/docs/theme/`) — the same build already used
by the interactive playground on every chart's documentation page — and calls the
matching `build<ChartType>()` WASM export (e.g. `buildBar`, `buildScatterChart`) with
the fields mapped from Power BI's `DataView`. The returned HTML is rendered in a
sandboxed (`allow-scripts` only, no `allow-same-origin`) iframe via a `blob:` URL.

This requires the `WebAccess` privilege (declared in `capabilities.json`) scoped to
those two asset URLs — Power BI will prompt for this permission on first load.

## Development

```
npm install
npm run package   # produces dist/*.pbiviz
npm start         # live-reload dev server for sideloading in Power BI Desktop
```

Sideload via Power BI Desktop's **Developer visual** (enable in
*Options → Preview features*) while `npm start` is running, or import the packaged
`.pbiviz` from `dist/` directly.

## Limitations / known gaps

- Only 6 chart types are wired up so far (bar/line/pie/area/histogram/scatter) out of
  SeraPlot's 90+ chart families — the `BUILD_FN` map in `src/visual.ts` is the place to
  extend this; any `build<X>` WASM export documented at the SeraPlot docs site works the
  same way.
- Requires network access to `feur25.github.io` at render time (see "How it works"
  above) — this has not been validated against Power BI Service's tighter tenant-level
  web-access policies, only local sideloading.
- Built and packaged successfully (`npm run package` produces a valid `.pbiviz`) and the
  WASM bridge itself was verified end-to-end in a headless browser against the live
  deployed engine, but full interactive behavior (resize, format-pane live updates,
  tooltips, cross-filtering) has not been exercised inside a real Power BI Desktop
  session — that needs manual verification since it's a Windows GUI app.
