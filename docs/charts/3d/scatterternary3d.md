# Scatter Ternary Chart 3D

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

`sp.scatterternary3d(title, x=None, y=None, z=None, labels=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_scatterternary3d_chart()`, `sp.scatterternary_3d()`, `sp.scatterternary3d_chart()`, `sp.scatterternary3d_family()`, `sp.scatterternaries3d()`.

## Description

`sp.scatterternary3d()` is the 3D twin of `sp.scatterternary()`: **every one of the 3 ternary variants has a 3D form**, selected with the same `variant` keyword and fed with the same three-part values. Every point is converted with the standard barycentric formula onto the floor of a triangle simplex, exactly the same footprint as the 2D chart's corners.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `basic` | `default`, `classic`, `dots` | `x, y, z` | A small marker per point on the triangle floor. |
| `bubble` | `sized`, `weighted`, `proportional` | `x, y, z` | The same markers scaled by their colour value: bigger for a higher value. |
| `labeled` | `labelled`, `annotated`, `named` | `x, y, z` | The same markers as basic; names show on hover in every variant. |

## Data

`x`, `y` and `z` give each point's three parts (only their relative share matters) and `labels` names them.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="scatterternary3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>A small marker per point on the triangle floor.</p><p class="sp-3d-uses">Uses: <code>x, y, z</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatterternary3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    x=[0.7, 0.2, 0.1, 0.4, 0.33],
    y=[0.2, 0.6, 0.1, 0.3, 0.33],
    z=[0.1, 0.2, 0.8, 0.3, 0.34],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>bubble</code></h3><p>The same markers scaled by their colour value: bigger for a higher value.</p><p class="sp-3d-uses">Uses: <code>x, y, z</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-bubble.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatterternary3d(
    &quot;Bubble 3D&quot;,
    variant=&quot;bubble&quot;,
    x=[0.7, 0.2, 0.1, 0.4, 0.33],
    y=[0.2, 0.6, 0.1, 0.3, 0.33],
    z=[0.1, 0.2, 0.8, 0.3, 0.34],
    color_values=[5, 20, 45, 12, 30],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>labeled</code></h3><p>The same markers as basic; names show on hover in every variant.</p><p class="sp-3d-uses">Uses: <code>x, y, z</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-labeled.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatterternary3d(
    &quot;Labeled 3D&quot;,
    variant=&quot;labeled&quot;,
    x=[0.7, 0.2, 0.1, 0.4, 0.33],
    y=[0.2, 0.6, 0.1, 0.3, 0.33],
    z=[0.1, 0.2, 0.8, 0.3, 0.34],
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="scatterternary3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.scatterternary3d(title, x=None, y=None, z=None, labels=None, *, variant="basic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_scatterternary3d_chart()`, `sp.scatterternary_3d()`, `sp.scatterternary3d_chart()`, `sp.scatterternary3d_family()`, `sp.scatterternaries3d()`.

<h2>Description</h2>

`sp.scatterternary3d()` est le jumeau 3D de `sp.scatterternary()` : **chacune des 3 variantes ternaires a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes valeurs en trois parts. Chaque point est converti par la formule barycentrique standard sur le sol d'un simplexe triangulaire, exactement la même empreinte que les coins du graphique 2D.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `basic` | `default`, `classic`, `dots` | `x, y, z` | Un petit repère par point sur le sol du triangle. |
| `bubble` | `sized`, `weighted`, `proportional` | `x, y, z` | Les mêmes repères mis à l'échelle selon leur valeur de couleur : plus grands pour une valeur plus haute. |
| `labeled` | `labelled`, `annotated`, `named` | `x, y, z` | Les mêmes repères que basic ; les noms s'affichent au survol dans toutes les variantes. |

<h2>Données</h2>

`x`, `y` et `z` donnent les trois parts de chaque point (seule leur part relative compte) et `labels` les nomme.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="scatterternary3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>basic</code></h3><p>Un petit repère par point sur le sol du triangle.</p><p class="sp-3d-uses">Utilise: <code>x, y, z</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-basic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatterternary3d(
    &quot;Basic 3D&quot;,
    variant=&quot;basic&quot;,
    x=[0.7, 0.2, 0.1, 0.4, 0.33],
    y=[0.2, 0.6, 0.1, 0.3, 0.33],
    z=[0.1, 0.2, 0.8, 0.3, 0.34],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>bubble</code></h3><p>Les mêmes repères mis à l&#x27;échelle selon leur valeur de couleur : plus grands pour une valeur plus haute.</p><p class="sp-3d-uses">Utilise: <code>x, y, z</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-bubble.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatterternary3d(
    &quot;Bubble 3D&quot;,
    variant=&quot;bubble&quot;,
    x=[0.7, 0.2, 0.1, 0.4, 0.33],
    y=[0.2, 0.6, 0.1, 0.3, 0.33],
    z=[0.1, 0.2, 0.8, 0.3, 0.34],
    color_values=[5, 20, 45, 12, 30],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>labeled</code></h3><p>Les mêmes repères que basic ; les noms s&#x27;affichent au survol dans toutes les variantes.</p><p class="sp-3d-uses">Utilise: <code>x, y, z</code></p><iframe class="sp-preview-frame" data-src="../../previews/scatterternary3d-labeled.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.scatterternary3d(
    &quot;Labeled 3D&quot;,
    variant=&quot;labeled&quot;,
    x=[0.7, 0.2, 0.1, 0.4, 0.33],
    y=[0.2, 0.6, 0.1, 0.3, 0.33],
    z=[0.1, 0.2, 0.8, 0.3, 0.34],
    labels=[&quot;Alpha&quot;, &quot;Beta&quot;, &quot;Gamma&quot;, &quot;Delta&quot;, &quot;Epsilon&quot;],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="scatterternary3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
