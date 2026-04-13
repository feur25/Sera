# Bubble Map

## Signature

```python
sp.build_bubble_map(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    latitudes: list[float] | None = None,
    longitudes: list[float] | None = None,
    iso_codes: list[str] | None = None,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    hover_json: str | None = None,
    bubble_opacity: float = 0.6,
    min_bubble_size: float = 5.0,
    max_bubble_size: float = 50.0,
) -> Chart
```

---

## Description

World map with proportional bubbles at geographic coordinates.
Use `iso_codes` for country-level data (the library resolves centroids automatically), or pass explicit `latitudes` / `longitudes`.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Location names |
| `values` | `list[float]` | required | Bubble sizes |
| `latitudes` | `list[float] \| None` | `None` | Manual latitudes |
| `longitudes` | `list[float] \| None` | `None` | Manual longitudes |
| `iso_codes` | `list[str] \| None` | `None` | ISO-3166 alpha-3 country codes |
| `color_hex` | `int` | `0x6366F1` | Bubble color |
| `palette` | `list[int] \| None` | `None` | Multi-group palette |
| `width` | `int` | `1000` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `bubble_opacity` | `float` | `0.6` | Bubble transparency (0–1) |
| `min_bubble_size` | `float` | `5.0` | Minimum bubble radius in pixels |
| `max_bubble_size` | `float` | `50.0` | Maximum bubble radius in pixels |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### GDP per country (ISO code lookup)

```python
import seraplot as sp

chart = sp.build_bubble_map(
    "GDP by Country",
    labels=["United States", "China", "Germany", "Japan", "France"],
    values=[25500, 17700, 4400, 4200, 3100],
    iso_codes=["USA", "CHN", "DEU", "JPN", "FRA"],
    color_hex=0x6366f1,
    bubble_opacity=0.7,
)
```

### Custom coordinates

```python
import seraplot as sp

chart = sp.build_bubble_map(
    "City Populations",
    labels=["Paris", "Tokyo", "New York", "Lagos"],
    values=[11, 37, 20, 15],
    latitudes=[48.85, 35.68, 40.71, 6.52],
    longitudes=[2.35, 139.69, -74.01, 3.38],
)
```

---

## See also

- [Choropleth](choropleth.md)
- [Globe 3D](../3d/globe3d.md)
