# Ridge / RidgeClassifier

## Signature

```python
reg = sp.Ridge(alpha: float = 1.0, fit_intercept: bool = True)
clf = sp.RidgeClassifier(alpha: float = 1.0)

reg.fit(x, y)
reg.predict(x) -> list[float]
reg.score(x, y) -> float

reg.coef_      -> list[float]
reg.intercept_ -> float
reg.alpha_     -> float

clf.fit(x, y)
clf.predict(x) -> list[int]
clf.score(x, y) -> float

clf.coef_      -> list[float]
clf.intercept_ -> float
clf.classes_   -> list[int]
```

---

## Description

L2-regularized linear model (Tikhonov regularization). Solved via Cholesky on `(X^T X + alpha * I)`.

- **Ridge** — regression with L2 penalty.
- **RidgeClassifier** — classification by rounding Ridge regression predictions to nearest class label.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | Regularization strength |
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

R² coefficient of determination.

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients |
| `intercept_` | `float` | Bias term |

---

## Example

<details>
<summary><strong>Ridge with regularization</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(200, 10)
y = X @ np.random.randn(10) + np.random.randn(200) * 0.5

model = sp.Ridge(alpha=0.5)
model.fit(X, y)
print(f"R²: {model.score(X, y):.4f}")
print(f"Max coef magnitude: {max(abs(c) for c in model.coef_):.4f}")
```

</details>
