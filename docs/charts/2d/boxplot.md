# Box Plot

<div class="lang-en">

## Signature

```python
sp.build_boxplot(
    title: str,
    category_labels: list[str],
    values: list[float],
    *,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    background: str | None = None,
    gridlines: bool = True,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Box-and-whisker plot showing statistical distribution per category.
Each box displays Q1, median, Q3, and IQR whiskers.

`values` is a flat list concatenating all category samples; the lengths must be equal across categories (same number of values per category).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | One label per box |
| `values` | `list[float]` | required | Flat list of all samples |
| `color_hex` | `int` | `0x6366F1` | Single box fill color |
| `palette` | `list[int] \| None` | `None` | Per-category colors |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Show horizontal gridlines |

---

## Returns

`Chart`

---

## Examples

### Test scores by class






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
<div class="sp-tabs" id="boxplot">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('boxplot','boxplot-py',this)">Python</button><button class="sp-tb" onclick="spTab('boxplot','boxplot-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('boxplot','boxplot-ts',this)">TypeScript</button></div>
<div id="boxplot-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random

n = 50
groups = {
    "Class A": [random.gauss(72, 10) for _ in range(n)],
    "Class B": [random.gauss(68, 15) for _ in range(n)],
    "Class C": [random.gauss(80, 8)  for _ in range(n)],
}

labels = list(groups.keys())
values = [v for g in groups.values() for v in g]

chart = sp.build_boxplot(
    "Test Score Distribution by Class",
    category_labels=labels,
    values=values,
    y_label="Score",
    gridlines=True,
)</code></pre></div>
<div id="boxplot-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import random

const n = 50
const groups = {
    "Class A": [random.gauss(72, 10) for _ in range(n)],
    "Class B": [random.gauss(68, 15) for _ in range(n)],
    "Class C": [random.gauss(80, 8)  for _ in range(n)],
}

const labels = list(groups.keys())
const values = [v for g in groups.values() for v in g]

const chart = sp.build_boxplot("Test Score Distribution by Class",
labels,
{
    values: values,
    y_label: "Score",
    gridlines: true
})</code></pre></div>
<div id="boxplot-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import random

const n: number = 50
const groups = {
    "Class A": [random.gauss(72, 10) for _ in range(n)],
    "Class B": [random.gauss(68, 15) for _ in range(n)],
    "Class C": [random.gauss(80, 8)  for _ in range(n)],
}

const labels = list(groups.keys())
const values: number[] = [v for g in groups.values() for v in g]

const chart = sp.build_boxplot("Test Score Distribution by Class",
labels,
{
    values: values,
    y_label: "Score",
    gridlines: true
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/boxplot.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Violin](violin.md)
- [Histogram](histogram.md)
- [KDE](kde.md)

</div>

<div class="lang-fr">

## Description

Boite à moustaches montrant la distribution statistique par catégorie. Affiche Q1, médiane, Q3 et les moustaches.

</div>
