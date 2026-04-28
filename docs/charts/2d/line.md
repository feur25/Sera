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

<div class="sp-tabs" id="l-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-basic','l-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-basic','l-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-basic','l-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('l-basic','l-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('l-basic','l-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('l-basic','l-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('l-basic','l-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('l-basic','l-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('l-basic','l-basic-cpp',this)">C++</button>
</div>
<div id="l-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Temperature over 12 months',
    variant='basic',
    labels=['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'],
    values=[2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0],
    y_label='°C',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="l-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Temperature over 12 months",
  variant: "basic",
  labels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"],
  values: [2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0],
  yLabel: "°C",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Temperature over 12 months",
  variant: "basic",
  labels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"],
  values: [2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0],
  yLabel: "°C",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Temperature over 12 months",
  variant = "basic",
  labels = c("Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"),
  values = c(2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0),
  y_label = "°C",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="l-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Temperature over 12 months")
        .variant("basic")
        .labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"])
        .values(vec![2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0])
        .y_label("°C")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="l-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Temperature over 12 months")
    .variant("basic")
    .labels(List.of("Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"))
    .values(List.of(2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0))
    .yLabel("°C")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="l-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Temperature over 12 months",
    variant: "basic",
    labels: new[] { "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec" },
    values: new[] { 2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0 },
    yLabel: "°C",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="l-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Temperature over 12 months",
  variant = "basic",
  labels = List("Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"),
  values = List(2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0),
  y_label = "°C",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="l-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Temperature over 12 months")
    .variant("basic")
    .labels({"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"})
    .values({2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0})
    .yLabel("°C")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-basic.html"></iframe>
</div>

<div class="sp-variant" id="line-en-multi">

Several series sharing the same x-axis. Pass `series=[(name, values), ...]`.


<div class="sp-vmeta"><span><strong>Variant</strong> <code>"multi"</code></span><span><strong>Aliases</strong> <code>"multi"</code> / <code>"multiline"</code> / <code>"multiple"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="l-multi">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-multi','l-multi-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-multi','l-multi-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-multi','l-multi-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('l-multi','l-multi-r',this)">R</button>
<button class="sp-tb" onclick="spTab('l-multi','l-multi-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('l-multi','l-multi-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('l-multi','l-multi-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('l-multi','l-multi-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('l-multi','l-multi-cpp',this)">C++</button>
</div>
<div id="l-multi-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Quarterly revenue by region',
    variant='multi',
    x_labels=['Q1', 'Q2', 'Q3', 'Q4'],
    series=[("EU", [120, 145, 160, 180]), ("NA", [90, 110, 135, 150]), ("APAC", [60, 75, 80, 95])],
    y_label='Revenue (M€)',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="l-multi-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Quarterly revenue by region",
  variant: "multi",
  xLabels: ["Q1", "Q2", "Q3", "Q4"],
  series: [["EU", [120, 145, 160, 180]], ["NA", [90, 110, 135, 150]], ["APAC", [60, 75, 80, 95]]],
  yLabel: "Revenue (M€)",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-multi-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Quarterly revenue by region",
  variant: "multi",
  xLabels: ["Q1", "Q2", "Q3", "Q4"],
  series: [["EU", [120, 145, 160, 180]], ["NA", [90, 110, 135, 150]], ["APAC", [60, 75, 80, 95]]],
  yLabel: "Revenue (M€)",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-multi-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Quarterly revenue by region",
  variant = "multi",
  x_labels = c("Q1", "Q2", "Q3", "Q4"),
  series = list(list("EU", c(120, 145, 160, 180)), list("NA", c(90, 110, 135, 150)), list("APAC", c(60, 75, 80, 95))),
  y_label = "Revenue (M€)",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="l-multi-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Quarterly revenue by region")
        .variant("multi")
        .x_labels(vec!["Q1", "Q2", "Q3", "Q4"])
        .series(vec![("EU".into(), vec![120.0, 145.0, 160.0, 180.0]), ("NA".into(), vec![90.0, 110.0, 135.0, 150.0]), ("APAC".into(), vec![60.0, 75.0, 80.0, 95.0])])
        .y_label("Revenue (M€)")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="l-multi-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Quarterly revenue by region")
    .variant("multi")
    .xLabels(List.of("Q1", "Q2", "Q3", "Q4"))
    .series(List.of(Map.entry("EU", List.of(120.0, 145.0, 160.0, 180.0)), Map.entry("NA", List.of(90.0, 110.0, 135.0, 150.0)), Map.entry("APAC", List.of(60.0, 75.0, 80.0, 95.0))))
    .yLabel("Revenue (M€)")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="l-multi-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Quarterly revenue by region",
    variant: "multi",
    xLabels: new[] { "Q1", "Q2", "Q3", "Q4" },
    series: new[] { ("EU", new[] { 120, 145, 160, 180 }), ("NA", new[] { 90, 110, 135, 150 }), ("APAC", new[] { 60, 75, 80, 95 }) },
    yLabel: "Revenue (M€)",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="l-multi-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Quarterly revenue by region",
  variant = "multi",
  x_labels = List("Q1", "Q2", "Q3", "Q4"),
  series = List(("EU", List(120.0, 145.0, 160.0, 180.0)), ("NA", List(90.0, 110.0, 135.0, 150.0)), ("APAC", List(60.0, 75.0, 80.0, 95.0))),
  y_label = "Revenue (M€)",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="l-multi-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Quarterly revenue by region")
    .variant("multi")
    .xLabels({"Q1", "Q2", "Q3", "Q4"})
    .series({{"EU", {120, 145, 160, 180}}, {"NA", {90, 110, 135, 150}}, {"APAC", {60, 75, 80, 95}}})
    .yLabel("Revenue (M€)")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-multi.html"></iframe>
</div>

<div class="sp-variant" id="line-en-stepped">

Step (staircase) line — ideal for piecewise-constant data. Use `step_shape` to control corner direction.


<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>"stepped"</code> / <code>"step"</code> / <code>"hv"</code> / <code>"vh"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="l-stepped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-stepped','l-stepped-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-stepped','l-stepped-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-stepped','l-stepped-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('l-stepped','l-stepped-r',this)">R</button>
<button class="sp-tb" onclick="spTab('l-stepped','l-stepped-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('l-stepped','l-stepped-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('l-stepped','l-stepped-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('l-stepped','l-stepped-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('l-stepped','l-stepped-cpp',this)">C++</button>
</div>
<div id="l-stepped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Server load (stepped)',
    variant='stepped',
    labels=['00:00', '04:00', '08:00', '12:00', '16:00', '20:00'],
    values=[12.0, 8.0, 35.0, 52.0, 40.0, 18.0],
    step_shape='hv',
    y_label='% CPU',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="l-stepped-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Server load (stepped)",
  variant: "stepped",
  labels: ["00:00", "04:00", "08:00", "12:00", "16:00", "20:00"],
  values: [12.0, 8.0, 35.0, 52.0, 40.0, 18.0],
  stepShape: "hv",
  yLabel: "% CPU",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-stepped-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Server load (stepped)",
  variant: "stepped",
  labels: ["00:00", "04:00", "08:00", "12:00", "16:00", "20:00"],
  values: [12.0, 8.0, 35.0, 52.0, 40.0, 18.0],
  stepShape: "hv",
  yLabel: "% CPU",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-stepped-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Server load (stepped)",
  variant = "stepped",
  labels = c("00:00", "04:00", "08:00", "12:00", "16:00", "20:00"),
  values = c(12.0, 8.0, 35.0, 52.0, 40.0, 18.0),
  step_shape = "hv",
  y_label = "% CPU",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="l-stepped-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Server load (stepped)")
        .variant("stepped")
        .labels(vec!["00:00", "04:00", "08:00", "12:00", "16:00", "20:00"])
        .values(vec![12.0, 8.0, 35.0, 52.0, 40.0, 18.0])
        .step_shape("hv")
        .y_label("% CPU")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="l-stepped-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Server load (stepped)")
    .variant("stepped")
    .labels(List.of("00:00", "04:00", "08:00", "12:00", "16:00", "20:00"))
    .values(List.of(12.0, 8.0, 35.0, 52.0, 40.0, 18.0))
    .stepShape("hv")
    .yLabel("% CPU")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="l-stepped-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Server load (stepped)",
    variant: "stepped",
    labels: new[] { "00:00", "04:00", "08:00", "12:00", "16:00", "20:00" },
    values: new[] { 12.0, 8.0, 35.0, 52.0, 40.0, 18.0 },
    stepShape: "hv",
    yLabel: "% CPU",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="l-stepped-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Server load (stepped)",
  variant = "stepped",
  labels = List("00:00", "04:00", "08:00", "12:00", "16:00", "20:00"),
  values = List(12.0, 8.0, 35.0, 52.0, 40.0, 18.0),
  step_shape = "hv",
  y_label = "% CPU",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="l-stepped-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Server load (stepped)")
    .variant("stepped")
    .labels({"00:00", "04:00", "08:00", "12:00", "16:00", "20:00"})
    .values({12.0, 8.0, 35.0, 52.0, 40.0, 18.0})
    .stepShape("hv")
    .yLabel("% CPU")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-stepped.html"></iframe>
</div>

<div class="sp-variant" id="line-en-spline">

Catmull-Rom smoothed curve. `spline_tension` (0–1) controls how tight the curve hugs the points.


<div class="sp-vmeta"><span><strong>Variant</strong> <code>"spline"</code></span><span><strong>Aliases</strong> <code>"spline"</code> / <code>"smooth"</code> / <code>"curved"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="l-spline">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-spline','l-spline-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-spline','l-spline-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-spline','l-spline-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('l-spline','l-spline-r',this)">R</button>
<button class="sp-tb" onclick="spTab('l-spline','l-spline-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('l-spline','l-spline-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('l-spline','l-spline-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('l-spline','l-spline-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('l-spline','l-spline-cpp',this)">C++</button>
</div>
<div id="l-spline-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Sales trend (smoothed)',
    variant='spline',
    labels=['W1', 'W2', 'W3', 'W4', 'W5', 'W6', 'W7', 'W8'],
    values=[120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0],
    spline_tension=0.5,
    y_label='Units',
)
chart.show()</code></pre></div>
<div id="l-spline-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Sales trend (smoothed)",
  variant: "spline",
  labels: ["W1", "W2", "W3", "W4", "W5", "W6", "W7", "W8"],
  values: [120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0],
  splineTension: 0.5,
  yLabel: "Units"
});
chart.show();</code></pre></div>
<div id="l-spline-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Sales trend (smoothed)",
  variant: "spline",
  labels: ["W1", "W2", "W3", "W4", "W5", "W6", "W7", "W8"],
  values: [120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0],
  splineTension: 0.5,
  yLabel: "Units"
});
chart.show();</code></pre></div>
<div id="l-spline-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Sales trend (smoothed)",
  variant = "spline",
  labels = c("W1", "W2", "W3", "W4", "W5", "W6", "W7", "W8"),
  values = c(120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0),
  spline_tension = 0.5,
  y_label = "Units"
)
chart$show()</code></pre></div>
<div id="l-spline-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Sales trend (smoothed)")
        .variant("spline")
        .labels(vec!["W1", "W2", "W3", "W4", "W5", "W6", "W7", "W8"])
        .values(vec![120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0])
        .spline_tension(0.5)
        .y_label("Units")
        .build();
    chart.show();
}</code></pre></div>
<div id="l-spline-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Sales trend (smoothed)")
    .variant("spline")
    .labels(List.of("W1", "W2", "W3", "W4", "W5", "W6", "W7", "W8"))
    .values(List.of(120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0))
    .splineTension(0.5)
    .yLabel("Units")
    .build();
chart.show();</code></pre></div>
<div id="l-spline-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Sales trend (smoothed)",
    variant: "spline",
    labels: new[] { "W1", "W2", "W3", "W4", "W5", "W6", "W7", "W8" },
    values: new[] { 120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0 },
    splineTension: 0.5,
    yLabel: "Units"
);
chart.Show();</code></pre></div>
<div id="l-spline-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Sales trend (smoothed)",
  variant = "spline",
  labels = List("W1", "W2", "W3", "W4", "W5", "W6", "W7", "W8"),
  values = List(120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0),
  spline_tension = 0.5,
  y_label = "Units"
)
chart.show()</code></pre></div>
<div id="l-spline-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Sales trend (smoothed)")
    .variant("spline")
    .labels({"W1", "W2", "W3", "W4", "W5", "W6", "W7", "W8"})
    .values({120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0})
    .splineTension(0.5)
    .yLabel("Units")
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-spline.html"></iframe>
</div>

<div class="sp-variant" id="line-en-filled">

Area chart — fills the region under the line. `fill_opacity` controls transparency; `stack_fill=True` stacks multiple series.


<div class="sp-vmeta"><span><strong>Variant</strong> <code>"filled"</code></span><span><strong>Aliases</strong> <code>"filled"</code> / <code>"area"</code> / <code>"fill"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="l-filled">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-filled','l-filled-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-filled','l-filled-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-filled','l-filled-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('l-filled','l-filled-r',this)">R</button>
<button class="sp-tb" onclick="spTab('l-filled','l-filled-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('l-filled','l-filled-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('l-filled','l-filled-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('l-filled','l-filled-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('l-filled','l-filled-cpp',this)">C++</button>
</div>
<div id="l-filled-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Daily active users',
    variant='filled',
    labels=['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
    values=[320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0],
    fill_opacity=0.35,
    y_label='DAU',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="l-filled-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Daily active users",
  variant: "filled",
  labels: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
  values: [320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0],
  fillOpacity: 0.35,
  yLabel: "DAU",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-filled-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Daily active users",
  variant: "filled",
  labels: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
  values: [320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0],
  fillOpacity: 0.35,
  yLabel: "DAU",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-filled-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Daily active users",
  variant = "filled",
  labels = c("Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"),
  values = c(320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0),
  fill_opacity = 0.35,
  y_label = "DAU",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="l-filled-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Daily active users")
        .variant("filled")
        .labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
        .values(vec![320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0])
        .fill_opacity(0.35)
        .y_label("DAU")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="l-filled-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Daily active users")
    .variant("filled")
    .labels(List.of("Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"))
    .values(List.of(320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0))
    .fillOpacity(0.35)
    .yLabel("DAU")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="l-filled-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Daily active users",
    variant: "filled",
    labels: new[] { "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun" },
    values: new[] { 320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0 },
    fillOpacity: 0.35,
    yLabel: "DAU",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="l-filled-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Daily active users",
  variant = "filled",
  labels = List("Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"),
  values = List(320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0),
  fill_opacity = 0.35,
  y_label = "DAU",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="l-filled-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Daily active users")
    .variant("filled")
    .labels({"Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"})
    .values({320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0})
    .fillOpacity(0.35)
    .yLabel("DAU")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-filled.html"></iframe>
</div>

<div class="sp-variant" id="line-en-sparkline">

Small inline chart — no axes, perfect for dashboards. `spark_cols` arranges multiple series in a grid.


<div class="sp-vmeta"><span><strong>Variant</strong> <code>"sparkline"</code></span><span><strong>Aliases</strong> <code>"sparkline"</code> / <code>"spark"</code> / <code>"tiny"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="l-spark">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-spark','l-spark-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-spark','l-spark-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-spark','l-spark-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('l-spark','l-spark-r',this)">R</button>
<button class="sp-tb" onclick="spTab('l-spark','l-spark-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('l-spark','l-spark-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('l-spark','l-spark-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('l-spark','l-spark-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('l-spark','l-spark-cpp',this)">C++</button>
</div>
<div id="l-spark-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='KPI summary',
    variant='sparkline',
    series=[("Revenue", [120, 135, 128, 160, 180, 175, 210]), ("Users", [80, 90, 85, 110, 130, 125, 145]), ("Errors", [5, 3, 4, 2, 1, 2, 1])],
    spark_cols=3,
    spark_cell_w=220,
    spark_cell_h=60,
)
chart.show()</code></pre></div>
<div id="l-spark-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "KPI summary",
  variant: "sparkline",
  series: [["Revenue", [120, 135, 128, 160, 180, 175, 210]], ["Users", [80, 90, 85, 110, 130, 125, 145]], ["Errors", [5, 3, 4, 2, 1, 2, 1]]],
  sparkCols: 3,
  sparkCellW: 220,
  sparkCellH: 60
});
chart.show();</code></pre></div>
<div id="l-spark-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "KPI summary",
  variant: "sparkline",
  series: [["Revenue", [120, 135, 128, 160, 180, 175, 210]], ["Users", [80, 90, 85, 110, 130, 125, 145]], ["Errors", [5, 3, 4, 2, 1, 2, 1]]],
  sparkCols: 3,
  sparkCellW: 220,
  sparkCellH: 60
});
chart.show();</code></pre></div>
<div id="l-spark-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "KPI summary",
  variant = "sparkline",
  series = list(list("Revenue", c(120, 135, 128, 160, 180, 175, 210)), list("Users", c(80, 90, 85, 110, 130, 125, 145)), list("Errors", c(5, 3, 4, 2, 1, 2, 1))),
  spark_cols = 3,
  spark_cell_w = 220,
  spark_cell_h = 60
)
chart$show()</code></pre></div>
<div id="l-spark-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("KPI summary")
        .variant("sparkline")
        .series(vec![("Revenue".into(), vec![120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0]), ("Users".into(), vec![80.0, 90.0, 85.0, 110.0, 130.0, 125.0, 145.0]), ("Errors".into(), vec![5.0, 3.0, 4.0, 2.0, 1.0, 2.0, 1.0])])
        .spark_cols(3)
        .spark_cell_w(220)
        .spark_cell_h(60)
        .build();
    chart.show();
}</code></pre></div>
<div id="l-spark-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("KPI summary")
    .variant("sparkline")
    .series(List.of(Map.entry("Revenue", List.of(120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0)), Map.entry("Users", List.of(80.0, 90.0, 85.0, 110.0, 130.0, 125.0, 145.0)), Map.entry("Errors", List.of(5.0, 3.0, 4.0, 2.0, 1.0, 2.0, 1.0))))
    .sparkCols(3)
    .sparkCellW(220)
    .sparkCellH(60)
    .build();
chart.show();</code></pre></div>
<div id="l-spark-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "KPI summary",
    variant: "sparkline",
    series: new[] { ("Revenue", new[] { 120, 135, 128, 160, 180, 175, 210 }), ("Users", new[] { 80, 90, 85, 110, 130, 125, 145 }), ("Errors", new[] { 5, 3, 4, 2, 1, 2, 1 }) },
    sparkCols: 3,
    sparkCellW: 220,
    sparkCellH: 60
);
chart.Show();</code></pre></div>
<div id="l-spark-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "KPI summary",
  variant = "sparkline",
  series = List(("Revenue", List(120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0)), ("Users", List(80.0, 90.0, 85.0, 110.0, 130.0, 125.0, 145.0)), ("Errors", List(5.0, 3.0, 4.0, 2.0, 1.0, 2.0, 1.0))),
  spark_cols = 3,
  spark_cell_w = 220,
  spark_cell_h = 60
)
chart.show()</code></pre></div>
<div id="l-spark-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("KPI summary")
    .variant("sparkline")
    .series({{"Revenue", {120, 135, 128, 160, 180, 175, 210}}, {"Users", {80, 90, 85, 110, 130, 125, 145}}, {"Errors", {5, 3, 4, 2, 1, 2, 1}}})
    .sparkCols(3)
    .sparkCellW(220)
    .sparkCellH(60)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-sparkline.html"></iframe>
</div>

<div class="sp-variant" id="line-en-dashed">

Custom stroke pattern. `dash_pattern="8,4"` means 8px on, 4px off. Use `"2,3"` for dotted.


<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dashed"</code></span><span><strong>Aliases</strong> <code>"dashed"</code> / <code>"dotted"</code> / <code>"styled"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="l-dashed">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-dashed','l-dashed-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-dashed','l-dashed-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-dashed','l-dashed-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('l-dashed','l-dashed-r',this)">R</button>
<button class="sp-tb" onclick="spTab('l-dashed','l-dashed-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('l-dashed','l-dashed-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('l-dashed','l-dashed-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('l-dashed','l-dashed-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('l-dashed','l-dashed-cpp',this)">C++</button>
</div>
<div id="l-dashed-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Forecast vs actual',
    variant='dashed',
    labels=['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun'],
    values=[10.0, 18.0, 22.0, 30.0, 38.0, 45.0],
    dash_pattern='8,4',
    stroke_width=2.5,
    y_label='Units',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="l-dashed-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Forecast vs actual",
  variant: "dashed",
  labels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
  values: [10.0, 18.0, 22.0, 30.0, 38.0, 45.0],
  dashPattern: "8,4",
  strokeWidth: 2.5,
  yLabel: "Units",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-dashed-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Forecast vs actual",
  variant: "dashed",
  labels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
  values: [10.0, 18.0, 22.0, 30.0, 38.0, 45.0],
  dashPattern: "8,4",
  strokeWidth: 2.5,
  yLabel: "Units",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-dashed-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Forecast vs actual",
  variant = "dashed",
  labels = c("Jan", "Feb", "Mar", "Apr", "May", "Jun"),
  values = c(10.0, 18.0, 22.0, 30.0, 38.0, 45.0),
  dash_pattern = "8,4",
  stroke_width = 2.5,
  y_label = "Units",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="l-dashed-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Forecast vs actual")
        .variant("dashed")
        .labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
        .values(vec![10.0, 18.0, 22.0, 30.0, 38.0, 45.0])
        .dash_pattern("8,4")
        .stroke_width(2.5)
        .y_label("Units")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="l-dashed-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Forecast vs actual")
    .variant("dashed")
    .labels(List.of("Jan", "Feb", "Mar", "Apr", "May", "Jun"))
    .values(List.of(10.0, 18.0, 22.0, 30.0, 38.0, 45.0))
    .dashPattern("8,4")
    .strokeWidth(2.5)
    .yLabel("Units")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="l-dashed-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Forecast vs actual",
    variant: "dashed",
    labels: new[] { "Jan", "Feb", "Mar", "Apr", "May", "Jun" },
    values: new[] { 10.0, 18.0, 22.0, 30.0, 38.0, 45.0 },
    dashPattern: "8,4",
    strokeWidth: 2.5,
    yLabel: "Units",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="l-dashed-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Forecast vs actual",
  variant = "dashed",
  labels = List("Jan", "Feb", "Mar", "Apr", "May", "Jun"),
  values = List(10.0, 18.0, 22.0, 30.0, 38.0, 45.0),
  dash_pattern = "8,4",
  stroke_width = 2.5,
  y_label = "Units",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="l-dashed-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Forecast vs actual")
    .variant("dashed")
    .labels({"Jan", "Feb", "Mar", "Apr", "May", "Jun"})
    .values({10.0, 18.0, 22.0, 30.0, 38.0, 45.0})
    .dashPattern("8,4")
    .strokeWidth(2.5)
    .yLabel("Units")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-dashed.html"></iframe>
</div>

<div class="sp-variant" id="line-en-connected_scatter">

Line plot with prominent markers. `marker_size` (px) controls dot size; `show_points=True` is implicit.


<div class="sp-vmeta"><span><strong>Variant</strong> <code>"connected_scatter"</code></span><span><strong>Aliases</strong> <code>"connected_scatter"</code> / <code>"markers"</code> / <code>"lines+markers"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="l-connected">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-connected','l-connected-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-connected','l-connected-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-connected','l-connected-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('l-connected','l-connected-r',this)">R</button>
<button class="sp-tb" onclick="spTab('l-connected','l-connected-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('l-connected','l-connected-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('l-connected','l-connected-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('l-connected','l-connected-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('l-connected','l-connected-cpp',this)">C++</button>
</div>
<div id="l-connected-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Experiment readings',
    variant='connected_scatter',
    labels=['t0', 't1', 't2', 't3', 't4', 't5', 't6'],
    values=[4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3],
    marker_size=7,
    y_label='mV',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="l-connected-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Experiment readings",
  variant: "connected_scatter",
  labels: ["t0", "t1", "t2", "t3", "t4", "t5", "t6"],
  values: [4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3],
  markerSize: 7,
  yLabel: "mV",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-connected-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Experiment readings",
  variant: "connected_scatter",
  labels: ["t0", "t1", "t2", "t3", "t4", "t5", "t6"],
  values: [4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3],
  markerSize: 7,
  yLabel: "mV",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-connected-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Experiment readings",
  variant = "connected_scatter",
  labels = c("t0", "t1", "t2", "t3", "t4", "t5", "t6"),
  values = c(4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3),
  marker_size = 7,
  y_label = "mV",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="l-connected-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Experiment readings")
        .variant("connected_scatter")
        .labels(vec!["t0", "t1", "t2", "t3", "t4", "t5", "t6"])
        .values(vec![4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3])
        .marker_size(7)
        .y_label("mV")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="l-connected-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Experiment readings")
    .variant("connected_scatter")
    .labels(List.of("t0", "t1", "t2", "t3", "t4", "t5", "t6"))
    .values(List.of(4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3))
    .markerSize(7)
    .yLabel("mV")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="l-connected-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Experiment readings",
    variant: "connected_scatter",
    labels: new[] { "t0", "t1", "t2", "t3", "t4", "t5", "t6" },
    values: new[] { 4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3 },
    markerSize: 7,
    yLabel: "mV",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="l-connected-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Experiment readings",
  variant = "connected_scatter",
  labels = List("t0", "t1", "t2", "t3", "t4", "t5", "t6"),
  values = List(4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3),
  marker_size = 7,
  y_label = "mV",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="l-connected-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Experiment readings")
    .variant("connected_scatter")
    .labels({"t0", "t1", "t2", "t3", "t4", "t5", "t6"})
    .values({4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3})
    .markerSize(7)
    .yLabel("mV")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/line-connected_scatter.html"></iframe>
</div>

<div class="sp-variant" id="line-en-gapped">

Line breaks where values exceed `gap_threshold`. Useful for time series with missing samples.


<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>"gapped"</code> / <code>"gaps"</code> / <code>"missing"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="l-gapped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-gapped','l-gapped-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-gapped','l-gapped-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-gapped','l-gapped-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('l-gapped','l-gapped-r',this)">R</button>
<button class="sp-tb" onclick="spTab('l-gapped','l-gapped-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('l-gapped','l-gapped-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('l-gapped','l-gapped-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('l-gapped','l-gapped-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('l-gapped','l-gapped-cpp',this)">C++</button>
</div>
<div id="l-gapped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Sensor signal (with dropouts)',
    variant='gapped',
    labels=['t0', 't1', 't2', 't3', 't4', 't5', 't6', 't7', 't8'],
    values=[3.2, 3.5, 3.4, float("nan"), float("nan"), 4.1, 4.5, 4.3, 4.8],
    gap_threshold=1.0,
    y_label='V',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="l-gapped-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Sensor signal (with dropouts)",
  variant: "gapped",
  labels: ["t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"],
  values: [3.2, 3.5, 3.4, NaN, NaN, 4.1, 4.5, 4.3, 4.8],
  gapThreshold: 1.0,
  yLabel: "V",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-gapped-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Sensor signal (with dropouts)",
  variant: "gapped",
  labels: ["t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"],
  values: [3.2, 3.5, 3.4, NaN, NaN, 4.1, 4.5, 4.3, 4.8],
  gapThreshold: 1.0,
  yLabel: "V",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="l-gapped-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Sensor signal (with dropouts)",
  variant = "gapped",
  labels = c("t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"),
  values = c(3.2, 3.5, 3.4, NA, NA, 4.1, 4.5, 4.3, 4.8),
  gap_threshold = 1.0,
  y_label = "V",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="l-gapped-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Sensor signal (with dropouts)")
        .variant("gapped")
        .labels(vec!["t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"])
        .values(vec![3.2, 3.5, 3.4, f64::NAN, f64::NAN, 4.1, 4.5, 4.3, 4.8])
        .gap_threshold(1.0)
        .y_label("V")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="l-gapped-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Sensor signal (with dropouts)")
    .variant("gapped")
    .labels(List.of("t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"))
    .values(List.of(3.2, 3.5, 3.4, Double.NaN, Double.NaN, 4.1, 4.5, 4.3, 4.8))
    .gapThreshold(1.0)
    .yLabel("V")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="l-gapped-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Sensor signal (with dropouts)",
    variant: "gapped",
    labels: new[] { "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8" },
    values: new[] { 3.2, 3.5, 3.4, double.NaN, double.NaN, 4.1, 4.5, 4.3, 4.8 },
    gapThreshold: 1.0,
    yLabel: "V",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="l-gapped-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Sensor signal (with dropouts)",
  variant = "gapped",
  labels = List("t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"),
  values = List(3.2, 3.5, 3.4, Double.NaN, Double.NaN, 4.1, 4.5, 4.3, 4.8),
  gap_threshold = 1.0,
  y_label = "V",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="l-gapped-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Sensor signal (with dropouts)")
    .variant("gapped")
    .labels({"t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"})
    .values({3.2, 3.5, 3.4, std::nan(""), std::nan(""), 4.1, 4.5, 4.3, 4.8})
    .gapThreshold(1.0)
    .yLabel("V")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
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

<div class="sp-tabs" id="lf-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('lf-basic','lf-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('lf-basic','lf-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('lf-basic','lf-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('lf-basic','lf-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('lf-basic','lf-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('lf-basic','lf-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('lf-basic','lf-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('lf-basic','lf-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('lf-basic','lf-basic-cpp',this)">C++</button>
</div>
<div id="lf-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Température sur 12 mois',
    variant='basic',
    labels=['Jan', 'Fév', 'Mar', 'Avr', 'Mai', 'Juin', 'Juil', 'Août', 'Sep', 'Oct', 'Nov', 'Déc'],
    values=[2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0],
    y_label='°C',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="lf-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Température sur 12 mois",
  variant: "basic",
  labels: ["Jan", "Fév", "Mar", "Avr", "Mai", "Juin", "Juil", "Août", "Sep", "Oct", "Nov", "Déc"],
  values: [2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0],
  yLabel: "°C",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Température sur 12 mois",
  variant: "basic",
  labels: ["Jan", "Fév", "Mar", "Avr", "Mai", "Juin", "Juil", "Août", "Sep", "Oct", "Nov", "Déc"],
  values: [2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0],
  yLabel: "°C",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Température sur 12 mois",
  variant = "basic",
  labels = c("Jan", "Fév", "Mar", "Avr", "Mai", "Juin", "Juil", "Août", "Sep", "Oct", "Nov", "Déc"),
  values = c(2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0),
  y_label = "°C",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="lf-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Température sur 12 mois")
        .variant("basic")
        .labels(vec!["Jan", "Fév", "Mar", "Avr", "Mai", "Juin", "Juil", "Août", "Sep", "Oct", "Nov", "Déc"])
        .values(vec![2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0])
        .y_label("°C")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="lf-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Température sur 12 mois")
    .variant("basic")
    .labels(List.of("Jan", "Fév", "Mar", "Avr", "Mai", "Juin", "Juil", "Août", "Sep", "Oct", "Nov", "Déc"))
    .values(List.of(2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0))
    .yLabel("°C")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="lf-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Température sur 12 mois",
    variant: "basic",
    labels: new[] { "Jan", "Fév", "Mar", "Avr", "Mai", "Juin", "Juil", "Août", "Sep", "Oct", "Nov", "Déc" },
    values: new[] { 2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0 },
    yLabel: "°C",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="lf-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Température sur 12 mois",
  variant = "basic",
  labels = List("Jan", "Fév", "Mar", "Avr", "Mai", "Juin", "Juil", "Août", "Sep", "Oct", "Nov", "Déc"),
  values = List(2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0),
  y_label = "°C",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="lf-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Température sur 12 mois")
    .variant("basic")
    .labels({"Jan", "Fév", "Mar", "Avr", "Mai", "Juin", "Juil", "Août", "Sep", "Oct", "Nov", "Déc"})
    .values({2.0, 3.5, 7.0, 12.0, 17.0, 22.0, 25.5, 24.5, 19.0, 13.0, 7.5, 3.0})
    .yLabel("°C")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-basic.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-multi">

Plusieurs séries partageant le même axe x. Passez `series=[(nom, valeurs), ...]`.


<div class="sp-vmeta"><span><strong>Variante</strong> <code>"multi"</code></span><span><strong>Alias</strong> <code>"multi"</code> / <code>"multiline"</code> / <code>"multiple"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="lf-multi">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('lf-multi','lf-multi-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('lf-multi','lf-multi-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('lf-multi','lf-multi-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('lf-multi','lf-multi-r',this)">R</button>
<button class="sp-tb" onclick="spTab('lf-multi','lf-multi-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('lf-multi','lf-multi-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('lf-multi','lf-multi-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('lf-multi','lf-multi-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('lf-multi','lf-multi-cpp',this)">C++</button>
</div>
<div id="lf-multi-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='CA trimestriel par région',
    variant='multi',
    x_labels=['T1', 'T2', 'T3', 'T4'],
    series=[("UE", [120, 145, 160, 180]), ("NA", [90, 110, 135, 150]), ("APAC", [60, 75, 80, 95])],
    y_label='CA (M€)',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="lf-multi-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "CA trimestriel par région",
  variant: "multi",
  xLabels: ["T1", "T2", "T3", "T4"],
  series: [["UE", [120, 145, 160, 180]], ["NA", [90, 110, 135, 150]], ["APAC", [60, 75, 80, 95]]],
  yLabel: "CA (M€)",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-multi-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "CA trimestriel par région",
  variant: "multi",
  xLabels: ["T1", "T2", "T3", "T4"],
  series: [["UE", [120, 145, 160, 180]], ["NA", [90, 110, 135, 150]], ["APAC", [60, 75, 80, 95]]],
  yLabel: "CA (M€)",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-multi-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "CA trimestriel par région",
  variant = "multi",
  x_labels = c("T1", "T2", "T3", "T4"),
  series = list(list("UE", c(120, 145, 160, 180)), list("NA", c(90, 110, 135, 150)), list("APAC", c(60, 75, 80, 95))),
  y_label = "CA (M€)",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="lf-multi-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("CA trimestriel par région")
        .variant("multi")
        .x_labels(vec!["T1", "T2", "T3", "T4"])
        .series(vec![("UE".into(), vec![120.0, 145.0, 160.0, 180.0]), ("NA".into(), vec![90.0, 110.0, 135.0, 150.0]), ("APAC".into(), vec![60.0, 75.0, 80.0, 95.0])])
        .y_label("CA (M€)")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="lf-multi-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("CA trimestriel par région")
    .variant("multi")
    .xLabels(List.of("T1", "T2", "T3", "T4"))
    .series(List.of(Map.entry("UE", List.of(120.0, 145.0, 160.0, 180.0)), Map.entry("NA", List.of(90.0, 110.0, 135.0, 150.0)), Map.entry("APAC", List.of(60.0, 75.0, 80.0, 95.0))))
    .yLabel("CA (M€)")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="lf-multi-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "CA trimestriel par région",
    variant: "multi",
    xLabels: new[] { "T1", "T2", "T3", "T4" },
    series: new[] { ("UE", new[] { 120, 145, 160, 180 }), ("NA", new[] { 90, 110, 135, 150 }), ("APAC", new[] { 60, 75, 80, 95 }) },
    yLabel: "CA (M€)",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="lf-multi-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "CA trimestriel par région",
  variant = "multi",
  x_labels = List("T1", "T2", "T3", "T4"),
  series = List(("UE", List(120.0, 145.0, 160.0, 180.0)), ("NA", List(90.0, 110.0, 135.0, 150.0)), ("APAC", List(60.0, 75.0, 80.0, 95.0))),
  y_label = "CA (M€)",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="lf-multi-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("CA trimestriel par région")
    .variant("multi")
    .xLabels({"T1", "T2", "T3", "T4"})
    .series({{"UE", {120, 145, 160, 180}}, {"NA", {90, 110, 135, 150}}, {"APAC", {60, 75, 80, 95}}})
    .yLabel("CA (M€)")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-multi.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-stepped">

Ligne en escalier — idéale pour des données constantes par morceaux. `step_shape` contrôle l'orientation des marches.


<div class="sp-vmeta"><span><strong>Variante</strong> <code>"stepped"</code></span><span><strong>Alias</strong> <code>"stepped"</code> / <code>"step"</code> / <code>"hv"</code> / <code>"vh"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="lf-stepped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('lf-stepped','lf-stepped-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('lf-stepped','lf-stepped-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('lf-stepped','lf-stepped-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('lf-stepped','lf-stepped-r',this)">R</button>
<button class="sp-tb" onclick="spTab('lf-stepped','lf-stepped-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('lf-stepped','lf-stepped-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('lf-stepped','lf-stepped-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('lf-stepped','lf-stepped-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('lf-stepped','lf-stepped-cpp',this)">C++</button>
</div>
<div id="lf-stepped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Charge serveur (escalier)',
    variant='stepped',
    labels=['00:00', '04:00', '08:00', '12:00', '16:00', '20:00'],
    values=[12.0, 8.0, 35.0, 52.0, 40.0, 18.0],
    step_shape='hv',
    y_label='% CPU',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="lf-stepped-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Charge serveur (escalier)",
  variant: "stepped",
  labels: ["00:00", "04:00", "08:00", "12:00", "16:00", "20:00"],
  values: [12.0, 8.0, 35.0, 52.0, 40.0, 18.0],
  stepShape: "hv",
  yLabel: "% CPU",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-stepped-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Charge serveur (escalier)",
  variant: "stepped",
  labels: ["00:00", "04:00", "08:00", "12:00", "16:00", "20:00"],
  values: [12.0, 8.0, 35.0, 52.0, 40.0, 18.0],
  stepShape: "hv",
  yLabel: "% CPU",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-stepped-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Charge serveur (escalier)",
  variant = "stepped",
  labels = c("00:00", "04:00", "08:00", "12:00", "16:00", "20:00"),
  values = c(12.0, 8.0, 35.0, 52.0, 40.0, 18.0),
  step_shape = "hv",
  y_label = "% CPU",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="lf-stepped-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Charge serveur (escalier)")
        .variant("stepped")
        .labels(vec!["00:00", "04:00", "08:00", "12:00", "16:00", "20:00"])
        .values(vec![12.0, 8.0, 35.0, 52.0, 40.0, 18.0])
        .step_shape("hv")
        .y_label("% CPU")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="lf-stepped-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Charge serveur (escalier)")
    .variant("stepped")
    .labels(List.of("00:00", "04:00", "08:00", "12:00", "16:00", "20:00"))
    .values(List.of(12.0, 8.0, 35.0, 52.0, 40.0, 18.0))
    .stepShape("hv")
    .yLabel("% CPU")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="lf-stepped-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Charge serveur (escalier)",
    variant: "stepped",
    labels: new[] { "00:00", "04:00", "08:00", "12:00", "16:00", "20:00" },
    values: new[] { 12.0, 8.0, 35.0, 52.0, 40.0, 18.0 },
    stepShape: "hv",
    yLabel: "% CPU",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="lf-stepped-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Charge serveur (escalier)",
  variant = "stepped",
  labels = List("00:00", "04:00", "08:00", "12:00", "16:00", "20:00"),
  values = List(12.0, 8.0, 35.0, 52.0, 40.0, 18.0),
  step_shape = "hv",
  y_label = "% CPU",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="lf-stepped-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Charge serveur (escalier)")
    .variant("stepped")
    .labels({"00:00", "04:00", "08:00", "12:00", "16:00", "20:00"})
    .values({12.0, 8.0, 35.0, 52.0, 40.0, 18.0})
    .stepShape("hv")
    .yLabel("% CPU")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-stepped.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-spline">

Courbe Catmull-Rom lissée. `spline_tension` (0–1) contrôle l'adhérence de la courbe aux points.


<div class="sp-vmeta"><span><strong>Variante</strong> <code>"spline"</code></span><span><strong>Alias</strong> <code>"spline"</code> / <code>"smooth"</code> / <code>"curved"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="lf-spline">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('lf-spline','lf-spline-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('lf-spline','lf-spline-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('lf-spline','lf-spline-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('lf-spline','lf-spline-r',this)">R</button>
<button class="sp-tb" onclick="spTab('lf-spline','lf-spline-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('lf-spline','lf-spline-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('lf-spline','lf-spline-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('lf-spline','lf-spline-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('lf-spline','lf-spline-cpp',this)">C++</button>
</div>
<div id="lf-spline-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Tendance des ventes (lissée)',
    variant='spline',
    labels=['S1', 'S2', 'S3', 'S4', 'S5', 'S6', 'S7', 'S8'],
    values=[120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0],
    spline_tension=0.5,
    y_label='Unités',
)
chart.show()</code></pre></div>
<div id="lf-spline-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Tendance des ventes (lissée)",
  variant: "spline",
  labels: ["S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8"],
  values: [120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0],
  splineTension: 0.5,
  yLabel: "Unités"
});
chart.show();</code></pre></div>
<div id="lf-spline-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Tendance des ventes (lissée)",
  variant: "spline",
  labels: ["S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8"],
  values: [120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0],
  splineTension: 0.5,
  yLabel: "Unités"
});
chart.show();</code></pre></div>
<div id="lf-spline-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Tendance des ventes (lissée)",
  variant = "spline",
  labels = c("S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8"),
  values = c(120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0),
  spline_tension = 0.5,
  y_label = "Unités"
)
chart$show()</code></pre></div>
<div id="lf-spline-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Tendance des ventes (lissée)")
        .variant("spline")
        .labels(vec!["S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8"])
        .values(vec![120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0])
        .spline_tension(0.5)
        .y_label("Unités")
        .build();
    chart.show();
}</code></pre></div>
<div id="lf-spline-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Tendance des ventes (lissée)")
    .variant("spline")
    .labels(List.of("S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8"))
    .values(List.of(120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0))
    .splineTension(0.5)
    .yLabel("Unités")
    .build();
chart.show();</code></pre></div>
<div id="lf-spline-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Tendance des ventes (lissée)",
    variant: "spline",
    labels: new[] { "S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8" },
    values: new[] { 120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0 },
    splineTension: 0.5,
    yLabel: "Unités"
);
chart.Show();</code></pre></div>
<div id="lf-spline-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Tendance des ventes (lissée)",
  variant = "spline",
  labels = List("S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8"),
  values = List(120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0),
  spline_tension = 0.5,
  y_label = "Unités"
)
chart.show()</code></pre></div>
<div id="lf-spline-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Tendance des ventes (lissée)")
    .variant("spline")
    .labels({"S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8"})
    .values({120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0, 225.0})
    .splineTension(0.5)
    .yLabel("Unités")
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-spline.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-filled">

Graphique en aire — remplit la zone sous la ligne. `fill_opacity` règle la transparence ; `stack_fill=True` empile plusieurs séries.


<div class="sp-vmeta"><span><strong>Variante</strong> <code>"filled"</code></span><span><strong>Alias</strong> <code>"filled"</code> / <code>"area"</code> / <code>"fill"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="lf-filled">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('lf-filled','lf-filled-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('lf-filled','lf-filled-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('lf-filled','lf-filled-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('lf-filled','lf-filled-r',this)">R</button>
<button class="sp-tb" onclick="spTab('lf-filled','lf-filled-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('lf-filled','lf-filled-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('lf-filled','lf-filled-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('lf-filled','lf-filled-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('lf-filled','lf-filled-cpp',this)">C++</button>
</div>
<div id="lf-filled-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Utilisateurs actifs par jour',
    variant='filled',
    labels=['Lun', 'Mar', 'Mer', 'Jeu', 'Ven', 'Sam', 'Dim'],
    values=[320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0],
    fill_opacity=0.35,
    y_label='DAU',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="lf-filled-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Utilisateurs actifs par jour",
  variant: "filled",
  labels: ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"],
  values: [320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0],
  fillOpacity: 0.35,
  yLabel: "DAU",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-filled-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Utilisateurs actifs par jour",
  variant: "filled",
  labels: ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"],
  values: [320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0],
  fillOpacity: 0.35,
  yLabel: "DAU",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-filled-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Utilisateurs actifs par jour",
  variant = "filled",
  labels = c("Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"),
  values = c(320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0),
  fill_opacity = 0.35,
  y_label = "DAU",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="lf-filled-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Utilisateurs actifs par jour")
        .variant("filled")
        .labels(vec!["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"])
        .values(vec![320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0])
        .fill_opacity(0.35)
        .y_label("DAU")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="lf-filled-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Utilisateurs actifs par jour")
    .variant("filled")
    .labels(List.of("Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"))
    .values(List.of(320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0))
    .fillOpacity(0.35)
    .yLabel("DAU")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="lf-filled-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Utilisateurs actifs par jour",
    variant: "filled",
    labels: new[] { "Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim" },
    values: new[] { 320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0 },
    fillOpacity: 0.35,
    yLabel: "DAU",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="lf-filled-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Utilisateurs actifs par jour",
  variant = "filled",
  labels = List("Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"),
  values = List(320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0),
  fill_opacity = 0.35,
  y_label = "DAU",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="lf-filled-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Utilisateurs actifs par jour")
    .variant("filled")
    .labels({"Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"})
    .values({320.0, 410.0, 395.0, 460.0, 520.0, 310.0, 280.0})
    .fillOpacity(0.35)
    .yLabel("DAU")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-filled.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-sparkline">

Petit graphique inline — sans axes, idéal pour les tableaux de bord. `spark_cols` arrange plusieurs séries dans une grille.


<div class="sp-vmeta"><span><strong>Variante</strong> <code>"sparkline"</code></span><span><strong>Alias</strong> <code>"sparkline"</code> / <code>"spark"</code> / <code>"tiny"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="lf-spark">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('lf-spark','lf-spark-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('lf-spark','lf-spark-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('lf-spark','lf-spark-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('lf-spark','lf-spark-r',this)">R</button>
<button class="sp-tb" onclick="spTab('lf-spark','lf-spark-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('lf-spark','lf-spark-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('lf-spark','lf-spark-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('lf-spark','lf-spark-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('lf-spark','lf-spark-cpp',this)">C++</button>
</div>
<div id="lf-spark-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Résumé des KPI',
    variant='sparkline',
    series=[("CA", [120, 135, 128, 160, 180, 175, 210]), ("Utilisateurs", [80, 90, 85, 110, 130, 125, 145]), ("Erreurs", [5, 3, 4, 2, 1, 2, 1])],
    spark_cols=3,
    spark_cell_w=220,
    spark_cell_h=60,
)
chart.show()</code></pre></div>
<div id="lf-spark-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Résumé des KPI",
  variant: "sparkline",
  series: [["CA", [120, 135, 128, 160, 180, 175, 210]], ["Utilisateurs", [80, 90, 85, 110, 130, 125, 145]], ["Erreurs", [5, 3, 4, 2, 1, 2, 1]]],
  sparkCols: 3,
  sparkCellW: 220,
  sparkCellH: 60
});
chart.show();</code></pre></div>
<div id="lf-spark-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Résumé des KPI",
  variant: "sparkline",
  series: [["CA", [120, 135, 128, 160, 180, 175, 210]], ["Utilisateurs", [80, 90, 85, 110, 130, 125, 145]], ["Erreurs", [5, 3, 4, 2, 1, 2, 1]]],
  sparkCols: 3,
  sparkCellW: 220,
  sparkCellH: 60
});
chart.show();</code></pre></div>
<div id="lf-spark-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Résumé des KPI",
  variant = "sparkline",
  series = list(list("CA", c(120, 135, 128, 160, 180, 175, 210)), list("Utilisateurs", c(80, 90, 85, 110, 130, 125, 145)), list("Erreurs", c(5, 3, 4, 2, 1, 2, 1))),
  spark_cols = 3,
  spark_cell_w = 220,
  spark_cell_h = 60
)
chart$show()</code></pre></div>
<div id="lf-spark-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Résumé des KPI")
        .variant("sparkline")
        .series(vec![("CA".into(), vec![120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0]), ("Utilisateurs".into(), vec![80.0, 90.0, 85.0, 110.0, 130.0, 125.0, 145.0]), ("Erreurs".into(), vec![5.0, 3.0, 4.0, 2.0, 1.0, 2.0, 1.0])])
        .spark_cols(3)
        .spark_cell_w(220)
        .spark_cell_h(60)
        .build();
    chart.show();
}</code></pre></div>
<div id="lf-spark-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Résumé des KPI")
    .variant("sparkline")
    .series(List.of(Map.entry("CA", List.of(120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0)), Map.entry("Utilisateurs", List.of(80.0, 90.0, 85.0, 110.0, 130.0, 125.0, 145.0)), Map.entry("Erreurs", List.of(5.0, 3.0, 4.0, 2.0, 1.0, 2.0, 1.0))))
    .sparkCols(3)
    .sparkCellW(220)
    .sparkCellH(60)
    .build();
chart.show();</code></pre></div>
<div id="lf-spark-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Résumé des KPI",
    variant: "sparkline",
    series: new[] { ("CA", new[] { 120, 135, 128, 160, 180, 175, 210 }), ("Utilisateurs", new[] { 80, 90, 85, 110, 130, 125, 145 }), ("Erreurs", new[] { 5, 3, 4, 2, 1, 2, 1 }) },
    sparkCols: 3,
    sparkCellW: 220,
    sparkCellH: 60
);
chart.Show();</code></pre></div>
<div id="lf-spark-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Résumé des KPI",
  variant = "sparkline",
  series = List(("CA", List(120.0, 135.0, 128.0, 160.0, 180.0, 175.0, 210.0)), ("Utilisateurs", List(80.0, 90.0, 85.0, 110.0, 130.0, 125.0, 145.0)), ("Erreurs", List(5.0, 3.0, 4.0, 2.0, 1.0, 2.0, 1.0))),
  spark_cols = 3,
  spark_cell_w = 220,
  spark_cell_h = 60
)
chart.show()</code></pre></div>
<div id="lf-spark-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Résumé des KPI")
    .variant("sparkline")
    .series({{"CA", {120, 135, 128, 160, 180, 175, 210}}, {"Utilisateurs", {80, 90, 85, 110, 130, 125, 145}}, {"Erreurs", {5, 3, 4, 2, 1, 2, 1}}})
    .sparkCols(3)
    .sparkCellW(220)
    .sparkCellH(60)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-sparkline.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-dashed">

Motif de ligne personnalisé. `dash_pattern="8,4"` signifie 8px de trait, 4px de vide. Utilisez `"2,3"` pour pointillé.


<div class="sp-vmeta"><span><strong>Variante</strong> <code>"dashed"</code></span><span><strong>Alias</strong> <code>"dashed"</code> / <code>"dotted"</code> / <code>"styled"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="lf-dashed">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('lf-dashed','lf-dashed-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('lf-dashed','lf-dashed-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('lf-dashed','lf-dashed-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('lf-dashed','lf-dashed-r',this)">R</button>
<button class="sp-tb" onclick="spTab('lf-dashed','lf-dashed-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('lf-dashed','lf-dashed-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('lf-dashed','lf-dashed-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('lf-dashed','lf-dashed-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('lf-dashed','lf-dashed-cpp',this)">C++</button>
</div>
<div id="lf-dashed-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Prévision vs réel',
    variant='dashed',
    labels=['Jan', 'Fév', 'Mar', 'Avr', 'Mai', 'Juin'],
    values=[10.0, 18.0, 22.0, 30.0, 38.0, 45.0],
    dash_pattern='8,4',
    stroke_width=2.5,
    y_label='Unités',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="lf-dashed-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Prévision vs réel",
  variant: "dashed",
  labels: ["Jan", "Fév", "Mar", "Avr", "Mai", "Juin"],
  values: [10.0, 18.0, 22.0, 30.0, 38.0, 45.0],
  dashPattern: "8,4",
  strokeWidth: 2.5,
  yLabel: "Unités",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-dashed-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Prévision vs réel",
  variant: "dashed",
  labels: ["Jan", "Fév", "Mar", "Avr", "Mai", "Juin"],
  values: [10.0, 18.0, 22.0, 30.0, 38.0, 45.0],
  dashPattern: "8,4",
  strokeWidth: 2.5,
  yLabel: "Unités",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-dashed-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Prévision vs réel",
  variant = "dashed",
  labels = c("Jan", "Fév", "Mar", "Avr", "Mai", "Juin"),
  values = c(10.0, 18.0, 22.0, 30.0, 38.0, 45.0),
  dash_pattern = "8,4",
  stroke_width = 2.5,
  y_label = "Unités",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="lf-dashed-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Prévision vs réel")
        .variant("dashed")
        .labels(vec!["Jan", "Fév", "Mar", "Avr", "Mai", "Juin"])
        .values(vec![10.0, 18.0, 22.0, 30.0, 38.0, 45.0])
        .dash_pattern("8,4")
        .stroke_width(2.5)
        .y_label("Unités")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="lf-dashed-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Prévision vs réel")
    .variant("dashed")
    .labels(List.of("Jan", "Fév", "Mar", "Avr", "Mai", "Juin"))
    .values(List.of(10.0, 18.0, 22.0, 30.0, 38.0, 45.0))
    .dashPattern("8,4")
    .strokeWidth(2.5)
    .yLabel("Unités")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="lf-dashed-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Prévision vs réel",
    variant: "dashed",
    labels: new[] { "Jan", "Fév", "Mar", "Avr", "Mai", "Juin" },
    values: new[] { 10.0, 18.0, 22.0, 30.0, 38.0, 45.0 },
    dashPattern: "8,4",
    strokeWidth: 2.5,
    yLabel: "Unités",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="lf-dashed-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Prévision vs réel",
  variant = "dashed",
  labels = List("Jan", "Fév", "Mar", "Avr", "Mai", "Juin"),
  values = List(10.0, 18.0, 22.0, 30.0, 38.0, 45.0),
  dash_pattern = "8,4",
  stroke_width = 2.5,
  y_label = "Unités",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="lf-dashed-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Prévision vs réel")
    .variant("dashed")
    .labels({"Jan", "Fév", "Mar", "Avr", "Mai", "Juin"})
    .values({10.0, 18.0, 22.0, 30.0, 38.0, 45.0})
    .dashPattern("8,4")
    .strokeWidth(2.5)
    .yLabel("Unités")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-dashed.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-connected_scatter">

Ligne avec marqueurs visibles. `marker_size` (px) règle la taille des points ; `show_points=True` est implicite.


<div class="sp-vmeta"><span><strong>Variante</strong> <code>"connected_scatter"</code></span><span><strong>Alias</strong> <code>"connected_scatter"</code> / <code>"markers"</code> / <code>"lines+markers"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="lf-connected">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('lf-connected','lf-connected-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('lf-connected','lf-connected-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('lf-connected','lf-connected-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('lf-connected','lf-connected-r',this)">R</button>
<button class="sp-tb" onclick="spTab('lf-connected','lf-connected-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('lf-connected','lf-connected-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('lf-connected','lf-connected-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('lf-connected','lf-connected-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('lf-connected','lf-connected-cpp',this)">C++</button>
</div>
<div id="lf-connected-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title="Mesures d'expérience",
    variant='connected_scatter',
    labels=['t0', 't1', 't2', 't3', 't4', 't5', 't6'],
    values=[4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3],
    marker_size=7,
    y_label='mV',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="lf-connected-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Mesures d'expérience",
  variant: "connected_scatter",
  labels: ["t0", "t1", "t2", "t3", "t4", "t5", "t6"],
  values: [4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3],
  markerSize: 7,
  yLabel: "mV",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-connected-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Mesures d'expérience",
  variant: "connected_scatter",
  labels: ["t0", "t1", "t2", "t3", "t4", "t5", "t6"],
  values: [4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3],
  markerSize: 7,
  yLabel: "mV",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-connected-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Mesures d'expérience",
  variant = "connected_scatter",
  labels = c("t0", "t1", "t2", "t3", "t4", "t5", "t6"),
  values = c(4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3),
  marker_size = 7,
  y_label = "mV",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="lf-connected-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Mesures d'expérience")
        .variant("connected_scatter")
        .labels(vec!["t0", "t1", "t2", "t3", "t4", "t5", "t6"])
        .values(vec![4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3])
        .marker_size(7)
        .y_label("mV")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="lf-connected-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Mesures d'expérience")
    .variant("connected_scatter")
    .labels(List.of("t0", "t1", "t2", "t3", "t4", "t5", "t6"))
    .values(List.of(4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3))
    .markerSize(7)
    .yLabel("mV")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="lf-connected-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Mesures d'expérience",
    variant: "connected_scatter",
    labels: new[] { "t0", "t1", "t2", "t3", "t4", "t5", "t6" },
    values: new[] { 4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3 },
    markerSize: 7,
    yLabel: "mV",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="lf-connected-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Mesures d'expérience",
  variant = "connected_scatter",
  labels = List("t0", "t1", "t2", "t3", "t4", "t5", "t6"),
  values = List(4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3),
  marker_size = 7,
  y_label = "mV",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="lf-connected-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Mesures d'expérience")
    .variant("connected_scatter")
    .labels({"t0", "t1", "t2", "t3", "t4", "t5", "t6"})
    .values({4.2, 5.8, 7.1, 6.5, 8.4, 9.0, 10.3})
    .markerSize(7)
    .yLabel("mV")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-connected_scatter.html"></iframe>
</div>

<div class="sp-variant" id="line-fr-gapped">

Rupture de ligne lorsque les valeurs dépassent `gap_threshold`. Utile pour des séries temporelles avec échantillons manquants.


<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gapped"</code></span><span><strong>Alias</strong> <code>"gapped"</code> / <code>"gaps"</code> / <code>"missing"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="lf-gapped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('lf-gapped','lf-gapped-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('lf-gapped','lf-gapped-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('lf-gapped','lf-gapped-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('lf-gapped','lf-gapped-r',this)">R</button>
<button class="sp-tb" onclick="spTab('lf-gapped','lf-gapped-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('lf-gapped','lf-gapped-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('lf-gapped','lf-gapped-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('lf-gapped','lf-gapped-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('lf-gapped','lf-gapped-cpp',this)">C++</button>
</div>
<div id="lf-gapped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title='Signal capteur (avec coupures)',
    variant='gapped',
    labels=['t0', 't1', 't2', 't3', 't4', 't5', 't6', 't7', 't8'],
    values=[3.2, 3.5, 3.4, float("nan"), float("nan"), 4.1, 4.5, 4.3, 4.8],
    gap_threshold=1.0,
    y_label='V',
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="lf-gapped-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Signal capteur (avec coupures)",
  variant: "gapped",
  labels: ["t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"],
  values: [3.2, 3.5, 3.4, NaN, NaN, 4.1, 4.5, 4.3, 4.8],
  gapThreshold: 1.0,
  yLabel: "V",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-gapped-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Signal capteur (avec coupures)",
  variant: "gapped",
  labels: ["t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"],
  values: [3.2, 3.5, 3.4, NaN, NaN, 4.1, 4.5, 4.3, 4.8],
  gapThreshold: 1.0,
  yLabel: "V",
  gridlines: true
});
chart.show();</code></pre></div>
<div id="lf-gapped-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$line(
  title = "Signal capteur (avec coupures)",
  variant = "gapped",
  labels = c("t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"),
  values = c(3.2, 3.5, 3.4, NA, NA, 4.1, 4.5, 4.3, 4.8),
  gap_threshold = 1.0,
  y_label = "V",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="lf-gapped-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::line()
        .title("Signal capteur (avec coupures)")
        .variant("gapped")
        .labels(vec!["t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"])
        .values(vec![3.2, 3.5, 3.4, f64::NAN, f64::NAN, 4.1, 4.5, 4.3, 4.8])
        .gap_threshold(1.0)
        .y_label("V")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="lf-gapped-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Map;

var chart = SeraPlot.line()
    .title("Signal capteur (avec coupures)")
    .variant("gapped")
    .labels(List.of("t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"))
    .values(List.of(3.2, 3.5, 3.4, Double.NaN, Double.NaN, 4.1, 4.5, 4.3, 4.8))
    .gapThreshold(1.0)
    .yLabel("V")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="lf-gapped-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Signal capteur (avec coupures)",
    variant: "gapped",
    labels: new[] { "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8" },
    values: new[] { 3.2, 3.5, 3.4, double.NaN, double.NaN, 4.1, 4.5, 4.3, 4.8 },
    gapThreshold: 1.0,
    yLabel: "V",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="lf-gapped-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.line(
  title = "Signal capteur (avec coupures)",
  variant = "gapped",
  labels = List("t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"),
  values = List(3.2, 3.5, 3.4, Double.NaN, Double.NaN, 4.1, 4.5, 4.3, 4.8),
  gap_threshold = 1.0,
  y_label = "V",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="lf-gapped-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot.hpp>

auto chart = sp::line()
    .title("Signal capteur (avec coupures)")
    .variant("gapped")
    .labels({"t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8"})
    .values({3.2, 3.5, 3.4, std::nan(""), std::nan(""), 4.1, 4.5, 4.3, 4.8})
    .gapThreshold(1.0)
    .yLabel("V")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/line-gapped.html"></iframe>
</div>

</div>
</div>

</div>
