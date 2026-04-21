# Scatter 3D

<div class="lang-en">

## Signature

```python
sp.build_scatter3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    *,
    color_values: list[float] | None = None,
    color_labels: list[str] | None = None,
    series_names: list[str] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    hover_json: str | None = None,
    palette: list[int] | None = None,
) -> Chart
```

---

## Description

GPU-accelerated 3D scatter plot rendered via WebGL.
Handles millions of points at interactive frame rates.

Use `color_values` for a continuous color scale, or `color_labels` for categorical coloring.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | X coordinates |
| `y` | `list[float]` | required | Y coordinates |
| `z` | `list[float]` | required | Z coordinates |
| `color_values` | `list[float] \| None` | `None` | Continuous colormap values |
| `color_labels` | `list[str] \| None` | `None` | Categorical color group labels |
| `series_names` | `list[str] \| None` | `None` | Series legend names |
| `bg_color` | `str` | `"#1a1a2e"` | Canvas background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `x_label` | `str` | `"X"` | X-axis label |
| `y_label` | `str` | `"Y"` | Y-axis label |
| `z_label` | `str` | `"Z"` | Z-axis label |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |
| `palette` | `list[int] \| None` | `None` | Custom color palette |

---

## Returns

`Chart`

---

## Performance

The renderer uses a single `gl.drawArrays(POINTS, …)` call per frame.
Tested at 10 million points at 60 fps on a mid-range GPU.

---

## Examples

### 3D scatter with categorical colors





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
<div class="sp-tabs" id="scatter3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('scatter3d','scatter3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('scatter3d','scatter3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('scatter3d','scatter3d-ts',this)">TypeScript</button></div>
<div id="scatter3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
n = 1000
x = [random.gauss(0, 1) for _ in range(n)]
y = [random.gauss(0, 1) for _ in range(n)]
z = [random.gauss(0, 1) for _ in range(n)]
groups = [random.choice(["A", "B", "C"]) for _ in range(n)]
chart = sp.build_scatter3d_chart(
    "3D Point Cloud",
    x_values=x, y_values=y, z_values=z,
    color_labels=groups,
    x_label="X", y_label="Y", z_label="Z",
)</code></pre></div>
<div id="scatter3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import random
const n = 1000
const x = [random.gauss(0, 1) for _ in range(n)]
const y = [random.gauss(0, 1) for _ in range(n)]
const z = [random.gauss(0, 1) for _ in range(n)]
const groups = [random.choice(["A", "B", "C"]) for _ in range(n)]
const chart = sp.build_scatter3d_chart("3D Point Cloud",
x,
y,
{
    z_values: z,
    color_labels: groups,
    x_label: "X",
    y_label: "Y",
    z_label: "Z"
})</code></pre></div>
<div id="scatter3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import random
const n: number = 1000
const x: number[] = [random.gauss(0, 1) for _ in range(n)]
const y: number[] = [random.gauss(0, 1) for _ in range(n)]
const z: number[] = [random.gauss(0, 1) for _ in range(n)]
const groups: string[] = [random.choice(["A", "B", "C"]) for _ in range(n)]
const chart = sp.build_scatter3d_chart("3D Point Cloud",
x,
y,
{
    z_values: z,
    color_labels: groups,
    x_label: "X",
    y_label: "Y",
    z_label: "Z"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/scatter3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Scatter 2D](../2d/scatter.md)
- [DBSCAN 3D](../../ml/dbscan.md)
- [Bubble 3D](bubble3d.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_scatter3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    *,
    color_values: list[float] | None = None,
    color_labels: list[str] | None = None,
    series_names: list[str] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    hover_json: str | None = None,
    palette: list[int] | None = None,
) -> Chart
```

---

## Description

Nuage de points 3D accéléré GPU via WebGL. Gère des millions de points à des fréquences d'images interactives.

Utilisez `color_values` pour une échelle de couleur continue, ou `color_labels` pour un coloriage catégoriel.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x` | `list[float]` | requis | Coordonnées X |
| `y` | `list[float]` | requis | Coordonnées Y |
| `z` | `list[float]` | requis | Coordonnées Z |
| `color_values` | `list[float] \| None` | `None` | Valeurs de colormap continues |
| `color_labels` | `list[str] \| None` | `None` | Groupes de couleur catégoriels |
| `series_names` | `list[str] \| None` | `None` | Noms des séries pour la légende |
| `bg_color` | `str` | `"#1a1a2e"` | Couleur de fond |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `600` | Hauteur du canvas |
| `x_label` | `str` | `"X"` | Étiquette de l'axe X |
| `y_label` | `str` | `"Y"` | Étiquette de l'axe Y |
| `z_label` | `str` | `"Z"` | Étiquette de l'axe Z |
| `hover_json` | `str \| None` | `None` | JSON d'infobulle personnalisée |
| `palette` | `list[int] \| None` | `None` | Palette de couleurs |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp
import random

n = 1000
x = [random.gauss(0, 1) for _ in range(n)]
y = [random.gauss(0, 1) for _ in range(n)]
z = [random.gauss(0, 1) for _ in range(n)]
groupes = [random.choice(["A", "B", "C"]) for _ in range(n)]

chart = sp.build_scatter3d_chart(
    "Nuage de points 3D",
    x_values=x, y_values=y, z_values=z,
    color_labels=groupes,
    x_label="X", y_label="Y", z_label="Z",
)
```

---

## Voir aussi

- [Nuage de points 2D](../2d/scatter.md)
- [DBSCAN 3D](../../ml/dbscan.md)
- [Bulles 3D](bubble3d.md)

</div>
