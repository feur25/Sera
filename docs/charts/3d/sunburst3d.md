# Sunburst Chart 3D

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

`sp.sunburst3d(title, labels=None, parents=None, values=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_sunburst3d_chart()`, `sp.sunburst_3d()`, `sp.sunburst3d_chart()`, `sp.sunburst3d_family()`, `sp.sunbursts3d()`.

## Description

`sp.sunburst3d()` is the 3D twin of `sp.sunburst()`: **every one of the 7 sunburst variants has a 3D form**, selected with the same `variant` keyword and fed with the same hierarchy. Every node draws its own ring arc at a radius set by its depth and an angular span set by its share of its parent's value, toned by which top-level branch it belongs to. Rings, depth-fade and gaps come from a shared hierarchy strategy also used by icicle3D, so geometry is never hand-written per variant. Very deep or wide trees are capped level by level to a block budget, always keeping whole depth levels intact.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic`, `ring` | `labels, parents, values` | Nested rings, one per depth level, each wedge sized by its own angular share of its parent. |
| `donut` | `hole`, `ring_hole`, `donut_ring` | `labels, parents, values` | The same nested rings with a wider hole at the centre. |
| `outlined` | `outline`, `stroke`, `wireframe` | `labels, parents, values` | Flatter wedges, closer to a wireframe reading of the same hierarchy. |
| `gapped` | `spaced`, `isolated`, `petals` | `labels, parents, values` | A small angular gap opens between every wedge. |
| `depth_fade` | `fade`, `fading`, `depth` | `labels, parents, values` | Rings shrink in height as they go deeper, so depth reads from the silhouette, not just the radius. |
| `mono` | `monochrome`, `single`, `uniform` | `labels, parents, values` | Every wedge takes the same hue instead of one per branch. |
| `zoomable` | `zoom`, `animated`, `interactive`, `drill` | `labels, parents, values` | The same nested rings as basic; the interactive drill-down is a 2D-only feature. |

## Data

`labels` name each node, `parents` gives the label of its parent (empty for a root) and `values` its own weight; a node's value is grown to the sum of its children when that sum is larger, exactly like the 2D chart. Each branch takes its own hue, so a tree with several top-level entries (several rows with an empty `parents`) reads more colourfully than one wrapped in a single root.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="sunburst3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Nested rings, one per depth level, each wedge sized by its own angular share of its parent.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>donut</code></h3><p>The same nested rings with a wider hole at the centre.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-donut.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Donut 3D&quot;,
    variant=&quot;donut&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outlined</code></h3><p>Flatter wedges, closer to a wireframe reading of the same hierarchy.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-outlined.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Outlined 3D&quot;,
    variant=&quot;outlined&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>gapped</code></h3><p>A small angular gap opens between every wedge.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-gapped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Gapped 3D&quot;,
    variant=&quot;gapped&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>depth_fade</code></h3><p>Rings shrink in height as they go deeper, so depth reads from the silhouette, not just the radius.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-depth_fade.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Depth Fade 3D&quot;,
    variant=&quot;depth_fade&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>mono</code></h3><p>Every wedge takes the same hue instead of one per branch.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-mono.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Mono 3D&quot;,
    variant=&quot;mono&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>zoomable</code></h3><p>The same nested rings as basic; the interactive drill-down is a 2D-only feature.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-zoomable.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Zoomable 3D&quot;,
    variant=&quot;zoomable&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="sunburst3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.sunburst3d(title, labels=None, parents=None, values=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_sunburst3d_chart()`, `sp.sunburst_3d()`, `sp.sunburst3d_chart()`, `sp.sunburst3d_family()`, `sp.sunbursts3d()`.

<h2>Description</h2>

`sp.sunburst3d()` est le jumeau 3D de `sp.sunburst()` : **chacune des 7 variantes de sunburst a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par la même hiérarchie. Chaque nœud trace son propre arc d'anneau, à un rayon fixé par sa profondeur et sur une portée angulaire fixée par sa part dans la valeur de son parent, teinté selon la branche de premier niveau à laquelle il appartient. Anneaux, estompage par profondeur et espacement viennent d'une stratégie de hiérarchie partagée, aussi utilisée par icicle3D : la géométrie n'est jamais écrite à la main par variante. Les arbres très profonds ou très larges sont plafonnés niveau par niveau selon un budget de blocs, en gardant toujours des niveaux de profondeur entiers.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic`, `ring` | `labels, parents, values` | Des anneaux imbriqués, un par niveau de profondeur, chaque quartier dimensionné par sa propre part angulaire dans son parent. |
| `donut` | `hole`, `ring_hole`, `donut_ring` | `labels, parents, values` | Les mêmes anneaux imbriqués avec un trou plus large au centre. |
| `outlined` | `outline`, `stroke`, `wireframe` | `labels, parents, values` | Des quartiers plus plats, une lecture plus proche d'un filaire de la même hiérarchie. |
| `gapped` | `spaced`, `isolated`, `petals` | `labels, parents, values` | Un petit espace angulaire s'ouvre entre chaque quartier. |
| `depth_fade` | `fade`, `fading`, `depth` | `labels, parents, values` | Les anneaux rétrécissent en hauteur à mesure qu'ils s'enfoncent : la profondeur se lit à la silhouette, pas seulement au rayon. |
| `mono` | `monochrome`, `single`, `uniform` | `labels, parents, values` | Chaque quartier prend la même teinte au lieu d'une par branche. |
| `zoomable` | `zoom`, `animated`, `interactive`, `drill` | `labels, parents, values` | Les mêmes anneaux imbriqués que basic ; l'exploration interactive est une fonction propre au 2D. |

<h2>Données</h2>

`labels` nomme chaque nœud, `parents` donne le libellé de son parent (vide pour une racine) et `values` son propre poids ; la valeur d'un nœud est portée à la somme de ses enfants quand cette somme est plus grande, exactement comme le graphique 2D. Chaque branche prend sa propre teinte : un arbre avec plusieurs entrées de premier niveau (plusieurs lignes à `parents` vide) se lit donc plus en couleur qu'un arbre enveloppé dans une seule racine.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="sunburst3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Des anneaux imbriqués, un par niveau de profondeur, chaque quartier dimensionné par sa propre part angulaire dans son parent.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>donut</code></h3><p>Les mêmes anneaux imbriqués avec un trou plus large au centre.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-donut.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Donut 3D&quot;,
    variant=&quot;donut&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outlined</code></h3><p>Des quartiers plus plats, une lecture plus proche d&#x27;un filaire de la même hiérarchie.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-outlined.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Outlined 3D&quot;,
    variant=&quot;outlined&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>gapped</code></h3><p>Un petit espace angulaire s&#x27;ouvre entre chaque quartier.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-gapped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Gapped 3D&quot;,
    variant=&quot;gapped&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>depth_fade</code></h3><p>Les anneaux rétrécissent en hauteur à mesure qu&#x27;ils s&#x27;enfoncent : la profondeur se lit à la silhouette, pas seulement au rayon.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-depth_fade.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Depth Fade 3D&quot;,
    variant=&quot;depth_fade&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>mono</code></h3><p>Chaque quartier prend la même teinte au lieu d&#x27;une par branche.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-mono.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Mono 3D&quot;,
    variant=&quot;mono&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>zoomable</code></h3><p>Les mêmes anneaux imbriqués que basic ; l&#x27;exploration interactive est une fonction propre au 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/sunburst3d-zoomable.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sunburst3d(
    &quot;Zoomable 3D&quot;,
    variant=&quot;zoomable&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;, &quot;B2&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 15, 15],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="sunburst3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
