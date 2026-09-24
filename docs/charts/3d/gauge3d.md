# Gauge Chart 3D

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:340px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>

## Signature

`sp.gauge3d(title, value=0, min_val=0, max_val=100, label="", *, variant="basic", comparison=0, history=None, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_gauge3d_chart()`, `sp.gauge_3d()`, `sp.gauge3d_chart()`, `sp.gauge3d_family()`, `sp.speedometer3d()`.

## Description

`sp.gauge3d()` is the 3D twin of `sp.gauge()`: **every one of the 9 gauge variants has a 3D form**, selected with the same `variant` keyword and fed with the same value and range. A ring of small columns sweeps the dial; the columns up to the value rise taller and take the tone of the threshold band the value falls in (green through amber to red by default), while the rest of the ring stays low as a track. Needle, ticks, halo and the inner comparison ring come from a shared dial strategy, so geometry is never hand-written per variant.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `half`, `classic` | `value, min_val, max_val, label` | A half-circle dial with background threshold bands, a filled ring up to the value and a needle pointing at it. |
| `radial` | `donut`, `ring`, `full` | `value, min_val, max_val, label` | A full donut ring filled clockwise from the top up to the value's fraction of the range. |
| `arc270` | `three_quarter`, `arc`, `wide` | `value, min_val, max_val, label` | A three-quarter (270°) sweep with the same threshold bands as basic, without a needle. |
| `sleek` | `minimal`, `clean`, `flat` | `value, min_val, max_val, label` | A minimal half-circle: a plain track and a single-tone filled ring, no bands or needle. |
| `tick` | `tickmarks`, `scaled`, `ruler` | `value, min_val, max_val, label` | A half-circle with graduation ticks around the rim, longer every fifth mark. |
| `segmented` | `battery`, `signal`, `chunked` | `value, min_val, max_val, label` | The fill broken into discrete chunky segments, like a battery or signal-strength meter. |
| `glow` | `neon`, `halo`, `luminous` | `value, min_val, max_val, label` | The filled ring sits on a wide, flat halo that makes it stand out. |
| `concentric` | `rings`, `target`, `dual` | `value, min_val, max_val, comparison` | Two rings: the value on the outer ring, the comparison value on an inner ring. |
| `sparkline` | `trend`, `history`, `with_trend` | `value, min_val, max_val, history` | The dial plus a small ribbon tracing the value's recent history. |

## Data

`value`, `min_val` and `max_val` set the needle position; `label` names the gauge in the tooltip. `comparison` draws a second mark used by `concentric`, and `history` feeds the trailing ribbon of `sparkline`.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="gauge3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Parameters

<div data-sp-registry-table="options" data-family="gauge3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.gauge3d(title, value=0, min_val=0, max_val=100, label="", *, variant="basic", comparison=0, history=None, orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_gauge3d_chart()`, `sp.gauge_3d()`, `sp.gauge3d_chart()`, `sp.gauge3d_family()`, `sp.speedometer3d()`.

<h2>Description</h2>

`sp.gauge3d()` est le jumeau 3D de `sp.gauge()` : **chacune des 9 variantes de jauge a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes valeur et plage. Un anneau de petites colonnes balaie le cadran ; les colonnes jusqu'à la valeur s'élèvent plus haut et prennent la teinte de la bande de seuil où tombe la valeur (vert puis ambre puis rouge par défaut), tandis que le reste de l'anneau reste bas comme piste. Aiguille, graduations, halo et anneau de comparaison intérieur viennent d'une stratégie de cadran partagée : la géométrie n'est jamais écrite à la main par variante.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `half`, `classic` | `value, min_val, max_val, label` | Un cadran demi-cercle avec des bandes de seuil en fond, un anneau rempli jusqu'à la valeur et une aiguille qui la pointe. |
| `radial` | `donut`, `ring`, `full` | `value, min_val, max_val, label` | Un anneau donut complet rempli depuis le haut, dans le sens horaire, jusqu'à la fraction de la valeur dans la plage. |
| `arc270` | `three_quarter`, `arc`, `wide` | `value, min_val, max_val, label` | Un balayage aux trois quarts (270°) avec les mêmes bandes de seuil que basic, sans aiguille. |
| `sleek` | `minimal`, `clean`, `flat` | `value, min_val, max_val, label` | Un demi-cercle minimal : une piste unie et un anneau rempli d'une seule teinte, sans bandes ni aiguille. |
| `tick` | `tickmarks`, `scaled`, `ruler` | `value, min_val, max_val, label` | Un demi-cercle avec des graduations sur le pourtour, plus longues tous les cinq traits. |
| `segmented` | `battery`, `signal`, `chunked` | `value, min_val, max_val, label` | Le remplissage découpé en segments discrets et épais, comme une jauge de batterie ou de signal. |
| `glow` | `neon`, `halo`, `luminous` | `value, min_val, max_val, label` | L'anneau rempli repose sur un large halo plat qui le fait ressortir. |
| `concentric` | `rings`, `target`, `dual` | `value, min_val, max_val, comparison` | Deux anneaux : la valeur sur l'anneau extérieur, la valeur de comparaison sur un anneau intérieur. |
| `sparkline` | `trend`, `history`, `with_trend` | `value, min_val, max_val, history` | Le cadran plus un petit ruban qui trace l'historique récent de la valeur. |

<h2>Données</h2>

`value`, `min_val` et `max_val` fixent la position de l'aiguille ; `label` nomme la jauge dans l'infobulle. `comparison` trace un second repère utilisé par `concentric`, et `history` alimente le ruban de traîne de `sparkline`.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="gauge3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/gauge3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="gauge3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
