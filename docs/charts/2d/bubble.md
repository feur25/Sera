# Bubble Charts

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

`sp.bubble(title, x_values, y_values, sizes, *, variant="basic", categories=None, labels=None, color_values=None, **kwargs) -> Chart`

Aliases: `sp.bubble`, `sp.bubbles`, `sp.bubble_unified`, `sp.bubble_family`

## Description

`sp.bubble()` is the unified entry point for the entire bubble-chart family. The `variant` keyword selects the rendering strategy — all other arguments remain consistent across variants. A bubble chart extends a 2D scatter plot with a third numeric dimension represented as bubble **area** (not radius), following best practices for perceptual accuracy.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Single-color bubbles, 3rd dim = size | `x_values`, `y_values`, `sizes` |
| `"categorical"` / `"grouped"` | Color-by-group with side legend | `categories` |
| `"gradient"` / `"colorscale"` | Continuous color mapping + colorbar | `color_values`, `color_low`, `color_high` |
| `"labeled"` / `"text"` | Always-on per-point text labels | `labels` |
| `"outlined"` / `"hollow"` | Hollow rings — great for overlap | `categories`, `stroke_width` |
| `"negative"` / `"signed"` | Diverging signed sizes (e.g. P&L) | signed `sizes`, `color_low`, `color_high` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | required | all | Chart title |
| `x_values` | `list[float]` | required | all | Horizontal position of each bubble |
| `y_values` | `list[float]` | required | all | Vertical position of each bubble |
| `sizes` | `list[float]` | required | all | Raw size values mapped to bubble area (signed in `negative`) |
| `variant` | `str` | `"basic"` | — | Rendering variant (see table above) |
| `categories` | `list[str]` | `None` | categorical, outlined, labeled | Per-point group name for color partitioning |
| `labels` | `list[str]` | `None` | all (hover) / labeled (rendered) | Per-point text label |
| `color_values` | `list[float]` | `None` | gradient | Continuous numeric value driving colorscale |
| `color_hex` | `int` | `0` | basic | Single fill color as hex int (auto when `0`) |
| `color_low` | `int` | `0x6366F1` | gradient, negative | Low end of color scale |
| `color_high` | `int` | `0xF43F5E` | gradient, negative | High end of color scale |
| `min_size` | `float` | `4.0` | all | Minimum bubble radius in pixels |
| `max_size` | `float` | `40.0` | all | Maximum bubble radius in pixels |
| `show_text` | `bool` | `False` | all | Force always-on text labels (auto-on for `labeled`) |
| `stroke_width` | `float` | `1.5` | all | Bubble stroke width in pixels |
| `palette` | `list[int]` | `None` | categorical, outlined, labeled | Custom color palette for groups |
| `width` | `int` | `900` | all | Canvas width in pixels |
| `height` | `int` | `500` | all | Canvas height in pixels |
| `x_label` | `str` | `""` | all | X-axis label |
| `y_label` | `str` | `""` | all | Y-axis label |
| `gridlines` | `bool` | `False` | all | Show gridlines |
| `sort_order` | `str` | `"none"` | all | `"asc"`, `"desc"` or `"none"` — draw order by abs(size) |
| `legend_position` | `str` | `"right"` | categorical, outlined | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | all | Background CSS color; `None` = transparent |
| `no_x_axis` | `bool` | `False` | all | Hide X axis |
| `no_y_axis` | `bool` | `False` | all | Hide Y axis |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="bubble-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bubble-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bubble-en','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-en','categorical',this)"><span class="sp-cic">◓</span><span class="sp-clb">Categorical</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-en','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-en','labeled',this)"><span class="sp-cic">◉</span><span class="sp-clb">Labeled</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-en','outlined',this)"><span class="sp-cic">○</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-en','negative',this)"><span class="sp-cic">±</span><span class="sp-clb">Negative</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bubble-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic</code> / <code>simple</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bb-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bb-basic','bb-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bb-basic','bb-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bb-basic','bb-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bb-basic','bb-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bb-basic','bb-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bb-basic','bb-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bb-basic','bb-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bb-basic','bb-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bb-basic','bb-basic-cpp',this)">C++</button>
</div>
<div id="bb-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bubble(
    title="GDP vs Life expectancy",
    variant="basic",
    x_values=[2.5, 5.0, 12.0, 18.5, 25.0],
    y_values=[72, 75, 80, 82, 79],
    sizes=[500, 1200, 3000, 4500, 8000],
    labels=["Brazil","Mexico","Germany","Japan","USA"],
    x_label="GDP per capita (k$)",
    y_label="Life expectancy",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="bb-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bubble({
  title: "GDP vs Life expectancy", variant: "basic",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  labels:  ["Brazil","Mexico","Germany","Japan","USA"],
  xLabel: "GDP per capita (k$)", yLabel: "Life expectancy", gridlines: true,
});
chart.show();</code></pre></div>
<div id="bb-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bubble({
  title: "GDP vs Life expectancy", variant: "basic",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  labels:  ["Brazil","Mexico","Germany","Japan","USA"],
  xLabel: "GDP per capita (k$)", yLabel: "Life expectancy", gridlines: true,
});
chart.show();</code></pre></div>
<div id="bb-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bubble(
  title = "GDP vs Life expectancy",
  variant = "basic",
  x_values = c(2.5,5.0,12.0,18.5,25.0),
  y_values = c(72,75,80,82,79),
  sizes    = c(500,1200,3000,4500,8000),
  labels   = c("Brazil","Mexico","Germany","Japan","USA"),
  x_label = "GDP per capita (k$)",
  y_label = "Life expectancy",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="bb-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bubble()
        .title("GDP vs Life expectancy")
        .variant("basic")
        .x_values(vec![2.5,5.0,12.0,18.5,25.0])
        .y_values(vec![72.0,75.0,80.0,82.0,79.0])
        .sizes(vec![500.0,1200.0,3000.0,4500.0,8000.0])
        .labels(vec!["Brazil","Mexico","Germany","Japan","USA"])
        .x_label("GDP per capita (k$)")
        .y_label("Life expectancy")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="bb-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("GDP vs Life expectancy")
    .variant("basic")
    .xValues(List.of(2.5,5.0,12.0,18.5,25.0))
    .yValues(List.of(72.0,75.0,80.0,82.0,79.0))
    .sizes(List.of(500.0,1200.0,3000.0,4500.0,8000.0))
    .labels(List.of("Brazil","Mexico","Germany","Japan","USA"))
    .xLabel("GDP per capita (k$)")
    .yLabel("Life expectancy")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="bb-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "GDP vs Life expectancy",
    variant: "basic",
    xValues: [2.5,5.0,12.0,18.5,25.0],
    yValues: [72,75,80,82,79],
    sizes:   [500,1200,3000,4500,8000],
    labels:  ["Brazil","Mexico","Germany","Japan","USA"],
    xLabel: "GDP per capita (k$)",
    yLabel: "Life expectancy",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="bb-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bubble(
  title    = "GDP vs Life expectancy",
  variant  = "basic",
  x_values = List(2.5,5.0,12.0,18.5,25.0),
  y_values = List(72.0,75.0,80.0,82.0,79.0),
  sizes    = List(500.0,1200.0,3000.0,4500.0,8000.0),
  labels   = List("Brazil","Mexico","Germany","Japan","USA"),
  x_label  = "GDP per capita (k$)",
  y_label  = "Life expectancy",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="bb-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
    .title    = "GDP vs Life expectancy",
    .variant  = "basic",
    .x_values = {2.5,5.0,12.0,18.5,25.0},
    .y_values = {72,75,80,82,79},
    .sizes    = {500,1200,3000,4500,8000},
    .labels   = {"Brazil","Mexico","Germany","Japan","USA"},
    .x_label  = "GDP per capita (k$)",
    .y_label  = "Life expectancy",
    .gridlines = true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-basic.html"></iframe>
</div>

<div class="sp-variant" id="bubble-en-categorical">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code> / <code>"grouped"</code></span><span><strong>Aliases</strong> <code>category</code> / <code>groups</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bb-cat">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bb-cat','bb-cat-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bb-cat','bb-cat-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bb-cat','bb-cat-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bb-cat','bb-cat-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bb-cat','bb-cat-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bb-cat','bb-cat-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bb-cat','bb-cat-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bb-cat','bb-cat-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bb-cat','bb-cat-cpp',this)">C++</button>
</div>
<div id="bb-cat-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bubble(
    title="GDP vs Life expectancy by region",
    variant="categorical",
    x_values=[2.5,5.0,12.0,18.5,25.0,3.2,8.0,14.0],
    y_values=[72,75,80,82,79,68,77,81],
    sizes=[500,1200,3000,4500,8000,800,2100,3600],
    labels=["Brazil","Mexico","Germany","Japan","USA","Nigeria","Turkey","France"],
    categories=["Americas","Americas","Europe","Asia","Americas","Africa","Europe","Europe"],
    x_label="GDP per capita (k$)", y_label="Life expectancy", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bb-cat-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bubble({
  title: "GDP vs Life expectancy by region", variant: "categorical",
  xValues: [2.5,5.0,12.0,18.5,25.0,3.2,8.0,14.0],
  yValues: [72,75,80,82,79,68,77,81],
  sizes:   [500,1200,3000,4500,8000,800,2100,3600],
  labels:  ["Brazil","Mexico","Germany","Japan","USA","Nigeria","Turkey","France"],
  categories: ["Americas","Americas","Europe","Asia","Americas","Africa","Europe","Europe"],
  xLabel: "GDP per capita (k$)", yLabel: "Life expectancy", gridlines: true,
});
chart.show();</code></pre></div>
<div id="bb-cat-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bubble({
  title: "GDP vs Life expectancy by region", variant: "categorical",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  categories: ["Americas","Americas","Europe","Asia","Americas"],
  xLabel: "GDP per capita (k$)", yLabel: "Life expectancy", gridlines: true,
});
chart.show();</code></pre></div>
<div id="bb-cat-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bubble(
  title = "GDP vs Life expectancy by region",
  variant = "categorical",
  x_values = c(2.5,5.0,12.0,18.5,25.0),
  y_values = c(72,75,80,82,79),
  sizes    = c(500,1200,3000,4500,8000),
  categories = c("Americas","Americas","Europe","Asia","Americas"),
  x_label = "GDP per capita (k$)", y_label = "Life expectancy", gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="bb-cat-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bubble()
        .title("GDP vs Life expectancy by region")
        .variant("categorical")
        .x_values(vec![2.5,5.0,12.0,18.5,25.0])
        .y_values(vec![72.0,75.0,80.0,82.0,79.0])
        .sizes(vec![500.0,1200.0,3000.0,4500.0,8000.0])
        .categories(vec!["Americas","Americas","Europe","Asia","Americas"])
        .x_label("GDP per capita (k$)")
        .y_label("Life expectancy")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="bb-cat-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("GDP vs Life expectancy by region")
    .variant("categorical")
    .xValues(List.of(2.5,5.0,12.0,18.5,25.0))
    .yValues(List.of(72.0,75.0,80.0,82.0,79.0))
    .sizes(List.of(500.0,1200.0,3000.0,4500.0,8000.0))
    .categories(List.of("Americas","Americas","Europe","Asia","Americas"))
    .xLabel("GDP per capita (k$)").yLabel("Life expectancy").gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="bb-cat-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "GDP vs Life expectancy by region",
    variant: "categorical",
    xValues: [2.5,5.0,12.0,18.5,25.0],
    yValues: [72,75,80,82,79],
    sizes:   [500,1200,3000,4500,8000],
    categories: ["Americas","Americas","Europe","Asia","Americas"],
    xLabel: "GDP per capita (k$)", yLabel: "Life expectancy", gridlines: true
);
chart.Show();</code></pre></div>
<div id="bb-cat-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bubble(
  title    = "GDP vs Life expectancy by region",
  variant  = "categorical",
  x_values = List(2.5,5.0,12.0,18.5,25.0),
  y_values = List(72.0,75.0,80.0,82.0,79.0),
  sizes    = List(500.0,1200.0,3000.0,4500.0,8000.0),
  categories = List("Americas","Americas","Europe","Asia","Americas"),
  x_label = "GDP per capita (k$)", y_label = "Life expectancy", gridlines = true
)
chart.show()</code></pre></div>
<div id="bb-cat-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
    .title    = "GDP vs Life expectancy by region",
    .variant  = "categorical",
    .x_values = {2.5,5.0,12.0,18.5,25.0},
    .y_values = {72,75,80,82,79},
    .sizes    = {500,1200,3000,4500,8000},
    .categories = {"Americas","Americas","Europe","Asia","Americas"},
    .x_label = "GDP per capita (k$)", .y_label = "Life expectancy", .gridlines = true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-categorical.html"></iframe>
</div>

<div class="sp-variant" id="bubble-en-gradient">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code> / <code>"colorscale"</code></span><span><strong>Aliases</strong> <code>continuous</code> / <code>scaled</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bb-grad">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bb-grad','bb-grad-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bb-grad','bb-grad-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bb-grad','bb-grad-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bb-grad','bb-grad-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bb-grad','bb-grad-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bb-grad','bb-grad-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bb-grad','bb-grad-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bb-grad','bb-grad-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bb-grad','bb-grad-cpp',this)">C++</button>
</div>
<div id="bb-grad-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bubble(
    title="Stations: temperature gradient",
    variant="gradient",
    x_values=[2.5,5.0,12.0,18.5,25.0,3.2,8.0,14.0],
    y_values=[72,75,80,82,79,68,77,81],
    sizes=[500,1200,3000,4500,8000,800,2100,3600],
    color_values=[12,18,41,55,63,8,24,47],
    color_low=0x0EA5E9, color_high=0xF43F5E,
    x_label="Longitude", y_label="Latitude", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bb-grad-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bubble({
  title: "Stations: temperature gradient", variant: "gradient",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  colorValues: [12,18,41,55,63],
  colorLow: 0x0EA5E9, colorHigh: 0xF43F5E,
  xLabel: "Longitude", yLabel: "Latitude", gridlines: true,
});
chart.show();</code></pre></div>
<div id="bb-grad-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bubble({
  title: "Stations: temperature gradient", variant: "gradient",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  colorValues: [12,18,41,55,63],
  colorLow: 0x0EA5E9, colorHigh: 0xF43F5E,
});
chart.show();</code></pre></div>
<div id="bb-grad-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bubble(
  title = "Stations: temperature gradient",
  variant = "gradient",
  x_values = c(2.5,5.0,12.0,18.5,25.0),
  y_values = c(72,75,80,82,79),
  sizes    = c(500,1200,3000,4500,8000),
  color_values = c(12,18,41,55,63),
  color_low = 0x0EA5E9, color_high = 0xF43F5E
)
chart$show()</code></pre></div>
<div id="bb-grad-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bubble()
        .title("Stations: temperature gradient")
        .variant("gradient")
        .x_values(vec![2.5,5.0,12.0,18.5,25.0])
        .y_values(vec![72.0,75.0,80.0,82.0,79.0])
        .sizes(vec![500.0,1200.0,3000.0,4500.0,8000.0])
        .color_values(vec![12.0,18.0,41.0,55.0,63.0])
        .color_low(0x0EA5E9).color_high(0xF43F5E)
        .build();
    chart.show();
}</code></pre></div>
<div id="bb-grad-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("Stations: temperature gradient")
    .variant("gradient")
    .xValues(List.of(2.5,5.0,12.0,18.5,25.0))
    .yValues(List.of(72.0,75.0,80.0,82.0,79.0))
    .sizes(List.of(500.0,1200.0,3000.0,4500.0,8000.0))
    .colorValues(List.of(12.0,18.0,41.0,55.0,63.0))
    .colorLow(0x0EA5E9).colorHigh(0xF43F5E)
    .build();
chart.show();</code></pre></div>
<div id="bb-grad-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "Stations: temperature gradient",
    variant: "gradient",
    xValues: [2.5,5.0,12.0,18.5,25.0],
    yValues: [72,75,80,82,79],
    sizes:   [500,1200,3000,4500,8000],
    colorValues: [12,18,41,55,63],
    colorLow: 0x0EA5E9, colorHigh: 0xF43F5E
);
chart.Show();</code></pre></div>
<div id="bb-grad-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bubble(
  title    = "Stations: temperature gradient",
  variant  = "gradient",
  x_values = List(2.5,5.0,12.0,18.5,25.0),
  y_values = List(72.0,75.0,80.0,82.0,79.0),
  sizes    = List(500.0,1200.0,3000.0,4500.0,8000.0),
  color_values = List(12.0,18.0,41.0,55.0,63.0),
  color_low = 0x0EA5E9, color_high = 0xF43F5E
)
chart.show()</code></pre></div>
<div id="bb-grad-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
    .title    = "Stations: temperature gradient",
    .variant  = "gradient",
    .x_values = {2.5,5.0,12.0,18.5,25.0},
    .y_values = {72,75,80,82,79},
    .sizes    = {500,1200,3000,4500,8000},
    .color_values = {12,18,41,55,63},
    .color_low = 0x0EA5E9, .color_high = 0xF43F5E,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-gradient.html"></iframe>
</div>

<div class="sp-variant" id="bubble-en-labeled">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"labeled"</code> / <code>"text"</code></span><span><strong>Aliases</strong> <code>labels</code> / <code>annotated</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bb-lbl">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bb-lbl','bb-lbl-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bb-lbl','bb-lbl-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bb-lbl','bb-lbl-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bb-lbl','bb-lbl-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bb-lbl','bb-lbl-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bb-lbl','bb-lbl-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bb-lbl','bb-lbl-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bb-lbl','bb-lbl-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bb-lbl','bb-lbl-cpp',this)">C++</button>
</div>
<div id="bb-lbl-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bubble(
    title="Top 6 economies",
    variant="labeled",
    x_values=[2.5,5.0,12.0,18.5,25.0,3.2],
    y_values=[72,75,80,82,79,68],
    sizes=[500,1200,3000,4500,8000,800],
    labels=["Brazil","Mexico","Germany","Japan","USA","Nigeria"],
    categories=["Americas","Americas","Europe","Asia","Americas","Africa"],
    x_label="GDP per capita (k$)", y_label="Life expectancy", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bb-lbl-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bubble({
  title: "Top 6 economies", variant: "labeled",
  xValues: [2.5,5.0,12.0,18.5,25.0,3.2],
  yValues: [72,75,80,82,79,68],
  sizes:   [500,1200,3000,4500,8000,800],
  labels:  ["Brazil","Mexico","Germany","Japan","USA","Nigeria"],
  categories: ["Americas","Americas","Europe","Asia","Americas","Africa"],
});
chart.show();</code></pre></div>
<div id="bb-lbl-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bubble({
  title: "Top 6 economies", variant: "labeled",
  xValues: [2.5,5.0,12.0,18.5,25.0,3.2],
  yValues: [72,75,80,82,79,68],
  sizes:   [500,1200,3000,4500,8000,800],
  labels:  ["Brazil","Mexico","Germany","Japan","USA","Nigeria"],
});
chart.show();</code></pre></div>
<div id="bb-lbl-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bubble(
  title = "Top 6 economies", variant = "labeled",
  x_values = c(2.5,5.0,12.0,18.5,25.0,3.2),
  y_values = c(72,75,80,82,79,68),
  sizes    = c(500,1200,3000,4500,8000,800),
  labels   = c("Brazil","Mexico","Germany","Japan","USA","Nigeria")
)
chart$show()</code></pre></div>
<div id="bb-lbl-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bubble()
        .title("Top 6 economies").variant("labeled")
        .x_values(vec![2.5,5.0,12.0,18.5,25.0,3.2])
        .y_values(vec![72.0,75.0,80.0,82.0,79.0,68.0])
        .sizes(vec![500.0,1200.0,3000.0,4500.0,8000.0,800.0])
        .labels(vec!["Brazil","Mexico","Germany","Japan","USA","Nigeria"])
        .build();
    chart.show();
}</code></pre></div>
<div id="bb-lbl-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("Top 6 economies").variant("labeled")
    .xValues(List.of(2.5,5.0,12.0,18.5,25.0,3.2))
    .yValues(List.of(72.0,75.0,80.0,82.0,79.0,68.0))
    .sizes(List.of(500.0,1200.0,3000.0,4500.0,8000.0,800.0))
    .labels(List.of("Brazil","Mexico","Germany","Japan","USA","Nigeria"))
    .build();
chart.show();</code></pre></div>
<div id="bb-lbl-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "Top 6 economies", variant: "labeled",
    xValues: [2.5,5.0,12.0,18.5,25.0,3.2],
    yValues: [72,75,80,82,79,68],
    sizes:   [500,1200,3000,4500,8000,800],
    labels:  ["Brazil","Mexico","Germany","Japan","USA","Nigeria"]
);
chart.Show();</code></pre></div>
<div id="bb-lbl-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bubble(
  title = "Top 6 economies", variant = "labeled",
  x_values = List(2.5,5.0,12.0,18.5,25.0,3.2),
  y_values = List(72.0,75.0,80.0,82.0,79.0,68.0),
  sizes    = List(500.0,1200.0,3000.0,4500.0,8000.0,800.0),
  labels   = List("Brazil","Mexico","Germany","Japan","USA","Nigeria")
)
chart.show()</code></pre></div>
<div id="bb-lbl-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
    .title = "Top 6 economies", .variant = "labeled",
    .x_values = {2.5,5.0,12.0,18.5,25.0,3.2},
    .y_values = {72,75,80,82,79,68},
    .sizes    = {500,1200,3000,4500,8000,800},
    .labels   = {"Brazil","Mexico","Germany","Japan","USA","Nigeria"},
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-labeled.html"></iframe>
</div>

<div class="sp-variant" id="bubble-en-outlined">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code> / <code>"hollow"</code></span><span><strong>Aliases</strong> <code>ring</code> / <code>open</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bb-out">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bb-out','bb-out-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bb-out','bb-out-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bb-out','bb-out-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bb-out','bb-out-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bb-out','bb-out-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bb-out','bb-out-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bb-out','bb-out-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bb-out','bb-out-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bb-out','bb-out-cpp',this)">C++</button>
</div>
<div id="bb-out-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bubble(
    title="Dense overlap → outlined",
    variant="outlined",
    x_values=[2.5,5.0,12.0,18.5,25.0,3.2,8.0,14.0],
    y_values=[72,75,80,82,79,68,77,81],
    sizes=[500,1200,3000,4500,8000,800,2100,3600],
    categories=["A","A","B","B","A","C","B","C"],
    stroke_width=3.0,
    x_label="X", y_label="Y", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bb-out-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bubble({
  title: "Dense overlap → outlined", variant: "outlined",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  categories: ["A","A","B","B","A"], strokeWidth: 3.0,
});
chart.show();</code></pre></div>
<div id="bb-out-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bubble({
  title: "Dense overlap → outlined", variant: "outlined",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  categories: ["A","A","B","B","A"], strokeWidth: 3.0,
});
chart.show();</code></pre></div>
<div id="bb-out-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bubble(
  title = "Dense overlap", variant = "outlined",
  x_values = c(2.5,5.0,12.0,18.5,25.0),
  y_values = c(72,75,80,82,79),
  sizes    = c(500,1200,3000,4500,8000),
  categories = c("A","A","B","B","A"), stroke_width = 3.0
)
chart$show()</code></pre></div>
<div id="bb-out-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bubble()
        .title("Dense overlap").variant("outlined")
        .x_values(vec![2.5,5.0,12.0,18.5,25.0])
        .y_values(vec![72.0,75.0,80.0,82.0,79.0])
        .sizes(vec![500.0,1200.0,3000.0,4500.0,8000.0])
        .categories(vec!["A","A","B","B","A"])
        .stroke_width(3.0)
        .build();
    chart.show();
}</code></pre></div>
<div id="bb-out-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("Dense overlap").variant("outlined")
    .xValues(List.of(2.5,5.0,12.0,18.5,25.0))
    .yValues(List.of(72.0,75.0,80.0,82.0,79.0))
    .sizes(List.of(500.0,1200.0,3000.0,4500.0,8000.0))
    .categories(List.of("A","A","B","B","A"))
    .strokeWidth(3.0)
    .build();
chart.show();</code></pre></div>
<div id="bb-out-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "Dense overlap", variant: "outlined",
    xValues: [2.5,5.0,12.0,18.5,25.0],
    yValues: [72,75,80,82,79],
    sizes:   [500,1200,3000,4500,8000],
    categories: ["A","A","B","B","A"], strokeWidth: 3.0
);
chart.Show();</code></pre></div>
<div id="bb-out-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bubble(
  title = "Dense overlap", variant = "outlined",
  x_values = List(2.5,5.0,12.0,18.5,25.0),
  y_values = List(72.0,75.0,80.0,82.0,79.0),
  sizes    = List(500.0,1200.0,3000.0,4500.0,8000.0),
  categories = List("A","A","B","B","A"),
  stroke_width = 3.0
)
chart.show()</code></pre></div>
<div id="bb-out-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
    .title = "Dense overlap", .variant = "outlined",
    .x_values = {2.5,5.0,12.0,18.5,25.0},
    .y_values = {72,75,80,82,79},
    .sizes    = {500,1200,3000,4500,8000},
    .categories = {"A","A","B","B","A"}, .stroke_width = 3.0,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-outlined.html"></iframe>
</div>

<div class="sp-variant" id="bubble-en-negative">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"negative"</code> / <code>"signed"</code></span><span><strong>Aliases</strong> <code>diverging</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bb-neg">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bb-neg','bb-neg-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bb-neg','bb-neg-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bb-neg','bb-neg-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bb-neg','bb-neg-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bb-neg','bb-neg-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bb-neg','bb-neg-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bb-neg','bb-neg-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bb-neg','bb-neg-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bb-neg','bb-neg-cpp',this)">C++</button>
</div>
<div id="bb-neg-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bubble(
    title="Signed P&amp;L exposure",
    variant="negative",
    x_values=[1,2,3,4,5,6,7,8,9,10],
    y_values=[12,-8,15,22,-14,9,-5,18,-11,25],
    sizes=[120,-85,150,220,-140,90,-55,180,-115,250],
    labels=["Q1","Q2","Q3","Q4","Q5","Q6","Q7","Q8","Q9","Q10"],
    color_low=0xEF4444, color_high=0x10B981,
    x_label="Period", y_label="Return (%)", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bb-neg-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bubble({
  title: "Signed P&L", variant: "negative",
  xValues: [1,2,3,4,5,6,7,8,9,10],
  yValues: [12,-8,15,22,-14,9,-5,18,-11,25],
  sizes:   [120,-85,150,220,-140,90,-55,180,-115,250],
  colorLow: 0xEF4444, colorHigh: 0x10B981,
});
chart.show();</code></pre></div>
<div id="bb-neg-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bubble({
  title: "Signed P&L", variant: "negative",
  xValues: [1,2,3,4,5,6,7,8,9,10],
  yValues: [12,-8,15,22,-14,9,-5,18,-11,25],
  sizes:   [120,-85,150,220,-140,90,-55,180,-115,250],
  colorLow: 0xEF4444, colorHigh: 0x10B981,
});
chart.show();</code></pre></div>
<div id="bb-neg-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bubble(
  title = "Signed P&L", variant = "negative",
  x_values = 1:10,
  y_values = c(12,-8,15,22,-14,9,-5,18,-11,25),
  sizes    = c(120,-85,150,220,-140,90,-55,180,-115,250),
  color_low = 0xEF4444, color_high = 0x10B981
)
chart$show()</code></pre></div>
<div id="bb-neg-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bubble()
        .title("Signed P&L").variant("negative")
        .x_values(vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0])
        .y_values(vec![12.0,-8.0,15.0,22.0,-14.0,9.0,-5.0,18.0,-11.0,25.0])
        .sizes(vec![120.0,-85.0,150.0,220.0,-140.0,90.0,-55.0,180.0,-115.0,250.0])
        .color_low(0xEF4444).color_high(0x10B981)
        .build();
    chart.show();
}</code></pre></div>
<div id="bb-neg-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("Signed P&L").variant("negative")
    .xValues(List.of(1,2,3,4,5,6,7,8,9,10))
    .yValues(List.of(12.0,-8.0,15.0,22.0,-14.0,9.0,-5.0,18.0,-11.0,25.0))
    .sizes(List.of(120.0,-85.0,150.0,220.0,-140.0,90.0,-55.0,180.0,-115.0,250.0))
    .colorLow(0xEF4444).colorHigh(0x10B981)
    .build();
chart.show();</code></pre></div>
<div id="bb-neg-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "Signed P&L", variant: "negative",
    xValues: [1,2,3,4,5,6,7,8,9,10],
    yValues: [12,-8,15,22,-14,9,-5,18,-11,25],
    sizes:   [120,-85,150,220,-140,90,-55,180,-115,250],
    colorLow: 0xEF4444, colorHigh: 0x10B981
);
chart.Show();</code></pre></div>
<div id="bb-neg-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bubble(
  title = "Signed P&L", variant = "negative",
  x_values = List(1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0),
  y_values = List(12.0,-8.0,15.0,22.0,-14.0,9.0,-5.0,18.0,-11.0,25.0),
  sizes    = List(120.0,-85.0,150.0,220.0,-140.0,90.0,-55.0,180.0,-115.0,250.0),
  color_low = 0xEF4444, color_high = 0x10B981
)
chart.show()</code></pre></div>
<div id="bb-neg-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
    .title = "Signed P&L", .variant = "negative",
    .x_values = {1,2,3,4,5,6,7,8,9,10},
    .y_values = {12,-8,15,22,-14,9,-5,18,-11,25},
    .sizes    = {120,-85,150,220,-140,90,-55,180,-115,250},
    .color_low = 0xEF4444, .color_high = 0x10B981,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-negative.html"></iframe>
</div>

</div>
</div>

---

## See also

- [Scatter](scatter.md) — `sp.scatter()`
- [Bubble 3D](../3d/bubble3d.md) — `sp.bubble3d()`
- [Bar](bar.md) — `sp.bar()`

</div><!-- /lang-en -->


<div class="lang-fr" style="display:none">

## Signature

`sp.bubble(title, x_values, y_values, sizes, *, variant="basic", categories=None, labels=None, color_values=None, **kwargs) -> Chart`

Alias : `sp.bubble`, `sp.bubbles`, `sp.bubble_unified`, `sp.bubble_family`

## Description

`sp.bubble()` est le point d'entrée unique de toute la famille des graphiques à bulles. Le paramètre `variant` choisit la stratégie de rendu — tous les autres arguments restent cohérents entre variantes. Une bulle représente une troisième dimension via son **aire** (et non son rayon), pour une lecture perceptive correcte.

| Variante | Cas d'usage | Arguments clés |
|----------|-------------|----------------|
| `"basic"` | Bulles unicolores, 3ᵉ dim = taille | `x_values`, `y_values`, `sizes` |
| `"categorical"` / `"grouped"` | Couleur par groupe + légende | `categories` |
| `"gradient"` / `"colorscale"` | Couleur continue + colorbar | `color_values`, `color_low`, `color_high` |
| `"labeled"` / `"text"` | Étiquettes texte permanentes | `labels` |
| `"outlined"` / `"hollow"` | Cercles vides — chevauchement | `categories`, `stroke_width` |
| `"negative"` / `"signed"` | Tailles signées (P&L, écarts) | `sizes` signés, `color_low`, `color_high` |

---

## Paramètres

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `x_values` | `list[float]` | requis | toutes | Position horizontale |
| `y_values` | `list[float]` | requis | toutes | Position verticale |
| `sizes` | `list[float]` | requis | toutes | Tailles brutes (signées en `negative`) |
| `variant` | `str` | `"basic"` | — | Variante de rendu |
| `categories` | `list[str]` | `None` | categorical, outlined, labeled | Groupe de chaque point |
| `labels` | `list[str]` | `None` | toutes (hover) / labeled (rendu) | Étiquette texte par point |
| `color_values` | `list[float]` | `None` | gradient | Valeur continue pour la colorscale |
| `color_hex` | `int` | `0` | basic | Couleur unique (hex) |
| `color_low` | `int` | `0x6366F1` | gradient, negative | Couleur basse de l'échelle |
| `color_high` | `int` | `0xF43F5E` | gradient, negative | Couleur haute de l'échelle |
| `min_size` / `max_size` | `float` | `4.0` / `40.0` | toutes | Rayon mini/maxi en pixels |
| `show_text` | `bool` | `False` | toutes | Force l'affichage permanent des labels |
| `stroke_width` | `float` | `1.5` | toutes | Épaisseur du contour |
| `palette` | `list[int]` | `None` | categorical, outlined, labeled | Palette personnalisée |
| `width` / `height` | `int` | `900` / `500` | toutes | Dimensions |
| `x_label` / `y_label` | `str` | `""` | toutes | Étiquettes des axes |
| `gridlines` | `bool` | `False` | toutes | Affiche la grille |
| `sort_order` | `str` | `"none"` | toutes | `"asc"`, `"desc"` ou `"none"` (par |size|) |
| `legend_position` | `str` | `"right"` | categorical, outlined | Position de la légende |
| `background` | `str` | `None` | toutes | Couleur de fond |
| `no_x_axis` / `no_y_axis` | `bool` | `False` | toutes | Cache les axes |

---

## Retourne

`Chart` — objet exposant `.html` et `.show()`.

---

<div class="sp-cls sp-open" id="bubble-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bubble-fr')" title="Réduire / développer">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bubble-fr','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-fr','categorical',this)"><span class="sp-cic">◓</span><span class="sp-clb">Catégoriel</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-fr','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Dégradé</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-fr','labeled',this)"><span class="sp-cic">◉</span><span class="sp-clb">Étiqueté</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-fr','outlined',this)"><span class="sp-cic">○</span><span class="sp-clb">Contour</span></button>
<button class="sp-cls-tab" onclick="spCls('bubble-fr','negative',this)"><span class="sp-cic">±</span><span class="sp-clb">Négatif</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bubble-fr-basic">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic</code> / <code>simple</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bbfr-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bbfr-basic','bbfr-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bbfr-basic','bbfr-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bbfr-basic','bbfr-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bbfr-basic','bbfr-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bbfr-basic','bbfr-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bbfr-basic','bbfr-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bbfr-basic','bbfr-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bbfr-basic','bbfr-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bbfr-basic','bbfr-basic-cpp',this)">C++</button>
</div>
<div id="bbfr-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bubble(
    title="PIB vs Espérance de vie",
    variant="basic",
    x_values=[2.5, 5.0, 12.0, 18.5, 25.0],
    y_values=[72, 75, 80, 82, 79],
    sizes=[500, 1200, 3000, 4500, 8000],
    labels=["Brésil","Mexique","Allemagne","Japon","USA"],
    x_label="PIB/hab (k$)",
    y_label="Espérance de vie",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="bbfr-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bubble({
  title: "PIB vs Espérance de vie", variant: "basic",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  labels:  ["Brésil","Mexique","Allemagne","Japon","USA"],
  xLabel: "PIB/hab (k$)", yLabel: "Espérance de vie", gridlines: true,
});
chart.show();</code></pre></div>
<div id="bbfr-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bubble({
  title: "PIB vs Espérance de vie", variant: "basic",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  xLabel: "PIB/hab (k$)", yLabel: "Espérance de vie", gridlines: true,
});
chart.show();</code></pre></div>
<div id="bbfr-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bubble(
  title = "PIB vs Espérance de vie",
  variant = "basic",
  x_values = c(2.5,5.0,12.0,18.5,25.0),
  y_values = c(72,75,80,82,79),
  sizes    = c(500,1200,3000,4500,8000),
  labels   = c("Brésil","Mexique","Allemagne","Japon","USA"),
  x_label = "PIB/hab (k$)", y_label = "Espérance de vie", gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="bbfr-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bubble()
        .title("PIB vs Espérance de vie")
        .variant("basic")
        .x_values(vec![2.5,5.0,12.0,18.5,25.0])
        .y_values(vec![72.0,75.0,80.0,82.0,79.0])
        .sizes(vec![500.0,1200.0,3000.0,4500.0,8000.0])
        .x_label("PIB/hab (k$)").y_label("Espérance de vie").gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="bbfr-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("PIB vs Espérance de vie").variant("basic")
    .xValues(List.of(2.5,5.0,12.0,18.5,25.0))
    .yValues(List.of(72.0,75.0,80.0,82.0,79.0))
    .sizes(List.of(500.0,1200.0,3000.0,4500.0,8000.0))
    .xLabel("PIB/hab (k$)").yLabel("Espérance de vie").gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="bbfr-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "PIB vs Espérance de vie", variant: "basic",
    xValues: [2.5,5.0,12.0,18.5,25.0],
    yValues: [72,75,80,82,79],
    sizes:   [500,1200,3000,4500,8000],
    xLabel: "PIB/hab (k$)", yLabel: "Espérance de vie", gridlines: true
);
chart.Show();</code></pre></div>
<div id="bbfr-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bubble(
  title = "PIB vs Espérance de vie", variant = "basic",
  x_values = List(2.5,5.0,12.0,18.5,25.0),
  y_values = List(72.0,75.0,80.0,82.0,79.0),
  sizes    = List(500.0,1200.0,3000.0,4500.0,8000.0),
  x_label = "PIB/hab (k$)", y_label = "Espérance de vie", gridlines = true
)
chart.show()</code></pre></div>
<div id="bbfr-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
    .title    = "PIB vs Espérance de vie",
    .variant  = "basic",
    .x_values = {2.5,5.0,12.0,18.5,25.0},
    .y_values = {72,75,80,82,79},
    .sizes    = {500,1200,3000,4500,8000},
    .x_label  = "PIB/hab (k$)", .y_label = "Espérance de vie", .gridlines = true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-basic.html"></iframe>
</div>

<div class="sp-variant" id="bubble-fr-categorical">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"categorical"</code> / <code>"grouped"</code></span><span><strong>Alias</strong> <code>category</code> / <code>groups</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bbfr-cat">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bbfr-cat','bbfr-cat-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bbfr-cat','bbfr-cat-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bbfr-cat','bbfr-cat-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bbfr-cat','bbfr-cat-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bbfr-cat','bbfr-cat-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bbfr-cat','bbfr-cat-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bbfr-cat','bbfr-cat-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bbfr-cat','bbfr-cat-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bbfr-cat','bbfr-cat-cpp',this)">C++</button>
</div>
<div id="bbfr-cat-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bubble(
    title="PIB vs Espérance de vie par région",
    variant="categorical",
    x_values=[2.5,5.0,12.0,18.5,25.0,3.2,8.0,14.0],
    y_values=[72,75,80,82,79,68,77,81],
    sizes=[500,1200,3000,4500,8000,800,2100,3600],
    labels=["Brésil","Mexique","Allemagne","Japon","USA","Nigéria","Turquie","France"],
    categories=["Amériques","Amériques","Europe","Asie","Amériques","Afrique","Europe","Europe"],
    x_label="PIB/hab (k$)", y_label="Espérance de vie", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bbfr-cat-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bubble({
  title: "PIB vs Espérance de vie par région", variant: "categorical",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  categories: ["Amériques","Amériques","Europe","Asie","Amériques"],
  xLabel: "PIB/hab (k$)", yLabel: "Espérance de vie", gridlines: true,
});
chart.show();</code></pre></div>
<div id="bbfr-cat-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bubble({
  title: "PIB vs Espérance de vie par région", variant: "categorical",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  categories: ["Amériques","Amériques","Europe","Asie","Amériques"],
});
chart.show();</code></pre></div>
<div id="bbfr-cat-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bubble(
  title = "PIB vs Espérance de vie par région",
  variant = "categorical",
  x_values = c(2.5,5.0,12.0,18.5,25.0),
  y_values = c(72,75,80,82,79),
  sizes    = c(500,1200,3000,4500,8000),
  categories = c("Amériques","Amériques","Europe","Asie","Amériques"),
  x_label = "PIB/hab (k$)", y_label = "Espérance de vie", gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="bbfr-cat-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bubble()
        .title("PIB vs Espérance de vie par région").variant("categorical")
        .x_values(vec![2.5,5.0,12.0,18.5,25.0])
        .y_values(vec![72.0,75.0,80.0,82.0,79.0])
        .sizes(vec![500.0,1200.0,3000.0,4500.0,8000.0])
        .categories(vec!["Amériques","Amériques","Europe","Asie","Amériques"])
        .gridlines(true).build();
    chart.show();
}</code></pre></div>
<div id="bbfr-cat-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("PIB vs Espérance de vie par région").variant("categorical")
    .xValues(List.of(2.5,5.0,12.0,18.5,25.0))
    .yValues(List.of(72.0,75.0,80.0,82.0,79.0))
    .sizes(List.of(500.0,1200.0,3000.0,4500.0,8000.0))
    .categories(List.of("Amériques","Amériques","Europe","Asie","Amériques"))
    .gridlines(true).build();
chart.show();</code></pre></div>
<div id="bbfr-cat-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "PIB vs Espérance de vie par région", variant: "categorical",
    xValues: [2.5,5.0,12.0,18.5,25.0],
    yValues: [72,75,80,82,79],
    sizes:   [500,1200,3000,4500,8000],
    categories: ["Amériques","Amériques","Europe","Asie","Amériques"],
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="bbfr-cat-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bubble(
  title = "PIB vs Espérance de vie par région", variant = "categorical",
  x_values = List(2.5,5.0,12.0,18.5,25.0),
  y_values = List(72.0,75.0,80.0,82.0,79.0),
  sizes    = List(500.0,1200.0,3000.0,4500.0,8000.0),
  categories = List("Amériques","Amériques","Europe","Asie","Amériques"),
  gridlines = true
)
chart.show()</code></pre></div>
<div id="bbfr-cat-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
    .title = "PIB vs Espérance de vie par région", .variant = "categorical",
    .x_values = {2.5,5.0,12.0,18.5,25.0},
    .y_values = {72,75,80,82,79},
    .sizes    = {500,1200,3000,4500,8000},
    .categories = {"Amériques","Amériques","Europe","Asie","Amériques"},
    .gridlines = true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-categorical.html"></iframe>
</div>

<div class="sp-variant" id="bubble-fr-gradient">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code> / <code>"colorscale"</code></span><span><strong>Alias</strong> <code>continuous</code> / <code>scaled</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bbfr-grad">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bbfr-grad','bbfr-grad-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bbfr-grad','bbfr-grad-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bbfr-grad','bbfr-grad-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bbfr-grad','bbfr-grad-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bbfr-grad','bbfr-grad-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bbfr-grad','bbfr-grad-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bbfr-grad','bbfr-grad-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bbfr-grad','bbfr-grad-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bbfr-grad','bbfr-grad-cpp',this)">C++</button>
</div>
<div id="bbfr-grad-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bubble(
    title="Stations : dégradé de température",
    variant="gradient",
    x_values=[2.5,5.0,12.0,18.5,25.0,3.2,8.0,14.0],
    y_values=[72,75,80,82,79,68,77,81],
    sizes=[500,1200,3000,4500,8000,800,2100,3600],
    color_values=[12,18,41,55,63,8,24,47],
    color_low=0x0EA5E9, color_high=0xF43F5E,
    x_label="Longitude", y_label="Latitude", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bbfr-grad-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bubble({
  title: "Stations : dégradé", variant: "gradient",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  colorValues: [12,18,41,55,63],
  colorLow: 0x0EA5E9, colorHigh: 0xF43F5E,
});
chart.show();</code></pre></div>
<div id="bbfr-grad-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bubble({
  title: "Stations : dégradé", variant: "gradient",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  colorValues: [12,18,41,55,63],
  colorLow: 0x0EA5E9, colorHigh: 0xF43F5E,
});
chart.show();</code></pre></div>
<div id="bbfr-grad-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bubble(
  title = "Stations : dégradé", variant = "gradient",
  x_values = c(2.5,5.0,12.0,18.5,25.0),
  y_values = c(72,75,80,82,79),
  sizes    = c(500,1200,3000,4500,8000),
  color_values = c(12,18,41,55,63),
  color_low = 0x0EA5E9, color_high = 0xF43F5E
)
chart$show()</code></pre></div>
<div id="bbfr-grad-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bubble()
        .title("Stations : dégradé").variant("gradient")
        .x_values(vec![2.5,5.0,12.0,18.5,25.0])
        .y_values(vec![72.0,75.0,80.0,82.0,79.0])
        .sizes(vec![500.0,1200.0,3000.0,4500.0,8000.0])
        .color_values(vec![12.0,18.0,41.0,55.0,63.0])
        .color_low(0x0EA5E9).color_high(0xF43F5E)
        .build();
    chart.show();
}</code></pre></div>
<div id="bbfr-grad-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("Stations : dégradé").variant("gradient")
    .xValues(List.of(2.5,5.0,12.0,18.5,25.0))
    .yValues(List.of(72.0,75.0,80.0,82.0,79.0))
    .sizes(List.of(500.0,1200.0,3000.0,4500.0,8000.0))
    .colorValues(List.of(12.0,18.0,41.0,55.0,63.0))
    .colorLow(0x0EA5E9).colorHigh(0xF43F5E)
    .build();
chart.show();</code></pre></div>
<div id="bbfr-grad-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "Stations : dégradé", variant: "gradient",
    xValues: [2.5,5.0,12.0,18.5,25.0],
    yValues: [72,75,80,82,79],
    sizes:   [500,1200,3000,4500,8000],
    colorValues: [12,18,41,55,63],
    colorLow: 0x0EA5E9, colorHigh: 0xF43F5E
);
chart.Show();</code></pre></div>
<div id="bbfr-grad-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bubble(
  title = "Stations : dégradé", variant = "gradient",
  x_values = List(2.5,5.0,12.0,18.5,25.0),
  y_values = List(72.0,75.0,80.0,82.0,79.0),
  sizes    = List(500.0,1200.0,3000.0,4500.0,8000.0),
  color_values = List(12.0,18.0,41.0,55.0,63.0),
  color_low = 0x0EA5E9, color_high = 0xF43F5E
)
chart.show()</code></pre></div>
<div id="bbfr-grad-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
    .title = "Stations : dégradé", .variant = "gradient",
    .x_values = {2.5,5.0,12.0,18.5,25.0},
    .y_values = {72,75,80,82,79},
    .sizes    = {500,1200,3000,4500,8000},
    .color_values = {12,18,41,55,63},
    .color_low = 0x0EA5E9, .color_high = 0xF43F5E,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-gradient.html"></iframe>
</div>

<div class="sp-variant" id="bubble-fr-labeled">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"labeled"</code> / <code>"text"</code></span><span><strong>Alias</strong> <code>labels</code> / <code>annotated</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bbfr-lbl">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bbfr-lbl','bbfr-lbl-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bbfr-lbl','bbfr-lbl-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bbfr-lbl','bbfr-lbl-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bbfr-lbl','bbfr-lbl-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bbfr-lbl','bbfr-lbl-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bbfr-lbl','bbfr-lbl-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bbfr-lbl','bbfr-lbl-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bbfr-lbl','bbfr-lbl-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bbfr-lbl','bbfr-lbl-cpp',this)">C++</button>
</div>
<div id="bbfr-lbl-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bubble(
    title="Top 6 économies",
    variant="labeled",
    x_values=[2.5,5.0,12.0,18.5,25.0,3.2],
    y_values=[72,75,80,82,79,68],
    sizes=[500,1200,3000,4500,8000,800],
    labels=["Brésil","Mexique","Allemagne","Japon","USA","Nigéria"],
    categories=["Amériques","Amériques","Europe","Asie","Amériques","Afrique"],
    x_label="PIB/hab (k$)", y_label="Espérance de vie", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bbfr-lbl-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bubble({
  title: "Top 6 économies", variant: "labeled",
  xValues: [2.5,5.0,12.0,18.5,25.0,3.2],
  yValues: [72,75,80,82,79,68],
  sizes:   [500,1200,3000,4500,8000,800],
  labels:  ["Brésil","Mexique","Allemagne","Japon","USA","Nigéria"],
});
chart.show();</code></pre></div>
<div id="bbfr-lbl-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bubble({
  title: "Top 6 économies", variant: "labeled",
  xValues: [2.5,5.0,12.0,18.5,25.0,3.2],
  yValues: [72,75,80,82,79,68],
  sizes:   [500,1200,3000,4500,8000,800],
  labels:  ["Brésil","Mexique","Allemagne","Japon","USA","Nigéria"],
});
chart.show();</code></pre></div>
<div id="bbfr-lbl-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bubble(
  title = "Top 6 économies", variant = "labeled",
  x_values = c(2.5,5.0,12.0,18.5,25.0,3.2),
  y_values = c(72,75,80,82,79,68),
  sizes    = c(500,1200,3000,4500,8000,800),
  labels   = c("Brésil","Mexique","Allemagne","Japon","USA","Nigéria")
)
chart$show()</code></pre></div>
<div id="bbfr-lbl-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bubble()
        .title("Top 6 économies").variant("labeled")
        .x_values(vec![2.5,5.0,12.0,18.5,25.0,3.2])
        .y_values(vec![72.0,75.0,80.0,82.0,79.0,68.0])
        .sizes(vec![500.0,1200.0,3000.0,4500.0,8000.0,800.0])
        .labels(vec!["Brésil","Mexique","Allemagne","Japon","USA","Nigéria"])
        .build();
    chart.show();
}</code></pre></div>
<div id="bbfr-lbl-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("Top 6 économies").variant("labeled")
    .xValues(List.of(2.5,5.0,12.0,18.5,25.0,3.2))
    .yValues(List.of(72.0,75.0,80.0,82.0,79.0,68.0))
    .sizes(List.of(500.0,1200.0,3000.0,4500.0,8000.0,800.0))
    .labels(List.of("Brésil","Mexique","Allemagne","Japon","USA","Nigéria"))
    .build();
chart.show();</code></pre></div>
<div id="bbfr-lbl-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "Top 6 économies", variant: "labeled",
    xValues: [2.5,5.0,12.0,18.5,25.0,3.2],
    yValues: [72,75,80,82,79,68],
    sizes:   [500,1200,3000,4500,8000,800],
    labels:  ["Brésil","Mexique","Allemagne","Japon","USA","Nigéria"]
);
chart.Show();</code></pre></div>
<div id="bbfr-lbl-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bubble(
  title = "Top 6 économies", variant = "labeled",
  x_values = List(2.5,5.0,12.0,18.5,25.0,3.2),
  y_values = List(72.0,75.0,80.0,82.0,79.0,68.0),
  sizes    = List(500.0,1200.0,3000.0,4500.0,8000.0,800.0),
  labels   = List("Brésil","Mexique","Allemagne","Japon","USA","Nigéria")
)
chart.show()</code></pre></div>
<div id="bbfr-lbl-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
    .title = "Top 6 économies", .variant = "labeled",
    .x_values = {2.5,5.0,12.0,18.5,25.0,3.2},
    .y_values = {72,75,80,82,79,68},
    .sizes    = {500,1200,3000,4500,8000,800},
    .labels   = {"Brésil","Mexique","Allemagne","Japon","USA","Nigéria"},
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-labeled.html"></iframe>
</div>

<div class="sp-variant" id="bubble-fr-outlined">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"outlined"</code> / <code>"hollow"</code></span><span><strong>Alias</strong> <code>ring</code> / <code>open</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bbfr-out">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bbfr-out','bbfr-out-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bbfr-out','bbfr-out-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bbfr-out','bbfr-out-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bbfr-out','bbfr-out-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bbfr-out','bbfr-out-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bbfr-out','bbfr-out-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bbfr-out','bbfr-out-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bbfr-out','bbfr-out-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bbfr-out','bbfr-out-cpp',this)">C++</button>
</div>
<div id="bbfr-out-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bubble(
    title="Chevauchement dense → contours",
    variant="outlined",
    x_values=[2.5,5.0,12.0,18.5,25.0,3.2,8.0,14.0],
    y_values=[72,75,80,82,79,68,77,81],
    sizes=[500,1200,3000,4500,8000,800,2100,3600],
    categories=["A","A","B","B","A","C","B","C"],
    stroke_width=3.0,
    x_label="X", y_label="Y", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bbfr-out-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bubble({
  title: "Chevauchement dense", variant: "outlined",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  categories: ["A","A","B","B","A"], strokeWidth: 3.0,
});
chart.show();</code></pre></div>
<div id="bbfr-out-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bubble({
  title: "Chevauchement dense", variant: "outlined",
  xValues: [2.5,5.0,12.0,18.5,25.0],
  yValues: [72,75,80,82,79],
  sizes:   [500,1200,3000,4500,8000],
  categories: ["A","A","B","B","A"], strokeWidth: 3.0,
});
chart.show();</code></pre></div>
<div id="bbfr-out-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bubble(
  title = "Chevauchement dense", variant = "outlined",
  x_values = c(2.5,5.0,12.0,18.5,25.0),
  y_values = c(72,75,80,82,79),
  sizes    = c(500,1200,3000,4500,8000),
  categories = c("A","A","B","B","A"), stroke_width = 3.0
)
chart$show()</code></pre></div>
<div id="bbfr-out-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bubble()
        .title("Chevauchement dense").variant("outlined")
        .x_values(vec![2.5,5.0,12.0,18.5,25.0])
        .y_values(vec![72.0,75.0,80.0,82.0,79.0])
        .sizes(vec![500.0,1200.0,3000.0,4500.0,8000.0])
        .categories(vec!["A","A","B","B","A"])
        .stroke_width(3.0).build();
    chart.show();
}</code></pre></div>
<div id="bbfr-out-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("Chevauchement dense").variant("outlined")
    .xValues(List.of(2.5,5.0,12.0,18.5,25.0))
    .yValues(List.of(72.0,75.0,80.0,82.0,79.0))
    .sizes(List.of(500.0,1200.0,3000.0,4500.0,8000.0))
    .categories(List.of("A","A","B","B","A"))
    .strokeWidth(3.0).build();
chart.show();</code></pre></div>
<div id="bbfr-out-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "Chevauchement dense", variant: "outlined",
    xValues: [2.5,5.0,12.0,18.5,25.0],
    yValues: [72,75,80,82,79],
    sizes:   [500,1200,3000,4500,8000],
    categories: ["A","A","B","B","A"], strokeWidth: 3.0
);
chart.Show();</code></pre></div>
<div id="bbfr-out-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bubble(
  title = "Chevauchement dense", variant = "outlined",
  x_values = List(2.5,5.0,12.0,18.5,25.0),
  y_values = List(72.0,75.0,80.0,82.0,79.0),
  sizes    = List(500.0,1200.0,3000.0,4500.0,8000.0),
  categories = List("A","A","B","B","A"),
  stroke_width = 3.0
)
chart.show()</code></pre></div>
<div id="bbfr-out-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
    .title = "Chevauchement dense", .variant = "outlined",
    .x_values = {2.5,5.0,12.0,18.5,25.0},
    .y_values = {72,75,80,82,79},
    .sizes    = {500,1200,3000,4500,8000},
    .categories = {"A","A","B","B","A"}, .stroke_width = 3.0,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-outlined.html"></iframe>
</div>

<div class="sp-variant" id="bubble-fr-negative">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"negative"</code> / <code>"signed"</code></span><span><strong>Alias</strong> <code>diverging</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bbfr-neg">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bbfr-neg','bbfr-neg-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bbfr-neg','bbfr-neg-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bbfr-neg','bbfr-neg-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bbfr-neg','bbfr-neg-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bbfr-neg','bbfr-neg-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('bbfr-neg','bbfr-neg-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bbfr-neg','bbfr-neg-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bbfr-neg','bbfr-neg-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bbfr-neg','bbfr-neg-cpp',this)">C++</button>
</div>
<div id="bbfr-neg-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bubble(
    title="Exposition P&amp;L signée",
    variant="negative",
    x_values=[1,2,3,4,5,6,7,8,9,10],
    y_values=[12,-8,15,22,-14,9,-5,18,-11,25],
    sizes=[120,-85,150,220,-140,90,-55,180,-115,250],
    labels=["T1","T2","T3","T4","T5","T6","T7","T8","T9","T10"],
    color_low=0xEF4444, color_high=0x10B981,
    x_label="Période", y_label="Rendement (%)", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bbfr-neg-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bubble({
  title: "P&L signé", variant: "negative",
  xValues: [1,2,3,4,5,6,7,8,9,10],
  yValues: [12,-8,15,22,-14,9,-5,18,-11,25],
  sizes:   [120,-85,150,220,-140,90,-55,180,-115,250],
  colorLow: 0xEF4444, colorHigh: 0x10B981,
});
chart.show();</code></pre></div>
<div id="bbfr-neg-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bubble({
  title: "P&L signé", variant: "negative",
  xValues: [1,2,3,4,5,6,7,8,9,10],
  yValues: [12,-8,15,22,-14,9,-5,18,-11,25],
  sizes:   [120,-85,150,220,-140,90,-55,180,-115,250],
  colorLow: 0xEF4444, colorHigh: 0x10B981,
});
chart.show();</code></pre></div>
<div id="bbfr-neg-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bubble(
  title = "P&L signé", variant = "negative",
  x_values = 1:10,
  y_values = c(12,-8,15,22,-14,9,-5,18,-11,25),
  sizes    = c(120,-85,150,220,-140,90,-55,180,-115,250),
  color_low = 0xEF4444, color_high = 0x10B981
)
chart$show()</code></pre></div>
<div id="bbfr-neg-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bubble()
        .title("P&L signé").variant("negative")
        .x_values(vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0])
        .y_values(vec![12.0,-8.0,15.0,22.0,-14.0,9.0,-5.0,18.0,-11.0,25.0])
        .sizes(vec![120.0,-85.0,150.0,220.0,-140.0,90.0,-55.0,180.0,-115.0,250.0])
        .color_low(0xEF4444).color_high(0x10B981)
        .build();
    chart.show();
}</code></pre></div>
<div id="bbfr-neg-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("P&L signé").variant("negative")
    .xValues(List.of(1,2,3,4,5,6,7,8,9,10))
    .yValues(List.of(12.0,-8.0,15.0,22.0,-14.0,9.0,-5.0,18.0,-11.0,25.0))
    .sizes(List.of(120.0,-85.0,150.0,220.0,-140.0,90.0,-55.0,180.0,-115.0,250.0))
    .colorLow(0xEF4444).colorHigh(0x10B981)
    .build();
chart.show();</code></pre></div>
<div id="bbfr-neg-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "P&L signé", variant: "negative",
    xValues: [1,2,3,4,5,6,7,8,9,10],
    yValues: [12,-8,15,22,-14,9,-5,18,-11,25],
    sizes:   [120,-85,150,220,-140,90,-55,180,-115,250],
    colorLow: 0xEF4444, colorHigh: 0x10B981
);
chart.Show();</code></pre></div>
<div id="bbfr-neg-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bubble(
  title = "P&L signé", variant = "negative",
  x_values = List(1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0),
  y_values = List(12.0,-8.0,15.0,22.0,-14.0,9.0,-5.0,18.0,-11.0,25.0),
  sizes    = List(120.0,-85.0,150.0,220.0,-140.0,90.0,-55.0,180.0,-115.0,250.0),
  color_low = 0xEF4444, color_high = 0x10B981
)
chart.show()</code></pre></div>
<div id="bbfr-neg-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
    .title = "P&L signé", .variant = "negative",
    .x_values = {1,2,3,4,5,6,7,8,9,10},
    .y_values = {12,-8,15,22,-14,9,-5,18,-11,25},
    .sizes    = {120,-85,150,220,-140,90,-55,180,-115,250},
    .color_low = 0xEF4444, .color_high = 0x10B981,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bubble-negative.html"></iframe>
</div>

</div>
</div>

---

## Voir aussi

- [Nuage de points (scatter)](scatter.md) — `sp.scatter()`
- [Bulles 3D](../3d/bubble3d.md) — `sp.bubble3d()`
- [Barres](bar.md) — `sp.bar()`

</div><!-- /lang-fr -->
