# Flow Map

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

`sp.flow_map(title, labels=None, *, edges_i=None, edges_j=None, edges_w=None, variant="arc", map="world", region=None, **kwargs) -> Chart`

Aliases: `sp.flow_map`, `sp.flowmap`, `sp.flow_map_chart`, `sp.geo_flow`, `sp.connection_map`, `sp.great_circle_map`

## Description

`sp.flow_map()` draws origin → destination flows between matched regions. `labels` names every node; `edges_i` / `edges_j` are index pairs into `labels` for each flow's origin and destination (the same edge-list shape `sankey()` uses); `edges_w` is each flow's magnitude, which sets line width. `variant` selects the curve style; `map` selects which registered geographic region set to draw; `region` optionally restricts drawing to one named group inside that set. All of this is read live from the same register/inventory system the rest of the framework uses — nothing below is hardcoded.

## Variants

<div data-sp-registry-table="variants" data-family="flow_map"></div>

## Region Sets

<div data-sp-registry-table="regions"></div>

## Parameters

<div data-sp-registry-table="options" data-family="flow_map"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="flowmap-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('flowmap-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('flowmap-en','arc',this)"><span class="sp-cic">⌒</span><span class="sp-clb">Arc</span></button>
<button class="sp-cls-tab" onclick="spCls('flowmap-en','straight',this)"><span class="sp-cic">╱</span><span class="sp-clb">Straight</span></button>
<button class="sp-cls-tab" onclick="spCls('flowmap-en','animated',this)"><span class="sp-cic">┄</span><span class="sp-clb">Animated</span></button>
<button class="sp-cls-tab" onclick="spCls('flowmap-en','ribbon',this)"><span class="sp-cic">〰</span><span class="sp-clb">Ribbon</span></button>
<button class="sp-cls-tab" onclick="spCls('flowmap-en','track',this)"><span class="sp-cic">➤</span><span class="sp-clb">Track</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="flowmap-en-arc">

Each flow bows outward along a quadratic curve, the classic "flight path" look — reads cleanly even when several flows share an endpoint, since they fan out instead of stacking on the same line.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arc"</code></span><span><strong>Aliases</strong> <code>"arc"</code> / <code>"basic"</code> / <code>"default"</code> / <code>"curved"</code> / <code>"great_circle"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/flow-map-arc.html"></iframe>
</div>

<div class="sp-variant" id="flowmap-en-straight">

The direct-line reading: origin and destination joined by a plain segment, width still scaled to each flow's magnitude.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"straight"</code></span><span><strong>Aliases</strong> <code>"straight"</code> / <code>"line"</code> / <code>"direct"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/flow-map-straight.html"></iframe>
</div>

<div class="sp-variant" id="flowmap-en-animated">

The arc plus real motion: a thin white dashed stroke rides on top of every flow, driven by a `stroke-dashoffset` `@keyframes` animation embedded right in the svg, so direction reads as travel instead of a static line (falls back to still when the viewer has `prefers-reduced-motion` on).

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"animated"</code></span><span><strong>Aliases</strong> <code>"animated"</code> / <code>"dashed"</code> / <code>"moving"</code> / <code>"flow_dash"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/flow-map-animated.html"></iframe>
</div>

<div class="sp-variant" id="flowmap-en-ribbon">

Every flow becomes a real tapered band instead of a stroked line — wide at the origin, narrowing toward the destination, sampled along the same curve the arc variant bows through and offset perpendicular to it at each point. Reads like a river or a Sankey band laid over the map.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ribbon"</code></span><span><strong>Aliases</strong> <code>"ribbon"</code> / <code>"tapered"</code> / <code>"band"</code> / <code>"river"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/flow-map-ribbon.html"></iframe>
</div>

<div class="sp-variant" id="flowmap-en-track">

Not pairwise connections at all — a single ordered path through `lats`/`lons`, storm-track style, with `field` as the intensity at each point (wind speed, category, whatever the reader should read off the color). Every point gets its own marker on a low/mid/high color tier, and the most recent leg ends in a direction arrowhead. `labels`/`edges_i`/`edges_j`/`edges_w` are ignored for this variant.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"track"</code></span><span><strong>Aliases</strong> <code>"track"</code> / <code>"storm_track"</code> / <code>"path_track"</code> / <code>"hurricane"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/flow-map-track.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.flow_map(title, labels=None, *, edges_i=None, edges_j=None, edges_w=None, variant="arc", map="world", region=None, **kwargs) -> Chart`

Alias : `sp.flow_map`, `sp.flowmap`, `sp.flow_map_chart`, `sp.geo_flow`, `sp.connection_map`, `sp.great_circle_map`

## Description

`sp.flow_map()` dessine des flux origine → destination entre régions correspondantes. `labels` nomme chaque nœud ; `edges_i` / `edges_j` sont des paires d'indices dans `labels` pour l'origine et la destination de chaque flux (la même forme de liste d'arêtes que `sankey()`) ; `edges_w` est la magnitude de chaque flux, qui règle l'épaisseur du trait. `variant` sélectionne le style de courbe ; `map` sélectionne l'ensemble de régions géographiques enregistré à dessiner ; `region` restreint optionnellement le dessin à un seul groupe nommé de cet ensemble. Tout ceci est lu en direct depuis le même système de register/inventory utilisé dans tout le framework — rien ci-dessous n'est codé en dur.

## Variantes

<div data-sp-registry-table="variants" data-family="flow_map"></div>

## Ensembles de régions

<div data-sp-registry-table="regions"></div>

## Paramètres

<div data-sp-registry-table="options" data-family="flow_map"></div>

---

## Retourne

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

---

<div class="sp-cls sp-open" id="flowmap-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('flowmap-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('flowmap-fr','arc',this)"><span class="sp-cic">⌒</span><span class="sp-clb">Arc</span></button>
<button class="sp-cls-tab" onclick="spCls('flowmap-fr','straight',this)"><span class="sp-cic">╱</span><span class="sp-clb">Droite</span></button>
<button class="sp-cls-tab" onclick="spCls('flowmap-fr','animated',this)"><span class="sp-cic">┄</span><span class="sp-clb">Animé</span></button>
<button class="sp-cls-tab" onclick="spCls('flowmap-fr','ribbon',this)"><span class="sp-cic">〰</span><span class="sp-clb">Ruban</span></button>
<button class="sp-cls-tab" onclick="spCls('flowmap-fr','track',this)"><span class="sp-cic">➤</span><span class="sp-clb">Trajectoire</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="flowmap-fr-arc">

Chaque flux s'arque le long d'une courbe quadratique, l'allure classique « trajet de vol » — reste lisible même quand plusieurs flux partagent une extrémité, puisqu'ils s'éventent au lieu de s'empiler sur la même ligne.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"arc"</code></span><span><strong>Alias</strong> <code>"arc"</code> / <code>"basic"</code> / <code>"default"</code> / <code>"curved"</code> / <code>"great_circle"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/flow-map-arc.html"></iframe>
</div>

<div class="sp-variant" id="flowmap-fr-straight">

La lecture en ligne directe : origine et destination reliées par un segment simple, l'épaisseur restant mise à l'échelle de la magnitude de chaque flux.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"straight"</code></span><span><strong>Alias</strong> <code>"straight"</code> / <code>"line"</code> / <code>"direct"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/flow-map-straight.html"></iframe>
</div>

<div class="sp-variant" id="flowmap-fr-animated">

L'arc plus un vrai mouvement : un fin trait blanc en pointillés chevauche chaque flux, piloté par une animation `@keyframes` sur `stroke-dashoffset` intégrée directement dans le svg, si bien que la direction se lit comme un déplacement plutôt qu'une ligne statique (revient à l'immobile si le lecteur a activé `prefers-reduced-motion`).

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"animated"</code></span><span><strong>Alias</strong> <code>"animated"</code> / <code>"dashed"</code> / <code>"moving"</code> / <code>"flow_dash"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/flow-map-animated.html"></iframe>
</div>

<div class="sp-variant" id="flowmap-fr-ribbon">

Chaque flux devient une vraie bande effilée plutôt qu'un trait — large à l'origine, se resserrant vers la destination, échantillonnée le long de la même courbe que la variante arc et décalée perpendiculairement à celle-ci à chaque point. Se lit comme une rivière ou une bande Sankey posée sur la carte.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"ribbon"</code></span><span><strong>Alias</strong> <code>"ribbon"</code> / <code>"tapered"</code> / <code>"band"</code> / <code>"river"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/flow-map-ribbon.html"></iframe>
</div>

<div class="sp-variant" id="flowmap-fr-track">

Pas du tout des connexions par paires — un unique chemin ordonné à travers `lats`/`lons`, façon trajectoire de tempête, avec `field` comme intensité à chaque point (vitesse du vent, catégorie, tout ce que la couleur doit faire lire). Chaque point reçoit son propre marqueur sur un palier de couleur bas/moyen/haut, et le dernier segment se termine par une flèche de direction. `labels`/`edges_i`/`edges_j`/`edges_w` sont ignorés pour cette variante.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"track"</code></span><span><strong>Alias</strong> <code>"track"</code> / <code>"storm_track"</code> / <code>"path_track"</code> / <code>"hurricane"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/flow-map-track.html"></iframe>
</div>

</div>
</div>

</div>
