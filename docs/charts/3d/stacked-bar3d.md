# Stacked Bar Chart 3D

## Signature

```python
sp.build_stacked_bar3d_chart(
    title: str,
    category_labels: list[str],
    series_values: list[float],
    *,
    series_names: list[str] | None = None,
    show_values: bool = False,
    palette: list[int] | None = None,
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

3D stacked bar chart — each bar is segmented into series, rendered as stacked prisms.

`series_values` is a flat list in row-major order: `[cat0_s0, cat0_s1, …, cat1_s0, …]`.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | Category labels |
| `series_values` | `list[float]` | required | Flat row-major series data |
| `series_names` | `list[str] \| None` | `None` | Legend names |
| `show_values` | `bool` | `False` | Labels on segments |
| `palette` | `list[int] \| None` | `None` | Per-series colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp

categories = ["Q1", "Q2", "Q3", "Q4"]
values = [
    30, 20, 50,
    40, 35, 25,
    25, 45, 30,
    50, 30, 20,
]

chart = sp.build_stacked_bar3d_chart(
    "Quarterly Revenue 3D",
    category_labels=categories,
    series_values=values,
    series_names=["Product A", "Product B", "Product C"],
)
```

---

## See also

- [Stacked Bar 2D](../2d/stacked-bar.md)
- [Bar 3D](bar3d.md)
