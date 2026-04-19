# GradientBoostingClassifier / GradientBoostingRegressor

<div class="lang-en">

## Code

**Signature**

```python
clf = sp.GradientBoostingClassifier(
    n_estimators=100, learning_rate=0.1, max_depth=3,
    min_samples_split=2, min_samples_leaf=1
)
reg = sp.GradientBoostingRegressor(
    n_estimators=100, learning_rate=0.1, max_depth=3,
    min_samples_split=2, min_samples_leaf=1
)

model.fit(X, y)
model.predict(X)               -> list[int] | list[float]
model.predict_proba(X)         -> ndarray (n, K)   # classifier only
model.score(X, y)              -> float
model.get_params()             -> dict
model.set_params(n_estimators=..., learning_rate=..., max_depth=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `100` | Number of boosting stages (trees) |
| `learning_rate` | `float` | `0.1` | Shrinkage applied to each tree's contribution |
| `max_depth` | `int` | `3` | Maximum depth per tree |
| `min_samples_split` | `int` | `2` | Minimum samples to split a node |
| `min_samples_leaf` | `int` | `1` | Minimum samples at a leaf |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels (classifier only) |
| `n_estimators_` | `int` | Number of trees |
| `learning_rate_` | `float` | Shrinkage factor |
| `max_depth_` | `int` | Tree depth |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(600, 8)
y = (X[:, 0] ** 2 + X[:, 1] > 1).astype(int)

clf = sp.GradientBoostingClassifier(n_estimators=150, learning_rate=0.05, max_depth=4)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")
```

</details>

---

## Algorithmic Functioning

Gradient Boosting constructs an **additive model** $F_M(x)$ by sequentially fitting shallow trees to the negative gradient of the loss.

**Initialisation** with the optimal constant prediction:

$$F_0(x) = \underset{\gamma}{\arg\min} \sum_{i=1}^n \mathcal{L}(y_i, \gamma)$$

**Boosting iteration** $m = 1, \ldots, M$:

**1.** Compute **pseudo-residuals** (negative gradient of the loss w.r.t. the current prediction):

$$r_{im} = -\frac{\partial \mathcal{L}(y_i, F(x_i))}{\partial F(x_i)}\Bigg|_{F = F_{m-1}}$$

**2.** Fit a decision tree $h_m$ to the pseudo-residuals $\{(x_i, r_{im})\}$.

**3.** Update the model with shrinkage $\nu$ (learning rate):

$$F_m(x) = F_{m-1}(x) + \nu \cdot h_m(x)$$

**Regressor (L2 loss)** — pseudo-residuals are simply the ordinary residuals:

$$r_{im} = y_i - F_{m-1}(x_i)$$

**Classifier (log loss / deviance)** — models the log-odds. Pseudo-residuals are:

$$r_{im} = y_i - p_{m-1}(x_i), \qquad p_{m-1}(x_i) = \sigma(F_{m-1}(x_i))$$

Multiclass: $K$ trees are grown per stage, one per class (OvR log-loss).

**Effect of `learning_rate`** — smaller $\nu$ requires more trees but generalises better. The optimal model balances $M$ and $\nu$ jointly.

</div>

<div class="lang-fr">

## Code

**Signature**

```python
clf = sp.GradientBoostingClassifier(
    n_estimators=100, learning_rate=0.1, max_depth=3,
    min_samples_split=2, min_samples_leaf=1
)
reg = sp.GradientBoostingRegressor(
    n_estimators=100, learning_rate=0.1, max_depth=3,
    min_samples_split=2, min_samples_leaf=1
)

model.fit(X, y)
model.predict(X)               -> list[int] | list[float]
model.predict_proba(X)         -> ndarray (n, K)   # classificateur seulement
model.score(X, y)              -> float
model.get_params()             -> dict
model.set_params(n_estimators=..., learning_rate=..., max_depth=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_estimators` | `int` | `100` | Nombre de stages de boosting (arbres) |
| `learning_rate` | `float` | `0.1` | Rétrécissement appliqué à la contribution de chaque arbre |
| `max_depth` | `int` | `3` | Profondeur maximale par arbre |
| `min_samples_split` | `int` | `2` | Nombre minimum d'échantillons pour diviser un nœud |
| `min_samples_leaf` | `int` | `1` | Nombre minimum d'échantillons dans une feuille |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `classes_` | `list[int]` | Labels de classes uniques (classificateur seulement) |
| `n_estimators_` | `int` | Nombre d'arbres |
| `learning_rate_` | `float` | Facteur de rétrécissement |
| `max_depth_` | `int` | Profondeur des arbres |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(600, 8)
y = (X[:, 0] ** 2 + X[:, 1] > 1).astype(int)

clf = sp.GradientBoostingClassifier(n_estimators=150, learning_rate=0.05, max_depth=4)
clf.fit(X, y)
print(f"Précision : {clf.score(X, y):.4f}")
```

</details>

---

## Fonctionnement algorithmique

Le Gradient Boosting construit un **modèle additif** $F_M(x)$ en ajustant séquentiellement des arbres peu profonds au gradient négatif de la perte.

**Initialisation** avec la prédiction constante optimale :

$$F_0(x) = \underset{\gamma}{\arg\min} \sum_{i=1}^n \mathcal{L}(y_i, \gamma)$$

**Itération de boosting** $m = 1, \ldots, M$ :

**1.** Calcul des **pseudo-résidus** (gradient négatif de la perte par rapport à la prédiction courante) :

$$r_{im} = -\frac{\partial \mathcal{L}(y_i, F(x_i))}{\partial F(x_i)}\Bigg|_{F = F_{m-1}}$$

**2.** Ajuster un arbre de décision $h_m$ aux pseudo-résidus $\{(x_i, r_{im})\}$.

**3.** Mettre à jour le modèle avec le rétrécissement $\nu$ (learning rate) :

$$F_m(x) = F_{m-1}(x) + \nu \cdot h_m(x)$$

**Régresseur (perte L2)** — les pseudo-résidus sont simplement les résidus ordinaires :

$$r_{im} = y_i - F_{m-1}(x_i)$$

**Classificateur (log loss / déviance)** — modélise les log-odds. Pseudo-résidus :

$$r_{im} = y_i - p_{m-1}(x_i), \qquad p_{m-1}(x_i) = \sigma(F_{m-1}(x_i))$$

Multiclasse : $K$ arbres sont construits par stage, un par classe (log-loss OvR).

**Effet du `learning_rate`** — un $\nu$ plus petit nécessite plus d'arbres mais généralise mieux. Le modèle optimal équilibre $M$ et $\nu$ conjointement.

</div>
# GradientBoostingClassifier / GradientBoostingRegressor

<div class="lang-en">

## Signature

```python
clf = sp.GradientBoostingClassifier(
    n_estimators: int = 100,
    learning_rate: float = 0.1,
    max_depth: int = 3,
    min_samples_split: int = 2,
    min_samples_leaf: int = 1,
)

reg = sp.GradientBoostingRegressor(
    n_estimators: int = 100,
    learning_rate: float = 0.1,
    max_depth: int = 3,
    min_samples_split: int = 2,
    min_samples_leaf: int = 1,
)

clf.n_estimators_       -> int
clf.learning_rate_      -> float
clf.max_depth_          -> int
clf.min_samples_split_  -> int
clf.min_samples_leaf_   -> int

reg.n_estimators_       -> int
reg.learning_rate_      -> float
reg.max_depth_          -> int
reg.min_samples_split_  -> int
reg.min_samples_leaf_   -> int
```

---

## Description

Sequential ensemble: each tree fits residuals of previous. Uses negative gradient of loss function (MSE for regression, log-loss for classification).

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `100` | Number of boosting stages |
| `learning_rate` | `float` | `0.1` | Shrinkage factor per stage |
| `max_depth` | `int` | `3` | Maximum depth of each tree |
| `min_samples_split` | `int` | `2` | Min samples to split |
| `min_samples_leaf` | `int` | `1` | Min samples in leaf |

</div>

<div class="lang-fr">

## Description

Ensemble séquentiel : chaque arbre ajuste les résidus du précédent. Utilise le gradient négatif de la fonction de perte (MSE pour la régression, log-loss pour la classification).

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_estimators` | `int` | `100` | Nombre d'étapes de boosting |
| `learning_rate` | `float` | `0.1` | Facteur de réduction par étape |
| `max_depth` | `int` | `3` | Profondeur maximale de chaque arbre |
| `min_samples_split` | `int` | `2` | Minimum d'échantillons pour diviser |
| `min_samples_leaf` | `int` | `1` | Minimum d'échantillons en feuille |

</div>

clf.predict_proba(x) -> ndarray (n, n_classes)
clf.classes_ -> list[int]
```

---

## Description

Sequential boosting with shallow decision trees. The classifier uses multiclass softmax boosting with **Newton-Raphson leaf values** (gradient + hessian). The regressor uses residual fitting with shrinkage.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `100` | Number of boosting rounds |
| `learning_rate` | `float` | `0.1` | Shrinkage per step |
| `max_depth` | `int` | `3` | Depth of individual trees |
| `min_samples_split` | `int` | `2` | Minimum samples to split |
| `min_samples_leaf` | `int` | `1` | Minimum samples in a leaf |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels (classifier only) |

---

## Examples

<details>
<summary><strong>Multiclass classification</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(600, 5)
y = np.where(X[:, 0] > 0.5, 2, np.where(X[:, 0] < -0.5, 0, 1)).astype(np.int32)

clf = sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1, max_depth=3)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")
```

</details>

<details>
<summary><strong>Regression</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = X[:, 0] ** 2 + np.sin(X[:, 1] * 3) + np.random.randn(500) * 0.1

reg = sp.GradientBoostingRegressor(n_estimators=200, learning_rate=0.05, max_depth=4)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}")
```

</details>
