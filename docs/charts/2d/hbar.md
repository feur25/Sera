# Horizontal Bar Chart

<div class="lang-en">

## Signature

```python
sp.build_hbar(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0,
    show_text: bool = True,
    color_groups: list[str] | None = None,
    width: int = 900,
    height: int = 500,
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

Horizontal bar chart. Best for long category labels or ranking comparisons.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Bar values |
| `show_text` | `bool` | `True` | Show values on bars (default `True` for hbar) |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"`, `"none"` |
| `color_groups` | `list[str] \| None` | `None` | Group names for color mapping |
| `palette` | `list[int] \| None` | `None` | Custom hex color palette |
| `background` | `str \| None` | `None` | Background CSS color |
| `width` | `int` | `900` | Width in pixels |
| `height` | `int` | `500` | Height in pixels |

---

## Returns

`Chart`

---

## Examples

### Top 10 ranking






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
<div class="sp-tabs" id="hbar">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('hbar','hbar-py',this)">Python</button><button class="sp-tb" onclick="spTab('hbar','hbar-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('hbar','hbar-ts',this)">TypeScript</button></div>
<div id="hbar-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_hbar(
    "Top Countries by GDP",
    labels=["USA", "China", "Germany", "Japan", "India", "UK", "France", "Brazil", "Canada", "Korea"],
    values=[25.0, 18.0, 4.1, 4.2, 3.7, 3.1, 2.9, 2.1, 2.0, 1.7],
    sort_order="desc",
    x_label="GDP (trillion USD)",
    show_text=True,
    width=900,
    height=460,
)</code></pre></div>
<div id="hbar-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.build_hbar("Top Countries by GDP",
["USA", "China", "Germany", "Japan", "India", "UK", "France", "Brazil", "Canada", "Korea"],
{
    values: [25.0, 18.0, 4.1, 4.2, 3.7, 3.1, 2.9, 2.1, 2.0, 1.7],
    sort_order: "desc",
    x_label: "GDP (trillion USD)",
    show_text: true,
    width: 900,
    height: 460
})</code></pre></div>
<div id="hbar-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.build_hbar("Top Countries by GDP",
["USA", "China", "Germany", "Japan", "India", "UK", "France", "Brazil", "Canada", "Korea"],
{
    values: [25.0, 18.0, 4.1, 4.2, 3.7, 3.1, 2.9, 2.1, 2.0, 1.7],
    sort_order: "desc",
    x_label: "GDP (trillion USD)",
    show_text: true,
    width: 900,
    height: 460
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/hbar.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Lollipop](lollipop.md) — `sp.build_lollipop_chart()`

</div>

<div class="lang-fr">

## Description

Graphique en barres horizontal. Idéal pour les étiquettes longues ou les comparaisons de classement.

</div>
