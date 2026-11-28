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
<button class="sp-cls-tab" onclick="spCls('scatter-en','regression',this)"><span class="sp-cic">↗</span><span class="sp-clb">Regression</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','residual',this)"><span class="sp-cic">⌇</span><span class="sp-clb">Residual</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','dual_style',this)"><span class="sp-cic">◈</span><span class="sp-clb">Dual Style</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','continuous_hue',this)"><span class="sp-cic">◍</span><span class="sp-clb">Continuous Hue</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','facet',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Facet</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','sized',this)"><span class="sp-cic">◉</span><span class="sp-clb">Sized</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','wide_form',this)"><span class="sp-cic">▤</span><span class="sp-clb">Wide form</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','rug',this)"><span class="sp-cic">⌇</span><span class="sp-clb">Rug</span></button></div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="scatter-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / simple / default</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-basic.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-categorical">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code></span><span><strong>Aliases</strong> <code>categorical / grouped / category</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-categorical.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-symbols">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"symbols"</code></span><span><strong>Aliases</strong> <code>symbols / shapes / markers</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-symbols.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-labeled">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"labeled"</code></span><span><strong>Aliases</strong> <code>labeled / labels / text</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-labeled.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-regression">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"regression"</code></span><span><strong>Aliases</strong> <code>regression / trendline / fit</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-regression.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-residual">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"residual"</code></span><span><strong>Aliases</strong> <code>residuals / residplot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-residual.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-dual_style">

<p>Two independent categorical variables: <code>categories</code> drives color, <code>categories2</code> drives marker shape - matching seaborn's "hue and style with different variables" example.</p>

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dual_style"</code></span><span><strong>Aliases</strong> <code>dual_style / hue_style / two_way</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-dual_style.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-continuous_hue">

<p>Points colored by a continuous numeric variable (<code>color_values</code>) interpolated between <code>color_low</code> and <code>color_high</code>, with a gradient legend bar - seaborn's numeric hue mapping.</p>

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"continuous_hue"</code></span><span><strong>Aliases</strong> <code>continuous_hue / numeric_hue / colormap</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-continuous_hue.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-facet">

<p>Splits the data into one small-multiple panel per unique <code>categories</code> value, all sharing the same x/y domain - a native equivalent of seaborn's <code>relplot()</code> faceting.</p>

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"facet"</code></span><span><strong>Aliases</strong> <code>facet / facets / small_multiples / relplot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-facet.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-sized">

<p>Both marker radius and color are driven by the same continuous variable (<code>color_values</code>, scaled between <code>min_size</code> and <code>max_size</code>) with a combined size+color legend - seaborn's <code>hue=</code> and <code>size=</code> mapped to the same column, with <code>sizes=(min, max)</code>.</p>

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"sized"</code></span><span><strong>Aliases</strong> <code>sized / size_scale / bubble_scatter / magnitude_size</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-sized.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-wide_form">

<p>Plots several numeric columns (<code>series</code>, named via <code>series_names</code>) against one shared <code>x_values</code> axis, one color per column with a legend - the native equivalent of calling <code>seaborn.scatterplot(data=wide_dataframe)</code> directly on a wide-form table.</p>

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"wide_form"</code></span><span><strong>Aliases</strong> <code>wide_form / wide / multi_series / columns</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-wide_form.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-rug">

<p>Scatter + rug plot fusion: each point gets a short tick mark projected onto both axes, right at the plot edges — a Tufte-style view of the marginal distributions along x and y without the overhead of a separate marginal panel (unlike <code>joint()</code>, no iframe composition needed).</p>

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rug"</code></span><span><strong>Aliases</strong> <code>rug / rugplot / marginal_ticks / carpet_ticks</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-rug.html"></iframe>
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
<button class="sp-cls-tab" onclick="spCls('scatter-fr','regression',this)"><span class="sp-cic">↗</span><span class="sp-clb">Régression</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','residual',this)"><span class="sp-cic">⌇</span><span class="sp-clb">Résidus</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','dual_style',this)"><span class="sp-cic">◈</span><span class="sp-clb">Style double</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','continuous_hue',this)"><span class="sp-cic">◍</span><span class="sp-clb">Teinte continue</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','facet',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Facettes</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','sized',this)"><span class="sp-cic">◉</span><span class="sp-clb">Taille</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','wide_form',this)"><span class="sp-cic">▤</span><span class="sp-clb">Format large</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','rug',this)"><span class="sp-cic">⌇</span><span class="sp-clb">Rug</span></button></div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="scatter-fr-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / simple / default</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-basic.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-categorical">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code></span><span><strong>Aliases</strong> <code>categorical / grouped / category</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-categorical.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-symbols">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"symbols"</code></span><span><strong>Aliases</strong> <code>symbols / shapes / markers</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-symbols.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-labeled">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"labeled"</code></span><span><strong>Aliases</strong> <code>labeled / labels / text</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-labeled.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-regression">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"regression"</code></span><span><strong>Aliases</strong> <code>regression / trendline / fit</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-regression.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-residual">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"residual"</code></span><span><strong>Alias</strong> <code>residuals / residplot</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-residual.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-dual_style">

<p>Deux variables catégorielles indépendantes : <code>categories</code> pilote la couleur, <code>categories2</code> pilote la forme du marqueur - comme l'exemple seaborn "hue and style" avec des variables différentes.</p>

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"dual_style"</code></span><span><strong>Alias</strong> <code>dual_style / hue_style / two_way</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-dual_style.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-continuous_hue">

<p>Points colorés selon une variable numérique continue (<code>color_values</code>) interpolée entre <code>color_low</code> et <code>color_high</code>, avec une barre de légende en dégradé - le mapping de teinte numérique de seaborn.</p>

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"continuous_hue"</code></span><span><strong>Alias</strong> <code>continuous_hue / numeric_hue / colormap</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-continuous_hue.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-facet">

<p>Sépare les données en un panneau petit-multiple par valeur unique de <code>categories</code>, tous partageant le même domaine x/y - un équivalent natif du facettage <code>relplot()</code> de seaborn.</p>

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"facet"</code></span><span><strong>Alias</strong> <code>facet / facets / small_multiples / relplot</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-facet.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-sized">

<p>Le rayon du marqueur et sa couleur sont pilotés par la même variable continue (<code>color_values</code>, mise à l'échelle entre <code>min_size</code> et <code>max_size</code>) avec une légende combinée taille+couleur - l'équivalent de <code>hue=</code> et <code>size=</code> pointant sur la même colonne en seaborn, avec <code>sizes=(min, max)</code>.</p>

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"sized"</code></span><span><strong>Alias</strong> <code>sized / size_scale / bubble_scatter / magnitude_size</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-sized.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-wide_form">

<p>Trace plusieurs colonnes numériques (<code>series</code>, nommées via <code>series_names</code>) sur un même axe <code>x_values</code> partagé, une couleur par colonne avec une légende - l'équivalent natif d'appeler <code>seaborn.scatterplot(data=wide_dataframe)</code> directement sur un tableau au format large.</p>

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"wide_form"</code></span><span><strong>Alias</strong> <code>wide_form / wide / multi_series / columns</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-wide_form.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-rug">

<p>Fusion scatter + rug plot : chaque point projette une petite marque sur les deux axes, au bord du graphique — une vue façon Tufte des distributions marginales en x et en y sans le coût d'un panneau marginal séparé (contrairement à <code>joint()</code>, aucune composition iframe nécessaire).</p>

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"rug"</code></span><span><strong>Alias</strong> <code>rug / rugplot / marginal_ticks / carpet_ticks</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/scatter-rug.html"></iframe>
</div>

</div>
</div>

</div>
