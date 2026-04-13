# Lollipop Chart

## Signature

```python
sp.build_lollipop_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    orientation: str = "v",
    show_text: bool = False,
    sort_order: str = "none",
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Lollipop chart — a cleaner alternative to bar charts using a thin stem and a circle at the end.
Reduces ink-to-data ratio compared to filled bars.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Values per category |
| `color_hex` | `int` | `0x6366F1` | Stem and dot color |
| `palette` | `list[int] \| None` | `None` | Per-category colors |
| `orientation` | `str` | `"v"` | `"v"` (vertical) or `"h"` (horizontal) |
| `show_text` | `bool` | `False` | Show value labels |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"`, or `"none"` |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Show gridlines |

---

## Returns

`Chart`

---

## Examples

### Sorted lollipop

```python
import seraplot as sp

chart = sp.build_lollipop_chart(
    "Top Countries by GDP per Capita",
    labels=["Luxembourg", "Switzerland", "USA", "Germany", "France"],
    values=[135605, 92434, 76398, 50802, 42409],
    orientation="h",
    sort_order="desc",
    show_text=True,
    x_label="GDP per capita ($)",
)
```

---

## See also

- [Bar Chart](bar.md)
- [Dumbbell](dumbbell.md)
- [Lollipop 3D](../3d/lollipop3d.md)
