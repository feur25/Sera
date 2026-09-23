# KDE Chart 3D

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

`sp.kde3d(title, categories=None, values=None, *, variant="basic", x=None, y=None, bins=30, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_kde3d_chart()`, `sp.kde_3d()`, `sp.kde3d_chart()`, `sp.density3d()`.

## Description

`sp.kde3d()` is the 3D twin of `sp.kde()`: **every one of the 11 KDE variants has a 3D form**, selected with the same `variant` keyword and fed with the same samples. Each named group becomes its own Gaussian KDE curve on its own depth row, filled, stacked or drawn as an outline depending on the variant; `contour` and `levels` instead compute a full bivariate density field from paired `x`/`y` samples, reusing the same 2D Gaussian kernel as the 2D contour chart.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `filled`, `default`, `single`, `multi` | `categories, values` | One filled density curve per group, each on its own depth row. |
| `outline` | `line`, `stroke`, `compare`, `no_fill` | `categories, values` | The same curves as a thin ribbon only, no fill underneath. |
| `stepped` | `step`, `stair`, `stairs` | `categories, values` | The density curve turned into a staircase profile before it is filled. |
| `rug` | `carpet`, `ticks`, `rugplot` | `categories, values` | A thin filled curve plus a small tick at every raw sample's position. |
| `histogram` | `hist`, `with_hist`, `kdehist`, `distplot` | `categories, values, bins` | The density curve as a ribbon in front of the group's own binned histogram columns. |
| `normalized` | `pdf`, `norm`, `density` | `categories, values` | The same filled curves as basic, read as a strict probability density. |
| `cumulative` | `cdf`, `cum` | `categories, values` | The running integral of the density instead of the density itself, rising monotonically to one. |
| `contour` | `bivariate`, `kde2d`, `joint_density`, `smooth` | `categories, x, y` | A bivariate density field from paired x/y samples, one grid of columns per group toned by density. |
| `levels` | `bands`, `iso_bands`, `ring_contour`, `banded` | `categories, x, y` | The same bivariate field quantised into a handful of discrete height bands, like iso-contour rings. |
| `stack` | `stacked`, `layered_stack` | `categories, values` | Every group's density curve stacked on top of the one before it. |
| `fill` | `stack100`, `percent_stack`, `filled_stack` | `categories, values` | The same stack normalised so every point's total reads exactly 100%. |

## Data

`categories` names each sample's group and `values` (or `x`) gives its number; groups without `categories` become one series. `y` (paired with `x`) feeds the bivariate `contour`/`levels` density field, and `bins` sets the `histogram` overlay's bin count.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="kde3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One filled density curve per group, each on its own depth row.</p><p class="sp-3d-uses">Uses: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    values=[4.74, 5.51, 4.77, 4.68, 4.07, 4.79, ...],
    categories=[&quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outline</code></h3><p>The same curves as a thin ribbon only, no fill underneath.</p><p class="sp-3d-uses">Uses: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-outline.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(&quot;Outline 3D&quot;, variant=&quot;outline&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stepped</code></h3><p>The density curve turned into a staircase profile before it is filled.</p><p class="sp-3d-uses">Uses: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-stepped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(&quot;Stepped 3D&quot;, variant=&quot;stepped&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>rug</code></h3><p>A thin filled curve plus a small tick at every raw sample&#x27;s position.</p><p class="sp-3d-uses">Uses: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-rug.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(&quot;Rug 3D&quot;, variant=&quot;rug&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>histogram</code></h3><p>The density curve as a ribbon in front of the group&#x27;s own binned histogram columns.</p><p class="sp-3d-uses">Uses: <code>categories, values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-histogram.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(&quot;Histogram 3D&quot;, variant=&quot;histogram&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>normalized</code></h3><p>The same filled curves as basic, read as a strict probability density.</p><p class="sp-3d-uses">Uses: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-normalized.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(
    &quot;Normalized 3D&quot;,
    variant=&quot;normalized&quot;,
    values=[4.74, 5.51, 4.77, 4.68, 4.07, 4.79, ...],
    categories=[&quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>cumulative</code></h3><p>The running integral of the density instead of the density itself, rising monotonically to one.</p><p class="sp-3d-uses">Uses: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-cumulative.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(&quot;Cumulative 3D&quot;, variant=&quot;cumulative&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>contour</code></h3><p>A bivariate density field from paired x/y samples, one grid of columns per group toned by density.</p><p class="sp-3d-uses">Uses: <code>categories, x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-contour.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(
    &quot;Contour 3D&quot;,
    variant=&quot;contour&quot;,
    x=[55.4, 60.0, 51.3, 59.0, 54.0, 54.0, ...],
    y=[1.51, 1.93, 1.95, 1.9, 2.02, 1.6, ...],
    categories=[&quot;short&quot;, &quot;short&quot;, &quot;short&quot;, &quot;short&quot;, &quot;short&quot;, &quot;short&quot;, ...],
    x_label=&quot;waiting&quot;,
    y_label=&quot;duration&quot;,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>levels</code></h3><p>The same bivariate field quantised into a handful of discrete height bands, like iso-contour rings.</p><p class="sp-3d-uses">Uses: <code>categories, x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-levels.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(
    &quot;Levels 3D&quot;,
    variant=&quot;levels&quot;,
    x=[55.4, 60.0, 51.3, 59.0, 54.0, 54.0, ...],
    y=[1.51, 1.93, 1.95, 1.9, 2.02, 1.6, ...],
    categories=[&quot;short&quot;, &quot;short&quot;, &quot;short&quot;, &quot;short&quot;, &quot;short&quot;, &quot;short&quot;, ...],
    x_label=&quot;waiting&quot;,
    y_label=&quot;duration&quot;,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stack</code></h3><p>Every group&#x27;s density curve stacked on top of the one before it.</p><p class="sp-3d-uses">Uses: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-stack.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(
    &quot;Stack 3D&quot;,
    variant=&quot;stack&quot;,
    values=[4.74, 5.51, 4.77, 4.68, 4.07, 4.79, ...],
    categories=[&quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>fill</code></h3><p>The same stack normalised so every point&#x27;s total reads exactly 100%.</p><p class="sp-3d-uses">Uses: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-fill.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(
    &quot;Fill 3D&quot;,
    variant=&quot;fill&quot;,
    values=[4.74, 5.51, 4.77, 4.68, 4.07, 4.79, ...],
    categories=[&quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, ...],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="kde3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.kde3d(title, categories=None, values=None, *, variant="basic", x=None, y=None, bins=30, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_kde3d_chart()`, `sp.kde_3d()`, `sp.kde3d_chart()`, `sp.density3d()`.

<h2>Description</h2>

`sp.kde3d()` est le jumeau 3D de `sp.kde()` : **chacune des 11 variantes de KDE a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes échantillons. Chaque groupe nommé devient sa propre courbe de densité gaussienne sur sa propre rangée de profondeur, remplie, empilée ou tracée en contour selon la variante ; `contour` et `levels` calculent à la place un champ de densité bivarié complet à partir de paires `x`/`y`, réutilisant le même noyau gaussien 2D que le graphique de contour 2D.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `filled`, `default`, `single`, `multi` | `categories, values` | Une courbe de densité remplie par groupe, chacune sur sa propre rangée de profondeur. |
| `outline` | `line`, `stroke`, `compare`, `no_fill` | `categories, values` | Les mêmes courbes en simple ruban fin, sans remplissage en dessous. |
| `stepped` | `step`, `stair`, `stairs` | `categories, values` | La courbe de densité transformée en profil d'escalier avant d'être remplie. |
| `rug` | `carpet`, `ticks`, `rugplot` | `categories, values` | Une fine courbe remplie plus une petite marque à la position de chaque échantillon brut. |
| `histogram` | `hist`, `with_hist`, `kdehist`, `distplot` | `categories, values, bins` | La courbe de densité en ruban devant les colonnes de l'histogramme binné propre au groupe. |
| `normalized` | `pdf`, `norm`, `density` | `categories, values` | Les mêmes courbes remplies que basic, lues comme une densité de probabilité stricte. |
| `cumulative` | `cdf`, `cum` | `categories, values` | L'intégrale cumulée de la densité au lieu de la densité elle-même, montant de façon monotone jusqu'à un. |
| `contour` | `bivariate`, `kde2d`, `joint_density`, `smooth` | `categories, x, y` | Un champ de densité bivarié à partir de paires x/y, une grille de colonnes par groupe teintée selon la densité. |
| `levels` | `bands`, `iso_bands`, `ring_contour`, `banded` | `categories, x, y` | Le même champ bivarié quantifié en une poignée de bandes de hauteur discrètes, façon anneaux iso-contour. |
| `stack` | `stacked`, `layered_stack` | `categories, values` | La courbe de densité de chaque groupe empilée sur celle qui précède. |
| `fill` | `stack100`, `percent_stack`, `filled_stack` | `categories, values` | Le même empilement normalisé pour que le total de chaque point lise exactement 100 %. |

<h2>Données</h2>

`categories` nomme le groupe de chaque échantillon et `values` (ou `x`) donne son nombre ; sans `categories`, les échantillons forment une seule série. `y` (associé à `x`) alimente le champ de densité bivarié de `contour`/`levels`, et `bins` fixe le nombre de classes de la superposition `histogram`.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="kde3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/kde3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une courbe de densité remplie par groupe, chacune sur sa propre rangée de profondeur.</p><p class="sp-3d-uses">Utilise: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    values=[4.74, 5.51, 4.77, 4.68, 4.07, 4.79, ...],
    categories=[&quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outline</code></h3><p>Les mêmes courbes en simple ruban fin, sans remplissage en dessous.</p><p class="sp-3d-uses">Utilise: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-outline.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(&quot;Outline 3D&quot;, variant=&quot;outline&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stepped</code></h3><p>La courbe de densité transformée en profil d&#x27;escalier avant d&#x27;être remplie.</p><p class="sp-3d-uses">Utilise: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-stepped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(&quot;Stepped 3D&quot;, variant=&quot;stepped&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>rug</code></h3><p>Une fine courbe remplie plus une petite marque à la position de chaque échantillon brut.</p><p class="sp-3d-uses">Utilise: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-rug.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(&quot;Rug 3D&quot;, variant=&quot;rug&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>histogram</code></h3><p>La courbe de densité en ruban devant les colonnes de l&#x27;histogramme binné propre au groupe.</p><p class="sp-3d-uses">Utilise: <code>categories, values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-histogram.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(&quot;Histogram 3D&quot;, variant=&quot;histogram&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>normalized</code></h3><p>Les mêmes courbes remplies que basic, lues comme une densité de probabilité stricte.</p><p class="sp-3d-uses">Utilise: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-normalized.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(
    &quot;Normalized 3D&quot;,
    variant=&quot;normalized&quot;,
    values=[4.74, 5.51, 4.77, 4.68, 4.07, 4.79, ...],
    categories=[&quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>cumulative</code></h3><p>L&#x27;intégrale cumulée de la densité au lieu de la densité elle-même, montant de façon monotone jusqu&#x27;à un.</p><p class="sp-3d-uses">Utilise: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-cumulative.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(&quot;Cumulative 3D&quot;, variant=&quot;cumulative&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>contour</code></h3><p>Un champ de densité bivarié à partir de paires x/y, une grille de colonnes par groupe teintée selon la densité.</p><p class="sp-3d-uses">Utilise: <code>categories, x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-contour.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(
    &quot;Contour 3D&quot;,
    variant=&quot;contour&quot;,
    x=[55.4, 60.0, 51.3, 59.0, 54.0, 54.0, ...],
    y=[1.51, 1.93, 1.95, 1.9, 2.02, 1.6, ...],
    categories=[&quot;short&quot;, &quot;short&quot;, &quot;short&quot;, &quot;short&quot;, &quot;short&quot;, &quot;short&quot;, ...],
    x_label=&quot;waiting&quot;,
    y_label=&quot;duration&quot;,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>levels</code></h3><p>Le même champ bivarié quantifié en une poignée de bandes de hauteur discrètes, façon anneaux iso-contour.</p><p class="sp-3d-uses">Utilise: <code>categories, x, y</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-levels.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(
    &quot;Levels 3D&quot;,
    variant=&quot;levels&quot;,
    x=[55.4, 60.0, 51.3, 59.0, 54.0, 54.0, ...],
    y=[1.51, 1.93, 1.95, 1.9, 2.02, 1.6, ...],
    categories=[&quot;short&quot;, &quot;short&quot;, &quot;short&quot;, &quot;short&quot;, &quot;short&quot;, &quot;short&quot;, ...],
    x_label=&quot;waiting&quot;,
    y_label=&quot;duration&quot;,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stack</code></h3><p>La courbe de densité de chaque groupe empilée sur celle qui précède.</p><p class="sp-3d-uses">Utilise: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-stack.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(
    &quot;Stack 3D&quot;,
    variant=&quot;stack&quot;,
    values=[4.74, 5.51, 4.77, 4.68, 4.07, 4.79, ...],
    categories=[&quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>fill</code></h3><p>Le même empilement normalisé pour que le total de chaque point lise exactement 100 %.</p><p class="sp-3d-uses">Utilise: <code>categories, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/kde3d-fill.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.kde3d(
    &quot;Fill 3D&quot;,
    variant=&quot;fill&quot;,
    values=[4.74, 5.51, 4.77, 4.68, 4.07, 4.79, ...],
    categories=[&quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, &quot;Large group&quot;, ...],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="kde3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
