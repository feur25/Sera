# Globe 3D

<div class="lang-en">

## Signature

```python
sp.build_globe3d_chart(
    title: str,
    labels: list[str],
    latitudes: list[float],
    longitudes: list[float],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    bg_color: str = "#0f172a",
    width: int = 900,
    height: int = 700,
    hover_json: str | None = None,
    bar_height_scale: float = 1.0,
    show_graticule: bool = True,
) -> Chart
```

Aliases: `sp.globe3d`

---

## Description

Interactive 3D globe — geo data plotted as vertical bars or spikes on a sphere.
Drag to rotate, scroll to zoom.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Location names |
| `latitudes` | `list[float]` | required | Decimal latitudes |
| `longitudes` | `list[float]` | required | Decimal longitudes |
| `values` | `list[float]` | required | Bar heights per location |
| `color_hex` | `int` | `0x6366F1` | Default bar color |
| `palette` | `list[int] \| None` | `None` | Custom palette |
| `bg_color` | `str` | `"#0f172a"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `700` | Canvas height |
| `bar_height_scale` | `float` | `1.0` | Global height multiplier |
| `show_graticule` | `bool` | `True` | Draw lat/lon grid lines |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### World population





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
<div class="sp-tabs" id="globe3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('globe3d','globe3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('globe3d','globe3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('globe3d','globe3d-ts',this)">TypeScript</button></div>
<div id="globe3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.build_globe3d_chart(
    "World Population Spikes",
    labels=["China", "India", "USA", "Brazil", "Nigeria"],
    latitudes=[35.86, 20.59, 37.09, -14.23, 9.08],
    longitudes=[104.19, 78.96, -95.71, -51.92, 8.67],
    values=[1412, 1380, 331, 212, 218],
)</code></pre></div>
<div id="globe3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.build_globe3d_chart("World Population Spikes",
["China", "India", "USA", "Brazil", "Nigeria"],
[35.86, 20.59, 37.09, -14.23, 9.08],
{
    longitudes: [104.19, 78.96, -95.71, -51.92, 8.67],
    values: [1412, 1380, 331, 212, 218]
})</code></pre></div>
<div id="globe3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.build_globe3d_chart("World Population Spikes",
["China", "India", "USA", "Brazil", "Nigeria"],
[35.86, 20.59, 37.09, -14.23, 9.08],
{
    longitudes: [104.19, 78.96, -95.71, -51.92, 8.67],
    values: [1412, 1380, 331, 212, 218]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/globe3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_globe3d_chart(
    title: str,
    labels: list[str],
    latitudes: list[float],
    longitudes: list[float],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    bg_color: str = "#0f172a",
    width: int = 900,
    height: int = 700,
    hover_json: str | None = None,
    bar_height_scale: float = 1.0,
    show_graticule: bool = True,
) -> Chart
```

Aliases: `sp.globe3d`

---

<h2>Description</h2>

Globe interactif 3D — données géographiques tracées comme des barres verticales sur une sphère. Faites glisser pour tourner, défilez pour zoomer.

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Noms des lieux |
| `latitudes` | `list[float]` | requis | Latitudes décimales |
| `longitudes` | `list[float]` | requis | Longitudes décimales |
| `values` | `list[float]` | requis | Hauteurs des barres par lieu |
| `color_hex` | `int` | `0x6366F1` | Couleur par défaut des barres |
| `palette` | `list[int] \| None` | `None` | Palette personnalisée |
| `bg_color` | `str` | `"#0f172a"` | Couleur de fond |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `700` | Hauteur du canvas |
| `bar_height_scale` | `float` | `1.0` | Multiplicateur global de hauteur |
| `show_graticule` | `bool` | `True` | Dessiner les lignes de latitude/longitude |
| `hover_json` | `str \| None` | `None` | JSON d'infobulle personnalisée |

---

<h2>Retourne</h2>

`Chart`

---

<h2>Exemples</h2>

```python
import seraplot as sp

chart = sp.build_globe3d_chart(
    "Population mondiale",
    labels=["Chine", "Inde", "USA", "Brésil", "Nigéria"],
    latitudes=[35.86, 20.59, 37.09, -14.23, 9.08],
    longitudes=[104.19, 78.96, -95.71, -51.92, 8.67],
    values=[1412, 1380, 331, 212, 218],
)
```

---

</div>
