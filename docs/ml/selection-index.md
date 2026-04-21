# Model Selection

<div class="lang-en">

Model selection tools automate hyperparameter search using cross-validation. SeraPlot provides four search strategies, from exhaustive grid search to efficient successive halving.

| Class | Strategy | Description |
|-------|----------|-------------|
| [GridSearchCV](grid-search.md) | Exhaustive | Evaluates every combination in the parameter grid |
| [RandomizedSearchCV](grid-search.md) | Random | Samples `n_iter` combinations uniformly at random |
| [HalvingGridSearchCV](grid-search.md) | Halving | Exhaustive candidates + successive halving to reduce cost |
| [HalvingRandomSearchCV](grid-search.md) | Halving + Random | Random candidates + successive halving |

All four are documented on the same page: [GridSearchCV / RandomizedSearchCV / Halving](grid-search.md).

### Choosing a search strategy

| Situation | Recommended |
|-----------|-------------|
| Small parameter grid | `GridSearchCV` |
| Large parameter space | `RandomizedSearchCV` |
| Large dataset + large grid | `HalvingGridSearchCV` |
| Large dataset + large space | `HalvingRandomSearchCV` |

### Quick example

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 5)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

gs = sp.GridSearchCV(
    "LogisticRegression",
    {"C": [0.01, 0.1, 1.0, 10.0]},
    cv=5,
    scoring="accuracy",
)
gs.fit(X, y)
print(f"Best C: {gs.best_params_}")
print(f"CV accuracy: {gs.best_score_:.4f}")
```

</div>

<div class="lang-fr">

Les outils de sélection de modèles automatisent la recherche d'hyperparamètres par validation croisée. SeraPlot propose quatre stratégies de recherche, de la grille exhaustive au halving successif efficace.

| Classe | Stratégie | Description |
|-------|----------|-------------|
| [GridSearchCV](grid-search.md) | Exhaustive | Évalue toutes les combinaisons de la grille de paramètres |
| [RandomizedSearchCV](grid-search.md) | Aléatoire | Échantillonne `n_iter` combinaisons uniformément au hasard |
| [HalvingGridSearchCV](grid-search.md) | Halving | Candidats exhaustifs + halving successif pour réduire le coût |
| [HalvingRandomSearchCV](grid-search.md) | Halving + Aléatoire | Candidats aléatoires + halving successif |

Les quatre sont documentés sur la même page : [GridSearchCV / RandomizedSearchCV / Halving](grid-search.md).

### Choisir une stratégie de recherche

| Situation | Recommandé |
|-----------|-------------|
| Petite grille de paramètres | `GridSearchCV` |
| Grand espace de paramètres | `RandomizedSearchCV` |
| Grand jeu de données + grande grille | `HalvingGridSearchCV` |
| Grand jeu de données + grand espace | `HalvingRandomSearchCV` |

### Exemple rapide

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 5)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

gs = sp.GridSearchCV(
    "LogisticRegression",
    {"C": [0.01, 0.1, 1.0, 10.0]},
    cv=5,
    scoring="accuracy",
)
gs.fit(X, y)
print(f"Meilleur C : {gs.best_params_}")
print(f"Précision CV : {gs.best_score_:.4f}")
```

</div>
