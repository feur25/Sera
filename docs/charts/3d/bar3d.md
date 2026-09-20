# Bar Chart 3D

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

`sp.bar3d(title, labels=None, values=None, *, variant="basic", series=None, series_names=None, orientation3d="iso", color_hex=0x6366F1, palette=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_bar3d_chart()`, `sp.bar_3d()`, `sp.bar3d_chart()`, `sp.bar3d_family()`, `sp.bars3d()`.

## Description

`sp.bar3d()` renders the SeraPlot bar family as extruded prisms on an interactive 3D canvas. It is the 3D twin of [`sp.bar()`](../2d/bar.md): **every one of the 17 bar variants has a 3D form**, selected with the same `variant=` keyword and fed with the same data fields. Layouts come from shared, family-agnostic strategies (linear, grouped, stacked, radial, spiral, box-and-whisker), so geometry is never hand-written per variant.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | — | `labels, values` | One extruded column per category along the X axis; height encodes the value. |
| `horizontal` | `h`, `hbar` | `labels, values` | Bars lie flat along the X axis, one row per category; length encodes the value. |
| `grouped` | `group` | `labels, series, series_names` | Series side by side in depth: one row of columns per series, categories along X. |
| `stacked` | `stack` | `labels, series, series_names` | Series stacked into a single column per category; each segment keeps its series colour. |
| `relative` | `rel` | `labels, series, series_names` | Stacked columns normalised to 100 %: every column reaches the same height, segments show the share. |
| `grouped_stacked` | `groupstack`, `grouped-stacked` | `labels, series, offset_groups` | Stack groups placed side by side in depth, each stack built from its own series segments. |
| `marimekko` | `mekko`, `mosaic` | `labels, series, widths` | Columns whose footprint width follows `widths`, stacked segments encode each series share. |
| `pictogram` | `icon` | `labels, values, units_per_icon` | Every icon becomes a small block; blocks pile up in columns of `max_icons_per_column`. |
| `multicategory` | `multi`, `hierarchical` | `labels, values, super_categories` | Columns grouped under their `super_categories`, with an extra gap between two groups. |
| `circular` | `circular_basic`, `radial_bar`, `polar_bar` | `labels, values` | Columns stand on a ring, one per category; height encodes the value. |
| `circular_grouped` | `radial_grouped`, `circular_groups` | `labels, values, color_groups` | The same ring split into angular arcs by `color_groups`, one colour per group. |
| `population_pyramid` | `pyramid`, `age_pyramid` | `labels, series, series_names` | Two back-to-back rows of columns, one per series (male / female), age groups along X. |
| `diverging` | `signed`, `delta`, `bidirectional` | `labels, values` | Columns rise above or sink below the floor with the sign of the value; with `series`, positive and negative segments stack on each side. |
| `distribution` | `bar_box`, `boxbar`, `bar_boxplot` | `labels, series` | For each sample a slim column spans the whiskers and a wider box spans the interquartile range. |
| `spiral` | `spiral_bar`, `nautilus`, `radial_spiral`, `growth_spiral` | `labels, values` | Columns follow an outward spiral, one per index — made for long time series. |
| `hedgehog` | `flow_fan`, `quill`, `spike_flow`, `relocation_fan` | `labels, values` | Signed spikes around a ring: positive values rise, negative values sink. |
| `radial_flow` | `capital_flow`, `hierarchical_radial`, `sankey_radial`, `flow_arc` | `labels, values, super_categories` | A ring split into arcs by `super_categories`, columns grouped under their parent. |

## 3D planes

The viewpoint is independent from the variant: `orientation3d` (aliases `tilt3d`, `rotate3d`) picks the initial camera plane and applies to **every** variant. The plane can also be switched afterwards with `.orient3d(mode)`, and the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> — Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> — Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> — Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> — Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-front.html"></iframe></div>
</div>

```python
chart = sp.bar3d("Sales", variant="grouped", labels=labels, series=series, series_names=names, orientation3d="vertical")
chart = chart.orient3d("front")
```

## 3D scenes

The scene is a second independent axis: `scene` swaps the environment the bars are drawn in and applies to **every** variant too. Every scene works with every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-podium.html"></iframe></div>
</div>

```python
chart = sp.bar3d("Sales", variant="spiral", labels=years, values=counts, scene="terrain", orientation3d="vertical")
```

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One extruded column per category along the X axis; height encodes the value.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, 38, 17, 42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>horizontal</code></h3><p>Bars lie flat along the X axis, one row per category; length encodes the value.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-horizontal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Horizontal 3D&quot;,
    variant=&quot;horizontal&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, 38, 17, 42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>grouped</code></h3><p>Series side by side in depth: one row of columns per series, categories along X.</p><p class="sp-3d-uses">Uses: <code>labels, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-grouped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Grouped 3D&quot;,
    variant=&quot;grouped&quot;,
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[24,38,17,42], [18,29,33,21], [12,15,28,30]],
    series_names=[&quot;Product A&quot;, &quot;Product B&quot;, &quot;Product C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stacked</code></h3><p>Series stacked into a single column per category; each segment keeps its series colour.</p><p class="sp-3d-uses">Uses: <code>labels, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-stacked.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Stacked 3D&quot;,
    variant=&quot;stacked&quot;,
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[5,30,60,10], [8,45,70,15], [4,25,50,8]],
    series_names=[&quot;Product A&quot;, &quot;Product B&quot;, &quot;Product C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>relative</code></h3><p>Stacked columns normalised to 100 %: every column reaches the same height, segments show the share.</p><p class="sp-3d-uses">Uses: <code>labels, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-relative.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Relative 3D&quot;,
    variant=&quot;relative&quot;,
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[24,38,17,42], [18,29,33,21], [12,15,28,30]],
    series_names=[&quot;Revenue&quot;, &quot;Cost&quot;, &quot;Tax&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>grouped_stacked</code></h3><p>Stack groups placed side by side in depth, each stack built from its own series segments.</p><p class="sp-3d-uses">Uses: <code>labels, series, offset_groups</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-grouped_stacked.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Grouped-Stacked 3D&quot;,
    variant=&quot;grouped_stacked&quot;,
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[24,38,17,42], [18,29,33,21], [12,15,28,30], [20,25,18,22]],
    series_names=[&quot;2023 A&quot;, &quot;2023 B&quot;, &quot;2024 A&quot;, &quot;2024 B&quot;],
    offset_groups=[&quot;2023&quot;, &quot;2023&quot;, &quot;2024&quot;, &quot;2024&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>marimekko</code></h3><p>Columns whose footprint width follows <code>widths</code>, stacked segments encode each series share.</p><p class="sp-3d-uses">Uses: <code>labels, series, widths</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-marimekko.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Marimekko 3D&quot;,
    variant=&quot;marimekko&quot;,
    labels=[&quot;North&quot;, &quot;South&quot;, &quot;East&quot;, &quot;West&quot;],
    series=[[42,28,33,17], [18,38,22,30], [24,15,28,20]],
    series_names=[&quot;Product A&quot;, &quot;Product B&quot;, &quot;Product C&quot;],
    widths=[2.0, 1.5, 1.0, 1.2],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>pictogram</code></h3><p>Every icon becomes a small block; blocks pile up in columns of <code>max_icons_per_column</code>.</p><p class="sp-3d-uses">Uses: <code>labels, values, units_per_icon</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-pictogram.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Pictogram 3D&quot;,
    variant=&quot;pictogram&quot;,
    labels=[&quot;Bikes&quot;, &quot;Cars&quot;, &quot;Buses&quot;, &quot;Trains&quot;],
    values=[24, 38, 17, 42],
    unit_description=&quot;units&quot;,
    units_per_icon=2.0,
    icon_size=24,
    max_icons_per_column=10,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>multicategory</code></h3><p>Columns grouped under their <code>super_categories</code>, with an extra gap between two groups.</p><p class="sp-3d-uses">Uses: <code>labels, values, super_categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-multicategory.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Multicategory 3D&quot;,
    variant=&quot;multicategory&quot;,
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[24,38,17,42], [18,29,33,21]],
    series_names=[&quot;2023&quot;, &quot;2024&quot;],
    super_categories=[&quot;H1&quot;, &quot;H1&quot;, &quot;H2&quot;, &quot;H2&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>circular</code></h3><p>Columns stand on a ring, one per category; height encodes the value.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-circular.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Circular 3D&quot;,
    variant=&quot;circular&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;, &quot;H&quot;],
    values=[24, 38, 17, 42, 29, 33, 20, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>circular_grouped</code></h3><p>The same ring split into angular arcs by <code>color_groups</code>, one colour per group.</p><p class="sp-3d-uses">Uses: <code>labels, values, color_groups</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-circular_grouped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Circular Grouped 3D&quot;,
    variant=&quot;circular_grouped&quot;,
    labels=[&quot;A1&quot;, &quot;A2&quot;, &quot;A3&quot;, &quot;B1&quot;, &quot;B2&quot;, &quot;B3&quot;, ...],
    values=[24, 38, 17, 42, 29, 33, ...],
    color_groups=[&quot;Group A&quot;, &quot;Group A&quot;, &quot;Group A&quot;, &quot;Group B&quot;, &quot;Group B&quot;, &quot;Group B&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>population_pyramid</code></h3><p>Two back-to-back rows of columns, one per series (male / female), age groups along X.</p><p class="sp-3d-uses">Uses: <code>labels, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-population_pyramid.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Population Pyramid 3D&quot;,
    variant=&quot;population_pyramid&quot;,
    labels=[&quot;0-9&quot;, &quot;10-19&quot;, &quot;20-29&quot;, &quot;30-39&quot;, &quot;40-49&quot;, &quot;50-59&quot;, &quot;60-69&quot;, &quot;70+&quot;],
    series=[[12,18,24,22,17,13,9,5], [11,17,25,23,18,14,10,6]],
    series_names=[&quot;Male&quot;, &quot;Female&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>diverging</code></h3><p>Columns rise above or sink below the floor with the sign of the value; with <code>series</code>, positive and negative segments stack on each side.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-diverging.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Diverging 3D&quot;,
    variant=&quot;diverging&quot;,
    title=&quot;Extremes regionaux&quot;,
    y_label=&quot;Robustesse Score median&quot;,
    labels=[&quot;Consecrated Snowfield&quot;, &quot;Gravesite Plain&quot;, &quot;Scadu Altus&quot;, &quot;Jagged Peak&quot;, &quot;Liurnia of the Lakes&quot;, &quot;Limgrave&quot;, &quot;Charo&#x27;s Hidden Grave&quot;, &quot;Weeping Peninsula&quot;],
    values=[0.55, 0.54, 0.53, 0.41, -0.35, -0.38, -0.41, -0.47],
    sort_order=&quot;desc&quot;,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>distribution</code></h3><p>For each sample a slim column spans the whiskers and a wider box spans the interquartile range.</p><p class="sp-3d-uses">Uses: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-distribution.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Distribution 3D&quot;,
    variant=&quot;distribution&quot;,
    labels=[&quot;Control&quot;, &quot;Treatment A&quot;, &quot;Treatment B&quot;],
    series=[[23,25,19,30,22,27,24], [15,18,20,22,17,19,16], [30,32,28,35,31,29,33]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>spiral</code></h3><p>Columns follow an outward spiral, one per index — made for long time series.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-spiral.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Spiral 3D&quot;,
    variant=&quot;spiral&quot;,
    labels=[&quot;1950&quot;, &quot;1951&quot;, &quot;1952&quot;, &quot;1953&quot;, &quot;1954&quot;, &quot;1955&quot;, ...],
    values=[6, 7, 9, 8, 10, 11, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>hedgehog</code></h3><p>Signed spikes around a ring: positive values rise, negative values sink.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-hedgehog.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Hedgehog 3D&quot;,
    variant=&quot;hedgehog&quot;,
    labels=[&quot;Chico to Seattle&quot;, &quot;Fresno to Portland&quot;, &quot;Reno to Denver&quot;, &quot;Modesto to Austin&quot;, &quot;Bakersfield to Sacramento&quot;, &quot;Chico to Sacramento&quot;, ...],
    values=[9.0, 6.0, 4.0, 3.0, 2.0, 7.0, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>radial_flow</code></h3><p>A ring split into arcs by <code>super_categories</code>, columns grouped under their parent.</p><p class="sp-3d-uses">Uses: <code>labels, values, super_categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-radial_flow.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Radial Flow 3D&quot;,
    variant=&quot;radial_flow&quot;,
    labels=[&quot;KKR&quot;, &quot;Blackstone&quot;, &quot;Clayton Dubilier &amp; Rice&quot;, &quot;Warburg Pincus&quot;, &quot;General Atlantic&quot;, &quot;GTCR&quot;, ...],
    values=[117.9, 95.7, 49.8, 34.2, 44.7, 30.2, ...],
    super_categories=[&quot;New York&quot;, &quot;New York&quot;, &quot;New York&quot;, &quot;New York&quot;, &quot;New York&quot;, &quot;Chicago&quot;, ...],
    offset_groups=[&quot;U.S.&quot;, &quot;U.S.&quot;, &quot;U.S.&quot;, &quot;U.S.&quot;, &quot;U.S.&quot;, &quot;U.S.&quot;, ...],
    palette=[3049182, 1482885, 13934615, 15277708, 1780298, 9317439, 8207041],
    title=&quot;Top Private Equity Firms by Capital Raised&quot;,
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="bar_3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.bar3d(title, labels=None, values=None, *, variant="basic", series=None, series_names=None, orientation3d="iso", color_hex=0x6366F1, palette=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_bar3d_chart()`, `sp.bar_3d()`, `sp.bar3d_chart()`, `sp.bar3d_family()`, `sp.bars3d()`.

<h2>Description</h2>

`sp.bar3d()` rend la famille de barres de SeraPlot en prismes extrudés sur un canvas 3D interactif. C'est le jumeau 3D de [`sp.bar()`](../2d/bar.md) : **chacune des 17 variantes de bar possède une forme 3D**, choisie avec le même mot-clé `variant=` et alimentée par les mêmes champs de données. Les géométries viennent de stratégies partagées et indépendantes de la famille (linéaire, groupé, empilé, radial, spirale, boîte à moustaches) : rien n'est écrit à la main par variante.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | — | `labels, values` | Une colonne extrudée par catégorie le long de l'axe X ; la hauteur encode la valeur. |
| `horizontal` | `h`, `hbar` | `labels, values` | Les barres sont couchées le long de l'axe X, une rangée par catégorie ; la longueur encode la valeur. |
| `grouped` | `group` | `labels, series, series_names` | Séries côte à côte en profondeur : une rangée de colonnes par série, catégories le long de X. |
| `stacked` | `stack` | `labels, series, series_names` | Séries empilées en une seule colonne par catégorie ; chaque segment garde la couleur de sa série. |
| `relative` | `rel` | `labels, series, series_names` | Colonnes normalisées à 100 % : toutes atteignent la même hauteur, les segments montrent la part. |
| `grouped_stacked` | `groupstack`, `grouped-stacked` | `labels, series, offset_groups` | Groupes d'empilements placés côte à côte en profondeur, chaque pile construite depuis ses propres séries. |
| `marimekko` | `mekko`, `mosaic` | `labels, series, widths` | Colonnes dont l'emprise suit `widths`, segments empilés encodant la part de chaque série. |
| `pictogram` | `icon` | `labels, values, units_per_icon` | Chaque icône devient un petit bloc ; les blocs s'empilent par colonnes de `max_icons_per_column`. |
| `multicategory` | `multi`, `hierarchical` | `labels, values, super_categories` | Colonnes regroupées sous leurs `super_categories`, avec un écart supplémentaire entre deux groupes. |
| `circular` | `circular_basic`, `radial_bar`, `polar_bar` | `labels, values` | Les colonnes se dressent sur un anneau, une par catégorie ; la hauteur encode la valeur. |
| `circular_grouped` | `radial_grouped`, `circular_groups` | `labels, values, color_groups` | Le même anneau découpé en arcs par `color_groups`, une couleur par groupe. |
| `population_pyramid` | `pyramid`, `age_pyramid` | `labels, series, series_names` | Deux rangées de colonnes dos à dos, une par série (hommes / femmes), tranches d'âge le long de X. |
| `diverging` | `signed`, `delta`, `bidirectional` | `labels, values` | Les colonnes montent ou s'enfoncent selon le signe de la valeur ; avec `series`, les segments positifs et négatifs s'empilent de chaque côté. |
| `distribution` | `bar_box`, `boxbar`, `bar_boxplot` | `labels, series` | Pour chaque échantillon, une colonne fine couvre les moustaches et une boîte plus large couvre l'intervalle interquartile. |
| `spiral` | `spiral_bar`, `nautilus`, `radial_spiral`, `growth_spiral` | `labels, values` | Les colonnes suivent une spirale vers l'extérieur, une par indice — pensé pour les longues séries temporelles. |
| `hedgehog` | `flow_fan`, `quill`, `spike_flow`, `relocation_fan` | `labels, values` | Pics signés autour d'un anneau : les valeurs positives montent, les négatives s'enfoncent. |
| `radial_flow` | `capital_flow`, `hierarchical_radial`, `sankey_radial`, `flow_arc` | `labels, values, super_categories` | Un anneau découpé en arcs par `super_categories`, colonnes regroupées sous leur parent. |

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` (alias `tilt3d`, `rotate3d`) choisit le plan initial de la caméra et s'applique à **toutes** les variantes. Le plan peut aussi être changé après coup avec `.orient3d(mode)`, et la vue reste toujours orientable à la souris.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> — Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> — Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> — Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> — Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-front.html"></iframe></div>
</div>

```python
chart = sp.bar3d("Ventes", variant="grouped", labels=labels, series=series, series_names=names, orientation3d="vertical")
chart = chart.orient3d("front")
```

<h2>Scènes 3D</h2>

La scène est un second axe indépendant : `scene` change l'environnement dans lequel les barres sont dessinées et s'applique aussi à **toutes** les variantes. Chaque scène fonctionne avec chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-podium.html"></iframe></div>
</div>

```python
chart = sp.bar3d("Ventes", variant="spiral", labels=years, values=counts, scene="terrain", orientation3d="vertical")
```

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une colonne extrudée par catégorie le long de l&#x27;axe X ; la hauteur encode la valeur.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, 38, 17, 42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>horizontal</code></h3><p>Les barres sont couchées le long de l&#x27;axe X, une rangée par catégorie ; la longueur encode la valeur.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-horizontal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Horizontal 3D&quot;,
    variant=&quot;horizontal&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, 38, 17, 42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>grouped</code></h3><p>Séries côte à côte en profondeur : une rangée de colonnes par série, catégories le long de X.</p><p class="sp-3d-uses">Utilise: <code>labels, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-grouped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Grouped 3D&quot;,
    variant=&quot;grouped&quot;,
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[24,38,17,42], [18,29,33,21], [12,15,28,30]],
    series_names=[&quot;Product A&quot;, &quot;Product B&quot;, &quot;Product C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stacked</code></h3><p>Séries empilées en une seule colonne par catégorie ; chaque segment garde la couleur de sa série.</p><p class="sp-3d-uses">Utilise: <code>labels, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-stacked.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Stacked 3D&quot;,
    variant=&quot;stacked&quot;,
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[5,30,60,10], [8,45,70,15], [4,25,50,8]],
    series_names=[&quot;Product A&quot;, &quot;Product B&quot;, &quot;Product C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>relative</code></h3><p>Colonnes normalisées à 100 % : toutes atteignent la même hauteur, les segments montrent la part.</p><p class="sp-3d-uses">Utilise: <code>labels, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-relative.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Relative 3D&quot;,
    variant=&quot;relative&quot;,
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[24,38,17,42], [18,29,33,21], [12,15,28,30]],
    series_names=[&quot;Revenue&quot;, &quot;Cost&quot;, &quot;Tax&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>grouped_stacked</code></h3><p>Groupes d&#x27;empilements placés côte à côte en profondeur, chaque pile construite depuis ses propres séries.</p><p class="sp-3d-uses">Utilise: <code>labels, series, offset_groups</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-grouped_stacked.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Grouped-Stacked 3D&quot;,
    variant=&quot;grouped_stacked&quot;,
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[24,38,17,42], [18,29,33,21], [12,15,28,30], [20,25,18,22]],
    series_names=[&quot;2023 A&quot;, &quot;2023 B&quot;, &quot;2024 A&quot;, &quot;2024 B&quot;],
    offset_groups=[&quot;2023&quot;, &quot;2023&quot;, &quot;2024&quot;, &quot;2024&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>marimekko</code></h3><p>Colonnes dont l&#x27;emprise suit <code>widths</code>, segments empilés encodant la part de chaque série.</p><p class="sp-3d-uses">Utilise: <code>labels, series, widths</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-marimekko.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Marimekko 3D&quot;,
    variant=&quot;marimekko&quot;,
    labels=[&quot;North&quot;, &quot;South&quot;, &quot;East&quot;, &quot;West&quot;],
    series=[[42,28,33,17], [18,38,22,30], [24,15,28,20]],
    series_names=[&quot;Product A&quot;, &quot;Product B&quot;, &quot;Product C&quot;],
    widths=[2.0, 1.5, 1.0, 1.2],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>pictogram</code></h3><p>Chaque icône devient un petit bloc ; les blocs s&#x27;empilent par colonnes de <code>max_icons_per_column</code>.</p><p class="sp-3d-uses">Utilise: <code>labels, values, units_per_icon</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-pictogram.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Pictogram 3D&quot;,
    variant=&quot;pictogram&quot;,
    labels=[&quot;Bikes&quot;, &quot;Cars&quot;, &quot;Buses&quot;, &quot;Trains&quot;],
    values=[24, 38, 17, 42],
    unit_description=&quot;units&quot;,
    units_per_icon=2.0,
    icon_size=24,
    max_icons_per_column=10,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>multicategory</code></h3><p>Colonnes regroupées sous leurs <code>super_categories</code>, avec un écart supplémentaire entre deux groupes.</p><p class="sp-3d-uses">Utilise: <code>labels, values, super_categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-multicategory.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Multicategory 3D&quot;,
    variant=&quot;multicategory&quot;,
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[24,38,17,42], [18,29,33,21]],
    series_names=[&quot;2023&quot;, &quot;2024&quot;],
    super_categories=[&quot;H1&quot;, &quot;H1&quot;, &quot;H2&quot;, &quot;H2&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>circular</code></h3><p>Les colonnes se dressent sur un anneau, une par catégorie ; la hauteur encode la valeur.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-circular.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Circular 3D&quot;,
    variant=&quot;circular&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;, &quot;H&quot;],
    values=[24, 38, 17, 42, 29, 33, 20, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>circular_grouped</code></h3><p>Le même anneau découpé en arcs par <code>color_groups</code>, une couleur par groupe.</p><p class="sp-3d-uses">Utilise: <code>labels, values, color_groups</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-circular_grouped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Circular Grouped 3D&quot;,
    variant=&quot;circular_grouped&quot;,
    labels=[&quot;A1&quot;, &quot;A2&quot;, &quot;A3&quot;, &quot;B1&quot;, &quot;B2&quot;, &quot;B3&quot;, ...],
    values=[24, 38, 17, 42, 29, 33, ...],
    color_groups=[&quot;Group A&quot;, &quot;Group A&quot;, &quot;Group A&quot;, &quot;Group B&quot;, &quot;Group B&quot;, &quot;Group B&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>population_pyramid</code></h3><p>Deux rangées de colonnes dos à dos, une par série (hommes / femmes), tranches d&#x27;âge le long de X.</p><p class="sp-3d-uses">Utilise: <code>labels, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-population_pyramid.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Population Pyramid 3D&quot;,
    variant=&quot;population_pyramid&quot;,
    labels=[&quot;0-9&quot;, &quot;10-19&quot;, &quot;20-29&quot;, &quot;30-39&quot;, &quot;40-49&quot;, &quot;50-59&quot;, &quot;60-69&quot;, &quot;70+&quot;],
    series=[[12,18,24,22,17,13,9,5], [11,17,25,23,18,14,10,6]],
    series_names=[&quot;Male&quot;, &quot;Female&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>diverging</code></h3><p>Les colonnes montent ou s&#x27;enfoncent selon le signe de la valeur ; avec <code>series</code>, les segments positifs et négatifs s&#x27;empilent de chaque côté.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-diverging.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Diverging 3D&quot;,
    variant=&quot;diverging&quot;,
    title=&quot;Extremes regionaux&quot;,
    y_label=&quot;Robustesse Score median&quot;,
    labels=[&quot;Consecrated Snowfield&quot;, &quot;Gravesite Plain&quot;, &quot;Scadu Altus&quot;, &quot;Jagged Peak&quot;, &quot;Liurnia of the Lakes&quot;, &quot;Limgrave&quot;, &quot;Charo&#x27;s Hidden Grave&quot;, &quot;Weeping Peninsula&quot;],
    values=[0.55, 0.54, 0.53, 0.41, -0.35, -0.38, -0.41, -0.47],
    sort_order=&quot;desc&quot;,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>distribution</code></h3><p>Pour chaque échantillon, une colonne fine couvre les moustaches et une boîte plus large couvre l&#x27;intervalle interquartile.</p><p class="sp-3d-uses">Utilise: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-distribution.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Distribution 3D&quot;,
    variant=&quot;distribution&quot;,
    labels=[&quot;Control&quot;, &quot;Treatment A&quot;, &quot;Treatment B&quot;],
    series=[[23,25,19,30,22,27,24], [15,18,20,22,17,19,16], [30,32,28,35,31,29,33]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>spiral</code></h3><p>Les colonnes suivent une spirale vers l&#x27;extérieur, une par indice — pensé pour les longues séries temporelles.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-spiral.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Spiral 3D&quot;,
    variant=&quot;spiral&quot;,
    labels=[&quot;1950&quot;, &quot;1951&quot;, &quot;1952&quot;, &quot;1953&quot;, &quot;1954&quot;, &quot;1955&quot;, ...],
    values=[6, 7, 9, 8, 10, 11, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>hedgehog</code></h3><p>Pics signés autour d&#x27;un anneau : les valeurs positives montent, les négatives s&#x27;enfoncent.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-hedgehog.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Hedgehog 3D&quot;,
    variant=&quot;hedgehog&quot;,
    labels=[&quot;Chico to Seattle&quot;, &quot;Fresno to Portland&quot;, &quot;Reno to Denver&quot;, &quot;Modesto to Austin&quot;, &quot;Bakersfield to Sacramento&quot;, &quot;Chico to Sacramento&quot;, ...],
    values=[9.0, 6.0, 4.0, 3.0, 2.0, 7.0, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>radial_flow</code></h3><p>Un anneau découpé en arcs par <code>super_categories</code>, colonnes regroupées sous leur parent.</p><p class="sp-3d-uses">Utilise: <code>labels, values, super_categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/bar3d-radial_flow.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bar3d(
    &quot;Radial Flow 3D&quot;,
    variant=&quot;radial_flow&quot;,
    labels=[&quot;KKR&quot;, &quot;Blackstone&quot;, &quot;Clayton Dubilier &amp; Rice&quot;, &quot;Warburg Pincus&quot;, &quot;General Atlantic&quot;, &quot;GTCR&quot;, ...],
    values=[117.9, 95.7, 49.8, 34.2, 44.7, 30.2, ...],
    super_categories=[&quot;New York&quot;, &quot;New York&quot;, &quot;New York&quot;, &quot;New York&quot;, &quot;New York&quot;, &quot;Chicago&quot;, ...],
    offset_groups=[&quot;U.S.&quot;, &quot;U.S.&quot;, &quot;U.S.&quot;, &quot;U.S.&quot;, &quot;U.S.&quot;, &quot;U.S.&quot;, ...],
    palette=[3049182, 1482885, 13934615, 15277708, 1780298, 9317439, 8207041],
    title=&quot;Top Private Equity Firms by Capital Raised&quot;,
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="bar_3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
