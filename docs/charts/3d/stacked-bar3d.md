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
series_data = [
    [30, 40, 25, 50],
    [20, 35, 45, 30],
    [50, 25, 30, 20],
]

chart = sp.build_stacked_bar3d_chart(
    "Quarterly Revenue 3D",
    category_labels=categories,
    series_values=series_data,
    series_names=["Product A", "Product B", "Product C"],
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/stacked-bar3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Stacked Bar 2D](../2d/stacked-bar.md)
- [Bar 3D](bar3d.md)
