# StandardScaler / MinMaxScaler / RobustScaler / MaxAbsScaler / Normalizer

<div class="lang-en">

## API Reference

**Signature**

```python
scaler = sp.StandardScaler(with_mean=True, with_std=True)
scaler = sp.MinMaxScaler(feature_range=(0.0, 1.0))
scaler = sp.RobustScaler(with_centering=True, with_scaling=True, quantile_range=(25.0, 75.0))
scaler = sp.MaxAbsScaler()
scaler = sp.Normalizer(norm="l2")

scaler.fit(X)
X_scaled  = scaler.transform(X)       -> ndarray
X_back    = scaler.inverse_transform(X_scaled) -> ndarray   # most scalers
X_scaled  = scaler.fit_transform(X)   -> ndarray
```

**Constructor parameters — StandardScaler**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `with_mean` | `bool` | `True` | Subtract the mean |
| `with_std` | `bool` | `True` | Divide by standard deviation |

**Constructor parameters — MinMaxScaler**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `feature_range` | `tuple(float,float)` | `(0.0, 1.0)` | Desired output range |

**Constructor parameters — RobustScaler**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `with_centering` | `bool` | `True` | Subtract the median |
| `with_scaling` | `bool` | `True` | Divide by IQR |
| `quantile_range` | `tuple(float,float)` | `(25.0, 75.0)` | Quantile range for IQR |

**Constructor parameters — Normalizer**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `norm` | `str` | `"l2"` | Per-sample norm: `"l1"`, `"l2"`, `"max"` |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `mean_` | `list[float]` | Per-feature mean (StandardScaler) |
| `scale_` | `list[float]` | Per-feature scale factor |
| `min_` | `list[float]` | Per-feature minimum (MinMaxScaler) |
| `data_range_` | `list[float]` | Per-feature range (MinMaxScaler) |
| `center_` | `list[float]` | Per-feature median (RobustScaler) |
| `max_abs_` | `list[float]` | Per-feature maximum absolute value (MaxAbsScaler) |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X_train = np.random.randn(300, 4) * [5, 2, 0.1, 100]
X_test  = np.random.randn(50, 4)  * [5, 2, 0.1, 100]

scaler = sp.StandardScaler()
X_tr = scaler.fit_transform(X_train)
X_te = scaler.transform(X_test)
print(f"Train mean≈0: {X_tr.mean(0).round(3)}")

mm = sp.MinMaxScaler(feature_range=(0.0, 1.0))
print(f"MinMax range: {mm.fit_transform(X_train).min(0).round(3)}, {mm.fit_transform(X_train).max(0).round(3)}")
```

</details>

---

## Algorithmic Functioning

Each scaler applies a **column-wise linear transformation** fitted on the training set and applied identically to any new data.

---

### StandardScaler

Standardises features to zero mean and unit variance:

$$x'_j = \frac{x_j - \mu_j}{\sigma_j}$$

where $\mu_j = \frac{1}{n}\sum_i x_{ij}$ and $\sigma_j = \sqrt{\frac{1}{n}\sum_i (x_{ij}-\mu_j)^2}$.

---

### MinMaxScaler

Rescales each feature into the interval $[a, b]$ (`feature_range`):

$$x'_j = a + \frac{x_j - \min_j}{\max_j - \min_j}(b - a)$$

Sensitive to outliers since it uses $\min$ and $\max$.

---

### RobustScaler

Uses **median** and **interquartile range** (IQR), making it robust to outliers:

$$x'_j = \frac{x_j - Q_{50}(j)}{Q_{75}(j) - Q_{25}(j)}$$

where $Q_p(j)$ is the $p$-th percentile of feature $j$.

---

### MaxAbsScaler

Scales each feature by its maximum absolute value, preserving sparsity and the origin:

$$x'_j = \frac{x_j}{\max_i |x_{ij}|}$$

Result lies in $[-1, 1]$.

---

### Normalizer

Scales each **sample** (row) to unit norm, applied independently of fit:

$$x'_i = \frac{x_i}{\|x_i\|_p}, \qquad p \in \{1, 2, \infty\}$$

No `fit` step is required — the transformation is stateless.

**Inverse transform** is defined for StandardScaler, MinMaxScaler, RobustScaler, and MaxAbsScaler:

$$x_j = x'_j \cdot \text{scale}_j + \text{center}_j$$

</div>

<div class="lang-fr">

## Référence API

**Signature**

```python
scaler = sp.StandardScaler(with_mean=True, with_std=True)
scaler = sp.MinMaxScaler(feature_range=(0.0, 1.0))
scaler = sp.RobustScaler(with_centering=True, with_scaling=True, quantile_range=(25.0, 75.0))
scaler = sp.MaxAbsScaler()
scaler = sp.Normalizer(norm="l2")

scaler.fit(X)
X_scaled  = scaler.transform(X)       -> ndarray
X_back    = scaler.inverse_transform(X_scaled) -> ndarray   # la plupart des scalers
X_scaled  = scaler.fit_transform(X)   -> ndarray
```

**Paramètres du constructeur — StandardScaler**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `with_mean` | `bool` | `True` | Soustraire la moyenne |
| `with_std` | `bool` | `True` | Diviser par l'écart-type |

**Paramètres du constructeur — MinMaxScaler**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `feature_range` | `tuple(float,float)` | `(0.0, 1.0)` | Plage de sortie souhaitée |

**Paramètres du constructeur — RobustScaler**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `with_centering` | `bool` | `True` | Soustraire la médiane |
| `with_scaling` | `bool` | `True` | Diviser par l'IQR |
| `quantile_range` | `tuple(float,float)` | `(25.0, 75.0)` | Plage de quantiles pour l'IQR |

**Paramètres du constructeur — Normalizer**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `norm` | `str` | `"l2"` | Norme par échantillon : `"l1"`, `"l2"`, `"max"` |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `mean_` | `list[float]` | Moyenne par feature (StandardScaler) |
| `scale_` | `list[float]` | Facteur d'échelle par feature |
| `min_` | `list[float]` | Minimum par feature (MinMaxScaler) |
| `data_range_` | `list[float]` | Plage par feature (MinMaxScaler) |
| `center_` | `list[float]` | Médiane par feature (RobustScaler) |
| `max_abs_` | `list[float]` | Valeur absolue maximale par feature (MaxAbsScaler) |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X_train = np.random.randn(300, 4) * [5, 2, 0.1, 100]
X_test  = np.random.randn(50, 4)  * [5, 2, 0.1, 100]

scaler = sp.StandardScaler()
X_tr = scaler.fit_transform(X_train)
X_te = scaler.transform(X_test)
print(f"Moyenne train≈0 : {X_tr.mean(0).round(3)}")

mm = sp.MinMaxScaler(feature_range=(0.0, 1.0))
print(f"Plage MinMax : {mm.fit_transform(X_train).min(0).round(3)}, {mm.fit_transform(X_train).max(0).round(3)}")
```

</details>

---

## Fonctionnement algorithmique

Chaque scaler applique une **transformation linéaire colonne par colonne** ajustée sur l'ensemble d'entraînement et appliquée de manière identique à toute nouvelle donnée.

---

### StandardScaler

Standardise les features à moyenne nulle et variance unitaire :

$$x'_j = \frac{x_j - \mu_j}{\sigma_j}$$

où $\mu_j = \frac{1}{n}\sum_i x_{ij}$ et $\sigma_j = \sqrt{\frac{1}{n}\sum_i (x_{ij}-\mu_j)^2}$.

---

### MinMaxScaler

Redimensionne chaque feature dans l'intervalle $[a, b]$ (`feature_range`) :

$$x'_j = a + \frac{x_j - \min_j}{\max_j - \min_j}(b - a)$$

Sensible aux valeurs aberrantes car il utilise $\min$ et $\max$.

---

### RobustScaler

Utilise la **médiane** et l'**écart interquartile** (IQR), le rendant robuste aux valeurs aberrantes :

$$x'_j = \frac{x_j - Q_{50}(j)}{Q_{75}(j) - Q_{25}(j)}$$

où $Q_p(j)$ est le $p$-ième percentile de la feature $j$.

---

### MaxAbsScaler

Met à l'échelle chaque feature par sa valeur absolue maximale, préservant la sparsité et l'origine :

$$x'_j = \frac{x_j}{\max_i |x_{ij}|}$$

Le résultat est dans $[-1, 1]$.

---

### Normalizer

Met à l'échelle chaque **échantillon** (ligne) à une norme unitaire, appliqué indépendamment du fit :

$$x'_i = \frac{x_i}{\|x_i\|_p}, \qquad p \in \{1, 2, \infty\}$$

Aucune étape `fit` n'est requise — la transformation est sans état.

**Transformation inverse** définie pour StandardScaler, MinMaxScaler, RobustScaler et MaxAbsScaler :

$$x_j = x'_j \cdot \text{scale}_j + \text{center}_j$$

</div>
