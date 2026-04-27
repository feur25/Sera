# Line Charts

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
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spVar(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-vbtn').forEach(function(b){b.classList.remove('sp-vact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-vact');}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

<div id="line-en">
<div class="sp-variant-nav">
<button class="sp-vbtn sp-vact" onclick="spVar('line-en','basic',this)">Basic</button>
<button class="sp-vbtn" onclick="spVar('line-en','multi',this)">Multi</button>
<button class="sp-vbtn" onclick="spVar('line-en','stepped',this)">Stepped</button>
<button class="sp-vbtn" onclick="spVar('line-en','spline',this)">Spline</button>
<button class="sp-vbtn" onclick="spVar('line-en','filled',this)">Filled</button>
<button class="sp-vbtn" onclick="spVar('line-en','sparkline',this)">Sparkline</button>
<button class="sp-vbtn" onclick="spVar('line-en','dashed',this)">Dashed</button>
<button class="sp-vbtn" onclick="spVar('line-en','connected',this)">ConnectedScatter</button>
<button class="sp-vbtn" onclick="spVar('line-en','gapped',this)">Gapped</button>
</div>

<!-- â”€â”€ Basic â”€â”€ -->
<div class="sp-variant sp-von" id="line-en-basic">

Single series connecting ordered data points.

```python
sp.line(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    variant: str = "basic",
    color_hex: int = 0x6366F1,
    show_points: bool = True,
    gridlines: bool = False,
    sort_order: str = "none",
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    no_y_axis: bool = False,
    palette: list[int] | None = None,
    background: str | None = None,
) -> Chart
```

<div class="sp-tabs" id="l-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-basic','l-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-basic','l-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-basic','l-basic-ts',this)">TypeScript</button>
</div>
<div id="l-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title="Temperature over 12 months",
    variant="basic",
    labels=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],
    values=[3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2],
    gridlines=True,
    y_label="Â°C",
)
chart.show()</code></pre></div>
<div id="l-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Temperature over 12 months", variant: "basic",
  labels: ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],
  values: [3.2,4.1,8.5,13.2,17.8,22.1,24.6,24.0,19.3,13.7,7.8,4.2],
  gridlines: true, yLabel: "Â°C",
});
chart.show();</code></pre></div>
<div id="l-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Temperature over 12 months", variant: "basic",
  labels: ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],
  values: [3.2,4.1,8.5,13.2,17.8,22.1,24.6,24.0,19.3,13.7,7.8,4.2],
  gridlines: true, yLabel: "Â°C",
});
chart.show();</code></pre></div>
</div>
</div>

<!-- â”€â”€ Multi â”€â”€ -->
<div class="sp-variant" id="line-en-multi">

Multiple series on shared axes. Alias: `"multiline"`, `"multi_line"`.

```python
sp.line(
    title: str,
    labels: list[str],
    *,
    variant: str = "multi",
    series: list[list[float]],
    series_names: list[str] | None = None,
    x_labels: list[str] | None = None,
    show_points: bool = True,
    gridlines: bool = False,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    legend_position: str = "right",
    palette: list[int] | None = None,
    background: str | None = None,
) -> Chart
```

<div class="sp-tabs" id="l-multi">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-multi','l-multi-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-multi','l-multi-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-multi','l-multi-ts',this)">TypeScript</button>
</div>
<div id="l-multi-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title="CPU vs Memory usage",
    variant="multi",
    labels=["00:00","04:00","08:00","12:00","16:00","20:00"],
    series=[[20,22,55,80,72,35],[40,38,45,60,55,42]],
    series_names=["CPU %","RAM %"],
    gridlines=True,
    y_label="%",
)
chart.show()</code></pre></div>
<div id="l-multi-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "CPU vs Memory usage", variant: "multi",
  labels: ["00:00","04:00","08:00","12:00","16:00","20:00"],
  series: [[20,22,55,80,72,35],[40,38,45,60,55,42]],
  seriesNames: ["CPU %","RAM %"],
  gridlines: true, yLabel: "%",
});
chart.show();</code></pre></div>
<div id="l-multi-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "CPU vs Memory usage", variant: "multi",
  labels: ["00:00","04:00","08:00","12:00","16:00","20:00"],
  series: [[20,22,55,80,72,35],[40,38,45,60,55,42]],
  seriesNames: ["CPU %","RAM %"],
  gridlines: true, yLabel: "%",
});
chart.show();</code></pre></div>
</div>
</div>

<!-- â”€â”€ Stepped â”€â”€ -->
<div class="sp-variant" id="line-en-stepped">

Step line â€” value holds constant until the next point. `step_shape` controls where the step occurs. Aliases: `"hv"`, `"vh"`, `"hvh"`, `"vhv"`, `"step"`.

```python
sp.line(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    variant: str = "stepped",
    step_shape: str = "hv",
    color_hex: int = 0x6366F1,
    show_points: bool = False,
    gridlines: bool = False,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    palette: list[int] | None = None,
) -> Chart
```

`step_shape` values: `"hv"` (horizontal then vertical, default), `"vh"` (vertical then horizontal), `"hvh"`, `"vhv"`.

<div class="sp-tabs" id="l-stepped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-stepped','l-stepped-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-stepped','l-stepped-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-stepped','l-stepped-ts',this)">TypeScript</button>
</div>
<div id="l-stepped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title="Price Tier Changes",
    variant="stepped",
    step_shape="hv",
    labels=["Jan","Feb","Mar","Apr","May","Jun"],
    values=[10, 10, 15, 15, 20, 20],
    y_label="Price (â‚¬)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="l-stepped-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Price Tier Changes", variant: "stepped",
  stepShape: "hv",
  labels: ["Jan","Feb","Mar","Apr","May","Jun"],
  values: [10,10,15,15,20,20],
  yLabel: "Price (â‚¬)", gridlines: true,
});
chart.show();</code></pre></div>
<div id="l-stepped-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Price Tier Changes", variant: "stepped",
  stepShape: "hv",
  labels: ["Jan","Feb","Mar","Apr","May","Jun"],
  values: [10,10,15,15,20,20],
  yLabel: "Price (â‚¬)", gridlines: true,
});
chart.show();</code></pre></div>
</div>
</div>

<!-- â”€â”€ Spline â”€â”€ -->
<div class="sp-variant" id="line-en-spline">

Smooth Catmull-Rom curve through data points. `spline_tension` controls curvature (0.0 = straight, 1.0 = maximum curve). Alias: `"smooth"`.

```python
sp.line(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    variant: str = "spline",
    spline_tension: float = 0.5,
    color_hex: int = 0x6366F1,
    show_points: bool = True,
    gridlines: bool = False,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    palette: list[int] | None = None,
) -> Chart
```

<div class="sp-tabs" id="l-spline">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-spline','l-spline-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-spline','l-spline-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-spline','l-spline-ts',this)">TypeScript</button>
</div>
<div id="l-spline-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title="Heart Rate During Workout",
    variant="spline",
    spline_tension=0.5,
    labels=["0:00","5:00","10:00","15:00","20:00","25:00","30:00"],
    values=[68, 95, 142, 158, 161, 145, 110],
    y_label="BPM",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="l-spline-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Heart Rate During Workout", variant: "spline",
  splineTension: 0.5,
  labels: ["0:00","5:00","10:00","15:00","20:00","25:00","30:00"],
  values: [68,95,142,158,161,145,110],
  yLabel: "BPM", gridlines: true,
});
chart.show();</code></pre></div>
<div id="l-spline-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Heart Rate During Workout", variant: "spline",
  splineTension: 0.5,
  labels: ["0:00","5:00","10:00","15:00","20:00","25:00","30:00"],
  values: [68,95,142,158,161,145,110],
  yLabel: "BPM", gridlines: true,
});
chart.show();</code></pre></div>
</div>
</div>

<!-- â”€â”€ Filled â”€â”€ -->
<div class="sp-variant" id="line-en-filled">

Area fill under the line. `fill_opacity` controls transparency; `stack_fill=True` stacks multiple series. Alias: `"area"`.

```python
sp.line(
    title: str,
    labels: list[str],
    *,
    variant: str = "filled",
    values: list[float] | None = None,
    series: list[list[float]] | None = None,
    series_names: list[str] | None = None,
    fill_opacity: float = 0.25,
    stack_fill: bool = False,
    color_hex: int = 0x6366F1,
    show_points: bool = False,
    gridlines: bool = False,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    palette: list[int] | None = None,
) -> Chart
```

<div class="sp-tabs" id="l-filled">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-filled','l-filled-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-filled','l-filled-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-filled','l-filled-ts',this)">TypeScript</button>
</div>
<div id="l-filled-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title="Monthly Active Users",
    variant="filled",
    labels=["Jan","Feb","Mar","Apr","May","Jun"],
    values=[1200, 1850, 2100, 1750, 2400, 2800],
    fill_opacity=0.3,
    y_label="Users",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="l-filled-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Monthly Active Users", variant: "filled",
  labels: ["Jan","Feb","Mar","Apr","May","Jun"],
  values: [1200,1850,2100,1750,2400,2800],
  fillOpacity: 0.3, yLabel: "Users", gridlines: true,
});
chart.show();</code></pre></div>
<div id="l-filled-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Monthly Active Users", variant: "filled",
  labels: ["Jan","Feb","Mar","Apr","May","Jun"],
  values: [1200,1850,2100,1750,2400,2800],
  fillOpacity: 0.3, yLabel: "Users", gridlines: true,
});
chart.show();</code></pre></div>
</div>
</div>

<!-- â”€â”€ Sparkline â”€â”€ -->
<div class="sp-variant" id="line-en-sparkline">

Small-multiple grid â€” one mini chart per series. `spark_cols` controls columns, `spark_cell_w`/`spark_cell_h` control cell dimensions. Alias: `"spark"`.

```python
sp.line(
    title: str,
    *,
    variant: str = "sparkline",
    series: list[list[float]],
    series_names: list[str] | None = None,
    labels: list[str] | None = None,
    spark_cols: int = 3,
    spark_cell_w: int = 220,
    spark_cell_h: int = 80,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    background: str | None = None,
) -> Chart
```

<div class="sp-tabs" id="l-spark">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-spark','l-spark-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-spark','l-spark-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-spark','l-spark-ts',this)">TypeScript</button>
</div>
<div id="l-spark-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title="KPI Sparklines",
    variant="sparkline",
    series=[
        [10,12,15,11,18,22],
        [80,75,70,85,90,88],
        [5,8,6,10,9,12],
    ],
    series_names=["Revenue","Satisfaction","Churn"],
    spark_cols=3,
    spark_cell_w=200,
    spark_cell_h=90,
)
chart.show()</code></pre></div>
<div id="l-spark-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "KPI Sparklines", variant: "sparkline",
  series: [[10,12,15,11,18,22],[80,75,70,85,90,88],[5,8,6,10,9,12]],
  seriesNames: ["Revenue","Satisfaction","Churn"],
  sparkCols: 3, sparkCellW: 200, sparkCellH: 90,
});
chart.show();</code></pre></div>
<div id="l-spark-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "KPI Sparklines", variant: "sparkline",
  series: [[10,12,15,11,18,22],[80,75,70,85,90,88],[5,8,6,10,9,12]],
  seriesNames: ["Revenue","Satisfaction","Churn"],
  sparkCols: 3, sparkCellW: 200, sparkCellH: 90,
});
chart.show();</code></pre></div>
</div>
</div>

<!-- â”€â”€ Dashed â”€â”€ -->
<div class="sp-variant" id="line-en-dashed">

Dashed line with a `stroke-dasharray` pattern. `dash_pattern="auto"` cycles through preset patterns per series. Alias: `"dash"`.

```python
sp.line(
    title: str,
    labels: list[str],
    *,
    variant: str = "dashed",
    values: list[float] | None = None,
    series: list[list[float]] | None = None,
    series_names: list[str] | None = None,
    dash_pattern: str = "auto",
    stroke_width: float = 2.0,
    color_hex: int = 0x6366F1,
    show_points: bool = False,
    gridlines: bool = False,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    palette: list[int] | None = None,
) -> Chart
```

`dash_pattern` examples: `"auto"`, `"4 4"`, `"8 4"`, `"8 4 2 4"` (SVG dasharray syntax).

<div class="sp-tabs" id="l-dashed">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-dashed','l-dashed-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-dashed','l-dashed-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-dashed','l-dashed-ts',this)">TypeScript</button>
</div>
<div id="l-dashed-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title="Forecast vs Actual",
    variant="dashed",
    labels=["Jan","Feb","Mar","Apr","May","Jun"],
    series=[
        [100, 110, 120, 115, 130, 140],
        [100, 105, 118, 112, 125, 138],
    ],
    series_names=["Forecast","Actual"],
    dash_pattern="auto",
    gridlines=True,
    y_label="Revenue (kâ‚¬)",
)
chart.show()</code></pre></div>
<div id="l-dashed-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Forecast vs Actual", variant: "dashed",
  labels: ["Jan","Feb","Mar","Apr","May","Jun"],
  series: [[100,110,120,115,130,140],[100,105,118,112,125,138]],
  seriesNames: ["Forecast","Actual"],
  dashPattern: "auto", gridlines: true, yLabel: "Revenue (kâ‚¬)",
});
chart.show();</code></pre></div>
<div id="l-dashed-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Forecast vs Actual", variant: "dashed",
  labels: ["Jan","Feb","Mar","Apr","May","Jun"],
  series: [[100,110,120,115,130,140],[100,105,118,112,125,138]],
  seriesNames: ["Forecast","Actual"],
  dashPattern: "auto", gridlines: true, yLabel: "Revenue (kâ‚¬)",
});
chart.show();</code></pre></div>
</div>
</div>

<!-- â”€â”€ ConnectedScatter â”€â”€ -->
<div class="sp-variant" id="line-en-connected">

Line + large filled markers â€” emphasises individual data points while preserving trend. `marker_size` defaults to 5 (minimum). Aliases: `"connected_scatter"`, `"markers"`, `"lines+markers"`.

```python
sp.line(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    variant: str = "connected_scatter",
    marker_size: int = 5,
    stroke_width: float = 2.0,
    color_hex: int = 0x6366F1,
    gridlines: bool = False,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    palette: list[int] | None = None,
) -> Chart
```

<div class="sp-tabs" id="l-cs">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-cs','l-cs-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-cs','l-cs-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-cs','l-cs-ts',this)">TypeScript</button>
</div>
<div id="l-cs-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.line(
    title="Weekly Sales",
    variant="connected_scatter",
    labels=["Mon","Tue","Wed","Thu","Fri","Sat","Sun"],
    values=[42, 55, 60, 48, 70, 95, 80],
    marker_size=8,
    y_label="Orders",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="l-cs-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Weekly Sales", variant: "connected_scatter",
  labels: ["Mon","Tue","Wed","Thu","Fri","Sat","Sun"],
  values: [42,55,60,48,70,95,80],
  markerSize: 8, yLabel: "Orders", gridlines: true,
});
chart.show();</code></pre></div>
<div id="l-cs-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Weekly Sales", variant: "connected_scatter",
  labels: ["Mon","Tue","Wed","Thu","Fri","Sat","Sun"],
  values: [42,55,60,48,70,95,80],
  markerSize: 8, yLabel: "Orders", gridlines: true,
});
chart.show();</code></pre></div>
</div>
</div>

<!-- â”€â”€ Gapped â”€â”€ -->
<div class="sp-variant" id="line-en-gapped">

Breaks the line at `null`/`NaN` values or at large jumps. `gap_threshold=NaN` breaks only at missing values; a numeric threshold breaks when consecutive delta exceeds it. Aliases: `"gaps"`, `"missing"`.

```python
sp.line(
    title: str,
    labels: list[str],
    values: list[float | None],
    *,
    variant: str = "gapped",
    gap_threshold: float = float("nan"),
    color_hex: int = 0x6366F1,
    show_points: bool = True,
    gridlines: bool = False,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    palette: list[int] | None = None,
) -> Chart
```

<div class="sp-tabs" id="l-gapped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('l-gapped','l-gapped-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('l-gapped','l-gapped-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('l-gapped','l-gapped-ts',this)">TypeScript</button>
</div>
<div id="l-gapped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import math
chart = sp.line(
    title="Sensor Readings (with outages)",
    variant="gapped",
    labels=["00:00","01:00","02:00","03:00","04:00","05:00","06:00"],
    values=[22.1, 22.4, math.nan, math.nan, 23.0, 22.8, 23.2],
    y_label="Â°C",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="l-gapped-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.line({
  title: "Sensor Readings (with outages)", variant: "gapped",
  labels: ["00:00","01:00","02:00","03:00","04:00","05:00","06:00"],
  values: [22.1, 22.4, null, null, 23.0, 22.8, 23.2],
  yLabel: "Â°C", gridlines: true,
});
chart.show();</code></pre></div>
<div id="l-gapped-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.line({
  title: "Sensor Readings (with outages)", variant: "gapped",
  labels: ["00:00","01:00","02:00","03:00","04:00","05:00","06:00"],
  values: [22.1, 22.4, null, null, 23.0, 22.8, 23.2],
  yLabel: "Â°C", gridlines: true,
});
chart.show();</code></pre></div>
</div>
</div>

</div><!-- /line-en -->

Aliases: `sp.line_chart`, `sp.line_unified`, `sp.lines_unified`, `sp.line_family`

---

## Description

`sp.line()` is the unified entry point for the entire line-chart family. The `variant` keyword selects the rendering strategy.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Single series, simple line | `labels`, `values` |
| `"multi"` / `"multiline"` | Multiple series, shared axes | `series`, `series_names` |
| `"stepped"` / `"hv"` / `"vh"` | Step function | `step_shape` |
| `"spline"` / `"smooth"` | Catmull-Rom smooth curve | `spline_tension` |
| `"filled"` / `"area"` | Filled area under line | `fill_opacity`, `stack_fill` |
| `"sparkline"` / `"spark"` | Small-multiple grid | `spark_cols`, `spark_cell_w`, `spark_cell_h` |
| `"dashed"` / `"dash"` | Dashed stroke patterns | `dash_pattern`, `stroke_width` |
| `"connected_scatter"` / `"markers"` | Line + large markers | `marker_size`, `stroke_width` |
| `"gapped"` / `"gaps"` | Break at missing values | `gap_threshold` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | required | all | Chart title |
| `labels` | `list[str]` | `None` | all except sparkline | Category or time labels |
| `values` | `list[float\|None]` | `None` | basic, stepped, spline, filled, connected_scatter, gapped | Single-series values |
| `variant` | `str` | `"basic"` | â€” | Rendering variant |
| `series` | `list[list[float]]` | `None` | multi, filled, dashed, sparkline | Multi-series data |
| `series_names` | `list[str]` | `None` | multi, dashed, sparkline | Legend label per series |
| `x_labels` | `list[str]` | `None` | multi | Override x axis labels per series |
| `color_hex` | `int` | `0x6366F1` | single-series | Line and point color as hex int |
| `palette` | `list[int]` | `None` | all | Custom color list |
| `show_points` | `bool` | `True` | basic, multi, spline, filled, gapped | Draw circle markers at data points |
| `gridlines` | `bool` | `False` | all | Show horizontal gridlines |
| `sort_order` | `str` | `"none"` | basic | `"asc"`, `"desc"`, or `"none"` |
| `step_shape` | `str` | `"hv"` | stepped | `"hv"`, `"vh"`, `"hvh"`, `"vhv"` |
| `spline_tension` | `float` | `0.5` | spline | Curvature strength (0.0â€“1.0) |
| `fill_opacity` | `float` | `0.25` | filled | Fill area alpha (0.0â€“1.0) |
| `fill_opacity_f` | `float` | `0.25` | filled | Alternative alias for `fill_opacity` |
| `stack_fill` | `bool` | `False` | filled | Stack multiple filled series |
| `dash_pattern` | `str` | `"auto"` | dashed | SVG dasharray string or `"auto"` |
| `stroke_width` | `float` | `2.0` | dashed, connected_scatter | Line stroke width in px |
| `marker_size` | `int` | `5` | connected_scatter | Marker circle radius (min 5) |
| `gap_threshold` | `float` | `NaN` | gapped | Delta threshold to break line; `NaN` = break only at null |
| `spark_cols` | `int` | `3` | sparkline | Number of columns in the grid |
| `spark_cell_w` | `int` | `220` | sparkline | Width of each sparkline cell in px |
| `spark_cell_h` | `int` | `80` | sparkline | Height of each sparkline cell in px |
| `width` | `int` | `900` | all | Canvas width in pixels |
| `height` | `int` | `480` | all | Canvas height in pixels |
| `x_label` | `str` | `""` | all | X-axis label |
| `y_label` | `str` | `""` | all | Y-axis label |
| `no_y_axis` | `bool` | `False` | all | Hide Y axis |
| `legend_position` | `str` | `"right"` | multi, dashed | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | all | Background CSS color; `None` = transparent |

---

## Returns

`Chart` â€” object with `.html` property and `.show()` method.

---

<iframe src="../../previews/line.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [multiline.md](multiline.md) — `sp.build_multiline_chart()` (legacy)
- [area.md](area.md) — `sp.area()`
- [scatter.md](scatter.md) — `sp.scatter()`

</div><!-- /lang-en -->


<div class="lang-fr" style="display:none">

<div id="line-fr">
<div class="sp-variant-nav">
<button class="sp-vbtn sp-vact" onclick="spVar('line-fr','basic',this)">Basique</button>
<button class="sp-vbtn" onclick="spVar('line-fr','multi',this)">Multi</button>
<button class="sp-vbtn" onclick="spVar('line-fr','stepped',this)">Escalier</button>
<button class="sp-vbtn" onclick="spVar('line-fr','spline',this)">Spline</button>
<button class="sp-vbtn" onclick="spVar('line-fr','filled',this)">Rempli</button>
<button class="sp-vbtn" onclick="spVar('line-fr','sparkline',this)">Sparkline</button>
<button class="sp-vbtn" onclick="spVar('line-fr','dashed',this)">Tirets</button>
<button class="sp-vbtn" onclick="spVar('line-fr','connected',this)">ConnectedScatter</button>
<button class="sp-vbtn" onclick="spVar('line-fr','gapped',this)">Lacunes</button>
</div>
<div class="sp-variant sp-von" id="line-fr-basic"><p>Série unique reliant des points ordonnés.</p>
<pre><code class="language-python">sp.line(title, labels, values, *, variant="basic",
        color_hex=0x6366F1, show_points=True, gridlines=False,
        sort_order="none", width=900, height=480,
        x_label="", y_label="") -> Chart</code></pre>
<p>Pour les exemples de code, voir la version EN.</p></div>
<div class="sp-variant" id="line-fr-multi"><p>Plusieurs séries sur les mêmes axes.</p>
<pre><code class="language-python">sp.line(title, labels, *, variant="multi",
        series, series_names=None, show_points=True, ...) -> Chart</code></pre></div>
<div class="sp-variant" id="line-fr-stepped"><p>Ligne en escalier — la valeur reste constante jusqu'au point suivant.</p>
<pre><code class="language-python">sp.line(title, labels, values, *, variant="stepped",
        step_shape="hv", ...) -> Chart</code></pre>
<p><code>step_shape</code> : <code>"hv"</code> (défaut), <code>"vh"</code>, <code>"hvh"</code>, <code>"vhv"</code>.</p></div>
<div class="sp-variant" id="line-fr-spline"><p>Courbe Catmull-Rom lissée. <code>spline_tension</code> contrôle la courbure (0.0–1.0).</p>
<pre><code class="language-python">sp.line(title, labels, values, *, variant="spline",
        spline_tension=0.5, ...) -> Chart</code></pre></div>
<div class="sp-variant" id="line-fr-filled"><p>Zone remplie sous la courbe. <code>stack_fill=True</code> empile plusieurs séries.</p>
<pre><code class="language-python">sp.line(title, labels, *, variant="filled",
        values=None, series=None, fill_opacity=0.25,
        stack_fill=False, ...) -> Chart</code></pre></div>
<div class="sp-variant" id="line-fr-sparkline"><p>Grille de petits graphiques — un par série.</p>
<pre><code class="language-python">sp.line(title, *, variant="sparkline",
        series, series_names=None,
        spark_cols=3, spark_cell_w=220, spark_cell_h=80, ...) -> Chart</code></pre></div>
<div class="sp-variant" id="line-fr-dashed"><p>Ligne avec tirets SVG. <code>dash_pattern="auto"</code> cycle à travers les motifs.</p>
<pre><code class="language-python">sp.line(title, labels, *, variant="dashed",
        values=None, series=None, dash_pattern="auto",
        stroke_width=2.0, ...) -> Chart</code></pre></div>
<div class="sp-variant" id="line-fr-connected"><p>Ligne + grands marqueurs — accentue les points individuels.</p>
<pre><code class="language-python">sp.line(title, labels, values, *, variant="connected_scatter",
        marker_size=5, stroke_width=2.0, ...) -> Chart</code></pre></div>
<div class="sp-variant" id="line-fr-gapped"><p>Rupture de ligne aux valeurs manquantes ou aux sauts importants.</p>
<pre><code class="language-python">sp.line(title, labels, values, *, variant="gapped",
        gap_threshold=float("nan"), ...) -> Chart</code></pre></div>
</div><!-- /line-fr -->

Alias : `sp.line_chart`, `sp.line_unified`, `sp.lines_unified`, `sp.line_family`

---

## Description

`sp.line()` est le point d'entrée unique pour toute la famille de graphiques en ligne.

| Variante | Cas d'usage | Arguments clés |
|----------|-------------|----------------|
| `"basic"` | Série unique, ligne simple | `labels`, `values` |
| `"multi"` / `"multiline"` | Plusieurs séries, axes partagés | `series`, `series_names` |
| `"stepped"` / `"hv"` | Fonction en escalier | `step_shape` |
| `"spline"` / `"smooth"` | Courbe lissée | `spline_tension` |
| `"filled"` / `"area"` | Zone remplie sous la ligne | `fill_opacity`, `stack_fill` |
| `"sparkline"` / `"spark"` | Grille de mini-graphiques | `spark_cols`, `spark_cell_w`, `spark_cell_h` |
| `"dashed"` / `"dash"` | Motifs de tirets | `dash_pattern`, `stroke_width` |
| `"connected_scatter"` / `"markers"` | Ligne + grands marqueurs | `marker_size`, `stroke_width` |
| `"gapped"` / `"gaps"` | Rupture aux valeurs manquantes | `gap_threshold` |

---

## Paramètres

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `labels` | `list[str]` | `None` | toutes sauf sparkline | Labels de catégorie ou de temps |
| `values` | `list[float\|None]` | `None` | basic, stepped, spline, filled, connected_scatter, gapped | Valeurs série unique |
| `variant` | `str` | `"basic"` | — | Variante de rendu |
| `series` | `list[list[float]]` | `None` | multi, filled, dashed, sparkline | Données multi-séries |
| `series_names` | `list[str]` | `None` | multi, dashed, sparkline | Nom de chaque série |
| `color_hex` | `int` | `0x6366F1` | série unique | Couleur ligne et points (hex int) |
| `palette` | `list[int]` | `None` | toutes | Palette personnalisée |
| `show_points` | `bool` | `True` | basic, multi, spline, filled, gapped | Afficher les marqueurs |
| `gridlines` | `bool` | `False` | toutes | Lignes de grille horizontales |
| `sort_order` | `str` | `"none"` | basic | `"asc"`, `"desc"`, `"none"` |
| `step_shape` | `str` | `"hv"` | stepped | `"hv"`, `"vh"`, `"hvh"`, `"vhv"` |
| `spline_tension` | `float` | `0.5` | spline | Courbure (0.0–1.0) |
| `fill_opacity` | `float` | `0.25` | filled | Opacité du remplissage (0.0–1.0) |
| `stack_fill` | `bool` | `False` | filled | Empiler plusieurs séries remplies |
| `dash_pattern` | `str` | `"auto"` | dashed | Motif SVG dasharray |
| `stroke_width` | `float` | `2.0` | dashed, connected_scatter | Épaisseur du trait |
| `marker_size` | `int` | `5` | connected_scatter | Rayon des marqueurs (min 5) |
| `gap_threshold` | `float` | `NaN` | gapped | Seuil delta pour rupture ; `NaN` = rupture uniquement sur null |
| `spark_cols` | `int` | `3` | sparkline | Colonnes dans la grille |
| `spark_cell_w` | `int` | `220` | sparkline | Largeur d'une cellule en px |
| `spark_cell_h` | `int` | `80` | sparkline | Hauteur d'une cellule en px |
| `width` | `int` | `900` | toutes | Largeur du canevas en px |
| `height` | `int` | `480` | toutes | Hauteur du canevas en px |
| `x_label` | `str` | `""` | toutes | Label axe X |
| `y_label` | `str` | `""` | toutes | Label axe Y |
| `no_y_axis` | `bool` | `False` | toutes | Masquer l'axe Y |
| `legend_position` | `str` | `"right"` | multi, dashed | Position de la légende |
| `background` | `str` | `None` | toutes | Couleur de fond CSS |

---

## Retourne

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

## Voir aussi

- [multiline.md](multiline.md) — `sp.build_multiline_chart()` (legacy)
- [area.md](area.md) — `sp.area()`
- [scatter.md](scatter.md) — `sp.scatter()`

</div><!-- /lang-fr -->
