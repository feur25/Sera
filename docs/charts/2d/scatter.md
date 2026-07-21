# Scatter Charts

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:380px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.scatter(title, x_values, y_values, *, variant="basic", categories=None, labels=None, color_values=None, **kwargs) -> Chart`

## Description

`sp.scatter()` is the unified entry point for the entire scatter family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. Scatter plots are the canonical way to display the joint distribution of two numeric variables; SeraPlot adds optional grouping, continuous color, distinct marker shapes, on-point labels and OLS regression — all in pure Rust SVG, thousands of times faster than Plotly.
## Variants

<div data-sp-registry-table="variants" data-family="scatter"></div>

## Parameters

<div data-sp-registry-table="options" data-family="scatter"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="scatter-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('scatter-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('scatter-en','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','categorical',this)"><span class="sp-cic">◓</span><span class="sp-clb">Categorical</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','symbols',this)"><span class="sp-cic">◆</span><span class="sp-clb">Symbols</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','labeled',this)"><span class="sp-cic">◉</span><span class="sp-clb">Labeled</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','regression',this)"><span class="sp-cic">↗</span><span class="sp-clb">Regression</span></button></div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="scatter-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / simple / default</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-basic.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-categorical">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code></span><span><strong>Aliases</strong> <code>categorical / grouped / category</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-categorical.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-symbols">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"symbols"</code></span><span><strong>Aliases</strong> <code>symbols / shapes / markers</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-symbols.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-labeled">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"labeled"</code></span><span><strong>Aliases</strong> <code>labeled / labels / text</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-labeled.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-regression">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"regression"</code></span><span><strong>Aliases</strong> <code>regression / trendline / fit</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-regression.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.scatter(title, x_values, y_values, *, variant="basic", categories=None, labels=None, color_values=None, **kwargs) -> Chart`

<h2>Description</h2>

`sp.scatter()` est le point d'entrée unifié de toute la famille scatter. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments gardent le même nom d'une variante à l'autre. Les nuages de points sont la façon canonique d'afficher la distribution conjointe de deux variables numériques ; SeraPlot ajoute groupement optionnel, couleur continue, formes de marqueurs distinctes, étiquettes sur les points et régression OLS — le tout en SVG Rust pur, des milliers de fois plus rapide que Plotly.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="scatter"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="scatter"></div>

---

<h2>Retour</h2>

`Chart` — objet exposant `.html` et `.show()`.

---

<div class="sp-cls sp-open" id="scatter-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('scatter-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('scatter-fr','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">De base</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','categorical',this)"><span class="sp-cic">◓</span><span class="sp-clb">Catégoriel</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','symbols',this)"><span class="sp-cic">◆</span><span class="sp-clb">Symboles</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','labeled',this)"><span class="sp-cic">◉</span><span class="sp-clb">Étiquetés</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','regression',this)"><span class="sp-cic">↗</span><span class="sp-clb">Régression</span></button></div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="scatter-fr-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / simple / default</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-basic.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-categorical">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code></span><span><strong>Aliases</strong> <code>categorical / grouped / category</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-categorical.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-symbols">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"symbols"</code></span><span><strong>Aliases</strong> <code>symbols / shapes / markers</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-symbols.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-labeled">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"labeled"</code></span><span><strong>Aliases</strong> <code>labeled / labels / text</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-labeled.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-regression">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"regression"</code></span><span><strong>Aliases</strong> <code>regression / trendline / fit</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-regression.html"></iframe>
</div>

</div>
</div>

</div>
