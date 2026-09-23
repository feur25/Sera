# SPLOM Chart 3D

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

`sp.splom3d(title, axes=None, series=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_splom3d_chart()`, `sp.splom_3d()`, `sp.splom3d_chart()`, `sp.splom3d_family()`, `sp.sploms3d()`.

## Description

`sp.splom3d()` is the 3D twin of `sp.splom()`: **every one of the 4 scatterplot-matrix variants has a 3D form**, selected with the same `variant` keyword and fed with the same rows. Every axis pair becomes its own small scatter cell laid out on a grid, up to 8 axes at once; long tables are sampled down so every cell stays legible.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic`, `dots` | `axes, series` | One small scatter cell per axis pair, laid out on a grid. |
| `correlation` | `corr`, `heat`, `shaded` | `axes, series` | The same grid of cells, each toned by its own Pearson correlation coefficient. |
| `density` | `alpha`, `overplot`, `cloud` | `axes, series` | The same grid of cells, read as an overplotted cloud rather than individually styled points. |
| `regression` | `trend`, `fit`, `lm` | `axes, series` | The same grid of cells, each with a linear best-fit line laid across its points. |

## Data

`axes` names each variable and `series` gives one row per sample, one value per axis in the same order.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="splom3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>One small scatter cell per axis pair, laid out on a grid.</p><p class="sp-3d-uses">Uses: <code>axes, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/splom3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.splom3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;],
    series=[[80, 65, 70], [60, 80, 55], [40, 70, 90], [90, 40, 60], [55, 85, 45], [70, 55, 80]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>correlation</code></h3><p>The same grid of cells, each toned by its own Pearson correlation coefficient.</p><p class="sp-3d-uses">Uses: <code>axes, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/splom3d-correlation.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.splom3d(
    &quot;Correlation 3D&quot;,
    variant=&quot;correlation&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;],
    series=[[80, 65, 70], [60, 80, 55], [40, 70, 90], [90, 40, 60], [55, 85, 45], [70, 55, 80]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>density</code></h3><p>The same grid of cells, read as an overplotted cloud rather than individually styled points.</p><p class="sp-3d-uses">Uses: <code>axes, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/splom3d-density.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.splom3d(
    &quot;Density 3D&quot;,
    variant=&quot;density&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;],
    series=[[80, 65, 70], [60, 80, 55], [40, 70, 90], [90, 40, 60], [55, 85, 45], [70, 55, 80], ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>regression</code></h3><p>The same grid of cells, each with a linear best-fit line laid across its points.</p><p class="sp-3d-uses">Uses: <code>axes, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/splom3d-regression.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.splom3d(
    &quot;Regression 3D&quot;,
    variant=&quot;regression&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;],
    series=[[80, 65, 70], [60, 80, 55], [40, 70, 90], [90, 40, 60], [55, 85, 45], [70, 55, 80]],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="splom3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.splom3d(title, axes=None, series=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_splom3d_chart()`, `sp.splom_3d()`, `sp.splom3d_chart()`, `sp.splom3d_family()`, `sp.sploms3d()`.

<h2>Description</h2>

`sp.splom3d()` est le jumeau 3D de `sp.splom()` : **chacune des 4 variantes de matrice de nuages de points a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes lignes. Chaque paire d'axes devient sa propre petite cellule de nuage de points posée sur une grille, jusqu'à 8 axes à la fois ; les longs tableaux sont échantillonnés pour que chaque cellule reste lisible.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic`, `dots` | `axes, series` | Une petite cellule de nuage de points par paire d'axes, posée sur une grille. |
| `correlation` | `corr`, `heat`, `shaded` | `axes, series` | La même grille de cellules, chacune teintée selon son propre coefficient de corrélation de Pearson. |
| `density` | `alpha`, `overplot`, `cloud` | `axes, series` | La même grille de cellules, lue comme un nuage superposé plutôt que des points stylés individuellement. |
| `regression` | `trend`, `fit`, `lm` | `axes, series` | La même grille de cellules, chacune avec une droite de régression tracée à travers ses points. |

<h2>Données</h2>

`axes` nomme chaque variable et `series` donne une ligne par échantillon, une valeur par axe dans le même ordre.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="splom3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/splom3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Une petite cellule de nuage de points par paire d&#x27;axes, posée sur une grille.</p><p class="sp-3d-uses">Utilise: <code>axes, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/splom3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.splom3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;],
    series=[[80, 65, 70], [60, 80, 55], [40, 70, 90], [90, 40, 60], [55, 85, 45], [70, 55, 80]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>correlation</code></h3><p>La même grille de cellules, chacune teintée selon son propre coefficient de corrélation de Pearson.</p><p class="sp-3d-uses">Utilise: <code>axes, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/splom3d-correlation.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.splom3d(
    &quot;Correlation 3D&quot;,
    variant=&quot;correlation&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;],
    series=[[80, 65, 70], [60, 80, 55], [40, 70, 90], [90, 40, 60], [55, 85, 45], [70, 55, 80]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>density</code></h3><p>La même grille de cellules, lue comme un nuage superposé plutôt que des points stylés individuellement.</p><p class="sp-3d-uses">Utilise: <code>axes, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/splom3d-density.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.splom3d(
    &quot;Density 3D&quot;,
    variant=&quot;density&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;],
    series=[[80, 65, 70], [60, 80, 55], [40, 70, 90], [90, 40, 60], [55, 85, 45], [70, 55, 80], ...],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>regression</code></h3><p>La même grille de cellules, chacune avec une droite de régression tracée à travers ses points.</p><p class="sp-3d-uses">Utilise: <code>axes, series</code></p><iframe class="sp-preview-frame" data-src="../../previews/splom3d-regression.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.splom3d(
    &quot;Regression 3D&quot;,
    variant=&quot;regression&quot;,
    axes=[&quot;Speed&quot;, &quot;Power&quot;, &quot;Range&quot;],
    series=[[80, 65, 70], [60, 80, 55], [40, 70, 90], [90, 40, 60], [55, 85, 45], [70, 55, 80]],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="splom3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
