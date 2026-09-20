# Heatmap Chart 3D

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:340px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-3d-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(420px,1fr));gap:18px;margin-top:14px}
.sp-3d-card{border:1px solid rgba(128,128,128,.28);border-radius:12px;padding:6px 16px 14px}
.sp-3d-card h3{margin:10px 0 6px}
.sp-3d-uses{margin:4px 0;font-size:.9em;opacity:.85}
.sp-3d-card details{margin-top:10px}
</style>

## Signature

`sp.heatmap3d(title, labels=None, col_labels=None, values=None, *, variant="basic", matrix=None, orientation3d="iso", scene="default", theme="none", bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_heatmap3d_chart()`, `sp.heatmap_3d()`, `sp.heatmap3d_chart()`, `sp.heatmaps3d()`.

## Description

`sp.heatmap3d()` is the 3D twin of `sp.heatmap()`: **every one of the 20 heatmap variants has a 3D form**, selected with the same `variant` keyword and fed with the same data. Layouts come from shared, family-agnostic grid strategies (rectangular, weighted, bubble, radial, hex, ridge), so geometry is never hand-written per variant. Height and colour both encode the value.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `simple`, `default`, `matrix` | `labels, col_labels, values` | One column per cell on a rectangular grid: height and colour both follow the value. |
| `annotated` | `annotate`, `labeled`, `values` | `labels, col_labels, values` | Same landscape as basic; the value labels of the 2D variant become hover read-outs. |
| `categorical` | `category`, `discrete_labels`, `cat` | `labels, col_labels, values` | Columns take a palette colour per category value instead of a continuous colour scale. |
| `unequal` | `irregular`, `weighted`, `uneven` | `labels, col_labels, values, widths, ranges` | Cell footprints follow `widths` and `ranges` (or a default irregular grid), like the 2D variable-size cells. |
| `log` | `logarithmic`, `log_scale`, `log10` | `labels, col_labels, values` | Heights and colours follow a logarithmic scale, so small values stay visible next to large ones. |
| `discrete` | `binned`, `stepped`, `bands` | `labels, col_labels, values, bins` | Values quantised into `bins` steps (5 by default): the landscape becomes terraces of equal height. |
| `correlation` | `corr`, `diverging`, `pearson` | `labels, col_labels, values` | Diverging around zero: positive values rise above the floor, negative ones sink below it. |
| `density` | `imshow`, `viridis`, `smooth` | `labels, col_labels, values` | The grid is interpolated three times denser into a field of thin columns, coloured with viridis. |
| `contour` | `iso`, `isolines`, `level` | `labels, col_labels, values, bins` | Interpolated and quantised into contour levels: stepped bands that follow the iso-lines. |
| `temporal` | `calendar`, `time`, `date`, `timeseries` | `labels, col_labels, values` | Time grids interpolated into a smooth viridis terrain, one column per interpolated cell. |
| `cluster` | `clustermap`, `dendrogram`, `reorder` | `labels, col_labels, values` | Rows and columns reordered by hierarchical clustering so similar lines sit side by side. |
| `bubble` | `size_scaled`, `circle_heatmap`, `punchcard` | `labels, col_labels, values` | The footprint of every column scales with its value: heavy cells fatten, light cells thin out. |
| `marginal` | `with_marginals`, `histograms`, `side_bars` | `labels, col_labels, values` | Row totals and column totals stand as extra columns beyond the grid edges, like marginal histograms. |
| `confusion` | `confusion_matrix`, `classifier`, `cm` | `labels, col_labels, values` | Diagonal cells (correct predictions) keep their full tone while off-diagonal cells are muted. |
| `pivot` | `pivot_table`, `totals`, `summary` | `labels, col_labels, values` | Row and column totals are appended right next to the grid, pivot-table style. |
| `polar` | `wheel`, `clock`, `radial_heat`, `carbon_wheel` | `labels, col_labels, values` | Rows become concentric rings and columns become angular sectors around the centre. |
| `radial_cluster` | `circular_cluster`, `circos`, `radial_dendrogram`, `circular_dendrogram` | `labels, col_labels, values` | The polar layout with rows and columns reordered by hierarchical clustering. |
| `hex_grid` | `hexbin_grid`, `hex_calendar`, `honeycomb`, `hex_matrix` | `labels, col_labels, matrix` | Staggered grid where odd rows shift half a cell, approximating a hexagonal tiling. |
| `horizon` | `horizon_chart`, `stock_ridge`, `banded_horizon`, `sentiment_bands` | `labels, col_labels, matrix` | Every row is a thin ridge line, rows spaced apart like stacked time-series bands. |
| `moods` | `mood_matrix`, `punchcard_grouped`, `library`, `sentiment_grid` | `labels, col_labels, matrix` | Rows grouped by their first `::` token, with a gap between groups. |

## Data

`labels` are the row names, `col_labels` the column names and `values` the row-major matrix (or pass `matrix=[[...], ...]`). The legacy `categories` / `x_labels` spelling of `heatmap3d` keeps working.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="heatmap3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-cosmic.html"></iframe></div>
</div>

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One column per cell on a rectangular grid: height and colour both follow the value.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>annotated</code></h3><p>Same landscape as basic; the value labels of the 2D variant become hover read-outs.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-annotated.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Annotated 3D&quot;,
    variant=&quot;annotated&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>categorical</code></h3><p>Columns take a palette colour per category value instead of a continuous colour scale.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-categorical.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Categorical 3D&quot;,
    variant=&quot;categorical&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>unequal</code></h3><p>Cell footprints follow <code>widths</code> and <code>ranges</code> (or a default irregular grid), like the 2D variable-size cells.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values, widths, ranges</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-unequal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Unequal 3D&quot;,
    variant=&quot;unequal&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>log</code></h3><p>Heights and colours follow a logarithmic scale, so small values stay visible next to large ones.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-log.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Log 3D&quot;,
    variant=&quot;log&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>discrete</code></h3><p>Values quantised into <code>bins</code> steps (5 by default): the landscape becomes terraces of equal height.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-discrete.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Discrete 3D&quot;,
    variant=&quot;discrete&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>correlation</code></h3><p>Diverging around zero: positive values rise above the floor, negative ones sink below it.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-correlation.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Correlation 3D&quot;,
    variant=&quot;correlation&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>density</code></h3><p>The grid is interpolated three times denser into a field of thin columns, coloured with viridis.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-density.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Density 3D&quot;,
    variant=&quot;density&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>contour</code></h3><p>Interpolated and quantised into contour levels: stepped bands that follow the iso-lines.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-contour.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Contour 3D&quot;,
    variant=&quot;contour&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>temporal</code></h3><p>Time grids interpolated into a smooth viridis terrain, one column per interpolated cell.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-temporal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Temporal 3D&quot;,
    variant=&quot;temporal&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>cluster</code></h3><p>Rows and columns reordered by hierarchical clustering so similar lines sit side by side.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-cluster.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Cluster 3D&quot;,
    variant=&quot;cluster&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>bubble</code></h3><p>The footprint of every column scales with its value: heavy cells fatten, light cells thin out.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-bubble.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Bubble 3D&quot;,
    variant=&quot;bubble&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>marginal</code></h3><p>Row totals and column totals stand as extra columns beyond the grid edges, like marginal histograms.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-marginal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Marginal 3D&quot;,
    variant=&quot;marginal&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>confusion</code></h3><p>Diagonal cells (correct predictions) keep their full tone while off-diagonal cells are muted.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-confusion.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Confusion 3D&quot;,
    variant=&quot;confusion&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>pivot</code></h3><p>Row and column totals are appended right next to the grid, pivot-table style.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-pivot.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Pivot 3D&quot;,
    variant=&quot;pivot&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>polar</code></h3><p>Rows become concentric rings and columns become angular sectors around the centre.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-polar.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Polar 3D&quot;,
    variant=&quot;polar&quot;,
    labels=[&quot;North&quot;, &quot;South&quot;, &quot;East&quot;, &quot;West&quot;],
    col_labels=[&quot;0h&quot;, &quot;3h&quot;, &quot;6h&quot;, &quot;9h&quot;, &quot;12h&quot;, &quot;15h&quot;, &quot;18h&quot;, &quot;21h&quot;],
    values=[210, 190, 180, 220, 260, 240, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>radial_cluster</code></h3><p>The polar layout with rows and columns reordered by hierarchical clustering.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-radial_cluster.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Radial Cluster 3D&quot;,
    variant=&quot;radial_cluster&quot;,
    labels=[&quot;GSM1&quot;, &quot;GSM2&quot;, &quot;GSM3&quot;, &quot;GSM4&quot;, &quot;GSM5&quot;, &quot;GSM6&quot;],
    col_labels=[&quot;10001_at&quot;, &quot;10005_at&quot;, &quot;10013_at&quot;, &quot;10020_at&quot;, &quot;10025_at&quot;, &quot;10004_at&quot;, ...],
    values=[9, 9, 10, 8, 10, 7, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>hex_grid</code></h3><p>Staggered grid where odd rows shift half a cell, approximating a hexagonal tiling.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-hex_grid.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Hex Grid 3D&quot;,
    variant=&quot;hex_grid&quot;,
    labels=[&quot;Figma Design Systems&quot;, &quot;Product Management&quot;, &quot;Design Research&quot;, &quot;Dev Web&quot;],
    col_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;, &quot;Jun&quot;, ...],
    matrix=[[61, 25, 42, 25, 61, 42, ...], [25, 61, 86, 42, 61, 25, ...], [25, 42, 25, 86, 42, 25, ...], [62, 54, 54, 21, 12, 25, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>horizon</code></h3><p>Every row is a thin ridge line, rows spaced apart like stacked time-series bands.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-horizon.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Horizon 3D&quot;,
    variant=&quot;horizon&quot;,
    labels=[&quot;Adobe&quot;, &quot;Alphabet&quot;, &quot;Amazon&quot;, &quot;AMD&quot;, &quot;Apple&quot;, &quot;ASML Holding&quot;, ...],
    col_labels=[&quot;Jan 20&quot;, &quot;&quot;, &quot;&quot;, &quot;&quot;, &quot;Feb 20&quot;, &quot;&quot;, ...],
    matrix=[[-0.84, -1.19, -1.6, -1.09, -0.71, 0.02, ...], [0.46, 0.48, -0.37, -0.64, -1.39, -0.6, ...], [0.71, 0.18, 0.52, 0.07, -0.11, 0.18, ...], [-0.19, -0.05, -0.44, -0.08, -0.88, -0.14, ...], [0.92, 0.7, 1.16, 1.18, 1.32, 0.68, ...], [-0.02, 0.5, -0.25, -0.73, -0.34, -0.67, ...], ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>moods</code></h3><p>Rows grouped by their first <code>::</code> token, with a gap between groups.</p><p class="sp-3d-uses">Uses: <code>labels, col_labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-moods.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Moods 3D&quot;,
    variant=&quot;moods&quot;,
    labels=[&quot;Rock::Ironclad Sky::Silver Gravity&quot;, &quot;Rock::Ironclad Sky::Hollow Bloom&quot;, &quot;Rock::Ironclad Sky::Distant Ember&quot;, &quot;Rock::Voltage Parade::Slow Gravity&quot;, &quot;Rock::Voltage Parade::Wild Gravity&quot;, &quot;Rock::Rust Horizon::Distant Bloom&quot;, ...],
    col_labels=[&quot;Angry&quot;, &quot;Passionate&quot;, &quot;Excited&quot;, &quot;Happy&quot;, &quot;Curious&quot;, &quot;Content&quot;, ...],
    values=[0.891, 0.941, 0.615, 0.129, 0.218, 0.119, ...],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="heatmap3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.heatmap3d(title, labels=None, col_labels=None, values=None, *, variant="basic", matrix=None, orientation3d="iso", scene="default", theme="none", bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_heatmap3d_chart()`, `sp.heatmap_3d()`, `sp.heatmap3d_chart()`, `sp.heatmaps3d()`.

<h2>Description</h2>

`sp.heatmap3d()` est le jumeau 3D de `sp.heatmap()` : **chacune des 20 variantes de heatmap a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes données. Les dispositions viennent de stratégies de grille partagées et indépendantes de la famille (rectangulaire, pondérée, bulle, radiale, hex, crête) : la géométrie n'est jamais écrite à la main par variante. La hauteur et la couleur encodent la valeur.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `simple`, `default`, `matrix` | `labels, col_labels, values` | Une colonne par cellule sur une grille rectangulaire : la hauteur et la couleur suivent la valeur. |
| `annotated` | `annotate`, `labeled`, `values` | `labels, col_labels, values` | Même paysage que basic ; les étiquettes de valeur de la variante 2D deviennent des infobulles. |
| `categorical` | `category`, `discrete_labels`, `cat` | `labels, col_labels, values` | Les colonnes prennent une couleur de palette par valeur de catégorie plutôt qu'une échelle continue. |
| `unequal` | `irregular`, `weighted`, `uneven` | `labels, col_labels, values, widths, ranges` | Les empreintes des cellules suivent `widths` et `ranges` (ou une grille irrégulière par défaut), comme les cellules de taille variable du 2D. |
| `log` | `logarithmic`, `log_scale`, `log10` | `labels, col_labels, values` | Hauteurs et couleurs suivent une échelle logarithmique : les petites valeurs restent visibles à côté des grandes. |
| `discrete` | `binned`, `stepped`, `bands` | `labels, col_labels, values, bins` | Valeurs quantifiées en `bins` paliers (5 par défaut) : le paysage devient des terrasses de hauteur égale. |
| `correlation` | `corr`, `diverging`, `pearson` | `labels, col_labels, values` | Divergent autour de zéro : les valeurs positives montent au-dessus du plancher, les négatives descendent en dessous. |
| `density` | `imshow`, `viridis`, `smooth` | `labels, col_labels, values` | La grille est interpolée trois fois plus dense en un champ de fines colonnes, colorées en viridis. |
| `contour` | `iso`, `isolines`, `level` | `labels, col_labels, values, bins` | Interpolée et quantifiée en niveaux de contour : des bandes en escalier qui suivent les iso-lignes. |
| `temporal` | `calendar`, `time`, `date`, `timeseries` | `labels, col_labels, values` | Grilles temporelles interpolées en un terrain viridis lisse, une colonne par cellule interpolée. |
| `cluster` | `clustermap`, `dendrogram`, `reorder` | `labels, col_labels, values` | Lignes et colonnes réordonnées par classification hiérarchique pour rapprocher les lignes semblables. |
| `bubble` | `size_scaled`, `circle_heatmap`, `punchcard` | `labels, col_labels, values` | L'empreinte de chaque colonne dépend de sa valeur : les cellules lourdes grossissent, les légères s'amincissent. |
| `marginal` | `with_marginals`, `histograms`, `side_bars` | `labels, col_labels, values` | Totaux de lignes et de colonnes en colonnes supplémentaires au-delà des bords, comme des histogrammes marginaux. |
| `confusion` | `confusion_matrix`, `classifier`, `cm` | `labels, col_labels, values` | Les cellules diagonales (bonnes prédictions) gardent leur teinte pleine, les autres sont atténuées. |
| `pivot` | `pivot_table`, `totals`, `summary` | `labels, col_labels, values` | Totaux de lignes et de colonnes ajoutés juste à côté de la grille, façon tableau croisé. |
| `polar` | `wheel`, `clock`, `radial_heat`, `carbon_wheel` | `labels, col_labels, values` | Les lignes deviennent des anneaux concentriques et les colonnes des secteurs angulaires autour du centre. |
| `radial_cluster` | `circular_cluster`, `circos`, `radial_dendrogram`, `circular_dendrogram` | `labels, col_labels, values` | La disposition polaire avec lignes et colonnes réordonnées par classification hiérarchique. |
| `hex_grid` | `hexbin_grid`, `hex_calendar`, `honeycomb`, `hex_matrix` | `labels, col_labels, matrix` | Grille décalée où les lignes impaires glissent d'une demi-cellule, approchant un pavage hexagonal. |
| `horizon` | `horizon_chart`, `stock_ridge`, `banded_horizon`, `sentiment_bands` | `labels, col_labels, matrix` | Chaque ligne est une fine crête, les lignes étant espacées comme des bandes de séries temporelles empilées. |
| `moods` | `mood_matrix`, `punchcard_grouped`, `library`, `sentiment_grid` | `labels, col_labels, matrix` | Lignes regroupées selon leur premier jeton `::`, avec un écart entre les groupes. |

<h2>Données</h2>

`labels` sont les noms de lignes, `col_labels` les noms de colonnes et `values` la matrice à plat par lignes (ou `matrix=[[...], ...]`). L'ancienne écriture `categories` / `x_labels` de `heatmap3d` continue de fonctionner.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="heatmap3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une colonne par cellule sur une grille rectangulaire : la hauteur et la couleur suivent la valeur.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>annotated</code></h3><p>Même paysage que basic ; les étiquettes de valeur de la variante 2D deviennent des infobulles.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-annotated.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Annotated 3D&quot;,
    variant=&quot;annotated&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>categorical</code></h3><p>Les colonnes prennent une couleur de palette par valeur de catégorie plutôt qu&#x27;une échelle continue.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-categorical.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Categorical 3D&quot;,
    variant=&quot;categorical&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>unequal</code></h3><p>Les empreintes des cellules suivent <code>widths</code> et <code>ranges</code> (ou une grille irrégulière par défaut), comme les cellules de taille variable du 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values, widths, ranges</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-unequal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Unequal 3D&quot;,
    variant=&quot;unequal&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>log</code></h3><p>Hauteurs et couleurs suivent une échelle logarithmique : les petites valeurs restent visibles à côté des grandes.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-log.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Log 3D&quot;,
    variant=&quot;log&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>discrete</code></h3><p>Valeurs quantifiées en <code>bins</code> paliers (5 par défaut) : le paysage devient des terrasses de hauteur égale.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-discrete.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Discrete 3D&quot;,
    variant=&quot;discrete&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>correlation</code></h3><p>Divergent autour de zéro : les valeurs positives montent au-dessus du plancher, les négatives descendent en dessous.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-correlation.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Correlation 3D&quot;,
    variant=&quot;correlation&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>density</code></h3><p>La grille est interpolée trois fois plus dense en un champ de fines colonnes, colorées en viridis.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-density.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Density 3D&quot;,
    variant=&quot;density&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>contour</code></h3><p>Interpolée et quantifiée en niveaux de contour : des bandes en escalier qui suivent les iso-lignes.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-contour.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Contour 3D&quot;,
    variant=&quot;contour&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>temporal</code></h3><p>Grilles temporelles interpolées en un terrain viridis lisse, une colonne par cellule interpolée.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-temporal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Temporal 3D&quot;,
    variant=&quot;temporal&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>cluster</code></h3><p>Lignes et colonnes réordonnées par classification hiérarchique pour rapprocher les lignes semblables.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-cluster.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Cluster 3D&quot;,
    variant=&quot;cluster&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>bubble</code></h3><p>L&#x27;empreinte de chaque colonne dépend de sa valeur : les cellules lourdes grossissent, les légères s&#x27;amincissent.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-bubble.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Bubble 3D&quot;,
    variant=&quot;bubble&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>marginal</code></h3><p>Totaux de lignes et de colonnes en colonnes supplémentaires au-delà des bords, comme des histogrammes marginaux.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-marginal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Marginal 3D&quot;,
    variant=&quot;marginal&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>confusion</code></h3><p>Les cellules diagonales (bonnes prédictions) gardent leur teinte pleine, les autres sont atténuées.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-confusion.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Confusion 3D&quot;,
    variant=&quot;confusion&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>pivot</code></h3><p>Totaux de lignes et de colonnes ajoutés juste à côté de la grille, façon tableau croisé.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-pivot.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Pivot 3D&quot;,
    variant=&quot;pivot&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    col_labels=[&quot;8h&quot;, &quot;12h&quot;, &quot;16h&quot;, &quot;20h&quot;],
    values=[5, 9, 7, 3, 6, 12, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>polar</code></h3><p>Les lignes deviennent des anneaux concentriques et les colonnes des secteurs angulaires autour du centre.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-polar.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Polar 3D&quot;,
    variant=&quot;polar&quot;,
    labels=[&quot;North&quot;, &quot;South&quot;, &quot;East&quot;, &quot;West&quot;],
    col_labels=[&quot;0h&quot;, &quot;3h&quot;, &quot;6h&quot;, &quot;9h&quot;, &quot;12h&quot;, &quot;15h&quot;, &quot;18h&quot;, &quot;21h&quot;],
    values=[210, 190, 180, 220, 260, 240, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>radial_cluster</code></h3><p>La disposition polaire avec lignes et colonnes réordonnées par classification hiérarchique.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-radial_cluster.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Radial Cluster 3D&quot;,
    variant=&quot;radial_cluster&quot;,
    labels=[&quot;GSM1&quot;, &quot;GSM2&quot;, &quot;GSM3&quot;, &quot;GSM4&quot;, &quot;GSM5&quot;, &quot;GSM6&quot;],
    col_labels=[&quot;10001_at&quot;, &quot;10005_at&quot;, &quot;10013_at&quot;, &quot;10020_at&quot;, &quot;10025_at&quot;, &quot;10004_at&quot;, ...],
    values=[9, 9, 10, 8, 10, 7, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>hex_grid</code></h3><p>Grille décalée où les lignes impaires glissent d&#x27;une demi-cellule, approchant un pavage hexagonal.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-hex_grid.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Hex Grid 3D&quot;,
    variant=&quot;hex_grid&quot;,
    labels=[&quot;Figma Design Systems&quot;, &quot;Product Management&quot;, &quot;Design Research&quot;, &quot;Dev Web&quot;],
    col_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;, &quot;Jun&quot;, ...],
    matrix=[[61, 25, 42, 25, 61, 42, ...], [25, 61, 86, 42, 61, 25, ...], [25, 42, 25, 86, 42, 25, ...], [62, 54, 54, 21, 12, 25, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>horizon</code></h3><p>Chaque ligne est une fine crête, les lignes étant espacées comme des bandes de séries temporelles empilées.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-horizon.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Horizon 3D&quot;,
    variant=&quot;horizon&quot;,
    labels=[&quot;Adobe&quot;, &quot;Alphabet&quot;, &quot;Amazon&quot;, &quot;AMD&quot;, &quot;Apple&quot;, &quot;ASML Holding&quot;, ...],
    col_labels=[&quot;Jan 20&quot;, &quot;&quot;, &quot;&quot;, &quot;&quot;, &quot;Feb 20&quot;, &quot;&quot;, ...],
    matrix=[[-0.84, -1.19, -1.6, -1.09, -0.71, 0.02, ...], [0.46, 0.48, -0.37, -0.64, -1.39, -0.6, ...], [0.71, 0.18, 0.52, 0.07, -0.11, 0.18, ...], [-0.19, -0.05, -0.44, -0.08, -0.88, -0.14, ...], [0.92, 0.7, 1.16, 1.18, 1.32, 0.68, ...], [-0.02, 0.5, -0.25, -0.73, -0.34, -0.67, ...], ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>moods</code></h3><p>Lignes regroupées selon leur premier jeton <code>::</code>, avec un écart entre les groupes.</p><p class="sp-3d-uses">Utilise: <code>labels, col_labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-moods.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.heatmap3d(
    &quot;Moods 3D&quot;,
    variant=&quot;moods&quot;,
    labels=[&quot;Rock::Ironclad Sky::Silver Gravity&quot;, &quot;Rock::Ironclad Sky::Hollow Bloom&quot;, &quot;Rock::Ironclad Sky::Distant Ember&quot;, &quot;Rock::Voltage Parade::Slow Gravity&quot;, &quot;Rock::Voltage Parade::Wild Gravity&quot;, &quot;Rock::Rust Horizon::Distant Bloom&quot;, ...],
    col_labels=[&quot;Angry&quot;, &quot;Passionate&quot;, &quot;Excited&quot;, &quot;Happy&quot;, &quot;Curious&quot;, &quot;Content&quot;, ...],
    values=[0.891, 0.941, 0.615, 0.129, 0.218, 0.119, ...],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="heatmap3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
