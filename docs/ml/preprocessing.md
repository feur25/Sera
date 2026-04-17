# StandardScaler / MinMaxScaler / RobustScaler / MaxAbsScaler / Normalizer

<div class="lang-en">

## Signature

```python
ss   = sp.StandardScaler(with_mean: bool = True, with_std: bool = True)
mm   = sp.MinMaxScaler(feature_range_min: float = 0.0, feature_range_max: float = 1.0)
rs   = sp.RobustScaler(with_centering: bool = True, with_scaling: bool = True, quantile_range: tuple = (25.0, 75.0))
mas  = sp.MaxAbsScaler()
norm = sp.Normalizer(norm: str = "l2")

scaler.fit(x)
scaler.transform(x)       -> ndarray (n, p)
scaler.fit_transform(x)   -> ndarray (n, p)
scaler.inverse_transform(x) -> ndarray (n, p)

# StandardScaler
ss.mean_         -> list[float]
ss.scale_        -> list[float]
ss.with_mean_    -> bool
ss.with_std_     -> bool
ss.var_          -> list[float]

# MinMaxScaler
mm.data_min_     -> list[float]
mm.data_range_   -> list[float]
mm.feature_range_ -> tuple[float, float]
mm.scale_        -> list[float]

# RobustScaler
rs.center_          -> list[float]
rs.scale_           -> list[float]
rs.with_centering_  -> bool
rs.with_scaling_    -> bool
rs.quantile_range_  -> tuple[float, float]
```

---

## Description

Feature scaling and normalization transformers. Parallelized in Rust — transform runs in batches of 2048 rows using Rayon.

- **StandardScaler** — zero mean, unit variance (z-score).
- **MinMaxScaler** — scales features to `[min, max]` range.
- **RobustScaler** — uses median and IQR (configurable via `quantile_range`), robust to outliers.
- **MaxAbsScaler** — scales by the maximum absolute value per feature (no shift).
- **Normalizer** — row-wise normalization (`"l1"`, `"l2"`, or `"max"`).

---

## Constructor Parameters

### StandardScaler

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `with_mean` | `bool` | `True` | Center to zero mean |
| `with_std` | `bool` | `True` | Scale to unit variance |

### MinMaxScaler

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `feature_range_min` | `float` | `0.0` | Target minimum |
| `feature_range_max` | `float` | `1.0` | Target maximum |

### RobustScaler

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `with_centering` | `bool` | `True` | Subtract median |
| `with_scaling` | `bool` | `True` | Divide by IQR |
| `quantile_range` | `tuple` | `(25.0, 75.0)` | Percentile range for IQR calculation |

---

## Examples

<details>
<summary><strong>StandardScaler</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
ss = sp.StandardScaler()
X_scaled = ss.fit_transform(X)
print(f"mean: {ss.mean_[:2]}  scale: {ss.scale_[:2]}")
print(f"var: {ss.var_[:2]}  with_std: {ss.with_std_}")
```

</details>

<details>
<summary><strong>RobustScaler with custom quantile range</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
rs = sp.RobustScaler(quantile_range=(10.0, 90.0))
X_scaled = rs.fit_transform(X)
print(f"quantile_range: {rs.quantile_range_}")
print(f"center: {rs.center_}")
```

</details>
</div>

<div class="lang-fr">

## Signature

```python
ss   = sp.StandardScaler(with_mean: bool = True, with_std: bool = True)
mm   = sp.MinMaxScaler(feature_range_min: float = 0.0, feature_range_max: float = 1.0)
rs   = sp.RobustScaler(with_centering: bool = True, with_scaling: bool = True, quantile_range: tuple = (25.0, 75.0))
mas  = sp.MaxAbsScaler()
norm = sp.Normalizer(norm: str = "l2")
```

---

## Description

Transformateurs de mise à l'échelle et de normalisation des variables. Parallélisés en Rust — la transformation s'effectue par lots de 2048 lignes via Rayon.

- **StandardScaler** — moyenne nulle, variance unitaire (score z).
- **MinMaxScaler** — mise à l'échelle dans l'intervalle `[min, max]`.
- **RobustScaler** — utilise la médiane et l'IQR (configurable via `quantile_range`), robuste aux valeurs aberrantes.
- **MaxAbsScaler** — mise à l'échelle par la valeur absolue maximale (sans décalage).
- **Normalizer** — normalisation par ligne (`"l1"`, `"l2"`, ou `"max"`).

---

## Paramètres du constructeur

### StandardScaler

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `with_mean` | `bool` | `True` | Centrer à la moyenne nulle |
| `with_std` | `bool` | `True` | Mettre à l'échelle à variance unitaire |

### MinMaxScaler

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `feature_range_min` | `float` | `0.0` | Minimum cible |
| `feature_range_max` | `float` | `1.0` | Maximum cible |

### RobustScaler

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `with_centering` | `bool` | `True` | Soustraire la médiane |
| `with_scaling` | `bool` | `True` | Diviser par l'IQR |
| `quantile_range` | `tuple` | `(25.0, 75.0)` | Plage de percentiles pour le calcul de l'IQR |

---

## Exemples

<details>
<summary><strong>StandardScaler</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
ss = sp.StandardScaler()
X_scaled = ss.fit_transform(X)
print(f"moyenne : {ss.mean_[:2]}  échelle : {ss.scale_[:2]}")
print(f"variance : {ss.var_[:2]}  with_std : {ss.with_std_}")
```

</details>

<details>
<summary><strong>RobustScaler avec plage de quantiles personnalisée</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
rs = sp.RobustScaler(quantile_range=(10.0, 90.0))
X_scaled = rs.fit_transform(X)
print(f"quantile_range : {rs.quantile_range_}")
print(f"centre : {rs.center_}")
```

</details>
</div>

mm   = sp.MinMaxScaler(feature_range_min: float = 0.0, feature_range_max: float = 1.0)
rs   = sp.RobustScaler(with_centering: bool = True, with_scaling: bool = True)
mas  = sp.MaxAbsScaler()
norm = sp.Normalizer(norm: str = "l2")

scaler.fit(x)
scaler.transform(x) -> ndarray (n, p)
scaler.fit_transform(x) -> ndarray (n, p)
scaler.inverse_transform(x) -> ndarray (n, p)   # StandardScaler, MinMaxScaler

# StandardScaler
ss.mean_  -> list[float]
ss.scale_ -> list[float]

# MinMaxScaler
mm.data_min_   -> list[float]
mm.data_range_ -> list[float]

# RobustScaler
rs.center_ -> list[float]
rs.scale_  -> list[float]

# MaxAbsScaler
mas.max_abs_ -> list[float]

# Normalizer (stateless — no fit needed)
norm.transform(x) -> ndarray (n, p)
```

---

## Description

Feature scaling and normalization transformers.

- **StandardScaler** — zero mean, unit variance (z-score).
- **MinMaxScaler** — scales features to `[min, max]` range.
- **RobustScaler** — uses median and IQR (Q75 − Q25), robust to outliers.
- **MaxAbsScaler** — scales by the maximum absolute value per feature (no shift).
- **Normalizer** — row-wise normalization (`"l1"`, `"l2"`, or `"max"`).

---

## Constructor Parameters

### StandardScaler

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `with_mean` | `bool` | `True` | Center to zero mean |
| `with_std` | `bool` | `True` | Scale to unit variance |

### MinMaxScaler

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `feature_range_min` | `float` | `0.0` | Target minimum |
| `feature_range_max` | `float` | `1.0` | Target maximum |

### RobustScaler

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `with_centering` | `bool` | `True` | Subtract median |
| `with_scaling` | `bool` | `True` | Divide by IQR |

### MaxAbsScaler

No parameters.

### Normalizer

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `norm` | `str` | `"l2"` | `"l1"`, `"l2"`, or `"max"` |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `mean_` | `list[float]` | Per-feature means (StandardScaler) |
| `scale_` | `list[float]` | Per-feature std devs (StandardScaler) |
| `data_min_` | `list[float]` | Per-feature minimums (MinMaxScaler) |
| `data_range_` | `list[float]` | Per-feature ranges (MinMaxScaler) |
| `center_` | `list[float]` | Per-feature medians (RobustScaler) |
| `scale_` | `list[float]` | Per-feature IQR (RobustScaler) |
| `max_abs_` | `list[float]` | Per-feature max absolute values (MaxAbsScaler) |

---

## Examples

<details>
<summary><strong>StandardScaler</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3) * [10, 0.1, 50] + [100, -5, 0]

scaler = sp.StandardScaler()
X_scaled = np.array(scaler.fit_transform(X))
print(f"Mean after: {X_scaled.mean(axis=0).round(6)}")
print(f"Std after:  {X_scaled.std(axis=0).round(4)}")
```

</details>

<details>
<summary><strong>MinMaxScaler</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(200, 3)

scaler = sp.MinMaxScaler(0.0, 1.0)
X_scaled = np.array(scaler.fit_transform(X))
print(f"Min: {X_scaled.min(axis=0)}")
print(f"Max: {X_scaled.max(axis=0)}")
```

</details>

<details>
<summary><strong>RobustScaler</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 3)
X[0, 0] = 1000  # outlier

scaler = sp.RobustScaler()
X_scaled = np.array(scaler.fit_transform(X))
print(f"Median after: {np.median(X_scaled, axis=0).round(6)}")
```

</details>
