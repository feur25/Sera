# KMeans Class

<div class="lang-en">

## Signature

```python
model = sp.KMeans(
    k=3,
    max_iter=300,
    tol=1e-4,
    mini_batch=False,
    batch_size=1000,
    n_init=10,
)

model.fit(x: list[list[float]]) -> None
model.fit_predict(x: list[list[float]]) -> list[int]
model.predict(x: list[list[float]]) -> list[int]
model.transform(x: list[list[float]]) -> list[list[float]]

model.labels_     -> list[int]
model.centroids_  -> list[list[float]]
model.inertia_    -> float
model.n_iter_     -> int
model.n_clusters  -> int
```

---

## Description

High-performance K-Means class for N-dimensional data with a scikit-learn-compatible API.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `k` | `int` | `3` | Number of clusters |
| `max_iter` | `int` | `300` | Maximum EM iterations |
| `tol` | `float` | `1e-4` | Convergence tolerance on inertia delta |
| `mini_batch` | `bool` | `False` | Force mini-batch mode |
| `batch_size` | `int` | `1000` | Mini-batch sample size |
| `n_init` | `int` | `10` | Number of times to run with different seeds (best inertia is kept) |

---

## Methods

### `fit(x)`

Runs K-Means on the N-D data. Populates `labels_`, `centroids_`, and `inertia_`.

| Argument | Type | Description |
|----------|------|-------------|
| `x` | `list[list[float]]` | Data matrix (rows = samples, cols = features) |

### `fit_predict(x) -> list[int]`

Equivalent to `fit(x)` then returning `labels_`.

### `predict(x) -> list[int]`

Assign new samples to the nearest centroid (does not refit).

### `transform(x) -> list[list[float]]`

Return Euclidean distance from each sample to each centroid (shape: n_samples × k).

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `labels_` | `list[int]` | Cluster index per point (0-based) |
| `centroids_` | `list[list[float]]` | Final centroid coordinates (k × dims) |
| `inertia_` | `float` | Sum of squared distances to assigned centroids |
| `n_iter_` | `int` | Number of iterations run |
| `n_clusters` | `int` | Effective number of clusters found |
| `k` | `int` | Requested k |

---

## Examples

### Basic N-D clustering


<style>
.sp-tabs{border:1px solid var(--sp-border);border-radius:8px;overflow:hidden;margin:1.5em 0}
.sp-tab-btns{display:flex;background:var(--sp-surface);border-bottom:1px solid var(--sp-border)}
.sp-tb{padding:9px 22px;border:none;background:none;color:var(--sp-text-muted);cursor:pointer;font-size:13px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb.active{color:var(--sp-text);border-bottom-color:var(--sp-accent)}
.sp-tp{display:none;padding:18px 20px;background:var(--sp-surface)}
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
import random

random.seed(42)
centers = [(-2, -2, 0), (2, -2, 0), (0, 2, 1)]
data = [[cx + random.gauss(0, 0.4), cy + random.gauss(0, 0.4), cz + random.gauss(0, 0.3)]
        for cx, cy, cz in centers for _ in range(300)]

model = sp.KMeans(k=3)
labels = model.fit_predict(data)

print(f"Clusters: {model.n_clusters}")
print(f"Inertia: {model.inertia_:.2f}")
print(f"Centroids: {model.centroids_}")
```

</div>
</div>
<script>function spTab(btn,id){btn.closest('.sp-tabs').querySelectorAll('.sp-tb,.sp-tp').forEach(e=>e.classList.remove('active'));btn.classList.add('active');document.getElementById(id).classList.add('active');}</script>

---

### Combine class + chart


<div class="sp-tabs">
<div class="sp-tab-btns">
<button class="sp-tb active" onclick="spTab(this,'t2')">Python</button>
</div>
<div id="t2" class="sp-tp active">

```python
import seraplot as sp
import random

random.seed(0)
pts = [(random.gauss(cx, 0.3), random.gauss(cy, 0.3))
       for cx, cy in [(0,0),(3,0),(1.5,2.5)] for _ in range(500)]
x, y = zip(*pts)

model = sp.KMeans(k=3)
labels = model.fit_predict([[xi, yi] for xi, yi in zip(x, y)])

# Build chart with known labels
chart = sp.kmeans(
    title="K-Means Result",
    x_values=list(x),
    y_values=list(y),
    k=3,
)
chart.show()
print(f"Inertia: {model.inertia_:.4f}")
```

</div>
</div>

---

### Distance transform


<div class="sp-tabs">
<div class="sp-tab-btns">
<button class="sp-tb active" onclick="spTab(this,'t3')">Python</button>
</div>
<div id="t3" class="sp-tp active">

```python
import seraplot as sp

data = [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0], [0.0, 0.0]]

model = sp.KMeans(k=2)
model.fit(data)

distances = model.transform(data)
for i, row in enumerate(distances):
    print(f"Point {i}: distances to centroids = {[f'{d:.3f}' for d in row]}")
```

</div>
</div>

---

## Algorithmic Functioning

K-Means minimises the total inertia — the sum of squared distances from each point to
its assigned centroid:

<div>$$J = \sum_{i=1}^{n} \|x_i - \mu_{c(x_i)}\|^2$$</div>

**K-Means++ initialisation**

The first centroid $\mu_1$ is chosen uniformly at random. Each subsequent centroid
$\mu_j$ is sampled with probability proportional to $D(x)^2$ — the squared distance to
the nearest already-placed centroid. This reduces the expected inertia at convergence
to $O(\log k)$ of optimal.

**EM iterations**

1. **Assignment:** $c(x_i) = \underset{k}{\arg\min}\ \|x_i - \mu_k\|^2$
2. **Update:** $\mu_k = \dfrac{1}{|C_k|}\displaystyle\sum_{x_i \in C_k} x_i$

Iterations run until inertia delta $< $ `tol` or `max_iter` is reached.

**`transform(x)`** returns the $n \times k$ matrix of Euclidean distances from each
sample to each centroid, useful for soft-assignment and feature engineering.

</div>

<div class="lang-fr">

## Description

Classe K-Means haute performance pour données N-dimensionnelles, compatible avec l'API scikit-learn. Passe automatiquement en mode mini-batch pour `n > 100 000`.

## Constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `k` | `int` | `3` | Nombre de clusters |
| `max_iter` | `int` | `300` | Nombre maximum d'itérations |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `mini_batch` | `bool` | `False` | Forcer le mode mini-batch |
| `batch_size` | `int` | `1000` | Taille du mini-batch |

## Méthodes

| Méthode | Description |
|---------|-------------|
| `fit(x)` | Ajuste le modèle |
| `fit_predict(x)` | Ajuste et retourne les labels |
| `predict(x)` | Prédit les clusters |
| `transform(x)` | Distances aux centroïdes |

## Attributs

| Attribut | Description |
|---------|-------------|
| `labels_` | Labels par point |
| `centroids_` | Coordonnées des centroïdes |
| `inertia_` | Inertie finale |
| `n_iter_` | Nombre d'itérations |

---

## Fonctionnement algorithmique

K-Means minimise l'inertie totale — la somme des carrés des distances de chaque point
à son centroïde assigné :

<div>$$J = \sum_{i=1}^{n} \|x_i - \mu_{c(x_i)}\|^2$$</div>

**Initialisation K-Means++**

Le premier centroïde $\mu_1$ est choisi de façon uniforme aléatoire. Chaque centroïde
suivant $\mu_j$ est échantillonné avec une probabilité proportionnelle à $D(x)^2$ — la
distance au carré au centroïde le plus proche déjà placé. Cela réduit l'inertie attendue
à la convergence à $O(\log k)$ de l'optimal.

**Itérations EM**

1. **Affectation :** $c(x_i) = \underset{k}{\arg\min}\ \|x_i - \mu_k\|^2$
2. **Mise à jour :** $\mu_k = \dfrac{1}{|C_k|}\displaystyle\sum_{x_i \in C_k} x_i$

Les itérations tournent jusqu'à ce que le delta d'inertie passe sous `tol` ou que
`max_iter` soit atteint.

**`transform(x)`** retourne la matrice $n \times k$ des distances euclidiennes de chaque
échantillon à chaque centroïde, utile pour l'affectation douce et l'ingénierie de
caractéristiques.

</div>

