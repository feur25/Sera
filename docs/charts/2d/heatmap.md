# Heatmap

## Signature

```python
sp.build_heatmap(
    title: str,
    labels: list[str],
    flat_matrix: list[float],
    *,
    show_values: bool = True,
    color_low: int = 0,
    color_mid: int = 0,
    color_high: int = 0,
    col_labels: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Color-coded matrix heatmap. Values are automatically normalized for color mapping.

Matrice heatmap avec code couleur. Les valeurs sont normalisées automatiquement.

`flat_matrix` must contain `n_rows × n_cols` values in row-major order.

`labels` = row labels. `col_labels` = column labels (if different from rows).

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Row labels |
| `flat_matrix` | `list[float]` | required | Matrix values, row-major |
| `show_values` | `bool` | `True` | Overlay numeric values in cells |
| `color_low` | `int` | auto | Low value color (hex int) |
| `color_mid` | `int` | auto | Mid value color |
| `color_high` | `int` | auto | High value color |
| `col_labels` | `list[str] \| None` | `None` | Column labels (defaults to `labels`) |

---

## Returns / Retour

`Chart`

---

## Examples / Exemples

### Correlation matrix

```python
import seraplot as sp
import numpy as np

features = ["Age", "Income", "Score", "Visits"]
n = len(features)
matrix = np.corrcoef(np.random.randn(4, 100)).flatten().tolist()

chart = sp.build_heatmap(
    "Feature Correlation Matrix",
    labels=features,
    flat_matrix=matrix,
    color_low=0x3b82f6,
    color_mid=0xffffff,
    color_high=0xef4444,
    show_values=True,
)
```

---

## See also / Voir aussi

- [Heatmap 3D](../3d/heatmap3d.md)
