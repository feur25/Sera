# Icicle Chart 3D

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

`sp.icicle3d(title, labels=None, parents=None, values=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_icicle3d_chart()`, `sp.icicle_3d()`, `sp.icicle3d_chart()`, `sp.icicle3d_family()`, `sp.icicles3d()`.

## Description

`sp.icicle3d()` is the 3D twin of `sp.icicle()`: **every one of the 5 icicle variants has a 3D form**, selected with the same `variant` keyword and fed with the same hierarchy. Every node is a flat rectangle on its own row (one row per depth level), as wide as its share of its parent, toned by branch or by rank. The same hierarchy strategy that powers sunburst3D also wraps the tree onto a ring for `radial`. Very deep or wide trees are capped level by level to a block budget, always keeping whole depth levels intact.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic`, `layers` | `labels, parents, values` | Flat tiers stacked by depth, each node a rectangle whose width is its share of its parent. |
| `gapped` | `spaced`, `isolated`, `padded` | `labels, parents, values` | The same tiers with a thin gap opening between neighbouring rectangles. |
| `horizontal` | `h`, `sideways`, `left_to_right` | `labels, parents, values` | The tiers turned a quarter-turn: depth runs along the width axis instead of the row axis. |
| `radial` | `sunburst`, `polar`, `mandala` | `labels, parents, values` | The same tree wrapped around a ring instead of laid out on straight tiers. |
| `rank` | `percentile`, `peer_rank`, `standing` | `labels, parents, values` | Each rectangle toned by its percentile among same-depth siblings, brightest for the largest. |

## Data

`labels` name each node, `parents` gives the label of its parent (empty for a root) and `values` its own weight; a node's value is grown to the sum of its children when that sum is larger, exactly like the 2D chart.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="icicle3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Flat tiers stacked by depth, each node a rectangle whose width is its share of its parent.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/icicle3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.icicle3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Company&quot;, &quot;Engineering&quot;, &quot;Sales&quot;, &quot;Marketing&quot;, &quot;Operations&quot;, &quot;Backend&quot;, ...],
    parents=[&quot;&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Engineering&quot;, ...],
    values=[0, 0, 0, 0, 0, 18, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>gapped</code></h3><p>The same tiers with a thin gap opening between neighbouring rectangles.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/icicle3d-gapped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.icicle3d(
    &quot;Gapped 3D&quot;,
    variant=&quot;gapped&quot;,
    labels=[&quot;Company&quot;, &quot;Engineering&quot;, &quot;Sales&quot;, &quot;Marketing&quot;, &quot;Operations&quot;, &quot;Backend&quot;, ...],
    parents=[&quot;&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Engineering&quot;, ...],
    values=[0, 0, 0, 0, 0, 18, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>horizontal</code></h3><p>The tiers turned a quarter-turn: depth runs along the width axis instead of the row axis.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/icicle3d-horizontal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.icicle3d(
    &quot;Horizontal 3D&quot;,
    variant=&quot;horizontal&quot;,
    labels=[&quot;Company&quot;, &quot;Engineering&quot;, &quot;Sales&quot;, &quot;Marketing&quot;, &quot;Operations&quot;, &quot;Backend&quot;, ...],
    parents=[&quot;&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Engineering&quot;, ...],
    values=[0, 0, 0, 0, 0, 18, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>radial</code></h3><p>The same tree wrapped around a ring instead of laid out on straight tiers.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/icicle3d-radial.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.icicle3d(
    &quot;Radial 3D&quot;,
    variant=&quot;radial&quot;,
    labels=[&quot;Company&quot;, &quot;Engineering&quot;, &quot;Sales&quot;, &quot;Marketing&quot;, &quot;Operations&quot;, &quot;Backend&quot;, ...],
    parents=[&quot;&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Engineering&quot;, ...],
    values=[0, 0, 0, 0, 0, 18, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>rank</code></h3><p>Each rectangle toned by its percentile among same-depth siblings, brightest for the largest.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/icicle3d-rank.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.icicle3d(
    &quot;Rank 3D&quot;,
    variant=&quot;rank&quot;,
    labels=[&quot;Company&quot;, &quot;Engineering&quot;, &quot;Sales&quot;, &quot;Marketing&quot;, &quot;Operations&quot;, &quot;Backend&quot;, ...],
    parents=[&quot;&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Engineering&quot;, ...],
    values=[0, 0, 0, 0, 0, 18, ...],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="icicle3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.icicle3d(title, labels=None, parents=None, values=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_icicle3d_chart()`, `sp.icicle_3d()`, `sp.icicle3d_chart()`, `sp.icicle3d_family()`, `sp.icicles3d()`.

<h2>Description</h2>

`sp.icicle3d()` est le jumeau 3D de `sp.icicle()` : **chacune des 5 variantes d'icicle a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par la même hiérarchie. Chaque nœud est un rectangle plat sur sa propre ligne (une ligne par niveau de profondeur), aussi large que sa part dans son parent, teinté par branche ou par rang. La même stratégie de hiérarchie qui alimente sunburst3D enroule aussi l'arbre sur un anneau pour `radial`. Les arbres très profonds ou très larges sont plafonnés niveau par niveau selon un budget de blocs, en gardant toujours des niveaux de profondeur entiers.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic`, `layers` | `labels, parents, values` | Des paliers plats empilés par profondeur, chaque nœud un rectangle dont la largeur est sa part dans son parent. |
| `gapped` | `spaced`, `isolated`, `padded` | `labels, parents, values` | Les mêmes paliers avec un fin espace qui s'ouvre entre rectangles voisins. |
| `horizontal` | `h`, `sideways`, `left_to_right` | `labels, parents, values` | Les paliers tournés d'un quart de tour : la profondeur court le long de l'axe de largeur au lieu de l'axe des lignes. |
| `radial` | `sunburst`, `polar`, `mandala` | `labels, parents, values` | Le même arbre enroulé autour d'un anneau plutôt que posé sur des paliers droits. |
| `rank` | `percentile`, `peer_rank`, `standing` | `labels, parents, values` | Chaque rectangle teinté selon son percentile parmi les frères et sœurs de même profondeur, le plus clair pour le plus grand. |

<h2>Données</h2>

`labels` nomme chaque nœud, `parents` donne le libellé de son parent (vide pour une racine) et `values` son propre poids ; la valeur d'un nœud est portée à la somme de ses enfants quand cette somme est plus grande, exactement comme le graphique 2D.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="icicle3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/icicle3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Des paliers plats empilés par profondeur, chaque nœud un rectangle dont la largeur est sa part dans son parent.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/icicle3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.icicle3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Company&quot;, &quot;Engineering&quot;, &quot;Sales&quot;, &quot;Marketing&quot;, &quot;Operations&quot;, &quot;Backend&quot;, ...],
    parents=[&quot;&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Engineering&quot;, ...],
    values=[0, 0, 0, 0, 0, 18, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>gapped</code></h3><p>Les mêmes paliers avec un fin espace qui s&#x27;ouvre entre rectangles voisins.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/icicle3d-gapped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.icicle3d(
    &quot;Gapped 3D&quot;,
    variant=&quot;gapped&quot;,
    labels=[&quot;Company&quot;, &quot;Engineering&quot;, &quot;Sales&quot;, &quot;Marketing&quot;, &quot;Operations&quot;, &quot;Backend&quot;, ...],
    parents=[&quot;&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Engineering&quot;, ...],
    values=[0, 0, 0, 0, 0, 18, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>horizontal</code></h3><p>Les paliers tournés d&#x27;un quart de tour : la profondeur court le long de l&#x27;axe de largeur au lieu de l&#x27;axe des lignes.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/icicle3d-horizontal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.icicle3d(
    &quot;Horizontal 3D&quot;,
    variant=&quot;horizontal&quot;,
    labels=[&quot;Company&quot;, &quot;Engineering&quot;, &quot;Sales&quot;, &quot;Marketing&quot;, &quot;Operations&quot;, &quot;Backend&quot;, ...],
    parents=[&quot;&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Engineering&quot;, ...],
    values=[0, 0, 0, 0, 0, 18, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>radial</code></h3><p>Le même arbre enroulé autour d&#x27;un anneau plutôt que posé sur des paliers droits.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/icicle3d-radial.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.icicle3d(
    &quot;Radial 3D&quot;,
    variant=&quot;radial&quot;,
    labels=[&quot;Company&quot;, &quot;Engineering&quot;, &quot;Sales&quot;, &quot;Marketing&quot;, &quot;Operations&quot;, &quot;Backend&quot;, ...],
    parents=[&quot;&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Engineering&quot;, ...],
    values=[0, 0, 0, 0, 0, 18, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>rank</code></h3><p>Chaque rectangle teinté selon son percentile parmi les frères et sœurs de même profondeur, le plus clair pour le plus grand.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/icicle3d-rank.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.icicle3d(
    &quot;Rank 3D&quot;,
    variant=&quot;rank&quot;,
    labels=[&quot;Company&quot;, &quot;Engineering&quot;, &quot;Sales&quot;, &quot;Marketing&quot;, &quot;Operations&quot;, &quot;Backend&quot;, ...],
    parents=[&quot;&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Company&quot;, &quot;Engineering&quot;, ...],
    values=[0, 0, 0, 0, 0, 18, ...],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="icicle3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
