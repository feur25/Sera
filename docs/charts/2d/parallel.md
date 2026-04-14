# Parallel Coordinates

## Signature

```python
sp.build_parallel(
    title: str,
    axes: list[str],
    series: list[list[float]],
    *,
    series_names: list[str] | None = None,
    color_groups: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 1000,
    height: int = 480,
    background: str | None = None,
    line_opacity: float = 0.6,
) -> Chart
```

---

## Description

Parallel coordinates chart — each axis is a dimension, each line is an observation.
Ideal for detecting patterns in high-dimensional data.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `axes` | `list[str]` | required | Axis labels (one per dimension) |
| `series` | `list[list[float]]` | required | One inner list per observation (must match `len(axes)`) |
| `series_names` | `list[str] \| None` | `None` | Label per observation |
| `color_groups` | `list[str] \| None` | `None` | Group names for coloring lines |
| `palette` | `list[int] \| None` | `None` | Custom group colors |
| `width` | `int` | `1000` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `background` | `str \| None` | `None` | Background color |
| `line_opacity` | `float` | `0.6` | Line opacity (0.0–1.0) |

---

## Returns

`Chart`

---

## Examples

### Iris dataset parallel coordinates






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
<div class="sp-tabs" id="parallel">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('parallel','parallel-py',this)">Python</button><button class="sp-tb" onclick="spTab('parallel','parallel-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('parallel','parallel-ts',this)">TypeScript</button></div>
<div id="parallel-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Sepal Length", "Sepal Width", "Petal Length", "Petal Width"]

data = [
    [5.1, 3.5, 1.4, 0.2],
    [6.7, 3.1, 4.7, 1.5],
    [6.3, 3.3, 6.0, 2.5],
]
groups = ["Setosa", "Versicolor", "Virginica"]

chart = sp.build_parallel(
    "Iris Parallel Coordinates",
    axes=axes,
    series_values=data,
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e],
)</code></pre></div>
<div id="parallel-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const axes = ["Sepal Length", "Sepal Width", "Petal Length", "Petal Width"]

const data = [
    [5.1, 3.5, 1.4, 0.2],
    [6.7, 3.1, 4.7, 1.5],
    [6.3, 3.3, 6.0, 2.5],
]
const groups = ["Setosa", "Versicolor", "Virginica"]

const chart = sp.build_parallel("Iris Parallel Coordinates",
axes,
{
    series_values: data,
    palette: [0x6366f1, 0x22d3ee, 0xf43f5e]
})</code></pre></div>
<div id="parallel-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const axes: string[] = ["Sepal Length", "Sepal Width", "Petal Length", "Petal Width"]

const data: number[] = [
    [5.1, 3.5, 1.4, 0.2],
    [6.7, 3.1, 4.7, 1.5],
    [6.3, 3.3, 6.0, 2.5],
]
const groups: string[] = ["Setosa", "Versicolor", "Virginica"]

const chart = sp.build_parallel("Iris Parallel Coordinates",
axes,
{
    series_values: data,
    palette: [0x6366f1, 0x22d3ee, 0xf43f5e]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/parallel.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Scatter](scatter.md)
- [Radar Chart](radar.md)
