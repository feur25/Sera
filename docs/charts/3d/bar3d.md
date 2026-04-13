# Bar Chart 3D

## Signature

```python
sp.build_bar3d_chart(
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

3D bar chart rendering bars as extruded rectangular prisms on a WebGL canvas.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Bar labels |
| `values` | `list[float]` | required | Bar heights |
| `color_hex` | `int` | `0x6366F1` | Single bar color |
| `palette` | `list[int] \| None` | `None` | Per-bar colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `z_label` | `str` | `""` | Z-axis label |
| `show_text` | `bool` | `False` | Show value labels |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp

chart = sp.build_bar3d_chart(
    "Sales by Product (3D)",
    labels=["Alpha", "Beta", "Gamma", "Delta"],
    values=[420, 380, 290, 510],
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e, 0xf59e0b],
)
```

---

## See also

- [Bar Chart 2D](../2d/bar.md)
- [Stacked Bar 3D](stacked-bar3d.md)
