# Dendrogram Chart 3D

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:340px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>

## Signature

`sp.dendrogram3d(title, labels=None, matrix=None, *, variant="vertical", parents=None, k=3, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_dendrogram3d_chart()`, `sp.dendrogram_3d()`, `sp.dendrogram3d_chart()`, `sp.dendrogram3d_family()`, `sp.dendrograms3d()`.

## Description

`sp.dendrogram3d()` is the 3D twin of `sp.dendrogram()`: **every one of the 8 dendrogram variants has a 3D form**, selected with the same `variant` keyword and fed with the same leaves. Every merge node is a small cube positioned by its depth and order among siblings, and every edge between a node and its parent is a chain of small interpolated cubes rather than a single rotated block, since a block can only align to the axes. Nodes and edges come from a shared node/edge strategy (`lineage`) built for any tree or network family, not just dendrograms. `k` highlights that many clusters by tone.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `vertical` | `top`, `default`, `classic` | `labels, matrix` | Nodes stacked depth by depth top to bottom, elbowed edges (a right-angle turn at the child's depth) linking child to parent. |
| `horizontal` | `left`, `h` | `labels, matrix` | The same elbowed tree turned a quarter-turn: depth runs along the width axis instead of the row axis. |
| `radial` | `circular`, `polar` | `labels, matrix` | The tree wrapped around a ring: depth becomes radius and leaves spread by angle, edges still elbowed. |
| `compact` | `dense`, `tight` | `labels, matrix` | The same vertical elbowed tree with a tighter row spacing between depth levels. |
| `elegant` | `smooth`, `rounded` | `labels, matrix` | The same vertical tree with a straight diagonal edge from parent to child instead of an elbow. |
| `triangular` | `diagonal`, `straight`, `angular` | `labels, matrix` | The same straight diagonal parent-to-child edges as elegant, from the triangular 2D layout. |
| `genealogy` | `evolution`, `generative`, `spiral_tree`, `design_space` | `labels, parents` | The same radial ring as radial, from a hand-built genealogy tree passed through `parents`. |
| `bloom` | `cluster_bloom`, `petal`, `emotion_map`, `constellation` | `labels, parents` | The same radial ring with a tighter radius step between depth levels, drawing the rings closer together. |

## Data

`labels` name each leaf and `matrix` gives its raw feature row, clustered into a tree the same way as the 2D chart; `k` sets the highlighted cluster count. A tree can instead be supplied ready-made with `parents` (the label of each node's parent, empty for a root), as `genealogy` and `bloom` do.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="dendrogram3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Parameters

<div data-sp-registry-table="options" data-family="dendrogram3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.dendrogram3d(title, labels=None, matrix=None, *, variant="vertical", parents=None, k=3, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_dendrogram3d_chart()`, `sp.dendrogram_3d()`, `sp.dendrogram3d_chart()`, `sp.dendrogram3d_family()`, `sp.dendrograms3d()`.

<h2>Description</h2>

`sp.dendrogram3d()` est le jumeau 3D de `sp.dendrogram()` : **chacune des 8 variantes de dendrogramme a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes feuilles. Chaque nœud de fusion est un petit cube positionné par sa profondeur et son ordre parmi ses frères, et chaque arête entre un nœud et son parent est une chaîne de petits cubes interpolés plutôt qu'un seul bloc pivoté, un bloc ne pouvant s'aligner que sur les axes. Nœuds et arêtes viennent d'une stratégie nœud/arête partagée (`lineage`) conçue pour tout arbre ou réseau, pas seulement les dendrogrammes. `k` met en évidence par teinte ce nombre de clusters.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `vertical` | `top`, `default`, `classic` | `labels, matrix` | Nœuds empilés profondeur par profondeur de haut en bas, arêtes coudées (un angle droit à la profondeur de l'enfant) reliant l'enfant au parent. |
| `horizontal` | `left`, `h` | `labels, matrix` | Le même arbre coudé tourné d'un quart de tour : la profondeur court le long de l'axe de largeur au lieu de l'axe des lignes. |
| `radial` | `circular`, `polar` | `labels, matrix` | L'arbre enroulé autour d'un anneau : la profondeur devient rayon et les feuilles s'écartent par angle, arêtes toujours coudées. |
| `compact` | `dense`, `tight` | `labels, matrix` | Le même arbre coudé vertical avec un espacement de lignes plus serré entre niveaux de profondeur. |
| `elegant` | `smooth`, `rounded` | `labels, matrix` | Le même arbre vertical avec une arête diagonale droite du parent à l'enfant au lieu d'un coude. |
| `triangular` | `diagonal`, `straight`, `angular` | `labels, matrix` | Les mêmes arêtes diagonales droites parent-enfant qu'elegant, depuis la disposition 2D triangulaire. |
| `genealogy` | `evolution`, `generative`, `spiral_tree`, `design_space` | `labels, parents` | Le même anneau radial que radial, depuis un arbre généalogique construit à la main et passé via `parents`. |
| `bloom` | `cluster_bloom`, `petal`, `emotion_map`, `constellation` | `labels, parents` | Le même anneau radial avec un pas de rayon plus serré entre niveaux de profondeur, rapprochant les anneaux. |

<h2>Données</h2>

`labels` nomme chaque feuille et `matrix` donne sa ligne de caractéristiques brute, regroupée en arbre de la même façon que le graphique 2D ; `k` fixe le nombre de clusters mis en évidence. Un arbre peut aussi être fourni tout fait via `parents` (le libellé du parent de chaque nœud, vide pour une racine), comme le font `genealogy` et `bloom`.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="dendrogram3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/dendrogram3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="dendrogram3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
