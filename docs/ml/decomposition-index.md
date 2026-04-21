# Decomposition

<div class="lang-en">

Decomposition methods reduce the dimensionality of the data by projecting it onto a lower-dimensional subspace that captures the most variance or structure.

| Method | Description |
|--------|-------------|
| [PCA](decomposition.md) | Principal Component Analysis — finds orthogonal axes of maximum variance |
| [TruncatedSVD](decomposition.md) | Singular Value Decomposition — works directly on sparse matrices |

Both are documented on the same page: [PCA / TruncatedSVD](decomposition.md).

### When to use

| Situation | Method |
|-----------|--------|
| Dense feature matrix | `PCA` |
| Sparse data (e.g., TF-IDF text) | `TruncatedSVD` |
| Visualisation in 2D / 3D | `PCA(n_components=2)` or `(n_components=3)` |
| Noise reduction before modelling | `PCA` with a variance threshold |

### Typical pipeline

```python
import seraplot as sp
import numpy as np

pca = sp.PCA(n_components=2)
X_2d = pca.fit_transform(X)

print(f"Explained variance: {pca.explained_variance_ratio_}")
```

</div>

<div class="lang-fr">

Les méthodes de décomposition réduisent la dimensionnalité des données en les projetant sur un sous-espace de dimension inférieure qui capture le maximum de variance ou de structure.

| Méthode | Description |
|--------|-------------|
| [PCA](decomposition.md) | Analyse en composantes principales — trouve les axes orthogonaux de variance maximale |
| [TruncatedSVD](decomposition.md) | Décomposition en valeurs singulières — fonctionne directement sur les matrices sparses |

Les deux sont documentés sur la même page : [PCA / TruncatedSVD](decomposition.md).

### Quand utiliser

| Situation | Méthode |
|-----------|--------|
| Matrice dense | `PCA` |
| Données sparses (ex. : TF-IDF) | `TruncatedSVD` |
| Visualisation en 2D / 3D | `PCA(n_components=2)` ou `(n_components=3)` |
| Réduction de bruit avant modélisation | `PCA` avec un seuil de variance |

### Pipeline typique

```python
import seraplot as sp
import numpy as np

pca = sp.PCA(n_components=2)
X_2d = pca.fit_transform(X)

print(f"Variance expliquée : {pca.explained_variance_ratio_}")
```

</div>
