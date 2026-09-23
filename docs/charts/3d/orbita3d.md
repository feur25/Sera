# Orbita Chart 3D

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

`sp.orbita3d(title, series_names=None, labels=None, matrix=None, *, variant="classic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_orbita3d_chart()`, `sp.orbita_3d()`, `sp.orbita3d_chart()`, `sp.orbita3d_family()`, `sp.orbitas3d()`.

## Description

`sp.orbita3d()` is the 3D twin of `sp.orbita()`: **every one of the 6 orbita variants has a 3D form**, selected with the same `variant` keyword and fed with the same matrix. Every series stands on its own concentric ring, one marker per label spread evenly around it, height following the matrix value at that series/label pair.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `classic` | `default`, `basic` | `series_names, labels, matrix` | One marker per series/label pair on its ring, height following the value. |
| `bubble` | `sized`, `area` | `series_names, labels, matrix` | The same rings with marker size also scaling with the value. |
| `trail` | `line`, `connected` | `series_names, labels, matrix` | The same rings plus a thin band joining every ring's markers into a closed loop. |
| `glow` | `neon`, `light` | `series_names, labels, matrix` | The same rings as classic, from the 2D chart's soft glow styling. |
| `minimal` | `thin`, `clean` | `series_names, labels, matrix` | The same rings as classic, from the 2D chart's bare unstyled reading. |
| `delta` | `change`, `trend`, `momentum` | `series_names, labels, matrix` | Markers toned green or red by whether the value rose or fell from the ring before it. |

## Data

`series_names` names each ring, `labels` names each angular position and `matrix` gives one row per series, one value per label.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="orbita3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>classic</code></h3><p>One marker per series/label pair on its ring, height following the value.</p><p class="sp-3d-uses">Uses: <code>series_names, labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/orbita3d-classic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.orbita3d(
    &quot;Classic 3D&quot;,
    variant=&quot;classic&quot;,
    series_names=[&quot;2021&quot;, &quot;2022&quot;, &quot;2023&quot;],
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    matrix=[[0.4, 0.7, 0.5, 0.8], [0.6, 0.3, 0.9, 0.5], [0.8, 0.6, 0.4, 0.7]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>bubble</code></h3><p>The same rings with marker size also scaling with the value.</p><p class="sp-3d-uses">Uses: <code>series_names, labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/orbita3d-bubble.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.orbita3d(
    &quot;Bubble 3D&quot;,
    variant=&quot;bubble&quot;,
    series_names=[&quot;2021&quot;, &quot;2022&quot;, &quot;2023&quot;],
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    matrix=[[0.4, 0.7, 0.5, 0.8], [0.6, 0.3, 0.9, 0.5], [0.8, 0.6, 0.4, 0.7]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>trail</code></h3><p>The same rings plus a thin band joining every ring&#x27;s markers into a closed loop.</p><p class="sp-3d-uses">Uses: <code>series_names, labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/orbita3d-trail.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.orbita3d(
    &quot;Trail 3D&quot;,
    variant=&quot;trail&quot;,
    series_names=[&quot;2021&quot;, &quot;2022&quot;, &quot;2023&quot;],
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    matrix=[[0.4, 0.7, 0.5, 0.8], [0.6, 0.3, 0.9, 0.5], [0.8, 0.6, 0.4, 0.7]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>glow</code></h3><p>The same rings as classic, from the 2D chart&#x27;s soft glow styling.</p><p class="sp-3d-uses">Uses: <code>series_names, labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/orbita3d-glow.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.orbita3d(
    &quot;Glow 3D&quot;,
    variant=&quot;glow&quot;,
    series_names=[&quot;2021&quot;, &quot;2022&quot;, &quot;2023&quot;],
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    matrix=[[0.4, 0.7, 0.5, 0.8], [0.6, 0.3, 0.9, 0.5], [0.8, 0.6, 0.4, 0.7]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>minimal</code></h3><p>The same rings as classic, from the 2D chart&#x27;s bare unstyled reading.</p><p class="sp-3d-uses">Uses: <code>series_names, labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/orbita3d-minimal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.orbita3d(
    &quot;Minimal 3D&quot;,
    variant=&quot;minimal&quot;,
    series_names=[&quot;2021&quot;, &quot;2022&quot;, &quot;2023&quot;],
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    matrix=[[0.4, 0.7, 0.5, 0.8], [0.6, 0.3, 0.9, 0.5], [0.8, 0.6, 0.4, 0.7]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>delta</code></h3><p>Markers toned green or red by whether the value rose or fell from the ring before it.</p><p class="sp-3d-uses">Uses: <code>series_names, labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/orbita3d-delta.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.orbita3d(
    &quot;Delta 3D&quot;,
    variant=&quot;delta&quot;,
    series_names=[&quot;2021&quot;, &quot;2022&quot;, &quot;2023&quot;],
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    matrix=[[0.4, 0.7, 0.5, 0.8], [0.6, 0.3, 0.9, 0.5], [0.8, 0.6, 0.4, 0.7]],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="orbita3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.orbita3d(title, series_names=None, labels=None, matrix=None, *, variant="classic", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=900, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_orbita3d_chart()`, `sp.orbita_3d()`, `sp.orbita3d_chart()`, `sp.orbita3d_family()`, `sp.orbitas3d()`.

<h2>Description</h2>

`sp.orbita3d()` est le jumeau 3D de `sp.orbita()` : **chacune des 6 variantes d'orbita a une forme 3D**, choisie avec le même mot-clé `variant` et alimentée par la même matrice. Chaque série se tient sur son propre anneau concentrique, un repère par libellé réparti régulièrement autour de lui, la hauteur suivant la valeur de la matrice pour cette paire série/libellé.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `classic` | `default`, `basic` | `series_names, labels, matrix` | Un repère par paire série/libellé sur son anneau, la hauteur suivant la valeur. |
| `bubble` | `sized`, `area` | `series_names, labels, matrix` | Les mêmes anneaux avec la taille du repère qui suit aussi la valeur. |
| `trail` | `line`, `connected` | `series_names, labels, matrix` | Les mêmes anneaux plus une fine bande reliant les repères de chaque anneau en une boucle fermée. |
| `glow` | `neon`, `light` | `series_names, labels, matrix` | Les mêmes anneaux que classic, depuis le style en halo doux du graphique 2D. |
| `minimal` | `thin`, `clean` | `series_names, labels, matrix` | Les mêmes anneaux que classic, depuis la lecture nue et sans style du graphique 2D. |
| `delta` | `change`, `trend`, `momentum` | `series_names, labels, matrix` | Repères teintés en vert ou rouge selon que la valeur a monté ou baissé par rapport à l'anneau précédent. |

<h2>Données</h2>

`series_names` nomme chaque anneau, `labels` nomme chaque position angulaire et `matrix` donne une ligne par série, une valeur par libellé.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="orbita3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/orbita3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>classic</code></h3><p>Un repère par paire série/libellé sur son anneau, la hauteur suivant la valeur.</p><p class="sp-3d-uses">Utilise: <code>series_names, labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/orbita3d-classic.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.orbita3d(
    &quot;Classic 3D&quot;,
    variant=&quot;classic&quot;,
    series_names=[&quot;2021&quot;, &quot;2022&quot;, &quot;2023&quot;],
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    matrix=[[0.4, 0.7, 0.5, 0.8], [0.6, 0.3, 0.9, 0.5], [0.8, 0.6, 0.4, 0.7]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>bubble</code></h3><p>Les mêmes anneaux avec la taille du repère qui suit aussi la valeur.</p><p class="sp-3d-uses">Utilise: <code>series_names, labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/orbita3d-bubble.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.orbita3d(
    &quot;Bubble 3D&quot;,
    variant=&quot;bubble&quot;,
    series_names=[&quot;2021&quot;, &quot;2022&quot;, &quot;2023&quot;],
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    matrix=[[0.4, 0.7, 0.5, 0.8], [0.6, 0.3, 0.9, 0.5], [0.8, 0.6, 0.4, 0.7]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>trail</code></h3><p>Les mêmes anneaux plus une fine bande reliant les repères de chaque anneau en une boucle fermée.</p><p class="sp-3d-uses">Utilise: <code>series_names, labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/orbita3d-trail.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.orbita3d(
    &quot;Trail 3D&quot;,
    variant=&quot;trail&quot;,
    series_names=[&quot;2021&quot;, &quot;2022&quot;, &quot;2023&quot;],
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    matrix=[[0.4, 0.7, 0.5, 0.8], [0.6, 0.3, 0.9, 0.5], [0.8, 0.6, 0.4, 0.7]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>glow</code></h3><p>Les mêmes anneaux que classic, depuis le style en halo doux du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>series_names, labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/orbita3d-glow.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.orbita3d(
    &quot;Glow 3D&quot;,
    variant=&quot;glow&quot;,
    series_names=[&quot;2021&quot;, &quot;2022&quot;, &quot;2023&quot;],
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    matrix=[[0.4, 0.7, 0.5, 0.8], [0.6, 0.3, 0.9, 0.5], [0.8, 0.6, 0.4, 0.7]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>minimal</code></h3><p>Les mêmes anneaux que classic, depuis la lecture nue et sans style du graphique 2D.</p><p class="sp-3d-uses">Utilise: <code>series_names, labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/orbita3d-minimal.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.orbita3d(
    &quot;Minimal 3D&quot;,
    variant=&quot;minimal&quot;,
    series_names=[&quot;2021&quot;, &quot;2022&quot;, &quot;2023&quot;],
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    matrix=[[0.4, 0.7, 0.5, 0.8], [0.6, 0.3, 0.9, 0.5], [0.8, 0.6, 0.4, 0.7]],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>delta</code></h3><p>Repères teintés en vert ou rouge selon que la valeur a monté ou baissé par rapport à l&#x27;anneau précédent.</p><p class="sp-3d-uses">Utilise: <code>series_names, labels, matrix</code></p><iframe class="sp-preview-frame" data-src="../../previews/orbita3d-delta.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.orbita3d(
    &quot;Delta 3D&quot;,
    variant=&quot;delta&quot;,
    series_names=[&quot;2021&quot;, &quot;2022&quot;, &quot;2023&quot;],
    labels=[&quot;Q1&quot;, &quot;Q2&quot;, &quot;Q3&quot;, &quot;Q4&quot;],
    matrix=[[0.4, 0.7, 0.5, 0.8], [0.6, 0.3, 0.9, 0.5], [0.8, 0.6, 0.4, 0.7]],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="orbita3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
