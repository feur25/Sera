# Choropleth Map

## Signature

```python
sp.build_choropleth(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    iso_codes: list[str] | None = None,
    color_low: int = 0,
    color_high: int = 0,
    palette: list[int] | None = None,
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    hover_json: str | None = None,
    show_legend: bool = True,
    null_color: int = 0xdddddd,
) -> Chart
```

---

## Description

Choropleth (filled map) — country or region polygons colored by a scalar value.

Countries without data receive the `null_color`. Provide `iso_codes` (ISO-3166 alpha-3) to match countries automatically.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Country|
| `values` | `list[float]` | required | Values to color by |
| `iso_codes` | `list[str] \| None` | `None` | ISO-3166 alpha-3 codes |
| `color_low` | `int` | auto | Low value color |
| `color_high` | `int` | auto | High value color |
| `null_color` | `int` | `0xdddddd` | Color for countries with no data |
| `width` | `int` | `1000` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `show_legend` | `bool` | `True` | Show color scale legend |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### Unemployment rate choropleth




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
<div class="sp-tabs" id="choropleth">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('choropleth','choropleth-py',this)">Python</button><button class="sp-tb" onclick="spTab('choropleth','choropleth-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('choropleth','choropleth-ts',this)">TypeScript</button></div>
<div id="choropleth-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_choropleth(
    "Unemployment Rate by Country",
    labels=["FRA", "DEU", "ESP", "ITA", "PRT"],
    values=[7.1, 3.0, 11.8, 6.7, 6.2],
)</code></pre></div>
<div id="choropleth-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.buildChoropleth("Unemployment Rate by Country",
["FRA", "DEU", "ESP", "ITA", "PRT"],
{
    values: [7.1, 3.0, 11.8, 6.7, 6.2]
})</code></pre></div>
<div id="choropleth-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.buildChoropleth("Unemployment Rate by Country",
["FRA", "DEU", "ESP", "ITA", "PRT"],
{
    values: [7.1, 3.0, 11.8, 6.7, 6.2]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/choropleth.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Bubble Map](bubble-map.md)
- [Globe 3D](../3d/globe3d.md)
