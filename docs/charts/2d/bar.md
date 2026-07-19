# Bar Charts

<div class="lang-en">

<style>

.lang-en table,.lang-fr table{width:100%}
.sp-panel-source{display:none!important}
.sp-tabs{border:1px solid var(--sp-border);border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:var(--sp-surface);border-bottom:1px solid var(--sp-border);flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:var(--sp-text-muted);cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:var(--sp-text)}
.sp-tb.sp-act{color:var(--sp-accent);border-bottom-color:var(--sp-accent)}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}

/* ── Filing-cabinet (classeur) — bookmark tabs that protrude LEFT ─────── */
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}
.sp-cls-toggle:hover{background:#312e81;color:#e0e7ff;transform:translateY(-1px)}

/* The actual bookmark — sticks out 30px LEFT of the rail */
.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540 0%,#141d33 70%,var(--sp-surface) 100%);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;box-shadow:-6px 4px 14px -4px rgba(0,0,0,.55),inset 0 1px 0 rgba(255,255,255,.04),inset 1px 0 0 rgba(255,255,255,.05);transition:all .25s cubic-bezier(.5,0,.2,1);clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}
.sp-cls-tab:hover{background:linear-gradient(90deg,#23304d,#1a2540 70%,#141d33);color:#e0e7ff;margin-left:-40px;box-shadow:-8px 6px 18px -4px rgba(0,0,0,.6),inset 0 1px 0 rgba(255,255,255,.06)}
.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3 0%,#1e1b4b 50%,var(--sp-surface) 100%);color:#f5f3ff;margin-left:-46px;box-shadow:-10px 8px 22px -4px rgba(99,102,241,.35),-3px 0 0 0 #818cf8 inset,inset 0 1px 0 rgba(165,180,252,.2);font-weight:700;z-index:3}
.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;font-weight:900;letter-spacing:-1px;width:16px;text-align:center;text-shadow:0 0 6px rgba(165,180,252,.4)}
.sp-cls-tab.sp-cact .sp-cic{color:#e0e7ff;text-shadow:0 0 10px rgba(165,180,252,.7)}
.sp-cls-tab .sp-clb{display:none;font-weight:inherit;letter-spacing:.01em}
.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}
.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;position:relative;z-index:1;border-radius:0 14px 14px 0;overflow:hidden}
.sp-variant{display:none}
.sp-variant.sp-von{display:block;animation:spFade .25s ease}
@keyframes spFade{from{opacity:0;transform:translateX(8px)}to{opacity:1;transform:translateX(0)}}

/* Description + signature isolated as a card ABOVE the meta block.
   Built from the first <p> + first <pre> in each variant — no markdown rewrite needed. */
.sp-variant > p:first-of-type{margin:0;padding:14px 18px 8px;background:linear-gradient(180deg,rgba(99,102,241,.08),rgba(99,102,241,.03));border:1px solid rgba(99,102,241,.18);border-bottom:none;border-radius:10px 10px 0 0;color:var(--sp-text);font-size:14px;line-height:1.55;font-weight:500}
.sp-variant > p:first-of-type + pre{margin:0 0 18px;padding:14px 18px 16px;background:linear-gradient(180deg,#0d1326,#080d1a);border:1px solid rgba(99,102,241,.18);border-top:none;border-radius:0 0 10px 10px;box-shadow:0 6px 18px -8px rgba(0,0,0,.6);overflow-x:auto}
.sp-variant > p:first-of-type + pre code{background:none;padding:0;font-size:12.5px;line-height:1.55;color:#cbd5e1}

.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid var(--sp-accent);border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:var(--sp-text);font-size:12px}
.sp-preview-frame{width:100%;height:380px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}

/* Inside the API panel: the rail is hoisted out as a sidebar (see custom.css) */
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

Unknown variant strings fall back to the registered default. Variant keys may be prefixed with `en_`, `fr_`, `en-` or `fr-`.

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

Une variante inconnue retombe sur la valeur par défaut enregistrée. Les clés de variantes peuvent être préfixées par `en_`, `fr_`, `en-` ou `fr-`.

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

</div><!-- /sp-cls-body -->
</div>

</div><!-- /lang-fr -->
