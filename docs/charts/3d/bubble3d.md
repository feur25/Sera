# Bubble Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_bubble3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    sizes: list[float],
    *,
    color_labels: list[str] | None = None,
    color_values: list[float] | None = None,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    hover_json: str | None = None,
) -> Chart
```

---

## Description

3D bubble chart — scatter in XYZ space where bubble radius encodes a fourth dimension.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | X positions |
| `y` | `list[float]` | required | Y positions |
| `z` | `list[float]` | required | Z positions |
| `sizes` | `list[float]` | required | Bubble radii |
| `color_labels` | `list[str] \| None` | `None` | Categorical color groups |
| `color_values` | `list[float] \| None` | `None` | Continuous colormap values |
| `palette` | `list[int] \| None` | `None` | Custom color palette |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
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
<div class="sp-tabs" id="bubble3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('bubble3d','bubble3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('bubble3d','bubble3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('bubble3d','bubble3d-ts',this)">TypeScript</button></div>
<div id="bubble3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
n = 200
chart = sp.build_bubble3d_chart(
    "4D Dataset",
    x_values=[random.gauss(0,1) for _ in range(n)],
    y_values=[random.gauss(0,1) for _ in range(n)],
    z_values=[random.gauss(0,1) for _ in range(n)],
    size_values=[random.uniform(5, 30) for _ in range(n)],
    color_labels=[random.choice(["A","B","C"]) for _ in range(n)],
)</code></pre></div>
<div id="bubble3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import random
const n = 200
const chart = sp.build_bubble3d_chart("4D Dataset",
[random.gauss(0,1) for _ in range(n)],
[random.gauss(0,1) for _ in range(n)],
[random.gauss(0,1) for _ in range(n)],
{
    size_values: [random.uniform(5, 30) for _ in range(n)],
    color_labels: [random.choice(["A","B","C"]) for _ in range(n)]
})</code></pre></div>
<div id="bubble3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import random
const n: number = 200
const chart = sp.build_bubble3d_chart("4D Dataset",
[random.gauss(0,1) for _ in range(n)],
[random.gauss(0,1) for _ in range(n)],
[random.gauss(0,1) for _ in range(n)],
{
    size_values: [random.uniform(5, 30) for _ in range(n)],
    color_labels: [random.choice(["A","B","C"]) for _ in range(n)]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/bubble3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Scatter 3D](scatter3d.md)
- [Bubble 2D](../2d/bubble.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_bubble3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    sizes: list[float],
    *,
    color_labels: list[str] | None = None,
    color_values: list[float] | None = None,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Graphique à bulles 3D — nuage de points XYZ où le rayon des bulles encode une quatrième dimension.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x` | `list[float]` | requis | Positions X |
| `y` | `list[float]` | requis | Positions Y |
| `z` | `list[float]` | requis | Positions Z |
| `sizes` | `list[float]` | requis | Rayons des bulles |
| `color_labels` | `list[str] \| None` | `None` | Groupes de couleur catégoriels |
| `color_values` | `list[float] \| None` | `None` | Valeurs de colormap continues |
| `palette` | `list[int] \| None` | `None` | Palette de couleurs |
| `bg_color` | `str` | `"#1a1a2e"` | Couleur de fond |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `600` | Hauteur du canvas |
| `hover_json` | `str \| None` | `None` | JSON d'infobulle personnalisée |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp
import random

n = 200
chart = sp.build_bubble3d_chart(
    "Jeu de données 4D",
    x_values=[random.gauss(0,1) for _ in range(n)],
    y_values=[random.gauss(0,1) for _ in range(n)],
    z_values=[random.gauss(0,1) for _ in range(n)],
    size_values=[random.uniform(5, 30) for _ in range(n)],
    color_labels=[random.choice(["A","B","C"]) for _ in range(n)],
)
```

---

## Voir aussi

- [Nuage de points 3D](scatter3d.md)
- [Bulles 2D](../2d/bubble.md)

</div>
