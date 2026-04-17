# K-Means Chart

<div class="lang-en">

## Signature

```python
sp.build_kmeans_chart(
    title: str,
    x_values: list[float],
    y_values: list[float],
    *,
    k: int = 3,
    max_iter: int = 300,
    tol: float = 1e-4,
    mini_batch: bool = False,
    batch_size: int = 1000,
    width: int = 1000,
    height: int = 580,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    palette: list[int] | None = None,
    background: str | None = None,
) -> Chart
```

Alias: `sp.kmeans(...)`

---

## Description

2D K-Means clustering chart. Runs K-Means++ initialization followed by parallel centroid assignment and converges in typically < 20 iterations. Each cluster is displayed in a distinct color with its centroid  shown as a bold `+` marker.

SeraPlot's K-Means runs **thousands× faster** than scikit-learn on large datasets thanks to:
- **K-Means++** seeding for fast convergence (O(k·n) deterministic-quality init)
- **Parallel assignment** — scoped threads over CPU-affine chunks (zero-copy)
- **Mini-batch** — automatic switch for n > 100 000, or set `mini_batch=True`
- **SIMD-friendly** distance — 4-way unrolled inner loop autovectorized by LLVM

Inertia (sum of squared distances to centroids) is displayed in the chart corner.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x_values` | `list[float]` | required | X coordinates |
| `y_values` | `list[float]` | required | Y coordinates |
| `k` | `int` | `3` | Number of clusters |
| `max_iter` | `int` | `300` | Maximum number of EM iterations |
| `tol` | `float` | `1e-4` | Convergence tolerance on inertia delta |
| `mini_batch` | `bool` | `False` | Force mini-batch mode (auto for n > 100 000) |
| `batch_size` | `int` | `1000` | Mini-batch size |
| `width` | `int` | `1000` | Canvas width in pixels |
| `height` | `int` | `580` | Canvas height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Show gridlines |
| `palette` | `list[int] \| None` | `None` | Custom cluster colors (hex int list) |
| `background` | `str \| None` | `None` | Chart background color |

---

## Returns

`Chart`

---

## Examples

### Basic usage


<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155}
.sp-tb{padding:9px 22px;border:none;background:none;color:#64748b;cursor:pointer;font-size:13px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb.active{color:#f8fafc;border-bottom-color:#6366f1}
.sp-tp{display:none;padding:18px 20px;background:#0b0e18}
.sp-tp.active{display:block}
pre{margin:0;background:transparent!important}
</style>
<div class="sp-tabs">
<div class="sp-tab-btns">
<button class="sp-tb active" onclick="spTab(this,'t1')">Python</button>
</div>
<div id="t1" class="sp-tp active">

```python
import seraplot as sp
import random, math

random.seed(42)
centers = [(-2, -2), (2, -2), (0, 2)]
pts = [(cx + random.gauss(0, 0.4), cy + random.gauss(0, 0.4))
       for cx, cy in centers for _ in range(400)]
x, y = zip(*pts)

chart = sp.kmeans(
    title="K-Means Clustering",
    x_values=list(x),
    y_values=list(y),
    k=3,
    x_label="Feature 1",
    y_label="Feature 2",
)
chart.show()
```

</div>
</div>
<script>function spTab(btn,id){btn.closest('.sp-tabs').querySelectorAll('.sp-tb,.sp-tp').forEach(e=>e.classList.remove('active'));btn.classList.add('active');document.getElementById(id).classList.add('active');}</script>

---

### Large dataset (mini-batch)


<div class="sp-tabs">
<div class="sp-tab-btns">
<button class="sp-tb active" onclick="spTab(this,'t2')">Python</button>
</div>
<div id="t2" class="sp-tp active">

```python
import seraplot as sp
import random

random.seed(0)
x = [random.gauss(i % 5, 0.3) for i in range(500_000)]
y = [random.gauss(i % 5, 0.3) for i in range(500_000)]

# mini_batch activates automatically for n > 100 000
chart = sp.kmeans(
    title="Large Dataset K-Means",
    x_values=x,
    y_values=y,
    k=5,
    mini_batch=True,
    batch_size=2000,
)
chart.show()
```

</div>
</div>

</div>

<div class="lang-fr">

## Description

Graphique de clustering K-Means en 2D. Exécute l'initialisation K-Means++ suivie d'une assignation parallèle des centroïdes. Chaque cluster est affiché dans une couleur distincte avec son centroïde marqué par un `+` gras.

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x_values` | `list[float]` | requis | Coordonnées X |
| `y_values` | `list[float]` | requis | Coordonnées Y |
| `k` | `int` | `3` | Nombre de clusters |
| `max_iter` | `int` | `300` | Nombre maximum d'itérations EM |
| `mini_batch` | `bool` | `False` | Forcer le mode mini-batch |

## Exemple

```python
import seraplot as sp
import numpy as np

rng = np.random.default_rng(42)
centres = [(-2, -2), (2, -2), (0, 2)]
pts = [(cx + rng.normal(0, 0.4), cy + rng.normal(0, 0.4))
       for cx, cy in centres for _ in range(400)]
x, y = zip(*pts)

chart = sp.kmeans("K-Means", list(x), list(y), k=3)
chart.show()
```

</div>

