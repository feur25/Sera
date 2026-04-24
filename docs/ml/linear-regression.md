# LinearRegression

<div class="lang-en">

## API Reference

**Signature**

```python
model = sp.LinearRegression(fit_intercept=True)

model.fit(X, y)
model.predict(X)   -> list[float]
model.score(X, y)  -> float
model.get_params() -> dict
model.set_params(fit_intercept=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `fit_intercept` | `bool` | `True` | Fit a bias term |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients, shape $(p,)$ |
| `intercept_` | `float` | Bias term (0 if `fit_intercept=False`) |
| `fit_intercept_` | `bool` | Whether a bias was fitted |
| `n_features_in_` | `int` | Number of input features |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
y = X @ np.array([2.0, -1.0, 0.5]) + np.random.randn(500) * 0.1

model = sp.LinearRegression()
model.fit(X, y)
print(f"R2: {model.score(X, y):.6f}")
print(f"coef: {model.coef_}")
```

</details>

---

## Algorithmic Functioning

Ordinary Least Squares minimises the residual sum of squares:

<div>$$\hat{\beta} = \underset{\beta}{\arg\min}\ \|y - X\beta\|_2^2$$</div>

The closed-form solution is the **normal equation**:

<div>$$\hat{\beta} = (X^TX)^{-1}X^T y$$</div>

**Solver** — The Gram matrix $G = X^TX \in \mathbb{R}^{p \times p}$ is factored via **Cholesky** ($G = LL^T$). If $G$ is not positive-definite the solver falls back to **QR decomposition** of $X$.

When `fit_intercept=True`, $X$ is first augmented with a column of ones:

<div>$$X_{\text{aug}} = \begin{bmatrix} 1 & x_{1}^T \\ \vdots & \vdots \\ 1 & x_{n}^T \end{bmatrix} \in \mathbb{R}^{n \times (p+1)}$$</div>

The intercept $\hat{\beta}_0$ and coefficients $\hat{\beta}_{1:p}$ are then extracted from the joint solution. The intercept is never regularised.

**Score** returns the coefficient of determination:

<div>$$R^2 = 1 - \frac{\displaystyle\sum_{i=1}^n (y_i - \hat{y}_i)^2}{\displaystyle\sum_{i=1}^n (y_i - \bar{y})^2}$$</div>

</div>

<div class="lang-fr">

## Référence API

**Signature**

```python
model = sp.LinearRegression(fit_intercept=True)

model.fit(X, y)
model.predict(X)   -> list[float]
model.score(X, y)  -> float
model.get_params() -> dict
model.set_params(fit_intercept=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `coef_` | `list[float]` | Coefficients ajustés, forme $(p,)$ |
| `intercept_` | `float` | Terme de biais (0 si `fit_intercept=False`) |
| `fit_intercept_` | `bool` | Si le biais a été ajusté |
| `n_features_in_` | `int` | Nombre de variables d'entrée |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
y = X @ np.array([2.0, -1.0, 0.5]) + np.random.randn(500) * 0.1

model = sp.LinearRegression()
model.fit(X, y)
print(f"R2 : {model.score(X, y):.6f}")
print(f"coef : {model.coef_}")
```

</details>

---

## Fonctionnement algorithmique

La régression par moindres carrés ordinaires minimise la somme des carrés des résidus :

<div>$$\hat{\beta} = \underset{\beta}{\arg\min}\ \|y - X\beta\|_2^2$$</div>

La solution exacte est l'**équation normale** :

<div>$$\hat{\beta} = (X^TX)^{-1}X^T y$$</div>

**Solveur** — La matrice de Gram $G = X^TX \in \mathbb{R}^{p \times p}$ est factorisée via **Cholesky** ($G = LL^T$). Si $G$ n'est pas définie positive, le solveur bascule sur la **décomposition QR** de $X$.

Avec `fit_intercept=True`, $X$ est d'abord augmentée d'une colonne de uns :

<div>$$X_{\text{aug}} = \begin{bmatrix} 1 & x_{1}^T \\ \vdots & \vdots \\ 1 & x_{n}^T \end{bmatrix} \in \mathbb{R}^{n \times (p+1)}$$</div>

Le biais $\hat{\beta}_0$ et les coefficients $\hat{\beta}_{1:p}$ sont extraits de la solution jointe. Le biais n'est jamais régularisé.

**Score** retourne le coefficient de détermination :

<div>$$R^2 = 1 - \frac{\displaystyle\sum_{i=1}^n (y_i - \hat{y}_i)^2}{\displaystyle\sum_{i=1}^n (y_i - \bar{y})^2}$$</div>

</div>