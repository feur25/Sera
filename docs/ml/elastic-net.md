# ElasticNet

<div class="lang-en">

## Code

**Signature**

```python
model = sp.ElasticNet(alpha=1.0, l1_ratio=0.5, max_iter=1000, tol=1e-4, fit_intercept=True)

model.fit(X, y, checkpoint_id=None)
model.predict(X)   -> list[float]
model.score(X, y)  -> float
model.get_params() -> dict
model.set_params(alpha=..., l1_ratio=..., max_iter=..., tol=..., fit_intercept=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | Overall regularisation strength |
| `l1_ratio` | `float` | `0.5` | Mix between L1 and L2 — `0` = pure Ridge, `1` = pure Lasso |
| `max_iter` | `int` | `1000` | Maximum coordinate descent passes |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Fit a bias term |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients, shape $(p,)$ |
| `intercept_` | `float` | Bias term |
| `n_iter_` | `int` | Iterations run |
| `alpha_` | `float` | Regularisation strength |
| `l1_ratio_` | `float` | L1 mix ratio |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(200, 15)
y = X[:, :3] @ [2.0, -1.0, 0.5] + np.random.randn(200)

model = sp.ElasticNet(alpha=0.1, l1_ratio=0.7)
model.fit(X, y)
print(f"R²: {model.score(X, y):.4f}")
print(f"Non-zero: {(np.abs(model.coef_) > 1e-6).sum()}")
```

</details>

---

## Algorithmic Functioning

ElasticNet combines **L1 and L2 penalties**, inheriting sparsity from Lasso and grouping stability from Ridge:

$$\hat{\beta} = \underset{\beta}{\arg\min}\ \frac{1}{2n}\|y - X\beta\|_2^2 + \alpha\rho\|\beta\|_1 + \frac{\alpha(1-\rho)}{2}\|\beta\|_2^2$$

where $\rho = \texttt{l1\_ratio}$.

**Coordinate Descent** — Each coordinate $j$ is updated in closed form. The partial residual is:

$$r_{ij} = y_i - \sum_{k \neq j} x_{ik}\beta_k$$

The per-coordinate update combines soft-thresholding with L2 shrinkage:

$$\beta_j \leftarrow \frac{\mathcal{S}\!\left(\frac{1}{n}\sum_{i=1}^n x_{ij}\,r_{ij},\ \alpha\rho\right)}{\frac{1}{n}\sum_{i=1}^n x_{ij}^2 + \alpha(1-\rho)}$$

The denominator adds $\alpha(1-\rho)$ relative to Lasso, which prevents two correlated features from having their coefficients arbitrarily split (the grouping effect of Ridge).

**Special cases:**

| `l1_ratio` | Equivalent model |
|-----------|-----------------|
| `0.0` | Ridge regression |
| `1.0` | Lasso |
| `(0, 1)` | ElasticNet — sparse + stable |

The `checkpoint_id` argument enables resumable training: calling `fit` again with the same id continues from the last checkpoint.

</div>

<div class="lang-fr">

## Code

**Signature**

```python
model = sp.ElasticNet(alpha=1.0, l1_ratio=0.5, max_iter=1000, tol=1e-4, fit_intercept=True)

model.fit(X, y, checkpoint_id=None)
model.predict(X)   -> list[float]
model.score(X, y)  -> float
model.get_params() -> dict
model.set_params(alpha=..., l1_ratio=..., max_iter=..., tol=..., fit_intercept=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `alpha` | `float` | `1.0` | Force de régularisation globale |
| `l1_ratio` | `float` | `0.5` | Mélange L1/L2 — `0` = Ridge pur, `1` = Lasso pur |
| `max_iter` | `int` | `1000` | Passes maximales de descente par coordonnées |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `coef_` | `list[float]` | Coefficients ajustés, forme $(p,)$ |
| `intercept_` | `float` | Terme de biais |
| `n_iter_` | `int` | Itérations effectuées |
| `alpha_` | `float` | Force de régularisation |
| `l1_ratio_` | `float` | Ratio de mélange L1 |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(200, 15)
y = X[:, :3] @ [2.0, -1.0, 0.5] + np.random.randn(200)

model = sp.ElasticNet(alpha=0.1, l1_ratio=0.7)
model.fit(X, y)
print(f"R² : {model.score(X, y):.4f}")
print(f"Non nuls : {(np.abs(model.coef_) > 1e-6).sum()}")
```

</details>

---

## Fonctionnement algorithmique

ElasticNet combine les **pénalités L1 et L2**, héritant de la parcimonie du Lasso et de la stabilité de groupement du Ridge :

$$\hat{\beta} = \underset{\beta}{\arg\min}\ \frac{1}{2n}\|y - X\beta\|_2^2 + \alpha\rho\|\beta\|_1 + \frac{\alpha(1-\rho)}{2}\|\beta\|_2^2$$

où $\rho = \texttt{l1\_ratio}$.

**Descente par coordonnées** — Chaque coordonnée $j$ est mise à jour en forme fermée. Le résidu partiel est :

$$r_{ij} = y_i - \sum_{k \neq j} x_{ik}\beta_k$$

La mise à jour par coordonnée combine seuillage doux et rétrécissement L2 :

$$\beta_j \leftarrow \frac{\mathcal{S}\!\left(\frac{1}{n}\sum_{i=1}^n x_{ij}\,r_{ij},\ \alpha\rho\right)}{\frac{1}{n}\sum_{i=1}^n x_{ij}^2 + \alpha(1-\rho)}$$

Le dénominateur ajoute $\alpha(1-\rho)$ par rapport au Lasso, ce qui empêche deux features corrélées d'avoir leurs coefficients arbitrairement répartis (effet de groupement du Ridge).

**Cas particuliers :**

| `l1_ratio` | Modèle équivalent |
|-----------|-----------------|
| `0.0` | Régression Ridge |
| `1.0` | Lasso |
| `(0, 1)` | ElasticNet — parcimonieux + stable |

L'argument `checkpoint_id` permet un entraînement repris : appeler `fit` à nouveau avec le même id reprend depuis le dernier checkpoint.

</div>
# ElasticNet

<div class="lang-en">

## Signature

```python
model = sp.ElasticNet(
    alpha: float = 1.0,
    l1_ratio: float = 0.5,
    max_iter: int = 1000,
    tol: float = 1e-4,
    fit_intercept: bool = True,
)

model.coef_           -> list[float]
model.intercept_      -> float
model.n_iter_         -> int
model.alpha_          -> float
model.l1_ratio_       -> float
model.max_iter_       -> int
model.tol_            -> float
model.fit_intercept_  -> bool
```

---

## Description

Combines L1 and L2 penalties. `l1_ratio=1` → Lasso, `l1_ratio=0` → Ridge. Solved via coordinate descent.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | Overall penalty strength |
| `l1_ratio` | `float` | `0.5` | Mix between L1 and L2 |
| `max_iter` | `int` | `1000` | Maximum iterations |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Add bias term |

</div>

<div class="lang-fr">

## Description

Combine les pénalités L1 et L2. `l1_ratio=1` → Lasso, `l1_ratio=0` → Ridge. Résolu par descente de coordonnées.

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `alpha` | `float` | `1.0` | Force globale de pénalité |
| `l1_ratio` | `float` | `0.5` | Mélange entre L1 et L2 |
| `max_iter` | `int` | `1000` | Nombre maximum d'itérations |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajouter un biais |

</div>


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
