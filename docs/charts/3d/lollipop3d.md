# Lollipop Chart 3D

## Signature

```python
sp.build_lollipop3d_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "",
    show_text: bool = False,
) -> Chart
```

---

## Description

3D lollipop chart — stems and spheres rendered in a WebGL scene.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Values per category |
| `color_hex` | `int` | `0x6366F1` | Stem and sphere color |
| `palette` | `list[int] \| None` | `None` | Per-category colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `show_text` | `bool` | `False` | Show value labels |

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
<div class="sp-tabs" id="lollipop3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('lollipop3d','lollipop3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('lollipop3d','lollipop3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('lollipop3d','lollipop3d-ts',this)">TypeScript</button></div>
<div id="lollipop3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_lollipop3d_chart(
    "Monthly Sales 3D",
    x_values=[1.0, 2.0, 3.0, 4.0, 5.0],
    y_values=[0.0, 0.0, 0.0, 0.0, 0.0],
    z_values=[120.0, 95.0, 140.0, 110.0, 160.0],
)</code></pre></div>
<div id="lollipop3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.buildLollipop3dChart("Monthly Sales 3D",
[1.0, 2.0, 3.0, 4.0, 5.0],
[0.0, 0.0, 0.0, 0.0, 0.0],
{
    z_values: [120.0, 95.0, 140.0, 110.0, 160.0]
})</code></pre></div>
<div id="lollipop3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.buildLollipop3dChart("Monthly Sales 3D",
[1.0, 2.0, 3.0, 4.0, 5.0],
[0.0, 0.0, 0.0, 0.0, 0.0],
{
    z_values: [120.0, 95.0, 140.0, 110.0, 160.0]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/lollipop3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Lollipop 2D](../2d/lollipop.md)
- [Bar 3D](bar3d.md)
