# Bar Chart / Graphique en barres

## Signature

```python
sp.build_bar_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0,
    orientation: str = "v",
    show_text: bool = False,
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
    series_names: list[str] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Renders a vertical or horizontal bar chart.

Affiche un graphique en barres vertical ou horizontal.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title / Titre du graphique |
| `labels` | `list[str]` | required | Category labels / Étiquettes des catégories |
| `values` | `list[float]` | required | Bar values / Valeurs des barres |
| `color_hex` | `int` | `0` | Single bar color as hex int (e.g. `0xFF5733`) / Couleur unique en hex |
| `orientation` | `str` | `"v"` | `"v"` = vertical, `"h"` = horizontal |
| `show_text` | `bool` | `False` | Show value labels on bars / Afficher les valeurs sur les barres |
| `color_groups` | `list[str] \| None` | `None` | Per-bar group name for coloring / Groupe par barre pour la couleur |
| `width` | `int` | `900` | Canvas width in pixels / Largeur en pixels |
| `height` | `int` | `480` | Canvas height in pixels / Hauteur en pixels |
| `x_label` | `str` | `""` | X-axis label / Étiquette axe X |
| `y_label` | `str` | `""` | Y-axis label / Étiquette axe Y |
| `gridlines` | `bool` | `False` | Show gridlines / Afficher la grille |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"`, or `"none"` |
| `hover_json` | `str` | `""` | Custom hover tooltip JSON / JSON de tooltip personnalisé |
| `legend_position` | `str` | `"right"` | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `palette` | `list[int] \| None` | `None` | Custom color palette as list of hex ints |
| `background` | `str \| None` | `None` | Background color (e.g. `"#0f172a"`) or `None` = transparent |
| `no_x_axis` | `bool` | `False` | Hide X axis / Masquer l'axe X |
| `no_y_axis` | `bool` | `False` | Hide Y axis / Masquer l'axe Y |

---

## Returns / Retour

`Chart` — object with `.html` property containing the full self-contained HTML.

---

## Examples / Exemples

### Basic bar chart

```python
import seraplot as sp

chart = sp.build_bar_chart(
    "Monthly Revenue",
    labels=["Jan", "Feb", "Mar", "Apr", "May"],
    values=[1200.0, 1850.0, 2100.0, 1750.0, 2400.0],
    x_label="Month",
    y_label="Revenue (€)",
    gridlines=True,
    show_text=True,
)
```

### Colored groups

```python
chart = sp.build_bar_chart(
    "Products by Category",
    labels=["A1", "A2", "B1", "B2", "C1"],
    values=[10.0, 15.0, 8.0, 12.0, 20.0],
    color_groups=["Cat A", "Cat A", "Cat B", "Cat B", "Cat C"],
    legend_position="bottom",
)
```

### Dark background

```python
chart = sp.build_bar_chart(
    "Dark Theme",
    labels=["Q1", "Q2", "Q3", "Q4"],
    values=[300.0, 450.0, 380.0, 520.0],
    background="#0f172a",
    width=800,
    height=400,
)
```

---

## See also / Voir aussi

- [Horizontal Bar](hbar.md) — `sp.build_hbar()`
- [Grouped Bar](grouped-bar.md) — `sp.build_grouped_bar()`
- [Stacked Bar](stacked-bar.md) — `sp.build_stacked_bar()`
