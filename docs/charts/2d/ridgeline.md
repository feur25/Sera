# Ridgeline Chart

## Signature

```python
sp.build_ridgeline_chart(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    bandwidth: float = 1.0,
    overlap: float = 0.5,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    background: str | None = None,
    gridlines: bool = False,
) -> Chart
```

---

## Description

Ridgeline (joy) chart — stacked KDE curves per category.
Excellent for comparing distributional shapes across many groups.

Ridgeline chart — courbes KDE empilées par catégorie.
Excellent pour comparer les formes de distribution entre de nombreux groupes.

`values` is a flat list. The number of values must be divisible by `len(categories)` (equal samples per group).

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `categories` | `list[str]` | required | Category labels (one ridge each) |
| `values` | `list[float]` | required | Flat concatenated sample data |
| `bandwidth` | `float` | `1.0` | KDE bandwidth factor |
| `overlap` | `float` | `0.5` | Ridge overlap (0 = no overlap, 1 = full overlap) |
| `color_hex` | `int` | `0x6366F1` | Single fill color |
| `palette` | `list[int] \| None` | `None` | Per-ridge colors |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `gridlines` | `bool` | `False` | Vertical gridlines |

---

## Returns / Retour

`Chart`

---

## Examples / Exemples

### Daily temperature ridgeline

```python
import seraplot as sp
import random

months = ["Jan", "Apr", "Jul", "Oct"]
means  = [5, 15, 28, 16]

values = [v for m in means for v in [random.gauss(m, 4) for _ in range(100)]]

chart = sp.build_ridgeline_chart(
    "Monthly Temperature Distribution",
    categories=months,
    values=values,
    overlap=0.6,
    x_label="°C",
)
```

---

## See also / Voir aussi

- [KDE](kde.md)
- [Violin](violin.md)
- [Ridgeline 3D](../3d/ridgeline3d.md)
