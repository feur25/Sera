# PCA / TruncatedSVD

<div class="lang-en">

## Signature

```python
pca  = sp.PCA(n_components: int = 2, svd_solver: str = "auto", whiten: bool = False)
tsvd = sp.TruncatedSVD(n_components: int = 2)

model.fit(x)
model.transform(x)           -> ndarray (n, k)
model.fit_transform(x)       -> ndarray (n, k)
model.inverse_transform(x)   -> ndarray (n, p)   # PCA only

pca.components_               -> list[list[float]]
pca.explained_variance_       -> list[float]
pca.explained_variance_ratio_ -> list[float]
pca.singular_values_          -> list[float]
pca.mean_                     -> list[float]
pca.n_components_             -> int
pca.whiten_                   -> bool
pca.svd_solver_               -> str

tsvd.components_              -> list[list[float]]
tsvd.explained_variance_      -> list[float]
tsvd.explained_variance_ratio_ -> list[float]
tsvd.singular_values_         -> list[float]
tsvd.n_components_            -> int
```

---

## Description

Dimensionality reduction via matrix decomposition.

- **PCA** — centers data then computes SVD. Components ordered by explained variance. Supports optional whitening (`whiten=True`) that rescales each component by `1/sqrt(variance)` — useful before clustering or classification.
- **TruncatedSVD** — same decomposition without centering. Useful for sparse data.

`svd_solver` options: `"auto"` (heuristic), `"randomized"` (fast, approximate), `"full"` (exact).

---

## Constructor Parameters

### PCA

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_components` | `int` | `2` | Number of components to keep |
| `svd_solver` | `str` | `"auto"` | `"auto"`, `"full"`, `"randomized"` |
| `whiten` | `bool` | `False` | Normalize components to unit variance |

### TruncatedSVD

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_components` | `int` | `2` | Number of components to keep |

---

## Examples

<details>
<summary><strong>PCA with whitening</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 10)

pca = sp.PCA(n_components=3, whiten=True, svd_solver="full")
X_reduced = pca.fit_transform(X)
print(f"Shape: {X_reduced.shape}")
print(f"Solver: {pca.svd_solver_}  whiten: {pca.whiten_}")
print(f"Explained variance ratio: {[f'{v:.4f}' for v in pca.explained_variance_ratio_]}")
```

</details>

<details>
<summary><strong>TruncatedSVD</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 20)
tsvd = sp.TruncatedSVD(n_components=5)
X_reduced = tsvd.fit_transform(X)
print(f"Shape: {X_reduced.shape}  n_components: {tsvd.n_components_}")
```

</details>
</div>

<div class="lang-fr">

## Signature

```python
pca  = sp.PCA(n_components: int = 2, svd_solver: str = "auto", whiten: bool = False)
tsvd = sp.TruncatedSVD(n_components: int = 2)
```

---

## Description

Réduction de dimensionnalité par décomposition matricielle.

- **PCA** — centre les données puis calcule la SVD tronquée. Les composantes sont ordonnées par variance expliquée. Supporte le blanchiment optionnel (`whiten=True`) qui redimensionne chaque composante par `1/sqrt(variance)`.
- **TruncatedSVD** — même décomposition sans centrage. Utile pour les données creuses.

Options pour `svd_solver` : `"auto"` (heuristique), `"randomized"` (rapide, approximatif), `"full"` (exact).

---

## Paramètres du constructeur

### PCA

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_components` | `int` | `2` | Nombre de composantes à conserver |
| `svd_solver` | `str` | `"auto"` | `"auto"`, `"full"`, `"randomized"` |
| `whiten` | `bool` | `False` | Normaliser les composantes à variance unitaire |

### TruncatedSVD

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_components` | `int` | `2` | Nombre de composantes à conserver |

---

## Exemples

<details>
<summary><strong>ACP avec blanchiment</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 10)

pca = sp.PCA(n_components=3, whiten=True, svd_solver="full")
X_reduit = pca.fit_transform(X)
print(f"Forme : {X_reduit.shape}")
print(f"Solveur : {pca.svd_solver_}  blanchiment : {pca.whiten_}")
print(f"Ratio variance expliquée : {[f'{v:.4f}' for v in pca.explained_variance_ratio_]}")
```

</details>

<details>
<summary><strong>SVD Tronquée</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 20)
tsvd = sp.TruncatedSVD(n_components=5)
X_reduit = tsvd.fit_transform(X)
print(f"Forme : {X_reduit.shape}  n_composantes : {tsvd.n_components_}")
```

</details>
</div>

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
