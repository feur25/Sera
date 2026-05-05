# Gauge - Single-Value Arc Indicator

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

`sp.gauge(title, *, value, min_val=0.0, max_val=100.0, label="", variant="basic", comparison=0.0, **kwargs) -> Chart`

Aliases: `sp.gauge`, `sp.build_gauge`

## Description

`sp.gauge()` is the unified entry point for the gauge family. A gauge maps a single scalar to a colored arc with optional thresholds - perfect for status / health / utilization KPIs. The `variant` keyword switches the geometry (half, three-quarter, full ring), the embellishments (needle, ticks, glow) and the layering (single arc vs. concentric arcs for value-vs-target).

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / half / classic` | Half-circle gauge with needle and color thresholds - the speedometer everyone knows. |
| `"radial"` | `radial / donut / ring / full` | Full-circle donut progress arc - elegant ring KPI for dashboards. |
| `"arc270"` | `arc270 / three_quarter / arc / wide` | 270-degree arc - more arc length for finer reading than a half-circle. |
| `"sleek"` | `sleek / minimal / clean / flat` | No needle, no ticks - oversized value text on a clean colored arc. |
| `"tick"` | `tick / tickmarks / scaled / ruler` | Half-arc with ruler tick marks every 5% and major labels every 25%. |
| `"segmented"` | `segmented / battery / signal / chunked` | Battery / signal-bar style with discrete chunks lighting up by threshold. |
| `"glow"` | `glow / neon / halo / luminous` | Neon glow effect on the active arc - dramatic dark dashboard look. |
| `"concentric"` | `concentric / rings / target / dual` | Two concentric arcs: outer = current, inner = comparison or target. |

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title` | `str` | required | Chart title |
| `value` | `float` | required | Current value to display |
| `min_val` | `float` | `0.0` | Scale minimum |
| `max_val` | `float` | `100.0` | Scale maximum |
| `variant` | `str` | `"basic"` | Visual style (see table) |
| `label` | `str` | `""` | Sub-label below value |
| `comparison` | `float` | `0.0` | Comparison value (for `concentric`) |
| `thresholds` | `list[(float,int)]` | `None` | `[(value, color_hex), ...]` thresholds |
| `width` | `int` | `400` | Canvas width (px) |
| `height` | `int` | `300` | Canvas height (px) |

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="gauge-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('gauge-en')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('gauge-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','radial',this)"><span class="sp-cic">R</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','arc270',this)"><span class="sp-cic">A</span><span class="sp-clb">Arc 270</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','sleek',this)"><span class="sp-cic">S</span><span class="sp-clb">Sleek</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','tick',this)"><span class="sp-cic">T</span><span class="sp-clb">Tick</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','segmented',this)"><span class="sp-cic">Z</span><span class="sp-clb">Segmented</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','glow',this)"><span class="sp-cic">G</span><span class="sp-clb">Glow</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','concentric',this)"><span class="sp-cic">C</span><span class="sp-clb">Concentric</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="gauge-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / half / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Half-circle gauge with needle and color thresholds - the speedometer everyone knows.</p>
<div class="sp-tabs" id="gauge-en-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-en-basic','gauge-en-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-en-basic','gauge-en-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-basic','gauge-en-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-basic','gauge-en-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-en-basic','gauge-en-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-en-basic','gauge-en-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-en-basic','gauge-en-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-en-basic','gauge-en-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-en-basic','gauge-en-basic-cpp',this)">C++</button>
</div><div id="gauge-en-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="basic",
)
chart.show()</code></pre></div><div id="gauge-en-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "basic",
});
chart.show();</code></pre></div><div id="gauge-en-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "basic",
});
chart.show();</code></pre></div><div id="gauge-en-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "basic"
)
chart$show()</code></pre></div><div id="gauge-en-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-en-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="gauge-en-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "basic"
);
chart.Show();</code></pre></div><div id="gauge-en-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "basic"
)
chart.show()</code></pre></div><div id="gauge-en-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-basic.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-radial">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"radial"</code></span><span><strong>Aliases</strong> <code>radial / donut / ring / full</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Full-circle donut progress arc - elegant ring KPI for dashboards.</p>
<div class="sp-tabs" id="gauge-en-radial">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-en-radial','gauge-en-radial-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-en-radial','gauge-en-radial-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-radial','gauge-en-radial-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-radial','gauge-en-radial-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-en-radial','gauge-en-radial-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-en-radial','gauge-en-radial-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-en-radial','gauge-en-radial-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-en-radial','gauge-en-radial-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-en-radial','gauge-en-radial-cpp',this)">C++</button>
</div><div id="gauge-en-radial-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="radial",
)
chart.show()</code></pre></div><div id="gauge-en-radial-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "radial",
});
chart.show();</code></pre></div><div id="gauge-en-radial-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "radial",
});
chart.show();</code></pre></div><div id="gauge-en-radial-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "radial"
)
chart$show()</code></pre></div><div id="gauge-en-radial-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("radial")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-en-radial-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("radial")
    .build();
chart.show();</code></pre></div><div id="gauge-en-radial-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "radial"
);
chart.Show();</code></pre></div><div id="gauge-en-radial-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "radial"
)
chart.show()</code></pre></div><div id="gauge-en-radial-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "radial",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-radial.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-arc270">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arc270"</code></span><span><strong>Aliases</strong> <code>arc270 / three_quarter / arc / wide</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">270-degree arc - more arc length for finer reading than a half-circle.</p>
<div class="sp-tabs" id="gauge-en-arc270">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-en-arc270','gauge-en-arc270-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-en-arc270','gauge-en-arc270-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-arc270','gauge-en-arc270-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-arc270','gauge-en-arc270-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-en-arc270','gauge-en-arc270-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-en-arc270','gauge-en-arc270-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-en-arc270','gauge-en-arc270-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-en-arc270','gauge-en-arc270-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-en-arc270','gauge-en-arc270-cpp',this)">C++</button>
</div><div id="gauge-en-arc270-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="arc270",
)
chart.show()</code></pre></div><div id="gauge-en-arc270-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "arc270",
});
chart.show();</code></pre></div><div id="gauge-en-arc270-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "arc270",
});
chart.show();</code></pre></div><div id="gauge-en-arc270-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "arc270"
)
chart$show()</code></pre></div><div id="gauge-en-arc270-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("arc270")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-en-arc270-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("arc270")
    .build();
chart.show();</code></pre></div><div id="gauge-en-arc270-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "arc270"
);
chart.Show();</code></pre></div><div id="gauge-en-arc270-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "arc270"
)
chart.show()</code></pre></div><div id="gauge-en-arc270-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "arc270",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-arc270.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-sleek">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"sleek"</code></span><span><strong>Aliases</strong> <code>sleek / minimal / clean / flat</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">No needle, no ticks - oversized value text on a clean colored arc.</p>
<div class="sp-tabs" id="gauge-en-sleek">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-en-sleek','gauge-en-sleek-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-en-sleek','gauge-en-sleek-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-sleek','gauge-en-sleek-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-sleek','gauge-en-sleek-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-en-sleek','gauge-en-sleek-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-en-sleek','gauge-en-sleek-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-en-sleek','gauge-en-sleek-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-en-sleek','gauge-en-sleek-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-en-sleek','gauge-en-sleek-cpp',this)">C++</button>
</div><div id="gauge-en-sleek-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="sleek",
)
chart.show()</code></pre></div><div id="gauge-en-sleek-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "sleek",
});
chart.show();</code></pre></div><div id="gauge-en-sleek-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "sleek",
});
chart.show();</code></pre></div><div id="gauge-en-sleek-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "sleek"
)
chart$show()</code></pre></div><div id="gauge-en-sleek-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("sleek")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-en-sleek-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("sleek")
    .build();
chart.show();</code></pre></div><div id="gauge-en-sleek-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "sleek"
);
chart.Show();</code></pre></div><div id="gauge-en-sleek-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "sleek"
)
chart.show()</code></pre></div><div id="gauge-en-sleek-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "sleek",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-sleek.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-tick">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"tick"</code></span><span><strong>Aliases</strong> <code>tick / tickmarks / scaled / ruler</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Half-arc with ruler tick marks every 5% and major labels every 25%.</p>
<div class="sp-tabs" id="gauge-en-tick">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-en-tick','gauge-en-tick-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-en-tick','gauge-en-tick-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-tick','gauge-en-tick-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-tick','gauge-en-tick-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-en-tick','gauge-en-tick-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-en-tick','gauge-en-tick-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-en-tick','gauge-en-tick-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-en-tick','gauge-en-tick-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-en-tick','gauge-en-tick-cpp',this)">C++</button>
</div><div id="gauge-en-tick-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="tick",
)
chart.show()</code></pre></div><div id="gauge-en-tick-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "tick",
});
chart.show();</code></pre></div><div id="gauge-en-tick-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "tick",
});
chart.show();</code></pre></div><div id="gauge-en-tick-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "tick"
)
chart$show()</code></pre></div><div id="gauge-en-tick-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("tick")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-en-tick-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("tick")
    .build();
chart.show();</code></pre></div><div id="gauge-en-tick-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "tick"
);
chart.Show();</code></pre></div><div id="gauge-en-tick-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "tick"
)
chart.show()</code></pre></div><div id="gauge-en-tick-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "tick",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-tick.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-segmented">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"segmented"</code></span><span><strong>Aliases</strong> <code>segmented / battery / signal / chunked</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Battery / signal-bar style with discrete chunks lighting up by threshold.</p>
<div class="sp-tabs" id="gauge-en-segmented">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-en-segmented','gauge-en-segmented-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-en-segmented','gauge-en-segmented-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-segmented','gauge-en-segmented-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-segmented','gauge-en-segmented-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-en-segmented','gauge-en-segmented-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-en-segmented','gauge-en-segmented-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-en-segmented','gauge-en-segmented-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-en-segmented','gauge-en-segmented-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-en-segmented','gauge-en-segmented-cpp',this)">C++</button>
</div><div id="gauge-en-segmented-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="segmented",
)
chart.show()</code></pre></div><div id="gauge-en-segmented-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "segmented",
});
chart.show();</code></pre></div><div id="gauge-en-segmented-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "segmented",
});
chart.show();</code></pre></div><div id="gauge-en-segmented-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "segmented"
)
chart$show()</code></pre></div><div id="gauge-en-segmented-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("segmented")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-en-segmented-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("segmented")
    .build();
chart.show();</code></pre></div><div id="gauge-en-segmented-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "segmented"
);
chart.Show();</code></pre></div><div id="gauge-en-segmented-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "segmented"
)
chart.show()</code></pre></div><div id="gauge-en-segmented-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "segmented",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-segmented.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-glow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"glow"</code></span><span><strong>Aliases</strong> <code>glow / neon / halo / luminous</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Neon glow effect on the active arc - dramatic dark dashboard look.</p>
<div class="sp-tabs" id="gauge-en-glow">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-en-glow','gauge-en-glow-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-en-glow','gauge-en-glow-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-glow','gauge-en-glow-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-glow','gauge-en-glow-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-en-glow','gauge-en-glow-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-en-glow','gauge-en-glow-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-en-glow','gauge-en-glow-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-en-glow','gauge-en-glow-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-en-glow','gauge-en-glow-cpp',this)">C++</button>
</div><div id="gauge-en-glow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="glow",
)
chart.show()</code></pre></div><div id="gauge-en-glow-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "glow",
});
chart.show();</code></pre></div><div id="gauge-en-glow-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "glow",
});
chart.show();</code></pre></div><div id="gauge-en-glow-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "glow"
)
chart$show()</code></pre></div><div id="gauge-en-glow-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("glow")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-en-glow-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("glow")
    .build();
chart.show();</code></pre></div><div id="gauge-en-glow-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "glow"
);
chart.Show();</code></pre></div><div id="gauge-en-glow-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "glow"
)
chart.show()</code></pre></div><div id="gauge-en-glow-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "glow",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-glow.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-concentric">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"concentric"</code></span><span><strong>Aliases</strong> <code>concentric / rings / target / dual</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Two concentric arcs: outer = current, inner = comparison or target.</p>
<div class="sp-tabs" id="gauge-en-concentric">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-en-concentric','gauge-en-concentric-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-en-concentric','gauge-en-concentric-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-concentric','gauge-en-concentric-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-en-concentric','gauge-en-concentric-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-en-concentric','gauge-en-concentric-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-en-concentric','gauge-en-concentric-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-en-concentric','gauge-en-concentric-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-en-concentric','gauge-en-concentric-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-en-concentric','gauge-en-concentric-cpp',this)">C++</button>
</div><div id="gauge-en-concentric-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="concentric",
)
chart.show()</code></pre></div><div id="gauge-en-concentric-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "concentric",
});
chart.show();</code></pre></div><div id="gauge-en-concentric-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "concentric",
});
chart.show();</code></pre></div><div id="gauge-en-concentric-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "concentric"
)
chart$show()</code></pre></div><div id="gauge-en-concentric-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("concentric")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-en-concentric-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("concentric")
    .build();
chart.show();</code></pre></div><div id="gauge-en-concentric-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "concentric"
);
chart.Show();</code></pre></div><div id="gauge-en-concentric-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "concentric"
)
chart.show()</code></pre></div><div id="gauge-en-concentric-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "concentric",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-concentric.html"></iframe>
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

`sp.gauge(title, *, value, min_val=0.0, max_val=100.0, label="", variant="basic", comparison=0.0, **kwargs) -> Chart`

Aliases: `sp.gauge`, `sp.build_gauge`

## Description

`sp.gauge()` est le point d entree unique pour la famille jauge. Une jauge associe un scalaire unique a un arc colore avec des seuils optionnels - parfait pour des KPIs de statut / sante / utilisation. Le mot-cle `variant` change la geometrie (demi, trois-quart, anneau complet), les ornements (aiguille, ticks, glow) et la composition (arc simple ou arcs concentriques pour valeur-vs-cible).

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / half / classic` | Jauge demi-cercle avec aiguille et seuils colores - le compteur que tout le monde connait. |
| `"radial"` | `radial / donut / ring / full` | Arc de progression circulaire complet - KPI elegant en anneau pour tableaux de bord. |
| `"arc270"` | `arc270 / three_quarter / arc / wide` | Arc de 270 degres - plus de longueur pour une lecture plus fine qu un demi-cercle. |
| `"sleek"` | `sleek / minimal / clean / flat` | Sans aiguille ni ticks - valeur en grand sur un arc colore epure. |
| `"tick"` | `tick / tickmarks / scaled / ruler` | Demi-arc avec graduations regle tous les 5% et labels majeurs tous les 25%. |
| `"segmented"` | `segmented / battery / signal / chunked` | Style batterie / barre de reseau avec segments discrets s allumant par seuil. |
| `"glow"` | `glow / neon / halo / luminous` | Effet neon sur l arc actif - look dashboard sombre tres marquant. |
| `"concentric"` | `concentric / rings / target / dual` | Deux arcs concentriques : externe = courant, interne = comparaison ou cible. |

## Parametres

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title` | `str` | requis | Titre du graphique |
| `value` | `float` | requis | Valeur courante a afficher |
| `min_val` | `float` | `0.0` | Minimum de l echelle |
| `max_val` | `float` | `100.0` | Maximum de l echelle |
| `variant` | `str` | `"basic"` | Style visuel (voir tableau) |
| `label` | `str` | `""` | Sous-libelle sous la valeur |
| `comparison` | `float` | `0.0` | Valeur de comparaison (pour `concentric`) |
| `seuils` | `list[(float,int)]` | `None` | `[(value, color_hex), ...]` seuils |
| `width` | `int` | `400` | Largeur (px) |
| `height` | `int` | `300` | Hauteur (px) |

## Retour

`Chart` - objet avec propriete `.html` et methode `.show()`.

---

<div class="sp-cls sp-open" id="gauge-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('gauge-fr')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('gauge-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','radial',this)"><span class="sp-cic">R</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','arc270',this)"><span class="sp-cic">A</span><span class="sp-clb">Arc 270</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','sleek',this)"><span class="sp-cic">S</span><span class="sp-clb">Sleek</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','tick',this)"><span class="sp-cic">T</span><span class="sp-clb">Tick</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','segmented',this)"><span class="sp-cic">Z</span><span class="sp-clb">Segmented</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','glow',this)"><span class="sp-cic">G</span><span class="sp-clb">Glow</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','concentric',this)"><span class="sp-cic">C</span><span class="sp-clb">Concentric</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="gauge-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / half / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Jauge demi-cercle avec aiguille et seuils colores - le compteur que tout le monde connait.</p>
<div class="sp-tabs" id="gauge-fr-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-fr-basic','gauge-fr-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-fr-basic','gauge-fr-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-basic','gauge-fr-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-basic','gauge-fr-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-fr-basic','gauge-fr-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-fr-basic','gauge-fr-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-fr-basic','gauge-fr-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-fr-basic','gauge-fr-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-fr-basic','gauge-fr-basic-cpp',this)">C++</button>
</div><div id="gauge-fr-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="basic",
)
chart.show()</code></pre></div><div id="gauge-fr-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "basic",
});
chart.show();</code></pre></div><div id="gauge-fr-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "basic",
});
chart.show();</code></pre></div><div id="gauge-fr-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "basic"
)
chart$show()</code></pre></div><div id="gauge-fr-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-fr-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="gauge-fr-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "basic"
);
chart.Show();</code></pre></div><div id="gauge-fr-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "basic"
)
chart.show()</code></pre></div><div id="gauge-fr-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-basic.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-radial">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"radial"</code></span><span><strong>Aliases</strong> <code>radial / donut / ring / full</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Arc de progression circulaire complet - KPI elegant en anneau pour tableaux de bord.</p>
<div class="sp-tabs" id="gauge-fr-radial">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-fr-radial','gauge-fr-radial-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-fr-radial','gauge-fr-radial-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-radial','gauge-fr-radial-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-radial','gauge-fr-radial-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-fr-radial','gauge-fr-radial-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-fr-radial','gauge-fr-radial-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-fr-radial','gauge-fr-radial-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-fr-radial','gauge-fr-radial-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-fr-radial','gauge-fr-radial-cpp',this)">C++</button>
</div><div id="gauge-fr-radial-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="radial",
)
chart.show()</code></pre></div><div id="gauge-fr-radial-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "radial",
});
chart.show();</code></pre></div><div id="gauge-fr-radial-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "radial",
});
chart.show();</code></pre></div><div id="gauge-fr-radial-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "radial"
)
chart$show()</code></pre></div><div id="gauge-fr-radial-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("radial")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-fr-radial-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("radial")
    .build();
chart.show();</code></pre></div><div id="gauge-fr-radial-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "radial"
);
chart.Show();</code></pre></div><div id="gauge-fr-radial-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "radial"
)
chart.show()</code></pre></div><div id="gauge-fr-radial-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "radial",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-radial.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-arc270">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arc270"</code></span><span><strong>Aliases</strong> <code>arc270 / three_quarter / arc / wide</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Arc de 270 degres - plus de longueur pour une lecture plus fine qu un demi-cercle.</p>
<div class="sp-tabs" id="gauge-fr-arc270">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-fr-arc270','gauge-fr-arc270-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-fr-arc270','gauge-fr-arc270-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-arc270','gauge-fr-arc270-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-arc270','gauge-fr-arc270-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-fr-arc270','gauge-fr-arc270-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-fr-arc270','gauge-fr-arc270-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-fr-arc270','gauge-fr-arc270-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-fr-arc270','gauge-fr-arc270-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-fr-arc270','gauge-fr-arc270-cpp',this)">C++</button>
</div><div id="gauge-fr-arc270-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="arc270",
)
chart.show()</code></pre></div><div id="gauge-fr-arc270-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "arc270",
});
chart.show();</code></pre></div><div id="gauge-fr-arc270-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "arc270",
});
chart.show();</code></pre></div><div id="gauge-fr-arc270-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "arc270"
)
chart$show()</code></pre></div><div id="gauge-fr-arc270-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("arc270")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-fr-arc270-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("arc270")
    .build();
chart.show();</code></pre></div><div id="gauge-fr-arc270-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "arc270"
);
chart.Show();</code></pre></div><div id="gauge-fr-arc270-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "arc270"
)
chart.show()</code></pre></div><div id="gauge-fr-arc270-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "arc270",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-arc270.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-sleek">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"sleek"</code></span><span><strong>Aliases</strong> <code>sleek / minimal / clean / flat</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Sans aiguille ni ticks - valeur en grand sur un arc colore epure.</p>
<div class="sp-tabs" id="gauge-fr-sleek">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-fr-sleek','gauge-fr-sleek-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-fr-sleek','gauge-fr-sleek-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-sleek','gauge-fr-sleek-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-sleek','gauge-fr-sleek-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-fr-sleek','gauge-fr-sleek-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-fr-sleek','gauge-fr-sleek-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-fr-sleek','gauge-fr-sleek-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-fr-sleek','gauge-fr-sleek-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-fr-sleek','gauge-fr-sleek-cpp',this)">C++</button>
</div><div id="gauge-fr-sleek-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="sleek",
)
chart.show()</code></pre></div><div id="gauge-fr-sleek-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "sleek",
});
chart.show();</code></pre></div><div id="gauge-fr-sleek-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "sleek",
});
chart.show();</code></pre></div><div id="gauge-fr-sleek-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "sleek"
)
chart$show()</code></pre></div><div id="gauge-fr-sleek-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("sleek")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-fr-sleek-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("sleek")
    .build();
chart.show();</code></pre></div><div id="gauge-fr-sleek-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "sleek"
);
chart.Show();</code></pre></div><div id="gauge-fr-sleek-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "sleek"
)
chart.show()</code></pre></div><div id="gauge-fr-sleek-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "sleek",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-sleek.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-tick">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"tick"</code></span><span><strong>Aliases</strong> <code>tick / tickmarks / scaled / ruler</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Demi-arc avec graduations regle tous les 5% et labels majeurs tous les 25%.</p>
<div class="sp-tabs" id="gauge-fr-tick">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-fr-tick','gauge-fr-tick-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-fr-tick','gauge-fr-tick-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-tick','gauge-fr-tick-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-tick','gauge-fr-tick-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-fr-tick','gauge-fr-tick-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-fr-tick','gauge-fr-tick-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-fr-tick','gauge-fr-tick-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-fr-tick','gauge-fr-tick-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-fr-tick','gauge-fr-tick-cpp',this)">C++</button>
</div><div id="gauge-fr-tick-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="tick",
)
chart.show()</code></pre></div><div id="gauge-fr-tick-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "tick",
});
chart.show();</code></pre></div><div id="gauge-fr-tick-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "tick",
});
chart.show();</code></pre></div><div id="gauge-fr-tick-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "tick"
)
chart$show()</code></pre></div><div id="gauge-fr-tick-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("tick")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-fr-tick-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("tick")
    .build();
chart.show();</code></pre></div><div id="gauge-fr-tick-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "tick"
);
chart.Show();</code></pre></div><div id="gauge-fr-tick-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "tick"
)
chart.show()</code></pre></div><div id="gauge-fr-tick-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "tick",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-tick.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-segmented">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"segmented"</code></span><span><strong>Aliases</strong> <code>segmented / battery / signal / chunked</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Style batterie / barre de reseau avec segments discrets s allumant par seuil.</p>
<div class="sp-tabs" id="gauge-fr-segmented">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-fr-segmented','gauge-fr-segmented-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-fr-segmented','gauge-fr-segmented-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-segmented','gauge-fr-segmented-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-segmented','gauge-fr-segmented-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-fr-segmented','gauge-fr-segmented-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-fr-segmented','gauge-fr-segmented-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-fr-segmented','gauge-fr-segmented-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-fr-segmented','gauge-fr-segmented-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-fr-segmented','gauge-fr-segmented-cpp',this)">C++</button>
</div><div id="gauge-fr-segmented-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="segmented",
)
chart.show()</code></pre></div><div id="gauge-fr-segmented-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "segmented",
});
chart.show();</code></pre></div><div id="gauge-fr-segmented-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "segmented",
});
chart.show();</code></pre></div><div id="gauge-fr-segmented-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "segmented"
)
chart$show()</code></pre></div><div id="gauge-fr-segmented-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("segmented")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-fr-segmented-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("segmented")
    .build();
chart.show();</code></pre></div><div id="gauge-fr-segmented-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "segmented"
);
chart.Show();</code></pre></div><div id="gauge-fr-segmented-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "segmented"
)
chart.show()</code></pre></div><div id="gauge-fr-segmented-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "segmented",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-segmented.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-glow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"glow"</code></span><span><strong>Aliases</strong> <code>glow / neon / halo / luminous</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Effet neon sur l arc actif - look dashboard sombre tres marquant.</p>
<div class="sp-tabs" id="gauge-fr-glow">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-fr-glow','gauge-fr-glow-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-fr-glow','gauge-fr-glow-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-glow','gauge-fr-glow-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-glow','gauge-fr-glow-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-fr-glow','gauge-fr-glow-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-fr-glow','gauge-fr-glow-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-fr-glow','gauge-fr-glow-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-fr-glow','gauge-fr-glow-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-fr-glow','gauge-fr-glow-cpp',this)">C++</button>
</div><div id="gauge-fr-glow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="glow",
)
chart.show()</code></pre></div><div id="gauge-fr-glow-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "glow",
});
chart.show();</code></pre></div><div id="gauge-fr-glow-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "glow",
});
chart.show();</code></pre></div><div id="gauge-fr-glow-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "glow"
)
chart$show()</code></pre></div><div id="gauge-fr-glow-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("glow")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-fr-glow-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("glow")
    .build();
chart.show();</code></pre></div><div id="gauge-fr-glow-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "glow"
);
chart.Show();</code></pre></div><div id="gauge-fr-glow-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "glow"
)
chart.show()</code></pre></div><div id="gauge-fr-glow-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "glow",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-glow.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-concentric">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"concentric"</code></span><span><strong>Aliases</strong> <code>concentric / rings / target / dual</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Deux arcs concentriques : externe = courant, interne = comparaison ou cible.</p>
<div class="sp-tabs" id="gauge-fr-concentric">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-fr-concentric','gauge-fr-concentric-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-fr-concentric','gauge-fr-concentric-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-concentric','gauge-fr-concentric-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr-concentric','gauge-fr-concentric-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-fr-concentric','gauge-fr-concentric-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('gauge-fr-concentric','gauge-fr-concentric-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-fr-concentric','gauge-fr-concentric-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-fr-concentric','gauge-fr-concentric-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-fr-concentric','gauge-fr-concentric-cpp',this)">C++</button>
</div><div id="gauge-fr-concentric-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="CPU Load",
    value=72.5, min_val=0, max_val=100,
    label="usage %",
    variant="concentric",
)
chart.show()</code></pre></div><div id="gauge-fr-concentric-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "concentric",
});
chart.show();</code></pre></div><div id="gauge-fr-concentric-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.gauge({
  title: "CPU Load",
  value: 72.5, min_val: 0, max_val: 100,
  label: "usage %",
  variant: "concentric",
});
chart.show();</code></pre></div><div id="gauge-fr-concentric-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$gauge(
  title = "CPU Load",
  value = 72.5, min_val = 0, max_val = 100,
  label = "usage %",
  variant = "concentric"
)
chart$show()</code></pre></div><div id="gauge-fr-concentric-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::gauge()
        .title("CPU Load")
        .value(72.5).min_val(0.0).max_val(100.0)
        .label("usage %")
        .variant("concentric")
        .build();
    chart.show();
}</code></pre></div><div id="gauge-fr-concentric-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;

var chart = SeraPlot.gauge()
    .title("CPU Load")
    .value(72.5).minVal(0).maxVal(100)
    .label("usage %")
    .variant("concentric")
    .build();
chart.show();</code></pre></div><div id="gauge-fr-concentric-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "CPU Load",
    value: 72.5, minVal: 0, maxVal: 100,
    label: "usage %",
    variant: "concentric"
);
chart.Show();</code></pre></div><div id="gauge-fr-concentric-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.gauge(
  title   = "CPU Load",
  value   = 72.5, minVal = 0, maxVal = 100,
  label   = "usage %",
  variant = "concentric"
)
chart.show()</code></pre></div><div id="gauge-fr-concentric-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
    .title   = "CPU Load",
    .value   = 72.5, .min_val = 0, .max_val = 100,
    .label   = "usage %",
    .variant = "concentric",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-concentric.html"></iframe>
</div>
</div>
</div>

</div>
