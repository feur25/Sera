# StandardScaler / MinMaxScaler / RobustScaler / MaxAbsScaler / Normalizer

## Signature

```python
ss   = sp.StandardScaler(with_mean: bool = True, with_std: bool = True)
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
