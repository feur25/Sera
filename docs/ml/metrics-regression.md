# Regression Metrics

<div class="lang-en">

## API Reference

**Signatures**

```python
sp.mean_squared_error(y_true, y_pred)                       -> float
sp.root_mean_squared_error(y_true, y_pred)                  -> float
sp.mean_absolute_error(y_true, y_pred)                      -> float
sp.median_absolute_error(y_true, y_pred)                    -> float
sp.r2_score(y_true, y_pred)                                 -> float
sp.explained_variance_score(y_true, y_pred)                 -> float
sp.max_error(y_true, y_pred)                                -> float
sp.mean_absolute_percentage_error(y_true, y_pred)           -> float
sp.mean_squared_log_error(y_true, y_pred)                   -> float
sp.root_mean_squared_log_error(y_true, y_pred)              -> float
sp.mean_pinball_loss(y_true, y_pred, alpha=0.5)             -> float
sp.d2_absolute_error_score(y_true, y_pred)                  -> float
```

**Function summary**

| Function | Output | Description |
|----------|--------|-------------|
| `mean_squared_error` | `float` | Average squared error |
| `root_mean_squared_error` | `float` | $\sqrt{\text{MSE}}$, in target units |
| `mean_absolute_error` | `float` | Average absolute error |
| `median_absolute_error` | `float` | Median of $|y - \hat{y}|$, very robust |
| `r2_score` | `float` | Coefficient of determination |
| `explained_variance_score` | `float` | Variance ratio (allows bias) |
| `max_error` | `float` | Worst residual |
| `mean_absolute_percentage_error` | `float` | MAPE, scale-free |
| `mean_squared_log_error` | `float` | MSE in log space, requires $y, \hat{y} \geq 0$ |
| `root_mean_squared_log_error` | `float` | $\sqrt{\text{MSLE}}$ |
| `mean_pinball_loss` | `float` | Quantile loss (param `alpha` in $(0,1)$) |
| `d2_absolute_error_score` | `float` | $R^2$ analogue using MAE |

<details>
<summary><strong>Example</strong></summary>

```python
import seraplot as sp

y_true = [3.0, -0.5, 2.0, 7.0, 5.0, 4.5]
y_pred = [2.5, 0.0, 2.1, 7.8, 4.7, 4.6]

print("MSE   :", sp.mean_squared_error(y_true, y_pred))
print("RMSE  :", sp.root_mean_squared_error(y_true, y_pred))
print("MAE   :", sp.mean_absolute_error(y_true, y_pred))
print("MedAE :", sp.median_absolute_error(y_true, y_pred))
print("R²    :", sp.r2_score(y_true, y_pred))
print("EVS   :", sp.explained_variance_score(y_true, y_pred))
print("MaxE  :", sp.max_error(y_true, y_pred))
print("MAPE  :", sp.mean_absolute_percentage_error(y_true, y_pred))
print("MSLE  :", sp.mean_squared_log_error([1,2,3], [1.1,2.1,3.1]))
print("Q90   :", sp.mean_pinball_loss(y_true, y_pred, alpha=0.9))
print("D²-AE :", sp.d2_absolute_error_score(y_true, y_pred))
```

</details>

---

## Algorithmic Functioning

**MSE / RMSE / MAE** — pointwise error aggregates:

<div>$$\text{MSE} = \frac{1}{n}\sum_i (y_i - \hat{y}_i)^2 \qquad \text{MAE} = \frac{1}{n}\sum_i |y_i - \hat{y}_i|$$</div>

**Median absolute error** — robust to outliers:

<div>$$\text{MedAE} = \mathrm{median}_i \, |y_i - \hat{y}_i|$$</div>

**MAPE** — scale-free, undefined when $y_i = 0$:

<div>$$\text{MAPE} = \frac{1}{n}\sum_i \left|\frac{y_i - \hat{y}_i}{y_i}\right|$$</div>

**MSLE** — penalises under-prediction more than over-prediction; requires non-negative values:

<div>$$\text{MSLE} = \frac{1}{n}\sum_i \big(\log(1+y_i) - \log(1+\hat{y}_i)\big)^2$$</div>

**Pinball loss** — asymmetric quantile loss; minimised by the $\alpha$-quantile predictor:

<div>$$L_\alpha = \frac{1}{n}\sum_i \big(\alpha \max(y_i - \hat{y}_i, 0) + (1-\alpha)\max(\hat{y}_i - y_i, 0)\big)$$</div>

**Explained variance** allows for a constant bias:

<div>$$\text{EVS} = 1 - \frac{\mathrm{Var}(y - \hat{y})}{\mathrm{Var}(y)}$$</div>

**$R^2$** vs. **$D^2$-AE** — both are "1 minus loss / loss-of-the-mean-predictor", but using MSE for $R^2$ and MAE for $D^2$-AE:

<div>$$R^2 = 1 - \frac{\sum_i (y_i - \hat{y}_i)^2}{\sum_i (y_i - \bar{y})^2}, \qquad D^2_{\text{AE}} = 1 - \frac{\sum_i |y_i - \hat{y}_i|}{\sum_i |y_i - \tilde{y}|}$$</div>

with $\tilde{y}$ the median.

</div>

<div class="lang-fr">

## Référence API

**Signatures**

```python
sp.mean_squared_error(y_true, y_pred)                       -> float
sp.root_mean_squared_error(y_true, y_pred)                  -> float
sp.mean_absolute_error(y_true, y_pred)                      -> float
sp.median_absolute_error(y_true, y_pred)                    -> float
sp.r2_score(y_true, y_pred)                                 -> float
sp.explained_variance_score(y_true, y_pred)                 -> float
sp.max_error(y_true, y_pred)                                -> float
sp.mean_absolute_percentage_error(y_true, y_pred)           -> float
sp.mean_squared_log_error(y_true, y_pred)                   -> float
sp.root_mean_squared_log_error(y_true, y_pred)              -> float
sp.mean_pinball_loss(y_true, y_pred, alpha=0.5)             -> float
sp.d2_absolute_error_score(y_true, y_pred)                  -> float
```

**Résumé**

| Fonction | Sortie | Description |
|----------|--------|-------------|
| `mean_squared_error` | `float` | Erreur quadratique moyenne |
| `root_mean_squared_error` | `float` | $\sqrt{\text{MSE}}$, dans l'unité cible |
| `mean_absolute_error` | `float` | Erreur absolue moyenne |
| `median_absolute_error` | `float` | Médiane de $|y - \hat{y}|$, très robuste |
| `r2_score` | `float` | Coefficient de détermination |
| `explained_variance_score` | `float` | Ratio de variance (autorise un biais) |
| `max_error` | `float` | Pire résidu |
| `mean_absolute_percentage_error` | `float` | MAPE, sans échelle |
| `mean_squared_log_error` | `float` | MSE en espace log, requiert $y, \hat{y} \geq 0$ |
| `root_mean_squared_log_error` | `float` | $\sqrt{\text{MSLE}}$ |
| `mean_pinball_loss` | `float` | Perte quantile (paramètre `alpha` dans $(0,1)$) |
| `d2_absolute_error_score` | `float` | Analogue de $R^2$ avec MAE |

<details>
<summary><strong>Exemple</strong></summary>

```python
import seraplot as sp

y_true = [3.0, -0.5, 2.0, 7.0, 5.0, 4.5]
y_pred = [2.5, 0.0, 2.1, 7.8, 4.7, 4.6]

print("MSE   :", sp.mean_squared_error(y_true, y_pred))
print("RMSE  :", sp.root_mean_squared_error(y_true, y_pred))
print("MAE   :", sp.mean_absolute_error(y_true, y_pred))
print("MedAE :", sp.median_absolute_error(y_true, y_pred))
print("R²    :", sp.r2_score(y_true, y_pred))
print("EVS   :", sp.explained_variance_score(y_true, y_pred))
print("MaxE  :", sp.max_error(y_true, y_pred))
print("MAPE  :", sp.mean_absolute_percentage_error(y_true, y_pred))
print("MSLE  :", sp.mean_squared_log_error([1,2,3], [1.1,2.1,3.1]))
print("Q90   :", sp.mean_pinball_loss(y_true, y_pred, alpha=0.9))
print("D²-AE :", sp.d2_absolute_error_score(y_true, y_pred))
```

</details>

---

## Fonctionnement algorithmique

**MSE / RMSE / MAE** — agrégats d'erreur point par point :

<div>$$\text{MSE} = \frac{1}{n}\sum_i (y_i - \hat{y}_i)^2 \qquad \text{MAE} = \frac{1}{n}\sum_i |y_i - \hat{y}_i|$$</div>

**Erreur absolue médiane** — robuste aux outliers :

<div>$$\text{MedAE} = \mathrm{median}_i \, |y_i - \hat{y}_i|$$</div>

**MAPE** — sans échelle, indéfini quand $y_i = 0$ :

<div>$$\text{MAPE} = \frac{1}{n}\sum_i \left|\frac{y_i - \hat{y}_i}{y_i}\right|$$</div>

**MSLE** — pénalise davantage la sous-estimation que la sur-estimation ; requiert des valeurs positives :

<div>$$\text{MSLE} = \frac{1}{n}\sum_i \big(\log(1+y_i) - \log(1+\hat{y}_i)\big)^2$$</div>

**Pinball loss** — perte quantile asymétrique, minimisée par le prédicteur $\alpha$-quantile :

<div>$$L_\alpha = \frac{1}{n}\sum_i \big(\alpha \max(y_i - \hat{y}_i, 0) + (1-\alpha)\max(\hat{y}_i - y_i, 0)\big)$$</div>

**Variance expliquée** autorise un biais constant :

<div>$$\text{EVS} = 1 - \frac{\mathrm{Var}(y - \hat{y})}{\mathrm{Var}(y)}$$</div>

**$R^2$** vs. **$D^2$-AE** — tous deux « 1 moins perte / perte du prédicteur moyen », mais utilisant MSE pour $R^2$ et MAE pour $D^2$-AE :

<div>$$R^2 = 1 - \frac{\sum_i (y_i - \hat{y}_i)^2}{\sum_i (y_i - \bar{y})^2}, \qquad D^2_{\text{AE}} = 1 - \frac{\sum_i |y_i - \hat{y}_i|}{\sum_i |y_i - \tilde{y}|}$$</div>

avec $\tilde{y}$ la médiane.

</div>
