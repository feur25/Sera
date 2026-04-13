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

Camembert 3D rendu sous forme de segments d'arc extrudés dans une scène WebGL.

---

## Parameters / Paramètres

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

## Returns / Retour

`Chart`

---

## Examples / Exemples

```python
import seraplot as sp

chart = sp.build_pie3d_chart(
    "Market Share 3D",
    labels=["Chrome", "Safari", "Firefox", "Edge"],
    values=[65, 19, 4, 4],
    extrusion=0.3,
)
```

---

## See also / Voir aussi

- [Pie 2D](../2d/pie.md)
- [Sunburst 3D](sunburst3d.md)
