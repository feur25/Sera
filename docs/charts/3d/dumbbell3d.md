# Dumbbell Chart 3D

## Signature

```python
sp.build_dumbbell3d_chart(
    title: str,
    labels: list[str],
    values_start: list[float],
    values_end: list[float],
    *,
    color_start: int = 0x6366f1,
    color_end: int = 0xf43f5e,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "",
    show_text: bool = False,
) -> Chart
```

---

## Description

Dumbbell chart in 3D — connects start and end spheres with a 3D tube.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels |
| `values_start` | `list[float]` | required | Start values |
| `values_end` | `list[float]` | required | End values |
| `color_start` | `int` | `0x6366f1` | Start sphere color |
| `color_end` | `int` | `0xf43f5e` | End sphere color |
| `bg_color` | `str` | `"#1a1a2e"` | Background |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `show_text` | `bool` | `False` | Show value labels |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp

chart = sp.build_dumbbell3d_chart(
    "Before vs After 3D",
    labels=["Group A", "Group B", "Group C"],
    values_start=[40, 55, 70],
    values_end=[60, 75, 85],
)
```

---

## See also

- [Dumbbell 2D](../2d/dumbbell.md)
- [Slope](../2d/slope.md)
