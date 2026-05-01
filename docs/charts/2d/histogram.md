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

<div class="sp-tabs" id="hgen-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgen-basic','hgen-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgen-basic','hgen-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgen-basic','hgen-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgen-basic','hgen-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgen-basic','hgen-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgen-basic','hgen-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgen-basic','hgen-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgen-basic','hgen-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgen-basic','hgen-basic-cpp',this)">C++</button>
</div><div id="hgen-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
v = [random.gauss(70, 10) for _ in range(500)]
chart = sp.histogram(
    title="Distribution",
    values=v,
    variant="basic",
    bins=0,
    color_hex=0x6366F1,
    x_label="value", y_label="count",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgen-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgen-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgen-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgen-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgen-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgen-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-basic.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-horizontal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / h / barh / hbar</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hgen-horizontal">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgen-horizontal','hgen-horizontal-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgen-horizontal','hgen-horizontal-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgen-horizontal','hgen-horizontal-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgen-horizontal','hgen-horizontal-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgen-horizontal','hgen-horizontal-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgen-horizontal','hgen-horizontal-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgen-horizontal','hgen-horizontal-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgen-horizontal','hgen-horizontal-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgen-horizontal','hgen-horizontal-cpp',this)">C++</button>
</div><div id="hgen-horizontal-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
v = [random.gauss(70, 10) for _ in range(500)]
chart = sp.histogram(
    title="Distribution (Horizontal)",
    values=v,
    variant="horizontal",
    color_hex=0x6366F1,
    x_label="count", y_label="value",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgen-horizontal-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-horizontal-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-horizontal-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgen-horizontal-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgen-horizontal-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgen-horizontal-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgen-horizontal-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgen-horizontal-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-horizontal.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-normalized">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"normalized"</code></span><span><strong>Aliases</strong> <code>normalized / probability / density / pdf</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hgen-normalized">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgen-normalized','hgen-normalized-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgen-normalized','hgen-normalized-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgen-normalized','hgen-normalized-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgen-normalized','hgen-normalized-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgen-normalized','hgen-normalized-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgen-normalized','hgen-normalized-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgen-normalized','hgen-normalized-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgen-normalized','hgen-normalized-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgen-normalized','hgen-normalized-cpp',this)">C++</button>
</div><div id="hgen-normalized-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
v = [random.gauss(70, 10) for _ in range(500)]
chart = sp.histogram(
    title="Probability Density",
    values=v,
    variant="normalized",
    color_hex=0x10b981,
    x_label="value", y_label="density",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgen-normalized-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-normalized-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-normalized-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgen-normalized-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgen-normalized-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgen-normalized-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgen-normalized-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgen-normalized-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-normalized.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-cumulative">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cumulative"</code></span><span><strong>Aliases</strong> <code>cumulative / cdf / cum</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hgen-cumulative">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgen-cumulative','hgen-cumulative-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgen-cumulative','hgen-cumulative-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgen-cumulative','hgen-cumulative-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgen-cumulative','hgen-cumulative-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgen-cumulative','hgen-cumulative-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgen-cumulative','hgen-cumulative-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgen-cumulative','hgen-cumulative-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgen-cumulative','hgen-cumulative-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgen-cumulative','hgen-cumulative-cpp',this)">C++</button>
</div><div id="hgen-cumulative-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
v = [random.gauss(70, 10) for _ in range(500)]
chart = sp.histogram(
    title="Cumulative Distribution",
    values=v,
    variant="cumulative",
    color_hex=0x6366F1,
    x_label="value", y_label="cumulative probability",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgen-cumulative-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-cumulative-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-cumulative-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgen-cumulative-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgen-cumulative-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgen-cumulative-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgen-cumulative-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgen-cumulative-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-cumulative.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-stacked">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stack / stack_by</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hgen-stacked">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgen-stacked','hgen-stacked-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgen-stacked','hgen-stacked-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgen-stacked','hgen-stacked-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgen-stacked','hgen-stacked-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgen-stacked','hgen-stacked-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgen-stacked','hgen-stacked-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgen-stacked','hgen-stacked-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgen-stacked','hgen-stacked-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgen-stacked','hgen-stacked-cpp',this)">C++</button>
</div><div id="hgen-stacked-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
v = [random.gauss(70, 10) for _ in range(500)]
days = ["Mon", "Tue", "Wed", "Thu", "Fri"]
groups = [days[int((x - 40) / 12) % len(days)] for x in v]
chart = sp.histogram(
    title="Stacked by Day",
    values=v,
    variant="stacked",
    color_groups=groups,
    x_label="value", y_label="count",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgen-stacked-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-stacked-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-stacked-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgen-stacked-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgen-stacked-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgen-stacked-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgen-stacked-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgen-stacked-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-stacked.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-overlay">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"overlay"</code></span><span><strong>Aliases</strong> <code>overlay / overlapping / compare / ab</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hgen-overlay">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgen-overlay','hgen-overlay-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgen-overlay','hgen-overlay-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgen-overlay','hgen-overlay-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgen-overlay','hgen-overlay-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgen-overlay','hgen-overlay-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgen-overlay','hgen-overlay-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgen-overlay','hgen-overlay-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgen-overlay','hgen-overlay-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgen-overlay','hgen-overlay-cpp',this)">C++</button>
</div><div id="hgen-overlay-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
control = [random.gauss(70, 10) for _ in range(500)]
variant_ = [random.gauss(78, 9) for _ in range(500)]
chart = sp.histogram(
    title="A/B Test",
    values=control,
    overlay_values=variant_,
    variant="overlay",
    series_names=["Control", "Variant"],
    color_hex=0x6366F1, overlay_color_hex=0xF43F5E,
    x_label="metric", y_label="count",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgen-overlay-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-overlay-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-overlay-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgen-overlay-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgen-overlay-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgen-overlay-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgen-overlay-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgen-overlay-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-overlay.html"></iframe>
</div>

<div class="sp-variant" id="histogram-en-step">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"step"</code></span><span><strong>Aliases</strong> <code>step / outline / stair</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hgen-step">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgen-step','hgen-step-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgen-step','hgen-step-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgen-step','hgen-step-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgen-step','hgen-step-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgen-step','hgen-step-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgen-step','hgen-step-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgen-step','hgen-step-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgen-step','hgen-step-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgen-step','hgen-step-cpp',this)">C++</button>
</div><div id="hgen-step-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
v = [random.gauss(70, 10) for _ in range(500)]
chart = sp.histogram(
    title="Step Outline",
    values=v,
    variant="step",
    color_hex=0x6366F1,
    stroke_width=2.0,
    x_label="value", y_label="count",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgen-step-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-step-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgen-step-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgen-step-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgen-step-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgen-step-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgen-step-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgen-step-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
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


## Signature

`sp.histogram(title, values, *, variant="basic", bins=0, overlay_values=None, color_groups=None, series_names=None, **kwargs) -> Chart`

Alias : `sp.histogram`, `sp.histograms`, `sp.histogram_unified`, `sp.histogram_family`

## Description

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

## Paramètres

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

## Retour

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

<div class="sp-tabs" id="hgfr-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgfr-basic','hgfr-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgfr-basic','hgfr-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgfr-basic','hgfr-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgfr-basic','hgfr-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgfr-basic','hgfr-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgfr-basic','hgfr-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgfr-basic','hgfr-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgfr-basic','hgfr-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgfr-basic','hgfr-basic-cpp',this)">C++</button>
</div><div id="hgfr-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
v = [random.gauss(70, 10) for _ in range(500)]
chart = sp.histogram(
    title="Distribution",
    values=v,
    variant="basic",
    bins=0,
    color_hex=0x6366F1,
    x_label="value", y_label="count",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgfr-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgfr-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgfr-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgfr-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgfr-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgfr-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-basic.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-horizontal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / h / barh / hbar</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hgfr-horizontal">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgfr-horizontal','hgfr-horizontal-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgfr-horizontal','hgfr-horizontal-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgfr-horizontal','hgfr-horizontal-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgfr-horizontal','hgfr-horizontal-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgfr-horizontal','hgfr-horizontal-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgfr-horizontal','hgfr-horizontal-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgfr-horizontal','hgfr-horizontal-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgfr-horizontal','hgfr-horizontal-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgfr-horizontal','hgfr-horizontal-cpp',this)">C++</button>
</div><div id="hgfr-horizontal-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
v = [random.gauss(70, 10) for _ in range(500)]
chart = sp.histogram(
    title="Distribution (Horizontal)",
    values=v,
    variant="horizontal",
    color_hex=0x6366F1,
    x_label="count", y_label="value",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgfr-horizontal-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-horizontal-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-horizontal-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgfr-horizontal-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgfr-horizontal-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgfr-horizontal-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgfr-horizontal-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgfr-horizontal-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-horizontal.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-normalized">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"normalized"</code></span><span><strong>Aliases</strong> <code>normalized / probability / density / pdf</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hgfr-normalized">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgfr-normalized','hgfr-normalized-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgfr-normalized','hgfr-normalized-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgfr-normalized','hgfr-normalized-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgfr-normalized','hgfr-normalized-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgfr-normalized','hgfr-normalized-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgfr-normalized','hgfr-normalized-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgfr-normalized','hgfr-normalized-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgfr-normalized','hgfr-normalized-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgfr-normalized','hgfr-normalized-cpp',this)">C++</button>
</div><div id="hgfr-normalized-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
v = [random.gauss(70, 10) for _ in range(500)]
chart = sp.histogram(
    title="Probability Density",
    values=v,
    variant="normalized",
    color_hex=0x10b981,
    x_label="value", y_label="density",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgfr-normalized-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-normalized-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-normalized-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgfr-normalized-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgfr-normalized-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgfr-normalized-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgfr-normalized-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgfr-normalized-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-normalized.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-cumulative">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cumulative"</code></span><span><strong>Aliases</strong> <code>cumulative / cdf / cum</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hgfr-cumulative">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgfr-cumulative','hgfr-cumulative-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgfr-cumulative','hgfr-cumulative-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgfr-cumulative','hgfr-cumulative-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgfr-cumulative','hgfr-cumulative-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgfr-cumulative','hgfr-cumulative-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgfr-cumulative','hgfr-cumulative-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgfr-cumulative','hgfr-cumulative-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgfr-cumulative','hgfr-cumulative-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgfr-cumulative','hgfr-cumulative-cpp',this)">C++</button>
</div><div id="hgfr-cumulative-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
v = [random.gauss(70, 10) for _ in range(500)]
chart = sp.histogram(
    title="Cumulative Distribution",
    values=v,
    variant="cumulative",
    color_hex=0x6366F1,
    x_label="value", y_label="cumulative probability",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgfr-cumulative-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-cumulative-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-cumulative-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgfr-cumulative-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgfr-cumulative-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgfr-cumulative-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgfr-cumulative-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgfr-cumulative-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-cumulative.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-stacked">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stack / stack_by</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hgfr-stacked">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgfr-stacked','hgfr-stacked-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgfr-stacked','hgfr-stacked-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgfr-stacked','hgfr-stacked-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgfr-stacked','hgfr-stacked-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgfr-stacked','hgfr-stacked-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgfr-stacked','hgfr-stacked-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgfr-stacked','hgfr-stacked-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgfr-stacked','hgfr-stacked-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgfr-stacked','hgfr-stacked-cpp',this)">C++</button>
</div><div id="hgfr-stacked-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
v = [random.gauss(70, 10) for _ in range(500)]
days = ["Mon", "Tue", "Wed", "Thu", "Fri"]
groups = [days[int((x - 40) / 12) % len(days)] for x in v]
chart = sp.histogram(
    title="Stacked by Day",
    values=v,
    variant="stacked",
    color_groups=groups,
    x_label="value", y_label="count",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgfr-stacked-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-stacked-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-stacked-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgfr-stacked-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgfr-stacked-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgfr-stacked-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgfr-stacked-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgfr-stacked-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-stacked.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-overlay">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"overlay"</code></span><span><strong>Aliases</strong> <code>overlay / overlapping / compare / ab</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hgfr-overlay">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgfr-overlay','hgfr-overlay-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgfr-overlay','hgfr-overlay-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgfr-overlay','hgfr-overlay-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgfr-overlay','hgfr-overlay-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgfr-overlay','hgfr-overlay-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgfr-overlay','hgfr-overlay-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgfr-overlay','hgfr-overlay-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgfr-overlay','hgfr-overlay-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgfr-overlay','hgfr-overlay-cpp',this)">C++</button>
</div><div id="hgfr-overlay-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
control = [random.gauss(70, 10) for _ in range(500)]
variant_ = [random.gauss(78, 9) for _ in range(500)]
chart = sp.histogram(
    title="A/B Test",
    values=control,
    overlay_values=variant_,
    variant="overlay",
    series_names=["Control", "Variant"],
    color_hex=0x6366F1, overlay_color_hex=0xF43F5E,
    x_label="metric", y_label="count",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgfr-overlay-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-overlay-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-overlay-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgfr-overlay-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgfr-overlay-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgfr-overlay-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgfr-overlay-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgfr-overlay-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-overlay.html"></iframe>
</div>

<div class="sp-variant" id="histogram-fr-step">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"step"</code></span><span><strong>Aliases</strong> <code>step / outline / stair</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hgfr-step">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hgfr-step','hgfr-step-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hgfr-step','hgfr-step-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hgfr-step','hgfr-step-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hgfr-step','hgfr-step-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hgfr-step','hgfr-step-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hgfr-step','hgfr-step-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hgfr-step','hgfr-step-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hgfr-step','hgfr-step-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hgfr-step','hgfr-step-cpp',this)">C++</button>
</div><div id="hgfr-step-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
v = [random.gauss(70, 10) for _ in range(500)]
chart = sp.histogram(
    title="Step Outline",
    values=v,
    variant="step",
    color_hex=0x6366F1,
    stroke_width=2.0,
    x_label="value", y_label="count",
    gridlines=True,
)
chart.show()</code></pre></div><div id="hgfr-step-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-step-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogram({
  title: "Distribution",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hgfr-step-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$histogram(
  title = "Distribution",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hgfr-step-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::histogram()
        .title("Distribution")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hgfr-step-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.histogram()
    .title("Distribution")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hgfr-step-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Histogram(
    title: "Distribution",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hgfr-step-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.histogram(
  title   = "Distribution",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="hgfr-step-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::histogram({
    .title   = "Distribution",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/histogram-step.html"></iframe>
</div>

</div>
</div>

</div>
