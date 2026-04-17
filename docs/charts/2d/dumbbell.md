# Dumbbell Chart

<div class="lang-en">

## Signature

```python
sp.build_dumbbell(
    title: str,
    labels: list[str],
    values_start: list[float],
    values_end: list[float],
    *,
    show_text: bool = True,
    color_start: int = 0x6366f1,
    color_end: int = 0xf43f5e,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    background: str | None = None,
    gridlines: bool = True,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Dumbbell chart — a horizontal line connecting two data points per category,
ideal for showing the gap or change between two states.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels |
| `values_start` | `list[float]` | required | Left (start) values |
| `values_end` | `list[float]` | required | Right (end) values |
| `show_text` | `bool` | `True` | Show endpoint value labels |
| `color_start` | `int` | `0x6366f1` | Start point color |
| `color_end` | `int` | `0xf43f5e` | End point color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `gridlines` | `bool` | `True` | Vertical gridlines |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### Life expectancy 2000 vs 2023






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
<div class="sp-tabs" id="dumbbell">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('dumbbell','dumbbell-py',this)">Python</button><button class="sp-tb" onclick="spTab('dumbbell','dumbbell-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('dumbbell','dumbbell-ts',this)">TypeScript</button></div>
<div id="dumbbell-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_dumbbell(
    "Life Expectancy: 2000 vs 2023",
    labels=["Japan", "Germany", "Brazil", "India", "Nigeria"],
    values_start=[81.2, 78.1, 70.4, 62.8, 46.5],
    values_end=[84.3, 81.5, 75.2, 70.8, 54.9],
    x_label="Age (years)",
)</code></pre></div>
<div id="dumbbell-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.build_dumbbell("Life Expectancy: 2000 vs 2023",
["Japan", "Germany", "Brazil", "India", "Nigeria"],
[81.2, 78.1, 70.4, 62.8, 46.5],
{
    values_end: [84.3, 81.5, 75.2, 70.8, 54.9],
    x_label: "Age (years)"
})</code></pre></div>
<div id="dumbbell-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.build_dumbbell("Life Expectancy: 2000 vs 2023",
["Japan", "Germany", "Brazil", "India", "Nigeria"],
[81.2, 78.1, 70.4, 62.8, 46.5],
{
    values_end: [84.3, 81.5, 75.2, 70.8, 54.9],
    x_label: "Age (years)"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/dumbbell.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Slope](slope.md)
- [Dumbbell 3D](../3d/dumbbell3d.md)

</div>

<div class="lang-fr">

## Description

Graphique haltère — une ligne horizontale reliant deux valeurs par catégorie, idéal pour montrer une évolution ou un écart entre deux états.

</div>
