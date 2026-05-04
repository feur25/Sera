# LogisticRegression

<div class="lang-en">

## API Reference

**Signature**

```python
model = sp.LogisticRegression(
    C=1.0,
    max_iter=1000,
    tol=1e-4,
    fit_intercept=True,
)

model.fit(X, y)
model.predict(X)        -> list[int]
model.predict_proba(X)  -> ndarray (n, K)
model.score(X, y)       -> float
model.get_params()      -> dict
model.set_params(C=..., max_iter=..., tol=..., fit_intercept=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `C` | `float` | `1.0` | Inverse regularisation strength (larger = less regularisation) |
| `max_iter` | `int` | `1000` | Maximum L-BFGS iterations |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Fit a bias term |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `ndarray` | Fitted coefficients — shape `(p,)` binary, `(K, p)` multiclass |
| `intercept_` | `float \| ndarray` | Bias term(s) |
| `classes_` | `list[int]` | Unique class labels |
| `n_iter_` | `int` | Actual iterations performed |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

model = sp.LogisticRegression(C=1.0, max_iter=1000)
model.fit(X, y)
print(f"Accuracy: {model.score(X, y):.4f}")
proba = model.predict_proba(X)
print(f"Shape: {proba.shape}")   # (400, 2)
```

</details>

---

## Algorithmic Functioning

Logistic Regression fits a linear decision boundary using the **sigmoid** function:

<div>$$\sigma(z) = \frac{1}{1 + e^{-z}}, \qquad z = x^T\beta + \beta_0$$</div>

The model minimises the **cross-entropy loss** with L2 regularisation:

<div>$$\mathcal{L}(\beta) = -\frac{1}{n}\sum_{i=1}^n \left[y_i \log \hat{p}_i + (1-y_i)\log(1-\hat{p}_i)\right] + \frac{1}{2C}\|\beta\|_2^2$$</div>

**Optimiser** — parameters are updated via **L-BFGS** (Limited-memory Broyden-Fletcher-Goldfarb-Shanno), a quasi-Newton method that approximates the inverse Hessian using the last $m$ gradient differences.

**Multiclass** (OvR) — for $K > 2$ classes, $K$ binary classifiers are trained independently:

<div>$$\hat{p}_k(x) = \sigma\!\left(x^T\beta_k + \beta_{0,k}\right)$$</div>

<div>$$\hat{y} = \underset{k}{\arg\max}\ \sigma(x^T\beta_k)$$</div>

</div>

<div class="lang-fr">

## Référence API

**Signature**

```python
model = sp.LogisticRegression(
    C=1.0,
    max_iter=1000,
    tol=1e-4,
    fit_intercept=True,
)

model.fit(X, y)
model.predict(X)        -> list[int]
model.predict_proba(X)  -> ndarray (n, K)
model.score(X, y)       -> float
model.get_params()      -> dict
model.set_params(C=..., max_iter=..., tol=..., fit_intercept=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `C` | `float` | `1.0` | Inverse de la force de régularisation (plus grand = moins de régularisation) |
| `max_iter` | `int` | `1000` | Nombre maximum d'itérations L-BFGS |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `coef_` | `ndarray` | Coefficients ajustés — `(p,)` binaire, `(K, p)` multiclasse |
| `intercept_` | `float \| ndarray` | Terme(s) de biais |
| `classes_` | `list[int]` | Labels de classes uniques |
| `n_iter_` | `int` | Nombre d'itérations réalisées |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

model = sp.LogisticRegression(C=1.0, max_iter=1000)
model.fit(X, y)
print(f"Précision : {model.score(X, y):.4f}")
proba = model.predict_proba(X)
print(f"Forme : {proba.shape}")
```

</details>

---

## Fonctionnement algorithmique

La régression logistique ajuste une frontière de décision linéaire à l'aide de la fonction **sigmoïde** :

<div>$$\sigma(z) = \frac{1}{1 + e^{-z}}, \qquad z = x^T\beta + \beta_0$$</div>

Le modèle minimise la **perte d'entropie croisée** avec régularisation L2 :

<div>$$\mathcal{L}(\beta) = -\frac{1}{n}\sum_{i=1}^n \left[y_i \log \hat{p}_i + (1-y_i)\log(1-\hat{p}_i)\right] + \frac{1}{2C}\|\beta\|_2^2$$</div>

**Optimiseur** — L-BFGS, méthode quasi-Newton approximant l'inverse de la Hessienne.

**Multi-classe** (OvR) — pour $K > 2$ classes, $K$ classificateurs binaires :

<div>$$\hat{p}_k(x) = \sigma\!\left(x^T\beta_k + \beta_{0,k}\right)$$</div>

<div>$$\hat{y} = \underset{k}{\arg\max}\ \sigma(x^T\beta_k)$$</div>

</div>
