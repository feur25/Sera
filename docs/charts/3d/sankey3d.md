# Sankey Chart 3D

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

`sp.sankey3d(title, labels=None, edges_i=None, edges_j=None, edges_w=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_sankey3d_chart()`, `sp.sankey_3d()`, `sp.sankey3d_chart()`, `sp.sankey3d_family()`, `sp.sankeys3d()`.

## Description

`sp.sankey3d()` is the 3D twin of `sp.sankey()`: **all 8 sankey variants share the same 3D form**, selected with the same `variant` keyword and fed with the same flow. Every node stands as a column at its layered layer/stack position, height following its total flow; every edge is a thin ribbon running from its source's column to its target's, thickness and tone following its own weight against the flow's peak.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic` | `labels, edges_i, edges_j, edges_w` | One column per node, one weighted ribbon per link. |
| `gapped` | `spaced`, `separated` | `labels, edges_i, edges_j, edges_w` | The same layout as basic, from the 2D chart's wider node spacing. |
| `ribbon` | `wide`, `thick` | `labels, edges_i, edges_j, edges_w` | The same layout as basic with visibly thicker ribbons. |
| `minimal` | `thin`, `outline` | `labels, edges_i, edges_j, edges_w` | The same layout as basic with thin, understated ribbons. |
| `sorted` | `reordered`, `by_flow`, `ranked` | `labels, edges_i, edges_j, edges_w` | The same nodes reordered within each layer by their own total flow. |
| `hourglass` | `radiant_flow`, `nutrient_flow`, `braided`, `flow_bloom` | `labels, edges_i, edges_j, edges_w` | The same layout as basic, fed by the 2D chart's radiant hub-and-spoke flow. |
| `matrix` | `mosaic`, `dot_matrix`, `grid_flow`, `big_data` | `labels, edges_i, edges_j, edges_w` | The same layout as basic, fed by the 2D chart's dense grid-of-flows data. |
| `beacon` | `flight_radar`, `route_wheel`, `departure_board`, `hub_wheel` | `labels, edges_i, edges_j, edges_w` | The same layout as basic, fed by the 2D chart's hub-and-route flow. |

## Data

`labels` names each node, `edges_i`/`edges_j` give the source/target node index of every link and `edges_w` its weight.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="sankey3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One column per node, one weighted ribbon per link.</p><p class="sp-3d-uses">Uses: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;],
    edges_i=[0, 0, 1, 2],
    edges_j=[2, 3, 4, 4],
    edges_w=[10, 5, 8, 7],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>gapped</code></h3><p>The same layout as basic, from the 2D chart&#x27;s wider node spacing.</p><p class="sp-3d-uses">Uses: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-gapped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Gapped 3D&quot;,
    variant=&quot;gapped&quot;,
    labels=[&quot;In&quot;, &quot;Mid&quot;, &quot;Out1&quot;, &quot;Out2&quot;],
    edges_i=[0, 0, 1, 1],
    edges_j=[1, 2, 2, 3],
    edges_w=[20, 5, 12, 8],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>ribbon</code></h3><p>The same layout as basic with visibly thicker ribbons.</p><p class="sp-3d-uses">Uses: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-ribbon.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Ribbon 3D&quot;,
    variant=&quot;ribbon&quot;,
    labels=[&quot;Source&quot;, &quot;A&quot;, &quot;B&quot;, &quot;Sink&quot;],
    edges_i=[0, 0, 1, 2],
    edges_j=[1, 2, 3, 3],
    edges_w=[15, 10, 15, 10],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>minimal</code></h3><p>The same layout as basic with thin, understated ribbons.</p><p class="sp-3d-uses">Uses: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-minimal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Minimal 3D&quot;,
    variant=&quot;minimal&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;],
    edges_i=[0, 0, 1, 2],
    edges_j=[2, 3, 4, 4],
    edges_w=[10, 5, 8, 7],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>sorted</code></h3><p>The same nodes reordered within each layer by their own total flow.</p><p class="sp-3d-uses">Uses: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-sorted.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Sorted 3D&quot;,
    variant=&quot;sorted&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;],
    edges_i=[0, 1, 0, 1],
    edges_j=[2, 2, 3, 3],
    edges_w=[5, 15, 3, 2],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>hourglass</code></h3><p>The same layout as basic, fed by the 2D chart&#x27;s radiant hub-and-spoke flow.</p><p class="sp-3d-uses">Uses: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-hourglass.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Hourglass 3D&quot;,
    variant=&quot;hourglass&quot;,
    labels=[&quot;Spinach&quot;, &quot;Kale&quot;, &quot;Salmon&quot;, &quot;Beef Liver&quot;, &quot;Egg Yolk&quot;, &quot;Orange&quot;, ...],
    edges_i=[0, 0, 0, 0, 1, 1, ...],
    edges_j=[45, 46, 47, 48, 45, 49, ...],
    edges_w=[180, 15, 49, 20, 684, 206, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>matrix</code></h3><p>The same layout as basic, fed by the 2D chart&#x27;s dense grid-of-flows data.</p><p class="sp-3d-uses">Uses: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-matrix.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Matrix 3D&quot;,
    variant=&quot;matrix&quot;,
    labels=[&quot;S0&quot;, &quot;S1&quot;, &quot;S2&quot;, &quot;S3&quot;, &quot;S4&quot;, &quot;S5&quot;, ...],
    edges_i=[0, 1, 2, 3, 4, 5, ...],
    edges_j=[220, 221, 221, 222, 221, 222, ...],
    edges_w=[120.6, 43.7, 21.2, 29.1, 48.9, 34.7, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>beacon</code></h3><p>The same layout as basic, fed by the 2D chart&#x27;s hub-and-route flow.</p><p class="sp-3d-uses">Uses: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-beacon.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Beacon 3D&quot;,
    variant=&quot;beacon&quot;,
    labels=[&quot;ATL&quot;, &quot;MSP 06:10&quot;, &quot;JFK 07:05&quot;, &quot;CHS 07:35&quot;, &quot;BNA 07:45&quot;, &quot;PHL 08:00&quot;, ...],
    edges_i=[0, 0, 0, 0, 0, 0, ...],
    edges_j=[1, 2, 3, 4, 5, 6, ...],
    edges_w=[143, 115, 72, 66, 119, 63, ...],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="sankey3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.sankey3d(title, labels=None, edges_i=None, edges_j=None, edges_w=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_sankey3d_chart()`, `sp.sankey_3d()`, `sp.sankey3d_chart()`, `sp.sankey3d_family()`, `sp.sankeys3d()`.

<h2>Description</h2>

`sp.sankey3d()` est le jumeau 3D de `sp.sankey()` : **les 8 variantes de sankey partagent la même forme 3D**, choisie avec le même mot-clé `variant` et alimentée par le même flux. Chaque nœud se tient en colonne à sa position de couche/pile, la hauteur suivant son flux total ; chaque lien est un fin ruban courant de la colonne source à la colonne cible, épaisseur et teinte suivant son propre poids face au pic du flux.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic` | `labels, edges_i, edges_j, edges_w` | Une colonne par nœud, un ruban pondéré par lien. |
| `gapped` | `spaced`, `separated` | `labels, edges_i, edges_j, edges_w` | La même disposition que basic, depuis l'espacement de nœuds plus large du graphique 2D. |
| `ribbon` | `wide`, `thick` | `labels, edges_i, edges_j, edges_w` | La même disposition que basic avec des rubans visiblement plus épais. |
| `minimal` | `thin`, `outline` | `labels, edges_i, edges_j, edges_w` | La même disposition que basic avec des rubans fins et discrets. |
| `sorted` | `reordered`, `by_flow`, `ranked` | `labels, edges_i, edges_j, edges_w` | Les mêmes nœuds réordonnés au sein de chaque couche selon leur propre flux total. |
| `hourglass` | `radiant_flow`, `nutrient_flow`, `braided`, `flow_bloom` | `labels, edges_i, edges_j, edges_w` | La même disposition que basic, alimentée par le flux rayonnant en moyeu du graphique 2D. |
| `matrix` | `mosaic`, `dot_matrix`, `grid_flow`, `big_data` | `labels, edges_i, edges_j, edges_w` | La même disposition que basic, alimentée par les données denses en grille de flux du graphique 2D. |
| `beacon` | `flight_radar`, `route_wheel`, `departure_board`, `hub_wheel` | `labels, edges_i, edges_j, edges_w` | La même disposition que basic, alimentée par le flux en moyeu et routes du graphique 2D. |

<h2>Données</h2>

`labels` nomme chaque nœud, `edges_i`/`edges_j` donnent l'indice du nœud source/cible de chaque lien et `edges_w` son poids.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="sankey3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/sankey3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une colonne par nœud, un ruban pondéré par lien.</p><p class="sp-3d-uses">Utilise: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;],
    edges_i=[0, 0, 1, 2],
    edges_j=[2, 3, 4, 4],
    edges_w=[10, 5, 8, 7],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>gapped</code></h3><p>La même disposition que basic, depuis l&#x27;espacement de nœuds plus large du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-gapped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Gapped 3D&quot;,
    variant=&quot;gapped&quot;,
    labels=[&quot;In&quot;, &quot;Mid&quot;, &quot;Out1&quot;, &quot;Out2&quot;],
    edges_i=[0, 0, 1, 1],
    edges_j=[1, 2, 2, 3],
    edges_w=[20, 5, 12, 8],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>ribbon</code></h3><p>La même disposition que basic avec des rubans visiblement plus épais.</p><p class="sp-3d-uses">Utilise: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-ribbon.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Ribbon 3D&quot;,
    variant=&quot;ribbon&quot;,
    labels=[&quot;Source&quot;, &quot;A&quot;, &quot;B&quot;, &quot;Sink&quot;],
    edges_i=[0, 0, 1, 2],
    edges_j=[1, 2, 3, 3],
    edges_w=[15, 10, 15, 10],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>minimal</code></h3><p>La même disposition que basic avec des rubans fins et discrets.</p><p class="sp-3d-uses">Utilise: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-minimal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Minimal 3D&quot;,
    variant=&quot;minimal&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;],
    edges_i=[0, 0, 1, 2],
    edges_j=[2, 3, 4, 4],
    edges_w=[10, 5, 8, 7],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>sorted</code></h3><p>Les mêmes nœuds réordonnés au sein de chaque couche selon leur propre flux total.</p><p class="sp-3d-uses">Utilise: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-sorted.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Sorted 3D&quot;,
    variant=&quot;sorted&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;],
    edges_i=[0, 1, 0, 1],
    edges_j=[2, 2, 3, 3],
    edges_w=[5, 15, 3, 2],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>hourglass</code></h3><p>La même disposition que basic, alimentée par le flux rayonnant en moyeu du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-hourglass.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Hourglass 3D&quot;,
    variant=&quot;hourglass&quot;,
    labels=[&quot;Spinach&quot;, &quot;Kale&quot;, &quot;Salmon&quot;, &quot;Beef Liver&quot;, &quot;Egg Yolk&quot;, &quot;Orange&quot;, ...],
    edges_i=[0, 0, 0, 0, 1, 1, ...],
    edges_j=[45, 46, 47, 48, 45, 49, ...],
    edges_w=[180, 15, 49, 20, 684, 206, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>matrix</code></h3><p>La même disposition que basic, alimentée par les données denses en grille de flux du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-matrix.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Matrix 3D&quot;,
    variant=&quot;matrix&quot;,
    labels=[&quot;S0&quot;, &quot;S1&quot;, &quot;S2&quot;, &quot;S3&quot;, &quot;S4&quot;, &quot;S5&quot;, ...],
    edges_i=[0, 1, 2, 3, 4, 5, ...],
    edges_j=[220, 221, 221, 222, 221, 222, ...],
    edges_w=[120.6, 43.7, 21.2, 29.1, 48.9, 34.7, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>beacon</code></h3><p>La même disposition que basic, alimentée par le flux en moyeu et routes du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/sankey3d-beacon.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.sankey3d(
    &quot;Beacon 3D&quot;,
    variant=&quot;beacon&quot;,
    labels=[&quot;ATL&quot;, &quot;MSP 06:10&quot;, &quot;JFK 07:05&quot;, &quot;CHS 07:35&quot;, &quot;BNA 07:45&quot;, &quot;PHL 08:00&quot;, ...],
    edges_i=[0, 0, 0, 0, 0, 0, ...],
    edges_j=[1, 2, 3, 4, 5, 6, ...],
    edges_w=[143, 115, 72, 66, 119, 63, ...],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="sankey3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
