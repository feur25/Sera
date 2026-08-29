# Bubble Map

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

`sp.bubble_map(title, labels=None, values=None, *, variant="proportional", map="world", region=None, center_lat=None, center_lon=None, **kwargs) -> Chart`

Aliases: `sp.bubble_map`, `sp.bubblemap`, `sp.bubble_map_chart`, `sp.geo_bubble`, `sp.geo_bubble_map`, `sp.build_bubble_map`

## Description

`sp.bubble_map()` marks matched regions on a map with a proportionally-sized circle at each centroid by default — real bubbles, not filled shapes; pick `variant="filled"` instead when it's the regions themselves the reader should compare, not a magnitude. `map` selects which registered geographic region set to draw; `region` optionally restricts drawing to one named group inside that set. All three are read live from the same register/inventory system the rest of the framework uses — nothing below is hardcoded.

## Variants

<div data-sp-registry-table="variants" data-family="bubble_map"></div>

## Region Sets

<div data-sp-registry-table="regions"></div>

## Parameters

<div data-sp-registry-table="options" data-family="bubble_map"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="bubblemap-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bubblemap-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bubblemap-en','proportional',this)"><span class="sp-cic">●</span><span class="sp-clb">Proportional</span></button>
<button class="sp-cls-tab" onclick="spCls('bubblemap-en','filled',this)"><span class="sp-cic">▧</span><span class="sp-clb">Filled</span></button>
<button class="sp-cls-tab" onclick="spCls('bubblemap-en','globe',this)"><span class="sp-cic">◉</span><span class="sp-clb">Globe</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bubblemap-en-proportional">

The default, and the real graduated-symbol map: the base is muted, and a circle sits on each matched region's centroid, its radius scaled by `sqrt(value)` so *area* — not radius — reads proportionally to magnitude. `min_bubble_size` / `max_bubble_size` set the radius range in pixels.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"proportional"</code></span><span><strong>Aliases</strong> <code>"proportional"</code> / <code>"bubble"</code> / <code>"graduated"</code> / <code>"sized"</code> / <code>"basic"</code> / <code>"default"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble-map-proportional.html"></iframe>
</div>

<div class="sp-variant" id="bubblemap-en-filled">

Not a bubble at all by design: each matched region filled solid with its own palette color and labeled at its centroid — a categorical highlight map, for when it's the regions themselves the reader should compare rather than a magnitude. Reach for `choropleth` instead if the fill color should encode a continuous value.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"filled"</code></span><span><strong>Aliases</strong> <code>"filled"</code> / <code>"regions"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble-map-filled.html"></iframe>
</div>

<div class="sp-variant" id="bubblemap-en-globe">

Proportional circles projected onto an orthographic sphere — every centroid is converted to real latitude/longitude and re-projected with actual globe math, so bubbles on the far side simply aren't drawn. `center_lat` / `center_lon` aim the view (default `15, 10`). Only region sets that expose a coordinate inversion support this — today, `"world"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"globe"</code></span><span><strong>Aliases</strong> <code>"globe"</code> / <code>"orthographic"</code> / <code>"sphere"</code> / <code>"space"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble-map-globe.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.bubble_map(title, labels=None, values=None, *, variant="proportional", map="world", region=None, center_lat=None, center_lon=None, **kwargs) -> Chart`

Alias : `sp.bubble_map`, `sp.bubblemap`, `sp.bubble_map_chart`, `sp.geo_bubble`, `sp.geo_bubble_map`, `sp.build_bubble_map`

## Description

`sp.bubble_map()` repère les régions correspondantes sur une carte avec, par défaut, un cercle de taille proportionnelle à son centroïde — de vraies bulles, pas des régions remplies ; prendre `variant="filled"` plutôt quand ce sont les régions elles-mêmes que le lecteur doit comparer, pas une magnitude. `map` sélectionne l'ensemble de régions géographiques enregistré à dessiner ; `region` restreint optionnellement le dessin à un seul groupe nommé de cet ensemble. Les trois sont lus en direct depuis le même système de register/inventory utilisé dans tout le framework — rien ci-dessous n'est codé en dur.

## Variantes

<div data-sp-registry-table="variants" data-family="bubble_map"></div>

## Ensembles de régions

<div data-sp-registry-table="regions"></div>

## Paramètres

<div data-sp-registry-table="options" data-family="bubble_map"></div>

---

## Retourne

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

---

<div class="sp-cls sp-open" id="bubblemap-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bubblemap-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bubblemap-fr','proportional',this)"><span class="sp-cic">●</span><span class="sp-clb">Proportionnel</span></button>
<button class="sp-cls-tab" onclick="spCls('bubblemap-fr','filled',this)"><span class="sp-cic">▧</span><span class="sp-clb">Rempli</span></button>
<button class="sp-cls-tab" onclick="spCls('bubblemap-fr','globe',this)"><span class="sp-cic">◉</span><span class="sp-clb">Globe</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bubblemap-fr-proportional">

Le défaut, et la vraie carte à symboles proportionnels : le fond est estompé, et un cercle se place sur le centroïde de chaque région correspondante, son rayon mis à l'échelle par `sqrt(valeur)` pour que ce soit l'*aire* — pas le rayon — qui soit proportionnelle à la magnitude. `min_bubble_size` / `max_bubble_size` règlent la plage de rayon en pixels.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"proportional"</code></span><span><strong>Alias</strong> <code>"proportional"</code> / <code>"bubble"</code> / <code>"graduated"</code> / <code>"sized"</code> / <code>"basic"</code> / <code>"default"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble-map-proportional.html"></iframe>
</div>

<div class="sp-variant" id="bubblemap-fr-filled">

Pas du tout des bulles, volontairement : chaque région correspondante remplie en aplat de sa propre couleur de palette et étiquetée à son centroïde — une carte de mise en évidence catégorielle, pour quand ce sont les régions elles-mêmes que le lecteur doit comparer plutôt qu'une magnitude. Utiliser plutôt `choropleth` si la couleur de remplissage doit encoder une valeur continue.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"filled"</code></span><span><strong>Alias</strong> <code>"filled"</code> / <code>"regions"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble-map-filled.html"></iframe>
</div>

<div class="sp-variant" id="bubblemap-fr-globe">

Cercles proportionnels projetés sur une sphère orthographique — chaque centroïde est converti en vraie latitude/longitude puis reprojeté avec une véritable trigonométrie de globe, si bien que les bulles de la face cachée ne sont tout simplement pas dessinées. `center_lat` / `center_lon` orientent la vue (par défaut `15, 10`). Seuls les ensembles de régions exposant une inversion de coordonnées supportent ceci — aujourd'hui, `"world"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"globe"</code></span><span><strong>Alias</strong> <code>"globe"</code> / <code>"orthographic"</code> / <code>"sphere"</code> / <code>"space"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble-map-globe.html"></iframe>
</div>

</div>
</div>

</div>
