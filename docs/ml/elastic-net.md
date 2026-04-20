# ElasticNet

<div class="lang-en">

## Code

**Signature**

```python
model = sp.ElasticNet(
    alpha=1.0,
    l1_ratio=0.5,
    max_iter=1000,
    tol=1e-4,
    fit_intercept=True,
)

model.fit(X, y)
model.predict(X)   -> list[float]
model.score(X, y)  -> float
model.get_params() -> dict
model.set_params(alpha=..., l1_ratio=..., max_iter=..., tol=..., fit_intercept=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | Overall penalty strength |
| `l1_ratio` | `float` | `0.5` | Mix between L1 and L2: 0 = pure Ridge, 1 = pure Lasso |
| `max_iter` | `int` | `1000` | Maximum coordinate descent iterations |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Fit a bias term |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients |
| `intercept_` | `float` | Bias term |
| `n_iter_` | `int` | Actual iterations performed |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 15)
true_coef = np.zeros(15)
true_coef[:5] = [2.0, -1.5, 1.0, -0.5, 0.8]
y = X @ true_coef + np.random.randn(400) * 0.3

model = sp.ElasticNet(alpha=0.1, l1_ratio=0.7)
model.fit(X, y)
non_zero = sum(1 for c in model.coef_ if abs(c) > 1e-6)
print(f"R2: {model.score(X, y):.4f}")
print(f"Non-zero: {non_zero} / 15")
```

</details>

---

## Algorithmic Functioning

ElasticNet combines both L1 and L2 penalties:

$$\hat{\beta} = \underset{\beta}{\arg\min}\ \frac{1}{2n}\|y - X\beta\|_2^2 + \alpha\!\left[\rho\|\beta\|_1 + \frac{1-\rho}{2}\|\beta\|_2^2\right]$$

where $\rho$ = `l1_ratio`. Setting $\rho = 1$ recovers Lasso; $\rho = 0$ recovers Ridge.

**Coordinate descent** update for coefficient $j$:

$$\hat{\beta}_j \leftarrow \frac{S\!\left(X_j^T r_j / n,\ \alpha\rho\right)}{\|X_j\|_2^2 / n + \alpha(1-\rho)}$$

where $S(\cdot, \lambda)$ is the soft-threshold operator. The L2 term appears in the denominator, providing grouping behaviour: correlated features tend to receive similar non-zero coefficients.

| `l1_ratio` | Behaviour |
|------------|-----------|
| `1` | Lasso — sparse solutions |
| `0` | Ridge — dense, shrunk solutions |
| `(0, 1)` | ElasticNet — sparse + stable |

The `checkpoint_id` argument enables training resumed across multiple Python calls.

</div>

<div class="lang-fr">

## Code

**Signature**

```python
model = sp.ElasticNet(
    alpha=1.0,
    l1_ratio=0.5,
    max_iter=1000,
    tol=1e-4,
    fit_intercept=True,
)

model.fit(X, y)
model.predict(X)   -> list[float]
model.score(X, y)  -> float
model.get_params() -> dict
model.set_params(alpha=..., l1_ratio=..., max_iter=..., tol=..., fit_intercept=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `alpha` | `float` | `1.0` | Force globale de pénalité |
| `l1_ratio` | `float` | `0.5` | Mélange L1/L2 : 0 = Ridge pur, 1 = Lasso pur |
| `max_iter` | `int` | `1000` | Nombre maximum d'itérations |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `coef_` | `list[float]` | Coefficients ajustés |
| `intercept_` | `float` | Terme de biais |
| `n_iter_` | `int` | Nombre d'itérations réalisées |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 15)
true_coef = np.zeros(15)
true_coef[:5] = [2.0, -1.5, 1.0, -0.5, 0.8]
y = X @ true_coef + np.random.randn(400) * 0.3

model = sp.ElasticNet(alpha=0.1, l1_ratio=0.7)
model.fit(X, y)
non_zero = sum(1 for c in model.coef_ if abs(c) > 1e-6)
print(f"R2 : {model.score(X, y):.4f}")
print(f"Non nuls : {non_zero} / 15")
```

</details>

---

## Fonctionnement algorithmique

ElasticNet combine les pénalités L1 et L2 :

$$\hat{\beta} = \underset{\beta}{\arg\min}\ \frac{1}{2n}\|y - X\beta\|_2^2 + \alpha\!\left[\rho\|\beta\|_1 + \frac{1-\rho}{2}\|\beta\|_2^2\right]$$

où $\rho$ = `l1_ratio`. Avec $\rho = 1$ on retrouve Lasso ; avec $\rho = 0$ on retrouve Ridge.

**Descente de coordonnées** pour le coefficient $j$ :

$$\hat{\beta}_j \leftarrow \frac{S\!\left(X_j^T r_j / n,\ \alpha\rho\right)}{\|X_j\|_2^2 / n + \alpha(1-\rho)}$$

où $S(\cdot, \lambda)$ est l'opérateur de seuillage doux. Le terme L2 apparaît au dénominateur, favorisant le **regroupement** : les variables corrélées tendent à recevoir des coefficients non nuls similaires.

| `l1_ratio` | Comportement |
|------------|--------------|
| `1` | Lasso — solutions creuses |
| `0` | Ridge — solutions denses et rétrécies |
| `(0, 1)` | ElasticNet — creux + stable |

L'argument `checkpoint_id` permet un entraînement repris : appeler `fit` à nouveau avec le même id reprend depuis le dernier checkpoint.

</div>