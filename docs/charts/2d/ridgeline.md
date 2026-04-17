# Ridgeline Chart

<div class="lang-en">

## Signature

```python
sp.build_ridgeline_chart(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    bandwidth: float = 1.0,
    overlap: float = 0.5,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    background: str | None = None,
    gridlines: bool = False,
) -> Chart
```

---

## Description

Ridgeline (joy) chart — stacked KDE curves per category.
Excellent for comparing distributional shapes across many groups.

Excellent pour comparer les formes de distribution entre de nombreux groupes.

`values` is a flat list. The number of values must be divisible by `len(categories)` (equal samples per group).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `categories` | `list[str]` | required | Category labels (one ridge each) |
| `values` | `list[float]` | required | Flat concatenated sample data |
| `bandwidth` | `float` | `1.0` | KDE bandwidth factor |
| `overlap` | `float` | `0.5` | Ridge overlap (0 = no overlap, 1 = full overlap) |
| `color_hex` | `int` | `0x6366F1` | Single fill color |
| `palette` | `list[int] \| None` | `None` | Per-ridge colors |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `gridlines` | `bool` | `False` | Vertical gridlines |

---

## Returns

`Chart`

---

## Examples

### Daily temperature ridgeline






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
<div class="sp-tabs" id="ridgeline">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('ridgeline','ridgeline-py',this)">Python</button><button class="sp-tb" onclick="spTab('ridgeline','ridgeline-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('ridgeline','ridgeline-ts',this)">TypeScript</button></div>
<div id="ridgeline-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random

months = ["Jan", "Apr", "Jul", "Oct"]
means  = [5, 15, 28, 16]

values = [v for m in means for v in [random.gauss(m, 4) for _ in range(100)]]

chart = sp.build_ridgeline_chart(
    "Monthly Temperature Distribution",
    categories=months,
    values=values,
    overlap=0.6,
    x_label="°C",
)</code></pre></div>
<div id="ridgeline-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import random

const months = ["Jan", "Apr", "Jul", "Oct"]
const means  = [5, 15, 28, 16]

const values = [v for m in means for v in [random.gauss(m, 4) for _ in range(100)]]

const chart = sp.build_ridgeline_chart("Monthly Temperature Distribution",
months,
{
    values: values,
    overlap: 0.6,
    x_label: "°C"
})</code></pre></div>
<div id="ridgeline-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import random

const months: string[] = ["Jan", "Apr", "Jul", "Oct"]
const means: number[] = [5, 15, 28, 16]

const values: number[] = [v for m in means for v in [random.gauss(m, 4) for _ in range(100)]]

const chart = sp.build_ridgeline_chart("Monthly Temperature Distribution",
months,
{
    values: values,
    overlap: 0.6,
    x_label: "°C"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/ridgeline.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [KDE](kde.md)
- [Violin](violin.md)
- [Ridgeline 3D](../3d/ridgeline3d.md)

</div>

<div class="lang-fr">

## Description

Graphique ridgeline (joy plot) — courbes KDE empilées par catégorie. Excellent pour comparer les formes de distribution entre de nombreux groupes.

</div>
