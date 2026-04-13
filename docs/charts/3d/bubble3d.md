# Bubble Chart 3D

## Signature

```python
sp.build_bubble3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    sizes: list[float],
    *,
    color_labels: list[str] | None = None,
    color_values: list[float] | None = None,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    hover_json: str | None = None,
) -> Chart
```

---

## Description

3D bubble chart — scatter in XYZ space where bubble radius encodes a fourth dimension.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | X positions |
| `y` | `list[float]` | required | Y positions |
| `z` | `list[float]` | required | Z positions |
| `sizes` | `list[float]` | required | Bubble radii |
| `color_labels` | `list[str] \| None` | `None` | Categorical color groups |
| `color_values` | `list[float] \| None` | `None` | Continuous colormap values |
| `palette` | `list[int] \| None` | `None` | Custom color palette |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp
import random

n = 200
chart = sp.build_bubble3d_chart(
    "4D Dataset",
    x_values=[random.gauss(0,1) for _ in range(n)],
    y_values=[random.gauss(0,1) for _ in range(n)],
    z_values=[random.gauss(0,1) for _ in range(n)],
    size_values=[random.uniform(5, 30) for _ in range(n)],
    color_labels=[random.choice(["A","B","C"]) for _ in range(n)],
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/bubble3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Scatter 3D](scatter3d.md)
- [Bubble 2D](../2d/bubble.md)
