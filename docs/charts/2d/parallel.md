# Parallel Coordinates

## Signature

```python
sp.build_parallel(
    title: str,
    axes: list[str],
    series: list[list[float]],
    *,
    series_names: list[str] | None = None,
    color_groups: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 1000,
    height: int = 480,
    background: str | None = None,
    line_opacity: float = 0.6,
) -> Chart
```

---

## Description

Parallel coordinates chart — each axis is a dimension, each line is an observation.
Ideal for detecting patterns in high-dimensional data.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `axes` | `list[str]` | required | Axis labels (one per dimension) |
| `series` | `list[list[float]]` | required | One inner list per observation (must match `len(axes)`) |
| `series_names` | `list[str] \| None` | `None` | Label per observation |
| `color_groups` | `list[str] \| None` | `None` | Group names for coloring lines |
| `palette` | `list[int] \| None` | `None` | Custom group colors |
| `width` | `int` | `1000` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `background` | `str \| None` | `None` | Background color |
| `line_opacity` | `float` | `0.6` | Line opacity (0.0–1.0) |

---

## Returns

`Chart`

---

## Examples

### Iris dataset parallel coordinates

```python
import seraplot as sp

axes = ["Sepal Length", "Sepal Width", "Petal Length", "Petal Width"]

data = [
    [5.1, 3.5, 1.4, 0.2],
    [6.7, 3.1, 4.7, 1.5],
    [6.3, 3.3, 6.0, 2.5],
]
groups = ["Setosa", "Versicolor", "Virginica"]

chart = sp.build_parallel(
    "Iris Parallel Coordinates",
    axes=axes,
    series=data,
    color_groups=groups,
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e],
)
```

---

## See also

- [Scatter](scatter.md)
- [Radar Chart](radar.md)
