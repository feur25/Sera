# Histogram Charts

<div class="lang-en">

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}
.sp-cls-toggle:hover{background:#312e81;color:#e0e7ff;transform:translateY(-1px)}
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
.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
.sp-preview-frame{width:100%;height:380px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.histogram(title, values, *, variant="basic", bins=0, overlay_values=None, color_groups=None, series_names=None, **kwargs) -> Chart`

Aliases: `sp.histogram`, `sp.histograms`, `sp.histogram_unified`, `sp.histogram_family`

## Description

`sp.histogram()` is the unified entry point for the entire histogram family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. Histograms are the canonical way to visualize the distribution of a single numeric variable; SeraPlot adds horizontal layout, density normalization, cumulative distribution, stacked groups, A/B overlay and step outline — all in pure Rust SVG, thousands of times faster than Plotly.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Vertical bars, single distribution | `values`, `bins` |
| `"horizontal"` / `"barh"` | Bars grow rightward, bin labels on Y | `values` |
| `"normalized"` / `"density"` | Probability density (counts / total / bin width) | `values` |
| `"cumulative"` / `"cdf"` | Cumulative distribution function | `values` |
| `"stacked"` / `"stack"` | One bar per bin, stacked by group | `color_groups`, `palette` |
| `"overlay"` / `"compare"` | Two overlapping distributions for A/B | `overlay_values`, `series_names` |
| `"step"` / `"outline"` | Outline-only step curve, no fill | `stroke_width` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | required | all | Chart title |
| `values` | `list[float]` | required | all | Numeric values to bin |
| `variant` | `str` | `"basic"` | — | Rendering variant (see table) |
| `bins` | `int` | `0` | all | Number of bins. 0 = auto (Sturges) |
| `color_hex` | `int` | `0x6366F1` | all | Primary bar color |
| `overlay_values` | `list[float]` | `None` | overlay | Second distribution |
| `overlay_color_hex` | `int` | `0xF43F5E` | overlay | Overlay color |
| `color_groups` | `list[str]` | `None` | stacked | Per-value group label |
| `palette` | `list[int]` | `None` | stacked | Custom group palette |
| `series_names` | `list[str]` | `None` | overlay | Two legend names |
| `show_counts` | `bool` | `False` | basic, horizontal | Show count labels on bars |
| `stroke_width` | `float` | `1.0` | step | Outline stroke width |
| `width` | `int` | `860` | all | Canvas width (px) |
| `height` | `int` | `380` | all | Canvas height (px) |
| `x_label` | `str` | `""` | all | X-axis label |
| `y_label` | `str` | `"Count"` | all | Y-axis label |
| `gridlines` | `bool` | `False` | all | Show gridlines |
| `background` | `str` | `None` | all | Background CSS color |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="histogram-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('histogram-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('histogram-en','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','horizontal',this)"><span class="sp-cic">H</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','normalized',this)"><span class="sp-cic">P</span><span class="sp-clb">Density</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','cumulative',this)"><span class="sp-cic">C</span><span class="sp-clb">Cumulative</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','overlay',this)"><span class="sp-cic">O</span><span class="sp-clb">Overlay</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-en','step',this)"><span class="sp-cic">L</span><span class="sp-clb">Step</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="histogram-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / simple / default / vertical</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-basic.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-horizontal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / h / barh / hbar</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-horizontal.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-normalized">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"normalized"</code></span><span><strong>Aliases</strong> <code>normalized / probability / density / pdf</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-normalized.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-cumulative">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cumulative"</code></span><span><strong>Aliases</strong> <code>cumulative / cdf / cum</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-cumulative.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-stacked">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stack / stack_by</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-stacked.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-overlay">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"overlay"</code></span><span><strong>Aliases</strong> <code>overlay / overlapping / compare / ab</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-overlay.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-step">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"step"</code></span><span><strong>Aliases</strong> <code>step / outline / stair</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-step.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr">

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}
.sp-cls-toggle:hover{background:#312e81;color:#e0e7ff;transform:translateY(-1px)}
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
.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
.sp-preview-frame{width:100%;height:380px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


<h2>Signature</h2>

`sp.histogram(title, values, *, variant="basic", bins=0, overlay_values=None, color_groups=None, series_names=None, **kwargs) -> Chart`

Alias : `sp.histogram`, `sp.histograms`, `sp.histogram_unified`, `sp.histogram_family`

<h2>Description</h2>

`sp.histogram()` est le point d'entrée unifié de toute la famille histogramme. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments gardent le même nom d'une variante à l'autre. Les histogrammes sont la façon canonique de visualiser la distribution d'une variable numérique ; SeraPlot ajoute layout horizontal, normalisation densité, distribution cumulative, groupes empilés, superposition A/B et contour en escalier — le tout en SVG Rust pur, des milliers de fois plus rapide que Plotly.

| Variante | Usage | Arguments clés |
|----------|-------|----------------|
| `"basic"` | Barres verticales, une seule distribution | `values`, `bins` |
| `"horizontal"` / `"barh"` | Barres horizontales, étiquettes sur Y | `values` |
| `"normalized"` / `"density"` | Densité de probabilité | `values` |
| `"cumulative"` / `"cdf"` | Fonction de répartition cumulée | `values` |
| `"stacked"` / `"stack"` | Une barre par bin, empilée par groupe | `color_groups`, `palette` |
| `"overlay"` / `"compare"` | Deux distributions superposées (A/B) | `overlay_values`, `series_names` |
| `"step"` / `"outline"` | Courbe en escalier sans remplissage | `stroke_width` |

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `values` | `list[float]` | requis | toutes | Valeurs numériques à binner |
| `variant` | `str` | `"basic"` | — | Variante de rendu |
| `bins` | `int` | `0` | toutes | Nombre de bins. 0 = auto (Sturges) |
| `color_hex` | `int` | `0x6366F1` | toutes | Couleur principale |
| `overlay_values` | `list[float]` | `None` | overlay | Deuxième distribution |
| `overlay_color_hex` | `int` | `0xF43F5E` | overlay | Couleur d'overlay |
| `color_groups` | `list[str]` | `None` | stacked | Étiquette de groupe par valeur |
| `palette` | `list[int]` | `None` | stacked | Palette personnalisée |
| `series_names` | `list[str]` | `None` | overlay | Deux noms de légende |
| `show_counts` | `bool` | `False` | basic, horizontal | Afficher les comptes |
| `stroke_width` | `float` | `1.0` | step | Largeur du contour |
| `width` | `int` | `860` | toutes | Largeur (px) |
| `height` | `int` | `380` | toutes | Hauteur (px) |
| `x_label` | `str` | `""` | toutes | Légende axe X |
| `y_label` | `str` | `"Count"` | toutes | Légende axe Y |
| `gridlines` | `bool` | `False` | toutes | Afficher la grille |
| `background` | `str` | `None` | toutes | Couleur de fond CSS |

---

<h2>Retour</h2>

`Chart` — objet exposant `.html` et `.show()`.

---

<div class="sp-cls sp-open" id="histogram-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('histogram-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('histogram-fr','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">De base</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','horizontal',this)"><span class="sp-cic">H</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','normalized',this)"><span class="sp-cic">P</span><span class="sp-clb">Densité</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','cumulative',this)"><span class="sp-cic">C</span><span class="sp-clb">Cumulatif</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Empilé</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','overlay',this)"><span class="sp-cic">O</span><span class="sp-clb">Superposé</span></button>
<button class="sp-cls-tab" onclick="spCls('histogram-fr','step',this)"><span class="sp-cic">L</span><span class="sp-clb">Contour</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="histogram-fr-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / simple / default / vertical</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-basic.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-horizontal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / h / barh / hbar</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-horizontal.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-normalized">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"normalized"</code></span><span><strong>Aliases</strong> <code>normalized / probability / density / pdf</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-normalized.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-cumulative">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cumulative"</code></span><span><strong>Aliases</strong> <code>cumulative / cdf / cum</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-cumulative.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-stacked">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stack / stack_by</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-stacked.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-overlay">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"overlay"</code></span><span><strong>Aliases</strong> <code>overlay / overlapping / compare / ab</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-overlay.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-step">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"step"</code></span><span><strong>Aliases</strong> <code>step / outline / stair</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-step.html"></iframe>
</div>

</div>
</div>

</div>
