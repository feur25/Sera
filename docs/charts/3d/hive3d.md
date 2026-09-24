# Hive Plot 3D

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

`sp.hive3d(title, axes=None, labels=None, categories=None, values=None, edges_i=None, edges_j=None, edges_w=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=620, height=580, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_hive3d_chart()`, `sp.hive_3d()`, `sp.hive3d_chart()`, `sp.hive3d_family()`, `sp.radial_network3d()`.

## Description

`sp.hive3d()` is the 3D twin of `sp.hive()`: **all 5 hive variants share the same 3D form**, selected with the same `variant` keyword and fed with the same axes/edges. Every axis fans out from a shared hub at its own angle; every node sits on its axis at a radius (and, new in 3D, a column height) following its own value; every edge links two nodes directly, or through the hub's centre for `curved`.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic` | `axes, labels, categories, values, edges_i, edges_j, edges_w` | One column per node on its axis, one direct edge per link. |
| `curved` | `smooth`, `bezier` | `axes, labels, categories, values, edges_i, edges_j, edges_w` | The same columns with every edge routed through the hub's centre. |
| `weighted` | `width`, `value` | `axes, labels, categories, values, edges_i, edges_j, edges_w` | The same direct edges, thickness scaling with `edges_w`. |
| `minimal` | `thin`, `clean` | `axes, labels, categories, values, edges_i, edges_j, edges_w` | The same direct edges, thin and understated. |
| `directed` | `arrows`, `flow`, `dependency` | `axes, labels, categories, values, edges_i, edges_j, edges_w` | The same direct edges as basic; the source end reads from `edges_i`. |

## Data

`axes` names each spoke, `categories` assigns each node (named by `labels`) to its axis, `values` places it along that axis, and `edges_i`/`edges_j`/`edges_w` give the source/target/weight of every link.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="hive3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One column per node on its axis, one direct edge per link.</p><p class="sp-3d-uses">Uses: <code>axes, labels, categories, values, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/hive3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hive3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    axes=[&quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;],
    labels=[&quot;n1&quot;, &quot;n2&quot;, &quot;n3&quot;, &quot;n4&quot;, &quot;n5&quot;, &quot;n6&quot;],
    categories=[&quot;Biology&quot;, &quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;, &quot;Physics&quot;],
    values=[0.3, 0.7, 0.2, 0.9, 0.5, 0.8],
    edges_i=[0, 1, 2, 4],
    edges_j=[2, 3, 4, 5],
    edges_w=[1, 2, 1.5, 0.8],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>curved</code></h3><p>The same columns with every edge routed through the hub&#x27;s centre.</p><p class="sp-3d-uses">Uses: <code>axes, labels, categories, values, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/hive3d-curved.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hive3d(
    &quot;Curved 3D&quot;,
    variant=&quot;curved&quot;,
    axes=[&quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;],
    labels=[&quot;n1&quot;, &quot;n2&quot;, &quot;n3&quot;, &quot;n4&quot;, &quot;n5&quot;, &quot;n6&quot;],
    categories=[&quot;Biology&quot;, &quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;, &quot;Physics&quot;],
    values=[0.3, 0.7, 0.2, 0.9, 0.5, 0.8],
    edges_i=[0, 1, 2, 4],
    edges_j=[2, 3, 4, 5],
    edges_w=[1, 2, 1.5, 0.8],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>weighted</code></h3><p>The same direct edges, thickness scaling with <code>edges_w</code>.</p><p class="sp-3d-uses">Uses: <code>axes, labels, categories, values, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/hive3d-weighted.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hive3d(
    &quot;Weighted 3D&quot;,
    variant=&quot;weighted&quot;,
    axes=[&quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;],
    labels=[&quot;n1&quot;, &quot;n2&quot;, &quot;n3&quot;, &quot;n4&quot;, &quot;n5&quot;, &quot;n6&quot;],
    categories=[&quot;Biology&quot;, &quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;, &quot;Physics&quot;],
    values=[0.3, 0.7, 0.2, 0.9, 0.5, 0.8],
    edges_i=[0, 1, 2, 4],
    edges_j=[2, 3, 4, 5],
    edges_w=[1, 2, 1.5, 0.8],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>minimal</code></h3><p>The same direct edges, thin and understated.</p><p class="sp-3d-uses">Uses: <code>axes, labels, categories, values, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/hive3d-minimal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hive3d(
    &quot;Minimal 3D&quot;,
    variant=&quot;minimal&quot;,
    axes=[&quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;],
    labels=[&quot;n1&quot;, &quot;n2&quot;, &quot;n3&quot;, &quot;n4&quot;, &quot;n5&quot;, &quot;n6&quot;],
    categories=[&quot;Biology&quot;, &quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;, &quot;Physics&quot;],
    values=[0.3, 0.7, 0.2, 0.9, 0.5, 0.8],
    edges_i=[0, 1, 2, 4],
    edges_j=[2, 3, 4, 5],
    edges_w=[1, 2, 1.5, 0.8],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>directed</code></h3><p>The same direct edges as basic; the source end reads from <code>edges_i</code>.</p><p class="sp-3d-uses">Uses: <code>axes, labels, categories, values, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/hive3d-directed.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hive3d(
    &quot;Directed 3D&quot;,
    variant=&quot;directed&quot;,
    axes=[&quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;],
    labels=[&quot;n1&quot;, &quot;n2&quot;, &quot;n3&quot;, &quot;n4&quot;, &quot;n5&quot;, &quot;n6&quot;],
    categories=[&quot;Biology&quot;, &quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;, &quot;Physics&quot;],
    values=[0.3, 0.7, 0.2, 0.9, 0.5, 0.8],
    edges_i=[0, 1, 2, 4],
    edges_j=[2, 3, 4, 5],
    edges_w=[1, 2, 1.5, 0.8],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="hive3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.hive3d(title, axes=None, labels=None, categories=None, values=None, edges_i=None, edges_j=None, edges_w=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=620, height=580, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_hive3d_chart()`, `sp.hive_3d()`, `sp.hive3d_chart()`, `sp.hive3d_family()`, `sp.radial_network3d()`.

<h2>Description</h2>

`sp.hive3d()` est le jumeau 3D de `sp.hive()` : **les 5 variantes de hive partagent la même forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes axes/liens. Chaque axe rayonne depuis un moyeu commun à son propre angle ; chaque nœud se place sur son axe à un rayon (et, nouveauté en 3D, une hauteur de colonne) suivant sa propre valeur ; chaque lien relie deux nœuds directement, ou par le centre du moyeu pour `curved`.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic` | `axes, labels, categories, values, edges_i, edges_j, edges_w` | Une colonne par nœud sur son axe, un lien direct par arête. |
| `curved` | `smooth`, `bezier` | `axes, labels, categories, values, edges_i, edges_j, edges_w` | Les mêmes colonnes avec chaque lien routé par le centre du moyeu. |
| `weighted` | `width`, `value` | `axes, labels, categories, values, edges_i, edges_j, edges_w` | Les mêmes liens directs, l'épaisseur suivant `edges_w`. |
| `minimal` | `thin`, `clean` | `axes, labels, categories, values, edges_i, edges_j, edges_w` | Les mêmes liens directs, fins et discrets. |
| `directed` | `arrows`, `flow`, `dependency` | `axes, labels, categories, values, edges_i, edges_j, edges_w` | Les mêmes liens directs que basic ; l'extrémité source se lit dans `edges_i`. |

<h2>Données</h2>

`axes` nomme chaque rayon, `categories` assigne chaque nœud (nommé par `labels`) à son axe, `values` le place le long de cet axe, et `edges_i`/`edges_j`/`edges_w` donnent la source/cible/poids de chaque lien.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="hive3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/hive3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une colonne par nœud sur son axe, un lien direct par arête.</p><p class="sp-3d-uses">Utilise: <code>axes, labels, categories, values, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/hive3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hive3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    axes=[&quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;],
    labels=[&quot;n1&quot;, &quot;n2&quot;, &quot;n3&quot;, &quot;n4&quot;, &quot;n5&quot;, &quot;n6&quot;],
    categories=[&quot;Biology&quot;, &quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;, &quot;Physics&quot;],
    values=[0.3, 0.7, 0.2, 0.9, 0.5, 0.8],
    edges_i=[0, 1, 2, 4],
    edges_j=[2, 3, 4, 5],
    edges_w=[1, 2, 1.5, 0.8],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>curved</code></h3><p>Les mêmes colonnes avec chaque lien routé par le centre du moyeu.</p><p class="sp-3d-uses">Utilise: <code>axes, labels, categories, values, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/hive3d-curved.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hive3d(
    &quot;Curved 3D&quot;,
    variant=&quot;curved&quot;,
    axes=[&quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;],
    labels=[&quot;n1&quot;, &quot;n2&quot;, &quot;n3&quot;, &quot;n4&quot;, &quot;n5&quot;, &quot;n6&quot;],
    categories=[&quot;Biology&quot;, &quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;, &quot;Physics&quot;],
    values=[0.3, 0.7, 0.2, 0.9, 0.5, 0.8],
    edges_i=[0, 1, 2, 4],
    edges_j=[2, 3, 4, 5],
    edges_w=[1, 2, 1.5, 0.8],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>weighted</code></h3><p>Les mêmes liens directs, l&#x27;épaisseur suivant <code>edges_w</code>.</p><p class="sp-3d-uses">Utilise: <code>axes, labels, categories, values, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/hive3d-weighted.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hive3d(
    &quot;Weighted 3D&quot;,
    variant=&quot;weighted&quot;,
    axes=[&quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;],
    labels=[&quot;n1&quot;, &quot;n2&quot;, &quot;n3&quot;, &quot;n4&quot;, &quot;n5&quot;, &quot;n6&quot;],
    categories=[&quot;Biology&quot;, &quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;, &quot;Physics&quot;],
    values=[0.3, 0.7, 0.2, 0.9, 0.5, 0.8],
    edges_i=[0, 1, 2, 4],
    edges_j=[2, 3, 4, 5],
    edges_w=[1, 2, 1.5, 0.8],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>minimal</code></h3><p>Les mêmes liens directs, fins et discrets.</p><p class="sp-3d-uses">Utilise: <code>axes, labels, categories, values, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/hive3d-minimal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hive3d(
    &quot;Minimal 3D&quot;,
    variant=&quot;minimal&quot;,
    axes=[&quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;],
    labels=[&quot;n1&quot;, &quot;n2&quot;, &quot;n3&quot;, &quot;n4&quot;, &quot;n5&quot;, &quot;n6&quot;],
    categories=[&quot;Biology&quot;, &quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;, &quot;Physics&quot;],
    values=[0.3, 0.7, 0.2, 0.9, 0.5, 0.8],
    edges_i=[0, 1, 2, 4],
    edges_j=[2, 3, 4, 5],
    edges_w=[1, 2, 1.5, 0.8],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>directed</code></h3><p>Les mêmes liens directs que basic ; l&#x27;extrémité source se lit dans <code>edges_i</code>.</p><p class="sp-3d-uses">Utilise: <code>axes, labels, categories, values, edges_i, edges_j, edges_w</code></p><iframe class="sp-preview-frame" data-src="../../previews/hive3d-directed.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.hive3d(
    &quot;Directed 3D&quot;,
    variant=&quot;directed&quot;,
    axes=[&quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;],
    labels=[&quot;n1&quot;, &quot;n2&quot;, &quot;n3&quot;, &quot;n4&quot;, &quot;n5&quot;, &quot;n6&quot;],
    categories=[&quot;Biology&quot;, &quot;Biology&quot;, &quot;Chemistry&quot;, &quot;Chemistry&quot;, &quot;Physics&quot;, &quot;Physics&quot;],
    values=[0.3, 0.7, 0.2, 0.9, 0.5, 0.8],
    edges_i=[0, 1, 2, 4],
    edges_j=[2, 3, 4, 5],
    edges_w=[1, 2, 1.5, 0.8],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="hive3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
