# ElasticNet

## Signature

```python
model = sp.ElasticNet(
    alpha: float = 1.0,
    l1_ratio: float = 0.5,
    max_iter: int = 1000,
    tol: float = 1e-4,
    fit_intercept: bool = True,
)

model.fit(x, y)
model.predict(x) -> list[float]
model.score(x, y) -> float

model.coef_      -> list[float]
model.intercept_ -> float
```

---

## Description

Combined L1 + L2 regularization via coordinate descent. `l1_ratio=1.0` is pure Lasso, `l1_ratio=0.0` is pure Ridge.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | Overall penalty strength |
| `l1_ratio` | `float` | `0.5` | Mix ratio: 1.0 = Lasso, 0.0 = Ridge |
| `max_iter` | `int` | `1000` | Maximum iterations |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Add a bias term |

---

## Methods

### `fit(x, y)`

| Argument | Type | Description |
|----------|------|-------------|
| `x` | `ndarray (n, p)` | Feature matrix |
| `y` | `ndarray (n,)` | Target values |

### `predict(x) -> list[float]`

### `score(x, y) -> float`

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients |
| `intercept_` | `float` | Bias term |

---

## Example

<details>
<summary><strong>ElasticNet mix</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 15)
y = X[:, :3] @ np.array([2.0, -1.0, 0.5]) + np.random.randn(300) * 0.3

model = sp.ElasticNet(alpha=0.1, l1_ratio=0.7)
model.fit(X, y)

non_zero = sum(1 for c in model.coef_ if abs(c) > 1e-6)
print(f"R²: {model.score(X, y):.4f}")
print(f"Non-zero: {non_zero} / 15")
```

</details>
