# Radar — Spider / Star Chart

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:560px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.radar(title, axes, series, *, series_names=None, variant="basic", filled=True, fill_opacity=50, palette=None, **kwargs) -> Chart`

## Description

`sp.radar()` is the unified entry point for the entire radar / spider / star chart family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. Radar charts are ideal for multivariate comparison across 3+ axes — performance profiles, KPIs, skill maps, scoring systems. SeraPlot draws everything in pure Rust SVG with concentric grid rings, axis lines, automatic ring tick labels, optional legend and per-series palette colors. The polar-bar variant turns the chart into a categorical polar histogram, the stacked variant builds a cumulative composition view.
## Variants

<div data-sp-registry-table="variants" data-family="radar"></div>

## Parameters

<div data-sp-registry-table="options" data-family="radar"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-cls sp-open" id="radar-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('radar-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('radar-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','lines',this)"><span class="sp-cic">L</span><span class="sp-clb">Lines</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','filled',this)"><span class="sp-cic">F</span><span class="sp-clb">Filled</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','markers',this)"><span class="sp-cic">M</span><span class="sp-clb">Markers</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','dashed',this)"><span class="sp-cic">D</span><span class="sp-clb">Dashed</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','polar_bar',this)"><span class="sp-cic">P</span><span class="sp-clb">PolarBar</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="radar-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Filled polygon per series with stroke and dot markers — the standard radar.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-basic.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-lines">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"lines"</code></span><span><strong>Aliases</strong> <code>lines / outline / stroke / no_fill</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stroke-only polygons, no fill — clean overlay for many series.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-lines.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-filled">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"filled"</code></span><span><strong>Aliases</strong> <code>filled / fill / solid / area</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Strong fill, no stroke, sorted back-to-front by total area for clarity.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-filled.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-markers">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"markers"</code></span><span><strong>Aliases</strong> <code>markers / dots / points / marker</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Light stroke + bold outlined markers — emphasis on data points.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-markers.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-dashed">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dashed"</code></span><span><strong>Aliases</strong> <code>dashed / dash / dotted</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Dashed outline polygons — useful for projections, targets, baselines.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-dashed.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-stacked">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stack / cumulative</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Cumulative stacking on each axis — visualizes part-of-whole composition.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-stacked.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-polar_bar">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"polar_bar"</code></span><span><strong>Aliases</strong> <code>polar_bar / polar / bar / radial_bar</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Radial bars per axis grouped by series — categorical polar histogram.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-polar_bar.html"></iframe>
</div>
</div></div>

</div><!-- /lang-en -->

<div class="lang-fr">

<h2>Signature</h2>

`sp.radar(title, axes, series, *, series_names=None, variant="basic", filled=True, fill_opacity=50, palette=None, **kwargs) -> Chart`

<h2>Description</h2>

`sp.radar()` est le point d'entrée unifié pour toute la famille radar / spider / star. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments conservent le même nom d'une variante à l'autre. Le radar est idéal pour comparer plusieurs séries sur 3 axes ou plus — profils de performance, KPI, cartographie de compétences, systèmes de notation. SeraPlot dessine tout en SVG Rust natif avec anneaux de grille concentriques, axes, labels automatiques de graduation, légende optionnelle et couleurs de palette par série. La variante polar_bar transforme le radar en histogramme polaire catégoriel, la variante stacked construit une vue de composition cumulative.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="radar"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="radar"></div>

---

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

<div class="sp-cls sp-open" id="radar-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('radar-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('radar-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','lines',this)"><span class="sp-cic">L</span><span class="sp-clb">Lines</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','filled',this)"><span class="sp-cic">F</span><span class="sp-clb">Filled</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','markers',this)"><span class="sp-cic">M</span><span class="sp-clb">Markers</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','dashed',this)"><span class="sp-cic">D</span><span class="sp-clb">Dashed</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','polar_bar',this)"><span class="sp-cic">P</span><span class="sp-clb">PolarBar</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="radar-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Polygone rempli par série avec contour et points — le radar standard.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-basic.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-lines">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"lines"</code></span><span><strong>Alias</strong> <code>lines / outline / stroke / no_fill</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Polygones en trait seul, sans remplissage — overlay net pour plusieurs séries.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-lines.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-filled">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"filled"</code></span><span><strong>Alias</strong> <code>filled / fill / solid / area</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Remplissage fort sans contour, trié de l'arrière vers l'avant par aire totale.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-filled.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-markers">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"markers"</code></span><span><strong>Alias</strong> <code>markers / dots / points / marker</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trait léger + marqueurs détourés — accent sur les points de données.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-markers.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-dashed">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"dashed"</code></span><span><strong>Alias</strong> <code>dashed / dash / dotted</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Polygones à contour pointillé — utile pour projections, cibles, références.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-dashed.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-stacked">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"stacked"</code></span><span><strong>Alias</strong> <code>stacked / stack / cumulative</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Empilement cumulatif sur chaque axe — visualise une composition part/tout.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-stacked.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-polar_bar">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"polar_bar"</code></span><span><strong>Alias</strong> <code>polar_bar / polar / bar / radial_bar</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres radiales par axe groupées par série — histogramme polaire catégoriel.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-polar_bar.html"></iframe>
</div>
</div></div>

</div>

