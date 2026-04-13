# Globe 3D

## Signature

```python
sp.build_globe3d_chart(
    title: str,
    labels: list[str],
    latitudes: list[float],
    longitudes: list[float],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    bg_color: str = "#0f172a",
    width: int = 900,
    height: int = 700,
    hover_json: str | None = None,
    bar_height_scale: float = 1.0,
    show_graticule: bool = True,
) -> Chart
```

---

## Description

Interactive 3D globe — geo data plotted as vertical bars or spikes on a sphere.
Drag to rotate, scroll to zoom.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Location names |
| `latitudes` | `list[float]` | required | Decimal latitudes |
| `longitudes` | `list[float]` | required | Decimal longitudes |
| `values` | `list[float]` | required | Bar heights per location |
| `color_hex` | `int` | `0x6366F1` | Default bar color |
| `palette` | `list[int] \| None` | `None` | Custom palette |
| `bg_color` | `str` | `"#0f172a"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `700` | Canvas height |
| `bar_height_scale` | `float` | `1.0` | Global height multiplier |
| `show_graticule` | `bool` | `True` | Draw lat/lon grid lines |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### World population

```python
import seraplot as sp

chart = sp.build_globe3d_chart(
    "World Population Spikes",
    labels=["China", "India", "USA", "Brazil", "Nigeria"],
    latitudes=[35.86, 20.59, 37.09, -14.23, 9.08],
    longitudes=[104.19, 78.96, -95.71, -51.92, 8.67],
    values=[1412, 1380, 331, 212, 218],
    bar_height_scale=0.5,
)
```

---

## See also

- [Bubble Map](../map/bubble-map.md)
- [Choropleth](../map/choropleth.md)
