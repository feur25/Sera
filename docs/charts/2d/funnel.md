# Funnel Chart

## Signature

```python
sp.build_funnel(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Funnel chart visualizing progressive reduction through stages (sales pipeline, conversion funnel…).

Graphique en entonnoir visualisant une réduction progressive par étape (pipeline commercial, entonnoir de conversion…).

Bars are stacked and centered; each bar's width is proportional to its value.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Stage labels (top → bottom) |
| `values` | `list[float]` | required | Value at each stage |
| `show_text` | `bool` | `True` | Show value + drop-off % labels |
| `width` | `int` | `700` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `palette` | `list[int] \| None` | `None` | Per-stage colors |
| `background` | `str \| None` | `None` | Background color |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns / Retour

`Chart`

---

## Examples / Exemples

### Sales pipeline

```python
import seraplot as sp

chart = sp.build_funnel(
    "Sales Pipeline Q1",
    labels=["Leads", "Qualified", "Proposal", "Negotiation", "Closed"],
    values=[5000, 2800, 1200, 600, 250],
    show_text=True,
    palette=[0x6366f1, 0x8b5cf6, 0xa78bfa, 0xc4b5fd, 0xddd6fe],
)
```

---

## See also / Voir aussi

- [Waterfall](waterfall.md)
- [Funnel 3D](../3d/funnel3d.md)
