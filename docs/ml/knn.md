# KNeighborsClassifier / KNeighborsRegressor

<div class="lang-en">

## API Reference

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

<div>$$d(x, x') = \|x - x'\|_2 = \sqrt{\sum_{j=1}^p (x_j - x'_j)^2}$$</div>

**Neighbourhood** — for a query point $x$, the $k$ nearest training samples:

<div>$$\mathcal{N}_k(x) = \text{top-}k \text{ smallest } d(x, x_i), \quad x_i \in \mathcal{D}_{\text{train}}$$</div>

**Classifier — majority vote** across the $k$ neighbours:

<div>$$\hat{y} = \underset{c}{\arg\max} \sum_{x_i \in \mathcal{N}_k(x)} \mathbf{1}[y_i = c]$$</div>

**Class probability estimate:**

<div>$$\hat{p}(y = c \mid x) = \frac{1}{k}\sum_{x_i \in \mathcal{N}_k(x)} \mathbf{1}[y_i = c]$$</div>

**Regressor — mean of neighbours:**

<div>$$\hat{y} = \frac{1}{k}\sum_{x_i \in \mathcal{N}_k(x)} y_i$$</div>

**Complexity trade-offs:**
- Training: $O(1)$ — just store $\mathcal{D}$
- Prediction: $O(nd)$ — brute-force scan (no index built)
- Memory: $O(nd)$ — full training set retained

**Effect of $k$** — small $k$ fits the training data tightly (high variance); large $k$ smooths the decision boundary (high bias). Optimal $k$ is tuned via cross-validation.

</div>

<div class="lang-fr">

## Référence API

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

<div>$$d(x, x') = \|x - x'\|_2 = \sqrt{\sum_{j=1}^p (x_j - x'_j)^2}$$</div>

**Voisinage** — pour un point de requête $x$, les $k$ échantillons d'entraînement les plus proches :

<div>$$\mathcal{N}_k(x) = \text{top-}k \text{ plus petites } d(x, x_i), \quad x_i \in \mathcal{D}_{\text{train}}$$</div>

**Classificateur — vote majoritaire** parmi les $k$ voisins :

<div>$$\hat{y} = \underset{c}{\arg\max} \sum_{x_i \in \mathcal{N}_k(x)} \mathbf{1}[y_i = c]$$</div>

**Estimation de la probabilité de classe :**

<div>$$\hat{p}(y = c \mid x) = \frac{1}{k}\sum_{x_i \in \mathcal{N}_k(x)} \mathbf{1}[y_i = c]$$</div>

**Régresseur — moyenne des voisins :**

<div>$$\hat{y} = \frac{1}{k}\sum_{x_i \in \mathcal{N}_k(x)} y_i$$</div>

**Compromis de complexité :**
- Entraînement : $O(1)$ — juste stocker $\mathcal{D}$
- Prédiction : $O(nd)$ — scan brute-force (aucun index construit)
- Mémoire : $O(nd)$ — ensemble d'entraînement complet retenu

**Effet de $k$** — un $k$ petit ajuste étroitement les données d'entraînement (haute variance) ; un grand $k$ lisse la frontière de décision (biais élevé). Le $k$ optimal est ajusté par validation croisée.

</div>
