# Multi-Line Chart

## Signature

```python
sp.build_multiline_chart(
    title: str,
    x_labels: list[str],
    series_values: list[list[float]],
    *,
    show_points: bool = True,
    series_names: list[str] | None = None,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
    legend_position: str = "top",
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Multiple line series plotted on a shared axis.
Unlike `build_line_chart`, this accepts several series in one call.

Plusieurs séries de courbes sur un axe commun.
Contrairement à `build_line_chart`, cette fonction accepte plusieurs séries en un seul appel.

Each inner list in `series_values` must have the same length as `x_labels`.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x_labels` | `list[str]` | required | Shared X-axis tick labels |
| `series_values` | `list[list[float]]` | required | One inner list per series |
| `show_points` | `bool` | `True` | Show markers at data points |
| `series_names` | `list[str] \| None` | `None` | Legend names for each series |
| `palette` | `list[int] \| None` | `None` | Custom hex color per series |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Horizontal gridlines |
| `legend_position` | `str` | `"top"` | `"top"`, `"bottom"`, `"right"` |
| `hover_json` | `str \| None` | `None` | Custom hover tooltip JSON |

---

## Returns / Retour

`Chart`

---

## Examples / Exemples

### Monthly revenue by product

```python
import seraplot as sp

months = ["Jan","Feb","Mar","Apr","May","Jun"]

chart = sp.build_multiline_chart(
    "Monthly Revenue by Product",
    x_labels=months,
    series_values=[
        [12200, 13400, 15100, 14800, 16200, 17500],
        [8100,  9200,  9800,  10200, 11000, 12400],
        [3200,  3600,  4100,  4500,  4800,  5200],
    ],
    series_names=["Product A", "Product B", "Product C"],
    show_points=True,
    y_label="Revenue ($)",
)
```

---

## See also / Voir aussi

- [Line Chart](line.md)
- [Area Chart](area.md)
