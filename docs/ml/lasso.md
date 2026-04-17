# Lasso

## Signature

```python
model = sp.Lasso(
    alpha: float = 1.0,
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

L1-regularized regression via coordinate descent with soft thresholding. Produces sparse solutions where irrelevant features get exactly zero coefficients.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | L1 penalty strength |
| `max_iter` | `int` | `1000` | Maximum coordinate descent iterations |
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
| `coef_` | `list[float]` | Fitted coefficients (sparse) |
| `intercept_` | `float` | Bias term |

---

## Example

<details>
<summary><strong>Sparse feature selection</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 20)
true_coef = np.zeros(20)
true_coef[:3] = [3.0, -2.0, 1.5]
y = X @ true_coef + np.random.randn(300) * 0.2

model = sp.Lasso(alpha=0.1)
model.fit(X, y)

non_zero = sum(1 for c in model.coef_ if abs(c) > 1e-6)
print(f"R²: {model.score(X, y):.4f}")
print(f"Non-zero coefficients: {non_zero} / 20")
```

</details>
