# LinearSVC / LinearSVR

<div class="lang-en">

## Signature

```python
clf = sp.LinearSVC(
    c: float = 1.0,
    max_iter: int = 1000,
    tol: float = 1e-4,
    fit_intercept: bool = True,
)

reg = sp.LinearSVR(
    c: float = 1.0,
    epsilon: float = 0.1,
    max_iter: int = 1000,
    tol: float = 1e-4,
    fit_intercept: bool = True,
)

model.fit(x, y)
model.predict(x) -> list[int] | list[float]
model.score(x, y) -> float

clf.decision_function(x) -> list[float]
clf.classes_        -> list[int]
clf.coef_           -> list[list[float]]
clf.intercept_      -> list[float]
clf.C_              -> float
clf.max_iter_       -> int
clf.tol_            -> float
clf.fit_intercept_  -> bool

reg.coef_           -> list[float]
reg.intercept_      -> float
reg.C_              -> float
reg.epsilon_        -> float
reg.max_iter_       -> int
reg.tol_            -> float
reg.fit_intercept_  -> bool
```

---

## Description

Linear Support Vector Machines with dual coordinate descent solver.

- **LinearSVC** — hinge loss with One-vs-Rest for multiclass.
- **LinearSVR** — epsilon-insensitive loss for regression.

---

## Constructor Parameters

### LinearSVC

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `c` | `float` | `1.0` | Inverse regularization strength |
| `max_iter` | `int` | `1000` | Maximum epochs |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Fit a bias term |

### LinearSVR

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `c` | `float` | `1.0` | Inverse regularization strength |
| `epsilon` | `float` | `0.1` | Epsilon-tube width |
| `max_iter` | `int` | `1000` | Maximum epochs |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Fit a bias term |

---

## Examples

<details>
<summary><strong>LinearSVC classification</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(np.int32)

clf = sp.LinearSVC(c=1.0, fit_intercept=True)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")
print(f"fit_intercept: {clf.fit_intercept_}")
```

</details>

<details>
<summary><strong>LinearSVR regression</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 3)
y = X @ np.array([1.5, -0.5, 2.0]) + np.random.randn(300) * 0.3

reg = sp.LinearSVR(c=1.0, epsilon=0.05)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}")
print(f"epsilon: {reg.epsilon_}  C: {reg.C_}")
```

</details>
</div>

<div class="lang-fr">

## Signature

```python
clf = sp.LinearSVC(
    c: float = 1.0,
    max_iter: int = 1000,
    tol: float = 1e-4,
    fit_intercept: bool = True,
)

reg = sp.LinearSVR(
    c: float = 1.0,
    epsilon: float = 0.1,
    max_iter: int = 1000,
    tol: float = 1e-4,
    fit_intercept: bool = True,
)
```

---

## Description

Machines à vecteurs de support linéaires avec solveur par descente de coordonnées duale.

- **LinearSVC** — perte hinge avec stratégie One-vs-Rest pour la classification multiclasse.
- **LinearSVR** — perte epsilon-insensible pour la régression.

---

## Paramètres du constructeur

### LinearSVC

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `c` | `float` | `1.0` | Inverse de la force de régularisation |
| `max_iter` | `int` | `1000` | Nombre maximum d'époques |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |

### LinearSVR

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `c` | `float` | `1.0` | Inverse de la force de régularisation |
| `epsilon` | `float` | `0.1` | Largeur du tube epsilon |
| `max_iter` | `int` | `1000` | Nombre maximum d'époques |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |

---

## Exemples

<details>
<summary><strong>Classification avec LinearSVC</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(np.int32)

clf = sp.LinearSVC(c=1.0, fit_intercept=True)
clf.fit(X, y)
print(f"Précision : {clf.score(X, y):.4f}")
print(f"fit_intercept : {clf.fit_intercept_}")
```

</details>

<details>
<summary><strong>Régression avec LinearSVR</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 3)
y = X @ np.array([1.5, -0.5, 2.0]) + np.random.randn(300) * 0.3

reg = sp.LinearSVR(c=1.0, epsilon=0.05)
reg.fit(X, y)
print(f"R² : {reg.score(X, y):.4f}")
print(f"epsilon : {reg.epsilon_}  C : {reg.C_}")
```

</details>
</div>

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
