# Heatmap

<div class="lang-en">

## Signature

```python
sp.build_heatmap(
    title: str,
    labels: list[str],
    flat_matrix: list[float],
    *,
    show_values: bool = True,
    color_low: int = 0,
    color_mid: int = 0,
    color_high: int = 0,
    col_labels: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.heatmap`

---

## Description

Color-coded matrix heatmap. Values are automatically normalized for color mapping.

`flat_matrix` must contain `n_rows × n_cols` values in row-major order.

`labels` = row labels. `col_labels` = column labels (if different from rows).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Row labels |
| `flat_matrix` | `list[float]` | required | Matrix values, row-major |
| `show_values` | `bool` | `True` | Overlay numeric values in cells |
| `color_low` | `int` | auto | Low value color (hex int) |
| `color_mid` | `int` | auto | Mid value color |
| `color_high` | `int` | auto | High value color |
| `col_labels` | `list[str] \| None` | `None` | Column labels (defaults to `labels`) |

---

## Returns

`Chart`

---

## Examples

### Correlation matrix






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
<div class="sp-tabs" id="heatmap">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('heatmap','heatmap-py',this)">Python</button><button class="sp-tb" onclick="spTab('heatmap','heatmap-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('heatmap','heatmap-ts',this)">TypeScript</button></div>
<div id="heatmap-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import numpy as np
features = ["Age", "Income", "Score", "Visits"]
n = len(features)
matrix = np.corrcoef(np.random.randn(4, 100)).flatten().tolist()
chart = sp.build_heatmap(
    "Feature Correlation Matrix",
    labels=features,
    flat_matrix=matrix,
    color_low=0x3b82f6,
    color_mid=0xffffff,
    color_high=0xef4444,
    show_values=True,
)</code></pre></div>
<div id="heatmap-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const features = ["Age", "Income", "Score", "Visits"];
// Pre-computed 4×4 correlation matrix (row-major)
const matrix = [
   1.00,  0.72, -0.15,  0.43,
   0.72,  1.00,  0.08,  0.61,
  -0.15,  0.08,  1.00, -0.27,
   0.43,  0.61, -0.27,  1.00,
];
const chart = sp.build_heatmap("Feature Correlation Matrix", features, {
    flat_matrix: matrix,
    color_low: 0x3b82f6,
    color_mid: 0xffffff,
    color_high: 0xef4444,
    show_values: true,
});</code></pre></div>
<div id="heatmap-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const features: string[] = ["Age", "Income", "Score", "Visits"];
// Pre-computed 4×4 correlation matrix (row-major)
const matrix: number[] = [
   1.00,  0.72, -0.15,  0.43,
   0.72,  1.00,  0.08,  0.61,
  -0.15,  0.08,  1.00, -0.27,
   0.43,  0.61, -0.27,  1.00,
];
const chart = sp.build_heatmap("Feature Correlation Matrix", features, {
    flat_matrix: matrix,
    color_low: 0x3b82f6,
    color_mid: 0xffffff,
    color_high: 0xef4444,
    show_values: true,
});</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/heatmap.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Heatmap 3D](../3d/heatmap3d.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_heatmap(
    title: str,
    labels: list[str],
    flat_matrix: list[float],
    *,
    show_values: bool = True,
    color_low: int = 0,
    color_mid: int = 0,
    color_high: int = 0,
    col_labels: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.heatmap`

---

## Description

Matrice colorée (heatmap). Les valeurs sont normalisées automatiquement pour le mappage de couleurs.

`flat_matrix` doit contenir `n_lignes × n_colonnes` valeurs en ordre ligne-major. `labels` = étiquettes de lignes. `col_labels` = étiquettes de colonnes.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des lignes |
| `flat_matrix` | `list[float]` | requis | Valeurs de la matrice, en ligne-major |
| `show_values` | `bool` | `True` | Afficher les valeurs numériques dans les cellules |
| `color_low` | `int` | auto | Couleur pour les valeurs basses (hex int) |
| `color_mid` | `int` | auto | Couleur pour les valeurs médianes |
| `color_high` | `int` | auto | Couleur pour les valeurs hautes |
| `col_labels` | `list[str] \| None` | `None` | Étiquettes de colonnes (par défaut = `labels`) |

---

## Retourne

`Chart`

---

## Exemples

### Matrice de corrélation

```python
import seraplot as sp
import numpy as np

features = ["Age", "Revenu", "Score", "Visites"]
matrice = np.corrcoef(np.random.randn(4, 100)).flatten().tolist()

chart = sp.build_heatmap(
    "Matrice de corrélation des variables",
    labels=features,
    flat_matrix=matrice,
    color_low=0x3b82f6,
    color_mid=0xffffff,
    color_high=0xef4444,
    show_values=True,
)
```

---

## Voir aussi

- [Carte de chaleur 3D](../3d/heatmap3d.md)

</div>
