# Bubble Chart

## Signature

```python
sp.build_bubble(
    title: str,
    x_values: list[float],
    y_values: list[float],
    sizes: list[float],
    *,
    labels: list[str] | None = None,
    color_groups: list[str] | None = None,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Bubble chart: scatter plot where each point's area encodes a third dimension (`sizes`).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x_values` | `list[float]` | required | X-axis positions |
| `y_values` | `list[float]` | required | Y-axis positions |
| `sizes` | `list[float]` | required | Values that drive bubble radius |
| `labels` | `list[str] \| None` | `None` | Per-bubble text labels |
| `color_groups` | `list[str] \| None` | `None` | Group names for coloring |
| `color_hex` | `int` | `0x6366F1` | Default bubble color |
| `palette` | `list[int] \| None` | `None` | Custom palette per group |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Gridlines |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### Gapminder-style chart

```python
import seraplot as sp

countries = ["USA", "China", "Germany", "India", "Brazil"]
gdp       = [65000, 12500, 48000, 2100, 8800]
life_exp  = [78.5, 77.1, 81.3, 69.7, 75.2]
population= [331, 1412, 83, 1380, 212]

chart = sp.build_bubble(
    "GDP vs Life Expectancy (2023)",
    x_values=gdp,
    y_values=life_exp,
    sizes=[p / 10 for p in population],
    categories=countries,
    x_label="GDP per capita ($)",
    y_label="Life expectancy (years)",
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/bubble.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Scatter](scatter.md)
- [Bubble Map](../map/bubble-map.md)
- [Bubble 3D](../3d/bubble3d.md)
