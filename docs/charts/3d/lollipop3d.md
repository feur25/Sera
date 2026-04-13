# Lollipop Chart 3D

## Signature

```python
sp.build_lollipop3d_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
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

3D lollipop chart — stems and spheres rendered in a WebGL scene.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Values per category |
| `color_hex` | `int` | `0x6366F1` | Stem and sphere color |
| `palette` | `list[int] \| None` | `None` | Per-category colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
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

chart = sp.build_lollipop3d_chart(
    "Sales 3D",
    labels=["Jan", "Feb", "Mar", "Apr", "May"],
    values=[120, 95, 140, 110, 160],
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e, 0xf59e0b, 0x10b981],
)
```

---

## See also

- [Lollipop 2D](../2d/lollipop.md)
- [Bar 3D](bar3d.md)
