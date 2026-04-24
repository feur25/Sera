# DBSCAN Class

<div class="lang-en">

## Signature

```python
model = sp.DBSCAN(eps: float = 0.5, min_samples: int = 5)

model.fit(x: list[float], y: list[float]) -> None
model.fit_predict(x: list[float], y: list[float]) -> list[int]

model.labels_      -> list[int]
model.n_clusters_  -> int
model.n_noise_     -> int
```

---

## Description

Low-level DBSCAN class for programmatic access to cluster labels.
`-1` labels indicate noise points (not part of any cluster).

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `eps` | `float` | `0.5` | Neighborhood distance threshold |
| `min_samples` | `int` | `5` | Minimum points to form a cluster core |

---

## Methods

### `fit(x, y)`

Runs DBSCAN on the 2D data. Populates `labels_`, `n_clusters_`, and `n_noise_`.

| Argument | Type | Description |
|----------|------|-------------|
| `x` | `list[float]` | X coordinates |
| `y` | `list[float]` | Y coordinates |

### `fit_predict(x, y) -> list[int]`

Equivalent to calling `fit(x, y)` then returning `labels_`.

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `labels_` | `list[int]` | Cluster label per point (`-1` = noise) |
| `n_clusters_` | `int` | Number of identified clusters |
| `n_noise_` | `int` | Number of noise points |

---

## Examples

### Accessing labels





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
<div class="sp-tabs" id="dbscan-class">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('dbscan-class','dbscan-class-py',this)">Python</button><button class="sp-tb" onclick="spTab('dbscan-class','dbscan-class-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('dbscan-class','dbscan-class-ts',this)">TypeScript</button></div>
<div id="dbscan-class-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">from seraplot import DBSCAN
import seraplot as sp
x = [1.0, 1.1, 1.2, 10.0, 10.1, 99.0]
y = [1.0, 0.9, 1.1, 10.2, 10.0, 99.0]
xy = [[xi, yi] for xi, yi in zip(x, y)]
model = DBSCAN(eps=0.5, min_samples=2)
labels = model.fit_predict(xy)
chart = sp.build_scatter_chart(
    f"DBSCAN ({model.n_clusters_} clusters)",
    x_values=x,
    y_values=y,
    color_groups=[str(lbl) for lbl in labels],
)</code></pre></div>
<div id="dbscan-class-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">from seraplot import DBSCAN
const sp = require('seraplot');
const x = [1.0, 1.1, 1.2, 10.0, 10.1, 99.0]
const y = [1.0, 0.9, 1.1, 10.2, 10.0, 99.0]
const xy = [[xi, yi] for xi, yi in zip(x, y)]
const model = DBSCAN({eps: 0.5, min_samples: 2})
const labels = model.fit_predict(xy)
const chart = sp.build_scatter_chart(f"DBSCAN ({model.n_clusters_} clusters)",
x,
{
    y_values: y,
    color_groups: [str(lbl) for lbl in labels]
})</code></pre></div>
<div id="dbscan-class-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">from seraplot import DBSCAN
import * as sp from 'seraplot';
const x: number[] = [1.0, 1.1, 1.2, 10.0, 10.1, 99.0]
const y: number[] = [1.0, 0.9, 1.1, 10.2, 10.0, 99.0]
const xy: number[] = [[xi, yi] for xi, yi in zip(x, y)]
const model = DBSCAN({eps: 0.5, min_samples: 2})
const labels = model.fit_predict(xy)
const chart = sp.build_scatter_chart(f"DBSCAN ({model.n_clusters_} clusters)",
x,
{
    y_values: y,
    color_groups: [str(lbl) for lbl in labels]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../previews/dbscan-class.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

### Pipeline: cluster then visualize

```python
import seraplot as sp

model = sp.DBSCAN(eps=1.0, min_samples=5)
model.fit(x_data, y_data)

color_groups = [str(lbl) for lbl in model.labels_]

chart = sp.build_scatter_chart(
    f"DBSCAN ({model.n_clusters_} clusters)",
    x_values=x_data,
    y_values=y_data,
    color_groups=color_groups,
)
```

---

## Algorithmic Functioning

The DBSCAN class exposes the same Rust-backed algorithm as the chart variant.

For a point $p$, its $\epsilon$-neighbourhood is:

<div>$$N_\epsilon(p) = \{q \in D : \|p - q\| \leq \epsilon\}$$</div>

- **Core point:** $|N_\epsilon(p)| \geq \text{min\_samples}$
- **Border point:** reachable from a core point but not itself a core point
- **Noise point:** not reachable from any core point — label $-1$

SeraPlot builds a **KD-tree** for $O(\log n)$ radius queries and expands clusters via
parallel BFS with SIMD distance acceleration. `n_clusters_` counts only true clusters;
noise points are excluded.

---

## See also

- [DBSCAN Chart](dbscan.md)
- [DBSCAN 3D](dbscan3d.md)
- [Scatter Chart](../charts/2d/scatter.md)

</div>

<div class="lang-fr">

## Description

Classe DBSCAN bas niveau pour un accès programmatique aux labels de cluster. Les points bruit ont le label `-1`.

## Constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `eps` | `float` | `0.5` | Distance maximale de voisinage |
| `min_samples` | `int` | `5` | Nombre minimum de points pour une région dense |

## Méthodes

| Méthode | Description |
|---------|-------------|
| `fit(x, y)` | Ajuste le modèle |
| `fit_predict(x, y)` | Ajuste et retourne les labels |

## Attributs

| Attribut | Description |
|---------|-------------|
| `labels_` | Liste des labels par point (−1 = bruit) |
| `n_clusters_` | Nombre de clusters trouvés |
| `n_noise_` | Nombre de points bruit |

---

## Fonctionnement algorithmique

La classe DBSCAN expose le même algorithme Rust que la variante graphique.

Pour un point $p$, son $\epsilon$-voisinage est :

<div>$$N_\epsilon(p) = \{q \in D : \|p - q\| \leq \epsilon\}$$</div>

- **Point cœur :** $|N_\epsilon(p)| \geq \text{min\_samples}$
- **Point frontière :** accessible depuis un point cœur, mais pas lui-même un point cœur
- **Point bruit :** non accessible depuis aucun point cœur — label $-1$

SeraPlot construit un **KD-tree** pour des requêtes de rayon en $O(\log n)$ et étend les
clusters par BFS parallèle avec accélération SIMD. `n_clusters_` ne compte que les vrais
clusters ; les points bruit en sont exclus.

</div>
