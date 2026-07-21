# Violin Plot

<div class="lang-en">

<style>
.sp-cls.sp-open .sp-cls-rail{min-width:180px;padding:18px 8px}
.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.violin(title, labels=None, values=None, *, variant="box", **kwargs) -> Chart`

## Description

`sp.violin()` is the unified entry point for the entire violin-plot family. The `variant` keyword selects the rendering strategy — every other argument stays consistent across variants. The kernel-density estimation, quartiles and statistics are computed in pure Rust, no NumPy or pandas required.
## Variants

<div data-sp-registry-table="variants" data-family="violin"></div>

## Parameters

<div data-sp-registry-table="options" data-family="violin"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="vl-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('vl-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab" onclick="spCls('vl-en','basic',this)"><span class="sp-cic">◇</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab sp-cact" onclick="spCls('vl-en','box',this)"><span class="sp-cic">▭</span><span class="sp-clb">Box</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','quartile',this)"><span class="sp-cic">≣</span><span class="sp-clb">Quartile</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','mean',this)"><span class="sp-cic">μ</span><span class="sp-clb">Mean</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','points',this)"><span class="sp-cic">⁝</span><span class="sp-clb">Points</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','strip',this)"><span class="sp-cic">⋮</span><span class="sp-clb">Strip</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','horizontal',this)"><span class="sp-cic">⇆</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','split',this)"><span class="sp-cic">◐</span><span class="sp-clb">Split</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','half',this)"><span class="sp-cic">◗</span><span class="sp-clb">Half</span></button></div>
<div class="sp-cls-body">

<div class="sp-variant" id="vl-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-basic.html"></iframe>

</div>

<div class="sp-variant sp-von" id="vl-en-box">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"box"</code> (default)</span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-box.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-quartile">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"quartile"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-quartile.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-mean">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mean"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-mean.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-points">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"points"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-points.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-strip">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"strip"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-strip.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-horizontal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-horizontal.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-split">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"split"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-split.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-half">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"half"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-half.html"></iframe>

</div>

</div>
</div>

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.violin(title, labels=None, values=None, *, variant="box", **kwargs) -> Chart`

<h2>Description</h2>

`sp.violin()` est le point d'entrée unique pour toute la famille des violons. Le paramètre `variant` sélectionne la stratégie de rendu — tous les autres arguments restent identiques entre les variantes. L'estimation de densité par noyau (KDE), les quartiles et les statistiques sont calculés en pur Rust, sans NumPy ni pandas.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="violin"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="violin"></div>

---

<h2>Retourne</h2>

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

<div class="sp-cls sp-open" id="vl-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('vl-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','basic',this)"><span class="sp-cic">◇</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab sp-cact" onclick="spCls('vl-fr','box',this)"><span class="sp-cic">▭</span><span class="sp-clb">Boîte</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','quartile',this)"><span class="sp-cic">≣</span><span class="sp-clb">Quartile</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','mean',this)"><span class="sp-cic">μ</span><span class="sp-clb">Moyenne</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','points',this)"><span class="sp-cic">⁝</span><span class="sp-clb">Points</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','strip',this)"><span class="sp-cic">⋮</span><span class="sp-clb">Bande</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','horizontal',this)"><span class="sp-cic">⇆</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','split',this)"><span class="sp-cic">◐</span><span class="sp-clb">Scindé</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','half',this)"><span class="sp-cic">◗</span><span class="sp-clb">Demi</span></button></div>
<div class="sp-cls-body">

<div class="sp-variant" id="vl-fr-basic"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-basic.html"></iframe></div>

<div class="sp-variant sp-von" id="vl-fr-box"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"box"</code> (par défaut)</span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-box.html"></iframe></div>

<div class="sp-variant" id="vl-fr-quartile"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"quartile"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-quartile.html"></iframe></div>

<div class="sp-variant" id="vl-fr-mean"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"mean"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-mean.html"></iframe></div>

<div class="sp-variant" id="vl-fr-points"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"points"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-points.html"></iframe></div>

<div class="sp-variant" id="vl-fr-strip"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"strip"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-strip.html"></iframe></div>

<div class="sp-variant" id="vl-fr-horizontal"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"horizontal"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-horizontal.html"></iframe></div>

<div class="sp-variant" id="vl-fr-split"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"split"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-split.html"></iframe></div>

<div class="sp-variant" id="vl-fr-half"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"half"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-half.html"></iframe></div>

</div>
</div>

</div><!-- /lang-fr -->
