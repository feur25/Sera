# Area Chart

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

`sp.area(title, x_labels=None, series=None, *, variant="basic", series_names=None, palette=None, **kwargs) -> Chart`

Aliases: `sp.area`, `sp.area_chart`, `sp.area_family`, `sp.area_unified`, `sp.build_area_chart`

## Description

`sp.area()` is the unified entry point for the entire area-chart family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. Area charts fill the space between a line and the baseline, making the emphasis on cumulative magnitude rather than the line chart's point-to-point comparison. SeraPlot renders everything in pure Rust SVG with native multi-series overlay, stacking, 100%-normalized stacking, smooth splines, step interpolation and gradient fills.

## Variants

<div data-sp-registry-table="variants" data-family="area"></div>

## Parameters

<div data-sp-registry-table="options" data-family="area"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="area-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('area-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('area-en','basic',this)"><span class="sp-cic">A</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('area-en','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('area-en','percent',this)"><span class="sp-cic">%</span><span class="sp-clb">Percent</span></button>
<button class="sp-cls-tab" onclick="spCls('area-en','spline',this)"><span class="sp-cic">~</span><span class="sp-clb">Spline</span></button>
<button class="sp-cls-tab" onclick="spCls('area-en','step',this)"><span class="sp-cic">⌐</span><span class="sp-clb">Step</span></button>
<button class="sp-cls-tab" onclick="spCls('area-en','gradient',this)"><span class="sp-cic">▽</span><span class="sp-clb">Gradient</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="area-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / overlay / default / simple</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Each series drawn as an independent semi-transparent filled area from its own value down to the baseline - overlapping regions blend visually, useful to compare overlapping magnitudes directly.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/area-basic.html"></iframe>
</div>

<div class="sp-variant" id="area-en-stacked">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stack</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Series drawn on top of one another so the top boundary tracks the running total - reads both individual contribution and combined magnitude at once.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/area-stacked.html"></iframe>
</div>

<div class="sp-variant" id="area-en-percent">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"percent"</code></span><span><strong>Aliases</strong> <code>percent / percent_stacked / normalized / stream100</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">100%-stacked area - every x position sums to 100%, showing the changing composition (share of total) instead of absolute magnitude.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/area-percent.html"></iframe>
</div>

<div class="sp-variant" id="area-en-spline">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"spline"</code></span><span><strong>Aliases</strong> <code>spline / smooth / curved</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Catmull-Rom smoothed boundary through every point instead of straight segments - a softer, more organic silhouette for trend-focused series.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/area-spline.html"></iframe>
</div>

<div class="sp-variant" id="area-en-step">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"step"</code></span><span><strong>Aliases</strong> <code>step / stepped / stairs</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Step-interpolated boundary - the fill jumps at each data point instead of interpolating, correct for values that hold constant between samples (inventory levels, active counts...).</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/area-step.html"></iframe>
</div>

<div class="sp-variant" id="area-en-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / glow / fade</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Smooth spline boundary filled with a vertical gradient fading from the series color to transparent at the baseline - a modern dashboard look.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/area-gradient.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.area(title, x_labels=None, series=None, *, variant="basic", series_names=None, palette=None, **kwargs) -> Chart`

Alias : `sp.area`, `sp.area_chart`, `sp.area_family`, `sp.area_unified`, `sp.build_area_chart`

<h2>Description</h2>

`sp.area()` est le point d'entrée unifié de toute la famille des graphiques en aires. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments gardent le même nom d'une variante à l'autre. Les graphiques en aires remplissent l'espace entre une courbe et la ligne de base, mettant l'accent sur la magnitude cumulée plutôt que sur la comparaison point à point du graphique en ligne. SeraPlot rend tout en SVG Rust pur, avec superposition multi-séries native, empilement, empilement normalisé à 100%, courbes lissées, interpolation en escalier et remplissages en dégradé.

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="area"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="area"></div>

---

<h2>Retour</h2>

`Chart` — objet exposant `.html` et `.show()`.

---

<div class="sp-cls sp-open" id="area-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('area-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('area-fr','basic',this)"><span class="sp-cic">A</span><span class="sp-clb">De base</span></button>
<button class="sp-cls-tab" onclick="spCls('area-fr','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Empilé</span></button>
<button class="sp-cls-tab" onclick="spCls('area-fr','percent',this)"><span class="sp-cic">%</span><span class="sp-clb">Pourcentage</span></button>
<button class="sp-cls-tab" onclick="spCls('area-fr','spline',this)"><span class="sp-cic">~</span><span class="sp-clb">Lissé</span></button>
<button class="sp-cls-tab" onclick="spCls('area-fr','step',this)"><span class="sp-cic">⌐</span><span class="sp-clb">Escalier</span></button>
<button class="sp-cls-tab" onclick="spCls('area-fr','gradient',this)"><span class="sp-cic">▽</span><span class="sp-clb">Dégradé</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="area-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / overlay / default / simple</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Chaque série dessinée comme une aire remplie semi-transparente indépendante depuis sa propre valeur jusqu'à la ligne de base - les zones qui se chevauchent se mélangent visuellement, utile pour comparer directement des magnitudes qui se recouvrent.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/area-basic.html"></iframe>
</div>

<div class="sp-variant" id="area-fr-stacked">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"stacked"</code></span><span><strong>Alias</strong> <code>stacked / stack</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Séries dessinées les unes sur les autres pour que la frontière supérieure suive le total cumulé - lit à la fois la contribution individuelle et la magnitude combinée.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/area-stacked.html"></iframe>
</div>

<div class="sp-variant" id="area-fr-percent">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"percent"</code></span><span><strong>Alias</strong> <code>percent / percent_stacked / normalized / stream100</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Aire empilée à 100% - chaque position x totalise 100%, montrant la composition changeante (part du total) plutôt que la magnitude absolue.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/area-percent.html"></iframe>
</div>

<div class="sp-variant" id="area-fr-spline">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"spline"</code></span><span><strong>Alias</strong> <code>spline / smooth / curved</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Frontière lissée par Catmull-Rom passant par chaque point au lieu de segments droits - une silhouette plus douce et organique pour les séries orientées tendance.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/area-spline.html"></iframe>
</div>

<div class="sp-variant" id="area-fr-step">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"step"</code></span><span><strong>Alias</strong> <code>step / stepped / stairs</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Frontière interpolée en escalier - le remplissage saute à chaque point de donnée au lieu d'interpoler, correct pour des valeurs constantes entre échantillons (niveaux de stock, compteurs actifs...).</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/area-step.html"></iframe>
</div>

<div class="sp-variant" id="area-fr-gradient">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code></span><span><strong>Alias</strong> <code>gradient / glow / fade</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Frontière lissée remplie d'un dégradé vertical s'estompant de la couleur de la série vers la transparence à la ligne de base - un look de tableau de bord moderne.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/area-gradient.html"></iframe>
</div>

</div>
</div>

</div>
