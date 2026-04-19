# LogisticRegression

<div class="lang-en">

## Code

**Signature**

```python
model = sp.LogisticRegression(c=1.0, max_iter=1000, tol=1e-4, fit_intercept=True)

model.fit(X, y, checkpoint_id=None)
model.predict(X)        -> list[int]
model.predict_proba(X)  -> ndarray (n, K)
model.score(X, y)       -> float
model.get_params()      -> dict
model.set_params(C=..., max_iter=..., tol=..., fit_intercept=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `c` | `float` | `1.0` | Inverse regularisation strength — larger values = less regularisation |
| `max_iter` | `int` | `1000` | Maximum L-BFGS iterations |
| `tol` | `float` | `1e-4` | Gradient norm convergence tolerance |
| `fit_intercept` | `bool` | `True` | Fit a bias term |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `ndarray` | Coefficients — shape $(p,)$ for binary, $(K, p)$ for multiclass |
| `intercept_` | `float \| ndarray` | Bias — scalar for binary, $(K,)$ for multiclass |
| `classes_` | `list[int]` | Unique class labels |
| `n_iter_` | `int` | Iterations run |
| `C_` | `float` | Inverse regularisation strength |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

model = sp.LogisticRegression(c=1.0)
model.fit(X, y)
print(f"Accuracy: {model.score(X, y):.4f}")
proba = model.predict_proba(X)
print(f"Proba shape: {proba.shape}")
```

</details>

---

## Algorithmic Functioning

Logistic regression models the **log-odds** of class membership as a linear function:

$$\log\frac{P(y=1 \mid x)}{P(y=0 \mid x)} = x^T\beta + \beta_0$$

which implies the **sigmoid** link:

$$P(y=1 \mid x) = \sigma(x^T\beta) = \frac{1}{1 + e^{-x^T\beta}}$$

**Regularised log-likelihood objective** (L2, binary case):

$$\mathcal{L}(\beta) = \sum_{i=1}^n \bigl[y_i \log \sigma(x_i^T\beta) + (1-y_i) \log(1 - \sigma(x_i^T\beta))\bigr] - \frac{1}{2C}\|\beta\|_2^2$$

**Optimisation** — The gradient is:

$$\nabla_\beta \mathcal{L} = X^T(y - \hat{p}) - \frac{\beta}{C}, \qquad \hat{p}_i = \sigma(x_i^T\beta)$$

This is maximised via **L-BFGS** (Limited-memory Broyden–Fletcher–Goldfarb–Shanno), a quasi-Newton method that maintains a low-rank approximation of the Hessian inverse using the last $m$ gradient differences.

**Multiclass (OvR)** — For $K > 2$ classes, $K$ independent binary classifiers are trained (One-vs-Rest). Probabilities are normalised:

$$P(y=k \mid x) = \frac{\sigma(x^T\beta_k)}{\sum_{j=1}^K \sigma(x^T\beta_j)}$$

**Prediction** assigns the class with highest probability:

$$\hat{y} = \underset{k}{\arg\max}\ \sigma(x^T\beta_k)$$

</div>

<div class="lang-fr">

## Code

**Signature**

```python
model = sp.LogisticRegression(c=1.0, max_iter=1000, tol=1e-4, fit_intercept=True)

model.fit(X, y, checkpoint_id=None)
model.predict(X)        -> list[int]
model.predict_proba(X)  -> ndarray (n, K)
model.score(X, y)       -> float
model.get_params()      -> dict
model.set_params(C=..., max_iter=..., tol=..., fit_intercept=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `c` | `float` | `1.0` | Inverse de la force de régularisation — plus grand = moins de régularisation |
| `max_iter` | `int` | `1000` | Nombre maximum d'itérations L-BFGS |
| `tol` | `float` | `1e-4` | Tolérance de convergence sur la norme du gradient |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `coef_` | `ndarray` | Coefficients — forme $(p,)$ binaire, $(K, p)$ multiclasse |
| `intercept_` | `float \| ndarray` | Biais — scalaire binaire, $(K,)$ multiclasse |
| `classes_` | `list[int]` | Labels de classes uniques |
| `n_iter_` | `int` | Itérations effectuées |
| `C_` | `float` | Inverse de la force de régularisation |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

model = sp.LogisticRegression(c=1.0)
model.fit(X, y)
print(f"Précision : {model.score(X, y):.4f}")
proba = model.predict_proba(X)
print(f"Forme proba : {proba.shape}")
```

</details>

---

## Fonctionnement algorithmique

La régression logistique modélise le **log-odds** d'appartenance à une classe comme une fonction linéaire :

$$\log\frac{P(y=1 \mid x)}{P(y=0 \mid x)} = x^T\beta + \beta_0$$

ce qui implique la **fonction sigmoïde** :

$$P(y=1 \mid x) = \sigma(x^T\beta) = \frac{1}{1 + e^{-x^T\beta}}$$

**Objectif log-vraisemblance régularisé** (L2, cas binaire) :

$$\mathcal{L}(\beta) = \sum_{i=1}^n \bigl[y_i \log \sigma(x_i^T\beta) + (1-y_i) \log(1 - \sigma(x_i^T\beta))\bigr] - \frac{1}{2C}\|\beta\|_2^2$$

**Optimisation** — Le gradient est :

$$\nabla_\beta \mathcal{L} = X^T(y - \hat{p}) - \frac{\beta}{C}, \qquad \hat{p}_i = \sigma(x_i^T\beta)$$

Maximisé via **L-BFGS** (Limited-memory Broyden–Fletcher–Goldfarb–Shanno), une méthode quasi-Newton qui maintient une approximation rang réduit de l'inverse du Hessien à partir des $m$ dernières différences de gradients.

**Multiclasse (OvR)** — Pour $K > 2$ classes, $K$ classificateurs binaires indépendants sont entraînés (One-vs-Rest). Les probabilités sont normalisées :

$$P(y=k \mid x) = \frac{\sigma(x^T\beta_k)}{\sum_{j=1}^K \sigma(x^T\beta_j)}$$

**Prédiction** affecte la classe avec la probabilité la plus haute :

$$\hat{y} = \underset{k}{\arg\max}\ \sigma(x^T\beta_k)$$

</div>
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
