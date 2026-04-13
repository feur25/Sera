# Donut Chart

## Signature

```python
sp.build_donut_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_pct: bool = True,
    inner_radius_ratio: float = 0.55,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
    legend_position: str = "right",
) -> Chart
```

---

## Description

Donut chart — identical to a pie chart with a circular hole at the center.

Diagramme en anneau — comme un camembert mais avec un trou circulaire au centre.

The `inner_radius_ratio` controls what fraction of the radius is the hole (0.0 = solid pie, 1.0 = invisible ring).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Slice labels |
| `values` | `list[float]` | required | Slice values |
| `show_pct` | `bool` | `True` | Show percentage labels |
| `inner_radius_ratio` | `float` | `0.55` | Hole size (0.0–0.9) |
| `width` | `int` | `700` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `palette` | `list[int] \| None` | `None` | Custom hex color palette |
| `background` | `str \| None` | `None` | Chart background |
| `hover_json` | `str \| None` | `None` | Custom hover tooltip JSON |
| `legend_position` | `str` | `"right"` | Legend position |

---

## Returns

`Chart`

---

## Examples

### Basic donut

```python
import seraplot as sp

chart = sp.build_donut_chart(
    "Budget Allocation",
    labels=["R&D", "Marketing", "Operations", "Support"],
    values=[35, 25, 30, 10],
    inner_radius_ratio=0.55,
    show_pct=True,
)
```

---

## See also

- [Pie Chart](pie.md)
- [Sunburst](sunburst.md)
