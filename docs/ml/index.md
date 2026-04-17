# Machine Learning

<div class="lang-en">

SeraPlot provides a complete scikit-learn-compatible ML framework written in Rust with Python bindings. All models follow the same `fit` / `predict` / `score` API.

> **All models are faster than scikit-learn** on equivalent tasks, with 1.3× to 55× speedups depending on the algorithm.

---

## Quick Start

```python
import seraplot as sp
import numpy as np

X_train, X_test, y_train, y_test = sp.train_test_split(X, y, test_size=0.2)

model = sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1)
model.fit(X_train, y_train)

print(f"Accuracy: {model.score(X_test, y_test):.4f}")
print(f"Classes: {model.classes_}")
proba = model.predict_proba(X_test)
```

---

## Model Index

### Supervised — Linear Models

| Class | Task | Description |
|-------|------|-------------|
| [`LinearRegression`](linear-regression.md) | Regression | Ordinary least squares |
| [`Ridge`](ridge.md) | Regression | L2-regularized OLS (Cholesky solver) |
| [`RidgeClassifier`](ridge.md) | Classification | Ridge regression rounded to nearest class |
| [`Lasso`](lasso.md) | Regression | L1-regularized (coordinate descent) |
| [`ElasticNet`](elastic-net.md) | Regression | L1 + L2 combined (coordinate descent) |
| [`LogisticRegression`](logistic-regression.md) | Classification | Newton-Raphson with full joint Hessian + line search |
| [`SGDClassifier`](sgd.md) | Classification | Stochastic gradient descent (hinge / log / huber) |
| [`SGDRegressor`](sgd.md) | Regression | Stochastic gradient descent (squared loss) |

### Supervised — Tree-Based

| Class | Task | Description |
|-------|------|-------------|
| [`DecisionTreeClassifier`](decision-tree.md) | Classification | CART with Gini or Entropy criterion |
| [`DecisionTreeRegressor`](decision-tree.md) | Regression | CART with MSE criterion |
| [`RandomForestClassifier`](random-forest.md) | Classification | Bagged trees with feature subsampling |
| [`RandomForestRegressor`](random-forest.md) | Regression | Bagged trees with feature subsampling |
| [`GradientBoostingClassifier`](gradient-boosting.md) | Classification | Softmax boosting with Newton-Raphson leaf values |
| [`GradientBoostingRegressor`](gradient-boosting.md) | Regression | Residual boosting with shrinkage |
| [`AdaBoostClassifier`](adaboost.md) | Classification | SAMME.R with Laplace-smoothed probabilities |
| [`AdaBoostRegressor`](adaboost.md) | Regression | Weighted median AdaBoost.R2 |

### Supervised — Neighbors

| Class | Task | Description |
|-------|------|-------------|
| [`KNeighborsClassifier`](knn.md) | Classification | Brute-force KNN with thread-local buffers |
| [`KNeighborsRegressor`](knn.md) | Regression | KNN with uniform or distance weighting |
| [`NearestCentroid`](knn.md) | Classification | Classify by nearest class centroid |

### Supervised — Naive Bayes

| Class | Task | Description |
|-------|------|-------------|
| [`GaussianNB`](naive-bayes.md) | Classification | Gaussian likelihood per feature |
| [`MultinomialNB`](naive-bayes.md) | Classification | Count/frequency features |
| [`BernoulliNB`](naive-bayes.md) | Classification | Binary features with binarization threshold |

### Supervised — SVM

| Class | Task | Description |
|-------|------|-------------|
| [`LinearSVC`](svm.md) | Classification | Dual coordinate descent hinge loss |
| [`LinearSVR`](svm.md) | Regression | Epsilon-insensitive loss |

### Unsupervised — Clustering

| Class | Task | Description |
|-------|------|-------------|
| [`KMeans`](kmeans.md) | Clustering | Lloyd's algorithm with k-means++ init |
| [`DBSCAN`](dbscan.md) | Clustering | Density-based spatial clustering |

### Preprocessing

| Class | Description |
|-------|-------------|
| [`StandardScaler`](preprocessing.md) | Zero mean, unit variance |
| [`MinMaxScaler`](preprocessing.md) | Scale to [min, max] range |
| [`RobustScaler`](preprocessing.md) | Median/IQR scaling (outlier-robust) |
| [`MaxAbsScaler`](preprocessing.md) | Scale by max absolute value |
| [`Normalizer`](preprocessing.md) | Row-wise L1/L2/Max normalization |

### Decomposition

| Class | Description |
|-------|-------------|
| [`PCA`](decomposition.md) | Principal Component Analysis |
| [`TruncatedSVD`](decomposition.md) | Truncated SVD (no centering) |

### Evaluation

| Function | Description |
|----------|-------------|
| [`accuracy_score`](metrics.md) | Classification accuracy |
| [`mean_squared_error`](metrics.md) | MSE for regression |
| [`mean_absolute_error`](metrics.md) | MAE for regression |
| [`r2_score`](metrics.md) | Coefficient of determination |
| [`classification_report`](metrics.md) | Per-class precision/recall/f1 |
| [`train_test_split`](train-test-split.md) | Stratified train/test split |

---

## Common API

All supervised models implement:

```python
model.fit(X, y)                 # Train on data
model.predict(X) -> list        # Predict labels/values
model.score(X, y) -> float      # Accuracy (clf) or R² (reg)
```

Classifiers additionally provide:

```python
model.predict_proba(X) -> ndarray   # Class probabilities (n, n_classes)
model.classes_ -> list[int]          # Unique sorted class labels
```

Linear models expose:

```python
model.coef_ -> list[float] | ndarray
model.intercept_ -> float | ndarray
```

---

## Benchmarks vs scikit-learn

| Model | Speedup | Notes |
|-------|---------|-------|
| GradientBoosting | **55×** | Newton-Raphson leaf values |
| RandomForest | **4–14×** | Rayon parallel tree building |
| AdaBoost | **6.7×** | SAMME.R with Laplace smoothing |
| DecisionTree | **6×** | Optimized column-major splitting |
| GaussianNB | **4.5×** | SIMD-friendly log-likelihood |
| LinearSVC | **3.3×** | Dual coordinate descent |
| KNN | **1.3×** | Thread-local zero-alloc buffers |
| LogisticRegression | **1.2×** | Full joint Hessian Newton |
| Pipeline (10 classes) | **8.3×** | Digits dataset end-to-end |

</div>

<div class="lang-fr">

SeraPlot fournit un framework ML complet, compatible scikit-learn, écrit en Rust avec des liaisons Python. Tous les modèles respectent la même API `fit` / `predict` / `score`.

> **Tous les modèles sont plus rapides que scikit-learn** sur des tâches équivalentes, avec des accélérations de 1,3× à 55× selon l'algorithme.

---

## Démarrage rapide

```python
import seraplot as sp
import numpy as np

X_train, X_test, y_train, y_test = sp.train_test_split(X, y, test_size=0.2)

model = sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1)
model.fit(X_train, y_train)
print(f"Précision : {model.score(X_test, y_test):.4f}")
```

---

## API commune

Tous les modèles supervisés implémentent :

```python
model.fit(X, y)              # Entraîner sur les données
model.predict(X) -> list     # Prédire les étiquettes/valeurs
model.score(X, y) -> float   # Précision (clf) ou R² (rég)
```

---

## Performances vs scikit-learn

| Modèle | Accélération | Notes |
|-------|-------------|-------|
| GradientBoosting | **55×** | Valeurs feuilles Newton-Raphson |
| RandomForest | **4–14×** | Construction d'arbres parallèle |
| AdaBoost | **6,7×** | SAMME.R avec lissage de Laplace |
| DecisionTree | **6×** | Division colonne-majeure optimisée |
| GaussianNB | **4,5×** | Log-vraisemblance optimisée SIMD |
| LinearSVC | **3,3×** | Descente de coordonnées duale |
| KNN | **1,3×** | Tampons thread-local sans allocation |

</div>

