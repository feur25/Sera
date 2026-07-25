# SeraPlot — cahier des charges (known gaps and architecture notes)

Living tracker of known issues, architectural constraints, and open gaps.
Not user-facing documentation — this is a maintenance checklist.

## Fixed

- **Chainable methods silently no-op on `joint()`/`facet()` output.** Every
  method like `no_background()`, `set_bg()`, `hide_legend()`, `font()`, etc.
  worked by injecting a `<style>`/`<script>` snippet right before the chart's
  own `</head>`/`</body>`/`</svg>`. `joint()`/`facet()` return a *composed*
  page — an outer shell with no `<svg>` of its own, wrapping each real chart
  in a `<iframe srcdoc="...">` (a separate browsing context CSS/JS from the
  parent document can never reach). ~95 call sites across
  `bindings/chart_methods/*.rs`, `services/plot/chart_input/html.rs`, and
  `lib.rs` now route through `services/plot/html/hover.rs`'s
  `inject_before_head` / `inject_before_body` / `inject_before_svg_close` /
  `insert_after_svg_open`, all built on one shared `apply_deep` primitive that
  recursively decodes, transforms, and re-encodes nested `srcdoc` content.
  Fast path (no nested iframe) is byte-for-byte identical to the old
  behavior — this is not expected to change any output for a normal,
  non-composed chart.
  - **Not yet covered** (found late, deliberately left alone rather than
    rushed): `mask_circle()`, `mask_poly()` (rewrite the `<svg>` tag's own
    attributes, not just insert nearby — needs a different primitive) and
    `hover_slots()` (finds/replaces a `var data=...;` JS literal). All three
    will silently no-op on `joint()`/`facet()` output today, the same way
    `no_background()` used to.

- **Every Jupyter cell displayed its chart twice.** `AUTO_DISPLAY` defaulted
  to `true`, so `Chart::new()` eagerly called `display()` on construction —
  *in addition to* Jupyter's own "last expression in a cell gets displayed"
  convention. Defaulted to `false`; still togglable via `sp.set_auto_display()`
  for the (rarer) case of wanting a chart to appear immediately even when it
  isn't a cell's final expression.

- **`heatmap(variant="polar")` had no `chart_demo`**, unlike every sibling
  variant — since `"polar"` also happens to be a 3D scene alias
  (radial/circular/polar), the exact-match demo lookup failed and fell back
  to emitting `scene="polar"` instead of `variant="polar"`, a kwarg heatmap
  never reads. That's why the playground's live-preview panel rendered a
  plain grid while the side panel (reading the real static preview)
  correctly showed the wheel. Fixed by giving `polar.rs` its own demo.

- **`heat_scatter` was aliased to the wrong family** (`joint(variant="hexbin")`
  instead of `heatmap(variant="bubble")` — verified against the actual
  seaborn page, which draws a correlation matrix as sized/colored scatter
  dots, not a 2D density plot).

- **`heatmap:cluster`** was a fake ("sort rows/cols by sum"), now real
  average-linkage agglomerative clustering.

- **`kde` was 1D-only**, so `joint()`'s legacy bivariate-KDE preset names
  (`layered_bivariate`, `joint_kde`, `smooth_bivariate_kde`, ...) silently
  dropped `y` and rendered a flat density curve. `kde` now has a genuine
  `contour` variant (product-kernel Gaussian KDE evaluated on a 2D grid),
  and the legacy names default `panel_variant="contour"` automatically.

- **`scatter:regression`** only ever fit one line across all points and only
  supported linear/quadratic. Added per-`categories=` grouped regression
  (one fit + legend entry per group) and a logistic fit type (Newton-
  Raphson/IRLS).

- **`heatmap:basic` and `heatmap:annotated` were visually identical** —
  `show_values` defaulted to `true` for every variant. Defaulted to `false`;
  `annotated`/`confusion`/`correlation`/`pivot` already force it `true`.

- **`heatmap:bubble`** had a hardcoded dark background inconsistent with
  every other heatmap variant.

## Known architecture constraint: two independent checkouts

`C:\Users\Quentin\Desktop\SeraPlot\v2\src` and
`C:\Users\Quentin\Desktop\SeraPlot\v2` (the outer directory) are **not** the
same git tree. `v2/src` is a standalone clone of `feur25/seraplot` — its own
repo root has `Cargo.toml`, `lib.rs`, `services/`, etc. directly. The outer
`v2/` is a subdirectory of a *different* repo (rooted at
`C:\Users\Quentin\Desktop\SeraPlot`) with its own, differently-structured
`v2/Cargo.toml` that does not exist anywhere in `origin/main`'s history —
it's a separate, manually-maintained copy, not a lagging mirror that a
`git pull` could reconcile safely.

All of this session's work lives only in `v2/src` (and is pushed to
`origin/main` from there). The outer `v2/`'s compiled `python/seraplot`
module may be built from a different, unrelated point in time — running
`seaborn_gallery.ipynb` from there is not guaranteed to reflect anything
described in this file. Run it from `v2/src`.

## Seaborn gallery coverage

See `seaborn_gallery.ipynb` for all 49 examples, each executed against a
real registered SeraPlot family/variant. Two are honest approximations, not
first-class charts yet:

- **`pointplot_anova`** — no dedicated mean+CI point-and-line-per-category
  chart. Approximated with `scatter(variant="labeled", error_bars_margin=...)`.
- **`radial_facets`** — no native polar/rose histogram. `facet(family="bar")`
  reproduces the faceting *technique* only.
