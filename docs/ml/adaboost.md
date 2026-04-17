# AdaBoostClassifier / AdaBoostRegressor

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

model.fit(x, y)
model.predict(x) -> list[int] | list[float]
model.score(x, y) -> float

clf.predict_proba(x) -> ndarray (n, n_classes)
clf.classes_ -> list[int]
```

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
