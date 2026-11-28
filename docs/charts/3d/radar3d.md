# Radar Chart 3D

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:340px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>

## Signature

`sp.radar3d(title, axes=None, series=None, series_names=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_radar3d_chart()`, `sp.radar_3d()`, `sp.radar3d_chart()`, `sp.radar3d_family()`.

## Description

`sp.radar3d()` is the 3D twin of `sp.radar()`: selected with the same `variant` keyword and fed with the same series. `basic`/`lines`/`filled`/`markers`/`dashed` each become a closed loop around the axes, one per series at its own depth row, radius following its values, carrying the same fill/stroke/marker treatment as their 2D sibling — a real spider shape, not a flat outline. `polar_bar`/`petal`/`band`/`stacked` keep their existing ring-of-columns forms, one concentric ring per series.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic` | `axes, series, series_names` | One closed loop per series, filled and outlined, with small vertex markers, radius following its values around the axes. |
| `lines` | `outline`, `stroke`, `no_fill` | `axes, series, series_names` | The same loop with no fill, just its own stroke and small vertex markers — matching the 2D chart's outline-only look. |
| `filled` | `fill`, `solid`, `area` | `axes, series, series_names` | The same loop with a heavier fill and no stroke or vertex markers — matching the 2D chart's solid filled-area look. |
| `markers` | `dots`, `points`, `marker` | `axes, series, series_names` | The same loop with a thinner, faded stroke and large outlined vertex markers — matching the 2D chart's per-vertex marker look. |
| `dashed` | `dash`, `dotted` | `axes, series, series_names` | The same loop with a dashed stroke and small vertex markers — matching the 2D chart's dashed-outline look. |
| `stacked` | `stack`, `cumulative` | `axes, series, series_names` | A closed loop per series at a cumulative radius shared across all series — each loop's radius is its own values plus every series before it, so later series nest around earlier ones, matching the 2D chart's stacked-area look. |
| `polar_bar` | `polar`, `bar`, `radial_bar` | `axes, series, series_names` | A real pie-wedge per series within each axis's sector, evenly subdivided side by side, radius following its values — matching the 2D chart's radial bars. |
| `band` | `range`, `uncertainty`, `minmax` | `axes, series, series_names` | A single ribbon-shaped loop per group, tracing the high values forward around the axes and the low values back — matching the 2D chart's filled range band. |
| `petal` | `flower`, `bloom`, `petal_diagram`, `nightingale_petal` | `axes, series, series_names` | A wide wedge per series within each axis's sector, nested rather than subdivided and narrowing slightly for later series — matching the 2D chart's layered petal look. |

## Data

`axes` names each spoke, `series` gives one row of values per series (one entry per axis) and `series_names` names each series.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="radar3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Parameters

<div data-sp-registry-table="options" data-family="radar3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.radar3d(title, axes=None, series=None, series_names=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_radar3d_chart()`, `sp.radar_3d()`, `sp.radar3d_chart()`, `sp.radar3d_family()`.

<h2>Description</h2>

`sp.radar3d()` est le jumeau 3D de `sp.radar()` : choisie avec le même mot-clé `variant` et alimentée par les mêmes séries. `basic`/`lines`/`filled`/`markers`/`dashed` deviennent chacune une boucle fermée autour des axes, une par série à sa propre rangée de profondeur, le rayon suivant ses valeurs, avec le même traitement de remplissage/trait/marqueurs que sa version 2D — une vraie forme d'araignée, pas un simple contour à plat. `polar_bar`/`petal`/`band`/`stacked` gardent leurs formes existantes en anneaux de colonnes, un anneau concentrique par série.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic` | `axes, series, series_names` | Une boucle fermée par série, remplie et contourée, avec de petits marqueurs de sommet, le rayon suivant ses valeurs autour des axes. |
| `lines` | `outline`, `stroke`, `no_fill` | `axes, series, series_names` | La même boucle sans remplissage, juste son propre trait et de petits marqueurs de sommet — comme le style en trait seul du graphique 2D. |
| `filled` | `fill`, `solid`, `area` | `axes, series, series_names` | La même boucle avec un remplissage plus marqué et sans trait ni marqueurs de sommet — comme le style à aire remplie du graphique 2D. |
| `markers` | `dots`, `points`, `marker` | `axes, series, series_names` | La même boucle avec un trait plus fin et atténué et de grands marqueurs de sommet contourés — comme le style à points par sommet du graphique 2D. |
| `dashed` | `dash`, `dotted` | `axes, series, series_names` | La même boucle avec un trait pointillé et de petits marqueurs de sommet — comme le style en trait pointillé du graphique 2D. |
| `stacked` | `stack`, `cumulative` | `axes, series, series_names` | Une boucle fermée par série à un rayon cumulatif partagé entre toutes les séries — chaque boucle a pour rayon ses propres valeurs plus celles de chaque série précédente, donc les séries suivantes s'emboîtent autour des précédentes, comme le style en aire empilée du graphique 2D. |
| `polar_bar` | `polar`, `bar`, `radial_bar` | `axes, series, series_names` | Un vrai secteur en pointe de camembert par série dans le secteur angulaire de chaque axe, subdivisé également côte à côte, le rayon suivant ses valeurs — comme les barres radiales du graphique 2D. |
| `band` | `range`, `uncertainty`, `minmax` | `axes, series, series_names` | Une seule boucle en forme de ruban par groupe, parcourant les valeurs hautes vers l'avant autour des axes puis les valeurs basses vers l'arrière — comme la bande de plage remplie du graphique 2D. |
| `petal` | `flower`, `bloom`, `petal_diagram`, `nightingale_petal` | `axes, series, series_names` | Un large secteur par série dans le secteur angulaire de chaque axe, emboîté plutôt que subdivisé et légèrement plus étroit pour les séries suivantes — comme le style de pétales superposés du graphique 2D. |

<h2>Données</h2>

`axes` nomme chaque rayon, `series` donne une ligne de valeurs par série (une entrée par axe) et `series_names` nomme chaque série.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="radar3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="radar3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
