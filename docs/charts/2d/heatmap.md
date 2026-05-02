# Heatmap

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
.sp-cls.sp-open .sp-cls-rail{min-width:180px;padding:18px 8px}
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
.sp-preview-frame{width:100%;height:440px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.heatmap(title, labels=None, values=None, *, variant="basic", col_labels=None, **kwargs) -> Chart`

Aliases: `sp.heatmap`, `sp.heat_map`

## Description

`sp.heatmap()` is the unified entry point for the entire heatmap family. The `variant` keyword selects the rendering strategy — every other argument stays consistent across variants. Cell colors are computed in pure Rust, no NumPy required. The matrix is passed as a flat list of length `len(labels) * len(col_labels)` (row-major).

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Default colored grid, 3-stop interpolation | `colorscale`, `colorbar_position` |
| `"annotated"` | Cell values printed on top of the grid | `show_values=True` |
| `"categorical"` | Discrete palette indexed by integer values | `palette` |
| `"unequal"` / `"variable"` | Variable cell widths/heights | `widths`, `ranges` |
| `"log"` / `"log_scale"` | Log10 normalization for skewed data | `colorscale` |
| `"discrete"` / `"binned"` | N quantized bands instead of a gradient | `bins` |
| `"correlation"` / `"corr"` | Diverging colorscale around 0 | `colorscale="rdbu_r"` |
| `"density"` / `"imshow"` | Continuous viridis-style field, no values | `colorscale`, `origin_lower` |
| `"contour"` | Smooth interpolation with isoline overlays | `colorscale` |
| `"temporal"` / `"calendar"` | Calendar-style activity grid | `colorscale="greens"` |
| `"cluster"` / `"clustermap"` | Rows + columns reordered by sum (dendrogram-like) | `colorscale` |
| `"bubble"` / `"punchcard"` | Each cell rendered as a sized circle | `colorscale` |
| `"marginal"` / `"with_marginals"` | Side bar charts for row + column totals | `colorscale` |
| `"confusion"` / `"confusion_matrix"` | Diagonal-emphasized matrix for classifiers | `colorscale="blues"` |
| `"pivot"` / `"pivot_table"` | Pivot table with row + column + grand totals | `colorscale` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | — | all | Chart title shown at the top |
| `labels` | `list[str]` | `None` | all | Row labels |
| `col_labels` | `list[str]` | `None` | all | Column labels |
| `values` | `list[float]` | `None` | all | Flat row-major matrix (`len(labels) * len(col_labels)`) |
| `variant` | `str` | `"basic"` | all | Selects the heatmap variant |
| `colorscale` | `str` | `""` | most | Named gradient, see list below |
| `colorbar_position` | `str` | `"right"` | continuous | `"right"` (vertical) or `"bottom"` (horizontal) |
| `origin_lower` | `bool` | `False` | all | Flip the y-axis so row 0 is at the bottom |
| `show_values` | `bool` | `True` | all | Print numeric values in cells |
| `bins` | `int` | `0` | discrete | Number of quantized bands (3–9) |
| `palette` | `list[int]` | `None` | categorical, discrete | Hex colors for categorical/discrete cells |
| `widths` | `list[float]` | `None` | unequal | Per-column width multipliers |
| `ranges` | `list[float]` | `None` | unequal | Per-row height multipliers |
| `color_low` | `int` | `0x6366F1` | basic, annotated, log, contour | Low-value hex color (3-stop interp) |
| `color_mid` | `int` | `0xFAFBFC` | basic, annotated, log, contour | Mid-value hex color |
| `color_high` | `int` | `0xF43F5E` | basic, annotated, log, contour | High-value hex color |
| `width` | `int` | `720` | all | Canvas width in pixels |
| `height` | `int` | `440` | all | Canvas height in pixels |
| `sort_order` | `str` | `"none"` | all | `"asc"`, `"desc"`, `"alpha"`, `"alpha_desc"`, or `"none"` |
| `background` | `str` | `None` | all | Background CSS color; `None` = transparent |

### Available colorscales

| Name | Type | Use case |
|------|------|----------|
| `"viridis"` | sequential | Perceptually uniform, default scientific colormap |
| `"plasma"` | sequential | Purple → orange, high contrast |
| `"inferno"` | sequential | Black → yellow, dark backgrounds |
| `"magma"` | sequential | Black → pink, soft warm tones |
| `"cividis"` | sequential | Color-blind safe, blue → yellow |
| `"turbo"` | sequential | Rainbow replacement, smooth |
| `"rdbu_r"` / `"rdbu"` | diverging | Red ↔ Blue around zero (recommended for correlations) |
| `"blues"` | sequential | White → deep blue (confusion matrices) |
| `"reds"` | sequential | White → deep red (alerts, errors) |
| `"greens"` | sequential | White → deep green (calendar contributions) |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="hm-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('hm-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('hm-en','basic',this)"><span class="sp-cic">▦</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','annotated',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Annotated</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','categorical',this)"><span class="sp-cic">#</span><span class="sp-clb">Categorical</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','unequal',this)"><span class="sp-cic">⊟</span><span class="sp-clb">Unequal</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','log',this)"><span class="sp-cic">㏒</span><span class="sp-clb">Log</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','discrete',this)"><span class="sp-cic">▤</span><span class="sp-clb">Discrete</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','correlation',this)"><span class="sp-cic">◰</span><span class="sp-clb">Correlation</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','density',this)"><span class="sp-cic">░</span><span class="sp-clb">Density</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','contour',this)"><span class="sp-cic">◌</span><span class="sp-clb">Contour</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','temporal',this)"><span class="sp-cic">▥</span><span class="sp-clb">Temporal</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','cluster',this)"><span class="sp-cic">⌘</span><span class="sp-clb">Cluster</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','bubble',this)"><span class="sp-cic">◯</span><span class="sp-clb">Bubble</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','marginal',this)"><span class="sp-cic">⊥</span><span class="sp-clb">Marginal</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','confusion',this)"><span class="sp-cic">⊠</span><span class="sp-clb">Confusion</span></button>
<button class="sp-cls-tab" onclick="spCls('hm-en','pivot',this)"><span class="sp-cic">⊡</span><span class="sp-clb">Pivot</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="hm-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-basic.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-annotated">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"annotated"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-annotated.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-categorical">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code>, <code>palette</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-categorical.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-unequal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"unequal"</code> / <code>"variable"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code>, <code>widths</code>, <code>ranges</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-unequal.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-log">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"log"</code> / <code>"log_scale"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-log.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-discrete">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"discrete"</code> / <code>"binned"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code>, <code>bins</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-discrete.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-correlation">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"correlation"</code> / <code>"corr"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-correlation.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-density">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"density"</code> / <code>"imshow"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-density.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-contour">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"contour"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-contour.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-temporal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"temporal"</code> / <code>"calendar"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-temporal.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-cluster">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cluster"</code> / <code>"clustermap"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-cluster.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-bubble">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bubble"</code> / <code>"punchcard"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-bubble.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-marginal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"marginal"</code> / <code>"with_marginals"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-marginal.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-confusion">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"confusion"</code> / <code>"confusion_matrix"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-confusion.html"></iframe>

</div>

<div class="sp-variant" id="hm-en-pivot">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"pivot"</code> / <code>"pivot_table"</code></span><span><strong>Required</strong> <code>labels</code>, <code>col_labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-pivot.html"></iframe>

</div>

</div>
</div>

</div>
