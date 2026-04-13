# Ridgeline Chart 3D

## Signature

```python
sp.build_ridgeline3d_chart(
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

Ridgeline chart in 3D — KDE surfaces per category arranged along the Y axis in a WebGL scene.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `categories` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Flat concatenated sample data |
| `bandwidth` | `float` | `1.0` | KDE bandwidth |
| `palette` | `list[int] \| None` | `None` | Per-ridge colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `z_label` | `str` | `"Density"` | Z-axis label |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp
import random

cats   = ["Low", "Medium", "High"]
means  = [10, 50, 90]
values = [v for m in means for v in [random.gauss(m, 8) for _ in range(150)]]

chart = sp.build_ridgeline3d_chart(
    "Score Distribution by Group",
    categories=cats,
    values=values,
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/ridgeline3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Ridgeline 2D](../2d/ridgeline.md)
- [KDE 3D](kde3d.md)
