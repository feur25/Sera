# Sunburst Chart 3D

## Signature

```python
sp.build_sunburst3d_chart(
    title: str,
    labels: list[str],
    parents: list[str],
    values: list[float],
    *,
    extrusion: float = 0.15,
    bg_color: str = "#1a1a2e",
    palette: list[int] | None = None,
    width: int = 700,
    height: int = 600,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

3D sunburst chart — concentric extruded arc rings in a WebGL scene.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Node labels |
| `parents` | `list[str]` | required | Parent labels |
| `values` | `list[float]` | required | Node sizes |
| `extrusion` | `float` | `0.15` | Depth of arc extrusion |
| `bg_color` | `str` | `"#1a1a2e"` | Background |
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

labels  = ["Root", "A", "B", "A1", "A2", "B1"]
parents = ["", "Root", "Root", "A", "A", "B"]
values  = [1, 40, 60, 25, 15, 60]

chart = sp.build_sunburst3d_chart(
    "Org Chart 3D",
    labels=labels, parents=parents, values=values,
)
```

---

## See also

- [Sunburst 2D](../2d/sunburst.md)
- [Pie 3D](pie3d.md)
