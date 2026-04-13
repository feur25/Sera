# Gauge Chart

## Signature

```python
sp.build_gauge(
    title: str,
    value: float,
    *,
    min_val: float = 0.0,
    max_val: float = 100.0,
    thresholds: list[float] | None = None,
    threshold_colors: list[int] | None = None,
    color_hex: int = 0x6366F1,
    width: int = 500,
    height: int = 350,
    show_value: bool = True,
    background: str | None = None,
    label: str = "",
) -> Chart
```

---

## Description

Semicircular gauge (speedometer) chart. Colored arcs can indicate thresholds (danger / warning / ok).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `value` | `float` | required | Current reading |
| `min_val` | `float` | `0.0` | Scale minimum |
| `max_val` | `float` | `100.0` | Scale maximum |
| `thresholds` | `list[float] \| None` | `None` | Zone boundaries within [min,max] |
| `threshold_colors` | `list[int] \| None` | `None` | Colors for each threshold zone |
| `color_hex` | `int` | `0x6366F1` | Needle color |
| `width` | `int` | `500` | Canvas width |
| `height` | `int` | `350` | Canvas height |
| `show_value` | `bool` | `True` | Display numeric value below needle |
| `label` | `str` | `""` | Unit label under the value |

---

## Returns

`Chart`

---

## Examples

### CPU usage gauge

```python
import seraplot as sp

chart = sp.build_gauge(
    "CPU Usage",
    value=73.5,
    min_val=0,
    max_val=100,
    thresholds=[50, 80, 100],
    threshold_colors=[0x22c55e, 0xf59e0b, 0xef4444],
    show_value=True,
    label="%",
)
```

---

## See also

- [Bullet](bullet.md)
- [Bar Chart](bar.md)
