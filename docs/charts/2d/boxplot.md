# Box Plot

## Signature

```python
sp.build_boxplot(
    title: str,
    category_labels: list[str],
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
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Box-and-whisker plot showing statistical distribution per category.
Each box displays Q1, median, Q3, and IQR whiskers.

`values` is a flat list concatenating all category samples; the lengths must be equal across categories (same number of values per category).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | One label per box |
| `values` | `list[float]` | required | Flat list of all samples |
| `color_hex` | `int` | `0x6366F1` | Single box fill color |
| `palette` | `list[int] \| None` | `None` | Per-category colors |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Show horizontal gridlines |

---

## Returns

`Chart`

---

## Examples

### Test scores by class

```python
import seraplot as sp
import random

n = 50
groups = {
    "Class A": [random.gauss(72, 10) for _ in range(n)],
    "Class B": [random.gauss(68, 15) for _ in range(n)],
    "Class C": [random.gauss(80, 8)  for _ in range(n)],
}

labels = list(groups.keys())
values = [v for g in groups.values() for v in g]

chart = sp.build_boxplot(
    "Test Score Distribution by Class",
    category_labels=labels,
    values=values,
    y_label="Score",
    gridlines=True,
)
```

---

## See also

- [Violin](violin.md)
- [Histogram](histogram.md)
- [KDE](kde.md)
