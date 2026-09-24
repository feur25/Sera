# Choropleth Map 3D

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:340px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>

## Signature

`sp.choropleth3d(title, labels=None, values=None, secondary_values=None, *, map="world", region="", bins=5, variant="sequential", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=1200, height=600, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_choropleth3d_chart()`, `sp.choropleth_3d()`, `sp.choropleth3d_chart()`, `sp.geo_map3d()`.

## Description

`sp.choropleth3d()` is the 3D twin of `sp.choropleth()`: **all 8 choropleth variants share the same 3D form**, selected with the same `variant` keyword and fed with the same values. A literal country outline has no honest cuboid interior, so every matched region instead gets a column at its own geographic centroid, height following its value, ringed by a thin trace of its real coastline (for a region set small enough to trace legibly). `daynight` is the one exception: it carries no per-region data at all, so it draws the day/night terminator as a curved trace plus a sun marker at the subsolar point instead.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `sequential` | `basic`, `default`, `heat` | `labels, values` | One column per matched region at its centroid, height and tone following its value. |
| `binned` | `quantile`, `classed`, `steps` | `labels, values` | The same columns, height and tone stepped into discrete bins instead of a continuous ramp. |
| `diverging` | `delta`, `change`, `rdbu` | `labels, values` | Columns rise above the floor for a positive value and sink below it for a negative one. |
| `orthographic` | `globe`, `sphere`, `space` | `labels, values` | The same columns as sequential; the 2D globe-look projection has no separate 3D form. |
| `polar` | `azimuthal`, `pole` | `labels, values` | The same columns as sequential; the 2D polar projection has no separate 3D form. |
| `bivariate` | `two_variable`, `cross`, `dual` | `labels, values, secondary_values` | The same columns, toned by `secondary_values` instead of `values`. |
| `dot_density` | `dots`, `stipple`, `scatter_fill` | `labels, values` | The same columns as sequential; the 2D chart's random dot-fill has no separate 3D form. |
| `daynight` | `day_night`, `terminator`, `solar` | `` | No per-region columns at all: a curved terminator trace plus a sun marker at the subsolar point. |

## Data

`labels` names each region (ISO codes or full names, matched against the `map` region set), `values` sets its column height and `secondary_values` feeds `bivariate`'s second channel.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="choropleth3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Parameters

<div data-sp-registry-table="options" data-family="choropleth3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.choropleth3d(title, labels=None, values=None, secondary_values=None, *, map="world", region="", bins=5, variant="sequential", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=1200, height=600, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_choropleth3d_chart()`, `sp.choropleth_3d()`, `sp.choropleth3d_chart()`, `sp.geo_map3d()`.

<h2>Description</h2>

`sp.choropleth3d()` est le jumeau 3D de `sp.choropleth()` : **les 8 variantes de choropleth partagent la même forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes valeurs. Un contour de pays littéral n'a pas d'intérieur cuboïde honnête, donc chaque région reconnue reçoit plutôt une colonne à son propre centroïde géographique, la hauteur suivant sa valeur, cerclée d'un fin tracé de son vrai littoral (pour un jeu de régions assez petit pour rester lisible). `daynight` est la seule exception : elle ne porte aucune donnée par région, donc elle trace plutôt le terminateur jour/nuit en courbe plus un repère solaire au point sous-solaire.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `sequential` | `basic`, `default`, `heat` | `labels, values` | Une colonne par région reconnue à son centroïde, hauteur et teinte suivant sa valeur. |
| `binned` | `quantile`, `classed`, `steps` | `labels, values` | Les mêmes colonnes, hauteur et teinte réparties en paliers discrets plutôt qu'une rampe continue. |
| `diverging` | `delta`, `change`, `rdbu` | `labels, values` | Les colonnes s'élèvent au-dessus du sol pour une valeur positive et s'enfoncent en dessous pour une négative. |
| `orthographic` | `globe`, `sphere`, `space` | `labels, values` | Les mêmes colonnes que sequential ; la projection 2D façon globe n'a pas de forme 3D séparée. |
| `polar` | `azimuthal`, `pole` | `labels, values` | Les mêmes colonnes que sequential ; la projection polaire 2D n'a pas de forme 3D séparée. |
| `bivariate` | `two_variable`, `cross`, `dual` | `labels, values, secondary_values` | Les mêmes colonnes, teintées selon `secondary_values` plutôt que `values`. |
| `dot_density` | `dots`, `stipple`, `scatter_fill` | `labels, values` | Les mêmes colonnes que sequential ; le remplissage aléatoire en points du 2D n'a pas de forme 3D séparée. |
| `daynight` | `day_night`, `terminator`, `solar` | `` | Aucune colonne par région : un tracé de terminateur courbe plus un repère solaire au point sous-solaire. |

<h2>Données</h2>

`labels` nomme chaque région (codes ISO ou noms complets, reconnus dans le jeu de régions `map`), `values` fixe la hauteur de sa colonne et `secondary_values` alimente le second canal de `bivariate`.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="choropleth3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="choropleth3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
