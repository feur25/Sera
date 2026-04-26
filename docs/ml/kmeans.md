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

Aliases: `sp.kmeans`, `sp.kmeans_chart`

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

## Algorithmic Functioning

K-Means minimises the total inertia — the sum of squared distances from each point to
its assigned centroid:

<div>$$J = \sum_{i=1}^{n} \|x_i - \mu_{c(x_i)}\|^2$$</div>

**K-Means++ initialisation**

The first centroid $\mu_1$ is chosen uniformly at random. Each subsequent centroid
$\mu_j$ is sampled with probability proportional to $D(x)^2$ — the squared distance to
the nearest already-placed centroid. This seeding strategy reduces the expected inertia
at convergence to $O(\log k)$ of optimal.

**EM iterations**

1. **Assignment:** $c(x_i) = \underset{k}{\arg\min}\ \|x_i - \mu_k\|^2$
2. **Update:** $\mu_k = \dfrac{1}{|C_k|}\displaystyle\sum_{x_i \in C_k} x_i$

Iterations run until the inertia delta falls below `tol` or `max_iter` is reached.

**Mini-batch**

At each step a random subset of `batch_size` points updates the centroids. This reduces
memory pressure and runtime for large datasets ($n > 100\,000$) at a small quality cost.

</div>

<div class="lang-fr">

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

Alias : `sp.kmeans`, `sp.kmeans_chart`

---
## Description

Graphique de clustering K-Means en 2D. Exécute l'initialisation K-Means++ suivie d'une
assignation parallèle des centroïdes et converge généralement en moins de 20 itérations.
Chaque cluster est affiché dans une couleur distincte avec son centroïde marqué par un
`+` en gras.

Le K-Means de SeraPlot tourne **des milliers de fois plus vite** que scikit-learn sur
de grands jeux de données grâce à :
- **K-Means++** pour une convergence rapide (init de qualité déterministe en O(k·n))
- **Assignation parallèle** — threads scopés sur chunks affines au CPU (zéro copie)
- **Mini-batch** — bascule automatique pour n > 100 000, ou via `mini_batch=True`
- **Distance SIMD-friendly** — boucle interne déroulée 4× autovectorisée par LLVM

L'inertie (somme des distances au carré aux centroïdes) est affichée dans le coin du graphique.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x_values` | `list[float]` | requis | Coordonnées X |
| `y_values` | `list[float]` | requis | Coordonnées Y |
| `k` | `int` | `3` | Nombre de clusters |
| `max_iter` | `int` | `300` | Nombre maximum d'itérations EM |
| `tol` | `float` | `1e-4` | Tolérance de convergence sur le delta d'inertie |
| `mini_batch` | `bool` | `False` | Forcer le mode mini-batch (auto pour n > 100 000) |
| `batch_size` | `int` | `1000` | Taille des mini-batchs |
| `width` | `int` | `1000` | Largeur du canevas en pixels |
| `height` | `int` | `580` | Hauteur du canevas en pixels |
| `x_label` | `str` | `""` | Étiquette de l'axe X |
| `y_label` | `str` | `""` | Étiquette de l'axe Y |
| `gridlines` | `bool` | `True` | Afficher les lignes de grille |
| `palette` | `list[int] \| None` | `None` | Couleurs personnalisées des clusters |
| `background` | `str \| None` | `None` | Couleur de fond du graphique |

---

## Retourne

`Chart`

---

### Usage basique

```python
import seraplot as sp
import random

random.seed(42)
centres = [(-2, -2), (2, -2), (0, 2)]
pts = [(cx + random.gauss(0, 0.4), cy + random.gauss(0, 0.4))
       for cx, cy in centres for _ in range(400)]
x, y = zip(*pts)

chart = sp.kmeans(
    title="Clustering K-Means",
    x_values=list(x),
    y_values=list(y),
    k=3,
    x_label="Variable 1",
    y_label="Variable 2",
)
chart.show()
```
---

## Fonctionnement algorithmique

K-Means minimise l'inertie totale — la somme des carrés des distances de chaque point
à son centroïde assigné :

<div>$$J = \sum_{i=1}^{n} \|x_i - \mu_{c(x_i)}\|^2$$</div>

**Initialisation K-Means++**

Le premier centroïde $\mu_1$ est choisi de façon uniforme aléatoire. Chaque centroïde
suivant $\mu_j$ est échantillonné avec une probabilité proportionnelle à $D(x)^2$ — la
distance au carré au centroïde le plus proche déjà placé. Cette stratégie d'amorçage
réduit l'inertie attendue à la convergence à $O(\log k)$ de l'optimal.

**Itérations EM**

1. **Affectation :** $c(x_i) = \underset{k}{\arg\min}\ \|x_i - \mu_k\|^2$
2. **Mise à jour :** $\mu_k = \dfrac{1}{|C_k|}\displaystyle\sum_{x_i \in C_k} x_i$

Les itérations tournent jusqu'à ce que le delta d'inertie passe sous `tol` ou que
`max_iter` soit atteint.

**Mini-batch**

À chaque étape, un sous-ensemble aléatoire de `batch_size` points met à jour les
centroïdes. Cela réduit la pression mémoire et le temps d'exécution pour les grands
jeux de données ($n > 100\,000$) au prix d'une légère perte de qualité.

</div>
