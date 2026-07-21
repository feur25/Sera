# Histogram Charts

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

`sp.histogram(title, values, *, variant="basic", bins=0, overlay_values=None, color_groups=None, series_names=None, **kwargs) -> Chart`

## Description

`sp.histogram()` is the unified entry point for the entire histogram family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. Histograms are the canonical way to visualize the distribution of a single numeric variable; SeraPlot adds horizontal layout, density normalization, cumulative distribution, stacked groups, A/B overlay and step outline — all in pure Rust SVG, thousands of times faster than Plotly.
## Variants

<div data-sp-registry-table="variants" data-family="histogram"></div>

## Parameters

<div data-sp-registry-table="options" data-family="histogram"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="histogram-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('histogram-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('histogram-en','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','horizontal',this)"><span class="sp-cic">H</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','normalized',this)"><span class="sp-cic">P</span><span class="sp-clb">Density</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','cumulative',this)"><span class="sp-cic">C</span><span class="sp-clb">Cumulative</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','overlay',this)"><span class="sp-cic">O</span><span class="sp-clb">Overlay</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','step',this)"><span class="sp-cic">L</span><span class="sp-clb">Step</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','deluxe',this)"><span class="sp-cic">✦</span><span class="sp-clb">Deluxe</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="histogram-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / simple / default / vertical</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-basic.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-horizontal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / h / barh / hbar</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-horizontal.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-normalized">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"normalized"</code></span><span><strong>Aliases</strong> <code>normalized / probability / density / pdf</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-normalized.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-cumulative">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cumulative"</code></span><span><strong>Aliases</strong> <code>cumulative / cdf / cum</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-cumulative.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-stacked">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stack / stack_by</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-stacked.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-overlay">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"overlay"</code></span><span><strong>Aliases</strong> <code>overlay / overlapping / compare / ab</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-overlay.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-step">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"step"</code></span><span><strong>Aliases</strong> <code>step / outline / stair</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-step.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-deluxe">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"deluxe"</code></span><span><strong>Aliases</strong> <code>deluxe / premium / neon</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-deluxe.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr">
<h2>Signature</h2>

`sp.histogram(title, values, *, variant="basic", bins=0, overlay_values=None, color_groups=None, series_names=None, **kwargs) -> Chart`

<h2>Description</h2>

`sp.histogram()` est le point d'entrée unifié de toute la famille histogramme. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments gardent le même nom d'une variante à l'autre. Les histogrammes sont la façon canonique de visualiser la distribution d'une variable numérique ; SeraPlot ajoute layout horizontal, normalisation densité, distribution cumulative, groupes empilés, superposition A/B et contour en escalier — le tout en SVG Rust pur, des milliers de fois plus rapide que Plotly.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="histogram"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="histogram"></div>

---

<h2>Retour</h2>

`Chart` — objet exposant `.html` et `.show()`.

---

<div class="sp-cls sp-open" id="histogram-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('histogram-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('histogram-fr','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">De base</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','horizontal',this)"><span class="sp-cic">H</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','normalized',this)"><span class="sp-cic">P</span><span class="sp-clb">Densité</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','cumulative',this)"><span class="sp-cic">C</span><span class="sp-clb">Cumulatif</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Empilé</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','overlay',this)"><span class="sp-cic">O</span><span class="sp-clb">Superposé</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','step',this)"><span class="sp-cic">L</span><span class="sp-clb">Contour</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','deluxe',this)"><span class="sp-cic">✦</span><span class="sp-clb">Deluxe</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="histogram-fr-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / simple / default / vertical</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-basic.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-horizontal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / h / barh / hbar</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-horizontal.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-normalized">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"normalized"</code></span><span><strong>Aliases</strong> <code>normalized / probability / density / pdf</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-normalized.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-cumulative">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cumulative"</code></span><span><strong>Aliases</strong> <code>cumulative / cdf / cum</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-cumulative.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-stacked">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stack / stack_by</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-stacked.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-overlay">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"overlay"</code></span><span><strong>Aliases</strong> <code>overlay / overlapping / compare / ab</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-overlay.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-step">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"step"</code></span><span><strong>Aliases</strong> <code>step / outline / stair</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-step.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-deluxe">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"deluxe"</code></span><span><strong>Alias</strong> <code>deluxe / premium / neon</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-deluxe.html"></iframe>
</div>

</div>
</div>

</div>
