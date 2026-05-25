# Bar Charts

<div class="lang-en">

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}

/* ── Filing-cabinet (classeur) — bookmark tabs that protrude LEFT ─────── */
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}
.sp-cls-toggle:hover{background:#312e81;color:#e0e7ff;transform:translateY(-1px)}

/* The actual bookmark — sticks out 30px LEFT of the rail */
.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540 0%,#141d33 70%,#0f172a 100%);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;box-shadow:-6px 4px 14px -4px rgba(0,0,0,.55),inset 0 1px 0 rgba(255,255,255,.04),inset 1px 0 0 rgba(255,255,255,.05);transition:all .25s cubic-bezier(.5,0,.2,1);clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}
.sp-cls-tab:hover{background:linear-gradient(90deg,#23304d,#1a2540 70%,#141d33);color:#e0e7ff;margin-left:-40px;box-shadow:-8px 6px 18px -4px rgba(0,0,0,.6),inset 0 1px 0 rgba(255,255,255,.06)}
.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3 0%,#1e1b4b 50%,#0f172a 100%);color:#f5f3ff;margin-left:-46px;box-shadow:-10px 8px 22px -4px rgba(99,102,241,.35),-3px 0 0 0 #818cf8 inset,inset 0 1px 0 rgba(165,180,252,.2);font-weight:700;z-index:3}
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
.sp-variant > p:first-of-type{margin:0;padding:14px 18px 8px;background:linear-gradient(180deg,rgba(99,102,241,.08),rgba(99,102,241,.03));border:1px solid rgba(99,102,241,.18);border-bottom:none;border-radius:10px 10px 0 0;color:#e2e8f0;font-size:14px;line-height:1.55;font-weight:500}
.sp-variant > p:first-of-type + pre{margin:0 0 18px;padding:14px 18px 16px;background:linear-gradient(180deg,#0d1326,#080d1a);border:1px solid rgba(99,102,241,.18);border-top:none;border-radius:0 0 10px 10px;box-shadow:0 6px 18px -8px rgba(0,0,0,.6);overflow-x:auto}
.sp-variant > p:first-of-type + pre code{background:none;padding:0;font-size:12.5px;line-height:1.55;color:#cbd5e1}

.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
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

`sp.bar(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

Aliases: `sp.bar`, `sp.bars`, `sp.bar_unified`, `sp.bars_unified`, `sp.bar_family`, `sp.bar_chart`

## Description


`sp.bar()` is the unified entry point for the entire bar-chart family. The `variant` keyword selects the rendering strategy — all other arguments remain consistent across variants.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Single series, vertical | `labels`, `values` |
| `"horizontal"` / `"h"` | Single series, horizontal | `labels`, `values` |
| `"grouped"` / `"group"` | Multi-series side-by-side | `series`, `series_names` |
| `"stacked"` / `"stack"` | Multi-series stacked | `series`, `series_names` |
| `"relative"` / `"rel"` | Stacked + negatives below 0 | `series`, `series_names` |
| `"grouped_stacked"` | Groups of stacked bars | `series`, `series_names`, `offset_groups` |
| `"marimekko"` / `"mekko"` / `"mosaic"` | Variable-width mosaic | `series`, `series_names`, `widths` |
| `"pictogram"` / `"icon"` | Repeated-icon bar | `labels`, `values`, `units_per_icon` |
| `"multicategory"` / `"multi"` | Two-level x axis | `labels`, `values`, `super_categories` |
| `"deluxe"` / `"premium"` / `"neon"` | Dark glowing neon bars | `labels`, `values` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | required | all | Chart title |
| `labels` | `list[str]` | `None` | all | Category labels |
| `values` | `list[float]` | `None` | basic, horizontal, pictogram, multicategory | Single-series values |
| `variant` | `str` | `"basic"` | — | Rendering variant (see table above) |
| `series` | `list[list[float]]` | `None` | grouped, stacked, relative, grouped_stacked, marimekko, multicategory | Multi-series data |
| `series_names` | `list[str]` | `None` | multi-series | Legend label per series |
| `offset_groups` | `list[str]` | `None` | grouped_stacked | Stack-group name per series |
| `widths` | `list[float]` | `None` | marimekko | Relative column width per category |
| `super_categories` | `list[str]` | `None` | multicategory | Bracket label per bar |
| `icon_size` | `int` | `24` | pictogram | Icon square size in px |
| `max_icons_per_column` | `int` | `10` | pictogram | Max icons per column before wrap |
| `units_per_icon` | `float` | `1.0` | pictogram | Data units each icon represents |
| `unit_description` | `str` | `""` | pictogram | Unit label shown in header |
| `color_hex` | `int` | `0` | basic, horizontal, pictogram | Single override color as hex int |
| `palette` | `list[int]` | `None` | all | Custom color list |
| `show_text` | `bool` | `False` | basic, horizontal, marimekko | Render value labels on bars |
| `corner_radius` | `int` | `0` | basic, horizontal | Rounded-corner radius in px |
| `bar_gap` | `float` | `0.2` | basic, horizontal, relative, grouped_stacked | Fraction of category width as spacing |
| `bargroup_gap` | `float` | `0.1` | grouped, grouped_stacked | Fraction of group width as gap between bars |
| `width` | `int` | `900` | all | Canvas width in pixels |
| `height` | `int` | `480` | all | Canvas height in pixels |
| `x_label` | `str` | `""` | all | X-axis label |
| `y_label` | `str` | `""` | all | Y-axis label |
| `gridlines` | `bool` | `False` | all | Show horizontal gridlines |
| `sort_order` | `str` | `"none"` | basic, horizontal | `"asc"`, `"desc"`, `"alpha"`, `"alpha_desc"`, or `"none"` |
| `legend_position` | `str` | `"right"` | multi-series | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | all | Background CSS color; `None` = transparent |
| `no_x_axis` | `bool` | `False` | all | Hide X axis |
| `no_y_axis` | `bool` | `False` | all | Hide Y axis |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="bar-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bar-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bar-en','basic',this)"><span class="sp-cic">▮</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','horizontal',this)"><span class="sp-cic">▬</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','grouped',this)"><span class="sp-cic">▐▐</span><span class="sp-clb">Grouped</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','stacked',this)"><span class="sp-cic">▦</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','relative',this)"><span class="sp-cic">±</span><span class="sp-clb">Relative</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','gstack',this)"><span class="sp-cic">▤</span><span class="sp-clb">Grouped-Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','marimekko',this)"><span class="sp-cic">▤</span><span class="sp-clb">Marimekko</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','pictogram',this)"><span class="sp-cic">☰</span><span class="sp-clb">Pictogram</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','multicategory',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Multicategory</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','deluxe',this)"><span class="sp-cic">✦</span><span class="sp-clb">Deluxe</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','prism',this)"><span class="sp-cic">◈</span><span class="sp-clb">Prism</span></button>
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

Stacked with **negative values below the zero baseline** — P&L, gains/losses. Alias: `"rel"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"relative"</code> / <code>"rel"</code></span><span><strong>Required</strong> <code>series</code>, <code>series_names</code></span><span><strong>Note</strong> negatives render below 0</span><span><strong>Returns</strong> <code>Chart</code></span></div>


<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/relative-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-gstack">

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

<div class="sp-variant" id="bar-en-deluxe">

Dark background, per-bar neon gradient fill, gaussian glow filter, bright top highlight strip. Aliases: `"premium"`, `"neon"`, `"dark"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"deluxe"</code> / <code>"premium"</code> / <code>"neon"</code></span><span><strong>Style</strong> dark background · neon gradients · glow</span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bar-deluxe.html"></iframe>

</div>

<div class="sp-variant" id="bar-en-prism">

Prismatic crystal bars with diagonal facet lines and shine overlay. Each bar gets a vivid rainbow gradient with glowing highlights.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"prism"</code></span><span><strong>Aliases</strong> <code>prism / crystal / glass / rainbow</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bar-prism.html"></iframe>
</div>

</div><!-- /sp-cls-body -->
</div><!-- /bar-en -->

---

</div><!-- /lang-en -->


<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.bar(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

Alias : `sp.bar`, `sp.bars`, `sp.bar_unified`, `sp.bars_unified`, `sp.bar_family`, `sp.bar_chart`

<h2>Description</h2>

`sp.bar()` est le point d'entrée unique pour toute la famille de graphiques en barres. Le paramètre `variant` sélectionne la stratégie de rendu.

| Variante | Cas d'usage | Arguments clés |
|----------|-------------|----------------|
| `"basic"` | Série unique, barres verticales | `labels`, `values` |
| `"horizontal"` / `"h"` | Série unique, barres horizontales | `labels`, `values` |
| `"grouped"` / `"group"` | Multi-séries côte à côte | `series`, `series_names` |
| `"stacked"` / `"stack"` | Multi-séries empilées | `series`, `series_names` |
| `"relative"` / `"rel"` | Empilé + négatifs sous 0 | `series`, `series_names` |
| `"grouped_stacked"` | Groupes de barres empilées | `series`, `series_names`, `offset_groups` |
| `"marimekko"` / `"mekko"` | Mosaïque à largeur variable | `series`, `series_names`, `widths` |
| `"pictogram"` / `"icon"` | Barre d'icônes répétées | `labels`, `values`, `units_per_icon` |
| `"multicategory"` / `"multi"` | Axe x hiérarchique | `labels`, `values`, `super_categories` |

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `labels` | `list[str]` | `None` | toutes | Labels de catégorie |
| `values` | `list[float]` | `None` | basic, horizontal, pictogram, multicategory | Valeurs série unique |
| `variant` | `str` | `"basic"` | — | Variante de rendu |
| `series` | `list[list[float]]` | `None` | multi-séries | Données multi-séries |
| `series_names` | `list[str]` | `None` | multi-séries | Noms des séries (légende) |
| `offset_groups` | `list[str]` | `None` | grouped_stacked | Groupe d'empilement par série |
| `widths` | `list[float]` | `None` | marimekko | Largeur relative de chaque colonne |
| `super_categories` | `list[str]` | `None` | multicategory | Label chapeau de chaque barre |
| `icon_size` | `int` | `24` | pictogram | Taille des icônes en px |
| `max_icons_per_column` | `int` | `10` | pictogram | Icônes max par colonne |
| `units_per_icon` | `float` | `1.0` | pictogram | Unités par icône |
| `unit_description` | `str` | `""` | pictogram | Label unité |
| `color_hex` | `int` | `0` | basic, horizontal, pictogram | Couleur unique (entier hex) |
| `palette` | `list[int]` | `None` | toutes | Palette personnalisée |
| `show_text` | `bool` | `False` | basic, horizontal, marimekko | Afficher les valeurs sur les barres |
| `corner_radius` | `int` | `0` | basic, horizontal | Rayon des coins en px |
| `bar_gap` | `float` | `0.2` | basic, horizontal, relative, grouped_stacked | Espacement entre catégories |
| `bargroup_gap` | `float` | `0.1` | grouped, grouped_stacked | Espacement entre barres d'un groupe |
| `width` | `int` | `900` | toutes | Largeur du canevas en px |
| `height` | `int` | `480` | toutes | Hauteur du canevas en px |
| `x_label` | `str` | `""` | toutes | Label axe X |
| `y_label` | `str` | `""` | toutes | Label axe Y |
| `gridlines` | `bool` | `False` | toutes | Lignes de grille horizontales |
| `sort_order` | `str` | `"none"` | basic, horizontal | `"asc"`, `"desc"`, `"alpha"`, `"alpha_desc"`, `"none"` |
| `legend_position` | `str` | `"right"` | multi-séries | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | toutes | Couleur de fond CSS |
| `no_x_axis` | `bool` | `False` | toutes | Masquer l'axe X |
| `no_y_axis` | `bool` | `False` | toutes | Masquer l'axe Y |

---

<h2>Retourne</h2>

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

<div class="sp-cls sp-open" id="bar-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bar-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bar-fr','basic',this)"><span class="sp-cic">▮</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','horizontal',this)"><span class="sp-cic">▬</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','grouped',this)"><span class="sp-cic">▐▐</span><span class="sp-clb">Groupé</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','stacked',this)"><span class="sp-cic">▦</span><span class="sp-clb">Empilé</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','relative',this)"><span class="sp-cic">±</span><span class="sp-clb">Relatif</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','gstack',this)"><span class="sp-cic">▤</span><span class="sp-clb">Groupé-Empilé</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','marimekko',this)"><span class="sp-cic">▤</span><span class="sp-clb">Marimekko</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','pictogram',this)"><span class="sp-cic">☰</span><span class="sp-clb">Pictogramme</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','multicategory',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Multi-catégories</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','deluxe',this)"><span class="sp-cic">✦</span><span class="sp-clb">Deluxe</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','prism',this)"><span class="sp-cic">◈</span><span class="sp-clb">Prisme</span></button>
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

Empilé avec **valeurs négatives sous l'axe zéro** — flux de trésorerie, P&L. Alias : `"rel"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"relative"</code> / <code>"rel"</code></span><span><strong>Requis</strong> <code>series</code>, <code>series_names</code></span><span><strong>Note</strong> les négatifs s'affichent sous 0</span><span><strong>Retourne</strong> <code>Chart</code></span></div>


<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/relative-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-gstack">

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

<div class="sp-variant" id="bar-fr-deluxe">

Fond sombre, gradient néon par barre, filtre de glow gaussien, liseré blanc en haut. Alias : `"premium"`, `"neon"`, `"dark"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"deluxe"</code> / <code>"premium"</code> / <code>"neon"</code></span><span><strong>Style</strong> fond sombre · dégradés néon · lueur</span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bar-deluxe.html"></iframe>

</div>

<div class="sp-variant" id="bar-fr-prism">

Barres cristal prismatiques avec lignes de facettes diagonales et surbrillance. Chaque barre reçoit un dégradé arc-en-ciel vif avec effets lumineux.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"prism"</code></span><span><strong>Alias</strong> <code>prism / crystal / glass / rainbow</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bar-prism.html"></iframe>
</div>

</div><!-- /sp-cls-body -->
</div><!-- /bar-fr -->

---

</div><!-- /lang-fr -->
