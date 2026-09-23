# Scatter Chart 3D

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

`sp.scatter3d(title, x=None, y=None, z=None, labels=None, categories=None, color_values=None, series=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_scatter3d_chart()`, `sp.scatter_3d()`, `sp.scatter3d_chart()`, `sp.scatter3d_family()`, `sp.scatters3d()`.

## Description

`sp.scatter3d()` carries two independent forms under one name, chosen by whether `z` is given. Pass `x`, `y` and `z` together and it keeps its original job: a real spatial point cloud in 3D space, optionally coloured by `color_values`, capped to a budget for huge clouds. Leave `z` out and **every one of the 12 2D scatter variants gets a 3D form instead**, selected with the same `variant` keyword and fed with the same `x`/`y` rows — the family this page otherwise documents.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `simple`, `default` | `x, y` | A small marker per point. |
| `categorical` | `grouped`, `groups`, `category` | `x, y, categories` | The same markers, one class (and colour) per category. |
| `symbols` | `marker`, `markers`, `shape`, `shapes` | `x, y` | The same markers as basic, from the 2D chart's per-category marker shapes. |
| `labeled` | `labels`, `text`, `annotated` | `x, y` | The same markers as basic; names show on hover in every variant. |
| `regression` | `trendline`, `fit`, `ols` | `x, y` | The markers plus a linear best-fit line laid across them. |
| `residual` | `residuals`, `residplot` | `x, y` | Every point repositioned to its vertical distance from the best-fit line, flattening the trend. |
| `dual_style` | `hue_style`, `two_way` | `x, y` | The same markers as basic, from the 2D chart's two-hue category styling. |
| `continuous_hue` | `numeric_hue`, `colormap` | `x, y, color_values` | The same markers toned on a continuous ramp by `color_values`. |
| `facet` | `facets`, `small_multiples`, `relplot` | `x, y, categories` | Every category on its own depth row, a small multiple per group. |
| `sized` | `size_scale`, `bubble_scatter`, `magnitude_size` | `x, y, color_values` | Marker size and tone both scale with `color_values`. |
| `wide_form` | `wide`, `multi_series`, `columns` | `x, series` | Every column of `series` its own depth row, all sharing the same `x`. |
| `rug` | `rugplot`, `marginal_ticks`, `carpet_ticks` | `x, y` | The markers plus a tick at every point's position along both floor margins. |

## Data

Without `z`: `x`/`y` position each point, `categories` groups or facets it, `color_values` drives continuous tone or size, and `series` (as parallel y-columns) feeds `wide_form`.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="scatter3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>A small marker per point.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(&quot;Basic 3D&quot;, variant=&quot;basic&quot;, x=[1, 2, 3, 4, 5, 6, ...], y=[2, 5, 3, 8, 7, 9, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>categorical</code></h3><p>The same markers, one class (and colour) per category.</p><p class="sp-3d-uses">Uses: <code>x, y, categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-categorical.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Categorical 3D&quot;,
    variant=&quot;categorical&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    y=[2, 5, 3, 8, 7, 9, ...],
    categories=[&quot;A&quot;, &quot;B&quot;, &quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;A&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>symbols</code></h3><p>The same markers as basic, from the 2D chart&#x27;s per-category marker shapes.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-symbols.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(&quot;Symbols 3D&quot;, variant=&quot;symbols&quot;, x=[1, 2, 3, 4, 5, 6, ...], y=[2, 5, 3, 8, 7, 9, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>labeled</code></h3><p>The same markers as basic; names show on hover in every variant.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-labeled.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Labeled 3D&quot;,
    variant=&quot;labeled&quot;,
    x=[1, 2, 3, 4, 5, 6],
    y=[2, 5, 3, 8, 7, 9],
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>regression</code></h3><p>The markers plus a linear best-fit line laid across them.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-regression.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Regression 3D&quot;,
    variant=&quot;regression&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    y=[2, 3.8, 5.1, 7.2, 8.5, 10.3, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>residual</code></h3><p>Every point repositioned to its vertical distance from the best-fit line, flattening the trend.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-residual.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Residual 3D&quot;,
    variant=&quot;residual&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    y=[2, 3.8, 5.1, 7.2, 8.5, 10.3, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>dual_style</code></h3><p>The same markers as basic, from the 2D chart&#x27;s two-hue category styling.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-dual_style.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Dual Style 3D&quot;,
    variant=&quot;dual_style&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    y=[2, 5, 3, 8, 7, 9, ...],
    categories=[&quot;Lunch&quot;, &quot;Lunch&quot;, &quot;Dinner&quot;, &quot;Dinner&quot;, &quot;Lunch&quot;, &quot;Dinner&quot;, ...],
    categories2=[&quot;Thur&quot;, &quot;Fri&quot;, &quot;Thur&quot;, &quot;Sat&quot;, &quot;Sun&quot;, &quot;Thur&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>continuous_hue</code></h3><p>The same markers toned on a continuous ramp by <code>color_values</code>.</p><p class="sp-3d-uses">Uses: <code>x, y, color_values</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-continuous_hue.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Continuous Hue 3D&quot;,
    variant=&quot;continuous_hue&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    y=[2, 5, 3, 8, 7, 9, ...],
    color_values=[12, 18, 9, 27, 22, 31, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>facet</code></h3><p>Every category on its own depth row, a small multiple per group.</p><p class="sp-3d-uses">Uses: <code>x, y, categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-facet.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Facet 3D&quot;,
    variant=&quot;facet&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    y=[2, 5, 3, 8, 7, 9, ...],
    categories=[&quot;Lunch&quot;, &quot;Lunch&quot;, &quot;Lunch&quot;, &quot;Lunch&quot;, &quot;Lunch&quot;, &quot;Lunch&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>sized</code></h3><p>Marker size and tone both scale with <code>color_values</code>.</p><p class="sp-3d-uses">Uses: <code>x, y, color_values</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-sized.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Sized 3D&quot;,
    variant=&quot;sized&quot;,
    x=[16.99, 10.34, 21.01, 23.68, 24.59, 25.29, ...],
    y=[1.01, 1.66, 3.5, 3.31, 3.61, 4.71, ...],
    color_values=[2, 3, 3, 2, 4, 4, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>wide_form</code></h3><p>Every column of <code>series</code> its own depth row, all sharing the same <code>x</code>.</p><p class="sp-3d-uses">Uses: <code>x, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-wide_form.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Wide Form 3D&quot;,
    variant=&quot;wide_form&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    series=[[0.5, 1.4, 0.3, 1.8, 2.6, 2.1, ...], [-0.3, -1.1, -0.8, -1.9, -1.2, -2.4, ...], [0.2, 0.9, 1.6, 1.1, 2.0, 2.7, ...], [1.0, 0.6, 1.3, 0.4, 1.1, 0.2, ...]],
    series_names=[&quot;a&quot;, &quot;b&quot;, &quot;c&quot;, &quot;d&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>rug</code></h3><p>The markers plus a tick at every point&#x27;s position along both floor margins.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-rug.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(&quot;Rug 3D&quot;, variant=&quot;rug&quot;, x=[1, 2, 3, 4, 5, 6, ...], y=[2, 5, 3, 8, 7, 9, ...])</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="scatter3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.scatter3d(title, x=None, y=None, z=None, labels=None, categories=None, color_values=None, series=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_scatter3d_chart()`, `sp.scatter_3d()`, `sp.scatter3d_chart()`, `sp.scatter3d_family()`, `sp.scatters3d()`.

<h2>Description</h2>

`sp.scatter3d()` porte deux formes indépendantes sous un seul nom, choisies selon que `z` est fourni. Passez `x`, `y` et `z` ensemble et il garde son rôle d'origine : un vrai nuage de points spatial en 3D, coloré en option par `color_values`, plafonné pour les nuages énormes. Omettez `z` et **chacune des 12 variantes de scatter 2D obtient à la place une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes lignes `x`/`y` — la famille que documente le reste de cette page.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `simple`, `default` | `x, y` | Un petit repère par point. |
| `categorical` | `grouped`, `groups`, `category` | `x, y, categories` | Les mêmes repères, une classe (et une couleur) par catégorie. |
| `symbols` | `marker`, `markers`, `shape`, `shapes` | `x, y` | Les mêmes repères que basic, depuis les formes de marqueur par catégorie du graphique 2D. |
| `labeled` | `labels`, `text`, `annotated` | `x, y` | Les mêmes repères que basic ; les noms s'affichent au survol dans toutes les variantes. |
| `regression` | `trendline`, `fit`, `ols` | `x, y` | Les repères plus une droite de régression tracée à travers eux. |
| `residual` | `residuals`, `residplot` | `x, y` | Chaque point repositionné selon sa distance verticale à la droite de régression, aplatissant la tendance. |
| `dual_style` | `hue_style`, `two_way` | `x, y` | Les mêmes repères que basic, depuis le style à deux teintes par catégorie du graphique 2D. |
| `continuous_hue` | `numeric_hue`, `colormap` | `x, y, color_values` | Les mêmes repères teintés sur une rampe continue selon `color_values`. |
| `facet` | `facets`, `small_multiples`, `relplot` | `x, y, categories` | Chaque catégorie sur sa propre rangée de profondeur, un petit multiple par groupe. |
| `sized` | `size_scale`, `bubble_scatter`, `magnitude_size` | `x, y, color_values` | Taille et teinte du repère suivent toutes deux `color_values`. |
| `wide_form` | `wide`, `multi_series`, `columns` | `x, series` | Chaque colonne de `series` sur sa propre rangée de profondeur, toutes partageant le même `x`. |
| `rug` | `rugplot`, `marginal_ticks`, `carpet_ticks` | `x, y` | Les repères plus une marque à la position de chaque point le long des deux marges du sol. |

<h2>Données</h2>

Sans `z` : `x`/`y` positionne chaque point, `categories` le groupe ou le facette, `color_values` pilote la teinte continue ou la taille, et `series` (en colonnes y parallèles) alimente `wide_form`.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="scatter3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Un petit repère par point.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(&quot;Basic 3D&quot;, variant=&quot;basic&quot;, x=[1, 2, 3, 4, 5, 6, ...], y=[2, 5, 3, 8, 7, 9, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>categorical</code></h3><p>Les mêmes repères, une classe (et une couleur) par catégorie.</p><p class="sp-3d-uses">Utilise: <code>x, y, categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-categorical.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Categorical 3D&quot;,
    variant=&quot;categorical&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    y=[2, 5, 3, 8, 7, 9, ...],
    categories=[&quot;A&quot;, &quot;B&quot;, &quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;A&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>symbols</code></h3><p>Les mêmes repères que basic, depuis les formes de marqueur par catégorie du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-symbols.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(&quot;Symbols 3D&quot;, variant=&quot;symbols&quot;, x=[1, 2, 3, 4, 5, 6, ...], y=[2, 5, 3, 8, 7, 9, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>labeled</code></h3><p>Les mêmes repères que basic ; les noms s&#x27;affichent au survol dans toutes les variantes.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-labeled.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Labeled 3D&quot;,
    variant=&quot;labeled&quot;,
    x=[1, 2, 3, 4, 5, 6],
    y=[2, 5, 3, 8, 7, 9],
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>regression</code></h3><p>Les repères plus une droite de régression tracée à travers eux.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-regression.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Regression 3D&quot;,
    variant=&quot;regression&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    y=[2, 3.8, 5.1, 7.2, 8.5, 10.3, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>residual</code></h3><p>Chaque point repositionné selon sa distance verticale à la droite de régression, aplatissant la tendance.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-residual.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Residual 3D&quot;,
    variant=&quot;residual&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    y=[2, 3.8, 5.1, 7.2, 8.5, 10.3, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>dual_style</code></h3><p>Les mêmes repères que basic, depuis le style à deux teintes par catégorie du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-dual_style.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Dual Style 3D&quot;,
    variant=&quot;dual_style&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    y=[2, 5, 3, 8, 7, 9, ...],
    categories=[&quot;Lunch&quot;, &quot;Lunch&quot;, &quot;Dinner&quot;, &quot;Dinner&quot;, &quot;Lunch&quot;, &quot;Dinner&quot;, ...],
    categories2=[&quot;Thur&quot;, &quot;Fri&quot;, &quot;Thur&quot;, &quot;Sat&quot;, &quot;Sun&quot;, &quot;Thur&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>continuous_hue</code></h3><p>Les mêmes repères teintés sur une rampe continue selon <code>color_values</code>.</p><p class="sp-3d-uses">Utilise: <code>x, y, color_values</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-continuous_hue.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Continuous Hue 3D&quot;,
    variant=&quot;continuous_hue&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    y=[2, 5, 3, 8, 7, 9, ...],
    color_values=[12, 18, 9, 27, 22, 31, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>facet</code></h3><p>Chaque catégorie sur sa propre rangée de profondeur, un petit multiple par groupe.</p><p class="sp-3d-uses">Utilise: <code>x, y, categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-facet.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Facet 3D&quot;,
    variant=&quot;facet&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    y=[2, 5, 3, 8, 7, 9, ...],
    categories=[&quot;Lunch&quot;, &quot;Lunch&quot;, &quot;Lunch&quot;, &quot;Lunch&quot;, &quot;Lunch&quot;, &quot;Lunch&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>sized</code></h3><p>Taille et teinte du repère suivent toutes deux <code>color_values</code>.</p><p class="sp-3d-uses">Utilise: <code>x, y, color_values</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-sized.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Sized 3D&quot;,
    variant=&quot;sized&quot;,
    x=[16.99, 10.34, 21.01, 23.68, 24.59, 25.29, ...],
    y=[1.01, 1.66, 3.5, 3.31, 3.61, 4.71, ...],
    color_values=[2, 3, 3, 2, 4, 4, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>wide_form</code></h3><p>Chaque colonne de <code>series</code> sur sa propre rangée de profondeur, toutes partageant le même <code>x</code>.</p><p class="sp-3d-uses">Utilise: <code>x, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-wide_form.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(
    &quot;Wide Form 3D&quot;,
    variant=&quot;wide_form&quot;,
    x=[1, 2, 3, 4, 5, 6, ...],
    series=[[0.5, 1.4, 0.3, 1.8, 2.6, 2.1, ...], [-0.3, -1.1, -0.8, -1.9, -1.2, -2.4, ...], [0.2, 0.9, 1.6, 1.1, 2.0, 2.7, ...], [1.0, 0.6, 1.3, 0.4, 1.1, 0.2, ...]],
    series_names=[&quot;a&quot;, &quot;b&quot;, &quot;c&quot;, &quot;d&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>rug</code></h3><p>Les repères plus une marque à la position de chaque point le long des deux marges du sol.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatter3d-rug.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatter3d(&quot;Rug 3D&quot;, variant=&quot;rug&quot;, x=[1, 2, 3, 4, 5, 6, ...], y=[2, 5, 3, 8, 7, 9, ...])</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="scatter3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
