# Choropleth Map

## Signature

```python
sp.build_choropleth(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    iso_codes: list[str] | None = None,
    color_low: int = 0,
    color_high: int = 0,
    palette: list[int] | None = None,
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    hover_json: str | None = None,
    show_legend: bool = True,
    null_color: int = 0xdddddd,
) -> Chart
```

---

## Description

Choropleth (filled map) — country or region polygons colored by a scalar value.

Countries without data receive the `null_color`. Provide `iso_codes` (ISO-3166 alpha-3) to match countries automatically.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Country / region names |
| `values` | `list[float]` | required | Values to color by |
| `iso_codes` | `list[str] \| None` | `None` | ISO-3166 alpha-3 codes |
| `color_low` | `int` | auto | Low value color |
| `color_high` | `int` | auto | High value color |
| `null_color` | `int` | `0xdddddd` | Color for countries with no data |
| `width` | `int` | `1000` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `show_legend` | `bool` | `True` | Show color scale legend |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### Unemployment rate choropleth

```python
import seraplot as sp

chart = sp.build_choropleth(
    "Unemployment Rate by Country",
    labels=["France", "Germany", "Spain", "Italy", "Portugal"],
    values=[7.1, 3.0, 11.8, 6.7, 6.2],
    iso_codes=["FRA", "DEU", "ESP", "ITA", "PRT"],
    color_low=0xd1fae5,
    color_high=0xef4444,
)
```

---

## See also

- [Bubble Map](bubble-map.md)
- [Globe 3D](../3d/globe3d.md)
