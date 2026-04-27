# Bar Charts

<div class="lang-en">

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-variant-nav{display:flex;flex-wrap:wrap;border-bottom:2px solid #1e293b;margin:0 0 1.4em}
.sp-vbtn{padding:9px 15px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;margin-bottom:-2px;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-vbtn:hover{color:#cbd5e1}
.sp-vbtn.sp-vact{color:#818cf8;border-bottom-color:#818cf8}
.sp-variant{display:none}
.sp-variant.sp-von{display:block}
</style>
<script>
var barPreviewMap={'basic':'bar','horizontal':'hbar','grouped':'grouped-bar','stacked':'stacked-bar','relative':'relative-bar','gstack':'grouped-stacked-bar','marimekko':'marimekko-bar','pictogram':'pictogram-bar','multicategory':'multicategory-bar'};
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spVar(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-vbtn').forEach(function(b){b.classList.remove('sp-vact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-vact');var frame=document.getElementById(scope+'-preview');if(frame&&barPreviewMap[name]){frame.src='../../previews/'+barPreviewMap[name]+'.html';}}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.bar(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

Aliases: `sp.bar`, `sp.bars`, `sp.bar_unified`, `sp.bars_unified`, `sp.bar_family`, `sp.bar_chart`

<div id="bar-en">
<div class="sp-variant-nav">
<button class="sp-vbtn sp-vact" onclick="spVar('bar-en','basic',this)">Basic</button>
<button class="sp-vbtn" onclick="spVar('bar-en','horizontal',this)">Horizontal</button>
<button class="sp-vbtn" onclick="spVar('bar-en','grouped',this)">Grouped</button>
<button class="sp-vbtn" onclick="spVar('bar-en','stacked',this)">Stacked</button>
<button class="sp-vbtn" onclick="spVar('bar-en','relative',this)">Relative</button>
<button class="sp-vbtn" onclick="spVar('bar-en','gstack',this)">Grouped-Stacked</button>
<button class="sp-vbtn" onclick="spVar('bar-en','marimekko',this)">Marimekko</button>
<button class="sp-vbtn" onclick="spVar('bar-en','pictogram',this)">Pictogram</button>
<button class="sp-vbtn" onclick="spVar('bar-en','multicategory',this)">Multicategory</button>
</div>

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
</div>

</div><!-- /bar-en -->

<iframe id="bar-en-preview" loading="lazy" src="../../previews/bar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117"></iframe>

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
</div>

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

<div id="bar-fr">
<div class="sp-variant-nav">
<button class="sp-vbtn sp-vact" onclick="spVar('bar-fr','basic',this)">Basique</button>
<button class="sp-vbtn" onclick="spVar('bar-fr','horizontal',this)">Horizontal</button>
<button class="sp-vbtn" onclick="spVar('bar-fr','grouped',this)">Groupé</button>
<button class="sp-vbtn" onclick="spVar('bar-fr','stacked',this)">Empilé</button>
<button class="sp-vbtn" onclick="spVar('bar-fr','relative',this)">Relatif</button>
<button class="sp-vbtn" onclick="spVar('bar-fr','gstack',this)">Groupé-Empilé</button>
<button class="sp-vbtn" onclick="spVar('bar-fr','marimekko',this)">Marimekko</button>
<button class="sp-vbtn" onclick="spVar('bar-fr','pictogram',this)">Pictogramme</button>
<button class="sp-vbtn" onclick="spVar('bar-fr','multicategory',this)">Multi-catégories</button>
</div>
<div class="sp-variant sp-von" id="bar-fr-basic"><p>Barres verticales — une valeur par catégorie.</p>
<pre><code class="language-python">sp.bar(title, labels, values, *, variant="basic", color_hex=0,
       show_text=False, corner_radius=0, bar_gap=0.2,
       width=900, height=480, x_label="", y_label="",
       gridlines=False, sort_order="none") -> Chart</code></pre>
</div>
<div class="sp-variant" id="bar-fr-horizontal"><p>Barres horizontales — idéal pour les longs noms de catégories.</p>
<pre><code class="language-python">sp.bar(title, labels, values, *, variant="horizontal",
       color_hex=0, show_text=False, corner_radius=0, ...) -> Chart</code></pre></div>
<div class="sp-variant" id="bar-fr-grouped"><p>Plusieurs séries côte à côte par catégorie.</p>
<pre><code class="language-python">sp.bar(title, labels, *, variant="grouped",
       series, series_names=None, ...) -> Chart</code></pre></div>
<div class="sp-variant" id="bar-fr-stacked"><p>Séries empilées — montre la part de chaque série dans le total.</p>
<pre><code class="language-python">sp.bar(title, labels, *, variant="stacked",
       series, series_names=None, ...) -> Chart</code></pre></div>
<div class="sp-variant" id="bar-fr-relative"><p>Empilé avec valeurs négatives sous l'axe zéro — flux de trésorerie, P&amp;L.</p>
<pre><code class="language-python">sp.bar(title, labels, *, variant="relative",
       series, series_names=None, ...) -> Chart</code></pre></div>
<div class="sp-variant" id="bar-fr-gstack"><p>Groupes de barres empilées. <code>offset_groups</code> assigne une pile à chaque série.</p>
<pre><code class="language-python">sp.bar(title, labels, *, variant="grouped_stacked",
       series, series_names=None, offset_groups, ...) -> Chart</code></pre></div>
<div class="sp-variant" id="bar-fr-marimekko"><p>Barres empilées à largeur variable. <code>widths</code> encode la taille de chaque catégorie.</p>
<pre><code class="language-python">sp.bar(title, labels, *, variant="marimekko",
       series, series_names=None, widths, ...) -> Chart</code></pre></div>
<div class="sp-variant" id="bar-fr-pictogram"><p>Barre composée d'icônes répétées. Chaque icône représente <code>units_per_icon</code> unités.</p>
<pre><code class="language-python">sp.bar(title, labels, values, *, variant="pictogram",
       icon_size=24, max_icons_per_column=10,
       units_per_icon=1.0, unit_description="", ...) -> Chart</code></pre></div>
<div class="sp-variant" id="bar-fr-multicategory"><p>Axe x à deux niveaux. <code>super_categories</code> regroupe les barres sous un label chapeau.</p>
<pre><code class="language-python">sp.bar(title, labels, values, *, variant="multicategory",
       super_categories, ...) -> Chart</code></pre></div>
</div><!-- /bar-fr -->

<iframe id="bar-fr-preview" loading="lazy" src="../../previews/bar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117"></iframe>

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
</div>

## Voir aussi

- [Lollipop](lollipop.md) — `sp.lollipop()`
- [Cascade / Waterfall](waterfall.md) — `sp.waterfall()`
- [Bullet](bullet.md) — `sp.bullet()`

</div><!-- /lang-fr -->
