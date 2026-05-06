# Line Charts

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

.sp-variant > p:first-of-type{margin:0;padding:14px 18px 8px;background:linear-gradient(180deg,rgba(99,102,241,.08),rgba(99,102,241,.03));border:1px solid rgba(99,102,241,.18);border-bottom:none;border-radius:10px 10px 0 0;color:#e2e8f0;font-size:14px;line-height:1.55;font-weight:500}
.sp-variant > p:first-of-type + pre{margin:0 0 18px;padding:14px 18px 16px;background:linear-gradient(180deg,#0d1326,#080d1a);border:1px solid rgba(99,102,241,.18);border-top:none;border-radius:0 0 10px 10px;box-shadow:0 6px 18px -8px rgba(0,0,0,.6);overflow-x:auto}
.sp-variant > p:first-of-type + pre code{background:none;padding:0;font-size:12.5px;line-height:1.55;color:#cbd5e1}

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

`sp.line(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

Aliases: `sp.line`, `sp.lines`, `sp.line_unified`, `sp.lines_unified`, `sp.line_family`, `sp.line_chart`

## Description

`sp.line()` is the unified entry point for the entire line-chart family. The `variant` keyword selects the rendering strategy — every other argument is shared across variants.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Single ordered series | `labels`, `values` |
| `"multi"` / `"multiline"` | Several series, shared x-axis | `series`, `x_labels` |
| `"stepped"` / `"step"` / `"hv"` | Piecewise-constant signal | `step_shape` |
| `"spline"` / `"smooth"` | Smoothed Catmull-Rom curve | `spline_tension` |
| `"filled"` / `"area"` | Area chart | `fill_opacity`, `stack_fill` |
| `"sparkline"` / `"spark"` | Tiny inline charts grid | `spark_cols`, `spark_cell_w/h` |
| `"dashed"` / `"dotted"` | Custom stroke pattern | `dash_pattern`, `stroke_width` |
| `"connected_scatter"` / `"markers"` | Line + visible markers | `marker_size` |
| `"gapped"` / `"missing"` | Breaks across NaN gaps | `gap_threshold` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | required | all | Chart title |
| `labels` | `list[str]` | `None` | basic, stepped, spline, filled, dashed, connected_scatter, gapped | Category labels for x-axis |
| `values` | `list[float]` | `None` | single-series variants | Y-values |
| `series` | `list[(str, list[float])]` | `None` | multi, sparkline | Named series tuples |
| `x_labels` | `list[str]` | `None` | multi | Shared x-axis labels |
| `variant` | `str` | `"basic"` | — | Which variant to render |
| `step_shape` | `str` | `"hv"` | stepped | `"hv"`, `"vh"`, `"hvh"` or `"vhv"` |
| `spline_tension` | `float` | `0.5` | spline | Catmull-Rom tension (0–1) |
| `fill_opacity` | `float` | `0.3` | filled | Fill alpha (0–1) |
| `stack_fill` | `bool` | `False` | filled | Stack multiple filled series |
| `dash_pattern` | `str` | `"8,4"` | dashed | SVG `stroke-dasharray` |
| `stroke_width` | `float` | `2.0` | all | Line thickness in px |
| `marker_size` | `int` | `4` | connected_scatter | Marker radius in px |
| `gap_threshold` | `float` | `NaN` | gapped | Break line when |Δy| > threshold |
| `spark_cols` | `int` | `3` | sparkline | Columns in the grid |
| `spark_cell_w` | `int` | `220` | sparkline | Each cell width in px |
| `spark_cell_h` | `int` | `60` | sparkline | Each cell height in px |
| `show_points` | `bool` | `True` | all | Render data-point markers |
| `color_hex` | `int` | `0` | single-series | Override line color (hex int) |
| `palette` | `list[int]` | `None` | all | Custom color list |
| `width` | `int` | `900` | all | Canvas width in px |
| `height` | `int` | `480` | all | Canvas height in px |
| `x_label` | `str` | `""` | all | X-axis label |
| `y_label` | `str` | `""` | all | Y-axis label |
| `gridlines` | `bool` | `False` | all | Show horizontal gridlines |
| `sort_order` | `str` | `"none"` | basic | `"asc"`, `"desc"`, `"alpha"`, `"alpha_desc"`, `"none"` |
| `legend_position` | `str` | `"right"` | multi, sparkline | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | all | Background CSS color; `None` = transparent |

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
</div>
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

## Signature

`sp.line(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

Alias : `sp.line`, `sp.lines`, `sp.line_unified`, `sp.lines_unified`, `sp.line_family`, `sp.line_chart`

## Description

`sp.line()` est le point d'entrée unifié pour toute la famille de graphiques en ligne. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments sont partagés entre les variantes.

| Variante | Cas d'usage | Arguments clés |
|----------|-------------|----------------|
| `"basic"` | Série unique ordonnée | `labels`, `values` |
| `"multi"` / `"multiline"` | Plusieurs séries, axe x partagé | `series`, `x_labels` |
| `"stepped"` / `"step"` / `"hv"` | Signal constant par morceaux | `step_shape` |
| `"spline"` / `"smooth"` | Courbe Catmull-Rom lissée | `spline_tension` |
| `"filled"` / `"area"` | Graphique en aire | `fill_opacity`, `stack_fill` |
| `"sparkline"` / `"spark"` | Grille de petits graphiques inline | `spark_cols`, `spark_cell_w/h` |
| `"dashed"` / `"dotted"` | Motif de trait personnalisé | `dash_pattern`, `stroke_width` |
| `"connected_scatter"` / `"markers"` | Ligne + marqueurs visibles | `marker_size` |
| `"gapped"` / `"missing"` | Coupures sur les NaN | `gap_threshold` |

---

## Paramètres

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `labels` | `list[str]` | `None` | basic, stepped, spline, filled, dashed, connected_scatter, gapped | Étiquettes de l'axe x |
| `values` | `list[float]` | `None` | variantes mono-série | Valeurs y |
| `series` | `list[(str, list[float])]` | `None` | multi, sparkline | Tuples (nom, valeurs) par série |
| `x_labels` | `list[str]` | `None` | multi | Étiquettes partagées de l'axe x |
| `variant` | `str` | `"basic"` | — | Variante à rendre |
| `step_shape` | `str` | `"hv"` | stepped | `"hv"`, `"vh"`, `"hvh"` ou `"vhv"` |
| `spline_tension` | `float` | `0.5` | spline | Tension Catmull-Rom (0–1) |
| `fill_opacity` | `float` | `0.3` | filled | Alpha de remplissage (0–1) |
| `stack_fill` | `bool` | `False` | filled | Empile plusieurs séries remplies |
| `dash_pattern` | `str` | `"8,4"` | dashed | `stroke-dasharray` SVG |
| `stroke_width` | `float` | `2.0` | toutes | Épaisseur du trait en px |
| `marker_size` | `int` | `4` | connected_scatter | Rayon des marqueurs en px |
| `gap_threshold` | `float` | `NaN` | gapped | Coupe la ligne quand |Δy| > seuil |
| `spark_cols` | `int` | `3` | sparkline | Colonnes dans la grille |
| `spark_cell_w` | `int` | `220` | sparkline | Largeur d'une cellule en px |
| `spark_cell_h` | `int` | `60` | sparkline | Hauteur d'une cellule en px |
| `show_points` | `bool` | `True` | toutes | Affiche les marqueurs |
| `color_hex` | `int` | `0` | mono-série | Couleur de ligne (entier hex) |
| `palette` | `list[int]` | `None` | toutes | Palette de couleurs personnalisée |
| `width` | `int` | `900` | toutes | Largeur du canvas en px |
| `height` | `int` | `480` | toutes | Hauteur du canvas en px |
| `x_label` | `str` | `""` | toutes | Étiquette de l'axe x |
| `y_label` | `str` | `""` | toutes | Étiquette de l'axe y |
| `gridlines` | `bool` | `False` | toutes | Affiche les lignes de grille horizontales |
| `sort_order` | `str` | `"none"` | basic | `"asc"`, `"desc"`, `"alpha"`, `"alpha_desc"`, `"none"` |
| `legend_position` | `str` | `"right"` | multi, sparkline | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | toutes | Couleur de fond CSS ; `None` = transparent |

---

## Retourne

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
</div>
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
