# Candlestick Chart 3D

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:340px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
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

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

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

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="candlestick3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
