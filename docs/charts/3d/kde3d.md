# KDE Chart 3D

## Signature

```python
sp.build_kde3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    *,
    bandwidth: float = 1.0,
    resolution: int = 50,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Density",
) -> Chart
```

---

## Description

2D Kernel Density Estimation rendered as a 3D surface mesh over a grid.
Visualizes the joint density of two variables.

Estimation par noyau 2D rendue sous forme de surface maillée 3D sur une grille.
Visualise la densité jointe de deux variables.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | X sample data |
| `y` | `list[float]` | required | Y sample data |
| `bandwidth` | `float` | `1.0` | KDE bandwidth factor |
| `resolution` | `int` | `50` | Grid resolution (n × n) |
| `palette` | `list[int] \| None` | `None` | Color gradient palette |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `x_label` | `str` | `"X"` | X-axis label |
| `y_label` | `str` | `"Y"` | Y-axis label |
| `z_label` | `str` | `"Density"` | Z-axis label |

---

## Returns / Retour

`Chart`

---

## Examples / Exemples

```python
import seraplot as sp
import random

x = [random.gauss(0, 1) + random.choice([-2, 2]) for _ in range(500)]
y = [random.gauss(0, 1) for _ in range(500)]

chart = sp.build_kde3d_chart(
    "Bimodal Joint Density",
    x=x, y=y,
    bandwidth=0.8,
    resolution=60,
)
```

---

## See also / Voir aussi

- [KDE 2D](../2d/kde.md)
- [Scatter 3D](scatter3d.md)
