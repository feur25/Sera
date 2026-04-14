# Multi-Line Chart

## Signature

```python
sp.build_multiline_chart(
    title: str,
    x_labels: list[str],
    series_values: list[list[float]],
    *,
    show_points: bool = True,
    series_names: list[str] | None = None,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
    legend_position: str = "top",
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Multiple line series plotted on a shared axis.
Unlike `build_line_chart`, this accepts several series in one call.

Each inner list in `series_values` must have the same length as `x_labels`.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x_labels` | `list[str]` | required | Shared X-axis tick labels |
| `series_values` | `list[list[float]]` | required | One inner list per series |
| `show_points` | `bool` | `True` | Show markers at data points |
| `series_names` | `list[str] \| None` | `None` | Legend names for each series |
| `palette` | `list[int] \| None` | `None` | Custom hex color per series |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Horizontal gridlines |
| `legend_position` | `str` | `"top"` | `"top"`, `"bottom"`, `"right"` |
| `hover_json` | `str \| None` | `None` | Custom hover tooltip JSON |

---

## Returns

`Chart`

---

## Examples

### Monthly revenue by product





<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155}
.sp-tb{padding:9px 22px;border:none;background:none;color:#64748b;cursor:pointer;font-size:13px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){hljs.highlightElement(c)})}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc code').forEach(function(c){hljs.highlightElement(c)})});
</script>
<div class="sp-tabs" id="multiline">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('multiline','multiline-py',this)">Python</button><button class="sp-tb" onclick="spTab('multiline','multiline-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('multiline','multiline-ts',this)">TypeScript</button></div>
<div id="multiline-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

months = ["Jan","Feb","Mar","Apr","May","Jun"]

chart = sp.build_multiline_chart(
    "Monthly Revenue by Product",
    x_labels=months,
    series_values=[
        [12200, 13400, 15100, 14800, 16200, 17500],
        [8100,  9200,  9800,  10200, 11000, 12400],
        [3200,  3600,  4100,  4500,  4800,  5200],
    ],
    series_names=["Product A", "Product B", "Product C"],
    show_points=True,
    y_label="Revenue ($)",
)</code></pre></div>
<div id="multiline-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const months = ["Jan","Feb","Mar","Apr","May","Jun"]

const chart = sp.buildMultilineChart("Monthly Revenue by Product",
months,
{
    series_values: [[12200, 13400, 15100, 14800, 16200, 17500], [8100, 9200, 9800, 10200, 11000, 12400], [3200, 3600, 4100, 4500, 4800, 5200]],
    series_names: ["Product A", "Product B", "Product C"],
    show_points: true,
    y_label: "Revenue ($)"
})</code></pre></div>
<div id="multiline-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const months: string[] = ["Jan","Feb","Mar","Apr","May","Jun"]

const chart = sp.buildMultilineChart("Monthly Revenue by Product",
months,
{
    series_values: [[12200, 13400, 15100, 14800, 16200, 17500], [8100, 9200, 9800, 10200, 11000, 12400], [3200, 3600, 4100, 4500, 4800, 5200]],
    series_names: ["Product A", "Product B", "Product C"],
    show_points: true,
    y_label: "Revenue ($)"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/multiline.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Line Chart](line.md)
- [Area Chart](area.md)
