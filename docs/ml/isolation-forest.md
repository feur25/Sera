# Isolation Forest

<div class="lang-en">

Unsupervised anomaly / outlier detector. Builds an ensemble of *isolation trees* (iTrees) where each split picks a feature and threshold uniformly at random. Anomalies have **shorter expected path lengths** because they are easier to isolate.

## API Reference

**Signature (Python)**

```python
clf = sp.IsolationForest(
    n_estimators=100,
    max_samples=256,
    contamination=0.1,
    seed=42,
)
clf.fit(X)
labels  = clf.predict(X)            -> ndarray[int32]   {-1, +1}
scores  = clf.score_samples(X)      -> ndarray[float64] (lower = more anomalous)
margin  = clf.decision_function(X)  -> ndarray[float64] (negative = anomalous)
labels  = clf.fit_predict(X)
```

**Cross-language JSON dispatcher**

Available from Python / WebAssembly / C FFI as `ml_isolation_forest`:

```python
sp.ml_isolation_forest(json.dumps({
    "data":          [[0.0, 0.0], [0.1, 0.1], [10.0, 10.0]],
    "x_test":        [[0.0, 0.05], [9.5, 9.5]],
    "n_estimators":  100,
    "max_samples":   256,
    "contamination": 0.1,
    "seed":          42,
}))
```

Returns:

```json
{
  "labels":      [1, 1, -1],
  "scores":      [-0.42, -0.41, -0.54],
  "threshold":   -0.49,
  "test_labels": [1, -1],
  "test_scores": [-0.43, -0.55]
}
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_estimators` | `int` | `100` | Number of iTrees |
| `max_samples` | `int` | `256` | Sub-sample size $\psi$ used to build each tree |
| `contamination` | `float` | `0.1` | Expected outlier fraction (calibrates `threshold_`) |
| `seed` | `int` | `42` | Master RNG seed |

Attribute: `threshold_ : float` — the score below which a point is flagged as outlier (`-1`).

## Algorithmic Functioning

For each iTree we recursively pick a random feature $f$ and a threshold $t \sim \mathcal{U}(\min_f, \max_f)$ until the node is pure or depth reaches $\lceil \log_2 \psi \rceil$. The path length $h(x)$ is the depth at which $x$ falls.

The anomaly score is

<div>$$s(x) = -2^{-\,\frac{\mathrm{E}[h(x)]}{c(\psi)}},\qquad c(\psi) = 2 H(\psi-1) - \frac{2(\psi-1)}{\psi}$$</div>

where $H(k) \approx \ln k + 0.5772$ is the harmonic number. Lower $s(x)$ = more anomalous. The `threshold_` is the empirical $\lceil \texttt{contamination} \cdot n \rceil$-th lowest training score.

</div>

<div class="lang-fr">

Détecteur d'anomalies non supervisé. Construit un ensemble d'*arbres d'isolation* (iTrees) où chaque coupure choisit aléatoirement une feature et un seuil. Les anomalies ont une **profondeur d'isolement attendue plus faible** car plus faciles à séparer.

## Référence API

**Signature (Python)**

```python
clf = sp.IsolationForest(
    n_estimators=100,
    max_samples=256,
    contamination=0.1,
    seed=42,
)
clf.fit(X)
labels  = clf.predict(X)            -> ndarray[int32]   {-1, +1}
scores  = clf.score_samples(X)      -> ndarray[float64] (plus bas = plus anormal)
margin  = clf.decision_function(X)  -> ndarray[float64] (négatif = anormal)
labels  = clf.fit_predict(X)
```

**Dispatcher JSON inter-langage**

Disponible depuis Python / WebAssembly / C FFI via `ml_isolation_forest` :

```python
sp.ml_isolation_forest(json.dumps({
    "data":          [[0.0, 0.0], [0.1, 0.1], [10.0, 10.0]],
    "x_test":        [[0.0, 0.05], [9.5, 9.5]],
    "n_estimators":  100,
    "max_samples":   256,
    "contamination": 0.1,
    "seed":          42,
}))
```

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_estimators` | `int` | `100` | Nombre d'iTrees |
| `max_samples` | `int` | `256` | Taille du sous-échantillon $\psi$ par arbre |
| `contamination` | `float` | `0.1` | Fraction d'anomalies attendue (calibre `threshold_`) |
| `seed` | `int` | `42` | Graine maître |

Attribut : `threshold_ : float` — score sous lequel un point est marqué `-1`.

## Fonctionnement algorithmique

Chaque iTree choisit récursivement une feature $f$ et un seuil $t \sim \mathcal{U}(\min_f, \max_f)$ jusqu'à pureté du nœud ou profondeur $\lceil \log_2 \psi \rceil$. La longueur de chemin $h(x)$ est la profondeur à laquelle $x$ tombe.

Le score d'anomalie est

<div>$$s(x) = -2^{-\,\frac{\mathrm{E}[h(x)]}{c(\psi)}},\qquad c(\psi) = 2 H(\psi-1) - \frac{2(\psi-1)}{\psi}$$</div>

où $H(k) \approx \ln k + 0.5772$ est le nombre harmonique. Plus $s(x)$ est bas, plus le point est anormal. Le `threshold_` est le $\lceil \texttt{contamination} \cdot n \rceil$-ième score le plus bas du jeu d'entraînement.

</div>
