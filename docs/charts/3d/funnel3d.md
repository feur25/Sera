# Funnel Chart 3D

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:340px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>

## Signature

`sp.funnel3d(title, labels=None, values=None, *, variant="basic", series=None, series_names=None, category_series=None, sort_order="none", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_funnel3d_chart()`, `sp.funnel_3d()`, `sp.funnel3d_chart()`, `sp.funnel3d_family()`, `sp.funnels3d()`.

## Description

`sp.funnel3d()` is the 3D twin of `sp.funnel()`: **every one of the 9 funnel variants has a 3D form**, selected with the same `variant` keyword and fed with the same stages. The funnel flows along the depth axis, from its widest stage at the back to its narrowest at the front, and every stage is a square section whose *area* is proportional to its value, so a stage worth 4% of the first one stays visible instead of vanishing. Tapered frusta, barrels, chevrons and split stages come from a shared tier strategy, so geometry is never hand-written per variant. Long inputs are pooled to a budget (`max_points`), so hundreds of thousands of stages stay interactive.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `trapezoid`, `classic` | `labels, values` | Smooth tapered frusta: every stage narrows towards the next one along the flow. |
| `stepped` | `bar`, `rect`, `rectangle` | `labels, values` | One straight slab per stage, like a staircase of tunnels. |
| `rounded` | `round`, `pill`, `smooth` | `labels, values` | Barrel-shaped stages that bulge in the middle of each slab. |
| `chevron` | `arrow`, `pipeline`, `pointer` | `labels, values` | Every stage is a wedge that narrows towards the flow, like a pipeline of arrows. |
| `pyramid` | `triangle`, `cone`, `point` | `labels, values` | Sections shrink faster, following a pyramid profile that blends the values with a linear taper. |
| `inverted` | `inverse`, `reverse`, `upside_down` | `labels, values` | The funnel turned around: the widest stage sits at the front and the narrowest at the back. |
| `conversion` | `dropoff`, `rate`, `steps` | `labels, values` | Stages toned by their conversion rate from the previous stage, red for a heavy drop and green for a high rate. |
| `compare` | `multi`, `side_by_side`, `funnels` | `series, series_names, category_series` | Several funnels side by side, each with its own stages (`series` and `category_series`). |
| `grouped` | `colored`, `by_color`, `shared_stages` | `labels, series, series_names` | Stages shared by several series, split across the section and toned by series. |

## Data

`labels` name the stages and `values` are their counts. Several `series` (with `series_names`) make a grouped funnel that shares the stages, and adding `category_series` (one list of stage names per series) draws side-by-side funnels that each have their own stages. Hovering a stage reads its value and its share of the first stage, for example `Signups · 520 (52% of first)`.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="funnel3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Parameters

<div data-sp-registry-table="options" data-family="funnel3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.funnel3d(title, labels=None, values=None, *, variant="basic", series=None, series_names=None, category_series=None, sort_order="none", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_funnel3d_chart()`, `sp.funnel_3d()`, `sp.funnel3d_chart()`, `sp.funnel3d_family()`, `sp.funnels3d()`.

<h2>Description</h2>

`sp.funnel3d()` est le jumeau 3D de `sp.funnel()` : **chacune des 9 variantes d'entonnoir a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes étapes. L'entonnoir s'écoule le long de l'axe de profondeur, de son étape la plus large au fond à la plus étroite à l'avant, et chaque étape est une section carrée dont l'*aire* est proportionnelle à sa valeur : une étape à 4 % de la première reste visible au lieu de disparaître. Troncs effilés, tonneaux, chevrons et étapes partagées viennent d'une stratégie d'étages partagée : la géométrie n'est jamais écrite à la main par variante. Les longues entrées sont regroupées selon un budget (`max_points`) : des centaines de milliers d'étapes restent fluides.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `trapezoid`, `classic` | `labels, values` | Des troncs effilés et lisses : chaque étape se rétrécit vers la suivante le long du flux. |
| `stepped` | `bar`, `rect`, `rectangle` | `labels, values` | Une dalle droite par étape, comme un escalier de tunnels. |
| `rounded` | `round`, `pill`, `smooth` | `labels, values` | Des étapes en forme de tonneau qui bombent au milieu de chaque dalle. |
| `chevron` | `arrow`, `pipeline`, `pointer` | `labels, values` | Chaque étape est un coin qui se rétrécit dans le sens du flux, comme un pipeline de flèches. |
| `pyramid` | `triangle`, `cone`, `point` | `labels, values` | Les sections rétrécissent plus vite, suivant un profil de pyramide qui mêle les valeurs à un effilement linéaire. |
| `inverted` | `inverse`, `reverse`, `upside_down` | `labels, values` | L'entonnoir retourné : l'étape la plus large est à l'avant et la plus étroite au fond. |
| `conversion` | `dropoff`, `rate`, `steps` | `labels, values` | Étapes teintées par leur taux de conversion depuis l'étape précédente, rouge pour une forte chute et vert pour un taux élevé. |
| `compare` | `multi`, `side_by_side`, `funnels` | `series, series_names, category_series` | Plusieurs entonnoirs côte à côte, chacun avec ses propres étapes (`series` et `category_series`). |
| `grouped` | `colored`, `by_color`, `shared_stages` | `labels, series, series_names` | Étapes partagées par plusieurs séries, réparties sur la section et teintées par série. |

<h2>Données</h2>

`labels` nomme les étapes et `values` sont leurs effectifs. Plusieurs `series` (avec `series_names`) font un entonnoir groupé qui partage les étapes, et ajouter `category_series` (une liste de noms d'étapes par série) dessine des entonnoirs côte à côte ayant chacun leurs étapes. Survoler une étape affiche sa valeur et sa part de la première étape, par exemple `Signups · 520 (52% of first)`.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="funnel3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/funnel3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="funnel3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
