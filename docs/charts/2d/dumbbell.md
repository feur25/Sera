# Dumbbell Chart

## Signature

```python
sp.build_dumbbell(
    title: str,
    labels: list[str],
    values_start: list[float],
    values_end: list[float],
    *,
    show_text: bool = True,
    color_start: int = 0x6366f1,
    color_end: int = 0xf43f5e,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    background: str | None = None,
    gridlines: bool = True,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Dumbbell chart — a horizontal line connecting two data points per category,
ideal for showing the gap or change between two states.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels |
| `values_start` | `list[float]` | required | Left (start) values |
| `values_end` | `list[float]` | required | Right (end) values |
| `show_text` | `bool` | `True` | Show endpoint value labels |
| `color_start` | `int` | `0x6366f1` | Start point color |
| `color_end` | `int` | `0xf43f5e` | End point color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `gridlines` | `bool` | `True` | Vertical gridlines |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### Life expectancy 2000 vs 2023

```python
import seraplot as sp

chart = sp.build_dumbbell(
    "Life Expectancy: 2000 vs 2023",
    labels=["Japan", "Germany", "Brazil", "India", "Nigeria"],
    values_start=[81.2, 78.1, 70.4, 62.8, 46.5],
    values_end=[84.3, 81.5, 75.2, 70.8, 54.9],
    x_label="Age (years)",
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/dumbbell.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Slope](slope.md)
- [Dumbbell 3D](../3d/dumbbell3d.md)
