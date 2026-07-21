# Heatmap

<div class="lang-en">

<style>
.sp-cls.sp-open .sp-cls-rail{min-width:180px;padding:18px 8px}
.sp-preview-frame{width:100%;height:440px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.heatmap(title, labels=None, values=None, *, variant="basic", col_labels=None, **kwargs) -> Chart`

## Description

`sp.heatmap()` is the unified entry point for the entire heatmap family. The `variant` keyword selects the rendering strategy — every other argument stays consistent across variants. Cell colors are computed in pure Rust, no NumPy required. The matrix is passed as a flat list of length `len(labels) * len(col_labels)` (row-major).
## Variants

<div data-sp-registry-table="variants" data-family="heatmap"></div>

## Parameters

<div data-sp-registry-table="options" data-family="heatmap"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="hm-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('hm-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('hm-en','basic',this)"><span class="sp-cic">▦</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','annotated',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Annotated</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','categorical',this)"><span class="sp-cic">#</span><span class="sp-clb">Categorical</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','unequal',this)"><span class="sp-cic">⊟</span><span class="sp-clb">Unequal</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','log',this)"><span class="sp-cic">㏒</span><span class="sp-clb">Log</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','discrete',this)"><span class="sp-cic">▤</span><span class="sp-clb">Discrete</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','correlation',this)"><span class="sp-cic">◰</span><span class="sp-clb">Correlation</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','density',this)"><span class="sp-cic">░</span><span class="sp-clb">Density</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','contour',this)"><span class="sp-cic">◌</span><span class="sp-clb">Contour</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','temporal',this)"><span class="sp-cic">▥</span><span class="sp-clb">Temporal</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','cluster',this)"><span class="sp-cic">⌘</span><span class="sp-clb">Cluster</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','bubble',this)"><span class="sp-cic">◯</span><span class="sp-clb">Bubble</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','marginal',this)"><span class="sp-cic">⊥</span><span class="sp-clb">Marginal</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','confusion',this)"><span class="sp-cic">⊠</span><span class="sp-clb">Confusion</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','pivot',this)"><span class="sp-cic">⊡</span><span class="sp-clb">Pivot</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="hm-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-basic.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-annotated">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"annotated"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-annotated.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-categorical">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code>, <code>palette</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-categorical.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-unequal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"unequal"</code> / <code>"variable"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code>, <code>widths</code>, <code>ranges</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-unequal.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-log">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"log"</code> / <code>"log_scale"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-log.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-discrete">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"discrete"</code> / <code>"binned"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code>, <code>bins</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-discrete.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-correlation">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"correlation"</code> / <code>"corr"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-correlation.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-density">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"density"</code> / <code>"imshow"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-density.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-contour">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"contour"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-contour.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-temporal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"temporal"</code> / <code>"calendar"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-temporal.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-cluster">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cluster"</code> / <code>"clustermap"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-cluster.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-bubble">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bubble"</code> / <code>"punchcard"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-bubble.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-marginal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"marginal"</code> / <code>"with_marginals"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-marginal.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-confusion">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"confusion"</code> / <code>"confusion_matrix"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-confusion.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-pivot">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"pivot"</code> / <code>"pivot_table"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-pivot.html"></iframe>

</div>

</div>
</div>

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.heatmap(title, matrix, *, variant="basic", x_labels=None, y_labels=None, **kwargs) -> Chart`

<h2>Description</h2>

`sp.heatmap()` est le point d'entrée unique pour toute la famille des cartes de chaleur. Le paramètre `variant` sélectionne la stratégie de rendu — annotée, catégorielle, log, contour, clustering hiérarchique, etc. — tout en partageant la même API de base.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="heatmap"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="heatmap"></div>

---

<h2>Retourne</h2>

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

<div class="sp-cls sp-open" id="hm-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('hm-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('hm-fr','basic',this)"><span class="sp-cic">▦</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','annotated',this)"><span class="sp-cic">▤</span><span class="sp-clb">Annotée</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','categorical',this)"><span class="sp-cic">▩</span><span class="sp-clb">Catégorielle</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','unequal',this)"><span class="sp-cic">▥</span><span class="sp-clb">Inégale</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','log',this)"><span class="sp-cic">∿</span><span class="sp-clb">Log</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','discrete',this)"><span class="sp-cic">≣</span><span class="sp-clb">Discrète</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','correlation',this)"><span class="sp-cic">◤</span><span class="sp-clb">Corrélation</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','density',this)"><span class="sp-cic">⁂</span><span class="sp-clb">Densité</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','contour',this)"><span class="sp-cic">◉</span><span class="sp-clb">Contour</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','temporal',this)"><span class="sp-cic">◷</span><span class="sp-clb">Temporelle</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','cluster',this)"><span class="sp-cic">⌬</span><span class="sp-clb">Cluster</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','bubble',this)"><span class="sp-cic">⊙</span><span class="sp-clb">Bulles</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','marginal',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Marginale</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','confusion',this)"><span class="sp-cic">⊠</span><span class="sp-clb">Confusion</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-fr','pivot',this)"><span class="sp-cic">⊟</span><span class="sp-clb">Pivot</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="hm-fr-basic"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Requis</strong> <code>matrix</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-basic.html"></iframe></div>

<div class="sp-variant" id="hm-fr-annotated"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"annotated"</code> / <code>"text"</code></span><span><strong>Requis</strong> <code>matrix</code>, <code>annotation_format</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-annotated.html"></iframe></div>

<div class="sp-variant" id="hm-fr-categorical"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"categorical"</code> / <code>"cat"</code></span><span><strong>Requis</strong> <code>matrix</code>, <code>categories</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-categorical.html"></iframe></div>

<div class="sp-variant" id="hm-fr-unequal"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"unequal"</code> / <code>"variable"</code></span><span><strong>Requis</strong> <code>matrix</code>, <code>x_widths</code>, <code>y_heights</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-unequal.html"></iframe></div>

<div class="sp-variant" id="hm-fr-log"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"log"</code> / <code>"logscale"</code></span><span><strong>Requis</strong> <code>matrix</code>, <code>colorscale</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-log.html"></iframe></div>

<div class="sp-variant" id="hm-fr-discrete"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"discrete"</code> / <code>"binned"</code></span><span><strong>Requis</strong> <code>matrix</code>, <code>levels</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-discrete.html"></iframe></div>

<div class="sp-variant" id="hm-fr-correlation"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"correlation"</code> / <code>"corr"</code></span><span><strong>Requis</strong> <code>matrix</code>, <code>mask_upper</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-correlation.html"></iframe></div>

<div class="sp-variant" id="hm-fr-density"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"density"</code> / <code>"hist2d"</code></span><span><strong>Requis</strong> <code>x</code>, <code>y</code>, <code>bins</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-density.html"></iframe></div>

<div class="sp-variant" id="hm-fr-contour"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"contour"</code> / <code>"isoline"</code></span><span><strong>Requis</strong> <code>matrix</code>, <code>n_contours</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-contour.html"></iframe></div>

<div class="sp-variant" id="hm-fr-temporal"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"temporal"</code> / <code>"calendar"</code></span><span><strong>Requis</strong> <code>matrix</code>, <code>time_axis</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-temporal.html"></iframe></div>

<div class="sp-variant" id="hm-fr-cluster"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"cluster"</code> / <code>"dendro"</code></span><span><strong>Requis</strong> <code>matrix</code>, <code>cluster_method</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-cluster.html"></iframe></div>

<div class="sp-variant" id="hm-fr-bubble"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"bubble"</code> / <code>"dot"</code></span><span><strong>Requis</strong> <code>matrix</code>, <code>size_matrix</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-bubble.html"></iframe></div>

<div class="sp-variant" id="hm-fr-marginal"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"marginal"</code> / <code>"histograms"</code></span><span><strong>Requis</strong> <code>matrix</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-marginal.html"></iframe></div>

<div class="sp-variant" id="hm-fr-confusion"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"confusion"</code> / <code>"cm"</code></span><span><strong>Requis</strong> <code>matrix</code>, <code>class_labels</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-confusion.html"></iframe></div>

<div class="sp-variant" id="hm-fr-pivot"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"pivot"</code> / <code>"crosstab"</code></span><span><strong>Requis</strong> <code>data</code>, <code>index</code>, <code>columns</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/heatmap-pivot.html"></iframe></div>

</div>
</div>

</div><!-- /lang-fr -->
