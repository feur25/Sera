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
<button class="sp-cls-tab" onclick="spCls('line-en','gapped',this)"><span class="sp-cic">⋯</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('line-en','band',this)"><span class="sp-cic">▨</span><span class="sp-clb">Band</span></button>
<button class="sp-cls-tab" onclick="spCls('line-en','momentum',this)"><span class="sp-cic">◉</span><span class="sp-clb">Momentum</span></button>
<button class="sp-cls-tab" onclick="spCls('line-en','epoch',this)"><span class="sp-cic">◧</span><span class="sp-clb">Epoch</span></button>
<button class="sp-cls-tab" onclick="spCls('line-en','pace',this)"><span class="sp-cic">◈</span><span class="sp-clb">Pace</span></button></div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="line-en-basic">

Single series connecting ordered data points.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>"basic"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-basic.html"></iframe>
</div>

<div class="sp-variant" id="line-en-multi">

Several series sharing the same x-axis. Pass `series=[(name, values), ...]`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"multi"</code></span><span><strong>Aliases</strong> <code>"multi"</code> / <code>"multiline"</code> / <code>"multiple"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-multi.html"></iframe>
</div>

<div class="sp-variant" id="line-en-stepped">

Step (staircase) line — ideal for piecewise-constant data. Use `step_shape` to control corner direction.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>"stepped"</code> / <code>"step"</code> / <code>"hv"</code> / <code>"vh"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-stepped.html"></iframe>
</div>

<div class="sp-variant" id="line-en-spline">

Catmull-Rom smoothed curve. `spline_tension` (0–1) controls how tight the curve hugs the points.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"spline"</code></span><span><strong>Aliases</strong> <code>"spline"</code> / <code>"smooth"</code> / <code>"curved"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-spline.html"></iframe>
</div>

<div class="sp-variant" id="line-en-filled">

Area chart — fills the region under the line. `fill_opacity` controls transparency; `stack_fill=True` stacks multiple series.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"filled"</code></span><span><strong>Aliases</strong> <code>"filled"</code> / <code>"area"</code> / <code>"fill"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-filled.html"></iframe>
</div>

<div class="sp-variant" id="line-en-sparkline">

Small inline chart — no axes, perfect for dashboards. `spark_cols` arranges multiple series in a grid.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"sparkline"</code></span><span><strong>Aliases</strong> <code>"sparkline"</code> / <code>"spark"</code> / <code>"tiny"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-sparkline.html"></iframe>
</div>

<div class="sp-variant" id="line-en-dashed">

Custom stroke pattern. `dash_pattern="8,4"` means 8px on, 4px off. Use `"2,3"` for dotted.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dashed"</code></span><span><strong>Aliases</strong> <code>"dashed"</code> / <code>"dotted"</code> / <code>"styled"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-dashed.html"></iframe>
</div>

<div class="sp-variant" id="line-en-connected_scatter">

Line plot with prominent markers. `marker_size` (px) controls dot size; `show_points=True` is implicit.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"connected_scatter"</code></span><span><strong>Aliases</strong> <code>"connected_scatter"</code> / <code>"markers"</code> / <code>"lines+markers"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-connected_scatter.html"></iframe>
</div>

<div class="sp-variant" id="line-en-gapped">

Line breaks where values exceed `gap_threshold`. Useful for time series with missing samples.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>"gapped"</code> / <code>"gaps"</code> / <code>"missing"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-gapped.html"></iframe>
</div>

<div class="sp-variant" id="line-en-band">

Line + confidence/forecast band fusion: pass `series` as low/high pairs (`[group1_low, group1_high, group2_low, group2_high, ...]`) instead of single traces. Each group gets a shaded area between its two bounds, thin dashed edge lines, and a solid midline with markers — a forecast interval, min/max envelope or confidence band, natively.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"band"</code></span><span><strong>Aliases</strong> <code>"band"</code> / <code>"confidence_band"</code> / <code>"forecast"</code> / <code>"range_band"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-band.html"></iframe>
</div>

<div class="sp-variant" id="line-en-momentum">

Each segment is colored on a diverging scale by its own local slope, with a matching soft gradient fill beneath, automatic peak/trough callouts, and a glowing pulse on the latest point — momentum, read at a glance instead of computed by eye.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"momentum"</code></span><span><strong>Aliases</strong> <code>"momentum"</code> / <code>"slope_glow"</code> / <code>"trend_pulse"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-momentum.html"></iframe>
</div>

<div class="sp-variant" id="line-en-epoch">

Auto-segments the series into narrative chapters at its own significant turning points, shades each by net direction, and captions it with a plain-language badge — the chart writes its own headline instead of asking the reader to compute one. `epoch_pos_color` / `epoch_neg_color` / `epoch_flat_color` (hex ints) recolor the rising/falling/flat chapters.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"epoch"</code></span><span><strong>Aliases</strong> <code>"epoch"</code> / <code>"chapters"</code> / <code>"regime_bands"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-epoch.html"></iframe>
</div>

<div class="sp-variant" id="line-en-pace">

Compares the series against its own ideal glidepath to a `pace_target`, shading ahead-of-pace and behind-of-pace stretches in different colors, then extrapolates the recent trend past the last real point to project whether it clears the target. `pace_ahead_color` / `pace_behind_color` recolor the comparison.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"pace"</code></span><span><strong>Aliases</strong> <code>"pace"</code> / <code>"pacing"</code> / <code>"glidepath"</code> / <code>"runway"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-pace.html"></iframe>
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
<button class="sp-cls-tab" onclick="spCls('line-fr','gapped',this)"><span class="sp-cic">⋯</span><span class="sp-clb">Avec lacunes</span></button>
<button class="sp-cls-tab" onclick="spCls('line-fr','band',this)"><span class="sp-cic">▨</span><span class="sp-clb">Bande</span></button>
<button class="sp-cls-tab" onclick="spCls('line-fr','momentum',this)"><span class="sp-cic">◉</span><span class="sp-clb">Momentum</span></button>
<button class="sp-cls-tab" onclick="spCls('line-fr','epoch',this)"><span class="sp-cic">◧</span><span class="sp-clb">Epoch</span></button>
<button class="sp-cls-tab" onclick="spCls('line-fr','pace',this)"><span class="sp-cic">◈</span><span class="sp-clb">Pace</span></button></div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="line-fr-basic">

Série unique reliant des points ordonnés.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>"basic"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-basic.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-multi">

Plusieurs séries partageant le même axe x. Passez `series=[(nom, valeurs), ...]`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"multi"</code></span><span><strong>Alias</strong> <code>"multi"</code> / <code>"multiline"</code> / <code>"multiple"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-multi.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-stepped">

Ligne en escalier — idéale pour des données constantes par morceaux. `step_shape` contrôle l'orientation des marches.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"stepped"</code></span><span><strong>Alias</strong> <code>"stepped"</code> / <code>"step"</code> / <code>"hv"</code> / <code>"vh"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-stepped.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-spline">

Courbe Catmull-Rom lissée. `spline_tension` (0–1) contrôle l'adhérence de la courbe aux points.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"spline"</code></span><span><strong>Alias</strong> <code>"spline"</code> / <code>"smooth"</code> / <code>"curved"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-spline.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-filled">

Graphique en aire — remplit la zone sous la ligne. `fill_opacity` règle la transparence ; `stack_fill=True` empile plusieurs séries.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"filled"</code></span><span><strong>Alias</strong> <code>"filled"</code> / <code>"area"</code> / <code>"fill"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-filled.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-sparkline">

Petit graphique inline — sans axes, idéal pour les tableaux de bord. `spark_cols` arrange plusieurs séries dans une grille.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"sparkline"</code></span><span><strong>Alias</strong> <code>"sparkline"</code> / <code>"spark"</code> / <code>"tiny"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-sparkline.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-dashed">

Motif de ligne personnalisé. `dash_pattern="8,4"` signifie 8px de trait, 4px de vide. Utilisez `"2,3"` pour pointillé.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"dashed"</code></span><span><strong>Alias</strong> <code>"dashed"</code> / <code>"dotted"</code> / <code>"styled"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-dashed.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-connected_scatter">

Ligne avec marqueurs visibles. `marker_size` (px) règle la taille des points ; `show_points=True` est implicite.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"connected_scatter"</code></span><span><strong>Alias</strong> <code>"connected_scatter"</code> / <code>"markers"</code> / <code>"lines+markers"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-connected_scatter.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-gapped">

Rupture de ligne lorsque les valeurs dépassent `gap_threshold`. Utile pour des séries temporelles avec échantillons manquants.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gapped"</code></span><span><strong>Alias</strong> <code>"gapped"</code> / <code>"gaps"</code> / <code>"missing"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-gapped.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-band">

Fusion ligne + bande de confiance/prévision : passez `series` comme des paires bas/haut (`[groupe1_bas, groupe1_haut, groupe2_bas, groupe2_haut, ...]`) au lieu de traces simples. Chaque groupe reçoit une zone ombrée entre ses deux bornes, de fines lignes en pointillés sur les bords, et une ligne médiane pleine avec marqueurs — un intervalle de prévision, une enveloppe min/max ou une bande de confiance, nativement.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"band"</code></span><span><strong>Alias</strong> <code>"band"</code> / <code>"confidence_band"</code> / <code>"forecast"</code> / <code>"range_band"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-band.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-momentum">

Chaque segment est coloré sur une échelle divergente selon sa propre pente locale, avec un remplissage en dégradé assorti en dessous, des annotations automatiques de pics/creux, et un pouls lumineux sur le dernier point — la dynamique, lisible en un coup d'œil plutôt que calculée à l'œil nu.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"momentum"</code></span><span><strong>Alias</strong> <code>"momentum"</code> / <code>"slope_glow"</code> / <code>"trend_pulse"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-momentum.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-epoch">

Segmente automatiquement la série en chapitres narratifs à ses propres points de retournement significatifs, teinte chacun selon sa direction nette, et le légende avec un badge en langage clair — le graphique écrit lui-même son titre au lieu de laisser le lecteur le calculer. `epoch_pos_color` / `epoch_neg_color` / `epoch_flat_color` (entiers hexadécimaux) recolorent les chapitres en hausse/baisse/stable.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"epoch"</code></span><span><strong>Alias</strong> <code>"epoch"</code> / <code>"chapters"</code> / <code>"regime_bands"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-epoch.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-pace">

Compare la série à sa propre trajectoire idéale vers un `pace_target`, teinte différemment les tronçons en avance et en retard sur ce rythme, puis extrapole la tendance récente au-delà du dernier point réel pour projeter si elle atteint la cible. `pace_ahead_color` / `pace_behind_color` recolorent la comparaison.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"pace"</code></span><span><strong>Alias</strong> <code>"pace"</code> / <code>"pacing"</code> / <code>"glidepath"</code> / <code>"runway"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/line-pace.html"></iframe>
</div>

</div>
</div>

</div>
