# Bullet Chart 3D

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

`sp.bullet3d(title, labels=None, values=None, *, variant="basic", targets=None, max_vals=None, ranges=None, comparisons=None, sort_order="none", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_bullet3d_chart()`, `sp.bullet_3d()`, `sp.bullet3d_chart()`, `sp.bullet3d_family()`, `sp.bullets3d()`.

## Description

`sp.bullet3d()` is the 3D twin of `sp.bullet()`: **every one of the 8 bullet variants has a 3D form**, selected with the same `variant` keyword and fed with the same labels, values and optional `targets`, `max_vals`, `ranges` and `comparisons`. Every row is scaled to a percentage of its own maximum, so rows with different units share the height axis: qualitative bands stand behind as a wall, the measure is a bar in front and the target is a flat plate at its level. Bands, bars, beads and marks come from a shared layered bullet strategy, so geometry is never hand-written per variant. Long inputs are pooled to a budget (`max_points`), so hundreds of thousands of rows stay interactive.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic`, `standard` | `labels, values, targets, max_vals, ranges` | A dark range band up to `ranges` (75% by default) on a lighter track, the value as a bar in front and a plate at the target. |
| `stacked` | `stacked_ranges`, `zones`, `qualitative` | `labels, values, targets, max_vals` | Three graded qualitative bands (0-40%, 40-75%, 75-100%) stand behind the value bar. |
| `thermo` | `thermometer`, `vertical`, `column` | `labels, values, targets, max_vals` | A thermometer: a slim rail, a red tube rising from a bulb up to the value, and the target plate. |
| `segmented` | `traffic`, `rag`, `zones_color` | `labels, values, targets, max_vals` | Traffic-light bands (red, amber, green) behind the value bar and the target plate. |
| `minimal` | `sparkline`, `clean`, `naked` | `labels, values, targets, max_vals` | Only the value bar and the target plate, with no background bands. |
| `dot` | `point`, `marker`, `pip` | `labels, values, targets, max_vals` | The value is a single bead on a track instead of a bar, beside the target plate. |
| `progress` | `pill`, `bar`, `percent` | `labels, values, max_vals` | A wide pill of progress filling the track up to the value, with no target. |
| `compare` | `vs`, `ghost`, `prior` | `labels, values, targets, max_vals, comparisons` | The value bar beside a ghost bar for the prior period (`comparisons`), plus the target plate. |

## Data

`labels` name the rows and `values` are the measures; `targets` draws the target plate, `max_vals` sets each row's full scale (default: 120% of the largest of value, target and comparison), `ranges` sets where the range band ends and `comparisons` gives the prior value drawn by `compare`. Hovering a part reads its raw numbers, for example `Revenue · 80 of 120`.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="bullet3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>A dark range band up to <code>ranges</code> (75% by default) on a lighter track, the value as a bar in front and a plate at the target.</p><p class="sp-3d-uses">Uses: <code>labels, values, targets, max_vals, ranges</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stacked</code></h3><p>Three graded qualitative bands (0-40%, 40-75%, 75-100%) stand behind the value bar.</p><p class="sp-3d-uses">Uses: <code>labels, values, targets, max_vals</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-stacked.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Stacked 3D&quot;,
    variant=&quot;stacked&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>thermo</code></h3><p>A thermometer: a slim rail, a red tube rising from a bulb up to the value, and the target plate.</p><p class="sp-3d-uses">Uses: <code>labels, values, targets, max_vals</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-thermo.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Thermo 3D&quot;,
    variant=&quot;thermo&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>segmented</code></h3><p>Traffic-light bands (red, amber, green) behind the value bar and the target plate.</p><p class="sp-3d-uses">Uses: <code>labels, values, targets, max_vals</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-segmented.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Segmented 3D&quot;,
    variant=&quot;segmented&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>minimal</code></h3><p>Only the value bar and the target plate, with no background bands.</p><p class="sp-3d-uses">Uses: <code>labels, values, targets, max_vals</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-minimal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Minimal 3D&quot;,
    variant=&quot;minimal&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>dot</code></h3><p>The value is a single bead on a track instead of a bar, beside the target plate.</p><p class="sp-3d-uses">Uses: <code>labels, values, targets, max_vals</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-dot.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Dot 3D&quot;,
    variant=&quot;dot&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>progress</code></h3><p>A wide pill of progress filling the track up to the value, with no target.</p><p class="sp-3d-uses">Uses: <code>labels, values, max_vals</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-progress.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Progress 3D&quot;,
    variant=&quot;progress&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>compare</code></h3><p>The value bar beside a ghost bar for the prior period (<code>comparisons</code>), plus the target plate.</p><p class="sp-3d-uses">Uses: <code>labels, values, targets, max_vals, comparisons</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-compare.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Compare 3D&quot;,
    variant=&quot;compare&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
    comparisons=[70, 55, 3.8],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="bullet3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.bullet3d(title, labels=None, values=None, *, variant="basic", targets=None, max_vals=None, ranges=None, comparisons=None, sort_order="none", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_bullet3d_chart()`, `sp.bullet_3d()`, `sp.bullet3d_chart()`, `sp.bullet3d_family()`, `sp.bullets3d()`.

<h2>Description</h2>

`sp.bullet3d()` est le jumeau 3D de `sp.bullet()` : **chacune des 8 variantes de bullet a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes libellés, valeurs et éventuels `targets`, `max_vals`, `ranges` et `comparisons`. Chaque ligne est ramenée à un pourcentage de son propre maximum : des lignes d'unités différentes partagent donc l'axe de hauteur. Les bandes qualitatives se dressent derrière comme un mur, la mesure est une barre devant et la cible une plaque plate à son niveau. Bandes, barres, billes et repères viennent d'une stratégie de bullet en couches partagée : la géométrie n'est jamais écrite à la main par variante. Les longues entrées sont regroupées selon un budget (`max_points`) : des centaines de milliers de lignes restent fluides.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic`, `standard` | `labels, values, targets, max_vals, ranges` | Une bande de plage sombre jusqu'à `ranges` (75 % par défaut) sur une piste plus claire, la valeur en barre devant et une plaque à la cible. |
| `stacked` | `stacked_ranges`, `zones`, `qualitative` | `labels, values, targets, max_vals` | Trois bandes qualitatives graduées (0-40 %, 40-75 %, 75-100 %) se dressent derrière la barre de valeur. |
| `thermo` | `thermometer`, `vertical`, `column` | `labels, values, targets, max_vals` | Un thermomètre : un rail fin, un tube rouge qui monte d'un bulbe jusqu'à la valeur, et la plaque cible. |
| `segmented` | `traffic`, `rag`, `zones_color` | `labels, values, targets, max_vals` | Des bandes de feu tricolore (rouge, ambre, vert) derrière la barre de valeur et la plaque cible. |
| `minimal` | `sparkline`, `clean`, `naked` | `labels, values, targets, max_vals` | Seulement la barre de valeur et la plaque cible, sans bandes de fond. |
| `dot` | `point`, `marker`, `pip` | `labels, values, targets, max_vals` | La valeur est une simple bille sur une piste au lieu d'une barre, à côté de la plaque cible. |
| `progress` | `pill`, `bar`, `percent` | `labels, values, max_vals` | Une large pilule de progression qui remplit la piste jusqu'à la valeur, sans cible. |
| `compare` | `vs`, `ghost`, `prior` | `labels, values, targets, max_vals, comparisons` | La barre de valeur à côté d'une barre fantôme pour la période précédente (`comparisons`), plus la plaque cible. |

<h2>Données</h2>

`labels` nomme les lignes et `values` sont les mesures ; `targets` trace la plaque cible, `max_vals` fixe l'échelle complète de chaque ligne (par défaut 120 % du plus grand de la valeur, de la cible et de la comparaison), `ranges` fixe où s'arrête la bande de plage et `comparisons` donne la valeur précédente dessinée par `compare`. Survoler une partie affiche ses chiffres bruts, par exemple `Revenue · 80 of 120`.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="bullet3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/bullet3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une bande de plage sombre jusqu&#x27;à <code>ranges</code> (75 % par défaut) sur une piste plus claire, la valeur en barre devant et une plaque à la cible.</p><p class="sp-3d-uses">Utilise: <code>labels, values, targets, max_vals, ranges</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stacked</code></h3><p>Trois bandes qualitatives graduées (0-40 %, 40-75 %, 75-100 %) se dressent derrière la barre de valeur.</p><p class="sp-3d-uses">Utilise: <code>labels, values, targets, max_vals</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-stacked.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Stacked 3D&quot;,
    variant=&quot;stacked&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>thermo</code></h3><p>Un thermomètre : un rail fin, un tube rouge qui monte d&#x27;un bulbe jusqu&#x27;à la valeur, et la plaque cible.</p><p class="sp-3d-uses">Utilise: <code>labels, values, targets, max_vals</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-thermo.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Thermo 3D&quot;,
    variant=&quot;thermo&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>segmented</code></h3><p>Des bandes de feu tricolore (rouge, ambre, vert) derrière la barre de valeur et la plaque cible.</p><p class="sp-3d-uses">Utilise: <code>labels, values, targets, max_vals</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-segmented.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Segmented 3D&quot;,
    variant=&quot;segmented&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>minimal</code></h3><p>Seulement la barre de valeur et la plaque cible, sans bandes de fond.</p><p class="sp-3d-uses">Utilise: <code>labels, values, targets, max_vals</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-minimal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Minimal 3D&quot;,
    variant=&quot;minimal&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>dot</code></h3><p>La valeur est une simple bille sur une piste au lieu d&#x27;une barre, à côté de la plaque cible.</p><p class="sp-3d-uses">Utilise: <code>labels, values, targets, max_vals</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-dot.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Dot 3D&quot;,
    variant=&quot;dot&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>progress</code></h3><p>Une large pilule de progression qui remplit la piste jusqu&#x27;à la valeur, sans cible.</p><p class="sp-3d-uses">Utilise: <code>labels, values, max_vals</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-progress.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Progress 3D&quot;,
    variant=&quot;progress&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>compare</code></h3><p>La barre de valeur à côté d&#x27;une barre fantôme pour la période précédente (<code>comparisons</code>), plus la plaque cible.</p><p class="sp-3d-uses">Utilise: <code>labels, values, targets, max_vals, comparisons</code></p><iframe class="sp-preview-frame" data-src="../../previews/bullet3d-compare.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.bullet3d(
    &quot;Compare 3D&quot;,
    variant=&quot;compare&quot;,
    labels=[&quot;Revenue&quot;, &quot;Profit&quot;, &quot;CSAT&quot;],
    values=[80, 65, 4.2],
    targets=[90, 70, 4.5],
    max_vals=[120, 100, 5],
    comparisons=[70, 55, 3.8],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="bullet3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
