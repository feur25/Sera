# Ridgeline Chart 3D

## Signature

```python
sp.build_ridgeline3d_chart(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    bandwidth: float = 1.0,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "Density",
) -> Chart
```

---

## Description

Ridgeline chart in 3D — KDE surfaces per category arranged along the Y axis in a WebGL scene.

Ridgeline 3D — surfaces KDE par catégorie disposées le long de l'axe Y dans une scène WebGL.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `categories` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Flat concatenated sample data |
| `bandwidth` | `float` | `1.0` | KDE bandwidth |
| `palette` | `list[int] \| None` | `None` | Per-ridge colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `z_label` | `str` | `"Density"` | Z-axis label |

---

## Returns / Retour

`Chart`

---

## Examples / Exemples

```python
import seraplot as sp
import random

cats   = ["Low", "Medium", "High"]
means  = [10, 50, 90]
values = [v for m in means for v in [random.gauss(m, 8) for _ in range(150)]]

chart = sp.build_ridgeline3d_chart(
    "Score Distribution by Group",
    categories=cats,
    values=values,
)
```

---

## See also / Voir aussi

- [Ridgeline 2D](../2d/ridgeline.md)
- [KDE 3D](kde3d.md)
