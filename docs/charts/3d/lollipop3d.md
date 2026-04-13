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
    "Monthly Sales 3D",
    x_values=[1.0, 2.0, 3.0, 4.0, 5.0],
    y_values=[0.0, 0.0, 0.0, 0.0, 0.0],
    z_values=[120.0, 95.0, 140.0, 110.0, 160.0],
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/lollipop3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Lollipop 2D](../2d/lollipop.md)
- [Bar 3D](bar3d.md)
