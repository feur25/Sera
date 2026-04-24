# GaussianNB / MultinomialNB / BernoulliNB

<div class="lang-en">

## API Reference

**Signature**

```python
clf = sp.GaussianNB(var_smoothing=1e-9)
clf = sp.MultinomialNB(alpha=1.0)
clf = sp.BernoulliNB(alpha=1.0)

model.fit(X, y)
model.predict(X)               -> list[int]
model.predict_proba(X)         -> ndarray (n, K)
model.score(X, y)              -> float
model.get_params()             -> dict
model.set_params(var_smoothing=...) | set_params(alpha=...)
```

**Constructor parameters — GaussianNB**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `var_smoothing` | `float` | `1e-9` | Fraction of the largest variance added to all variances for stability |

**Constructor parameters — MultinomialNB**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | Laplace/Lidstone smoothing parameter |

**Constructor parameters — BernoulliNB**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `alpha` | `float` | `1.0` | Laplace/Lidstone smoothing parameter |

**Attributes (all variants)**

| Attribute | Type | Description |
|-----------|------|-------------|
| `classes_` | `list[int]` | Unique class labels |
| `class_prior_` | `list[float]` | Prior probability $P(y=k)$ per class |
| `class_count_` | `list[float]` | Number of training samples per class |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 6)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

gnb = sp.GaussianNB()
gnb.fit(X, y)
print(f"GaussianNB accuracy: {gnb.score(X, y):.4f}")

X_counts = np.random.randint(0, 10, (500, 6)).astype(float)
mnb = sp.MultinomialNB(alpha=1.0)
mnb.fit(X_counts, y)
print(f"MultinomialNB accuracy: {mnb.score(X_counts, y):.4f}")

X_bin = (X > 0).astype(float)
bnb = sp.BernoulliNB(alpha=1.0)
bnb.fit(X_bin, y)
print(f"BernoulliNB accuracy: {bnb.score(X_bin, y):.4f}")
```

</details>

---

## Algorithmic Functioning

All three variants apply **Bayes' theorem** with class-conditional independence:

<div>$$\hat{y} = \underset{k}{\arg\max}\; P(y=k) \prod_{j=1}^p P(x_j \mid y=k)$$</div>

The three models differ only in how $P(x_j \mid y=k)$ is modelled.

---

### GaussianNB — continuous features

Assumes each feature is Gaussian within each class. Parameters are estimated from training data:

<div>$$\mu_{kj} = \frac{1}{n_k}\sum_{i: y_i=k} x_{ij}, \qquad \sigma^2_{kj} = \frac{1}{n_k}\sum_{i: y_i=k}(x_{ij} - \mu_{kj})^2 + \varepsilon_{\text{smooth}}$$</div>

where $\varepsilon_{\text{smooth}} = \texttt{var\_smoothing} \cdot \max_j \hat{\sigma}^2_j$ prevents zero variances.

Likelihood:

<div>$$P(x_j \mid y=k) = \frac{1}{\sqrt{2\pi\sigma^2_{kj}}} \exp\!\left(-\frac{(x_j - \mu_{kj})^2}{2\sigma^2_{kj}}\right)$$</div>

---

### MultinomialNB — count features

Designed for count data (e.g. word frequencies). Feature conditional is a **multinomial** distribution:

<div>$$P(x_j \mid y=k) = \frac{N_{kj} + \alpha}{N_k + \alpha p}$$</div>

where $N_{kj} = \sum_{i:y_i=k} x_{ij}$ is the total count of feature $j$ in class $k$, $N_k = \sum_j N_{kj}$, and $\alpha$ is the Laplace smoothing term.

---

### BernoulliNB — binary features

Designed for binary/boolean feature vectors. For each feature $j$:

<div>$$P(x_j = 1 \mid y=k) = \frac{N_{kj} + \alpha}{n_k + 2\alpha}$$</div>

and the likelihood explicitly accounts for absent features:

<div>$$P(x_j \mid y=k) = P(x_j=1 \mid y=k)^{x_j}\cdot\bigl(1 - P(x_j=1 \mid y=k)\bigr)^{1-x_j}$$</div>

All three variants compute the final log-probability in log-space to avoid underflow:

<div>$$\log P(y=k \mid x) \propto \log P(y=k) + \sum_{j=1}^p \log P(x_j \mid y=k)$$</div>

</div>

<div class="lang-fr">

## Référence API

**Signature**

```python
clf = sp.GaussianNB(var_smoothing=1e-9)
clf = sp.MultinomialNB(alpha=1.0)
clf = sp.BernoulliNB(alpha=1.0)

model.fit(X, y)
model.predict(X)               -> list[int]
model.predict_proba(X)         -> ndarray (n, K)
model.score(X, y)              -> float
model.get_params()             -> dict
model.set_params(var_smoothing=...) | set_params(alpha=...)
```

**Paramètres du constructeur — GaussianNB**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `var_smoothing` | `float` | `1e-9` | Fraction de la plus grande variance ajoutée à toutes les variances pour la stabilité |

**Paramètres du constructeur — MultinomialNB**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `alpha` | `float` | `1.0` | Paramètre de lissage Laplace/Lidstone |

**Paramètres du constructeur — BernoulliNB**

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `alpha` | `float` | `1.0` | Paramètre de lissage Laplace/Lidstone |

**Attributs (toutes variantes)**

| Attribut | Type | Description |
|----------|------|-------------|
| `classes_` | `list[int]` | Labels de classes uniques |
| `class_prior_` | `list[float]` | Probabilité a priori $P(y=k)$ par classe |
| `class_count_` | `list[float]` | Nombre d'échantillons d'entraînement par classe |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 6)
y = (X[:, 0] + X[:, 1] > 0).astype(int)

gnb = sp.GaussianNB()
gnb.fit(X, y)
print(f"Précision GaussianNB : {gnb.score(X, y):.4f}")

X_counts = np.random.randint(0, 10, (500, 6)).astype(float)
mnb = sp.MultinomialNB(alpha=1.0)
mnb.fit(X_counts, y)
print(f"Précision MultinomialNB : {mnb.score(X_counts, y):.4f}")

X_bin = (X > 0).astype(float)
bnb = sp.BernoulliNB(alpha=1.0)
bnb.fit(X_bin, y)
print(f"Précision BernoulliNB : {bnb.score(X_bin, y):.4f}")
```

</details>

---

## Fonctionnement algorithmique

Les trois variantes appliquent le **théorème de Bayes** avec indépendance conditionnelle aux classes :

<div>$$\hat{y} = \underset{k}{\arg\max}\; P(y=k) \prod_{j=1}^p P(x_j \mid y=k)$$</div>

Les trois modèles diffèrent uniquement dans la façon dont $P(x_j \mid y=k)$ est modélisé.

---

### GaussianNB — features continues

Suppose que chaque feature suit une loi gaussienne au sein de chaque classe. Les paramètres sont estimés à partir des données d'entraînement :

<div>$$\mu_{kj} = \frac{1}{n_k}\sum_{i: y_i=k} x_{ij}, \qquad \sigma^2_{kj} = \frac{1}{n_k}\sum_{i: y_i=k}(x_{ij} - \mu_{kj})^2 + \varepsilon_{\text{smooth}}$$</div>

où $\varepsilon_{\text{smooth}} = \texttt{var\_smoothing} \cdot \max_j \hat{\sigma}^2_j$ évite les variances nulles.

Vraisemblance :

<div>$$P(x_j \mid y=k) = \frac{1}{\sqrt{2\pi\sigma^2_{kj}}} \exp\!\left(-\frac{(x_j - \mu_{kj})^2}{2\sigma^2_{kj}}\right)$$</div>

---

### MultinomialNB — features de comptage

Conçu pour les données de comptage (par ex. fréquences de mots). La conditionnelle de feature est une distribution **multinomiale** :

<div>$$P(x_j \mid y=k) = \frac{N_{kj} + \alpha}{N_k + \alpha p}$$</div>

où $N_{kj} = \sum_{i:y_i=k} x_{ij}$ est le comptage total de la feature $j$ dans la classe $k$, $N_k = \sum_j N_{kj}$, et $\alpha$ est le terme de lissage de Laplace.

---

### BernoulliNB — features binaires

Conçu pour les vecteurs de features binaires/booléens. Pour chaque feature $j$ :

<div>$$P(x_j = 1 \mid y=k) = \frac{N_{kj} + \alpha}{n_k + 2\alpha}$$</div>

et la vraisemblance prend explicitement en compte les features absentes :

<div>$$P(x_j \mid y=k) = P(x_j=1 \mid y=k)^{x_j}\cdot\bigl(1 - P(x_j=1 \mid y=k)\bigr)^{1-x_j}$$</div>

Les trois variantes calculent la log-probabilité finale dans l'espace logarithmique pour éviter le sous-dépassement :

<div>$$\log P(y=k \mid x) \propto \log P(y=k) + \sum_{j=1}^p \log P(x_j \mid y=k)$$</div>

</div>
