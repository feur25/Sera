# Ridgeline — Joyplot / Stacked KDE

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:520px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.ridgeline(title, categories, values, *, variant="basic", overlap=0.5, bandwidth=0.0, n_points=60, fill_opacity=56, palette=None, **kwargs) -> Chart`

## Description

`sp.ridgeline()` is the unified entry point for the entire ridgeline family — also known as joyplot. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. A ridgeline plot stacks one KDE curve per category along a shared X axis with a controllable vertical overlap, making it ideal to compare distributions across many groups (years, regions, segments…). SeraPlot renders everything in pure Rust SVG, with quartile/mean overlays, rug ticks, gradient fills and a built-in viridis colormap.
## Variants

<div data-sp-registry-table="variants" data-family="ridgeline"></div>

## Parameters

<div data-sp-registry-table="options" data-family="ridgeline"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="ridgeline-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('ridgeline-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('ridgeline-en','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','lines',this)"><span class="sp-cic">L</span><span class="sp-clb">Lines</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','quartiles',this)"><span class="sp-cic">Q</span><span class="sp-clb">Quartiles</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','mean',this)"><span class="sp-cic">M</span><span class="sp-clb">Mean</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','rug',this)"><span class="sp-cic">R</span><span class="sp-clb">Rug</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','heatmap',this)"><span class="sp-cic">H</span><span class="sp-clb">Heatmap</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','spaced',this)"><span class="sp-cic">S</span><span class="sp-clb">Spaced</span></button></div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="ridgeline-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / filled / default / single / multi</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stacked filled ridges (one per category) with white underlay.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-basic.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-lines">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"lines"</code></span><span><strong>Aliases</strong> <code>lines / outline / stroke / no_fill</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stroke-only ridges, no fill — clean outline view.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-lines.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-quartiles">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"quartiles"</code></span><span><strong>Aliases</strong> <code>quartiles / q / qrt / iqr</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Marks Q1, median (solid), and Q3 vertical lines on each ridge.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-quartiles.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-mean">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mean"</code></span><span><strong>Aliases</strong> <code>mean / average / avg / mean_dot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Dashed line + dot at the mean of each distribution.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-mean.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-rug">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rug"</code></span><span><strong>Aliases</strong> <code>rug / ticks / carpet / rugplot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Filled ridge with rug ticks below the baseline at sample positions.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-rug.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-heatmap">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heatmap"</code></span><span><strong>Aliases</strong> <code>heatmap / heat / rainbow / colored</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Auto viridis palette across ridges (or custom palette).</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-heatmap.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-spaced">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"spaced"</code></span><span><strong>Aliases</strong> <code>spaced / separated / no_overlap / split</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Forced low overlap — ridges are separated for clarity.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-spaced.html"></iframe>
</div>
</div></div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.ridgeline(title, categories, values, *, variant="basic", overlap=0.5, bandwidth=0.0, n_points=60, fill_opacity=56, palette=None, **kwargs) -> Chart`

<h2>Description</h2>

`sp.ridgeline()` est le point d'entrée unifié pour toute la famille ridgeline — aussi appelé joyplot. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments conservent le même nom d'une variante à l'autre. Un ridgeline empile une courbe KDE par catégorie sur un axe X partagé avec un recouvrement vertical réglable, idéal pour comparer des distributions à travers plusieurs groupes (années, régions, segments…). SeraPlot rend tout en SVG Rust natif, avec marqueurs quartiles/moyenne, ticks rug, dégradés et palette viridis intégrée.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="ridgeline"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="ridgeline"></div>

---

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

<div class="sp-cls sp-open" id="ridgeline-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('ridgeline-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('ridgeline-fr','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','lines',this)"><span class="sp-cic">L</span><span class="sp-clb">Lines</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','quartiles',this)"><span class="sp-cic">Q</span><span class="sp-clb">Quartiles</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','mean',this)"><span class="sp-cic">M</span><span class="sp-clb">Mean</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','rug',this)"><span class="sp-cic">R</span><span class="sp-clb">Rug</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','heatmap',this)"><span class="sp-cic">H</span><span class="sp-clb">Heatmap</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','spaced',this)"><span class="sp-cic">S</span><span class="sp-clb">Spaced</span></button></div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="ridgeline-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / filled / default / single / multi</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Crêtes empilées remplies (une par catégorie) avec fond blanc.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-basic.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-lines">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"lines"</code></span><span><strong>Alias</strong> <code>lines / outline / stroke / no_fill</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Crêtes en trait seul, sans remplissage — vue épurée.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-lines.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-quartiles">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"quartiles"</code></span><span><strong>Alias</strong> <code>quartiles / q / qrt / iqr</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trace les verticales Q1, médiane (pleine) et Q3 sur chaque crête.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-quartiles.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-mean">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"mean"</code></span><span><strong>Alias</strong> <code>mean / average / avg / mean_dot</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trait pointillé + point à la moyenne de chaque distribution.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-mean.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-rug">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"rug"</code></span><span><strong>Alias</strong> <code>rug / ticks / carpet / rugplot</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Crête remplie avec ticks rug sous la ligne de base aux positions des points.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-rug.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-heatmap">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"heatmap"</code></span><span><strong>Alias</strong> <code>heatmap / heat / rainbow / colored</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Palette viridis automatique sur les crêtes (ou palette personnalisée).</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-heatmap.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-spaced">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"spaced"</code></span><span><strong>Alias</strong> <code>spaced / separated / no_overlap / split</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Recouvrement forcé bas — crêtes séparées pour la lisibilité.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-spaced.html"></iframe>
</div>
</div></div>

</div>

