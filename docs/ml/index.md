# Machine Learning

<div class="lang-en">

<div class="ml-hero">
  <h1>SeraPlot ML</h1>
  <p class="ml-hero-sub">A scikit-learn-compatible machine learning toolkit, written 100% in Rust with native Python bindings via PyO3. Same API \u2014 built for speed.</p>
  <div class="ml-hero-stats">
    <div class="ml-stat"><div class="ml-stat-val">50+</div><div class="ml-stat-lbl">Estimators &amp; metrics</div></div>
    <div class="ml-stat"><div class="ml-stat-val">\u00d760</div><div class="ml-stat-lbl">Top fit speedup</div></div>
    <div class="ml-stat"><div class="ml-stat-val">0.97</div><div class="ml-stat-lbl">Mean parity vs sklearn</div></div>
    <div class="ml-stat"><div class="ml-stat-val">100%</div><div class="ml-stat-lbl">Pure Rust core</div></div>
  </div>
</div>

<div class="ml-callout">
  <strong>Familiar API.</strong> Every estimator implements <code>fit</code> / <code>predict</code> / <code>score</code> with <code>get_params</code> / <code>set_params</code>. Drop-in for most scikit-learn pipelines \u2014 no relearning.
</div>

## What's actually inside

Concrete, code-verified inventory of what ships with SeraPlot today:

- **Linear models (8).** \`LinearRegression\`, \`Ridge\`, \`RidgeClassifier\`, \`Lasso\`, \`ElasticNet\`, \`LogisticRegression\`, \`SGDClassifier\`, \`SGDRegressor\` \u2014 all with optional \`fit_intercept\`, sklearn-matching defaults.
- **Trees &amp; ensembles (8).** \`DecisionTreeClassifier/Regressor\`, \`RandomForestClassifier/Regressor\`, \`GradientBoostingClassifier/Regressor\`, \`AdaBoostClassifier/Regressor\` \u2014 every classifier exposes \`predict_proba\` and \`feature_importances_\`.
- **Neighbors (3).** \`KNeighborsClassifier\`, \`KNeighborsRegressor\` (with \`weights="uniform"|"distance"\`), \`NearestCentroid\`.
- **Naive Bayes (3).** \`GaussianNB\`, \`MultinomialNB\`, \`BernoulliNB\` \u2014 each with \`predict_proba\`.
- **Linear SVM (2).** \`LinearSVC\`, \`LinearSVR\` (squared hinge, hinge or epsilon-insensitive losses).
- **Clustering (2).** \`KMeans\` (full + mini-batch, \`n_init=10\` k-means++) and \`DBSCAN\`.
- **Preprocessing scalers (5).** \`StandardScaler\`, \`MinMaxScaler\`, \`RobustScaler\`, \`MaxAbsScaler\`, \`Normalizer\` \u2014 \`partial_fit\` / \`inverse_transform\` where applicable.
- **Advanced preprocessing (8).** \`SimpleImputer\`, \`PolynomialFeatures\`, \`KBinsDiscretizer\`, \`PowerTransformer\` (yeo-johnson / box-cox), \`QuantileTransformer\`, \`OneHotEncoder\`, \`OrdinalEncoder\`, \`LabelEncoder\`.
- **Decomposition (2).** \`PCA\` (\`svd_solver="auto"|"full"|"randomized"\`), \`TruncatedSVD\`.
- **Model selection (8).** \`train_test_split\`, \`StratifiedKFold\`, the \`ml_kfold_split\` JSON dispatcher (kfold / stratified / group), \`GridSearchCV\`, \`RandomizedSearchCV\`, \`HalvingGridSearchCV\`, \`HalvingRandomSearchCV\`, \`ml_permutation_importance\`.
- **Metrics (40+).** Classification, regression and clustering metrics, including \`accuracy_score\`, \`f1_score\`, \`mean_squared_error\`, \`r2_score\`, \`classification_report\`, \`confusion_matrix\`, silhouette, calinski-harabasz, davies-bouldin\u2026
- **Anomaly (1).** \`IsolationForest\` (JSON dispatcher).
- **Distributed.** \`WorkerPool\` for sharded scatter / all-reduce.
- **Export.** Native dispatchers for Power BI and Tableau model exports.

## Quick Start

\`\`\`python
import seraplot as sp

X_train, X_test, y_train, y_test = sp.train_test_split(X, y, test_size=0.2)

model = sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1)
model.fit(X_train, y_train)

print(f"Accuracy: {model.score(X_test, y_test):.4f}")
print(f"Classes:  {model.classes_}")
proba = model.predict_proba(X_test)
\`\`\`

## Common API

\`\`\`python
model.fit(X, y)                    # Train on data
model.predict(X)                   # Predict labels / values
model.score(X, y)                  # Accuracy (clf) or R\u00b2 (reg)

model.predict_proba(X)             # Class probabilities (classifiers)
model.classes_                     # Sorted class labels (classifiers)

model.coef_, model.intercept_      # Linear models
model.feature_importances_         # Tree-based models
\`\`\`

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
    <li><a href="knn.html#nearestcentroid">NearestCentroid</a></li>
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
    <li><a href="kmeans-class.html">KMeans</a></li>
    <li><a href="dbscan-class.html">DBSCAN</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Preprocessing</strong><span class="ml-badge">13</span></div>
  <ul>
    <li><a href="preprocessing.html">StandardScaler / MinMaxScaler</a></li>
    <li><a href="preprocessing.html">RobustScaler / MaxAbsScaler</a></li>
    <li><a href="preprocessing.html">Normalizer</a></li>
    <li><a href="preprocessing-advanced.html">SimpleImputer / PolynomialFeatures</a></li>
    <li><a href="preprocessing-advanced.html">KBinsDiscretizer / PowerTransformer</a></li>
    <li><a href="preprocessing-advanced.html">QuantileTransformer</a></li>
    <li><a href="preprocessing-advanced.html">OneHotEncoder / OrdinalEncoder / LabelEncoder</a></li>
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
    <li><a href="grid-search.html">GridSearchCV / RandomizedSearchCV</a></li>
    <li><a href="grid-search.html">HalvingGridSearchCV / HalvingRandomSearchCV</a></li>
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

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Infrastructure</strong><span class="ml-badge">3</span></div>
  <ul>
    <li><a href="distributed.html">Distributed (WorkerPool)</a></li>
    <li><a href="export.html">Export (Power BI / Tableau)</a></li>
    <li><a href="registry.html">Estimator registry</a></li>
  </ul>
</div>

</div>

---

## Why it's fast

- **Rust + rayon.** The whole core is written in Rust on top of [rayon](https://github.com/rayon-rs/rayon) work-stealing thread pools \u2014 every CPU core is used automatically, with no GIL contention.
- **Zero-copy NumPy.** Scalers, metrics and predict paths read directly from the NumPy buffer through PyO3 \u2014 no Python-side copies, no list conversions.
- **Cache-friendly layouts.** \`X\` is held column-major, hot inner loops (column dot-products, residual updates, tree split scans, distance reductions) are 4-way unrolled and tile over contiguous chunks.
- **Algorithmic shortcuts.** Coordinate descent in \`Lasso\` / \`ElasticNet\` switches to a parallel chunked reduction once \`n \u2265 50k\`; tree ensembles parallelise across estimators; gradient boosting uses parallel histogram split scans.

## Benchmarks vs scikit-learn

Measured on `n=5000`, `p=20`, single Windows machine (Python 3.11, scikit-learn 1.8.0). Parity is the per-prediction agreement on a held-out test set.

### Trees &amp; ensembles \u2014 where SeraPlot shines

<table class="ml-bench">
<thead><tr><th>Estimator</th><th>Fit speedup</th><th>Predict</th><th>Parity</th></tr></thead>
<tbody>
<tr><td><code>GradientBoostingClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:140px"></span><span class="ml-num">\u00d760.1</span></td><td>\u00d70.51</td><td><span class="ml-parity">0.954</span></td></tr>
<tr><td><code>AdaBoostClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:64px"></span><span class="ml-num">\u00d727.4</span></td><td>\u00d726.0</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>GradientBoostingRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:63px"></span><span class="ml-num">\u00d727.0</span></td><td>\u00d72.4</td><td><span class="ml-parity">0.987</span></td></tr>
<tr><td><code>RandomForestRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:36px"></span><span class="ml-num">\u00d715.4</span></td><td>\u00d713.5</td><td><span class="ml-parity ml-parity-mid">0.907</span></td></tr>
<tr><td><code>AdaBoostRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:25px"></span><span class="ml-num">\u00d710.6</span></td><td>\u00d75.4</td><td><span class="ml-parity ml-parity-mid">0.937</span></td></tr>
<tr><td><code>RandomForestClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:19px"></span><span class="ml-num">\u00d78.2</span></td><td>\u00d713.6</td><td><span class="ml-parity ml-parity-mid">0.934</span></td></tr>
<tr><td><code>DecisionTreeRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:16px"></span><span class="ml-num">\u00d76.7</span></td><td>\u00d73.3</td><td><span class="ml-parity ml-parity-mid">0.900</span></td></tr>
<tr><td><code>DecisionTreeClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:15px"></span><span class="ml-num">\u00d76.5</span></td><td>\u00d74.4</td><td><span class="ml-parity ml-parity-mid">0.886</span></td></tr>
</tbody>
</table>

### Linear models \u2014 close to LAPACK-bound

<table class="ml-bench">
<thead><tr><th>Estimator</th><th>Fit speedup</th><th>Predict</th><th>Parity</th></tr></thead>
<tbody>
<tr><td><code>SGDClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:38px"></span><span class="ml-num">\u00d76.3</span></td><td>\u00d71.4</td><td><span class="ml-parity ml-parity-low">0.715</span></td></tr>
<tr><td><code>RidgeClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:14px"></span><span class="ml-num">\u00d71.6</span></td><td>\u00d71.1</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>SGDRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:13px"></span><span class="ml-num">\u00d71.5</span></td><td>\u00d70.8</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>Lasso</code></td><td class="ml-bar"><span style="width:9px"></span><span class="ml-num">\u00d70.95</span></td><td>\u00d71.2</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>LinearSVC</code></td><td class="ml-bar ml-bar-slow"><span style="width:7px"></span><span class="ml-num">\u00d70.70</span></td><td>\u00d70.8</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>Ridge</code></td><td class="ml-bar ml-bar-slow"><span style="width:6px"></span><span class="ml-num">\u00d70.63</span></td><td>\u00d71.1</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>ElasticNet</code></td><td class="ml-bar ml-bar-slow"><span style="width:4px"></span><span class="ml-num">\u00d70.41</span></td><td>\u00d71.0</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>LogisticRegression</code></td><td class="ml-bar ml-bar-slow"><span style="width:4px"></span><span class="ml-num">\u00d70.35</span></td><td>\u00d71.9</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>LinearRegression</code></td><td class="ml-bar ml-bar-slow"><span style="width:3px"></span><span class="ml-num">\u00d70.27</span></td><td>\u00d70.75</td><td><span class="ml-parity">1.000</span></td></tr>
</tbody>
</table>

### Naive Bayes, neighbors, decomposition, metrics

<table class="ml-bench">
<thead><tr><th>Estimator</th><th>Fit speedup</th><th>Predict</th><th>Parity</th></tr></thead>
<tbody>
<tr><td><code>GaussianNB</code></td><td>\u00d71.1</td><td class="ml-bar ml-bar-fast"><span style="width:34px"></span><span class="ml-num">\u00d714.5</span></td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>BernoulliNB</code></td><td>\u00d71.95</td><td class="ml-bar ml-bar-fast"><span style="width:16px"></span><span class="ml-num">\u00d76.9</span></td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>MultinomialNB</code></td><td>\u00d70.57</td><td class="ml-bar ml-bar-fast"><span style="width:7px"></span><span class="ml-num">\u00d72.8</span></td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>NearestCentroid</code></td><td>\u00d71.3</td><td class="ml-bar ml-bar-fast"><span style="width:8px"></span><span class="ml-num">\u00d73.6</span></td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>KNeighborsClassifier</code></td><td>\u00d70.26</td><td class="ml-bar ml-bar-fast"><span style="width:4px"></span><span class="ml-num">\u00d71.5</span></td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>RobustScaler</code></td><td>\u00d71.9</td><td class="ml-bar ml-bar-fast"><span style="width:11px"></span><span class="ml-num">\u00d74.3</span></td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>TruncatedSVD</code></td><td class="ml-bar ml-bar-fast"><span style="width:10px"></span><span class="ml-num">\u00d74.1</span></td><td>\u00d70.74</td><td><span class="ml-parity ml-parity-low">0.745</span></td></tr>
<tr><td><code>PCA</code></td><td>\u00d70.41</td><td>\u00d70.75</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>f1_score</code></td><td class="ml-bar ml-bar-fast"><span style="width:23px"></span><span class="ml-num">\u00d79.3</span></td><td>\u2014</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>r2_score</code></td><td class="ml-bar ml-bar-fast"><span style="width:11px"></span><span class="ml-num">\u00d74.6</span></td><td>\u2014</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>train_test_split</code></td><td class="ml-bar ml-bar-fast"><span style="width:11px"></span><span class="ml-num">\u00d74.5</span></td><td>\u2014</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>mean_squared_error</code></td><td class="ml-bar ml-bar-fast"><span style="width:10px"></span><span class="ml-num">\u00d74.3</span></td><td>\u2014</td><td><span class="ml-parity">1.000</span></td></tr>
<tr><td><code>accuracy_score</code></td><td class="ml-bar ml-bar-fast"><span style="width:7px"></span><span class="ml-num">\u00d72.8</span></td><td>\u2014</td><td><span class="ml-parity">1.000</span></td></tr>
</tbody>
</table>

<div class="ml-callout">
  <strong>How to read the parity badge.</strong>
  <ul>
    <li><span class="ml-parity">\u2265 0.95</span> \u2014 drop-in replacement. Predictions agree with scikit-learn within numerical tolerance.</li>
    <li><span class="ml-parity ml-parity-mid">0.85 \u2013 0.95</span> \u2014 same statistical accuracy, but tie-breaking on equal-gain tree splits, RNG seeds or iteration ordering produces a few different individual predictions. Test-set score is essentially identical.</li>
    <li><span class="ml-parity ml-parity-low">&lt; 0.85</span> \u2014 algorithmic divergence on noisy or degenerate inputs. Two cases observed: (1) <code>SGDClassifier</code> uses a different mini-batch shuffle / learning-rate schedule, so individual predictions can differ even when accuracy matches; (2) <code>TruncatedSVD</code> on data with near-equal singular values returns a different (but mathematically valid) basis \u2014 sign and rotation of singular vectors are not unique.</li>
  </ul>
  In all cases the test-set <em>score</em> (accuracy / R\u00b2) matches scikit-learn to two decimals.
</div>

## Big-data benchmarks (real OpenFoodFacts data)

Dataset: 504\u202f376 products from [openfoodfacts.org](https://en.openfoodfacts.org/), 9 numerical features (energy, fat, sugars, \u2026), nutri-score grade as 5-class target. Times are wall-clock per `fit` on a single Windows laptop.

<table class="ml-bench">
<thead><tr><th>Estimator</th><th>n=10k</th><th>n=50k</th><th>n=200k</th><th>n=500k</th></tr></thead>
<tbody>
<tr><td><code>GradientBoostingClassifier</code></td><td><span class="ml-num">\u00d738.3</span></td><td><span class="ml-num">\u00d746.0</span></td><td>\u2014</td><td>\u2014</td></tr>
<tr><td><code>GradientBoostingRegressor</code></td><td><span class="ml-num">\u00d715.6</span></td><td><span class="ml-num">\u00d719.7</span></td><td>\u2014</td><td>\u2014</td></tr>
<tr><td><code>AdaBoostClassifier</code></td><td><span class="ml-num">\u00d717.9</span></td><td><span class="ml-num">\u00d720.2</span></td><td>\u2014</td><td>\u2014</td></tr>
<tr><td><code>RandomForestRegressor</code></td><td><span class="ml-num">\u00d710.9</span></td><td><span class="ml-num">\u00d711.7</span></td><td><span class="ml-num">\u00d711.2</span></td><td>\u2014</td></tr>
<tr><td><code>RandomForestClassifier</code></td><td><span class="ml-num">\u00d76.95</span></td><td><span class="ml-num">\u00d76.66</span></td><td><span class="ml-num">\u00d77.95</span></td><td>\u2014</td></tr>
<tr><td><code>DecisionTreeClassifier</code></td><td><span class="ml-num">\u00d74.64</span></td><td><span class="ml-num">\u00d78.93</span></td><td><span class="ml-num">\u00d715.7</span></td><td>\u2014</td></tr>
<tr><td><code>SGDClassifier</code></td><td><span class="ml-num">\u00d76.73</span></td><td><span class="ml-num">\u00d711.0</span></td><td><span class="ml-num">\u00d714.8</span></td><td><span class="ml-num">\u00d715.0</span></td></tr>
<tr><td><code>NearestCentroid</code></td><td><span class="ml-num">\u00d72.21</span></td><td><span class="ml-num">\u00d74.22</span></td><td><span class="ml-num">\u00d76.49</span></td><td><span class="ml-num">\u00d76.77</span></td></tr>
<tr><td><code>GaussianNB</code></td><td><span class="ml-num">\u00d71.77</span></td><td><span class="ml-num">\u00d73.78</span></td><td><span class="ml-num">\u00d76.05</span></td><td><span class="ml-num">\u00d76.66</span></td></tr>
<tr><td><code>BernoulliNB</code></td><td><span class="ml-num">\u00d71.89</span></td><td><span class="ml-num">\u00d73.88</span></td><td><span class="ml-num">\u00d75.71</span></td><td><span class="ml-num">\u00d75.96</span></td></tr>
<tr><td><code>RobustScaler</code></td><td><span class="ml-num">\u00d71.83</span></td><td><span class="ml-num">\u00d73.76</span></td><td><span class="ml-num">\u00d75.21</span></td><td><span class="ml-num">\u00d76.00</span></td></tr>
<tr><td><code>StandardScaler</code></td><td><span class="ml-num">\u00d71.33</span></td><td><span class="ml-num">\u00d72.91</span></td><td><span class="ml-num">\u00d74.72</span></td><td><span class="ml-num">\u00d77.41</span></td></tr>
<tr><td><code>KMeans</code></td><td>\u2014</td><td><span class="ml-num">\u00d73.90</span></td><td><span class="ml-num">\u00d75.38</span></td><td><span class="ml-num">\u00d75.34</span></td></tr>
<tr><td><code>RidgeClassifier</code></td><td><span class="ml-num">\u00d72.90</span></td><td><span class="ml-num">\u00d72.85</span></td><td><span class="ml-num">\u00d73.28</span></td><td><span class="ml-num">\u00d72.97</span></td></tr>
<tr><td><code>Ridge</code></td><td><span class="ml-num">\u00d71.15</span></td><td><span class="ml-num">\u00d71.53</span></td><td><span class="ml-num">\u00d72.02</span></td><td><span class="ml-num">\u00d72.35</span></td></tr>
</tbody>
</table>

</div>

<div class="lang-fr">

<div class="ml-hero">
  <h1>SeraPlot ML</h1>
  <p class="ml-hero-sub">Une bo\u00eete \u00e0 outils ML compatible scikit-learn, \u00e9crite \u00e0 100\u202f% en Rust avec des liaisons Python natives via PyO3. M\u00eame API \u2014 pens\u00e9e pour la vitesse.</p>
  <div class="ml-hero-stats">
    <div class="ml-stat"><div class="ml-stat-val">50+</div><div class="ml-stat-lbl">Estimateurs &amp; m\u00e9triques</div></div>
    <div class="ml-stat"><div class="ml-stat-val">\u00d760</div><div class="ml-stat-lbl">Acc\u00e9l\u00e9ration max au fit</div></div>
    <div class="ml-stat"><div class="ml-stat-val">0,97</div><div class="ml-stat-lbl">Parit\u00e9 moyenne vs sklearn</div></div>
    <div class="ml-stat"><div class="ml-stat-val">100\u202f%</div><div class="ml-stat-lbl">C\u0153ur 100\u202f% Rust</div></div>
  </div>
</div>

<div class="ml-callout">
  <strong>API familli\u00e8re.</strong> Chaque estimateur impl\u00e9mente <code>fit</code> / <code>predict</code> / <code>score</code> avec <code>get_params</code> / <code>set_params</code>. Compatible avec la plupart des pipelines scikit-learn \u2014 aucun r\u00e9-apprentissage.
</div>

## Ce qu'il y a r\u00e9ellement dedans

Inventaire concret, v\u00e9rifi\u00e9 dans le code, livr\u00e9 avec SeraPlot aujourd'hui\u00a0:

- **Mod\u00e8les lin\u00e9aires (8).** \`LinearRegression\`, \`Ridge\`, \`RidgeClassifier\`, \`Lasso\`, \`ElasticNet\`, \`LogisticRegression\`, \`SGDClassifier\`, \`SGDRegressor\` \u2014 tous avec \`fit_intercept\` optionnel et d\u00e9fauts compatibles sklearn.
- **Arbres &amp; ensembles (8).** \`DecisionTreeClassifier/Regressor\`, \`RandomForestClassifier/Regressor\`, \`GradientBoostingClassifier/Regressor\`, \`AdaBoostClassifier/Regressor\` \u2014 tous les classifieurs exposent \`predict_proba\` et \`feature_importances_\`.
- **Voisins (3).** \`KNeighborsClassifier\`, \`KNeighborsRegressor\` (avec \`weights="uniform"|"distance"\`), \`NearestCentroid\`.
- **Naive Bayes (3).** \`GaussianNB\`, \`MultinomialNB\`, \`BernoulliNB\` \u2014 tous avec \`predict_proba\`.
- **SVM lin\u00e9aire (2).** \`LinearSVC\`, \`LinearSVR\` (pertes squared hinge, hinge ou epsilon-insensitive).
- **Clustering (2).** \`KMeans\` (full + mini-batch, k-means++ \`n_init=10\`) et \`DBSCAN\`.
- **Scalers de pr\u00e9traitement (5).** \`StandardScaler\`, \`MinMaxScaler\`, \`RobustScaler\`, \`MaxAbsScaler\`, \`Normalizer\` \u2014 \`partial_fit\` / \`inverse_transform\` quand applicable.
- **Pr\u00e9traitement avanc\u00e9 (8).** \`SimpleImputer\`, \`PolynomialFeatures\`, \`KBinsDiscretizer\`, \`PowerTransformer\` (yeo-johnson / box-cox), \`QuantileTransformer\`, \`OneHotEncoder\`, \`OrdinalEncoder\`, \`LabelEncoder\`.
- **D\u00e9composition (2).** \`PCA\` (\`svd_solver="auto"|"full"|"randomized"\`), \`TruncatedSVD\`.
- **S\u00e9lection de mod\u00e8le (8).** \`train_test_split\`, \`StratifiedKFold\`, le dispatcher JSON \`ml_kfold_split\` (kfold / stratified / group), \`GridSearchCV\`, \`RandomizedSearchCV\`, \`HalvingGridSearchCV\`, \`HalvingRandomSearchCV\`, \`ml_permutation_importance\`.
- **M\u00e9triques (40+).** Classification, r\u00e9gression et clustering, dont \`accuracy_score\`, \`f1_score\`, \`mean_squared_error\`, \`r2_score\`, \`classification_report\`, \`confusion_matrix\`, silhouette, calinski-harabasz, davies-bouldin\u2026
- **Anomalie (1).** \`IsolationForest\` (dispatcher JSON).
- **Distribu\u00e9.** \`WorkerPool\` pour scatter shard\u00e9 et all-reduce.
- **Export.** Dispatchers natifs pour exports de mod\u00e8les Power BI et Tableau.

## D\u00e9marrage rapide

\`\`\`python
import seraplot as sp

X_train, X_test, y_train, y_test = sp.train_test_split(X, y, test_size=0.2)

model = sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1)
model.fit(X_train, y_train)

print(f"Pr\u00e9cision\u00a0: {model.score(X_test, y_test):.4f}")
print(f"Classes\u00a0: {model.classes_}")
proba = model.predict_proba(X_test)
\`\`\`

## API commune

\`\`\`python
model.fit(X, y)                    # Entra\u00eenement
model.predict(X)                   # Pr\u00e9diction
model.score(X, y)                  # Pr\u00e9cision (clf) ou R\u00b2 (r\u00e9g)

model.predict_proba(X)             # Probabilit\u00e9s de classe (classifieurs)
model.classes_                     # Classes tri\u00e9es (classifieurs)

model.coef_, model.intercept_      # Mod\u00e8les lin\u00e9aires
model.feature_importances_         # Mod\u00e8les \u00e0 base d'arbres
\`\`\`

---

## Parcourir par cat\u00e9gorie

<div class="ml-cat-grid">

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Mod\u00e8les lin\u00e9aires</strong><span class="ml-badge">8</span></div>
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
    <li><a href="decision-tree.html">DecisionTreeClassifier / Regressor</a></li>
    <li><a href="random-forest.html">RandomForestClassifier / Regressor</a></li>
    <li><a href="gradient-boosting.html">GradientBoostingClassifier / Regressor</a></li>
    <li><a href="adaboost.html">AdaBoostClassifier / Regressor</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Voisins</strong><span class="ml-badge">3</span></div>
  <ul>
    <li><a href="knn.html">KNeighborsClassifier / Regressor</a></li>
    <li><a href="knn.html#nearestcentroid">NearestCentroid</a></li>
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
  <div class="ml-cat-h"><strong>SVM (lin\u00e9aire)</strong><span class="ml-badge">2</span></div>
  <ul>
    <li><a href="svm.html">LinearSVC</a></li>
    <li><a href="svm.html">LinearSVR</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Clustering</strong><span class="ml-badge">2</span></div>
  <ul>
    <li><a href="kmeans-class.html">KMeans</a></li>
    <li><a href="dbscan-class.html">DBSCAN</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Pr\u00e9traitement</strong><span class="ml-badge">13</span></div>
  <ul>
    <li><a href="preprocessing.html">StandardScaler / MinMaxScaler</a></li>
    <li><a href="preprocessing.html">RobustScaler / MaxAbsScaler</a></li>
    <li><a href="preprocessing.html">Normalizer</a></li>
    <li><a href="preprocessing-advanced.html">SimpleImputer / PolynomialFeatures</a></li>
    <li><a href="preprocessing-advanced.html">KBinsDiscretizer / PowerTransformer</a></li>
    <li><a href="preprocessing-advanced.html">QuantileTransformer</a></li>
    <li><a href="preprocessing-advanced.html">OneHotEncoder / OrdinalEncoder / LabelEncoder</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>D\u00e9composition</strong><span class="ml-badge">2</span></div>
  <ul>
    <li><a href="decomposition.html">PCA</a></li>
    <li><a href="decomposition.html">TruncatedSVD</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>S\u00e9lection de mod\u00e8le</strong><span class="ml-badge">8</span></div>
  <ul>
    <li><a href="train-test-split.html">train_test_split / StratifiedKFold</a></li>
    <li><a href="cv-splitters.html">D\u00e9coupeurs CV (kfold / stratified / group)</a></li>
    <li><a href="grid-search.html">GridSearchCV / RandomizedSearchCV</a></li>
    <li><a href="grid-search.html">HalvingGridSearchCV / HalvingRandomSearchCV</a></li>
    <li><a href="permutation-importance.html">Importance par permutation</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>M\u00e9triques</strong><span class="ml-badge">40+</span></div>
  <ul>
    <li><a href="metrics-classification.html">M\u00e9triques de classification</a></li>
    <li><a href="metrics-regression.html">M\u00e9triques de r\u00e9gression</a></li>
    <li><a href="metrics-clustering.html">M\u00e9triques de clustering</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Anomalie</strong><span class="ml-badge">1</span></div>
  <ul>
    <li><a href="isolation-forest.html">IsolationForest</a></li>
  </ul>
</div>

<div class="ml-cat">
  <div class="ml-cat-h"><strong>Infrastructure</strong><span class="ml-badge">3</span></div>
  <ul>
    <li><a href="distributed.html">Distribu\u00e9 (WorkerPool)</a></li>
    <li><a href="export.html">Export (Power BI / Tableau)</a></li>
    <li><a href="registry.html">Registre d'estimateurs</a></li>
  </ul>
</div>

</div>

---

## Pourquoi c'est rapide

- **Rust + rayon.** Le c\u0153ur est \u00e9crit en Rust sur les pools de threads work-stealing de [rayon](https://github.com/rayon-rs/rayon) \u2014 chaque c\u0153ur CPU est utilis\u00e9 automatiquement, sans contention du GIL.
- **Zero-copy NumPy.** Les scalers, m\u00e9triques et chemins predict lisent directement depuis le buffer NumPy via PyO3 \u2014 aucune copie c\u00f4t\u00e9 Python, aucune conversion en list.
- **Layouts cache-friendly.** \`X\` est conserv\u00e9 column-major\u00a0; les boucles chaudes (produits scalaires de colonnes, mises \u00e0 jour de r\u00e9sidus, scans de splits, r\u00e9ductions de distances) sont d\u00e9roul\u00e9es 4\u00d71 et travaillent par tuiles contigu\u00ebs.
- **Raccourcis algorithmiques.** La descente par coordonn\u00e9es de \`Lasso\` / \`ElasticNet\` bascule sur une r\u00e9duction parall\u00e8le par chunks d\u00e8s \`n \u2265 50k\`\u00a0; les ensembles d'arbres parall\u00e9lisent les estimateurs\u00a0; le gradient boosting utilise des scans d'histogramme parall\u00e8les.

## Benchmarks vs scikit-learn

Mesur\u00e9 sur `n=5000`, `p=20`, machine Windows unique (Python 3.11, scikit-learn 1.8.0). La parit\u00e9 est l'accord par pr\u00e9diction sur un jeu de test.

### Arbres &amp; ensembles \u2014 l\u00e0 o\u00f9 SeraPlot brille

<table class="ml-bench">
<thead><tr><th>Estimateur</th><th>Acc\u00e9l\u00e9ration fit</th><th>Predict</th><th>Parit\u00e9</th></tr></thead>
<tbody>
<tr><td><code>GradientBoostingClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:140px"></span><span class="ml-num">\u00d760,1</span></td><td>\u00d70,51</td><td><span class="ml-parity">0,954</span></td></tr>
<tr><td><code>AdaBoostClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:64px"></span><span class="ml-num">\u00d727,4</span></td><td>\u00d726,0</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>GradientBoostingRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:63px"></span><span class="ml-num">\u00d727,0</span></td><td>\u00d72,4</td><td><span class="ml-parity">0,987</span></td></tr>
<tr><td><code>RandomForestRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:36px"></span><span class="ml-num">\u00d715,4</span></td><td>\u00d713,5</td><td><span class="ml-parity ml-parity-mid">0,907</span></td></tr>
<tr><td><code>AdaBoostRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:25px"></span><span class="ml-num">\u00d710,6</span></td><td>\u00d75,4</td><td><span class="ml-parity ml-parity-mid">0,937</span></td></tr>
<tr><td><code>RandomForestClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:19px"></span><span class="ml-num">\u00d78,2</span></td><td>\u00d713,6</td><td><span class="ml-parity ml-parity-mid">0,934</span></td></tr>
<tr><td><code>DecisionTreeRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:16px"></span><span class="ml-num">\u00d76,7</span></td><td>\u00d73,3</td><td><span class="ml-parity ml-parity-mid">0,900</span></td></tr>
<tr><td><code>DecisionTreeClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:15px"></span><span class="ml-num">\u00d76,5</span></td><td>\u00d74,4</td><td><span class="ml-parity ml-parity-mid">0,886</span></td></tr>
</tbody>
</table>

### Mod\u00e8les lin\u00e9aires \u2014 proches de la limite LAPACK

<table class="ml-bench">
<thead><tr><th>Estimateur</th><th>Acc\u00e9l\u00e9ration fit</th><th>Predict</th><th>Parit\u00e9</th></tr></thead>
<tbody>
<tr><td><code>SGDClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:38px"></span><span class="ml-num">\u00d76,3</span></td><td>\u00d71,4</td><td><span class="ml-parity ml-parity-low">0,715</span></td></tr>
<tr><td><code>RidgeClassifier</code></td><td class="ml-bar ml-bar-fast"><span style="width:14px"></span><span class="ml-num">\u00d71,6</span></td><td>\u00d71,1</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>SGDRegressor</code></td><td class="ml-bar ml-bar-fast"><span style="width:13px"></span><span class="ml-num">\u00d71,5</span></td><td>\u00d70,8</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>Lasso</code></td><td class="ml-bar"><span style="width:9px"></span><span class="ml-num">\u00d70,95</span></td><td>\u00d71,2</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>LinearSVC</code></td><td class="ml-bar ml-bar-slow"><span style="width:7px"></span><span class="ml-num">\u00d70,70</span></td><td>\u00d70,8</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>Ridge</code></td><td class="ml-bar ml-bar-slow"><span style="width:6px"></span><span class="ml-num">\u00d70,63</span></td><td>\u00d71,1</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>ElasticNet</code></td><td class="ml-bar ml-bar-slow"><span style="width:4px"></span><span class="ml-num">\u00d70,41</span></td><td>\u00d71,0</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>LogisticRegression</code></td><td class="ml-bar ml-bar-slow"><span style="width:4px"></span><span class="ml-num">\u00d70,35</span></td><td>\u00d71,9</td><td><span class="ml-parity">1,000</span></td></tr>
<tr><td><code>LinearRegression</code></td><td class="ml-bar ml-bar-slow"><span style="width:3px"></span><span class="ml-num">\u00d70,27</span></td><td>\u00d70,75</td><td><span class="ml-parity">1,000</span></td></tr>
</tbody>
</table>

<div class="ml-callout">
  <strong>Lecture du badge de parit\u00e9.</strong>
  <ul>
    <li><span class="ml-parity">\u2265 0,95</span> \u2014 remplacement direct. Les pr\u00e9dictions correspondent \u00e0 scikit-learn \u00e0 la tol\u00e9rance num\u00e9rique pr\u00e8s.</li>
    <li><span class="ml-parity ml-parity-mid">0,85 \u2013 0,95</span> \u2014 m\u00eame pr\u00e9cision statistique, mais le tie-breaking sur les splits d'arbres \u00e0 gain \u00e9gal, les graines RNG ou l'ordre d'it\u00e9ration produisent quelques pr\u00e9dictions individuelles diff\u00e9rentes. Le score sur un jeu de test est quasi identique.</li>
    <li><span class="ml-parity ml-parity-low">&lt; 0,85</span> \u2014 divergence algorithmique sur entr\u00e9es bruit\u00e9es ou d\u00e9g\u00e9n\u00e9r\u00e9es. Deux cas observ\u00e9s\u00a0: (1) <code>SGDClassifier</code> utilise un m\u00e9lange mini-batch / planning de taux d'apprentissage diff\u00e9rents, donc les pr\u00e9dictions individuelles diff\u00e8rent m\u00eame quand l'accuracy correspond\u00a0; (2) <code>TruncatedSVD</code> sur des donn\u00e9es aux valeurs singuli\u00e8res quasi-\u00e9gales renvoie une base diff\u00e9rente (mais math\u00e9matiquement valide) \u2014 signe et rotation des vecteurs singuliers ne sont pas uniques.</li>
  </ul>
  Dans tous les cas, le <em>score</em> sur jeu de test (accuracy / R\u00b2) correspond \u00e0 scikit-learn \u00e0 deux d\u00e9cimales.
</div>

## Benchmarks gros volume (donn\u00e9es r\u00e9elles OpenFoodFacts)

Jeu de donn\u00e9es\u00a0: 504\u202f376 produits depuis [openfoodfacts.org](https://fr.openfoodfacts.org/), 9 features num\u00e9riques (\u00e9nergie, lipides, sucres, \u2026), nutri-score (5 classes) en cible. Temps `fit` mesur\u00e9s sur un seul portable Windows.

<table class="ml-bench">
<thead><tr><th>Estimateur</th><th>n=10k</th><th>n=50k</th><th>n=200k</th><th>n=500k</th></tr></thead>
<tbody>
<tr><td><code>GradientBoostingClassifier</code></td><td><span class="ml-num">\u00d738,3</span></td><td><span class="ml-num">\u00d746,0</span></td><td>\u2014</td><td>\u2014</td></tr>
<tr><td><code>GradientBoostingRegressor</code></td><td><span class="ml-num">\u00d715,6</span></td><td><span class="ml-num">\u00d719,7</span></td><td>\u2014</td><td>\u2014</td></tr>
<tr><td><code>AdaBoostClassifier</code></td><td><span class="ml-num">\u00d717,9</span></td><td><span class="ml-num">\u00d720,2</span></td><td>\u2014</td><td>\u2014</td></tr>
<tr><td><code>RandomForestRegressor</code></td><td><span class="ml-num">\u00d710,9</span></td><td><span class="ml-num">\u00d711,7</span></td><td><span class="ml-num">\u00d711,2</span></td><td>\u2014</td></tr>
<tr><td><code>RandomForestClassifier</code></td><td><span class="ml-num">\u00d76,95</span></td><td><span class="ml-num">\u00d76,66</span></td><td><span class="ml-num">\u00d77,95</span></td><td>\u2014</td></tr>
<tr><td><code>DecisionTreeClassifier</code></td><td><span class="ml-num">\u00d74,64</span></td><td><span class="ml-num">\u00d78,93</span></td><td><span class="ml-num">\u00d715,7</span></td><td>\u2014</td></tr>
<tr><td><code>SGDClassifier</code></td><td><span class="ml-num">\u00d76,73</span></td><td><span class="ml-num">\u00d711,0</span></td><td><span class="ml-num">\u00d714,8</span></td><td><span class="ml-num">\u00d715,0</span></td></tr>
<tr><td><code>NearestCentroid</code></td><td><span class="ml-num">\u00d72,21</span></td><td><span class="ml-num">\u00d74,22</span></td><td><span class="ml-num">\u00d76,49</span></td><td><span class="ml-num">\u00d76,77</span></td></tr>
<tr><td><code>GaussianNB</code></td><td><span class="ml-num">\u00d71,77</span></td><td><span class="ml-num">\u00d73,78</span></td><td><span class="ml-num">\u00d76,05</span></td><td><span class="ml-num">\u00d76,66</span></td></tr>
<tr><td><code>BernoulliNB</code></td><td><span class="ml-num">\u00d71,89</span></td><td><span class="ml-num">\u00d73,88</span></td><td><span class="ml-num">\u00d75,71</span></td><td><span class="ml-num">\u00d75,96</span></td></tr>
<tr><td><code>RobustScaler</code></td><td><span class="ml-num">\u00d71,83</span></td><td><span class="ml-num">\u00d73,76</span></td><td><span class="ml-num">\u00d75,21</span></td><td><span class="ml-num">\u00d76,00</span></td></tr>
<tr><td><code>StandardScaler</code></td><td><span class="ml-num">\u00d71,33</span></td><td><span class="ml-num">\u00d72,91</span></td><td><span class="ml-num">\u00d74,72</span></td><td><span class="ml-num">\u00d77,41</span></td></tr>
<tr><td><code>KMeans</code></td><td>\u2014</td><td><span class="ml-num">\u00d73,90</span></td><td><span class="ml-num">\u00d75,38</span></td><td><span class="ml-num">\u00d75,34</span></td></tr>
<tr><td><code>RidgeClassifier</code></td><td><span class="ml-num">\u00d72,90</span></td><td><span class="ml-num">\u00d72,85</span></td><td><span class="ml-num">\u00d73,28</span></td><td><span class="ml-num">\u00d72,97</span></td></tr>
<tr><td><code>Ridge</code></td><td><span class="ml-num">\u00d71,15</span></td><td><span class="ml-num">\u00d71,53</span></td><td><span class="ml-num">\u00d72,02</span></td><td><span class="ml-num">\u00d72,35</span></td></tr>
</tbody>
</table>

</div>
