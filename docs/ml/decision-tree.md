# DecisionTreeClassifier / DecisionTreeRegressor

## Signature

```python
clf = sp.DecisionTreeClassifier(
    max_depth: int = 10,
    min_samples_split: int = 2,
    min_samples_leaf: int = 1,
    max_features: int | None = None,
    criterion: str = "gini",
)

reg = sp.DecisionTreeRegressor(
    max_depth: int = 10,
    min_samples_split: int = 2,
    min_samples_leaf: int = 1,
    max_features: int | None = None,
)

model.fit(x, y)
model.predict(x) -> list[int] | list[float]
model.score(x, y) -> float

# Classifier only
clf.predict_proba(x) -> list[list[float]]
clf.classes_             -> list[int]

# Both
model.feature_importances_ -> list[float]
```

---

## Description

CART decision trees with Gini / Entropy / MSE splitting. Feature importance is computed from the total impurity decrease at each split, weighted by the number of samples reaching that node.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `max_depth` | `int` | `10` | Maximum tree depth |
| `min_samples_split` | `int` | `2` | Minimum samples to split a node |
| `min_samples_leaf` | `int` | `1` | Minimum samples in a leaf |
| `max_features` | `int \| None` | `None` | Random subset of features per split (`None` = all) |
| `criterion` | `str` | `"gini"` | `"gini"` or `"entropy"` (classifier only) |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `feature_importances_` | `list[float]` | Gini / MSE importance per feature |
| `classes_` | `list[int]` | Unique class labels (classifier only) |

---

## Examples

<details>
<summary><strong>Classification</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = (X[:, 0] ** 2 + X[:, 1] > 1).astype(np.int32)

clf = sp.DecisionTreeClassifier(max_depth=6, criterion="gini")
clf.fit(X, y)

print(f"Accuracy: {clf.score(X, y):.4f}")
print(f"Feature importances: {[f'{v:.3f}' for v in clf.feature_importances_]}")
```

</details>

<details>
<summary><strong>Regression</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
y = np.sin(X[:, 0]) + X[:, 1] ** 2 + np.random.randn(500) * 0.1

reg = sp.DecisionTreeRegressor(max_depth=8)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}")
```

</details>
