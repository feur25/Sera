# AdaBoostClassifier / AdaBoostRegressor

<div class="lang-en">

## Code

**Signature**

```python
clf = sp.AdaBoostClassifier(
    n_estimators=50, learning_rate=1.0
)
reg = sp.AdaBoostRegressor(
    n_estimators=50, learning_rate=1.0
)

model.fit(X, y)
model.predict(X)               -> list[int] | list[float]
model.predict_proba(X)         -> ndarray (n, K)   # classifier only
model.score(X, y)              -> float
model.get_params()             -> dict
model.set_params(n_estimators=..., learning_rate=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `50` | Maximum number of weak learners |
| `learning_rate` | `float` | `1.0` | Shrinkage applied to each weak learner's contribution |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels (classifier only) |
| `estimator_weights_` | `list[float]` | Weight $\alpha_m$ assigned to each weak learner |
| `estimator_errors_` | `list[float]` | Weighted error of each weak learner |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 4)
y = np.sign(X[:, 0] * X[:, 1]).astype(int)

clf = sp.AdaBoostClassifier(n_estimators=100, learning_rate=0.5)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")
```

</details>

---

## Algorithmic Functioning

AdaBoost (**Adaptive Boosting**) builds a **weighted sum of weak classifiers** by iteratively re-weighting misclassified samples.

**Initialisation** — uniform sample weights:

$$w_i^{(1)} = \frac{1}{n}, \quad i = 1, \ldots, n$$

**Boosting iteration** $m = 1, \ldots, M$:

**1.** Fit a weak learner $h_m$ to the weighted dataset.

**2.** Compute the **weighted error**:

$$\varepsilon_m = \sum_{i=1}^n w_i^{(m)} \cdot \mathbf{1}\bigl[h_m(x_i) \neq y_i\bigr]$$

**3.** Compute the **learner weight** (contribution of $h_m$):

$$\alpha_m = \nu \cdot \frac{1}{2}\ln\!\left(\frac{1 - \varepsilon_m}{\varepsilon_m}\right)$$

where $\nu$ is the `learning_rate`. A perfect classifier ($\varepsilon_m = 0$) receives $\alpha_m \to \infty$; a random guesser ($\varepsilon_m = 0.5$) receives $\alpha_m = 0$.

**4.** Update and **renormalise** sample weights, down-weighting correctly classified samples:

$$w_i^{(m+1)} \propto w_i^{(m)} \exp\!\bigl(-\alpha_m y_i h_m(x_i)\bigr)$$

**Final classifier** — weighted majority vote:

$$F(x) = \text{sign}\!\left(\sum_{m=1}^M \alpha_m h_m(x)\right)$$

**Regressor (AdaBoost.R2)** — weak learners are fitted to the residuals weighted by a loss-based sample reweighting scheme, and the ensemble prediction is the weighted median.

**Stopping condition**: if $\varepsilon_m \geq 0.5$, the iteration is stopped early (the current learner is no better than random).

</div>

<div class="lang-fr">

## Code

**Signature**

```python
clf = sp.AdaBoostClassifier(
    n_estimators=50, learning_rate=1.0
)
reg = sp.AdaBoostRegressor(
    n_estimators=50, learning_rate=1.0
)

model.fit(X, y)
model.predict(X)               -> list[int] | list[float]
model.predict_proba(X)         -> ndarray (n, K)   # classificateur seulement
model.score(X, y)              -> float
model.get_params()             -> dict
model.set_params(n_estimators=..., learning_rate=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_estimators` | `int` | `50` | Nombre maximum d'apprenants faibles |
| `learning_rate` | `float` | `1.0` | Rétrécissement appliqué à la contribution de chaque apprenant faible |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `classes_` | `list[int]` | Labels de classes uniques (classificateur seulement) |
| `estimator_weights_` | `list[float]` | Poids $\alpha_m$ attribué à chaque apprenant faible |
| `estimator_errors_` | `list[float]` | Erreur pondérée de chaque apprenant faible |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 4)
y = np.sign(X[:, 0] * X[:, 1]).astype(int)

clf = sp.AdaBoostClassifier(n_estimators=100, learning_rate=0.5)
clf.fit(X, y)
print(f"Précision : {clf.score(X, y):.4f}")
```

</details>

---

## Fonctionnement algorithmique

AdaBoost (**Adaptive Boosting**) construit une **somme pondérée de classificateurs faibles** en répondant de manière adaptative aux échantillons mal classifiés.

**Initialisation** — poids d'échantillons uniformes :

$$w_i^{(1)} = \frac{1}{n}, \quad i = 1, \ldots, n$$

**Itération de boosting** $m = 1, \ldots, M$ :

**1.** Ajuster un apprenant faible $h_m$ au jeu de données pondéré.

**2.** Calculer l'**erreur pondérée** :

$$\varepsilon_m = \sum_{i=1}^n w_i^{(m)} \cdot \mathbf{1}\bigl[h_m(x_i) \neq y_i\bigr]$$

**3.** Calculer le **poids de l'apprenant** (contribution de $h_m$) :

$$\alpha_m = \nu \cdot \frac{1}{2}\ln\!\left(\frac{1 - \varepsilon_m}{\varepsilon_m}\right)$$

où $\nu$ est le `learning_rate`. Un classificateur parfait ($\varepsilon_m = 0$) reçoit $\alpha_m \to \infty$ ; un devineur aléatoire ($\varepsilon_m = 0,5$) reçoit $\alpha_m = 0$.

**4.** Mettre à jour et **renormaliser** les poids des échantillons, en réduisant le poids des échantillons correctement classifiés :

$$w_i^{(m+1)} \propto w_i^{(m)} \exp\!\bigl(-\alpha_m y_i h_m(x_i)\bigr)$$

**Classificateur final** — vote majoritaire pondéré :

$$F(x) = \text{sign}\!\left(\sum_{m=1}^M \alpha_m h_m(x)\right)$$

**Régresseur (AdaBoost.R2)** — les apprenants faibles sont ajustés aux résidus pondérés par un schéma de ré-pondération basé sur la perte, et la prédiction de l'ensemble est la médiane pondérée.

**Condition d'arrêt** : si $\varepsilon_m \geq 0,5$, l'itération s'arrête prématurément (l'apprenant courant n'est pas meilleur qu'aléatoire).

</div>
# AdaBoostClassifier / AdaBoostRegressor

<div class="lang-en">

## Signature

```python
clf = sp.AdaBoostClassifier(
    n_estimators: int = 50,
    learning_rate: float = 1.0,
    max_depth: int = 1,
)

reg = sp.AdaBoostRegressor(
    n_estimators: int = 50,
    learning_rate: float = 1.0,
    max_depth: int = 3,
)

clf.classes_        -> list[int]
clf.n_estimators_   -> int
clf.learning_rate_  -> float
clf.max_depth_      -> int

reg.n_estimators_   -> int
reg.learning_rate_  -> float
reg.max_depth_      -> int
```

---

## Description

Adaptive Boosting. Reweights samples at each stage so that subsequent trees focus on previously misclassified examples.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `50` | Number of weak learners |
| `learning_rate` | `float` | `1.0` | Weight shrinkage per stage |
| `max_depth` | `int` | `1`/`3` | Depth of each base tree |

</div>

<div class="lang-fr">

## Description

Boosting adaptatif. Repondère les échantillons à chaque étape pour que les arbres suivants se concentrent sur les exemples mal classés.

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_estimators` | `int` | `50` | Nombre d'apprenants faibles |
| `learning_rate` | `float` | `1.0` | Réduction du poids par étape |
| `max_depth` | `int` | `1`/`3` | Profondeur de chaque arbre de base |

</div>

---

## Description

Adaptive Boosting using the **SAMME.R** (Real) algorithm for classification with Laplace-smoothed probability estimates. Each weak learner is a decision stump (classifier) or shallow tree (regressor). SAMME.R uses probability-based boosting which outperforms the discrete SAMME variant.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `50` | Number of boosting rounds |
| `learning_rate` | `float` | `1.0` | Shrinkage per step |
| `max_depth` | `int` | `1` (clf) / `3` (reg) | Depth of weak learner |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels (classifier only) |

---

## Example

<details>
<summary><strong>AdaBoost stumps</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = ((X[:, 0] + X[:, 1]) > 0).astype(np.int32)

clf = sp.AdaBoostClassifier(n_estimators=100)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")
```

</details>
