# RandomForestClassifier / RandomForestRegressor

<div class="lang-en">

## Code

**Signature**

```python
clf = sp.RandomForestClassifier(
    n_estimators=100, max_depth=10, min_samples_split=2,
    min_samples_leaf=1, max_features="sqrt"
)
reg = sp.RandomForestRegressor(
    n_estimators=100, max_depth=10, min_samples_split=2,
    min_samples_leaf=1, max_features="sqrt"
)

model.fit(X, y)
model.predict(X)               -> list[int] | list[float]
model.predict_proba(X)         -> ndarray (n, K)   # classifier only
model.score(X, y)              -> float
model.get_params()             -> dict
model.set_params(n_estimators=..., max_depth=..., min_samples_split=..., min_samples_leaf=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `100` | Number of trees |
| `max_depth` | `int` | `10` | Maximum depth per tree |
| `min_samples_split` | `int` | `2` | Minimum samples to split a node |
| `min_samples_leaf` | `int` | `1` | Minimum samples at a leaf |
| `max_features` | `str` | `"sqrt"` | Features per split: `"sqrt"`, `"log2"`, `"all"` |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `feature_importances_` | `list[float]` | Mean impurity decrease per feature across all trees |
| `classes_` | `list[int]` | Unique class labels (classifier only) |
| `n_estimators_` | `int` | Number of trees |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 10)
y = (X[:, 0] + X[:, 2] - X[:, 4] > 0).astype(int)

clf = sp.RandomForestClassifier(n_estimators=200, max_depth=8)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")
print(f"Top feature: {np.argmax(clf.feature_importances_)}")
```

</details>

---

## Algorithmic Functioning

Random Forest is an ensemble of $B$ decorrelated decision trees trained on **bootstrap samples** of the data, with random feature subsampling at each split.

**Bootstrap sampling** — each tree $T_b$ is trained on $n$ samples drawn with replacement from the training set:

$$\mathcal{D}_b = \{(x_i, y_i)\}_{i \sim \text{Uniform}(1,n)}^n$$

**Random feature subsampling** — at each node split, only $m$ features are considered (not all $p$):

$$m = \begin{cases} \lfloor\sqrt{p}\rfloor & \texttt{max\_features="sqrt"} \\ \lfloor\log_2 p\rfloor & \texttt{max\_features="log2"} \\ p & \texttt{max\_features="all"} \end{cases}$$

This decorrelates trees: even when one feature is dominant, other trees will be forced to find alternative splits.

**Prediction — Classifier** (majority vote):

$$\hat{y} = \underset{k}{\arg\max} \sum_{b=1}^B \mathbf{1}\bigl[T_b(x) = k\bigr]$$

**Prediction — Regressor** (average):

$$\hat{y} = \frac{1}{B}\sum_{b=1}^B T_b(x)$$

**Feature importance** averages per-tree importances:

$$\text{FI}(j) = \frac{1}{B}\sum_{b=1}^B \text{FI}_b(j)$$

The ensemble variance is reduced relative to a single tree by a factor approaching $\frac{1}{B}$ as trees become decorrelated (via the random subsampling).

</div>

<div class="lang-fr">

## Code

**Signature**

```python
clf = sp.RandomForestClassifier(
    n_estimators=100, max_depth=10, min_samples_split=2,
    min_samples_leaf=1, max_features="sqrt"
)
reg = sp.RandomForestRegressor(
    n_estimators=100, max_depth=10, min_samples_split=2,
    min_samples_leaf=1, max_features="sqrt"
)

model.fit(X, y)
model.predict(X)               -> list[int] | list[float]
model.predict_proba(X)         -> ndarray (n, K)   # classificateur seulement
model.score(X, y)              -> float
model.get_params()             -> dict
model.set_params(n_estimators=..., max_depth=..., min_samples_split=..., min_samples_leaf=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_estimators` | `int` | `100` | Nombre d'arbres |
| `max_depth` | `int` | `10` | Profondeur maximale par arbre |
| `min_samples_split` | `int` | `2` | Nombre minimum d'échantillons pour diviser un nœud |
| `min_samples_leaf` | `int` | `1` | Nombre minimum d'échantillons dans une feuille |
| `max_features` | `str` | `"sqrt"` | Features par division : `"sqrt"`, `"log2"`, `"all"` |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `feature_importances_` | `list[float]` | Diminution d'impureté moyenne par feature sur tous les arbres |
| `classes_` | `list[int]` | Labels de classes uniques (classificateur seulement) |
| `n_estimators_` | `int` | Nombre d'arbres |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 10)
y = (X[:, 0] + X[:, 2] - X[:, 4] > 0).astype(int)

clf = sp.RandomForestClassifier(n_estimators=200, max_depth=8)
clf.fit(X, y)
print(f"Précision : {clf.score(X, y):.4f}")
print(f"Feature principale : {np.argmax(clf.feature_importances_)}")
```

</details>

---

## Fonctionnement algorithmique

Random Forest est un ensemble de $B$ arbres de décision décorrélés entraînés sur des **échantillons bootstrap** des données, avec sous-échantillonnage aléatoire des features à chaque division.

**Échantillonnage bootstrap** — chaque arbre $T_b$ est entraîné sur $n$ échantillons tirés avec remise depuis l'ensemble d'entraînement :

$$\mathcal{D}_b = \{(x_i, y_i)\}_{i \sim \text{Uniforme}(1,n)}^n$$

**Sous-échantillonnage aléatoire des features** — à chaque division de nœud, seules $m$ features sont considérées (pas toutes les $p$) :

$$m = \begin{cases} \lfloor\sqrt{p}\rfloor & \texttt{max\_features="sqrt"} \\ \lfloor\log_2 p\rfloor & \texttt{max\_features="log2"} \\ p & \texttt{max\_features="all"} \end{cases}$$

Cela décorrèle les arbres : même quand une feature est dominante, les autres arbres sont forcés de trouver des divisions alternatives.

**Prédiction — Classificateur** (vote majoritaire) :

$$\hat{y} = \underset{k}{\arg\max} \sum_{b=1}^B \mathbf{1}\bigl[T_b(x) = k\bigr]$$

**Prédiction — Régresseur** (moyenne) :

$$\hat{y} = \frac{1}{B}\sum_{b=1}^B T_b(x)$$

**Importance des features** fait la moyenne des importances par arbre :

$$\text{FI}(j) = \frac{1}{B}\sum_{b=1}^B \text{FI}_b(j)$$

La variance de l'ensemble est réduite par rapport à un seul arbre d'un facteur approchant $\frac{1}{B}$ à mesure que les arbres se décorrèlent (via le sous-échantillonnage aléatoire).

</div>
# RandomForestClassifier / RandomForestRegressor

<div class="lang-en">

## Signature

```python
clf = sp.RandomForestClassifier(
    n_estimators: int = 100,
    max_depth: int = 10,
    min_samples_split: int = 2,
    min_samples_leaf: int = 1,
    max_features: str = "sqrt",
)

reg = sp.RandomForestRegressor(
    n_estimators: int = 100,
    max_depth: int = 10,
    min_samples_split: int = 2,
    min_samples_leaf: int = 1,
    max_features: str = "sqrt",
)

clf.classes_             -> list[int]
clf.feature_importances_ -> list[float]
clf.n_estimators_        -> int
clf.max_depth_           -> int
clf.min_samples_split_   -> int
clf.min_samples_leaf_    -> int
clf.max_features_        -> str

reg.feature_importances_ -> list[float]
reg.n_estimators_        -> int
reg.max_depth_           -> int
reg.min_samples_split_   -> int
reg.min_samples_leaf_    -> int
reg.max_features_        -> str
```

---

## Description

Ensemble of decision trees trained on bootstrapped samples. Each tree uses a random subset of features for splits. Parallelized with Rayon.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `100` | Number of trees |
| `max_depth` | `int` | `10` | Maximum depth per tree |
| `min_samples_split` | `int` | `2` | Min samples to split |
| `min_samples_leaf` | `int` | `1` | Min samples in leaf |
| `max_features` | `str` | `"sqrt"` | `"sqrt"`, `"log2"`, `"all"` |

</div>

<div class="lang-fr">

## Description

Ensemble d'arbres de décision entraînés sur des échantillons bootstrap. Chaque arbre utilise un sous-ensemble aléatoire de variables pour les divisions. Parallélisé avec Rayon.

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_estimators` | `int` | `100` | Nombre d'arbres |
| `max_depth` | `int` | `10` | Profondeur maximale par arbre |
| `min_samples_split` | `int` | `2` | Minimum d'échantillons pour diviser |
| `min_samples_leaf` | `int` | `1` | Minimum d'échantillons en feuille |
| `max_features` | `str` | `"sqrt"` | `"sqrt"`, `"log2"`, `"all"` |

</div>

model.feature_importances_ -> list[float]
```

---

## Description

Ensemble of decision trees trained on bootstrap samples with random feature subsets. Trees are built in parallel via Rayon. Prediction is majority vote (classifier) or mean (regressor).

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `100` | Number of trees |
| `max_depth` | `int` | `10` | Maximum depth per tree |
| `min_samples_split` | `int` | `2` | Minimum samples to split |
| `min_samples_leaf` | `int` | `1` | Minimum samples in a leaf |
| `max_features` | `str` | `"sqrt"` | `"sqrt"`, `"log2"`, or `"all"` |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `feature_importances_` | `list[float]` | Mean impurity decrease across all trees |
| `classes_` | `list[int]` | Unique class labels (classifier only) |

---

## Examples

<details>
<summary><strong>Classification</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(1000, 6)
y = ((X[:, 0] + X[:, 1]) > 0).astype(np.int32)

clf = sp.RandomForestClassifier(n_estimators=100, max_depth=8)
clf.fit(X, y)

print(f"Accuracy: {clf.score(X, y):.4f}")
print(f"Top features: {[f'{v:.3f}' for v in clf.feature_importances_]}")
```

</details>

<details>
<summary><strong>Regression</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = X[:, 0] * 3 + np.sin(X[:, 1]) + np.random.randn(500) * 0.2

reg = sp.RandomForestRegressor(n_estimators=50, max_depth=10)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}")
```

</details>
