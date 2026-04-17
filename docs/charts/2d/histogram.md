# Histogram

<div class="lang-en">

## Signature

```python
sp.build_histogram(
    title: str,
    values: list[float],
    *,
    color_hex: int = 0,
    bins: int = 20,
    show_counts: bool = False,
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

Distribution histogram with configurable bin count.

Histogramme de distribution avec nombre de bins configurable.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `values` | `list[float]` | required | Raw numerical data |
| `bins` | `int` | `20` | Number of histogram bins |
| `show_counts` | `bool` | `False` | Show count labels above each bar |
| `color_hex` | `int` | `0` | Bar color as hex int |
| `width` | `int` | `900` | Width in pixels |
| `height` | `int` | `480` | Height in pixels |

---

## Returns

`Chart`

---

## Examples

### Normal distribution






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
<div class="sp-tabs" id="histogram">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('histogram','histogram-py',this)">Python</button><button class="sp-tb" onclick="spTab('histogram','histogram-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('histogram','histogram-ts',this)">TypeScript</button></div>
<div id="histogram-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import numpy as np

data = np.random.normal(0, 1, 5000).tolist()

chart = sp.build_histogram(
    "Normal Distribution — N(0,1)",
    values=data,
    bins=40,
    x_label="Value",
    y_label="Count",
    gridlines=True,
    show_counts=False,
)</code></pre></div>
<div id="histogram-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import numpy as np

const data = np.random.normal(0, 1, 5000).tolist()

const chart = sp.build_histogram("Normal Distribution — N(0,1)",
{
    values: data,
    bins: 40,
    x_label: "Value",
    y_label: "Count",
    gridlines: true,
    show_counts: false
})</code></pre></div>
<div id="histogram-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import numpy as np

const data = np.random.normal(0, 1, 5000).tolist()

const chart = sp.build_histogram("Normal Distribution — N(0,1)",
{
    values: data,
    bins: 40,
    x_label: "Value",
    y_label: "Count",
    gridlines: true,
    show_counts: false
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/histogram.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Histogram Overlay](histogram-overlay.md) — compare two distributions
- [KDE](kde.md) — smooth density estimate
- [Violin](violin.md) — distribution by category

</div>

<div class="lang-fr">

## Description

Histogramme de distribution avec nombre de bins configurable.

</div>
