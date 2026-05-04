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
  <div class="ml-cat-h"><strong>Preprocessing</strong><span class="ml-badge">12</span></div>
  <ul>
    <li><a href="preprocessing.html">StandardScaler</a></li>
    <li><a href="preprocessing.html">MinMaxScaler</a></li>
    <li><a href="preprocessing.html">RobustScaler</a></li>
    <li><a href="preprocessing.html">MaxAbsScaler</a></li>
    <li><a href="preprocessing.html">Normalizer</a></li>
    <li><a href="preprocessing-advanced.html">SimpleImputer / PolynomialFeatures</a></li>
    <li><a href="preprocessing-advanced.html">KBinsDiscretizer / PowerTransformer</a></li>
    <li><a href="preprocessing-advanced.html">OneHotEncoder / OrdinalEncoder</a></li>
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
  <div class="ml-cat-h"><strong>Model selection</strong><span class="ml-badge">8</span></div>
  <ul>
    <li><a href="train-test-split.html">train_test_split / StratifiedKFold</a></li>
    <li><a href="cv-splitters.html">CV Splitters (kfold / stratified / group)</a></li>
    <li><a href="grid-search.html">GridSearchCV</a></li>
    <li><a href="grid-search.html">RandomizedSearchCV</a></li>
    <li><a href="grid-search.html">HalvingGridSearchCV</a></li>
    <li><a href="grid-search.html">HalvingRandomSearchCV</a></li>
    <li><a href="permutation-importance.html">Permutation Importance</a></li>
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

Measured on `n=5000`, `p=20`, single machine (Windows, Python 3.11, scikit-learn 1.8.0, seraplot 2.4.33). Parity is the per-prediction agreement on a held-out test set.

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
  <strong>Reading the parity badge.</strong>
  <ul>
    <li><span class="ml-parity">≥ 0.95</span> — drop-in replacement. Predictions agree with scikit-learn within numerical tolerance.</li>
    <li><span class="ml-parity ml-parity-mid">0.85 – 0.95</span> — same statistical accuracy, but tie-breaking on equal-gain tree splits, RNG seeds or iteration ordering produce a few different individual predictions. Score on a test set is essentially identical.</li>
    <li><span class="ml-parity ml-parity-low">&lt; 0.85</span> — algorithmic divergence on noisy or degenerate inputs. Two cases: (1) <code>SGDClassifier</code> uses a different mini-batch shuffle / learning-rate schedule, so individual predictions can differ even when accuracy matches; (2) <code>TruncatedSVD</code> on data with near-equal singular values returns a different (but mathematically valid) basis — sign and rotation of singular vectors are not unique.</li>
  </ul>
  In all cases the test-set <em>score</em> (accuracy / R²) matches scikit-learn to two decimals.
</div>

## Big-data benchmarks (real OpenFoodFacts data)

Dataset: 504,376 products from [openfoodfacts.org](https://en.openfoodfacts.org/), 9 numerical features (energy, fat, sugars, …), nutri-score grade as 5-class target. Times are wall-clock per `fit` on a single Windows laptop.

<table class="ml-bench">
<thead><tr><th>Estimator</th><th>n=10k</th><th>n=50k</th><th>n=200k</th><th>n=500k</th></tr></thead>
<tbody>
<tr><td><code>GradientBoostingClassifier</code></td><td><span class="ml-num">×38.3</span></td><td><span class="ml-num">×46.0</span></td><td>—</td><td>—</td></tr>
<tr><td><code>GradientBoostingRegressor</code></td><td><span class="ml-num">×15.6</span></td><td><span class="ml-num">×19.7</span></td><td>—</td><td>—</td></tr>
<tr><td><code>AdaBoostClassifier</code></td><td><span class="ml-num">×17.9</span></td><td><span class="ml-num">×20.2</span></td><td>—</td><td>—</td></tr>
<tr><td><code>RandomForestRegressor</code></td><td><span class="ml-num">×10.9</span></td><td><span class="ml-num">×11.7</span></td><td><span class="ml-num">×11.2</span></td><td>—</td></tr>
<tr><td><code>RandomForestClassifier</code></td><td><span class="ml-num">×6.95</span></td><td><span class="ml-num">×6.66</span></td><td><span class="ml-num">×7.95</span></td><td>—</td></tr>
<tr><td><code>DecisionTreeClassifier</code></td><td><span class="ml-num">×4.64</span></td><td><span class="ml-num">×8.93</span></td><td><span class="ml-num">×15.7</span></td><td>—</td></tr>
<tr><td><code>SGDClassifier</code></td><td><span class="ml-num">×6.73</span></td><td><span class="ml-num">×11.0</span></td><td><span class="ml-num">×14.8</span></td><td><span class="ml-num">×15.0</span></td></tr>
<tr><td><code>NearestCentroid</code></td><td><span class="ml-num">×2.21</span></td><td><span class="ml-num">×4.22</span></td><td><span class="ml-num">×6.49</span></td><td><span class="ml-num">×6.77</span></td></tr>
<tr><td><code>GaussianNB</code></td><td><span class="ml-num">×1.77</span></td><td><span class="ml-num">×3.78</span></td><td><span class="ml-num">×6.05</span></td><td><span class="ml-num">×6.66</span></td></tr>
<tr><td><code>BernoulliNB</code></td><td><span class="ml-num">×1.89</span></td><td><span class="ml-num">×3.88</span></td><td><span class="ml-num">×5.71</span></td><td><span class="ml-num">×5.96</span></td></tr>
<tr><td><code>RobustScaler</code></td><td><span class="ml-num">×1.83</span></td><td><span class="ml-num">×3.76</span></td><td><span class="ml-num">×5.21</span></td><td><span class="ml-num">×6.00</span></td></tr>
<tr><td><code>StandardScaler</code></td><td><span class="ml-num">×1.33</span></td><td><span class="ml-num">×2.91</span></td><td><span class="ml-num">×4.72</span></td><td><span class="ml-num">×7.41</span></td></tr>
<tr><td><code>KMeans</code></td><td>—</td><td><span class="ml-num">×3.90</span></td><td><span class="ml-num">×5.38</span></td><td><span class="ml-num">×5.34</span></td></tr>
<tr><td><code>LinearRegression</code></td><td><span class="ml-num">×0.00</span></td><td><span class="ml-num">×2.05</span></td><td><span class="ml-num">×3.54</span></td><td><span class="ml-num">×5.03</span></td></tr>
<tr><td><code>RidgeClassifier</code></td><td><span class="ml-num">×2.90</span></td><td><span class="ml-num">×2.85</span></td><td><span class="ml-num">×3.28</span></td><td><span class="ml-num">×2.97</span></td></tr>
<tr><td><code>Ridge</code></td><td><span class="ml-num">×1.15</span></td><td><span class="ml-num">×1.53</span></td><td><span class="ml-num">×2.02</span></td><td><span class="ml-num">×2.35</span></td></tr>
<tr><td><code>accuracy_score</code></td><td>—</td><td>—</td><td><span class="ml-num">×15.6</span></td><td>—</td></tr>
<tr><td><code>f1_score</code></td><td>—</td><td>—</td><td><span class="ml-num">×5.41</span></td><td>—</td></tr>
<tr><td><code>r2_score</code></td><td>—</td><td>—</td><td>—</td><td><span class="ml-num">×2.86</span></td></tr>
<tr><td><code>train_test_split</code></td><td>—</td><td>—</td><td>—</td><td><span class="ml-num">×1.64</span></td></tr>
</tbody>
</table>

<div class="ml-callout">
  <strong>How SeraPlot scales.</strong> The whole core is written in Rust on top of <a href="https://github.com/rayon-rs/rayon">rayon</a> — work-stealing thread pools that automatically span every CPU core. Hot inner loops (column dot-products, residual updates, tree split scans, distance reductions) are 4-way unrolled and laid out cache-friendly (column-major <code>X</code>, contiguous chunks). Coordinate descent in <code>Lasso</code> / <code>ElasticNet</code> switches to a parallel chunked reduction once n ≥ 50k; tree ensembles parallelize across estimators; metrics and scalers operate directly on the NumPy buffer with zero copies.
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
  <div class="ml-cat-h"><strong>Prétraitement</strong><span class="ml-badge">12</span></div>
  <ul>
    <li><a href="preprocessing.html">StandardScaler</a></li>
    <li><a href="preprocessing.html">MinMaxScaler</a></li>
    <li><a href="preprocessing.html">RobustScaler</a></li>
    <li><a href="preprocessing.html">MaxAbsScaler</a></li>
    <li><a href="preprocessing.html">Normalizer</a></li>
    <li><a href="preprocessing-advanced.html">SimpleImputer / PolynomialFeatures</a></li>
    <li><a href="preprocessing-advanced.html">KBinsDiscretizer / PowerTransformer</a></li>
    <li><a href="preprocessing-advanced.html">OneHotEncoder / OrdinalEncoder</a></li>
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
  <div class="ml-cat-h"><strong>Sélection de modèle</strong><span class="ml-badge">8</span></div>
  <ul>
    <li><a href="train-test-split.html">train_test_split / StratifiedKFold</a></li>
    <li><a href="cv-splitters.html">Découpeurs CV (kfold / stratified / group)</a></li>
    <li><a href="grid-search.html">GridSearchCV</a></li>
    <li><a href="grid-search.html">RandomizedSearchCV</a></li>
    <li><a href="grid-search.html">HalvingGridSearchCV</a></li>
    <li><a href="grid-search.html">HalvingRandomSearchCV</a></li>
    <li><a href="permutation-importance.html">Importance par permutation</a></li>
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

Mesuré sur `n=5000`, `p=20`, machine unique (Windows, Python 3.11, scikit-learn 1.8.0, seraplot 2.4.33). La parité est l'accord par prédiction sur un jeu de test.

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
  <strong>Lecture du badge de parité.</strong>
  <ul>
    <li><span class="ml-parity">≥ 0,95</span> — remplacement direct. Les prédictions correspondent à scikit-learn à la tolérance numérique près.</li>
    <li><span class="ml-parity ml-parity-mid">0,85 – 0,95</span> — même précision statistique, mais le tie-breaking sur les splits d'arbres à gain égal, les graines RNG ou l'ordre d'itération produisent quelques prédictions individuelles différentes. Le score sur un jeu de test est quasi identique.</li>
    <li><span class="ml-parity ml-parity-low">&lt; 0,85</span> — divergence algorithmique sur entrées bruitées ou dégénérées. Deux cas : (1) <code>SGDClassifier</code> utilise un mélange mini-batch / planning de taux d'apprentissage différents, donc les prédictions individuelles diffèrent même quand l'accuracy correspond ; (2) <code>TruncatedSVD</code> sur des données aux valeurs singulières quasi-égales renvoie une base différente (mais mathématiquement valide) — signe et rotation des vecteurs singuliers ne sont pas uniques.</li>
  </ul>
  Dans tous les cas, le <em>score</em> sur jeu de test (accuracy / R²) correspond à scikit-learn à deux décimales.
</div>

## Benchmarks gros volume (données réelles OpenFoodFacts)

Jeu de données : 504 376 produits depuis [openfoodfacts.org](https://fr.openfoodfacts.org/), 9 features numériques (énergie, lipides, sucres, …), nutri-score (5 classes) en cible. Temps `fit` mesurés sur un seul portable Windows.

<table class="ml-bench">
<thead><tr><th>Estimateur</th><th>n=10k</th><th>n=50k</th><th>n=200k</th><th>n=500k</th></tr></thead>
<tbody>
<tr><td><code>GradientBoostingClassifier</code></td><td><span class="ml-num">×38,3</span></td><td><span class="ml-num">×46,0</span></td><td>—</td><td>—</td></tr>
<tr><td><code>GradientBoostingRegressor</code></td><td><span class="ml-num">×15,6</span></td><td><span class="ml-num">×19,7</span></td><td>—</td><td>—</td></tr>
<tr><td><code>AdaBoostClassifier</code></td><td><span class="ml-num">×17,9</span></td><td><span class="ml-num">×20,2</span></td><td>—</td><td>—</td></tr>
<tr><td><code>RandomForestRegressor</code></td><td><span class="ml-num">×10,9</span></td><td><span class="ml-num">×11,7</span></td><td><span class="ml-num">×11,2</span></td><td>—</td></tr>
<tr><td><code>RandomForestClassifier</code></td><td><span class="ml-num">×6,95</span></td><td><span class="ml-num">×6,66</span></td><td><span class="ml-num">×7,95</span></td><td>—</td></tr>
<tr><td><code>DecisionTreeClassifier</code></td><td><span class="ml-num">×4,64</span></td><td><span class="ml-num">×8,93</span></td><td><span class="ml-num">×15,7</span></td><td>—</td></tr>
<tr><td><code>SGDClassifier</code></td><td><span class="ml-num">×6,73</span></td><td><span class="ml-num">×11,0</span></td><td><span class="ml-num">×14,8</span></td><td><span class="ml-num">×15,0</span></td></tr>
<tr><td><code>NearestCentroid</code></td><td><span class="ml-num">×2,21</span></td><td><span class="ml-num">×4,22</span></td><td><span class="ml-num">×6,49</span></td><td><span class="ml-num">×6,77</span></td></tr>
<tr><td><code>GaussianNB</code></td><td><span class="ml-num">×1,77</span></td><td><span class="ml-num">×3,78</span></td><td><span class="ml-num">×6,05</span></td><td><span class="ml-num">×6,66</span></td></tr>
<tr><td><code>BernoulliNB</code></td><td><span class="ml-num">×1,89</span></td><td><span class="ml-num">×3,88</span></td><td><span class="ml-num">×5,71</span></td><td><span class="ml-num">×5,96</span></td></tr>
<tr><td><code>RobustScaler</code></td><td><span class="ml-num">×1,83</span></td><td><span class="ml-num">×3,76</span></td><td><span class="ml-num">×5,21</span></td><td><span class="ml-num">×6,00</span></td></tr>
<tr><td><code>StandardScaler</code></td><td><span class="ml-num">×1,33</span></td><td><span class="ml-num">×2,91</span></td><td><span class="ml-num">×4,72</span></td><td><span class="ml-num">×7,41</span></td></tr>
<tr><td><code>KMeans</code></td><td>—</td><td><span class="ml-num">×3,90</span></td><td><span class="ml-num">×5,38</span></td><td><span class="ml-num">×5,34</span></td></tr>
<tr><td><code>LinearRegression</code></td><td><span class="ml-num">×0,00</span></td><td><span class="ml-num">×2,05</span></td><td><span class="ml-num">×3,54</span></td><td><span class="ml-num">×5,03</span></td></tr>
<tr><td><code>RidgeClassifier</code></td><td><span class="ml-num">×2,90</span></td><td><span class="ml-num">×2,85</span></td><td><span class="ml-num">×3,28</span></td><td><span class="ml-num">×2,97</span></td></tr>
<tr><td><code>Ridge</code></td><td><span class="ml-num">×1,15</span></td><td><span class="ml-num">×1,53</span></td><td><span class="ml-num">×2,02</span></td><td><span class="ml-num">×2,35</span></td></tr>
<tr><td><code>accuracy_score</code></td><td>—</td><td>—</td><td><span class="ml-num">×15,6</span></td><td>—</td></tr>
<tr><td><code>f1_score</code></td><td>—</td><td>—</td><td><span class="ml-num">×5,41</span></td><td>—</td></tr>
<tr><td><code>r2_score</code></td><td>—</td><td>—</td><td>—</td><td><span class="ml-num">×2,86</span></td></tr>
<tr><td><code>train_test_split</code></td><td>—</td><td>—</td><td>—</td><td><span class="ml-num">×1,64</span></td></tr>
</tbody>
</table>

<div class="ml-callout">
  <strong>Comment SeraPlot passe à l'échelle.</strong> Le cœur est écrit en Rust sur <a href="https://github.com/rayon-rs/rayon">rayon</a> — pools de threads en work-stealing qui exploitent automatiquement tous les cœurs CPU. Les boucles chaudes (produits scalaires de colonnes, mises à jour de résidus, scans de splits, réductions de distances) sont déroulées 4-à-1 et organisées en mémoire pour le cache (X column-major, chunks contigus). La descente par coordonnées de <code>Lasso</code> / <code>ElasticNet</code> bascule sur une réduction parallèle par chunks dès n ≥ 50k ; les ensembles d'arbres parallélisent les estimateurs ; les métriques et scalers travaillent directement sur le buffer NumPy sans copie.
</div>

</div>
