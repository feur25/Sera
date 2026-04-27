# Bar Charts

<div class="lang-en">

A unified entry point for **all** bar-family chart variants. One function, one config — pick a variant via the `variant` keyword.

## Signature

```python
sp.bar(
    title: str,
    labels: list[str] | None = None,
    values: list[float] | None = None,
    *,
    variant: str = "basic",
    # multi-series (Grouped / Stacked / Relative / GroupedStacked / Marimekko / Multicategory)
    series: list[list[float]] | None = None,
    series_names: list[str] | None = None,
    # GroupedStacked
    offset_groups: list[str] | None = None,
    # Marimekko
    widths: list[float] | None = None,
    # Multicategory
    super_categories: list[str] | None = None,
    # Pictogram
    icon_size: int = 24,
    max_icons_per_column: int = 10,
    units_per_icon: float = 1.0,
    unit_description: str = "",
    # Common styling
    color_hex: int = 0,
    palette: list[int] | None = None,
    show_text: bool = False,
    corner_radius: int = 0,
    bar_gap: float = 0.2,
    bargroup_gap: float = 0.1,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    legend_position: str = "right",
    background: str | None = None,
) -> Chart
```

Aliases: `sp.bar_unified`, `sp.bars_unified`, `sp.bar_family`

---

## Description

`sp.bar()` is the single, unified entry point for the entire bar-chart family. Its `variant` keyword selects the rendering strategy and the only fields that matter change accordingly:

| Variant | Use case | Required arguments |
|---------|----------|--------------------|
| `"basic"` | Single series, vertical bars | `labels`, `values` |
| `"horizontal"` | Single series, horizontal bars | `labels`, `values` |
| `"grouped"` | Multi-series side-by-side | `labels`, `series`, `series_names` |
| `"stacked"` | Multi-series stacked vertically | `labels`, `series`, `series_names` |
| `"relative"` | Stacked with negatives below 0 baseline | `labels`, `series`, `series_names` |
| `"grouped_stacked"` | Groups of stacked sub-bars per category | `labels`, `series`, `series_names`, `offset_groups` |
| `"marimekko"` | Variable-width stacked (mosaic plot) | `labels`, `series`, `series_names`, `widths` |
| `"pictogram"` | Bar of repeated icons | `labels`, `values`, `units_per_icon` |
| `"multicategory"` | Two-level hierarchical x axis | `labels`, `super_categories`, `values` (or `series`) |

The dispatcher delegates to the same battle-tested SVG renderers used by the legacy single-purpose functions (`sp.bar_chart`, `sp.grouped_bar`, …). Output is always self-contained HTML with hardware-accelerated SVG and sub-millisecond hover.

**Migration tip:** existing scripts using `sp.bar_chart(...)` keep working. Switch to `sp.bar(variant="…")` when you want consistent semantics across the whole family.

---

## Returns

`Chart` — object with `.html` property containing the full self-contained HTML.

---

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-variant-nav{display:flex;flex-wrap:wrap;gap:6px;margin:1em 0 1.5em;padding:10px;border:1px solid #334155;border-radius:8px;background:#0f172a}
.sp-vbtn{padding:8px 14px;border:1px solid #334155;background:#1e293b;color:#94a3b8;border-radius:6px;cursor:pointer;font-size:12px;font-weight:600;transition:all .15s}
.sp-vbtn:hover{color:#e2e8f0;border-color:#6366f1}
.sp-vbtn.sp-vact{background:#6366f1;color:#fff;border-color:#6366f1}
.sp-variant{display:none}
.sp-variant.sp-von{display:block}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spVar(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-vbtn').forEach(function(b){b.classList.remove('sp-vact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-vact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Examples — pick a variant

<div id="bar-charts-en">
<div class="sp-variant-nav">
<button class="sp-vbtn sp-vact" onclick="spVar('bar-charts-en','basic',this)">Basic</button>
<button class="sp-vbtn" onclick="spVar('bar-charts-en','horizontal',this)">Horizontal</button>
<button class="sp-vbtn" onclick="spVar('bar-charts-en','grouped',this)">Grouped</button>
<button class="sp-vbtn" onclick="spVar('bar-charts-en','stacked',this)">Stacked</button>
<button class="sp-vbtn" onclick="spVar('bar-charts-en','relative',this)">Relative</button>
<button class="sp-vbtn" onclick="spVar('bar-charts-en','gstack',this)">Grouped-Stacked</button>
<button class="sp-vbtn" onclick="spVar('bar-charts-en','marimekko',this)">Marimekko</button>
<button class="sp-vbtn" onclick="spVar('bar-charts-en','pictogram',this)">Pictogram</button>
<button class="sp-vbtn" onclick="spVar('bar-charts-en','multi',this)">Multicategory</button>
</div>

<!-- ─────────────── Basic ─────────────── -->
<div class="sp-variant sp-von" id="bar-charts-en-basic">

A simple vertical bar chart — one value per category.

<div class="sp-tabs" id="bce-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bce-basic','bce-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bce-basic','bce-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bce-basic','bce-basic-ts',this)">TypeScript</button>
</div>
<div id="bce-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
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
<div id="bce-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Monthly Revenue",
  variant: "basic",
  labels: ["Jan","Feb","Mar","Apr","May","Jun"],
  values: [1200, 1850, 2100, 1750, 2400, 2800],
  yLabel: "Revenue (€)",
  gridlines: true,
  showText: true,
});
chart.show();</code></pre></div>
<div id="bce-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Monthly Revenue",
  variant: "basic",
  labels: ["Jan","Feb","Mar","Apr","May","Jun"],
  values: [1200, 1850, 2100, 1750, 2400, 2800],
  yLabel: "Revenue (€)",
  gridlines: true,
  showText: true,
});
chart.show();</code></pre></div>
</div>

</div>

<!-- ─────────────── Horizontal ─────────────── -->
<div class="sp-variant" id="bar-charts-en-horizontal">

Same as basic but bars run horizontally — better for long category names.

<div class="sp-tabs" id="bce-h">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bce-h','bce-h-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bce-h','bce-h-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bce-h','bce-h-ts',this)">TypeScript</button>
</div>
<div id="bce-h-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Population by Country",
    variant="horizontal",
    labels=["China","India","USA","Indonesia","Pakistan"],
    values=[1411, 1393, 332, 273, 220],
    x_label="Population (M)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="bce-h-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Population by Country",
  variant: "horizontal",
  labels: ["China","India","USA","Indonesia","Pakistan"],
  values: [1411, 1393, 332, 273, 220],
  xLabel: "Population (M)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="bce-h-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Population by Country",
  variant: "horizontal",
  labels: ["China","India","USA","Indonesia","Pakistan"],
  values: [1411, 1393, 332, 273, 220],
  xLabel: "Population (M)",
  gridlines: true,
});
chart.show();</code></pre></div>
</div>

</div>

<!-- ─────────────── Grouped ─────────────── -->
<div class="sp-variant" id="bar-charts-en-grouped">

Multiple series shown side-by-side per category — direct comparison of values.

<div class="sp-tabs" id="bce-g">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bce-g','bce-g-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bce-g','bce-g-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bce-g','bce-g-ts',this)">TypeScript</button>
</div>
<div id="bce-g-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Q1 Revenue by Region",
    variant="grouped",
    labels=["Q1","Q2","Q3","Q4"],
    series=[[120, 145, 160, 180], [90, 110, 135, 150], [60, 75, 80, 95]],
    series_names=["EU","NA","APAC"],
    y_label="Revenue (M€)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="bce-g-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Q1 Revenue by Region",
  variant: "grouped",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[120,145,160,180],[90,110,135,150],[60,75,80,95]],
  seriesNames: ["EU","NA","APAC"],
  yLabel: "Revenue (M€)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="bce-g-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Q1 Revenue by Region",
  variant: "grouped",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[120,145,160,180],[90,110,135,150],[60,75,80,95]],
  seriesNames: ["EU","NA","APAC"],
  yLabel: "Revenue (M€)",
  gridlines: true,
});
chart.show();</code></pre></div>
</div>

</div>

<!-- ─────────────── Stacked ─────────────── -->
<div class="sp-variant" id="bar-charts-en-stacked">

Series stacked on top of each other — shows part-to-whole within each category.

<div class="sp-tabs" id="bce-s">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bce-s','bce-s-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bce-s','bce-s-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bce-s','bce-s-ts',this)">TypeScript</button>
</div>
<div id="bce-s-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Quarterly Revenue Mix",
    variant="stacked",
    labels=["Q1","Q2","Q3","Q4"],
    series=[[120,145,160,180],[90,110,135,150],[60,75,80,95]],
    series_names=["EU","NA","APAC"],
    y_label="Revenue (M€)",
)
chart.show()</code></pre></div>
<div id="bce-s-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Quarterly Revenue Mix",
  variant: "stacked",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[120,145,160,180],[90,110,135,150],[60,75,80,95]],
  seriesNames: ["EU","NA","APAC"],
  yLabel: "Revenue (M€)",
});
chart.show();</code></pre></div>
<div id="bce-s-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Quarterly Revenue Mix",
  variant: "stacked",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[120,145,160,180],[90,110,135,150],[60,75,80,95]],
  seriesNames: ["EU","NA","APAC"],
  yLabel: "Revenue (M€)",
});
chart.show();</code></pre></div>
</div>

</div>

<!-- ─────────────── Relative ─────────────── -->
<div class="sp-variant" id="bar-charts-en-relative">

Like stacked, but **negative values stack below the zero baseline** — perfect for net P&L, gains/losses.

<div class="sp-tabs" id="bce-r">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bce-r','bce-r-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bce-r','bce-r-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bce-r','bce-r-ts',this)">TypeScript</button>
</div>
<div id="bce-r-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Net Cash Flow by Quarter",
    variant="relative",
    labels=["Q1","Q2","Q3","Q4"],
    series=[[50, 80, -40, 120], [-30, 20, -50, 60], [40, -10, 30, -20]],
    series_names=["Operations","Financing","Investing"],
    y_label="Cash Flow (k€)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="bce-r-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Net Cash Flow by Quarter",
  variant: "relative",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[50,80,-40,120],[-30,20,-50,60],[40,-10,30,-20]],
  seriesNames: ["Operations","Financing","Investing"],
  yLabel: "Cash Flow (k€)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="bce-r-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Net Cash Flow by Quarter",
  variant: "relative",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[50,80,-40,120],[-30,20,-50,60],[40,-10,30,-20]],
  seriesNames: ["Operations","Financing","Investing"],
  yLabel: "Cash Flow (k€)",
  gridlines: true,
});
chart.show();</code></pre></div>
</div>

</div>

<!-- ─────────────── Grouped-Stacked ─────────────── -->
<div class="sp-variant" id="bar-charts-en-gstack">

Multiple stacked sub-bars per category, grouped via `offset_groups` (one group label per series).

<div class="sp-tabs" id="bce-gs">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bce-gs','bce-gs-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bce-gs','bce-gs-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bce-gs','bce-gs-ts',this)">TypeScript</button>
</div>
<div id="bce-gs-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Revenue by Channel × Region",
    variant="grouped_stacked",
    labels=["Q1","Q2","Q3","Q4"],
    series=[
        [50, 60, 70, 80],   # Online EU
        [30, 40, 45, 55],   # Retail EU
        [40, 50, 55, 65],   # Online NA
        [20, 30, 35, 40],   # Retail NA
    ],
    series_names=["Online·EU","Retail·EU","Online·NA","Retail·NA"],
    offset_groups=["EU","EU","NA","NA"],
    y_label="Revenue (M€)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="bce-gs-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Revenue by Channel × Region",
  variant: "grouped_stacked",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[50,60,70,80],[30,40,45,55],[40,50,55,65],[20,30,35,40]],
  seriesNames: ["Online·EU","Retail·EU","Online·NA","Retail·NA"],
  offsetGroups: ["EU","EU","NA","NA"],
  yLabel: "Revenue (M€)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="bce-gs-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Revenue by Channel × Region",
  variant: "grouped_stacked",
  labels: ["Q1","Q2","Q3","Q4"],
  series: [[50,60,70,80],[30,40,45,55],[40,50,55,65],[20,30,35,40]],
  seriesNames: ["Online·EU","Retail·EU","Online·NA","Retail·NA"],
  offsetGroups: ["EU","EU","NA","NA"],
  yLabel: "Revenue (M€)",
  gridlines: true,
});
chart.show();</code></pre></div>
</div>

</div>

<!-- ─────────────── Marimekko ─────────────── -->
<div class="sp-variant" id="bar-charts-en-marimekko">

Variable-width stacked bars (mosaic plot). Bar **width** encodes one variable (e.g. market size), bar **segments** encode share — two dimensions in one chart.

<div class="sp-tabs" id="bce-m">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bce-m','bce-m-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bce-m','bce-m-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bce-m','bce-m-ts',this)">TypeScript</button>
</div>
<div id="bce-m-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Market Share by Segment Size",
    variant="marimekko",
    labels=["Auto","Tech","Retail","Energy","Health"],
    widths=[40, 25, 15, 12, 8],   # market size
    series=[
        [45, 30, 20, 50, 60],     # Leader share
        [30, 35, 40, 25, 25],     # Challenger
        [25, 35, 40, 25, 15],     # Others
    ],
    series_names=["Leader","Challenger","Others"],
    show_text=True,
)
chart.show()</code></pre></div>
<div id="bce-m-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Market Share by Segment Size",
  variant: "marimekko",
  labels: ["Auto","Tech","Retail","Energy","Health"],
  widths: [40,25,15,12,8],
  series: [[45,30,20,50,60],[30,35,40,25,25],[25,35,40,25,15]],
  seriesNames: ["Leader","Challenger","Others"],
  showText: true,
});
chart.show();</code></pre></div>
<div id="bce-m-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Market Share by Segment Size",
  variant: "marimekko",
  labels: ["Auto","Tech","Retail","Energy","Health"],
  widths: [40,25,15,12,8],
  series: [[45,30,20,50,60],[30,35,40,25,25],[25,35,40,25,15]],
  seriesNames: ["Leader","Challenger","Others"],
  showText: true,
});
chart.show();</code></pre></div>
</div>

</div>

<!-- ─────────────── Pictogram ─────────────── -->
<div class="sp-variant" id="bar-charts-en-pictogram">

A "bar" rendered as repeated icons. Each icon represents `units_per_icon`. Icons wrap into columns of `max_icons_per_column` height — perfect for infographics.

<div class="sp-tabs" id="bce-p">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bce-p','bce-p-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bce-p','bce-p-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bce-p','bce-p-ts',this)">TypeScript</button>
</div>
<div id="bce-p-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
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
<div id="bce-p-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Coffees Per Day",
  variant: "pictogram",
  labels: ["Mon","Tue","Wed","Thu","Fri"],
  values: [3,5,4,7,6],
  iconSize: 22,
  unitsPerIcon: 1,
  maxIconsPerColumn: 4,
  unitDescription: "cup",
  colorHex: 0x6F4E37,
});
chart.show();</code></pre></div>
<div id="bce-p-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Coffees Per Day",
  variant: "pictogram",
  labels: ["Mon","Tue","Wed","Thu","Fri"],
  values: [3,5,4,7,6],
  iconSize: 22,
  unitsPerIcon: 1,
  maxIconsPerColumn: 4,
  unitDescription: "cup",
  colorHex: 0x6F4E37,
});
chart.show();</code></pre></div>
</div>

</div>

<!-- ─────────────── Multicategory ─────────────── -->
<div class="sp-variant" id="bar-charts-en-multi">

Two-level x axis. `super_categories` groups adjacent bars under a bracket label.

<div class="sp-tabs" id="bce-mc">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bce-mc','bce-mc-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bce-mc','bce-mc-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bce-mc','bce-mc-ts',this)">TypeScript</button>
</div>
<div id="bce-mc-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.bar(
    title="Sales by Region & Quarter",
    variant="multicategory",
    labels=["Q1","Q2","Q3","Q4","Q1","Q2","Q3","Q4"],
    super_categories=["EU","EU","EU","EU","NA","NA","NA","NA"],
    values=[120, 145, 160, 180, 90, 110, 135, 150],
    y_label="Revenue (M€)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="bce-mc-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.bar({
  title: "Sales by Region & Quarter",
  variant: "multicategory",
  labels: ["Q1","Q2","Q3","Q4","Q1","Q2","Q3","Q4"],
  superCategories: ["EU","EU","EU","EU","NA","NA","NA","NA"],
  values: [120,145,160,180,90,110,135,150],
  yLabel: "Revenue (M€)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="bce-mc-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Sales by Region & Quarter",
  variant: "multicategory",
  labels: ["Q1","Q2","Q3","Q4","Q1","Q2","Q3","Q4"],
  superCategories: ["EU","EU","EU","EU","NA","NA","NA","NA"],
  values: [120,145,160,180,90,110,135,150],
  yLabel: "Revenue (M€)",
  gridlines: true,
});
chart.show();</code></pre></div>
</div>

</div>

</div><!-- /bar-charts-en -->

</div><!-- /lang-en -->


<div class="lang-fr" style="display:none">

Point d'entrée unifié pour **toutes** les variantes de graphiques en barres. Une seule fonction, une seule config — choisissez la variante via le mot-clé `variant`.

## Signature

Identique à la version anglaise — voir ci-dessus en mode EN.

## Variantes disponibles

| Variante | Cas d'usage | Arguments requis |
|----------|-------------|------------------|
| `"basic"` | Série unique, barres verticales | `labels`, `values` |
| `"horizontal"` | Série unique, barres horizontales | `labels`, `values` |
| `"grouped"` | Plusieurs séries côte à côte | `labels`, `series`, `series_names` |
| `"stacked"` | Plusieurs séries empilées | `labels`, `series`, `series_names` |
| `"relative"` | Empilé avec valeurs négatives sous l'axe 0 | `labels`, `series`, `series_names` |
| `"grouped_stacked"` | Groupes de barres empilées par catégorie | `labels`, `series`, `series_names`, `offset_groups` |
| `"marimekko"` | Empilé à largeur variable (mosaïque) | `labels`, `series`, `series_names`, `widths` |
| `"pictogram"` | Barre faite d'icônes répétées | `labels`, `values`, `units_per_icon` |
| `"multicategory"` | Axe x hiérarchique à 2 niveaux | `labels`, `super_categories`, `values` |

**Conseil migration :** vos scripts existants `sp.bar_chart(...)` continuent de fonctionner. Migrez vers `sp.bar(variant="…")` pour bénéficier d'une sémantique uniforme sur toute la famille.

Pour les exemples de code, basculez en anglais via le sélecteur de langue — la syntaxe est identique.

</div><!-- /lang-fr -->
