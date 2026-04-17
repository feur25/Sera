# Bubble Chart

<div class="lang-en">

## Signature

```python
sp.build_bubble(
    title: str,
    x_values: list[float],
    y_values: list[float],
    sizes: list[float],
    *,
    labels: list[str] | None = None,
    color_groups: list[str] | None = None,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Bubble chart: scatter plot where each point's area encodes a third dimension (`sizes`).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x_values` | `list[float]` | required | X-axis positions |
| `y_values` | `list[float]` | required | Y-axis positions |
| `sizes` | `list[float]` | required | Values that drive bubble radius |
| `labels` | `list[str] \| None` | `None` | Per-bubble text labels |
| `color_groups` | `list[str] \| None` | `None` | Group names for coloring |
| `color_hex` | `int` | `0x6366F1` | Default bubble color |
| `palette` | `list[int] \| None` | `None` | Custom palette per group |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Gridlines |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### Gapminder-style chart






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
<div class="sp-tabs" id="bubble">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('bubble','bubble-py',this)">Python</button><button class="sp-tb" onclick="spTab('bubble','bubble-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('bubble','bubble-ts',this)">TypeScript</button></div>
<div id="bubble-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

countries = ["USA", "China", "Germany", "India", "Brazil"]
gdp       = [65000, 12500, 48000, 2100, 8800]
life_exp  = [78.5, 77.1, 81.3, 69.7, 75.2]
population= [331, 1412, 83, 1380, 212]

chart = sp.build_bubble(
    "GDP vs Life Expectancy (2023)",
    x_values=gdp,
    y_values=life_exp,
    sizes=[p / 10 for p in population],
    categories=countries,
    x_label="GDP per capita ($)",
    y_label="Life expectancy (years)",
)</code></pre></div>
<div id="bubble-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const countries = ["USA", "China", "Germany", "India", "Brazil"]
const gdp       = [65000, 12500, 48000, 2100, 8800]
const life_exp  = [78.5, 77.1, 81.3, 69.7, 75.2]
const population= [331, 1412, 83, 1380, 212]

const chart = sp.build_bubble("GDP vs Life Expectancy (2023)",
gdp,
life_exp,
{
    sizes: [p / 10 for p in population],
    categories: countries,
    x_label: "GDP per capita ($)",
    y_label: "Life expectancy (years)"
})</code></pre></div>
<div id="bubble-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const countries: string[] = ["USA", "China", "Germany", "India", "Brazil"]
const gdp: number[] = [65000, 12500, 48000, 2100, 8800]
const life_exp: number[] = [78.5, 77.1, 81.3, 69.7, 75.2]
const population= [331, 1412, 83, 1380, 212]

const chart = sp.build_bubble("GDP vs Life Expectancy (2023)",
gdp,
life_exp,
{
    sizes: [p / 10 for p in population],
    categories: countries,
    x_label: "GDP per capita ($)",
    y_label: "Life expectancy (years)"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/bubble.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Scatter](scatter.md)
- [Bubble Map](../map/bubble-map.md)
- [Bubble 3D](../3d/bubble3d.md)

</div>

<div class="lang-fr">

## Description

Graphique à bulles : nuage de points où la surface de chaque point encode une troisième dimension (`sizes`).

</div>
