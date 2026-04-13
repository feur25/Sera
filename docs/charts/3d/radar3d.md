# Radar Chart 3D

## Signature

```python
sp.build_radar3d_chart(
    title: str,
    axes: list[str],
    series: list[list[float]],
    *,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 700,
    height: int = 600,
    max_val: float | None = None,
    fill_opacity: float = 0.25,
) -> Chart
```

---

## Description

3D rendered radar (spider) chart. Same concept as the 2D radar but rendered in a WebGL 3D scene.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `axes` | `list[str]` | required | Axis labels |
| `series` | `list[list[float]]` | required | Values per series per axis |
| `series_names` | `list[str] \| None` | `None` | Legend names |
| `palette` | `list[int] \| None` | `None` | Series colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `700` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `max_val` | `float \| None` | `None` | Common scale maximum |
| `fill_opacity` | `float` | `0.25` | Fill opacity |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp

chart = sp.build_radar3d_chart(
    "Skills Comparison 3D",
    axes=["Python", "Rust", "SQL", "ML", "DevOps"],
    series=[
        [9, 7, 8, 8, 6],
        [5, 10, 6, 4, 9],
    ],
    series_names=["Alice", "Bob"],
    palette=[0x6366f1, 0xf43f5e],
)
```

---

## See also

- [Radar 2D](../2d/radar.md)
