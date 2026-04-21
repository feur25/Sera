# LogisticRegression

<div class="lang-en">

## API Reference

**Signature**

```python
model = sp.LogisticRegression(
    C=1.0,
    max_iter=100,
    tol=1e-4,
    fit_intercept=True,
    multi_class="ovr",
)

model.fit(X, y)
model.predict(X)        -> list[int]
model.predict_proba(X)  -> list[list[float]]
model.score(X, y)       -> float
model.get_params()      -> dict
model.set_params(C=..., max_iter=..., tol=..., fit_intercept=..., multi_class=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `C` | `float` | `1.0` | Inverse regularisation strength (larger = less regularisation) |
| `max_iter` | `int` | `100` | Maximum L-BFGS iterations |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Fit a bias term |
| `multi_class` | `str` | `"ovr"` | Strategy for multi-class: `"ovr"` (One-vs-Rest) |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients |
| `intercept_` | `float` | Bias term |
| `classes_` | `list[int]` | Unique class labels |
| `n_iter_` | `int` | Actual iterations performed |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

model = sp.LogisticRegression(C=1.0, max_iter=200)
model.fit(X, y)
print(f"Accuracy: {model.score(X, y):.4f}")
proba = model.predict_proba(X)
print(f"P(class=1) sample 0: {proba[0][1]:.4f}")
```

</details>

---

## Algorithmic Functioning

Logistic Regression fits a linear decision boundary using the **sigmoid** function:

$$\sigma(z) = \frac{1}{1 + e^{-z}}, \qquad z = x^T\beta + \beta_0$$

The model minimises the **cross-entropy loss** with L2 regularisation:

$$\mathcal{L}(\beta) = -\frac{1}{n}\sum_{i=1}^n \left[y_i \log \hat{p}_i + (1-y_i)\log(1-\hat{p}_i)\right] + \frac{1}{2C}\|\beta\|_2^2$$

**Optimiser** — parameters are updated via **L-BFGS** (Limited-memory Broyden-Fletcher-Goldfarb-Shanno), a quasi-Newton method that approximates the inverse Hessian using the last $m$ gradient differences.

**Multiclass** (OvR) — for $K > 2$ classes, $K$ binary classifiers are trained independently. Class $k$ vs rest:

$$\hat{p}_k(x) = \sigma\!\left(x^T\beta_k + \beta_{0,k}\right)$$

**Prediction** assigns the class with the highest probability:

$$\hat{y} = \underset{k}{\arg\max}\ \sigma(x^T\beta_k)$$

</div>

<div class="lang-fr">

## Référence API

**Signature**

```python
model = sp.LogisticRegression(
    C=1.0,
    max_iter=100,
    tol=1e-4,
    fit_intercept=True,
    multi_class="ovr",
)

model.fit(X, y)
model.predict(X)        -> list[int]
model.predict_proba(X)  -> list[list[float]]
model.score(X, y)       -> float
model.get_params()      -> dict
model.set_params(C=..., max_iter=..., tol=..., fit_intercept=..., multi_class=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `C` | `float` | `1.0` | Inverse de la force de régularisation (plus grand = moins de régularisation) |
| `max_iter` | `int` | `100` | Nombre maximum d'itérations L-BFGS |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |
| `multi_class` | `str` | `"ovr"` | Stratégie multi-classe : `"ovr"` (Un-contre-Tous) |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `coef_` | `list[float]` | Coefficients ajustés |
| `intercept_` | `float` | Terme de biais |
| `classes_` | `list[int]` | Labels de classes uniques |
| `n_iter_` | `int` | Nombre d'itérations réalisées |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

model = sp.LogisticRegression(C=1.0, max_iter=200)
model.fit(X, y)
print(f"Précision : {model.score(X, y):.4f}")
proba = model.predict_proba(X)
print(f"P(classe=1) échantillon 0 : {proba[0][1]:.4f}")
```

</details>

---

## Fonctionnement algorithmique

La régression logistique ajuste une frontière de décision linéaire à l'aide de la fonction **sigmoïde** :

$$\sigma(z) = \frac{1}{1 + e^{-z}}, \qquad z = x^T\beta + \beta_0$$

Le modèle minimise la **perte d'entropie croisée** avec régularisation L2 :

$$\mathcal{L}(\beta) = -\frac{1}{n}\sum_{i=1}^n \left[y_i \log \hat{p}_i + (1-y_i)\log(1-\hat{p}_i)\right] + \frac{1}{2C}\|\beta\|_2^2$$

**Optimiseur** — les paramètres sont mis à jour via **L-BFGS** (Limited-memory Broyden-Fletcher-Goldfarb-Shanno), une méthode quasi-Newton qui approxime l'inverse de la Hessienne à partir des $m$ dernières différences de gradient.

**Multi-classe** (OvR) — pour $K > 2$ classes, $K$ classificateurs binaires sont entraînés indépendamment. Classe $k$ contre le reste :

$$\hat{p}_k(x) = \sigma\!\left(x^T\beta_k + \beta_{0,k}\right)$$

**Prédiction** affecte la classe avec la probabilité la plus haute :

$$\hat{y} = \underset{k}{\arg\max}\ \sigma(x^T\beta_k)$$

</div>