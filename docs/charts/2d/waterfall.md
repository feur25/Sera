# Waterfall — Running-Total Bridge Chart

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

`sp.waterfall(title, labels, values, *, variant="basic", show_text=True, **kwargs) -> Chart`

Aliases: `sp.waterfall`, `sp.build_waterfall`

## Description

`sp.waterfall()` renders the entire waterfall-chart family: a sequence of bars where each step adds (positive) or subtracts (negative) from a running total. The `variant` keyword selects the geometry without touching any other parameter. Waterfalls are the standard for P&L bridges, variance analysis, cohort decomposition, fee/tax breakdowns and any "from A to B, what changed?" narrative.

> **Totals** — set a value to `0` and use a label containing `total`, `net`, `final`, `gross` or `ebitda` to mark a subtotal bar; it is rendered with the totals color and anchored on the running sum.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / bars` | Classic running-sum bars with dashed connectors between consecutive steps. |
| `"stepped"` | `stepped / step / staircase / stairs` | Bars touch each other forming a continuous staircase, no connectors needed. |
| `"gradient"` | `gradient / shaded / fade / vgrad` | Vertical linear gradient on each bar (opaque top -> faded bottom) for depth. |
| `"rounded"` | `rounded / round / pill / soft` | Large corner radius gives bars a soft pill shape, brand-friendly. |
| `"lollipop"` | `lollipop / stick / popsicle / lolly` | Stick + circle marker at the end of each step. Excellent ink-to-data ratio. |
| `"arrowed"` | `arrowed / arrow / directional / tipped` | Triangle on top (positives) or bottom (negatives) emphasizes direction at a glance. |
| `"delta"` | `delta / percent / annotated / pct` | Bars + signed percentage badge (Delta vs previous running total) above each step. |
| `"outlined"` | `outlined / outline / stroke / wireframe` | Stroke-only bars with translucent fill, perfect for slide overlays and minimalist reports. |

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title`     | `str`         | required  | Chart title |
| `labels`    | `list[str]`   | required  | Step labels (left to right) |
| `values`    | `list[float]` | required  | Step deltas (use 0 + total-keyword label for subtotals) |
| `variant`   | `str`         | `"basic"` | Geometry variant (see table) |
| `show_text` | `bool`        | `True`    | Show value annotations above bars |
| `width`     | `int`         | `900`     | Canvas width (px) |
| `height`    | `int`         | `480`     | Canvas height (px) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="waterfall-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('waterfall-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('waterfall-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-en','stepped',this)"><span class="sp-cic">S</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-en','gradient',this)"><span class="sp-cic">G</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-en','rounded',this)"><span class="sp-cic">R</span><span class="sp-clb">Rounded</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-en','lollipop',this)"><span class="sp-cic">L</span><span class="sp-clb">Lollipop</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-en','arrowed',this)"><span class="sp-cic">A</span><span class="sp-clb">Arrowed</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-en','delta',this)"><span class="sp-cic">D</span><span class="sp-clb">Delta</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-en','outlined',this)"><span class="sp-cic">O</span><span class="sp-clb">Outlined</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="waterfall-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / bars</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic running-sum bars with dashed connectors between consecutive steps.</p>
<div class="sp-tabs" id="waterfall-en-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-en-basic','waterfall-en-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-en-basic','waterfall-en-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-basic','waterfall-en-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-basic','waterfall-en-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-en-basic','waterfall-en-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-en-basic','waterfall-en-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-en-basic','waterfall-en-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-en-basic','waterfall-en-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-en-basic','waterfall-en-basic-cpp',this)">C++</button>
</div><div id="waterfall-en-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="basic",
)
chart.show()</code></pre></div><div id="waterfall-en-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="waterfall-en-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="waterfall-en-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="waterfall-en-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-en-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="waterfall-en-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="waterfall-en-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="waterfall-en-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-basic.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-en-stepped">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>stepped / step / staircase / stairs</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bars touch each other forming a continuous staircase, no connectors needed.</p>
<div class="sp-tabs" id="waterfall-en-stepped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-en-stepped','waterfall-en-stepped-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-en-stepped','waterfall-en-stepped-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-stepped','waterfall-en-stepped-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-stepped','waterfall-en-stepped-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-en-stepped','waterfall-en-stepped-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-en-stepped','waterfall-en-stepped-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-en-stepped','waterfall-en-stepped-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-en-stepped','waterfall-en-stepped-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-en-stepped','waterfall-en-stepped-cpp',this)">C++</button>
</div><div id="waterfall-en-stepped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="stepped",
)
chart.show()</code></pre></div><div id="waterfall-en-stepped-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "stepped",
});
chart.show();</code></pre></div><div id="waterfall-en-stepped-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "stepped",
});
chart.show();</code></pre></div><div id="waterfall-en-stepped-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "stepped"
)
chart$show()</code></pre></div><div id="waterfall-en-stepped-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("stepped")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-en-stepped-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("stepped")
    .build();
chart.show();</code></pre></div><div id="waterfall-en-stepped-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "stepped"
);
chart.Show();</code></pre></div><div id="waterfall-en-stepped-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "stepped"
)
chart.show()</code></pre></div><div id="waterfall-en-stepped-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "stepped",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-stepped.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-en-gradient">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / shaded / fade / vgrad</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Vertical linear gradient on each bar (opaque top -> faded bottom) for depth.</p>
<div class="sp-tabs" id="waterfall-en-gradient">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-en-gradient','waterfall-en-gradient-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-en-gradient','waterfall-en-gradient-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-gradient','waterfall-en-gradient-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-gradient','waterfall-en-gradient-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-en-gradient','waterfall-en-gradient-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-en-gradient','waterfall-en-gradient-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-en-gradient','waterfall-en-gradient-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-en-gradient','waterfall-en-gradient-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-en-gradient','waterfall-en-gradient-cpp',this)">C++</button>
</div><div id="waterfall-en-gradient-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="gradient",
)
chart.show()</code></pre></div><div id="waterfall-en-gradient-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "gradient",
});
chart.show();</code></pre></div><div id="waterfall-en-gradient-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "gradient",
});
chart.show();</code></pre></div><div id="waterfall-en-gradient-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "gradient"
)
chart$show()</code></pre></div><div id="waterfall-en-gradient-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("gradient")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-en-gradient-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("gradient")
    .build();
chart.show();</code></pre></div><div id="waterfall-en-gradient-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "gradient"
);
chart.Show();</code></pre></div><div id="waterfall-en-gradient-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "gradient"
)
chart.show()</code></pre></div><div id="waterfall-en-gradient-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "gradient",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-gradient.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-en-rounded">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rounded"</code></span><span><strong>Aliases</strong> <code>rounded / round / pill / soft</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Large corner radius gives bars a soft pill shape, brand-friendly.</p>
<div class="sp-tabs" id="waterfall-en-rounded">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-en-rounded','waterfall-en-rounded-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-en-rounded','waterfall-en-rounded-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-rounded','waterfall-en-rounded-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-rounded','waterfall-en-rounded-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-en-rounded','waterfall-en-rounded-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-en-rounded','waterfall-en-rounded-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-en-rounded','waterfall-en-rounded-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-en-rounded','waterfall-en-rounded-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-en-rounded','waterfall-en-rounded-cpp',this)">C++</button>
</div><div id="waterfall-en-rounded-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="rounded",
)
chart.show()</code></pre></div><div id="waterfall-en-rounded-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "rounded",
});
chart.show();</code></pre></div><div id="waterfall-en-rounded-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "rounded",
});
chart.show();</code></pre></div><div id="waterfall-en-rounded-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "rounded"
)
chart$show()</code></pre></div><div id="waterfall-en-rounded-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("rounded")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-en-rounded-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("rounded")
    .build();
chart.show();</code></pre></div><div id="waterfall-en-rounded-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "rounded"
);
chart.Show();</code></pre></div><div id="waterfall-en-rounded-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "rounded"
)
chart.show()</code></pre></div><div id="waterfall-en-rounded-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "rounded",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-rounded.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-en-lollipop">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"lollipop"</code></span><span><strong>Aliases</strong> <code>lollipop / stick / popsicle / lolly</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stick + circle marker at the end of each step. Excellent ink-to-data ratio.</p>
<div class="sp-tabs" id="waterfall-en-lollipop">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-en-lollipop','waterfall-en-lollipop-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-en-lollipop','waterfall-en-lollipop-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-lollipop','waterfall-en-lollipop-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-lollipop','waterfall-en-lollipop-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-en-lollipop','waterfall-en-lollipop-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-en-lollipop','waterfall-en-lollipop-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-en-lollipop','waterfall-en-lollipop-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-en-lollipop','waterfall-en-lollipop-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-en-lollipop','waterfall-en-lollipop-cpp',this)">C++</button>
</div><div id="waterfall-en-lollipop-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="lollipop",
)
chart.show()</code></pre></div><div id="waterfall-en-lollipop-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "lollipop",
});
chart.show();</code></pre></div><div id="waterfall-en-lollipop-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "lollipop",
});
chart.show();</code></pre></div><div id="waterfall-en-lollipop-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "lollipop"
)
chart$show()</code></pre></div><div id="waterfall-en-lollipop-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("lollipop")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-en-lollipop-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("lollipop")
    .build();
chart.show();</code></pre></div><div id="waterfall-en-lollipop-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "lollipop"
);
chart.Show();</code></pre></div><div id="waterfall-en-lollipop-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "lollipop"
)
chart.show()</code></pre></div><div id="waterfall-en-lollipop-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "lollipop",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-lollipop.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-en-arrowed">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arrowed"</code></span><span><strong>Aliases</strong> <code>arrowed / arrow / directional / tipped</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Triangle on top (positives) or bottom (negatives) emphasizes direction at a glance.</p>
<div class="sp-tabs" id="waterfall-en-arrowed">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-en-arrowed','waterfall-en-arrowed-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-en-arrowed','waterfall-en-arrowed-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-arrowed','waterfall-en-arrowed-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-arrowed','waterfall-en-arrowed-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-en-arrowed','waterfall-en-arrowed-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-en-arrowed','waterfall-en-arrowed-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-en-arrowed','waterfall-en-arrowed-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-en-arrowed','waterfall-en-arrowed-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-en-arrowed','waterfall-en-arrowed-cpp',this)">C++</button>
</div><div id="waterfall-en-arrowed-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="arrowed",
)
chart.show()</code></pre></div><div id="waterfall-en-arrowed-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "arrowed",
});
chart.show();</code></pre></div><div id="waterfall-en-arrowed-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "arrowed",
});
chart.show();</code></pre></div><div id="waterfall-en-arrowed-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "arrowed"
)
chart$show()</code></pre></div><div id="waterfall-en-arrowed-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("arrowed")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-en-arrowed-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("arrowed")
    .build();
chart.show();</code></pre></div><div id="waterfall-en-arrowed-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "arrowed"
);
chart.Show();</code></pre></div><div id="waterfall-en-arrowed-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "arrowed"
)
chart.show()</code></pre></div><div id="waterfall-en-arrowed-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "arrowed",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-arrowed.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-en-delta">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"delta"</code></span><span><strong>Aliases</strong> <code>delta / percent / annotated / pct</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bars + signed percentage badge (Delta vs previous running total) above each step.</p>
<div class="sp-tabs" id="waterfall-en-delta">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-en-delta','waterfall-en-delta-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-en-delta','waterfall-en-delta-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-delta','waterfall-en-delta-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-delta','waterfall-en-delta-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-en-delta','waterfall-en-delta-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-en-delta','waterfall-en-delta-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-en-delta','waterfall-en-delta-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-en-delta','waterfall-en-delta-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-en-delta','waterfall-en-delta-cpp',this)">C++</button>
</div><div id="waterfall-en-delta-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="delta",
)
chart.show()</code></pre></div><div id="waterfall-en-delta-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "delta",
});
chart.show();</code></pre></div><div id="waterfall-en-delta-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "delta",
});
chart.show();</code></pre></div><div id="waterfall-en-delta-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "delta"
)
chart$show()</code></pre></div><div id="waterfall-en-delta-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("delta")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-en-delta-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("delta")
    .build();
chart.show();</code></pre></div><div id="waterfall-en-delta-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "delta"
);
chart.Show();</code></pre></div><div id="waterfall-en-delta-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "delta"
)
chart.show()</code></pre></div><div id="waterfall-en-delta-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "delta",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-delta.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-en-outlined">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / wireframe</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stroke-only bars with translucent fill, perfect for slide overlays and minimalist reports.</p>
<div class="sp-tabs" id="waterfall-en-outlined">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-en-outlined','waterfall-en-outlined-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-en-outlined','waterfall-en-outlined-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-outlined','waterfall-en-outlined-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-en-outlined','waterfall-en-outlined-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-en-outlined','waterfall-en-outlined-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-en-outlined','waterfall-en-outlined-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-en-outlined','waterfall-en-outlined-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-en-outlined','waterfall-en-outlined-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-en-outlined','waterfall-en-outlined-cpp',this)">C++</button>
</div><div id="waterfall-en-outlined-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="outlined",
)
chart.show()</code></pre></div><div id="waterfall-en-outlined-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "outlined",
});
chart.show();</code></pre></div><div id="waterfall-en-outlined-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "outlined",
});
chart.show();</code></pre></div><div id="waterfall-en-outlined-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "outlined"
)
chart$show()</code></pre></div><div id="waterfall-en-outlined-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("outlined")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-en-outlined-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("outlined")
    .build();
chart.show();</code></pre></div><div id="waterfall-en-outlined-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "outlined"
);
chart.Show();</code></pre></div><div id="waterfall-en-outlined-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "outlined"
)
chart.show()</code></pre></div><div id="waterfall-en-outlined-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "outlined",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-outlined.html"></iframe>
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

`sp.waterfall(title, labels, values, *, variant="basic", show_text=True, **kwargs) -> Chart`

Alias : `sp.waterfall`, `sp.build_waterfall`

## Description

`sp.waterfall()` rassemble toute la famille des graphiques waterfall : une suite de barres ou chaque etape ajoute (positif) ou retranche (negatif) au cumul courant. Le mot-cle `variant` change la geometrie sans toucher aux autres parametres. Les waterfalls sont la reference pour les ponts de P&L, l analyse d ecarts, la decomposition de cohortes, le detail des frais/taxes et tout recit du type "de A vers B, qu est-ce qui a change ?".

> **Totaux** — mettez la valeur a `0` et utilisez un libelle contenant `total`, `net`, `final`, `gross` ou `ebitda` pour marquer une barre de sous-total ; elle est rendue avec la couleur des totaux et ancree sur le cumul courant.

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / bars` | Barres classiques a somme cumulee avec connecteurs pointilles entre etapes. |
| `"stepped"` | `stepped / step / staircase / stairs` | Barres jointives formant un escalier continu, sans connecteur. |
| `"gradient"` | `gradient / shaded / fade / vgrad` | Degrade vertical sur chaque barre (opaque haut -> attenue bas) pour le relief. |
| `"rounded"` | `rounded / round / pill / soft` | Grand rayon de coin pour des barres en forme de pilule, soft et brandable. |
| `"lollipop"` | `lollipop / stick / popsicle / lolly` | Tige + cercle en fin de chaque etape. Excellent ratio encre/donnee. |
| `"arrowed"` | `arrowed / arrow / directional / tipped` | Triangle dessus (positifs) ou dessous (negatifs) pour souligner la direction en un coup d oeil. |
| `"delta"` | `delta / percent / annotated / pct` | Barres + badge de pourcentage signe (Delta vs cumul precedent) au-dessus de chaque etape. |
| `"outlined"` | `outlined / outline / stroke / wireframe` | Barres uniquement en contour avec fond translucide, ideal pour overlays et rapports minimalistes. |

## Parametres

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title`     | `str`         | requis    | Titre du graphique |
| `labels`    | `list[str]`   | requis    | Libelles des etapes (gauche -> droite) |
| `values`    | `list[float]` | requis    | Deltas par etape (mettez 0 + libelle "total" pour les sous-totaux) |
| `variant`   | `str`         | `"basic"` | Variante geometrique (voir tableau) |
| `show_text` | `bool`        | `True`    | Afficher les annotations de valeur au-dessus des barres |
| `width`     | `int`         | `900`     | Largeur du canvas (px) |
| `height`    | `int`         | `480`     | Hauteur du canvas (px) |

## Retour

`Chart` — objet avec une propriete `.html` et une methode `.show()`.

---

<div class="sp-cls sp-open" id="waterfall-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('waterfall-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('waterfall-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-fr','stepped',this)"><span class="sp-cic">S</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-fr','gradient',this)"><span class="sp-cic">G</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-fr','rounded',this)"><span class="sp-cic">R</span><span class="sp-clb">Rounded</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-fr','lollipop',this)"><span class="sp-cic">L</span><span class="sp-clb">Lollipop</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-fr','arrowed',this)"><span class="sp-cic">A</span><span class="sp-clb">Arrowed</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-fr','delta',this)"><span class="sp-cic">D</span><span class="sp-clb">Delta</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-fr','outlined',this)"><span class="sp-cic">O</span><span class="sp-clb">Outlined</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="waterfall-fr-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / bars</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres classiques a somme cumulee avec connecteurs pointilles entre etapes.</p>
<div class="sp-tabs" id="waterfall-fr-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-fr-basic','waterfall-fr-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-basic','waterfall-fr-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-basic','waterfall-fr-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-basic','waterfall-fr-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-basic','waterfall-fr-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-basic','waterfall-fr-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-basic','waterfall-fr-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-basic','waterfall-fr-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-basic','waterfall-fr-basic-cpp',this)">C++</button>
</div><div id="waterfall-fr-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="basic",
)
chart.show()</code></pre></div><div id="waterfall-fr-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="waterfall-fr-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="waterfall-fr-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="waterfall-fr-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-fr-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="waterfall-fr-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="waterfall-fr-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="waterfall-fr-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-basic.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-fr-stepped">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>stepped / step / staircase / stairs</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres jointives formant un escalier continu, sans connecteur.</p>
<div class="sp-tabs" id="waterfall-fr-stepped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-fr-stepped','waterfall-fr-stepped-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-stepped','waterfall-fr-stepped-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-stepped','waterfall-fr-stepped-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-stepped','waterfall-fr-stepped-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-stepped','waterfall-fr-stepped-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-stepped','waterfall-fr-stepped-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-stepped','waterfall-fr-stepped-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-stepped','waterfall-fr-stepped-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-stepped','waterfall-fr-stepped-cpp',this)">C++</button>
</div><div id="waterfall-fr-stepped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="stepped",
)
chart.show()</code></pre></div><div id="waterfall-fr-stepped-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "stepped",
});
chart.show();</code></pre></div><div id="waterfall-fr-stepped-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "stepped",
});
chart.show();</code></pre></div><div id="waterfall-fr-stepped-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "stepped"
)
chart$show()</code></pre></div><div id="waterfall-fr-stepped-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("stepped")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-fr-stepped-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("stepped")
    .build();
chart.show();</code></pre></div><div id="waterfall-fr-stepped-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "stepped"
);
chart.Show();</code></pre></div><div id="waterfall-fr-stepped-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "stepped"
)
chart.show()</code></pre></div><div id="waterfall-fr-stepped-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "stepped",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-stepped.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-fr-gradient">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / shaded / fade / vgrad</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Degrade vertical sur chaque barre (opaque haut -> attenue bas) pour le relief.</p>
<div class="sp-tabs" id="waterfall-fr-gradient">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-fr-gradient','waterfall-fr-gradient-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-gradient','waterfall-fr-gradient-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-gradient','waterfall-fr-gradient-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-gradient','waterfall-fr-gradient-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-gradient','waterfall-fr-gradient-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-gradient','waterfall-fr-gradient-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-gradient','waterfall-fr-gradient-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-gradient','waterfall-fr-gradient-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-gradient','waterfall-fr-gradient-cpp',this)">C++</button>
</div><div id="waterfall-fr-gradient-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="gradient",
)
chart.show()</code></pre></div><div id="waterfall-fr-gradient-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "gradient",
});
chart.show();</code></pre></div><div id="waterfall-fr-gradient-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "gradient",
});
chart.show();</code></pre></div><div id="waterfall-fr-gradient-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "gradient"
)
chart$show()</code></pre></div><div id="waterfall-fr-gradient-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("gradient")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-fr-gradient-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("gradient")
    .build();
chart.show();</code></pre></div><div id="waterfall-fr-gradient-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "gradient"
);
chart.Show();</code></pre></div><div id="waterfall-fr-gradient-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "gradient"
)
chart.show()</code></pre></div><div id="waterfall-fr-gradient-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "gradient",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-gradient.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-fr-rounded">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rounded"</code></span><span><strong>Aliases</strong> <code>rounded / round / pill / soft</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Grand rayon de coin pour des barres en forme de pilule, soft et brandable.</p>
<div class="sp-tabs" id="waterfall-fr-rounded">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-fr-rounded','waterfall-fr-rounded-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-rounded','waterfall-fr-rounded-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-rounded','waterfall-fr-rounded-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-rounded','waterfall-fr-rounded-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-rounded','waterfall-fr-rounded-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-rounded','waterfall-fr-rounded-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-rounded','waterfall-fr-rounded-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-rounded','waterfall-fr-rounded-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-rounded','waterfall-fr-rounded-cpp',this)">C++</button>
</div><div id="waterfall-fr-rounded-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="rounded",
)
chart.show()</code></pre></div><div id="waterfall-fr-rounded-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "rounded",
});
chart.show();</code></pre></div><div id="waterfall-fr-rounded-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "rounded",
});
chart.show();</code></pre></div><div id="waterfall-fr-rounded-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "rounded"
)
chart$show()</code></pre></div><div id="waterfall-fr-rounded-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("rounded")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-fr-rounded-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("rounded")
    .build();
chart.show();</code></pre></div><div id="waterfall-fr-rounded-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "rounded"
);
chart.Show();</code></pre></div><div id="waterfall-fr-rounded-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "rounded"
)
chart.show()</code></pre></div><div id="waterfall-fr-rounded-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "rounded",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-rounded.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-fr-lollipop">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"lollipop"</code></span><span><strong>Aliases</strong> <code>lollipop / stick / popsicle / lolly</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Tige + cercle en fin de chaque etape. Excellent ratio encre/donnee.</p>
<div class="sp-tabs" id="waterfall-fr-lollipop">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-fr-lollipop','waterfall-fr-lollipop-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-lollipop','waterfall-fr-lollipop-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-lollipop','waterfall-fr-lollipop-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-lollipop','waterfall-fr-lollipop-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-lollipop','waterfall-fr-lollipop-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-lollipop','waterfall-fr-lollipop-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-lollipop','waterfall-fr-lollipop-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-lollipop','waterfall-fr-lollipop-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-lollipop','waterfall-fr-lollipop-cpp',this)">C++</button>
</div><div id="waterfall-fr-lollipop-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="lollipop",
)
chart.show()</code></pre></div><div id="waterfall-fr-lollipop-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "lollipop",
});
chart.show();</code></pre></div><div id="waterfall-fr-lollipop-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "lollipop",
});
chart.show();</code></pre></div><div id="waterfall-fr-lollipop-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "lollipop"
)
chart$show()</code></pre></div><div id="waterfall-fr-lollipop-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("lollipop")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-fr-lollipop-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("lollipop")
    .build();
chart.show();</code></pre></div><div id="waterfall-fr-lollipop-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "lollipop"
);
chart.Show();</code></pre></div><div id="waterfall-fr-lollipop-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "lollipop"
)
chart.show()</code></pre></div><div id="waterfall-fr-lollipop-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "lollipop",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-lollipop.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-fr-arrowed">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arrowed"</code></span><span><strong>Aliases</strong> <code>arrowed / arrow / directional / tipped</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Triangle dessus (positifs) ou dessous (negatifs) pour souligner la direction en un coup d oeil.</p>
<div class="sp-tabs" id="waterfall-fr-arrowed">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-fr-arrowed','waterfall-fr-arrowed-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-arrowed','waterfall-fr-arrowed-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-arrowed','waterfall-fr-arrowed-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-arrowed','waterfall-fr-arrowed-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-arrowed','waterfall-fr-arrowed-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-arrowed','waterfall-fr-arrowed-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-arrowed','waterfall-fr-arrowed-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-arrowed','waterfall-fr-arrowed-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-arrowed','waterfall-fr-arrowed-cpp',this)">C++</button>
</div><div id="waterfall-fr-arrowed-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="arrowed",
)
chart.show()</code></pre></div><div id="waterfall-fr-arrowed-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "arrowed",
});
chart.show();</code></pre></div><div id="waterfall-fr-arrowed-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "arrowed",
});
chart.show();</code></pre></div><div id="waterfall-fr-arrowed-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "arrowed"
)
chart$show()</code></pre></div><div id="waterfall-fr-arrowed-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("arrowed")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-fr-arrowed-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("arrowed")
    .build();
chart.show();</code></pre></div><div id="waterfall-fr-arrowed-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "arrowed"
);
chart.Show();</code></pre></div><div id="waterfall-fr-arrowed-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "arrowed"
)
chart.show()</code></pre></div><div id="waterfall-fr-arrowed-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "arrowed",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-arrowed.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-fr-delta">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"delta"</code></span><span><strong>Aliases</strong> <code>delta / percent / annotated / pct</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres + badge de pourcentage signe (Delta vs cumul precedent) au-dessus de chaque etape.</p>
<div class="sp-tabs" id="waterfall-fr-delta">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-fr-delta','waterfall-fr-delta-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-delta','waterfall-fr-delta-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-delta','waterfall-fr-delta-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-delta','waterfall-fr-delta-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-delta','waterfall-fr-delta-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-delta','waterfall-fr-delta-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-delta','waterfall-fr-delta-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-delta','waterfall-fr-delta-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-delta','waterfall-fr-delta-cpp',this)">C++</button>
</div><div id="waterfall-fr-delta-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="delta",
)
chart.show()</code></pre></div><div id="waterfall-fr-delta-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "delta",
});
chart.show();</code></pre></div><div id="waterfall-fr-delta-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "delta",
});
chart.show();</code></pre></div><div id="waterfall-fr-delta-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "delta"
)
chart$show()</code></pre></div><div id="waterfall-fr-delta-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("delta")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-fr-delta-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("delta")
    .build();
chart.show();</code></pre></div><div id="waterfall-fr-delta-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "delta"
);
chart.Show();</code></pre></div><div id="waterfall-fr-delta-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "delta"
)
chart.show()</code></pre></div><div id="waterfall-fr-delta-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "delta",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-delta.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-fr-outlined">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / wireframe</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres uniquement en contour avec fond translucide, ideal pour overlays et rapports minimalistes.</p>
<div class="sp-tabs" id="waterfall-fr-outlined">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-fr-outlined','waterfall-fr-outlined-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-outlined','waterfall-fr-outlined-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-outlined','waterfall-fr-outlined-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-outlined','waterfall-fr-outlined-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-outlined','waterfall-fr-outlined-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-outlined','waterfall-fr-outlined-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-outlined','waterfall-fr-outlined-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-outlined','waterfall-fr-outlined-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-fr-outlined','waterfall-fr-outlined-cpp',this)">C++</button>
</div><div id="waterfall-fr-outlined-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["Revenue", "COGS", "Gross", "OpEx", "EBITDA", "D&amp;A", "EBIT", "Tax", "Net Income"]
values = [180.0, -65.0, 0.0, -42.0, 0.0, -12.0, 0.0, -18.0, 0.0]

chart = sp.waterfall(
    title="P&amp;L Bridge (M$)",
    labels=labels,
    values=values,
    variant="outlined",
)
chart.show()</code></pre></div><div id="waterfall-fr-outlined-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "outlined",
});
chart.show();</code></pre></div><div id="waterfall-fr-outlined-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.waterfall({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "outlined",
});
chart.show();</code></pre></div><div id="waterfall-fr-outlined-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$waterfall(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "outlined"
)
chart$show()</code></pre></div><div id="waterfall-fr-outlined-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::waterfall()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("outlined")
        .build();
    chart.show();
}</code></pre></div><div id="waterfall-fr-outlined-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("outlined")
    .build();
chart.show();</code></pre></div><div id="waterfall-fr-outlined-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "outlined"
);
chart.Show();</code></pre></div><div id="waterfall-fr-outlined-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.waterfall(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "outlined"
)
chart.show()</code></pre></div><div id="waterfall-fr-outlined-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "outlined",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-outlined.html"></iframe>
</div>

</div>
</div>

</div>
