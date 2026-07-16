# Machine Learning

<div class="lang-en">

SeraML is SeraPlot's native machine learning surface — clustering, linear models, tree-based ensembles, neighbors, naive Bayes, SVM, preprocessing, decomposition, model selection, metrics, and GPU/distributed execution, all callable directly from `seraplot` without a separate install.

| Category | Contents |
|---|---|
| [Clustering](clustering-index.md) | [K-Means](kmeans.md), [K-Means Class](kmeans-class.md), [DBSCAN 2D](dbscan.md), [DBSCAN 3D](dbscan3d.md), [DBSCAN Class](dbscan-class.md) |
| [Linear Models](linear-index.md) | [LinearRegression](linear-regression.md), [Ridge / RidgeClassifier](ridge.md), [Lasso](lasso.md), [ElasticNet](elastic-net.md), [LogisticRegression](logistic-regression.md), [SGDClassifier / SGDRegressor](sgd.md) |
| [Tree-Based](tree-index.md) | [DecisionTree](decision-tree.md), [RandomForest](random-forest.md), [GradientBoosting](gradient-boosting.md), [AdaBoost](adaboost.md) |
| [Neighbors](neighbors-index.md) | [KNN / NearestCentroid](knn.md) |
| [Naive Bayes](bayes-index.md) | [GaussianNB / MultinomialNB / BernoulliNB](naive-bayes.md) |
| [SVM](svm-index.md) | [LinearSVC / LinearSVR](svm.md) |
| [Preprocessing](preprocessing-index.md) | [StandardScaler / MinMaxScaler / RobustScaler / MaxAbsScaler / Normalizer](preprocessing.md), [Advanced Preprocessing](preprocessing-advanced.md) |
| [Decomposition](decomposition-index.md) | [PCA / TruncatedSVD](decomposition.md) |
| [Model Selection](selection-index.md) | [GridSearchCV / RandomizedSearchCV / Halving](grid-search.md), [Cross-Validation Splitters](cv-splitters.md), [Permutation Importance](permutation-importance.md) |
| — | [Anomaly Detection](isolation-forest.md), [Model Registry](registry.md), [PowerBI / Tableau Export](export.md), [GPU Backend](gpu.md), [Distributed & Cloud](distributed.md) |
| [Metrics](metrics.md) | [Classification](metrics-classification.md), [Regression](metrics-regression.md), [Clustering](metrics-clustering.md) |
| — | [Train / Test Split](train-test-split.md) |

Every estimator follows the same shape: construct with keyword hyperparameters, `.fit(x, y)` / `.fit_predict(x)`, then either read `.labels_`/`.coef_`-style attributes or hand the result straight to a SeraPlot chart function.

</div>

<div class="lang-fr">

SeraML est la surface de machine learning native de SeraPlot — clustering, modèles linéaires, ensembles d'arbres, voisins, naive Bayes, SVM, prétraitement, décomposition, sélection de modèle, métriques, et exécution GPU/distribuée, le tout appelable directement depuis `seraplot` sans installation séparée.

| Catégorie | Contenu |
|---|---|
| [Clustering](clustering-index.md) | [K-Means](kmeans.md), [Classe K-Means](kmeans-class.md), [DBSCAN 2D](dbscan.md), [DBSCAN 3D](dbscan3d.md), [Classe DBSCAN](dbscan-class.md) |
| [Modèles linéaires](linear-index.md) | [LinearRegression](linear-regression.md), [Ridge / RidgeClassifier](ridge.md), [Lasso](lasso.md), [ElasticNet](elastic-net.md), [LogisticRegression](logistic-regression.md), [SGDClassifier / SGDRegressor](sgd.md) |
| [Arbres](tree-index.md) | [DecisionTree](decision-tree.md), [RandomForest](random-forest.md), [GradientBoosting](gradient-boosting.md), [AdaBoost](adaboost.md) |
| [Voisins](neighbors-index.md) | [KNN / NearestCentroid](knn.md) |
| [Naive Bayes](bayes-index.md) | [GaussianNB / MultinomialNB / BernoulliNB](naive-bayes.md) |
| [SVM](svm-index.md) | [LinearSVC / LinearSVR](svm.md) |
| [Prétraitement](preprocessing-index.md) | [StandardScaler / MinMaxScaler / RobustScaler / MaxAbsScaler / Normalizer](preprocessing.md), [Prétraitement avancé](preprocessing-advanced.md) |
| [Décomposition](decomposition-index.md) | [PCA / TruncatedSVD](decomposition.md) |
| [Sélection de modèle](selection-index.md) | [GridSearchCV / RandomizedSearchCV / Halving](grid-search.md), [Splitters de validation croisée](cv-splitters.md), [Importance par permutation](permutation-importance.md) |
| — | [Détection d'anomalies](isolation-forest.md), [Registre de modèles](registry.md), [Export PowerBI / Tableau](export.md), [Backend GPU](gpu.md), [Distribué & Cloud](distributed.md) |
| [Métriques](metrics.md) | [Classification](metrics-classification.md), [Régression](metrics-regression.md), [Clustering](metrics-clustering.md) |
| — | [Split entraînement / test](train-test-split.md) |

Chaque estimateur suit la même forme : construction avec des hyperparamètres en kwargs, `.fit(x, y)` / `.fit_predict(x)`, puis lecture des attributs de type `.labels_`/`.coef_` ou passage direct du résultat à une fonction de chart SeraPlot.

</div>
