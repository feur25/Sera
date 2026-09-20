# Boxplot Chart 3D

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

`sp.boxplot3d(title, labels=None, values=None, *, variant="basic", series=None, notch=False, jitter=0.35, boxen_depth=4, orientation3d="iso", scene="default", theme="none", bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_boxplot3d_chart()`, `sp.boxplot_3d()`, `sp.boxplot3d_chart()`, `sp.box3d()`.

## Description

`sp.boxplot3d()` is the 3D twin of `sp.boxplot()`: **every one of the 10 boxplot variants has a 3D form**, selected with the same `variant` keyword and fed with the same samples. Boxes, whiskers, notches, point clouds, violin slices and letter-value bands come from shared distribution strategies, so geometry is never hand-written per variant. The median is a highlighted slab across every box.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `simple`, `vertical` | `labels, series` | One box per category from the first to the third quartile, a thin whisker column and a highlighted median slab. |
| `horizontal` | `hbox`, `horiz`, `h` | `labels, series` | The basic layout turned a quarter-turn: categories run along the depth axis instead of the width axis. |
| `notched` | `notch`, `ci`, `confidence` | `labels, series, notch` | Boxes split around a narrower waist at the median, sized by the confidence interval of the median. |
| `grouped` | `group`, `side_by_side`, `multi` | `labels, series` | Each series becomes a row of boxes side by side in depth, categories along the width axis. |
| `points` | `all_points`, `scatter`, `raw` | `labels, series` | Boxes plus every raw sample as a small cube aligned on the box axis. |
| `outliers` | `outlier`, `fliers`, `anomalies` | `labels, series` | Boxes plus the samples beyond the whisker fences drawn as cubes. |
| `strip` | `jitter`, `stripplot` | `labels, series, jitter` | Slim boxes with every sample scattered across the width by a deterministic jitter. |
| `swarm` | `beeswarm`, `swarmplot` | `labels, series` | Slim boxes with every sample pushed sideways just enough not to overlap its neighbours. |
| `violin` | `kde_overlay`, `density` | `labels, series` | A stack of slices whose width follows the Gaussian density of the samples. |
| `letter_value` | `boxen`, `lv`, `tukey` | `labels, series, boxen_depth` | Nested bands for the quartile, eighth, sixteenth... ranges, each narrower than the last (`boxen_depth` levels). |

## Data

Pass one list of samples per category in `series` (with `labels` naming them), or flat `values` with repeated `labels`. The `grouped` variant reads `series` as groups, each split evenly across the categories.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="boxplot3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One box per category from the first to the third quartile, a thin whisker column and a highlighted median slab.</p><p class="sp-3d-uses">Uses: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>horizontal</code></h3><p>The basic layout turned a quarter-turn: categories run along the depth axis instead of the width axis.</p><p class="sp-3d-uses">Uses: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-horizontal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Horizontal 3D&quot;,
    variant=&quot;horizontal&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>notched</code></h3><p>Boxes split around a narrower waist at the median, sized by the confidence interval of the median.</p><p class="sp-3d-uses">Uses: <code>labels, series, notch</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-notched.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Notched 3D&quot;,
    variant=&quot;notched&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>grouped</code></h3><p>Each series becomes a row of boxes side by side in depth, categories along the width axis.</p><p class="sp-3d-uses">Uses: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-grouped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Grouped 3D&quot;,
    variant=&quot;grouped&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>points</code></h3><p>Boxes plus every raw sample as a small cube aligned on the box axis.</p><p class="sp-3d-uses">Uses: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-points.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Points 3D&quot;,
    variant=&quot;points&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outliers</code></h3><p>Boxes plus the samples beyond the whisker fences drawn as cubes.</p><p class="sp-3d-uses">Uses: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-outliers.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Outliers 3D&quot;,
    variant=&quot;outliers&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
    show_text=True,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>strip</code></h3><p>Slim boxes with every sample scattered across the width by a deterministic jitter.</p><p class="sp-3d-uses">Uses: <code>labels, series, jitter</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-strip.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Strip 3D&quot;,
    variant=&quot;strip&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>swarm</code></h3><p>Slim boxes with every sample pushed sideways just enough not to overlap its neighbours.</p><p class="sp-3d-uses">Uses: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-swarm.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Swarm 3D&quot;,
    variant=&quot;swarm&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>violin</code></h3><p>A stack of slices whose width follows the Gaussian density of the samples.</p><p class="sp-3d-uses">Uses: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-violin.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Violin 3D&quot;,
    variant=&quot;violin&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>letter_value</code></h3><p>Nested bands for the quartile, eighth, sixteenth... ranges, each narrower than the last (<code>boxen_depth</code> levels).</p><p class="sp-3d-uses">Uses: <code>labels, series, boxen_depth</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-letter_value.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Letter Value 3D&quot;,
    variant=&quot;letter_value&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="boxplot3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.boxplot3d(title, labels=None, values=None, *, variant="basic", series=None, notch=False, jitter=0.35, boxen_depth=4, orientation3d="iso", scene="default", theme="none", bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_boxplot3d_chart()`, `sp.boxplot_3d()`, `sp.boxplot3d_chart()`, `sp.box3d()`.

<h2>Description</h2>

`sp.boxplot3d()` est le jumeau 3D de `sp.boxplot()` : **chacune des 10 variantes de boxplot a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes échantillons. Boîtes, moustaches, encoches, nuages de points, tranches de violon et bandes letter-value viennent de stratégies de distribution partagées : la géométrie n'est jamais écrite à la main par variante. La médiane est une dalle mise en évidence à travers chaque boîte.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `simple`, `vertical` | `labels, series` | Une boîte par catégorie du premier au troisième quartile, une fine colonne de moustache et une dalle de médiane mise en évidence. |
| `horizontal` | `hbox`, `horiz`, `h` | `labels, series` | La disposition de base tournée d'un quart de tour : les catégories courent le long de l'axe de profondeur plutôt que de largeur. |
| `notched` | `notch`, `ci`, `confidence` | `labels, series, notch` | Boîtes coupées autour d'une taille plus étroite à la médiane, dimensionnée par l'intervalle de confiance de la médiane. |
| `grouped` | `group`, `side_by_side`, `multi` | `labels, series` | Chaque série devient une rangée de boîtes côte à côte en profondeur, les catégories le long de l'axe de largeur. |
| `points` | `all_points`, `scatter`, `raw` | `labels, series` | Boîtes plus chaque échantillon brut en petit cube aligné sur l'axe de la boîte. |
| `outliers` | `outlier`, `fliers`, `anomalies` | `labels, series` | Boîtes plus les échantillons au-delà des barrières des moustaches dessinés en cubes. |
| `strip` | `jitter`, `stripplot` | `labels, series, jitter` | Boîtes fines avec chaque échantillon dispersé sur la largeur par un jitter déterministe. |
| `swarm` | `beeswarm`, `swarmplot` | `labels, series` | Boîtes fines avec chaque échantillon poussé sur le côté juste assez pour ne pas chevaucher ses voisins. |
| `violin` | `kde_overlay`, `density` | `labels, series` | Une pile de tranches dont la largeur suit la densité gaussienne des échantillons. |
| `letter_value` | `boxen`, `lv`, `tukey` | `labels, series, boxen_depth` | Bandes emboîtées pour les plages du quartile, du huitième, du seizième... chacune plus étroite que la précédente (`boxen_depth` niveaux). |

<h2>Données</h2>

Passez une liste d'échantillons par catégorie dans `series` (avec `labels` pour les nommer), ou des `values` à plat avec des `labels` répétés. La variante `grouped` lit `series` comme des groupes, chacun découpé équitablement entre les catégories.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="boxplot3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une boîte par catégorie du premier au troisième quartile, une fine colonne de moustache et une dalle de médiane mise en évidence.</p><p class="sp-3d-uses">Utilise: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>horizontal</code></h3><p>La disposition de base tournée d&#x27;un quart de tour : les catégories courent le long de l&#x27;axe de profondeur plutôt que de largeur.</p><p class="sp-3d-uses">Utilise: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-horizontal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Horizontal 3D&quot;,
    variant=&quot;horizontal&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>notched</code></h3><p>Boîtes coupées autour d&#x27;une taille plus étroite à la médiane, dimensionnée par l&#x27;intervalle de confiance de la médiane.</p><p class="sp-3d-uses">Utilise: <code>labels, series, notch</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-notched.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Notched 3D&quot;,
    variant=&quot;notched&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>grouped</code></h3><p>Chaque série devient une rangée de boîtes côte à côte en profondeur, les catégories le long de l&#x27;axe de largeur.</p><p class="sp-3d-uses">Utilise: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-grouped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Grouped 3D&quot;,
    variant=&quot;grouped&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>points</code></h3><p>Boîtes plus chaque échantillon brut en petit cube aligné sur l&#x27;axe de la boîte.</p><p class="sp-3d-uses">Utilise: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-points.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Points 3D&quot;,
    variant=&quot;points&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outliers</code></h3><p>Boîtes plus les échantillons au-delà des barrières des moustaches dessinés en cubes.</p><p class="sp-3d-uses">Utilise: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-outliers.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Outliers 3D&quot;,
    variant=&quot;outliers&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
    show_text=True,
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>strip</code></h3><p>Boîtes fines avec chaque échantillon dispersé sur la largeur par un jitter déterministe.</p><p class="sp-3d-uses">Utilise: <code>labels, series, jitter</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-strip.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Strip 3D&quot;,
    variant=&quot;strip&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>swarm</code></h3><p>Boîtes fines avec chaque échantillon poussé sur le côté juste assez pour ne pas chevaucher ses voisins.</p><p class="sp-3d-uses">Utilise: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-swarm.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Swarm 3D&quot;,
    variant=&quot;swarm&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>violin</code></h3><p>Une pile de tranches dont la largeur suit la densité gaussienne des échantillons.</p><p class="sp-3d-uses">Utilise: <code>labels, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-violin.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Violin 3D&quot;,
    variant=&quot;violin&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>letter_value</code></h3><p>Bandes emboîtées pour les plages du quartile, du huitième, du seizième... chacune plus étroite que la précédente (<code>boxen_depth</code> niveaux).</p><p class="sp-3d-uses">Utilise: <code>labels, series, boxen_depth</code></p><iframe class="sp-preview-frame" data-src="../../previews/boxplot3d-letter_value.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.boxplot3d(
    &quot;Letter Value 3D&quot;,
    variant=&quot;letter_value&quot;,
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;],
    series=[[1.2, 2.4, 2.7, 3.1, 3.5, 3.8, ...], [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, ...], [1.8, 2.2, 2.6, 3.0, 3.4, 3.9, ...]],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="boxplot3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
