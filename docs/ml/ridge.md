# Ridge / RidgeClassifier

<div class="lang-en">

## Code

**Signature**

```python
reg = sp.Ridge(alpha=1.0, fit_intercept=True)
clf = sp.RidgeClassifier(alpha=1.0)

model.fit(X, y)
model.predict(X)   -> list[float] | list[int]
model.score(X, y)  -> float
model.get_params() -> dict
model.set_params(alpha=..., fit_intercept=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | L2 regularisation strength — larger values shrink coefficients more |
| `fit_intercept` | `bool` | `True` | Fit a bias term (Ridge only) |

**Attributes — Ridge**

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients, shape $(p,)$ |
| `intercept_` | `float` | Bias term |
| `alpha_` | `float` | Regularisation parameter |
| `fit_intercept_` | `bool` | Whether a bias was fitted |

**Attributes — RidgeClassifier**

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients |
| `intercept_` | `float` | Bias term |
| `classes_` | `list[int]` | Unique class labels |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 5)
y = X @ np.array([1.0, -2.0, 0.5, 1.5, -0.8]) + np.random.randn(300)

reg = sp.Ridge(alpha=0.5)
reg.fit(X, y)
print(f"R2: {reg.score(X, y):.4f}")

clf = sp.RidgeClassifier(alpha=1.0)
clf.fit(X, (y > 0).astype(int))
print(f"Accuracy: {clf.score(X, (y > 0).astype(int)):.4f}")
```

</details>

---

## Algorithmic Functioning

Ridge adds an **L2 penalty** to the OLS objective to shrink coefficients toward zero:

$$\hat{\beta} = \underset{\beta}{\arg\min}\ \|y - X\beta\|_2^2 + \alpha\|\beta\|_2^2$$

The closed-form solution is:

$$\hat{\beta} = (X^TX + \alpha I)^{-1}X^T y$$

The ridge term $\alpha I$ shifts all eigenvalues of $X^TX$ upward by $\alpha$, guaranteeing the matrix is positive-definite and invertible regardless of multicollinearity. The solution is computed via **Cholesky decomposition** of $(X^TX + \alpha I)$.

When `fit_intercept=True`, $X$ is centered before regularisation:

$$\hat{\beta} = (X^TX + \alpha I_p)^{-1}X^T y, \qquad \hat{\beta}_0 = \bar{y} - \bar{x}^T\hat{\beta}$$

**RidgeClassifier** encodes multi-class labels as a binary indicator matrix $Y \in \{0,1\}^{n \times K}$, solves Ridge regression jointly, and assigns:

$$\hat{y} = \underset{k}{\arg\max}\ \hat{Y}_{:,k}$$

The variance-bias trade-off is controlled by $\alpha$: larger $\alpha$ increases bias but reduces variance.

</div>

<div class="lang-fr">

## Code

**Signature**

```python
reg = sp.Ridge(alpha=1.0, fit_intercept=True)
clf = sp.RidgeClassifier(alpha=1.0)

model.fit(X, y)
model.predict(X)   -> list[float] | list[int]
model.score(X, y)  -> float
model.get_params() -> dict
model.set_params(alpha=..., fit_intercept=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `alpha` | `float` | `1.0` | Force de régularisation L2 — plus $\alpha$ est grand, plus les coefficients sont pénalisés |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais (Ridge uniquement) |

**Attributs — Ridge**

| Attribut | Type | Description |
|----------|------|-------------|
| `coef_` | `list[float]` | Coefficients ajustés, forme $(p,)$ |
| `intercept_` | `float` | Terme de biais |
| `alpha_` | `float` | Paramètre de régularisation |
| `fit_intercept_` | `bool` | Si le biais a été ajusté |

**Attributs — RidgeClassifier**

| Attribut | Type | Description |
|----------|------|-------------|
| `coef_` | `list[float]` | Coefficients ajustés |
| `intercept_` | `float` | Terme de biais |
| `classes_` | `list[int]` | Labels de classes uniques |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 5)
y = X @ np.array([1.0, -2.0, 0.5, 1.5, -0.8]) + np.random.randn(300)

reg = sp.Ridge(alpha=0.5)
reg.fit(X, y)
print(f"R2 : {reg.score(X, y):.4f}")

clf = sp.RidgeClassifier(alpha=1.0)
clf.fit(X, (y > 0).astype(int))
print(f"Précision : {clf.score(X, (y > 0).astype(int)):.4f}")
```

</details>

---

## Fonctionnement algorithmique

Ridge ajoute une **pénalité L2** à l'objectif des moindres carrés pour rétrécir les coefficients vers zéro :

$$\hat{\beta} = \underset{\beta}{\arg\min}\ \|y - X\beta\|_2^2 + \alpha\|\beta\|_2^2$$

La solution exacte est :

$$\hat{\beta} = (X^TX + \alpha I)^{-1}X^T y$$

Le terme $\alpha I$ décale vers le haut toutes les valeurs propres de $X^TX$, garantissant que la matrice est définie positive et donc inversible même en cas de multicolinéarité. La solution est calculée via **décomposition de Cholesky** de $(X^TX + \alpha I)$.

Avec `fit_intercept=True`, $X$ est centrée avant régularisation :

$$\hat{\beta} = (X^TX + \alpha I_p)^{-1}X^T y, \qquad \hat{\beta}_0 = \bar{y} - \bar{x}^T\hat{\beta}$$

**RidgeClassifier** encode les labels multi-classes dans une matrice indicatrice $Y \in \{0,1\}^{n \times K}$, résout Ridge de façon jointe, puis affecte :

$$\hat{y} = \underset{k}{\arg\max}\ \hat{Y}_{:,k}$$

Le compromis biais-variance est contrôlé par $\alpha$ : un $\alpha$ plus grand augmente le biais mais réduit la variance.

</div>