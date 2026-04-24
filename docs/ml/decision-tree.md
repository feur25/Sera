# DecisionTreeClassifier / DecisionTreeRegressor

<div class="lang-en">

## API Reference

**Signature**

```python
clf = sp.DecisionTreeClassifier(
    max_depth=10, min_samples_split=2,
    min_samples_leaf=1, max_features=None, criterion="gini"
)
reg = sp.DecisionTreeRegressor(
    max_depth=10, min_samples_split=2,
    min_samples_leaf=1, max_features=None
)

model.fit(X, y)
model.predict(X)                -> list[int] | list[float]
model.predict_proba(X)          -> ndarray (n, K)   # classifier only
model.score(X, y)               -> float
model.get_params()              -> dict
model.set_params(max_depth=..., min_samples_split=..., min_samples_leaf=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `max_depth` | `int` | `10` | Maximum tree depth |
| `min_samples_split` | `int` | `2` | Minimum samples to split an internal node |
| `min_samples_leaf` | `int` | `1` | Minimum samples required at a leaf |
| `max_features` | `int \| None` | `None` | Number of features to consider per split (all if `None`) |
| `criterion` | `str` | `"gini"` | Split quality: `"gini"` or `"entropy"` (classifier only) |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `feature_importances_` | `list[float]` | Total impurity decrease per feature, normalised to sum 1 |
| `classes_` | `list[int]` | Unique class labels (classifier only) |
| `max_depth_` | `int` | Tree depth parameter |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 5)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

clf = sp.DecisionTreeClassifier(max_depth=5, criterion="gini")
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")
print(f"Importances: {clf.feature_importances_}")
```

</details>

---

## Algorithmic Functioning

A decision tree recursively partitions the feature space by finding the **best binary split** at each node.

**Classifier — Impurity measures:**

*Gini impurity* at node $t$ containing class proportions $p_{tk}$:

<div>$$G(t) = 1 - \sum_{k=1}^K p_{tk}^2$$</div>

*Entropy* (information content):

<div>$$H(t) = -\sum_{k=1}^K p_{tk} \log_2 p_{tk}$$</div>

**Best split** on feature $j$ at threshold $\theta$:

<div>$$\Delta I(t, j, \theta) = I(t) - \frac{|t_L|}{|t|}I(t_L) - \frac{|t_R|}{|t|}I(t_R)$$</div>

where $I \in \{G, H\}$, and $t_L, t_R$ are the left/right child nodes. The split $(j^*, \theta^*)$ that maximises $\Delta I$ is selected.

**Regressor — MSE impurity:**

<div>$$I_{\text{MSE}}(t) = \frac{1}{|t|}\sum_{i \in t}(y_i - \bar{y}_t)^2, \qquad \bar{y}_t = \frac{1}{|t|}\sum_{i \in t} y_i$$</div>

Leaf prediction is the mean $\bar{y}_t$; split selection maximises variance reduction.

**Stopping conditions:** a node becomes a leaf when $\text{depth} \geq \texttt{max\_depth}$, $|t| < \texttt{min\_samples\_split}$, or all child nodes would have $|t_{\text{child}}| < \texttt{min\_samples\_leaf}$.

**Feature importance** aggregates impurity decreases weighted by node sample count:

<div>$$\text{FI}(j) = \sum_{t : j_t = j} \frac{|t|}{n} \cdot \Delta I(t, j, \theta_t)$$</div>

normalised so $\sum_j \text{FI}(j) = 1$.

</div>

<div class="lang-fr">

## Référence API

**Signature**

```python
clf = sp.DecisionTreeClassifier(
    max_depth=10, min_samples_split=2,
    min_samples_leaf=1, max_features=None, criterion="gini"
)
reg = sp.DecisionTreeRegressor(
    max_depth=10, min_samples_split=2,
    min_samples_leaf=1, max_features=None
)

model.fit(X, y)
model.predict(X)                -> list[int] | list[float]
model.predict_proba(X)          -> ndarray (n, K)   # classificateur seulement
model.score(X, y)               -> float
model.get_params()              -> dict
model.set_params(max_depth=..., min_samples_split=..., min_samples_leaf=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `max_depth` | `int` | `10` | Profondeur maximale de l'arbre |
| `min_samples_split` | `int` | `2` | Nombre minimum d'échantillons pour diviser un nœud interne |
| `min_samples_leaf` | `int` | `1` | Nombre minimum d'échantillons requis dans une feuille |
| `max_features` | `int \| None` | `None` | Nombre de features à considérer par division (tous si `None`) |
| `criterion` | `str` | `"gini"` | Qualité de division : `"gini"` ou `"entropy"` (classificateur seulement) |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `feature_importances_` | `list[float]` | Diminution totale d'impureté par feature, normalisée à 1 |
| `classes_` | `list[int]` | Labels de classes uniques (classificateur seulement) |
| `max_depth_` | `int` | Paramètre de profondeur de l'arbre |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 5)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

clf = sp.DecisionTreeClassifier(max_depth=5, criterion="gini")
clf.fit(X, y)
print(f"Précision : {clf.score(X, y):.4f}")
print(f"Importances : {clf.feature_importances_}")
```

</details>

---

## Fonctionnement algorithmique

Un arbre de décision partitionne récursivement l'espace des features en trouvant la **meilleure division binaire** à chaque nœud.

**Classificateur — Mesures d'impureté :**

*Impureté de Gini* au nœud $t$ avec les proportions de classes $p_{tk}$ :

<div>$$G(t) = 1 - \sum_{k=1}^K p_{tk}^2$$</div>

*Entropie* (contenu informationnel) :

<div>$$H(t) = -\sum_{k=1}^K p_{tk} \log_2 p_{tk}$$</div>

**Meilleure division** sur la feature $j$ au seuil $\theta$ :

<div>$$\Delta I(t, j, \theta) = I(t) - \frac{|t_L|}{|t|}I(t_L) - \frac{|t_R|}{|t|}I(t_R)$$</div>

où $I \in \{G, H\}$, et $t_L, t_R$ sont les nœuds enfants gauche/droite. La division $(j^*, \theta^*)$ maximisant $\Delta I$ est sélectionnée.

**Régresseur — Impureté MSE :**

<div>$$I_{\text{MSE}}(t) = \frac{1}{|t|}\sum_{i \in t}(y_i - \bar{y}_t)^2, \qquad \bar{y}_t = \frac{1}{|t|}\sum_{i \in t} y_i$$</div>

La prédiction d'une feuille est la moyenne $\bar{y}_t$ ; la sélection de division maximise la réduction de variance.

**Conditions d'arrêt :** un nœud devient une feuille quand $\text{profondeur} \geq \texttt{max\_depth}$, $|t| < \texttt{min\_samples\_split}$, ou tous les nœuds enfants auraient $|t_{\text{enfant}}| < \texttt{min\_samples\_leaf}$.

**Importance des features** agrège les diminutions d'impureté pondérées par le nombre d'échantillons du nœud :

<div>$$\text{FI}(j) = \sum_{t : j_t = j} \frac{|t|}{n} \cdot \Delta I(t, j, \theta_t)$$</div>

normalisée de sorte que $\sum_j \text{FI}(j) = 1$.

</div>
