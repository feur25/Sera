# Pulse Chart 3D

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

`sp.pulse3d(title, labels=None, values=None, *, variant="radial", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=560, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Aliases: `sp.build_pulse3d_chart()`, `sp.pulse_3d()`, `sp.pulse3d_chart()`, `sp.radial_bar3d()`, `sp.rhythm3d()`.

## Description

`sp.pulse3d()` is the 3D twin of `sp.pulse()`: **all 5 pulse variants share the same 3D form**, selected with the same `variant` keyword and fed with the same values. Every label gets its own column around a ring, height following its value; `wave` and `dot` add a thin closing wire joining every column's top into a loop, echoing the 2D chart's own closed curve/dot reading.

## Variants

| Variant | Aliases | Uses | 3D form |
|---|---|---|---|
| `radial` | `default`, `classic` | `labels, values` | One column per label around a ring, height following its value. |
| `wave` | `sine`, `smooth` | `labels, values` | The same ring of columns plus a thin wire closing their tops into a loop. |
| `dot` | `scatter`, `bubble` | `labels, values` | The same ring of columns, thinner, plus the same closing wire. |
| `filled` | `area`, `solid` | `labels, values` | The same ring of columns, visibly wider. |
| `outlined` | `outline`, `stroke`, `clean` | `labels, values` | The same ring of columns, thinner and understated. |

## Data

`labels` names each position around the ring and `values` sets its column height.

## 3D planes

The viewpoint is independent from the variant: `orientation3d` picks the initial camera plane and applies to **every** variant; the view always stays draggable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Isometric three-quarter view (default).</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Side-on view, almost at floor level.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Top-down view from above.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Front elevation, nearly flat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-plane-front.html"></iframe></div>
</div>

## 3D scenes

`scene` swaps the environment the columns are drawn in and applies to every variant and every plane.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-scene-podium.html"></iframe></div>
</div>

## Themes

`theme` restyles the whole canvas (colour grade and glow) and applies to every variant, scene and plane.

<div data-sp-registry-table="themes" data-family="pulse3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-cosmic.html"></iframe></div>
</div>

## Auto-scaling zone

The 3D zone (floor, walls, axes and camera) scales to the elements: its length, width and height follow the extents of the drawn blocks, a minimum floor depth keeps single rows readable, wide scenes are drawn flatter and the camera frames the whole box. The axis ticks read the real data range. Pass `zone=[x, y, z]` to force the proportions of the box instead; the longest side is normalised to 1.

## Gallery

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>radial</code></h3><p>One column per label around a ring, height following its value.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pulse3d-radial.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pulse3d(
    &quot;Radial 3D&quot;,
    variant=&quot;radial&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    values=[0.4, 0.7, 0.9, 0.6, 0.8, 0.3, 0.5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>wave</code></h3><p>The same ring of columns plus a thin wire closing their tops into a loop.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pulse3d-wave.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pulse3d(
    &quot;Wave 3D&quot;,
    variant=&quot;wave&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    values=[0.4, 0.7, 0.9, 0.6, 0.8, 0.3, 0.5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>dot</code></h3><p>The same ring of columns, thinner, plus the same closing wire.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pulse3d-dot.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pulse3d(
    &quot;Dot 3D&quot;,
    variant=&quot;dot&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    values=[0.4, 0.7, 0.9, 0.6, 0.8, 0.3, 0.5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>filled</code></h3><p>The same ring of columns, visibly wider.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pulse3d-filled.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pulse3d(
    &quot;Filled 3D&quot;,
    variant=&quot;filled&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    values=[0.4, 0.7, 0.9, 0.6, 0.8, 0.3, 0.5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outlined</code></h3><p>The same ring of columns, thinner and understated.</p><p class="sp-3d-uses">Uses: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pulse3d-outlined.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pulse3d(
    &quot;Outlined 3D&quot;,
    variant=&quot;outlined&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    values=[0.4, 0.7, 0.9, 0.6, 0.8, 0.3, 0.5],
)</code></pre></details></div>
</div>

## Parameters

<div data-sp-registry-table="options" data-family="pulse3d"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.pulse3d(title, labels=None, values=None, *, variant="radial", orientation3d="iso", scene="default", theme="none", zone=None, max_points=None, bg_color="#1a1a2e", width=560, height=560, x_label="", y_label="", z_label="", **kwargs) -> Chart`

Alias : `sp.build_pulse3d_chart()`, `sp.pulse_3d()`, `sp.pulse3d_chart()`, `sp.radial_bar3d()`, `sp.rhythm3d()`.

<h2>Description</h2>

`sp.pulse3d()` est le jumeau 3D de `sp.pulse()` : **les 5 variantes de pulse partagent la même forme 3D**, choisie avec le même mot-clé `variant` et alimentée par les mêmes valeurs. Chaque libellé obtient sa propre colonne autour d'un anneau, la hauteur suivant sa valeur ; `wave` et `dot` ajoutent un fin fil de fermeture reliant le sommet de chaque colonne en boucle, faisant écho à la lecture en courbe fermée/points du graphique 2D.

<h2>Variantes</h2>

| Variante | Alias | Utilise | Forme 3D |
|---|---|---|---|
| `radial` | `default`, `classic` | `labels, values` | Une colonne par libellé autour d'un anneau, la hauteur suivant sa valeur. |
| `wave` | `sine`, `smooth` | `labels, values` | Le même anneau de colonnes plus un fin fil fermant leurs sommets en boucle. |
| `dot` | `scatter`, `bubble` | `labels, values` | Le même anneau de colonnes, plus fines, plus le même fil de fermeture. |
| `filled` | `area`, `solid` | `labels, values` | Le même anneau de colonnes, visiblement plus larges. |
| `outlined` | `outline`, `stroke`, `clean` | `labels, values` | Le même anneau de colonnes, plus fines et discrètes. |

<h2>Données</h2>

`labels` nomme chaque position autour de l'anneau et `values` fixe la hauteur de sa colonne.

<h2>Plans 3D</h2>

Le point de vue est indépendant de la variante : `orientation3d` choisit le plan initial de la caméra et s'applique à **toutes** les variantes ; la vue reste toujours orientable.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>orientation3d="iso"</code> &mdash; Vue isométrique trois-quarts (défaut).</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-plane-iso.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="horizontal"</code> &mdash; Vue de côté, presque au niveau du sol.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-plane-horizontal.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="vertical"</code> &mdash; Vue du dessus, plongeante.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-plane-vertical.html"></iframe></div>
<div><div class="sp-preview-label"><code>orientation3d="front"</code> &mdash; Élévation de face, presque à plat.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-plane-front.html"></iframe></div>
</div>

<h2>Scènes 3D</h2>

`scene` change l'environnement dans lequel les colonnes sont dessinées et s'applique à chaque variante et chaque plan.

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>scene="default"</code> &mdash; Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-scene-default.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="terrain"</code> &mdash; Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-scene-terrain.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="tower"</code> &mdash; Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-scene-tower.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="radial"</code> &mdash; Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-scene-radial.html"></iframe></div>
<div><div class="sp-preview-label"><code>scene="podium"</code> &mdash; Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.</div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-scene-podium.html"></iframe></div>
</div>

<h2>Thèmes</h2>

`theme` restyle tout le canevas (étalonnage des couleurs et halo) et s'applique à chaque variante, scène et plan.

<div data-sp-registry-table="themes" data-family="pulse3d"></div>

<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:14px;margin-top:10px">
<div><div class="sp-preview-label"><code>theme="deluxe"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-deluxe.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="prism"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-prism.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="aurora"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-aurora.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="inferno"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-inferno.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="frost"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-frost.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glow"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-glow.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="glass"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-glass.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="neon"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-neon.html"></iframe></div>
<div><div class="sp-preview-label"><code>theme="cosmic"</code></div>
<iframe class="sp-preview-frame" data-src="../../previews/pulse3d-theme-cosmic.html"></iframe></div>
</div>

<h2>Zone auto-ajustée</h2>

La zone 3D (sol, parois, axes et caméra) s'adapte aux éléments : sa longueur, sa largeur et sa hauteur suivent l'étendue des blocs dessinés, une profondeur minimale garde les rangées seules lisibles, les scènes larges sont dessinées plus basses et la caméra cadre toute la boîte. Les graduations des axes lisent la vraie plage des données. Passez `zone=[x, y, z]` pour forcer plutôt les proportions de la boîte ; le côté le plus long est normalisé à 1.

<h2>Galerie</h2>

<div class="sp-3d-grid">
<div class="sp-3d-card"><h3><code>radial</code></h3><p>Une colonne par libellé autour d&#x27;un anneau, la hauteur suivant sa valeur.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pulse3d-radial.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pulse3d(
    &quot;Radial 3D&quot;,
    variant=&quot;radial&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    values=[0.4, 0.7, 0.9, 0.6, 0.8, 0.3, 0.5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>wave</code></h3><p>Le même anneau de colonnes plus un fin fil fermant leurs sommets en boucle.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pulse3d-wave.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pulse3d(
    &quot;Wave 3D&quot;,
    variant=&quot;wave&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    values=[0.4, 0.7, 0.9, 0.6, 0.8, 0.3, 0.5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>dot</code></h3><p>Le même anneau de colonnes, plus fines, plus le même fil de fermeture.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pulse3d-dot.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pulse3d(
    &quot;Dot 3D&quot;,
    variant=&quot;dot&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    values=[0.4, 0.7, 0.9, 0.6, 0.8, 0.3, 0.5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>filled</code></h3><p>Le même anneau de colonnes, visiblement plus larges.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pulse3d-filled.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pulse3d(
    &quot;Filled 3D&quot;,
    variant=&quot;filled&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    values=[0.4, 0.7, 0.9, 0.6, 0.8, 0.3, 0.5],
)</code></pre></details></div>
<div class="sp-3d-card"><h3><code>outlined</code></h3><p>Le même anneau de colonnes, plus fines et discrètes.</p><p class="sp-3d-uses">Utilise: <code>labels, values</code></p><iframe class="sp-preview-frame" data-src="../../previews/pulse3d-outlined.html"></iframe><details><summary>Python</summary><pre><code class="language-python">import seraplot as sp

chart = sp.pulse3d(
    &quot;Outlined 3D&quot;,
    variant=&quot;outlined&quot;,
    labels=[&quot;Mon&quot;, &quot;Tue&quot;, &quot;Wed&quot;, &quot;Thu&quot;, &quot;Fri&quot;, &quot;Sat&quot;, &quot;Sun&quot;],
    values=[0.4, 0.7, 0.9, 0.6, 0.8, 0.3, 0.5],
)</code></pre></details></div>
</div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="pulse3d"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
