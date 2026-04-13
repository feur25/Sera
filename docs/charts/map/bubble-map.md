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
    labels=["USA", "CHN", "DEU", "JPN", "FRA"],
    values=[25500, 17700, 4400, 4200, 3100],
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/bubble-map.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

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
