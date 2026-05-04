# AdaBoost — Adaptive Boosting

<div class="lang-en">

## API Reference

```python
clf = sp.AdaBoostClassifier(n_estimators=50, learning_rate=1.0, max_depth=1)
reg = sp.AdaBoostRegressor(n_estimators=50, learning_rate=1.0, max_depth=3)

clf.fit(X, y)
clf.predict(X)               -> list[int]
clf.predict_proba(X)         -> ndarray (n, K)
clf.score(X, y)              -> float
clf.get_params()             -> dict
clf.set_params(n_estimators=100, learning_rate=0.5, max_depth=3)

reg.fit(X, y)
reg.predict(X)               -> list[float]
reg.score(X, y)              -> float
```

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `50` | Maximum number of weak learners |
| `learning_rate` | `float` | `1.0` | Shrinkage parameter $\nu$ for learner contribution |
| `max_depth` | `int` | `1` (clf) / `3` (reg) | Max tree depth; `1` = stumps, `>1` = full trees |

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels (classifier only) |
| `n_estimators_` | `int` | Number of estimators used |
| `learning_rate_` | `float` | Shrinkage parameter |
| `max_depth_` | `int` | Tree depth setting |

## Example

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 5)
y = np.random.randint(0, 3, 500)

clf = sp.AdaBoostClassifier(n_estimators=100, learning_rate=0.8, max_depth=3)
clf.fit(X, y)

proba = clf.predict_proba(X)
score = clf.score(X, y)
print(f"Accuracy: {score:.4f}, Class probs shape: {np.array(proba).shape}")
```

---

## Algorithmic Functioning

**AdaBoost.M1** (multi-class Adaptive Boosting) combines weak learners by iteratively re-weighting misclassified samples.

**Initialization** — uniform sample weights:

$$w_i^{(1)} = \frac{1}{n}, \quad i = 1, \ldots, n$$

**Iteration** $m = 1, \ldots, M$:

**1.** Fit weak learner $h_m$ (stump or tree) on weighted dataset.

**2.** Compute weighted error:

$$\varepsilon_m = \sum_{i=1}^n w_i^{(m)} \cdot \mathbb{1}\bigl[h_m(x_i) \neq y_i\bigr]$$

**3.** If $\varepsilon_m \geq 1 - \frac{1}{K}$ (worse than random for $K$ classes), stop.

**4.** Compute learner weight:

$$\alpha_m = \nu \left[\frac{1}{2}\ln\left(\frac{1 - \varepsilon_m}{\varepsilon_m}\right) + \ln(K - 1)\right]$$

where $\nu$ is `learning_rate` and $K = |\text{classes}|$.

**5.** Update weights (down-weight correct predictions):

$$w_i^{(m+1)} \propto w_i^{(m)} \exp\left(-\alpha_m \mathbb{1}\bigl[h_m(x_i) = y_i\bigr]\right)$$

Renormalize: $\sum_i w_i^{(m+1)} = 1$.

**Final classifier** — weighted majority:

$$F(x) = \arg\max_c \sum_{m: h_m(x) = c} \alpha_m$$

**Regressor (AdaBoost.R2)** — weak learners fit to residuals with exponential loss reweighting; final prediction is weighted median.

### Weak Learners

- `max_depth=1`: **Decision stumps** (1-level trees) — fast, O(n log p) per iteration
- `max_depth>1`: **Full decision trees** — more expressive, captures non-linear splits

---

</div>

<div class="lang-fr">

## Référence API

```python
clf = sp.AdaBoostClassifier(n_estimators=50, learning_rate=1.0, max_depth=1)
reg = sp.AdaBoostRegressor(n_estimators=50, learning_rate=1.0, max_depth=3)

clf.fit(X, y)
clf.predict(X)               -> list[int]
clf.predict_proba(X)         -> ndarray (n, K)
clf.score(X, y)              -> float
clf.get_params()             -> dict
clf.set_params(n_estimators=100, learning_rate=0.5, max_depth=3)

reg.fit(X, y)
reg.predict(X)               -> list[float]
reg.score(X, y)              -> float
```

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_estimators` | `int` | `50` | Nombre maximum d'apprenants faibles |
| `learning_rate` | `float` | `1.0` | Paramètre de rétrécissement $\nu$ |
| `max_depth` | `int` | `1` | Profondeur max; `1` = stumps, `>1` = arbres |

## Attributs

| Attribut | Type | Description |
|----------|------|-------------|
| `classes_` | `list[int]` | Labels de classes uniques |
| `n_estimators_` | `int` | Nombre d'estimateurs utilisés |
| `learning_rate_` | `float` | Paramètre de rétrécissement |
| `max_depth_` | `int` | Profondeur d'arbre |

## Exemple

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 5)
y = np.random.randint(0, 3, 500)

clf = sp.AdaBoostClassifier(n_estimators=100, learning_rate=0.8, max_depth=3)
clf.fit(X, y)

proba = clf.predict_proba(X)
score = clf.score(X, y)
print(f"Précision: {score:.4f}, Shape proba: {np.array(proba).shape}")
```

---

## Fonctionnement algorithmique

**AdaBoost.M1** (Adaptive Boosting multi-classe) combine des apprenants faibles par re-pondération itérative des échantillons mal classés.

**Initialisation** — poids uniformes:

$$w_i^{(1)} = \frac{1}{n}, \quad i = 1, \ldots, n$$

**Itération** $m = 1, \ldots, M$:

**1.** Ajuster l'apprenant faible $h_m$ sur l'ensemble pondéré.

**2.** Calculer l'erreur pondérée:

$$\varepsilon_m = \sum_{i=1}^n w_i^{(m)} \cdot \mathbb{1}\bigl[h_m(x_i) \neq y_i\bigr]$$

**3.** Si $\varepsilon_m \geq 1 - \frac{1}{K}$ (pire qu'aléatoire), arrêter.

**4.** Calculer le poids de l'apprenant:

$$\alpha_m = \nu \left[\frac{1}{2}\ln\left(\frac{1 - \varepsilon_m}{\varepsilon_m}\right) + \ln(K - 1)\right]$$

où $\nu$ est `learning_rate` et $K = |\text{classes}|$.

**5.** Mettre à jour les poids (diminuer les bonnes prédictions):

$$w_i^{(m+1)} \propto w_i^{(m)} \exp\left(-\alpha_m \mathbb{1}\bigl[h_m(x_i) = y_i\bigr]\right)$$

Renormaliser: $\sum_i w_i^{(m+1)} = 1$.

**Classificateur final** — vote pondéré:

$$F(x) = \arg\max_c \sum_{m: h_m(x) = c} \alpha_m$$

**Régresseur (AdaBoost.R2)** — apprenants faibles ajustés aux résidus; prédiction finale = médiane pondérée.

### Apprenants faibles

- `max_depth=1`: **Decision stumps** (arbres 1-niveau) — rapide, O(n log p) par itération
- `max_depth>1`: **Arbres complets** — plus expressif, capture les splits non-linéaires

---

</div>
