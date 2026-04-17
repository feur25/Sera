# Pie Chart

<div class="lang-en">

## Signature

```python
sp.build_pie_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_pct: bool = True,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
    legend_position: str = "right",
) -> Chart
```

---

## Description

Standard pie chart with optional percentage labels.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Slice labels |
| `values` | `list[float]` | required | Slice values (auto-normalized to 100 %) |
| `show_pct` | `bool` | `True` | Show percentage text inside slices |
| `width` | `int` | `700` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `palette` | `list[int] \| None` | `None` | Custom hex color palette |
| `background` | `str \| None` | `None` | Chart background color |
| `hover_json` | `str \| None` | `None` | Custom hover tooltip JSON |
| `legend_position` | `str` | `"right"` | `"right"`, `"bottom"`, `"top"` |

---

## Returns

`Chart`

---

## Examples

### Market share


### Custom palette






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
<div class="sp-tabs" id="pie">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('pie','pie-py',this)">Python</button><button class="sp-tb" onclick="spTab('pie','pie-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('pie','pie-ts',this)">TypeScript</button></div>
<div id="pie-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_pie_chart(
    "Revenue by Region",
    labels=["North", "South", "East", "West"],
    values=[30, 25, 20, 25],
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e, 0xf59e0b],
    legend_position="bottom",
)</code></pre></div>
<div id="pie-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.build_pie_chart("Revenue by Region",
["North", "South", "East", "West"],
{
    values: [30, 25, 20, 25],
    palette: [0x6366f1, 0x22d3ee, 0xf43f5e, 0xf59e0b],
    legend_position: "bottom"
})</code></pre></div>
<div id="pie-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.build_pie_chart("Revenue by Region",
["North", "South", "East", "West"],
{
    values: [30, 25, 20, 25],
    palette: [0x6366f1, 0x22d3ee, 0xf43f5e, 0xf59e0b],
    legend_position: "bottom"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/pie.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Donut Chart](donut.md)
- [Pie 3D](../3d/pie3d.md)
- [Sunburst](sunburst.md)

</div>

<div class="lang-fr">

## Description

Camembert standard avec étiquettes de pourcentage optionnelles.

</div>
