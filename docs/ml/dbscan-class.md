# DBSCAN Class

<div class="lang-en">

## API Reference

```python
model = sp.DBSCAN(eps=0.5, min_samples=5)

model.fit(X)                 -> None
model.fit_predict(X)         -> list[int]

model.labels_                -> list[int]
model.n_clusters_            -> int
model.n_noise_               -> int
```

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `eps` | `float` | `0.5` | Neighborhood radius threshold $\epsilon$ |
| `min_samples` | `int` | `5` | Minimum points to form a dense core |

## Methods

| Method | Signature | Returns | Description |
|--------|-----------|---------|-------------|
| `fit(X)` | `fit(X: list[list[float]])` | `None` | Fit DBSCAN on N-dimensional data, populates `labels_`, `n_clusters_`, `n_noise_` |
| `fit_predict(X)` | `fit_predict(X: list[list[float]])` | `list[int]` | Fit and return cluster labels (convenience wrapper) |

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `labels_` | `list[int]` | Cluster label per point (`-1` = noise) |
| `n_clusters_` | `int` | Number of identified clusters (noise not counted) |
| `n_noise_` | `int` | Count of noise points (label $-1$) |

## Example

```python
import seraplot as sp
import numpy as np

data = np.random.randn(100, 3)

model = sp.DBSCAN(eps=0.8, min_samples=5)
labels = model.fit_predict(data.tolist())

print(f"Clusters: {model.n_clusters_}, Noise: {model.n_noise_}")
print(f"Labels shape: {len(labels)}")

x, y, z = data[:, 0].tolist(), data[:, 1].tolist(), data[:, 2].tolist()
color_groups = [str(lbl) for lbl in labels]

chart = sp.build_dbscan_chart_3d(
    f"DBSCAN ({model.n_clusters_} clusters)",
    x, y, z,
    eps=0.8, min_samples=5,
    color_groups=color_groups
)
```

---

## Algorithmic Functioning

DBSCAN (**Density-Based Spatial Clustering of Applications with Noise**) groups points in dense regions and marks isolated points as noise.

**Core concepts** — for point $p$:

- **$\epsilon$-neighborhood:** $N_\epsilon(p) = \{q \in D : \|p - q\|_2 \leq \epsilon\}$
- **Core point:** $|N_\epsilon(p)| \geq \text{min\_samples}$
- **Border point:** not core, but within $\epsilon$ of a core point
- **Noise point:** not reachable from any core point → label $-1$

**Algorithm:**

1. For each unvisited point $p$:
   - If $p$ is core, start a new cluster via **BFS** (expand through density-connected neighbors)
   - Otherwise, mark as noise (or leave unvisited)

2. Clusters are maximal sets of density-connected points.

**Implementation:**

SeraPlot uses **KD-tree** for $O(\log n)$ radius queries and **parallel BFS** with SIMD distance acceleration. `n_clusters_` counts only true clusters; noise points are excluded.

**Complexity:** $O(n \log n)$ average time; $O(n^2)$ worst case.

---

---

</div>

<div class="lang-fr">

## Référence API

```python
model = sp.DBSCAN(eps=0.5, min_samples=5)

model.fit(X)                 -> None
model.fit_predict(X)         -> list[int]

model.labels_                -> list[int]
model.n_clusters_            -> int
model.n_noise_               -> int
```

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `eps` | `float` | `0.5` | Rayon de voisinage $\epsilon$ |
| `min_samples` | `int` | `5` | Points min pour former un cœur |

## Méthodes

| Méthode | Signature | Retourne | Description |
|---------|-----------|----------|-------------|
| `fit(X)` | `fit(X: list[list[float]])` | `None` | Ajuste DBSCAN, remplit `labels_`, `n_clusters_`, `n_noise_` |
| `fit_predict(X)` | `fit_predict(X: list[list[float]])` | `list[int]` | Ajuste et retourne labels |

## Attributs

| Attribut | Type | Description |
|----------|------|-------------|
| `labels_` | `list[int]` | Label de cluster par point (`-1` = bruit) |
| `n_clusters_` | `int` | Nombre de clusters (bruit non compté) |
| `n_noise_` | `int` | Nombre de points bruit (label $-1$) |

## Exemple

```python
import seraplot as sp
import numpy as np

data = np.random.randn(100, 3)

model = sp.DBSCAN(eps=0.8, min_samples=5)
labels = model.fit_predict(data.tolist())

print(f"Clusters: {model.n_clusters_}, Bruit: {model.n_noise_}")

x, y, z = data[:, 0].tolist(), data[:, 1].tolist(), data[:, 2].tolist()
color_groups = [str(lbl) for lbl in labels]

chart = sp.build_dbscan_chart_3d(
    f"DBSCAN ({model.n_clusters_} clusters)",
    x, y, z,
    eps=0.8, min_samples=5,
    color_groups=color_groups
)
```

---

## Fonctionnement algorithmique

DBSCAN groupe les points dans les **régions denses** et marque les points isolés comme bruit.

**Concepts clés** — pour un point $p$:

- **$\epsilon$-voisinage:** $N_\epsilon(p) = \{q \in D : \|p - q\|_2 \leq \epsilon\}$
- **Point cœur:** $|N_\epsilon(p)| \geq \text{min\_samples}$
- **Point frontière:** non cœur, mais dans $\epsilon$ d'un point cœur
- **Point bruit:** non accessible depuis aucun point cœur → label $-1$

**Algorithme:**

1. Pour chaque point $p$ non visité:
   - Si $p$ est cœur, démarrer cluster via **BFS**
   - Sinon, marquer comme bruit

2. Les clusters sont ensembles maximaux de points densément connexes.

**Implémentation:**

SeraPlot utilise **KD-tree** pour $O(\log n)$ requêtes de rayon et **BFS parallèle** avec accélération SIMD. `n_clusters_` ne compte que vrais clusters; bruit exclu.

**Complexité:** $O(n \log n)$ en moyenne; $O(n^2)$ pire cas.

---

---

</div>



