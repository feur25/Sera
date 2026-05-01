# Heatmap Charts

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

`sp.heatmap(title, labels, values, *, variant="basic", col_labels=None, palette=None, bins=0, widths=None, ranges=None, **kwargs) -> Chart`

Aliases: `sp.heatmap`, `sp.heatmaps`, `sp.heatmap_unified`, `sp.heatmap_family`

## Description

`sp.heatmap()` is the unified entry point for the entire heatmap family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. Heatmaps are the canonical way to visualize a numeric matrix; SeraPlot adds annotated cells, categorical coloring, irregular cell sizes, logarithmic scaling, discrete bands and a diverging correlation palette — all in pure Rust SVG, thousands of times faster than Plotly.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Continuous 3-stop gradient | `color_low`, `color_mid`, `color_high` |
| `"annotated"` | Force value labels in every cell | (no extra) |
| `"categorical"` | Each discrete value gets a palette color | `palette` |
| `"unequal"` | Cells with custom widths/heights | `widths`, `ranges` |
| `"log"` | Logarithmic color mapping | (no extra) |
| `"discrete"` | Quantize values into N color bands | `bins`, `palette` |
| `"correlation"` | Diverging palette centered at 0 | (no extra) |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | required | all | Chart title |
| `labels` | `list[str]` | required | all | Row labels |
| `values` | `list[float]` | required | all | Row-major flattened matrix (length = n_rows * n_cols) |
| `col_labels` | `list[str]` | `None` | all | Column labels (defaults to row labels) |
| `variant` | `str` | `"basic"` | — | Rendering variant |
| `show_values` | `bool` | `True` | all | Overlay numeric values when cell is large enough |
| `color_low` | `int` | `0x6366F1` | basic, annotated, log, discrete | Gradient low-stop color |
| `color_mid` | `int` | `0xfafbfc` | basic, annotated, log, discrete | Gradient mid-stop color |
| `color_high` | `int` | `0xF43F5E` | basic, annotated, log, discrete | Gradient high-stop color |
| `palette` | `list[int]` | `None` | categorical, discrete | Per-band / per-category colors |
| `bins` | `int` | `0` | discrete | Number of color bands |
| `widths` | `list[float]` | `None` | unequal | Relative column widths |
| `ranges` | `list[float]` | `None` | unequal | Relative row heights |
| `width` | `int` | `720` | all | Canvas width (px) |
| `height` | `int` | `440` | all | Canvas height (px) |
| `gridlines` | `bool` | `False` | all | Show gridlines |
| `background` | `str` | `None` | all | Background CSS color |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="heatmap-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('heatmap-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('heatmap-en','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('heatmap-en','annotated',this)"><span class="sp-cic">A</span><span class="sp-clb">Annotated</span></button>
<button class="sp-cls-tab" onclick="spCls('heatmap-en','categorical',this)"><span class="sp-cic">C</span><span class="sp-clb">Categorical</span></button>
<button class="sp-cls-tab" onclick="spCls('heatmap-en','unequal',this)"><span class="sp-cic">U</span><span class="sp-clb">Unequal</span></button>
<button class="sp-cls-tab" onclick="spCls('heatmap-en','log',this)"><span class="sp-cic">L</span><span class="sp-clb">Log scale</span></button>
<button class="sp-cls-tab" onclick="spCls('heatmap-en','discrete',this)"><span class="sp-cic">D</span><span class="sp-clb">Discrete</span></button>
<button class="sp-cls-tab" onclick="spCls('heatmap-en','correlation',this)"><span class="sp-cic">R</span><span class="sp-clb">Correlation</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="heatmap-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / simple / default / matrix</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmen-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmen-basic','hmen-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmen-basic','hmen-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmen-basic','hmen-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmen-basic','hmen-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmen-basic','hmen-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmen-basic','hmen-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmen-basic','hmen-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmen-basic','hmen-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmen-basic','hmen-basic-cpp',this)">C++</button>
</div><div id="hmen-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
rows = ["Alpha","Beta","Gamma","Delta"]
cols = ["Jan","Feb","Mar","Apr","May","Jun"]
m = [v for r in range(4) for v in [10,40,75,30,55,20]]
chart = sp.heatmap(
    title="Sales by Month",
    labels=rows, col_labels=cols, values=m,
    variant="basic",
    color_low=0x6366F1, color_mid=0xfafbfc, color_high=0xF43F5E,
)
chart.show()</code></pre></div><div id="hmen-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmen-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmen-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmen-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmen-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmen-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-basic.html"></iframe>
</div>

<div class="sp-variant" id="heatmap-en-annotated">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"annotated"</code></span><span><strong>Aliases</strong> <code>annotated / annotate / labeled / values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmen-annotated">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmen-annotated','hmen-annotated-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmen-annotated','hmen-annotated-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmen-annotated','hmen-annotated-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmen-annotated','hmen-annotated-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmen-annotated','hmen-annotated-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmen-annotated','hmen-annotated-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmen-annotated','hmen-annotated-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmen-annotated','hmen-annotated-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmen-annotated','hmen-annotated-cpp',this)">C++</button>
</div><div id="hmen-annotated-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
rows = ["Alpha","Beta","Gamma","Delta"]
cols = ["Jan","Feb","Mar","Apr","May","Jun"]
m = [v for r in range(4) for v in [10,40,75,30,55,20]]
chart = sp.heatmap(
    title="Sales (Annotated)",
    labels=rows, col_labels=cols, values=m,
    variant="annotated",
)
chart.show()</code></pre></div><div id="hmen-annotated-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-annotated-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-annotated-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmen-annotated-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmen-annotated-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmen-annotated-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmen-annotated-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmen-annotated-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-annotated.html"></iframe>
</div>

<div class="sp-variant" id="heatmap-en-categorical">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code></span><span><strong>Aliases</strong> <code>categorical / category / cat</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmen-categorical">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmen-categorical','hmen-categorical-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmen-categorical','hmen-categorical-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmen-categorical','hmen-categorical-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmen-categorical','hmen-categorical-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmen-categorical','hmen-categorical-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmen-categorical','hmen-categorical-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmen-categorical','hmen-categorical-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmen-categorical','hmen-categorical-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmen-categorical','hmen-categorical-cpp',this)">C++</button>
</div><div id="hmen-categorical-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
rows = ["North","South","East","West"]
cols = ["A","B","C","D","E"]
m = [0,1,2,1,0,  3,2,1,4,2,  1,1,0,3,2,  4,3,2,1,0]
chart = sp.heatmap(
    title="Region x Cluster",
    labels=rows, col_labels=cols, values=m,
    variant="categorical",
    palette=[0x6366F1,0xF43F5E,0x10B981,0xF59E0B,0x8B5CF6],
)
chart.show()</code></pre></div><div id="hmen-categorical-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-categorical-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-categorical-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmen-categorical-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmen-categorical-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmen-categorical-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmen-categorical-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmen-categorical-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-categorical.html"></iframe>
</div>

<div class="sp-variant" id="heatmap-en-unequal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"unequal"</code></span><span><strong>Aliases</strong> <code>unequal / irregular / weighted</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmen-unequal">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmen-unequal','hmen-unequal-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmen-unequal','hmen-unequal-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmen-unequal','hmen-unequal-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmen-unequal','hmen-unequal-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmen-unequal','hmen-unequal-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmen-unequal','hmen-unequal-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmen-unequal','hmen-unequal-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmen-unequal','hmen-unequal-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmen-unequal','hmen-unequal-cpp',this)">C++</button>
</div><div id="hmen-unequal-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
rows = ["Premium","Standard","Basic"]
cols = ["Q1","Q2","Q3","Q4"]
m = [80,55,40,30,  60,50,35,25,  40,30,20,15]
chart = sp.heatmap(
    title="Tier x Quarter (Unequal cells)",
    labels=rows, col_labels=cols, values=m,
    variant="unequal",
    widths=[1.0, 2.5, 1.6, 0.9],
    ranges=[2.2, 1.0, 1.5],
)
chart.show()</code></pre></div><div id="hmen-unequal-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-unequal-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-unequal-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmen-unequal-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmen-unequal-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmen-unequal-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmen-unequal-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmen-unequal-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-unequal.html"></iframe>
</div>

<div class="sp-variant" id="heatmap-en-log">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"log"</code></span><span><strong>Aliases</strong> <code>log / logarithmic / log10</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmen-log">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmen-log','hmen-log-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmen-log','hmen-log-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmen-log','hmen-log-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmen-log','hmen-log-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmen-log','hmen-log-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmen-log','hmen-log-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmen-log','hmen-log-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmen-log','hmen-log-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmen-log','hmen-log-cpp',this)">C++</button>
</div><div id="hmen-log-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
rows = ["Network","CPU","Memory","Disk"]
cols = ["t1","t2","t3","t4","t5","t6"]
m = [10,100,1000,5000,80,2,  3,30,800,9000,500,8,  5,200,1500,3000,40,1,  20,300,2200,7000,90,4]
chart = sp.heatmap(
    title="Resource Load (Log)",
    labels=rows, col_labels=cols, values=m,
    variant="log",
)
chart.show()</code></pre></div><div id="hmen-log-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-log-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-log-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmen-log-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmen-log-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmen-log-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmen-log-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmen-log-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-log.html"></iframe>
</div>

<div class="sp-variant" id="heatmap-en-discrete">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"discrete"</code></span><span><strong>Aliases</strong> <code>discrete / binned / stepped / bands</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmen-discrete">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmen-discrete','hmen-discrete-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmen-discrete','hmen-discrete-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmen-discrete','hmen-discrete-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmen-discrete','hmen-discrete-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmen-discrete','hmen-discrete-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmen-discrete','hmen-discrete-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmen-discrete','hmen-discrete-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmen-discrete','hmen-discrete-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmen-discrete','hmen-discrete-cpp',this)">C++</button>
</div><div id="hmen-discrete-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
rows = ["Alpha","Beta","Gamma","Delta"]
cols = ["Jan","Feb","Mar","Apr","May","Jun"]
m = [v for r in range(4) for v in [10,40,75,30,55,20]]
chart = sp.heatmap(
    title="Sales tiers (Discrete)",
    labels=rows, col_labels=cols, values=m,
    variant="discrete",
    bins=6,
    palette=[0x1E3A8A,0x3B82F6,0x60A5FA,0xFBBF24,0xF97316,0xDC2626],
)
chart.show()</code></pre></div><div id="hmen-discrete-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-discrete-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-discrete-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmen-discrete-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmen-discrete-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmen-discrete-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmen-discrete-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmen-discrete-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-discrete.html"></iframe>
</div>

<div class="sp-variant" id="heatmap-en-correlation">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"correlation"</code></span><span><strong>Aliases</strong> <code>correlation / corr / diverging / pearson</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmen-correlation">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmen-correlation','hmen-correlation-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmen-correlation','hmen-correlation-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmen-correlation','hmen-correlation-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmen-correlation','hmen-correlation-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmen-correlation','hmen-correlation-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmen-correlation','hmen-correlation-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmen-correlation','hmen-correlation-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmen-correlation','hmen-correlation-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmen-correlation','hmen-correlation-cpp',this)">C++</button>
</div><div id="hmen-correlation-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
features = ["Sales","Ads","Visits","Signups","Revenue"]
m = [
     1.00, 0.82, 0.55,-0.12, 0.91,
     0.82, 1.00, 0.43,-0.05, 0.78,
     0.55, 0.43, 1.00, 0.62, 0.49,
    -0.12,-0.05, 0.62, 1.00,-0.08,
     0.91, 0.78, 0.49,-0.08, 1.00,
]
chart = sp.heatmap(
    title="Feature Correlation",
    labels=features, col_labels=features, values=m,
    variant="correlation",
)
chart.show()</code></pre></div><div id="hmen-correlation-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-correlation-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmen-correlation-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmen-correlation-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmen-correlation-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmen-correlation-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmen-correlation-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmen-correlation-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-correlation.html"></iframe>
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

`sp.heatmap(title, labels, values, *, variant="basic", col_labels=None, palette=None, bins=0, widths=None, ranges=None, **kwargs) -> Chart`

Alias : `sp.heatmap`, `sp.heatmaps`, `sp.heatmap_unified`, `sp.heatmap_family`

## Description

`sp.heatmap()` est le point d'entrée unifié de toute la famille heatmap. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments gardent le même nom d'une variante à l'autre. Les heatmaps sont la façon canonique de visualiser une matrice numérique ; SeraPlot ajoute des cellules annotées, coloration catégorielle, tailles irrégulières, échelle logarithmique, bandes discrètes et une palette divergente pour la corrélation — le tout en SVG Rust pur, des milliers de fois plus rapide que Plotly.

| Variante | Usage | Arguments clés |
|----------|-------|------------------|
| `"basic"` | Dégradé continu à 3 étapes | `color_low`, `color_mid`, `color_high` |
| `"annotated"` | Affiche la valeur dans chaque cellule | (rien de plus) |
| `"categorical"` | Une couleur de palette par valeur discrète | `palette` |
| `"unequal"` | Cellules de tailles personnalisées | `widths`, `ranges` |
| `"log"` | Mappage de couleur logarithmique | (rien de plus) |
| `"discrete"` | Quantifie les valeurs en N bandes | `bins`, `palette` |
| `"correlation"` | Palette divergente centrée en 0 | (rien de plus) |

---

## Paramètres

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|---------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `labels` | `list[str]` | requis | toutes | Étiquettes des lignes |
| `values` | `list[float]` | requis | toutes | Matrice aplatie row-major (n_rows × n_cols) |
| `col_labels` | `list[str]` | `None` | toutes | Étiquettes des colonnes |
| `variant` | `str` | `"basic"` | — | Variante de rendu |
| `show_values` | `bool` | `True` | toutes | Afficher la valeur dans la cellule |
| `color_low` / `color_mid` / `color_high` | `int` | gradient | basic, annotated, log, discrete | Couleurs du dégradé |
| `palette` | `list[int]` | `None` | categorical, discrete | Palette personnalisée |
| `bins` | `int` | `0` | discrete | Nombre de bandes de couleur |
| `widths` | `list[float]` | `None` | unequal | Largeurs relatives des colonnes |
| `ranges` | `list[float]` | `None` | unequal | Hauteurs relatives des lignes |
| `width` / `height` | `int` | `720` / `440` | toutes | Dimensions (px) |
| `gridlines` | `bool` | `False` | toutes | Afficher la grille |
| `background` | `str` | `None` | toutes | Couleur de fond CSS |

---

## Retour

`Chart` — objet exposant `.html` et `.show()`.

---

<div class="sp-cls sp-open" id="heatmap-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('heatmap-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('heatmap-fr','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">De base</span></button>
<button class="sp-cls-tab" onclick="spCls('heatmap-fr','annotated',this)"><span class="sp-cic">A</span><span class="sp-clb">Annoté</span></button>
<button class="sp-cls-tab" onclick="spCls('heatmap-fr','categorical',this)"><span class="sp-cic">C</span><span class="sp-clb">Catégoriel</span></button>
<button class="sp-cls-tab" onclick="spCls('heatmap-fr','unequal',this)"><span class="sp-cic">U</span><span class="sp-clb">Cellules irrégulières</span></button>
<button class="sp-cls-tab" onclick="spCls('heatmap-fr','log',this)"><span class="sp-cic">L</span><span class="sp-clb">Échelle log</span></button>
<button class="sp-cls-tab" onclick="spCls('heatmap-fr','discrete',this)"><span class="sp-cic">D</span><span class="sp-clb">Discret</span></button>
<button class="sp-cls-tab" onclick="spCls('heatmap-fr','correlation',this)"><span class="sp-cic">R</span><span class="sp-clb">Corrélation</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="heatmap-fr-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / simple / default / matrix</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmfr-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmfr-basic','hmfr-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmfr-basic','hmfr-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmfr-basic','hmfr-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmfr-basic','hmfr-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmfr-basic','hmfr-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmfr-basic','hmfr-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmfr-basic','hmfr-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmfr-basic','hmfr-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmfr-basic','hmfr-basic-cpp',this)">C++</button>
</div><div id="hmfr-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
rows = ["Alpha","Beta","Gamma","Delta"]
cols = ["Jan","Feb","Mar","Apr","May","Jun"]
m = [v for r in range(4) for v in [10,40,75,30,55,20]]
chart = sp.heatmap(
    title="Sales by Month",
    labels=rows, col_labels=cols, values=m,
    variant="basic",
    color_low=0x6366F1, color_mid=0xfafbfc, color_high=0xF43F5E,
)
chart.show()</code></pre></div><div id="hmfr-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmfr-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmfr-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmfr-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmfr-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmfr-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-basic.html"></iframe>
</div>

<div class="sp-variant" id="heatmap-fr-annotated">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"annotated"</code></span><span><strong>Aliases</strong> <code>annotated / annotate / labeled / values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmfr-annotated">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmfr-annotated','hmfr-annotated-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmfr-annotated','hmfr-annotated-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmfr-annotated','hmfr-annotated-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmfr-annotated','hmfr-annotated-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmfr-annotated','hmfr-annotated-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmfr-annotated','hmfr-annotated-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmfr-annotated','hmfr-annotated-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmfr-annotated','hmfr-annotated-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmfr-annotated','hmfr-annotated-cpp',this)">C++</button>
</div><div id="hmfr-annotated-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
rows = ["Alpha","Beta","Gamma","Delta"]
cols = ["Jan","Feb","Mar","Apr","May","Jun"]
m = [v for r in range(4) for v in [10,40,75,30,55,20]]
chart = sp.heatmap(
    title="Sales (Annotated)",
    labels=rows, col_labels=cols, values=m,
    variant="annotated",
)
chart.show()</code></pre></div><div id="hmfr-annotated-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-annotated-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-annotated-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmfr-annotated-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmfr-annotated-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmfr-annotated-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmfr-annotated-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmfr-annotated-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-annotated.html"></iframe>
</div>

<div class="sp-variant" id="heatmap-fr-categorical">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code></span><span><strong>Aliases</strong> <code>categorical / category / cat</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmfr-categorical">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmfr-categorical','hmfr-categorical-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmfr-categorical','hmfr-categorical-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmfr-categorical','hmfr-categorical-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmfr-categorical','hmfr-categorical-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmfr-categorical','hmfr-categorical-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmfr-categorical','hmfr-categorical-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmfr-categorical','hmfr-categorical-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmfr-categorical','hmfr-categorical-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmfr-categorical','hmfr-categorical-cpp',this)">C++</button>
</div><div id="hmfr-categorical-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
rows = ["North","South","East","West"]
cols = ["A","B","C","D","E"]
m = [0,1,2,1,0,  3,2,1,4,2,  1,1,0,3,2,  4,3,2,1,0]
chart = sp.heatmap(
    title="Region x Cluster",
    labels=rows, col_labels=cols, values=m,
    variant="categorical",
    palette=[0x6366F1,0xF43F5E,0x10B981,0xF59E0B,0x8B5CF6],
)
chart.show()</code></pre></div><div id="hmfr-categorical-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-categorical-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-categorical-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmfr-categorical-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmfr-categorical-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmfr-categorical-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmfr-categorical-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmfr-categorical-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-categorical.html"></iframe>
</div>

<div class="sp-variant" id="heatmap-fr-unequal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"unequal"</code></span><span><strong>Aliases</strong> <code>unequal / irregular / weighted</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmfr-unequal">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmfr-unequal','hmfr-unequal-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmfr-unequal','hmfr-unequal-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmfr-unequal','hmfr-unequal-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmfr-unequal','hmfr-unequal-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmfr-unequal','hmfr-unequal-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmfr-unequal','hmfr-unequal-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmfr-unequal','hmfr-unequal-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmfr-unequal','hmfr-unequal-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmfr-unequal','hmfr-unequal-cpp',this)">C++</button>
</div><div id="hmfr-unequal-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
rows = ["Premium","Standard","Basic"]
cols = ["Q1","Q2","Q3","Q4"]
m = [80,55,40,30,  60,50,35,25,  40,30,20,15]
chart = sp.heatmap(
    title="Tier x Quarter (Unequal cells)",
    labels=rows, col_labels=cols, values=m,
    variant="unequal",
    widths=[1.0, 2.5, 1.6, 0.9],
    ranges=[2.2, 1.0, 1.5],
)
chart.show()</code></pre></div><div id="hmfr-unequal-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-unequal-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-unequal-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmfr-unequal-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmfr-unequal-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmfr-unequal-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmfr-unequal-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmfr-unequal-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-unequal.html"></iframe>
</div>

<div class="sp-variant" id="heatmap-fr-log">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"log"</code></span><span><strong>Aliases</strong> <code>log / logarithmic / log10</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmfr-log">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmfr-log','hmfr-log-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmfr-log','hmfr-log-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmfr-log','hmfr-log-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmfr-log','hmfr-log-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmfr-log','hmfr-log-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmfr-log','hmfr-log-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmfr-log','hmfr-log-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmfr-log','hmfr-log-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmfr-log','hmfr-log-cpp',this)">C++</button>
</div><div id="hmfr-log-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
rows = ["Network","CPU","Memory","Disk"]
cols = ["t1","t2","t3","t4","t5","t6"]
m = [10,100,1000,5000,80,2,  3,30,800,9000,500,8,  5,200,1500,3000,40,1,  20,300,2200,7000,90,4]
chart = sp.heatmap(
    title="Resource Load (Log)",
    labels=rows, col_labels=cols, values=m,
    variant="log",
)
chart.show()</code></pre></div><div id="hmfr-log-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-log-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-log-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmfr-log-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmfr-log-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmfr-log-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmfr-log-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmfr-log-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-log.html"></iframe>
</div>

<div class="sp-variant" id="heatmap-fr-discrete">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"discrete"</code></span><span><strong>Aliases</strong> <code>discrete / binned / stepped / bands</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmfr-discrete">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmfr-discrete','hmfr-discrete-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmfr-discrete','hmfr-discrete-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmfr-discrete','hmfr-discrete-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmfr-discrete','hmfr-discrete-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmfr-discrete','hmfr-discrete-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmfr-discrete','hmfr-discrete-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmfr-discrete','hmfr-discrete-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmfr-discrete','hmfr-discrete-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmfr-discrete','hmfr-discrete-cpp',this)">C++</button>
</div><div id="hmfr-discrete-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
rows = ["Alpha","Beta","Gamma","Delta"]
cols = ["Jan","Feb","Mar","Apr","May","Jun"]
m = [v for r in range(4) for v in [10,40,75,30,55,20]]
chart = sp.heatmap(
    title="Sales tiers (Discrete)",
    labels=rows, col_labels=cols, values=m,
    variant="discrete",
    bins=6,
    palette=[0x1E3A8A,0x3B82F6,0x60A5FA,0xFBBF24,0xF97316,0xDC2626],
)
chart.show()</code></pre></div><div id="hmfr-discrete-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-discrete-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-discrete-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmfr-discrete-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmfr-discrete-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmfr-discrete-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmfr-discrete-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmfr-discrete-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-discrete.html"></iframe>
</div>

<div class="sp-variant" id="heatmap-fr-correlation">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"correlation"</code></span><span><strong>Aliases</strong> <code>correlation / corr / diverging / pearson</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="hmfr-correlation">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hmfr-correlation','hmfr-correlation-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hmfr-correlation','hmfr-correlation-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hmfr-correlation','hmfr-correlation-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hmfr-correlation','hmfr-correlation-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hmfr-correlation','hmfr-correlation-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('hmfr-correlation','hmfr-correlation-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hmfr-correlation','hmfr-correlation-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hmfr-correlation','hmfr-correlation-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hmfr-correlation','hmfr-correlation-cpp',this)">C++</button>
</div><div id="hmfr-correlation-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
features = ["Sales","Ads","Visits","Signups","Revenue"]
m = [
     1.00, 0.82, 0.55,-0.12, 0.91,
     0.82, 1.00, 0.43,-0.05, 0.78,
     0.55, 0.43, 1.00, 0.62, 0.49,
    -0.12,-0.05, 0.62, 1.00,-0.08,
     0.91, 0.78, 0.49,-0.08, 1.00,
]
chart = sp.heatmap(
    title="Feature Correlation",
    labels=features, col_labels=features, values=m,
    variant="correlation",
)
chart.show()</code></pre></div><div id="hmfr-correlation-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-correlation-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.heatmap({
  title: "Heatmap",
  labels: ["A","B","C"],
  col_labels: ["X","Y","Z"],
  values: [/* row-major matrix */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="hmfr-correlation-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$heatmap(
  title = "Heatmap",
  labels = c("A","B","C"),
  col_labels = c("X","Y","Z"),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="hmfr-correlation-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::heatmap()
        .title("Heatmap")
        .labels(vec!["A","B","C"])
        .col_labels(vec!["X","Y","Z"])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="hmfr-correlation-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Heatmap")
    .labels(List.of("A","B","C"))
    .colLabels(List.of("X","Y","Z"))
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="hmfr-correlation-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Heatmap",
    labels: new[]{"A","B","C"},
    colLabels: new[]{"X","Y","Z"},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="hmfr-correlation-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.heatmap(
  title      = "Heatmap",
  labels     = List("A","B","C"),
  colLabels  = List("X","Y","Z"),
  values     = List(),
  variant    = "basic"
)
chart.show()</code></pre></div><div id="hmfr-correlation-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
    .title      = "Heatmap",
    .labels     = {"A","B","C"},
    .col_labels = {"X","Y","Z"},
    .values     = {},
    .variant    = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/heatmap-correlation.html"></iframe>
</div>

</div>
</div>

</div>
