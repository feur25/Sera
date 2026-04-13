
## Signature

```python
sp.build_line_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    show_points: bool = True,
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

Single-series line chart with optional data points.

For **multiple series**, use [`build_multiline_chart`](multiline.md).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | X-axis labels |
| `values` | `list[float]` | required | Y values |
| `color_hex` | `int` | `0x6366F1` | Line color as hex int (indigo by default) |
| `show_points` | `bool` | `True` | Draw circles at data points |
| `gridlines` | `bool` | `False` | Horizontal gridlines |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"`, `"none"` |
| `width` | `int` | `900` | Width in pixels |
| `height` | `int` | `480` | Height in pixels |

---

## Returns

`Chart`

---

## Examples

### Time series

```python
import seraplot as sp

months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
          "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
revenue = [1200.0, 1350.0, 1100.0, 1600.0, 1800.0, 2100.0,
           1950.0, 2300.0, 2000.0, 2500.0, 2200.0, 2800.0]

logo = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png"
hover = sp.build_hover_json(months, images=[logo] * len(months))

chart = (
    sp.build_line_chart(
        "Annual Revenue",
        labels=months,
        values=revenue,
        x_label="Month",
        y_label="Revenue (€)",
        gridlines=True,
        color_hex=0x22d3ee,
        hover_json=hover,
    )
    .set_bg(None)
    .show_grid()
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/line.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Multi-line](multiline.md) — `sp.build_multiline_chart()` for multiple series
- [Area Chart](area.md) — `sp.build_area_chart()`
