# GaussianNB / MultinomialNB / BernoulliNB

## Signature

```python
gnb = sp.GaussianNB(var_smoothing: float = 1e-9)
mnb = sp.MultinomialNB(alpha: float = 1.0)
bnb = sp.BernoulliNB(alpha: float = 1.0, binarize: float = 0.0)

model.fit(x, y)
model.predict(x) -> list[int]
model.predict_proba(x) -> ndarray (n, n_classes)
model.score(x, y) -> float

model.classes_ -> list[int]

# GaussianNB only
gnb.theta_       -> list[list[float]]   # per-class feature means
gnb.var_         -> list[list[float]]   # per-class feature variances
gnb.class_prior_ -> list[float]         # class prior probabilities
```

---

## Description

Naive Bayes classifiers assuming feature independence given the class.

- **GaussianNB** — continuous features, models each feature as a normal distribution per class.
- **MultinomialNB** — count features (e.g. word counts), with Laplace smoothing (`alpha`).
- **BernoulliNB** — binary features. Continuous values are binarized at threshold `binarize`.

---

## Constructor Parameters

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
