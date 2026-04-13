
## Signature

```python
sp.build_stacked_bar(
    title: str,
    category_labels: list[str],
    series_values: list[float],
    *,
    show_values: bool = False,
    series_names: list[str] | None = None,
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

Stacked bar chart. Each bar is split into segments representing series contributions.

Same flat `series_values` layout as `build_grouped_bar`.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | X-axis categories |
| `series_values` | `list[float]` | required | Flat values row-major `[cat0_s0, cat0_s1, ...]` |
| `show_values` | `bool` | `False` | Show segment value labels |
| `series_names` | `list[str] \| None` | `None` | Labels for each series |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp

months = ["Jan", "Feb", "Mar"]
costs = [
    400.0, 150.0, 80.0,
    380.0, 170.0, 95.0,
    420.0, 160.0, 90.0,
]

chart = sp.build_stacked_bar(
    "Monthly Costs",
    category_labels=months,
    series_values=costs,
    series_names=["Labor", "Materials", "Overhead"],
    legend_position="right",
    gridlines=True,
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/stacked-bar.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Grouped Bar](grouped-bar.md)
- [Stacked Bar 3D](../3d/stacked-bar3d.md)
