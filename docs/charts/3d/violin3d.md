# Violin Chart 3D

## Signature

```python
sp.build_violin3d_chart(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    bandwidth: float = 1.0,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "Density",
) -> Chart
```

---

## Description

3D violin chart — KDE-based distribution surfaces per category rendered in WebGL.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `categories` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Flat sample data (equal count per category) |
| `bandwidth` | `float` | `1.0` | KDE bandwidth |
| `palette` | `list[int] \| None` | `None` | Per-category colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp
import random

groups = ["Control", "Treatment A", "Treatment B"]
means  = [50, 65, 72]
values = [v for m in means for v in [random.gauss(m, 8) for _ in range(80)]]

chart = sp.build_violin3d_chart(
    "Trial Results",
    categories=groups,
    values=values,
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/violin3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Violin 2D](../2d/violin.md)
- [KDE 3D](kde3d.md)
