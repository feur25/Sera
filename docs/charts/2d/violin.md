# Violin Chart

## Signature

```python
sp.build_violin(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    background: str | None = None,
    gridlines: bool = True,
    bandwidth: float = 1.0,
) -> Chart
```

---

## Description

Violin chart combining KDE density estimation with box-plot summary.
The mirrored shape shows the full probability distribution of each group.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `categories` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Flat list of all category samples |
| `color_hex` | `int` | `0x6366F1` | Single fill color |
| `palette` | `list[int] \| None` | `None` | Per-category colors |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Horizontal gridlines |
| `bandwidth` | `float` | `1.0` | KDE smoothing bandwidth multiplier |

---

## Returns

`Chart`

---

## Examples

### Comparing salary distributions

```python
import seraplot as sp
import random

roles = {
    "Engineer": [random.gauss(95000, 15000) for _ in range(60)],
    "Manager":  [random.gauss(110000, 20000) for _ in range(60)],
    "Analyst":  [random.gauss(75000, 12000) for _ in range(60)],
}

chart = sp.build_violin(
    "Salary Distribution by Role",
    categories=list(roles.keys()),
    values=[v for g in roles.values() for v in g],
    y_label="Salary ($)",
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e],
)
```

---

## See also

- [Box Plot](boxplot.md)
- [KDE](kde.md)
- [Ridgeline](ridgeline.md)
- [Violin 3D](../3d/violin3d.md)
