# Waterfall Chart

## Signature

```python
sp.build_waterfall(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    color_pos: int = 0x22c55e,
    color_neg: int = 0xef4444,
    color_total: int = 0x6366f1,
    palette: list[int] | None = None,
    background: str | None = None,
    gridlines: bool = True,
) -> Chart
```

---

## Description

Waterfall chart showing sequential positive and negative contributions to a running total.
The last bar can act as the cumulative total.

Positive values rise, negative values fall. The last bar typically represents the final total.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Step labels |
| `values` | `list[float]` | required | Step values (positive or negative) |
| `show_text` | `bool` | `True` | Show value labels on bars |
| `color_pos` | `int` | `0x22c55e` | Color for positive bars |
| `color_neg` | `int` | `0xef4444` | Color for negative bars |
| `color_total` | `int` | `0x6366f1` | Color for total bar |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Horizontal gridlines |

---

## Returns

`Chart`

---

## Examples

### P&L breakdown

```python
import seraplot as sp

chart = sp.build_waterfall(
    "Annual P&L Waterfall",
    labels=["Revenue", "COGS", "Gross Profit", "OpEx", "EBITDA", "D&A", "Net Income"],
    values=[100000, -45000, 0, -30000, 0, -5000, 0],
    show_text=True,
    y_label="$",
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/waterfall.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Bar Chart](bar.md)
- [Funnel](funnel.md)
- [Bullet](bullet.md)
