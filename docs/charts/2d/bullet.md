# Bullet Chart

## Signature

```python
sp.build_bullet(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    targets: list[float] | None = None,
    max_vals: list[float] | None = None,
    ranges: list[list[float]] | None = None,
    show_text: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    background: str | None = None,
    palette: list[int] | None = None,
) -> Chart
```

---

## Description

Bullet chart — a compact bar that shows a value against a target and qualitative ranges (poor / acceptable / good).

Inspired by Tufte's bullet graph design.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Metric labels |
| `values` | `list[float]` | required | Actual measured values |
| `targets` | `list[float] \| None` | `None` | Target lines per metric |
| `max_vals` | `list[float] \| None` | `None` | Scale maximum per metric |
| `ranges` | `list[list[float]] \| None` | `None` | Qualitative ranges `[[poor, ok, good], ...]` |
| `show_text` | `bool` | `True` | Show value labels |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |

---

## Returns

`Chart`

---

## Examples

### KPI dashboard

```python
import seraplot as sp

chart = sp.build_bullet(
    "KPI Dashboard",
    labels=["Revenue", "Satisfaction", "New Users"],
    values=[87500, 4.2, 1340],
    targets=[100000, 4.5, 1500],
    max_vals=[120000, 5.0, 2000],
    ranges=[
        [60000, 80000, 100000],
        [3.0, 4.0, 4.5],
        [800, 1200, 1500],
    ],
    show_text=True,
)
```

---

## See also

- [Bar Chart](bar.md)
- [Gauge](gauge.md)
- [Waterfall](waterfall.md)
