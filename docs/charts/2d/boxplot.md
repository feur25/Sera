# Box Plot

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
.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.boxplot(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

Aliases: `sp.boxplot`, `sp.box_plot`

## Description

`sp.boxplot()` is the unified entry point for the entire box-plot family. The `variant` keyword selects the rendering strategy — every other argument stays consistent across variants. Quartiles, 1.5×IQR whiskers and outliers are computed in pure Rust without NumPy or pandas.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Standard vertical box-and-whisker | `labels`, `values` |
| `"horizontal"` / `"hbox"` | Rotated layout, ideal for long category names | `labels`, `values` |
| `"notched"` / `"ci"` | Notch around the median = 95% confidence interval | `notch=True` |
| `"grouped"` / `"side_by_side"` | Multiple series side-by-side per category | `series`, `series_names` |
| `"points"` / `"all_points"` | Box overlaid with every individual sample | `show_points=True`, `jitter` |
| `"outliers"` / `"fliers"` | Larger outlier markers + numeric labels | `labels`, `values` |
| `"strip"` / `"swarm"` | Pure jittered scatter (no box) with median line | `jitter` |
| `"violin"` / `"density"` | Box wrapped in a kernel-density silhouette | `labels`, `values` |
| `"letter_value"` / `"boxen"` | Tukey letter-value plot for tail-heavy data | `boxen_depth` |
| `"rainbow"` / `"gradient"` | Spectral palette interpolated across categories | `labels`, `values` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | — | all | Chart title shown at the top |
| `labels` | `list[str]` | `None` | all | One category label per sample (flat list) |
| `values` | `list[float]` | `None` | all except grouped | Flat list of numeric samples aligned with `labels` |
| `series` | `list[list[float]]` | `None` | grouped | One flat list per series, split evenly across categories |
| `series_names` | `list[str]` | `None` | grouped | Legend labels for each series |
| `variant` | `str` | `"basic"` | all | Selects the box-plot variant |
| `notch` | `bool` | `False` | basic, notched, points, outliers, grouped | Render a notched waist around the median (95% CI) |
| `show_points` | `bool` | `False` | basic, points | Overlay individual samples on top of the box |
| `jitter` | `float` | `0.35` | points, strip, grouped | Horizontal scatter spread (0–1) |
| `boxen_depth` | `int` | `4` | letter_value | Number of nested quantile bands (2–7) |
| `palette` | `list[int]` | `None` | all | Per-category fill colors as hex integers |
| `width` | `int` | `900` | all | Canvas width in pixels |
| `height` | `int` | `500` | all | Canvas height in pixels |
| `x_label` | `str` | `""` | all | X-axis label |
| `y_label` | `str` | `""` | all | Y-axis label |
| `gridlines` | `bool` | `False` | all | Show horizontal gridlines |
| `sort_order` | `str` | `"none"` | all | `"asc"`, `"desc"`, `"alpha"`, `"alpha_desc"`, or `"none"` |
| `legend_position` | `str` | `"right"` | all | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | all | Background CSS color; `None` = transparent |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="bx-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bx-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bx-en','basic',this)"><span class="sp-cic">▭</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','horizontal',this)"><span class="sp-cic">⇆</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','notched',this)"><span class="sp-cic">◊</span><span class="sp-clb">Notched</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','grouped',this)"><span class="sp-cic">▦</span><span class="sp-clb">Grouped</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','points',this)"><span class="sp-cic">⁝</span><span class="sp-clb">Points</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','outliers',this)"><span class="sp-cic">⊙</span><span class="sp-clb">Outliers</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','strip',this)"><span class="sp-cic">⋮</span><span class="sp-clb">Strip</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','violin',this)"><span class="sp-cic">◇</span><span class="sp-clb">Violin</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','letter',this)"><span class="sp-cic">≡</span><span class="sp-clb">Letter-Value</span></button>
<button class="sp-cls-tab" onclick="spCls('bx-en','rainbow',this)"><span class="sp-cic">◑</span><span class="sp-clb">Rainbow</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bx-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-basic','b-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('b-basic','b-basic-rust',this)">Rust</button>
</div>
<div id="b-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(1)
cats, vals = [], []
for i, c in enumerate(["Class A", "Class B", "Class C", "Class D"]):
    for _ in range(55):
        cats.append(c)
        vals.append(random.gauss(70 + i*6, 12 - i*0.8))
chart = sp.boxplot("Exam Scores", labels=cats, values=vals, variant="basic")
chart.show()</code></pre></div>
<div id="b-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;
let chart = sp::boxplot()
    .title("Exam Scores")
    .variant("basic")
    .labels(cats)
    .values(vals)
    .build();
chart.show();</code></pre></div>
</div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-basic.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-horizontal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code> / <code>"hbox"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-horizontal">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-horizontal','b-horizontal-py',this)">Python</button>
</div>
<div id="b-horizontal-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.boxplot(
    "Salaries by Department (USD)",
    labels=cats, values=vals,
    variant="horizontal",
)
chart.show()</code></pre></div>
</div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-horizontal.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-notched">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"notched"</code> / <code>"ci"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-notched">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-notched','b-notched-py',this)">Python</button>
</div>
<div id="b-notched-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">chart = sp.boxplot(
    "Blood Pressure (mmHg)",
    labels=cats, values=vals,
    variant="notched", notch=True,
)
chart.show()</code></pre></div>
</div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-notched.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-grouped">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"grouped"</code> / <code>"side_by_side"</code></span><span><strong>Required</strong> <code>labels</code>, <code>series</code>, <code>series_names</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-grouped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-grouped','b-grouped-py',this)">Python</button>
</div>
<div id="b-grouped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">chart = sp.boxplot(
    "Quarterly Performance by Region",
    labels=["Q1","Q2","Q3","Q4"],
    series=[s_north, s_south, s_east],
    series_names=["North","South","East"],
    variant="grouped",
)
chart.show()</code></pre></div>
</div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-grouped.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-points">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"points"</code> / <code>"all_points"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-points">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-points','b-points-py',this)">Python</button>
</div>
<div id="b-points-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">chart = sp.boxplot(
    "Sleep Quality Score",
    labels=cats, values=vals,
    variant="points", show_points=True, jitter=0.4,
)
chart.show()</code></pre></div>
</div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-points.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-outliers">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outliers"</code> / <code>"fliers"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-outliers">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-outliers','b-outliers-py',this)">Python</button>
</div>
<div id="b-outliers-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">chart = sp.boxplot(
    "Sensor Readings (Outlier Highlight)",
    labels=cats, values=vals,
    variant="outliers",
)
chart.show()</code></pre></div>
</div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-outliers.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-strip">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"strip"</code> / <code>"swarm"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-strip">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-strip','b-strip-py',this)">Python</button>
</div>
<div id="b-strip-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">chart = sp.boxplot(
    "Conversion Rate Distribution",
    labels=cats, values=vals,
    variant="strip", jitter=0.55,
)
chart.show()</code></pre></div>
</div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-strip.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-violin">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"violin"</code> / <code>"density"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-violin">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-violin','b-violin-py',this)">Python</button>
</div>
<div id="b-violin-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">chart = sp.boxplot(
    "Model Accuracy Distribution",
    labels=cats, values=vals,
    variant="violin",
)
chart.show()</code></pre></div>
</div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-violin.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-letter">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"letter_value"</code> / <code>"boxen"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Optional</strong> <code>boxen_depth</code> (2–7)</span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-letter">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-letter','b-letter-py',this)">Python</button>
</div>
<div id="b-letter-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">chart = sp.boxplot(
    "Order Value (USD) by Region",
    labels=cats, values=vals,
    variant="letter_value", boxen_depth=5,
)
chart.show()</code></pre></div>
</div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-letter_value.html"></iframe>

</div>

<div class="sp-variant" id="bx-en-rainbow">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rainbow"</code> / <code>"gradient"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-rainbow">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-rainbow','b-rainbow-py',this)">Python</button>
</div>
<div id="b-rainbow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">chart = sp.boxplot(
    "Daily Temperature (Celsius)",
    labels=months, values=vals,
    variant="rainbow",
)
chart.show()</code></pre></div>
</div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/boxplot-rainbow.html"></iframe>

</div>

</div>
</div>

</div>
