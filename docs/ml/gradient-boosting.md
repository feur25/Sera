# GradientBoostingClassifier / GradientBoostingRegressor

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

model.fit(x, y)
model.predict(x) -> list[int] | list[float]
model.score(x, y) -> float

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
