# Area Chart

## Signature

```python
sp.build_area_chart(
    title: str,
    x_labels: list[str],
    series_values: list[list[float]],
    *,
    stacked: bool = False,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    legend_position: str = "top",
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Filled area chart, optionally stacked. Ideal for showing cumulative part-to-whole over time.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x_labels` | `list[str]` | required | X-axis tick labels |
| `series_values` | `list[list[float]]` | required | One list per series |
| `stacked` | `bool` | `False` | Stack areas instead of overlapping |
| `series_names` | `list[str] \| None` | `None` | Legend names per series |
| `palette` | `list[int] \| None` | `None` | Custom colors |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Horizontal gridlines |
| `legend_position` | `str` | `"top"` | Legend position |
| `hover_json` | `str \| None` | `None` | Custom hover tooltip JSON |

---

## Returns

`Chart`

---

## Examples

### Overlapping areas

```python
import seraplot as sp

months = ["Jan","Feb","Mar","Apr","May","Jun"]

chart = sp.build_area_chart(
    "Website Traffic Sources",
    x_labels=months,
    series_values=[
        [4200, 5100, 5800, 6200, 6900, 7400],
        [2100, 2400, 2800, 3100, 3300, 3700],
        [800,  900,  950,  1000, 1100, 1200],
    ],
    series_names=["Organic", "Paid", "Direct"],
    stacked=False,
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/area.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

### Stacked area

```python
import seraplot as sp

chart = sp.build_area_chart(
    "Revenue Stacked by Region",
    x_labels=["Q1","Q2","Q3","Q4"],
    series_values=[
        [12000, 14000, 13500, 16000],
        [8000,  9200,  10000, 11500],
        [4500,  5000,  5200,  6000],
    ],
    series_names=["North", "South", "East"],
    stacked=True,
    y_label="Revenue ($)",
)
```

---

## See also

- [Multi-Line Chart](multiline.md)
- [Line Chart](line.md)
- [Stacked Bar](stacked-bar.md)
