# Scatter Charts

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

`sp.scatter(title, x_values, y_values, *, variant="basic", categories=None, labels=None, color_values=None, **kwargs) -> Chart`

Aliases: `sp.scatter`, `sp.scatters`, `sp.scatter_unified`, `sp.scatter_family`

## Description

`sp.scatter()` is the unified entry point for the entire scatter family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. Scatter plots are the canonical way to display the joint distribution of two numeric variables; SeraPlot adds optional grouping, continuous color, distinct marker shapes, on-point labels and OLS regression — all in pure Rust SVG, thousands of times faster than Plotly.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Plain points, single color | `x_values`, `y_values` |
| `"categorical"` / `"grouped"` | Color-by-group with side legend | `categories` |
| `"gradient"` / `"colorscale"` | Continuous color mapping + colorbar | `color_values`, `color_low`, `color_high` |
| `"symbols"` / `"shapes"` | Distinct marker shapes per group | `categories` |
| `"labeled"` / `"text"` | Always-on per-point labels | `labels` |
| `"regression"` / `"trendline"` | OLS fit overlay (linear / poly2) | `regression_type` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | required | all | Chart title |
| `x_values` | `list[float]` | required | all | Horizontal coordinate of each point |
| `y_values` | `list[float]` | required | all | Vertical coordinate of each point |
| `variant` | `str` | `"basic"` | — | Rendering variant (see table) |
| `categories` | `list[str]` | `None` | categorical, symbols | Per-point group name |
| `labels` | `list[str]` | `None` | all (hover) / labeled (rendered) | Per-point text label |
| `color_values` | `list[float]` | `None` | gradient | Continuous numeric value for colorscale |
| `color_hex` | `int` | `0x6366F1` | basic, regression | Single fill color |
| `color_low` | `int` | `0x6366F1` | gradient | Low end of color scale |
| `color_high` | `int` | `0xF43F5E` | gradient, regression | High end of color scale / regression line |
| `point_size` | `float` | `5.0` | all | Marker radius in pixels |
| `stroke_width` | `float` | `1.0` | all | Marker stroke width |
| `symbol` | `str` | `"circle"` | all | Default marker shape (`circle`, `square`, `diamond`, `triangle`, `cross`, `plus`, `star`, `triangle-down`) |
| `symbols` | `list[str]` | `None` | symbols | Per-point marker shape override |
| `regression_type` | `str` | `"linear"` | regression | `"linear"` or `"polynomial2"` |
| `palette` | `list[int]` | `None` | categorical, symbols | Custom color palette |
| `width` | `int` | `900` | all | Canvas width (px) |
| `height` | `int` | `500` | all | Canvas height (px) |
| `x_label` | `str` | `""` | all | X-axis label |
| `y_label` | `str` | `""` | all | Y-axis label |
| `gridlines` | `bool` | `False` | all | Show gridlines |
| `legend_position` | `str` | `"right"` | categorical, symbols | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | all | Background CSS color (None = transparent) |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="scatter-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('scatter-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('scatter-en','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','categorical',this)"><span class="sp-cic">◓</span><span class="sp-clb">Categorical</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','symbols',this)"><span class="sp-cic">◆</span><span class="sp-clb">Symbols</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','labeled',this)"><span class="sp-cic">◉</span><span class="sp-clb">Labeled</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-en','regression',this)"><span class="sp-cic">↗</span><span class="sp-clb">Regression</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="scatter-en-basic">

Single-color markers — the simplest scatter, perfect to inspect raw correlation between two numeric variables.

```python
import seraplot as sp
import random
random.seed(0)
xs = [random.gauss(0,1) for _ in range(120)]
ys = [x*0.6 + random.gauss(0,0.45) for x in xs]
chart = sp.scatter(
    title="Returns vs Volatility",
    variant="basic",
    x_values=xs, y_values=ys,
    color_hex=0x6366F1,
    point_size=5.5,
    x_label="Volatility", y_label="Return",
    gridlines=True,
)
chart.show()
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / simple / default</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="scen-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scen-basic','scen-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scen-basic','scen-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scen-basic','scen-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scen-basic','scen-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scen-basic','scen-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('scen-basic','scen-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scen-basic','scen-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scen-basic','scen-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scen-basic','scen-basic-cpp',this)">C++</button>
</div><div id="scen-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
xs = [random.gauss(0,1) for _ in range(120)]
ys = [x*0.6 + random.gauss(0,0.45) for x in xs]
chart = sp.scatter(
    title="Returns vs Volatility",
    variant="basic",
    x_values=xs, y_values=ys,
    color_hex=0x6366F1,
    point_size=5.5,
    x_label="Volatility", y_label="Return",
    gridlines=True,
)
chart.show()</code></pre></div><div id="scen-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scen-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scen-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$scatter(
  title = "Scatter",
  variant = "basic",
  x_values = c(),
  y_values = c()
)
chart$show()</code></pre></div><div id="scen-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::scatter()
        .title("Scatter")
        .variant("basic")
        .x_values(vec![])
        .y_values(vec![])
        .build();
    chart.show();
}</code></pre></div><div id="scen-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Scatter")
    .variant("basic")
    .xValues(List.of())
    .yValues(List.of())
    .build();
chart.show();</code></pre></div><div id="scen-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Scatter",
    variant: "basic",
    xValues: new double[]{},
    yValues: new double[]{}
);
chart.Show();</code></pre></div><div id="scen-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.scatter(
  title    = "Scatter",
  variant  = "basic",
  x_values = List(),
  y_values = List()
)
chart.show()</code></pre></div><div id="scen-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
    .title    = "Scatter",
    .variant  = "basic",
    .x_values = {},
    .y_values = {},
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-basic.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-categorical">

Color-by-group with a side legend. Use it to compare distributions between categories on the same axes.

```python
import seraplot as sp
chart = sp.scatter(
    title="Iris — Sepal vs Petal",
    variant="categorical",
    x_values=[5.1,4.9,7.0,6.4,6.3,5.8],
    y_values=[1.4,1.4,4.7,4.5,6.0,5.1],
    categories=["setosa","setosa","versicolor","versicolor","virginica","virginica"],
    point_size=7.0,
    x_label="Sepal length", y_label="Petal length",
    gridlines=True,
)
chart.show()
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code></span><span><strong>Aliases</strong> <code>categorical / grouped / category</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="scen-categorical">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scen-categorical','scen-categorical-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scen-categorical','scen-categorical-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scen-categorical','scen-categorical-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scen-categorical','scen-categorical-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scen-categorical','scen-categorical-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('scen-categorical','scen-categorical-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scen-categorical','scen-categorical-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scen-categorical','scen-categorical-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scen-categorical','scen-categorical-cpp',this)">C++</button>
</div><div id="scen-categorical-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.scatter(
    title="Iris — Sepal vs Petal",
    variant="categorical",
    x_values=[5.1,4.9,7.0,6.4,6.3,5.8],
    y_values=[1.4,1.4,4.7,4.5,6.0,5.1],
    categories=["setosa","setosa","versicolor","versicolor","virginica","virginica"],
    point_size=7.0,
    x_label="Sepal length", y_label="Petal length",
    gridlines=True,
)
chart.show()</code></pre></div><div id="scen-categorical-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scen-categorical-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scen-categorical-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$scatter(
  title = "Scatter",
  variant = "basic",
  x_values = c(),
  y_values = c()
)
chart$show()</code></pre></div><div id="scen-categorical-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::scatter()
        .title("Scatter")
        .variant("basic")
        .x_values(vec![])
        .y_values(vec![])
        .build();
    chart.show();
}</code></pre></div><div id="scen-categorical-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Scatter")
    .variant("basic")
    .xValues(List.of())
    .yValues(List.of())
    .build();
chart.show();</code></pre></div><div id="scen-categorical-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Scatter",
    variant: "basic",
    xValues: new double[]{},
    yValues: new double[]{}
);
chart.Show();</code></pre></div><div id="scen-categorical-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.scatter(
  title    = "Scatter",
  variant  = "basic",
  x_values = List(),
  y_values = List()
)
chart.show()</code></pre></div><div id="scen-categorical-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
    .title    = "Scatter",
    .variant  = "basic",
    .x_values = {},
    .y_values = {},
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-categorical.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-gradient">

Continuous color mapping driven by a third numeric value, with a vertical colorbar legend.

```python
import seraplot as sp
chart = sp.scatter(
    title="Stars — Mass vs Luminosity",
    variant="gradient",
    x_values=[0.3,0.8,1.0,1.5,3.0,8.0,15.0],
    y_values=[0.01,0.4,1.0,5.0,80.0,3000.0,30000.0],
    color_values=[3000,4500,5800,6500,8500,15000,28000],
    color_low=0xef4444, color_high=0x3b82f6,
    point_size=8.0,
    x_label="Mass (M☉)", y_label="Luminosity (L☉)",
)
chart.show()
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / colorscale / continuous</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="scen-gradient">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scen-gradient','scen-gradient-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scen-gradient','scen-gradient-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scen-gradient','scen-gradient-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scen-gradient','scen-gradient-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scen-gradient','scen-gradient-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('scen-gradient','scen-gradient-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scen-gradient','scen-gradient-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scen-gradient','scen-gradient-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scen-gradient','scen-gradient-cpp',this)">C++</button>
</div><div id="scen-gradient-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.scatter(
    title="Stars — Mass vs Luminosity",
    variant="gradient",
    x_values=[0.3,0.8,1.0,1.5,3.0,8.0,15.0],
    y_values=[0.01,0.4,1.0,5.0,80.0,3000.0,30000.0],
    color_values=[3000,4500,5800,6500,8500,15000,28000],
    color_low=0xef4444, color_high=0x3b82f6,
    point_size=8.0,
    x_label="Mass (M☉)", y_label="Luminosity (L☉)",
)
chart.show()</code></pre></div><div id="scen-gradient-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scen-gradient-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scen-gradient-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$scatter(
  title = "Scatter",
  variant = "basic",
  x_values = c(),
  y_values = c()
)
chart$show()</code></pre></div><div id="scen-gradient-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::scatter()
        .title("Scatter")
        .variant("basic")
        .x_values(vec![])
        .y_values(vec![])
        .build();
    chart.show();
}</code></pre></div><div id="scen-gradient-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Scatter")
    .variant("basic")
    .xValues(List.of())
    .yValues(List.of())
    .build();
chart.show();</code></pre></div><div id="scen-gradient-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Scatter",
    variant: "basic",
    xValues: new double[]{},
    yValues: new double[]{}
);
chart.Show();</code></pre></div><div id="scen-gradient-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.scatter(
  title    = "Scatter",
  variant  = "basic",
  x_values = List(),
  y_values = List()
)
chart.show()</code></pre></div><div id="scen-gradient-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
    .title    = "Scatter",
    .variant  = "basic",
    .x_values = {},
    .y_values = {},
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-gradient.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-symbols">

Use distinct marker shapes per category in addition to color — improves readability for color-blind viewers.

```python
import seraplot as sp
chart = sp.scatter(
    title="Penguins — Bill vs Flipper",
    variant="symbols",
    x_values=[39.1,46.5,50.0,49.5,38.9,47.5],
    y_values=[181,217,222,209,184,219],
    categories=["Adelie","Gentoo","Chinstrap","Chinstrap","Adelie","Gentoo"],
    point_size=8.0,
    x_label="Bill length (mm)", y_label="Flipper length (mm)",
    gridlines=True,
)
chart.show()
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"symbols"</code></span><span><strong>Aliases</strong> <code>symbols / shapes / markers</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="scen-symbols">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scen-symbols','scen-symbols-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scen-symbols','scen-symbols-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scen-symbols','scen-symbols-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scen-symbols','scen-symbols-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scen-symbols','scen-symbols-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('scen-symbols','scen-symbols-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scen-symbols','scen-symbols-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scen-symbols','scen-symbols-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scen-symbols','scen-symbols-cpp',this)">C++</button>
</div><div id="scen-symbols-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.scatter(
    title="Penguins — Bill vs Flipper",
    variant="symbols",
    x_values=[39.1,46.5,50.0,49.5,38.9,47.5],
    y_values=[181,217,222,209,184,219],
    categories=["Adelie","Gentoo","Chinstrap","Chinstrap","Adelie","Gentoo"],
    point_size=8.0,
    x_label="Bill length (mm)", y_label="Flipper length (mm)",
    gridlines=True,
)
chart.show()</code></pre></div><div id="scen-symbols-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scen-symbols-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scen-symbols-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$scatter(
  title = "Scatter",
  variant = "basic",
  x_values = c(),
  y_values = c()
)
chart$show()</code></pre></div><div id="scen-symbols-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::scatter()
        .title("Scatter")
        .variant("basic")
        .x_values(vec![])
        .y_values(vec![])
        .build();
    chart.show();
}</code></pre></div><div id="scen-symbols-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Scatter")
    .variant("basic")
    .xValues(List.of())
    .yValues(List.of())
    .build();
chart.show();</code></pre></div><div id="scen-symbols-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Scatter",
    variant: "basic",
    xValues: new double[]{},
    yValues: new double[]{}
);
chart.Show();</code></pre></div><div id="scen-symbols-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.scatter(
  title    = "Scatter",
  variant  = "basic",
  x_values = List(),
  y_values = List()
)
chart.show()</code></pre></div><div id="scen-symbols-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
    .title    = "Scatter",
    .variant  = "basic",
    .x_values = {},
    .y_values = {},
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-symbols.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-labeled">

Always-on text labels above each marker, with a halo for legibility on busy charts.

```python
import seraplot as sp
chart = sp.scatter(
    title="French Cities — Cost vs Quality",
    variant="labeled",
    x_values=[2.1,1.8,1.5,1.3,1.1,1.6,1.9,1.4,1.2,2.4],
    y_values=[8.2,7.8,7.4,7.0,6.5,7.6,7.9,7.2,6.8,8.5],
    labels=["Paris","Lyon","Marseille","Toulouse","Nice","Nantes","Bordeaux","Lille","Rennes","Strasbourg"],
    point_size=7.5,
    color_hex=0x10b981,
    x_label="Cost index", y_label="Quality of life",
    gridlines=True,
)
chart.show()
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"labeled"</code></span><span><strong>Aliases</strong> <code>labeled / labels / text</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="scen-labeled">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scen-labeled','scen-labeled-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scen-labeled','scen-labeled-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scen-labeled','scen-labeled-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scen-labeled','scen-labeled-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scen-labeled','scen-labeled-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('scen-labeled','scen-labeled-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scen-labeled','scen-labeled-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scen-labeled','scen-labeled-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scen-labeled','scen-labeled-cpp',this)">C++</button>
</div><div id="scen-labeled-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.scatter(
    title="French Cities — Cost vs Quality",
    variant="labeled",
    x_values=[2.1,1.8,1.5,1.3,1.1,1.6,1.9,1.4,1.2,2.4],
    y_values=[8.2,7.8,7.4,7.0,6.5,7.6,7.9,7.2,6.8,8.5],
    labels=["Paris","Lyon","Marseille","Toulouse","Nice","Nantes","Bordeaux","Lille","Rennes","Strasbourg"],
    point_size=7.5,
    color_hex=0x10b981,
    x_label="Cost index", y_label="Quality of life",
    gridlines=True,
)
chart.show()</code></pre></div><div id="scen-labeled-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scen-labeled-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scen-labeled-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$scatter(
  title = "Scatter",
  variant = "basic",
  x_values = c(),
  y_values = c()
)
chart$show()</code></pre></div><div id="scen-labeled-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::scatter()
        .title("Scatter")
        .variant("basic")
        .x_values(vec![])
        .y_values(vec![])
        .build();
    chart.show();
}</code></pre></div><div id="scen-labeled-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Scatter")
    .variant("basic")
    .xValues(List.of())
    .yValues(List.of())
    .build();
chart.show();</code></pre></div><div id="scen-labeled-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Scatter",
    variant: "basic",
    xValues: new double[]{},
    yValues: new double[]{}
);
chart.Show();</code></pre></div><div id="scen-labeled-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.scatter(
  title    = "Scatter",
  variant  = "basic",
  x_values = List(),
  y_values = List()
)
chart.show()</code></pre></div><div id="scen-labeled-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
    .title    = "Scatter",
    .variant  = "basic",
    .x_values = {},
    .y_values = {},
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-labeled.html"></iframe>
</div>

<div class="sp-variant" id="scatter-en-regression">

Overlay an ordinary least-squares fit (linear or polynomial) and display the equation with R².

```python
import seraplot as sp
import random
random.seed(1)
xs = [i + random.gauss(0,0.6) for i in range(40)]
ys = [2.1*x + 4 + random.gauss(0,4.5) for x in xs]
chart = sp.scatter(
    title="Linear Trend",
    variant="regression",
    x_values=xs, y_values=ys,
    regression_type="linear",
    color_hex=0x6366F1, color_high=0xef4444,
    point_size=5.5,
    x_label="X", y_label="Y",
    gridlines=True,
)
chart.show()
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"regression"</code></span><span><strong>Aliases</strong> <code>regression / trendline / fit</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="scen-regression">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scen-regression','scen-regression-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scen-regression','scen-regression-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scen-regression','scen-regression-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scen-regression','scen-regression-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scen-regression','scen-regression-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('scen-regression','scen-regression-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scen-regression','scen-regression-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scen-regression','scen-regression-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scen-regression','scen-regression-cpp',this)">C++</button>
</div><div id="scen-regression-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(1)
xs = [i + random.gauss(0,0.6) for i in range(40)]
ys = [2.1*x + 4 + random.gauss(0,4.5) for x in xs]
chart = sp.scatter(
    title="Linear Trend",
    variant="regression",
    x_values=xs, y_values=ys,
    regression_type="linear",
    color_hex=0x6366F1, color_high=0xef4444,
    point_size=5.5,
    x_label="X", y_label="Y",
    gridlines=True,
)
chart.show()</code></pre></div><div id="scen-regression-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scen-regression-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scen-regression-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$scatter(
  title = "Scatter",
  variant = "basic",
  x_values = c(),
  y_values = c()
)
chart$show()</code></pre></div><div id="scen-regression-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::scatter()
        .title("Scatter")
        .variant("basic")
        .x_values(vec![])
        .y_values(vec![])
        .build();
    chart.show();
}</code></pre></div><div id="scen-regression-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Scatter")
    .variant("basic")
    .xValues(List.of())
    .yValues(List.of())
    .build();
chart.show();</code></pre></div><div id="scen-regression-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Scatter",
    variant: "basic",
    xValues: new double[]{},
    yValues: new double[]{}
);
chart.Show();</code></pre></div><div id="scen-regression-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.scatter(
  title    = "Scatter",
  variant  = "basic",
  x_values = List(),
  y_values = List()
)
chart.show()</code></pre></div><div id="scen-regression-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
    .title    = "Scatter",
    .variant  = "basic",
    .x_values = {},
    .y_values = {},
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-regression.html"></iframe>
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

`sp.scatter(title, x_values, y_values, *, variant="basic", categories=None, labels=None, color_values=None, **kwargs) -> Chart`

Alias : `sp.scatter`, `sp.scatters`, `sp.scatter_unified`, `sp.scatter_family`

## Description

`sp.scatter()` est le point d'entrée unifié de toute la famille scatter. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments gardent le même nom d'une variante à l'autre. Les nuages de points sont la façon canonique d'afficher la distribution conjointe de deux variables numériques ; SeraPlot ajoute groupement optionnel, couleur continue, formes de marqueurs distinctes, étiquettes sur les points et régression OLS — le tout en SVG Rust pur, des milliers de fois plus rapide que Plotly.

| Variante | Usage | Arguments clés |
|----------|-------|----------------|
| `"basic"` | Points simples, couleur unique | `x_values`, `y_values` |
| `"categorical"` / `"grouped"` | Couleur par groupe + légende latérale | `categories` |
| `"gradient"` / `"colorscale"` | Couleur continue + barre de couleur | `color_values`, `color_low`, `color_high` |
| `"symbols"` / `"shapes"` | Formes distinctes par groupe | `categories` |
| `"labeled"` / `"text"` | Étiquettes permanentes par point | `labels` |
| `"regression"` / `"trendline"` | Superposition d'un ajustement OLS | `regression_type` |

---

## Paramètres

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `x_values` | `list[float]` | requis | toutes | Coordonnée horizontale de chaque point |
| `y_values` | `list[float]` | requis | toutes | Coordonnée verticale de chaque point |
| `variant` | `str` | `"basic"` | — | Variante de rendu |
| `categories` | `list[str]` | `None` | categorical, symbols | Nom de groupe par point |
| `labels` | `list[str]` | `None` | toutes (hover) / labeled (rendu) | Étiquette texte par point |
| `color_values` | `list[float]` | `None` | gradient | Valeur numérique continue pour l'échelle de couleur |
| `color_hex` | `int` | `0x6366F1` | basic, regression | Couleur unique de remplissage |
| `color_low` | `int` | `0x6366F1` | gradient | Borne basse de l'échelle |
| `color_high` | `int` | `0xF43F5E` | gradient, regression | Borne haute / couleur de la régression |
| `point_size` | `float` | `5.0` | toutes | Rayon du marqueur (px) |
| `stroke_width` | `float` | `1.0` | toutes | Largeur du contour |
| `symbol` | `str` | `"circle"` | toutes | Forme par défaut (`circle`, `square`, `diamond`, `triangle`, `cross`, `plus`, `star`, `triangle-down`) |
| `symbols` | `list[str]` | `None` | symbols | Forme du marqueur point par point |
| `regression_type` | `str` | `"linear"` | regression | `"linear"` ou `"polynomial2"` |
| `palette` | `list[int]` | `None` | categorical, symbols | Palette personnalisée |
| `width` | `int` | `900` | toutes | Largeur (px) |
| `height` | `int` | `500` | toutes | Hauteur (px) |
| `x_label` | `str` | `""` | toutes | Légende axe X |
| `y_label` | `str` | `""` | toutes | Légende axe Y |
| `gridlines` | `bool` | `False` | toutes | Afficher la grille |
| `legend_position` | `str` | `"right"` | categorical, symbols | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | toutes | Couleur de fond CSS (None = transparent) |

---

## Retour

`Chart` — objet exposant `.html` et `.show()`.

---

<div class="sp-cls sp-open" id="scatter-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('scatter-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('scatter-fr','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">De base</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','categorical',this)"><span class="sp-cic">◓</span><span class="sp-clb">Catégoriel</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Dégradé</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','symbols',this)"><span class="sp-cic">◆</span><span class="sp-clb">Symboles</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','labeled',this)"><span class="sp-cic">◉</span><span class="sp-clb">Étiquetés</span></button>
<button class="sp-cls-tab" onclick="spCls('scatter-fr','regression',this)"><span class="sp-cic">↗</span><span class="sp-clb">Régression</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="scatter-fr-basic">

Marqueurs d'une seule couleur — le scatter le plus simple, idéal pour observer la corrélation brute entre deux variables numériques.

```python
import seraplot as sp
import random
random.seed(0)
xs = [random.gauss(0,1) for _ in range(120)]
ys = [x*0.6 + random.gauss(0,0.45) for x in xs]
chart = sp.scatter(
    title="Returns vs Volatility",
    variant="basic",
    x_values=xs, y_values=ys,
    color_hex=0x6366F1,
    point_size=5.5,
    x_label="Volatility", y_label="Return",
    gridlines=True,
)
chart.show()
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / simple / default</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="scfr-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scfr-basic','scfr-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scfr-basic','scfr-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scfr-basic','scfr-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scfr-basic','scfr-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scfr-basic','scfr-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('scfr-basic','scfr-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scfr-basic','scfr-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scfr-basic','scfr-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scfr-basic','scfr-basic-cpp',this)">C++</button>
</div><div id="scfr-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
xs = [random.gauss(0,1) for _ in range(120)]
ys = [x*0.6 + random.gauss(0,0.45) for x in xs]
chart = sp.scatter(
    title="Returns vs Volatility",
    variant="basic",
    x_values=xs, y_values=ys,
    color_hex=0x6366F1,
    point_size=5.5,
    x_label="Volatility", y_label="Return",
    gridlines=True,
)
chart.show()</code></pre></div><div id="scfr-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scfr-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scfr-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$scatter(
  title = "Scatter",
  variant = "basic",
  x_values = c(),
  y_values = c()
)
chart$show()</code></pre></div><div id="scfr-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::scatter()
        .title("Scatter")
        .variant("basic")
        .x_values(vec![])
        .y_values(vec![])
        .build();
    chart.show();
}</code></pre></div><div id="scfr-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Scatter")
    .variant("basic")
    .xValues(List.of())
    .yValues(List.of())
    .build();
chart.show();</code></pre></div><div id="scfr-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Scatter",
    variant: "basic",
    xValues: new double[]{},
    yValues: new double[]{}
);
chart.Show();</code></pre></div><div id="scfr-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.scatter(
  title    = "Scatter",
  variant  = "basic",
  x_values = List(),
  y_values = List()
)
chart.show()</code></pre></div><div id="scfr-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
    .title    = "Scatter",
    .variant  = "basic",
    .x_values = {},
    .y_values = {},
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-basic.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-categorical">

Couleur par groupe avec légende latérale. À utiliser pour comparer les distributions entre catégories sur les mêmes axes.

```python
import seraplot as sp
chart = sp.scatter(
    title="Iris — Sepal vs Petal",
    variant="categorical",
    x_values=[5.1,4.9,7.0,6.4,6.3,5.8],
    y_values=[1.4,1.4,4.7,4.5,6.0,5.1],
    categories=["setosa","setosa","versicolor","versicolor","virginica","virginica"],
    point_size=7.0,
    x_label="Sepal length", y_label="Petal length",
    gridlines=True,
)
chart.show()
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code></span><span><strong>Aliases</strong> <code>categorical / grouped / category</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="scfr-categorical">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scfr-categorical','scfr-categorical-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scfr-categorical','scfr-categorical-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scfr-categorical','scfr-categorical-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scfr-categorical','scfr-categorical-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scfr-categorical','scfr-categorical-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('scfr-categorical','scfr-categorical-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scfr-categorical','scfr-categorical-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scfr-categorical','scfr-categorical-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scfr-categorical','scfr-categorical-cpp',this)">C++</button>
</div><div id="scfr-categorical-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.scatter(
    title="Iris — Sepal vs Petal",
    variant="categorical",
    x_values=[5.1,4.9,7.0,6.4,6.3,5.8],
    y_values=[1.4,1.4,4.7,4.5,6.0,5.1],
    categories=["setosa","setosa","versicolor","versicolor","virginica","virginica"],
    point_size=7.0,
    x_label="Sepal length", y_label="Petal length",
    gridlines=True,
)
chart.show()</code></pre></div><div id="scfr-categorical-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scfr-categorical-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scfr-categorical-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$scatter(
  title = "Scatter",
  variant = "basic",
  x_values = c(),
  y_values = c()
)
chart$show()</code></pre></div><div id="scfr-categorical-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::scatter()
        .title("Scatter")
        .variant("basic")
        .x_values(vec![])
        .y_values(vec![])
        .build();
    chart.show();
}</code></pre></div><div id="scfr-categorical-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Scatter")
    .variant("basic")
    .xValues(List.of())
    .yValues(List.of())
    .build();
chart.show();</code></pre></div><div id="scfr-categorical-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Scatter",
    variant: "basic",
    xValues: new double[]{},
    yValues: new double[]{}
);
chart.Show();</code></pre></div><div id="scfr-categorical-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.scatter(
  title    = "Scatter",
  variant  = "basic",
  x_values = List(),
  y_values = List()
)
chart.show()</code></pre></div><div id="scfr-categorical-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
    .title    = "Scatter",
    .variant  = "basic",
    .x_values = {},
    .y_values = {},
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-categorical.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-gradient">

Couleur continue pilotée par une troisième valeur numérique, avec barre de couleur verticale.

```python
import seraplot as sp
chart = sp.scatter(
    title="Stars — Mass vs Luminosity",
    variant="gradient",
    x_values=[0.3,0.8,1.0,1.5,3.0,8.0,15.0],
    y_values=[0.01,0.4,1.0,5.0,80.0,3000.0,30000.0],
    color_values=[3000,4500,5800,6500,8500,15000,28000],
    color_low=0xef4444, color_high=0x3b82f6,
    point_size=8.0,
    x_label="Mass (M☉)", y_label="Luminosity (L☉)",
)
chart.show()
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / colorscale / continuous</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="scfr-gradient">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scfr-gradient','scfr-gradient-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scfr-gradient','scfr-gradient-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scfr-gradient','scfr-gradient-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scfr-gradient','scfr-gradient-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scfr-gradient','scfr-gradient-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('scfr-gradient','scfr-gradient-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scfr-gradient','scfr-gradient-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scfr-gradient','scfr-gradient-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scfr-gradient','scfr-gradient-cpp',this)">C++</button>
</div><div id="scfr-gradient-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.scatter(
    title="Stars — Mass vs Luminosity",
    variant="gradient",
    x_values=[0.3,0.8,1.0,1.5,3.0,8.0,15.0],
    y_values=[0.01,0.4,1.0,5.0,80.0,3000.0,30000.0],
    color_values=[3000,4500,5800,6500,8500,15000,28000],
    color_low=0xef4444, color_high=0x3b82f6,
    point_size=8.0,
    x_label="Mass (M☉)", y_label="Luminosity (L☉)",
)
chart.show()</code></pre></div><div id="scfr-gradient-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scfr-gradient-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scfr-gradient-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$scatter(
  title = "Scatter",
  variant = "basic",
  x_values = c(),
  y_values = c()
)
chart$show()</code></pre></div><div id="scfr-gradient-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::scatter()
        .title("Scatter")
        .variant("basic")
        .x_values(vec![])
        .y_values(vec![])
        .build();
    chart.show();
}</code></pre></div><div id="scfr-gradient-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Scatter")
    .variant("basic")
    .xValues(List.of())
    .yValues(List.of())
    .build();
chart.show();</code></pre></div><div id="scfr-gradient-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Scatter",
    variant: "basic",
    xValues: new double[]{},
    yValues: new double[]{}
);
chart.Show();</code></pre></div><div id="scfr-gradient-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.scatter(
  title    = "Scatter",
  variant  = "basic",
  x_values = List(),
  y_values = List()
)
chart.show()</code></pre></div><div id="scfr-gradient-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
    .title    = "Scatter",
    .variant  = "basic",
    .x_values = {},
    .y_values = {},
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-gradient.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-symbols">

Formes de marqueurs distinctes par catégorie en plus de la couleur — améliore la lisibilité pour les daltoniens.

```python
import seraplot as sp
chart = sp.scatter(
    title="Penguins — Bill vs Flipper",
    variant="symbols",
    x_values=[39.1,46.5,50.0,49.5,38.9,47.5],
    y_values=[181,217,222,209,184,219],
    categories=["Adelie","Gentoo","Chinstrap","Chinstrap","Adelie","Gentoo"],
    point_size=8.0,
    x_label="Bill length (mm)", y_label="Flipper length (mm)",
    gridlines=True,
)
chart.show()
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"symbols"</code></span><span><strong>Aliases</strong> <code>symbols / shapes / markers</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="scfr-symbols">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scfr-symbols','scfr-symbols-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scfr-symbols','scfr-symbols-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scfr-symbols','scfr-symbols-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scfr-symbols','scfr-symbols-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scfr-symbols','scfr-symbols-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('scfr-symbols','scfr-symbols-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scfr-symbols','scfr-symbols-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scfr-symbols','scfr-symbols-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scfr-symbols','scfr-symbols-cpp',this)">C++</button>
</div><div id="scfr-symbols-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.scatter(
    title="Penguins — Bill vs Flipper",
    variant="symbols",
    x_values=[39.1,46.5,50.0,49.5,38.9,47.5],
    y_values=[181,217,222,209,184,219],
    categories=["Adelie","Gentoo","Chinstrap","Chinstrap","Adelie","Gentoo"],
    point_size=8.0,
    x_label="Bill length (mm)", y_label="Flipper length (mm)",
    gridlines=True,
)
chart.show()</code></pre></div><div id="scfr-symbols-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scfr-symbols-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scfr-symbols-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$scatter(
  title = "Scatter",
  variant = "basic",
  x_values = c(),
  y_values = c()
)
chart$show()</code></pre></div><div id="scfr-symbols-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::scatter()
        .title("Scatter")
        .variant("basic")
        .x_values(vec![])
        .y_values(vec![])
        .build();
    chart.show();
}</code></pre></div><div id="scfr-symbols-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Scatter")
    .variant("basic")
    .xValues(List.of())
    .yValues(List.of())
    .build();
chart.show();</code></pre></div><div id="scfr-symbols-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Scatter",
    variant: "basic",
    xValues: new double[]{},
    yValues: new double[]{}
);
chart.Show();</code></pre></div><div id="scfr-symbols-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.scatter(
  title    = "Scatter",
  variant  = "basic",
  x_values = List(),
  y_values = List()
)
chart.show()</code></pre></div><div id="scfr-symbols-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
    .title    = "Scatter",
    .variant  = "basic",
    .x_values = {},
    .y_values = {},
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-symbols.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-labeled">

Étiquettes textuelles permanentes au-dessus de chaque marqueur, avec halo pour rester lisible.

```python
import seraplot as sp
chart = sp.scatter(
    title="French Cities — Cost vs Quality",
    variant="labeled",
    x_values=[2.1,1.8,1.5,1.3,1.1,1.6,1.9,1.4,1.2,2.4],
    y_values=[8.2,7.8,7.4,7.0,6.5,7.6,7.9,7.2,6.8,8.5],
    labels=["Paris","Lyon","Marseille","Toulouse","Nice","Nantes","Bordeaux","Lille","Rennes","Strasbourg"],
    point_size=7.5,
    color_hex=0x10b981,
    x_label="Cost index", y_label="Quality of life",
    gridlines=True,
)
chart.show()
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"labeled"</code></span><span><strong>Aliases</strong> <code>labeled / labels / text</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="scfr-labeled">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scfr-labeled','scfr-labeled-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scfr-labeled','scfr-labeled-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scfr-labeled','scfr-labeled-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scfr-labeled','scfr-labeled-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scfr-labeled','scfr-labeled-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('scfr-labeled','scfr-labeled-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scfr-labeled','scfr-labeled-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scfr-labeled','scfr-labeled-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scfr-labeled','scfr-labeled-cpp',this)">C++</button>
</div><div id="scfr-labeled-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.scatter(
    title="French Cities — Cost vs Quality",
    variant="labeled",
    x_values=[2.1,1.8,1.5,1.3,1.1,1.6,1.9,1.4,1.2,2.4],
    y_values=[8.2,7.8,7.4,7.0,6.5,7.6,7.9,7.2,6.8,8.5],
    labels=["Paris","Lyon","Marseille","Toulouse","Nice","Nantes","Bordeaux","Lille","Rennes","Strasbourg"],
    point_size=7.5,
    color_hex=0x10b981,
    x_label="Cost index", y_label="Quality of life",
    gridlines=True,
)
chart.show()</code></pre></div><div id="scfr-labeled-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scfr-labeled-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scfr-labeled-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$scatter(
  title = "Scatter",
  variant = "basic",
  x_values = c(),
  y_values = c()
)
chart$show()</code></pre></div><div id="scfr-labeled-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::scatter()
        .title("Scatter")
        .variant("basic")
        .x_values(vec![])
        .y_values(vec![])
        .build();
    chart.show();
}</code></pre></div><div id="scfr-labeled-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Scatter")
    .variant("basic")
    .xValues(List.of())
    .yValues(List.of())
    .build();
chart.show();</code></pre></div><div id="scfr-labeled-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Scatter",
    variant: "basic",
    xValues: new double[]{},
    yValues: new double[]{}
);
chart.Show();</code></pre></div><div id="scfr-labeled-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.scatter(
  title    = "Scatter",
  variant  = "basic",
  x_values = List(),
  y_values = List()
)
chart.show()</code></pre></div><div id="scfr-labeled-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
    .title    = "Scatter",
    .variant  = "basic",
    .x_values = {},
    .y_values = {},
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-labeled.html"></iframe>
</div>

<div class="sp-variant" id="scatter-fr-regression">

Superpose un ajustement par moindres carrés (linéaire ou polynomial) et affiche l'équation avec R².

```python
import seraplot as sp
import random
random.seed(1)
xs = [i + random.gauss(0,0.6) for i in range(40)]
ys = [2.1*x + 4 + random.gauss(0,4.5) for x in xs]
chart = sp.scatter(
    title="Linear Trend",
    variant="regression",
    x_values=xs, y_values=ys,
    regression_type="linear",
    color_hex=0x6366F1, color_high=0xef4444,
    point_size=5.5,
    x_label="X", y_label="Y",
    gridlines=True,
)
chart.show()
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"regression"</code></span><span><strong>Aliases</strong> <code>regression / trendline / fit</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="scfr-regression">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scfr-regression','scfr-regression-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scfr-regression','scfr-regression-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scfr-regression','scfr-regression-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scfr-regression','scfr-regression-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scfr-regression','scfr-regression-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('scfr-regression','scfr-regression-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scfr-regression','scfr-regression-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scfr-regression','scfr-regression-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scfr-regression','scfr-regression-cpp',this)">C++</button>
</div><div id="scfr-regression-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(1)
xs = [i + random.gauss(0,0.6) for i in range(40)]
ys = [2.1*x + 4 + random.gauss(0,4.5) for x in xs]
chart = sp.scatter(
    title="Linear Trend",
    variant="regression",
    x_values=xs, y_values=ys,
    regression_type="linear",
    color_hex=0x6366F1, color_high=0xef4444,
    point_size=5.5,
    x_label="X", y_label="Y",
    gridlines=True,
)
chart.show()</code></pre></div><div id="scfr-regression-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scfr-regression-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.scatter({
  title: "Scatter",
  variant: "basic",
  xValues: [/* numbers */],
  yValues: [/* numbers */],
});
chart.show();</code></pre></div><div id="scfr-regression-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$scatter(
  title = "Scatter",
  variant = "basic",
  x_values = c(),
  y_values = c()
)
chart$show()</code></pre></div><div id="scfr-regression-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::scatter()
        .title("Scatter")
        .variant("basic")
        .x_values(vec![])
        .y_values(vec![])
        .build();
    chart.show();
}</code></pre></div><div id="scfr-regression-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Scatter")
    .variant("basic")
    .xValues(List.of())
    .yValues(List.of())
    .build();
chart.show();</code></pre></div><div id="scfr-regression-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Scatter",
    variant: "basic",
    xValues: new double[]{},
    yValues: new double[]{}
);
chart.Show();</code></pre></div><div id="scfr-regression-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.scatter(
  title    = "Scatter",
  variant  = "basic",
  x_values = List(),
  y_values = List()
)
chart.show()</code></pre></div><div id="scfr-regression-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
    .title    = "Scatter",
    .variant  = "basic",
    .x_values = {},
    .y_values = {},
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatter-regression.html"></iframe>
</div>

</div>
</div>

</div>
