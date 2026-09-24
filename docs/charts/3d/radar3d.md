# Radar Chart 3D

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

`sp.radar3d(title, axes=None, series=None, series_names=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_radar3d_chart()`, `sp.radar_3d()`, `sp.radar3d_chart()`, `sp.radar3d_family()`.

## Description

`sp.radar3d()` is the 3D twin of `sp.radar()`: **all 9 radar variants share the same 3D form**, selected with the same `variant` keyword and fed with the same series. `basic`/`lines`/`filled`/`markers`/`dashed` each become a closed loop around the axes, one per series at its own depth row, radius following its values — a real spider shape, not a flat outline. `polar_bar`/`petal`/`band`/`stacked` keep their existing ring-of-columns forms, one concentric ring per series.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic` | `axes, series, series_names` | One closed loop per series, radius following its values around the axes. |
| `lines` | `outline`, `stroke`, `no_fill` | `axes, series, series_names` | The same loops as basic; the 2D chart's stroke-only styling has no separate 3D form. |
| `filled` | `fill`, `solid`, `area` | `axes, series, series_names` | The same loops as basic; the 2D chart's filled-area styling has no separate 3D form. |
| `markers` | `dots`, `points`, `marker` | `axes, series, series_names` | The same loops as basic; the 2D chart's per-vertex dot styling has no separate 3D form. |
| `dashed` | `dash`, `dotted` | `axes, series, series_names` | The same loops as basic; the 2D chart's dashed-stroke styling has no separate 3D form. |
| `stacked` | `stack`, `cumulative` | `axes, series, series_names` | One ring of stacked columns per axis, series piled on top of each other. |
| `polar_bar` | `polar`, `bar`, `radial_bar` | `axes, series, series_names` | One concentric ring of columns per series, height following its values. |
| `band` | `range`, `uncertainty`, `minmax` | `axes, series, series_names` | A band of columns between two series' values, one per axis. |
| `petal` | `flower`, `bloom`, `petal_diagram`, `nightingale_petal` | `axes, series, series_names` | The same rings as polar_bar, visibly wider. |

## Data

`axes` names each spoke, `series` gives one row of values per series (one entry per axis) and `series_names` names each series.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="radar3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One closed loop per series, radius following its values around the axes.</p><p class="sp-3d-uses">Uses: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>lines</code></h3><p>The same loops as basic; the 2D chart&#x27;s stroke-only styling has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-lines.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Lines 3D&quot;,
    variant=&quot;lines&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>filled</code></h3><p>The same loops as basic; the 2D chart&#x27;s filled-area styling has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-filled.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Filled 3D&quot;,
    variant=&quot;filled&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>markers</code></h3><p>The same loops as basic; the 2D chart&#x27;s per-vertex dot styling has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-markers.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Markers 3D&quot;,
    variant=&quot;markers&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>dashed</code></h3><p>The same loops as basic; the 2D chart&#x27;s dashed-stroke styling has no separate 3D form.</p><p class="sp-3d-uses">Uses: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-dashed.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Dashed 3D&quot;,
    variant=&quot;dashed&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stacked</code></h3><p>One ring of stacked columns per axis, series piled on top of each other.</p><p class="sp-3d-uses">Uses: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-stacked.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Stacked 3D&quot;,
    variant=&quot;stacked&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>polar_bar</code></h3><p>One concentric ring of columns per series, height following its values.</p><p class="sp-3d-uses">Uses: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-polar_bar.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Polar Bar 3D&quot;,
    variant=&quot;polar_bar&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>band</code></h3><p>A band of columns between two series&#x27; values, one per axis.</p><p class="sp-3d-uses">Uses: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-band.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Band 3D&quot;,
    variant=&quot;band&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[55, 45, 50, 25, 50], [80, 70, 75, 45, 78], [30, 60, 35, 50, 40], [50, 85, 55, 70, 65]],
    series_names=[&quot;A low&quot;, &quot;A high&quot;, &quot;B low&quot;, &quot;B high&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>petal</code></h3><p>The same rings as polar_bar, visibly wider.</p><p class="sp-3d-uses">Uses: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-petal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Petal 3D&quot;,
    variant=&quot;petal&quot;,
    axes=[&quot;1960&quot;, &quot;1970&quot;, &quot;1980&quot;, &quot;1990&quot;, &quot;2000&quot;, &quot;2010&quot;, ...],
    series=[[102.26, 124.12, 126.79, 142.45, 134.01, 141.63, ...], [54.97, 48.53, 47.32, 49.62, 52.10, 54.33, ...]],
    series_names=[&quot;Nacimientos&quot;, &quot;Muertes&quot;],
    palette=[6274976, 4020864],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="radar3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.radar3d(title, axes=None, series=None, series_names=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_radar3d_chart()`, `sp.radar_3d()`, `sp.radar3d_chart()`, `sp.radar3d_family()`.

<h2>Description</h2>

`sp.radar3d()` est le jumeau 3D de `sp.radar()` : **les 9 variantes de radar partagent la même forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes séries. `basic`/`lines`/`filled`/`markers`/`dashed` deviennent chacune une boucle fermée autour des axes, une par série à sa propre rangée de profondeur, le rayon suivant ses valeurs — une vraie forme d'araignée, pas un simple contour à plat. `polar_bar`/`petal`/`band`/`stacked` gardent leurs formes existantes en anneaux de colonnes, un anneau concentrique par série.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic` | `axes, series, series_names` | Une boucle fermée par série, le rayon suivant ses valeurs autour des axes. |
| `lines` | `outline`, `stroke`, `no_fill` | `axes, series, series_names` | Les mêmes boucles que basic ; le style en trait seul du graphique 2D n'a pas de forme 3D séparée. |
| `filled` | `fill`, `solid`, `area` | `axes, series, series_names` | Les mêmes boucles que basic ; le style à aire remplie du graphique 2D n'a pas de forme 3D séparée. |
| `markers` | `dots`, `points`, `marker` | `axes, series, series_names` | Les mêmes boucles que basic ; le style à points par sommet du graphique 2D n'a pas de forme 3D séparée. |
| `dashed` | `dash`, `dotted` | `axes, series, series_names` | Les mêmes boucles que basic ; le style en trait pointillé du graphique 2D n'a pas de forme 3D séparée. |
| `stacked` | `stack`, `cumulative` | `axes, series, series_names` | Un anneau de colonnes empilées par axe, les séries superposées. |
| `polar_bar` | `polar`, `bar`, `radial_bar` | `axes, series, series_names` | Un anneau concentrique de colonnes par série, la hauteur suivant ses valeurs. |
| `band` | `range`, `uncertainty`, `minmax` | `axes, series, series_names` | Une bande de colonnes entre les valeurs de deux séries, une par axe. |
| `petal` | `flower`, `bloom`, `petal_diagram`, `nightingale_petal` | `axes, series, series_names` | Les mêmes anneaux que polar_bar, visiblement plus larges. |

<h2>Données</h2>

`axes` nomme chaque rayon, `series` donne une ligne de valeurs par série (une entrée par axe) et `series_names` nomme chaque série.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="radar3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/radar3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une boucle fermée par série, le rayon suivant ses valeurs autour des axes.</p><p class="sp-3d-uses">Utilise: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>lines</code></h3><p>Les mêmes boucles que basic ; le style en trait seul du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-lines.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Lines 3D&quot;,
    variant=&quot;lines&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>filled</code></h3><p>Les mêmes boucles que basic ; le style à aire remplie du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-filled.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Filled 3D&quot;,
    variant=&quot;filled&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>markers</code></h3><p>Les mêmes boucles que basic ; le style à points par sommet du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-markers.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Markers 3D&quot;,
    variant=&quot;markers&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>dashed</code></h3><p>Les mêmes boucles que basic ; le style en trait pointillé du graphique 2D n&#x27;a pas de forme 3D séparée.</p><p class="sp-3d-uses">Utilise: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-dashed.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Dashed 3D&quot;,
    variant=&quot;dashed&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stacked</code></h3><p>Un anneau de colonnes empilées par axe, les séries superposées.</p><p class="sp-3d-uses">Utilise: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-stacked.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Stacked 3D&quot;,
    variant=&quot;stacked&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>polar_bar</code></h3><p>Un anneau concentrique de colonnes par série, la hauteur suivant ses valeurs.</p><p class="sp-3d-uses">Utilise: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-polar_bar.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Polar Bar 3D&quot;,
    variant=&quot;polar_bar&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[80, 65, 70, 40, 75], [60, 80, 55, 60, 70]],
    series_names=[&quot;A&quot;, &quot;B&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>band</code></h3><p>Une bande de colonnes entre les valeurs de deux séries, une par axe.</p><p class="sp-3d-uses">Utilise: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-band.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Band 3D&quot;,
    variant=&quot;band&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;, &quot;Cost&quot;, &quot;Style&quot;],
    series=[[55, 45, 50, 25, 50], [80, 70, 75, 45, 78], [30, 60, 35, 50, 40], [50, 85, 55, 70, 65]],
    series_names=[&quot;A low&quot;, &quot;A high&quot;, &quot;B low&quot;, &quot;B high&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>petal</code></h3><p>Les mêmes anneaux que polar_bar, visiblement plus larges.</p><p class="sp-3d-uses">Utilise: <code>axes, series, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/radar3d-petal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.radar3d(
    &quot;Petal 3D&quot;,
    variant=&quot;petal&quot;,
    axes=[&quot;1960&quot;, &quot;1970&quot;, &quot;1980&quot;, &quot;1990&quot;, &quot;2000&quot;, &quot;2010&quot;, ...],
    series=[[102.26, 124.12, 126.79, 142.45, 134.01, 141.63, ...], [54.97, 48.53, 47.32, 49.62, 52.10, 54.33, ...]],
    series_names=[&quot;Nacimientos&quot;, &quot;Muertes&quot;],
    palette=[6274976, 4020864],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="radar3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
