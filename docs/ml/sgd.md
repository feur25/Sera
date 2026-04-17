# SGDClassifier / SGDRegressor

## Signature

```python
clf = sp.SGDClassifier(
    loss: str = "hinge",
    alpha: float = 0.0001,
    max_iter: int = 1000,
    tol: float = 1e-3,
    fit_intercept: bool = True,
    eta0: float = 1.0,
)

reg = sp.SGDRegressor(
    alpha: float = 0.0001,
    max_iter: int = 1000,
    tol: float = 1e-3,
    fit_intercept: bool = True,
    eta0: float = 0.01,
)

# Common methods
model.fit(x, y)
model.predict(x) -> list[int] | list[float]
model.score(x, y) -> float

# SGDClassifier only
clf.decision_function(x) -> list[float]
clf.coef_       -> list[float]
clf.intercept_  -> float
clf.classes_    -> list[int]
clf.n_iter_     -> int

# SGDRegressor only
reg.coef_       -> list[float]
reg.intercept_  -> float
reg.n_iter_     -> int
```

---

## Description

Stochastic Gradient Descent linear models. The classifier supports multiple loss functions — `"hinge"` (SVM), `"log"` (logistic), `"modified_huber"`, `"squared_hinge"`. The regressor uses squared loss with L2 regularization.

---

## Constructor Parameters

### SGDClassifier

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `loss` | `str` | `"hinge"` | `"hinge"`, `"log"`, `"modified_huber"`, `"squared_hinge"` |
| `alpha` | `float` | `0.0001` | Regularization multiplier |
| `max_iter` | `int` | `1000` | Maximum epochs |
| `tol` | `float` | `1e-3` | Early stopping tolerance |
| `fit_intercept` | `bool` | `True` | Add bias |
| `eta0` | `float` | `1.0` | Initial learning rate |

### SGDRegressor

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `0.0001` | Regularization multiplier |
| `max_iter` | `int` | `1000` | Maximum epochs |
| `tol` | `float` | `1e-3` | Early stopping tolerance |
| `fit_intercept` | `bool` | `True` | Add bias |
| `eta0` | `float` | `0.01` | Initial learning rate |

---

## Example

<details>
<summary><strong>SGD classifier with hinge loss</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(1000, 5)
y = (X[:, 0] * 2 + X[:, 1] > 0).astype(np.int32)

clf = sp.SGDClassifier(loss="hinge", alpha=0.001)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")
```

</details>

<details>
<summary><strong>SGD regressor</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
y = X @ np.array([1.0, -0.5, 2.0]) + np.random.randn(500) * 0.2

reg = sp.SGDRegressor(alpha=0.001, eta0=0.01)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}")
```

</details>
