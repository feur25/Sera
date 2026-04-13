# Heatmap 3D

## Signature

```python
sp.build_heatmap3d_chart(
    title: str,
    labels: list[str],
    flat_matrix: list[float],
    *,
    col_labels: list[str] | None = None,
    color_low: int = 0,
    color_high: int = 0,
    extrusion_scale: float = 1.0,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "",
) -> Chart
```

---

## Description

3D heatmap where cell values are extruded as bars rising from a flat grid.
Higher values produce taller columns.

Heatmap 3D où les valeurs de cellules sont extrudées sous forme de barres émergeant d'une grille plate.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Row labels |
| `flat_matrix` | `list[float]` | required | Matrix values, row-major |
| `col_labels` | `list[str] \| None` | `None` | Column labels |
| `color_low` | `int` | auto | Low value color |
| `color_high` | `int` | auto | High value color |
| `extrusion_scale` | `float` | `1.0` | Height multiplier for bars |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |

---

## Returns / Retour

`Chart`

---

## Examples / Exemples

```python
import seraplot as sp

features = ["A", "B", "C", "D"]
n = len(features)
matrix = [abs(i - j) * 0.25 for i in range(n) for j in range(n)]

chart = sp.build_heatmap3d_chart(
    "Distance Matrix 3D",
    labels=features,
    flat_matrix=matrix,
    color_low=0x3b82f6,
    color_high=0xef4444,
)
```

---

## See also / Voir aussi

- [Heatmap 2D](../2d/heatmap.md)
- [Bar 3D](bar3d.md)
