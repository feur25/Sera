# Pie Chart

## Signature

```python
sp.build_pie_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_pct: bool = True,
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

Standard pie chart with optional percentage labels.

Camembert standard avec étiquettes de pourcentage optionnelles.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Slice labels |
| `values` | `list[float]` | required | Slice values (auto-normalized to 100 %) |
| `show_pct` | `bool` | `True` | Show percentage text inside slices |
| `width` | `int` | `700` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `palette` | `list[int] \| None` | `None` | Custom hex color palette |
| `background` | `str \| None` | `None` | Chart background color |
| `hover_json` | `str \| None` | `None` | Custom hover tooltip JSON |
| `legend_position` | `str` | `"right"` | `"right"`, `"bottom"`, `"top"` |

---

## Returns / Retour

`Chart`

---

## Examples / Exemples

### Market share

```python
import seraplot as sp

chart = sp.build_pie_chart(
    "Browser Market Share 2024",
    labels=["Chrome", "Safari", "Firefox", "Edge", "Other"],
    values=[65.3, 18.7, 4.1, 4.2, 7.7],
    show_pct=True,
)
```

### Custom palette

```python
import seraplot as sp

chart = sp.build_pie_chart(
    "Revenue by Region",
    labels=["North", "South", "East", "West"],
    values=[30, 25, 20, 25],
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e, 0xf59e0b],
    legend_position="bottom",
)
```

---

## See also / Voir aussi

- [Donut Chart](donut.md)
- [Pie 3D](../3d/pie3d.md)
- [Sunburst](sunburst.md)
