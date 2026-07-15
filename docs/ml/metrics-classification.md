# Classification Metrics

<div class="lang-en">

## API Reference

**Signatures**

```python
sp.accuracy_score(y_true, y_pred)                                   -> float
sp.balanced_accuracy_score(y_true, y_pred)                          -> float
sp.precision_score(y_true, y_pred, average="binary", pos_label=1)   -> float
sp.recall_score(y_true, y_pred, average="binary", pos_label=1)      -> float
sp.f1_score(y_true, y_pred, average="binary", pos_label=1)          -> float
sp.fbeta_score(y_true, y_pred, beta=1.0, average="binary", pos_label=1) -> float
sp.jaccard_score(y_true, y_pred, pos_label=1)                       -> float
sp.matthews_corrcoef(y_true, y_pred)                                -> float
sp.cohen_kappa_score(y_true, y_pred)                                -> float
sp.hamming_loss(y_true, y_pred)                                     -> float
sp.zero_one_loss(y_true, y_pred)                                    -> float
sp.confusion_matrix(y_true, y_pred)                                 -> list[list[int]]
sp.classification_report(y_true, y_pred)                            -> str

sp.log_loss(y_true, y_proba, n_classes, eps=1e-15)                  -> float
sp.binary_log_loss(y_true, y_proba, eps=1e-15)                      -> float
sp.brier_score_loss(y_true, y_proba)                                -> float
sp.hinge_loss(y_true, decision)                                     -> float

sp.roc_curve(y_true, y_score, pos_label=1)                          -> (fpr, tpr, thresholds)
sp.roc_auc_score(y_true, y_score)                                   -> float
sp.precision_recall_curve(y_true, y_score, pos_label=1)             -> (precision, recall, thresholds)
sp.average_precision_score(y_true, y_score)                         -> float
```

**Function summary**

| Function | Domain | Output | Description |
|----------|--------|--------|-------------|
| `accuracy_score` | any | `float` | Fraction correct |
| `balanced_accuracy_score` | any | `float` | Mean of per-class recall |
| `precision_score` | binary / multiclass | `float` | TP / (TP+FP) |
| `recall_score` | binary / multiclass | `float` | TP / (TP+FN) |
| `f1_score` | binary / multiclass | `float` | Harmonic mean of P and R |
| `fbeta_score` | binary / multiclass | `float` | F-beta with weight `beta` |
| `jaccard_score` | binary | `float` | Intersection over Union |
| `matthews_corrcoef` | binary | `float` | Phi coefficient (range $[-1,1]$) |
| `cohen_kappa_score` | any | `float` | Agreement vs. chance |
| `hamming_loss` | any | `float` | Fraction of wrong predictions |
| `zero_one_loss` | any | `float` | $1 - \text{accuracy}$ |
| `log_loss` | $K$-class proba | `float` | Cross-entropy |
| `binary_log_loss` | binary proba | `float` | Cross-entropy (binary) |
| `brier_score_loss` | binary proba | `float` | $(\hat{p} - y)^2$ averaged |
| `hinge_loss` | $\pm 1$ labels | `float` | $\max(0, 1 - y \cdot s)$ averaged |
| `roc_curve` | binary score | `(fpr, tpr, thr)` | ROC points |
| `roc_auc_score` | binary score | `float` | Area under ROC |
| `precision_recall_curve` | binary score | `(p, r, thr)` | PR points |
| `average_precision_score` | binary score | `float` | Area under PR curve |

`average` accepts `"binary"`, `"macro"`, `"weighted"`.

<details>
<summary><strong>Example — full classification report</strong></summary>

```python
import seraplot as sp
import numpy as np

rng = np.random.default_rng(0)
y_true = rng.integers(0, 2, size=200).tolist()
y_score = rng.uniform(size=200).tolist()
y_pred  = [1 if s >= 0.5 else 0 for s in y_score]

print("accuracy           :", sp.accuracy_score(y_true, y_pred))
print("balanced_accuracy  :", sp.balanced_accuracy_score(y_true, y_pred))
print("precision          :", sp.precision_score(y_true, y_pred))
print("recall             :", sp.recall_score(y_true, y_pred))
print("f1                 :", sp.f1_score(y_true, y_pred))
print("f2                 :", sp.fbeta_score(y_true, y_pred, beta=2.0))
print("matthews_corrcoef  :", sp.matthews_corrcoef(y_true, y_pred))
print("cohen_kappa        :", sp.cohen_kappa_score(y_true, y_pred))
print("jaccard            :", sp.jaccard_score(y_true, y_pred))
print("hamming_loss       :", sp.hamming_loss(y_true, y_pred))
print("zero_one_loss      :", sp.zero_one_loss(y_true, y_pred))

print("brier              :", sp.brier_score_loss(y_true, y_score))
print("binary_log_loss    :", sp.binary_log_loss(y_true, y_score))
print("roc_auc            :", sp.roc_auc_score(y_true, y_score))
print("average_precision  :", sp.average_precision_score(y_true, y_score))
```

</details>

<details>
<summary><strong>Example — ROC and PR curves</strong></summary>

```python
import seraplot as sp

fpr, tpr, thr = sp.roc_curve(y_true, y_score, pos_label=1)
sp.line(fpr, tpr, title=f"ROC (AUC={sp.roc_auc_score(y_true, y_score):.3f})")

prec, rec, thr = sp.precision_recall_curve(y_true, y_score, pos_label=1)
sp.line(rec, prec, title=f"PR (AP={sp.average_precision_score(y_true, y_score):.3f})")
```

</details>

---

## Algorithmic Functioning

**Accuracy** — fraction of correct predictions:

<div>$$\text{Accuracy} = \frac{1}{n}\sum_{i=1}^n \mathbf{1}[\hat{y}_i = y_i]$$</div>

**Balanced accuracy** — mean per-class recall, robust to class imbalance:

<div>$$\text{BAcc} = \frac{1}{K}\sum_{k=1}^K \text{Recall}_k$$</div>

**Matthews correlation coefficient** (binary) — uses all four cells of the confusion matrix:

<div>$$\text{MCC} = \frac{TP \cdot TN - FP \cdot FN}{\sqrt{(TP+FP)(TP+FN)(TN+FP)(TN+FN)}}$$</div>

**Cohen's kappa** — agreement adjusted for chance, with $p_o$ observed agreement and $p_e$ chance agreement:

<div>$$\kappa = \frac{p_o - p_e}{1 - p_e}$$</div>

**F-beta** generalises F1 by weighting recall $\beta$ times more than precision:

<div>$$F_\beta = (1+\beta^2)\frac{P \cdot R}{\beta^2 P + R}$$</div>

**Jaccard score** (binary) — intersection over union of positive predictions and labels:

<div>$$J = \frac{|y \cap \hat{y}|}{|y \cup \hat{y}|}$$</div>

**Log loss** (cross-entropy) for $K$ classes with predicted probabilities $p_{i,k}$:

<div>$$\mathcal{L} = -\frac{1}{n}\sum_{i=1}^n \sum_{k=1}^K \mathbf{1}[y_i = k] \log p_{i,k}$$</div>

Probabilities are clipped to $[\varepsilon, 1-\varepsilon]$ before the log to avoid $-\infty$.

**Brier score** — mean squared error between predicted probabilities and binary labels:

<div>$$\text{Brier} = \frac{1}{n}\sum_{i=1}^n (\hat{p}_i - y_i)^2$$</div>

**Hinge loss** (margin loss) with labels in $\{-1, +1\}$ and decision values $s_i$:

<div>$$\text{Hinge} = \frac{1}{n}\sum_{i=1}^n \max(0,\; 1 - y_i s_i)$$</div>

**ROC curve / AUC** — sweep all thresholds of $s_i$, plotting FPR vs. TPR; AUC is the area under that curve, equal to the probability that a random positive scores higher than a random negative.

**Precision-Recall curve / Average Precision** — same sweep, plotting Precision vs. Recall; AP is computed as the step-area:

<div>$$\text{AP} = \sum_{k} (R_k - R_{k-1}) \cdot P_k$$</div>

</div>

<div class="lang-fr">

## Référence API

**Signatures**

```python
sp.accuracy_score(y_true, y_pred)                                   -> float
sp.balanced_accuracy_score(y_true, y_pred)                          -> float
sp.precision_score(y_true, y_pred, average="binary", pos_label=1)   -> float
sp.recall_score(y_true, y_pred, average="binary", pos_label=1)      -> float
sp.f1_score(y_true, y_pred, average="binary", pos_label=1)          -> float
sp.fbeta_score(y_true, y_pred, beta=1.0, average="binary", pos_label=1) -> float
sp.jaccard_score(y_true, y_pred, pos_label=1)                       -> float
sp.matthews_corrcoef(y_true, y_pred)                                -> float
sp.cohen_kappa_score(y_true, y_pred)                                -> float
sp.hamming_loss(y_true, y_pred)                                     -> float
sp.zero_one_loss(y_true, y_pred)                                    -> float
sp.confusion_matrix(y_true, y_pred)                                 -> list[list[int]]
sp.classification_report(y_true, y_pred)                            -> str

sp.log_loss(y_true, y_proba, n_classes, eps=1e-15)                  -> float
sp.binary_log_loss(y_true, y_proba, eps=1e-15)                      -> float
sp.brier_score_loss(y_true, y_proba)                                -> float
sp.hinge_loss(y_true, decision)                                     -> float

sp.roc_curve(y_true, y_score, pos_label=1)                          -> (fpr, tpr, thresholds)
sp.roc_auc_score(y_true, y_score)                                   -> float
sp.precision_recall_curve(y_true, y_score, pos_label=1)             -> (precision, recall, thresholds)
sp.average_precision_score(y_true, y_score)                         -> float
```

**Résumé**

| Fonction | Domaine | Sortie | Description |
|----------|---------|--------|-------------|
| `accuracy_score` | tout | `float` | Fraction correcte |
| `balanced_accuracy_score` | tout | `float` | Moyenne du rappel par classe |
| `precision_score` | binaire / multiclasse | `float` | $TP / (TP+FP)$ |
| `recall_score` | binaire / multiclasse | `float` | $TP / (TP+FN)$ |
| `f1_score` | binaire / multiclasse | `float` | Moyenne harmonique de P et R |
| `fbeta_score` | binaire / multiclasse | `float` | F-bêta avec poids `beta` |
| `jaccard_score` | binaire | `float` | Intersection sur union |
| `matthews_corrcoef` | binaire | `float` | Coefficient phi (intervalle $[-1,1]$) |
| `cohen_kappa_score` | tout | `float` | Accord corrigé du hasard |
| `hamming_loss` | tout | `float` | Fraction d'erreurs |
| `zero_one_loss` | tout | `float` | $1 - \text{accuracy}$ |
| `log_loss` | proba $K$ classes | `float` | Entropie croisée |
| `binary_log_loss` | proba binaire | `float` | Entropie croisée (binaire) |
| `brier_score_loss` | proba binaire | `float` | $(\hat{p} - y)^2$ moyen |
| `hinge_loss` | étiquettes $\pm 1$ | `float` | $\max(0, 1 - y \cdot s)$ moyen |
| `roc_curve` | score binaire | `(fpr, tpr, thr)` | Points ROC |
| `roc_auc_score` | score binaire | `float` | Aire sous ROC |
| `precision_recall_curve` | score binaire | `(p, r, thr)` | Points PR |
| `average_precision_score` | score binaire | `float` | Aire sous courbe PR |

`average` accepte `"binary"`, `"macro"`, `"weighted"`.

<details>
<summary><strong>Exemple — rapport de classification complet</strong></summary>

```python
import seraplot as sp
import numpy as np

rng = np.random.default_rng(0)
y_true = rng.integers(0, 2, size=200).tolist()
y_score = rng.uniform(size=200).tolist()
y_pred  = [1 if s >= 0.5 else 0 for s in y_score]

print("accuracy           :", sp.accuracy_score(y_true, y_pred))
print("balanced_accuracy  :", sp.balanced_accuracy_score(y_true, y_pred))
print("precision          :", sp.precision_score(y_true, y_pred))
print("recall             :", sp.recall_score(y_true, y_pred))
print("f1                 :", sp.f1_score(y_true, y_pred))
print("f2                 :", sp.fbeta_score(y_true, y_pred, beta=2.0))
print("matthews_corrcoef  :", sp.matthews_corrcoef(y_true, y_pred))
print("cohen_kappa        :", sp.cohen_kappa_score(y_true, y_pred))
print("jaccard            :", sp.jaccard_score(y_true, y_pred))
print("hamming_loss       :", sp.hamming_loss(y_true, y_pred))
print("zero_one_loss      :", sp.zero_one_loss(y_true, y_pred))

print("brier              :", sp.brier_score_loss(y_true, y_score))
print("binary_log_loss    :", sp.binary_log_loss(y_true, y_score))
print("roc_auc            :", sp.roc_auc_score(y_true, y_score))
print("average_precision  :", sp.average_precision_score(y_true, y_score))
```

</details>

<details>
<summary><strong>Exemple — courbes ROC et PR</strong></summary>

```python
import seraplot as sp

fpr, tpr, thr = sp.roc_curve(y_true, y_score, pos_label=1)
sp.line(fpr, tpr, title=f"ROC (AUC={sp.roc_auc_score(y_true, y_score):.3f})")

prec, rec, thr = sp.precision_recall_curve(y_true, y_score, pos_label=1)
sp.line(rec, prec, title=f"PR (AP={sp.average_precision_score(y_true, y_score):.3f})")
```

</details>

---

## Fonctionnement algorithmique

**Précision (accuracy)** — fraction des prédictions correctes :

<div>$$\text{Accuracy} = \frac{1}{n}\sum_{i=1}^n \mathbf{1}[\hat{y}_i = y_i]$$</div>

**Précision équilibrée** — moyenne du rappel par classe, robuste au déséquilibre :

<div>$$\text{BAcc} = \frac{1}{K}\sum_{k=1}^K \text{Recall}_k$$</div>

**Coefficient de corrélation de Matthews** (binaire) — utilise les quatre cases de la matrice de confusion :

<div>$$\text{MCC} = \frac{TP \cdot TN - FP \cdot FN}{\sqrt{(TP+FP)(TP+FN)(TN+FP)(TN+FN)}}$$</div>

**Kappa de Cohen** — accord corrigé du hasard, avec $p_o$ accord observé et $p_e$ accord aléatoire :

<div>$$\kappa = \frac{p_o - p_e}{1 - p_e}$$</div>

**F-bêta** généralise F1 en pondérant le rappel $\beta$ fois plus que la précision :

<div>$$F_\beta = (1+\beta^2)\frac{P \cdot R}{\beta^2 P + R}$$</div>

**Score de Jaccard** (binaire) — intersection sur union des positifs prédits et labellisés :

<div>$$J = \frac{|y \cap \hat{y}|}{|y \cup \hat{y}|}$$</div>

**Log loss** (entropie croisée) pour $K$ classes avec probabilités $p_{i,k}$ :

<div>$$\mathcal{L} = -\frac{1}{n}\sum_{i=1}^n \sum_{k=1}^K \mathbf{1}[y_i = k] \log p_{i,k}$$</div>

Les probabilités sont clampées à $[\varepsilon, 1-\varepsilon]$ avant le log pour éviter $-\infty$.

**Score de Brier** — erreur quadratique moyenne entre probabilités prédites et labels binaires :

<div>$$\text{Brier} = \frac{1}{n}\sum_{i=1}^n (\hat{p}_i - y_i)^2$$</div>

**Hinge loss** (perte de marge) avec labels dans $\{-1, +1\}$ et valeurs de décision $s_i$ :

<div>$$\text{Hinge} = \frac{1}{n}\sum_{i=1}^n \max(0,\; 1 - y_i s_i)$$</div>

**Courbe ROC / AUC** — balayage de tous les seuils de $s_i$, traçant FPR vs. TPR ; l'AUC est l'aire sous la courbe, égale à la probabilité qu'un positif aléatoire ait un score supérieur à celui d'un négatif aléatoire.

**Courbe Précision-Rappel / Average Precision** — même balayage, traçant Précision vs. Rappel ; AP est l'aire en escalier :

<div>$$\text{AP} = \sum_{k} (R_k - R_{k-1}) \cdot P_k$$</div>

</div>
