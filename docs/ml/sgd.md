# SGDClassifier / SGDRegressor

<div class="lang-en">

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
    loss: str = "squared_error",
    alpha: float = 0.0001,
    max_iter: int = 1000,
    tol: float = 1e-3,
    fit_intercept: bool = True,
    eta0: float = 0.01,
    epsilon: float = 0.1,
)

model.fit(x, y)
model.predict(x) -> list[int] | list[float]
model.score(x, y) -> float

clf.decision_function(x) -> list[float]
clf.coef_           -> list[float]
clf.intercept_      -> float
clf.classes_        -> list[int]
clf.n_iter_         -> int
clf.alpha_          -> float
clf.max_iter_       -> int
clf.tol_            -> float
clf.fit_intercept_  -> bool
clf.eta0_           -> float
clf.loss_           -> str

reg.coef_           -> list[float]
reg.intercept_      -> float
reg.n_iter_         -> int
reg.alpha_          -> float
reg.max_iter_       -> int
reg.tol_            -> float
reg.fit_intercept_  -> bool
reg.eta0_           -> float
reg.epsilon_        -> float
reg.loss_           -> str
```

---

## Description

Stochastic Gradient Descent linear models. The classifier supports multiple loss functions — `"hinge"` (SVM), `"log_loss"` (logistic), `"modified_huber"`, `"squared_hinge"`. The regressor supports `"squared_error"`, `"huber"`, and `"epsilon_insensitive"` losses.

Parallelized with block-based mini-batch SGD (blocks of 4096 rows) using Rayon. Performance scales super-linearly with larger datasets.

---

## Constructor Parameters

### SGDClassifier

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `loss` | `str` | `"hinge"` | `"hinge"`, `"log_loss"`, `"modified_huber"`, `"squared_hinge"` |
| `alpha` | `float` | `0.0001` | L2 regularization strength |
| `max_iter` | `int` | `1000` | Maximum epochs |
| `tol` | `float` | `1e-3` | Early stopping tolerance |
| `fit_intercept` | `bool` | `True` | Add bias term |
| `eta0` | `float` | `1.0` | Initial learning rate |

### SGDRegressor

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `loss` | `str` | `"squared_error"` | `"squared_error"`, `"huber"`, `"epsilon_insensitive"` |
| `alpha` | `float` | `0.0001` | L2 regularization strength |
| `max_iter` | `int` | `1000` | Maximum epochs |
| `tol` | `float` | `1e-3` | Early stopping tolerance |
| `fit_intercept` | `bool` | `True` | Add bias term |
| `eta0` | `float` | `0.01` | Initial learning rate |
| `epsilon` | `float` | `0.1` | Tube width for `"huber"` and `"epsilon_insensitive"` |

---

## Examples

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
print(f"Loss: {clf.loss_}  eta0: {clf.eta0_}")
```

</details>

<details>
<summary><strong>SGD regressor with Huber loss</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
y = X @ np.array([1.0, -0.5, 2.0]) + np.random.randn(500) * 0.2

reg = sp.SGDRegressor(loss="huber", epsilon=0.5, alpha=0.001, eta0=0.01)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}  loss: {reg.loss_}")
```

</details>
</div>

<div class="lang-fr">

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
    loss: str = "squared_error",
    alpha: float = 0.0001,
    max_iter: int = 1000,
    tol: float = 1e-3,
    fit_intercept: bool = True,
    eta0: float = 0.01,
    epsilon: float = 0.1,
)
```

---

## Description

Modèles linéaires par descente de gradient stochastique. Le classifieur accepte plusieurs fonctions de perte : `"hinge"` (SVM), `"log_loss"` (logistique), `"modified_huber"`, `"squared_hinge"`. Le régresseur supporte `"squared_error"`, `"huber"` et `"epsilon_insensitive"`.

Parallélisé avec mini-lots de 4096 lignes via Rayon. La performance s'améliore super-linéairement avec de grands jeux de données.

---

## Paramètres du constructeur

### SGDClassifier

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `loss` | `str` | `"hinge"` | Fonction de perte : `"hinge"`, `"log_loss"`, `"modified_huber"`, `"squared_hinge"` |
| `alpha` | `float` | `0.0001` | Force de régularisation L2 |
| `max_iter` | `int` | `1000` | Nombre maximum d'époques |
| `tol` | `float` | `1e-3` | Tolérance d'arrêt anticipé |
| `fit_intercept` | `bool` | `True` | Ajouter un biais |
| `eta0` | `float` | `1.0` | Taux d'apprentissage initial |

### SGDRegressor

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `loss` | `str` | `"squared_error"` | Fonction de perte : `"squared_error"`, `"huber"`, `"epsilon_insensitive"` |
| `alpha` | `float` | `0.0001` | Force de régularisation L2 |
| `max_iter` | `int` | `1000` | Nombre maximum d'époques |
| `tol` | `float` | `1e-3` | Tolérance d'arrêt anticipé |
| `fit_intercept` | `bool` | `True` | Ajouter un biais |
| `eta0` | `float` | `0.01` | Taux d'apprentissage initial |
| `epsilon` | `float` | `0.1` | Largeur du tube pour `"huber"` et `"epsilon_insensitive"` |

---

## Exemples

<details>
<summary><strong>Classifieur SGD avec perte hinge</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(1000, 5)
y = (X[:, 0] * 2 + X[:, 1] > 0).astype(np.int32)

clf = sp.SGDClassifier(loss="hinge", alpha=0.001)
clf.fit(X, y)
print(f"Précision : {clf.score(X, y):.4f}")
print(f"Perte : {clf.loss_}  eta0 : {clf.eta0_}")
```

</details>

<details>
<summary><strong>Régresseur SGD avec perte Huber</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
y = X @ np.array([1.0, -0.5, 2.0]) + np.random.randn(500) * 0.2

reg = sp.SGDRegressor(loss="huber", epsilon=0.5, alpha=0.001, eta0=0.01)
reg.fit(X, y)
print(f"R² : {reg.score(X, y):.4f}  perte : {reg.loss_}")
```

</details>
</div>

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
