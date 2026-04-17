# RandomForestClassifier / RandomForestRegressor

<div class="lang-en">

## Signature

```python
clf = sp.RandomForestClassifier(
    n_estimators: int = 100,
    max_depth: int = 10,
    min_samples_split: int = 2,
    min_samples_leaf: int = 1,
    max_features: str = "sqrt",
)

reg = sp.RandomForestRegressor(
    n_estimators: int = 100,
    max_depth: int = 10,
    min_samples_split: int = 2,
    min_samples_leaf: int = 1,
    max_features: str = "sqrt",
)

clf.classes_             -> list[int]
clf.feature_importances_ -> list[float]
clf.n_estimators_        -> int
clf.max_depth_           -> int
clf.min_samples_split_   -> int
clf.min_samples_leaf_    -> int
clf.max_features_        -> str

reg.feature_importances_ -> list[float]
reg.n_estimators_        -> int
reg.max_depth_           -> int
reg.min_samples_split_   -> int
reg.min_samples_leaf_    -> int
reg.max_features_        -> str
```

---

## Description

Ensemble of decision trees trained on bootstrapped samples. Each tree uses a random subset of features for splits. Parallelized with Rayon.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `100` | Number of trees |
| `max_depth` | `int` | `10` | Maximum depth per tree |
| `min_samples_split` | `int` | `2` | Min samples to split |
| `min_samples_leaf` | `int` | `1` | Min samples in leaf |
| `max_features` | `str` | `"sqrt"` | `"sqrt"`, `"log2"`, `"all"` |

</div>

<div class="lang-fr">

## Description

Ensemble d'arbres de décision entraînés sur des échantillons bootstrap. Chaque arbre utilise un sous-ensemble aléatoire de variables pour les divisions. Parallélisé avec Rayon.

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_estimators` | `int` | `100` | Nombre d'arbres |
| `max_depth` | `int` | `10` | Profondeur maximale par arbre |
| `min_samples_split` | `int` | `2` | Minimum d'échantillons pour diviser |
| `min_samples_leaf` | `int` | `1` | Minimum d'échantillons en feuille |
| `max_features` | `str` | `"sqrt"` | `"sqrt"`, `"log2"`, `"all"` |

</div>

model.feature_importances_ -> list[float]
```

---

## Description

Ensemble of decision trees trained on bootstrap samples with random feature subsets. Trees are built in parallel via Rayon. Prediction is majority vote (classifier) or mean (regressor).

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `100` | Number of trees |
| `max_depth` | `int` | `10` | Maximum depth per tree |
| `min_samples_split` | `int` | `2` | Minimum samples to split |
| `min_samples_leaf` | `int` | `1` | Minimum samples in a leaf |
| `max_features` | `str` | `"sqrt"` | `"sqrt"`, `"log2"`, or `"all"` |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `feature_importances_` | `list[float]` | Mean impurity decrease across all trees |
| `classes_` | `list[int]` | Unique class labels (classifier only) |

---

## Examples

<details>
<summary><strong>Classification</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(1000, 6)
y = ((X[:, 0] + X[:, 1]) > 0).astype(np.int32)

clf = sp.RandomForestClassifier(n_estimators=100, max_depth=8)
clf.fit(X, y)

print(f"Accuracy: {clf.score(X, y):.4f}")
print(f"Top features: {[f'{v:.3f}' for v in clf.feature_importances_]}")
```

</details>

<details>
<summary><strong>Regression</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = X[:, 0] * 3 + np.sin(X[:, 1]) + np.random.randn(500) * 0.2

reg = sp.RandomForestRegressor(n_estimators=50, max_depth=10)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}")
```

</details>
