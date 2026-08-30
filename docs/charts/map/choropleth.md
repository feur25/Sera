# Choropleth Map

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

`sp.choropleth(title, labels=None, values=None, *, variant="sequential", map="world", region=None, center_lat=None, center_lon=None, **kwargs) -> Chart`

Aliases: `sp.choropleth`, `sp.choropleths`, `sp.choropleth_map`, `sp.choropleth_chart`, `sp.geo_map`, `sp.build_choropleth`

## Description

`sp.choropleth()` fills country or region polygons by a scalar value. `variant` selects the color-classification strategy; `map` selects which registered geographic region set to draw; `region` optionally restricts drawing to one named group inside that set (e.g. a continent or a census region). All three are read live from the same register/inventory system the rest of the framework uses — nothing below is hardcoded.

## Variants

<div data-sp-registry-table="variants" data-family="choropleth"></div>

## Region Sets

<div data-sp-registry-table="regions"></div>

## Parameters

<div data-sp-registry-table="options" data-family="choropleth"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="choropleth-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('choropleth-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('choropleth-en','sequential',this)"><span class="sp-cic">▧</span><span class="sp-clb">Sequential</span></button>
<button class="sp-cls-tab" onclick="spCls('choropleth-en','binned',this)"><span class="sp-cic">▤</span><span class="sp-clb">Binned</span></button>
<button class="sp-cls-tab" onclick="spCls('choropleth-en','diverging',this)"><span class="sp-cic">◐</span><span class="sp-clb">Diverging</span></button>
<button class="sp-cls-tab" onclick="spCls('choropleth-en','orthographic',this)"><span class="sp-cic">◉</span><span class="sp-clb">Orthographic</span></button>
<button class="sp-cls-tab" onclick="spCls('choropleth-en','polar',this)"><span class="sp-cic">◎</span><span class="sp-clb">Polar</span></button>
<button class="sp-cls-tab" onclick="spCls('choropleth-en','bivariate',this)"><span class="sp-cic">▦</span><span class="sp-clb">Bivariate</span></button>
<button class="sp-cls-tab" onclick="spCls('choropleth-en','dot_density',this)"><span class="sp-cic">⁘</span><span class="sp-clb">Dot density</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="choropleth-en-sequential">

Continuous heat gradient from the lowest to the highest value in the visible set — the classic choropleth, one smooth color ramp.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"sequential"</code></span><span><strong>Aliases</strong> <code>"sequential"</code> / <code>"basic"</code> / <code>"default"</code> / <code>"heat"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-sequential.html"></iframe>
</div>

<div class="sp-variant" id="choropleth-en-binned">

Splits the values into `bins` discrete quantile classes (equal counts per class, not equal-width) and paints each class its own step of a `viridis` scale — the standard cartographic technique for reading patterns at a glance instead of guessing shades on a gradient.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"binned"</code></span><span><strong>Aliases</strong> <code>"binned"</code> / <code>"quantile"</code> / <code>"classed"</code> / <code>"steps"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-binned.html"></iframe>
</div>

<div class="sp-variant" id="choropleth-en-diverging">

Red–white–blue diverging scale centered on `diverging_midpoint` (default `0`) — built for signed data where the story is above/below a reference point: year-over-year change, deviation from a target, a delta rather than a magnitude.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"diverging"</code></span><span><strong>Aliases</strong> <code>"diverging"</code> / <code>"delta"</code> / <code>"change"</code> / <code>"rdbu"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-diverging.html"></iframe>
</div>

<div class="sp-variant" id="choropleth-en-orthographic">

The globe, seen from space: every polygon vertex is converted from map pixels to real latitude/longitude and re-projected with actual orthographic sphere math, then clipped to the visible hemisphere. `center_lat` / `center_lon` aim the view (default `15, 10`). Only region sets that expose a coordinate inversion support this — today, `"world"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"orthographic"</code></span><span><strong>Aliases</strong> <code>"orthographic"</code> / <code>"globe"</code> / <code>"sphere"</code> / <code>"space"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-orthographic.html"></iframe>
</div>

<div class="sp-variant" id="choropleth-en-polar">

Azimuthal equidistant, centered on a pole by default (`center_lat=90, center_lon=0`) — the classic polar-projection read, where distance from the center is true distance from the chosen point.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"polar"</code></span><span><strong>Aliases</strong> <code>"polar"</code> / <code>"azimuthal"</code> / <code>"pole"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-polar.html"></iframe>
</div>

<div class="sp-variant" id="choropleth-en-bivariate">

Two value series at once instead of one: `values` and `secondary_values` each split into three bins, looked up in a 3×3 color grid instead of a single ramp, so a single glance separates "high on both", "high on one, low on the other", and "low on both" — a technique real cartographers use and almost no charting library ships.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bivariate"</code></span><span><strong>Aliases</strong> <code>"bivariate"</code> / <code>"two_variable"</code> / <code>"cross"</code> / <code>"dual"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-bivariate.html"></iframe>
</div>

<div class="sp-variant" id="choropleth-en-dot_density">

One dot per fixed unit of value, scattered at random inside the real region outline (rejection-sampled against the actual polygon, not just its bounding box) instead of one flat fill color — texture and density carry the magnitude the way an old-school population map does.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dot_density"</code></span><span><strong>Aliases</strong> <code>"dot_density"</code> / <code>"dots"</code> / <code>"stipple"</code> / <code>"scatter_fill"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-dot_density.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.choropleth(title, labels=None, values=None, *, variant="sequential", map="world", region=None, center_lat=None, center_lon=None, **kwargs) -> Chart`

Alias : `sp.choropleth`, `sp.choropleths`, `sp.choropleth_map`, `sp.choropleth_chart`, `sp.geo_map`, `sp.build_choropleth`

## Description

`sp.choropleth()` remplit des polygones de pays ou de régions selon une valeur scalaire. `variant` sélectionne la stratégie de classification des couleurs ; `map` sélectionne l'ensemble de régions géographiques enregistré à dessiner ; `region` restreint optionnellement le dessin à un seul groupe nommé de cet ensemble (un continent, une région de recensement...). Les trois sont lus en direct depuis le même système de register/inventory utilisé dans tout le framework — rien ci-dessous n'est codé en dur.

## Variantes

<div data-sp-registry-table="variants" data-family="choropleth"></div>

## Ensembles de régions

<div data-sp-registry-table="regions"></div>

## Paramètres

<div data-sp-registry-table="options" data-family="choropleth"></div>

---

## Retourne

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

---

<div class="sp-cls sp-open" id="choropleth-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('choropleth-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('choropleth-fr','sequential',this)"><span class="sp-cic">▧</span><span class="sp-clb">Séquentiel</span></button>
<button class="sp-cls-tab" onclick="spCls('choropleth-fr','binned',this)"><span class="sp-cic">▤</span><span class="sp-clb">Par classes</span></button>
<button class="sp-cls-tab" onclick="spCls('choropleth-fr','diverging',this)"><span class="sp-cic">◐</span><span class="sp-clb">Divergent</span></button>
<button class="sp-cls-tab" onclick="spCls('choropleth-fr','orthographic',this)"><span class="sp-cic">◉</span><span class="sp-clb">Orthographique</span></button>
<button class="sp-cls-tab" onclick="spCls('choropleth-fr','polar',this)"><span class="sp-cic">◎</span><span class="sp-clb">Polaire</span></button>
<button class="sp-cls-tab" onclick="spCls('choropleth-fr','bivariate',this)"><span class="sp-cic">▦</span><span class="sp-clb">Bivarié</span></button>
<button class="sp-cls-tab" onclick="spCls('choropleth-fr','dot_density',this)"><span class="sp-cic">⁘</span><span class="sp-clb">Densité de points</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="choropleth-fr-sequential">

Dégradé continu de la valeur la plus basse à la plus haute dans l'ensemble visible — le choroplèthe classique, une seule rampe de couleur lisse.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"sequential"</code></span><span><strong>Alias</strong> <code>"sequential"</code> / <code>"basic"</code> / <code>"default"</code> / <code>"heat"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-sequential.html"></iframe>
</div>

<div class="sp-variant" id="choropleth-fr-binned">

Répartit les valeurs en `bins` classes de quantiles discrètes (effectifs égaux, pas des largeurs égales) et peint chaque classe avec son propre palier d'une échelle `viridis` — la technique cartographique standard pour lire les motifs d'un coup d'œil plutôt que deviner des nuances sur un dégradé.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"binned"</code></span><span><strong>Alias</strong> <code>"binned"</code> / <code>"quantile"</code> / <code>"classed"</code> / <code>"steps"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-binned.html"></iframe>
</div>

<div class="sp-variant" id="choropleth-fr-diverging">

Échelle divergente rouge–blanc–bleu centrée sur `diverging_midpoint` (par défaut `0`) — conçue pour des données signées où l'histoire se joue au-dessus/en dessous d'un point de référence : évolution d'une année sur l'autre, écart à une cible, une différence plutôt qu'une magnitude.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"diverging"</code></span><span><strong>Alias</strong> <code>"diverging"</code> / <code>"delta"</code> / <code>"change"</code> / <code>"rdbu"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-diverging.html"></iframe>
</div>

<div class="sp-variant" id="choropleth-fr-orthographic">

Le globe, vu depuis l'espace : chaque sommet de polygone est converti des pixels de la carte vers une vraie latitude/longitude puis reprojeté avec une véritable trigonométrie orthographique de sphère, puis découpé à l'hémisphère visible. `center_lat` / `center_lon` orientent la vue (par défaut `15, 10`). Seuls les ensembles de régions exposant une inversion de coordonnées supportent ceci — aujourd'hui, `"world"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"orthographic"</code></span><span><strong>Alias</strong> <code>"orthographic"</code> / <code>"globe"</code> / <code>"sphere"</code> / <code>"space"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-orthographic.html"></iframe>
</div>

<div class="sp-variant" id="choropleth-fr-polar">

Azimutale équidistante, centrée par défaut sur un pôle (`center_lat=90, center_lon=0`) — la lecture classique en projection polaire, où la distance au centre est la vraie distance au point choisi.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"polar"</code></span><span><strong>Alias</strong> <code>"polar"</code> / <code>"azimuthal"</code> / <code>"pole"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-polar.html"></iframe>
</div>

<div class="sp-variant" id="choropleth-fr-bivariate">

Deux séries de valeurs à la fois plutôt qu'une seule : `values` et `secondary_values`, chacune répartie en trois classes, recherchées dans une grille de couleurs 3×3 plutôt qu'une seule rampe — un coup d'œil suffit à distinguer "haut sur les deux", "haut sur l'une, bas sur l'autre" et "bas sur les deux", une technique que les vrais cartographes utilisent et que presque aucune librairie de graphiques ne propose.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"bivariate"</code></span><span><strong>Alias</strong> <code>"bivariate"</code> / <code>"two_variable"</code> / <code>"cross"</code> / <code>"dual"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-bivariate.html"></iframe>
</div>

<div class="sp-variant" id="choropleth-fr-dot_density">

Un point par unité fixe de valeur, dispersé aléatoirement à l'intérieur du vrai contour de la région (échantillonnage par rejet contre le polygone réel, pas juste sa boîte englobante) plutôt qu'une seule couleur de remplissage plate — la texture et la densité portent la magnitude, comme sur une vieille carte de population.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"dot_density"</code></span><span><strong>Alias</strong> <code>"dot_density"</code> / <code>"dots"</code> / <code>"stipple"</code> / <code>"scatter_fill"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/choropleth-dot_density.html"></iframe>
</div>

</div>
</div>

</div>
