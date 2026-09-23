# Area Chart 3D

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

`sp.area3d(title, x_labels=None, series=None, *, variant="basic", series_names=None, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_area3d_chart()`, `sp.area_3d()`, `sp.area3d_chart()`, `sp.area3d_family()`, `sp.areas3d()`.

## Description

`sp.area3d()` is the 3D twin of `sp.area()`: **every one of the 9 area variants has a 3D form**, selected with the same `variant` keyword and fed with the same series. A single series is a sloped-top strip rising from the floor; several series each stand on their own depth row, or stack on top of one another (in raw or percent terms) as a shared band strategy also used by line3D. Long series are pooled to a budget (`max_points`), so hundreds of thousands of points stay interactive.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `overlay`, `default`, `simple` | `x_labels, series` | Each series its own sloped-top strip on a separate depth row, rising from the floor. |
| `stacked` | `stack` | `x_labels, series` | Series stacked on top of each other, each band's floor the running total of the ones before it. |
| `percent` | `percent_stacked`, `normalized`, `stream100` | `x_labels, series` | The same stack normalised so every point's total reads exactly 100%. |
| `spline` | `smooth`, `curved` | `x_labels, series` | A Catmull-Rom curve through the points, densified into many short strips, on separate rows. |
| `step` | `stepped`, `stairs` | `x_labels, series` | A staircase profile filled down to the floor instead of a sloped one. |
| `gradient` | `glow`, `fade` | `x_labels, series` | A smooth spline strip toned by height, fading from the floor to the peak. |
| `ribbon` | `outlined`, `bordered`, `ggplot` | `x_labels, series` | The percent stack pressed to a thin, striped band, a lighter outlined reading of the stack. |
| `wave` | `signed`, `oscillating`, `stackplot` | `x_labels, series` | Series stacked with signed values, so the whole band can swing above and below zero. |
| `leader` | `leaderboard`, `frontrunner`, `lead_race` | `x_labels, series` | Every series as a faint thin band plus a highlighted ribbon that switches to whichever series currently leads. |

## Data

`x_labels` name the points and `series` (with `series_names`) gives one or more value lists; passing a single flat `values` list is equivalent to one series.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="area3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Each series its own sloped-top strip on a separate depth row, rising from the floor.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    x_labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[11800, 11500, 12300, 12800], [10500, 10900, 11100, 11400], [10700, 10800, 10500, 11300]],
    series_names=[&quot;North&quot;, &quot;South&quot;, &quot;East&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stacked</code></h3><p>Series stacked on top of each other, each band&#x27;s floor the running total of the ones before it.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-stacked.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Stacked 3D&quot;,
    variant=&quot;stacked&quot;,
    x_labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[11800, 11500, 12300, 12800], [10500, 10900, 11100, 11400], [10700, 10800, 10500, 11300]],
    series_names=[&quot;North&quot;, &quot;South&quot;, &quot;East&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>percent</code></h3><p>The same stack normalised so every point&#x27;s total reads exactly 100%.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-percent.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Percent 3D&quot;,
    variant=&quot;percent&quot;,
    x_labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[11800, 11500, 12300, 12800], [10500, 10900, 11100, 11400], [10700, 10800, 10500, 11300]],
    series_names=[&quot;North&quot;, &quot;South&quot;, &quot;East&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>spline</code></h3><p>A Catmull-Rom curve through the points, densified into many short strips, on separate rows.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-spline.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Spline 3D&quot;,
    variant=&quot;spline&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;, &quot;Jun&quot;, &quot;Jul&quot;, &quot;Aug&quot;],
    series=[[8200, 7100, 9400, 12300, 15800, 19200, 21500, 20100]],
    series_names=[&quot;Visitors&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>step</code></h3><p>A staircase profile filled down to the floor instead of a sloped one.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-step.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Step 3D&quot;,
    variant=&quot;step&quot;,
    x_labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    series=[[42, 42, 58, 58, 58, 71, 71]],
    series_names=[&quot;Active servers&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>gradient</code></h3><p>A smooth spline strip toned by height, fading from the floor to the peak.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-gradient.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Gradient 3D&quot;,
    variant=&quot;gradient&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;, &quot;Jun&quot;, &quot;Jul&quot;, &quot;Aug&quot;],
    series=[[8200, 7100, 9400, 12300, 15800, 19200, 21500, 20100]],
    series_names=[&quot;Revenue&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>ribbon</code></h3><p>The percent stack pressed to a thin, striped band, a lighter outlined reading of the stack.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-ribbon.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Ribbon 3D&quot;,
    variant=&quot;ribbon&quot;,
    x_labels=[&quot;1&quot;, &quot;2&quot;, &quot;3&quot;, &quot;4&quot;, &quot;5&quot;, &quot;6&quot;, ...],
    series=[[18, 22, 16, 25, 30, 21, ...], [15, 19, 24, 17, 20, 26, ...], [22, 17, 20, 23, 15, 19, ...], [12, 15, 18, 14, 17, 13, ...], [25, 21, 19, 26, 22, 28, ...], [10, 13, 11, 15, 12, 14, ...], [20, 18, 23, 19, 25, 16, ...]],
    series_names=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>wave</code></h3><p>Series stacked with signed values, so the whole band can swing above and below zero.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-wave.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Wave 3D&quot;,
    variant=&quot;wave&quot;,
    x_labels=[&quot;0.00&quot;, &quot;0.26&quot;, &quot;0.51&quot;, &quot;0.77&quot;, &quot;1.03&quot;, &quot;1.28&quot;, ...],
    series=[[0.0, 0.254, 0.491, 0.696, 0.855, 0.959, ...], [1.0, 0.967, 0.871, 0.718, 0.519, 0.285, ...], [0.0, 0.247, 0.466, 0.644, 0.772, 0.843, ...]],
    series_names=[&quot;Sin(x)&quot;, &quot;Cos(x)&quot;, &quot;Exp(-0.1*x)*Sin(x)&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>leader</code></h3><p>Every series as a faint thin band plus a highlighted ribbon that switches to whichever series currently leads.</p><p class="sp-3d-uses">Uses: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-leader.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Leader 3D&quot;,
    variant=&quot;leader&quot;,
    x_labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;, &quot;Q5&quot;, &quot;Q6&quot;, ...],
    series=[[40, 46, 52, 58, 66, 70, ...], [38, 44, 55, 63, 68, 72, ...], [30, 36, 42, 49, 58, 86, ...]],
    series_names=[&quot;Atlas&quot;, &quot;Nova&quot;, &quot;Vertex&quot;],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="area3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.area3d(title, x_labels=None, series=None, *, variant="basic", series_names=None, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_area3d_chart()`, `sp.area_3d()`, `sp.area3d_chart()`, `sp.area3d_family()`, `sp.areas3d()`.

<h2>Description</h2>

`sp.area3d()` est le jumeau 3D de `sp.area()` : **chacune des 9 variantes d'aire a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes séries. Une seule série est une bande à dessus incliné qui s'élève depuis le sol ; plusieurs séries se posent chacune sur leur propre rangée de profondeur, ou s'empilent les unes sur les autres (en valeur brute ou en pourcentage) via une stratégie de bande partagée, aussi utilisée par line3D. Les longues séries sont regroupées selon un budget (`max_points`) : des centaines de milliers de points restent fluides.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `overlay`, `default`, `simple` | `x_labels, series` | Chaque série sur sa propre bande à dessus incliné, sur une rangée de profondeur distincte, s'élevant depuis le sol. |
| `stacked` | `stack` | `x_labels, series` | Séries empilées les unes sur les autres, le plancher de chaque bande étant le cumul de celles qui la précèdent. |
| `percent` | `percent_stacked`, `normalized`, `stream100` | `x_labels, series` | Le même empilement normalisé pour que le total de chaque point lise exactement 100 %. |
| `spline` | `smooth`, `curved` | `x_labels, series` | Une courbe de Catmull-Rom à travers les points, densifiée en nombreuses bandes courtes, sur des rangées séparées. |
| `step` | `stepped`, `stairs` | `x_labels, series` | Un profil en escalier rempli jusqu'au sol plutôt qu'incliné. |
| `gradient` | `glow`, `fade` | `x_labels, series` | Une bande lisse en spline teintée selon la hauteur, du sol jusqu'au sommet. |
| `ribbon` | `outlined`, `bordered`, `ggplot` | `x_labels, series` | L'empilement en pourcentage pressé en une bande fine et striée, une lecture contourée plus légère de l'empilement. |
| `wave` | `signed`, `oscillating`, `stackplot` | `x_labels, series` | Séries empilées avec des valeurs signées : toute la bande peut osciller au-dessus et en dessous de zéro. |
| `leader` | `leaderboard`, `frontrunner`, `lead_race` | `x_labels, series` | Chaque série en bande fine et pâle plus un ruban mis en évidence qui bascule vers la série actuellement en tête. |

<h2>Données</h2>

`x_labels` nomme les points et `series` (avec `series_names`) donne une ou plusieurs listes de valeurs ; passer une seule liste `values` à plat équivaut à une série.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="area3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/area3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Chaque série sur sa propre bande à dessus incliné, sur une rangée de profondeur distincte, s&#x27;élevant depuis le sol.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    x_labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[11800, 11500, 12300, 12800], [10500, 10900, 11100, 11400], [10700, 10800, 10500, 11300]],
    series_names=[&quot;North&quot;, &quot;South&quot;, &quot;East&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stacked</code></h3><p>Séries empilées les unes sur les autres, le plancher de chaque bande étant le cumul de celles qui la précèdent.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-stacked.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Stacked 3D&quot;,
    variant=&quot;stacked&quot;,
    x_labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[11800, 11500, 12300, 12800], [10500, 10900, 11100, 11400], [10700, 10800, 10500, 11300]],
    series_names=[&quot;North&quot;, &quot;South&quot;, &quot;East&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>percent</code></h3><p>Le même empilement normalisé pour que le total de chaque point lise exactement 100 %.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-percent.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Percent 3D&quot;,
    variant=&quot;percent&quot;,
    x_labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    series=[[11800, 11500, 12300, 12800], [10500, 10900, 11100, 11400], [10700, 10800, 10500, 11300]],
    series_names=[&quot;North&quot;, &quot;South&quot;, &quot;East&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>spline</code></h3><p>Une courbe de Catmull-Rom à travers les points, densifiée en nombreuses bandes courtes, sur des rangées séparées.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-spline.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Spline 3D&quot;,
    variant=&quot;spline&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;, &quot;Jun&quot;, &quot;Jul&quot;, &quot;Aug&quot;],
    series=[[8200, 7100, 9400, 12300, 15800, 19200, 21500, 20100]],
    series_names=[&quot;Visitors&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>step</code></h3><p>Un profil en escalier rempli jusqu&#x27;au sol plutôt qu&#x27;incliné.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-step.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Step 3D&quot;,
    variant=&quot;step&quot;,
    x_labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    series=[[42, 42, 58, 58, 58, 71, 71]],
    series_names=[&quot;Active servers&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>gradient</code></h3><p>Une bande lisse en spline teintée selon la hauteur, du sol jusqu&#x27;au sommet.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-gradient.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Gradient 3D&quot;,
    variant=&quot;gradient&quot;,
    x_labels=[&quot;Jan&quot;, &quot;Feb&quot;, &quot;Mar&quot;, &quot;Apr&quot;, &quot;May&quot;, &quot;Jun&quot;, &quot;Jul&quot;, &quot;Aug&quot;],
    series=[[8200, 7100, 9400, 12300, 15800, 19200, 21500, 20100]],
    series_names=[&quot;Revenue&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>ribbon</code></h3><p>L&#x27;empilement en pourcentage pressé en une bande fine et striée, une lecture contourée plus légère de l&#x27;empilement.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-ribbon.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Ribbon 3D&quot;,
    variant=&quot;ribbon&quot;,
    x_labels=[&quot;1&quot;, &quot;2&quot;, &quot;3&quot;, &quot;4&quot;, &quot;5&quot;, &quot;6&quot;, ...],
    series=[[18, 22, 16, 25, 30, 21, ...], [15, 19, 24, 17, 20, 26, ...], [22, 17, 20, 23, 15, 19, ...], [12, 15, 18, 14, 17, 13, ...], [25, 21, 19, 26, 22, 28, ...], [10, 13, 11, 15, 12, 14, ...], [20, 18, 23, 19, 25, 16, ...]],
    series_names=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;, &quot;G&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>wave</code></h3><p>Séries empilées avec des valeurs signées : toute la bande peut osciller au-dessus et en dessous de zéro.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-wave.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Wave 3D&quot;,
    variant=&quot;wave&quot;,
    x_labels=[&quot;0.00&quot;, &quot;0.26&quot;, &quot;0.51&quot;, &quot;0.77&quot;, &quot;1.03&quot;, &quot;1.28&quot;, ...],
    series=[[0.0, 0.254, 0.491, 0.696, 0.855, 0.959, ...], [1.0, 0.967, 0.871, 0.718, 0.519, 0.285, ...], [0.0, 0.247, 0.466, 0.644, 0.772, 0.843, ...]],
    series_names=[&quot;Sin(x)&quot;, &quot;Cos(x)&quot;, &quot;Exp(-0.1*x)*Sin(x)&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>leader</code></h3><p>Chaque série en bande fine et pâle plus un ruban mis en évidence qui bascule vers la série actuellement en tête.</p><p class="sp-3d-uses">Utilise: <code>x_labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/area3d-leader.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.area3d(
    &quot;Leader 3D&quot;,
    variant=&quot;leader&quot;,
    x_labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;, &quot;Q5&quot;, &quot;Q6&quot;, ...],
    series=[[40, 46, 52, 58, 66, 70, ...], [38, 44, 55, 63, 68, 72, ...], [30, 36, 42, 49, 58, 86, ...]],
    series_names=[&quot;Atlas&quot;, &quot;Nova&quot;, &quot;Vertex&quot;],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="area3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
