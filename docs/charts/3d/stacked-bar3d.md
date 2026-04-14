# Stacked Bar Chart 3D

## Signature

```python
sp.build_stacked_bar3d_chart(
    title: str,
    category_labels: list[str],
    series_values: list[float],
    *,
    series_names: list[str] | None = None,
    show_values: bool = False,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "",
) -> Chart
```

---

## Description

3D stacked bar chart — each bar is segmented into series, rendered as stacked prisms.

`series_values` is a flat list in row-major order: `[cat0_s0, cat0_s1, …, cat1_s0, …]`.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | Category labels |
| `series_values` | `list[float]` | required | Flat row-major series data |
| `series_names` | `list[str] \| None` | `None` | Legend names |
| `show_values` | `bool` | `False` | Labels on segments |
| `palette` | `list[int] \| None` | `None` | Per-series colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |

---

## Returns

`Chart`

---

## Examples




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
<div class="sp-tabs" id="stacked-bar3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('stacked-bar3d','stacked-bar3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('stacked-bar3d','stacked-bar3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('stacked-bar3d','stacked-bar3d-ts',this)">TypeScript</button></div>
<div id="stacked-bar3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

categories = ["Q1", "Q2", "Q3", "Q4"]
series_data = [
    [30, 40, 25, 50],
    [20, 35, 45, 30],
    [50, 25, 30, 20],
]

chart = sp.build_stacked_bar3d_chart(
    "Quarterly Revenue 3D",
    category_labels=categories,
    series_values=series_data,
    series_names=["Product A", "Product B", "Product C"],
)</code></pre></div>
<div id="stacked-bar3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const categories = ["Q1", "Q2", "Q3", "Q4"]
const series_data = [
    [30, 40, 25, 50],
    [20, 35, 45, 30],
    [50, 25, 30, 20],
]

const chart = sp.buildStackedBar3dChart("Quarterly Revenue 3D",
categories,
{
    series_values: series_data,
    series_names: ["Product A", "Product B", "Product C"]
})</code></pre></div>
<div id="stacked-bar3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const categories: string[] = ["Q1", "Q2", "Q3", "Q4"]
const series_data: number[] = [
    [30, 40, 25, 50],
    [20, 35, 45, 30],
    [50, 25, 30, 20],
]

const chart = sp.buildStackedBar3dChart("Quarterly Revenue 3D",
categories,
{
    series_values: series_data,
    series_names: ["Product A", "Product B", "Product C"]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/stacked-bar3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Stacked Bar 2D](../2d/stacked-bar.md)
- [Bar 3D](bar3d.md)
