# Custom Hover Tooltips

## Signature

```python
sp.build_hover_json(
    *,
    fields: list[str],
    values: list[list[str | float]],
    format: str = "table",
) -> str
```

---

## Description

Builds a JSON string suitable for the `hover_json` parameter accepted by most chart functions.
The JSON format drives the interactive tooltip rendered on mouse hover.

Génère une chaîne JSON adaptée au paramètre `hover_json` accepté par la plupart des fonctions de graphique.
Ce JSON pilote les infobulles interactives affichées au survol de la souris.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `fields` | `list[str]` | required | Column headers for the tooltip |
| `values` | `list[list[str \| float]]` | required | One inner list per data point (same order as chart data) |
| `format` | `str` | `"table"` | Tooltip layout: `"table"` or `"list"` |

---

## Returns / Retour

`str` — JSON string to pass to `hover_json=` parameters.

---

## Examples / Exemples

### Bar chart with rich tooltip

```python
import seraplot as sp

labels = ["Alpha", "Beta", "Gamma"]
values = [420, 380, 290]

hover = sp.build_hover_json(
    fields=["Product", "Revenue ($)", "Growth (%)"],
    values=[
        ["Alpha", 420, "+12%"],
        ["Beta",  380, "+5%"],
        ["Gamma", 290, "-3%"],
    ],
)

chart = sp.build_bar_chart(
    "Revenue by Product",
    labels=labels,
    values=values,
    hover_json=hover,
)
```

---

## See also / Voir aussi

- [Background](background.md)
- [Palette & Colors](palette.md)
