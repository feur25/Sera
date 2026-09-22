# Treemap Chart 3D

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

`sp.treemap3d(title, labels=None, values=None, *, variant="basic", parents=None, comparisons=None, sort_order="none", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_treemap3d_chart()`, `sp.treemap_3d()`, `sp.treemap3d_chart()`, `sp.treemap3d_family()`, `sp.treemaps3d()`.

## Description

`sp.treemap3d()` is the 3D twin of `sp.treemap()`: **every one of the 9 treemap variants has a 3D form**, selected with the same `variant` keyword and fed with the same hierarchy. Every leaf rectangle of the 2D pack becomes a block at the same footprint, so the packing algorithm (squarified, flat, or the organic voronoi layout) is never re-derived in 3D. A non-leaf container (the root, a branch group) always flattens to a thin base pad instead of standing as tall as its own leaves, so the real data never disappears under its own container. Height and colour both come from a shared hierarchy strategy also used by sunburst3D and icicle3D.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic`, `rounded` | `labels, values` | Leaf rectangles tiled edge to edge on the floor at the 2D pack's own footprint, toned by top-level branch. |
| `flat` | `mosaic`, `edge`, `tiled` | `labels, values` | The same tiled, branch-toned leaves as basic, from the flat-mosaic 2D pack. |
| `outlined` | `outline`, `stroke`, `wireframe` | `labels, values` | The same tiled leaves pressed to a much lower height, a wireframe-like reading of the tree. |
| `gapped` | `spaced`, `isolated`, `padded` | `labels, values` | A small margin shrinks every rectangle away from its neighbours, opening a visible gap between leaves. |
| `nested` | `grouped`, `framed`, `parent` | `labels, values, parents` | Leaf rectangles plus a thin frame plate under every parent group, so branches read as a shape of their own. |
| `heat` | `ramp`, `magnitude`, `intensity` | `labels, values` | Every leaf toned by its own value on a continuous ramp, brightest for the largest, independent of branch. |
| `mono` | `monochrome`, `single`, `uniform` | `labels, values` | Every leaf takes the same flat tone, leaving only the footprint to read the hierarchy. |
| `trend` | `change`, `delta`, `yoy` | `labels, values, comparisons` | Every leaf toned green or red by whether its value rose or fell against `comparisons`. |
| `voronoi` | `voronoi_treemap`, `organic`, `polygon_treemap`, `power_diagram` | `labels, values, parents` | The same tiled, branch-toned leaves as basic, from the organic power-diagram 2D pack. |

## Data

`labels` name each node, `values` its own weight and `parents` the label of its parent (empty for a root, omit entirely for a flat single-level treemap). `comparisons` gives the prior value read by `trend`.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="treemap3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Leaf rectangles tiled edge to edge on the floor at the 2D pack&#x27;s own footprint, toned by top-level branch.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>flat</code></h3><p>The same tiled, branch-toned leaves as basic, from the flat-mosaic 2D pack.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-flat.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Flat 3D&quot;,
    variant=&quot;flat&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outlined</code></h3><p>The same tiled leaves pressed to a much lower height, a wireframe-like reading of the tree.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-outlined.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Outlined 3D&quot;,
    variant=&quot;outlined&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>gapped</code></h3><p>A small margin shrinks every rectangle away from its neighbours, opening a visible gap between leaves.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-gapped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Gapped 3D&quot;,
    variant=&quot;gapped&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>nested</code></h3><p>Leaf rectangles plus a thin frame plate under every parent group, so branches read as a shape of their own.</p><p class="sp-3d-uses">Uses: <code>labels, values, parents</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-nested.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Nested 3D&quot;,
    variant=&quot;nested&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 30],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>heat</code></h3><p>Every leaf toned by its own value on a continuous ramp, brightest for the largest, independent of branch.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-heat.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Heat 3D&quot;,
    variant=&quot;heat&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>mono</code></h3><p>Every leaf takes the same flat tone, leaving only the footprint to read the hierarchy.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-mono.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Mono 3D&quot;,
    variant=&quot;mono&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>trend</code></h3><p>Every leaf toned green or red by whether its value rose or fell against <code>comparisons</code>.</p><p class="sp-3d-uses">Uses: <code>labels, values, comparisons</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-trend.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Trend 3D&quot;,
    variant=&quot;trend&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
    comparisons=[34, 27, 15, 12, 4, 10, 9],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>voronoi</code></h3><p>The same tiled, branch-toned leaves as basic, from the organic power-diagram 2D pack.</p><p class="sp-3d-uses">Uses: <code>labels, values, parents</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-voronoi.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Voronoi 3D&quot;,
    variant=&quot;voronoi&quot;,
    labels=[&quot;Glyphosate&quot;, &quot;Glufosinate&quot;, &quot;Paraquat&quot;, &quot;Atrazine&quot;, &quot;Acetochlor&quot;, &quot;Epoxiconazole&quot;, ...],
    parents=[&quot;Highly Hazardous&quot;, &quot;Highly Hazardous&quot;, &quot;Highly Hazardous&quot;, &quot;Highly Hazardous&quot;, &quot;Highly Hazardous&quot;, &quot;Highly Hazardous&quot;, ...],
    values=[940.0, 336.0, 105.0, 210.0, 133.0, 191.0, ...],
    categories=[&quot;Herbicide&quot;, &quot;Herbicide&quot;, &quot;Herbicide&quot;, &quot;Herbicide&quot;, &quot;Herbicide&quot;, &quot;Fungicide&quot;, ...],
    categories2=[&quot;Probable carcinogen&quot;, &quot;Reproductive toxicant&quot;, &quot;Acutely toxic&quot;, &quot;Endocrine disruptor&quot;, &quot;&quot;, &quot;Endocrine disruptor&quot;, ...],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="treemap3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.treemap3d(title, labels=None, values=None, *, variant="basic", parents=None, comparisons=None, sort_order="none", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_treemap3d_chart()`, `sp.treemap_3d()`, `sp.treemap3d_chart()`, `sp.treemap3d_family()`, `sp.treemaps3d()`.

<h2>Description</h2>

`sp.treemap3d()` est le jumeau 3D de `sp.treemap()` : **chacune des 9 variantes de treemap a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par la même hiérarchie. Chaque rectangle feuille de la disposition 2D devient un bloc à la même empreinte : l'algorithme de pavage (squarifié, plat, ou la disposition organique voronoi) n'est jamais redérivé en 3D. Un conteneur non-feuille (la racine, un groupe de branche) s'aplatit toujours en une fine dalle de base plutôt que de s'élever aussi haut que ses propres feuilles : la vraie donnée ne disparaît jamais sous son conteneur. Hauteur et couleur viennent d'une stratégie de hiérarchie partagée, aussi utilisée par sunburst3D et icicle3D.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic`, `rounded` | `labels, values` | Rectangles feuilles pavés bord à bord au sol, à l'empreinte du pavage 2D, teintés par branche de premier niveau. |
| `flat` | `mosaic`, `edge`, `tiled` | `labels, values` | Les mêmes feuilles pavées et teintées par branche que basic, depuis le pavage 2D en mosaïque plate. |
| `outlined` | `outline`, `stroke`, `wireframe` | `labels, values` | Les mêmes feuilles pavées écrasées à une hauteur bien plus basse, une lecture filaire de l'arbre. |
| `gapped` | `spaced`, `isolated`, `padded` | `labels, values` | Une petite marge rétrécit chaque rectangle loin de ses voisins, ouvrant un espace visible entre feuilles. |
| `nested` | `grouped`, `framed`, `parent` | `labels, values, parents` | Rectangles feuilles plus une fine dalle-cadre sous chaque groupe parent, pour que les branches se lisent comme une forme propre. |
| `heat` | `ramp`, `magnitude`, `intensity` | `labels, values` | Chaque feuille teintée selon sa propre valeur sur une rampe continue, la plus claire pour la plus grande, indépendamment de la branche. |
| `mono` | `monochrome`, `single`, `uniform` | `labels, values` | Chaque feuille prend la même teinte plate, laissant seulement l'empreinte lire la hiérarchie. |
| `trend` | `change`, `delta`, `yoy` | `labels, values, comparisons` | Chaque feuille teintée en vert ou rouge selon que sa valeur a monté ou baissé face à `comparisons`. |
| `voronoi` | `voronoi_treemap`, `organic`, `polygon_treemap`, `power_diagram` | `labels, values, parents` | Les mêmes feuilles pavées et teintées par branche que basic, depuis le pavage 2D organique en diagramme de puissance. |

<h2>Données</h2>

`labels` nomme chaque nœud, `values` son propre poids et `parents` le libellé de son parent (vide pour une racine, à omettre entièrement pour un treemap plat à un seul niveau). `comparisons` donne la valeur précédente lue par `trend`.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="treemap3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/treemap3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Rectangles feuilles pavés bord à bord au sol, à l&#x27;empreinte du pavage 2D, teintés par branche de premier niveau.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>flat</code></h3><p>Les mêmes feuilles pavées et teintées par branche que basic, depuis le pavage 2D en mosaïque plate.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-flat.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Flat 3D&quot;,
    variant=&quot;flat&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outlined</code></h3><p>Les mêmes feuilles pavées écrasées à une hauteur bien plus basse, une lecture filaire de l&#x27;arbre.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-outlined.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Outlined 3D&quot;,
    variant=&quot;outlined&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>gapped</code></h3><p>Une petite marge rétrécit chaque rectangle loin de ses voisins, ouvrant un espace visible entre feuilles.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-gapped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Gapped 3D&quot;,
    variant=&quot;gapped&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>nested</code></h3><p>Rectangles feuilles plus une fine dalle-cadre sous chaque groupe parent, pour que les branches se lisent comme une forme propre.</p><p class="sp-3d-uses">Utilise: <code>labels, values, parents</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-nested.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Nested 3D&quot;,
    variant=&quot;nested&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 30],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>heat</code></h3><p>Chaque feuille teintée selon sa propre valeur sur une rampe continue, la plus claire pour la plus grande, indépendamment de la branche.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-heat.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Heat 3D&quot;,
    variant=&quot;heat&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>mono</code></h3><p>Chaque feuille prend la même teinte plate, laissant seulement l&#x27;empreinte lire la hiérarchie.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-mono.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Mono 3D&quot;,
    variant=&quot;mono&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>trend</code></h3><p>Chaque feuille teintée en vert ou rouge selon que sa valeur a monté ou baissé face à <code>comparisons</code>.</p><p class="sp-3d-uses">Utilise: <code>labels, values, comparisons</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-trend.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Trend 3D&quot;,
    variant=&quot;trend&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
    values=[40, 25, 20, 10, 5, 8, 12],
    comparisons=[34, 27, 15, 12, 4, 10, 9],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>voronoi</code></h3><p>Les mêmes feuilles pavées et teintées par branche que basic, depuis le pavage 2D organique en diagramme de puissance.</p><p class="sp-3d-uses">Utilise: <code>labels, values, parents</code></p><iframe class="sp-preview-frame" data-src="../../previews/treemap3d-voronoi.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.treemap3d(
    &quot;Voronoi 3D&quot;,
    variant=&quot;voronoi&quot;,
    labels=[&quot;Glyphosate&quot;, &quot;Glufosinate&quot;, &quot;Paraquat&quot;, &quot;Atrazine&quot;, &quot;Acetochlor&quot;, &quot;Epoxiconazole&quot;, ...],
    parents=[&quot;Highly Hazardous&quot;, &quot;Highly Hazardous&quot;, &quot;Highly Hazardous&quot;, &quot;Highly Hazardous&quot;, &quot;Highly Hazardous&quot;, &quot;Highly Hazardous&quot;, ...],
    values=[940.0, 336.0, 105.0, 210.0, 133.0, 191.0, ...],
    categories=[&quot;Herbicide&quot;, &quot;Herbicide&quot;, &quot;Herbicide&quot;, &quot;Herbicide&quot;, &quot;Herbicide&quot;, &quot;Fungicide&quot;, ...],
    categories2=[&quot;Probable carcinogen&quot;, &quot;Reproductive toxicant&quot;, &quot;Acutely toxic&quot;, &quot;Endocrine disruptor&quot;, &quot;&quot;, &quot;Endocrine disruptor&quot;, ...],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="treemap3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
