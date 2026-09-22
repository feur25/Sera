# Circle Pack Chart 3D

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

`sp.circle_pack3d(title, labels=None, parents=None, values=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_circle_pack3d_chart()`, `sp.circle_pack_3d()`, `sp.circle_pack3d_chart()`, `sp.circle_pack3d_family()`, `sp.circlepacks3d()`.

## Description

`sp.circle_pack3d()` is the 3D twin of `sp.circle_pack()`: **every one of the 7 circle-pack variants has a 3D form**, selected with the same `variant` keyword and fed with the same hierarchy. Every circle of the 2D pack becomes a cylinder at the same centre and radius. A non-leaf container circle (the root, a branch) always flattens to a thin base pad instead of standing as tall as its own children, so a big root never swallows the smaller leaves under it; `leaf_focus` goes further and drops containers from the scene entirely. Up to `NODE_CAP` circles are kept, the largest first.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic` | `labels, parents, values` | Circles packed and stacked as cylinders at the pack's own position; containers flatten to a thin base so the real leaves stand out on top. |
| `flat` | `single`, `packed` | `labels, parents, values` | The same packed cylinders all pressed to one thin height, a flat schematic reading of the pack. |
| `outlined` | `outline`, `stroke` | `labels, parents, values` | The same flattened packed cylinders as flat, from the outlined 2D pack. |
| `bubble` | `bubbles`, `plain` | `labels, parents, values` | The same packed-with-flattened-containers treatment as basic, from the plain bubble 2D pack. |
| `leaf_focus` | `leaves`, `leaves_only`, `focus` | `labels, parents, values` | Only the true leaves are drawn, each a full-height cylinder at its packed position; every container circle is dropped. |
| `swarm` | `commit_swarm`, `commit_history`, `dev_swarm`, `orca` | `labels, parents, values` | The same packed-with-flattened-containers treatment as basic, from the commit-swarm styled 2D pack. |
| `matrix` | `grid`, `swarm_matrix`, `category_grid`, `space_wars` | `labels, parents, values` | Only the leaves, moved off their packed position onto a uniform square grid, each cylinder still sized by its value. |

## Data

`labels` name each node, `parents` gives the label of its parent (empty or omitted for a root/flat pack) and `values` its own weight, exactly like the 2D chart.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="circle_pack3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Circles packed and stacked as cylinders at the pack&#x27;s own position; containers flatten to a thin base so the real leaves stand out on top.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 20, 30],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>flat</code></h3><p>The same packed cylinders all pressed to one thin height, a flat schematic reading of the pack.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-flat.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Flat 3D&quot;,
    variant=&quot;flat&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[40, 30, 25, 20, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outlined</code></h3><p>The same flattened packed cylinders as flat, from the outlined 2D pack.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-outlined.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Outlined 3D&quot;,
    variant=&quot;outlined&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 20, 30],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>bubble</code></h3><p>The same packed-with-flattened-containers treatment as basic, from the plain bubble 2D pack.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-bubble.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Bubble 3D&quot;,
    variant=&quot;bubble&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;, &quot;Zeta&quot;, &quot;Eta&quot;],
    values=[40, 30, 25, 20, 15, 12, 10],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>leaf_focus</code></h3><p>Only the true leaves are drawn, each a full-height cylinder at its packed position; every container circle is dropped.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-leaf_focus.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Leaf Focus 3D&quot;,
    variant=&quot;leaf_focus&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 20, 30],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>swarm</code></h3><p>The same packed-with-flattened-containers treatment as basic, from the commit-swarm styled 2D pack.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-swarm.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Swarm 3D&quot;,
    variant=&quot;swarm&quot;,
    labels=[&quot;other::0::2e9e0bb::17::feur25::first commit&quot;, &quot;feat::0::bea5ffb::17::feur25::feat(chart): image loader in hov..&quot;, &quot;style::0::249091b::18::feur25::style(chart): fix bar spacing&quot;, &quot;feat::0::2282a4a::18::feur25::feat: tranform plot selection&quot;, &quot;style::0::d20231a::18::feur25::style: remove useless border in ..&quot;, &quot;feat::0::dca63d7::18::feur25::feat: create generic method usin..&quot;, ...],
    parents=[&quot;2026-01&quot;, &quot;2026-01&quot;, &quot;2026-01&quot;, &quot;2026-01&quot;, &quot;2026-01&quot;, &quot;2026-01&quot;, ...],
    values=[0, 243, 25, 360, 65, 1291, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>matrix</code></h3><p>Only the leaves, moved off their packed position onto a uniform square grid, each cylinder still sized by its value.</p><p class="sp-3d-uses">Uses: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-matrix.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Matrix 3D&quot;,
    variant=&quot;matrix&quot;,
    labels=[&quot;Lyra-190&quot;, &quot;Nimbus-552&quot;, &quot;Comet-631&quot;, &quot;Terra-31&quot;, &quot;Halo-5&quot;, &quot;Vega-261&quot;, ...],
    parents=[&quot;China&quot;, &quot;Japan&quot;, &quot;Japan&quot;, &quot;W. Europe&quot;, &quot;Japan&quot;, &quot;W. Europe&quot;, ...],
    categories=[&quot;Low Earth Orbit&quot;, &quot;Low Earth Orbit&quot;, &quot;Medium Earth Orbit&quot;, &quot;Medium Earth Orbit&quot;, &quot;Low Earth Orbit&quot;, &quot;Low Earth Orbit&quot;, ...],
    categories2=[&quot;Research&quot;, &quot;Navigation&quot;, &quot;Communications&quot;, &quot;Research&quot;, &quot;Navigation&quot;, &quot;Communications&quot;, ...],
    symbols=[&quot;star&quot;, &quot;circle&quot;, &quot;triangle&quot;, &quot;circle&quot;, &quot;circle&quot;, &quot;triangle&quot;, ...],
    values=[476.7, 424.1, 57.9, 70.7, 240.9, 59.7, ...],
    color_values=[2012, 1984, 2016, 1990, 1993, 1980, ...],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="circle_pack3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.circle_pack3d(title, labels=None, parents=None, values=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_circle_pack3d_chart()`, `sp.circle_pack_3d()`, `sp.circle_pack3d_chart()`, `sp.circle_pack3d_family()`, `sp.circlepacks3d()`.

<h2>Description</h2>

`sp.circle_pack3d()` est le jumeau 3D de `sp.circle_pack()` : **chacune des 7 variantes de circle-pack a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par la même hiérarchie. Chaque cercle de la disposition 2D devient un cylindre au même centre et rayon. Un cercle conteneur non-feuille (la racine, une branche) s'aplatit toujours en une fine dalle de base plutôt que de s'élever aussi haut que ses propres enfants : une grosse racine n'engloutit jamais les petites feuilles en dessous ; `leaf_focus` va plus loin et retire complètement les conteneurs de la scène. Un maximum de `NODE_CAP` cercles est gardé, les plus grands d'abord.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic` | `labels, parents, values` | Cercles empilés en cylindres à la position du pack ; les conteneurs s'aplatissent en base fine pour que les vraies feuilles ressortent au-dessus. |
| `flat` | `single`, `packed` | `labels, parents, values` | Les mêmes cylindres empilés tous écrasés à une même hauteur fine, une lecture schématique plate du pack. |
| `outlined` | `outline`, `stroke` | `labels, parents, values` | Les mêmes cylindres empilés aplatis que flat, depuis le pack 2D contouré. |
| `bubble` | `bubbles`, `plain` | `labels, parents, values` | Le même traitement empilé à conteneurs aplatis que basic, depuis le pack 2D bulle uni. |
| `leaf_focus` | `leaves`, `leaves_only`, `focus` | `labels, parents, values` | Seules les vraies feuilles sont dessinées, chacune un cylindre pleine hauteur à sa position empilée ; tout cercle conteneur est retiré. |
| `swarm` | `commit_swarm`, `commit_history`, `dev_swarm`, `orca` | `labels, parents, values` | Le même traitement empilé à conteneurs aplatis que basic, depuis le pack 2D façon essaim de commits. |
| `matrix` | `grid`, `swarm_matrix`, `category_grid`, `space_wars` | `labels, parents, values` | Seules les feuilles, déplacées de leur position empilée vers une grille carrée uniforme, chaque cylindre restant dimensionné par sa valeur. |

<h2>Données</h2>

`labels` nomme chaque nœud, `parents` donne le libellé de son parent (vide ou omis pour une racine/un pack plat) et `values` son propre poids, exactement comme le graphique 2D.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="circle_pack3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Cercles empilés en cylindres à la position du pack ; les conteneurs s&#x27;aplatissent en base fine pour que les vraies feuilles ressortent au-dessus.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 20, 30],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>flat</code></h3><p>Les mêmes cylindres empilés tous écrasés à une même hauteur fine, une lecture schématique plate du pack.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-flat.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Flat 3D&quot;,
    variant=&quot;flat&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[40, 30, 25, 20, 15],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outlined</code></h3><p>Les mêmes cylindres empilés aplatis que flat, depuis le pack 2D contouré.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-outlined.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Outlined 3D&quot;,
    variant=&quot;outlined&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 20, 30],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>bubble</code></h3><p>Le même traitement empilé à conteneurs aplatis que basic, depuis le pack 2D bulle uni.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-bubble.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Bubble 3D&quot;,
    variant=&quot;bubble&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;, &quot;Zeta&quot;, &quot;Eta&quot;],
    values=[40, 30, 25, 20, 15, 12, 10],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>leaf_focus</code></h3><p>Seules les vraies feuilles sont dessinées, chacune un cylindre pleine hauteur à sa position empilée ; tout cercle conteneur est retiré.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-leaf_focus.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Leaf Focus 3D&quot;,
    variant=&quot;leaf_focus&quot;,
    labels=[&quot;Root&quot;, &quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;A1&quot;, &quot;A2&quot;, &quot;B1&quot;],
    parents=[&quot;&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;Root&quot;, &quot;A&quot;, &quot;A&quot;, &quot;B&quot;],
    values=[0, 40, 30, 20, 20, 20, 30],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>swarm</code></h3><p>Le même traitement empilé à conteneurs aplatis que basic, depuis le pack 2D façon essaim de commits.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-swarm.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Swarm 3D&quot;,
    variant=&quot;swarm&quot;,
    labels=[&quot;other::0::2e9e0bb::17::feur25::first commit&quot;, &quot;feat::0::bea5ffb::17::feur25::feat(chart): image loader in hov..&quot;, &quot;style::0::249091b::18::feur25::style(chart): fix bar spacing&quot;, &quot;feat::0::2282a4a::18::feur25::feat: tranform plot selection&quot;, &quot;style::0::d20231a::18::feur25::style: remove useless border in ..&quot;, &quot;feat::0::dca63d7::18::feur25::feat: create generic method usin..&quot;, ...],
    parents=[&quot;2026-01&quot;, &quot;2026-01&quot;, &quot;2026-01&quot;, &quot;2026-01&quot;, &quot;2026-01&quot;, &quot;2026-01&quot;, ...],
    values=[0, 243, 25, 360, 65, 1291, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>matrix</code></h3><p>Seules les feuilles, déplacées de leur position empilée vers une grille carrée uniforme, chaque cylindre restant dimensionné par sa valeur.</p><p class="sp-3d-uses">Utilise: <code>labels, parents, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/circle_pack3d-matrix.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.circle_pack3d(
    &quot;Matrix 3D&quot;,
    variant=&quot;matrix&quot;,
    labels=[&quot;Lyra-190&quot;, &quot;Nimbus-552&quot;, &quot;Comet-631&quot;, &quot;Terra-31&quot;, &quot;Halo-5&quot;, &quot;Vega-261&quot;, ...],
    parents=[&quot;China&quot;, &quot;Japan&quot;, &quot;Japan&quot;, &quot;W. Europe&quot;, &quot;Japan&quot;, &quot;W. Europe&quot;, ...],
    categories=[&quot;Low Earth Orbit&quot;, &quot;Low Earth Orbit&quot;, &quot;Medium Earth Orbit&quot;, &quot;Medium Earth Orbit&quot;, &quot;Low Earth Orbit&quot;, &quot;Low Earth Orbit&quot;, ...],
    categories2=[&quot;Research&quot;, &quot;Navigation&quot;, &quot;Communications&quot;, &quot;Research&quot;, &quot;Navigation&quot;, &quot;Communications&quot;, ...],
    symbols=[&quot;star&quot;, &quot;circle&quot;, &quot;triangle&quot;, &quot;circle&quot;, &quot;circle&quot;, &quot;triangle&quot;, ...],
    values=[476.7, 424.1, 57.9, 70.7, 240.9, 59.7, ...],
    color_values=[2012, 1984, 2016, 1990, 1993, 1980, ...],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="circle_pack3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
