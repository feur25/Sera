# Radar Chart

## Signature

```python
sp.build_radar_chart(
    title: str,
    axes: list[str],
    series: list[list[float]],
    *,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    fill_opacity: float = 0.25,
    width: int = 600,
    height: int = 500,
    background: str | None = None,
    max_val: float | None = None,
) -> Chart
```

---

## Description

Spider / radar chart — polygon per series across radial axes.
Useful for profiling entities across multiple dimensions simultaneously.

Graphique araignée / radar — polygone par série sur des axes radiaux.
Utile pour profiler des entités sur plusieurs dimensions simultanément.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `axes` | `list[str]` | required | Axis labels (dimension names) |
| `series` | `list[list[float]]` | required | One inner list per series (same length as `axes`) |
| `series_names` | `list[str] \| None` | `None` | Legend names per series |
| `palette` | `list[int] \| None` | `None` | Series fill colors |
| `fill_opacity` | `float` | `0.25` | Polygon fill opacity (0.0–1.0) |
| `width` | `int` | `600` | Canvas width |
| `height` | `int` | `500` | Canvas height |
| `background` | `str \| None` | `None` | Background color |
| `max_val` | `float \| None` | `None` | Common max scale value (auto if None) |

---

## Returns / Retour

`Chart`

---

## Examples / Exemples

### Player comparison

```python
import seraplot as sp

chart = sp.build_radar_chart(
    "Player Stats Comparison",
    axes=["Speed", "Strength", "Defense", "Dribbling", "Shooting", "Passing"],
    series=[
        [85, 70, 65, 90, 88, 82],
        [72, 88, 79, 68, 75, 85],
    ],
    series_names=["Player A", "Player B"],
    palette=[0x6366f1, 0xf43f5e],
)
```

---

## See also / Voir aussi

- [Parallel Coordinates](parallel.md)
- [Radar 3D](../3d/radar3d.md)
