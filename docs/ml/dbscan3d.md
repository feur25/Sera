# DBSCAN 3D Chart

## Signature

```python
sp.build_dbscan_chart_3d(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    *,
    eps: float = 0.5,
    min_samples: int = 5,
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    bg_color: str = "#1a1a2e",
    normalize: bool = False,
    palette: list[int] | None = None,
) -> Chart
```

---

## Description

DBSCAN clustering in 3D — rendered via GPU WebGL for maximum performance.
Each cluster is assigned a distinct color; noise points are grey.

Clustering DBSCAN 3D — rendu via WebGL GPU pour des performances maximales.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | X coordinates |
| `y` | `list[float]` | required | Y coordinates |
| `z` | `list[float]` | required | Z coordinates |
| `eps` | `float` | `0.5` | Neighborhood radius |
| `min_samples` | `int` | `5` | Core point threshold |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `x_label` | `str` | `"X"` | X-axis label |
| `y_label` | `str` | `"Y"` | Y-axis label |
| `z_label` | `str` | `"Z"` | Z-axis label |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `normalize` | `bool` | `False` | Normalize XYZ to [0, 1] |
| `palette` | `list[int] \| None` | `None` | Custom cluster colors |

---

## Returns

`Chart`

---

## Examples

### 3D clusters

```python
import seraplot as sp
import random

def blob3d(cx, cy, cz, n=200, s=0.4):
    return [(cx+random.gauss(0,s), cy+random.gauss(0,s), cz+random.gauss(0,s))
            for _ in range(n)]

pts = blob3d(0,0,0) + blob3d(5,5,5) + blob3d(10,0,5)
x, y, z = zip(*pts)

chart = sp.build_dbscan_chart_3d(
    "3D DBSCAN",
    x=list(x), y=list(y), z=list(z),
    eps=1.2,
    min_samples=5,
)
```

---

## See also

- [DBSCAN 2D](dbscan.md)
- [DBSCAN Class](dbscan-class.md)
- [Scatter 3D](../charts/3d/scatter3d.md)
