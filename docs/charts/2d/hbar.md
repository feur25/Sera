# Horizontal Bar Chart / Barres horizontales

## Signature

```python
sp.build_hbar(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0,
    show_text: bool = True,
    color_groups: list[str] | None = None,
    width: int = 900,
    height: int = 500,
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

Horizontal bar chart. Best for long category labels or ranking comparisons.

Graphique en barres horizontales. Idéal pour les étiquettes longues ou les classements.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Bar values |
| `show_text` | `bool` | `True` | Show values on bars (default `True` for hbar) |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"`, `"none"` |
| `color_groups` | `list[str] \| None` | `None` | Group names for color mapping |
| `palette` | `list[int] \| None` | `None` | Custom hex color palette |
| `background` | `str \| None` | `None` | Background CSS color |
| `width` | `int` | `900` | Width in pixels |
| `height` | `int` | `500` | Height in pixels |

---

## Returns / Retour

`Chart`

---

## Examples / Exemples

### Top 10 ranking

```python
import seraplot as sp

chart = sp.build_hbar(
    "Top Countries by GDP",
    labels=["USA", "China", "Germany", "Japan", "India", "UK", "France", "Brazil", "Canada", "Korea"],
    values=[25.0, 18.0, 4.1, 4.2, 3.7, 3.1, 2.9, 2.1, 2.0, 1.7],
    sort_order="desc",
    x_label="GDP (trillion USD)",
    show_text=True,
    width=900,
    height=460,
)
```

---

## See also / Voir aussi

- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Lollipop](lollipop.md) — `sp.build_lollipop_chart()`
