# Waterfall Chart 3D

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

`sp.waterfall3d(title, labels=None, values=None, *, variant="basic", sort_order="none", orientation3d="iso", scene="default", theme="none", bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_waterfall3d_chart()`, `sp.waterfall_3d()`, `sp.waterfall3d_chart()`.

## Description

`sp.waterfall3d()` is the 3D twin of `sp.waterfall()`: **every one of the 7 waterfall variants has a 3D form**, selected with the same `variant` keyword and fed with the same labels and deltas. Floating bars, stems, arrow tips and running-total tracks come from shared step strategies, so geometry is never hand-written per variant. Colour encodes the step: green for a rise, red for a fall, yellow for a total.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic`, `bars` | `labels, values` | One floating bar per step between the running total before and after it; totals stand from the floor. |
| `stepped` | `step`, `staircase`, `stairs` | `labels, values` | Bars widened until they touch, so the steps read as a continuous staircase. |
| `lollipop` | `stick`, `popsicle`, `lolly` | `labels, values` | A thin stem across each step topped by a head cube at the new running total. |
| `arrowed` | `arrow`, `directional`, `tipped` | `labels, values` | Floating bars with a small tip beyond the new total, pointing up for a rise and down for a fall. |
| `delta` | `percent`, `annotated`, `pct` | `labels, values` | Tones graded by the size of the change, so big moves stand out from small ones. |
| `horizontal` | `rows`, `sideways`, `h` | `labels, values` | The basic layout turned a quarter-turn: steps run along the depth axis. |
| `trend` | `running_total`, `cumulative_line`, `with_trend` | `labels, values` | Floating bars plus a running-total track laid beside them. |

## Data

`labels` name each step and `values` are the deltas. A label containing `total`, `net` or `final` is drawn as a total from the floor to the running sum, exactly like the 2D chart.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="waterfall3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-cosmic.html"></iframe></div>
</div>

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One floating bar per step between the running total before and after it; totals stand from the floor.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 155],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stepped</code></h3><p>Bars widened until they touch, so the steps read as a continuous staircase.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-stepped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Stepped 3D&quot;,
    variant=&quot;stepped&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 155],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>lollipop</code></h3><p>A thin stem across each step topped by a head cube at the new running total.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-lollipop.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Lollipop 3D&quot;,
    variant=&quot;lollipop&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 155],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>arrowed</code></h3><p>Floating bars with a small tip beyond the new total, pointing up for a rise and down for a fall.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-arrowed.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Arrowed 3D&quot;,
    variant=&quot;arrowed&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 155],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>delta</code></h3><p>Tones graded by the size of the change, so big moves stand out from small ones.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-delta.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Delta 3D&quot;,
    variant=&quot;delta&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 155],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>horizontal</code></h3><p>The basic layout turned a quarter-turn: steps run along the depth axis.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-horizontal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Horizontal 3D&quot;,
    variant=&quot;horizontal&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 155],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>trend</code></h3><p>Floating bars plus a running-total track laid beside them.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-trend.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Trend 3D&quot;,
    variant=&quot;trend&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 12, 167],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="waterfall3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.waterfall3d(title, labels=None, values=None, *, variant="basic", sort_order="none", orientation3d="iso", scene="default", theme="none", bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_waterfall3d_chart()`, `sp.waterfall_3d()`, `sp.waterfall3d_chart()`.

<h2>Description</h2>

`sp.waterfall3d()` est le jumeau 3D de `sp.waterfall()` : **chacune des 7 variantes de cascade a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes libellés et variations. Barres flottantes, tiges, pointes de flèche et courbes de cumul viennent de stratégies d'étapes partagées : la géométrie n'est jamais écrite à la main par variante. La couleur encode l'étape : vert pour une hausse, rouge pour une baisse, jaune pour un total.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic`, `bars` | `labels, values` | Une barre flottante par étape entre le cumul avant et après elle ; les totaux partent du plancher. |
| `stepped` | `step`, `staircase`, `stairs` | `labels, values` | Barres élargies jusqu'à se toucher : les étapes se lisent comme un escalier continu. |
| `lollipop` | `stick`, `popsicle`, `lolly` | `labels, values` | Une fine tige à travers chaque étape, coiffée d'un cube au nouveau cumul. |
| `arrowed` | `arrow`, `directional`, `tipped` | `labels, values` | Barres flottantes avec une petite pointe au-delà du nouveau total, vers le haut pour une hausse et vers le bas pour une baisse. |
| `delta` | `percent`, `annotated`, `pct` | `labels, values` | Teintes graduées selon l'ampleur du changement : les gros mouvements se détachent des petits. |
| `horizontal` | `rows`, `sideways`, `h` | `labels, values` | La disposition de base tournée d'un quart de tour : les étapes courent le long de l'axe de profondeur. |
| `trend` | `running_total`, `cumulative_line`, `with_trend` | `labels, values` | Barres flottantes plus une courbe de cumul posée à côté. |

<h2>Données</h2>

`labels` nomme chaque étape et `values` sont les variations. Un libellé contenant `total`, `net` ou `final` est dessiné comme un total du plancher jusqu'à la somme cumulée, exactement comme le graphique 2D.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="waterfall3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une barre flottante par étape entre le cumul avant et après elle ; les totaux partent du plancher.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 155],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>stepped</code></h3><p>Barres élargies jusqu&#x27;à se toucher : les étapes se lisent comme un escalier continu.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-stepped.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Stepped 3D&quot;,
    variant=&quot;stepped&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 155],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>lollipop</code></h3><p>Une fine tige à travers chaque étape, coiffée d&#x27;un cube au nouveau cumul.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-lollipop.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Lollipop 3D&quot;,
    variant=&quot;lollipop&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 155],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>arrowed</code></h3><p>Barres flottantes avec une petite pointe au-delà du nouveau total, vers le haut pour une hausse et vers le bas pour une baisse.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-arrowed.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Arrowed 3D&quot;,
    variant=&quot;arrowed&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 155],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>delta</code></h3><p>Teintes graduées selon l&#x27;ampleur du changement : les gros mouvements se détachent des petits.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-delta.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Delta 3D&quot;,
    variant=&quot;delta&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 155],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>horizontal</code></h3><p>La disposition de base tournée d&#x27;un quart de tour : les étapes courent le long de l&#x27;axe de profondeur.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-horizontal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Horizontal 3D&quot;,
    variant=&quot;horizontal&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 155],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>trend</code></h3><p>Barres flottantes plus une courbe de cumul posée à côté.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/waterfall3d-trend.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.waterfall3d(
    &quot;Trend 3D&quot;,
    variant=&quot;trend&quot;,
    labels=[&quot;Start&quot;, &quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;, &quot;End&quot;],
    values=[100, 30, -15, 40, 12, 167],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="waterfall3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
