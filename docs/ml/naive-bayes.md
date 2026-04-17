# GaussianNB / MultinomialNB / BernoulliNB

<div class="lang-en">

## Signature

```python
gnb = sp.GaussianNB(var_smoothing: float = 1e-9)
mnb = sp.MultinomialNB(alpha: float = 1.0)
bnb = sp.BernoulliNB(alpha: float = 1.0, binarize: float = 0.0)

gnb.theta_          -> list[list[float]]
gnb.var_            -> list[list[float]]
gnb.class_prior_    -> list[float]
gnb.classes_        -> list[int]
gnb.var_smoothing_  -> float

mnb.classes_        -> list[int]
mnb.alpha_          -> float

bnb.classes_        -> list[int]
bnb.alpha_          -> float
bnb.binarize_       -> float
```

---

## Description

Naive Bayes classifiers assuming feature independence given the class.

- **GaussianNB** — continuous features, models each feature as a normal distribution per class.
- **MultinomialNB** — count features (word counts), with Laplace smoothing (`alpha`).
- **BernoulliNB** — binary features. Continuous values binarized at threshold `binarize`.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `var_smoothing` | `float` | `1e-9` | Added to variance for stability (GaussianNB) |
| `alpha` | `float` | `1.0` | Laplace smoothing parameter |
| `binarize` | `float` | `0.0` | Binarization threshold (BernoulliNB) |

</div>

<div class="lang-fr">

## Description

Classificateurs Naive Bayes supposant l'indépendance des variables conditionnellement à la classe.

- **GaussianNB** — variables continues, modélise chaque variable comme une distribution normale par classe.
- **MultinomialNB** — variables de comptage (fréquences de mots), avec lissage de Laplace (`alpha`).
- **BernoulliNB** — variables binaires. Les valeurs continues sont binarisées au seuil `binarize`.

## Paramètres du constructeur

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `var_smoothing` | `float` | `1e-9` | Ajouté à la variance pour la stabilité (GaussianNB) |
| `alpha` | `float` | `1.0` | Paramètre de lissage de Laplace |
| `binarize` | `float` | `0.0` | Seuil de binarisation (BernoulliNB) |

</div>


### GaussianNB

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `var_smoothing` | `float` | `1e-9` | Portion of the largest variance added to all variances for stability |

### MultinomialNB

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | Laplace smoothing |

### BernoulliNB

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | Laplace smoothing |
| `binarize` | `float` | `0.0` | Threshold for binarizing features |

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels |
| `theta_` | `list[list[float]]` | Per-class feature means (GaussianNB) |
| `var_` | `list[list[float]]` | Per-class feature variances (GaussianNB) |
| `class_prior_` | `list[float]` | Class prior probabilities (GaussianNB) |

---

## Examples

<details>
<summary><strong>GaussianNB</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = (X[:, 0] + X[:, 1] > 0).astype(np.int32)

model = sp.GaussianNB()
model.fit(X, y)
print(f"Accuracy: {model.score(X, y):.4f}")
```

</details>

<details>
<summary><strong>MultinomialNB on count data</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randint(0, 10, size=(300, 5)).astype(np.float64)
y = (X[:, 0] + X[:, 1] > 8).astype(np.int32)

model = sp.MultinomialNB(alpha=1.0)
model.fit(X, y)
print(f"Accuracy: {model.score(X, y):.4f}")
```

</details>

<details>
<summary><strong>BernoulliNB with binarization</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(400, 6)
y = ((X[:, 0] > 0) & (X[:, 1] > 0)).astype(np.int32)

model = sp.BernoulliNB(alpha=1.0, binarize=0.0)
model.fit(X, y)
print(f"Accuracy: {model.score(X, y):.4f}")
```

</details>
