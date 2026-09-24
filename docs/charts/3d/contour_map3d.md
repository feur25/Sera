# Contour Map 3D

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

`sp.contour_map3d(title, lats=None, lons=None, field=None, *, levels=6, variant="filled", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=1200, height=650, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_contour_map3d_chart()`, `sp.contour_map_3d()`, `sp.contour_map3d_chart()`, `sp.contourmap3d()`.

## Description

`sp.contour_map3d()` is the 3D twin of `sp.contour_map()`: **all 3 contour-map variants share the same 3D form**, selected with the same `variant` keyword. The scattered `lats`/`lons`/`field` samples are interpolated onto a regular grid (the same inverse-distance-weighted interpolation the 2D chart itself uses) and drawn as one column per cell, height and tone following the interpolated value there — a real density surface, not a scatter of points. `extrema` adds a tall marker at every local high and low the grid contains.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `filled` | `bands`, `heat`, `basic`, `default` | `lats, lons, field` | A full grid surface, one column per cell. |
| `isolines` | `lines`, `contour_lines`, `iso` | `lats, lons, field` | The same grid surface as filled; the 2D chart's stroked-line styling has no separate 3D form. |
| `extrema` | `highs_lows`, `hl`, `pressure_centers` | `lats, lons, field` | The same grid surface plus a tall marker at every local high and low. |

## Data

`lats`/`lons` position each sample and `field` gives its value; all three are interpolated onto the same regular grid regardless of variant.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="contour_map3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>filled</code></h3><p>A full grid surface, one column per cell.</p><p class="sp-3d-uses">Uses: <code>lats, lons, field</code></p><iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-filled.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.contour_map3d(
    &quot;Filled 3D&quot;,
    variant=&quot;filled&quot;,
    lats=[64.1, 61.2, 68.9, 62.0, 59.9, 55.75, ...],
    lons=[-21.9, -149.9, 33.0, 129.7, 10.75, 37.6, ...],
    field=[4, 2, -3, -15, 6, 5, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>isolines</code></h3><p>The same grid surface as filled; the 2D chart&#x27;s stroked-line styling has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>lats, lons, field</code></p><iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-isolines.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.contour_map3d(
    &quot;Isolines 3D&quot;,
    variant=&quot;isolines&quot;,
    lats=[65, 55, 35, 55, 32, -28, ...],
    lons=[-20, -165, -25, 90, -140, -105, ...],
    field=[-12, -10, 11, 15, 9, 10, ...],
    bins=7,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>extrema</code></h3><p>The same grid surface plus a tall marker at every local high and low.</p><p class="sp-3d-uses">Uses: <code>lats, lons, field</code></p><iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-extrema.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.contour_map3d(
    &quot;Extrema 3D&quot;,
    variant=&quot;extrema&quot;,
    lats=[65, 55, 35, 55, 32, -28, ...],
    lons=[-20, -165, -25, 90, -140, -105, ...],
    field=[-12, -10, 11, 15, 9, 10, ...],
    bins=5,
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="contour_map3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.contour_map3d(title, lats=None, lons=None, field=None, *, levels=6, variant="filled", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=1200, height=650, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_contour_map3d_chart()`, `sp.contour_map_3d()`, `sp.contour_map3d_chart()`, `sp.contourmap3d()`.

<h2>Description</h2>

`sp.contour_map3d()` est le jumeau 3D de `sp.contour_map()` : **les 3 variantes de contour_map partagent la même forme 3D**, choisie avec le même mot-clé `variant`. Les échantillons épars `lats`/`lons`/`field` sont interpolés sur une grille régulière (la même interpolation par pondération inverse à la distance que le graphique 2D utilise lui-même) et dessinés comme une colonne par cellule, hauteur et teinte suivant la valeur interpolée à cet endroit — une vraie surface de densité, pas un nuage de points. `extrema` ajoute un repère haut à chaque maximum et minimum local que la grille contient.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `filled` | `bands`, `heat`, `basic`, `default` | `lats, lons, field` | Une surface de grille complète, une colonne par cellule. |
| `isolines` | `lines`, `contour_lines`, `iso` | `lats, lons, field` | La même surface de grille que filled ; le style en lignes tracées du graphique 2D n'a pas de forme 3D séparée. |
| `extrema` | `highs_lows`, `hl`, `pressure_centers` | `lats, lons, field` | La même surface de grille plus un repère haut à chaque maximum et minimum local. |

<h2>Données</h2>

`lats`/`lons` positionne chaque échantillon et `field` donne sa valeur ; les trois sont interpolés sur la même grille régulière quelle que soit la variante.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="contour_map3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>filled</code></h3><p>Une surface de grille complète, une colonne par cellule.</p><p class="sp-3d-uses">Utilise: <code>lats, lons, field</code></p><iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-filled.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.contour_map3d(
    &quot;Filled 3D&quot;,
    variant=&quot;filled&quot;,
    lats=[64.1, 61.2, 68.9, 62.0, 59.9, 55.75, ...],
    lons=[-21.9, -149.9, 33.0, 129.7, 10.75, 37.6, ...],
    field=[4, 2, -3, -15, 6, 5, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>isolines</code></h3><p>La même surface de grille que filled ; le style en lignes tracées du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>lats, lons, field</code></p><iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-isolines.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.contour_map3d(
    &quot;Isolines 3D&quot;,
    variant=&quot;isolines&quot;,
    lats=[65, 55, 35, 55, 32, -28, ...],
    lons=[-20, -165, -25, 90, -140, -105, ...],
    field=[-12, -10, 11, 15, 9, 10, ...],
    bins=7,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>extrema</code></h3><p>La même surface de grille plus un repère haut à chaque maximum et minimum local.</p><p class="sp-3d-uses">Utilise: <code>lats, lons, field</code></p><iframe class="sp-preview-frame" data-src="../../previews/contour_map3d-extrema.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.contour_map3d(
    &quot;Extrema 3D&quot;,
    variant=&quot;extrema&quot;,
    lats=[65, 55, 35, 55, 32, -28, ...],
    lons=[-20, -165, -25, 90, -140, -105, ...],
    field=[-12, -10, 11, 15, 9, 10, ...],
    bins=5,
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="contour_map3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
