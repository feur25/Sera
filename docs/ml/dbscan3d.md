# DBSCAN 3D Chart

<div class="lang-en">

## Signature

```python
sp.build_dbscan_chart_3d(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    *,
    eps: float = 0.5,
    min_samples: int = 5,
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    bg_color: str = "#1a1a2e",
    normalize: bool = False,
    palette: list[int] | None = None,
) -> Chart
```

Aliases: `sp.dbscan3d`

---

## Description

DBSCAN clustering in 3D — rendered via GPU WebGL.
Each cluster is assigned a distinct color; noise points are grey.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | X coordinates |
| `y` | `list[float]` | required | Y coordinates |
| `z` | `list[float]` | required | Z coordinates |
| `eps` | `float` | `0.5` | Neighborhood radius |
| `min_samples` | `int` | `5` | Core point threshold |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `x_label` | `str` | `"X"` | X-axis label |
| `y_label` | `str` | `"Y"` | Y-axis label |
| `z_label` | `str` | `"Z"` | Z-axis label |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `normalize` | `bool` | `False` | Normalize XYZ to [0, 1] |
| `palette` | `list[int] \| None` | `None` | Custom cluster colors |

---

## Returns

`Chart`

---

## Examples

### 3D clusters





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
<div class="sp-tabs" id="dbscan3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('dbscan3d','dbscan3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('dbscan3d','dbscan3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('dbscan3d','dbscan3d-ts',this)">TypeScript</button></div>
<div id="dbscan3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
def blob3d(cx, cy, cz, n=200, s=0.4):
    return [(cx+random.gauss(0,s), cy+random.gauss(0,s), cz+random.gauss(0,s))
            for _ in range(n)]
pts = blob3d(0,0,0) + blob3d(5,5,5) + blob3d(10,0,5)
x, y, z = zip(*pts)
chart = sp.build_dbscan_chart_3d(
    "3D DBSCAN",
    x_values=list(x), y_values=list(y), z_values=list(z),
    eps=1.2,
    min_samples=5,
)</code></pre></div>
<div id="dbscan3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import random
def blob3d(cx, cy, cz, {n: 200, s: 0.4}):
    return [(cx+random.gauss(0,s), cy+random.gauss(0,s), cz+random.gauss(0,s))
            for _ in range(n)]
const pts = blob3d(0,0,0) + blob3d(5,5,5) + blob3d(10,0,5)
x, y, z = zip(*pts)
const chart = sp.build_dbscan_chart_3d("3D DBSCAN",
list(x),
list(y),
{
    z_values: list(z),
    eps: 1.2,
    min_samples: 5
})</code></pre></div>
<div id="dbscan3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import random
def blob3d(cx, cy, cz, {n: 200, s: 0.4}):
    return [(cx+random.gauss(0,s), cy+random.gauss(0,s), cz+random.gauss(0,s))
            for _ in range(n)]
const pts = blob3d(0,0,0) + blob3d(5,5,5) + blob3d(10,0,5)
x, y, z = zip(*pts)
const chart = sp.build_dbscan_chart_3d("3D DBSCAN",
list(x),
list(y),
{
    z_values: list(z),
    eps: 1.2,
    min_samples: 5
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../previews/dbscan3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## Algorithmic Functioning

DBSCAN groups points that lie in **dense regions** and marks isolated points as noise.
It requires no prior specification of the number of clusters.

For a point $p$, its $\epsilon$-neighbourhood is:

<div>$$N_\epsilon(p) = \{q \in D : \|p - q\| \leq \epsilon\}$$</div>

- **Core point:** $|N_\epsilon(p)| \geq \text{min\_samples}$
- **Border point:** reachable from a core point but not itself a core point
- **Noise point:** not reachable from any core point — assigned label $-1$

The 3D variant operates identically in $\mathbb{R}^3$ — the KD-tree extends to three
dimensions with SIMD-accelerated Euclidean distance $\|p - q\| = \sqrt{\Delta x^2 + \Delta y^2 + \Delta z^2}$.

When `normalize=True`, each axis is scaled to $[0, 1]$ independently before clustering,
so that the scale of $z$ does not distort $\epsilon$.

---

## See also

- [DBSCAN 2D](dbscan.md)
- [DBSCAN Class](dbscan-class.md)
- [Scatter 3D](../charts/3d/scatter3d.md)

</div>

<div class="lang-fr">

## Description

Clustering DBSCAN en 3D — rendu via WebGL GPU. Chaque cluster est coloré distinctement ; les points bruit sont gris.

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x` | `list[float]` | requis | Coordonnées X |
| `y` | `list[float]` | requis | Coordonnées Y |
| `z` | `list[float]` | requis | Coordonnées Z |
| `eps` | `float` | `0.5` | Distance maximale de voisinage |
| `min_samples` | `int` | `5` | Minimum de points pour une région dense |
| `normalize` | `bool` | `False` | Normaliser les variables avant le clustering |

---

## Fonctionnement algorithmique

DBSCAN regroupe les points situés dans des **régions denses** et marque les points
isolés comme du bruit. Il ne nécessite pas de spécifier le nombre de clusters à
l'avance.

Pour un point $p$, son $\epsilon$-voisinage est :

<div>$$N_\epsilon(p) = \{q \in D : \|p - q\| \leq \epsilon\}$$</div>

- **Point cœur :** $|N_\epsilon(p)| \geq \text{min\_samples}$
- **Point frontière :** accessible depuis un point cœur, mais pas lui-même un point cœur
- **Point bruit :** non accessible depuis aucun point cœur — label $-1$

La variante 3D fonctionne identiquement dans $\mathbb{R}^3$ — le KD-tree s'étend à
trois dimensions avec un calcul de distance euclidienne $\|p - q\| = \sqrt{\Delta x^2 + \Delta y^2 + \Delta z^2}$ accéléré par SIMD.

Avec `normalize=True`, chaque axe est normalisé dans $[0, 1]$ indépendamment avant le
clustering, de façon à ce que l'échelle de $z$ ne distorde pas $\epsilon$.

</div>
