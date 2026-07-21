# Line Charts

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

`sp.line(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

## Description

`sp.line()` is the unified entry point for the entire line-chart family. The `variant` keyword selects the rendering strategy — every other argument is shared across variants.
## Variants

<div data-sp-registry-table="variants" data-family="line"></div>

## Parameters

<div data-sp-registry-table="options" data-family="line"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="line-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('line-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('line-en','basic',this)"><span class="sp-cic">─</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('line-en','multi',this)"><span class="sp-cic">≡</span><span class="sp-clb">Multi</span></button>
<button class="sp-cls-tab" onclick="spCls('line-en','stepped',this)"><span class="sp-cic">⌐</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('line-en','spline',this)"><span class="sp-cic">∽</span><span class="sp-clb">Spline</span></button>
<button class="sp-cls-tab" onclick="spCls('line-en','filled',this)"><span class="sp-cic">▰</span><span class="sp-clb">Filled</span></button>
<button class="sp-cls-tab" onclick="spCls('line-en','sparkline',this)"><span class="sp-cic">⌁</span><span class="sp-clb">Sparkline</span></button>
<button class="sp-cls-tab" onclick="spCls('line-en','dashed',this)"><span class="sp-cic">┈</span><span class="sp-clb">Dashed</span></button>
<button class="sp-cls-tab" onclick="spCls('line-en','connected_scatter',this)"><span class="sp-cic">●</span><span class="sp-clb">Connected Scatter</span></button>
<button class="sp-cls-tab" onclick="spCls('line-en','gapped',this)"><span class="sp-cic">⋯</span><span class="sp-clb">Gapped</span></button></div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="line-en-basic">

Single series connecting ordered data points.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>"basic"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-basic.html"></iframe>
</div>

<div class="sp-variant" id="line-en-multi">

Several series sharing the same x-axis. Pass `series=[(name, values), ...]`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"multi"</code></span><span><strong>Aliases</strong> <code>"multi"</code> / <code>"multiline"</code> / <code>"multiple"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-multi.html"></iframe>
</div>

<div class="sp-variant" id="line-en-stepped">

Step (staircase) line — ideal for piecewise-constant data. Use `step_shape` to control corner direction.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>"stepped"</code> / <code>"step"</code> / <code>"hv"</code> / <code>"vh"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-stepped.html"></iframe>
</div>

<div class="sp-variant" id="line-en-spline">

Catmull-Rom smoothed curve. `spline_tension` (0–1) controls how tight the curve hugs the points.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"spline"</code></span><span><strong>Aliases</strong> <code>"spline"</code> / <code>"smooth"</code> / <code>"curved"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-spline.html"></iframe>
</div>

<div class="sp-variant" id="line-en-filled">

Area chart — fills the region under the line. `fill_opacity` controls transparency; `stack_fill=True` stacks multiple series.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"filled"</code></span><span><strong>Aliases</strong> <code>"filled"</code> / <code>"area"</code> / <code>"fill"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-filled.html"></iframe>
</div>

<div class="sp-variant" id="line-en-sparkline">

Small inline chart — no axes, perfect for dashboards. `spark_cols` arranges multiple series in a grid.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"sparkline"</code></span><span><strong>Aliases</strong> <code>"sparkline"</code> / <code>"spark"</code> / <code>"tiny"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-sparkline.html"></iframe>
</div>

<div class="sp-variant" id="line-en-dashed">

Custom stroke pattern. `dash_pattern="8,4"` means 8px on, 4px off. Use `"2,3"` for dotted.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dashed"</code></span><span><strong>Aliases</strong> <code>"dashed"</code> / <code>"dotted"</code> / <code>"styled"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-dashed.html"></iframe>
</div>

<div class="sp-variant" id="line-en-connected_scatter">

Line plot with prominent markers. `marker_size` (px) controls dot size; `show_points=True` is implicit.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"connected_scatter"</code></span><span><strong>Aliases</strong> <code>"connected_scatter"</code> / <code>"markers"</code> / <code>"lines+markers"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-connected_scatter.html"></iframe>
</div>

<div class="sp-variant" id="line-en-gapped">

Line breaks where values exceed `gap_threshold`. Useful for time series with missing samples.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>"gapped"</code> / <code>"gaps"</code> / <code>"missing"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-gapped.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.line(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

<h2>Description</h2>

`sp.line()` est le point d'entrée unifié pour toute la famille de graphiques en ligne. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments sont partagés entre les variantes.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="line"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="line"></div>

---

<h2>Retourne</h2>

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

---

<div class="sp-cls sp-open" id="line-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('line-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('line-fr','basic',this)"><span class="sp-cic">─</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('line-fr','multi',this)"><span class="sp-cic">≡</span><span class="sp-clb">Multi</span></button>
<button class="sp-cls-tab" onclick="spCls('line-fr','stepped',this)"><span class="sp-cic">⌐</span><span class="sp-clb">Escalier</span></button>
<button class="sp-cls-tab" onclick="spCls('line-fr','spline',this)"><span class="sp-cic">∽</span><span class="sp-clb">Spline</span></button>
<button class="sp-cls-tab" onclick="spCls('line-fr','filled',this)"><span class="sp-cic">▰</span><span class="sp-clb">Remplie</span></button>
<button class="sp-cls-tab" onclick="spCls('line-fr','sparkline',this)"><span class="sp-cic">⌁</span><span class="sp-clb">Sparkline</span></button>
<button class="sp-cls-tab" onclick="spCls('line-fr','dashed',this)"><span class="sp-cic">┈</span><span class="sp-clb">Tirets</span></button>
<button class="sp-cls-tab" onclick="spCls('line-fr','connected_scatter',this)"><span class="sp-cic">●</span><span class="sp-clb">Scatter Connecté</span></button>
<button class="sp-cls-tab" onclick="spCls('line-fr','gapped',this)"><span class="sp-cic">⋯</span><span class="sp-clb">Avec lacunes</span></button></div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="line-fr-basic">

Série unique reliant des points ordonnés.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>"basic"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-basic.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-multi">

Plusieurs séries partageant le même axe x. Passez `series=[(nom, valeurs), ...]`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"multi"</code></span><span><strong>Alias</strong> <code>"multi"</code> / <code>"multiline"</code> / <code>"multiple"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-multi.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-stepped">

Ligne en escalier — idéale pour des données constantes par morceaux. `step_shape` contrôle l'orientation des marches.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"stepped"</code></span><span><strong>Alias</strong> <code>"stepped"</code> / <code>"step"</code> / <code>"hv"</code> / <code>"vh"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-stepped.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-spline">

Courbe Catmull-Rom lissée. `spline_tension` (0–1) contrôle l'adhérence de la courbe aux points.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"spline"</code></span><span><strong>Alias</strong> <code>"spline"</code> / <code>"smooth"</code> / <code>"curved"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-spline.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-filled">

Graphique en aire — remplit la zone sous la ligne. `fill_opacity` règle la transparence ; `stack_fill=True` empile plusieurs séries.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"filled"</code></span><span><strong>Alias</strong> <code>"filled"</code> / <code>"area"</code> / <code>"fill"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-filled.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-sparkline">

Petit graphique inline — sans axes, idéal pour les tableaux de bord. `spark_cols` arrange plusieurs séries dans une grille.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"sparkline"</code></span><span><strong>Alias</strong> <code>"sparkline"</code> / <code>"spark"</code> / <code>"tiny"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-sparkline.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-dashed">

Motif de ligne personnalisé. `dash_pattern="8,4"` signifie 8px de trait, 4px de vide. Utilisez `"2,3"` pour pointillé.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"dashed"</code></span><span><strong>Alias</strong> <code>"dashed"</code> / <code>"dotted"</code> / <code>"styled"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-dashed.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-connected_scatter">

Ligne avec marqueurs visibles. `marker_size` (px) règle la taille des points ; `show_points=True` est implicite.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"connected_scatter"</code></span><span><strong>Alias</strong> <code>"connected_scatter"</code> / <code>"markers"</code> / <code>"lines+markers"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-connected_scatter.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-gapped">

Rupture de ligne lorsque les valeurs dépassent `gap_threshold`. Utile pour des séries temporelles avec échantillons manquants.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gapped"</code></span><span><strong>Alias</strong> <code>"gapped"</code> / <code>"gaps"</code> / <code>"missing"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-gapped.html"></iframe>
</div>

</div>
</div>

</div>
