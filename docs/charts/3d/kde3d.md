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

---

## Parameters

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

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp
import random

values = (
    [random.gauss(-2, 1) for _ in range(200)] +
    [random.gauss(0, 0.8) for _ in range(200)] +
    [random.gauss(3, 1.2) for _ in range(200)]
)
categories = ["Group A"] * 200 + ["Group B"] * 200 + ["Group C"] * 200

chart = sp.build_kde3d_chart(
    "Density by Group",
    values,
    categories=categories,
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/kde3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [KDE 2D](../2d/kde.md)
- [Scatter 3D](scatter3d.md)
