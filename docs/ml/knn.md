# KNeighborsClassifier / KNeighborsRegressor / NearestCentroid

## Signature

```python
clf = sp.KNeighborsClassifier(n_neighbors: int = 5, weights: str = "uniform")
reg = sp.KNeighborsRegressor(n_neighbors: int = 5, weights: str = "uniform")
nc  = sp.NearestCentroid()

model.fit(x, y)
model.predict(x) -> list[int] | list[float]
model.score(x, y) -> float

clf.predict_proba(x) -> ndarray (n, n_classes)
clf.classes_             -> list[int]
nc.classes_              -> list[int]
```

---

## Description

K-Nearest Neighbors with brute-force distance computation and partial sort (`select_nth_unstable`) for O(n) neighbor selection. Prediction is parallelized via Rayon when `n >= 256`. Uses thread-local buffers for zero-allocation queries.

**NearestCentroid** computes the centroid of each class and classifies by nearest centroid.

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
