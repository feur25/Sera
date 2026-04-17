# LinearSVC / LinearSVR

## Signature

```python
clf = sp.LinearSVC(
    c: float = 1.0,
    max_iter: int = 1000,
    tol: float = 1e-4,
)

reg = sp.LinearSVR(
    c: float = 1.0,
    epsilon: float = 0.1,
    max_iter: int = 1000,
    tol: float = 1e-4,
)

model.fit(x, y)
model.predict(x) -> list[int] | list[float]
model.score(x, y) -> float

clf.decision_function(x) -> list[float]
clf.classes_    -> list[int]
clf.coef_       -> list[list[float]]
clf.intercept_  -> list[float]

reg.coef_       -> list[float]
reg.intercept_  -> float
```

---

## Description

Linear Support Vector Machines with dual coordinate descent solver.

- **LinearSVC** — hinge loss with One-vs-Rest for multiclass.
- **LinearSVR** — epsilon-insensitive loss.

---

## Constructor Parameters

### LinearSVC

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `c` | `float` | `1.0` | Inverse regularization strength |
| `max_iter` | `int` | `1000` | Maximum epochs |
| `tol` | `float` | `1e-4` | Convergence tolerance |

### LinearSVR

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `c` | `float` | `1.0` | Inverse regularization strength |
| `epsilon` | `float` | `0.1` | Epsilon-tube width |
| `max_iter` | `int` | `1000` | Maximum epochs |
| `tol` | `float` | `1e-4` | Convergence tolerance |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels (LinearSVC only) |
| `coef_` | `list` | Weight coefficients |
| `intercept_` | `float` or `list` | Bias term(s) |

---

## Examples

<details>
<summary><strong>LinearSVC classification</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(np.int32)

clf = sp.LinearSVC(c=1.0)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")

margins = clf.decision_function(X[:5])
print(f"Decision values: {[f'{v:.3f}' for v in margins]}")
```

</details>

<details>
<summary><strong>LinearSVR regression</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 3)
y = X @ np.array([2.0, -1.0, 0.5]) + np.random.randn(300) * 0.3

reg = sp.LinearSVR(c=1.0, epsilon=0.1)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}")
```

</details>
