# Bubble Map 3D

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

`sp.bubble_map3d(title, labels=None, values=None, lats=None, lons=None, series=None, *, map="world", region="", variant="proportional", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=1200, height=600, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_bubble_map3d_chart()`, `sp.bubble_map_3d()`, `sp.bubble_map3d_chart()`, `sp.bubblemap3d()`.

## Description

`sp.bubble_map3d()` is the 3D twin of `sp.bubble_map()`: **all 7 bubble-map variants share the same 3D form**, selected with the same `variant` keyword. Pass `lats`/`lons` and every point gets its own marker at that exact position; leave them out and `labels` are matched against the `map` region set instead, one marker per region centroid. Either way, marker size and column height follow the value (for `pie_markers`, the sum of its `series` row).

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `proportional` | `bubble`, `graduated`, `sized`, `basic`, `default` | `labels, values` | One marker per point or region, size and height following its value. |
| `filled` | `regions` | `labels, values` | The same markers as proportional; the 2D chart's region-fill styling has no separate 3D form. |
| `globe` | `orthographic`, `sphere`, `space` | `labels, values` | The same markers as proportional; the 2D globe-look projection has no separate 3D form. |
| `ring` | `donut`, `outline`, `hollow` | `labels, values` | The same markers, visibly thinner. |
| `pulse` | `ripple`, `radar`, `ping` | `labels, values` | The same markers as proportional, from the 2D chart's animated ripple styling. |
| `hexbin` | `hex`, `density`, `honeycomb` | `lats, lons` | One marker per raw sampled point, from `lats`/`lons` directly. |
| `pie_markers` | `pie_map`, `mini_pie`, `category_pie` | `lats, lons, series, categories` | One marker per point, sized by the sum of its `series` row. |

## Data

Either `lats`/`lons` (raw points) or `labels` (region codes); `values` sizes each marker, and `series` (summed per row) sizes it instead for `pie_markers`.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="bubble_map3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>proportional</code></h3><p>One marker per point or region, size and height following its value.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-proportional.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Proportional 3D&quot;,
    variant=&quot;proportional&quot;,
    labels=[&quot;AL&quot;, &quot;AK&quot;, &quot;AZ&quot;, &quot;AR&quot;, &quot;CA&quot;, &quot;CO&quot;, ...],
    values=[5.1, 0.73, 7.4, 3.0, 38.9, 5.9, ...],
    map=&quot;usa_states&quot;,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>filled</code></h3><p>The same markers as proportional; the 2D chart&#x27;s region-fill styling has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-filled.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Filled 3D&quot;,
    variant=&quot;filled&quot;,
    labels=[&quot;AL&quot;, &quot;AK&quot;, &quot;AZ&quot;, &quot;AR&quot;, &quot;CA&quot;, &quot;CO&quot;, ...],
    values=[5.1, 0.73, 7.4, 3.0, 38.9, 5.9, ...],
    map=&quot;usa_states&quot;,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>globe</code></h3><p>The same markers as proportional; the 2D globe-look projection has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-globe.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Globe 3D&quot;,
    variant=&quot;globe&quot;,
    labels=[&quot;US&quot;, &quot;CN&quot;, &quot;IN&quot;, &quot;ID&quot;, &quot;PK&quot;, &quot;BR&quot;, ...],
    values=[331.9, 1412.0, 1380.0, 273.5, 220.9, 213.3, ...],
    center_lat=15,
    center_lon=10,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>ring</code></h3><p>The same markers, visibly thinner.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-ring.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Ring 3D&quot;,
    variant=&quot;ring&quot;,
    labels=[&quot;CA&quot;, &quot;TX&quot;, &quot;NY&quot;, &quot;FL&quot;, &quot;IL&quot;, &quot;PA&quot;, ...],
    values=[38.9, 30.5, 19.6, 22.6, 12.6, 12.9, ...],
    map=&quot;usa_states&quot;,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>pulse</code></h3><p>The same markers as proportional, from the 2D chart&#x27;s animated ripple styling.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-pulse.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Pulse 3D&quot;,
    variant=&quot;pulse&quot;,
    labels=[&quot;JP&quot;, &quot;ID&quot;, &quot;CL&quot;, &quot;TR&quot;, &quot;US&quot;, &quot;NZ&quot;, ...],
    values=[9.1, 8.7, 8.9, 7.4, 7.2, 7.8, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>hexbin</code></h3><p>One marker per raw sampled point, from <code>lats</code>/<code>lons</code> directly.</p><p class="sp-3d-uses">Uses: <code>lats, lons</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-hexbin.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Hexbin 3D&quot;,
    variant=&quot;hexbin&quot;,
    lats=[40.7, 40.75, 40.68, 34.0, 34.05, 33.9, ...],
    lons=[-74.0, -73.95, -73.9, -118.2, -118.25, -118.3, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>pie_markers</code></h3><p>One marker per point, sized by the sum of its <code>series</code> row.</p><p class="sp-3d-uses">Uses: <code>lats, lons, series, categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-pie_markers.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Pie Markers 3D&quot;,
    variant=&quot;pie_markers&quot;,
    lats=[40.7, 51.5, 35.7, -23.5, -33.9, 30.0],
    lons=[-74.0, -0.12, 139.7, -46.6, 18.4, 31.2],
    series=[[40, 25, 35], [55, 15, 30], [30, 45, 25], [60, 10, 30], [35, 35, 30], [20, 55, 25]],
    categories=[&quot;Wind&quot;, &quot;Solar&quot;, &quot;Hydro&quot;],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="bubble_map3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.bubble_map3d(title, labels=None, values=None, lats=None, lons=None, series=None, *, map="world", region="", variant="proportional", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=1200, height=600, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_bubble_map3d_chart()`, `sp.bubble_map_3d()`, `sp.bubble_map3d_chart()`, `sp.bubblemap3d()`.

<h2>Description</h2>

`sp.bubble_map3d()` est le jumeau 3D de `sp.bubble_map()` : **les 7 variantes de bubble_map partagent la même forme 3D**, choisie avec le même mot-clé `variant`. Passez `lats`/`lons` et chaque point reçoit son propre repère à cette position exacte ; omettez-les et `labels` est plutôt reconnu dans le jeu de régions `map`, un repère par centroïde de région. Dans les deux cas, la taille du repère et la hauteur de colonne suivent la valeur (pour `pie_markers`, la somme de sa ligne `series`).

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `proportional` | `bubble`, `graduated`, `sized`, `basic`, `default` | `labels, values` | Un repère par point ou région, taille et hauteur suivant sa valeur. |
| `filled` | `regions` | `labels, values` | Les mêmes repères que proportional ; le style à remplissage de région du 2D n'a pas de forme 3D séparée. |
| `globe` | `orthographic`, `sphere`, `space` | `labels, values` | Les mêmes repères que proportional ; la projection 2D façon globe n'a pas de forme 3D séparée. |
| `ring` | `donut`, `outline`, `hollow` | `labels, values` | Les mêmes repères, visiblement plus fins. |
| `pulse` | `ripple`, `radar`, `ping` | `labels, values` | Les mêmes repères que proportional, depuis le style en onde animée du graphique 2D. |
| `hexbin` | `hex`, `density`, `honeycomb` | `lats, lons` | Un repère par point brut échantillonné, depuis `lats`/`lons` directement. |
| `pie_markers` | `pie_map`, `mini_pie`, `category_pie` | `lats, lons, series, categories` | Un repère par point, dimensionné par la somme de sa ligne `series`. |

<h2>Données</h2>

Soit `lats`/`lons` (points bruts), soit `labels` (codes de région) ; `values` dimensionne chaque repère, et `series` (sommée par ligne) le dimensionne à la place pour `pie_markers`.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="bubble_map3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>proportional</code></h3><p>Un repère par point ou région, taille et hauteur suivant sa valeur.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-proportional.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Proportional 3D&quot;,
    variant=&quot;proportional&quot;,
    labels=[&quot;AL&quot;, &quot;AK&quot;, &quot;AZ&quot;, &quot;AR&quot;, &quot;CA&quot;, &quot;CO&quot;, ...],
    values=[5.1, 0.73, 7.4, 3.0, 38.9, 5.9, ...],
    map=&quot;usa_states&quot;,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>filled</code></h3><p>Les mêmes repères que proportional ; le style à remplissage de région du 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-filled.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Filled 3D&quot;,
    variant=&quot;filled&quot;,
    labels=[&quot;AL&quot;, &quot;AK&quot;, &quot;AZ&quot;, &quot;AR&quot;, &quot;CA&quot;, &quot;CO&quot;, ...],
    values=[5.1, 0.73, 7.4, 3.0, 38.9, 5.9, ...],
    map=&quot;usa_states&quot;,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>globe</code></h3><p>Les mêmes repères que proportional ; la projection 2D façon globe n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-globe.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Globe 3D&quot;,
    variant=&quot;globe&quot;,
    labels=[&quot;US&quot;, &quot;CN&quot;, &quot;IN&quot;, &quot;ID&quot;, &quot;PK&quot;, &quot;BR&quot;, ...],
    values=[331.9, 1412.0, 1380.0, 273.5, 220.9, 213.3, ...],
    center_lat=15,
    center_lon=10,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>ring</code></h3><p>Les mêmes repères, visiblement plus fins.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-ring.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Ring 3D&quot;,
    variant=&quot;ring&quot;,
    labels=[&quot;CA&quot;, &quot;TX&quot;, &quot;NY&quot;, &quot;FL&quot;, &quot;IL&quot;, &quot;PA&quot;, ...],
    values=[38.9, 30.5, 19.6, 22.6, 12.6, 12.9, ...],
    map=&quot;usa_states&quot;,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>pulse</code></h3><p>Les mêmes repères que proportional, depuis le style en onde animée du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-pulse.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Pulse 3D&quot;,
    variant=&quot;pulse&quot;,
    labels=[&quot;JP&quot;, &quot;ID&quot;, &quot;CL&quot;, &quot;TR&quot;, &quot;US&quot;, &quot;NZ&quot;, ...],
    values=[9.1, 8.7, 8.9, 7.4, 7.2, 7.8, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>hexbin</code></h3><p>Un repère par point brut échantillonné, depuis <code>lats</code>/<code>lons</code> directement.</p><p class="sp-3d-uses">Utilise: <code>lats, lons</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-hexbin.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Hexbin 3D&quot;,
    variant=&quot;hexbin&quot;,
    lats=[40.7, 40.75, 40.68, 34.0, 34.05, 33.9, ...],
    lons=[-74.0, -73.95, -73.9, -118.2, -118.25, -118.3, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>pie_markers</code></h3><p>Un repère par point, dimensionné par la somme de sa ligne <code>series</code>.</p><p class="sp-3d-uses">Utilise: <code>lats, lons, series, categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/bubble_map3d-pie_markers.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bubble_map3d(
    &quot;Pie Markers 3D&quot;,
    variant=&quot;pie_markers&quot;,
    lats=[40.7, 51.5, 35.7, -23.5, -33.9, 30.0],
    lons=[-74.0, -0.12, 139.7, -46.6, 18.4, 31.2],
    series=[[40, 25, 35], [55, 15, 30], [30, 45, 25], [60, 10, 30], [35, 35, 30], [20, 55, 25]],
    categories=[&quot;Wind&quot;, &quot;Solar&quot;, &quot;Hydro&quot;],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="bubble_map3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
