# KNeighborsClassifier / KNeighborsRegressor

<div class="lang-en">

## Code

**Signature**

```python
clf = sp.KNeighborsClassifier(n_neighbors=5)
reg = sp.KNeighborsRegressor(n_neighbors=5)

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

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels (classifier only) |
| `n_neighbors_` | `int` | The $k$ value in use |

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
print(f"R²: {reg.score(X, X[:, 0] + X[:, 1]):.4f}")
```

</details>

---

## Algorithmic Functioning

$k$-Nearest Neighbours is a **non-parametric, lazy** algorithm: no model is fitted at training time — the entire dataset is stored and queried at prediction time.

**Distance metric** — Euclidean distance between two points $x, x' \in \mathbb{R}^p$:

$$d(x, x') = \|x - x'\|_2 = \sqrt{\sum_{j=1}^p (x_j - x'_j)^2}$$

**Neighbourhood** — for a query point $x$, the $k$ nearest training samples:

$$\mathcal{N}_k(x) = \text{top-}k \text{ smallest } d(x, x_i), \quad x_i \in \mathcal{D}_{\text{train}}$$

**Classifier — majority vote** across the $k$ neighbours:

$$\hat{y} = \underset{c}{\arg\max} \sum_{x_i \in \mathcal{N}_k(x)} \mathbf{1}[y_i = c]$$

**Class probability estimate:**

$$\hat{p}(y = c \mid x) = \frac{1}{k}\sum_{x_i \in \mathcal{N}_k(x)} \mathbf{1}[y_i = c]$$

**Regressor — mean of neighbours:**

$$\hat{y} = \frac{1}{k}\sum_{x_i \in \mathcal{N}_k(x)} y_i$$

**Complexity trade-offs:**
- Training: $O(1)$ — just store $\mathcal{D}$
- Prediction: $O(nd)$ — brute-force scan (no index built)
- Memory: $O(nd)$ — full training set retained

**Effect of $k$** — small $k$ fits the training data tightly (high variance); large $k$ smooths the decision boundary (high bias). Optimal $k$ is tuned via cross-validation.

</div>

<div class="lang-fr">

## Code

**Signature**

```python
clf = sp.KNeighborsClassifier(n_neighbors=5)
reg = sp.KNeighborsRegressor(n_neighbors=5)

model.fit(X, y)
model.predict(X)               -> list[int] | list[float]
model.predict_proba(X)         -> ndarray (n, K)   # classificateur seulement
model.score(X, y)              -> float
model.get_params()             -> dict
model.set_params(n_neighbors=...)
```

**Paramètres du constructeur**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_neighbors` | `int` | `5` | Nombre de voisins les plus proches $k$ |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `classes_` | `list[int]` | Labels de classes uniques (classificateur seulement) |
| `n_neighbors_` | `int` | La valeur $k$ utilisée |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 4)
y = (X[:, 0] ** 2 + X[:, 1] ** 2 < 1).astype(int)

clf = sp.KNeighborsClassifier(n_neighbors=7)
clf.fit(X, y)
print(f"Précision : {clf.score(X, y):.4f}")

reg = sp.KNeighborsRegressor(n_neighbors=7)
reg.fit(X, X[:, 0] + X[:, 1])
print(f"R² : {reg.score(X, X[:, 0] + X[:, 1]):.4f}")
```

</details>

---

## Fonctionnement algorithmique

$k$-Nearest Neighbours est un algorithme **non-paramétrique et paresseux** : aucun modèle n'est ajusté lors de l'entraînement — l'ensemble du jeu de données est stocké et interrogé au moment de la prédiction.

**Métrique de distance** — distance euclidienne entre deux points $x, x' \in \mathbb{R}^p$ :

$$d(x, x') = \|x - x'\|_2 = \sqrt{\sum_{j=1}^p (x_j - x'_j)^2}$$

**Voisinage** — pour un point de requête $x$, les $k$ échantillons d'entraînement les plus proches :

$$\mathcal{N}_k(x) = \text{top-}k \text{ plus petites } d(x, x_i), \quad x_i \in \mathcal{D}_{\text{train}}$$

**Classificateur — vote majoritaire** parmi les $k$ voisins :

$$\hat{y} = \underset{c}{\arg\max} \sum_{x_i \in \mathcal{N}_k(x)} \mathbf{1}[y_i = c]$$

**Estimation de la probabilité de classe :**

$$\hat{p}(y = c \mid x) = \frac{1}{k}\sum_{x_i \in \mathcal{N}_k(x)} \mathbf{1}[y_i = c]$$

**Régresseur — moyenne des voisins :**

$$\hat{y} = \frac{1}{k}\sum_{x_i \in \mathcal{N}_k(x)} y_i$$

**Compromis de complexité :**
- Entraînement : $O(1)$ — juste stocker $\mathcal{D}$
- Prédiction : $O(nd)$ — scan brute-force (aucun index construit)
- Mémoire : $O(nd)$ — ensemble d'entraînement complet retenu

**Effet de $k$** — un $k$ petit ajuste étroitement les données d'entraînement (haute variance) ; un grand $k$ lisse la frontière de décision (biais élevé). Le $k$ optimal est ajusté par validation croisée.

</div>
# KNeighborsClassifier / KNeighborsRegressor / NearestCentroid

<div class="lang-en">

## Signature

```python
clf = sp.KNeighborsClassifier(n_neighbors: int = 5, weights: str = "uniform")
reg = sp.KNeighborsRegressor(n_neighbors: int = 5, weights: str = "uniform")
nc  = sp.NearestCentroid()

clf.classes_       -> list[int]
clf.n_neighbors_   -> int
clf.weights_       -> str

reg.n_neighbors_   -> int
reg.weights_       -> str

nc.classes_        -> list[int]
```

---

## Description

K-Nearest Neighbors with brute-force distance computation and partial sort for O(n) neighbor selection. Parallelized via Rayon when `n >= 256`.

`weights` options: `"uniform"` (all neighbors equal weight), `"distance"` (inverse distance weighting).

**NearestCentroid** classifies by computing the centroid of each class, then assigning the nearest.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_neighbors` | `int` | `5` | Number of neighbors |
| `weights` | `str` | `"uniform"` | `"uniform"` or `"distance"` |

</div>

<div class="lang-fr">

## Description

K plus proches voisins avec calcul brute-force et tri partiel pour la sélection en O(n). Parallélisé via Rayon lorsque `n >= 256`.

Options pour `weights` : `"uniform"` (poids égaux), `"distance"` (pondération inverse par la distance).

**NearestCentroid** classifie en calculant le centroïde de chaque classe puis en assignant au plus proche.

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_neighbors` | `int` | `5` | Nombre de voisins |
| `weights` | `str` | `"uniform"` | `"uniform"` ou `"distance"` |

</div>


---

## Constructor Parameters

### KNeighborsClassifier / KNeighborsRegressor

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_neighbors` | `int` | `5` | Number of neighbors |
| `weights` | `str` | `"uniform"` | `"uniform"` or `"distance"` (inverse distance weighting) |

### NearestCentroid

No parameters.

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels |

---

## Examples

<details>
<summary><strong>KNN classification</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
y = (X[:, 0] > 0).astype(np.int32)

clf = sp.KNeighborsClassifier(n_neighbors=7)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.4f}")

proba = clf.predict_proba(X[:3])
for row in proba:
    print(f"  {[f'{v:.3f}' for v in row]}")
```

</details>

<details>
<summary><strong>KNN regression</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 2)
y = np.sin(X[:, 0]) + X[:, 1]

reg = sp.KNeighborsRegressor(n_neighbors=5)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}")
```

</details>

<details>
<summary><strong>NearestCentroid</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.vstack([np.random.randn(100, 2) + [2, 2], np.random.randn(100, 2) + [-2, -2]])
y = np.array([0] * 100 + [1] * 100, dtype=np.int32)

nc = sp.NearestCentroid()
nc.fit(X, y)
print(f"Accuracy: {nc.score(X, y):.4f}")
```

</details>
