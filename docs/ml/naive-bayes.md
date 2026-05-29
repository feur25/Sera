<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>GaussianNb</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-cls">Classifier</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📊 Naive Bayes</span>
      </div>
      <p class="ml-pg-tagline">Gaussian Naive Bayes — likelihood modelled as Gaussian per class per feature. / Naive Bayes Gaussien — vraisemblance modélisée comme Gaussienne par classe et feature.</p>
    </div>
    <div class="ml-pg-badges">
      <span class="ml-pg-badge ml-pg-badge-speed-hi">⚡ Rust-native</span>
      <span class="ml-pg-badge ml-pg-badge-parity-hi">✓ sklearn parity</span>
    </div>
  </div>
</div>

<div class="ml-pg-qs">
  <div class="ml-pg-qs-header">
    <span class="ml-pg-qs-title">Quick start — Python</span>
  </div>

```python
import seraplot as sp
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
gnb = sp.GaussianNB()
gnb.fit(X, y)
print(gnb.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.GaussianNb</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_gaussian_nb` — aliases: `gaussian_nb`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.GaussianNb()
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<p><em>No constructor parameters.</em></p>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$P(x_j | y=c) = \frac{1}{\sqrt{2\pi\sigma_{cj}^2}} \exp\!\left(-\frac{(x_j-\mu_{cj})^2}{2\sigma_{cj}^2}\right)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
gnb = sp.GaussianNB()
gnb.fit(X, y)
print(gnb.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_gaussian_nb` — alias : `gaussian_nb`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.GaussianNb()
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<p><em>Aucun paramètre de constructeur.</em></p>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$P(x_j | y=c) = \frac{1}{\sqrt{2\pi\sigma_{cj}^2}} \exp\!\left(-\frac{(x_j-\mu_{cj})^2}{2\sigma_{cj}^2}\right)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
gnb = sp.GaussianNB()
gnb.fit(X, y)
print(gnb.score(X, y))
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>MultinomialNb</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-cls">Classifier</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📊 Naive Bayes</span>
      </div>
      <p class="ml-pg-tagline">Multinomial Naive Bayes — for count/frequency features (text, bag-of-words). / Naive Bayes Multinomial — pour features de comptage/fréquence (texte, sac de mots).</p>
    </div>
    <div class="ml-pg-badges">
      <span class="ml-pg-badge ml-pg-badge-speed-hi">⚡ Rust-native</span>
      <span class="ml-pg-badge ml-pg-badge-parity-hi">✓ sklearn parity</span>
    </div>
  </div>
</div>

<div class="ml-pg-qs">
  <div class="ml-pg-qs-header">
    <span class="ml-pg-qs-title">Quick start — Python</span>
  </div>

```python
import seraplot as sp, numpy as np
X = np.random.randint(0, 10, size=(300, 5)).astype(float)
y = (X[:, 0] > 5).astype(int)
mnb = sp.MultinomialNB(alpha=1.0)
mnb.fit(X, y)
print(mnb.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.MultinomialNb</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_multinomial_nb` — aliases: `multinomial_nb`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.MultinomialNb(alpha=1.0)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Additive (Laplace) smoothing.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$P(x|y=c) = \frac{N_{cy} + \alpha}{N_c + \alpha p}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randint(0, 10, size=(300, 5)).astype(float)
y = (X[:, 0] > 5).astype(int)
mnb = sp.MultinomialNB(alpha=1.0)
mnb.fit(X, y)
print(mnb.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_multinomial_nb` — alias : `multinomial_nb`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.MultinomialNb(alpha=1.0)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Lissage additif (Laplace).</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$P(x|y=c) = \frac{N_{cy} + \alpha}{N_c + \alpha p}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randint(0, 10, size=(300, 5)).astype(float)
y = (X[:, 0] > 5).astype(int)
mnb = sp.MultinomialNB(alpha=1.0)
mnb.fit(X, y)
print(mnb.score(X, y))
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>BernoulliNb</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-cls">Classifier</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📊 Naive Bayes</span>
      </div>
      <p class="ml-pg-tagline">Bernoulli Naive Bayes — for binary/boolean features. / Naive Bayes Bernoulli — pour features binaires/booléennes.</p>
    </div>
    <div class="ml-pg-badges">
      <span class="ml-pg-badge ml-pg-badge-speed-hi">⚡ Rust-native</span>
      <span class="ml-pg-badge ml-pg-badge-parity-hi">✓ sklearn parity</span>
    </div>
  </div>
</div>

<div class="ml-pg-qs">
  <div class="ml-pg-qs-header">
    <span class="ml-pg-qs-title">Quick start — Python</span>
  </div>

```python
import seraplot as sp, numpy as np
X = (np.random.randn(400, 6) > 0).astype(float)
y = (X[:, 0] & X[:, 1]).astype(int)
bnb = sp.BernoulliNB(alpha=1.0)
bnb.fit(X, y)
print(bnb.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.BernoulliNb</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_bernoulli_nb` — aliases: `bernoulli_nb`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.BernoulliNb(alpha=1.0, binarize=0.0)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Additive (Laplace) smoothing.</td></tr>
<tr><td><code>binarize</code></td><td><code>float</code></td><td><code>0.0</code></td><td>Threshold for binarising features.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$P(x_j|y=c) = p_{cj}^{x_j}(1-p_{cj})^{1-x_j}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = (np.random.randn(400, 6) > 0).astype(float)
y = (X[:, 0] & X[:, 1]).astype(int)
bnb = sp.BernoulliNB(alpha=1.0)
bnb.fit(X, y)
print(bnb.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_bernoulli_nb` — alias : `bernoulli_nb`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.BernoulliNb(alpha=1.0, binarize=0.0)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Lissage additif (Laplace).</td></tr>
<tr><td><code>binarize</code></td><td><code>float</code></td><td><code>0.0</code></td><td>Seuil pour binariser les features.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$P(x_j|y=c) = p_{cj}^{x_j}(1-p_{cj})^{1-x_j}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = (np.random.randn(400, 6) > 0).astype(float)
y = (X[:, 0] & X[:, 1]).astype(int)
bnb = sp.BernoulliNB(alpha=1.0)
bnb.fit(X, y)
print(bnb.score(X, y))
```

</div>

</div>
