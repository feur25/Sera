# PCA / TruncatedSVD

<div class="lang-en">

## API Reference

**Signature**

```python
pca  = sp.PCA(n_components=2, whiten=False)
tsvd = sp.TruncatedSVD(n_components=2)

model.fit(X)
X_reduced  = model.transform(X)        -> ndarray (n, k)
X_back     = model.inverse_transform(T) -> ndarray (n, p)
X_reduced  = model.fit_transform(X)    -> ndarray (n, k)
model.get_params()                     -> dict
model.set_params(n_components=...)
```

**Constructor parameters — PCA**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_components` | `int` | `2` | Number of principal components to keep |
| `whiten` | `bool` | `False` | Scale components to unit variance |

**Constructor parameters — TruncatedSVD**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_components` | `int` | `2` | Number of singular vectors to compute |

**Attributes**

| Attribute | Type | Description |
|-----------|------|-------------|
| `components_` | `ndarray (k, p)` | Principal axes in feature space |
| `explained_variance_` | `list[float]` | Variance explained per component |
| `explained_variance_ratio_` | `list[float]` | Fraction of total variance per component |
| `singular_values_` | `list[float]` | Singular values of the centred data matrix |
| `mean_` | `list[float]` | Per-feature mean used for centring (PCA only) |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 20)

pca = sp.PCA(n_components=5)
T = pca.fit_transform(X)
print(f"Explained variance ratio: {[f'{v:.3f}' for v in pca.explained_variance_ratio_]}")
print(f"Reduced shape: {T.shape}")   # (400, 5)

tsvd = sp.TruncatedSVD(n_components=5)
T2 = tsvd.fit_transform(X)
print(f"TruncatedSVD shape: {T2.shape}")
```

</details>

---

## Algorithmic Functioning

Both algorithms find low-dimensional **linear projections** that maximise preserved variance.

---

### PCA — Principal Component Analysis

**1. Centre** the data matrix:

<div>$$\tilde{X} = X - \mathbf{1}\mu^\top, \qquad \mu_j = \frac{1}{n}\sum_i x_{ij}$$</div>

**2. Compute** the covariance matrix and its eigendecomposition:

<div>$$C = \frac{1}{n}\tilde{X}^\top\tilde{X} = V \Lambda V^\top$$</div>

where $V \in \mathbb{R}^{p \times p}$ has eigenvectors as columns and $\Lambda = \text{diag}(\lambda_1, \ldots, \lambda_p)$ with $\lambda_1 \geq \cdots \geq \lambda_p \geq 0$.

In practice this is computed via the **economy SVD** of $\tilde{X}$:

<div>$$\tilde{X} = U \Sigma V^\top \implies \lambda_i = \frac{\sigma_i^2}{n}$$</div>

**3. Project** onto the $k$ leading components:

<div>$$T = \tilde{X} V_k, \qquad V_k = V[:, :k]$$</div>

**Whitening** (optional): $T_{\text{white}} = T \cdot \text{diag}(\lambda_1^{-1/2}, \ldots, \lambda_k^{-1/2})$, giving each component unit variance.

**Explained variance ratio:**

<div>$$\text{EVR}_i = \frac{\lambda_i}{\sum_j \lambda_j}$$</div>

**Inverse transform** (approximate reconstruction):

<div>$$\hat{X} = T V_k^\top + \mu^\top$$</div>

---

### TruncatedSVD

Directly computes a **rank-$k$ SVD** without centring, making it suitable for sparse matrices (e.g. TF-IDF):

<div>$$X \approx U_k \Sigma_k V_k^\top$$</div>

Uses a **randomised power iteration** algorithm:

<div>$$Y = (XX^\top)^q X \Omega, \quad \Omega \in \mathbb{R}^{p \times (k + \text{oversampling})}$$</div>

where $q = \lceil\texttt{n\_iter}/2\rceil$ power steps amplify the signal of the top singular vectors. The QR factorisation of $Y$ gives an orthonormal basis, and the final SVD is computed on the reduced system.

**Projection**: $T = X V_k$, with inverse $\hat{X} = T V_k^\top$.

</div>

<div class="lang-fr">

## Référence API

**Signature**

```python
pca  = sp.PCA(n_components=2, whiten=False)
tsvd = sp.TruncatedSVD(n_components=2)

model.fit(X)
X_reduced  = model.transform(X)        -> ndarray (n, k)
X_back     = model.inverse_transform(T) -> ndarray (n, p)
X_reduced  = model.fit_transform(X)    -> ndarray (n, k)
model.get_params()                     -> dict
model.set_params(n_components=...)
```

**Paramètres du constructeur — PCA**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_components` | `int` | `2` | Nombre de composantes principales à conserver |
| `whiten` | `bool` | `False` | Mettre les composantes à variance unitaire |

**Paramètres du constructeur — TruncatedSVD**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_components` | `int` | `2` | Nombre de vecteurs singuliers à calculer |

**Attributs**

| Attribut | Type | Description |
|----------|------|-------------|
| `components_` | `ndarray (k, p)` | Axes principaux dans l'espace des features |
| `explained_variance_` | `list[float]` | Variance expliquée par composante |
| `explained_variance_ratio_` | `list[float]` | Fraction de la variance totale par composante |
| `singular_values_` | `list[float]` | Valeurs singulières de la matrice de données centrée |
| `mean_` | `list[float]` | Moyenne par feature pour le centrage (PCA seulement) |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 20)

pca = sp.PCA(n_components=5)
T = pca.fit_transform(X)
print(f"Ratio de variance expliquée : {[f'{v:.3f}' for v in pca.explained_variance_ratio_]}")
print(f"Forme réduite : {T.shape}")   # (400, 5)

tsvd = sp.TruncatedSVD(n_components=5)
T2 = tsvd.fit_transform(X)
print(f"Forme TruncatedSVD : {T2.shape}")
```

</details>

---

## Fonctionnement algorithmique

Les deux algorithmes trouvent des **projections linéaires** de faible dimension qui maximisent la variance préservée.

---

### PCA — Analyse en Composantes Principales

**1. Centrer** la matrice de données :

<div>$$\tilde{X} = X - \mathbf{1}\mu^\top, \qquad \mu_j = \frac{1}{n}\sum_i x_{ij}$$</div>

**2. Calculer** la matrice de covariance et sa décomposition propre :

<div>$$C = \frac{1}{n}\tilde{X}^\top\tilde{X} = V \Lambda V^\top$$</div>

où $V \in \mathbb{R}^{p \times p}$ a les vecteurs propres en colonnes et $\Lambda = \text{diag}(\lambda_1, \ldots, \lambda_p)$ avec $\lambda_1 \geq \cdots \geq \lambda_p \geq 0$.

En pratique, cela est calculé via la **SVD économique** de $\tilde{X}$ :

<div>$$\tilde{X} = U \Sigma V^\top \implies \lambda_i = \frac{\sigma_i^2}{n}$$</div>

**3. Projeter** sur les $k$ composantes principales :

<div>$$T = \tilde{X} V_k, \qquad V_k = V[:, :k]$$</div>

**Blanchiment** (optionnel) : $T_{\text{blanc}} = T \cdot \text{diag}(\lambda_1^{-1/2}, \ldots, \lambda_k^{-1/2})$, donnant à chaque composante une variance unitaire.

**Ratio de variance expliquée :**

<div>$$\text{EVR}_i = \frac{\lambda_i}{\sum_j \lambda_j}$$</div>

**Transformation inverse** (reconstruction approchée) :

<div>$$\hat{X} = T V_k^\top + \mu^\top$$</div>

---

### TruncatedSVD

Calcule directement une **SVD de rang $k$** sans centrage, la rendant adaptée aux matrices creuses (ex. TF-IDF) :

<div>$$X \approx U_k \Sigma_k V_k^\top$$</div>

Utilise un algorithme d'**itération de puissance randomisée** :

<div>$$Y = (XX^\top)^q X \Omega, \quad \Omega \in \mathbb{R}^{p \times (k + \text{sursampling})}$$</div>

où $q = \lceil\texttt{n\_iter}/2\rceil$ étapes de puissance amplifient le signal des vecteurs singuliers principaux. La factorisation QR de $Y$ fournit une base orthonormale, et la SVD finale est calculée sur le système réduit.

**Projection** : $T = X V_k$, avec inverse $\hat{X} = T V_k^\top$.

</div>
