# KDE Chart

## Signature

```python
sp.build_kde_chart(
    title: str,
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    bandwidth: float = 1.0,
    fill: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "Density",
    gridlines: bool = True,
    background: str | None = None,
    palette: list[int] | None = None,
    series_names: list[str] | None = None,
) -> Chart
```

---

## Description

Kernel Density Estimation (KDE) curve — a smooth, continuous estimate of a probability distribution.
Better than histograms for identifying the underlying shape of data.

When multiple series are provided via a flat `values` list with matching `series_names`, several overlaid density curves are drawn.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `values` | `list[float]` | required | Sample data points |
| `color_hex` | `int` | `0x6366F1` | Curve color |
| `bandwidth` | `float` | `1.0` | Smoothing bandwidth scale factor |
| `fill` | `bool` | `True` | Fill area under curve |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `"Density"` | Y-axis label |
| `gridlines` | `bool` | `True` | Horizontal gridlines |
| `palette` | `list[int] \| None` | `None` | Multi-series color palette |
| `series_names` | `list[str] \| None` | `None` | Multi-series legend names |

---

## Returns

`Chart`

---

## Examples

### Single distribution

```python
import seraplot as sp
import random

values = [random.gauss(50, 10) for _ in range(500)]

chart = sp.build_kde_chart(
    "Score Distribution",
    values=values,
    x_label="Score",
    filled=True,
    bandwidth=1.0,
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/kde.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Histogram](histogram.md)
- [Violin](violin.md)
- [Ridgeline](ridgeline.md)
- [KDE 3D](../3d/kde3d.md)
