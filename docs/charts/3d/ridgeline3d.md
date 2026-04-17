# Ridgeline Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_ridgeline3d_chart(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    bandwidth: float = 1.0,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "Density",
) -> Chart
```

---

## Description

Ridgeline chart in 3D — KDE surfaces per category arranged along the Y axis in a WebGL scene.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `categories` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Flat concatenated sample data |
| `bandwidth` | `float` | `1.0` | KDE bandwidth |
| `palette` | `list[int] \| None` | `None` | Per-ridge colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
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
<div class="sp-tabs" id="ridgeline3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('ridgeline3d','ridgeline3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('ridgeline3d','ridgeline3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('ridgeline3d','ridgeline3d-ts',this)">TypeScript</button></div>
<div id="ridgeline3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random

cats   = ["Low", "Medium", "High"]
means  = [10, 50, 90]
values = [v for m in means for v in [random.gauss(m, 8) for _ in range(150)]]

chart = sp.build_ridgeline3d_chart(
    "Score Distribution by Group",
    categories=cats,
    values=values,
)</code></pre></div>
<div id="ridgeline3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import random

const cats   = ["Low", "Medium", "High"]
const means  = [10, 50, 90]
const values = [v for m in means for v in [random.gauss(m, 8) for _ in range(150)]]

const chart = sp.build_ridgeline3d_chart("Score Distribution by Group",
cats,
{
    values: values
})</code></pre></div>
<div id="ridgeline3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import random

const cats: string[] = ["Low", "Medium", "High"]
const means: number[] = [10, 50, 90]
const values: number[] = [v for m in means for v in [random.gauss(m, 8) for _ in range(150)]]

const chart = sp.build_ridgeline3d_chart("Score Distribution by Group",
cats,
{
    values: values
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/ridgeline3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Ridgeline 2D](../2d/ridgeline.md)
- [KDE 3D](kde3d.md)

</div>

<div class="lang-fr">

## Description

Ridgeline 3D — surfaces KDE par catégorie arrangées le long de l'axe Y dans une scène WebGL.

</div>
