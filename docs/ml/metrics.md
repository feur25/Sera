# Metrics

<div class="lang-en">

## API Reference

**Signature**

```python
sp.accuracy_score(y_true, y_pred)               -> float
sp.mean_squared_error(y_true, y_pred)           -> float
sp.mean_absolute_error(y_true, y_pred)          -> float
sp.r2_score(y_true, y_pred)                     -> float
sp.classification_report(y_true, y_pred)        -> str
sp.confusion_matrix(y_true, y_pred)             -> list[list[int]]
```

**Function signatures**

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| `accuracy_score` | `y_true, y_pred: list[int]` | `float` | Fraction of correct predictions |
| `mean_squared_error` | `y_true, y_pred: list[float]` | `float` | Average squared error |
| `mean_absolute_error` | `y_true, y_pred: list[float]` | `float` | Average absolute error |
| `r2_score` | `y_true, y_pred: list[float]` | `float` | Coefficient of determination |
| `classification_report` | `y_true, y_pred: list[int]` | `str` | Per-class precision / recall / F1 table |
| `confusion_matrix` | `y_true, y_pred: list[int]` | `list[list[int]]` | $K \times K$ confusion matrix |

<details>
<summary><strong>Classification metrics example</strong></summary>

```python
import seraplot as sp

y_true = [0, 0, 1, 1, 2, 2, 2]
y_pred = [0, 1, 1, 1, 2, 0, 2]

print(f"Accuracy: {sp.accuracy_score(y_true, y_pred):.4f}")
print(sp.classification_report(y_true, y_pred))
print(sp.confusion_matrix(y_true, y_pred))
```

</details>

<details>
<summary><strong>Regression metrics example</strong></summary>

```python
import seraplot as sp
import numpy as np

y_true = [3.0, -0.5, 2.0, 7.0]
y_pred = [2.5, 0.0, 2.1, 7.8]

print(f"MSE: {sp.mean_squared_error(y_true, y_pred):.4f}")
print(f"MAE: {sp.mean_absolute_error(y_true, y_pred):.4f}")
print(f"R²:  {sp.r2_score(y_true, y_pred):.4f}")
```

</details>

---

## Algorithmic Functioning

---

### Classification metrics

**Accuracy** — fraction of predictions that match the true label:

<div>$$\text{Accuracy} = \frac{1}{n}\sum_{i=1}^n \mathbf{1}[\hat{y}_i = y_i]$$</div>

**Confusion matrix** — $K \times K$ matrix where entry $(k, j)$ is the number of samples of true class $k$ predicted as class $j$:

<div>$$C_{kj} = |\{i : y_i = k,\; \hat{y}_i = j\}|$$</div>

**Per-class metrics** derived from the confusion matrix (TP, FP, FN per class $k$):

<div>$$\text{Precision}_k = \frac{C_{kk}}{\sum_j C_{jk}}, \qquad \text{Recall}_k = \frac{C_{kk}}{\sum_j C_{kj}}$$</div>

<div>$$\text{F1}_k = 2 \cdot \frac{\text{Precision}_k \cdot \text{Recall}_k}{\text{Precision}_k + \text{Recall}_k}$$</div>

`classification_report` aggregates these per class and also reports **macro** (unweighted mean) and **weighted** (weighted by support) averages.

---

### Regression metrics

**Mean Squared Error (MSE):**

<div>$$\text{MSE} = \frac{1}{n}\sum_{i=1}^n (y_i - \hat{y}_i)^2$$</div>

**Mean Absolute Error (MAE):**

<div>$$\text{MAE} = \frac{1}{n}\sum_{i=1}^n |y_i - \hat{y}_i|$$</div>

MAE is less sensitive to outliers than MSE since it uses $|\cdot|$ instead of $(\cdot)^2$.

**$R^2$ score** (coefficient of determination) — proportion of variance explained by the model:

<div>$$R^2 = 1 - \frac{\sum_i (y_i - \hat{y}_i)^2}{\sum_i (y_i - \bar{y})^2}, \qquad \bar{y} = \frac{1}{n}\sum_i y_i$$</div>

$R^2 = 1$ means a perfect fit; $R^2 = 0$ means the model predicts the mean; $R^2 < 0$ means worse than the mean predictor.

</div>

<div class="lang-fr">

## Référence API

**Signature**

```python
sp.accuracy_score(y_true, y_pred)               -> float
sp.mean_squared_error(y_true, y_pred)           -> float
sp.mean_absolute_error(y_true, y_pred)          -> float
sp.r2_score(y_true, y_pred)                     -> float
sp.classification_report(y_true, y_pred)        -> str
sp.confusion_matrix(y_true, y_pred)             -> list[list[int]]
```

**Signatures des fonctions**

| Fonction | Entrée | Sortie | Description |
|----------|--------|--------|-------------|
| `accuracy_score` | `y_true, y_pred: list[int]` | `float` | Fraction de prédictions correctes |
| `mean_squared_error` | `y_true, y_pred: list[float]` | `float` | Erreur quadratique moyenne |
| `mean_absolute_error` | `y_true, y_pred: list[float]` | `float` | Erreur absolue moyenne |
| `r2_score` | `y_true, y_pred: list[float]` | `float` | Coefficient de détermination |
| `classification_report` | `y_true, y_pred: list[int]` | `str` | Tableau précision / rappel / F1 par classe |
| `confusion_matrix` | `y_true, y_pred: list[int]` | `list[list[int]]` | Matrice de confusion $K \times K$ |

<details>
<summary><strong>Exemple métriques de classification</strong></summary>

```python
import seraplot as sp

y_true = [0, 0, 1, 1, 2, 2, 2]
y_pred = [0, 1, 1, 1, 2, 0, 2]

print(f"Précision : {sp.accuracy_score(y_true, y_pred):.4f}")
print(sp.classification_report(y_true, y_pred))
print(sp.confusion_matrix(y_true, y_pred))
```

</details>

<details>
<summary><strong>Exemple métriques de régression</strong></summary>

```python
import seraplot as sp
import numpy as np

y_true = [3.0, -0.5, 2.0, 7.0]
y_pred = [2.5, 0.0, 2.1, 7.8]

print(f"MSE : {sp.mean_squared_error(y_true, y_pred):.4f}")
print(f"MAE : {sp.mean_absolute_error(y_true, y_pred):.4f}")
print(f"R² :  {sp.r2_score(y_true, y_pred):.4f}")
```

</details>

---

## Fonctionnement algorithmique

---

### Métriques de classification

**Exactitude** — fraction des prédictions qui correspondent à la vraie étiquette :

<div>$$\text{Exactitude} = \frac{1}{n}\sum_{i=1}^n \mathbf{1}[\hat{y}_i = y_i]$$</div>

**Matrice de confusion** — matrice $K \times K$ où l'entrée $(k, j)$ est le nombre d'échantillons de la vraie classe $k$ prédits comme classe $j$ :

<div>$$C_{kj} = |\{i : y_i = k,\; \hat{y}_i = j\}|$$</div>

**Métriques par classe** dérivées de la matrice de confusion (VP, FP, FN par classe $k$) :

<div>$$\text{Précision}_k = \frac{C_{kk}}{\sum_j C_{jk}}, \qquad \text{Rappel}_k = \frac{C_{kk}}{\sum_j C_{kj}}$$</div>

<div>$$\text{F1}_k = 2 \cdot \frac{\text{Précision}_k \cdot \text{Rappel}_k}{\text{Précision}_k + \text{Rappel}_k}$$</div>

`classification_report` agrège ces métriques par classe et rapporte également les moyennes **macro** (moyenne non pondérée) et **pondérée** (pondérée par le support).

---

### Métriques de régression

**Erreur Quadratique Moyenne (MSE) :**

<div>$$\text{MSE} = \frac{1}{n}\sum_{i=1}^n (y_i - \hat{y}_i)^2$$</div>

**Erreur Absolue Moyenne (MAE) :**

<div>$$\text{MAE} = \frac{1}{n}\sum_{i=1}^n |y_i - \hat{y}_i|$$</div>

La MAE est moins sensible aux valeurs aberrantes que la MSE car elle utilise $|\cdot|$ au lieu de $(\cdot)^2$.

**Score $R^2$** (coefficient de détermination) — proportion de variance expliquée par le modèle :

<div>$$R^2 = 1 - \frac{\sum_i (y_i - \hat{y}_i)^2}{\sum_i (y_i - \bar{y})^2}, \qquad \bar{y} = \frac{1}{n}\sum_i y_i$$</div>

$R^2 = 1$ signifie un ajustement parfait ; $R^2 = 0$ signifie que le modèle prédit la moyenne ; $R^2 < 0$ signifie pire que le prédicteur moyen.

</div>
