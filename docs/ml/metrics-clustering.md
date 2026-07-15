# Clustering Metrics

<div class="lang-en">

## API Reference

**Signatures**

```python
sp.silhouette_score(X, labels)                          -> float
sp.davies_bouldin_score(X, labels)                      -> float
sp.calinski_harabasz_score(X, labels)                   -> float

sp.adjusted_rand_score(labels_true, labels_pred)        -> float
sp.normalized_mutual_info_score(labels_true, labels_pred) -> float
sp.fowlkes_mallows_score(labels_true, labels_pred)      -> float
sp.homogeneity_score(labels_true, labels_pred)          -> float
sp.completeness_score(labels_true, labels_pred)         -> float
sp.v_measure_score(labels_true, labels_pred)            -> float
```

**Function summary**

| Function | Type | Range | Best | Description |
|----------|------|-------|------|-------------|
| `silhouette_score` | internal | $[-1, 1]$ | $\to 1$ | Mean silhouette over samples |
| `davies_bouldin_score` | internal | $[0, \infty)$ | $\to 0$ | Mean cluster similarity ratio |
| `calinski_harabasz_score` | internal | $[0, \infty)$ | $\to \infty$ | Variance ratio criterion |
| `adjusted_rand_score` | external | $[-1, 1]$ | $\to 1$ | Rand index adjusted for chance |
| `normalized_mutual_info_score` | external | $[0, 1]$ | $\to 1$ | MI normalised by mean entropy |
| `fowlkes_mallows_score` | external | $[0, 1]$ | $\to 1$ | Geometric mean of pairwise P/R |
| `homogeneity_score` | external | $[0, 1]$ | $\to 1$ | Each cluster contains one class |
| `completeness_score` | external | $[0, 1]$ | $\to 1$ | Each class lies in one cluster |
| `v_measure_score` | external | $[0, 1]$ | $\to 1$ | Harmonic mean of H and C |

`X` is a 2D `ndarray` `(n_samples, n_features)` (the silhouette is parallelised with rayon).

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

rng = np.random.default_rng(0)
X = np.vstack([
    rng.normal(loc=( 0,  0), scale=0.6, size=(100, 2)),
    rng.normal(loc=( 5,  5), scale=0.6, size=(100, 2)),
    rng.normal(loc=(-5,  5), scale=0.6, size=(100, 2)),
])
y_true = [0]*100 + [1]*100 + [2]*100

km = sp.KMeans(k=3, max_iter=100, tol=1e-4, mini_batch=False, batch_size=0, n_init=4)
labels = list(km.fit_predict(X))

print("silhouette        :", sp.silhouette_score(X, labels))
print("davies_bouldin    :", sp.davies_bouldin_score(X, labels))
print("calinski_harabasz :", sp.calinski_harabasz_score(X, labels))

print("ARI               :", sp.adjusted_rand_score(y_true, labels))
print("NMI               :", sp.normalized_mutual_info_score(y_true, labels))
print("FMI               :", sp.fowlkes_mallows_score(y_true, labels))
print("homogeneity       :", sp.homogeneity_score(y_true, labels))
print("completeness      :", sp.completeness_score(y_true, labels))
print("v_measure         :", sp.v_measure_score(y_true, labels))
```

</details>

---

## Algorithmic Functioning

**Silhouette** — for each sample $i$, with $a_i$ the mean distance to the same-cluster points and $b_i$ the mean distance to the nearest other cluster:

<div>$$s_i = \frac{b_i - a_i}{\max(a_i, b_i)}, \qquad S = \frac{1}{n}\sum_i s_i$$</div>

**Davies-Bouldin** — for each cluster $k$ with intra-cluster dispersion $S_k$ and centroid distance $d_{kj}$:

<div>$$\text{DB} = \frac{1}{K}\sum_k \max_{j \neq k} \frac{S_k + S_j}{d_{kj}}$$</div>

**Calinski-Harabasz** — variance ratio between/within clusters, with $B$ between-cluster scatter and $W$ within-cluster scatter:

<div>$$\text{CH} = \frac{\mathrm{tr}(B)}{\mathrm{tr}(W)} \cdot \frac{n - K}{K - 1}$$</div>

**Adjusted Rand Index** — Rand index corrected for chance by subtracting the expected value:

<div>$$\text{ARI} = \frac{\text{RI} - E[\text{RI}]}{\max(\text{RI}) - E[\text{RI}]}$$</div>

**NMI** — normalised mutual information:

<div>$$\text{NMI} = \frac{2 \cdot I(U; V)}{H(U) + H(V)}$$</div>

**Homogeneity / Completeness / V-measure** — entropy-based duals:

<div>$$h = 1 - \frac{H(C \mid K)}{H(C)}, \qquad c = 1 - \frac{H(K \mid C)}{H(K)}, \qquad V = \frac{2 h c}{h + c}$$</div>

**Fowlkes-Mallows** — geometric mean of pairwise precision and recall computed from TP/FP/FN of the pair confusion matrix:

<div>$$\text{FMI} = \sqrt{\frac{TP}{TP+FP} \cdot \frac{TP}{TP+FN}}$$</div>

</div>

<div class="lang-fr">

## Référence API

**Signatures**

```python
sp.silhouette_score(X, labels)                          -> float
sp.davies_bouldin_score(X, labels)                      -> float
sp.calinski_harabasz_score(X, labels)                   -> float

sp.adjusted_rand_score(labels_true, labels_pred)        -> float
sp.normalized_mutual_info_score(labels_true, labels_pred) -> float
sp.fowlkes_mallows_score(labels_true, labels_pred)      -> float
sp.homogeneity_score(labels_true, labels_pred)          -> float
sp.completeness_score(labels_true, labels_pred)         -> float
sp.v_measure_score(labels_true, labels_pred)            -> float
```

**Résumé**

| Fonction | Type | Plage | Idéal | Description |
|----------|------|-------|-------|-------------|
| `silhouette_score` | interne | $[-1, 1]$ | $\to 1$ | Silhouette moyenne |
| `davies_bouldin_score` | interne | $[0, \infty)$ | $\to 0$ | Ratio de similarité moyen |
| `calinski_harabasz_score` | interne | $[0, \infty)$ | $\to \infty$ | Critère de ratio de variance |
| `adjusted_rand_score` | externe | $[-1, 1]$ | $\to 1$ | Index de Rand corrigé du hasard |
| `normalized_mutual_info_score` | externe | $[0, 1]$ | $\to 1$ | MI normalisée par l'entropie |
| `fowlkes_mallows_score` | externe | $[0, 1]$ | $\to 1$ | Moyenne géométrique de P/R par paires |
| `homogeneity_score` | externe | $[0, 1]$ | $\to 1$ | Chaque cluster contient une classe |
| `completeness_score` | externe | $[0, 1]$ | $\to 1$ | Chaque classe est dans un cluster |
| `v_measure_score` | externe | $[0, 1]$ | $\to 1$ | Moyenne harmonique de H et C |

`X` est un `ndarray` 2D `(n_samples, n_features)` (la silhouette est parallélisée avec rayon).

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

rng = np.random.default_rng(0)
X = np.vstack([
    rng.normal(loc=( 0,  0), scale=0.6, size=(100, 2)),
    rng.normal(loc=( 5,  5), scale=0.6, size=(100, 2)),
    rng.normal(loc=(-5,  5), scale=0.6, size=(100, 2)),
])
y_true = [0]*100 + [1]*100 + [2]*100

km = sp.KMeans(k=3, max_iter=100, tol=1e-4, mini_batch=False, batch_size=0, n_init=4)
labels = list(km.fit_predict(X))

print("silhouette        :", sp.silhouette_score(X, labels))
print("davies_bouldin    :", sp.davies_bouldin_score(X, labels))
print("calinski_harabasz :", sp.calinski_harabasz_score(X, labels))

print("ARI               :", sp.adjusted_rand_score(y_true, labels))
print("NMI               :", sp.normalized_mutual_info_score(y_true, labels))
print("FMI               :", sp.fowlkes_mallows_score(y_true, labels))
print("homogeneity       :", sp.homogeneity_score(y_true, labels))
print("completeness      :", sp.completeness_score(y_true, labels))
print("v_measure         :", sp.v_measure_score(y_true, labels))
```

</details>

---

## Fonctionnement algorithmique

**Silhouette** — pour chaque échantillon $i$, avec $a_i$ la distance moyenne aux points du même cluster et $b_i$ la distance moyenne au cluster voisin le plus proche :

<div>$$s_i = \frac{b_i - a_i}{\max(a_i, b_i)}, \qquad S = \frac{1}{n}\sum_i s_i$$</div>

**Davies-Bouldin** — pour chaque cluster $k$ avec dispersion intra $S_k$ et distance entre centroïdes $d_{kj}$ :

<div>$$\text{DB} = \frac{1}{K}\sum_k \max_{j \neq k} \frac{S_k + S_j}{d_{kj}}$$</div>

**Calinski-Harabasz** — ratio de variance inter/intra, avec $B$ dispersion inter-cluster et $W$ dispersion intra-cluster :

<div>$$\text{CH} = \frac{\mathrm{tr}(B)}{\mathrm{tr}(W)} \cdot \frac{n - K}{K - 1}$$</div>

**Index de Rand ajusté** — Rand corrigé en soustrayant la valeur attendue :

<div>$$\text{ARI} = \frac{\text{RI} - E[\text{RI}]}{\max(\text{RI}) - E[\text{RI}]}$$</div>

**NMI** — information mutuelle normalisée :

<div>$$\text{NMI} = \frac{2 \cdot I(U; V)}{H(U) + H(V)}$$</div>

**Homogénéité / Complétude / V-mesure** — duaux basés sur l'entropie :

<div>$$h = 1 - \frac{H(C \mid K)}{H(C)}, \qquad c = 1 - \frac{H(K \mid C)}{H(K)}, \qquad V = \frac{2 h c}{h + c}$$</div>

**Fowlkes-Mallows** — moyenne géométrique de la précision et du rappel par paires, calculées à partir de TP/FP/FN de la matrice de confusion par paires :

<div>$$\text{FMI} = \sqrt{\frac{TP}{TP+FP} \cdot \frac{TP}{TP+FN}}$$</div>

</div>
