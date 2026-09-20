# Candlestick Chart 3D

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

`sp.candlestick3d(title, labels=None, open=None, high=None, low=None, close=None, *, variant="basic", volume=None, orientation3d="iso", scene="default", theme="none", bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_candlestick3d_chart()`, `sp.candlestick_3d()`, `sp.candlestick3d_chart()`, `sp.ohlc3d()`.

## Description

`sp.candlestick3d()` is the 3D twin of `sp.candlestick()`: **every one of the 11 candlestick variants has a 3D form**, selected with the same `variant` keyword and fed with the same quotes. Candles, ticks, ranges, tracks, areas and volume rows are drawn by shared OHLC strategies, so geometry is never hand-written per variant. Colour encodes direction: green when the close is above the open, red when it is below.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic`, `filled` | `labels, open, high, low, close` | One filled body per bar between open and close, with a thin wick from low to high; green up, red down. |
| `hollow` | `empty`, `japanese`, `white_up` | `labels, open, high, low, close` | Same candles with the up bodies softened (hollow look) while down bodies stay solid. |
| `ohlc` | `western`, `bar`, `tick` | `labels, open, high, low, close` | Western OHLC bars: a stem from low to high with an open tick on the left and a close tick on the right. |
| `heikin` | `heikin_ashi`, `ha`, `smoothed` | `labels, open, high, low, close` | Heikin-Ashi candles: the quotes are smoothed by the same averaging as the 2D chart before being drawn. |
| `outlined` | `outline`, `stroke`, `wireframe` | `labels, open, high, low, close` | Slimmer bodies with heavier wicks, giving the outlined wireframe look. |
| `line` | `close`, `lineplot`, `trend` | `labels, open, high, low, close` | The close series as a stepped track: every segment spans from one close to the next. |
| `mountain` | `area`, `filled_area`, `shade` | `labels, open, high, low, close` | The close series as an area of columns rising from the price floor, coloured by height. |
| `range` | `hl`, `highlow`, `spread` | `labels, open, high, low, close` | One thick stem per bar from the low to the high, coloured by direction. |
| `volume` | `crypto`, `with_volume`, `trading` | `labels, open, high, low, close, volume` | Candles plus a second row of volume columns behind them, scaled to the traded peak. |
| `milestone` | `milestones`, `story`, `narrative`, `highlight_reel` | `labels, open, high, low, close` | Candles plus a marker above the highest high and below the lowest low. |
| `indicators` | `multi_ma`, `moving_averages`, `technical`, `grafana` | `labels, open, high, low, close` | Candles plus two moving-average tracks (short and long window) laid in front of them. |

## Data

`labels` name each bar; `open`, `high`, `low` and `close` are the quotes. The `volume` variant also reads `volume`. The older three-bar `candlestick3d` call keeps working.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="candlestick3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-cosmic.html"></iframe></div>
</div>

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One filled body per bar between open and close, with a thin wick from low to high; green up, red down.</p><p class="sp-3d-uses">Uses: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>hollow</code></h3><p>Same candles with the up bodies softened (hollow look) while down bodies stay solid.</p><p class="sp-3d-uses">Uses: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-hollow.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Hollow 3D&quot;,
    variant=&quot;hollow&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>ohlc</code></h3><p>Western OHLC bars: a stem from low to high with an open tick on the left and a close tick on the right.</p><p class="sp-3d-uses">Uses: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-ohlc.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Ohlc 3D&quot;,
    variant=&quot;ohlc&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>heikin</code></h3><p>Heikin-Ashi candles: the quotes are smoothed by the same averaging as the 2D chart before being drawn.</p><p class="sp-3d-uses">Uses: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-heikin.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Heikin 3D&quot;,
    variant=&quot;heikin&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outlined</code></h3><p>Slimmer bodies with heavier wicks, giving the outlined wireframe look.</p><p class="sp-3d-uses">Uses: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-outlined.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Outlined 3D&quot;,
    variant=&quot;outlined&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>line</code></h3><p>The close series as a stepped track: every segment spans from one close to the next.</p><p class="sp-3d-uses">Uses: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-line.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Line 3D&quot;,
    variant=&quot;line&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>mountain</code></h3><p>The close series as an area of columns rising from the price floor, coloured by height.</p><p class="sp-3d-uses">Uses: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-mountain.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Mountain 3D&quot;,
    variant=&quot;mountain&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>range</code></h3><p>One thick stem per bar from the low to the high, coloured by direction.</p><p class="sp-3d-uses">Uses: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-range.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Range 3D&quot;,
    variant=&quot;range&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>volume</code></h3><p>Candles plus a second row of volume columns behind them, scaled to the traded peak.</p><p class="sp-3d-uses">Uses: <code>labels, open, high, low, close, volume</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-volume.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Volume 3D&quot;,
    variant=&quot;volume&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    open=[100, 102, 105, 103, 108, 111, 109],
    high=[105, 107, 109, 110, 114, 116, 113],
    low=[99, 101, 103, 102, 107, 109, 106],
    close=[102, 105, 103, 108, 112, 110, 107],
    volume=[1200, 900, 1500, 700, 2100, 1800, 1000],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>milestone</code></h3><p>Candles plus a marker above the highest high and below the lowest low.</p><p class="sp-3d-uses">Uses: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-milestone.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Milestone 3D&quot;,
    variant=&quot;milestone&quot;,
    labels=[&quot;Jan 01&quot;, &quot;Jan 02&quot;, &quot;Jan 03&quot;, &quot;Jan 04&quot;, &quot;Jan 05&quot;, &quot;Jan 06&quot;, ...],
    open=[100.0, 99.94, 99.79, 101.92, 102.66, 100.35, ...],
    high=[100.86, 100.08, 102.73, 103.46, 103.69, 101.74, ...],
    low=[99.45, 99.29, 98.67, 101.22, 99.49, 100.08, ...],
    close=[99.94, 99.79, 101.92, 102.66, 100.35, 101.5, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>indicators</code></h3><p>Candles plus two moving-average tracks (short and long window) laid in front of them.</p><p class="sp-3d-uses">Uses: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-indicators.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Indicators 3D&quot;,
    variant=&quot;indicators&quot;,
    labels=[&quot;Jan 01&quot;, &quot;Jan 02&quot;, &quot;Jan 03&quot;, &quot;Jan 04&quot;, &quot;Jan 05&quot;, &quot;Jan 06&quot;, ...],
    open=[200.0, 203.08, 205.03, 203.88, 203.87, 204.86, ...],
    high=[205.24, 207.52, 206.12, 204.83, 206.19, 208.82, ...],
    low=[199.5, 201.64, 202.01, 202.61, 201.46, 203.06, ...],
    close=[203.08, 205.03, 203.88, 203.87, 204.86, 208.5, ...],
    volume=[1318199, 1096044, 1601918, 1151082, 1566597, 1886037, ...],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="candlestick3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.candlestick3d(title, labels=None, open=None, high=None, low=None, close=None, *, variant="basic", volume=None, orientation3d="iso", scene="default", theme="none", bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_candlestick3d_chart()`, `sp.candlestick_3d()`, `sp.candlestick3d_chart()`, `sp.ohlc3d()`.

<h2>Description</h2>

`sp.candlestick3d()` est le jumeau 3D de `sp.candlestick()` : **chacune des 11 variantes de chandelier a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes cotations. Chandeliers, ticks, plages, courbes, aires et rangées de volume sont dessinés par des stratégies OHLC partagées : la géométrie n'est jamais écrite à la main par variante. La couleur encode la direction : vert quand la clôture dépasse l'ouverture, rouge sinon.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic`, `filled` | `labels, open, high, low, close` | Un corps plein par barre entre ouverture et clôture, avec une fine mèche du plus bas au plus haut ; vert à la hausse, rouge à la baisse. |
| `hollow` | `empty`, `japanese`, `white_up` | `labels, open, high, low, close` | Mêmes chandeliers avec les corps haussiers adoucis (aspect creux) tandis que les corps baissiers restent pleins. |
| `ohlc` | `western`, `bar`, `tick` | `labels, open, high, low, close` | Barres OHLC occidentales : une tige du plus bas au plus haut avec un tick d'ouverture à gauche et un tick de clôture à droite. |
| `heikin` | `heikin_ashi`, `ha`, `smoothed` | `labels, open, high, low, close` | Chandeliers Heikin-Ashi : les cotations sont lissées par la même moyenne que le graphique 2D avant d'être dessinées. |
| `outlined` | `outline`, `stroke`, `wireframe` | `labels, open, high, low, close` | Corps plus fins et mèches plus épaisses, pour l'aspect filaire contouré. |
| `line` | `close`, `lineplot`, `trend` | `labels, open, high, low, close` | La série de clôture en escalier : chaque segment va d'une clôture à la suivante. |
| `mountain` | `area`, `filled_area`, `shade` | `labels, open, high, low, close` | La série de clôture en aire de colonnes qui montent depuis le plancher de prix, colorées selon la hauteur. |
| `range` | `hl`, `highlow`, `spread` | `labels, open, high, low, close` | Une tige épaisse par barre du plus bas au plus haut, colorée selon la direction. |
| `volume` | `crypto`, `with_volume`, `trading` | `labels, open, high, low, close, volume` | Chandeliers plus une seconde rangée de colonnes de volume derrière eux, mises à l'échelle du pic échangé. |
| `milestone` | `milestones`, `story`, `narrative`, `highlight_reel` | `labels, open, high, low, close` | Chandeliers plus un repère au-dessus du plus haut sommet et sous le plus bas creux. |
| `indicators` | `multi_ma`, `moving_averages`, `technical`, `grafana` | `labels, open, high, low, close` | Chandeliers plus deux courbes de moyenne mobile (fenêtre courte et longue) posées devant eux. |

<h2>Données</h2>

`labels` nomme chaque barre ; `open`, `high`, `low` et `close` sont les cotations. La variante `volume` lit aussi `volume`. L'ancien appel à trois barres de `candlestick3d` continue de fonctionner.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="candlestick3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Un corps plein par barre entre ouverture et clôture, avec une fine mèche du plus bas au plus haut ; vert à la hausse, rouge à la baisse.</p><p class="sp-3d-uses">Utilise: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>hollow</code></h3><p>Mêmes chandeliers avec les corps haussiers adoucis (aspect creux) tandis que les corps baissiers restent pleins.</p><p class="sp-3d-uses">Utilise: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-hollow.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Hollow 3D&quot;,
    variant=&quot;hollow&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>ohlc</code></h3><p>Barres OHLC occidentales : une tige du plus bas au plus haut avec un tick d&#x27;ouverture à gauche et un tick de clôture à droite.</p><p class="sp-3d-uses">Utilise: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-ohlc.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Ohlc 3D&quot;,
    variant=&quot;ohlc&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>heikin</code></h3><p>Chandeliers Heikin-Ashi : les cotations sont lissées par la même moyenne que le graphique 2D avant d&#x27;être dessinées.</p><p class="sp-3d-uses">Utilise: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-heikin.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Heikin 3D&quot;,
    variant=&quot;heikin&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outlined</code></h3><p>Corps plus fins et mèches plus épaisses, pour l&#x27;aspect filaire contouré.</p><p class="sp-3d-uses">Utilise: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-outlined.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Outlined 3D&quot;,
    variant=&quot;outlined&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>line</code></h3><p>La série de clôture en escalier : chaque segment va d&#x27;une clôture à la suivante.</p><p class="sp-3d-uses">Utilise: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-line.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Line 3D&quot;,
    variant=&quot;line&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>mountain</code></h3><p>La série de clôture en aire de colonnes qui montent depuis le plancher de prix, colorées selon la hauteur.</p><p class="sp-3d-uses">Utilise: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-mountain.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Mountain 3D&quot;,
    variant=&quot;mountain&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>range</code></h3><p>Une tige épaisse par barre du plus bas au plus haut, colorée selon la direction.</p><p class="sp-3d-uses">Utilise: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-range.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Range 3D&quot;,
    variant=&quot;range&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;],
    open=[100, 102, 105, 103, 108],
    high=[105, 107, 109, 110, 114],
    low=[99, 101, 103, 102, 107],
    close=[102, 105, 103, 108, 112],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>volume</code></h3><p>Chandeliers plus une seconde rangée de colonnes de volume derrière eux, mises à l&#x27;échelle du pic échangé.</p><p class="sp-3d-uses">Utilise: <code>labels, open, high, low, close, volume</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-volume.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Volume 3D&quot;,
    variant=&quot;volume&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    open=[100, 102, 105, 103, 108, 111, 109],
    high=[105, 107, 109, 110, 114, 116, 113],
    low=[99, 101, 103, 102, 107, 109, 106],
    close=[102, 105, 103, 108, 112, 110, 107],
    volume=[1200, 900, 1500, 700, 2100, 1800, 1000],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>milestone</code></h3><p>Chandeliers plus un repère au-dessus du plus haut sommet et sous le plus bas creux.</p><p class="sp-3d-uses">Utilise: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-milestone.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Milestone 3D&quot;,
    variant=&quot;milestone&quot;,
    labels=[&quot;Jan 01&quot;, &quot;Jan 02&quot;, &quot;Jan 03&quot;, &quot;Jan 04&quot;, &quot;Jan 05&quot;, &quot;Jan 06&quot;, ...],
    open=[100.0, 99.94, 99.79, 101.92, 102.66, 100.35, ...],
    high=[100.86, 100.08, 102.73, 103.46, 103.69, 101.74, ...],
    low=[99.45, 99.29, 98.67, 101.22, 99.49, 100.08, ...],
    close=[99.94, 99.79, 101.92, 102.66, 100.35, 101.5, ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>indicators</code></h3><p>Chandeliers plus deux courbes de moyenne mobile (fenêtre courte et longue) posées devant eux.</p><p class="sp-3d-uses">Utilise: <code>labels, open, high, low, close</code></p><iframe class="sp-preview-frame" data-src="../../previews/candlestick3d-indicators.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.candlestick3d(
    &quot;Indicators 3D&quot;,
    variant=&quot;indicators&quot;,
    labels=[&quot;Jan 01&quot;, &quot;Jan 02&quot;, &quot;Jan 03&quot;, &quot;Jan 04&quot;, &quot;Jan 05&quot;, &quot;Jan 06&quot;, ...],
    open=[200.0, 203.08, 205.03, 203.88, 203.87, 204.86, ...],
    high=[205.24, 207.52, 206.12, 204.83, 206.19, 208.82, ...],
    low=[199.5, 201.64, 202.01, 202.61, 201.46, 203.06, ...],
    close=[203.08, 205.03, 203.88, 203.87, 204.86, 208.5, ...],
    volume=[1318199, 1096044, 1601918, 1151082, 1566597, 1886037, ...],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="candlestick3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
