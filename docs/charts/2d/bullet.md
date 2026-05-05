# Bullet - Compact KPI vs Target

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
.sp-preview-frame{width:100%;height:360px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.bullet(title, labels, values, *, targets=None, max_vals=None, ranges=None, comparisons=None, variant="basic", **kwargs) -> Chart`

Aliases: `sp.bullet`, `sp.build_bullet`

## Description

`sp.bullet()` is the unified entry point for the bullet-chart family. Inspired by Edward Tufte, a bullet packs an actual value, a target, qualitative ranges and a scale into a single horizontal row - perfect for KPI dashboards where space is precious. The `variant` keyword switches the visual treatment (zones, traffic light, thermometer, progress pill, dot, ghost-bar comparison) without touching the data.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / standard` | Classic Tufte bullet: track + qualitative range + value bar + target tick. |
| `"stacked"` | `stacked / stacked_ranges / zones / qualitative` | Three graduated qualitative bands (poor / satisfactory / good) drawn behind the value bar. |
| `"thermo"` | `thermo / thermometer / vertical / column` | Vertical thermometer style with a bulb base - dramatic for KPIs in a row. |
| `"segmented"` | `segmented / traffic / rag / zones_color` | Traffic-light segmented track (red / amber / green) for status dashboards. |
| `"minimal"` | `minimal / sparkline / clean / naked` | Sparkline-thin pill bar with target tick only - ultra-clean inline indicator. |
| `"dot"` | `dot / point / marker / pip` | Single dot on a track instead of a bar - dot-plot variant of the bullet. |
| `"progress"` | `progress / pill / bar / percent` | Pill-shape gradient progress bar with a percentage label centered inside. |
| `"compare"` | `compare / vs / ghost / prior` | Adds a ghost bar (e.g. previous period via comparisons) behind the current value. |

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Row labels |
| `values` | `list[float]` | required | Actual values |
| `variant` | `str` | `"basic"` | Visual style (see table) |
| `targets` | `list[float]` | `None` | Target tick value per row |
| `max_vals` | `list[float]` | `None` | Per-row scale maximum (auto if 0) |
| `ranges` | `list[float]` | `None` | Qualitative range threshold per row |
| `comparisons` | `list[float]` | `None` | Comparison values for `compare` variant |
| `width` | `int` | `800` | Canvas width (px) |
| `height` | `int` | `300` | Canvas height (px, auto-grows with rows) |

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="bullet-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bullet-en')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bullet-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','thermo',this)"><span class="sp-cic">T</span><span class="sp-clb">Thermo</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','segmented',this)"><span class="sp-cic">Z</span><span class="sp-clb">Segmented</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','minimal',this)"><span class="sp-cic">M</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','dot',this)"><span class="sp-cic">D</span><span class="sp-clb">Dot</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','progress',this)"><span class="sp-cic">P</span><span class="sp-clb">Progress</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','compare',this)"><span class="sp-cic">C</span><span class="sp-clb">Compare</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="bullet-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / standard</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic Tufte bullet: track + qualitative range + value bar + target tick.</p>
<div class="sp-tabs" id="bullet-en-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-en-basic','bullet-en-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-en-basic','bullet-en-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-basic','bullet-en-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-basic','bullet-en-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-en-basic','bullet-en-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-en-basic','bullet-en-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-en-basic','bullet-en-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-en-basic','bullet-en-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-en-basic','bullet-en-basic-cpp',this)">C++</button>
</div><div id="bullet-en-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="basic",
)
chart.show()</code></pre></div><div id="bullet-en-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="bullet-en-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="bullet-en-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="bullet-en-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-en-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="bullet-en-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="bullet-en-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="bullet-en-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-basic.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-stacked">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stacked_ranges / zones / qualitative</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Three graduated qualitative bands (poor / satisfactory / good) drawn behind the value bar.</p>
<div class="sp-tabs" id="bullet-en-stacked">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-en-stacked','bullet-en-stacked-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-en-stacked','bullet-en-stacked-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-stacked','bullet-en-stacked-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-stacked','bullet-en-stacked-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-en-stacked','bullet-en-stacked-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-en-stacked','bullet-en-stacked-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-en-stacked','bullet-en-stacked-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-en-stacked','bullet-en-stacked-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-en-stacked','bullet-en-stacked-cpp',this)">C++</button>
</div><div id="bullet-en-stacked-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="stacked",
)
chart.show()</code></pre></div><div id="bullet-en-stacked-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "stacked",
});
chart.show();</code></pre></div><div id="bullet-en-stacked-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "stacked",
});
chart.show();</code></pre></div><div id="bullet-en-stacked-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "stacked"
)
chart$show()</code></pre></div><div id="bullet-en-stacked-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("stacked")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-en-stacked-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("stacked")
    .build();
chart.show();</code></pre></div><div id="bullet-en-stacked-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "stacked"
);
chart.Show();</code></pre></div><div id="bullet-en-stacked-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "stacked"
)
chart.show()</code></pre></div><div id="bullet-en-stacked-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "stacked",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-stacked.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-thermo">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"thermo"</code></span><span><strong>Aliases</strong> <code>thermo / thermometer / vertical / column</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Vertical thermometer style with a bulb base - dramatic for KPIs in a row.</p>
<div class="sp-tabs" id="bullet-en-thermo">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-en-thermo','bullet-en-thermo-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-en-thermo','bullet-en-thermo-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-thermo','bullet-en-thermo-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-thermo','bullet-en-thermo-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-en-thermo','bullet-en-thermo-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-en-thermo','bullet-en-thermo-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-en-thermo','bullet-en-thermo-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-en-thermo','bullet-en-thermo-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-en-thermo','bullet-en-thermo-cpp',this)">C++</button>
</div><div id="bullet-en-thermo-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="thermo",
)
chart.show()</code></pre></div><div id="bullet-en-thermo-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "thermo",
});
chart.show();</code></pre></div><div id="bullet-en-thermo-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "thermo",
});
chart.show();</code></pre></div><div id="bullet-en-thermo-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "thermo"
)
chart$show()</code></pre></div><div id="bullet-en-thermo-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("thermo")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-en-thermo-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("thermo")
    .build();
chart.show();</code></pre></div><div id="bullet-en-thermo-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "thermo"
);
chart.Show();</code></pre></div><div id="bullet-en-thermo-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "thermo"
)
chart.show()</code></pre></div><div id="bullet-en-thermo-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "thermo",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-thermo.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-segmented">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"segmented"</code></span><span><strong>Aliases</strong> <code>segmented / traffic / rag / zones_color</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Traffic-light segmented track (red / amber / green) for status dashboards.</p>
<div class="sp-tabs" id="bullet-en-segmented">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-en-segmented','bullet-en-segmented-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-en-segmented','bullet-en-segmented-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-segmented','bullet-en-segmented-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-segmented','bullet-en-segmented-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-en-segmented','bullet-en-segmented-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-en-segmented','bullet-en-segmented-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-en-segmented','bullet-en-segmented-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-en-segmented','bullet-en-segmented-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-en-segmented','bullet-en-segmented-cpp',this)">C++</button>
</div><div id="bullet-en-segmented-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="segmented",
)
chart.show()</code></pre></div><div id="bullet-en-segmented-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "segmented",
});
chart.show();</code></pre></div><div id="bullet-en-segmented-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "segmented",
});
chart.show();</code></pre></div><div id="bullet-en-segmented-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "segmented"
)
chart$show()</code></pre></div><div id="bullet-en-segmented-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("segmented")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-en-segmented-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("segmented")
    .build();
chart.show();</code></pre></div><div id="bullet-en-segmented-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "segmented"
);
chart.Show();</code></pre></div><div id="bullet-en-segmented-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "segmented"
)
chart.show()</code></pre></div><div id="bullet-en-segmented-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "segmented",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-segmented.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-minimal">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"minimal"</code></span><span><strong>Aliases</strong> <code>minimal / sparkline / clean / naked</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Sparkline-thin pill bar with target tick only - ultra-clean inline indicator.</p>
<div class="sp-tabs" id="bullet-en-minimal">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-en-minimal','bullet-en-minimal-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-en-minimal','bullet-en-minimal-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-minimal','bullet-en-minimal-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-minimal','bullet-en-minimal-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-en-minimal','bullet-en-minimal-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-en-minimal','bullet-en-minimal-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-en-minimal','bullet-en-minimal-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-en-minimal','bullet-en-minimal-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-en-minimal','bullet-en-minimal-cpp',this)">C++</button>
</div><div id="bullet-en-minimal-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="minimal",
)
chart.show()</code></pre></div><div id="bullet-en-minimal-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "minimal",
});
chart.show();</code></pre></div><div id="bullet-en-minimal-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "minimal",
});
chart.show();</code></pre></div><div id="bullet-en-minimal-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "minimal"
)
chart$show()</code></pre></div><div id="bullet-en-minimal-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("minimal")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-en-minimal-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("minimal")
    .build();
chart.show();</code></pre></div><div id="bullet-en-minimal-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "minimal"
);
chart.Show();</code></pre></div><div id="bullet-en-minimal-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "minimal"
)
chart.show()</code></pre></div><div id="bullet-en-minimal-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "minimal",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-minimal.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-dot">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dot"</code></span><span><strong>Aliases</strong> <code>dot / point / marker / pip</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Single dot on a track instead of a bar - dot-plot variant of the bullet.</p>
<div class="sp-tabs" id="bullet-en-dot">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-en-dot','bullet-en-dot-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-en-dot','bullet-en-dot-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-dot','bullet-en-dot-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-dot','bullet-en-dot-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-en-dot','bullet-en-dot-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-en-dot','bullet-en-dot-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-en-dot','bullet-en-dot-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-en-dot','bullet-en-dot-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-en-dot','bullet-en-dot-cpp',this)">C++</button>
</div><div id="bullet-en-dot-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="dot",
)
chart.show()</code></pre></div><div id="bullet-en-dot-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "dot",
});
chart.show();</code></pre></div><div id="bullet-en-dot-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "dot",
});
chart.show();</code></pre></div><div id="bullet-en-dot-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "dot"
)
chart$show()</code></pre></div><div id="bullet-en-dot-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("dot")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-en-dot-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("dot")
    .build();
chart.show();</code></pre></div><div id="bullet-en-dot-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "dot"
);
chart.Show();</code></pre></div><div id="bullet-en-dot-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "dot"
)
chart.show()</code></pre></div><div id="bullet-en-dot-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "dot",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-dot.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-progress">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"progress"</code></span><span><strong>Aliases</strong> <code>progress / pill / bar / percent</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Pill-shape gradient progress bar with a percentage label centered inside.</p>
<div class="sp-tabs" id="bullet-en-progress">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-en-progress','bullet-en-progress-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-en-progress','bullet-en-progress-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-progress','bullet-en-progress-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-progress','bullet-en-progress-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-en-progress','bullet-en-progress-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-en-progress','bullet-en-progress-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-en-progress','bullet-en-progress-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-en-progress','bullet-en-progress-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-en-progress','bullet-en-progress-cpp',this)">C++</button>
</div><div id="bullet-en-progress-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="progress",
)
chart.show()</code></pre></div><div id="bullet-en-progress-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "progress",
});
chart.show();</code></pre></div><div id="bullet-en-progress-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "progress",
});
chart.show();</code></pre></div><div id="bullet-en-progress-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "progress"
)
chart$show()</code></pre></div><div id="bullet-en-progress-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("progress")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-en-progress-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("progress")
    .build();
chart.show();</code></pre></div><div id="bullet-en-progress-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "progress"
);
chart.Show();</code></pre></div><div id="bullet-en-progress-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "progress"
)
chart.show()</code></pre></div><div id="bullet-en-progress-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "progress",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-progress.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-compare">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"compare"</code></span><span><strong>Aliases</strong> <code>compare / vs / ghost / prior</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Adds a ghost bar (e.g. previous period via comparisons) behind the current value.</p>
<div class="sp-tabs" id="bullet-en-compare">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-en-compare','bullet-en-compare-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-en-compare','bullet-en-compare-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-compare','bullet-en-compare-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-en-compare','bullet-en-compare-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-en-compare','bullet-en-compare-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-en-compare','bullet-en-compare-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-en-compare','bullet-en-compare-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-en-compare','bullet-en-compare-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-en-compare','bullet-en-compare-cpp',this)">C++</button>
</div><div id="bullet-en-compare-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="compare",
)
chart.show()</code></pre></div><div id="bullet-en-compare-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "compare",
});
chart.show();</code></pre></div><div id="bullet-en-compare-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "compare",
});
chart.show();</code></pre></div><div id="bullet-en-compare-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "compare"
)
chart$show()</code></pre></div><div id="bullet-en-compare-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("compare")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-en-compare-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("compare")
    .build();
chart.show();</code></pre></div><div id="bullet-en-compare-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "compare"
);
chart.Show();</code></pre></div><div id="bullet-en-compare-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "compare"
)
chart.show()</code></pre></div><div id="bullet-en-compare-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "compare",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-compare.html"></iframe>
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
.sp-preview-frame{width:100%;height:360px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.bullet(title, labels, values, *, targets=None, max_vals=None, ranges=None, comparisons=None, variant="basic", **kwargs) -> Chart`

Aliases: `sp.bullet`, `sp.build_bullet`

## Description

`sp.bullet()` est le point d entree unique pour la famille bullet. Inspire par Edward Tufte, le bullet condense valeur, cible, zones qualitatives et echelle dans une seule ligne horizontale - parfait pour des dashboards KPIs serres. Le mot-cle `variant` change l aspect (zones, feu tricolore, thermometre, pillule de progression, point, comparaison par barre fantome) sans toucher aux donnees.

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / standard` | Bullet de Tufte classique : piste + zone qualitative + barre de valeur + tick de cible. |
| `"stacked"` | `stacked / stacked_ranges / zones / qualitative` | Trois bandes qualitatives graduees (faible / correct / bon) derriere la barre de valeur. |
| `"thermo"` | `thermo / thermometer / vertical / column` | Style thermometre vertical avec bulbe - tres parlant pour des KPIs alignes. |
| `"segmented"` | `segmented / traffic / rag / zones_color` | Piste segmentee feu tricolore (rouge / orange / vert) pour tableaux de bord. |
| `"minimal"` | `minimal / sparkline / clean / naked` | Barre pillule fine type sparkline avec uniquement le tick cible - indicateur inline epure. |
| `"dot"` | `dot / point / marker / pip` | Un seul point sur la piste au lieu d une barre - variante dot-plot du bullet. |
| `"progress"` | `progress / pill / bar / percent` | Barre de progression pillule en degrade avec pourcentage centre. |
| `"compare"` | `compare / vs / ghost / prior` | Ajoute une barre fantome (par ex. periode precedente via comparisons) derriere la valeur courante. |

## Parametres

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Libelles de ligne |
| `values` | `list[float]` | requis | Valeurs reelles |
| `variant` | `str` | `"basic"` | Style visuel (voir tableau) |
| `targets` | `list[float]` | `None` | Valeur cible par ligne |
| `max_vals` | `list[float]` | `None` | Maximum d echelle par ligne (auto si 0) |
| `ranges` | `list[float]` | `None` | Seuil de zone qualitative par ligne |
| `comparisons` | `list[float]` | `None` | Valeurs de comparaison pour la variante `compare` |
| `width` | `int` | `800` | Largeur (px) |
| `height` | `int` | `300` | Hauteur (px, auto si trop petite) |

## Retour

`Chart` - objet avec propriete `.html` et methode `.show()`.

---

<div class="sp-cls sp-open" id="bullet-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bullet-fr')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bullet-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','thermo',this)"><span class="sp-cic">T</span><span class="sp-clb">Thermo</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','segmented',this)"><span class="sp-cic">Z</span><span class="sp-clb">Segmented</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','minimal',this)"><span class="sp-cic">M</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','dot',this)"><span class="sp-cic">D</span><span class="sp-clb">Dot</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','progress',this)"><span class="sp-cic">P</span><span class="sp-clb">Progress</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','compare',this)"><span class="sp-cic">C</span><span class="sp-clb">Compare</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="bullet-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / standard</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bullet de Tufte classique : piste + zone qualitative + barre de valeur + tick de cible.</p>
<div class="sp-tabs" id="bullet-fr-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-fr-basic','bullet-fr-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-fr-basic','bullet-fr-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-basic','bullet-fr-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-basic','bullet-fr-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-fr-basic','bullet-fr-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-fr-basic','bullet-fr-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-fr-basic','bullet-fr-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-fr-basic','bullet-fr-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-fr-basic','bullet-fr-basic-cpp',this)">C++</button>
</div><div id="bullet-fr-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="basic",
)
chart.show()</code></pre></div><div id="bullet-fr-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="bullet-fr-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="bullet-fr-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="bullet-fr-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-fr-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="bullet-fr-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="bullet-fr-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="bullet-fr-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-basic.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-stacked">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stacked_ranges / zones / qualitative</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trois bandes qualitatives graduees (faible / correct / bon) derriere la barre de valeur.</p>
<div class="sp-tabs" id="bullet-fr-stacked">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-fr-stacked','bullet-fr-stacked-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-fr-stacked','bullet-fr-stacked-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-stacked','bullet-fr-stacked-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-stacked','bullet-fr-stacked-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-fr-stacked','bullet-fr-stacked-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-fr-stacked','bullet-fr-stacked-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-fr-stacked','bullet-fr-stacked-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-fr-stacked','bullet-fr-stacked-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-fr-stacked','bullet-fr-stacked-cpp',this)">C++</button>
</div><div id="bullet-fr-stacked-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="stacked",
)
chart.show()</code></pre></div><div id="bullet-fr-stacked-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "stacked",
});
chart.show();</code></pre></div><div id="bullet-fr-stacked-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "stacked",
});
chart.show();</code></pre></div><div id="bullet-fr-stacked-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "stacked"
)
chart$show()</code></pre></div><div id="bullet-fr-stacked-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("stacked")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-fr-stacked-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("stacked")
    .build();
chart.show();</code></pre></div><div id="bullet-fr-stacked-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "stacked"
);
chart.Show();</code></pre></div><div id="bullet-fr-stacked-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "stacked"
)
chart.show()</code></pre></div><div id="bullet-fr-stacked-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "stacked",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-stacked.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-thermo">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"thermo"</code></span><span><strong>Aliases</strong> <code>thermo / thermometer / vertical / column</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Style thermometre vertical avec bulbe - tres parlant pour des KPIs alignes.</p>
<div class="sp-tabs" id="bullet-fr-thermo">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-fr-thermo','bullet-fr-thermo-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-fr-thermo','bullet-fr-thermo-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-thermo','bullet-fr-thermo-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-thermo','bullet-fr-thermo-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-fr-thermo','bullet-fr-thermo-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-fr-thermo','bullet-fr-thermo-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-fr-thermo','bullet-fr-thermo-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-fr-thermo','bullet-fr-thermo-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-fr-thermo','bullet-fr-thermo-cpp',this)">C++</button>
</div><div id="bullet-fr-thermo-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="thermo",
)
chart.show()</code></pre></div><div id="bullet-fr-thermo-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "thermo",
});
chart.show();</code></pre></div><div id="bullet-fr-thermo-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "thermo",
});
chart.show();</code></pre></div><div id="bullet-fr-thermo-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "thermo"
)
chart$show()</code></pre></div><div id="bullet-fr-thermo-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("thermo")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-fr-thermo-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("thermo")
    .build();
chart.show();</code></pre></div><div id="bullet-fr-thermo-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "thermo"
);
chart.Show();</code></pre></div><div id="bullet-fr-thermo-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "thermo"
)
chart.show()</code></pre></div><div id="bullet-fr-thermo-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "thermo",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-thermo.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-segmented">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"segmented"</code></span><span><strong>Aliases</strong> <code>segmented / traffic / rag / zones_color</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Piste segmentee feu tricolore (rouge / orange / vert) pour tableaux de bord.</p>
<div class="sp-tabs" id="bullet-fr-segmented">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-fr-segmented','bullet-fr-segmented-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-fr-segmented','bullet-fr-segmented-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-segmented','bullet-fr-segmented-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-segmented','bullet-fr-segmented-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-fr-segmented','bullet-fr-segmented-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-fr-segmented','bullet-fr-segmented-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-fr-segmented','bullet-fr-segmented-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-fr-segmented','bullet-fr-segmented-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-fr-segmented','bullet-fr-segmented-cpp',this)">C++</button>
</div><div id="bullet-fr-segmented-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="segmented",
)
chart.show()</code></pre></div><div id="bullet-fr-segmented-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "segmented",
});
chart.show();</code></pre></div><div id="bullet-fr-segmented-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "segmented",
});
chart.show();</code></pre></div><div id="bullet-fr-segmented-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "segmented"
)
chart$show()</code></pre></div><div id="bullet-fr-segmented-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("segmented")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-fr-segmented-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("segmented")
    .build();
chart.show();</code></pre></div><div id="bullet-fr-segmented-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "segmented"
);
chart.Show();</code></pre></div><div id="bullet-fr-segmented-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "segmented"
)
chart.show()</code></pre></div><div id="bullet-fr-segmented-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "segmented",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-segmented.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-minimal">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"minimal"</code></span><span><strong>Aliases</strong> <code>minimal / sparkline / clean / naked</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barre pillule fine type sparkline avec uniquement le tick cible - indicateur inline epure.</p>
<div class="sp-tabs" id="bullet-fr-minimal">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-fr-minimal','bullet-fr-minimal-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-fr-minimal','bullet-fr-minimal-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-minimal','bullet-fr-minimal-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-minimal','bullet-fr-minimal-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-fr-minimal','bullet-fr-minimal-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-fr-minimal','bullet-fr-minimal-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-fr-minimal','bullet-fr-minimal-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-fr-minimal','bullet-fr-minimal-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-fr-minimal','bullet-fr-minimal-cpp',this)">C++</button>
</div><div id="bullet-fr-minimal-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="minimal",
)
chart.show()</code></pre></div><div id="bullet-fr-minimal-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "minimal",
});
chart.show();</code></pre></div><div id="bullet-fr-minimal-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "minimal",
});
chart.show();</code></pre></div><div id="bullet-fr-minimal-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "minimal"
)
chart$show()</code></pre></div><div id="bullet-fr-minimal-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("minimal")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-fr-minimal-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("minimal")
    .build();
chart.show();</code></pre></div><div id="bullet-fr-minimal-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "minimal"
);
chart.Show();</code></pre></div><div id="bullet-fr-minimal-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "minimal"
)
chart.show()</code></pre></div><div id="bullet-fr-minimal-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "minimal",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-minimal.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-dot">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dot"</code></span><span><strong>Aliases</strong> <code>dot / point / marker / pip</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Un seul point sur la piste au lieu d une barre - variante dot-plot du bullet.</p>
<div class="sp-tabs" id="bullet-fr-dot">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-fr-dot','bullet-fr-dot-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-fr-dot','bullet-fr-dot-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-dot','bullet-fr-dot-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-dot','bullet-fr-dot-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-fr-dot','bullet-fr-dot-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-fr-dot','bullet-fr-dot-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-fr-dot','bullet-fr-dot-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-fr-dot','bullet-fr-dot-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-fr-dot','bullet-fr-dot-cpp',this)">C++</button>
</div><div id="bullet-fr-dot-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="dot",
)
chart.show()</code></pre></div><div id="bullet-fr-dot-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "dot",
});
chart.show();</code></pre></div><div id="bullet-fr-dot-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "dot",
});
chart.show();</code></pre></div><div id="bullet-fr-dot-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "dot"
)
chart$show()</code></pre></div><div id="bullet-fr-dot-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("dot")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-fr-dot-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("dot")
    .build();
chart.show();</code></pre></div><div id="bullet-fr-dot-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "dot"
);
chart.Show();</code></pre></div><div id="bullet-fr-dot-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "dot"
)
chart.show()</code></pre></div><div id="bullet-fr-dot-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "dot",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-dot.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-progress">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"progress"</code></span><span><strong>Aliases</strong> <code>progress / pill / bar / percent</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barre de progression pillule en degrade avec pourcentage centre.</p>
<div class="sp-tabs" id="bullet-fr-progress">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-fr-progress','bullet-fr-progress-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-fr-progress','bullet-fr-progress-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-progress','bullet-fr-progress-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-progress','bullet-fr-progress-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-fr-progress','bullet-fr-progress-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-fr-progress','bullet-fr-progress-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-fr-progress','bullet-fr-progress-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-fr-progress','bullet-fr-progress-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-fr-progress','bullet-fr-progress-cpp',this)">C++</button>
</div><div id="bullet-fr-progress-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="progress",
)
chart.show()</code></pre></div><div id="bullet-fr-progress-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "progress",
});
chart.show();</code></pre></div><div id="bullet-fr-progress-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "progress",
});
chart.show();</code></pre></div><div id="bullet-fr-progress-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "progress"
)
chart$show()</code></pre></div><div id="bullet-fr-progress-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("progress")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-fr-progress-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("progress")
    .build();
chart.show();</code></pre></div><div id="bullet-fr-progress-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "progress"
);
chart.Show();</code></pre></div><div id="bullet-fr-progress-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "progress"
)
chart.show()</code></pre></div><div id="bullet-fr-progress-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "progress",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-progress.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-compare">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"compare"</code></span><span><strong>Aliases</strong> <code>compare / vs / ghost / prior</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Ajoute une barre fantome (par ex. periode precedente via comparisons) derriere la valeur courante.</p>
<div class="sp-tabs" id="bullet-fr-compare">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-fr-compare','bullet-fr-compare-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-fr-compare','bullet-fr-compare-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-compare','bullet-fr-compare-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr-compare','bullet-fr-compare-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-fr-compare','bullet-fr-compare-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bullet-fr-compare','bullet-fr-compare-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-fr-compare','bullet-fr-compare-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-fr-compare','bullet-fr-compare-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-fr-compare','bullet-fr-compare-cpp',this)">C++</button>
</div><div id="bullet-fr-compare-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Revenue","Profit","Satisfaction","NPS","Speed","Quality"]
values  = [78, 42, 88, 65, 33, 71]
targets = [80, 50, 90, 70, 60, 85]
max_vals = [100]*6
ranges   = [60]*6

chart = sp.bullet(
    title="KPIs Q3",
    labels=labels, values=values,
    targets=targets, max_vals=max_vals, ranges=ranges,
    variant="compare",
)
chart.show()</code></pre></div><div id="bullet-fr-compare-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "compare",
});
chart.show();</code></pre></div><div id="bullet-fr-compare-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bullet({
  title: "KPIs Q3",
  labels: [/* ... */], values: [/* ... */],
  targets: [/* ... */], max_vals: [/* ... */], ranges: [/* ... */],
  variant: "compare",
});
chart.show();</code></pre></div><div id="bullet-fr-compare-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$bullet(
  title = "KPIs Q3",
  labels = c(), values = c(),
  targets = c(), max_vals = c(), ranges = c(),
  variant = "compare"
)
chart$show()</code></pre></div><div id="bullet-fr-compare-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bullet()
        .title("KPIs Q3")
        .labels(vec![]).values(vec![])
        .targets(vec![]).max_vals(vec![]).ranges(vec![])
        .variant("compare")
        .build();
    chart.show();
}</code></pre></div><div id="bullet-fr-compare-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("KPIs Q3")
    .labels(List.of()).values(List.of())
    .targets(List.of()).maxVals(List.of()).ranges(List.of())
    .variant("compare")
    .build();
chart.show();</code></pre></div><div id="bullet-fr-compare-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "KPIs Q3",
    labels: new string[]{}, values: new double[]{},
    targets: new double[]{}, maxVals: new double[]{}, ranges: new double[]{},
    variant: "compare"
);
chart.Show();</code></pre></div><div id="bullet-fr-compare-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bullet(
  title   = "KPIs Q3",
  labels  = List(), values = List(),
  targets = List(), maxVals = List(), ranges = List(),
  variant = "compare"
)
chart.show()</code></pre></div><div id="bullet-fr-compare-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
    .title    = "KPIs Q3",
    .labels   = {}, .values = {},
    .targets  = {}, .max_vals = {}, .ranges = {},
    .variant  = "compare",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-compare.html"></iframe>
</div>
</div>
</div>

</div>
