# Lollipop Chart 3D

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

`sp.lollipop3d(title, labels=None, values=None, *, variant="basic", color_groups=None, sort_order="none", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_lollipop3d_chart()`, `sp.lollipop_3d()`, `sp.lollipop3d_chart()`, `sp.lollipop3d_family()`, `sp.lollipops3d()`.

## Description

`sp.lollipop3d()` is the 3D twin of `sp.lollipop()`: **every one of the 9 lollipop variants has a 3D form**, selected with the same `variant` keyword and fed with the same labels and values. Every lollipop is a thin stem topped by a head; the head is sized so it looks cubic once the zone is fitted, and rows, rings and panels come from the shared column layouts, so geometry is never hand-written per variant. Long inputs are pooled to a budget (`max_points`), so hundreds of thousands of values stay interactive.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic`, `vertical` | `labels, values` | One thin stem per value topped by a head cube; the height is the value. |
| `cleveland` | `horizontal`, `h`, `row` | `labels, values` | The basic layout turned a quarter-turn: the categories run along the depth axis. |
| `diverging` | `div`, `signed`, `delta` | `labels, values` | Stems grow up or down from the mean instead of zero, toned green above it and red below. |
| `circular` | `polar`, `radial`, `round` | `labels, values` | The stems stand on a ring, one per category, like a radial dot plot. |
| `office` | `grouped`, `season`, `panel` | `labels, values, color_groups` | Categories laid out panel by panel with a gap between `color_groups`, one colour per group (a season, a team). |
| `conditional_color` | `conditional`, `threshold_color` | `labels, values` | Horizontal stems coloured by sign: one colour for values at or above zero, another for negative ones. |
| `trend` | `colormap`, `arrow`, `annotated` | `labels, values` | Heads toned along the colormap by value and joined by a sloped ribbon that follows the trend. |
| `custom` | `diamond`, `styled` | `labels, values` | The head is a faceted diamond, a rhombic prism cut from two sloped wedges, instead of a cube. |
| `duel` | `head_to_head`, `versus`, `rivalry`, `radial_diverging` | `labels, values, color_groups` | Two groups face each other on a ring: the first rises above the ring plane, the second hangs below it. |

## Data

`labels` name the lollipops and `values` give their height; `color_groups` separates and colours the panels of `office` and the two sides of `duel`. Passing `x`, `y` and `z` together keeps the historical call that draws one lollipop per spatial point.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="lollipop3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One thin stem per value topped by a head cube; the height is the value.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, 38, 17, 42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>cleveland</code></h3><p>The basic layout turned a quarter-turn: the categories run along the depth axis.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-cleveland.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Cleveland 3D&quot;,
    variant=&quot;cleveland&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, 38, 17, 42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>diverging</code></h3><p>Stems grow up or down from the mean instead of zero, toned green above it and red below.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-diverging.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Diverging 3D&quot;,
    variant=&quot;diverging&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, -38, 17, -42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>circular</code></h3><p>The stems stand on a ring, one per category, like a radial dot plot.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-circular.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Circular 3D&quot;,
    variant=&quot;circular&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, 38, 17, 42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>office</code></h3><p>Categories laid out panel by panel with a gap between <code>color_groups</code>, one colour per group (a season, a team).</p><p class="sp-3d-uses">Uses: <code>labels, values, color_groups</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-office.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Office 3D&quot;,
    variant=&quot;office&quot;,
    y_label=&quot;rating&quot;,
    labels=[&quot;S1E1&quot;, &quot;S1E2&quot;, &quot;S1E3&quot;, &quot;S1E4&quot;, &quot;S2E1&quot;, &quot;S2E2&quot;, ...],
    values=[7.5, 8.3, 7.9, 8.1, 8.4, 8.7, ...],
    color_groups=[&quot;S1&quot;, &quot;S1&quot;, &quot;S1&quot;, &quot;S1&quot;, &quot;S2&quot;, &quot;S2&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>conditional_color</code></h3><p>Horizontal stems coloured by sign: one colour for values at or above zero, another for negative ones.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-conditional_color.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Conditional Color 3D&quot;,
    variant=&quot;conditional_color&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;, &quot;Zeta&quot;],
    values=[24, -38, 17, -12, 29, -6],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>trend</code></h3><p>Heads toned along the colormap by value and joined by a sloped ribbon that follows the trend.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-trend.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Trend 3D&quot;,
    variant=&quot;trend&quot;,
    labels=[&quot;1951&quot;, &quot;1961&quot;, &quot;1971&quot;, &quot;1981&quot;, &quot;1991&quot;, &quot;2001&quot;, &quot;2011&quot;, &quot;2021&quot;],
    values=[-0.3, -0.15, 0.05, 0.22, 0.38, 0.61, 0.85, 1.1],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>custom</code></h3><p>The head is a faceted diamond, a rhombic prism cut from two sloped wedges, instead of a cube.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-custom.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Custom 3D&quot;,
    variant=&quot;custom&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, 38, 17, 42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>duel</code></h3><p>Two groups face each other on a ring: the first rises above the ring plane, the second hangs below it.</p><p class="sp-3d-uses">Uses: <code>labels, values, color_groups</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-duel.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Duel 3D&quot;,
    variant=&quot;duel&quot;,
    labels=[&quot;1929&quot;, &quot;1929&quot;, &quot;1929&quot;, &quot;1929&quot;, &quot;1929&quot;, &quot;1929&quot;, ...],
    values=[1, 27, 28, 88, 73, 81, ...],
    color_groups=[&quot;Barcelona&quot;, &quot;Real Madrid&quot;, &quot;Real Madrid&quot;, &quot;Barcelona&quot;, &quot;Real Madrid&quot;, &quot;Barcelona&quot;, ...],
    palette=[3316734, 15547189],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="lollipop3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.lollipop3d(title, labels=None, values=None, *, variant="basic", color_groups=None, sort_order="none", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_lollipop3d_chart()`, `sp.lollipop_3d()`, `sp.lollipop3d_chart()`, `sp.lollipop3d_family()`, `sp.lollipops3d()`.

<h2>Description</h2>

`sp.lollipop3d()` est le jumeau 3D de `sp.lollipop()` : **chacune des 9 variantes de sucette a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes libellés et valeurs. Chaque sucette est une fine tige coiffée d'une tête ; la tête est dimensionnée pour paraître cubique une fois la zone ajustée, et les rangées, anneaux et panneaux viennent des dispositions de colonnes partagées : la géométrie n'est jamais écrite à la main par variante. Les longues entrées sont regroupées selon un budget (`max_points`) : des centaines de milliers de valeurs restent fluides.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic`, `vertical` | `labels, values` | Une fine tige par valeur coiffée d'un cube ; la hauteur est la valeur. |
| `cleveland` | `horizontal`, `h`, `row` | `labels, values` | La disposition de base tournée d'un quart de tour : les catégories courent le long de l'axe de profondeur. |
| `diverging` | `div`, `signed`, `delta` | `labels, values` | Les tiges montent ou descendent depuis la moyenne au lieu de zéro, vertes au-dessus et rouges en dessous. |
| `circular` | `polar`, `radial`, `round` | `labels, values` | Les tiges se dressent sur un anneau, une par catégorie, comme un nuage de points radial. |
| `office` | `grouped`, `season`, `panel` | `labels, values, color_groups` | Catégories disposées panneau par panneau avec un espace entre les `color_groups`, une couleur par groupe (une saison, une équipe). |
| `conditional_color` | `conditional`, `threshold_color` | `labels, values` | Tiges horizontales colorées selon le signe : une couleur pour les valeurs supérieures ou égales à zéro, une autre pour les négatives. |
| `trend` | `colormap`, `arrow`, `annotated` | `labels, values` | Têtes teintées le long de la palette selon la valeur et reliées par un ruban incliné qui suit la tendance. |
| `custom` | `diamond`, `styled` | `labels, values` | La tête est un diamant à facettes, un prisme rhombique taillé dans deux coins inclinés, au lieu d'un cube. |
| `duel` | `head_to_head`, `versus`, `rivalry`, `radial_diverging` | `labels, values, color_groups` | Deux groupes s'affrontent sur un anneau : le premier s'élève au-dessus du plan de l'anneau, le second pend en dessous. |

<h2>Données</h2>

`labels` nomme les sucettes et `values` donne leur hauteur ; `color_groups` sépare et colore les panneaux de `office` et les deux côtés de `duel`. Passer `x`, `y` et `z` ensemble conserve l'appel historique qui dessine une sucette par point de l'espace.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="lollipop3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une fine tige par valeur coiffée d&#x27;un cube ; la hauteur est la valeur.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, 38, 17, 42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>cleveland</code></h3><p>La disposition de base tournée d&#x27;un quart de tour : les catégories courent le long de l&#x27;axe de profondeur.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-cleveland.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Cleveland 3D&quot;,
    variant=&quot;cleveland&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, 38, 17, 42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>diverging</code></h3><p>Les tiges montent ou descendent depuis la moyenne au lieu de zéro, vertes au-dessus et rouges en dessous.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-diverging.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Diverging 3D&quot;,
    variant=&quot;diverging&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, -38, 17, -42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>circular</code></h3><p>Les tiges se dressent sur un anneau, une par catégorie, comme un nuage de points radial.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-circular.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Circular 3D&quot;,
    variant=&quot;circular&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, 38, 17, 42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>office</code></h3><p>Catégories disposées panneau par panneau avec un espace entre les <code>color_groups</code>, une couleur par groupe (une saison, une équipe).</p><p class="sp-3d-uses">Utilise: <code>labels, values, color_groups</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-office.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Office 3D&quot;,
    variant=&quot;office&quot;,
    y_label=&quot;rating&quot;,
    labels=[&quot;S1E1&quot;, &quot;S1E2&quot;, &quot;S1E3&quot;, &quot;S1E4&quot;, &quot;S2E1&quot;, &quot;S2E2&quot;, ...],
    values=[7.5, 8.3, 7.9, 8.1, 8.4, 8.7, ...],
    color_groups=[&quot;S1&quot;, &quot;S1&quot;, &quot;S1&quot;, &quot;S1&quot;, &quot;S2&quot;, &quot;S2&quot;, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>conditional_color</code></h3><p>Tiges horizontales colorées selon le signe : une couleur pour les valeurs supérieures ou égales à zéro, une autre pour les négatives.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-conditional_color.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Conditional Color 3D&quot;,
    variant=&quot;conditional_color&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;, &quot;Zeta&quot;],
    values=[24, -38, 17, -12, 29, -6],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>trend</code></h3><p>Têtes teintées le long de la palette selon la valeur et reliées par un ruban incliné qui suit la tendance.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-trend.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Trend 3D&quot;,
    variant=&quot;trend&quot;,
    labels=[&quot;1951&quot;, &quot;1961&quot;, &quot;1971&quot;, &quot;1981&quot;, &quot;1991&quot;, &quot;2001&quot;, &quot;2011&quot;, &quot;2021&quot;],
    values=[-0.3, -0.15, 0.05, 0.22, 0.38, 0.61, 0.85, 1.1],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>custom</code></h3><p>La tête est un diamant à facettes, un prisme rhombique taillé dans deux coins inclinés, au lieu d&#x27;un cube.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-custom.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Custom 3D&quot;,
    variant=&quot;custom&quot;,
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
    values=[24, 38, 17, 42, 29],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>duel</code></h3><p>Deux groupes s&#x27;affrontent sur un anneau : le premier s&#x27;élève au-dessus du plan de l&#x27;anneau, le second pend en dessous.</p><p class="sp-3d-uses">Utilise: <code>labels, values, color_groups</code></p><iframe class="sp-preview-frame" data-src="../../previews/lollipop3d-duel.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.lollipop3d(
    &quot;Duel 3D&quot;,
    variant=&quot;duel&quot;,
    labels=[&quot;1929&quot;, &quot;1929&quot;, &quot;1929&quot;, &quot;1929&quot;, &quot;1929&quot;, &quot;1929&quot;, ...],
    values=[1, 27, 28, 88, 73, 81, ...],
    color_groups=[&quot;Barcelona&quot;, &quot;Real Madrid&quot;, &quot;Real Madrid&quot;, &quot;Barcelona&quot;, &quot;Real Madrid&quot;, &quot;Barcelona&quot;, ...],
    palette=[3316734, 15547189],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="lollipop3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
