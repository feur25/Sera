# KNeighborsClassifier / KNeighborsRegressor

<div class="lang-en">

## API Reference

**Signature**

```python
clf = sp.KNeighborsClassifier(n_neighbors=5, weights="uniform")
reg = sp.KNeighborsRegressor(n_neighbors=5, weights="uniform")

model.fit(X, y)
model.predict(X)               -> list[int] | list[float]
model.predict_proba(X)         -> ndarray (n, K)   # classifier only
model.score(X, y)              -> float
model.get_params()             -> dict
model.set_params(n_neighbors=...)
```

**Constructor parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_neighbors` | `int` | `5` | Number of nearest neighbours $k$ |
| `weights` | `str` | `"uniform"` | Weighting strategy: `"uniform"` (equal) or `"distance"` (inverse distance) |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels (classifier only) |
| `n_neighbors_` | `int` | The $k$ value in use |
| `weights_` | `str` | The weighting strategy in use |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 4)
y = (X[:, 0] ** 2 + X[:, 1] ** 2 < 1).astype(int)

clf = sp.KNeighborsClassifier(n_neighbors=7)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")

reg = sp.KNeighborsRegressor(n_neighbors=7)
reg.fit(X, X[:, 0] + X[:, 1])
print(f"R\u00b2: {reg.score(X, X[:, 0] + X[:, 1]):.4f}")
```

</details>

---

## Algorithmic Functioning

$k$-Nearest Neighbours is a **non-parametric, lazy** algorithm: no model is fitted at training time \u2014 the entire dataset is stored and queried at prediction time.

**Distance metric** \u2014 Euclidean distance between two points $x, x' \in \mathbb{R}^p$:

<div>$$d(x, x') = \|x - x'\|_2 = \sqrt{\sum_{j=1}^p (x_j - x'_j)^2}$$</div>

**Neighbourhood** \u2014 for a query point $x$, the $k$ nearest training samples:

<div>$$\mathcal{N}_k(x) = \text{top-}k \text{ smallest } d(x, x_i), \quad x_i \in \mathcal{D}_{\text{train}}$$</div>

**Classifier \u2014 majority vote** across the $k$ neighbours:

<div>$$\hat{y} = \underset{c}{\arg\max} \sum_{x_i \in \mathcal{N}_k(x)} \mathbf{1}[y_i = c]$$</div>

**Class probability estimate:**

<div>$$\hat{p}(y = c \mid x) = \frac{1}{k}\sum_{x_i \in \mathcal{N}_k(x)} \mathbf{1}[y_i = c]$$</div>

**Regressor \u2014 mean of neighbours:**

<div>$$\hat{y} = \frac{1}{k}\sum_{x_i \in \mathcal{N}_k(x)} y_i$$</div>

**Complexity trade-offs:**
- Training: $O(1)$ \u2014 just store $\mathcal{D}$
- Prediction: $O(nd)$ \u2014 brute-force scan (no index built)
- Memory: $O(nd)$ \u2014 full training set retained

**Effect of $k$** \u2014 small $k$ fits the training data tightly (high variance); large $k$ smooths the decision boundary (high bias). Optimal $k$ is tuned via cross-validation.

---

## NearestCentroid

**Signature**

```python
clf = sp.NearestCentroid()

clf.fit(X, y)
clf.predict(X)   -> list[int]
clf.score(X, y)  -> float
```

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels |

No constructor parameters. Assigns each query point to the class whose centroid (mean of training points) is closest.

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(int)
clf = sp.NearestCentroid()
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")
```

</div>

<div class="lang-fr">

## R\u00e9f\u00e9rence API

**Signature**

```python
clf = sp.KNeighborsClassifier(n_neighbors=5, weights="uniform")
reg = sp.KNeighborsRegressor(n_neighbors=5, weights="uniform")

model.fit(X, y)
model.predict(X)               -> list[int] | list[float]
model.predict_proba(X)         -> ndarray (n, K)   # classificateur seulement
model.score(X, y)              -> float
model.get_params()             -> dict
model.set_params(n_neighbors=...)
```

**Param\u00e8tres du constructeur**

| Param\u00e8tre | Type | D\u00e9faut | Description |
|-----------|------|--------|-------------|
| `n_neighbors` | `int` | `5` | Nombre de voisins les plus proches $k$ |
| `weights` | `str` | `"uniform"` | Pond\u00e9ration\u00a0: `"uniform"` (\u00e9gale) ou `"distance"` (distance inverse) |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `classes_` | `list[int]` | Labels de classes uniques (classificateur seulement) |
| `n_neighbors_` | `int` | La valeur $k$ utilis\u00e9e |
| `weights_` | `str` | La strat\u00e9gie de pond\u00e9ration utilis\u00e9e |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 4)
y = (X[:, 0] ** 2 + X[:, 1] ** 2 < 1).astype(int)

clf = sp.KNeighborsClassifier(n_neighbors=7)
clf.fit(X, y)
print(f"Pr\u00e9cision\u00a0: {clf.score(X, y):.4f}")

reg = sp.KNeighborsRegressor(n_neighbors=7)
reg.fit(X, X[:, 0] + X[:, 1])
print(f"R\u00b2\u00a0: {reg.score(X, X[:, 0] + X[:, 1]):.4f}")
```

</details>

---

## Fonctionnement algorithmique

$k$-Nearest Neighbours est un algorithme **non-param\u00e9trique et paresseux**\u00a0: aucun mod\u00e8le n'est ajust\u00e9 lors de l'entra\u00eenement \u2014 l'ensemble du jeu de donn\u00e9es est stock\u00e9 et interrog\u00e9 au moment de la pr\u00e9diction.

**M\u00e9trique de distance** \u2014 distance euclidienne entre deux points $x, x' \in \mathbb{R}^p$\u00a0:

<div>$$d(x, x') = \|x - x'\|_2 = \sqrt{\sum_{j=1}^p (x_j - x'_j)^2}$$</div>

**Voisinage** \u2014 pour un point de requ\u00eate $x$, les $k$ \u00e9chantillons d'entra\u00eenement les plus proches\u00a0:

<div>$$\mathcal{N}_k(x) = \text{top-}k \text{ plus petites } d(x, x_i), \quad x_i \in \mathcal{D}_{\text{train}}$$</div>

**Classificateur \u2014 vote majoritaire** parmi les $k$ voisins\u00a0:

<div>$$\hat{y} = \underset{c}{\arg\max} \sum_{x_i \in \mathcal{N}_k(x)} \mathbf{1}[y_i = c]$$</div>

**Estimation de la probabilit\u00e9 de classe\u00a0:**

<div>$$\hat{p}(y = c \mid x) = \frac{1}{k}\sum_{x_i \in \mathcal{N}_k(x)} \mathbf{1}[y_i = c]$$</div>

**R\u00e9gresseur \u2014 moyenne des voisins\u00a0:**

<div>$$\hat{y} = \frac{1}{k}\sum_{x_i \in \mathcal{N}_k(x)} y_i$$</div>

**Compromis de complexit\u00e9\u00a0:**
- Entra\u00eenement\u00a0: $O(1)$ \u2014 juste stocker $\mathcal{D}$
- Pr\u00e9diction\u00a0: $O(nd)$ \u2014 scan brute-force (aucun index construit)
- M\u00e9moire\u00a0: $O(nd)$ \u2014 ensemble d'entra\u00eenement complet retenu

**Effet de $k$** \u2014 un $k$ petit ajuste \u00e9troitement les donn\u00e9es d'entra\u00eenement (haute variance)\u00a0; un grand $k$ lisse la fronti\u00e8re de d\u00e9cision (biais \u00e9lev\u00e9). Le $k$ optimal est ajust\u00e9 par validation crois\u00e9e.

---

## NearestCentroid

**Signature**

```python
clf = sp.NearestCentroid()

clf.fit(X, y)
clf.predict(X)   -> list[int]
clf.score(X, y)  -> float
```

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `classes_` | `list[int]` | Labels de classes uniques |

Aucun param\u00e8tre de constructeur. Affecte chaque point de requ\u00eate \u00e0 la classe dont le centro\u00efde (moyenne des points d'entra\u00eenement) est le plus proche.

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(int)
clf = sp.NearestCentroid()
clf.fit(X, y)
print(f"Pr\u00e9cision\u00a0: {clf.score(X, y):.4f}")
```

</div>
