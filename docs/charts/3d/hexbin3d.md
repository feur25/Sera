# Hexbin Chart 3D

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

`sp.hexbin3d(title, x=None, y=None, values=None, *, variant="basic", bins=20, min_count=0, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_hexbin3d_chart()`, `sp.hexbin_3d()`, `sp.hexbin3d_chart()`, `sp.hexbin3d_family()`, `sp.hexbins3d()`.

## Description

`sp.hexbin3d()` is the 3D twin of `sp.hexbin()`: **every one of the 13 hexbin variants has a 3D form**, selected with the same `variant` keyword and fed with the same points. Raw x/y points are binned into a hexagonal grid exactly like the 2D chart, and every occupied cell becomes a column, its height and tone following the bin count (or the mean of `values` for `weighted`).

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic`, `filled` | `x, y` | One column per hexagonal bin, height and tone following how many points fell in it. |
| `outlined` | `outline`, `stroke`, `labeled` | `x, y` | The same bins pressed to a thin, wireframe-like footprint. |
| `spaced` | `gapped`, `confetti` | `x, y` | The same bins with a visible gap between neighbours instead of touching. |
| `highlight` | `top`, `hotspot`, `peak` | `x, y` | The five densest bins stand at full tone; every other bin fades to a dim grey. |
| `mincnt` | `threshold`, `sparse` | `x, y, min_count` | Bins below a minimum count are dropped entirely instead of drawn empty. |
| `nested` | `magnitude`, `rings`, `centroids` | `x, y` | Bin height quantised into a handful of magnitude bands instead of a smooth scale. |
| `log_counts` | `log`, `log_scale`, `logarithmic` | `x, y` | Bin height follows the logarithm of the count, so a few crowded bins don't flatten the rest. |
| `weighted` | `mean`, `aggregate`, `reduce_mean` | `x, y, values` | Bin height and tone follow the mean of `values` inside it instead of the raw point count. |
| `dotted` | `dashed`, `styled`, `magma` | `x, y` | The same bins as basic, from the 2D chart's dashed-outline styling. |
| `marginals` | `joint`, `with_histograms`, `density_marginals` | `x, y` | The same bins plus a marginal histogram column strip along each floor edge. |
| `voronoi` | `density_voronoi`, `tessellation`, `particle_density`, `cells` | `x, y` | The same bins as basic, from the 2D chart's Voronoi-tessellation styling. |
| `neural` | `mesh`, `turbidity`, `particle_mesh`, `neural_mesh` | `x, y` | The same bins as basic, from the 2D chart's mesh-styled rendering. |
| `bloom` | `contour`, `density_bloom`, `iso_contour`, `organic_contour` | `x, y` | The same bins as basic, from the 2D chart's soft contour-bloom styling. |

## Data

`x`/`y` are the raw point coordinates, `bins` sets the grid resolution and `min_count` the cutoff for `mincnt`; `values` feeds `weighted`'s per-bin average.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="hexbin3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One column per hexagonal bin, height and tone following how many points fell in it.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(&quot;Basic 3D&quot;, variant=&quot;basic&quot;, x=[1, 2, 2, 3, 3, 3, ...], y=[1, 2, 3, 2, 3, 4, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outlined</code></h3><p>The same bins pressed to a thin, wireframe-like footprint.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-outlined.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(&quot;Outlined 3D&quot;, variant=&quot;outlined&quot;, x=[1, 2, 2, 3, 3, 3, ...], y=[1, 2, 3, 2, 3, 4, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>spaced</code></h3><p>The same bins with a visible gap between neighbours instead of touching.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-spaced.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(&quot;Spaced 3D&quot;, variant=&quot;spaced&quot;, x=[1, 2, 2, 3, 3, 3, ...], y=[1, 2, 3, 2, 3, 4, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>highlight</code></h3><p>The five densest bins stand at full tone; every other bin fades to a dim grey.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-highlight.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(&quot;Highlight 3D&quot;, variant=&quot;highlight&quot;, x=[1, 2, 2, 3, 3, 3, ...], y=[1, 2, 3, 2, 3, 4, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>mincnt</code></h3><p>Bins below a minimum count are dropped entirely instead of drawn empty.</p><p class="sp-3d-uses">Uses: <code>x, y, min_count</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-mincnt.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(&quot;Mincnt 3D&quot;, variant=&quot;mincnt&quot;, x=[1, 2, 2, 3, 3, 3, ...], y=[1, 2, 3, 2, 3, 4, ...], min_count=2)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>nested</code></h3><p>Bin height quantised into a handful of magnitude bands instead of a smooth scale.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-nested.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Nested 3D&quot;,
    variant=&quot;nested&quot;,
    gridsize=10,
    x=[4.95, 4.84, 5.9, 5.0, 5.01, 1.12, ...],
    y=[4.99, 4.88, 5.39, 5.0, 5.07, 5.73, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>log_counts</code></h3><p>Bin height follows the logarithm of the count, so a few crowded bins don&#x27;t flatten the rest.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-log_counts.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Log Counts 3D&quot;,
    variant=&quot;log_counts&quot;,
    gridsize=18,
    x=[1, 1, 1, 1, 1, 2, ...],
    y=[1, 2, 3, 4, 5, 1, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>weighted</code></h3><p>Bin height and tone follow the mean of <code>values</code> inside it instead of the raw point count.</p><p class="sp-3d-uses">Uses: <code>x, y, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-weighted.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Weighted 3D&quot;,
    variant=&quot;weighted&quot;,
    gridsize=12,
    x=[1, 2, 2, 3, 3, 3, ...],
    y=[1, 2, 3, 2, 3, 4, ...],
    values=[10, 20, 15, 30, 45, 25, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>dotted</code></h3><p>The same bins as basic, from the 2D chart&#x27;s dashed-outline styling.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-dotted.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Dotted 3D&quot;,
    variant=&quot;dotted&quot;,
    gridsize=15,
    colorscale=&quot;magma&quot;,
    x=[11.03, 12.37, 13.2, 14.37, 13.24, 14.2, ...],
    y=[2.16, 1.63, 1.78, 1.95, 2.59, 1.76, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>marginals</code></h3><p>The same bins plus a marginal histogram column strip along each floor edge.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-marginals.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Marginals 3D&quot;,
    variant=&quot;marginals&quot;,
    gridsize=16,
    x=[11.03, 12.37, 13.2, 14.37, 13.24, 14.2, ...],
    y=[2.16, 1.63, 1.78, 1.95, 2.59, 1.76, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>voronoi</code></h3><p>The same bins as basic, from the 2D chart&#x27;s Voronoi-tessellation styling.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-voronoi.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Voronoi 3D&quot;,
    variant=&quot;voronoi&quot;,
    x=[29.57, 30.24, 30.76, 32.71, 31.63, 33.55, ...],
    y=[7.31, 11.92, 4.24, 10.27, 12.92, 18.23, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>neural</code></h3><p>The same bins as basic, from the 2D chart&#x27;s mesh-styled rendering.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-neural.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Neural 3D&quot;,
    variant=&quot;neural&quot;,
    x=[34.33, 20.67, 32.62, 28.09, 39.82, 28.19, ...],
    y=[16.1, 14.58, 2.2, 7.51, 2.35, 9.34, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>bloom</code></h3><p>The same bins as basic, from the 2D chart&#x27;s soft contour-bloom styling.</p><p class="sp-3d-uses">Uses: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-bloom.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Bloom 3D&quot;,
    variant=&quot;bloom&quot;,
    x=[61.91, 53.31, 61.03, 59.59, 51.74, 60.96, ...],
    y=[12.06, 10.14, 13.19, 12.68, 14.82, 14.32, ...],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="hexbin3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.hexbin3d(title, x=None, y=None, values=None, *, variant="basic", bins=20, min_count=0, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_hexbin3d_chart()`, `sp.hexbin_3d()`, `sp.hexbin3d_chart()`, `sp.hexbin3d_family()`, `sp.hexbins3d()`.

<h2>Description</h2>

`sp.hexbin3d()` est le jumeau 3D de `sp.hexbin()` : **chacune des 13 variantes de hexbin a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes points. Les points x/y bruts sont regroupés en grille hexagonale exactement comme le graphique 2D, et chaque cellule occupée devient une colonne, sa hauteur et sa teinte suivant le nombre de points du bac (ou la moyenne de `values` pour `weighted`).

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic`, `filled` | `x, y` | Une colonne par bac hexagonal, hauteur et teinte suivant le nombre de points qui y tombent. |
| `outlined` | `outline`, `stroke`, `labeled` | `x, y` | Les mêmes bacs pressés en une empreinte fine, façon filaire. |
| `spaced` | `gapped`, `confetti` | `x, y` | Les mêmes bacs avec un espace visible entre voisins au lieu de se toucher. |
| `highlight` | `top`, `hotspot`, `peak` | `x, y` | Les cinq bacs les plus denses ressortent en teinte pleine ; tous les autres s'estompent en gris terne. |
| `mincnt` | `threshold`, `sparse` | `x, y, min_count` | Les bacs sous un seuil minimal sont entièrement retirés plutôt que dessinés vides. |
| `nested` | `magnitude`, `rings`, `centroids` | `x, y` | La hauteur des bacs quantifiée en une poignée de bandes de magnitude plutôt qu'une échelle continue. |
| `log_counts` | `log`, `log_scale`, `logarithmic` | `x, y` | La hauteur des bacs suit le logarithme du nombre de points, pour qu'une poignée de bacs très denses n'écrase pas les autres. |
| `weighted` | `mean`, `aggregate`, `reduce_mean` | `x, y, values` | La hauteur et la teinte des bacs suivent la moyenne de `values` plutôt que le nombre brut de points. |
| `dotted` | `dashed`, `styled`, `magma` | `x, y` | Les mêmes bacs que basic, depuis le style en contour pointillé du graphique 2D. |
| `marginals` | `joint`, `with_histograms`, `density_marginals` | `x, y` | Les mêmes bacs plus une bande de colonnes d'histogramme marginal le long de chaque bord du sol. |
| `voronoi` | `density_voronoi`, `tessellation`, `particle_density`, `cells` | `x, y` | Les mêmes bacs que basic, depuis le style en tessellation de Voronoï du graphique 2D. |
| `neural` | `mesh`, `turbidity`, `particle_mesh`, `neural_mesh` | `x, y` | Les mêmes bacs que basic, depuis le rendu façon maillage du graphique 2D. |
| `bloom` | `contour`, `density_bloom`, `iso_contour`, `organic_contour` | `x, y` | Les mêmes bacs que basic, depuis le style en halo de contour doux du graphique 2D. |

<h2>Données</h2>

`x`/`y` sont les coordonnées brutes des points, `bins` fixe la résolution de la grille et `min_count` le seuil de `mincnt` ; `values` alimente la moyenne par bac de `weighted`.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="hexbin3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une colonne par bac hexagonal, hauteur et teinte suivant le nombre de points qui y tombent.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(&quot;Basic 3D&quot;, variant=&quot;basic&quot;, x=[1, 2, 2, 3, 3, 3, ...], y=[1, 2, 3, 2, 3, 4, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outlined</code></h3><p>Les mêmes bacs pressés en une empreinte fine, façon filaire.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-outlined.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(&quot;Outlined 3D&quot;, variant=&quot;outlined&quot;, x=[1, 2, 2, 3, 3, 3, ...], y=[1, 2, 3, 2, 3, 4, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>spaced</code></h3><p>Les mêmes bacs avec un espace visible entre voisins au lieu de se toucher.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-spaced.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(&quot;Spaced 3D&quot;, variant=&quot;spaced&quot;, x=[1, 2, 2, 3, 3, 3, ...], y=[1, 2, 3, 2, 3, 4, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>highlight</code></h3><p>Les cinq bacs les plus denses ressortent en teinte pleine ; tous les autres s&#x27;estompent en gris terne.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-highlight.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(&quot;Highlight 3D&quot;, variant=&quot;highlight&quot;, x=[1, 2, 2, 3, 3, 3, ...], y=[1, 2, 3, 2, 3, 4, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>mincnt</code></h3><p>Les bacs sous un seuil minimal sont entièrement retirés plutôt que dessinés vides.</p><p class="sp-3d-uses">Utilise: <code>x, y, min_count</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-mincnt.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(&quot;Mincnt 3D&quot;, variant=&quot;mincnt&quot;, x=[1, 2, 2, 3, 3, 3, ...], y=[1, 2, 3, 2, 3, 4, ...], min_count=2)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>nested</code></h3><p>La hauteur des bacs quantifiée en une poignée de bandes de magnitude plutôt qu&#x27;une échelle continue.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-nested.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Nested 3D&quot;,
    variant=&quot;nested&quot;,
    gridsize=10,
    x=[4.95, 4.84, 5.9, 5.0, 5.01, 1.12, ...],
    y=[4.99, 4.88, 5.39, 5.0, 5.07, 5.73, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>log_counts</code></h3><p>La hauteur des bacs suit le logarithme du nombre de points, pour qu&#x27;une poignée de bacs très denses n&#x27;écrase pas les autres.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-log_counts.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Log Counts 3D&quot;,
    variant=&quot;log_counts&quot;,
    gridsize=18,
    x=[1, 1, 1, 1, 1, 2, ...],
    y=[1, 2, 3, 4, 5, 1, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>weighted</code></h3><p>La hauteur et la teinte des bacs suivent la moyenne de <code>values</code> plutôt que le nombre brut de points.</p><p class="sp-3d-uses">Utilise: <code>x, y, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-weighted.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Weighted 3D&quot;,
    variant=&quot;weighted&quot;,
    gridsize=12,
    x=[1, 2, 2, 3, 3, 3, ...],
    y=[1, 2, 3, 2, 3, 4, ...],
    values=[10, 20, 15, 30, 45, 25, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>dotted</code></h3><p>Les mêmes bacs que basic, depuis le style en contour pointillé du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-dotted.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Dotted 3D&quot;,
    variant=&quot;dotted&quot;,
    gridsize=15,
    colorscale=&quot;magma&quot;,
    x=[11.03, 12.37, 13.2, 14.37, 13.24, 14.2, ...],
    y=[2.16, 1.63, 1.78, 1.95, 2.59, 1.76, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>marginals</code></h3><p>Les mêmes bacs plus une bande de colonnes d&#x27;histogramme marginal le long de chaque bord du sol.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-marginals.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Marginals 3D&quot;,
    variant=&quot;marginals&quot;,
    gridsize=16,
    x=[11.03, 12.37, 13.2, 14.37, 13.24, 14.2, ...],
    y=[2.16, 1.63, 1.78, 1.95, 2.59, 1.76, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>voronoi</code></h3><p>Les mêmes bacs que basic, depuis le style en tessellation de Voronoï du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-voronoi.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Voronoi 3D&quot;,
    variant=&quot;voronoi&quot;,
    x=[29.57, 30.24, 30.76, 32.71, 31.63, 33.55, ...],
    y=[7.31, 11.92, 4.24, 10.27, 12.92, 18.23, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>neural</code></h3><p>Les mêmes bacs que basic, depuis le rendu façon maillage du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-neural.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Neural 3D&quot;,
    variant=&quot;neural&quot;,
    x=[34.33, 20.67, 32.62, 28.09, 39.82, 28.19, ...],
    y=[16.1, 14.58, 2.2, 7.51, 2.35, 9.34, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>bloom</code></h3><p>Les mêmes bacs que basic, depuis le style en halo de contour doux du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/hexbin3d-bloom.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hexbin3d(
    &quot;Bloom 3D&quot;,
    variant=&quot;bloom&quot;,
    x=[61.91, 53.31, 61.03, 59.59, 51.74, 60.96, ...],
    y=[12.06, 10.14, 13.19, 12.68, 14.82, 14.32, ...],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="hexbin3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
