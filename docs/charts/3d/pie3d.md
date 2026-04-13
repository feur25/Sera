# Pie Chart 3D

## Signature

```python
sp.build_pie3d_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_pct: bool = True,
    extrusion: float = 0.2,
    bg_color: str = "#1a1a2e",
    palette: list[int] | None = None,
    width: int = 700,
    height: int = 600,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

3D pie chart rendered as extruded arc segments in a WebGL scene.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Slice labels |
| `values` | `list[float]` | required | Slice values |
| `show_pct` | `bool` | `True` | Show percentage labels |
| `extrusion` | `float` | `0.2` | Depth of pie extrusion |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `palette` | `list[int] \| None` | `None` | Custom palette |
| `width` | `int` | `700` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp

chart = sp.build_pie3d_chart(
    "Market Share 3D",
    labels=["Chrome", "Safari", "Firefox", "Edge"],
    values=[65, 19, 4, 4],
)

```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/pie3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Pie 2D](../2d/pie.md)
- [Sunburst 3D](sunburst3d.md)
