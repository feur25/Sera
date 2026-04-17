# LogisticRegression

<div class="lang-en">

## Signature

```python
model = sp.LogisticRegression(
    c: float = 1.0,
    max_iter: int = 1000,
    tol: float = 1e-4,
    fit_intercept: bool = True,
)

model.classes_        -> list[int]
model.coef_           -> ndarray
model.intercept_      -> float | ndarray
model.n_iter_         -> int
model.C_              -> float
model.max_iter_       -> int
model.tol_            -> float
model.fit_intercept_  -> bool
```

---

## Description

Multinomial logistic regression via Newton's method. For binary problems uses a single weight vector; for multiclass uses One-vs-Rest.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `c` | `float` | `1.0` | Inverse of regularization strength |
| `max_iter` | `int` | `1000` | Maximum Newton iterations |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Add bias term |

</div>

<div class="lang-fr">

## Description

Régression logistique multinomiale via la méthode de Newton. Pour les problèmes binaires, un seul vecteur de poids ; pour la classification multi-classes, stratégie One-vs-Rest.

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `c` | `float` | `1.0` | Inverse de la force de régularisation |
| `max_iter` | `int` | `1000` | Nombre maximum d'itérations Newton |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajouter un biais |

</div>

model.n_iter_     -> int
```

---

## Description

Logistic regression with Newton-Raphson optimization. Binary uses a 2-class softmax; multiclass uses a **full joint Hessian** Newton solver with **backtracking line search** for robust convergence on 10+ class problems.

`c` is the inverse regularization strength (higher = less regularization), matching sklearn's `C` parameter.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `c` | `float` | `1.0` | Inverse regularization (like sklearn `C`) |
| `max_iter` | `int` | `1000` | Maximum Newton iterations |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Add a bias term |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique sorted class labels |
| `coef_` | `ndarray` | Coefficient matrix (n_classes, n_features) or (n_features,) for binary |
| `intercept_` | `float` or `ndarray` | Intercept(s) |
| `n_iter_` | `int` | Number of Newton iterations run |

---

## Example

<details>
<summary><strong>Binary classification</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(np.int32)

model = sp.LogisticRegression(c=1.0)
model.fit(X, y)

print(f"Accuracy: {model.score(X, y):.4f}")
print(f"Coef: {model.coef_}")
print(f"Intercept: {model.intercept_}")
print(f"Converged in {model.n_iter_} iterations")
```

</details>
