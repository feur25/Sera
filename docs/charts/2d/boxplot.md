# Box Plot

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

`sp.boxplot(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

## Description

`sp.boxplot()` is the unified entry point for the entire box-plot family. The `variant` keyword selects the rendering strategy — every other argument stays consistent across variants. Quartiles, 1.5×IQR whiskers and outliers are computed in pure Rust without NumPy or pandas.
## Variants

<div data-sp-registry-table="variants" data-family="boxplot"></div>

## Parameters

<div data-sp-registry-table="options" data-family="boxplot"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="bx-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bx-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bx-en','basic',this)"><span class="sp-cic">▭</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','horizontal',this)"><span class="sp-cic">⇆</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','notched',this)"><span class="sp-cic">◊</span><span class="sp-clb">Notched</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','grouped',this)"><span class="sp-cic">▦</span><span class="sp-clb">Grouped</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','points',this)"><span class="sp-cic">⁝</span><span class="sp-clb">Points</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','outliers',this)"><span class="sp-cic">⊙</span><span class="sp-clb">Outliers</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','strip',this)"><span class="sp-cic">⋮</span><span class="sp-clb">Strip</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','violin',this)"><span class="sp-cic">◇</span><span class="sp-clb">Violin</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','letter',this)"><span class="sp-cic">≡</span><span class="sp-clb">Letter-Value</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bx-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-basic.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-horizontal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code> / <code>"hbox"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-horizontal.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-notched">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"notched"</code> / <code>"ci"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-notched.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-grouped">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"grouped"</code> / <code>"side_by_side"</code></span><span><strong>Required</strong> <code>labels</code>, <code>series</code>, <code>series_names</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-grouped.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-points">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"points"</code> / <code>"all_points"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-points.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-outliers">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outliers"</code> / <code>"fliers"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-outliers.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-strip">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"strip"</code> / <code>"swarm"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-strip.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-violin">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"violin"</code> / <code>"density"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-violin.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-letter">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"letter_value"</code> / <code>"boxen"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Optional</strong> <code>boxen_depth</code> (2–7)</span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-letter_value.html"></iframe>

</div>

</div>
</div>

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.boxplot(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

<h2>Description</h2>

`sp.boxplot()` est le point d'entrée unique pour toute la famille des boîtes à moustaches. Le paramètre `variant` sélectionne la stratégie de rendu — tous les autres arguments restent identiques entre les variantes. Quartiles, moustaches 1,5×IQR et valeurs aberrantes sont calculés en pur Rust, sans NumPy ni pandas.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="boxplot"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="boxplot"></div>

---

<h2>Retourne</h2>

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

<div class="sp-cls sp-open" id="bx-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bx-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bx-fr','basic',this)"><span class="sp-cic">▭</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-fr','horizontal',this)"><span class="sp-cic">⇆</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-fr','notched',this)"><span class="sp-cic">◊</span><span class="sp-clb">Encoché</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-fr','grouped',this)"><span class="sp-cic">▦</span><span class="sp-clb">Groupé</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-fr','points',this)"><span class="sp-cic">⁝</span><span class="sp-clb">Points</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-fr','outliers',this)"><span class="sp-cic">⊙</span><span class="sp-clb">Aberrants</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-fr','strip',this)"><span class="sp-cic">⋮</span><span class="sp-clb">Bande</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-fr','violin',this)"><span class="sp-cic">◇</span><span class="sp-clb">Violon</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-fr','letter',this)"><span class="sp-cic">≡</span><span class="sp-clb">Valeurs-Lettres</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bx-fr-basic"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/boxplot-basic.html"></iframe></div>

<div class="sp-variant" id="bx-fr-horizontal"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"horizontal"</code> / <code>"hbox"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/boxplot-horizontal.html"></iframe></div>

<div class="sp-variant" id="bx-fr-notched"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"notched"</code> / <code>"ci"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/boxplot-notched.html"></iframe></div>

<div class="sp-variant" id="bx-fr-grouped"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"grouped"</code> / <code>"side_by_side"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>series</code>, <code>series_names</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/boxplot-grouped.html"></iframe></div>

<div class="sp-variant" id="bx-fr-points"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"points"</code> / <code>"all_points"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/boxplot-points.html"></iframe></div>

<div class="sp-variant" id="bx-fr-outliers"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"outliers"</code> / <code>"fliers"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/boxplot-outliers.html"></iframe></div>

<div class="sp-variant" id="bx-fr-strip"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"strip"</code> / <code>"swarm"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/boxplot-strip.html"></iframe></div>

<div class="sp-variant" id="bx-fr-violin"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"violin"</code> / <code>"density"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/boxplot-violin.html"></iframe></div>

<div class="sp-variant" id="bx-fr-letter"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"letter_value"</code> / <code>"boxen"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/boxplot-letter_value.html"></iframe></div>

</div>
</div>

</div><!-- /lang-fr -->
