# KDE Chart

## Signature

```python
sp.build_kde_chart(
    title: str,
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    bandwidth: float = 1.0,
    fill: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "Density",
    gridlines: bool = True,
    background: str | None = None,
    palette: list[int] | None = None,
    series_names: list[str] | None = None,
) -> Chart
```

---

## Description

Kernel Density Estimation (KDE) curve — a smooth, continuous estimate of a probability distribution.
Better than histograms for identifying the underlying shape of data.

When multiple series are provided via a flat `values` list with matching `series_names`, several overlaid density curves are drawn.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `values` | `list[float]` | required | Sample data points |
| `color_hex` | `int` | `0x6366F1` | Curve color |
| `bandwidth` | `float` | `1.0` | Smoothing bandwidth scale factor |
| `fill` | `bool` | `True` | Fill area under curve |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `"Density"` | Y-axis label |
| `gridlines` | `bool` | `True` | Horizontal gridlines |
| `palette` | `list[int] \| None` | `None` | Multi-series color palette |
| `series_names` | `list[str] \| None` | `None` | Multi-series legend names |

---

## Returns

`Chart`

---

## Examples

### Single distribution






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
<div class="sp-tabs" id="kde">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('kde','kde-py',this)">Python</button><button class="sp-tb" onclick="spTab('kde','kde-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('kde','kde-ts',this)">TypeScript</button></div>
<div id="kde-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random

values = [random.gauss(50, 10) for _ in range(500)]

chart = sp.build_kde_chart(
    "Score Distribution",
    values=values,
    x_label="Score",
    filled=True,
    bandwidth=1.0,
)</code></pre></div>
<div id="kde-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import random

const values = [random.gauss(50, 10) for _ in range(500)]

const chart = sp.build_kde_chart("Score Distribution",
{
    values: values,
    x_label: "Score",
    filled: true,
    bandwidth: 1.0
})</code></pre></div>
<div id="kde-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import random

const values: number[] = [random.gauss(50, 10) for _ in range(500)]

const chart = sp.build_kde_chart("Score Distribution",
{
    values: values,
    x_label: "Score",
    filled: true,
    bandwidth: 1.0
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/kde.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Histogram](histogram.md)
- [Violin](violin.md)
- [Ridgeline](ridgeline.md)
- [KDE 3D](../3d/kde3d.md)
