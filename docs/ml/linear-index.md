# Linear Models

<div class="lang-en">

Linear models learn a linear relationship between features and the target. SeraPlot implements the full sklearn-compatible suite for both regression and classification.

| Model | Task | Description |
|-------|------|-------------|
| [LinearRegression](linear-regression.md) | Regression | Ordinary least squares (OLS) with optional intercept |
| [Ridge / RidgeClassifier](ridge.md) | Both | L2-regularised regression and classification |
| [Lasso](lasso.md) | Regression | L1-regularised regression with coordinate descent |
| [ElasticNet](elastic-net.md) | Regression | Combined L1 + L2 regularisation |
| [LogisticRegression](logistic-regression.md) | Classification | Sigmoid + L2 regularisation, L-BFGS optimiser |
| [SGDClassifier / SGDRegressor](sgd.md) | Both | Stochastic gradient descent with multiple loss functions |

### Choosing the right model

| Situation | Recommended model |
|-----------|------------------|
| No regularisation needed | `LinearRegression` |
| Collinear features | `Ridge` |
| Feature selection (sparse output) | `Lasso` |
| Mix of sparsity and grouping | `ElasticNet` |
| Binary / multi-class classification | `LogisticRegression` |
| Very large datasets | `SGDClassifier` / `SGDRegressor` |

</div>

<div class="lang-fr">

Les modèles linéaires apprennent une relation linéaire entre les variables et la cible. SeraPlot implémente la suite complète compatible scikit-learn pour la régression et la classification.

| Modèle | Tâche | Description |
|-------|------|-------------|
| [LinearRegression](linear-regression.md) | Régression | Moindres carrés ordinaires (OLS) avec intercept optionnel |
| [Ridge / RidgeClassifier](ridge.md) | Les deux | Régression et classification avec régularisation L2 |
| [Lasso](lasso.md) | Régression | Régression avec régularisation L1 et descente de coordonnées |
| [ElasticNet](elastic-net.md) | Régression | Régularisation combinée L1 + L2 |
| [LogisticRegression](logistic-regression.md) | Classification | Sigmoïde + régularisation L2, optimiseur L-BFGS |
| [SGDClassifier / SGDRegressor](sgd.md) | Les deux | Descente de gradient stochastique avec plusieurs fonctions de perte |

### Choisir le bon modèle

| Situation | Modèle recommandé |
|-----------|------------------|
| Aucune régularisation nécessaire | `LinearRegression` |
| Variables corrélées | `Ridge` |
| Sélection de variables (sortie sparse) | `Lasso` |
| Mix de sparsité et groupement | `ElasticNet` |
| Classification binaire / multi-classe | `LogisticRegression` |
| Très grands ensembles de données | `SGDClassifier` / `SGDRegressor` |

</div>
