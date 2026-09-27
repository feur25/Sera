# Bar Chart 3D

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:340px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>

## Signature

`sp.bar3d(title, labels=None, values=None, *, variant="basic", series=None, series_names=None, orientation3d="iso", color_hex=0x6366F1, palette=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_bar3d_chart()`, `sp.bar_3d()`, `sp.bar3d_chart()`, `sp.bar3d_family()`, `sp.bars3d()`.

## Description

`sp.bar3d()` renders the SeraPlot bar family as extruded prisms on an interactive 3D canvas. It is the 3D twin of [`sp.bar()`](../2d/bar.md): **every one of the 17 bar variants has a 3D form**, selected with the same `variant=` keyword and fed with the same data fields. Layouts come from shared, family-agnostic strategies (linear, grouped, stacked, radial, spiral, box-and-whisker), so geometry is never hand-written per variant.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | — | `labels, values` | One extruded column per category along the X axis; height encodes the value. |
| `horizontal` | `h`, `hbar` | `labels, values` | Bars lie flat along the X axis, one row per category; length encodes the value. |
| `grouped` | `group` | `labels, series, series_names` | Series side by side in depth: one row of columns per series, categories along X. |
| `stacked` | `stack` | `labels, series, series_names` | Series stacked into a single column per category; each segment keeps its series colour. |
| `relative` | `rel` | `labels, series, series_names` | Stacked columns normalised to 100 %: every column reaches the same height, segments show the share. |
| `grouped_stacked` | `groupstack`, `grouped-stacked` | `labels, series, offset_groups` | Stack groups placed side by side in depth, each stack built from its own series segments. |
| `marimekko` | `mekko`, `mosaic` | `labels, series, widths` | Columns whose footprint width follows `widths`, stacked segments encode each series share. |
| `pictogram` | `icon` | `labels, values, units_per_icon` | Every icon becomes a small block; blocks pile up in columns of `max_icons_per_column`. |
| `multicategory` | `multi`, `hierarchical` | `labels, values, super_categories` | Columns grouped under their `super_categories`, with an extra gap between two groups. |
| `circular` | `circular_basic`, `radial_bar`, `polar_bar` | `labels, values` | Columns stand on a ring, one per category; height encodes the value. |
| `circular_grouped` | `radial_grouped`, `circular_groups` | `labels, values, color_groups` | The same ring split into angular arcs by `color_groups`, one colour per group. |
| `population_pyramid` | `pyramid`, `age_pyramid` | `labels, series, series_names` | Two back-to-back rows of columns, one per series (male / female), age groups along X. |
| `diverging` | `signed`, `delta`, `bidirectional` | `labels, values` | Columns rise above or sink below the floor with the sign of the value; with `series`, positive and negative segments stack on each side. |
| `distribution` | `bar_box`, `boxbar`, `bar_boxplot` | `labels, series` | For each sample a slim column spans the whiskers and a wider box spans the interquartile range. |
| `spiral` | `spiral_bar`, `nautilus`, `radial_spiral`, `growth_spiral` | `labels, values` | Columns follow an outward spiral, one per index — made for long time series. |
| `hedgehog` | `flow_fan`, `quill`, `spike_flow`, `relocation_fan` | `labels, values` | Signed spikes around a ring: positive values rise, negative values sink. |
| `radial_flow` | `capital_flow`, `hierarchical_radial`, `sankey_radial`, `flow_arc` | `labels, values, super_categories` | A ring split into arcs by `super_categories`, columns grouped under their parent. |

## 3D planes

The viewpoint is independent from the variant: `orientation3d` (aliases `tilt3d`, `rotate3d`) picks the initial camera plane and applies to **every** variant. The plane can also be switched afterwards with `.orient3d(mode)`, and the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> — Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> — Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> — Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> — Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-front.html"></iframe></div>
</div>

```python
chart = sp.bar3d("Sales", variant="grouped", labels=labels, series=series, series_names=names, orientation3d="vertical")
chart = chart.orient3d("front")
```

## 3D scenes

The scene is a second independent axis: `scene` swaps the environment the bars are drawn in and applies to **every** variant too. Every scene works with every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-podium.html"></iframe></div>
</div>

```python
chart = sp.bar3d("Sales", variant="spiral", labels=years, values=counts, scene="terrain", orientation3d="vertical")
```

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane. Sorting follows the 2D chart too: `sort_order="desc"` reorders the columns of every single-series variant.

<div data-sp-registry-table="themes" data-family="bar_3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Parameters

<div data-sp-registry-table="options" data-family="bar_3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.bar3d(title, labels=None, values=None, *, variant="basic", series=None, series_names=None, orientation3d="iso", color_hex=0x6366F1, palette=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_bar3d_chart()`, `sp.bar_3d()`, `sp.bar3d_chart()`, `sp.bar3d_family()`, `sp.bars3d()`.

<h2>Description</h2>

`sp.bar3d()` rend la famille de barres de SeraPlot en prismes extrudés sur un canvas 3D interactif. C'est le jumeau 3D de [`sp.bar()`](../2d/bar.md) : **chacune des 17 variantes de bar possède une forme 3D**, choisie avec le même mot-clé `variant=` et alimentée par les mêmes champs de données. Les géométries viennent de stratégies partagées et indépendantes de la famille (linéaire, groupé, empilé, radial, spirale, boîte à moustaches) : rien n'est écrit à la main par variante.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | — | `labels, values` | Une colonne extrudée par catégorie le long de l'axe X ; la hauteur encode la valeur. |
| `horizontal` | `h`, `hbar` | `labels, values` | Les barres sont couchées le long de l'axe X, une rangée par catégorie ; la longueur encode la valeur. |
| `grouped` | `group` | `labels, series, series_names` | Séries côte à côte en profondeur : une rangée de colonnes par série, catégories le long de X. |
| `stacked` | `stack` | `labels, series, series_names` | Séries empilées en une seule colonne par catégorie ; chaque segment garde la couleur de sa série. |
| `relative` | `rel` | `labels, series, series_names` | Colonnes normalisées à 100 % : toutes atteignent la même hauteur, les segments montrent la part. |
| `grouped_stacked` | `groupstack`, `grouped-stacked` | `labels, series, offset_groups` | Groupes d'empilements placés côte à côte en profondeur, chaque pile construite depuis ses propres séries. |
| `marimekko` | `mekko`, `mosaic` | `labels, series, widths` | Colonnes dont l'emprise suit `widths`, segments empilés encodant la part de chaque série. |
| `pictogram` | `icon` | `labels, values, units_per_icon` | Chaque icône devient un petit bloc ; les blocs s'empilent par colonnes de `max_icons_per_column`. |
| `multicategory` | `multi`, `hierarchical` | `labels, values, super_categories` | Colonnes regroupées sous leurs `super_categories`, avec un écart supplémentaire entre deux groupes. |
| `circular` | `circular_basic`, `radial_bar`, `polar_bar` | `labels, values` | Les colonnes se dressent sur un anneau, une par catégorie ; la hauteur encode la valeur. |
| `circular_grouped` | `radial_grouped`, `circular_groups` | `labels, values, color_groups` | Le même anneau découpé en arcs par `color_groups`, une couleur par groupe. |
| `population_pyramid` | `pyramid`, `age_pyramid` | `labels, series, series_names` | Deux rangées de colonnes dos à dos, une par série (hommes / femmes), tranches d'âge le long de X. |
| `diverging` | `signed`, `delta`, `bidirectional` | `labels, values` | Les colonnes montent ou s'enfoncent selon le signe de la valeur ; avec `series`, les segments positifs et négatifs s'empilent de chaque côté. |
| `distribution` | `bar_box`, `boxbar`, `bar_boxplot` | `labels, series` | Pour chaque échantillon, une colonne fine couvre les moustaches et une boîte plus large couvre l'intervalle interquartile. |
| `spiral` | `spiral_bar`, `nautilus`, `radial_spiral`, `growth_spiral` | `labels, values` | Les colonnes suivent une spirale vers l'extérieur, une par indice — pensé pour les longues séries temporelles. |
| `hedgehog` | `flow_fan`, `quill`, `spike_flow`, `relocation_fan` | `labels, values` | Pics signés autour d'un anneau : les valeurs positives montent, les négatives s'enfoncent. |
| `radial_flow` | `capital_flow`, `hierarchical_radial`, `sankey_radial`, `flow_arc` | `labels, values, super_categories` | Un anneau découpé en arcs par `super_categories`, colonnes regroupées sous leur parent. |

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` (alias `tilt3d`, `rotate3d`) choisit le plan initial de la caméra et s'applique à **toutes** les variantes. Le plan peut aussi être changé après coup avec `.orient3d(mode)`, et la vue reste toujours orientable à la souris.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> — Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> — Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> — Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> — Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-plane-front.html"></iframe></div>
</div>

```python
chart = sp.bar3d("Ventes", variant="grouped", labels=labels, series=series, series_names=names, orientation3d="vertical")
chart = chart.orient3d("front")
```

<h2>Scènes 3D</h2>

La scène est un second axe indépendant : `scene` change l'environnement dans lequel les barres sont dessinées et s'applique aussi à **toutes** les variantes. Chaque scène fonctionne avec chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-scene-podium.html"></iframe></div>
</div>

```python
chart = sp.bar3d("Ventes", variant="spiral", labels=years, values=counts, scene="terrain", orientation3d="vertical")
```

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan. Le tri suit aussi le graphique 2D : `sort_order="desc"` réordonne les colonnes de toute variante à série unique.

<div data-sp-registry-table="themes" data-family="bar_3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bar3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="bar_3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
