# Histogram Chart 3D

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

`sp.histogram3d(title, values=None, *, variant="basic", bins=0, overlay=None, color_groups=None, series_names=None, orientation3d="iso", scene="default", theme="none", bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_histogram3d_chart()`, `sp.histogram_3d()`, `sp.histogram3d_chart()`, `sp.hist3d()`.

## Description

`sp.histogram3d()` is the 3D twin of `sp.histogram()`: **every one of the 7 histogram variants has a 3D form**, selected with the same `variant` keyword and binned by the same rule as the 2D chart. Columns, stacks, side-by-side rows and plates come from the shared column strategies, so geometry is never hand-written per variant.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `simple`, `default`, `vertical` | `values, bins` | One column per bin; height is the sample count. |
| `horizontal` | `h`, `barh`, `hbar` | `values, bins` | The basic layout turned a quarter-turn: bins run along the depth axis. |
| `normalized` | `probability`, `density`, `norm`, `pdf` | `values, bins` | Heights are probability densities, so the columns integrate to one. |
| `cumulative` | `cdf`, `cum` | `values, bins` | Heights accumulate from the first bin to the last, ending at the sample count. |
| `stacked` | `stack`, `stack_by` | `values, color_groups` | Each bin stacks one segment per `color_groups` category. |
| `overlay` | `overlapping`, `compare`, `ab` | `values, overlay, series_names` | A second sample set (`overlay`) drawn as a parallel row beside the first. |
| `step` | `outline`, `stair` | `values, bins` | Only the top of every bin as a thin plate, giving the outlined staircase look. |

## Data

`values` are the raw samples and `bins` the bin count (0 picks it automatically). `color_groups` splits the samples into stacked groups and `overlay` adds a second sample set drawn beside the first.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="histogram3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-cosmic.html"></iframe></div>
</div>

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One column per bin; height is the sample count.</p><p class="sp-3d-uses">Uses: <code>values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(&quot;Basic 3D&quot;, variant=&quot;basic&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>horizontal</code></h3><p>The basic layout turned a quarter-turn: bins run along the depth axis.</p><p class="sp-3d-uses">Uses: <code>values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-horizontal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(&quot;Horizontal 3D&quot;, variant=&quot;horizontal&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>normalized</code></h3><p>Heights are probability densities, so the columns integrate to one.</p><p class="sp-3d-uses">Uses: <code>values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-normalized.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(&quot;Normalized 3D&quot;, variant=&quot;normalized&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>cumulative</code></h3><p>Heights accumulate from the first bin to the last, ending at the sample count.</p><p class="sp-3d-uses">Uses: <code>values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-cumulative.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(&quot;Cumulative 3D&quot;, variant=&quot;cumulative&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stacked</code></h3><p>Each bin stacks one segment per <code>color_groups</code> category.</p><p class="sp-3d-uses">Uses: <code>values, color_groups</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-stacked.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(
    &quot;Stacked 3D&quot;,
    variant=&quot;stacked&quot;,
    values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...],
    color_groups=[&quot;A&quot;, &quot;A&quot;, &quot;A&quot;, &quot;A&quot;, &quot;A&quot;, &quot;A&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>overlay</code></h3><p>A second sample set (<code>overlay</code>) drawn as a parallel row beside the first.</p><p class="sp-3d-uses">Uses: <code>values, overlay, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-overlay.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(&quot;Overlay 3D&quot;, variant=&quot;overlay&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>step</code></h3><p>Only the top of every bin as a thin plate, giving the outlined staircase look.</p><p class="sp-3d-uses">Uses: <code>values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-step.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(&quot;Step 3D&quot;, variant=&quot;step&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="histogram3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.histogram3d(title, values=None, *, variant="basic", bins=0, overlay=None, color_groups=None, series_names=None, orientation3d="iso", scene="default", theme="none", bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_histogram3d_chart()`, `sp.histogram_3d()`, `sp.histogram3d_chart()`, `sp.hist3d()`.

<h2>Description</h2>

`sp.histogram3d()` est le jumeau 3D de `sp.histogram()` : **chacune des 7 variantes d'histogramme a une forme 3D**, choisie avec le même mot-clé `variant` et classée par la même règle que le graphique 2D. Colonnes, empilements, rangées côte à côte et plaques viennent des stratégies de colonnes partagées : la géométrie n'est jamais écrite à la main par variante.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `simple`, `default`, `vertical` | `values, bins` | Une colonne par classe ; la hauteur est le nombre d'échantillons. |
| `horizontal` | `h`, `barh`, `hbar` | `values, bins` | La disposition de base tournée d'un quart de tour : les classes courent le long de l'axe de profondeur. |
| `normalized` | `probability`, `density`, `norm`, `pdf` | `values, bins` | Les hauteurs sont des densités de probabilité : les colonnes s'intègrent à un. |
| `cumulative` | `cdf`, `cum` | `values, bins` | Les hauteurs s'accumulent de la première à la dernière classe, jusqu'au nombre d'échantillons. |
| `stacked` | `stack`, `stack_by` | `values, color_groups` | Chaque classe empile un segment par catégorie de `color_groups`. |
| `overlay` | `overlapping`, `compare`, `ab` | `values, overlay, series_names` | Un second jeu d'échantillons (`overlay`) dessiné en rangée parallèle à côté du premier. |
| `step` | `outline`, `stair` | `values, bins` | Seul le dessus de chaque classe en fine plaque, pour l'aspect d'escalier contouré. |

<h2>Données</h2>

`values` sont les échantillons bruts et `bins` le nombre de classes (0 le choisit automatiquement). `color_groups` répartit les échantillons en groupes empilés et `overlay` ajoute un second jeu d'échantillons dessiné à côté du premier.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="histogram3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/histogram3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une colonne par classe ; la hauteur est le nombre d&#x27;échantillons.</p><p class="sp-3d-uses">Utilise: <code>values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(&quot;Basic 3D&quot;, variant=&quot;basic&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>horizontal</code></h3><p>La disposition de base tournée d&#x27;un quart de tour : les classes courent le long de l&#x27;axe de profondeur.</p><p class="sp-3d-uses">Utilise: <code>values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-horizontal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(&quot;Horizontal 3D&quot;, variant=&quot;horizontal&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>normalized</code></h3><p>Les hauteurs sont des densités de probabilité : les colonnes s&#x27;intègrent à un.</p><p class="sp-3d-uses">Utilise: <code>values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-normalized.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(&quot;Normalized 3D&quot;, variant=&quot;normalized&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>cumulative</code></h3><p>Les hauteurs s&#x27;accumulent de la première à la dernière classe, jusqu&#x27;au nombre d&#x27;échantillons.</p><p class="sp-3d-uses">Utilise: <code>values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-cumulative.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(&quot;Cumulative 3D&quot;, variant=&quot;cumulative&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stacked</code></h3><p>Chaque classe empile un segment par catégorie de <code>color_groups</code>.</p><p class="sp-3d-uses">Utilise: <code>values, color_groups</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-stacked.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(
    &quot;Stacked 3D&quot;,
    variant=&quot;stacked&quot;,
    values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...],
    color_groups=[&quot;A&quot;, &quot;A&quot;, &quot;A&quot;, &quot;A&quot;, &quot;A&quot;, &quot;A&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>overlay</code></h3><p>Un second jeu d&#x27;échantillons (<code>overlay</code>) dessiné en rangée parallèle à côté du premier.</p><p class="sp-3d-uses">Utilise: <code>values, overlay, series_names</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-overlay.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(&quot;Overlay 3D&quot;, variant=&quot;overlay&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
<div class="sp-3d-card"><h3><code>step</code></h3><p>Seul le dessus de chaque classe en fine plaque, pour l&#x27;aspect d&#x27;escalier contouré.</p><p class="sp-3d-uses">Utilise: <code>values, bins</code></p><iframe class="sp-preview-frame" data-src="../../previews/histogram3d-step.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.histogram3d(&quot;Step 3D&quot;, variant=&quot;step&quot;, values=[2.1, 2.3, 2.7, 3.1, 3.4, 3.6, ...])</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="histogram3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
