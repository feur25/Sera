# Vector Field Map

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

`sp.vector_field_map(title, lats=None, lons=None, u=None, v=None, *, variant="arrows", color_low=None, color_high=None, **kwargs) -> Chart`

Aliases: `sp.vector_field_map`, `sp.vectorfieldmap`, `sp.vector_field_map_chart`, `sp.wind_map`, `sp.quiver_map`, `sp.geo_vector_field`, `sp.build_vector_field_map`

## Description

`sp.vector_field_map()` draws a direction-and-magnitude sample at each `lats`/`lons` point from its `u` (east-west) and `v` (north-south) components — wind, ocean currents, anything that has a direction as well as a size at a location. Magnitude scales both length and color from `color_low` to `color_high`; direction accounts for the map's own north-is-up orientation, not raw pixel angles.

## Variants

<div data-sp-registry-table="variants" data-family="vector_field_map"></div>

## Parameters

<div data-sp-registry-table="options" data-family="vector_field_map"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="vectorfieldmap-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('vectorfieldmap-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('vectorfieldmap-en','arrows',this)"><span class="sp-cic">➤</span><span class="sp-clb">Arrows</span></button>
<button class="sp-cls-tab" onclick="spCls('vectorfieldmap-en','streamlines',this)"><span class="sp-cic">〰</span><span class="sp-clb">Streamlines</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="vectorfieldmap-en-arrows">

The default, and the classic weather-map quiver: one shaft-and-arrowhead per sample, pointing the true `u`/`v` direction, length and color both scaled by magnitude so the strongest vectors are immediately the most visible ones.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arrows"</code></span><span><strong>Aliases</strong> <code>"arrows"</code> / <code>"quiver"</code> / <code>"wind"</code> / <code>"basic"</code> / <code>"default"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/vector-field-map-arrows.html"></iframe>
</div>

<div class="sp-variant" id="vectorfieldmap-en-streamlines">

True flow integration, not a static hint field: a grid of seed points is traced forward through the inverse-distance-weighted `u`/`v` field step by step, each curving path following the local direction it's actually moving through until the flow dies out or it leaves the sampled region — the same idea a real streamline or particle-flow visualization uses, not a decorative curve.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"streamlines"</code></span><span><strong>Aliases</strong> <code>"streamlines"</code> / <code>"streamline"</code> / <code>"flow"</code> / <code>"particles"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/vector-field-map-streamlines.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.vector_field_map(title, lats=None, lons=None, u=None, v=None, *, variant="arrows", color_low=None, color_high=None, **kwargs) -> Chart`

Alias : `sp.vector_field_map`, `sp.vectorfieldmap`, `sp.vector_field_map_chart`, `sp.wind_map`, `sp.quiver_map`, `sp.geo_vector_field`, `sp.build_vector_field_map`

## Description

`sp.vector_field_map()` dessine un échantillon de direction et magnitude à chaque point `lats`/`lons` à partir de ses composantes `u` (est-ouest) et `v` (nord-sud) — vent, courants océaniques, tout ce qui a une direction en plus d'une taille à un endroit donné. La magnitude met à l'échelle à la fois la longueur et la couleur de `color_low` à `color_high` ; la direction tient compte de l'orientation propre de la carte (nord vers le haut), pas d'angles de pixels bruts.

## Variantes

<div data-sp-registry-table="variants" data-family="vector_field_map"></div>

## Paramètres

<div data-sp-registry-table="options" data-family="vector_field_map"></div>

---

## Retourne

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

---

<div class="sp-cls sp-open" id="vectorfieldmap-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('vectorfieldmap-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('vectorfieldmap-fr','arrows',this)"><span class="sp-cic">➤</span><span class="sp-clb">Flèches</span></button>
<button class="sp-cls-tab" onclick="spCls('vectorfieldmap-fr','streamlines',this)"><span class="sp-cic">〰</span><span class="sp-clb">Lignes de courant</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="vectorfieldmap-fr-arrows">

Le défaut, et le quiver classique des cartes météo : une flèche (tige + pointe) par échantillon, pointant la vraie direction `u`/`v`, longueur et couleur toutes deux mises à l'échelle par la magnitude, si bien que les vecteurs les plus forts sont immédiatement les plus visibles.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"arrows"</code></span><span><strong>Alias</strong> <code>"arrows"</code> / <code>"quiver"</code> / <code>"wind"</code> / <code>"basic"</code> / <code>"default"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/vector-field-map-arrows.html"></iframe>
</div>

<div class="sp-variant" id="vectorfieldmap-fr-streamlines">

Une vraie intégration de flux, pas un champ d'indices statique : une grille de points de départ est tracée pas à pas à travers le champ `u`/`v` interpolé par pondération inverse à la distance, chaque chemin courbant selon la direction locale qu'il traverse réellement jusqu'à ce que le flux s'éteigne ou qu'il quitte la région échantillonnée — la même idée qu'une vraie visualisation de lignes de courant ou de flux de particules, pas une courbe décorative.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"streamlines"</code></span><span><strong>Alias</strong> <code>"streamlines"</code> / <code>"streamline"</code> / <code>"flow"</code> / <code>"particles"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/vector-field-map-streamlines.html"></iframe>
</div>

</div>
</div>

</div>
