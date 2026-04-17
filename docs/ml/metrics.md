# Metrics

## Functions

```python
sp.accuracy_score(y_true, y_pred) -> float
sp.mean_squared_error(y_true, y_pred) -> float
sp.mean_absolute_error(y_true, y_pred) -> float
sp.r2_score(y_true, y_pred) -> float
sp.classification_report(y_true, y_pred) -> str
```

---

## Description

Standalone evaluation functions for classification and regression. Also available as `.score()` on every model — classifiers use accuracy, regressors use R².

---

## Function Reference

### `accuracy_score(y_true, y_pred) -> float`

Fraction of correct predictions.

| Argument | Type | Description |
|----------|------|-------------|
| `y_true` | `list[int]` | Ground truth labels |
| `y_pred` | `list[int]` | Predicted labels |

### `mean_squared_error(y_true, y_pred) -> float`

Mean of squared residuals.

| Argument | Type | Description |
|----------|------|-------------|
| `y_true` | `list[float]` | Ground truth |
| `y_pred` | `list[float]` | Predictions |

### `mean_absolute_error(y_true, y_pred) -> float`

Mean of absolute residuals.

### `r2_score(y_true, y_pred) -> float`

Coefficient of determination (1.0 = perfect).

### `classification_report(y_true, y_pred) -> str`

Text report with precision, recall, F1 per class + macro/weighted averages.

---

## Examples

<details>
<summary><strong>Classification metrics</strong></summary>

```python
import seraplot as sp

y_true = [0, 0, 1, 1, 2, 2, 2]
y_pred = [0, 1, 1, 1, 2, 0, 2]

print(f"Accuracy: {sp.accuracy_score(y_true, y_pred):.4f}")
print(sp.classification_report(y_true, y_pred))
```

</details>

<details>
<summary><strong>Regression metrics</strong></summary>

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
