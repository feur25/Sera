# Bubble Map

<div class="lang-en">

## Signature

```python
sp.build_bubble_map(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    latitudes: list[float] | None = None,
    longitudes: list[float] | None = None,
    iso_codes: list[str] | None = None,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    hover_json: str | None = None,
    bubble_opacity: float = 0.6,
    min_bubble_size: float = 5.0,
    max_bubble_size: float = 50.0,
) -> Chart
```

---

## Description

World map with proportional bubbles at geographic coordinates.
Use `iso_codes` for country-level data (the library resolves centroids automatically), or pass explicit `latitudes` / `longitudes`.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Location names |
| `values` | `list[float]` | required | Bubble sizes |
| `latitudes` | `list[float] \| None` | `None` | Manual latitudes |
| `longitudes` | `list[float] \| None` | `None` | Manual longitudes |
| `iso_codes` | `list[str] \| None` | `None` | ISO-3166 alpha-3 country codes |
| `color_hex` | `int` | `0x6366F1` | Bubble color |
| `palette` | `list[int] \| None` | `None` | Multi-group palette |
| `width` | `int` | `1000` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `bubble_opacity` | `float` | `0.6` | Bubble transparency (0–1) |
| `min_bubble_size` | `float` | `5.0` | Minimum bubble radius in pixels |
| `max_bubble_size` | `float` | `50.0` | Maximum bubble radius in pixels |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### GDP per country (ISO code lookup)





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
<div class="sp-tabs" id="bubble-map">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('bubble-map','bubble-map-py',this)">Python</button><button class="sp-tb" onclick="spTab('bubble-map','bubble-map-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('bubble-map','bubble-map-ts',this)">TypeScript</button></div>
<div id="bubble-map-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_bubble_map(
    "GDP by Country",
    labels=["USA", "CHN", "DEU", "JPN", "FRA"],
    values=[25500, 17700, 4400, 4200, 3100],
)</code></pre></div>
<div id="bubble-map-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.build_bubble_map("GDP by Country",
["USA", "CHN", "DEU", "JPN", "FRA"],
{
    values: [25500, 17700, 4400, 4200, 3100]
})</code></pre></div>
<div id="bubble-map-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.build_bubble_map("GDP by Country",
["USA", "CHN", "DEU", "JPN", "FRA"],
{
    values: [25500, 17700, 4400, 4200, 3100]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/bubble-map.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

### Custom coordinates

```python
import seraplot as sp

chart = sp.build_bubble_map(
    "City Populations",
    labels=["Paris", "Tokyo", "New York", "Lagos"],
    values=[11, 37, 20, 15],
    latitudes=[48.85, 35.68, 40.71, 6.52],
    longitudes=[2.35, 139.69, -74.01, 3.38],
)
```

---

## See also

- [Choropleth](choropleth.md)
- [Globe 3D](../3d/globe3d.md)
