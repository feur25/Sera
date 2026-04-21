# Heatmap 3D

<div class="lang-en">

## Signature

```python
sp.build_heatmap3d_chart(
    title: str,
    labels: list[str],
    flat_matrix: list[float],
    *,
    col_labels: list[str] | None = None,
    color_low: int = 0,
    color_high: int = 0,
    extrusion_scale: float = 1.0,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "",
) -> Chart
```

---

## Description

3D heatmap where cell values are extruded as bars rising from a flat grid.
Higher values produce taller columns.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Row labels |
| `flat_matrix` | `list[float]` | required | Matrix values, row-major |
| `col_labels` | `list[str] \| None` | `None` | Column labels |
| `color_low` | `int` | auto | Low value color |
| `color_high` | `int` | auto | High value color |
| `extrusion_scale` | `float` | `1.0` | Height multiplier for bars |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |

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
<div class="sp-tabs" id="heatmap3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('heatmap3d','heatmap3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('heatmap3d','heatmap3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('heatmap3d','heatmap3d-ts',this)">TypeScript</button></div>
<div id="heatmap3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
features = ["A", "B", "C", "D"]
n = len(features)
matrix = [[abs(i - j) * 0.25 for j in range(n)] for i in range(n)]
chart = sp.build_heatmap3d_chart(
    "Distance Matrix 3D",
    x_labels=features,
    y_labels=features,
    values=matrix,
)</code></pre></div>
<div id="heatmap3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const features = ["A", "B", "C", "D"]
const n = len(features)
const matrix = [[abs(i - j) * 0.25 for j in range(n)] for i in range(n)]
const chart = sp.build_heatmap3d_chart("Distance Matrix 3D",
features,
features,
{
    values: matrix
})</code></pre></div>
<div id="heatmap3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const features: string[] = ["A", "B", "C", "D"]
const n = len(features)
const matrix: number[] = [[abs(i - j) * 0.25 for j in range(n)] for i in range(n)]
const chart = sp.build_heatmap3d_chart("Distance Matrix 3D",
features,
features,
{
    values: matrix
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/heatmap3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Heatmap 2D](../2d/heatmap.md)
- [Bar 3D](bar3d.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_heatmap3d_chart(
    title: str,
    labels: list[str],
    flat_matrix: list[float],
    *,
    col_labels: list[str] | None = None,
    color_low: int = 0,
    color_high: int = 0,
    extrusion_scale: float = 1.0,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "",
) -> Chart
```

---

## Description

Heatmap 3D où les valeurs sont extrudées comme des barres s'élevant d'une grille plate. Les valeurs plus hautes produisent des colonnes plus élevées.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des lignes |
| `flat_matrix` | `list[float]` | requis | Valeurs de la matrice, en ligne-major |
| `col_labels` | `list[str] \| None` | `None` | Étiquettes de colonnes |
| `color_low` | `int` | auto | Couleur pour les valeurs basses |
| `color_high` | `int` | auto | Couleur pour les valeurs hautes |
| `extrusion_scale` | `float` | `1.0` | Multiplicateur de hauteur des barres |
| `bg_color` | `str` | `"#1a1a2e"` | Couleur de fond |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `600` | Hauteur du canvas |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp

features = ["A", "B", "C", "D"]
n = len(features)
matrice = [[abs(i - j) * 0.25 for j in range(n)] for i in range(n)]
flat = [v for row in matrice for v in row]

chart = sp.build_heatmap3d_chart(
    "Matrice de distance 3D",
    labels=features,
    flat_matrix=flat,
    col_labels=features,
)
```

---

## Voir aussi

- [Heatmap 2D](../2d/heatmap.md)
- [Barres 3D](bar3d.md)

</div>
