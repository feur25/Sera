# SGDClassifier / SGDRegressor

<div class="lang-en">

## Code

**Signature**

```python
clf = sp.SGDClassifier(
    loss="hinge", alpha=0.0001, max_iter=1000,
    tol=1e-3, fit_intercept=True, eta0=1.0
)
reg = sp.SGDRegressor(
    loss="squared_error", alpha=0.0001, max_iter=1000,
    tol=1e-3, fit_intercept=True, eta0=0.01, epsilon=0.1
)

model.fit(X, y, checkpoint_id=None)
model.predict(X)             -> list[int] | list[float]
model.score(X, y)            -> float
model.decision_function(X)   -> list[float]   # classifier only
model.get_params()           -> dict
model.set_params(alpha=..., max_iter=..., tol=..., eta0=..., epsilon=...)
```

**Constructor parameters — SGDClassifier**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `loss` | `str` | `"hinge"` | `"hinge"`, `"squared_hinge"`, `"log_loss"`, `"modified_huber"` |
| `alpha` | `float` | `1e-4` | L2 regularisation strength |
| `max_iter` | `int` | `1000` | Passes over the training set |
| `tol` | `float` | `1e-3` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Fit a bias term |
| `eta0` | `float` | `1.0` | Initial learning rate |

**Constructor parameters — SGDRegressor**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `loss` | `str` | `"squared_error"` | `"squared_error"`, `"huber"`, `"epsilon_insensitive"` |
| `alpha` | `float` | `1e-4` | L2 regularisation strength |
| `max_iter` | `int` | `1000` | Passes over the training set |
| `tol` | `float` | `1e-3` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Fit a bias term |
| `eta0` | `float` | `0.01` | Initial learning rate |
| `epsilon` | `float` | `0.1` | Insensitivity zone for Huber / $\varepsilon$-SVR loss |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients |
| `intercept_` | `float` | Bias term |
| `n_iter_` | `int` | Iterations run |
| `classes_` | `list[int]` | Unique classes (classifier only) |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(1000, 10)
y = (X[:, 0] - X[:, 1] > 0).astype(int)

clf = sp.SGDClassifier(loss="hinge", alpha=1e-4, eta0=0.1)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")

reg = sp.SGDRegressor(loss="squared_error", alpha=1e-4, eta0=0.01)
reg.fit(X, X[:, 0] * 2 - 1)
print(f"R²: {reg.score(X, X[:, 0] * 2 - 1):.4f}")
```

</details>

---

## Algorithmic Functioning

**Stochastic Gradient Descent** processes one sample at a time, performing a noisy gradient step that scales to large datasets:

$$\beta^{(t+1)} \leftarrow \beta^{(t)} - \eta_t \nabla_\beta \mathcal{L}_i(\beta^{(t)})$$

where $i$ is drawn uniformly from $\{1, \ldots, n\}$ and $\mathcal{L}_i$ is the per-sample loss. The learning rate decays over time:

$$\eta_t = \frac{\eta_0}{1 + \alpha \cdot t}$$

**L2 regularisation** is applied as weight decay before each step:

$$\beta^{(t)} \leftarrow \beta^{(t)}(1 - \eta_t \alpha)$$

**Classifier losses and their gradients:**

| Loss | $\mathcal{L}(y, f)$ | Gradient w.r.t. $f$ |
|------|---------------------|---------------------|
| Hinge | $\max(0,\ 1 - yf)$ | $-y \cdot \mathbf{1}[yf < 1]$ |
| Squared Hinge | $\max(0,\ 1 - yf)^2$ | $-2y(1-yf) \cdot \mathbf{1}[yf < 1]$ |
| Log loss | $\log(1 + e^{-yf})$ | $-y\,\sigma(-yf)$ |
| Modified Huber | $\max(0, 1-yf)^2$ if $yf \geq -1$, else $-4yf$ | smooth hinge |

**Regressor losses:**

| Loss | $\mathcal{L}(y, f)$ | Notes |
|------|---------------------|-------|
| Squared error | $\tfrac{1}{2}(y-f)^2$ | Standard MSE gradient |
| Huber | $\tfrac{1}{2}(y-f)^2$ if $\|y-f\| \leq \varepsilon$, else $\varepsilon(\|y-f\| - \tfrac{\varepsilon}{2})$ | Robust to outliers |
| $\varepsilon$-insensitive | $\max(0, \|y-f\| - \varepsilon)$ | SVR-style zero zone |

The `checkpoint_id` argument enables resumable multi-epoch training.

</div>

<div class="lang-fr">

## Code

**Signature**

```python
clf = sp.SGDClassifier(
    loss="hinge", alpha=0.0001, max_iter=1000,
    tol=1e-3, fit_intercept=True, eta0=1.0
)
reg = sp.SGDRegressor(
    loss="squared_error", alpha=0.0001, max_iter=1000,
    tol=1e-3, fit_intercept=True, eta0=0.01, epsilon=0.1
)

model.fit(X, y, checkpoint_id=None)
model.predict(X)             -> list[int] | list[float]
model.score(X, y)            -> float
model.decision_function(X)   -> list[float]   # classificateur seulement
model.get_params()           -> dict
model.set_params(alpha=..., max_iter=..., tol=..., eta0=..., epsilon=...)
```

**Paramètres du constructeur — SGDClassifier**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `loss` | `str` | `"hinge"` | `"hinge"`, `"squared_hinge"`, `"log_loss"`, `"modified_huber"` |
| `alpha` | `float` | `1e-4` | Force de régularisation L2 |
| `max_iter` | `int` | `1000` | Passes sur l'ensemble d'entraînement |
| `tol` | `float` | `1e-3` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |
| `eta0` | `float` | `1.0` | Taux d'apprentissage initial |

**Paramètres du constructeur — SGDRegressor**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `loss` | `str` | `"squared_error"` | `"squared_error"`, `"huber"`, `"epsilon_insensitive"` |
| `alpha` | `float` | `1e-4` | Force de régularisation L2 |
| `max_iter` | `int` | `1000` | Passes sur l'ensemble d'entraînement |
| `tol` | `float` | `1e-3` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |
| `eta0` | `float` | `0.01` | Taux d'apprentissage initial |
| `epsilon` | `float` | `0.1` | Zone d'insensibilité pour la perte Huber / $\varepsilon$-SVR |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `coef_` | `list[float]` | Coefficients ajustés |
| `intercept_` | `float` | Terme de biais |
| `n_iter_` | `int` | Itérations effectuées |
| `classes_` | `list[int]` | Classes uniques (classificateur uniquement) |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(1000, 10)
y = (X[:, 0] - X[:, 1] > 0).astype(int)

clf = sp.SGDClassifier(loss="hinge", alpha=1e-4, eta0=0.1)
clf.fit(X, y)
print(f"Précision : {clf.score(X, y):.4f}")

reg = sp.SGDRegressor(loss="squared_error", alpha=1e-4, eta0=0.01)
reg.fit(X, X[:, 0] * 2 - 1)
print(f"R² : {reg.score(X, X[:, 0] * 2 - 1):.4f}")
```

</details>

---

## Fonctionnement algorithmique

La **descente de gradient stochastique** traite un échantillon à la fois, effectuant une mise à jour de gradient bruitée qui passe à l'échelle pour les grands jeux de données :

$$\beta^{(t+1)} \leftarrow \beta^{(t)} - \eta_t \nabla_\beta \mathcal{L}_i(\beta^{(t)})$$

où $i$ est tiré uniformément dans $\{1, \ldots, n\}$ et $\mathcal{L}_i$ est la perte par échantillon. Le taux d'apprentissage décroît au fil du temps :

$$\eta_t = \frac{\eta_0}{1 + \alpha \cdot t}$$

La **régularisation L2** est appliquée comme décroissance des poids avant chaque étape :

$$\beta^{(t)} \leftarrow \beta^{(t)}(1 - \eta_t \alpha)$$

**Pertes du classificateur et leurs gradients :**

| Perte | $\mathcal{L}(y, f)$ | Gradient par rapport à $f$ |
|------|---------------------|---------------------|
| Hinge | $\max(0,\ 1 - yf)$ | $-y \cdot \mathbf{1}[yf < 1]$ |
| Hinge carré | $\max(0,\ 1 - yf)^2$ | $-2y(1-yf) \cdot \mathbf{1}[yf < 1]$ |
| Log loss | $\log(1 + e^{-yf})$ | $-y\,\sigma(-yf)$ |
| Huber modifié | $\max(0, 1-yf)^2$ si $yf \geq -1$, sinon $-4yf$ | hinge lisse |

**Pertes du régresseur :**

| Perte | $\mathcal{L}(y, f)$ | Notes |
|------|---------------------|-------|
| Erreur quadratique | $\tfrac{1}{2}(y-f)^2$ | Gradient MSE standard |
| Huber | $\tfrac{1}{2}(y-f)^2$ si $\|y-f\| \leq \varepsilon$, sinon $\varepsilon(\|y-f\| - \tfrac{\varepsilon}{2})$ | Robuste aux valeurs aberrantes |
| $\varepsilon$-insensible | $\max(0, \|y-f\| - \varepsilon)$ | Zone zéro style SVR |

L'argument `checkpoint_id` permet un entraînement multi-époque repris.

</div>
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
