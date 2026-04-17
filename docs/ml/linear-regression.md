# LinearRegression

## Signature

```python
model = sp.LinearRegression(fit_intercept: bool = True)

model.fit(x, y)
model.predict(x) -> list[float]
model.score(x, y) -> float

model.coef_      -> list[float]
model.intercept_ -> float
```

---

## Description

Ordinary Least Squares regression solved via Cholesky decomposition (falls back to QR if the Gram matrix is not positive-definite). Pure Rust, no external BLAS — runs in microseconds on typical datasets.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `fit_intercept` | `bool` | `True` | Add a bias term |

---

## Methods

### `fit(x, y)`

Fit the model on training data.

| Argument | Type | Description |
|----------|------|-------------|
| `x` | `ndarray (n, p)` | Feature matrix |
| `y` | `ndarray (n,)` | Target values |

### `predict(x) -> list[float]`

Return predictions for each row of `x`.

### `score(x, y) -> float`

Return the R² coefficient of determination.

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients (length p) |
| `intercept_` | `float` | Bias term |

---

## Example

<details>
<summary><strong>Basic regression</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
y = X @ np.array([2.0, -1.0, 0.5]) + np.random.randn(500) * 0.1

model = sp.LinearRegression()
model.fit(X, y)

print(f"R²: {model.score(X, y):.6f}")
print(f"Coefficients: {model.coef_}")
print(f"Intercept: {model.intercept_:.6f}")
```

</details>
