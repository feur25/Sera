# Correlogram Chart 3D

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

`sp.correlogram3d(title, labels=None, matrix=None, *, variant="circle", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_correlogram3d_chart()`, `sp.correlogram_3d()`, `sp.correlogram3d_chart()`, `sp.correlogram3d_family()`, `sp.correlograms3d()`.

## Description

`sp.correlogram3d()` is the 3D twin of `sp.correlogram()`: **every one of the 7 correlogram variants has the same 3D form**, a grid of signed columns over the correlation matrix, one column per variable pair, rising above the floor for a positive correlation and sinking below it for a negative one. The 2D variants only change how a correlation is drawn (circle, shaded square, number, ellipse, pie slice); the matrix itself never changes, so the 3D reading is intentionally identical across every one of them.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `circle` | `default`, `classic` | `labels, matrix` | A signed column per variable pair: up for a positive correlation, down for a negative one. |
| `heatmap` | `heat`, `square` | `labels, matrix` | The same signed grid as circle, from the 2D chart's shaded-square layout. |
| `text` | `number`, `value` | `labels, matrix` | The same signed grid as circle, from the 2D chart's printed-number layout. |
| `ellipse` | `oval` | `labels, matrix` | The same signed grid as circle, from the 2D chart's tilted-ellipse layout. |
| `mixed` | `combo`, `both` | `labels, matrix` | The same signed grid as circle, from the 2D chart's split upper/lower-triangle layout. |
| `pie_square` | `pie`, `mixed_pie` | `labels, matrix` | The same signed grid as circle, from the 2D chart's pie-slice layout. |
| `circle_legend` | `legend`, `scale` | `labels, matrix` | The same signed grid as circle, plus a colour-scale legend in the 2D chart. |

## Data

`labels` names each variable and `matrix` is its row-major correlation matrix (values in -1..1); the diagonal reads as a perfect self-correlation.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="correlogram3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>circle</code></h3><p>A signed column per variable pair: up for a positive correlation, down for a negative one.</p><p class="sp-3d-uses">Uses: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-circle.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Circle 3D&quot;,
    variant=&quot;circle&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;],
    matrix=[[1, 0.8, -0.3, 0.5], [0.8, 1, 0.1, -0.2], [-0.3, 0.1, 1, 0.7], [0.5, -0.2, 0.7, 1]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>heatmap</code></h3><p>The same signed grid as circle, from the 2D chart&#x27;s shaded-square layout.</p><p class="sp-3d-uses">Uses: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-heatmap.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Heatmap 3D&quot;,
    variant=&quot;heatmap&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;],
    matrix=[[1, 0.8, -0.3, 0.5], [0.8, 1, 0.1, -0.2], [-0.3, 0.1, 1, 0.7], [0.5, -0.2, 0.7, 1]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>text</code></h3><p>The same signed grid as circle, from the 2D chart&#x27;s printed-number layout.</p><p class="sp-3d-uses">Uses: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-text.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Text 3D&quot;,
    variant=&quot;text&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;],
    matrix=[[1, 0.8, -0.3, 0.5], [0.8, 1, 0.1, -0.2], [-0.3, 0.1, 1, 0.7], [0.5, -0.2, 0.7, 1]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>ellipse</code></h3><p>The same signed grid as circle, from the 2D chart&#x27;s tilted-ellipse layout.</p><p class="sp-3d-uses">Uses: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-ellipse.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Ellipse 3D&quot;,
    variant=&quot;ellipse&quot;,
    labels=[&quot;mpg&quot;, &quot;cyl&quot;, &quot;disp&quot;, &quot;hp&quot;, &quot;drat&quot;, &quot;wt&quot;, &quot;qsec&quot;],
    matrix=[[1, -0.85, -0.85, -0.78, 0.68, -0.87, 0.42], [-0.85, 1, 0.9, 0.83, -0.7, 0.78, -0.59], [-0.85, 0.9, 1, 0.79, -0.71, 0.89, -0.43], [-0.78, 0.83, 0.79, 1, -0.45, 0.66, -0.71], [0.68, -0.7, -0.71, -0.45, 1, -0.71, 0.09], [-0.87, 0.78, 0.89, 0.66, -0.71, 1, -0.17], [0.42, -0.59, -0.43, -0.71, 0.09, -0.17, 1]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>mixed</code></h3><p>The same signed grid as circle, from the 2D chart&#x27;s split upper/lower-triangle layout.</p><p class="sp-3d-uses">Uses: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-mixed.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Mixed 3D&quot;,
    variant=&quot;mixed&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;],
    matrix=[[1, 0.8, -0.3, 0.5], [0.8, 1, 0.1, -0.2], [-0.3, 0.1, 1, 0.7], [0.5, -0.2, 0.7, 1]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>pie_square</code></h3><p>The same signed grid as circle, from the 2D chart&#x27;s pie-slice layout.</p><p class="sp-3d-uses">Uses: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-pie_square.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Pie Square 3D&quot;,
    variant=&quot;pie_square&quot;,
    labels=[&quot;mpg&quot;, &quot;cyl&quot;, &quot;disp&quot;, &quot;hp&quot;, &quot;drat&quot;, &quot;wt&quot;, ...],
    matrix=[[1, -0.85, -0.85, -0.78, 0.68, -0.87, ...], [-0.85, 1, 0.9, 0.83, -0.7, 0.78, ...], [-0.85, 0.9, 1, 0.79, -0.71, 0.89, ...], [-0.78, 0.83, 0.79, 1, -0.45, 0.66, ...], [0.68, -0.7, -0.71, -0.45, 1, -0.71, ...], [-0.87, 0.78, 0.89, 0.66, -0.71, 1, ...], ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>circle_legend</code></h3><p>The same signed grid as circle, plus a colour-scale legend in the 2D chart.</p><p class="sp-3d-uses">Uses: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-circle_legend.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Circle Legend 3D&quot;,
    variant=&quot;circle_legend&quot;,
    labels=[&quot;mpg&quot;, &quot;cyl&quot;, &quot;disp&quot;, &quot;hp&quot;, &quot;drat&quot;, &quot;wt&quot;, &quot;qsec&quot;, &quot;vs&quot;],
    matrix=[[1, -0.85, -0.85, -0.78, 0.68, -0.87, 0.42, 0.66], [-0.85, 1, 0.9, 0.83, -0.7, 0.78, -0.59, -0.81], [-0.85, 0.9, 1, 0.79, -0.71, 0.89, -0.43, -0.71], [-0.78, 0.83, 0.79, 1, -0.45, 0.66, -0.71, -0.72], [0.68, -0.7, -0.71, -0.45, 1, -0.71, 0.09, 0.44], [-0.87, 0.78, 0.89, 0.66, -0.71, 1, -0.17, -0.55], [0.42, -0.59, -0.43, -0.71, 0.09, -0.17, 1, 0.74], [0.66, -0.81, -0.71, -0.72, 0.44, -0.55, 0.74, 1]],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="correlogram3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.correlogram3d(title, labels=None, matrix=None, *, variant="circle", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_correlogram3d_chart()`, `sp.correlogram_3d()`, `sp.correlogram3d_chart()`, `sp.correlogram3d_family()`, `sp.correlograms3d()`.

<h2>Description</h2>

`sp.correlogram3d()` est le jumeau 3D de `sp.correlogram()` : **les 7 variantes de correlogram partagent exactement la même forme 3D**, une grille de colonnes signées sur la matrice de corrélation, une colonne par paire de variables, s'élevant au-dessus du sol pour une corrélation positive et s'enfonçant en dessous pour une corrélation négative. Les variantes 2D ne changent que la façon de dessiner une corrélation (cercle, carré ombré, nombre, ellipse, part de camembert) ; la matrice elle-même ne change jamais, la lecture 3D est donc volontairement identique pour chacune d'elles.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `circle` | `default`, `classic` | `labels, matrix` | Une colonne signée par paire de variables : vers le haut pour une corrélation positive, vers le bas pour une négative. |
| `heatmap` | `heat`, `square` | `labels, matrix` | La même grille signée que circle, depuis la disposition en carrés ombrés du graphique 2D. |
| `text` | `number`, `value` | `labels, matrix` | La même grille signée que circle, depuis la disposition en nombres imprimés du graphique 2D. |
| `ellipse` | `oval` | `labels, matrix` | La même grille signée que circle, depuis la disposition en ellipses inclinées du graphique 2D. |
| `mixed` | `combo`, `both` | `labels, matrix` | La même grille signée que circle, depuis la disposition en triangles haut/bas séparés du graphique 2D. |
| `pie_square` | `pie`, `mixed_pie` | `labels, matrix` | La même grille signée que circle, depuis la disposition en parts de camembert du graphique 2D. |
| `circle_legend` | `legend`, `scale` | `labels, matrix` | La même grille signée que circle, plus une légende d'échelle de couleur dans le graphique 2D. |

<h2>Données</h2>

`labels` nomme chaque variable et `matrix` est sa matrice de corrélation à plat par lignes (valeurs entre -1 et 1) ; la diagonale se lit comme une auto-corrélation parfaite.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="correlogram3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>circle</code></h3><p>Une colonne signée par paire de variables : vers le haut pour une corrélation positive, vers le bas pour une négative.</p><p class="sp-3d-uses">Utilise: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-circle.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Circle 3D&quot;,
    variant=&quot;circle&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;],
    matrix=[[1, 0.8, -0.3, 0.5], [0.8, 1, 0.1, -0.2], [-0.3, 0.1, 1, 0.7], [0.5, -0.2, 0.7, 1]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>heatmap</code></h3><p>La même grille signée que circle, depuis la disposition en carrés ombrés du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-heatmap.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Heatmap 3D&quot;,
    variant=&quot;heatmap&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;],
    matrix=[[1, 0.8, -0.3, 0.5], [0.8, 1, 0.1, -0.2], [-0.3, 0.1, 1, 0.7], [0.5, -0.2, 0.7, 1]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>text</code></h3><p>La même grille signée que circle, depuis la disposition en nombres imprimés du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-text.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Text 3D&quot;,
    variant=&quot;text&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;],
    matrix=[[1, 0.8, -0.3, 0.5], [0.8, 1, 0.1, -0.2], [-0.3, 0.1, 1, 0.7], [0.5, -0.2, 0.7, 1]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>ellipse</code></h3><p>La même grille signée que circle, depuis la disposition en ellipses inclinées du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-ellipse.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Ellipse 3D&quot;,
    variant=&quot;ellipse&quot;,
    labels=[&quot;mpg&quot;, &quot;cyl&quot;, &quot;disp&quot;, &quot;hp&quot;, &quot;drat&quot;, &quot;wt&quot;, &quot;qsec&quot;],
    matrix=[[1, -0.85, -0.85, -0.78, 0.68, -0.87, 0.42], [-0.85, 1, 0.9, 0.83, -0.7, 0.78, -0.59], [-0.85, 0.9, 1, 0.79, -0.71, 0.89, -0.43], [-0.78, 0.83, 0.79, 1, -0.45, 0.66, -0.71], [0.68, -0.7, -0.71, -0.45, 1, -0.71, 0.09], [-0.87, 0.78, 0.89, 0.66, -0.71, 1, -0.17], [0.42, -0.59, -0.43, -0.71, 0.09, -0.17, 1]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>mixed</code></h3><p>La même grille signée que circle, depuis la disposition en triangles haut/bas séparés du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-mixed.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Mixed 3D&quot;,
    variant=&quot;mixed&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;],
    matrix=[[1, 0.8, -0.3, 0.5], [0.8, 1, 0.1, -0.2], [-0.3, 0.1, 1, 0.7], [0.5, -0.2, 0.7, 1]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>pie_square</code></h3><p>La même grille signée que circle, depuis la disposition en parts de camembert du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-pie_square.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Pie Square 3D&quot;,
    variant=&quot;pie_square&quot;,
    labels=[&quot;mpg&quot;, &quot;cyl&quot;, &quot;disp&quot;, &quot;hp&quot;, &quot;drat&quot;, &quot;wt&quot;, ...],
    matrix=[[1, -0.85, -0.85, -0.78, 0.68, -0.87, ...], [-0.85, 1, 0.9, 0.83, -0.7, 0.78, ...], [-0.85, 0.9, 1, 0.79, -0.71, 0.89, ...], [-0.78, 0.83, 0.79, 1, -0.45, 0.66, ...], [0.68, -0.7, -0.71, -0.45, 1, -0.71, ...], [-0.87, 0.78, 0.89, 0.66, -0.71, 1, ...], ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>circle_legend</code></h3><p>La même grille signée que circle, plus une légende d&#x27;échelle de couleur dans le graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/correlogram3d-circle_legend.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.correlogram3d(
    &quot;Circle Legend 3D&quot;,
    variant=&quot;circle_legend&quot;,
    labels=[&quot;mpg&quot;, &quot;cyl&quot;, &quot;disp&quot;, &quot;hp&quot;, &quot;drat&quot;, &quot;wt&quot;, &quot;qsec&quot;, &quot;vs&quot;],
    matrix=[[1, -0.85, -0.85, -0.78, 0.68, -0.87, 0.42, 0.66], [-0.85, 1, 0.9, 0.83, -0.7, 0.78, -0.59, -0.81], [-0.85, 0.9, 1, 0.79, -0.71, 0.89, -0.43, -0.71], [-0.78, 0.83, 0.79, 1, -0.45, 0.66, -0.71, -0.72], [0.68, -0.7, -0.71, -0.45, 1, -0.71, 0.09, 0.44], [-0.87, 0.78, 0.89, 0.66, -0.71, 1, -0.17, -0.55], [0.42, -0.59, -0.43, -0.71, 0.09, -0.17, 1, 0.74], [0.66, -0.81, -0.71, -0.72, 0.44, -0.55, 0.74, 1]],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="correlogram3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
