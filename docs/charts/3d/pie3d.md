# Pie Chart 3D

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

`sp.pie3d(title, labels=None, values=None, secondary_values=None, *, sort_order=None, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=700, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_pie3d_chart()`, `sp.pie_3d()`, `sp.pie3d_chart()`, `sp.pie3d_family()`, `sp.pies3d()`.

## Description

`sp.pie3d()` is the 3D twin of `sp.pie()`: **all 11 pie variants share the same 3D form**, selected with the same `variant` keyword and fed with the same slices. A pie wedge has no honest cuboid shape, so every slice instead becomes a column around a ring, height following its value. `semi` sweeps that ring over a half-circle instead of a full one; `nested` adds a second, larger ring from `secondary_values`.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `pie` | `labels, values` | One column per slice around a ring, height following its value. |
| `donut` | `ring`, `hole` | `labels, values` | The same ring as basic; the 2D chart's hollow-center styling has no separate 3D form. |
| `exploded` | `pulled`, `pull`, `explode` | `labels, values` | The same ring as basic; the 2D chart's pulled-out slice styling has no separate 3D form. |
| `subplots` | `grid`, `facet`, `multi` | `labels, values` | The same ring as basic; the 2D chart's small-multiples layout has no separate 3D form. |
| `proportional` | `scaled`, `scalegroup`, `area_proportional` | `labels, values` | The same ring as basic; the 2D chart's area-scaled subplot sizing has no separate 3D form. |
| `semi` | `semicircle`, `half`, `halfpie`, `half_pie` | `labels, values` | The same columns, swept over a half-circle instead of a full ring. |
| `kpi` | `center`, `centered`, `indicator`, `donut_kpi`, `metric` | `labels, values` | The same ring as basic; the 2D chart's centered indicator has no separate 3D form. |
| `nested` | `concentric`, `rings`, `double_ring`, `multi_ring` | `labels, values, secondary_values` | The same ring as basic plus a second, larger ring from `secondary_values`. |
| `pattern` | `patterned`, `textured`, `hatched` | `labels, values` | The same ring as basic; the 2D chart's hatched-fill styling has no separate 3D form. |
| `nightingale` | `rose`, `coxcomb`, `polar_area` | `labels, values` | The same ring as basic; the 2D chart's radius-by-value coxcomb styling has no separate 3D form. |
| `waffle` | `square`, `pie_square`, `grid_pie`, `squarepie` | `labels, values` | The same ring as basic; the 2D chart's square-grid styling has no separate 3D form. |

## Data

`labels` names each slice and `values` sets its column height; `nested` additionally reads `secondary_values` for its outer ring.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="pie3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One column per slice around a ring, height following its value.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>donut</code></h3><p>The same ring as basic; the 2D chart&#x27;s hollow-center styling has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-donut.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Donut 3D&quot;,
    variant=&quot;donut&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>exploded</code></h3><p>The same ring as basic; the 2D chart&#x27;s pulled-out slice styling has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-exploded.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Exploded 3D&quot;,
    variant=&quot;exploded&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>subplots</code></h3><p>The same ring as basic; the 2D chart&#x27;s small-multiples layout has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-subplots.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Subplots 3D&quot;,
    variant=&quot;subplots&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;],
    series=[[40, 25, 20, 15], [30, 30, 20, 20], [50, 20, 15, 15]],
    series_names=[&quot;P1&quot;, &quot;P2&quot;, &quot;P3&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>proportional</code></h3><p>The same ring as basic; the 2D chart&#x27;s area-scaled subplot sizing has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-proportional.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Proportional 3D&quot;,
    variant=&quot;proportional&quot;,
    labels=[&quot;North&quot;, &quot;South&quot;, &quot;East&quot;, &quot;West&quot;],
    series=[[120, 80, 60, 40], [35, 25, 15, 10], [8, 6, 4, 2]],
    series_names=[&quot;Region A&quot;, &quot;Region B&quot;, &quot;Region C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>semi</code></h3><p>The same columns, swept over a half-circle instead of a full ring.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-semi.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Semi 3D&quot;,
    variant=&quot;semi&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>kpi</code></h3><p>The same ring as basic; the 2D chart&#x27;s centered indicator has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-kpi.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Kpi 3D&quot;,
    variant=&quot;kpi&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>nested</code></h3><p>The same ring as basic plus a second, larger ring from <code>secondary_values</code>.</p><p class="sp-3d-uses">Uses: <code>labels, values, secondary_values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-nested.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Nested 3D&quot;,
    variant=&quot;nested&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
    secondary_labels=[&quot;X&quot;, &quot;Y&quot;, &quot;Z&quot;],
    secondary_values=[55, 30, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>pattern</code></h3><p>The same ring as basic; the 2D chart&#x27;s hatched-fill styling has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-pattern.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Pattern 3D&quot;,
    variant=&quot;pattern&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>nightingale</code></h3><p>The same ring as basic; the 2D chart&#x27;s radius-by-value coxcomb styling has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-nightingale.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Nightingale 3D&quot;,
    variant=&quot;nightingale&quot;,
    labels=[&quot;1&quot;, &quot;2&quot;, &quot;3&quot;, &quot;4&quot;, &quot;5&quot;, &quot;6&quot;, &quot;7&quot;, &quot;8&quot;],
    values=[1.0, 0.802, 0.853, 0.879, 0.892, 0.886, 0.910, 0.966],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>waffle</code></h3><p>The same ring as basic; the 2D chart&#x27;s square-grid styling has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-waffle.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Waffle 3D&quot;,
    variant=&quot;waffle&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="pie3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.pie3d(title, labels=None, values=None, secondary_values=None, *, sort_order=None, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=700, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_pie3d_chart()`, `sp.pie_3d()`, `sp.pie3d_chart()`, `sp.pie3d_family()`, `sp.pies3d()`.

<h2>Description</h2>

`sp.pie3d()` est le jumeau 3D de `sp.pie()` : **les 11 variantes de pie partagent la même forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes parts. Une part de camembert n'a pas de forme cuboïde honnête, donc chaque part devient plutôt une colonne autour d'un anneau, la hauteur suivant sa valeur. `semi` balaie cet anneau sur un demi-cercle plutôt qu'un cercle complet ; `nested` ajoute un second anneau, plus grand, depuis `secondary_values`.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `pie` | `labels, values` | Une colonne par part autour d'un anneau, la hauteur suivant sa valeur. |
| `donut` | `ring`, `hole` | `labels, values` | Le même anneau que basic ; le style à centre creux du graphique 2D n'a pas de forme 3D séparée. |
| `exploded` | `pulled`, `pull`, `explode` | `labels, values` | Le même anneau que basic ; le style de part détachée du graphique 2D n'a pas de forme 3D séparée. |
| `subplots` | `grid`, `facet`, `multi` | `labels, values` | Le même anneau que basic ; la disposition en petits multiples du graphique 2D n'a pas de forme 3D séparée. |
| `proportional` | `scaled`, `scalegroup`, `area_proportional` | `labels, values` | Le même anneau que basic ; le dimensionnement par aire du graphique 2D n'a pas de forme 3D séparée. |
| `semi` | `semicircle`, `half`, `halfpie`, `half_pie` | `labels, values` | Les mêmes colonnes, balayées sur un demi-cercle plutôt qu'un anneau complet. |
| `kpi` | `center`, `centered`, `indicator`, `donut_kpi`, `metric` | `labels, values` | Le même anneau que basic ; l'indicateur centré du graphique 2D n'a pas de forme 3D séparée. |
| `nested` | `concentric`, `rings`, `double_ring`, `multi_ring` | `labels, values, secondary_values` | Le même anneau que basic plus un second anneau, plus grand, depuis `secondary_values`. |
| `pattern` | `patterned`, `textured`, `hatched` | `labels, values` | Le même anneau que basic ; le style à remplissage hachuré du graphique 2D n'a pas de forme 3D séparée. |
| `nightingale` | `rose`, `coxcomb`, `polar_area` | `labels, values` | Le même anneau que basic ; le style en rose des vents (rayon selon la valeur) du graphique 2D n'a pas de forme 3D séparée. |
| `waffle` | `square`, `pie_square`, `grid_pie`, `squarepie` | `labels, values` | Le même anneau que basic ; le style en grille carrée du graphique 2D n'a pas de forme 3D séparée. |

<h2>Données</h2>

`labels` nomme chaque part et `values` fixe la hauteur de sa colonne ; `nested` lit en plus `secondary_values` pour son anneau extérieur.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="pie3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pie3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une colonne par part autour d&#x27;un anneau, la hauteur suivant sa valeur.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>donut</code></h3><p>Le même anneau que basic ; le style à centre creux du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-donut.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Donut 3D&quot;,
    variant=&quot;donut&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>exploded</code></h3><p>Le même anneau que basic ; le style de part détachée du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-exploded.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Exploded 3D&quot;,
    variant=&quot;exploded&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>subplots</code></h3><p>Le même anneau que basic ; la disposition en petits multiples du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-subplots.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Subplots 3D&quot;,
    variant=&quot;subplots&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;],
    series=[[40, 25, 20, 15], [30, 30, 20, 20], [50, 20, 15, 15]],
    series_names=[&quot;P1&quot;, &quot;P2&quot;, &quot;P3&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>proportional</code></h3><p>Le même anneau que basic ; le dimensionnement par aire du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-proportional.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Proportional 3D&quot;,
    variant=&quot;proportional&quot;,
    labels=[&quot;North&quot;, &quot;South&quot;, &quot;East&quot;, &quot;West&quot;],
    series=[[120, 80, 60, 40], [35, 25, 15, 10], [8, 6, 4, 2]],
    series_names=[&quot;Region A&quot;, &quot;Region B&quot;, &quot;Region C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>semi</code></h3><p>Les mêmes colonnes, balayées sur un demi-cercle plutôt qu&#x27;un anneau complet.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-semi.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Semi 3D&quot;,
    variant=&quot;semi&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>kpi</code></h3><p>Le même anneau que basic ; l&#x27;indicateur centré du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-kpi.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Kpi 3D&quot;,
    variant=&quot;kpi&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>nested</code></h3><p>Le même anneau que basic plus un second anneau, plus grand, depuis <code>secondary_values</code>.</p><p class="sp-3d-uses">Utilise: <code>labels, values, secondary_values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-nested.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Nested 3D&quot;,
    variant=&quot;nested&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
    secondary_labels=[&quot;X&quot;, &quot;Y&quot;, &quot;Z&quot;],
    secondary_values=[55, 30, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>pattern</code></h3><p>Le même anneau que basic ; le style à remplissage hachuré du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-pattern.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Pattern 3D&quot;,
    variant=&quot;pattern&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>nightingale</code></h3><p>Le même anneau que basic ; le style en rose des vents (rayon selon la valeur) du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-nightingale.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Nightingale 3D&quot;,
    variant=&quot;nightingale&quot;,
    labels=[&quot;1&quot;, &quot;2&quot;, &quot;3&quot;, &quot;4&quot;, &quot;5&quot;, &quot;6&quot;, &quot;7&quot;, &quot;8&quot;],
    values=[1.0, 0.802, 0.853, 0.879, 0.892, 0.886, 0.910, 0.966],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>waffle</code></h3><p>Le même anneau que basic ; le style en grille carrée du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pie3d-waffle.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pie3d(
    &quot;Waffle 3D&quot;,
    variant=&quot;waffle&quot;,
    labels=[&quot;Apple&quot;, &quot;Banana&quot;, &quot;Cherry&quot;, &quot;Date&quot;, &quot;Fig&quot;],
    values=[40, 25, 20, 10, 5],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="pie3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
