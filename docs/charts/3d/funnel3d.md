# Funnel Chart 3D

## Signature

```python
sp.build_funnel3d_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 700,
    height: int = 600,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

3D funnel chart where each stage is a truncated cone (frustum) in a WebGL scene.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Stage labels |
| `values` | `list[float]` | required | Stage values |
| `show_text` | `bool` | `True` | Show value labels |
| `palette` | `list[int] \| None` | `None` | Per-stage colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background |
| `width` | `int` | `700` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp

chart = sp.build_funnel3d_chart(
    "Conversion Funnel 3D",
    labels=["Visitors", "Leads", "Opportunities", "Proposals", "Won"],
    values=[10000, 3200, 1100, 450, 120],
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/funnel3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Funnel 2D](../2d/funnel.md)
