# Heatmap Chart 3D

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:340px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>

## Signature

`sp.heatmap3d(title, labels=None, col_labels=None, values=None, *, variant="basic", matrix=None, orientation3d="iso", scene="default", theme="none", bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_heatmap3d_chart()`, `sp.heatmap_3d()`, `sp.heatmap3d_chart()`, `sp.heatmaps3d()`.

## Description

`sp.heatmap3d()` is the 3D twin of `sp.heatmap()`: **every one of the 20 heatmap variants has a 3D form**, selected with the same `variant` keyword and fed with the same data. Layouts come from shared, family-agnostic grid strategies (rectangular, weighted, bubble, radial, hex, ridge), so geometry is never hand-written per variant. Height and colour both encode the value.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `simple`, `default`, `matrix` | `labels, col_labels, values` | One column per cell on a rectangular grid: height and colour both follow the value. |
| `annotated` | `annotate`, `labeled`, `values` | `labels, col_labels, values` | Same landscape as basic; the value labels of the 2D variant become hover read-outs. |
| `categorical` | `category`, `discrete_labels`, `cat` | `labels, col_labels, values` | Columns take a palette colour per category value instead of a continuous colour scale. |
| `unequal` | `irregular`, `weighted`, `uneven` | `labels, col_labels, values, widths, ranges` | Cell footprints follow `widths` and `ranges` (or a default irregular grid), like the 2D variable-size cells. |
| `log` | `logarithmic`, `log_scale`, `log10` | `labels, col_labels, values` | Heights and colours follow a logarithmic scale, so small values stay visible next to large ones. |
| `discrete` | `binned`, `stepped`, `bands` | `labels, col_labels, values, bins` | Values quantised into `bins` steps (5 by default): the landscape becomes terraces of equal height. |
| `correlation` | `corr`, `diverging`, `pearson` | `labels, col_labels, values` | Diverging around zero: positive values rise above the floor, negative ones sink below it. |
| `density` | `imshow`, `viridis`, `smooth` | `labels, col_labels, values` | The grid is interpolated three times denser into a field of thin columns, coloured with viridis. |
| `contour` | `iso`, `isolines`, `level` | `labels, col_labels, values, bins` | Interpolated and quantised into contour levels: stepped bands that follow the iso-lines. |
| `temporal` | `calendar`, `time`, `date`, `timeseries` | `labels, col_labels, values` | Time grids interpolated into a smooth viridis terrain, one column per interpolated cell. |
| `cluster` | `clustermap`, `dendrogram`, `reorder` | `labels, col_labels, values` | Rows and columns reordered by hierarchical clustering so similar lines sit side by side. |
| `bubble` | `size_scaled`, `circle_heatmap`, `punchcard` | `labels, col_labels, values` | The footprint of every column scales with its value: heavy cells fatten, light cells thin out. |
| `marginal` | `with_marginals`, `histograms`, `side_bars` | `labels, col_labels, values` | Row totals and column totals stand as extra columns beyond the grid edges, like marginal histograms. |
| `confusion` | `confusion_matrix`, `classifier`, `cm` | `labels, col_labels, values` | Diagonal cells (correct predictions) keep their full tone while off-diagonal cells are muted. |
| `pivot` | `pivot_table`, `totals`, `summary` | `labels, col_labels, values` | Row and column totals are appended right next to the grid, pivot-table style. |
| `polar` | `wheel`, `clock`, `radial_heat`, `carbon_wheel` | `labels, col_labels, values` | Rows become concentric rings and columns become angular sectors around the centre. |
| `radial_cluster` | `circular_cluster`, `circos`, `radial_dendrogram`, `circular_dendrogram` | `labels, col_labels, values` | The polar layout with rows and columns reordered by hierarchical clustering. |
| `hex_grid` | `hexbin_grid`, `hex_calendar`, `honeycomb`, `hex_matrix` | `labels, col_labels, matrix` | Staggered grid where odd rows shift half a cell, approximating a hexagonal tiling. |
| `horizon` | `horizon_chart`, `stock_ridge`, `banded_horizon`, `sentiment_bands` | `labels, col_labels, matrix` | Every row is a thin ridge line, rows spaced apart like stacked time-series bands. |
| `moods` | `mood_matrix`, `punchcard_grouped`, `library`, `sentiment_grid` | `labels, col_labels, matrix` | Rows grouped by their first `::` token, with a gap between groups. |

## Data

`labels` are the row names, `col_labels` the column names and `values` the row-major matrix (or pass `matrix=[[...], ...]`). The legacy `categories` / `x_labels` spelling of `heatmap3d` keeps working.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="heatmap3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Parameters

<div data-sp-registry-table="options" data-family="heatmap3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.heatmap3d(title, labels=None, col_labels=None, values=None, *, variant="basic", matrix=None, orientation3d="iso", scene="default", theme="none", bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_heatmap3d_chart()`, `sp.heatmap_3d()`, `sp.heatmap3d_chart()`, `sp.heatmaps3d()`.

<h2>Description</h2>

`sp.heatmap3d()` est le jumeau 3D de `sp.heatmap()` : **chacune des 20 variantes de heatmap a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes données. Les dispositions viennent de stratégies de grille partagées et indépendantes de la famille (rectangulaire, pondérée, bulle, radiale, hex, crête) : la géométrie n'est jamais écrite à la main par variante. La hauteur et la couleur encodent la valeur.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `simple`, `default`, `matrix` | `labels, col_labels, values` | Une colonne par cellule sur une grille rectangulaire : la hauteur et la couleur suivent la valeur. |
| `annotated` | `annotate`, `labeled`, `values` | `labels, col_labels, values` | Même paysage que basic ; les étiquettes de valeur de la variante 2D deviennent des infobulles. |
| `categorical` | `category`, `discrete_labels`, `cat` | `labels, col_labels, values` | Les colonnes prennent une couleur de palette par valeur de catégorie plutôt qu'une échelle continue. |
| `unequal` | `irregular`, `weighted`, `uneven` | `labels, col_labels, values, widths, ranges` | Les empreintes des cellules suivent `widths` et `ranges` (ou une grille irrégulière par défaut), comme les cellules de taille variable du 2D. |
| `log` | `logarithmic`, `log_scale`, `log10` | `labels, col_labels, values` | Hauteurs et couleurs suivent une échelle logarithmique : les petites valeurs restent visibles à côté des grandes. |
| `discrete` | `binned`, `stepped`, `bands` | `labels, col_labels, values, bins` | Valeurs quantifiées en `bins` paliers (5 par défaut) : le paysage devient des terrasses de hauteur égale. |
| `correlation` | `corr`, `diverging`, `pearson` | `labels, col_labels, values` | Divergent autour de zéro : les valeurs positives montent au-dessus du plancher, les négatives descendent en dessous. |
| `density` | `imshow`, `viridis`, `smooth` | `labels, col_labels, values` | La grille est interpolée trois fois plus dense en un champ de fines colonnes, colorées en viridis. |
| `contour` | `iso`, `isolines`, `level` | `labels, col_labels, values, bins` | Interpolée et quantifiée en niveaux de contour : des bandes en escalier qui suivent les iso-lignes. |
| `temporal` | `calendar`, `time`, `date`, `timeseries` | `labels, col_labels, values` | Grilles temporelles interpolées en un terrain viridis lisse, une colonne par cellule interpolée. |
| `cluster` | `clustermap`, `dendrogram`, `reorder` | `labels, col_labels, values` | Lignes et colonnes réordonnées par classification hiérarchique pour rapprocher les lignes semblables. |
| `bubble` | `size_scaled`, `circle_heatmap`, `punchcard` | `labels, col_labels, values` | L'empreinte de chaque colonne dépend de sa valeur : les cellules lourdes grossissent, les légères s'amincissent. |
| `marginal` | `with_marginals`, `histograms`, `side_bars` | `labels, col_labels, values` | Totaux de lignes et de colonnes en colonnes supplémentaires au-delà des bords, comme des histogrammes marginaux. |
| `confusion` | `confusion_matrix`, `classifier`, `cm` | `labels, col_labels, values` | Les cellules diagonales (bonnes prédictions) gardent leur teinte pleine, les autres sont atténuées. |
| `pivot` | `pivot_table`, `totals`, `summary` | `labels, col_labels, values` | Totaux de lignes et de colonnes ajoutés juste à côté de la grille, façon tableau croisé. |
| `polar` | `wheel`, `clock`, `radial_heat`, `carbon_wheel` | `labels, col_labels, values` | Les lignes deviennent des anneaux concentriques et les colonnes des secteurs angulaires autour du centre. |
| `radial_cluster` | `circular_cluster`, `circos`, `radial_dendrogram`, `circular_dendrogram` | `labels, col_labels, values` | La disposition polaire avec lignes et colonnes réordonnées par classification hiérarchique. |
| `hex_grid` | `hexbin_grid`, `hex_calendar`, `honeycomb`, `hex_matrix` | `labels, col_labels, matrix` | Grille décalée où les lignes impaires glissent d'une demi-cellule, approchant un pavage hexagonal. |
| `horizon` | `horizon_chart`, `stock_ridge`, `banded_horizon`, `sentiment_bands` | `labels, col_labels, matrix` | Chaque ligne est une fine crête, les lignes étant espacées comme des bandes de séries temporelles empilées. |
| `moods` | `mood_matrix`, `punchcard_grouped`, `library`, `sentiment_grid` | `labels, col_labels, matrix` | Lignes regroupées selon leur premier jeton `::`, avec un écart entre les groupes. |

<h2>Données</h2>

`labels` sont les noms de lignes, `col_labels` les noms de colonnes et `values` la matrice à plat par lignes (ou `matrix=[[...], ...]`). L'ancienne écriture `categories` / `x_labels` de `heatmap3d` continue de fonctionner.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="heatmap3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/heatmap3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="heatmap3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
