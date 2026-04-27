# Bar Charts

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

/* The actual bookmark — sticks out 30px LEFT of the rail */
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

/* Description + signature isolated as a card ABOVE the meta block.
   Built from the first <p> + first <pre> in each variant — no markdown rewrite needed. */
.sp-variant > p:first-of-type{margin:0;padding:14px 18px 8px;background:linear-gradient(180deg,rgba(99,102,241,.08),rgba(99,102,241,.03));border:1px solid rgba(99,102,241,.18);border-bottom:none;border-radius:10px 10px 0 0;color:#e2e8f0;font-size:14px;line-height:1.55;font-weight:500}
.sp-variant > p:first-of-type + pre{margin:0 0 18px;padding:14px 18px 16px;background:linear-gradient(180deg,#0d1326,#080d1a);border:1px solid rgba(99,102,241,.18);border-top:none;border-radius:0 0 10px 10px;box-shadow:0 6px 18px -8px rgba(0,0,0,.6);overflow-x:auto}
.sp-variant > p:first-of-type + pre code{background:none;padding:0;font-size:12.5px;line-height:1.55;color:#cbd5e1}

.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
.sp-preview-frame{width:100%;height:380px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}

/* Inside the API panel: ensure the bookmarks can escape the section padding */
#sp-params-panel .sp-psec-returns,
#sp-params-panel .sp-psec-returns .sp-psec-content{overflow:visible}
#sp-params-panel .sp-cls{margin-left:46px;margin-right:0}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.bar(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

Aliases: `sp.bar`, `sp.bars`, `sp.bar_unified`, `sp.bars_unified`, `sp.bar_family`, `sp.bar_chart`

## Description


`sp.bar()` is the unified entry point for the entire bar-chart family. The `variant` keyword selects the rendering strategy — all other arguments remain consistent across variants.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Single series, vertical | `labels`, `values` |
| `"horizontal"` / `"h"` | Single series, horizontal | `labels`, `values` |
| `"grouped"` / `"group"` | Multi-series side-by-side | `series`, `series_names` |
| `"stacked"` / `"stack"` | Multi-series stacked | `series`, `series_names` |
| `"relative"` / `"rel"` | Stacked + negatives below 0 | `series`, `series_names` |
| `"grouped_stacked"` | Groups of stacked bars | `series`, `series_names`, `offset_groups` |
| `"marimekko"` / `"mekko"` / `"mosaic"` | Variable-width mosaic | `series`, `series_names`, `widths` |
| `"pictogram"` / `"icon"` | Repeated-icon bar | `labels`, `values`, `units_per_icon` |
| `"multicategory"` / `"multi"` | Two-level x axis | `labels`, `values`, `super_categories` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | required | all | Chart title |
| `labels` | `list[str]` | `None` | all | Category labels |
| `values` | `list[float]` | `None` | basic, horizontal, pictogram, multicategory | Single-series values |
| `variant` | `str` | `"basic"` | — | Rendering variant (see table above) |
| `series` | `list[list[float]]` | `None` | grouped, stacked, relative, grouped_stacked, marimekko, multicategory | Multi-series data |
| `series_names` | `list[str]` | `None` | multi-series | Legend label per series |
| `offset_groups` | `list[str]` | `None` | grouped_stacked | Stack-group name per series |
| `widths` | `list[float]` | `None` | marimekko | Relative column width per category |
| `super_categories` | `list[str]` | `None` | multicategory | Bracket label per bar |
| `icon_size` | `int` | `24` | pictogram | Icon square size in px |
| `max_icons_per_column` | `int` | `10` | pictogram | Max icons per column before wrap |
| `units_per_icon` | `float` | `1.0` | pictogram | Data units each icon represents |
| `unit_description` | `str` | `""` | pictogram | Unit label shown in header |
| `color_hex` | `int` | `0` | basic, horizontal, pictogram | Single override color as hex int |
| `palette` | `list[int]` | `None` | all | Custom color list |
| `show_text` | `bool` | `False` | basic, horizontal, marimekko | Render value labels on bars |
| `corner_radius` | `int` | `0` | basic, horizontal | Rounded-corner radius in px |
| `bar_gap` | `float` | `0.2` | basic, horizontal, relative, grouped_stacked | Fraction of category width as spacing |
| `bargroup_gap` | `float` | `0.1` | grouped, grouped_stacked | Fraction of group width as gap between bars |
| `width` | `int` | `900` | all | Canvas width in pixels |
| `height` | `int` | `480` | all | Canvas height in pixels |
| `x_label` | `str` | `""` | all | X-axis label |
| `y_label` | `str` | `""` | all | Y-axis label |
| `gridlines` | `bool` | `False` | all | Show horizontal gridlines |
| `sort_order` | `str` | `"none"` | basic, horizontal | `"asc"`, `"desc"`, `"alpha"`, `"alpha_desc"`, or `"none"` |
| `legend_position` | `str` | `"right"` | multi-series | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | all | Background CSS color; `None` = transparent |
| `no_x_axis` | `bool` | `False` | all | Hide X axis |
| `no_y_axis` | `bool` | `False` | all | Hide Y axis |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="bar-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bar-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bar-en','basic',this)"><span class="sp-cic">▮</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','horizontal',this)"><span class="sp-cic">▬</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','grouped',this)"><span class="sp-cic">▐▐</span><span class="sp-clb">Grouped</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','stacked',this)"><span class="sp-cic">▦</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','relative',this)"><span class="sp-cic">±</span><span class="sp-clb">Relative</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','gstack',this)"><span class="sp-cic">▤</span><span class="sp-clb">Grouped-Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','marimekko',this)"><span class="sp-cic">▤</span><span class="sp-clb">Marimekko</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','pictogram',this)"><span class="sp-cic">☰</span><span class="sp-clb">Pictogram</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-en','multicategory',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Multicategory</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bar-en-basic">

Vertical bars — one value per category.

```python
sp.bar(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    variant: str = "basic",
    color_hex: int = 0,
    show_text: bool = False,
    corner_radius: int = 0,
    bar_gap: float = 0.2,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>sp.bar</code> <code>sp.bars</code> <code>sp.bar_unified</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-basic','b-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('b-basic','b-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('b-basic','b-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('b-basic','b-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('b-basic','b-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('b-basic','b-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('b-basic','b-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('b-basic','b-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('b-basic','b-basic-cpp',this)">C++</button>
</div>
<div id="b-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Monthly Revenue",
    variant="basic",
    labels=["Jan","Feb","Mar","Apr","May","Jun"],
    values=[1200, 1850, 2100, 1750, 2400, 2800],
    y_label="Revenue (€)",
    gridlines=True,
    show_text=True,
)
chart.show()</code></pre></div>
<div id="b-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Monthly Revenue", variant: "basic",
  labels: ["Jan","Feb","Mar","Apr","May","Jun"],
  values: [1200, 1850, 2100, 1750, 2400, 2800],
  yLabel: "Revenue (€)", gridlines: true, showText: true,
});
chart.show();</code></pre></div>
<div id="b-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Monthly Revenue", variant: "basic",
  labels: ["Jan","Feb","Mar","Apr","May","Jun"],
  values: [1200, 1850, 2100, 1750, 2400, 2800],
  yLabel: "Revenue (€)", gridlines: true, showText: true,
});
chart.show();</code></pre></div>
<div id="b-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "Monthly Revenue",
  variant = "basic",
  labels = c("Jan","Feb","Mar","Apr","May","Jun"),
  values = c(1200, 1850, 2100, 1750, 2400, 2800),
  y_label = "Revenue (€)",
  gridlines = TRUE,
  show_text = TRUE
)
chart$show()</code></pre></div>
<div id="b-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bar()
        .title("Monthly Revenue")
        .variant("basic")
        .labels(vec!["Jan","Feb","Mar","Apr","May","Jun"])
        .values(vec![1200.0, 1850.0, 2100.0, 1750.0, 2400.0, 2800.0])
        .y_label("Revenue (€)")
        .gridlines(true)
        .show_text(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="b-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bar()
    .title("Monthly Revenue")
    .variant("basic")
    .labels(List.of("Jan","Feb","Mar","Apr","May","Jun"))
    .values(List.of(1200.0,1850.0,2100.0,1750.0,2400.0,2800.0))
    .yLabel("Revenue (€)")
    .gridlines(true)
    .showText(true)
    .build();
chart.show();</code></pre></div>
<div id="b-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bar(
    title: "Monthly Revenue",
    variant: "basic",
    labels: ["Jan","Feb","Mar","Apr","May","Jun"],
    values: [1200,1850,2100,1750,2400,2800],
    yLabel: "Revenue (€)",
    gridlines: true,
    showText: true
);
chart.Show();</code></pre></div>
<div id="b-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bar(
  title = "Monthly Revenue",
  variant = "basic",
  labels = List("Jan","Feb","Mar","Apr","May","Jun"),
  values = List(1200.0,1850.0,2100.0,1750.0,2400.0,2800.0),
  y_label = "Revenue (€)",
  gridlines = true,
  show_text = true
)
chart.show()</code></pre></div>
<div id="b-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bar({
    .title     = "Monthly Revenue",
    .variant   = "basic",
    .labels    = {"Jan","Feb","Mar","Apr","May","Jun"},
    .values    = {1200.0,1850.0,2100.0,1750.0,2400.0,2800.0},
    .y_label   = "Revenue (€)",
    .gridlines = true,
    .show_text = true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-horizontal">

Horizontal bars — better for long category names. Alias: `"h"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    variant: str = "horizontal",
    color_hex: int = 0,
    show_text: bool = False,
    bar_gap: float = 0.2,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    palette: list[int] | None = None,
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code> / <code>"h"</code></span><span><strong>Aliases</strong> <code>sp.bar</code> + <code>variant="h"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-h">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-h','b-h-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('b-h','b-h-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('b-h','b-h-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('b-h','b-h-r',this)">R</button>
<button class="sp-tb" onclick="spTab('b-h','b-h-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('b-h','b-h-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('b-h','b-h-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('b-h','b-h-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('b-h','b-h-cpp',this)">C++</button>
</div>
<div id="b-h-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Population by Country",
    variant="horizontal",
    labels=["China","India","USA","Indonesia","Pakistan"],
    values=[1411, 1393, 332, 273, 220],
    x_label="Population (M)",
    gridlines=True,
    sort_order="desc",
)
chart.show()</code></pre></div>
<div id="b-h-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Population by Country", variant: "horizontal",
  labels: ["China","India","USA","Indonesia","Pakistan"],
  values: [1411, 1393, 332, 273, 220],
  xLabel: "Population (M)", gridlines: true, sortOrder: "desc",
});
chart.show();</code></pre></div>
<div id="b-h-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Population by Country", variant: "horizontal",
  labels: ["China","India","USA","Indonesia","Pakistan"],
  values: [1411, 1393, 332, 273, 220],
  xLabel: "Population (M)", gridlines: true, sortOrder: "desc",
});
chart.show();</code></pre></div>
<div id="b-h-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "Population by Country",
  variant = "horizontal",
  labels = c("China","India","USA","Indonesia","Pakistan"),
  values = c(1411, 1393, 332, 273, 220),
  x_label = "Population (M)",
  gridlines = TRUE,
  sort_order = "desc"
)
chart$show()</code></pre></div>
<div id="b-h-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bar()
        .title("Population by Country")
        .variant("horizontal")
        .labels(vec!["China","India","USA","Indonesia","Pakistan"])
        .values(vec![1411.0, 1393.0, 332.0, 273.0, 220.0])
        .x_label("Population (M)")
        .gridlines(true)
        .sort_order("desc")
        .build();
    chart.show();
}</code></pre></div>
<div id="b-h-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bar()
    .title("Population by Country")
    .variant("horizontal")
    .labels(List.of("China","India","USA","Indonesia","Pakistan"))
    .values(List.of(1411.0,1393.0,332.0,273.0,220.0))
    .xLabel("Population (M)")
    .gridlines(true)
    .sortOrder("desc")
    .build();
chart.show();</code></pre></div>
<div id="b-h-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bar(
    title: "Population by Country",
    variant: "horizontal",
    labels: ["China","India","USA","Indonesia","Pakistan"],
    values: [1411,1393,332,273,220],
    xLabel: "Population (M)",
    gridlines: true,
    sortOrder: "desc"
);
chart.Show();</code></pre></div>
<div id="b-h-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bar(
  title = "Population by Country",
  variant = "horizontal",
  labels = List("China","India","USA","Indonesia","Pakistan"),
  values = List(1411.0,1393.0,332.0,273.0,220.0),
  x_label = "Population (M)",
  gridlines = true,
  sort_order = "desc"
)
chart.show()</code></pre></div>
<div id="b-h-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bar({
    .title      = "Population by Country",
    .variant    = "horizontal",
    .labels     = {"China","India","USA","Indonesia","Pakistan"},
    .values     = {1411.0,1393.0,332.0,273.0,220.0},
    .x_label    = "Population (M)",
    .gridlines  = true,
    .sort_order = "desc",
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hbar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-grouped">

Multiple series side-by-side per category. Alias: `"group"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    *,
    variant: str = "grouped",
    series: list[list[float]],
    series_names: list[str] | None = None,
    bar_gap: float = 0.2,
    bargroup_gap: float = 0.1,
    show_text: bool = False,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    legend_position: str = "right",
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"grouped"</code> / <code>"group"</code></span><span><strong>Required</strong> <code>series</code>, <code>series_names</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-g">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-g','b-g-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('b-g','b-g-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('b-g','b-g-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('b-g','b-g-r',this)">R</button>
<button class="sp-tb" onclick="spTab('b-g','b-g-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('b-g','b-g-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('b-g','b-g-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('b-g','b-g-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('b-g','b-g-cpp',this)">C++</button>
</div>
<div id="b-g-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Revenue by Region",
    variant="grouped",
    labels=["Q1","Q2","Q3","Q4"],
    series=[[120,145,160,180],[90,110,135,150],[60,75,80,95]],
    series_names=["EU","NA","APAC"],
    y_label="Revenue (M€)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="b-g-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Revenue by Region", variant: "grouped",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[120,145,160,180],[90,110,135,150],[60,75,80,95]],
  seriesNames: ["EU","NA","APAC"],
  yLabel: "Revenue (M€)", gridlines: true,
});
chart.show();</code></pre></div>
<div id="b-g-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Revenue by Region", variant: "grouped",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[120,145,160,180],[90,110,135,150],[60,75,80,95]],
  seriesNames: ["EU","NA","APAC"],
  yLabel: "Revenue (M€)", gridlines: true,
});
chart.show();</code></pre></div>
<div id="b-g-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "Revenue by Region",
  variant = "grouped",
  labels = c("Q1","Q2","Q3","Q4"),
  series = list(c(120,145,160,180), c(90,110,135,150), c(60,75,80,95)),
  series_names = c("EU","NA","APAC"),
  y_label = "Revenue (M€)",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="b-g-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bar()
        .title("Revenue by Region")
        .variant("grouped")
        .labels(vec!["Q1","Q2","Q3","Q4"])
        .series(vec![
            vec![120.0,145.0,160.0,180.0],
            vec![90.0,110.0,135.0,150.0],
            vec![60.0,75.0,80.0,95.0],
        ])
        .series_names(vec!["EU","NA","APAC"])
        .y_label("Revenue (M€)")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="b-g-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bar()
    .title("Revenue by Region")
    .variant("grouped")
    .labels(List.of("Q1","Q2","Q3","Q4"))
    .series(List.of(
        List.of(120.0,145.0,160.0,180.0),
        List.of(90.0,110.0,135.0,150.0),
        List.of(60.0,75.0,80.0,95.0)
    ))
    .seriesNames(List.of("EU","NA","APAC"))
    .yLabel("Revenue (M€)")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="b-g-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bar(
    title: "Revenue by Region",
    variant: "grouped",
    labels: ["Q1","Q2","Q3","Q4"],
    series: [[120,145,160,180],[90,110,135,150],[60,75,80,95]],
    seriesNames: ["EU","NA","APAC"],
    yLabel: "Revenue (M€)",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="b-g-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bar(
  title = "Revenue by Region",
  variant = "grouped",
  labels = List("Q1","Q2","Q3","Q4"),
  series = List(
    List(120.0,145.0,160.0,180.0),
    List(90.0,110.0,135.0,150.0),
    List(60.0,75.0,80.0,95.0)
  ),
  series_names = List("EU","NA","APAC"),
  y_label = "Revenue (M€)",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="b-g-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bar({
    .title        = "Revenue by Region",
    .variant      = "grouped",
    .labels       = {"Q1","Q2","Q3","Q4"},
    .series       = {{120,145,160,180},{90,110,135,150},{60,75,80,95}},
    .series_names = {"EU","NA","APAC"},
    .y_label      = "Revenue (M€)",
    .gridlines    = true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/grouped-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-stacked">

Series stacked vertically — shows part-to-whole within each category. Alias: `"stack"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    *,
    variant: str = "stacked",
    series: list[list[float]],
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    legend_position: str = "right",
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code> / <code>"stack"</code></span><span><strong>Required</strong> <code>series</code>, <code>series_names</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-s">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-s','b-s-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('b-s','b-s-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('b-s','b-s-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('b-s','b-s-r',this)">R</button>
<button class="sp-tb" onclick="spTab('b-s','b-s-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('b-s','b-s-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('b-s','b-s-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('b-s','b-s-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('b-s','b-s-cpp',this)">C++</button>
</div>
<div id="b-s-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Quarterly Revenue Mix",
    variant="stacked",
    labels=["Q1","Q2","Q3","Q4"],
    series=[[120,145,160,180],[90,110,135,150],[60,75,80,95]],
    series_names=["EU","NA","APAC"],
    y_label="Revenue (M€)",
)
chart.show()</code></pre></div>
<div id="b-s-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Quarterly Revenue Mix", variant: "stacked",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[120,145,160,180],[90,110,135,150],[60,75,80,95]],
  seriesNames: ["EU","NA","APAC"], yLabel: "Revenue (M€)",
});
chart.show();</code></pre></div>
<div id="b-s-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Quarterly Revenue Mix", variant: "stacked",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[120,145,160,180],[90,110,135,150],[60,75,80,95]],
  seriesNames: ["EU","NA","APAC"], yLabel: "Revenue (M€)",
});
chart.show();</code></pre></div>
<div id="b-s-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "Quarterly Revenue Mix",
  variant = "stacked",
  labels = c("Q1","Q2","Q3","Q4"),
  series = list(c(120,145,160,180), c(90,110,135,150), c(60,75,80,95)),
  series_names = c("EU","NA","APAC"),
  y_label = "Revenue (M€)"
)
chart$show()</code></pre></div>
<div id="b-s-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bar()
        .title("Quarterly Revenue Mix")
        .variant("stacked")
        .labels(vec!["Q1","Q2","Q3","Q4"])
        .series(vec![
            vec![120.0,145.0,160.0,180.0],
            vec![90.0,110.0,135.0,150.0],
            vec![60.0,75.0,80.0,95.0],
        ])
        .series_names(vec!["EU","NA","APAC"])
        .y_label("Revenue (M€)")
        .build();
    chart.show();
}</code></pre></div>
<div id="b-s-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bar()
    .title("Quarterly Revenue Mix")
    .variant("stacked")
    .labels(List.of("Q1","Q2","Q3","Q4"))
    .series(List.of(
        List.of(120.0,145.0,160.0,180.0),
        List.of(90.0,110.0,135.0,150.0),
        List.of(60.0,75.0,80.0,95.0)
    ))
    .seriesNames(List.of("EU","NA","APAC"))
    .yLabel("Revenue (M€)")
    .build();
chart.show();</code></pre></div>
<div id="b-s-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bar(
    title: "Quarterly Revenue Mix",
    variant: "stacked",
    labels: ["Q1","Q2","Q3","Q4"],
    series: [[120,145,160,180],[90,110,135,150],[60,75,80,95]],
    seriesNames: ["EU","NA","APAC"],
    yLabel: "Revenue (M€)"
);
chart.Show();</code></pre></div>
<div id="b-s-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bar(
  title = "Quarterly Revenue Mix",
  variant = "stacked",
  labels = List("Q1","Q2","Q3","Q4"),
  series = List(
    List(120.0,145.0,160.0,180.0),
    List(90.0,110.0,135.0,150.0),
    List(60.0,75.0,80.0,95.0)
  ),
  series_names = List("EU","NA","APAC"),
  y_label = "Revenue (M€)"
)
chart.show()</code></pre></div>
<div id="b-s-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bar({
    .title        = "Quarterly Revenue Mix",
    .variant      = "stacked",
    .labels       = {"Q1","Q2","Q3","Q4"},
    .series       = {{120,145,160,180},{90,110,135,150},{60,75,80,95}},
    .series_names = {"EU","NA","APAC"},
    .y_label      = "Revenue (M€)",
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/stacked-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-relative">

Stacked with **negative values below the zero baseline** — P&L, gains/losses. Alias: `"rel"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    *,
    variant: str = "relative",
    series: list[list[float]],
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    bar_gap: float = 0.2,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    legend_position: str = "right",
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"relative"</code> / <code>"rel"</code></span><span><strong>Required</strong> <code>series</code>, <code>series_names</code></span><span><strong>Note</strong> negatives render below 0</span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-r">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-r','b-r-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('b-r','b-r-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('b-r','b-r-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('b-r','b-r-r',this)">R</button>
<button class="sp-tb" onclick="spTab('b-r','b-r-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('b-r','b-r-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('b-r','b-r-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('b-r','b-r-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('b-r','b-r-cpp',this)">C++</button>
</div>
<div id="b-r-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Net Cash Flow by Quarter",
    variant="relative",
    labels=["Q1","Q2","Q3","Q4"],
    series=[[50,80,-40,120],[-30,20,-50,60],[40,-10,30,-20]],
    series_names=["Operations","Financing","Investing"],
    y_label="Cash Flow (k€)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="b-r-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Net Cash Flow by Quarter", variant: "relative",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[50,80,-40,120],[-30,20,-50,60],[40,-10,30,-20]],
  seriesNames: ["Operations","Financing","Investing"],
  yLabel: "Cash Flow (k€)", gridlines: true,
});
chart.show();</code></pre></div>
<div id="b-r-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Net Cash Flow by Quarter", variant: "relative",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[50,80,-40,120],[-30,20,-50,60],[40,-10,30,-20]],
  seriesNames: ["Operations","Financing","Investing"],
  yLabel: "Cash Flow (k€)", gridlines: true,
});
chart.show();</code></pre></div>
<div id="b-r-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "Net Cash Flow by Quarter",
  variant = "relative",
  labels = c("Q1","Q2","Q3","Q4"),
  series = list(c(50,80,-40,120), c(-30,20,-50,60), c(40,-10,30,-20)),
  series_names = c("Operations","Financing","Investing"),
  y_label = "Cash Flow (k€)",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="b-r-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bar()
        .title("Net Cash Flow by Quarter")
        .variant("relative")
        .labels(vec!["Q1","Q2","Q3","Q4"])
        .series(vec![
            vec![50.0, 80.0,-40.0,120.0],
            vec![-30.0,20.0,-50.0, 60.0],
            vec![40.0,-10.0, 30.0,-20.0],
        ])
        .series_names(vec!["Operations","Financing","Investing"])
        .y_label("Cash Flow (k€)")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="b-r-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bar()
    .title("Net Cash Flow by Quarter")
    .variant("relative")
    .labels(List.of("Q1","Q2","Q3","Q4"))
    .series(List.of(
        List.of(50.0,80.0,-40.0,120.0),
        List.of(-30.0,20.0,-50.0,60.0),
        List.of(40.0,-10.0,30.0,-20.0)
    ))
    .seriesNames(List.of("Operations","Financing","Investing"))
    .yLabel("Cash Flow (k€)")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="b-r-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bar(
    title: "Net Cash Flow by Quarter",
    variant: "relative",
    labels: ["Q1","Q2","Q3","Q4"],
    series: [[50,80,-40,120],[-30,20,-50,60],[40,-10,30,-20]],
    seriesNames: ["Operations","Financing","Investing"],
    yLabel: "Cash Flow (k€)",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="b-r-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bar(
  title = "Net Cash Flow by Quarter",
  variant = "relative",
  labels = List("Q1","Q2","Q3","Q4"),
  series = List(
    List(50.0,80.0,-40.0,120.0),
    List(-30.0,20.0,-50.0,60.0),
    List(40.0,-10.0,30.0,-20.0)
  ),
  series_names = List("Operations","Financing","Investing"),
  y_label = "Cash Flow (k€)",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="b-r-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bar({
    .title        = "Net Cash Flow by Quarter",
    .variant      = "relative",
    .labels       = {"Q1","Q2","Q3","Q4"},
    .series       = {{50,-40,120,80},{-30,20,-50,60},{40,-10,30,-20}},
    .series_names = {"Operations","Financing","Investing"},
    .y_label      = "Cash Flow (k€)",
    .gridlines    = true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/relative-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-gstack">

Groups of stacked sub-bars per category. `offset_groups` assigns a stack-group name to each series.

```python
sp.bar(
    title: str,
    labels: list[str],
    *,
    variant: str = "grouped_stacked",
    series: list[list[float]],
    series_names: list[str] | None = None,
    offset_groups: list[str],
    palette: list[int] | None = None,
    bar_gap: float = 0.2,
    bargroup_gap: float = 0.1,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    legend_position: str = "right",
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"grouped_stacked"</code></span><span><strong>Required</strong> <code>series</code>, <code>offset_groups</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-gs">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-gs','b-gs-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('b-gs','b-gs-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('b-gs','b-gs-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('b-gs','b-gs-r',this)">R</button>
<button class="sp-tb" onclick="spTab('b-gs','b-gs-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('b-gs','b-gs-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('b-gs','b-gs-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('b-gs','b-gs-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('b-gs','b-gs-cpp',this)">C++</button>
</div>
<div id="b-gs-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Revenue by Channel × Region",
    variant="grouped_stacked",
    labels=["Q1","Q2","Q3","Q4"],
    series=[
        [50,60,70,80],
        [30,40,45,55],
        [40,50,55,65],
        [20,30,35,40],
    ],
    series_names=["Online·EU","Retail·EU","Online·NA","Retail·NA"],
    offset_groups=["EU","EU","NA","NA"],
    y_label="Revenue (M€)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="b-gs-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Revenue by Channel × Region", variant: "grouped_stacked",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[50,60,70,80],[30,40,45,55],[40,50,55,65],[20,30,35,40]],
  seriesNames: ["Online·EU","Retail·EU","Online·NA","Retail·NA"],
  offsetGroups: ["EU","EU","NA","NA"],
  yLabel: "Revenue (M€)", gridlines: true,
});
chart.show();</code></pre></div>
<div id="b-gs-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Revenue by Channel × Region", variant: "grouped_stacked",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[50,60,70,80],[30,40,45,55],[40,50,55,65],[20,30,35,40]],
  seriesNames: ["Online·EU","Retail·EU","Online·NA","Retail·NA"],
  offsetGroups: ["EU","EU","NA","NA"],
  yLabel: "Revenue (M€)", gridlines: true,
});
chart.show();</code></pre></div>
<div id="b-gs-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "Revenue by Channel x Region",
  variant = "grouped_stacked",
  labels = c("Q1","Q2","Q3","Q4"),
  series = list(c(50,60,70,80), c(30,40,45,55), c(40,50,55,65), c(20,30,35,40)),
  series_names = c("Online.EU","Retail.EU","Online.NA","Retail.NA"),
  offset_groups = c("EU","EU","NA","NA"),
  y_label = "Revenue (M€)",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="b-gs-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bar()
        .title("Revenue by Channel x Region")
        .variant("grouped_stacked")
        .labels(vec!["Q1","Q2","Q3","Q4"])
        .series(vec![
            vec![50.0,60.0,70.0,80.0],
            vec![30.0,40.0,45.0,55.0],
            vec![40.0,50.0,55.0,65.0],
            vec![20.0,30.0,35.0,40.0],
        ])
        .series_names(vec!["Online-EU","Retail-EU","Online-NA","Retail-NA"])
        .offset_groups(vec!["EU","EU","NA","NA"])
        .y_label("Revenue (M€)")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="b-gs-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bar()
    .title("Revenue by Channel x Region")
    .variant("grouped_stacked")
    .labels(List.of("Q1","Q2","Q3","Q4"))
    .series(List.of(
        List.of(50.0,60.0,70.0,80.0),
        List.of(30.0,40.0,45.0,55.0),
        List.of(40.0,50.0,55.0,65.0),
        List.of(20.0,30.0,35.0,40.0)
    ))
    .seriesNames(List.of("Online-EU","Retail-EU","Online-NA","Retail-NA"))
    .offsetGroups(List.of("EU","EU","NA","NA"))
    .yLabel("Revenue (M€)")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="b-gs-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bar(
    title: "Revenue by Channel x Region",
    variant: "grouped_stacked",
    labels: ["Q1","Q2","Q3","Q4"],
    series: [[50,60,70,80],[30,40,45,55],[40,50,55,65],[20,30,35,40]],
    seriesNames: ["Online-EU","Retail-EU","Online-NA","Retail-NA"],
    offsetGroups: ["EU","EU","NA","NA"],
    yLabel: "Revenue (M€)",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="b-gs-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bar(
  title = "Revenue by Channel x Region",
  variant = "grouped_stacked",
  labels = List("Q1","Q2","Q3","Q4"),
  series = List(
    List(50.0,60.0,70.0,80.0),
    List(30.0,40.0,45.0,55.0),
    List(40.0,50.0,55.0,65.0),
    List(20.0,30.0,35.0,40.0)
  ),
  series_names = List("Online-EU","Retail-EU","Online-NA","Retail-NA"),
  offset_groups = List("EU","EU","NA","NA"),
  y_label = "Revenue (M€)",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="b-gs-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bar({
    .title         = "Revenue by Channel x Region",
    .variant       = "grouped_stacked",
    .labels        = {"Q1","Q2","Q3","Q4"},
    .series        = {{50,60,70,80},{30,40,45,55},{40,50,55,65},{20,30,35,40}},
    .series_names  = {"Online-EU","Retail-EU","Online-NA","Retail-NA"},
    .offset_groups = {"EU","EU","NA","NA"},
    .y_label       = "Revenue (M€)",
    .gridlines     = true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/grouped-stacked-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-marimekko">

Variable-width stacked bars (mosaic plot). `widths` encodes one dimension, stacked segments encode share. Aliases: `"mekko"`, `"mosaic"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    *,
    variant: str = "marimekko",
    series: list[list[float]],
    series_names: list[str] | None = None,
    widths: list[float],
    show_text: bool = False,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    legend_position: str = "right",
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"marimekko"</code> / <code>"mekko"</code> / <code>"mosaic"</code></span><span><strong>Required</strong> <code>series</code>, <code>widths</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-m">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-m','b-m-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('b-m','b-m-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('b-m','b-m-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('b-m','b-m-r',this)">R</button>
<button class="sp-tb" onclick="spTab('b-m','b-m-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('b-m','b-m-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('b-m','b-m-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('b-m','b-m-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('b-m','b-m-cpp',this)">C++</button>
</div>
<div id="b-m-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Market Share by Segment Size",
    variant="marimekko",
    labels=["Auto","Tech","Retail","Energy","Health"],
    widths=[40,25,15,12,8],
    series=[
        [45,30,20,50,60],
        [30,35,40,25,25],
        [25,35,40,25,15],
    ],
    series_names=["Leader","Challenger","Others"],
    show_text=True,
)
chart.show()</code></pre></div>
<div id="b-m-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Market Share by Segment Size", variant: "marimekko",
  labels: ["Auto","Tech","Retail","Energy","Health"],
  widths: [40,25,15,12,8],
  series: [[45,30,20,50,60],[30,35,40,25,25],[25,35,40,25,15]],
  seriesNames: ["Leader","Challenger","Others"],
  showText: true,
});
chart.show();</code></pre></div>
<div id="b-m-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Market Share by Segment Size", variant: "marimekko",
  labels: ["Auto","Tech","Retail","Energy","Health"],
  widths: [40,25,15,12,8],
  series: [[45,30,20,50,60],[30,35,40,25,25],[25,35,40,25,15]],
  seriesNames: ["Leader","Challenger","Others"],
  showText: true,
});
chart.show();</code></pre></div>
<div id="b-m-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "Market Share by Segment Size",
  variant = "marimekko",
  labels = c("Auto","Tech","Retail","Energy","Health"),
  widths = c(40,25,15,12,8),
  series = list(c(45,30,20,50,60), c(30,35,40,25,25), c(25,35,40,25,15)),
  series_names = c("Leader","Challenger","Others"),
  show_text = TRUE
)
chart$show()</code></pre></div>
<div id="b-m-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bar()
        .title("Market Share by Segment Size")
        .variant("marimekko")
        .labels(vec!["Auto","Tech","Retail","Energy","Health"])
        .widths(vec![40.0,25.0,15.0,12.0,8.0])
        .series(vec![
            vec![45.0,30.0,20.0,50.0,60.0],
            vec![30.0,35.0,40.0,25.0,25.0],
            vec![25.0,35.0,40.0,25.0,15.0],
        ])
        .series_names(vec!["Leader","Challenger","Others"])
        .show_text(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="b-m-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bar()
    .title("Market Share by Segment Size")
    .variant("marimekko")
    .labels(List.of("Auto","Tech","Retail","Energy","Health"))
    .widths(List.of(40.0,25.0,15.0,12.0,8.0))
    .series(List.of(
        List.of(45.0,30.0,20.0,50.0,60.0),
        List.of(30.0,35.0,40.0,25.0,25.0),
        List.of(25.0,35.0,40.0,25.0,15.0)
    ))
    .seriesNames(List.of("Leader","Challenger","Others"))
    .showText(true)
    .build();
chart.show();</code></pre></div>
<div id="b-m-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bar(
    title: "Market Share by Segment Size",
    variant: "marimekko",
    labels: ["Auto","Tech","Retail","Energy","Health"],
    widths: [40,25,15,12,8],
    series: [[45,30,20,50,60],[30,35,40,25,25],[25,35,40,25,15]],
    seriesNames: ["Leader","Challenger","Others"],
    showText: true
);
chart.Show();</code></pre></div>
<div id="b-m-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bar(
  title = "Market Share by Segment Size",
  variant = "marimekko",
  labels = List("Auto","Tech","Retail","Energy","Health"),
  widths = List(40.0,25.0,15.0,12.0,8.0),
  series = List(
    List(45.0,30.0,20.0,50.0,60.0),
    List(30.0,35.0,40.0,25.0,25.0),
    List(25.0,35.0,40.0,25.0,15.0)
  ),
  series_names = List("Leader","Challenger","Others"),
  show_text = true
)
chart.show()</code></pre></div>
<div id="b-m-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bar({
    .title        = "Market Share by Segment Size",
    .variant      = "marimekko",
    .labels       = {"Auto","Tech","Retail","Energy","Health"},
    .widths       = {40,25,15,12,8},
    .series       = {{45,30,20,50,60},{30,35,40,25,25},{25,35,40,25,15}},
    .series_names = {"Leader","Challenger","Others"},
    .show_text    = true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/marimekko-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-pictogram">

A bar made of repeated icons. Each icon represents `units_per_icon` units. Alias: `"icon"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    variant: str = "pictogram",
    icon_size: int = 24,
    max_icons_per_column: int = 10,
    units_per_icon: float = 1.0,
    unit_description: str = "",
    color_hex: int = 0,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"pictogram"</code> / <code>"icon"</code></span><span><strong>Required</strong> <code>values</code>, <code>units_per_icon</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-p">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-p','b-p-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('b-p','b-p-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('b-p','b-p-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('b-p','b-p-r',this)">R</button>
<button class="sp-tb" onclick="spTab('b-p','b-p-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('b-p','b-p-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('b-p','b-p-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('b-p','b-p-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('b-p','b-p-cpp',this)">C++</button>
</div>
<div id="b-p-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Coffees Per Day",
    variant="pictogram",
    labels=["Mon","Tue","Wed","Thu","Fri"],
    values=[3, 5, 4, 7, 6],
    icon_size=22,
    units_per_icon=1,
    max_icons_per_column=4,
    unit_description="cup",
    color_hex=0x6F4E37,
)
chart.show()</code></pre></div>
<div id="b-p-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Coffees Per Day", variant: "pictogram",
  labels: ["Mon","Tue","Wed","Thu","Fri"],
  values: [3,5,4,7,6],
  iconSize: 22, unitsPerIcon: 1,
  maxIconsPerColumn: 4, unitDescription: "cup",
  colorHex: 0x6F4E37,
});
chart.show();</code></pre></div>
<div id="b-p-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Coffees Per Day", variant: "pictogram",
  labels: ["Mon","Tue","Wed","Thu","Fri"],
  values: [3,5,4,7,6],
  iconSize: 22, unitsPerIcon: 1,
  maxIconsPerColumn: 4, unitDescription: "cup",
  colorHex: 0x6F4E37,
});
chart.show();</code></pre></div>
<div id="b-p-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "Coffees Per Day",
  variant = "pictogram",
  labels = c("Mon","Tue","Wed","Thu","Fri"),
  values = c(3, 5, 4, 7, 6),
  icon_size = 22,
  units_per_icon = 1,
  max_icons_per_column = 4,
  unit_description = "cup",
  color_hex = 0x6F4E37
)
chart$show()</code></pre></div>
<div id="b-p-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bar()
        .title("Coffees Per Day")
        .variant("pictogram")
        .labels(vec!["Mon","Tue","Wed","Thu","Fri"])
        .values(vec![3.0, 5.0, 4.0, 7.0, 6.0])
        .icon_size(22)
        .units_per_icon(1.0)
        .max_icons_per_column(4)
        .unit_description("cup")
        .color_hex(0x6F4E37)
        .build();
    chart.show();
}</code></pre></div>
<div id="b-p-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bar()
    .title("Coffees Per Day")
    .variant("pictogram")
    .labels(List.of("Mon","Tue","Wed","Thu","Fri"))
    .values(List.of(3.0,5.0,4.0,7.0,6.0))
    .iconSize(22)
    .unitsPerIcon(1.0)
    .maxIconsPerColumn(4)
    .unitDescription("cup")
    .colorHex(0x6F4E37)
    .build();
chart.show();</code></pre></div>
<div id="b-p-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bar(
    title: "Coffees Per Day",
    variant: "pictogram",
    labels: ["Mon","Tue","Wed","Thu","Fri"],
    values: [3,5,4,7,6],
    iconSize: 22,
    unitsPerIcon: 1,
    maxIconsPerColumn: 4,
    unitDescription: "cup",
    colorHex: 0x6F4E37
);
chart.Show();</code></pre></div>
<div id="b-p-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bar(
  title = "Coffees Per Day",
  variant = "pictogram",
  labels = List("Mon","Tue","Wed","Thu","Fri"),
  values = List(3.0, 5.0, 4.0, 7.0, 6.0),
  icon_size = 22,
  units_per_icon = 1.0,
  max_icons_per_column = 4,
  unit_description = "cup",
  color_hex = 0x6F4E37
)
chart.show()</code></pre></div>
<div id="b-p-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bar({
    .title                = "Coffees Per Day",
    .variant              = "pictogram",
    .labels               = {"Mon","Tue","Wed","Thu","Fri"},
    .values               = {3,5,4,7,6},
    .icon_size            = 22,
    .units_per_icon       = 1.0,
    .max_icons_per_column = 4,
    .unit_description     = "cup",
    .color_hex            = 0x6F4E37,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pictogram-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-en-multicategory">

Two-level hierarchical x axis. `super_categories` groups adjacent bars under a bracket label. Alias: `"multi"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    variant: str = "multicategory",
    super_categories: list[str],
    series: list[list[float]] | None = None,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"multicategory"</code> / <code>"multi"</code></span><span><strong>Required</strong> <code>values</code>, <code>super_categories</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="b-mc">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('b-mc','b-mc-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('b-mc','b-mc-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('b-mc','b-mc-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('b-mc','b-mc-r',this)">R</button>
<button class="sp-tb" onclick="spTab('b-mc','b-mc-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('b-mc','b-mc-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('b-mc','b-mc-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('b-mc','b-mc-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('b-mc','b-mc-cpp',this)">C++</button>
</div>
<div id="b-mc-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Sales by Region & Quarter",
    variant="multicategory",
    labels=["Q1","Q2","Q3","Q4","Q1","Q2","Q3","Q4"],
    super_categories=["EU","EU","EU","EU","NA","NA","NA","NA"],
    values=[120,145,160,180,90,110,135,150],
    y_label="Revenue (M€)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="b-mc-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Sales by Region & Quarter", variant: "multicategory",
  labels: ["Q1","Q2","Q3","Q4","Q1","Q2","Q3","Q4"],
  superCategories: ["EU","EU","EU","EU","NA","NA","NA","NA"],
  values: [120,145,160,180,90,110,135,150],
  yLabel: "Revenue (M€)", gridlines: true,
});
chart.show();</code></pre></div>
<div id="b-mc-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Sales by Region & Quarter", variant: "multicategory",
  labels: ["Q1","Q2","Q3","Q4","Q1","Q2","Q3","Q4"],
  superCategories: ["EU","EU","EU","EU","NA","NA","NA","NA"],
  values: [120,145,160,180,90,110,135,150],
  yLabel: "Revenue (M€)", gridlines: true,
});
chart.show();</code></pre></div>
<div id="b-mc-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "Sales by Region & Quarter",
  variant = "multicategory",
  labels = c("Q1","Q2","Q3","Q4","Q1","Q2","Q3","Q4"),
  super_categories = c("EU","EU","EU","EU","NA","NA","NA","NA"),
  values = c(120,145,160,180,90,110,135,150),
  y_label = "Revenue (M€)",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="b-mc-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::bar()
        .title("Sales by Region & Quarter")
        .variant("multicategory")
        .labels(vec!["Q1","Q2","Q3","Q4","Q1","Q2","Q3","Q4"])
        .super_categories(vec!["EU","EU","EU","EU","NA","NA","NA","NA"])
        .values(vec![120.0,145.0,160.0,180.0,90.0,110.0,135.0,150.0])
        .y_label("Revenue (M€)")
        .gridlines(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="b-mc-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bar()
    .title("Sales by Region & Quarter")
    .variant("multicategory")
    .labels(List.of("Q1","Q2","Q3","Q4","Q1","Q2","Q3","Q4"))
    .superCategories(List.of("EU","EU","EU","EU","NA","NA","NA","NA"))
    .values(List.of(120.0,145.0,160.0,180.0,90.0,110.0,135.0,150.0))
    .yLabel("Revenue (M€)")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="b-mc-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bar(
    title: "Sales by Region & Quarter",
    variant: "multicategory",
    labels: ["Q1","Q2","Q3","Q4","Q1","Q2","Q3","Q4"],
    superCategories: ["EU","EU","EU","EU","NA","NA","NA","NA"],
    values: [120,145,160,180,90,110,135,150],
    yLabel: "Revenue (M€)",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="b-mc-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.bar(
  title = "Sales by Region & Quarter",
  variant = "multicategory",
  labels = List("Q1","Q2","Q3","Q4","Q1","Q2","Q3","Q4"),
  super_categories = List("EU","EU","EU","EU","NA","NA","NA","NA"),
  values = List(120.0,145.0,160.0,180.0,90.0,110.0,135.0,150.0),
  y_label = "Revenue (M€)",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="b-mc-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bar({
    .title             = "Sales by Region & Quarter",
    .variant           = "multicategory",
    .labels            = {"Q1","Q2","Q3","Q4","Q1","Q2","Q3","Q4"},
    .super_categories  = {"EU","EU","EU","EU","NA","NA","NA","NA"},
    .values            = {120,145,160,180,90,110,135,150},
    .y_label           = "Revenue (M€)",
    .gridlines         = true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/multicategory-bar.html"></iframe>
</div>

</div><!-- /sp-cls-body -->
</div><!-- /bar-en -->

---

## See also

- [Lollipop](lollipop.md) — `sp.lollipop()`
- [Waterfall](waterfall.md) — `sp.waterfall()`
- [Bullet Chart](bullet.md) — `sp.bullet()`

</div><!-- /lang-en -->


<div class="lang-fr" style="display:none">

## Signature

`sp.bar(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

Alias : `sp.bar`, `sp.bars`, `sp.bar_unified`, `sp.bars_unified`, `sp.bar_family`, `sp.bar_chart`

## Description

`sp.bar()` est le point d'entrée unique pour toute la famille de graphiques en barres. Le paramètre `variant` sélectionne la stratégie de rendu.

| Variante | Cas d'usage | Arguments clés |
|----------|-------------|----------------|
| `"basic"` | Série unique, barres verticales | `labels`, `values` |
| `"horizontal"` / `"h"` | Série unique, barres horizontales | `labels`, `values` |
| `"grouped"` / `"group"` | Multi-séries côte à côte | `series`, `series_names` |
| `"stacked"` / `"stack"` | Multi-séries empilées | `series`, `series_names` |
| `"relative"` / `"rel"` | Empilé + négatifs sous 0 | `series`, `series_names` |
| `"grouped_stacked"` | Groupes de barres empilées | `series`, `series_names`, `offset_groups` |
| `"marimekko"` / `"mekko"` | Mosaïque à largeur variable | `series`, `series_names`, `widths` |
| `"pictogram"` / `"icon"` | Barre d'icônes répétées | `labels`, `values`, `units_per_icon` |
| `"multicategory"` / `"multi"` | Axe x hiérarchique | `labels`, `values`, `super_categories` |

---

## Paramètres

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `labels` | `list[str]` | `None` | toutes | Labels de catégorie |
| `values` | `list[float]` | `None` | basic, horizontal, pictogram, multicategory | Valeurs série unique |
| `variant` | `str` | `"basic"` | — | Variante de rendu |
| `series` | `list[list[float]]` | `None` | multi-séries | Données multi-séries |
| `series_names` | `list[str]` | `None` | multi-séries | Noms des séries (légende) |
| `offset_groups` | `list[str]` | `None` | grouped_stacked | Groupe d'empilement par série |
| `widths` | `list[float]` | `None` | marimekko | Largeur relative de chaque colonne |
| `super_categories` | `list[str]` | `None` | multicategory | Label chapeau de chaque barre |
| `icon_size` | `int` | `24` | pictogram | Taille des icônes en px |
| `max_icons_per_column` | `int` | `10` | pictogram | Icônes max par colonne |
| `units_per_icon` | `float` | `1.0` | pictogram | Unités par icône |
| `unit_description` | `str` | `""` | pictogram | Label unité |
| `color_hex` | `int` | `0` | basic, horizontal, pictogram | Couleur unique (entier hex) |
| `palette` | `list[int]` | `None` | toutes | Palette personnalisée |
| `show_text` | `bool` | `False` | basic, horizontal, marimekko | Afficher les valeurs sur les barres |
| `corner_radius` | `int` | `0` | basic, horizontal | Rayon des coins en px |
| `bar_gap` | `float` | `0.2` | basic, horizontal, relative, grouped_stacked | Espacement entre catégories |
| `bargroup_gap` | `float` | `0.1` | grouped, grouped_stacked | Espacement entre barres d'un groupe |
| `width` | `int` | `900` | toutes | Largeur du canevas en px |
| `height` | `int` | `480` | toutes | Hauteur du canevas en px |
| `x_label` | `str` | `""` | toutes | Label axe X |
| `y_label` | `str` | `""` | toutes | Label axe Y |
| `gridlines` | `bool` | `False` | toutes | Lignes de grille horizontales |
| `sort_order` | `str` | `"none"` | basic, horizontal | `"asc"`, `"desc"`, `"alpha"`, `"alpha_desc"`, `"none"` |
| `legend_position` | `str` | `"right"` | multi-séries | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | toutes | Couleur de fond CSS |
| `no_x_axis` | `bool` | `False` | toutes | Masquer l'axe X |
| `no_y_axis` | `bool` | `False` | toutes | Masquer l'axe Y |

---

## Retourne

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

<div class="sp-cls sp-open" id="bar-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bar-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bar-fr','basic',this)"><span class="sp-cic">▮</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','horizontal',this)"><span class="sp-cic">▬</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','grouped',this)"><span class="sp-cic">▐▐</span><span class="sp-clb">Groupé</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','stacked',this)"><span class="sp-cic">▦</span><span class="sp-clb">Empilé</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','relative',this)"><span class="sp-cic">±</span><span class="sp-clb">Relatif</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','gstack',this)"><span class="sp-cic">▤</span><span class="sp-clb">Groupé-Empilé</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','marimekko',this)"><span class="sp-cic">▤</span><span class="sp-clb">Marimekko</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','pictogram',this)"><span class="sp-cic">☰</span><span class="sp-clb">Pictogramme</span></button>
<button class="sp-cls-tab" onclick="spCls('bar-fr','multicategory',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Multi-catégories</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bar-fr-basic">

Barres verticales — une valeur par catégorie.

```python
sp.bar(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    variant: str = "basic",
    color_hex: int = 0,
    show_text: bool = False,
    corner_radius: int = 0,
    bar_gap: float = 0.2,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>sp.bar</code> <code>sp.bars</code> <code>sp.bar_unified</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bf-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bf-basic','bf-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bf-basic','bf-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bf-basic','bf-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bf-basic','bf-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bf-basic','bf-basic-rust',this)">Rust</button>
</div>
<div id="bf-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Chiffre d'affaires mensuel",
    variant="basic",
    labels=["Jan","Fév","Mar","Avr","Mai","Juin"],
    values=[1200, 1850, 2100, 1750, 2400, 2800],
    y_label="CA (€)",
    gridlines=True,
    show_text=True,
)
chart.show()</code></pre></div>
<div id="bf-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Chiffre d'affaires mensuel", variant: "basic",
  labels: ["Jan","Fév","Mar","Avr","Mai","Juin"],
  values: [1200,1850,2100,1750,2400,2800],
  yLabel: "CA (€)", gridlines: true, showText: true,
});
chart.show();</code></pre></div>
<div id="bf-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Chiffre d'affaires mensuel", variant: "basic",
  labels: ["Jan","Fév","Mar","Avr","Mai","Juin"],
  values: [1200,1850,2100,1750,2400,2800],
  yLabel: "CA (€)", gridlines: true, showText: true,
});
chart.show();</code></pre></div>
<div id="bf-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "Chiffre d'affaires mensuel",
  variant = "basic",
  labels = c("Jan","Fév","Mar","Avr","Mai","Juin"),
  values = c(1200,1850,2100,1750,2400,2800),
  y_label = "CA (€)", gridlines = TRUE, show_text = TRUE
)
chart$show()</code></pre></div>
<div id="bf-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;
fn main() {
    let chart = sp::bar()
        .title("Chiffre d'affaires mensuel").variant("basic")
        .labels(vec!["Jan","Fév","Mar","Avr","Mai","Juin"])
        .values(vec![1200.0,1850.0,2100.0,1750.0,2400.0,2800.0])
        .y_label("CA (€)").gridlines(true).show_text(true).build();
    chart.show();
}</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-horizontal">

Barres horizontales — idéal pour les longs noms de catégories. Alias : `"h"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    variant: str = "horizontal",
    color_hex: int = 0,
    show_text: bool = False,
    bar_gap: float = 0.2,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    palette: list[int] | None = None,
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"horizontal"</code> / <code>"h"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bf-h">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bf-h','bf-h-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bf-h','bf-h-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bf-h','bf-h-r',this)">R</button>
</div>
<div id="bf-h-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Population par pays",
    variant="horizontal",
    labels=["Chine","Inde","USA","Indonésie","Pakistan"],
    values=[1411, 1393, 332, 273, 220],
    x_label="Population (M)", gridlines=True, sort_order="desc",
)
chart.show()</code></pre></div>
<div id="bf-h-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Population par pays", variant: "horizontal",
  labels: ["Chine","Inde","USA","Indonésie","Pakistan"],
  values: [1411,1393,332,273,220],
  xLabel: "Population (M)", gridlines: true, sortOrder: "desc",
});
chart.show();</code></pre></div>
<div id="bf-h-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "Population par pays", variant = "horizontal",
  labels = c("Chine","Inde","USA","Indonésie","Pakistan"),
  values = c(1411,1393,332,273,220),
  x_label = "Population (M)", gridlines = TRUE, sort_order = "desc"
)
chart$show()</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hbar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-grouped">

Plusieurs séries côte à côte par catégorie. Alias : `"group"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    *,
    variant: str = "grouped",
    series: list[list[float]],
    series_names: list[str] | None = None,
    bar_gap: float = 0.2,
    bargroup_gap: float = 0.1,
    show_text: bool = False,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    legend_position: str = "right",
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"grouped"</code> / <code>"group"</code></span><span><strong>Requis</strong> <code>series</code>, <code>series_names</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bf-g">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bf-g','bf-g-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bf-g','bf-g-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bf-g','bf-g-r',this)">R</button>
</div>
<div id="bf-g-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="CA par région",
    variant="grouped",
    labels=["T1","T2","T3","T4"],
    series=[[120,145,160,180],[90,110,135,150],[60,75,80,95]],
    series_names=["UE","NA","APAC"],
    y_label="CA (M€)", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bf-g-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "CA par région", variant: "grouped",
  labels: ["T1","T2","T3","T4"],
  series: [[120,145,160,180],[90,110,135,150],[60,75,80,95]],
  seriesNames: ["UE","NA","APAC"],
  yLabel: "CA (M€)", gridlines: true,
});
chart.show();</code></pre></div>
<div id="bf-g-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "CA par région", variant = "grouped",
  labels = c("T1","T2","T3","T4"),
  series = list(c(120,145,160,180),c(90,110,135,150),c(60,75,80,95)),
  series_names = c("UE","NA","APAC"),
  y_label = "CA (M€)", gridlines = TRUE
)
chart$show()</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/grouped-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-stacked">

Séries empilées verticalement — montre la part de chaque série dans le total. Alias : `"stack"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    *,
    variant: str = "stacked",
    series: list[list[float]],
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    legend_position: str = "right",
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"stacked"</code> / <code>"stack"</code></span><span><strong>Requis</strong> <code>series</code>, <code>series_names</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bf-s">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bf-s','bf-s-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bf-s','bf-s-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bf-s','bf-s-r',this)">R</button>
</div>
<div id="bf-s-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Mix de revenus trimestriels",
    variant="stacked",
    labels=["T1","T2","T3","T4"],
    series=[[120,145,160,180],[90,110,135,150],[60,75,80,95]],
    series_names=["UE","NA","APAC"],
    y_label="CA (M€)",
)
chart.show()</code></pre></div>
<div id="bf-s-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Mix de revenus trimestriels", variant: "stacked",
  labels: ["T1","T2","T3","T4"],
  series: [[120,145,160,180],[90,110,135,150],[60,75,80,95]],
  seriesNames: ["UE","NA","APAC"], yLabel: "CA (M€)",
});
chart.show();</code></pre></div>
<div id="bf-s-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title = "Mix de revenus trimestriels", variant = "stacked",
  labels = c("T1","T2","T3","T4"),
  series = list(c(120,145,160,180),c(90,110,135,150),c(60,75,80,95)),
  series_names = c("UE","NA","APAC"), y_label = "CA (M€)"
)
chart$show()</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/stacked-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-relative">

Empilé avec **valeurs négatives sous l'axe zéro** — flux de trésorerie, P&L. Alias : `"rel"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    *,
    variant: str = "relative",
    series: list[list[float]],
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    bar_gap: float = 0.2,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    legend_position: str = "right",
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"relative"</code> / <code>"rel"</code></span><span><strong>Requis</strong> <code>series</code>, <code>series_names</code></span><span><strong>Note</strong> les négatifs s'affichent sous 0</span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bf-r">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bf-r','bf-r-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bf-r','bf-r-js',this)">JavaScript</button>
</div>
<div id="bf-r-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Flux de trésorerie net par trimestre",
    variant="relative",
    labels=["T1","T2","T3","T4"],
    series=[[50,80,-40,120],[-30,20,-50,60],[40,-10,30,-20]],
    series_names=["Exploitation","Financement","Investissement"],
    y_label="Cash Flow (k€)", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bf-r-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Flux de trésorerie net par trimestre", variant: "relative",
  labels: ["T1","T2","T3","T4"],
  series: [[50,80,-40,120],[-30,20,-50,60],[40,-10,30,-20]],
  seriesNames: ["Exploitation","Financement","Investissement"],
  yLabel: "Cash Flow (k€)", gridlines: true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/relative-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-gstack">

Groupes de sous-barres empilées par catégorie. `offset_groups` assigne un nom de pile à chaque série.

```python
sp.bar(
    title: str,
    labels: list[str],
    *,
    variant: str = "grouped_stacked",
    series: list[list[float]],
    series_names: list[str] | None = None,
    offset_groups: list[str],
    palette: list[int] | None = None,
    bar_gap: float = 0.2,
    bargroup_gap: float = 0.1,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    legend_position: str = "right",
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"grouped_stacked"</code></span><span><strong>Requis</strong> <code>series</code>, <code>offset_groups</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bf-gs">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bf-gs','bf-gs-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bf-gs','bf-gs-js',this)">JavaScript</button>
</div>
<div id="bf-gs-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="CA par canal × région",
    variant="grouped_stacked",
    labels=["T1","T2","T3","T4"],
    series=[
        [50,60,70,80],
        [30,40,45,55],
        [40,50,55,65],
        [20,30,35,40],
    ],
    series_names=["En ligne·UE","Magasin·UE","En ligne·NA","Magasin·NA"],
    offset_groups=["UE","UE","NA","NA"],
    y_label="CA (M€)", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bf-gs-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "CA par canal x région", variant: "grouped_stacked",
  labels: ["T1","T2","T3","T4"],
  series: [[50,60,70,80],[30,40,45,55],[40,50,55,65],[20,30,35,40]],
  seriesNames: ["En ligne UE","Magasin UE","En ligne NA","Magasin NA"],
  offsetGroups: ["UE","UE","NA","NA"],
  yLabel: "CA (M€)", gridlines: true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/grouped-stacked-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-marimekko">

Barres empilées à largeur variable (mosaïque). `widths` encode une dimension, les segments empilés encodent la part. Alias : `"mekko"`, `"mosaic"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    *,
    variant: str = "marimekko",
    series: list[list[float]],
    series_names: list[str] | None = None,
    widths: list[float],
    show_text: bool = False,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    legend_position: str = "right",
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"marimekko"</code> / <code>"mekko"</code> / <code>"mosaic"</code></span><span><strong>Requis</strong> <code>series</code>, <code>widths</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bf-m">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bf-m','bf-m-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bf-m','bf-m-js',this)">JavaScript</button>
</div>
<div id="bf-m-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Part de marché par taille de segment",
    variant="marimekko",
    labels=["Auto","Tech","Retail","Énergie","Santé"],
    widths=[40,25,15,12,8],
    series=[
        [45,30,20,50,60],
        [30,35,40,25,25],
        [25,35,40,25,15],
    ],
    series_names=["Leader","Challenger","Autres"],
    show_text=True,
)
chart.show()</code></pre></div>
<div id="bf-m-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Part de marché par taille de segment", variant: "marimekko",
  labels: ["Auto","Tech","Retail","Énergie","Santé"],
  widths: [40,25,15,12,8],
  series: [[45,30,20,50,60],[30,35,40,25,25],[25,35,40,25,15]],
  seriesNames: ["Leader","Challenger","Autres"],
  showText: true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/marimekko-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-pictogram">

Barre composée d'icônes répétées. Chaque icône représente `units_per_icon` unités. Alias : `"icon"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    variant: str = "pictogram",
    icon_size: int = 24,
    max_icons_per_column: int = 10,
    units_per_icon: float = 1.0,
    unit_description: str = "",
    color_hex: int = 0,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"pictogram"</code> / <code>"icon"</code></span><span><strong>Requis</strong> <code>values</code>, <code>units_per_icon</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bf-p">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bf-p','bf-p-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bf-p','bf-p-js',this)">JavaScript</button>
</div>
<div id="bf-p-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Cafés par jour",
    variant="pictogram",
    labels=["Lun","Mar","Mer","Jeu","Ven"],
    values=[3, 5, 4, 7, 6],
    icon_size=22, units_per_icon=1,
    max_icons_per_column=4, unit_description="tasse",
    color_hex=0x6F4E37,
)
chart.show()</code></pre></div>
<div id="bf-p-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Cafés par jour", variant: "pictogram",
  labels: ["Lun","Mar","Mer","Jeu","Ven"],
  values: [3,5,4,7,6],
  iconSize: 22, unitsPerIcon: 1,
  maxIconsPerColumn: 4, unitDescription: "tasse",
  colorHex: 0x6F4E37,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/pictogram-bar.html"></iframe>
</div>

<div class="sp-variant" id="bar-fr-multicategory">

Axe x à deux niveaux. `super_categories` regroupe les barres adjacentes sous un label chapeau. Alias : `"multi"`.

```python
sp.bar(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    variant: str = "multicategory",
    super_categories: list[str],
    series: list[list[float]] | None = None,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
) -> Chart
```

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"multicategory"</code> / <code>"multi"</code></span><span><strong>Requis</strong> <code>values</code>, <code>super_categories</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-tabs" id="bf-mc">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bf-mc','bf-mc-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bf-mc','bf-mc-js',this)">JavaScript</button>
</div>
<div id="bf-mc-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Ventes par région & trimestre",
    variant="multicategory",
    labels=["T1","T2","T3","T4","T1","T2","T3","T4"],
    super_categories=["UE","UE","UE","UE","NA","NA","NA","NA"],
    values=[120,145,160,180,90,110,135,150],
    y_label="CA (M€)", gridlines=True,
)
chart.show()</code></pre></div>
<div id="bf-mc-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Ventes par région & trimestre", variant: "multicategory",
  labels: ["T1","T2","T3","T4","T1","T2","T3","T4"],
  superCategories: ["UE","UE","UE","UE","NA","NA","NA","NA"],
  values: [120,145,160,180,90,110,135,150],
  yLabel: "CA (M€)", gridlines: true,
});
chart.show();</code></pre></div>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/multicategory-bar.html"></iframe>
</div>

</div><!-- /sp-cls-body -->
</div><!-- /bar-fr -->

---

## Voir aussi

- [Lollipop](lollipop.md) — `sp.lollipop()`
- [Cascade / Waterfall](waterfall.md) — `sp.waterfall()`
- [Bullet](bullet.md) — `sp.bullet()`

</div><!-- /lang-fr -->
