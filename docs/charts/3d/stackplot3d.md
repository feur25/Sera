# Stackplot Chart 3D

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

`sp.stackplot3d(title, x_labels=None, series=None, *, variant="basic", series_names=None, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_stackplot3d_chart()`, `sp.stackplot_3d()`, `sp.stackplot3d_chart()`, `sp.stackplot3d_family()`, `sp.stackplots3d()`.

## Description

`sp.stackplot3d()` is the 3D twin of `sp.stackplot()`: **every one of the 5 stackplot variants has a 3D form**, selected with the same `variant` keyword and fed with the same series. Every series is a sloped band whose floor is the running total of the series below it, reusing the same shared 2D stacking maths (`bottoms`/`tops` per point) as the 2D chart so the silhouette matches exactly.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic`, `stacked` | `x_labels, series` | Series stacked from the floor, each band's bottom the running total of the ones before it. |
| `streamgraph` | `stream`, `silhouette`, `themeriver` | `x_labels, series` | The same stack centred around zero instead of the floor, so it reads as an organic flowing ribbon. |
| `normalized` | `percent`, `hundred_percent`, `share` | `x_labels, series` | The stack normalised so every point's total reads exactly 100%. |
| `radial` | `polar`, `radar_stack`, `circular` | `x_labels, series` | The stack wrapped around a ring: each x position becomes an angle, with a vertical stack of series rising from it. |
| `ribbon` | `glow`, `smooth`, `flow` | `x_labels, series` | The same floor-anchored stack with smoothed, spline-interpolated edges. |

## Data

`x_labels` name the points and `series` (with `series_names`) gives the stacked value lists.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="stackplot3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Series stacked from the floor, each band&#x27;s bottom the running total of the ones before it.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.stackplot3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;],
    series=[[10, 14, 12, 18, 20], [8, 9, 11, 10, 13], [5, 6, 7, 9, 8]],
    series_names=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>streamgraph</code></h3><p>The same stack centred around zero instead of the floor, so it reads as an organic flowing ribbon.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-streamgraph.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.stackplot3d(
    &quot;Streamgraph 3D&quot;,
    variant=&quot;streamgraph&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;],
    series=[[10, 14, 12, 18, 20], [8, 9, 11, 10, 13], [5, 6, 7, 9, 8]],
    series_names=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>normalized</code></h3><p>The stack normalised so every point&#x27;s total reads exactly 100%.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-normalized.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.stackplot3d(
    &quot;Normalized 3D&quot;,
    variant=&quot;normalized&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;],
    series=[[10, 14, 12, 18, 20], [8, 9, 11, 10, 13], [5, 6, 7, 9, 8]],
    series_names=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>radial</code></h3><p>The stack wrapped around a ring: each x position becomes an angle, with a vertical stack of series rising from it.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-radial.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.stackplot3d(
    &quot;Radial 3D&quot;,
    variant=&quot;radial&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;, &quot;Jun&quot;, &quot;Jul&quot;, &quot;Aug&quot;],
    series=[[10, 14, 12, 18, 20, 16, 13, 17], [8, 9, 11, 10, 13, 12, 9, 10], [5, 6, 7, 9, 8, 7, 6, 8]],
    series_names=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>ribbon</code></h3><p>The same floor-anchored stack with smoothed, spline-interpolated edges.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-ribbon.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.stackplot3d(
    &quot;Ribbon 3D&quot;,
    variant=&quot;ribbon&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;, &quot;Jun&quot;],
    series=[[10, 14, 12, 18, 20, 17], [8, 9, 11, 10, 13, 15], [5, 6, 7, 9, 8, 11]],
    series_names=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="stackplot3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.stackplot3d(title, x_labels=None, series=None, *, variant="basic", series_names=None, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_stackplot3d_chart()`, `sp.stackplot_3d()`, `sp.stackplot3d_chart()`, `sp.stackplot3d_family()`, `sp.stackplots3d()`.

<h2>Description</h2>

`sp.stackplot3d()` est le jumeau 3D de `sp.stackplot()` : **chacune des 5 variantes de stackplot a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes séries. Chaque série est une bande inclinée dont le plancher est le cumul des séries en dessous : le même calcul d'empilement 2D partagé (`bottoms`/`tops` par point) est réutilisé tel quel, si bien que la silhouette correspond exactement.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic`, `stacked` | `x_labels, series` | Séries empilées depuis le sol, le bas de chaque bande étant le cumul de celles qui la précèdent. |
| `streamgraph` | `stream`, `silhouette`, `themeriver` | `x_labels, series` | Le même empilement centré autour de zéro plutôt que sur le sol, pour une lecture en ruban organique. |
| `normalized` | `percent`, `hundred_percent`, `share` | `x_labels, series` | L'empilement normalisé pour que le total de chaque point lise exactement 100 %. |
| `radial` | `polar`, `radar_stack`, `circular` | `x_labels, series` | L'empilement enroulé autour d'un anneau : chaque position x devient un angle, avec une pile verticale de séries qui s'en élève. |
| `ribbon` | `glow`, `smooth`, `flow` | `x_labels, series` | Le même empilement ancré au sol avec des bords lissés, interpolés en spline. |

<h2>Données</h2>

`x_labels` nomme les points et `series` (avec `series_names`) donne les listes de valeurs empilées.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="stackplot3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Séries empilées depuis le sol, le bas de chaque bande étant le cumul de celles qui la précèdent.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.stackplot3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;],
    series=[[10, 14, 12, 18, 20], [8, 9, 11, 10, 13], [5, 6, 7, 9, 8]],
    series_names=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>streamgraph</code></h3><p>Le même empilement centré autour de zéro plutôt que sur le sol, pour une lecture en ruban organique.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-streamgraph.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.stackplot3d(
    &quot;Streamgraph 3D&quot;,
    variant=&quot;streamgraph&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;],
    series=[[10, 14, 12, 18, 20], [8, 9, 11, 10, 13], [5, 6, 7, 9, 8]],
    series_names=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>normalized</code></h3><p>L&#x27;empilement normalisé pour que le total de chaque point lise exactement 100 %.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-normalized.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.stackplot3d(
    &quot;Normalized 3D&quot;,
    variant=&quot;normalized&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;],
    series=[[10, 14, 12, 18, 20], [8, 9, 11, 10, 13], [5, 6, 7, 9, 8]],
    series_names=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>radial</code></h3><p>L&#x27;empilement enroulé autour d&#x27;un anneau : chaque position x devient un angle, avec une pile verticale de séries qui s&#x27;en élève.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-radial.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.stackplot3d(
    &quot;Radial 3D&quot;,
    variant=&quot;radial&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;, &quot;Jun&quot;, &quot;Jul&quot;, &quot;Aug&quot;],
    series=[[10, 14, 12, 18, 20, 16, 13, 17], [8, 9, 11, 10, 13, 12, 9, 10], [5, 6, 7, 9, 8, 7, 6, 8]],
    series_names=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>ribbon</code></h3><p>Le même empilement ancré au sol avec des bords lissés, interpolés en spline.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/stackplot3d-ribbon.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.stackplot3d(
    &quot;Ribbon 3D&quot;,
    variant=&quot;ribbon&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;, &quot;Jun&quot;],
    series=[[10, 14, 12, 18, 20, 17], [8, 9, 11, 10, 13, 15], [5, 6, 7, 9, 8, 11]],
    series_names=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="stackplot3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
