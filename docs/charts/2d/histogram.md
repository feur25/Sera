# Histogram

## Signature

```python
sp.build_histogram(
    title: str,
    values: list[float],
    *,
    color_hex: int = 0,
    bins: int = 20,
    show_counts: bool = False,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    hover_json: str = "",
    legend_position: str = "right",
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Distribution histogram with configurable bin count.

Histogramme de distribution avec nombre de bins configurable.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `values` | `list[float]` | required | Raw numerical data |
| `bins` | `int` | `20` | Number of histogram bins |
| `show_counts` | `bool` | `False` | Show count labels above each bar |
| `color_hex` | `int` | `0` | Bar color as hex int |
| `width` | `int` | `900` | Width in pixels |
| `height` | `int` | `480` | Height in pixels |

---

## Returns

`Chart`

---

## Examples

### Normal distribution

```python
import seraplot as sp
import numpy as np

data = np.random.normal(0, 1, 5000).tolist()

chart = sp.build_histogram(
    "Normal Distribution — N(0,1)",
    values=data,
    bins=40,
    x_label="Value",
    y_label="Count",
    gridlines=True,
    show_counts=False,
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/histogram.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Histogram Overlay](histogram-overlay.md) — compare two distributions
- [KDE](kde.md) — smooth density estimate
- [Violin](violin.md) — distribution by category
