# Line Chart 3D

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:340px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>

## Signature

`sp.line3d(title, x_labels=None, values=None, *, variant="basic", series=None, series_names=None, step_shape="hv", spline_tension=0.5, dash_pattern="auto", gap_threshold=None, show_points=False, pace_target=None, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_line3d_chart()`, `sp.line_3d()`, `sp.line3d_chart()`, `sp.line3d_family()`, `sp.lines3d()`.

## Description

`sp.line3d()` is the 3D twin of `sp.line()`: **every one of the 13 line variants has a 3D form**, selected with the same `variant` keyword and fed with the same data. Lines are slanted ribbons between successive points, areas are sloped-top strips, steps are flat treads and risers, and several series stand on parallel depth rows, all from shared curve strategies so geometry is never hand-written per variant. Long series are decimated with the same LTTB rule as the 2D chart (`max_points`), so millions of points stay interactive.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | — | `x_labels, values` | One slanted ribbon following the values from point to point. |
| `multi` | `multiline`, `multiple` | `x_labels, series, series_names` | One ribbon per series, each on its own depth row so they never overlap. |
| `stepped` | `step`, `hv`, `vh`, `hvh`, `vhv` | `x_labels, values, step_shape` | Flat treads and vertical risers; `step_shape` picks hv, vh or hvh. |
| `spline` | `smooth`, `curved` | `x_labels, values, spline_tension` | A Catmull-Rom curve through the points, densified into many short ribbons. |
| `filled` | `area`, `fill` | `x_labels, values` | Sloped-top strips filled down to the baseline under every series. |
| `sparkline` | `spark`, `tiny` | `x_labels, values` | Thin compact ribbons, one per series, ready to stack as small multiples. |
| `dashed` | `dotted`, `styled` | `x_labels, values, dash_pattern` | The ribbon cut into dashes, with a different pattern per series. |
| `connected_scatter` | `markers`, `lines+markers` | `x_labels, series, series_names` | Ribbons plus a small cube on every data point. |
| `gapped` | `gaps`, `missing` | `x_labels, values, gap_threshold` | The ribbon breaks wherever a value is missing or jumps by more than `gap_threshold`. |
| `band` | `confidence_band`, `forecast`, `range_band` | `x_labels, series, series_names` | A ribbon-shaped band between the two series (low and high) with the mid line inside it. |
| `momentum` | `slope_glow`, `trend_pulse` | `x_labels, values` | The ribbon is toned by slope, rising green and falling red, with cubes on the strongest peaks. |
| `epoch` | `chapters`, `regime_bands` | `x_labels, values` | The ribbon is toned by regime (rise, fall, flat) over a floor plate marking each chapter. |
| `pace` | `pacing`, `glidepath`, `runway` | `x_labels, values, pace_target` | The actual ribbon toned ahead or behind, beside a dashed line to `pace_target`. |

## Data

`x_labels` (or `labels`, or numeric `x`) name the points and `values` (or `y`) is one series; pass `series` (with `series_names`) for several. Passing `z` together with `x` and `y` keeps the historical call that traces one spatial polyline through (x, y, z).

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="line_3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Parameters

<div data-sp-registry-table="options" data-family="line_3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.line3d(title, x_labels=None, values=None, *, variant="basic", series=None, series_names=None, step_shape="hv", spline_tension=0.5, dash_pattern="auto", gap_threshold=None, show_points=False, pace_target=None, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_line3d_chart()`, `sp.line_3d()`, `sp.line3d_chart()`, `sp.line3d_family()`, `sp.lines3d()`.

<h2>Description</h2>

`sp.line3d()` est le jumeau 3D de `sp.line()` : **chacune des 13 variantes de courbe a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes données. Les lignes sont des rubans inclinés entre points successifs, les aires des bandes à dessus incliné, les marches des paliers et contremarches plats, et plusieurs séries se posent sur des rangées de profondeur parallèles, le tout par des stratégies de courbes partagées : la géométrie n'est jamais écrite à la main par variante. Les longues séries sont décimées par la même règle LTTB que le graphique 2D (`max_points`) : des millions de points restent fluides.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | — | `x_labels, values` | Un ruban incliné qui suit les valeurs d'un point au suivant. |
| `multi` | `multiline`, `multiple` | `x_labels, series, series_names` | Un ruban par série, chacun sur sa propre rangée de profondeur pour qu'ils ne se recouvrent jamais. |
| `stepped` | `step`, `hv`, `vh`, `hvh`, `vhv` | `x_labels, values, step_shape` | Paliers plats et contremarches verticales ; `step_shape` choisit hv, vh ou hvh. |
| `spline` | `smooth`, `curved` | `x_labels, values, spline_tension` | Une courbe de Catmull-Rom à travers les points, densifiée en nombreux rubans courts. |
| `filled` | `area`, `fill` | `x_labels, values` | Bandes à dessus incliné remplies jusqu'à la ligne de base sous chaque série. |
| `sparkline` | `spark`, `tiny` | `x_labels, values` | Rubans fins et compacts, un par série, prêts à s'empiler en petits multiples. |
| `dashed` | `dotted`, `styled` | `x_labels, values, dash_pattern` | Le ruban découpé en tirets, avec un motif différent par série. |
| `connected_scatter` | `markers`, `lines+markers` | `x_labels, series, series_names` | Des rubans plus un petit cube sur chaque point de données. |
| `gapped` | `gaps`, `missing` | `x_labels, values, gap_threshold` | Le ruban se coupe là où une valeur manque ou saute de plus de `gap_threshold`. |
| `band` | `confidence_band`, `forecast`, `range_band` | `x_labels, series, series_names` | Une bande en forme de ruban entre les deux séries (basse et haute) avec la ligne médiane à l'intérieur. |
| `momentum` | `slope_glow`, `trend_pulse` | `x_labels, values` | Le ruban est teinté selon la pente, vert en montée et rouge en descente, avec des cubes sur les plus forts sommets. |
| `epoch` | `chapters`, `regime_bands` | `x_labels, values` | Le ruban est teinté par régime (hausse, baisse, plat) au-dessus d'une plaque au sol marquant chaque chapitre. |
| `pace` | `pacing`, `glidepath`, `runway` | `x_labels, values, pace_target` | Le ruban réel teinté en avance ou en retard, à côté d'une ligne pointillée vers `pace_target`. |

<h2>Données</h2>

`x_labels` (ou `labels`, ou `x` numérique) nomme les points et `values` (ou `y`) est une série ; passez `series` (avec `series_names`) pour plusieurs. Passer `z` avec `x` et `y` conserve l'appel historique qui trace une polyligne spatiale à travers (x, y, z).

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="line_3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/line3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="line_3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
