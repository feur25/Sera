# Bar Charts

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

`sp.bar(title, labels=None, values=None, *, variant="basic", series=None, series_names=None, theme="none", **kwargs) -> Chart`

Aliases: `sp.bar_chart()`, `sp.bars()`, `sp.bar_unified()`, `sp.bars_unified()`, `sp.bar_family()`.

## Description

`sp.bar()` is the unified entry point for the SeraPlot bar-chart family. It renders standalone Rust-generated HTML/SVG charts. The `variant` keyword selects the renderer, and shared chart options are applied by the common chart pipeline.

The default renderer is a vertical categorical bar chart. The same API also covers every bar variant registered in Rust.

## Variants

<div data-sp-registry-table="variants" data-family="bar"></div>

## Data

`labels` are category labels for bar variants. Single-series variants use `values`. Multi-series variants use `series`, where each inner list is one series, and `series_names` supplies legend names.

When `series` is missing but `series_names` is provided, `values` is interpreted as a flattened matrix split by `len(labels)`: the first category-length block is the first series, the next block is the second series, and so on.

## Parameters

<div data-sp-registry-table="options" data-family="bar"></div>

## Themes

<div data-sp-registry-table="themes" data-family="bar"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="bar"></div>
</div>

<div class="sp-cls sp-open" id="bar-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bar-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bar-en','basic',this)"><span class="sp-cic">▮</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','horizontal',this)"><span class="sp-cic">▬</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','grouped',this)"><span class="sp-cic">▐▐</span><span class="sp-clb">Grouped</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','stacked',this)"><span class="sp-cic">▦</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','relative',this)"><span class="sp-cic">±</span><span class="sp-clb">Relative</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','grouped_stacked',this)"><span class="sp-cic">▤</span><span class="sp-clb">Grouped-Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','marimekko',this)"><span class="sp-cic">▤</span><span class="sp-clb">Marimekko</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','pictogram',this)"><span class="sp-cic">☰</span><span class="sp-clb">Pictogram</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','multicategory',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Multicategory</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','circular',this)"><span class="sp-cic">◔</span><span class="sp-clb">Circular</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','circular_grouped',this)"><span class="sp-cic">◕</span><span class="sp-clb">Circular Grouped</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','population_pyramid',this)"><span class="sp-cic">▲</span><span class="sp-clb">Population Pyramid</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','diverging',this)"><span class="sp-cic">↔</span><span class="sp-clb">Diverging</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','distribution',this)"><span class="sp-cic">⊥</span><span class="sp-clb">Distribution</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bar-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>sp.bar</code> <code>sp.bars</code> <code>sp.bar_unified</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-horizontal">

Horizontal bars — better for long category names. Alias: `"h"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code> / <code>"h"</code></span><span><strong>Aliases</strong> <code>sp.bar</code> + <code>variant="h"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hbar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-grouped">

Multiple series side-by-side per category. Alias: `"group"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"grouped"</code> / <code>"group"</code></span><span><strong>Required</strong> <code>series</code>, <code>series_names</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/grouped-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-stacked">

Series stacked vertically — shows part-to-whole within each category. Alias: `"stack"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code> / <code>"stack"</code></span><span><strong>Required</strong> <code>series</code>, <code>series_names</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/stacked-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-relative">

100% stacked bars — every column fills from 0 to 100%, showing each series as a share of the total. Alias: `"rel"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"relative"</code> / <code>"rel"</code></span><span><strong>Required</strong> <code>series</code>, <code>series_names</code></span><span><strong>Note</strong> each column normalised to 100%</span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/relative-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-grouped_stacked">

Groups of stacked sub-bars per category. `offset_groups` assigns a stack-group name to each series.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"grouped_stacked"</code></span><span><strong>Required</strong> <code>series</code>, <code>offset_groups</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/grouped-stacked-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-marimekko">

Variable-width stacked bars (mosaic plot). `widths` encodes one dimension, stacked segments encode share. Aliases: `"mekko"`, `"mosaic"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"marimekko"</code> / <code>"mekko"</code> / <code>"mosaic"</code></span><span><strong>Required</strong> <code>series</code>, <code>widths</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/marimekko-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-pictogram">

A bar made of repeated icons. Each icon represents `units_per_icon` units. Alias: `"icon"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"pictogram"</code> / <code>"icon"</code></span><span><strong>Required</strong> <code>values</code>, <code>units_per_icon</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pictogram-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-multicategory">

Two-level hierarchical x axis. `super_categories` groups adjacent bars under a bracket label. Alias: `"multi"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"multicategory"</code> / <code>"multi"</code></span><span><strong>Required</strong> <code>values</code>, <code>super_categories</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/multicategory-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-circular">

Bars arranged radially around a center, length proportional to value. Pass `show_values=True` for a value at each bar's tip, `gridlines=True` for labeled concentric rings. Aliases: `"circular_basic"`, `"radial_bar"`, `"polar_bar"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"circular"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Optional</strong> <code>show_values</code>, <code>gridlines</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/circular-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-circular_grouped">

Circular bars split into groups via `color_groups`, with an extra gap between groups. Same `show_values`/`gridlines` options as `circular`. Aliases: `"radial_grouped"`, `"circular_groups"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"circular_grouped"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code>, <code>color_groups</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/circular_grouped-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-population_pyramid">

Two horizontal bar sets mirrored left/right around a shared category axis, from the first two entries of `series`. Aliases: `"pyramid"`, `"age_pyramid"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"population_pyramid"</code></span><span><strong>Required</strong> <code>labels</code>, <code>series</code> (≥ 2), <code>series_names</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/population_pyramid-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-diverging">

Horizontal bars extending left or right from a zero line, colored by sign — `color_high` (default red, `0xF43F5E`) for positive, `color_low` (default blue, `0x636EFA`) for negative — value printed inside the bar (white) when it's wide enough or just outside otherwise. Aliases: `"signed"`, `"delta"`, `"bidirectional"`. Pass `series` (with `category_labels`) instead of `values` for a stacked multi-series diverging chart — each series stacks on its own side of zero by its own sign, one color per series from `palette`, with a real legend naming each series (not a generic "positive/negative" label). Optional `error_low`/`error_high` (one entry per bar) draw a whisker; optional `overlay_line` (+ `overlay_line_label`) draws a connected line across the bars, matching each row by position. The value label is controlled by `show_values` (bool) like every other bar variant — it defaults to `True` here so existing charts keep their look, unlike other variants where it defaults to `False`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"diverging"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/diverging-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-distribution">

Bar + boxplot fusion: a semi-transparent bar up to each category's mean, with a real box (Q1/median/Q3, whiskers) overlaid on top showing the distribution behind that mean — pass `series` as one raw sample array per category (same shape as boxplot's grouped input) instead of single aggregate values. Aliases: `"bar_box"`, `"boxbar"`, `"bar_boxplot"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"distribution"</code></span><span><strong>Required</strong> <code>labels</code>, <code>series</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bar-distribution.html"></iframe>
</div>

</div><!-- /sp-cls-body -->
</div>

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.bar(title, labels=None, values=None, *, variant="basic", series=None, series_names=None, theme="none", **kwargs) -> Chart`

Alias : `sp.bar_chart()`, `sp.bars()`, `sp.bar_unified()`, `sp.bars_unified()`, `sp.bar_family()`.

<h2>Description</h2>

`sp.bar()` est le point d'entrée unifié de la famille de graphiques en barres de SeraPlot. Il génère des graphiques HTML/SVG autonomes depuis Rust. Le mot-clé `variant` choisit le renderer, et les options communes passent par le pipeline commun.

Le rendu par défaut est un bar chart catégoriel vertical. La même API couvre toutes les variantes bar enregistrées côté Rust.

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="bar"></div>

<h2>Données</h2>

`labels` sert de liste de catégories pour les variantes bar. Les variantes mono-série utilisent `values`. Les variantes multi-séries utilisent `series`, où chaque liste interne est une série, et `series_names` fournit les noms de légende.

Quand `series` manque mais que `series_names` est fourni, `values` est interprété comme une matrice aplatie découpée par `len(labels)` : le premier bloc appartient à la première série, le suivant à la deuxième, etc.

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="bar"></div>

<h2>Thèmes</h2>

<div data-sp-registry-table="themes" data-family="bar"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="bar"></div>
</div>

<div class="sp-cls sp-open" id="bar-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bar-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bar-fr','basic',this)"><span class="sp-cic">▮</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','horizontal',this)"><span class="sp-cic">▬</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','grouped',this)"><span class="sp-cic">▐▐</span><span class="sp-clb">Groupé</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','stacked',this)"><span class="sp-cic">▦</span><span class="sp-clb">Empilé</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','relative',this)"><span class="sp-cic">±</span><span class="sp-clb">Relatif</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','grouped_stacked',this)"><span class="sp-cic">▤</span><span class="sp-clb">Groupé-Empilé</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','marimekko',this)"><span class="sp-cic">▤</span><span class="sp-clb">Marimekko</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','pictogram',this)"><span class="sp-cic">☰</span><span class="sp-clb">Pictogramme</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','multicategory',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Multi-catégories</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','circular',this)"><span class="sp-cic">◔</span><span class="sp-clb">Circulaire</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','circular_grouped',this)"><span class="sp-cic">◕</span><span class="sp-clb">Circulaire groupé</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','population_pyramid',this)"><span class="sp-cic">▲</span><span class="sp-clb">Pyramide des âges</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','diverging',this)"><span class="sp-cic">↔</span><span class="sp-clb">Divergent</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','distribution',this)"><span class="sp-cic">⊥</span><span class="sp-clb">Distribution</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bar-fr-basic">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>sp.bar</code> <code>sp.bars</code> <code>sp.bar_unified</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-horizontal">

Barres horizontales — idéal pour les longs noms de catégories. Alias : `"h"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"horizontal"</code> / <code>"h"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hbar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-grouped">

Plusieurs séries côte à côte par catégorie. Alias : `"group"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"grouped"</code> / <code>"group"</code></span><span><strong>Requis</strong> <code>series</code>, <code>series_names</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/grouped-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-stacked">

Séries empilées verticalement — montre la part de chaque série dans le total. Alias : `"stack"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"stacked"</code> / <code>"stack"</code></span><span><strong>Requis</strong> <code>series</code>, <code>series_names</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/stacked-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-relative">

100% empilé — chaque colonne est normalisée à 100%, montrant la part de chaque série dans le total. Alias : `"rel"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"relative"</code> / <code>"rel"</code></span><span><strong>Requis</strong> <code>series</code>, <code>series_names</code></span><span><strong>Note</strong> chaque colonne normalisée à 100%</span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/relative-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-grouped_stacked">

Groupes de sous-barres empilées par catégorie. `offset_groups` assigne un nom de pile à chaque série.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"grouped_stacked"</code></span><span><strong>Requis</strong> <code>series</code>, <code>offset_groups</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/grouped-stacked-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-marimekko">

Barres empilées à largeur variable (mosaïque). `widths` encode une dimension, les segments empilés encodent la part. Alias : `"mekko"`, `"mosaic"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"marimekko"</code> / <code>"mekko"</code> / <code>"mosaic"</code></span><span><strong>Requis</strong> <code>series</code>, <code>widths</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/marimekko-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-pictogram">

Barre composée d'icônes répétées. Chaque icône représente `units_per_icon` unités. Alias : `"icon"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"pictogram"</code> / <code>"icon"</code></span><span><strong>Requis</strong> <code>values</code>, <code>units_per_icon</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/pictogram-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-multicategory">

Axe x à deux niveaux. `super_categories` regroupe les barres adjacentes sous un label chapeau. Alias : `"multi"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"multicategory"</code> / <code>"multi"</code></span><span><strong>Requis</strong> <code>values</code>, <code>super_categories</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/multicategory-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-circular">

Barres disposées radialement autour d'un centre, longueur proportionnelle à la valeur. Passe `show_values=True` pour une valeur à l'extrémité de chaque barre, `gridlines=True` pour des anneaux concentriques étiquetés. Alias : `"circular_basic"`, `"radial_bar"`, `"polar_bar"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"circular"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Optionnel</strong> <code>show_values</code>, <code>gridlines</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/circular-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-circular_grouped">

Barres circulaires réparties en groupes via `color_groups`, avec un écart supplémentaire entre groupes. Mêmes options `show_values`/`gridlines` que `circular`. Alias : `"radial_grouped"`, `"circular_groups"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"circular_grouped"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code>, <code>color_groups</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/circular_grouped-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-population_pyramid">

Deux jeux de barres horizontales en miroir de part et d'autre d'un axe catégoriel commun, à partir des deux premières entrées de `series`. Alias : `"pyramid"`, `"age_pyramid"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"population_pyramid"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>series</code> (≥ 2), <code>series_names</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/population_pyramid-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-diverging">

Barres horizontales partant d'une ligne zéro vers la gauche ou la droite, colorées selon le signe — `color_high` (rouge par défaut, `0xF43F5E`) pour le positif, `color_low` (bleu par défaut, `0x636EFA`) pour le négatif — valeur imprimée à l'intérieur de la barre (blanc) si assez large, sinon juste à l'extérieur. Alias : `"signed"`, `"delta"`, `"bidirectional"`. Passez `series` (avec `category_labels`) au lieu de `values` pour un empilement diverging multi-séries — chaque série s'empile de son propre côté de zéro selon son propre signe, une couleur par série issue de `palette`, avec une vraie légende nommant chaque série (pas une étiquette générique « positif/négatif »). `error_low`/`error_high` optionnels (une entrée par barre) dessinent une moustache ; `overlay_line` optionnel (+ `overlay_line_label`) dessine une ligne connectée par-dessus les barres, appariée par position. L'affichage de la valeur est piloté par `show_values` (bool) comme pour toutes les autres variantes de bar — il vaut `True` par défaut ici pour préserver l'apparence existante, contrairement aux autres variantes où il vaut `False` par défaut.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"diverging"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/diverging-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-distribution">

Fusion bar + boxplot : une barre semi-transparente jusqu'à la moyenne de chaque catégorie, avec une vraie boîte (Q1/médiane/Q3, moustaches) superposée montrant la distribution derrière cette moyenne — passez `series` comme un tableau d'échantillons bruts par catégorie (même forme que l'entrée groupée de boxplot) au lieu de valeurs agrégées uniques. Alias : `"bar_box"`, `"boxbar"`, `"bar_boxplot"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"distribution"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>series</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bar-distribution.html"></iframe>
</div>

</div><!-- /sp-cls-body -->
</div>

</div><!-- /lang-fr -->
