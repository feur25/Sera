# GradientBoostingClassifier / GradientBoostingRegressor

<div class="lang-en">

## Signature

```python
clf = sp.GradientBoostingClassifier(
    n_estimators: int = 100,
    learning_rate: float = 0.1,
    max_depth: int = 3,
    min_samples_split: int = 2,
    min_samples_leaf: int = 1,
)

reg = sp.GradientBoostingRegressor(
    n_estimators: int = 100,
    learning_rate: float = 0.1,
    max_depth: int = 3,
    min_samples_split: int = 2,
    min_samples_leaf: int = 1,
)

clf.n_estimators_       -> int
clf.learning_rate_      -> float
clf.max_depth_          -> int
clf.min_samples_split_  -> int
clf.min_samples_leaf_   -> int

reg.n_estimators_       -> int
reg.learning_rate_      -> float
reg.max_depth_          -> int
reg.min_samples_split_  -> int
reg.min_samples_leaf_   -> int
```

---

## Description

Sequential ensemble: each tree fits residuals of previous. Uses negative gradient of loss function (MSE for regression, log-loss for classification).

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `100` | Number of boosting stages |
| `learning_rate` | `float` | `0.1` | Shrinkage factor per stage |
| `max_depth` | `int` | `3` | Maximum depth of each tree |
| `min_samples_split` | `int` | `2` | Min samples to split |
| `min_samples_leaf` | `int` | `1` | Min samples in leaf |

</div>

<div class="lang-fr">

## Description

Ensemble séquentiel : chaque arbre ajuste les résidus du précédent. Utilise le gradient négatif de la fonction de perte (MSE pour la régression, log-loss pour la classification).

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_estimators` | `int` | `100` | Nombre d'étapes de boosting |
| `learning_rate` | `float` | `0.1` | Facteur de réduction par étape |
| `max_depth` | `int` | `3` | Profondeur maximale de chaque arbre |
| `min_samples_split` | `int` | `2` | Minimum d'échantillons pour diviser |
| `min_samples_leaf` | `int` | `1` | Minimum d'échantillons en feuille |

</div>

clf.predict_proba(x) -> ndarray (n, n_classes)
clf.classes_ -> list[int]
```

---

## Description

Sequential boosting with shallow decision trees. The classifier uses multiclass softmax boosting with **Newton-Raphson leaf values** (gradient + hessian). The regressor uses residual fitting with shrinkage.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `100` | Number of boosting rounds |
| `learning_rate` | `float` | `0.1` | Shrinkage per step |
| `max_depth` | `int` | `3` | Depth of individual trees |
| `min_samples_split` | `int` | `2` | Minimum samples to split |
| `min_samples_leaf` | `int` | `1` | Minimum samples in a leaf |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels (classifier only) |

---

## Examples

<details>
<summary><strong>Multiclass classification</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(600, 5)
y = np.where(X[:, 0] > 0.5, 2, np.where(X[:, 0] < -0.5, 0, 1)).astype(np.int32)

clf = sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1, max_depth=3)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")
```

</details>

<details>
<summary><strong>Regression</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = X[:, 0] ** 2 + np.sin(X[:, 1] * 3) + np.random.randn(500) * 0.1

reg = sp.GradientBoostingRegressor(n_estimators=200, learning_rate=0.05, max_depth=4)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}")
```

</details>
