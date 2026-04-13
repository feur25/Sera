
## Signature

```python
sp.build_grouped_bar(
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

Grouped bar chart for comparing multiple series across categories.

`series_values` must be a **flat list** of length `n_categories × n_series`, row-major (category-first).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | Category names on X axis |
| `series_values` | `list[float]` | required | Flat values: `[cat0_s0, cat0_s1, cat1_s0, cat1_s1, ...]` |
| `show_values` | `bool` | `False` | Show value labels |
| `series_names` | `list[str] \| None` | `None` | Series names for legend |
| `palette` | `list[int] \| None` | `None` | Custom color palette |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp

categories = ["Q1", "Q2", "Q3", "Q4"]
values = [
    120.0, 90.0, 150.0,
    130.0, 110.0, 140.0,
    100.0, 95.0,  160.0,
    140.0, 120.0, 175.0,
]

logo = "https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png"
hover = sp.build_hover_json(categories * 3, images=[logo] * len(categories * 3))

chart = (
    sp.build_grouped_bar(
        "Quarterly Sales by Product",
        category_labels=categories,
        series_values=values,
        series_names=["Product A", "Product B", "Product C"],
        show_values=True,
        gridlines=True,
        legend_position="bottom",
        hover_json=hover,
    )
    .set_bg(None)
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/grouped-bar.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Stacked Bar](stacked-bar.md) — `sp.build_stacked_bar()`
- [Bar Chart](bar.md)
