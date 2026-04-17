# KNeighborsClassifier / KNeighborsRegressor / NearestCentroid

<div class="lang-en">

## Signature

```python
clf = sp.KNeighborsClassifier(n_neighbors: int = 5, weights: str = "uniform")
reg = sp.KNeighborsRegressor(n_neighbors: int = 5, weights: str = "uniform")
nc  = sp.NearestCentroid()

clf.classes_       -> list[int]
clf.n_neighbors_   -> int
clf.weights_       -> str

reg.n_neighbors_   -> int
reg.weights_       -> str

nc.classes_        -> list[int]
```

---

## Description

K-Nearest Neighbors with brute-force distance computation and partial sort for O(n) neighbor selection. Parallelized via Rayon when `n >= 256`.

`weights` options: `"uniform"` (all neighbors equal weight), `"distance"` (inverse distance weighting).

**NearestCentroid** classifies by computing the centroid of each class, then assigning the nearest.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_neighbors` | `int` | `5` | Number of neighbors |
| `weights` | `str` | `"uniform"` | `"uniform"` or `"distance"` |

</div>

<div class="lang-fr">

## Description

K plus proches voisins avec calcul brute-force et tri partiel pour la sélection en O(n). Parallélisé via Rayon lorsque `n >= 256`.

Options pour `weights` : `"uniform"` (poids égaux), `"distance"` (pondération inverse par la distance).

**NearestCentroid** classifie en calculant le centroïde de chaque classe puis en assignant au plus proche.

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_neighbors` | `int` | `5` | Nombre de voisins |
| `weights` | `str` | `"uniform"` | `"uniform"` ou `"distance"` |

</div>


---

## Constructor Parameters

### KNeighborsClassifier / KNeighborsRegressor

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_neighbors` | `int` | `5` | Number of neighbors |
| `weights` | `str` | `"uniform"` | `"uniform"` or `"distance"` (inverse distance weighting) |

### NearestCentroid

No parameters.

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels |

---

## Examples

<details>
<summary><strong>KNN classification</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
y = (X[:, 0] > 0).astype(np.int32)

clf = sp.KNeighborsClassifier(n_neighbors=7)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")

proba = clf.predict_proba(X[:3])
for row in proba:
    print(f"  {[f'{v:.3f}' for v in row]}")
```

</details>

<details>
<summary><strong>KNN regression</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 2)
y = np.sin(X[:, 0]) + X[:, 1]

reg = sp.KNeighborsRegressor(n_neighbors=5)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}")
```

</details>

<details>
<summary><strong>NearestCentroid</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.vstack([np.random.randn(100, 2) + [2, 2], np.random.randn(100, 2) + [-2, -2]])
y = np.array([0] * 100 + [1] * 100, dtype=np.int32)

nc = sp.NearestCentroid()
nc.fit(X, y)
print(f"Accuracy: {nc.score(X, y):.4f}")
```

</details>
