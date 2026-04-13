# Grid Layout

## Signature

```python
sp.build_grid(
    charts: list[Chart],
    *,
    cols: int = 2,
    width: int = 1200,
    height: int = 800,
    background: str | None = None,
    gap: int = 12,
    title: str = "",
) -> Chart
```

---

## Description

Arranges multiple charts in a responsive grid layout within a single HTML output.

Arrange plusieurs graphiques dans une mise en page grille responsive dans un seul fichier HTML.

Charts are placed left-to-right, top-to-bottom. When `len(charts)` is not divisible by `cols`, the last row is left-aligned.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `charts` | `list[Chart]` | required | Chart objects to embed |
| `cols` | `int` | `2` | Number of columns |
| `width` | `int` | `1200` | Total grid container width in pixels |
| `height` | `int` | `800` | Total grid container height in pixels |
| `background` | `str \| None` | `None` | Grid background color |
| `gap` | `int` | `12` | Gap in pixels between cells |
| `title` | `str` | `""` | Optional header above the grid |

---

## Returns

`Chart` (composite)

---

## Examples

### Dashboard with 4 charts

```python
import seraplot as sp

bar   = sp.build_bar_chart("Revenue", labels=["A","B","C"], values=[100,200,150])
pie   = sp.build_pie_chart("Share",   labels=["A","B"],     values=[60,40])
line  = sp.build_line_chart("Trend",  labels=["Jan","Feb","Mar"], values=[10,20,15])
hist  = sp.build_histogram("Dist",    values=[1,2,2,3,3,3,4,4,5])

dashboard = sp.build_grid(
    [bar, pie, line, hist],
    cols=2,
    title="Q1 Dashboard",
)
```

---

## See also

- [Slideshow](slideshow.md)
