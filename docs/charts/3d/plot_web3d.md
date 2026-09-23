# Plot Web Chart 3D

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

`sp.plot_web3d(title, x=None, y=None, sizes=None, labels=None, categories=None, *, variant="scatter", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_plot_web3d_chart()`, `sp.plot_web_3d()`, `sp.plot_web3d_chart()`, `sp.plot_web3d_family()`, `sp.plot_webs3d()`.

## Description

`sp.plot_web3d()` is the 3D twin of `sp.plot_web()`: **both plot-web variants have a 3D form**, selected with the same `variant` keyword and fed with the same points. `scatter` places every point at its own (x, y); `radial` instead gives each category its own spoke and places points by x-position along the spoke's radius.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `scatter` | `web`, `connected`, `default`, `flow` | `x, y, labels, categories` | Every point at its own (x, y) position, sized by `sizes` and toned by its category. |
| `radial` | `solar`, `stellar`, `mandala`, `spider` | `x, y, labels, categories` | Every category its own spoke around the centre; points sit along it by x-position, height by y. |

## Data

`x`/`y` position each point, `sizes` scales its marker, `labels` names it and `categories` groups it (spokes for `radial`, colour for `scatter`).

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="plot_web3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>scatter</code></h3><p>Every point at its own (x, y) position, sized by <code>sizes</code> and toned by its category.</p><p class="sp-3d-uses">Uses: <code>x, y, labels, categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-scatter.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.plot_web3d(
    &quot;Scatter 3D&quot;,
    variant=&quot;scatter&quot;,
    x_values=[12, 25, 40, 55, 70, 85],
    y_values=[18, 42, 28, 55, 38, 72],
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;],
    groups=[&quot;G1&quot;, &quot;G1&quot;, &quot;G2&quot;, &quot;G2&quot;, &quot;G3&quot;, &quot;G3&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>radial</code></h3><p>Every category its own spoke around the centre; points sit along it by x-position, height by y.</p><p class="sp-3d-uses">Uses: <code>x, y, labels, categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-radial.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.plot_web3d(
    &quot;Radial 3D&quot;,
    variant=&quot;radial&quot;,
    x_values=[12, 25, 40, 55, 70, 85],
    y_values=[18, 42, 28, 55, 38, 72],
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;],
    groups=[&quot;G1&quot;, &quot;G1&quot;, &quot;G2&quot;, &quot;G2&quot;, &quot;G3&quot;, &quot;G3&quot;],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="plot_web3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.plot_web3d(title, x=None, y=None, sizes=None, labels=None, categories=None, *, variant="scatter", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_plot_web3d_chart()`, `sp.plot_web_3d()`, `sp.plot_web3d_chart()`, `sp.plot_web3d_family()`, `sp.plot_webs3d()`.

<h2>Description</h2>

`sp.plot_web3d()` est le jumeau 3D de `sp.plot_web()` : **les deux variantes de plot-web ont une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes points. `scatter` place chaque point à son propre (x, y) ; `radial` donne à chaque catégorie son propre rayon et place les points par position x le long de ce rayon.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `scatter` | `web`, `connected`, `default`, `flow` | `x, y, labels, categories` | Chaque point à sa propre position (x, y), mis à l'échelle par `sizes` et teinté selon sa catégorie. |
| `radial` | `solar`, `stellar`, `mandala`, `spider` | `x, y, labels, categories` | Chaque catégorie a son propre rayon autour du centre ; les points s'y placent par position x, la hauteur suivant y. |

<h2>Données</h2>

`x`/`y` positionne chaque point, `sizes` met à l'échelle son repère, `labels` le nomme et `categories` le groupe (rayons pour `radial`, couleur pour `scatter`).

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="plot_web3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>scatter</code></h3><p>Chaque point à sa propre position (x, y), mis à l&#x27;échelle par <code>sizes</code> et teinté selon sa catégorie.</p><p class="sp-3d-uses">Utilise: <code>x, y, labels, categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-scatter.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.plot_web3d(
    &quot;Scatter 3D&quot;,
    variant=&quot;scatter&quot;,
    x_values=[12, 25, 40, 55, 70, 85],
    y_values=[18, 42, 28, 55, 38, 72],
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;],
    groups=[&quot;G1&quot;, &quot;G1&quot;, &quot;G2&quot;, &quot;G2&quot;, &quot;G3&quot;, &quot;G3&quot;],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>radial</code></h3><p>Chaque catégorie a son propre rayon autour du centre ; les points s&#x27;y placent par position x, la hauteur suivant y.</p><p class="sp-3d-uses">Utilise: <code>x, y, labels, categories</code></p><iframe class="sp-preview-frame" data-src="../../previews/plot_web3d-radial.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.plot_web3d(
    &quot;Radial 3D&quot;,
    variant=&quot;radial&quot;,
    x_values=[12, 25, 40, 55, 70, 85],
    y_values=[18, 42, 28, 55, 38, 72],
    labels=[&quot;A&quot;, &quot;B&quot;, &quot;C&quot;, &quot;D&quot;, &quot;E&quot;, &quot;F&quot;],
    groups=[&quot;G1&quot;, &quot;G1&quot;, &quot;G2&quot;, &quot;G2&quot;, &quot;G3&quot;, &quot;G3&quot;],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="plot_web3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
