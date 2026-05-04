# LinearSVC / LinearSVR

<div class="lang-en">

## API Reference

**Signature**

```python
clf = sp.LinearSVC(
    C=1.0, max_iter=1000, tol=1e-4, fit_intercept=True
)
reg = sp.LinearSVR(
    C=1.0, epsilon=0.1, max_iter=1000, tol=1e-4, fit_intercept=True
)

model.fit(X, y)
model.predict(X)               -> list[int] | list[float]
model.score(X, y)              -> float
clf.decision_function(X)       -> list[float]   # LinearSVC only
model.get_params()             -> dict
model.set_params(C=..., max_iter=..., tol=...)
```

**Constructor parameters — LinearSVC**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `C` | `float` | `1.0` | Inverse regularisation strength (smaller = stronger) |
| `max_iter` | `int` | `1000` | Maximum training epochs |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Fit a bias term |

**Constructor parameters — LinearSVR**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `C` | `float` | `1.0` | Inverse regularisation strength |
| `epsilon` | `float` | `0.1` | Half-width of the $\varepsilon$-insensitive tube |
| `max_iter` | `int` | `1000` | Maximum training epochs |
| `tol` | `float` | `1e-4` | Convergence tolerance |
| `fit_intercept` | `bool` | `True` | Fit a bias term |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels (LinearSVC only) |
| `coef_` | `list[list[float]]` | Weight vectors — shape `(K, p)` for classifier, `(p,)` list for SVR |
| `intercept_` | `list[float]` | Bias term(s)|`intercept_` | `list[float]` | Bias term(s) |
| `C_` | `float` | Regularisation parameter |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

clf = sp.LinearSVC(
    C=1.0, fit_intercept=True)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")
print(f"Margins: {clf.decision_function(X[:3])}")

reg = sp.LinearSVR(
    C=1.0, epsilon=0.05)
reg.fit(X, X[:, 0] * 2 - X[:, 2])
print(f"R²: {reg.score(X, X[:, 0] * 2 - X[:, 2]):.4f}")
```

</details>

---

## Algorithmic Functioning

Both models are solved via the **dual coordinate descent** method on the kernelised SVM dual problem restricted to a linear kernel.

**LinearSVC — Primal objective** (hinge loss + L2 regularisation):

<div>$$\min_{w, b} \frac{1}{2}\|w\|^2 + C \sum_{i=1}^n \max\!\bigl(0,\, 1 - y_i(w^\top x_i + b)\bigr)$$</div>

The constraint $y_i(w^\top x_i + b) \geq 1$ defines the **margin**; misclassified points contribute a hinge penalty.

**Dual form** — introduce per-sample dual variables $\alpha_i \in [0, C]$:

<div>$$\max_{\alpha} \sum_{i=1}^n \alpha_i - \frac{1}{2}\sum_{i,j} \alpha_i \alpha_j y_i y_j x_i^\top x_j \quad \text{s.t.} \quad \sum_i \alpha_i y_i = 0$$</div>

Dual coordinate descent updates one $\alpha_i$ at a time, solving the 1-d quadratic subproblem analytically with clipping to $[0, C]$.

**Prediction** — signed margin:

<div>$$f(x) = w^\top x + b = \sum_{i} \alpha_i y_i x_i^\top x + b$$</div>

For **multiclass** (OvR): $K$ binary SVMs are trained, and the class with the highest margin wins.

**LinearSVR — Primal objective** ($\varepsilon$-insensitive loss):

<div>$$\min_{w, b} \frac{1}{2}\|w\|^2 + C \sum_{i=1}^n \max\!\bigl(0,\, |y_i - (w^\top x_i + b)| - \varepsilon\bigr)$$</div>

Residuals smaller than $\varepsilon$ incur zero penalty — the model ignores small errors and focuses on larger deviations.

</div>

<div class="lang-fr">

## Référence API

**Signature**

```python
clf = sp.LinearSVC(
    C=1.0, max_iter=1000, tol=1e-4, fit_intercept=True
)
reg = sp.LinearSVR(
    C=1.0, epsilon=0.1, max_iter=1000, tol=1e-4, fit_intercept=True
)

model.fit(X, y)
model.predict(X)               -> list[int] | list[float]
model.score(X, y)              -> float
clf.decision_function(X)       -> list[float]   # LinearSVC seulement
model.get_params()             -> dict
model.set_params(C=..., max_iter=..., tol=...)
```

**Paramètres du constructeur — LinearSVC**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `C` | `float` | `1.0` | Inverse de la force de régularisation (plus petit = plus fort) |
| `max_iter` | `int` | `1000` | Nombre maximum d'époques d'entraînement |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |

**Paramètres du constructeur — LinearSVR**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `C` | `float` | `1.0` | Inverse de la force de régularisation |
| `epsilon` | `float` | `0.1` | Demi-largeur du tube $\varepsilon$-insensible |
| `max_iter` | `int` | `1000` | Nombre maximum d'époques d'entraînement |
| `tol` | `float` | `1e-4` | Tolérance de convergence |
| `fit_intercept` | `bool` | `True` | Ajuster un terme de biais |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `classes_` | `list[int]` | Labels de classes uniques (LinearSVC seulement) |
| `coef_` | `list` | Coefficients de poids — forme `(K, p)` pour SVC multiclasse, `(p,)` pour SVR |
| `intercept_` | `list[float]` | Bias term(s)| list[float]` | Terme(s) de biais |
| `C_` | `float` | Paramètre de régularisation |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

clf = sp.LinearSVC(
    C=1.0, fit_intercept=True)
clf.fit(X, y)
print(f"Précision : {clf.score(X, y):.4f}")
print(f"Marges : {clf.decision_function(X[:3])}")

reg = sp.LinearSVR(
    C=1.0, epsilon=0.05)
reg.fit(X, X[:, 0] * 2 - X[:, 2])
print(f"R² : {reg.score(X, X[:, 0] * 2 - X[:, 2]):.4f}")
```

</details>

---

## Fonctionnement algorithmique

Les deux modèles sont résolus via la méthode de **descente de coordonnées duale** sur le problème dual SVM noyauté restreint au noyau linéaire.

**LinearSVC — Objectif primal** (perte hinge + régularisation L2) :

<div>$$\min_{w, b} \frac{1}{2}\|w\|^2 + C \sum_{i=1}^n \max\!\bigl(0,\, 1 - y_i(w^\top x_i + b)\bigr)$$</div>

La contrainte $y_i(w^\top x_i + b) \geq 1$ définit la **marge** ; les points mal classifiés contribuent une pénalité hinge.

**Forme duale** — introduire des variables duales $\alpha_i \in [0, C]$ par échantillon :

<div>$$\max_{\alpha} \sum_{i=1}^n \alpha_i - \frac{1}{2}\sum_{i,j} \alpha_i \alpha_j y_i y_j x_i^\top x_j \quad \text{s.t.} \quad \sum_i \alpha_i y_i = 0$$</div>

La descente de coordonnées duale met à jour un $\alpha_i$ à la fois, résolvant analytiquement le sous-problème quadratique 1-d avec écrêtage à $[0, C]$.

**Prédiction** — marge signée :

<div>$$f(x) = w^\top x + b = \sum_{i} \alpha_i y_i x_i^\top x + b$$</div>

Pour le **multiclasse** (OvR) : $K$ SVMs binaires sont entraînés, et la classe avec la marge la plus haute l'emporte.

**LinearSVR — Objectif primal** (perte $\varepsilon$-insensible) :

<div>$$\min_{w, b} \frac{1}{2}\|w\|^2 + C \sum_{i=1}^n \max\!\bigl(0,\, |y_i - (w^\top x_i + b)| - \varepsilon\bigr)$$</div>

Les résidus plus petits que $\varepsilon$ n'entraînent aucune pénalité — le modèle ignore les petites erreurs et se concentre sur les déviations plus grandes.

</div>
