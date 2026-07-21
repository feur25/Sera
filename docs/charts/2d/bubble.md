# Bubble Charts

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

`sp.bubble(title, x_values, y_values, sizes, *, variant="basic", categories=None, labels=None, color_values=None, **kwargs) -> Chart`

## Description

`sp.bubble()` is the unified entry point for the entire bubble-chart family. The `variant` keyword selects the rendering strategy — all other arguments remain consistent across variants. A bubble chart extends a 2D scatter plot with a third numeric dimension represented as bubble **area** (not radius), following best practices for perceptual accuracy.
## Variants

<div data-sp-registry-table="variants" data-family="bubble"></div>

## Parameters

<div data-sp-registry-table="options" data-family="bubble"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

---

<div class="sp-cls sp-open" id="bubble-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bubble-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bubble-en','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-en','categorical',this)"><span class="sp-cic">◓</span><span class="sp-clb">Categorical</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-en','labeled',this)"><span class="sp-cic">◉</span><span class="sp-clb">Labeled</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-en','outlined',this)"><span class="sp-cic">○</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-en','negative',this)"><span class="sp-cic">±</span><span class="sp-clb">Negative</span></button></div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bubble-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic</code> / <code>simple</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-basic.html"></iframe>
</div>

<div class="sp-variant" id="bubble-en-categorical">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code> / <code>"grouped"</code></span><span><strong>Aliases</strong> <code>category</code> / <code>groups</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-categorical.html"></iframe>
</div>

<div class="sp-variant" id="bubble-en-labeled">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"labeled"</code> / <code>"text"</code></span><span><strong>Aliases</strong> <code>labels</code> / <code>annotated</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-labeled.html"></iframe>
</div>

<div class="sp-variant" id="bubble-en-outlined">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code> / <code>"hollow"</code></span><span><strong>Aliases</strong> <code>ring</code> / <code>open</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-outlined.html"></iframe>
</div>

<div class="sp-variant" id="bubble-en-negative">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"negative"</code> / <code>"signed"</code></span><span><strong>Aliases</strong> <code>diverging</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-negative.html"></iframe>
</div>

</div>
</div>

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.bubble(title, x_values, y_values, sizes, *, variant="basic", categories=None, labels=None, color_values=None, **kwargs) -> Chart`

<h2>Description</h2>

`sp.bubble()` est le point d'entrée unique de toute la famille des graphiques à bulles. Le paramètre `variant` choisit la stratégie de rendu — tous les autres arguments restent cohérents entre variantes. Une bulle représente une troisième dimension via son **aire** (et non son rayon), pour une lecture perceptive correcte.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="bubble"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="bubble"></div>

---

<h2>Retourne</h2>

`Chart` — objet exposant `.html` et `.show()`.

---

---

<div class="sp-cls sp-open" id="bubble-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bubble-fr')" title="Réduire / développer">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bubble-fr','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-fr','categorical',this)"><span class="sp-cic">◓</span><span class="sp-clb">Catégoriel</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-fr','labeled',this)"><span class="sp-cic">◉</span><span class="sp-clb">Étiqueté</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-fr','outlined',this)"><span class="sp-cic">○</span><span class="sp-clb">Contour</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-fr','negative',this)"><span class="sp-cic">±</span><span class="sp-clb">Négatif</span></button></div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bubble-fr-basic">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic</code> / <code>simple</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-basic.html"></iframe>
</div>

<div class="sp-variant" id="bubble-fr-categorical">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"categorical"</code> / <code>"grouped"</code></span><span><strong>Alias</strong> <code>category</code> / <code>groups</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-categorical.html"></iframe>
</div>

<div class="sp-variant" id="bubble-fr-labeled">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"labeled"</code> / <code>"text"</code></span><span><strong>Alias</strong> <code>labels</code> / <code>annotated</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-labeled.html"></iframe>
</div>

<div class="sp-variant" id="bubble-fr-outlined">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"outlined"</code> / <code>"hollow"</code></span><span><strong>Alias</strong> <code>ring</code> / <code>open</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-outlined.html"></iframe>
</div>

<div class="sp-variant" id="bubble-fr-negative">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"negative"</code> / <code>"signed"</code></span><span><strong>Alias</strong> <code>diverging</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-negative.html"></iframe>
</div>

</div>
</div>

</div><!-- /lang-fr -->
