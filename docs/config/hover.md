# Infobulles personnalisées

<div class="lang-en">

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

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `fields` | `list[str]` | required | Column headers for the tooltip |
| `values` | `list[list[str \| float]]` | required | One inner list per data point |
| `format` | `str` | `"table"` | Tooltip layout: `"table"` or `"list"` |

</div>

<div class="lang-fr">

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

Construit une chaîne JSON adaptée au paramètre `hover_json` accepté par la plupart des fonctions graphiques.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `fields` | `list[str]` | requis | En-têtes de colonnes de l'infobulle |
| `values` | `list[list[str \| float]]` | requis | Une liste par point de données |
| `format` | `str` | `"table"` | Mise en page : `"table"` ou `"list"` |

---

## Retourne

`str` — Chaîne JSON à passer au paramètre `hover_json=`.

---

## Exemples

```python
import seraplot as sp

hover = sp.build_hover_json(
    fields=["Produit", "Revenu (€)", "Croissance"],
    values=[
        ["Alpha", 420, "+12%"],
        ["Bêta",  380, "+5%"],
        ["Gamma", 290, "-3%"],
    ],
)

chart = sp.build_bar_chart(
    "Revenus par produit",
    labels=["Alpha", "Bêta", "Gamma"],
    values=[420, 380, 290],
    hover_json=hover,
)
```

---

## Voir aussi

- [Fond du graphique](background.md)
- [Palette & Couleurs](palette.md)

</div>


---

## Returns

`str` — JSON string to pass to `hover_json=` parameters.

---

## Examples

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

## See also

- [Background](background.md)
- [Palette & Colors](palette.md)
