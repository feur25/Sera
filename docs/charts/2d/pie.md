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

---

## Parameters

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

## Returns

`Chart`

---

## Examples

### Market share

```python
import seraplot as sp

labels = ["Chrome", "Safari", "Firefox", "Edge", "Other"]
values = [65.3, 18.7, 4.1, 4.2, 7.7]

chart = (
    sp.build_pie_chart(
        "Browser Market Share 2024",
        labels=labels,
        values=values,
        show_pct=True,
    )
    .set_bg(None)
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/pie.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

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

## See also

- [Donut Chart](donut.md)
- [Pie 3D](../3d/pie3d.md)
- [Sunburst](sunburst.md)
