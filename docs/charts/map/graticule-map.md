# Graticule Map

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.graticule_map(title, *, variant="lines", step=15, center_lat=None, center_lon=None, color_low=None, color_high=None, **kwargs) -> Chart`

Aliases: `sp.graticule_map`, `sp.graticulemap`, `sp.graticule_map_chart`, `sp.graticule`, `sp.meridian_map`, `sp.grid_map`, `sp.build_graticule_map`

## Description

`sp.graticule_map()` needs no data at all — it draws the coordinate reference grid itself: meridians and parallels every `step` degrees, the equator and prime meridian emphasized. No `lats`/`lons`/`values` because the grid *is* the content, the same way `daynight` needs none for its terminator. Useful on its own as a reference layer, or as a way to see exactly how this toolkit's own projections behave.

## Variants

<div data-sp-registry-table="variants" data-family="graticule_map"></div>

## Parameters

<div data-sp-registry-table="options" data-family="graticule_map"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="graticulemap-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('graticulemap-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('graticulemap-en','lines',this)"><span class="sp-cic">#</span><span class="sp-clb">Lines</span></button>
<button class="sp-cls-tab" onclick="spCls('graticulemap-en','globe',this)"><span class="sp-cic">◉</span><span class="sp-clb">Globe</span></button>
<button class="sp-cls-tab" onclick="spCls('graticulemap-en','tissot',this)"><span class="sp-cic">○</span><span class="sp-clb">Tissot</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="graticulemap-en-lines">

The default: a flat meridian/parallel grid drawn directly through this map's own equirectangular-style projection, degree-labeled at the edges, equator and prime meridian drawn heavier than the rest.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"lines"</code></span><span><strong>Aliases</strong> <code>"lines"</code> / <code>"grid"</code> / <code>"basic"</code> / <code>"default"</code> / <code>"meridians"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/graticule-map-lines.html"></iframe>
</div>

<div class="sp-variant" id="graticulemap-en-globe">

The same grid wrapped onto a real sphere: every meridian and parallel is sampled and projected through the orthographic projection point by point, so each line genuinely curves and disappears past the limb exactly where a real globe's would — not a flat grid pasted onto a circle. `center_lat` / `center_lon` aim the view (default `20, 10`). Country outlines render on the sphere for context.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"globe"</code></span><span><strong>Aliases</strong> <code>"globe"</code> / <code>"orthographic"</code> / <code>"sphere"</code> / <code>"space"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/graticule-map-globe.html"></iframe>
</div>

<div class="sp-variant" id="graticulemap-en-tissot">

Tissot's indicatrix, the classic cartographic honesty check: a small circle of genuinely fixed angular radius — the real spherical destination-point formula, not a drawn ellipse — is placed at every grid intersection and re-projected through this map's own flat projection. Near the equator it stays close to circular; near the poles it visibly stretches, showing this toolkit's *actual* distortion rather than asserting it.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"tissot"</code></span><span><strong>Aliases</strong> <code>"tissot"</code> / <code>"indicatrix"</code> / <code>"distortion"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/graticule-map-tissot.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.graticule_map(title, *, variant="lines", step=15, center_lat=None, center_lon=None, color_low=None, color_high=None, **kwargs) -> Chart`

Alias : `sp.graticule_map`, `sp.graticulemap`, `sp.graticule_map_chart`, `sp.graticule`, `sp.meridian_map`, `sp.grid_map`, `sp.build_graticule_map`

## Description

`sp.graticule_map()` n'a besoin d'aucune donnée — il dessine la grille de référence de coordonnées elle-même : méridiens et parallèles tous les `step` degrés, équateur et méridien de Greenwich mis en évidence. Pas de `lats`/`lons`/`values` car la grille *est* le contenu, de la même façon que `daynight` n'en a besoin d'aucun pour son terminateur. Utile seule comme couche de référence, ou pour voir précisément comment se comportent les propres projections de cet outil.

## Variantes

<div data-sp-registry-table="variants" data-family="graticule_map"></div>

## Paramètres

<div data-sp-registry-table="options" data-family="graticule_map"></div>

---

## Retourne

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

---

<div class="sp-cls sp-open" id="graticulemap-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('graticulemap-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('graticulemap-fr','lines',this)"><span class="sp-cic">#</span><span class="sp-clb">Lignes</span></button>
<button class="sp-cls-tab" onclick="spCls('graticulemap-fr','globe',this)"><span class="sp-cic">◉</span><span class="sp-clb">Globe</span></button>
<button class="sp-cls-tab" onclick="spCls('graticulemap-fr','tissot',this)"><span class="sp-cic">○</span><span class="sp-clb">Tissot</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="graticulemap-fr-lines">

Le défaut : une grille plate de méridiens/parallèles dessinée directement à travers la propre projection de style équirectangulaire de cette carte, étiquetée en degrés sur les bords, équateur et méridien de Greenwich tracés plus épais que le reste.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"lines"</code></span><span><strong>Alias</strong> <code>"lines"</code> / <code>"grid"</code> / <code>"basic"</code> / <code>"default"</code> / <code>"meridians"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/graticule-map-lines.html"></iframe>
</div>

<div class="sp-variant" id="graticulemap-fr-globe">

La même grille enroulée sur une vraie sphère : chaque méridien et parallèle est échantillonné et projeté point par point à travers la projection orthographique, si bien que chaque ligne courbe vraiment et disparaît derrière le limbe exactement comme le ferait un vrai globe — pas une grille plate collée sur un cercle. `center_lat` / `center_lon` orientent la vue (par défaut `20, 10`). Les contours des pays se dessinent sur la sphère pour le contexte.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"globe"</code></span><span><strong>Alias</strong> <code>"globe"</code> / <code>"orthographic"</code> / <code>"sphere"</code> / <code>"space"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/graticule-map-globe.html"></iframe>
</div>

<div class="sp-variant" id="graticulemap-fr-tissot">

L'indicatrice de Tissot, le test d'honnêteté cartographique classique : un petit cercle de rayon angulaire réellement fixe — la vraie formule sphérique de point de destination, pas une ellipse dessinée à la main — est placé à chaque intersection de grille puis reprojeté à travers la propre projection plate de cette carte. Près de l'équateur il reste proche du cercle ; près des pôles il s'étire visiblement, montrant la *vraie* distorsion de cet outil plutôt que de l'affirmer.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"tissot"</code></span><span><strong>Alias</strong> <code>"tissot"</code> / <code>"indicatrix"</code> / <code>"distortion"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/graticule-map-tissot.html"></iframe>
</div>

</div>
</div>

</div>
