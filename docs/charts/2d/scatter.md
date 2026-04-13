# Scatter Chart

## Signature

```python
sp.build_scatter_chart(
    title: str,
    x_values: list[float],
    y_values: list[float],
    *,
    color_hex: int = 0,
    show_text: bool = False,
    labels: list[str] | None = None,
    sizes: list[float] | None = None,
    color_groups: list[str] | None = None,
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
    show_regression: bool = False,
    regression_type: str = "linear",
) -> Chart
```

---

## Description

2D scatter plot with optional per-point sizing, grouping, labels, and regression line.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x_values` | `list[float]` | required | X coordinates |
| `y_values` | `list[float]` | required | Y coordinates |
| `color_hex` | `int` | `0` | Uniform point color |
| `show_text` | `bool` | `False` | Show point labels on chart |
| `labels` | `list[str] \| None` | `None` | Per-point label text |
| `sizes` | `list[float] \| None` | `None` | Per-point relative size (0.0–1.0) |
| `color_groups` | `list[str] \| None` | `None` | Per-point group name for color |
| `show_regression` | `bool` | `False` | Overlay regression line |
| `regression_type` | `str` | `"linear"` | `"linear"` or `"polynomial"` |
| `width` | `int` | `900` | Width in pixels |
| `height` | `int` | `480` | Height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `False` | Show gridlines |
| `background` | `str \| None` | `None` | Background color |

---

## Returns

`Chart`

---

## Examples

### Basic scatter

```python
import seraplot as sp
import numpy as np

rng = np.random.default_rng(42)
x_arr = rng.normal(0, 1, 400)
y_arr = x_arr * 0.7 + rng.normal(0, 0.5, 400)
x = x_arr.tolist()
y = y_arr.tolist()
groups = ["High" if v > 0 else "Low" for v in x_arr]

chart = (
    sp.build_scatter_chart(
        "Correlated Random Variables",
        x_values=x,
        y_values=y,
        color_groups=groups,
        x_label="X",
        y_label="Y (correlated)",
        gridlines=True,
    )
    .set_bg(None)
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/scatter.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

### Colored groups with regression

```python
import seraplot as sp
import numpy as np

rng = np.random.default_rng(0)
x_a = rng.normal(0, 1, 200).tolist()
y_a = [xi * 1.5 + rng.normal() for xi in x_a]
x_b = rng.normal(3, 1, 200).tolist()
y_b = [xi * 0.5 + rng.normal() for xi in x_b]

chart = sp.build_scatter_chart(
    "Two Populations",
    x_values=x_a + x_b,
    y_values=y_a + y_b,
    color_groups=["Group A"] * 200 + ["Group B"] * 200,
    show_regression=True,
    regression_type="linear",
    x_label="X",
    y_label="Y",
)
```

---

## See also

- [DBSCAN 2D](../../ml/dbscan.md) — automatic clustering on scatter data
- [Bubble](bubble.md) — scatter with third size dimension
- [Scatter 3D](../3d/scatter3d.md)
