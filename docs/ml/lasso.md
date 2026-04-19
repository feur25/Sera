# Lasso

<div class="lang-en">

## Code

**Signature**

```python
model = sp.Lasso(alpha=1.0, max_iter=1000, tol=1e-4, fit_intercept=True)

model.fit(X, y, checkpoint_id=None)
model.predict(X)   -> list[float]
model.score(X, y)  -> float
model.get_params() -> dict
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | L1 regularisation strength |
| `max_iter` | `int` | `1000` | Maximum number of coordinate descent passes |
| `tol` | `float` | `1e-4` | Convergence tolerance on coefficient updates |
| `fit_intercept` | `bool` | `True` | Fit a bias term |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients, shape $(p,)$ — many will be exactly 0 |
| `intercept_` | `float` | Bias term |
| `alpha_` | `float` | Regularisation parameter |
| `n_iter_` | `int` | Actual number of iterations run |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(200, 20)
true_coef = np.zeros(20)
true_coef[:3] = [3.0, -2.0, 1.5]
y = X @ true_coef + np.random.randn(200) * 0.5

model = sp.Lasso(alpha=0.1)
model.fit(X, y)
print(f"Non-zero: {(np.abs(model.coef_) > 1e-6).sum()}")
print(f"R²: {model.score(X, y):.4f}")
```

</details>

---

## Algorithmic Functioning

Lasso (Least Absolute Shrinkage and Selection Operator) adds an **L1 penalty** that drives many coefficients to exactly zero, performing automatic feature selection:

$$\hat{\beta} = \underset{\beta}{\arg\min}\ \frac{1}{2n}\|y - X\beta\|_2^2 + \alpha\|\beta\|_1$$

**Coordinate Descent** — Since the L1 penalty is not differentiable at 0, Lasso is solved by cycling over each coordinate $j$ and analytically minimising the objective with all others fixed. The partial residual excluding feature $j$ is:

$$r_{ij} = y_i - \sum_{k \neq j} x_{ik}\beta_k$$

The optimal $\beta_j$ in closed form uses the **soft-thresholding** operator $\mathcal{S}$:

$$\beta_j \leftarrow \frac{\mathcal{S}\!\left(\frac{1}{n}\sum_{i=1}^n x_{ij}\,r_{ij},\ \alpha\right)}{\frac{1}{n}\sum_{i=1}^n x_{ij}^2}$$

where the soft-threshold function is:

$$\mathcal{S}(z, \lambda) = \operatorname{sign}(z)\max\!\bigl(|z| - \lambda,\ 0\bigr)$$

When $\left|\frac{1}{n}\sum_i x_{ij} r_{ij}\right| \leq \alpha$, the coordinate is set to exactly zero — this is the sparsity-inducing mechanism of L1.

**Convergence** — Iterations stop when $\max_j |\beta_j^{(t)} - \beta_j^{(t-1)}| < \texttt{tol}$ or after `max_iter` passes. The `checkpoint_id` argument allows resumable training across Python calls.

</div>

<div class="lang-fr">

## Code

**Signature**

```python
model = sp.Lasso(alpha=1.0, max_iter=1000, tol=1e-4, fit_intercept=True)

model.fit(X, y, checkpoint_id=None)
model.predict(X)   -> list[float]
model.score(X, y)  -> float
model.get_params() -> dict
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `alpha` | `float` | `1.0` | Force de régularisation L1 |
| `max_iter` | `int` | `1000` | Nombre maximum de passes de descente par coordonnées |
| `tol` | `float` | `1e-4` | Tolérance de convergence sur les mises à jour des coefficients |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `coef_` | `list[float]` | Coefficients ajustés — beaucoup seront exactement à 0 |
| `intercept_` | `float` | Terme de biais |
| `alpha_` | `float` | Paramètre de régularisation |
| `n_iter_` | `int` | Nombre réel d'itérations effectuées |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(200, 20)
true_coef = np.zeros(20)
true_coef[:3] = [3.0, -2.0, 1.5]
y = X @ true_coef + np.random.randn(200) * 0.5

model = sp.Lasso(alpha=0.1)
model.fit(X, y)
print(f"Non nuls : {(np.abs(model.coef_) > 1e-6).sum()}")
print(f"R² : {model.score(X, y):.4f}")
```

</details>

---

## Fonctionnement algorithmique

Lasso ajoute une **pénalité L1** qui pousse de nombreux coefficients exactement à zéro, réalisant ainsi une sélection automatique de variables :

$$\hat{\beta} = \underset{\beta}{\arg\min}\ \frac{1}{2n}\|y - X\beta\|_2^2 + \alpha\|\beta\|_1$$

**Descente par coordonnées** — Comme la pénalité L1 n'est pas différentiable en 0, Lasso est résolu en cyclant sur chaque coordonnée $j$ et en minimisant analytiquement l'objectif avec toutes les autres fixées. Le résidu partiel excluant la variable $j$ est :

$$r_{ij} = y_i - \sum_{k \neq j} x_{ik}\beta_k$$

Le $\beta_j$ optimal en forme fermée utilise l'opérateur de **seuillage doux** $\mathcal{S}$ :

$$\beta_j \leftarrow \frac{\mathcal{S}\!\left(\frac{1}{n}\sum_{i=1}^n x_{ij}\,r_{ij},\ \alpha\right)}{\frac{1}{n}\sum_{i=1}^n x_{ij}^2}$$

où la fonction de seuillage doux est :

$$\mathcal{S}(z, \lambda) = \operatorname{sign}(z)\max\!\bigl(|z| - \lambda,\ 0\bigr)$$

Quand $\left|\frac{1}{n}\sum_i x_{ij} r_{ij}\right| \leq \alpha$, la coordonnée est mise exactement à zéro — c'est le mécanisme inducteur de parcimonie de L1.

**Convergence** — Les itérations s'arrêtent quand $\max_j |\beta_j^{(t)} - \beta_j^{(t-1)}| < \texttt{tol}$ ou après `max_iter` passes. L'argument `checkpoint_id` permet un entraînement repris entre plusieurs appels Python.

</div>
# Lasso

<div class="lang-en">

## Signature

```python
model = sp.Lasso(
    alpha: float = 1.0,
    max_iter: int = 1000,
    tol: float = 1e-4,
    fit_intercept: bool = True,
)

model.coef_      -> list[float]
model.intercept_ -> float
model.alpha_     -> float
model.n_iter_    -> int
```

---

## Description

L1-regularized regression via coordinate descent. Produces sparse solutions.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | L1 penalty strength |
| `max_iter` | `int` | `1000` | Maximum iterations |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Add bias term |

</div>

<div class="lang-fr">

## Description

Régression avec régularisation L1 par descente de coordonnées. Produit des solutions creuses.

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `alpha` | `float` | `1.0` | Force de pénalité L1 |
| `max_iter` | `int` | `1000` | Nombre maximum d'itérations |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajouter un biais |

</div>


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
