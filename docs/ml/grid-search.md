# Model Selection — GridSearch

<div class="lang-en">

## API Reference

### GridSearchCV

```python
gs = sp.GridSearchCV(estimator, param_grid, cv=5, seed=42, scoring="auto")
gs.fit(X, y)
```

### RandomizedSearchCV

```python
rs = sp.RandomizedSearchCV(estimator, param_distributions, n_iter=10, cv=5, seed=42, scoring="auto")
rs.fit(X, y)
```

### HalvingGridSearchCV

```python
hgs = sp.HalvingGridSearchCV(estimator, param_grid, cv=5, factor=3, seed=42, scoring="auto")
hgs.fit(X, y)
```

### HalvingRandomSearchCV

```python
hrs = sp.HalvingRandomSearchCV(estimator, param_distributions, n_candidates=256, cv=5, factor=3, seed=42, scoring="auto")
hrs.fit(X, y)
```

---

**Constructor parameters — GridSearchCV**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `estimator` | `str` | — | Model name, e.g. `"Ridge"`, `"RandomForestClassifier"` |
| `param_grid` | `dict[str, list]` | — | Exhaustive grid of hyperparameters |
| `cv` | `int` | `5` | Number of cross-validation folds |
| `seed` | `int` | `42` | Random seed for fold shuffling |
| `scoring` | `str` | `"auto"` | Scoring metric (see below) |

**Constructor parameters — RandomizedSearchCV**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `estimator` | `str` | — | Model name |
| `param_distributions` | `dict[str, list]` | — | Parameter distributions to sample from |
| `n_iter` | `int` | `10` | Number of random parameter combinations |
| `cv` | `int` | `5` | Number of cross-validation folds |
| `seed` | `int` | `42` | Random seed |
| `scoring` | `str` | `"auto"` | Scoring metric |

**Constructor parameters — HalvingGridSearchCV**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `estimator` | `str` | — | Model name |
| `param_grid` | `dict[str, list]` | — | Exhaustive grid of hyperparameters |
| `cv` | `int` | `5` | Number of cross-validation folds |
| `factor` | `int` | `3` | Halving factor — eliminates $1 - \frac{1}{\text{factor}}$ candidates per round |
| `seed` | `int` | `42` | Random seed |
| `scoring` | `str` | `"auto"` | Scoring metric |

**Constructor parameters — HalvingRandomSearchCV**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `estimator` | `str` | — | Model name |
| `param_distributions` | `dict[str, list]` | — | Parameter distributions to sample from |
| `n_candidates` | `int` | `256` | Initial number of random candidates |
| `cv` | `int` | `5` | Number of cross-validation folds |
| `factor` | `int` | `3` | Halving factor |
| `seed` | `int` | `42` | Random seed |
| `scoring` | `str` | `"auto"` | Scoring metric |

**Attributes (all classes)**

| Attribute | Type | Description |
|-----------|------|-------------|
| `best_params_` | `dict` | Best hyperparameter combination found |
| `best_score_` | `float` | Mean CV score of the best combination |
| `n_iterations_` | `int` | Number of halving iterations (Halving variants only) |

---

### Scoring

`"auto"` selects the default metric: **R²** for regressors, **accuracy** for classifiers.

**Regression metrics**

| Value | Formula |
|-------|---------|
| `"r2"` | $R^2 = 1 - \frac{\sum(y_i - \hat y_i)^2}{\sum(y_i - \bar y)^2}$ |
| `"neg_mean_squared_error"` | $-\frac{1}{n}\sum(y_i - \hat y_i)^2$ |
| `"neg_mean_absolute_error"` | $-\frac{1}{n}\sum\|y_i - \hat y_i\|$ |

**Classification metrics**

| Value | Formula |
|-------|---------|
| `"accuracy"` | $\frac{\text{correct}}{n}$ |
| `"f1"` / `"f1_weighted"` / `"f1_macro"` | $F_1 = \frac{2 \cdot P \cdot R}{P + R}$ |
| `"precision"` / `"precision_weighted"` / `"precision_macro"` | $P = \frac{TP}{TP + FP}$ |
| `"recall"` / `"recall_weighted"` / `"recall_macro"` | $R = \frac{TP}{TP + FN}$ |

---

<details>
<summary><strong>Example — GridSearchCV (regression)</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 5)
y = X @ np.array([1.0, -2.0, 0.5, 1.5, -0.8]) + np.random.randn(500) * 0.5

gs = sp.GridSearchCV(
    "Ridge",
    {"alpha": [0.01, 0.1, 1.0, 10.0]},
    cv=5,
    scoring="neg_mean_squared_error",
)
gs.fit(X, y)
print(f"Best params: {gs.best_params_}")
print(f"Best score:  {gs.best_score_:.4f}")
```

</details>

<details>
<summary><strong>Example — RandomizedSearchCV (classification)</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 10)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

rs = sp.RandomizedSearchCV(
    "RandomForestClassifier",
    {
        "n_estimators": [50, 100, 200],
        "max_depth": [3, 5, 10, 15],
        "min_samples_split": [2, 5, 10],
    },
    n_iter=20,
    cv=5,
    scoring="f1",
)
rs.fit(X, y)
print(f"Best params: {rs.best_params_}")
print(f"Best score:  {rs.best_score_:.4f}")
```

</details>

<details>
<summary><strong>Example — HalvingRandomSearchCV</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(1000, 8)
y = X @ np.random.randn(8) + np.random.randn(1000) * 0.3

hrs = sp.HalvingRandomSearchCV(
    "Lasso",
    {"alpha": [0.001, 0.01, 0.1, 0.5, 1.0, 5.0, 10.0]},
    n_candidates=256,
    factor=3,
    cv=5,
)
hrs.fit(X, y)
print(f"Best params:  {hrs.best_params_}")
print(f"Best score:   {hrs.best_score_:.4f}")
print(f"Iterations:   {hrs.n_iterations_}")
```

</details>

---

## Algorithmic Functioning

### Cross-Validation

Each candidate is evaluated using **stratified k-fold** (classification) or **shuffled k-fold** (regression). For $k$ folds, the dataset is split into $k$ subsets; the model trains on $k-1$ folds and scores on the held-out fold. The final score is the mean over all $k$ folds:

$$\text{score} = \frac{1}{k}\sum_{i=1}^{k} S\bigl(\hat f_{-i},\, X_i,\, y_i\bigr)$$

### Exhaustive vs Random Search

**GridSearchCV** evaluates every combination in the Cartesian product of the parameter grid:

$$N_{\text{combos}} = \prod_{j=1}^{d} |V_j|$$

where $|V_j|$ is the number of values for the $j$-th parameter.

**RandomizedSearchCV** samples $n_{\text{iter}}$ combinations uniformly at random.

### Successive Halving

**HalvingGridSearchCV** and **HalvingRandomSearchCV** use the *successive halving* strategy. Starting with a small resource budget $r_0$ and all candidates, each round:

1. Evaluate all remaining candidates with resource $r_i$
2. Keep the top $\frac{1}{\text{factor}}$ candidates
3. Increase the resource: $r_{i+1} = r_i \times \text{factor}$

The initial resource is:

$$r_0 = \max\!\left(\left\lfloor\frac{n}{\text{factor}^{n_{\text{iters}}}}\right\rfloor,\, 1\right) \quad\text{where}\quad n_{\text{iters}} = \lceil\log_{\text{factor}}(C)\rceil$$

and $C$ is the number of candidates. This eliminates weak configurations early while spending full resources only on promising ones.

### Optimizations

| Optimization | Models | Description |
|-------------|--------|-------------|
| **Gram matrix cache** | Lasso, ElasticNet, LinearRegression | Precompute $X^TX$ and $X^Ty$ per fold once, reuse across all $\alpha$ values |
| **KNN distance cache** | KNeighborsClassifier, KNeighborsRegressor | Precompute full distance matrix per fold, reuse across all $k$ values |
| **IRLS fast path** | LogisticRegression | Warm-started iteratively reweighted least squares |
| **Parallel evaluation** | All | Rayon `par_iter` over parameter combinations |

---

## Benchmarks vs scikit-learn GridSearchCV

| Model | SeraPlot | scikit-learn | Speedup |
|-------|----------|-------------|---------|
| Ridge | 5.6 ms | 82 ms | **15×** |
| Lasso | 2.9 ms | 1 200 ms | **418×** |
| ElasticNet | 3.3 ms | 2 283 ms | **686×** |
| LogisticRegression | 137 ms | 5 745 ms | **42×** |
| KNN | 12.9 ms | 1 543 ms | **119×** |
| RandomForest | 6.9 s | 96.8 s | **14×** |
| GradientBoosting | 23.3 s | 320 s | **14×** |

</div>

<div class="lang-fr">

## Référence API

### GridSearchCV

```python
gs = sp.GridSearchCV(estimator, param_grid, cv=5, seed=42, scoring="auto")
gs.fit(X, y)
```

### RandomizedSearchCV

```python
rs = sp.RandomizedSearchCV(estimator, param_distributions, n_iter=10, cv=5, seed=42, scoring="auto")
rs.fit(X, y)
```

### HalvingGridSearchCV

```python
hgs = sp.HalvingGridSearchCV(estimator, param_grid, cv=5, factor=3, seed=42, scoring="auto")
hgs.fit(X, y)
```

### HalvingRandomSearchCV

```python
hrs = sp.HalvingRandomSearchCV(estimator, param_distributions, n_candidates=256, cv=5, factor=3, seed=42, scoring="auto")
hrs.fit(X, y)
```

---

**Paramètres du constructeur — GridSearchCV**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `estimator` | `str` | — | Nom du modèle, ex. `"Ridge"`, `"RandomForestClassifier"` |
| `param_grid` | `dict[str, list]` | — | Grille exhaustive d'hyperparamètres |
| `cv` | `int` | `5` | Nombre de plis de validation croisée |
| `seed` | `int` | `42` | Graine aléatoire |
| `scoring` | `str` | `"auto"` | Métrique de score (voir ci-dessous) |

**Paramètres du constructeur — RandomizedSearchCV**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `estimator` | `str` | — | Nom du modèle |
| `param_distributions` | `dict[str, list]` | — | Distributions à échantillonner |
| `n_iter` | `int` | `10` | Nombre de combinaisons aléatoires |
| `cv` | `int` | `5` | Nombre de plis |
| `seed` | `int` | `42` | Graine aléatoire |
| `scoring` | `str` | `"auto"` | Métrique de score |

**Paramètres du constructeur — HalvingGridSearchCV**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `estimator` | `str` | — | Nom du modèle |
| `param_grid` | `dict[str, list]` | — | Grille exhaustive |
| `cv` | `int` | `5` | Nombre de plis |
| `factor` | `int` | `3` | Facteur de réduction — élimine $1 - \frac{1}{\text{factor}}$ candidats par tour |
| `seed` | `int` | `42` | Graine aléatoire |
| `scoring` | `str` | `"auto"` | Métrique de score |

**Paramètres du constructeur — HalvingRandomSearchCV**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `estimator` | `str` | — | Nom du modèle |
| `param_distributions` | `dict[str, list]` | — | Distributions à échantillonner |
| `n_candidates` | `int` | `256` | Nombre initial de candidats |
| `cv` | `int` | `5` | Nombre de plis |
| `factor` | `int` | `3` | Facteur de réduction |
| `seed` | `int` | `42` | Graine aléatoire |
| `scoring` | `str` | `"auto"` | Métrique de score |

**Attributs (toutes les classes)**

| Attribut | Type | Description |
|----------|------|-------------|
| `best_params_` | `dict` | Meilleure combinaison d'hyperparamètres |
| `best_score_` | `float` | Score CV moyen de la meilleure combinaison |
| `n_iterations_` | `int` | Nombre d'itérations (variantes Halving uniquement) |

---

### Scoring

`"auto"` sélectionne la métrique par défaut : **R²** pour les régresseurs, **accuracy** pour les classifieurs.

**Métriques de régression**

| Valeur | Formule |
|--------|---------|
| `"r2"` | $R^2 = 1 - \frac{\sum(y_i - \hat y_i)^2}{\sum(y_i - \bar y)^2}$ |
| `"neg_mean_squared_error"` | $-\frac{1}{n}\sum(y_i - \hat y_i)^2$ |
| `"neg_mean_absolute_error"` | $-\frac{1}{n}\sum\|y_i - \hat y_i\|$ |

**Métriques de classification**

| Valeur | Formule |
|--------|---------|
| `"accuracy"` | $\frac{\text{correct}}{n}$ |
| `"f1"` / `"f1_weighted"` / `"f1_macro"` | $F_1 = \frac{2 \cdot P \cdot R}{P + R}$ |
| `"precision"` / `"precision_weighted"` / `"precision_macro"` | $P = \frac{TP}{TP + FP}$ |
| `"recall"` / `"recall_weighted"` / `"recall_macro"` | $R = \frac{TP}{TP + FN}$ |

---

<details>
<summary><strong>Exemple — GridSearchCV (régression)</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 5)
y = X @ np.array([1.0, -2.0, 0.5, 1.5, -0.8]) + np.random.randn(500) * 0.5

gs = sp.GridSearchCV(
    "Ridge",
    {"alpha": [0.01, 0.1, 1.0, 10.0]},
    cv=5,
    scoring="neg_mean_squared_error",
)
gs.fit(X, y)
print(f"Meilleurs params : {gs.best_params_}")
print(f"Meilleur score :   {gs.best_score_:.4f}")
```

</details>

<details>
<summary><strong>Exemple — RandomizedSearchCV (classification)</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 10)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

rs = sp.RandomizedSearchCV(
    "RandomForestClassifier",
    {
        "n_estimators": [50, 100, 200],
        "max_depth": [3, 5, 10, 15],
        "min_samples_split": [2, 5, 10],
    },
    n_iter=20,
    cv=5,
    scoring="f1",
)
rs.fit(X, y)
print(f"Meilleurs params : {rs.best_params_}")
print(f"Meilleur score :   {rs.best_score_:.4f}")
```

</details>

<details>
<summary><strong>Exemple — HalvingRandomSearchCV</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(1000, 8)
y = X @ np.random.randn(8) + np.random.randn(1000) * 0.3

hrs = sp.HalvingRandomSearchCV(
    "Lasso",
    {"alpha": [0.001, 0.01, 0.1, 0.5, 1.0, 5.0, 10.0]},
    n_candidates=256,
    factor=3,
    cv=5,
)
hrs.fit(X, y)
print(f"Meilleurs params : {hrs.best_params_}")
print(f"Meilleur score :   {hrs.best_score_:.4f}")
print(f"Itérations :       {hrs.n_iterations_}")
```

</details>

---

## Fonctionnement algorithmique

### Validation croisée

Chaque candidat est évalué par **k-fold stratifié** (classification) ou **k-fold mélangé** (régression). Le score final est la moyenne sur les $k$ plis :

$$\text{score} = \frac{1}{k}\sum_{i=1}^{k} S\bigl(\hat f_{-i},\, X_i,\, y_i\bigr)$$

### Recherche exhaustive vs aléatoire

**GridSearchCV** évalue chaque combinaison du produit cartésien :

$$N_{\text{combos}} = \prod_{j=1}^{d} |V_j|$$

**RandomizedSearchCV** échantillonne $n_{\text{iter}}$ combinaisons uniformément.

### Successive Halving (réduction successive)

**HalvingGridSearchCV** et **HalvingRandomSearchCV** utilisent la stratégie de *réduction successive*. À chaque tour :

1. Évaluer tous les candidats restants avec le budget $r_i$
2. Garder le top $\frac{1}{\text{factor}}$ des candidats
3. Augmenter le budget : $r_{i+1} = r_i \times \text{factor}$

Le budget initial est :

$$r_0 = \max\!\left(\left\lfloor\frac{n}{\text{factor}^{n_{\text{iters}}}}\right\rfloor,\, 1\right) \quad\text{où}\quad n_{\text{iters}} = \lceil\log_{\text{factor}}(C)\rceil$$

Les configurations faibles sont éliminées tôt ; seules les prometteuses reçoivent le budget complet.

### Optimisations

| Optimisation | Modèles | Description |
|-------------|---------|-------------|
| **Cache matrice de Gram** | Lasso, ElasticNet, LinearRegression | Précalcul de $X^TX$ et $X^Ty$ par pli, réutilisé pour toutes les valeurs de $\alpha$ |
| **Cache distances KNN** | KNeighborsClassifier, KNeighborsRegressor | Précalcul de la matrice de distances par pli |
| **Chemin rapide IRLS** | LogisticRegression | Moindres carrés repondérés itérativement avec démarrage à chaud |
| **Évaluation parallèle** | Tous | `par_iter` Rayon sur les combinaisons |

---

## Performances vs scikit-learn GridSearchCV

| Modèle | SeraPlot | scikit-learn | Accélération |
|--------|----------|-------------|--------------|
| Ridge | 5,6 ms | 82 ms | **15×** |
| Lasso | 2,9 ms | 1 200 ms | **418×** |
| ElasticNet | 3,3 ms | 2 283 ms | **686×** |
| LogisticRegression | 137 ms | 5 745 ms | **42×** |
| KNN | 12,9 ms | 1 543 ms | **119×** |
| RandomForest | 6,9 s | 96,8 s | **14×** |
| GradientBoosting | 23,3 s | 320 s | **14×** |

</div>
