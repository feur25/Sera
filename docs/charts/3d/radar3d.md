# Radar Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_radar3d_chart(
    title: str,
    axes: list[str],
    series: list[list[float]],
    *,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 700,
    height: int = 600,
    max_val: float | None = None,
    fill_opacity: float = 0.25,
) -> Chart
```

---

## Description

3D rendered radar (spider) chart. Same concept as the 2D radar but rendered in a WebGL 3D scene.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `axes` | `list[str]` | required | Axis labels |
| `series` | `list[list[float]]` | required | Values per series per axis |
| `series_names` | `list[str] \| None` | `None` | Legend names |
| `palette` | `list[int] \| None` | `None` | Series colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `700` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `max_val` | `float \| None` | `None` | Common scale maximum |
| `fill_opacity` | `float` | `0.25` | Fill opacity |

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
<div class="sp-tabs" id="radar3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('radar3d','radar3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('radar3d','radar3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('radar3d','radar3d-ts',this)">TypeScript</button></div>
<div id="radar3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_radar3d_chart(
    "Skills Comparison 3D",
    axes=["Python", "Rust", "SQL", "ML", "DevOps"],
    series_values=[
        [9, 7, 8, 8, 6],
        [5, 10, 6, 4, 9],
    ],
    series_names=["Alice", "Bob"],
)</code></pre></div>
<div id="radar3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.build_radar3d_chart("Skills Comparison 3D",
["Python", "Rust", "SQL", "ML", "DevOps"],
{
    series_values: [[9, 7, 8, 8, 6], [5, 10, 6, 4, 9]],
    series_names: ["Alice", "Bob"]
})</code></pre></div>
<div id="radar3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.build_radar3d_chart("Skills Comparison 3D",
["Python", "Rust", "SQL", "ML", "DevOps"],
{
    series_values: [[9, 7, 8, 8, 6], [5, 10, 6, 4, 9]],
    series_names: ["Alice", "Bob"]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/radar3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Radar 2D](../2d/radar.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_radar3d_chart(
    title: str,
    axes: list[str],
    series: list[list[float]],
    *,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 700,
    height: int = 600,
    max_val: float | None = None,
    fill_opacity: float = 0.25,
) -> Chart
```

---

## Description

Graphique radar 3D (toile d'araignée). Même concept que le radar 2D mais rendu dans une scène WebGL.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `axes` | `list[str]` | requis | Étiquettes des axes |
| `series` | `list[list[float]]` | requis | Valeurs par série par axe |
| `series_names` | `list[str] \| None` | `None` | Noms de légende |
| `palette` | `list[int] \| None` | `None` | Couleurs des séries |
| `bg_color` | `str` | `"#1a1a2e"` | Couleur de fond |
| `width` | `int` | `700` | Largeur du canvas |
| `height` | `int` | `600` | Hauteur du canvas |
| `max_val` | `float \| None` | `None` | Maximum de l'échelle commune |
| `fill_opacity` | `float` | `0.25` | Opacité du remplissage |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp

chart = sp.build_radar3d_chart(
    "Comparaison de compétences 3D",
    axes=["Python", "Rust", "SQL", "ML", "DevOps"],
    series_values=[
        [9, 7, 8, 8, 6],
        [5, 10, 6, 4, 9],
    ],
    series_names=["Alice", "Bob"],
)
```

---

## Voir aussi

- [Radar 2D](../2d/radar.md)

</div>
