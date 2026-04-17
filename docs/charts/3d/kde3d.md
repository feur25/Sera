# KDE Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_kde3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    *,
    bandwidth: float = 1.0,
    resolution: int = 50,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Density",
) -> Chart
```

---

## Description

2D Kernel Density Estimation rendered as a 3D surface mesh over a grid.
Visualizes the joint density of two variables.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | X sample data |
| `y` | `list[float]` | required | Y sample data |
| `bandwidth` | `float` | `1.0` | KDE bandwidth factor |
| `resolution` | `int` | `50` | Grid resolution (n × n) |
| `palette` | `list[int] \| None` | `None` | Color gradient palette |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `x_label` | `str` | `"X"` | X-axis label |
| `y_label` | `str` | `"Y"` | Y-axis label |
| `z_label` | `str` | `"Density"` | Z-axis label |

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
<div class="sp-tabs" id="kde3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('kde3d','kde3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('kde3d','kde3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('kde3d','kde3d-ts',this)">TypeScript</button></div>
<div id="kde3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random

values = (
    [random.gauss(-2, 1) for _ in range(200)] +
    [random.gauss(0, 0.8) for _ in range(200)] +
    [random.gauss(3, 1.2) for _ in range(200)]
)
categories = ["Group A"] * 200 + ["Group B"] * 200 + ["Group C"] * 200

chart = sp.build_kde3d_chart(
    "Density by Group",
    values,
    categories=categories,
)</code></pre></div>
<div id="kde3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import random

const values = (
    [random.gauss(-2, 1) for _ in range(200)] +
    [random.gauss(0, 0.8) for _ in range(200)] +
    [random.gauss(3, 1.2) for _ in range(200)]
)
const categories = ["Group A"] * 200 + ["Group B"] * 200 + ["Group C"] * 200

const chart = sp.build_kde3d_chart("Density by Group",
values,
{
    categories: categories
})</code></pre></div>
<div id="kde3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import random

const values = (
    [random.gauss(-2, 1) for _ in range(200)] +
    [random.gauss(0, 0.8) for _ in range(200)] +
    [random.gauss(3, 1.2) for _ in range(200)]
)
const categories: string[] = ["Group A"] * 200 + ["Group B"] * 200 + ["Group C"] * 200

const chart = sp.build_kde3d_chart("Density by Group",
values,
{
    categories: categories
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/kde3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [KDE 2D](../2d/kde.md)
- [Scatter 3D](scatter3d.md)

</div>

<div class="lang-fr">

## Description

Estimation par noyau 2D rendue comme une surface maillage 3D. Visualise la densité jointe de deux variables.

</div>
