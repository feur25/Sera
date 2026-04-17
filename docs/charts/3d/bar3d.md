# Bar Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_bar3d_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "",
    show_text: bool = False,
) -> Chart
```

---

## Description

3D bar chart rendering bars as extruded rectangular prisms on a WebGL canvas.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Bar labels |
| `values` | `list[float]` | required | Bar heights |
| `color_hex` | `int` | `0x6366F1` | Single bar color |
| `palette` | `list[int] \| None` | `None` | Per-bar colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `z_label` | `str` | `""` | Z-axis label |
| `show_text` | `bool` | `False` | Show value labels |

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
<div class="sp-tabs" id="bar3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('bar3d','bar3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('bar3d','bar3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('bar3d','bar3d-ts',this)">TypeScript</button></div>
<div id="bar3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_bar3d_chart(
    "Sales by Product",
    x_values=[0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0],
    y_values=[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
    z_values=[420.0, 380.0, 290.0, 510.0, 480.0, 420.0, 350.0, 590.0],
    x_label="Product",
    y_label="Year",
    z_label="Units",
)</code></pre></div>
<div id="bar3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.build_bar3d_chart("Sales by Product",
[0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0],
[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
{
    z_values: [420.0, 380.0, 290.0, 510.0, 480.0, 420.0, 350.0, 590.0],
    x_label: "Product",
    y_label: "Year",
    z_label: "Units"
})</code></pre></div>
<div id="bar3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.build_bar3d_chart("Sales by Product",
[0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0],
[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
{
    z_values: [420.0, 380.0, 290.0, 510.0, 480.0, 420.0, 350.0, 590.0],
    x_label: "Product",
    y_label: "Year",
    z_label: "Units"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/bar3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Bar Chart 2D](../2d/bar.md)
- [Stacked Bar 3D](stacked-bar3d.md)

</div>

<div class="lang-fr">

## Description

Graphique en barres 3D rendant les barres comme des prismes rectangulaires extrudes sur un canvas WebGL.

</div>
