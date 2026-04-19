# train_test_split / StratifiedKFold

<div class="lang-en">

## Code

**Signature**

```python
X_train, X_test, y_train, y_test = sp.train_test_split(
    X, y, test_size=0.2, random_state=None, stratify=False
)

kf = sp.StratifiedKFold(n_splits=5, shuffle=True, random_state=None)

for train_idx, test_idx in kf.split(X, y):
    X_train, X_test = X[train_idx], X[test_idx]
    y_train, y_test = y[train_idx], y[test_idx]
    ...
```

**Parameters — train_test_split**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `X` | `ndarray (n, p)` | — | Feature matrix |
| `y` | `list \| ndarray` | — | Target vector |
| `test_size` | `float` | `0.2` | Fraction of samples to hold out |
| `random_state` | `int \| None` | `None` | Seed for reproducibility |
| `stratify` | `bool` | `False` | Preserve class proportions in each split |

**Constructor parameters — StratifiedKFold**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_splits` | `int` | `5` | Number of folds $k$ |
| `shuffle` | `bool` | `True` | Shuffle data before splitting |
| `random_state` | `int \| None` | `None` | Seed for reproducibility |

**Returns — train_test_split**

| Return value | Type | Description |
|--------------|------|-------------|
| `X_train` | `ndarray` | Training features |
| `X_test` | `ndarray` | Test features |
| `y_train` | `list` | Training labels |
| `y_test` | `list` | Test labels |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 6)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

X_train, X_test, y_train, y_test = sp.train_test_split(
    X, y, test_size=0.2, random_state=42, stratify=True
)
print(f"Train: {len(y_train)}, Test: {len(y_test)}")

kf = sp.StratifiedKFold(n_splits=5, random_state=0)
for fold, (tr, te) in enumerate(kf.split(X, y)):
    clf = sp.LogisticRegression()
    clf.fit(X[tr], np.array(y)[tr].tolist())
    print(f"Fold {fold}: {clf.score(X[te], np.array(y)[te].tolist()):.4f}")
```

</details>

---

## Algorithmic Functioning

---

### train_test_split

**Non-stratified split** — randomly shuffle indices and cut at position $\lfloor n \cdot (1 - \texttt{test\_size})\rfloor$:

$$\text{train} = \sigma([0,n))[:n_{\text{tr}}], \qquad \text{test} = \sigma([0,n))[n_{\text{tr}}:]$$

where $\sigma$ is a random permutation seeded by `random_state`.

**Stratified split** — class proportions are preserved by splitting each class independently:

$$\forall k:\quad n_{\text{test},k} = \text{round}(n_k \cdot \texttt{test\_size})$$

then combining and shuffling all per-class test/train sets. This ensures that rare classes are not accidentally excluded from the test set.

---

### StratifiedKFold

Splits the dataset into $k$ non-overlapping folds whilst preserving class distributions in each fold.

**Algorithm:**

**1.** For each class $c$, collect its indices $\mathcal{I}_c = \{i : y_i = c\}$.

**2.** Optionally shuffle $\mathcal{I}_c$ with `random_state`.

**3.** Divide $\mathcal{I}_c$ into $k$ roughly equal sub-arrays of size $\lfloor|\mathcal{I}_c|/k\rfloor$ or $\lceil|\mathcal{I}_c|/k\rceil$.

**4.** For fold $f \in \{0,\ldots,k-1\}$: the **test** set is $\bigcup_c \mathcal{I}_c[f]$ and the **train** set is its complement.

The $f$-th fold test error estimate $\hat{e}_f$ gives the cross-validated score:

$$\widehat{\text{CV}} = \frac{1}{k}\sum_{f=0}^{k-1} \hat{e}_f$$

This estimate has lower variance than a single train/test split, especially for small datasets.

</div>

<div class="lang-fr">

## Code

**Signature**

```python
X_train, X_test, y_train, y_test = sp.train_test_split(
    X, y, test_size=0.2, random_state=None, stratify=False
)

kf = sp.StratifiedKFold(n_splits=5, shuffle=True, random_state=None)

for train_idx, test_idx in kf.split(X, y):
    X_train, X_test = X[train_idx], X[test_idx]
    y_train, y_test = y[train_idx], y[test_idx]
    ...
```

**Paramètres — train_test_split**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `X` | `ndarray (n, p)` | — | Matrice de features |
| `y` | `list \| ndarray` | — | Vecteur cible |
| `test_size` | `float` | `0.2` | Fraction des échantillons à mettre de côté |
| `random_state` | `int \| None` | `None` | Graine pour la reproductibilité |
| `stratify` | `bool` | `False` | Préserver les proportions de classes dans chaque partition |

**Paramètres du constructeur — StratifiedKFold**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `n_splits` | `int` | `5` | Nombre de plis $k$ |
| `shuffle` | `bool` | `True` | Mélanger les données avant de diviser |
| `random_state` | `int \| None` | `None` | Graine pour la reproductibilité |

**Valeurs de retour — train_test_split**

| Valeur de retour | Type | Description |
|------------------|------|-------------|
| `X_train` | `ndarray` | Features d'entraînement |
| `X_test` | `ndarray` | Features de test |
| `y_train` | `list` | Étiquettes d'entraînement |
| `y_test` | `list` | Étiquettes de test |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 6)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

X_train, X_test, y_train, y_test = sp.train_test_split(
    X, y, test_size=0.2, random_state=42, stratify=True
)
print(f"Train : {len(y_train)}, Test : {len(y_test)}")

kf = sp.StratifiedKFold(n_splits=5, random_state=0)
for pli, (tr, te) in enumerate(kf.split(X, y)):
    clf = sp.LogisticRegression()
    clf.fit(X[tr], np.array(y)[tr].tolist())
    print(f"Pli {pli} : {clf.score(X[te], np.array(y)[te].tolist()):.4f}")
```

</details>

---

## Fonctionnement algorithmique

---

### train_test_split

**Division non stratifiée** — mélange aléatoire des indices et coupe à la position $\lfloor n \cdot (1 - \texttt{test\_size})\rfloor$ :

$$\text{train} = \sigma([0,n))[:n_{\text{tr}}], \qquad \text{test} = \sigma([0,n))[n_{\text{tr}}:]$$

où $\sigma$ est une permutation aléatoire initialisée par `random_state`.

**Division stratifiée** — les proportions de classes sont préservées en divisant chaque classe indépendamment :

$$\forall k:\quad n_{\text{test},k} = \text{round}(n_k \cdot \texttt{test\_size})$$

puis en combinant et mélangeant tous les ensembles test/train par classe. Cela garantit que les classes rares ne sont pas accidentellement exclues de l'ensemble de test.

---

### StratifiedKFold

Divise le jeu de données en $k$ plis non-chevauchants tout en préservant les distributions de classes dans chaque pli.

**Algorithme :**

**1.** Pour chaque classe $c$, collecter ses indices $\mathcal{I}_c = \{i : y_i = c\}$.

**2.** Optionnellement mélanger $\mathcal{I}_c$ avec `random_state`.

**3.** Diviser $\mathcal{I}_c$ en $k$ sous-tableaux approximativement égaux de taille $\lfloor|\mathcal{I}_c|/k\rfloor$ ou $\lceil|\mathcal{I}_c|/k\rceil$.

**4.** Pour le pli $f \in \{0,\ldots,k-1\}$ : l'ensemble de **test** est $\bigcup_c \mathcal{I}_c[f]$ et l'ensemble d'**entraînement** est son complément.

L'estimation d'erreur du $f$-ième pli $\hat{e}_f$ donne le score de validation croisée :

$$\widehat{\text{VC}} = \frac{1}{k}\sum_{f=0}^{k-1} \hat{e}_f$$

Cette estimation a une variance plus faible qu'une seule division train/test, notamment pour les petits jeux de données.

</div>
# train_test_split

<div class="lang-en">

## Signature

```python
X_train, X_test, y_train, y_test = sp.train_test_split(
    x,
    y,
    test_size: float = 0.2,
    random_state: int = 42,
)
```

---

## Description

Splits data into training and test sets using a Fisher-Yates shuffle seeded by `random_state` (splitmix64 PRNG). Deterministic for the same seed.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `x` | `ndarray (n, p)` | — | Feature matrix |
| `y` | `ndarray (n,)` | — | Target vector |
| `test_size` | `float` | `0.2` | Fraction of data in the test set |
| `random_state` | `int` | `42` | Random seed |

</div>

<div class="lang-fr">

## Description

Divise les données en ensembles d'entraînement et de test via un mélange Fisher-Yates seeded par `random_state` (PRNG splitmix64). Déterministe pour la même graine.

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `x` | `ndarray (n, p)` | — | Matrice des variables |
| `y` | `ndarray (n,)` | — | Vecteur cible |
| `test_size` | `float` | `0.2` | Fraction des données pour le test |
| `random_state` | `int` | `42` | Graine aléatoire |

</div>


---

## Returns

| Return | Type | Description |
|--------|------|-------------|
| `X_train` | `list[list[float]]` | Training features |
| `X_test` | `list[list[float]]` | Test features |
| `y_train` | `list[float]` | Training targets |
| `y_test` | `list[float]` | Test targets |

---

## Example

<details>
<summary><strong>Basic split</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(1000, 5)
y = (X @ np.random.randn(5)).astype(np.float64)

X_train, X_test, y_train, y_test = sp.train_test_split(X, y, 0.2, 42)

print(f"Train: {len(y_train)} samples")
print(f"Test:  {len(y_test)} samples")
```

</details>

<details>
<summary><strong>Full pipeline</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = (X @ np.random.randn(4)).astype(np.float64)

X_train, X_test, y_train, y_test = sp.train_test_split(X, y, 0.25, 0)

scaler = sp.StandardScaler()
X_train = np.array(scaler.fit_transform(np.array(X_train)))
X_test = np.array(scaler.transform(np.array(X_test)))

model = sp.Ridge(alpha=0.5)
model.fit(X_train, np.array(y_train))

preds = model.predict(X_test)
r2 = sp.r2_score(y_test, preds)
print(f"Test R²: {r2:.4f}")
```

</details>
