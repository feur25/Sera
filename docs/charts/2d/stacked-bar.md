# Stacked Bar Chart

<div class="lang-en">

## Signature

```python
sp.build_stacked_bar(
    title: str,
    category_labels: list[str],
    series_values: list[float],
    *,
    show_values: bool = False,
    series_names: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    hover_json: str = "",
    legend_position: str = "right",
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Stacked bar chart. Each bar is split into segments representing series contributions.

Same flat `series_values` layout as `build_grouped_bar`.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | X-axis categories |
| `series_values` | `list[float]` | required | Flat values row-major `[cat0_s0, cat0_s1, ...]` |
| `show_values` | `bool` | `False` | Show segment value labels |
| `series_names` | `list[str] \| None` | `None` | Labels for each series |

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
<div class="sp-tabs" id="stacked-bar">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('stacked-bar','stacked-bar-py',this)">Python</button><button class="sp-tb" onclick="spTab('stacked-bar','stacked-bar-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('stacked-bar','stacked-bar-ts',this)">TypeScript</button></div>
<div id="stacked-bar-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

months = ["Jan", "Feb", "Mar"]
costs = [
    400.0, 150.0, 80.0,
    380.0, 170.0, 95.0,
    420.0, 160.0, 90.0,
]

chart = sp.build_stacked_bar(
    "Monthly Costs",
    category_labels=months,
    series_values=costs,
    series_names=["Labor", "Materials", "Overhead"],
    legend_position="right",
    gridlines=True,
)</code></pre></div>
<div id="stacked-bar-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const months = ["Jan", "Feb", "Mar"]
const costs = [
    400.0, 150.0, 80.0,
    380.0, 170.0, 95.0,
    420.0, 160.0, 90.0,
]

const chart = sp.build_stacked_bar("Monthly Costs",
months,
{
    series_values: costs,
    series_names: ["Labor", "Materials", "Overhead"],
    legend_position: "right",
    gridlines: true
})</code></pre></div>
<div id="stacked-bar-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const months: string[] = ["Jan", "Feb", "Mar"]
const costs: number[] = [
    400.0, 150.0, 80.0,
    380.0, 170.0, 95.0,
    420.0, 160.0, 90.0,
]

const chart = sp.build_stacked_bar("Monthly Costs",
months,
{
    series_values: costs,
    series_names: ["Labor", "Materials", "Overhead"],
    legend_position: "right",
    gridlines: true
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/stacked-bar.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Grouped Bar](grouped-bar.md)
- [Stacked Bar 3D](../3d/stacked-bar3d.md)

</div>

<div class="lang-fr">

## Description

Graphique en barres empilées. Chaque barre est divisée en segments représentant les contributions des séries.

</div>
