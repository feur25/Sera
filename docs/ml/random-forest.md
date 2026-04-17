# RandomForestClassifier / RandomForestRegressor

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
