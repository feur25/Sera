# Contour Map

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

`sp.contour_map(title, lats=None, lons=None, field=None, *, variant="filled", bins=6, color_low=None, color_high=None, **kwargs) -> Chart`

Aliases: `sp.contour_map`, `sp.contourmap`, `sp.contour_map_chart`, `sp.geo_contour`, `sp.isarithmic_map`, `sp.scalar_field_map`, `sp.build_contour_map`

## Description

`sp.contour_map()` takes scattered `lats`/`lons`/`field` samples — not named regions — and interpolates them into a continuous surface draped over the world outline: temperature, pressure, precipitation, anything that varies smoothly across space rather than stopping at a border. Interpolation is real inverse-distance-weighting over a grid spanning the data's own bounding box (padded, not the whole world), so the surface only ever describes the region actually sampled. `bins` controls how many discrete color bands (`filled`) or contour levels (`isolines`) the value range is split into.

## Variants

<div data-sp-registry-table="variants" data-family="contour_map"></div>

## Parameters

<div data-sp-registry-table="options" data-family="contour_map"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="contourmap-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('contourmap-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('contourmap-en','filled',this)"><span class="sp-cic">▧</span><span class="sp-clb">Filled</span></button>
<button class="sp-cls-tab" onclick="spCls('contourmap-en','isolines',this)"><span class="sp-cic">◠</span><span class="sp-clb">Isolines</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="contourmap-en-filled">

The default: the interpolated grid rendered as colored cells, a smooth heat-style drape from `color_low` to `color_high` across the sampled region — the fastest way to read where a field is high or low at a glance, without needing to trace individual lines.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"filled"</code></span><span><strong>Aliases</strong> <code>"filled"</code> / <code>"bands"</code> / <code>"heat"</code> / <code>"basic"</code> / <code>"default"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour-map-filled.html"></iframe>
</div>

<div class="sp-variant" id="contourmap-en-isolines">

Real contour lines, not an approximation: genuine marching squares over the same interpolated grid, tracing `bins` threshold levels by linearly interpolating exactly where each grid cell's edges cross the level — the same technique a real isobar or isotherm map uses, saddle cases included.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"isolines"</code></span><span><strong>Aliases</strong> <code>"isolines"</code> / <code>"lines"</code> / <code>"contour_lines"</code> / <code>"iso"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour-map-isolines.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.contour_map(title, lats=None, lons=None, field=None, *, variant="filled", bins=6, color_low=None, color_high=None, **kwargs) -> Chart`

Alias : `sp.contour_map`, `sp.contourmap`, `sp.contour_map_chart`, `sp.geo_contour`, `sp.isarithmic_map`, `sp.scalar_field_map`, `sp.build_contour_map`

## Description

`sp.contour_map()` prend des échantillons `lats`/`lons`/`field` dispersés — pas des régions nommées — et les interpole en une surface continue drapée sur le contour du monde : température, pression, précipitations, tout ce qui varie de façon lisse dans l'espace plutôt que de s'arrêter à une frontière. L'interpolation est une vraie pondération par distance inverse sur une grille couvrant la boîte englobante des données elles-mêmes (avec marge, pas le monde entier), donc la surface ne décrit jamais que la région réellement échantillonnée. `bins` contrôle en combien de bandes de couleur discrètes (`filled`) ou de niveaux de contour (`isolines`) la plage de valeurs est découpée.

## Variantes

<div data-sp-registry-table="variants" data-family="contour_map"></div>

## Paramètres

<div data-sp-registry-table="options" data-family="contour_map"></div>

---

## Retourne

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

---

<div class="sp-cls sp-open" id="contourmap-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('contourmap-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('contourmap-fr','filled',this)"><span class="sp-cic">▧</span><span class="sp-clb">Rempli</span></button>
<button class="sp-cls-tab" onclick="spCls('contourmap-fr','isolines',this)"><span class="sp-cic">◠</span><span class="sp-clb">Isolignes</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="contourmap-fr-filled">

Le défaut : la grille interpolée rendue en cellules colorées, un dégradé lisse façon carte de chaleur de `color_low` à `color_high` sur la région échantillonnée — la façon la plus rapide de voir en un coup d'œil où un champ est haut ou bas, sans avoir à suivre des lignes individuelles.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"filled"</code></span><span><strong>Alias</strong> <code>"filled"</code> / <code>"bands"</code> / <code>"heat"</code> / <code>"basic"</code> / <code>"default"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour-map-filled.html"></iframe>
</div>

<div class="sp-variant" id="contourmap-fr-isolines">

De vraies lignes de contour, pas une approximation : un authentique algorithme de marching squares sur la même grille interpolée, traçant `bins` niveaux de seuil en interpolant linéairement l'endroit exact où chaque arête de cellule croise le niveau — la même technique qu'une vraie carte d'isobares ou d'isothermes, cas de selle inclus.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"isolines"</code></span><span><strong>Alias</strong> <code>"isolines"</code> / <code>"lines"</code> / <code>"contour_lines"</code> / <code>"iso"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/contour-map-isolines.html"></iframe>
</div>

</div>
</div>

</div>
