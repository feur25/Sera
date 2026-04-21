# Lasso

<div class="lang-en">

## API Reference

**Signature**

```python
model = sp.Lasso(alpha=1.0, max_iter=1000, tol=1e-4, fit_intercept=True)

model.fit(X, y)
model.predict(X)   -> list[float]
model.score(X, y)  -> float
model.get_params() -> dict
model.set_params(alpha=..., max_iter=..., tol=..., fit_intercept=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | L1 penalty strength |
| `max_iter` | `int` | `1000` | Maximum coordinate descent iterations |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Fit a bias term |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients (sparse) |
| `intercept_` | `float` | Bias term |
| `alpha_` | `float` | Regularisation parameter |
| `n_iter_` | `int` | Actual iterations performed |

<details>
<summary><strong>Example</strong></summary>

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
print(f"R2: {model.score(X, y):.4f}")
print(f"Non-zero coefficients: {non_zero} / 20")
```

</details>

---

## Algorithmic Functioning

Lasso minimises the L1-penalised objective:

$$\hat{\beta} = \underset{\beta}{\arg\min}\ \frac{1}{2n}\|y - X\beta\|_2^2 + \alpha\|\beta\|_1$$

**Coordinate descent** updates one coefficient at a time while holding the rest fixed. For each $j$:

$$r_j = y - X_{-j}\hat{\beta}_{-j}$$

$$\hat{\beta}_j \leftarrow \frac{S\!\left(X_j^T r_j / n,\ \alpha\right)}{\|X_j\|_2^2 / n}$$

where $S(\cdot, \lambda)$ is the **soft-threshold operator**:

$$S(z, \lambda) = \text{sign}(z)\max(|z| - \lambda, 0)$$

This sets small coefficients exactly to zero, producing **sparse solutions**.

**Convergence** — iterations stop when $\max_j |\beta_j^{(t)} - \beta_j^{(t-1)}| < \texttt{tol}$ or after `max_iter` passes. The `checkpoint_id` argument enables training resumed across multiple Python calls.

</div>

<div class="lang-fr">

## Référence API

**Signature**

```python
model = sp.Lasso(alpha=1.0, max_iter=1000, tol=1e-4, fit_intercept=True)

model.fit(X, y)
model.predict(X)   -> list[float]
model.score(X, y)  -> float
model.get_params() -> dict
model.set_params(alpha=..., max_iter=..., tol=..., fit_intercept=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `alpha` | `float` | `1.0` | Force de pénalité L1 |
| `max_iter` | `int` | `1000` | Nombre maximum d'itérations de descente de coordonnées |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `coef_` | `list[float]` | Coefficients ajustés (creux) |
| `intercept_` | `float` | Terme de biais |
| `alpha_` | `float` | Paramètre de régularisation |
| `n_iter_` | `int` | Nombre d'itérations réalisées |

<details>
<summary><strong>Exemple</strong></summary>

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
print(f"R2 : {model.score(X, y):.4f}")
print(f"Coefficients non nuls : {non_zero} / 20")
```

</details>

---

## Fonctionnement algorithmique

Lasso minimise l'objectif avec pénalité L1 :

$$\hat{\beta} = \underset{\beta}{\arg\min}\ \frac{1}{2n}\|y - X\beta\|_2^2 + \alpha\|\beta\|_1$$

La **descente de coordonnées** met à jour un coefficient à la fois en fixant les autres. Pour chaque $j$ :

$$r_j = y - X_{-j}\hat{\beta}_{-j}$$

$$\hat{\beta}_j \leftarrow \frac{S\!\left(X_j^T r_j / n,\ \alpha\right)}{\|X_j\|_2^2 / n}$$

où $S(\cdot, \lambda)$ est l'**opérateur de seuillage doux** :

$$S(z, \lambda) = \text{sign}(z)\max(|z| - \lambda, 0)$$

Cela annule exactement les petits coefficients, produisant des **solutions creuses**.

**Convergence** — Les itérations s'arrêtent quand $\max_j |\beta_j^{(t)} - \beta_j^{(t-1)}| < \texttt{tol}$ ou après `max_iter` passes. L'argument `checkpoint_id` permet un entraînement repris entre plusieurs appels Python.

</div>