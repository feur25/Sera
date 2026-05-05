# Dumbbell - Before / After Two-Point Comparison

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
.sp-preview-frame{width:100%;height:520px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.dumbbell(title, labels, start, end, *, variant="basic", series_name_start="Start", series_name_end="End", **kwargs) -> Chart`

Aliases: `sp.dumbbell`, `sp.build_dumbbell`

## Description

`sp.dumbbell()` is the unified entry point for the dumbbell-chart family. Each row plots two values - typically a before and an after - linked by a connector, making it the chart of choice for change, gap or comparison-over-time analyses (salary equity, turnaround KPIs, A/B uplifts, etc.). The `variant` keyword switches the visual treatment without touching the data.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / dot` | Classic two-dot dumbbell with a gray connecting bar; the workhorse of before/after comparisons. |
| `"arrow"` | `arrow / directional / delta_arrow / flow` | Arrowhead points from start to end so direction of change is immediate. |
| `"delta"` | `delta / change / diff / signed` | Bar between dots is colored by sign (green up, red down) to encode direction and magnitude. |
| `"barbell"` | `barbell / thick / weighted / editorial` | Square weighted endpoints on a thick gray axis - editorial barbell look for slides. |
| `"glow"` | `glow / halo / neon / soft` | Soft halo around endpoints with thin connector for a luminous, modern feel. |
| `"dotted"` | `dotted / dashed / minimal / thin` | Dashed connector with hollow ring markers - lightweight and airy. |
| `"ranked"` | `ranked / ranking / ordered / numbered` | Adds a numeric rank in front of every label - perfect for top-N comparisons. |
| `"vertical"` | `vertical / column / rotated / v` | Rotates the dumbbells into vertical columns - ideal when many short labels would crowd horizontally. |

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Row labels |
| `start` | `list[float]` | required | Starting values (one per label) |
| `end` | `list[float]` | required | Ending values (one per label) |
| `variant` | `str` | `"basic"` | Visual style (see table) |
| `series_name_start` | `str` | `"Start"` | Legend label for the start series |
| `series_name_end` | `str` | `"End"` | Legend label for the end series |
| `palette` | `list[int]` | `None` | Custom palette: `[c_start, c_end, ...]` |
| `sort_order` | `str` | `"none"` | `"none"`, `"asc"` or `"desc"` |
| `width` | `int` | `1000` | Canvas width (px) |
| `height` | `int` | `500` | Canvas height (px) |

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="dumbbell-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('dumbbell-en')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('dumbbell-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','arrow',this)"><span class="sp-cic">A</span><span class="sp-clb">Arrow</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','delta',this)"><span class="sp-cic">D</span><span class="sp-clb">Delta</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','barbell',this)"><span class="sp-cic">W</span><span class="sp-clb">Barbell</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','glow',this)"><span class="sp-cic">G</span><span class="sp-clb">Glow</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','dotted',this)"><span class="sp-cic">O</span><span class="sp-clb">Dotted</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','ranked',this)"><span class="sp-cic">R</span><span class="sp-clb">Ranked</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','vertical',this)"><span class="sp-cic">V</span><span class="sp-clb">Vertical</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="dumbbell-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / dot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic two-dot dumbbell with a gray connecting bar; the workhorse of before/after comparisons.</p>
<div class="sp-tabs" id="dumbbell-en-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-en-basic','dumbbell-en-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-basic','dumbbell-en-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-basic','dumbbell-en-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-basic','dumbbell-en-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-basic','dumbbell-en-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-basic','dumbbell-en-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-basic','dumbbell-en-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-basic','dumbbell-en-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-basic','dumbbell-en-basic-cpp',this)">C++</button>
</div><div id="dumbbell-en-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="basic",
)
chart.show()</code></pre></div><div id="dumbbell-en-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="dumbbell-en-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="dumbbell-en-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="dumbbell-en-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-en-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="dumbbell-en-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="dumbbell-en-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="dumbbell-en-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-basic.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-arrow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arrow"</code></span><span><strong>Aliases</strong> <code>arrow / directional / delta_arrow / flow</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Arrowhead points from start to end so direction of change is immediate.</p>
<div class="sp-tabs" id="dumbbell-en-arrow">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-en-arrow','dumbbell-en-arrow-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-arrow','dumbbell-en-arrow-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-arrow','dumbbell-en-arrow-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-arrow','dumbbell-en-arrow-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-arrow','dumbbell-en-arrow-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-arrow','dumbbell-en-arrow-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-arrow','dumbbell-en-arrow-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-arrow','dumbbell-en-arrow-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-arrow','dumbbell-en-arrow-cpp',this)">C++</button>
</div><div id="dumbbell-en-arrow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="arrow",
)
chart.show()</code></pre></div><div id="dumbbell-en-arrow-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "arrow",
});
chart.show();</code></pre></div><div id="dumbbell-en-arrow-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "arrow",
});
chart.show();</code></pre></div><div id="dumbbell-en-arrow-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "arrow"
)
chart$show()</code></pre></div><div id="dumbbell-en-arrow-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("arrow")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-en-arrow-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("arrow")
    .build();
chart.show();</code></pre></div><div id="dumbbell-en-arrow-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "arrow"
);
chart.Show();</code></pre></div><div id="dumbbell-en-arrow-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "arrow"
)
chart.show()</code></pre></div><div id="dumbbell-en-arrow-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "arrow",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-arrow.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-delta">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"delta"</code></span><span><strong>Aliases</strong> <code>delta / change / diff / signed</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bar between dots is colored by sign (green up, red down) to encode direction and magnitude.</p>
<div class="sp-tabs" id="dumbbell-en-delta">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-en-delta','dumbbell-en-delta-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-delta','dumbbell-en-delta-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-delta','dumbbell-en-delta-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-delta','dumbbell-en-delta-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-delta','dumbbell-en-delta-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-delta','dumbbell-en-delta-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-delta','dumbbell-en-delta-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-delta','dumbbell-en-delta-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-delta','dumbbell-en-delta-cpp',this)">C++</button>
</div><div id="dumbbell-en-delta-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="delta",
)
chart.show()</code></pre></div><div id="dumbbell-en-delta-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "delta",
});
chart.show();</code></pre></div><div id="dumbbell-en-delta-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "delta",
});
chart.show();</code></pre></div><div id="dumbbell-en-delta-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "delta"
)
chart$show()</code></pre></div><div id="dumbbell-en-delta-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("delta")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-en-delta-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("delta")
    .build();
chart.show();</code></pre></div><div id="dumbbell-en-delta-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "delta"
);
chart.Show();</code></pre></div><div id="dumbbell-en-delta-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "delta"
)
chart.show()</code></pre></div><div id="dumbbell-en-delta-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "delta",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-delta.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-barbell">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"barbell"</code></span><span><strong>Aliases</strong> <code>barbell / thick / weighted / editorial</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Square weighted endpoints on a thick gray axis - editorial barbell look for slides.</p>
<div class="sp-tabs" id="dumbbell-en-barbell">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-en-barbell','dumbbell-en-barbell-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-barbell','dumbbell-en-barbell-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-barbell','dumbbell-en-barbell-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-barbell','dumbbell-en-barbell-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-barbell','dumbbell-en-barbell-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-barbell','dumbbell-en-barbell-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-barbell','dumbbell-en-barbell-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-barbell','dumbbell-en-barbell-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-barbell','dumbbell-en-barbell-cpp',this)">C++</button>
</div><div id="dumbbell-en-barbell-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="barbell",
)
chart.show()</code></pre></div><div id="dumbbell-en-barbell-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "barbell",
});
chart.show();</code></pre></div><div id="dumbbell-en-barbell-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "barbell",
});
chart.show();</code></pre></div><div id="dumbbell-en-barbell-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "barbell"
)
chart$show()</code></pre></div><div id="dumbbell-en-barbell-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("barbell")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-en-barbell-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("barbell")
    .build();
chart.show();</code></pre></div><div id="dumbbell-en-barbell-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "barbell"
);
chart.Show();</code></pre></div><div id="dumbbell-en-barbell-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "barbell"
)
chart.show()</code></pre></div><div id="dumbbell-en-barbell-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "barbell",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-barbell.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-glow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"glow"</code></span><span><strong>Aliases</strong> <code>glow / halo / neon / soft</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Soft halo around endpoints with thin connector for a luminous, modern feel.</p>
<div class="sp-tabs" id="dumbbell-en-glow">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-en-glow','dumbbell-en-glow-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-glow','dumbbell-en-glow-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-glow','dumbbell-en-glow-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-glow','dumbbell-en-glow-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-glow','dumbbell-en-glow-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-glow','dumbbell-en-glow-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-glow','dumbbell-en-glow-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-glow','dumbbell-en-glow-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-glow','dumbbell-en-glow-cpp',this)">C++</button>
</div><div id="dumbbell-en-glow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="glow",
)
chart.show()</code></pre></div><div id="dumbbell-en-glow-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "glow",
});
chart.show();</code></pre></div><div id="dumbbell-en-glow-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "glow",
});
chart.show();</code></pre></div><div id="dumbbell-en-glow-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "glow"
)
chart$show()</code></pre></div><div id="dumbbell-en-glow-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("glow")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-en-glow-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("glow")
    .build();
chart.show();</code></pre></div><div id="dumbbell-en-glow-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "glow"
);
chart.Show();</code></pre></div><div id="dumbbell-en-glow-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "glow"
)
chart.show()</code></pre></div><div id="dumbbell-en-glow-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "glow",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-glow.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-dotted">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dotted"</code></span><span><strong>Aliases</strong> <code>dotted / dashed / minimal / thin</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Dashed connector with hollow ring markers - lightweight and airy.</p>
<div class="sp-tabs" id="dumbbell-en-dotted">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-en-dotted','dumbbell-en-dotted-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-dotted','dumbbell-en-dotted-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-dotted','dumbbell-en-dotted-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-dotted','dumbbell-en-dotted-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-dotted','dumbbell-en-dotted-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-dotted','dumbbell-en-dotted-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-dotted','dumbbell-en-dotted-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-dotted','dumbbell-en-dotted-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-dotted','dumbbell-en-dotted-cpp',this)">C++</button>
</div><div id="dumbbell-en-dotted-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="dotted",
)
chart.show()</code></pre></div><div id="dumbbell-en-dotted-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "dotted",
});
chart.show();</code></pre></div><div id="dumbbell-en-dotted-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "dotted",
});
chart.show();</code></pre></div><div id="dumbbell-en-dotted-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "dotted"
)
chart$show()</code></pre></div><div id="dumbbell-en-dotted-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("dotted")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-en-dotted-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("dotted")
    .build();
chart.show();</code></pre></div><div id="dumbbell-en-dotted-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "dotted"
);
chart.Show();</code></pre></div><div id="dumbbell-en-dotted-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "dotted"
)
chart.show()</code></pre></div><div id="dumbbell-en-dotted-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "dotted",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-dotted.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-ranked">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ranked"</code></span><span><strong>Aliases</strong> <code>ranked / ranking / ordered / numbered</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Adds a numeric rank in front of every label - perfect for top-N comparisons.</p>
<div class="sp-tabs" id="dumbbell-en-ranked">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-en-ranked','dumbbell-en-ranked-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-ranked','dumbbell-en-ranked-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-ranked','dumbbell-en-ranked-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-ranked','dumbbell-en-ranked-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-ranked','dumbbell-en-ranked-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-ranked','dumbbell-en-ranked-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-ranked','dumbbell-en-ranked-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-ranked','dumbbell-en-ranked-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-ranked','dumbbell-en-ranked-cpp',this)">C++</button>
</div><div id="dumbbell-en-ranked-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="ranked",
)
chart.show()</code></pre></div><div id="dumbbell-en-ranked-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "ranked",
});
chart.show();</code></pre></div><div id="dumbbell-en-ranked-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "ranked",
});
chart.show();</code></pre></div><div id="dumbbell-en-ranked-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "ranked"
)
chart$show()</code></pre></div><div id="dumbbell-en-ranked-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("ranked")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-en-ranked-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("ranked")
    .build();
chart.show();</code></pre></div><div id="dumbbell-en-ranked-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "ranked"
);
chart.Show();</code></pre></div><div id="dumbbell-en-ranked-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "ranked"
)
chart.show()</code></pre></div><div id="dumbbell-en-ranked-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "ranked",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-ranked.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-vertical">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"vertical"</code></span><span><strong>Aliases</strong> <code>vertical / column / rotated / v</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Rotates the dumbbells into vertical columns - ideal when many short labels would crowd horizontally.</p>
<div class="sp-tabs" id="dumbbell-en-vertical">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-en-vertical','dumbbell-en-vertical-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-vertical','dumbbell-en-vertical-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-vertical','dumbbell-en-vertical-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-vertical','dumbbell-en-vertical-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-vertical','dumbbell-en-vertical-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-vertical','dumbbell-en-vertical-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-vertical','dumbbell-en-vertical-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-vertical','dumbbell-en-vertical-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-en-vertical','dumbbell-en-vertical-cpp',this)">C++</button>
</div><div id="dumbbell-en-vertical-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="vertical",
)
chart.show()</code></pre></div><div id="dumbbell-en-vertical-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "vertical",
});
chart.show();</code></pre></div><div id="dumbbell-en-vertical-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "vertical",
});
chart.show();</code></pre></div><div id="dumbbell-en-vertical-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "vertical"
)
chart$show()</code></pre></div><div id="dumbbell-en-vertical-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("vertical")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-en-vertical-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("vertical")
    .build();
chart.show();</code></pre></div><div id="dumbbell-en-vertical-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "vertical"
);
chart.Show();</code></pre></div><div id="dumbbell-en-vertical-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "vertical"
)
chart.show()</code></pre></div><div id="dumbbell-en-vertical-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "vertical",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-vertical.html"></iframe>
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
.sp-preview-frame{width:100%;height:520px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.dumbbell(title, labels, start, end, *, variant="basic", series_name_start="Start", series_name_end="End", **kwargs) -> Chart`

Aliases: `sp.dumbbell`, `sp.build_dumbbell`

## Description

`sp.dumbbell()` est le point d entree unique pour la famille dumbbell. Chaque ligne montre deux valeurs - typiquement avant/apres - reliees par un connecteur, ce qui en fait le choix naturel pour visualiser un changement, un ecart ou une evolution (equite salariale, KPIs de redressement, uplifts A/B, etc.). Le mot-cle `variant` change le style visuel sans toucher aux donnees.

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / dot` | Dumbbell classique a deux points et barre grise - la base des comparaisons avant/apres. |
| `"arrow"` | `arrow / directional / delta_arrow / flow` | Une fleche pointe du depart vers l arrivee, la direction du changement est immediate. |
| `"delta"` | `delta / change / diff / signed` | La barre entre les points prend la couleur du signe (vert hausse, rouge baisse). |
| `"barbell"` | `barbell / thick / weighted / editorial` | Halteres carres sur un axe epais - look editorial pour presentations. |
| `"glow"` | `glow / halo / neon / soft` | Halo doux autour des extremites avec connecteur fin - style lumineux et moderne. |
| `"dotted"` | `dotted / dashed / minimal / thin` | Connecteur en pointilles et marqueurs en anneaux - leger et aere. |
| `"ranked"` | `ranked / ranking / ordered / numbered` | Ajoute un rang numerique devant chaque label - ideal pour comparer un top-N. |
| `"vertical"` | `vertical / column / rotated / v` | Bascule les dumbbells en colonnes verticales - parfait pour de nombreux labels courts. |

## Parametres

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Libelles de ligne |
| `start` | `list[float]` | requis | Valeurs de depart |
| `end` | `list[float]` | requis | Valeurs d arrivee |
| `variant` | `str` | `"basic"` | Style visuel (voir tableau) |
| `series_name_start` | `str` | `"Start"` | Label legende serie depart |
| `series_name_end` | `str` | `"End"` | Label legende serie arrivee |
| `palette` | `list[int]` | `None` | Palette personnalisee: `[c_start, c_end, ...]` |
| `sort_order` | `str` | `"none"` | `"none"`, `"asc"` or `"desc"` |
| `width` | `int` | `1000` | Largeur (px) |
| `height` | `int` | `500` | Hauteur (px) |

## Retour

`Chart` - objet avec propriete `.html` et methode `.show()`.

---

<div class="sp-cls sp-open" id="dumbbell-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('dumbbell-fr')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('dumbbell-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','arrow',this)"><span class="sp-cic">A</span><span class="sp-clb">Arrow</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','delta',this)"><span class="sp-cic">D</span><span class="sp-clb">Delta</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','barbell',this)"><span class="sp-cic">W</span><span class="sp-clb">Barbell</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','glow',this)"><span class="sp-cic">G</span><span class="sp-clb">Glow</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','dotted',this)"><span class="sp-cic">O</span><span class="sp-clb">Dotted</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','ranked',this)"><span class="sp-cic">R</span><span class="sp-clb">Ranked</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','vertical',this)"><span class="sp-cic">V</span><span class="sp-clb">Vertical</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="dumbbell-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / dot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Dumbbell classique a deux points et barre grise - la base des comparaisons avant/apres.</p>
<div class="sp-tabs" id="dumbbell-fr-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-fr-basic','dumbbell-fr-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-basic','dumbbell-fr-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-basic','dumbbell-fr-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-basic','dumbbell-fr-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-basic','dumbbell-fr-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-basic','dumbbell-fr-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-basic','dumbbell-fr-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-basic','dumbbell-fr-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-basic','dumbbell-fr-basic-cpp',this)">C++</button>
</div><div id="dumbbell-fr-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="basic",
)
chart.show()</code></pre></div><div id="dumbbell-fr-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="dumbbell-fr-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="dumbbell-fr-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="dumbbell-fr-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-fr-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="dumbbell-fr-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="dumbbell-fr-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="dumbbell-fr-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-basic.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-arrow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arrow"</code></span><span><strong>Aliases</strong> <code>arrow / directional / delta_arrow / flow</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Une fleche pointe du depart vers l arrivee, la direction du changement est immediate.</p>
<div class="sp-tabs" id="dumbbell-fr-arrow">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-fr-arrow','dumbbell-fr-arrow-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-arrow','dumbbell-fr-arrow-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-arrow','dumbbell-fr-arrow-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-arrow','dumbbell-fr-arrow-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-arrow','dumbbell-fr-arrow-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-arrow','dumbbell-fr-arrow-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-arrow','dumbbell-fr-arrow-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-arrow','dumbbell-fr-arrow-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-arrow','dumbbell-fr-arrow-cpp',this)">C++</button>
</div><div id="dumbbell-fr-arrow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="arrow",
)
chart.show()</code></pre></div><div id="dumbbell-fr-arrow-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "arrow",
});
chart.show();</code></pre></div><div id="dumbbell-fr-arrow-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "arrow",
});
chart.show();</code></pre></div><div id="dumbbell-fr-arrow-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "arrow"
)
chart$show()</code></pre></div><div id="dumbbell-fr-arrow-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("arrow")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-fr-arrow-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("arrow")
    .build();
chart.show();</code></pre></div><div id="dumbbell-fr-arrow-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "arrow"
);
chart.Show();</code></pre></div><div id="dumbbell-fr-arrow-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "arrow"
)
chart.show()</code></pre></div><div id="dumbbell-fr-arrow-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "arrow",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-arrow.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-delta">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"delta"</code></span><span><strong>Aliases</strong> <code>delta / change / diff / signed</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">La barre entre les points prend la couleur du signe (vert hausse, rouge baisse).</p>
<div class="sp-tabs" id="dumbbell-fr-delta">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-fr-delta','dumbbell-fr-delta-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-delta','dumbbell-fr-delta-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-delta','dumbbell-fr-delta-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-delta','dumbbell-fr-delta-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-delta','dumbbell-fr-delta-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-delta','dumbbell-fr-delta-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-delta','dumbbell-fr-delta-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-delta','dumbbell-fr-delta-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-delta','dumbbell-fr-delta-cpp',this)">C++</button>
</div><div id="dumbbell-fr-delta-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="delta",
)
chart.show()</code></pre></div><div id="dumbbell-fr-delta-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "delta",
});
chart.show();</code></pre></div><div id="dumbbell-fr-delta-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "delta",
});
chart.show();</code></pre></div><div id="dumbbell-fr-delta-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "delta"
)
chart$show()</code></pre></div><div id="dumbbell-fr-delta-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("delta")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-fr-delta-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("delta")
    .build();
chart.show();</code></pre></div><div id="dumbbell-fr-delta-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "delta"
);
chart.Show();</code></pre></div><div id="dumbbell-fr-delta-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "delta"
)
chart.show()</code></pre></div><div id="dumbbell-fr-delta-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "delta",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-delta.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-barbell">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"barbell"</code></span><span><strong>Aliases</strong> <code>barbell / thick / weighted / editorial</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Halteres carres sur un axe epais - look editorial pour presentations.</p>
<div class="sp-tabs" id="dumbbell-fr-barbell">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-fr-barbell','dumbbell-fr-barbell-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-barbell','dumbbell-fr-barbell-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-barbell','dumbbell-fr-barbell-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-barbell','dumbbell-fr-barbell-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-barbell','dumbbell-fr-barbell-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-barbell','dumbbell-fr-barbell-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-barbell','dumbbell-fr-barbell-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-barbell','dumbbell-fr-barbell-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-barbell','dumbbell-fr-barbell-cpp',this)">C++</button>
</div><div id="dumbbell-fr-barbell-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="barbell",
)
chart.show()</code></pre></div><div id="dumbbell-fr-barbell-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "barbell",
});
chart.show();</code></pre></div><div id="dumbbell-fr-barbell-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "barbell",
});
chart.show();</code></pre></div><div id="dumbbell-fr-barbell-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "barbell"
)
chart$show()</code></pre></div><div id="dumbbell-fr-barbell-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("barbell")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-fr-barbell-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("barbell")
    .build();
chart.show();</code></pre></div><div id="dumbbell-fr-barbell-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "barbell"
);
chart.Show();</code></pre></div><div id="dumbbell-fr-barbell-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "barbell"
)
chart.show()</code></pre></div><div id="dumbbell-fr-barbell-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "barbell",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-barbell.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-glow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"glow"</code></span><span><strong>Aliases</strong> <code>glow / halo / neon / soft</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Halo doux autour des extremites avec connecteur fin - style lumineux et moderne.</p>
<div class="sp-tabs" id="dumbbell-fr-glow">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-fr-glow','dumbbell-fr-glow-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-glow','dumbbell-fr-glow-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-glow','dumbbell-fr-glow-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-glow','dumbbell-fr-glow-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-glow','dumbbell-fr-glow-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-glow','dumbbell-fr-glow-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-glow','dumbbell-fr-glow-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-glow','dumbbell-fr-glow-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-glow','dumbbell-fr-glow-cpp',this)">C++</button>
</div><div id="dumbbell-fr-glow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="glow",
)
chart.show()</code></pre></div><div id="dumbbell-fr-glow-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "glow",
});
chart.show();</code></pre></div><div id="dumbbell-fr-glow-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "glow",
});
chart.show();</code></pre></div><div id="dumbbell-fr-glow-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "glow"
)
chart$show()</code></pre></div><div id="dumbbell-fr-glow-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("glow")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-fr-glow-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("glow")
    .build();
chart.show();</code></pre></div><div id="dumbbell-fr-glow-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "glow"
);
chart.Show();</code></pre></div><div id="dumbbell-fr-glow-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "glow"
)
chart.show()</code></pre></div><div id="dumbbell-fr-glow-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "glow",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-glow.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-dotted">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dotted"</code></span><span><strong>Aliases</strong> <code>dotted / dashed / minimal / thin</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Connecteur en pointilles et marqueurs en anneaux - leger et aere.</p>
<div class="sp-tabs" id="dumbbell-fr-dotted">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-fr-dotted','dumbbell-fr-dotted-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-dotted','dumbbell-fr-dotted-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-dotted','dumbbell-fr-dotted-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-dotted','dumbbell-fr-dotted-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-dotted','dumbbell-fr-dotted-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-dotted','dumbbell-fr-dotted-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-dotted','dumbbell-fr-dotted-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-dotted','dumbbell-fr-dotted-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-dotted','dumbbell-fr-dotted-cpp',this)">C++</button>
</div><div id="dumbbell-fr-dotted-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="dotted",
)
chart.show()</code></pre></div><div id="dumbbell-fr-dotted-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "dotted",
});
chart.show();</code></pre></div><div id="dumbbell-fr-dotted-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "dotted",
});
chart.show();</code></pre></div><div id="dumbbell-fr-dotted-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "dotted"
)
chart$show()</code></pre></div><div id="dumbbell-fr-dotted-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("dotted")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-fr-dotted-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("dotted")
    .build();
chart.show();</code></pre></div><div id="dumbbell-fr-dotted-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "dotted"
);
chart.Show();</code></pre></div><div id="dumbbell-fr-dotted-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "dotted"
)
chart.show()</code></pre></div><div id="dumbbell-fr-dotted-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "dotted",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-dotted.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-ranked">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ranked"</code></span><span><strong>Aliases</strong> <code>ranked / ranking / ordered / numbered</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Ajoute un rang numerique devant chaque label - ideal pour comparer un top-N.</p>
<div class="sp-tabs" id="dumbbell-fr-ranked">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-fr-ranked','dumbbell-fr-ranked-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-ranked','dumbbell-fr-ranked-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-ranked','dumbbell-fr-ranked-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-ranked','dumbbell-fr-ranked-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-ranked','dumbbell-fr-ranked-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-ranked','dumbbell-fr-ranked-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-ranked','dumbbell-fr-ranked-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-ranked','dumbbell-fr-ranked-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-ranked','dumbbell-fr-ranked-cpp',this)">C++</button>
</div><div id="dumbbell-fr-ranked-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="ranked",
)
chart.show()</code></pre></div><div id="dumbbell-fr-ranked-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "ranked",
});
chart.show();</code></pre></div><div id="dumbbell-fr-ranked-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "ranked",
});
chart.show();</code></pre></div><div id="dumbbell-fr-ranked-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "ranked"
)
chart$show()</code></pre></div><div id="dumbbell-fr-ranked-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("ranked")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-fr-ranked-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("ranked")
    .build();
chart.show();</code></pre></div><div id="dumbbell-fr-ranked-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "ranked"
);
chart.Show();</code></pre></div><div id="dumbbell-fr-ranked-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "ranked"
)
chart.show()</code></pre></div><div id="dumbbell-fr-ranked-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "ranked",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-ranked.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-vertical">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"vertical"</code></span><span><strong>Aliases</strong> <code>vertical / column / rotated / v</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bascule les dumbbells en colonnes verticales - parfait pour de nombreux labels courts.</p>
<div class="sp-tabs" id="dumbbell-fr-vertical">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-fr-vertical','dumbbell-fr-vertical-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-vertical','dumbbell-fr-vertical-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-vertical','dumbbell-fr-vertical-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-vertical','dumbbell-fr-vertical-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-vertical','dumbbell-fr-vertical-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-vertical','dumbbell-fr-vertical-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-vertical','dumbbell-fr-vertical-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-vertical','dumbbell-fr-vertical-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr-vertical','dumbbell-fr-vertical-cpp',this)">C++</button>
</div><div id="dumbbell-fr-vertical-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Alice","Bob","Cara","Dan","Eve","Finn","Gina","Han"]
start  = [42, 55, 38, 61, 47, 33, 58, 50]
end    = [68, 49, 71, 58, 80, 52, 64, 75]

chart = sp.dumbbell(
    title="Salaries 2023 vs 2024",
    labels=labels, start=start, end=end,
    series_name_start="2023", series_name_end="2024",
    variant="vertical",
)
chart.show()</code></pre></div><div id="dumbbell-fr-vertical-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "vertical",
});
chart.show();</code></pre></div><div id="dumbbell-fr-vertical-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.dumbbell({
  title: "Salaries 2023 vs 2024",
  labels: [/* ... */], start: [/* ... */], end: [/* ... */],
  variant: "vertical",
});
chart.show();</code></pre></div><div id="dumbbell-fr-vertical-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$dumbbell(
  title = "Salaries 2023 vs 2024",
  labels = c(), start = c(), end = c(),
  variant = "vertical"
)
chart$show()</code></pre></div><div id="dumbbell-fr-vertical-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::dumbbell()
        .title("Salaries 2023 vs 2024")
        .labels(vec![]).start(vec![]).end(vec![])
        .variant("vertical")
        .build();
    chart.show();
}</code></pre></div><div id="dumbbell-fr-vertical-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Salaries 2023 vs 2024")
    .labels(List.of()).start(List.of()).end(List.of())
    .variant("vertical")
    .build();
chart.show();</code></pre></div><div id="dumbbell-fr-vertical-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title: "Salaries 2023 vs 2024",
    labels: new string[]{}, start: new double[]{}, end: new double[]{},
    variant: "vertical"
);
chart.Show();</code></pre></div><div id="dumbbell-fr-vertical-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title   = "Salaries 2023 vs 2024",
  labels  = List(), start = List(), end = List(),
  variant = "vertical"
)
chart.show()</code></pre></div><div id="dumbbell-fr-vertical-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title   = "Salaries 2023 vs 2024",
    .labels  = {}, .start = {}, .end = {},
    .variant = "vertical",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-vertical.html"></iframe>
</div>
</div>
</div>

</div>
