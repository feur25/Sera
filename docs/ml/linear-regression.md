# LinearRegression

<div class="lang-en">

## Signature

```python
model = sp.LinearRegression(fit_intercept: bool = True)

model.fit(x, y)
model.predict(x) -> list[float]
model.score(x, y) -> float

model.coef_           -> list[float]
model.intercept_      -> float
model.fit_intercept_  -> bool
model.n_features_in_  -> int
```

---

## Description

Ordinary Least Squares regression solved via Cholesky decomposition (falls back to QR if the Gram matrix is not positive-definite). Pure Rust, no external BLAS.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `fit_intercept` | `bool` | `True` | Add a bias term |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients (length p) |
| `intercept_` | `float` | Bias term (0 if `fit_intercept=False`) |
| `fit_intercept_` | `bool` | Whether a bias was fitted |
| `n_features_in_` | `int` | Number of features seen during fit |

---

## Example

<details>
<summary><strong>Basic regression</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
y = X @ np.array([2.0, -1.0, 0.5]) + np.random.randn(500) * 0.1

model = sp.LinearRegression()
model.fit(X, y)
print(f"R²: {model.score(X, y):.6f}")
print(f"Coefficients: {model.coef_}")
print(f"Features seen: {model.n_features_in_}")
```

</details>
</div>

<div class="lang-fr">

## Signature

```python
model = sp.LinearRegression(fit_intercept: bool = True)

model.fit(x, y)
model.predict(x) -> list[float]
model.score(x, y) -> float

model.coef_           -> list[float]
model.intercept_      -> float
model.fit_intercept_  -> bool
model.n_features_in_  -> int
```

---

## Description

Régression par moindres carrés ordinaires résolue par décomposition de Cholesky (retour sur QR si la matrice de Gram n'est pas définie positive). Implémenté en Rust pur, sans BLAS externe.

---

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `fit_intercept` | `bool` | `True` | Ajouter un terme de biais |

---

## Attributs

| Attribut | Type | Description |
|----------|------|-------------|
| `coef_` | `list[float]` | Coefficients ajustés (longueur p) |
| `intercept_` | `float` | Terme de biais (0 si `fit_intercept=False`) |
| `fit_intercept_` | `bool` | Indique si un biais a été ajusté |
| `n_features_in_` | `int` | Nombre de variables vues lors de l'entraînement |

---

## Exemple

<details>
<summary><strong>Régression de base</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
y = X @ np.array([2.0, -1.0, 0.5]) + np.random.randn(500) * 0.1

model = sp.LinearRegression()
model.fit(X, y)
print(f"R² : {model.score(X, y):.6f}")
print(f"Coefficients : {model.coef_}")
print(f"Variables vues : {model.n_features_in_}")
```

</details>
</div>


model.fit(x, y)
model.predict(x) -> list[float]
model.score(x, y) -> float

model.coef_      -> list[float]
model.intercept_ -> float
```

---

## Description

Ordinary Least Squares regression solved via Cholesky decomposition (falls back to QR if the Gram matrix is not positive-definite). Pure Rust, no external BLAS — runs in microseconds on typical datasets.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `fit_intercept` | `bool` | `True` | Add a bias term |

---

## Methods

### `fit(x, y)`

Fit the model on training data.

| Argument | Type | Description |
|----------|------|-------------|
| `x` | `ndarray (n, p)` | Feature matrix |
| `y` | `ndarray (n,)` | Target values |

### `predict(x) -> list[float]`

Return predictions for each row of `x`.

### `score(x, y) -> float`

Return the R² coefficient of determination.

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `coef_` | `list[float]` | Fitted coefficients (length p) |
| `intercept_` | `float` | Bias term |

---

## Example

<details>
<summary><strong>Basic regression</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 3)
y = X @ np.array([2.0, -1.0, 0.5]) + np.random.randn(500) * 0.1

model = sp.LinearRegression()
model.fit(X, y)

print(f"R²: {model.score(X, y):.6f}")
print(f"Coefficients: {model.coef_}")
print(f"Intercept: {model.intercept_:.6f}")
```

</details>
