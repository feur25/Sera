# KDE — Kernel Density Estimate

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

`sp.kde(title, values=None, *, x=None, y=None, variant="basic", categories=None, bandwidth=0.0, filled=True, fill_opacity=50, bins=30, n_points=80, palette=None, **kwargs) -> Chart`

## Description

`sp.kde()` is the unified entry point for the entire Kernel Density Estimate family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. KDE produces a smooth, continuous density estimate from a sample of points using a Gaussian kernel with Scott's rule for automatic bandwidth selection. SeraPlot renders the curves as pure Rust SVG, with native multi-series, normalization, CDF, rug, histogram overlay and gradient fills.
## Variants

<div data-sp-registry-table="variants" data-family="kde"></div>

## Parameters

<div data-sp-registry-table="options" data-family="kde"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="kde-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('kde-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('kde-en','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','outline',this)"><span class="sp-cic">O</span><span class="sp-clb">Outline</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','stepped',this)"><span class="sp-cic">T</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','rug',this)"><span class="sp-cic">R</span><span class="sp-clb">Rug</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','histogram',this)"><span class="sp-cic">H</span><span class="sp-clb">Histogram</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','normalized',this)"><span class="sp-cic">N</span><span class="sp-clb">Normalized</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','cumulative',this)"><span class="sp-cic">C</span><span class="sp-clb">Cumulative</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','contour',this)"><span class="sp-cic">◌</span><span class="sp-clb">Contour</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','levels',this)"><span class="sp-cic">◎</span><span class="sp-clb">Levels</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','stack',this)"><span class="sp-cic">▧</span><span class="sp-clb">Stack</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','fill',this)"><span class="sp-cic">▩</span><span class="sp-clb">Fill</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="kde-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / filled / default / single / multi</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Filled curve, single or multi-series.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-basic.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-outline">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outline"</code></span><span><strong>Aliases</strong> <code>outline / line / stroke / compare / no_fill</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stroke-only curves for clean overlays.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-outline.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-stepped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>stepped / step / stair / stairs</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stair-stepped density (rectangular look).</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-stepped.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-rug">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rug"</code></span><span><strong>Aliases</strong> <code>rug / carpet / ticks / rugplot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">KDE curve with rug ticks at sample positions.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-rug.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-histogram">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"histogram"</code></span><span><strong>Aliases</strong> <code>histogram / hist / with_hist / kdehist / distplot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">KDE curve overlaid on a normalized histogram.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-histogram.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-normalized">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"normalized"</code></span><span><strong>Aliases</strong> <code>normalized / pdf / norm / density</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Each series normalized so its area integrates to 1.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-normalized.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-cumulative">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cumulative"</code></span><span><strong>Aliases</strong> <code>cumulative / cdf / cum</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Cumulative density (CDF) curve in [0, 1].</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-cumulative.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-contour">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"contour"</code></span><span><strong>Aliases</strong> <code>contour / bivariate / kde2d / joint_density / smooth</code></span><span><strong>Required</strong> <code>x</code>, <code>y</code></span><span><strong>Optional</strong> <code>categories</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bivariate (2D) kernel density estimate — a product-kernel Gaussian evaluated on a grid and rendered as a shaded density surface with the raw points overlaid. Pass <code>categories</code> to overlay one density surface per group, each in its own color with a legend.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-contour.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-levels">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"levels"</code></span><span><strong>Aliases</strong> <code>levels / bands / iso_bands / ring_contour / banded</code></span><span><strong>Required</strong> <code>x</code>, <code>y</code></span><span><strong>Optional</strong> <code>categories</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bivariate KDE quantized into discrete iso-density bands with a visible ring border at each band boundary — matches seaborn's <code>kdeplot(x=, y=, hue=, fill=True)</code> stepped-level look, as opposed to <code>contour</code>'s continuous gradient.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-levels.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-stack">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stack"</code></span><span><strong>Aliases</strong> <code>stack / stacked / layered_stack</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Each group's density curve stacked on top of the previous one's (cumulative running total), all evaluated on one shared x-grid - matches seaborn's <code>kdeplot(hue=, multiple="stack")</code>.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-stack.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-fill">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"fill"</code></span><span><strong>Aliases</strong> <code>fill / stack100 / percent_stack / filled_stack</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">100%-stacked density - at every x position the groups sum to 100%, showing the changing share of the total instead of absolute density - matches seaborn's <code>kdeplot(hue=, multiple="fill")</code>.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-fill.html"></iframe>
</div>
</div></div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.kde(title, values=None, *, x=None, y=None, variant="basic", categories=None, bandwidth=0.0, filled=True, fill_opacity=50, bins=30, n_points=80, palette=None, **kwargs) -> Chart`

<h2>Description</h2>

`sp.kde()` est le point d'entrée unifié pour toute la famille KDE (Kernel Density Estimate). Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments conservent le même nom d'une variante à l'autre. La KDE produit une estimation de densité continue lissée à partir d'un échantillon de points avec un noyau gaussien et la règle de Scott pour le choix automatique de la bande passante. SeraPlot rend les courbes en SVG Rust natif, avec multi-séries, normalisation, CDF, rug, histogramme superposé et remplissage en dégradé.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="kde"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="kde"></div>

---

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

<div class="sp-cls sp-open" id="kde-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('kde-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('kde-fr','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','outline',this)"><span class="sp-cic">O</span><span class="sp-clb">Outline</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','stepped',this)"><span class="sp-cic">T</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','rug',this)"><span class="sp-cic">R</span><span class="sp-clb">Rug</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','histogram',this)"><span class="sp-cic">H</span><span class="sp-clb">Histogram</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','normalized',this)"><span class="sp-cic">N</span><span class="sp-clb">Normalized</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','cumulative',this)"><span class="sp-cic">C</span><span class="sp-clb">Cumulative</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','contour',this)"><span class="sp-cic">◌</span><span class="sp-clb">Contour</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','levels',this)"><span class="sp-cic">◎</span><span class="sp-clb">Niveaux</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','stack',this)"><span class="sp-cic">▧</span><span class="sp-clb">Empilé</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','fill',this)"><span class="sp-cic">▩</span><span class="sp-clb">Rempli 100%</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="kde-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / filled / default / single / multi</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Courbe pleine, mono ou multi-séries.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-basic.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-outline">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"outline"</code></span><span><strong>Alias</strong> <code>outline / line / stroke / compare / no_fill</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Courbes en trait seul pour des superpositions épurées.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-outline.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-stepped">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"stepped"</code></span><span><strong>Alias</strong> <code>stepped / step / stair / stairs</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Densité en escalier (rendu rectangulaire).</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-stepped.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-rug">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"rug"</code></span><span><strong>Alias</strong> <code>rug / carpet / ticks / rugplot</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Courbe KDE avec ticks rug aux positions des points.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-rug.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-histogram">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"histogram"</code></span><span><strong>Alias</strong> <code>histogram / hist / with_hist / kdehist / distplot</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Courbe KDE par-dessus un histogramme normalisé.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-histogram.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-normalized">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"normalized"</code></span><span><strong>Alias</strong> <code>normalized / pdf / norm / density</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Chaque série normalisée pour que son aire vaille 1.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-normalized.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-cumulative">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"cumulative"</code></span><span><strong>Alias</strong> <code>cumulative / cdf / cum</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Densité cumulée (CDF) dans [0, 1].</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-cumulative.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-contour">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"contour"</code></span><span><strong>Alias</strong> <code>contour / bivariate / kde2d / joint_density / smooth</code></span><span><strong>Requis</strong> <code>x</code>, <code>y</code></span><span><strong>Optionnel</strong> <code>categories</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Estimation de densité bivariée (2D) — un noyau gaussien produit évalué sur une grille et rendu comme une surface de densité ombrée, avec les points bruts superposés. Passez <code>categories</code> pour superposer une surface de densité par groupe, chacune dans sa propre couleur avec une légende.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-contour.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-levels">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"levels"</code></span><span><strong>Alias</strong> <code>levels / bands / iso_bands / ring_contour / banded</code></span><span><strong>Requis</strong> <code>x</code>, <code>y</code></span><span><strong>Optionnel</strong> <code>categories</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">KDE bivariée quantifiée en bandes iso-densité discrètes avec une bordure visible à chaque frontière de bande — reproduit le rendu par niveaux de <code>kdeplot(x=, y=, hue=, fill=True)</code> de seaborn, par opposition au dégradé continu de <code>contour</code>.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-levels.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-stack">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"stack"</code></span><span><strong>Alias</strong> <code>stack / stacked / layered_stack</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">La courbe de densité de chaque groupe est empilée sur celle du précédent (total cumulé), toutes évaluées sur une même grille x partagée - reproduit <code>kdeplot(hue=, multiple="stack")</code> de seaborn.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-stack.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-fill">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"fill"</code></span><span><strong>Alias</strong> <code>fill / stack100 / percent_stack / filled_stack</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Densité empilée à 100% - à chaque position x, les groupes totalisent 100%, montrant la part changeante du total plutôt que la densité absolue - reproduit <code>kdeplot(hue=, multiple="fill")</code> de seaborn.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-fill.html"></iframe>
</div>
</div></div>

</div>

