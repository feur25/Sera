# Advanced Preprocessing

<div class="lang-en">

## API Reference

**Signatures**

```python
imp  = sp.SimpleImputer(strategy="mean", fill_value=0.0)
poly = sp.PolynomialFeatures(degree=2, interaction_only=False, include_bias=True)
kbd  = sp.KBinsDiscretizer(n_bins=5, strategy="quantile")
pt   = sp.PowerTransformer(method="yeo-johnson")
qt   = sp.QuantileTransformer(n_quantiles=1000, output_distribution="uniform")

ohe  = sp.OneHotEncoder()
ord_ = sp.OrdinalEncoder()

est.fit(X)
Xt = est.transform(X)              -> ndarray
Xt = est.fit_transform(X)          -> ndarray
```

**SimpleImputer**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `strategy` | `str` | `"mean"` | `"mean"`, `"median"`, `"most_frequent"`, `"constant"` |
| `fill_value` | `float` | `0.0` | Value used when strategy is `"constant"` |

Attribute: `statistics_ : list[float]` — fitted per-column value used to fill missing entries (`NaN`/`±inf`).

**PolynomialFeatures**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `degree` | `int` | `2` | Maximum total degree |
| `interaction_only` | `bool` | `False` | Drop pure powers (no $x_i^2$) |
| `include_bias` | `bool` | `True` | Prepend a column of ones |

Attribute: `n_features_out_ : int`, `powers_ : list[list[int]]`.

**KBinsDiscretizer**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_bins` | `int` | `5` | Bins per feature |
| `strategy` | `str` | `"quantile"` | `"uniform"` or `"quantile"` |

Attribute: `bin_edges_ : list[list[float]]`.

**PowerTransformer**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `method` | `str` | `"yeo-johnson"` | `"yeo-johnson"` (any sign) or `"box-cox"` (positive only) |

Attribute: `lambdas_ : list[float]`. Lambda is found by grid-searching $[-2, 2]$ and minimising variance after transform.

**QuantileTransformer**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_quantiles` | `int` | `1000` | Quantile knots |
| `output_distribution` | `str` | `"uniform"` | `"uniform"` or `"normal"` |

Attribute: `quantiles_ : list[list[float]]`.

**OneHotEncoder / OrdinalEncoder**

`fit` / `transform` accept `list[list[Any]]` of strings or numbers; categories are deduced per column.
Attribute: `categories_ : list[list[Any]]`. `OneHotEncoder` exposes `n_features_out_`.

<details>
<summary><strong>Example — full preprocessing pipeline</strong></summary>

```python
import seraplot as sp
import numpy as np

rng = np.random.default_rng(0)
X = rng.normal(size=(500, 4))
X[10, 1] = np.nan
X[42, 3] = np.nan

imp  = sp.SimpleImputer(strategy="median")
poly = sp.PolynomialFeatures(degree=2, interaction_only=True, include_bias=False)
kbd  = sp.KBinsDiscretizer(n_bins=8, strategy="quantile")
pt   = sp.PowerTransformer(method="yeo-johnson")
qt   = sp.QuantileTransformer(n_quantiles=200, output_distribution="normal")

X1 = imp.fit_transform(X)
print("imputed   :", X1.shape, "stats:", imp.statistics_)
X2 = poly.fit_transform(X1)
print("poly      :", X2.shape, "n_out:", poly.n_features_out_)
X3 = kbd.fit_transform(X1)
print("discretised:", X3.shape)
X4 = pt.fit_transform(X1)
print("power     :", X4.shape, "lambdas:", pt.lambdas_)
X5 = qt.fit_transform(X1)
print("quantile  :", X5.shape, "mean ≈ 0:", X5.mean(0).round(2))
```

</details>

<details>
<summary><strong>Example — categorical encoders</strong></summary>

```python
import seraplot as sp

rows = [["cat", "red"], ["dog", "red"], ["cat", "blue"], ["fish", "blue"]]

ohe = sp.OneHotEncoder()
print(ohe.fit_transform(rows))
print(ohe.categories_, ohe.n_features_out_)

oe = sp.OrdinalEncoder()
print(oe.fit_transform(rows))
print(oe.categories_)
```

</details>

---

## Algorithmic Functioning

### SimpleImputer

For each feature $j$, fit a statistic $\theta_j$ over the **non-missing** values
($\mathrm{NaN}$, $+\infty$, $-\infty$ are treated as missing):

<div>$$\theta_j \in \{\mathrm{mean}_j,\; \mathrm{median}_j,\; \mathrm{mode}_j,\; \text{fill\_value}\}$$</div>

`transform` replaces every missing entry with $\theta_j$.

### PolynomialFeatures

Enumerates all monomials $\prod_j x_j^{a_j}$ with $\sum_j a_j \leq d$. With `interaction_only=True`, every $a_j \in \{0, 1\}$ (no pure powers). With `include_bias=True`, the constant 1 column is prepended.

### KBinsDiscretizer

Computes per-feature bin edges:
- **uniform** : $[\min, \max]$ split into $K$ equal-width intervals.
- **quantile** : $[\min, q_{1/K}, q_{2/K}, \dots, \max]$ using sample quantiles.

`transform` returns the integer bin index in $\{0, \dots, K-1\}$.

### PowerTransformer

Applies a parametric monotone transform to make data more Gaussian.

**Yeo-Johnson** (works with any sign):

<div>$$\psi_\lambda(y) = \begin{cases}
\frac{(y+1)^\lambda - 1}{\lambda} & \lambda \neq 0,\; y \geq 0 \\
\log(y+1) & \lambda = 0,\; y \geq 0 \\
-\frac{(-y+1)^{2-\lambda} - 1}{2-\lambda} & \lambda \neq 2,\; y < 0 \\
-\log(-y+1) & \lambda = 2,\; y < 0
\end{cases}$$</div>

**Box-Cox** (requires $y > 0$):

<div>$$\phi_\lambda(y) = \begin{cases}
\frac{y^\lambda - 1}{\lambda} & \lambda \neq 0 \\
\log(y) & \lambda = 0
\end{cases}$$</div>

$\lambda^*$ is selected per feature by a grid search over $[-2, 2]$ minimising the variance of the transformed feature.

### QuantileTransformer

Maps each feature to a uniform $[0, 1]$ distribution via its empirical CDF, then optionally re-maps to $\mathcal{N}(0, 1)$ via the inverse normal CDF (Beasley–Springer–Moro approximation).

### Categorical encoders

`OneHotEncoder` builds the union of observed categories per column and emits one indicator per category. `OrdinalEncoder` assigns each category an integer index in fit-time order.

</div>

<div class="lang-fr">

## Référence API

**Signatures**

```python
imp  = sp.SimpleImputer(strategy="mean", fill_value=0.0)
poly = sp.PolynomialFeatures(degree=2, interaction_only=False, include_bias=True)
kbd  = sp.KBinsDiscretizer(n_bins=5, strategy="quantile")
pt   = sp.PowerTransformer(method="yeo-johnson")
qt   = sp.QuantileTransformer(n_quantiles=1000, output_distribution="uniform")

ohe  = sp.OneHotEncoder()
ord_ = sp.OrdinalEncoder()

est.fit(X)
Xt = est.transform(X)              -> ndarray
Xt = est.fit_transform(X)          -> ndarray
```

**SimpleImputer**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `strategy` | `str` | `"mean"` | `"mean"`, `"median"`, `"most_frequent"`, `"constant"` |
| `fill_value` | `float` | `0.0` | Valeur utilisée si stratégie `"constant"` |

Attribut : `statistics_ : list[float]` — valeur ajustée par colonne pour remplir les entrées manquantes (`NaN`/`±inf`).

**PolynomialFeatures**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `degree` | `int` | `2` | Degré total maximal |
| `interaction_only` | `bool` | `False` | Supprime les puissances pures (pas de $x_i^2$) |
| `include_bias` | `bool` | `True` | Ajoute une colonne de uns en tête |

Attribut : `n_features_out_ : int`, `powers_ : list[list[int]]`.

**KBinsDiscretizer**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_bins` | `int` | `5` | Nombre de classes par feature |
| `strategy` | `str` | `"quantile"` | `"uniform"` ou `"quantile"` |

Attribut : `bin_edges_ : list[list[float]]`.

**PowerTransformer**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `method` | `str` | `"yeo-johnson"` | `"yeo-johnson"` (tout signe) ou `"box-cox"` (positif) |

Attribut : `lambdas_ : list[float]`. Lambda est trouvé par recherche sur grille $[-2, 2]$ minimisant la variance après transformation.

**QuantileTransformer**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_quantiles` | `int` | `1000` | Nœuds de quantiles |
| `output_distribution` | `str` | `"uniform"` | `"uniform"` ou `"normal"` |

Attribut : `quantiles_ : list[list[float]]`.

**OneHotEncoder / OrdinalEncoder**

`fit` / `transform` acceptent `list[list[Any]]` de chaînes ou de nombres ; les catégories sont déduites par colonne.
Attribut : `categories_ : list[list[Any]]`. `OneHotEncoder` expose `n_features_out_`.

<details>
<summary><strong>Exemple — pipeline complet</strong></summary>

```python
import seraplot as sp
import numpy as np

rng = np.random.default_rng(0)
X = rng.normal(size=(500, 4))
X[10, 1] = np.nan
X[42, 3] = np.nan

imp  = sp.SimpleImputer(strategy="median")
poly = sp.PolynomialFeatures(degree=2, interaction_only=True, include_bias=False)
kbd  = sp.KBinsDiscretizer(n_bins=8, strategy="quantile")
pt   = sp.PowerTransformer(method="yeo-johnson")
qt   = sp.QuantileTransformer(n_quantiles=200, output_distribution="normal")

X1 = imp.fit_transform(X)
print("imputé    :", X1.shape, "stats:", imp.statistics_)
X2 = poly.fit_transform(X1)
print("poly      :", X2.shape, "n_out:", poly.n_features_out_)
X3 = kbd.fit_transform(X1)
print("discrétisé:", X3.shape)
X4 = pt.fit_transform(X1)
print("power     :", X4.shape, "lambdas:", pt.lambdas_)
X5 = qt.fit_transform(X1)
print("quantile  :", X5.shape, "moy ≈ 0 :", X5.mean(0).round(2))
```

</details>

<details>
<summary><strong>Exemple — encodeurs catégoriels</strong></summary>

```python
import seraplot as sp

rows = [["cat", "red"], ["dog", "red"], ["cat", "blue"], ["fish", "blue"]]

ohe = sp.OneHotEncoder()
print(ohe.fit_transform(rows))
print(ohe.categories_, ohe.n_features_out_)

oe = sp.OrdinalEncoder()
print(oe.fit_transform(rows))
print(oe.categories_)
```

</details>

---

## Fonctionnement algorithmique

### SimpleImputer

Pour chaque feature $j$, ajuste une statistique $\theta_j$ sur les valeurs **non manquantes**
($\mathrm{NaN}$, $+\infty$, $-\infty$ sont considérés comme manquants) :

<div>$$\theta_j \in \{\mathrm{mean}_j,\; \mathrm{median}_j,\; \mathrm{mode}_j,\; \text{fill\_value}\}$$</div>

`transform` remplace toute entrée manquante par $\theta_j$.

### PolynomialFeatures

Énumère tous les monômes $\prod_j x_j^{a_j}$ avec $\sum_j a_j \leq d$. Avec `interaction_only=True`, chaque $a_j \in \{0, 1\}$ (pas de puissances pures). Avec `include_bias=True`, la colonne constante 1 est ajoutée en tête.

### KBinsDiscretizer

Calcule les bornes de classes par feature :
- **uniform** : $[\min, \max]$ découpé en $K$ intervalles équilarges.
- **quantile** : $[\min, q_{1/K}, q_{2/K}, \dots, \max]$ via les quantiles empiriques.

`transform` renvoie l'index de classe entier dans $\{0, \dots, K-1\}$.

### PowerTransformer

Applique une transformation monotone paramétrique pour rendre les données plus gaussiennes.

**Yeo-Johnson** (tout signe) :

<div>$$\psi_\lambda(y) = \begin{cases}
\frac{(y+1)^\lambda - 1}{\lambda} & \lambda \neq 0,\; y \geq 0 \\
\log(y+1) & \lambda = 0,\; y \geq 0 \\
-\frac{(-y+1)^{2-\lambda} - 1}{2-\lambda} & \lambda \neq 2,\; y < 0 \\
-\log(-y+1) & \lambda = 2,\; y < 0
\end{cases}$$</div>

**Box-Cox** (requiert $y > 0$) :

<div>$$\phi_\lambda(y) = \begin{cases}
\frac{y^\lambda - 1}{\lambda} & \lambda \neq 0 \\
\log(y) & \lambda = 0
\end{cases}$$</div>

$\lambda^*$ est sélectionné par feature via une recherche sur grille dans $[-2, 2]$ minimisant la variance de la feature transformée.

### QuantileTransformer

Mappe chaque feature vers une distribution uniforme $[0, 1]$ via sa CDF empirique, puis optionnellement re-mappe vers $\mathcal{N}(0, 1)$ via la CDF normale inverse (approximation de Beasley–Springer–Moro).

### Encodeurs catégoriels

`OneHotEncoder` construit l'union des catégories observées par colonne et émet un indicateur par catégorie. `OrdinalEncoder` attribue à chaque catégorie un index entier dans l'ordre du fit.

</div>
