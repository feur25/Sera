# Slideshow

## Signature

```python
sp.build_slideshow(
    charts: list[Chart],
    *,
    title: str = "",
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    autoplay: bool = False,
    interval_ms: int = 3000,
) -> Chart
```

---

## Description

Wraps multiple charts in an interactive slideshow with Previous / Next navigation controls.
All charts are pre-rendered; switching slides requires no server round-trip.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `charts` | `list[Chart]` | required | Chart objects to display |
| `title` | `str` | `""` | Slideshow title |
| `width` | `int` | `1000` | Container width in pixels |
| `height` | `int` | `600` | Container height in pixels |
| `background` | `str \| None` | `None` | Background color |
| `autoplay` | `bool` | `False` | Auto-advance slides |
| `interval_ms` | `int` | `3000` | Auto-advance interval in milliseconds |

---

## Returns

`Chart` (composite)

---

## Examples

### Quarterly report slideshow

```python
import seraplot as sp

slides = [
    sp.build_bar_chart("Q1 Revenue", labels=["A","B","C"], values=[120,80,95]),
    sp.build_line_chart("Growth Trend", labels=["Jan","Feb","Mar"], values=[10,14,18]),
    sp.build_pie_chart("Market Share", labels=["Us","Them"], values=[55,45]),
]

deck = sp.build_slideshow(slides, title="Q1 Board Deck", autoplay=False)
```

---

## See also

- [Grid Layout](grid.md)
