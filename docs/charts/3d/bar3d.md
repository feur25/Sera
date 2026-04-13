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
    "Sales by Product",
    x_values=[0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0],
    y_values=[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
    z_values=[420.0, 380.0, 290.0, 510.0, 480.0, 420.0, 350.0, 590.0],
    x_label="Product",
    y_label="Year",
    z_label="Units",
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/bar3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Bar Chart 2D](../2d/bar.md)
- [Stacked Bar 3D](stacked-bar3d.md)
