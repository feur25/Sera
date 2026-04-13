
## Signature

```python
sp.build_histogram_overlay(
    title: str,
    values: list[float],
    overlay_values: list[float],
    *,
    color_hex: int = 0,
    overlay_color_hex: int = 0,
    bins: int = 20,
    series_names: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Overlaid histogram comparing two distributions side-by-side with transparency.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `values` | `list[float]` | required | First distribution data |
| `overlay_values` | `list[float]` | required | Second distribution data |
| `bins` | `int` | `20` | Number of bins |
| `series_names` | `list[str] \| None` | `None` | Names for legend `["Series A", "Series B"]` |
| `color_hex` | `int` | `0` | Color for first series |
| `overlay_color_hex` | `int` | `0` | Color for second series |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp
import numpy as np

rng = np.random.default_rng(42)
control = rng.normal(5.0, 1.0, 1000).tolist()
treatment = rng.normal(5.8, 1.2, 1000).tolist()

chart = sp.build_histogram_overlay(
    "Control vs Treatment",
    values=control,
    overlay_values=treatment,
    bins=30,
    series_names=["Control", "Treatment"],
    x_label="Measurement",
    y_label="Frequency",
    gridlines=True,
)
```

---

## See also

- [Histogram](histogram.md)
- [KDE](kde.md)
