# PCA / TruncatedSVD

## Signature

```python
pca  = sp.PCA(n_components: int = 2)
tsvd = sp.TruncatedSVD(n_components: int = 2)

model.fit(x)
model.transform(x) -> list[list[float]]
model.fit_transform(x) -> list[list[float]]
model.inverse_transform(x) -> list[list[float]]  # PCA only

# Attributes
model.components_               -> list[list[float]]
model.explained_variance_       -> list[float]
model.explained_variance_ratio_ -> list[float]
model.singular_values_          -> list[float]
pca.mean_                       -> list[float]
```

---

## Description

Dimensionality reduction via matrix decomposition.

- **PCA** — centers data then computes truncated SVD. Returns principal components ordered by explained variance.
- **TruncatedSVD** — same decomposition without centering. Useful for sparse data or when zero-mean is not desired.

Both use power-iteration SVD (`svd_truncated`) — fast for low `n_components`.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_components` | `int` | `2` | Number of components to keep |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `components_` | `list[list[float]]` | Principal axes (k × p) |
| `explained_variance_` | `list[float]` | Variance explained by each component |
| `explained_variance_ratio_` | `list[float]` | Proportion of total variance |
| `singular_values_` | `list[float]` | Singular values |
| `mean_` | `list[float]` | Per-feature mean (PCA only) |

---

## Examples

<details>
<summary><strong>PCA dimensionality reduction</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 10)

pca = sp.PCA(n_components=3)
X_reduced = np.array(pca.fit_transform(X))
print(f"Shape: {X_reduced.shape}")
print(f"Explained variance ratio: {[f'{v:.4f}' for v in pca.explained_variance_ratio_]}")
print(f"Total explained: {sum(pca.explained_variance_ratio_):.4f}")
```

</details>

<details>
<summary><strong>TruncatedSVD</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(300, 20)

tsvd = sp.TruncatedSVD(n_components=5)
X_reduced = np.array(tsvd.fit_transform(X))
print(f"Shape: {X_reduced.shape}")
print(f"Singular values: {[f'{v:.2f}' for v in tsvd.singular_values_]}")
```

</details>

<details>
<summary><strong>PCA inverse transform</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(200, 6)

pca = sp.PCA(n_components=2)
X_reduced = np.array(pca.fit_transform(X))
X_reconstructed = np.array(pca.inverse_transform(X_reduced))

error = np.mean((X - X_reconstructed) ** 2)
print(f"Reconstruction MSE: {error:.4f}")
```

</details>
