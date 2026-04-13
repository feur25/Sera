# Slope Chart

## Signature

```python
sp.build_slope(
    title: str,
    labels: list[str],
    values_left: list[float],
    values_right: list[float],
    left_label: str,
    right_label: str,
    *,
    show_text: bool = True,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 600,
    height: int = 480,
    background: str | None = None,
) -> Chart
```

---

## Description

Slope chart comparing two values per entity (before/after, period A vs B).
Parallel vertical axes are connected by slope lines — rising or falling.

Graphique de pente comparant deux valeurs par entité (avant/après, période A vs B).
Des lignes de pente relient les deux axes verticaux parallèles.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Entity names (one per line) |
| `values_left` | `list[float]` | required | Values on the left axis |
| `values_right` | `list[float]` | required | Values on the right axis |
| `left_label` | `str` | required | Left axis label (e.g. `"2020"`) |
| `right_label` | `str` | required | Right axis label (e.g. `"2024"`) |
| `show_text` | `bool` | `True` | Show values next to endpoints |
| `color_hex` | `int` | `0x6366F1` | Line color (single) |
| `palette` | `list[int] \| None` | `None` | Per-entity line colors |
| `width` | `int` | `600` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `background` | `str \| None` | `None` | Chart background |

---

## Returns / Retour

`Chart`

---

## Examples / Exemples

### Country ranking change

```python
import seraplot as sp

chart = sp.build_slope(
    "HDI Change 2000 → 2023",
    labels=["Germany", "Japan", "Brazil", "India", "Nigeria"],
    values_left=[0.926, 0.909, 0.694, 0.493, 0.452],
    values_right=[0.950, 0.920, 0.760, 0.644, 0.548],
    left_label="2000",
    right_label="2023",
    show_text=True,
)
```

---

## See also / Voir aussi

- [Line Chart](line.md)
- [Dumbbell](dumbbell.md)
