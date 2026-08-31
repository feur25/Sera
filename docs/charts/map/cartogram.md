# Cartogram

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

`sp.cartogram(title, labels=None, values=None, *, lats=None, lons=None, variant="dorling", map="world", region=None, **kwargs) -> Chart`

Aliases: `sp.cartogram`, `sp.cartogram_chart`, `sp.dorling_cartogram`, `sp.geo_cartogram`, `sp.proportional_cartogram`, `sp.relaxed_map`, `sp.build_cartogram`

## Description

`sp.cartogram()` replaces true geography with a glyph sized by `values` at every `lats`/`lons` point, then runs a real iterative relaxation so overlapping glyphs push apart while a weak pull keeps each one near its true position — the classic technique named after cartographer Danny Dorling, used for things like election-result or population maps where area itself (not just color) should carry the number. A dashed line appears wherever a glyph had to move far from where it really sits, the standard way a real Dorling cartogram shows its own distortion honestly. `map` / `region` restrict which registered region set's outlines draw as faint background context, read live from the same register/inventory system the rest of the framework uses.

## Variants

<div data-sp-registry-table="variants" data-family="cartogram"></div>

## Region Sets

<div data-sp-registry-table="regions"></div>

## Parameters

<div data-sp-registry-table="options" data-family="cartogram"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="cartogram-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('cartogram-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('cartogram-en','dorling',this)"><span class="sp-cic">●</span><span class="sp-clb">Dorling</span></button>
<button class="sp-cls-tab" onclick="spCls('cartogram-en','demers',this)"><span class="sp-cic">■</span><span class="sp-clb">Demers</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="cartogram-en-dorling">

The default: a circle per point, radius scaled by `sqrt(value)` so area reads honestly, fill color repeating the same value on a sequential ramp for a redundant, easy-to-scan encoding. Circles start at their true projected position and relax apart over 180 iterations of pairwise repulsion balanced against a weak pull back home — real physics-style relaxation, not a packed layout that ignores geography.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dorling"</code></span><span><strong>Aliases</strong> <code>"dorling"</code> / <code>"circles"</code> / <code>"basic"</code> / <code>"default"</code> / <code>"bubble_relax"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/cartogram-dorling.html"></iframe>
</div>

<div class="sp-variant" id="cartogram-en-demers">

The same relaxation with square glyphs instead of circles — a second named cartogram style, after cartographer Steve Demers, useful whenever a grid-like reading suits the data better than a cluster of circles.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"demers"</code></span><span><strong>Aliases</strong> <code>"demers"</code> / <code>"squares"</code> / <code>"grid_relax"</code> / <code>"boxes"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/cartogram-demers.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.cartogram(title, labels=None, values=None, *, lats=None, lons=None, variant="dorling", map="world", region=None, **kwargs) -> Chart`

Alias : `sp.cartogram`, `sp.cartogram_chart`, `sp.dorling_cartogram`, `sp.geo_cartogram`, `sp.proportional_cartogram`, `sp.relaxed_map`, `sp.build_cartogram`

## Description

`sp.cartogram()` remplace la géographie réelle par un glyphe dimensionné selon `values` à chaque point `lats`/`lons`, puis exécute une vraie relaxation itérative pour que les glyphes qui se chevauchent s'écartent tandis qu'une faible attraction les maintient près de leur position réelle — la technique classique nommée d'après le cartographe Danny Dorling, utilisée par exemple pour des cartes de résultats électoraux ou de population où c'est l'aire elle-même (pas seulement la couleur) qui doit porter le chiffre. Une ligne en pointillés apparaît partout où un glyphe a dû s'éloigner beaucoup de sa position réelle, la façon standard dont un vrai cartogramme de Dorling montre honnêtement sa propre distorsion. `map` / `region` restreignent l'ensemble de régions enregistré dont les contours se dessinent en arrière-plan discret, lus en direct depuis le même système de register/inventory utilisé dans tout le framework.

## Variantes

<div data-sp-registry-table="variants" data-family="cartogram"></div>

## Ensembles de régions

<div data-sp-registry-table="regions"></div>

## Paramètres

<div data-sp-registry-table="options" data-family="cartogram"></div>

---

## Retourne

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

---

<div class="sp-cls sp-open" id="cartogram-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('cartogram-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('cartogram-fr','dorling',this)"><span class="sp-cic">●</span><span class="sp-clb">Dorling</span></button>
<button class="sp-cls-tab" onclick="spCls('cartogram-fr','demers',this)"><span class="sp-cic">■</span><span class="sp-clb">Demers</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="cartogram-fr-dorling">

Le défaut : un cercle par point, rayon mis à l'échelle par `sqrt(valeur)` pour que l'aire se lise honnêtement, couleur de remplissage reprenant la même valeur sur une rampe séquentielle pour un encodage redondant, facile à parcourir du regard. Les cercles démarrent à leur position projetée réelle et s'écartent par relaxation sur 180 itérations de répulsion par paires équilibrée contre une faible attraction vers leur position d'origine — une vraie relaxation façon physique, pas une disposition compacte qui ignore la géographie.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"dorling"</code></span><span><strong>Alias</strong> <code>"dorling"</code> / <code>"circles"</code> / <code>"basic"</code> / <code>"default"</code> / <code>"bubble_relax"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/cartogram-dorling.html"></iframe>
</div>

<div class="sp-variant" id="cartogram-fr-demers">

La même relaxation avec des glyphes carrés plutôt que circulaires — un second style de cartogramme nommé, d'après le cartographe Steve Demers, utile chaque fois qu'une lecture en grille convient mieux aux données qu'un amas de cercles.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"demers"</code></span><span><strong>Alias</strong> <code>"demers"</code> / <code>"squares"</code> / <code>"grid_relax"</code> / <code>"boxes"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/cartogram-demers.html"></iframe>
</div>

</div>
</div>

</div>
