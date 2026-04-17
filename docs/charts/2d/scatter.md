# Scatter Chart

<div class="lang-en">

## Signature

```python
sp.build_scatter_chart(
    title: str,
    x_values: list[float],
    y_values: list[float],
    *,
    color_hex: int = 0,
    show_text: bool = False,
    labels: list[str] | None = None,
    sizes: list[float] | None = None,
    color_groups: list[str] | None = None,
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
    show_regression: bool = False,
    regression_type: str = "linear",
) -> Chart
```

---

## Description

2D scatter plot with optional per-point sizing, grouping, labels, and regression line.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x_values` | `list[float]` | required | X coordinates |
| `y_values` | `list[float]` | required | Y coordinates |
| `color_hex` | `int` | `0` | Uniform point color |
| `show_text` | `bool` | `False` | Show point labels on chart |
| `labels` | `list[str] \| None` | `None` | Per-point label text |
| `sizes` | `list[float] \| None` | `None` | Per-point relative size (0.0–1.0) |
| `color_groups` | `list[str] \| None` | `None` | Per-point group name for color |
| `show_regression` | `bool` | `False` | Overlay regression line |
| `regression_type` | `str` | `"linear"` | `"linear"` or `"polynomial"` |
| `width` | `int` | `900` | Width in pixels |
| `height` | `int` | `480` | Height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `False` | Show gridlines |
| `background` | `str \| None` | `None` | Background color |

---

## Returns

`Chart`

---

## Examples

### Basic scatter


### Colored groups with regression






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
<div class="sp-tabs" id="scatter">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('scatter','scatter-py',this)">Python</button><button class="sp-tb" onclick="spTab('scatter','scatter-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('scatter','scatter-ts',this)">TypeScript</button></div>
<div id="scatter-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import numpy as np

rng = np.random.default_rng(0)
x_a = rng.normal(0, 1, 200).tolist()
y_a = [xi * 1.5 + rng.normal() for xi in x_a]
x_b = rng.normal(3, 1, 200).tolist()
y_b = [xi * 0.5 + rng.normal() for xi in x_b]

chart = sp.build_scatter_chart(
    "Two Populations",
    x_values=x_a + x_b,
    y_values=y_a + y_b,
    color_groups=["Group A"] * 200 + ["Group B"] * 200,
    show_regression=True,
    regression_type="linear",
    x_label="X",
    y_label="Y",
)</code></pre></div>
<div id="scatter-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import numpy as np

const rng = np.random.default_rng(0)
const x_a = rng.normal(0, 1, 200).tolist()
const y_a = [xi * 1.5 + rng.normal() for xi in x_a]
const x_b = rng.normal(3, 1, 200).tolist()
const y_b = [xi * 0.5 + rng.normal() for xi in x_b]

const chart = sp.build_scatter_chart("Two Populations",
x_a + x_b,
{
    y_values: y_a + y_b,
    color_groups: ["Group A"],
    show_regression: true,
    regression_type: "linear",
    x_label: "X",
    y_label: "Y"
})</code></pre></div>
<div id="scatter-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import numpy as np

const rng = np.random.default_rng(0)
const x_a = rng.normal(0, 1, 200).tolist()
const y_a: number[] = [xi * 1.5 + rng.normal() for xi in x_a]
const x_b = rng.normal(3, 1, 200).tolist()
const y_b: number[] = [xi * 0.5 + rng.normal() for xi in x_b]

const chart = sp.build_scatter_chart("Two Populations",
x_a + x_b,
{
    y_values: y_a + y_b,
    color_groups: ["Group A"],
    show_regression: true,
    regression_type: "linear",
    x_label: "X",
    y_label: "Y"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/scatter.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [DBSCAN 2D](../../ml/dbscan.md) — automatic clustering on scatter data
- [Bubble](bubble.md) — scatter with third size dimension
- [Scatter 3D](../3d/scatter3d.md)

</div>

<div class="lang-fr">

## Description

Nuage de points 2D avec taille par point, regroupement, étiquettes et droite de régression optionnels.

</div>
