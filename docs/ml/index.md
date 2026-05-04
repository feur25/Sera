# Machine Learning

<div class="lang-en">

<div class="ml-hero">
  <h1>SeraPlot ML</h1>
  <p class="ml-hero-sub">A scikit-learn-compatible machine learning toolkit, written in Rust with native Python bindings. Same API — built for speed.</p>
  <div class="ml-hero-stats">
    <div class="ml-stat"><div class="ml-stat-val">35+</div><div class="ml-stat-lbl">Estimators &amp; metrics</div></div>
    <div class="ml-stat"><div class="ml-stat-val">×60</div><div class="ml-stat-lbl">Top fit speedup</div></div>
    <div class="ml-stat"><div class="ml-stat-val">0.97</div><div class="ml-stat-lbl">Mean parity vs sklearn</div></div>
    <div class="ml-stat"><div class="ml-stat-val">100%</div><div class="ml-stat-lbl">Pure Rust core</div></div>
  </div>
</div>

<div class="ml-callout">
  <strong>Familiar API.</strong> Every estimator implements <code>fit</code> / <code>predict</code> / <code>score</code>. Drop-in for most scikit-learn pipelines — no relearning.
</div>

## Quick Start

```python
import seraplot as sp

X_train, X_test, y_train, y_test = sp.train_test_split(X, y, test_size=0.2)

model = sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1)
model.fit(X_train, y_train)

print(f"Accuracy: {model.score(X_test, y_test):.4f}")
print(f"Classes:  {model.classes_}")
proba = model.predict_proba(X_test)
```

---

## Browse by Category

<div class="ml-cat-grid">

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Linear models</strong><span class="ml-badge">8</span></div>
  <ul>
    <li><a href="linear-regression.html">LinearRegression</a></li>
    <li><a href="ridge.html">Ridge / RidgeClassifier</a></li>
    <li><a href="lasso.html">Lasso</a></li>
    <li><a href="elastic-net.html">ElasticNet</a></li>
    <li><a href="logistic-regression.html">LogisticRegression</a></li>
    <li><a href="sgd.html">SGDClassifier / SGDRegressor</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Trees &amp; ensembles</strong><span class="ml-badge">8</span></div>
  <ul>
    <li><a href="decision-tree.html">DecisionTreeClassifier / Regressor</a></li>
    <li><a href="random-forest.html">RandomForestClassifier / Regressor</a></li>
    <li><a href="gradient-boosting.html">GradientBoostingClassifier / Regressor</a></li>
    <li><a href="adaboost.html">AdaBoostClassifier / Regressor</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Neighbors</strong><span class="ml-badge">3</span></div>
  <ul>
    <li><a href="knn.html">KNeighborsClassifier / Regressor</a></li>
    <li><a href="knn.html">NearestCentroid</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Naive Bayes</strong><span class="ml-badge">3</span></div>
  <ul>
    <li><a href="naive-bayes.html">GaussianNB</a></li>
    <li><a href="naive-bayes.html">MultinomialNB</a></li>
    <li><a href="naive-bayes.html">BernoulliNB</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>SVM (linear)</strong><span class="ml-badge">2</span></div>
  <ul>
    <li><a href="svm.html">LinearSVC</a></li>
    <li><a href="svm.html">LinearSVR</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Clustering</strong><span class="ml-badge">2</span></div>
  <ul>
    <li><a href="kmeans.html">KMeans</a></li>
    <li><a href="dbscan.html">DBSCAN</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Preprocessing</strong><span class="ml-badge">5</span></div>
  <ul>
    <li><a href="preprocessing.html">StandardScaler</a></li>
    <li><a href="preprocessing.html">MinMaxScaler</a></li>
    <li><a href="preprocessing.html">RobustScaler</a></li>
    <li><a href="preprocessing.html">MaxAbsScaler</a></li>
    <li><a href="preprocessing.html">Normalizer</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Decomposition</strong><span class="ml-badge">2</span></div>
  <ul>
    <li><a href="decomposition.html">PCA</a></li>
    <li><a href="decomposition.html">TruncatedSVD</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Model selection</strong><span class="ml-badge">5</span></div>
  <ul>
    <li><a href="train-test-split.html">train_test_split</a></li>
    <li><a href="grid-search.html">GridSearchCV</a></li>
    <li><a href="grid-search.html">RandomizedSearchCV</a></li>
    <li><a href="grid-search.html">HalvingGridSearchCV</a></li>
    <li><a href="grid-search.html">HalvingRandomSearchCV</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Metrics</strong><span class="ml-badge">40+</span></div>
  <ul>
    <li><a href="metrics-classification.html">Classification metrics</a></li>
    <li><a href="metrics-regression.html">Regression metrics</a></li>
    <li><a href="metrics-clustering.html">Clustering metrics</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Anomaly</strong><span class="ml-badge">1</span></div>
  <ul>
    <li><a href="isolation-forest.html">IsolationForest</a></li>
  </ul>
</div>

</div>

---

## Common API

```python
model.fit(X, y)                    # Train on data
model.predict(X)                   # Predict labels / values
model.score(X, y)                  # Accuracy (clf) or R² (reg)

model.predict_proba(X)             # Class probabilities (classifiers)
model.classes_                     # Sorted class labels (classifiers)

model.coef_, model.intercept_      # Linear models
```

---

## Benchmarks vs scikit-learn

Measured on `n=5000`, `p=20`, single machine (Windows, Python 3.11, scikit-learn 1.8.0, seraplot 2.4.32). Parity is the per-prediction agreement on a held-out test set.

### Trees &amp; Ensembles — where SeraPlot shines

<table class="ml-bench">
<thead><tr><th>Estimator</th><th>Fit speedup</th><th>Predict</th><th>Parity</th></tr></thead>
<tbody>
<tr><td><code>GradientBoostingClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:140px"></span><span class="ml-num">×60.1</span></td><td>×0.51</td><td><span class="ml-parity">0.954</span></td></tr>
<tr><td><code>AdaBoostClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:64px"></span><span class="ml-num">×27.4</span></td><td>×26.0</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>GradientBoostingRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:63px"></span><span class="ml-num">×27.0</span></td><td>×2.4</td><td><span class="ml-parity">0.987</span></td></tr>
<tr><td><code>RandomForestRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:36px"></span><span class="ml-num">×15.4</span></td><td>×13.5</td><td><span class="ml-parity ml-parity-mid">0.907</span></td></tr>
<tr><td><code>AdaBoostRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:25px"></span><span class="ml-num">×10.6</span></td><td>×5.4</td><td><span class="ml-parity ml-parity-mid">0.937</span></td></tr>
<tr><td><code>RandomForestClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:19px"></span><span class="ml-num">×8.2</span></td><td>×13.6</td><td><span class="ml-parity ml-parity-mid">0.934</span></td></tr>
<tr><td><code>DecisionTreeRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:16px"></span><span class="ml-num">×6.7</span></td><td>×3.3</td><td><span class="ml-parity ml-parity-mid">0.900</span></td></tr>
<tr><td><code>DecisionTreeClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:15px"></span><span class="ml-num">×6.5</span></td><td>×4.4</td><td><span class="ml-parity ml-parity-mid">0.886</span></td></tr>
</tbody>
</table>

### Linear models — close to LAPACK-bound

<table class="ml-bench">
<thead><tr><th>Estimator</th><th>Fit speedup</th><th>Predict</th><th>Parity</th></tr></thead>
<tbody>
<tr><td><code>SGDClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:38px"></span><span class="ml-num">×6.3</span></td><td>×1.4</td><td><span class="ml-parity ml-parity-low">0.715</span></td></tr>
<tr><td><code>RidgeClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:14px"></span><span class="ml-num">×1.6</span></td><td>×1.1</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>SGDRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:13px"></span><span class="ml-num">×1.5</span></td><td>×0.8</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>Lasso</code></td><td class="ml-bar"><span style="width:9px"></span><span class="ml-num">×0.95</span></td><td>×1.2</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>LinearSVC</code></td><td class="ml-bar ml-bar-slow"><span style="width:7px"></span><span class="ml-num">×0.70</span></td><td>×0.8</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>Ridge</code></td><td class="ml-bar ml-bar-slow"><span style="width:6px"></span><span class="ml-num">×0.63</span></td><td>×1.1</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>ElasticNet</code></td><td class="ml-bar ml-bar-slow"><span style="width:4px"></span><span class="ml-num">×0.41</span></td><td>×1.0</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>LogisticRegression</code></td><td class="ml-bar ml-bar-slow"><span style="width:4px"></span><span class="ml-num">×0.35</span></td><td>×1.9</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>LinearRegression</code></td><td class="ml-bar ml-bar-slow"><span style="width:3px"></span><span class="ml-num">×0.27</span></td><td>×0.75</td><td><span class="ml-parity">1.000</span></td></tr>
</tbody>
</table>

### Naive Bayes, neighbors, decomposition, metrics

<table class="ml-bench">
<thead><tr><th>Estimator</th><th>Fit speedup</th><th>Predict</th><th>Parity</th></tr></thead>
<tbody>
<tr><td><code>GaussianNB</code></td><td>×1.1</td><td class="ml-bar ml-bar-fast"><span style="width:34px"></span><span class="ml-num">×14.5</span></td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>BernoulliNB</code></td><td>×1.95</td><td class="ml-bar ml-bar-fast"><span style="width:16px"></span><span class="ml-num">×6.9</span></td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>MultinomialNB</code></td><td>×0.57</td><td class="ml-bar ml-bar-fast"><span style="width:7px"></span><span class="ml-num">×2.8</span></td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>NearestCentroid</code></td><td>×1.3</td><td class="ml-bar ml-bar-fast"><span style="width:8px"></span><span class="ml-num">×3.6</span></td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>KNeighborsClassifier</code></td><td>×0.26</td><td class="ml-bar ml-bar-fast"><span style="width:4px"></span><span class="ml-num">×1.5</span></td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>RobustScaler</code></td><td>×1.9</td><td class="ml-bar ml-bar-fast"><span style="width:11px"></span><span class="ml-num">×4.3</span></td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>TruncatedSVD</code></td><td class="ml-bar ml-bar-fast"><span style="width:10px"></span><span class="ml-num">×4.1</span></td><td>×0.74</td><td><span class="ml-parity ml-parity-low">0.745</span></td></tr>
<tr><td><code>PCA</code></td><td>×0.41</td><td>×0.75</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>f1_score</code></td><td class="ml-bar ml-bar-fast"><span style="width:23px"></span><span class="ml-num">×9.3</span></td><td>—</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>r2_score</code></td><td class="ml-bar ml-bar-fast"><span style="width:11px"></span><span class="ml-num">×4.6</span></td><td>—</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>train_test_split</code></td><td class="ml-bar ml-bar-fast"><span style="width:11px"></span><span class="ml-num">×4.5</span></td><td>—</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>mean_squared_error</code></td><td class="ml-bar ml-bar-fast"><span style="width:10px"></span><span class="ml-num">×4.3</span></td><td>—</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>accuracy_score</code></td><td class="ml-bar ml-bar-fast"><span style="width:7px"></span><span class="ml-num">×2.8</span></td><td>—</td><td><span class="ml-parity">1.000</span></td></tr>
</tbody>
</table>

<div class="ml-callout">
  <strong>Parity notes.</strong> Tree-based models reach ~0.90 because tie-breaking on equal-gain splits differs across implementations. <code>SGDClassifier</code> diverges due to RNG / iteration ordering. <code>TruncatedSVD</code> falls below 1.0 on noise-only data because singular vectors are not uniquely defined when singular values are nearly equal.
</div>

</div>

<div class="lang-fr">

<div class="ml-hero">
  <h1>SeraPlot ML</h1>
  <p class="ml-hero-sub">Une boîte à outils ML compatible scikit-learn, écrite en Rust avec des liaisons Python natives. Même API — pensée pour la vitesse.</p>
  <div class="ml-hero-stats">
    <div class="ml-stat"><div class="ml-stat-val">35+</div><div class="ml-stat-lbl">Estimateurs &amp; métriques</div></div>
    <div class="ml-stat"><div class="ml-stat-val">×60</div><div class="ml-stat-lbl">Accélération max au fit</div></div>
    <div class="ml-stat"><div class="ml-stat-val">0,97</div><div class="ml-stat-lbl">Parité moyenne vs sklearn</div></div>
    <div class="ml-stat"><div class="ml-stat-val">100%</div><div class="ml-stat-lbl">Cœur 100% Rust</div></div>
  </div>
</div>

<div class="ml-callout">
  <strong>API familière.</strong> Chaque estimateur implémente <code>fit</code> / <code>predict</code> / <code>score</code>. Compatible avec la plupart des pipelines scikit-learn — aucun ré-apprentissage.
</div>

## Démarrage rapide

```python
import seraplot as sp

X_train, X_test, y_train, y_test = sp.train_test_split(X, y, test_size=0.2)

model = sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1)
model.fit(X_train, y_train)

print(f"Précision : {model.score(X_test, y_test):.4f}")
```

---

## Parcourir par catégorie

<div class="ml-cat-grid">

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Modèles linéaires</strong><span class="ml-badge">8</span></div>
  <ul>
    <li><a href="linear-regression.html">LinearRegression</a></li>
    <li><a href="ridge.html">Ridge / RidgeClassifier</a></li>
    <li><a href="lasso.html">Lasso</a></li>
    <li><a href="elastic-net.html">ElasticNet</a></li>
    <li><a href="logistic-regression.html">LogisticRegression</a></li>
    <li><a href="sgd.html">SGDClassifier / SGDRegressor</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Arbres &amp; ensembles</strong><span class="ml-badge">8</span></div>
  <ul>
    <li><a href="decision-tree.html">DecisionTree (Classifier / Regressor)</a></li>
    <li><a href="random-forest.html">RandomForest (Classifier / Regressor)</a></li>
    <li><a href="gradient-boosting.html">GradientBoosting (Classifier / Regressor)</a></li>
    <li><a href="adaboost.html">AdaBoost (Classifier / Regressor)</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Voisins</strong><span class="ml-badge">3</span></div>
  <ul>
    <li><a href="knn.html">KNeighbors (Classifier / Regressor)</a></li>
    <li><a href="knn.html">NearestCentroid</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Naive Bayes</strong><span class="ml-badge">3</span></div>
  <ul>
    <li><a href="naive-bayes.html">GaussianNB</a></li>
    <li><a href="naive-bayes.html">MultinomialNB</a></li>
    <li><a href="naive-bayes.html">BernoulliNB</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>SVM (linéaire)</strong><span class="ml-badge">2</span></div>
  <ul>
    <li><a href="svm.html">LinearSVC</a></li>
    <li><a href="svm.html">LinearSVR</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Clustering</strong><span class="ml-badge">2</span></div>
  <ul>
    <li><a href="kmeans.html">KMeans</a></li>
    <li><a href="dbscan.html">DBSCAN</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Prétraitement</strong><span class="ml-badge">5</span></div>
  <ul>
    <li><a href="preprocessing.html">StandardScaler</a></li>
    <li><a href="preprocessing.html">MinMaxScaler</a></li>
    <li><a href="preprocessing.html">RobustScaler</a></li>
    <li><a href="preprocessing.html">MaxAbsScaler</a></li>
    <li><a href="preprocessing.html">Normalizer</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Décomposition</strong><span class="ml-badge">2</span></div>
  <ul>
    <li><a href="decomposition.html">PCA</a></li>
    <li><a href="decomposition.html">TruncatedSVD</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Sélection de modèle</strong><span class="ml-badge">5</span></div>
  <ul>
    <li><a href="train-test-split.html">train_test_split</a></li>
    <li><a href="grid-search.html">GridSearchCV</a></li>
    <li><a href="grid-search.html">RandomizedSearchCV</a></li>
    <li><a href="grid-search.html">HalvingGridSearchCV</a></li>
    <li><a href="grid-search.html">HalvingRandomSearchCV</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Métriques</strong><span class="ml-badge">40+</span></div>
  <ul>
    <li><a href="metrics-classification.html">Métriques de classification</a></li>
    <li><a href="metrics-regression.html">Métriques de régression</a></li>
    <li><a href="metrics-clustering.html">Métriques de clustering</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Anomalie</strong><span class="ml-badge">1</span></div>
  <ul>
    <li><a href="isolation-forest.html">IsolationForest</a></li>
  </ul>
</div>

</div>

---

## API commune

```python
model.fit(X, y)                    # Entraînement
model.predict(X)                   # Prédiction
model.score(X, y)                  # Précision (clf) ou R² (rég)

model.predict_proba(X)             # Probabilités de classe (classifieurs)
model.classes_                     # Classes triées (classifieurs)

model.coef_, model.intercept_      # Modèles linéaires
```

---

## Benchmarks vs scikit-learn

Mesuré sur `n=5000`, `p=20`, machine unique (Windows, Python 3.11, scikit-learn 1.8.0, seraplot 2.4.32). La parité est l'accord par prédiction sur un jeu de test.

### Arbres &amp; ensembles — là où SeraPlot brille

<table class="ml-bench">
<thead><tr><th>Estimateur</th><th>Accélération fit</th><th>Predict</th><th>Parité</th></tr></thead>
<tbody>
<tr><td><code>GradientBoostingClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:140px"></span><span class="ml-num">×60,1</span></td><td>×0,51</td><td><span class="ml-parity">0,954</span></td></tr>
<tr><td><code>AdaBoostClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:64px"></span><span class="ml-num">×27,4</span></td><td>×26,0</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>GradientBoostingRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:63px"></span><span class="ml-num">×27,0</span></td><td>×2,4</td><td><span class="ml-parity">0,987</span></td></tr>
<tr><td><code>RandomForestRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:36px"></span><span class="ml-num">×15,4</span></td><td>×13,5</td><td><span class="ml-parity ml-parity-mid">0,907</span></td></tr>
<tr><td><code>AdaBoostRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:25px"></span><span class="ml-num">×10,6</span></td><td>×5,4</td><td><span class="ml-parity ml-parity-mid">0,937</span></td></tr>
<tr><td><code>RandomForestClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:19px"></span><span class="ml-num">×8,2</span></td><td>×13,6</td><td><span class="ml-parity ml-parity-mid">0,934</span></td></tr>
<tr><td><code>DecisionTreeRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:16px"></span><span class="ml-num">×6,7</span></td><td>×3,3</td><td><span class="ml-parity ml-parity-mid">0,900</span></td></tr>
<tr><td><code>DecisionTreeClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:15px"></span><span class="ml-num">×6,5</span></td><td>×4,4</td><td><span class="ml-parity ml-parity-mid">0,886</span></td></tr>
</tbody>
</table>

### Modèles linéaires — proches de la limite LAPACK

<table class="ml-bench">
<thead><tr><th>Estimateur</th><th>Accélération fit</th><th>Predict</th><th>Parité</th></tr></thead>
<tbody>
<tr><td><code>SGDClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:38px"></span><span class="ml-num">×6,3</span></td><td>×1,4</td><td><span class="ml-parity ml-parity-low">0,715</span></td></tr>
<tr><td><code>RidgeClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:14px"></span><span class="ml-num">×1,6</span></td><td>×1,1</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>SGDRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:13px"></span><span class="ml-num">×1,5</span></td><td>×0,8</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>Lasso</code></td><td class="ml-bar"><span style="width:9px"></span><span class="ml-num">×0,95</span></td><td>×1,2</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>LinearSVC</code></td><td class="ml-bar ml-bar-slow"><span style="width:7px"></span><span class="ml-num">×0,70</span></td><td>×0,8</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>Ridge</code></td><td class="ml-bar ml-bar-slow"><span style="width:6px"></span><span class="ml-num">×0,63</span></td><td>×1,1</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>ElasticNet</code></td><td class="ml-bar ml-bar-slow"><span style="width:4px"></span><span class="ml-num">×0,41</span></td><td>×1,0</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>LogisticRegression</code></td><td class="ml-bar ml-bar-slow"><span style="width:4px"></span><span class="ml-num">×0,35</span></td><td>×1,9</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>LinearRegression</code></td><td class="ml-bar ml-bar-slow"><span style="width:3px"></span><span class="ml-num">×0,27</span></td><td>×0,75</td><td><span class="ml-parity">1,000</span></td></tr>
</tbody>
</table>

<div class="ml-callout">
  <strong>Notes sur la parité.</strong> Les modèles à base d'arbres atteignent ~0,90 car le tie-breaking sur les splits d'égal gain diffère selon l'implémentation. <code>SGDClassifier</code> diverge à cause de l'ordre RNG / itérations. <code>TruncatedSVD</code> reste sous 1,0 sur des données purement aléatoires car les vecteurs singuliers ne sont pas uniques quand les valeurs singulières sont quasi-égales.
</div>

</div>
