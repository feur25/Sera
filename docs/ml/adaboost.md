# AdaBoostClassifier / AdaBoostRegressor

<div class="lang-en">

## Signature

```python
clf = sp.AdaBoostClassifier(
    n_estimators: int = 50,
    learning_rate: float = 1.0,
    max_depth: int = 1,
)

reg = sp.AdaBoostRegressor(
    n_estimators: int = 50,
    learning_rate: float = 1.0,
    max_depth: int = 3,
)

clf.classes_        -> list[int]
clf.n_estimators_   -> int
clf.learning_rate_  -> float
clf.max_depth_      -> int

reg.n_estimators_   -> int
reg.learning_rate_  -> float
reg.max_depth_      -> int
```

---

## Description

Adaptive Boosting. Reweights samples at each stage so that subsequent trees focus on previously misclassified examples.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `50` | Number of weak learners |
| `learning_rate` | `float` | `1.0` | Weight shrinkage per stage |
| `max_depth` | `int` | `1`/`3` | Depth of each base tree |

</div>

<div class="lang-fr">

## Description

Boosting adaptatif. Repondère les échantillons à chaque étape pour que les arbres suivants se concentrent sur les exemples mal classés.

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_estimators` | `int` | `50` | Nombre d'apprenants faibles |
| `learning_rate` | `float` | `1.0` | Réduction du poids par étape |
| `max_depth` | `int` | `1`/`3` | Profondeur de chaque arbre de base |

</div>

---

## Description

Adaptive Boosting using the **SAMME.R** (Real) algorithm for classification with Laplace-smoothed probability estimates. Each weak learner is a decision stump (classifier) or shallow tree (regressor). SAMME.R uses probability-based boosting which outperforms the discrete SAMME variant.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `50` | Number of boosting rounds |
| `learning_rate` | `float` | `1.0` | Shrinkage per step |
| `max_depth` | `int` | `1` (clf) / `3` (reg) | Depth of weak learner |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels (classifier only) |

---

## Example

<details>
<summary><strong>AdaBoost stumps</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = ((X[:, 0] + X[:, 1]) > 0).astype(np.int32)

clf = sp.AdaBoostClassifier(n_estimators=100)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")
```

</details>
