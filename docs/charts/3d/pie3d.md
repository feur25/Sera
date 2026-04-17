# Pie Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_pie3d_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_pct: bool = True,
    extrusion: float = 0.2,
    bg_color: str = "#1a1a2e",
    palette: list[int] | None = None,
    width: int = 700,
    height: int = 600,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

3D pie chart rendered as extruded arc segments in a WebGL scene.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Slice labels |
| `values` | `list[float]` | required | Slice values |
| `show_pct` | `bool` | `True` | Show percentage labels |
| `extrusion` | `float` | `0.2` | Depth of pie extrusion |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `palette` | `list[int] \| None` | `None` | Custom palette |
| `width` | `int` | `700` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

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
<div class="sp-tabs" id="pie3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('pie3d','pie3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('pie3d','pie3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('pie3d','pie3d-ts',this)">TypeScript</button></div>
<div id="pie3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_pie3d_chart(
    "Market Share 3D",
    labels=["Chrome", "Safari", "Firefox", "Edge"],
    values=[65, 19, 4, 4],
)
</code></pre></div>
<div id="pie3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.build_pie3d_chart("Market Share 3D",
["Chrome", "Safari", "Firefox", "Edge"],
{
    values: [65, 19, 4, 4]
})</code></pre></div>
<div id="pie3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.build_pie3d_chart("Market Share 3D",
["Chrome", "Safari", "Firefox", "Edge"],
{
    values: [65, 19, 4, 4]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/pie3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Pie 2D](../2d/pie.md)
- [Sunburst 3D](sunburst3d.md)

</div>

<div class="lang-fr">

## Description

Camembert 3D rendu comme des segments d'arc extrudes dans une scène WebGL.

</div>
