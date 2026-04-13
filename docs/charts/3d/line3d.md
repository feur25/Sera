# Line Chart 3D

## Signature

```python
sp.build_line3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    series_names: list[str] | None = None,
    show_points: bool = True,
) -> Chart
```

---

## Description

3D line chart connecting sequential points in 3D space.
Useful for trajectories, time-series in 3D space, and parametric curves.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | X coordinates |
| `y` | `list[float]` | required | Y coordinates |
| `z` | `list[float]` | required | Z coordinates |
| `color_hex` | `int` | `0x6366F1` | Line color |
| `palette` | `list[int] \| None` | `None` | Multi-series colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `x_label` | `str` | `"X"` | X-axis label |
| `y_label` | `str` | `"Y"` | Y-axis label |
| `z_label` | `str` | `"Z"` | Z-axis label |
| `series_names` | `list[str] \| None` | `None` | Series legend names |
| `show_points` | `bool` | `True` | Show point markers |

---

## Returns

`Chart`

---

## Examples

### Helix trajectory

```python
import seraplot as sp
import math

t = [i * 0.1 for i in range(100)]
x = [math.cos(v) for v in t]
y = [math.sin(v) for v in t]
z = t

chart = sp.build_line3d_chart(
    "Helix",
    x_values=x, y_values=y, z_values=z,
    x_label="cos(t)", y_label="sin(t)", z_label="t",
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/line3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Line Chart 2D](../2d/line.md)
- [Scatter 3D](scatter3d.md)
