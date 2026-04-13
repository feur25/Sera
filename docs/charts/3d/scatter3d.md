# Scatter 3D

## Signature

```python
sp.build_scatter3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    *,
    color_values: list[float] | None = None,
    color_labels: list[str] | None = None,
    series_names: list[str] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    hover_json: str | None = None,
    palette: list[int] | None = None,
) -> Chart
```

---

## Description

GPU-accelerated 3D scatter plot rendered via WebGL.
Handles millions of points at interactive frame rates.

Use `color_values` for a continuous color scale, or `color_labels` for categorical coloring.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | X coordinates |
| `y` | `list[float]` | required | Y coordinates |
| `z` | `list[float]` | required | Z coordinates |
| `color_values` | `list[float] \| None` | `None` | Continuous colormap values |
| `color_labels` | `list[str] \| None` | `None` | Categorical color group labels |
| `series_names` | `list[str] \| None` | `None` | Series legend names |
| `bg_color` | `str` | `"#1a1a2e"` | Canvas background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `x_label` | `str` | `"X"` | X-axis label |
| `y_label` | `str` | `"Y"` | Y-axis label |
| `z_label` | `str` | `"Z"` | Z-axis label |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |
| `palette` | `list[int] \| None` | `None` | Custom color palette |

---

## Returns

`Chart`

---

## Performance

The renderer uses a single `gl.drawArrays(POINTS, …)` call per frame.
Tested at 10 million points at 60 fps on a mid-range GPU.

---

## Examples

### 3D scatter with categorical colors

```python
import seraplot as sp
import random

n = 1000
x = [random.gauss(0, 1) for _ in range(n)]
y = [random.gauss(0, 1) for _ in range(n)]
z = [random.gauss(0, 1) for _ in range(n)]
groups = [random.choice(["A", "B", "C"]) for _ in range(n)]

chart = sp.build_scatter3d_chart(
    "3D Point Cloud",
    x=x, y=y, z=z,
    color_labels=groups,
    x_label="X", y_label="Y", z_label="Z",
)
```

---

## See also

- [Scatter 2D](../2d/scatter.md)
- [DBSCAN 3D](../../ml/dbscan.md)
- [Bubble 3D](bubble3d.md)
