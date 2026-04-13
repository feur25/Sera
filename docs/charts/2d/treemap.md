# Treemap

## Signature

```python
sp.build_treemap(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    parents: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Treemap — square-based space-filling hierarchy visualization.
Tiles are sized proportionally to their value.

When `parents` is provided, the hierarchy is rendered as nested rectangles.
Without `parents`, a flat treemap is drawn.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Tile labels |
| `values` | `list[float]` | required | Tile sizes |
| `parents` | `list[str] \| None` | `None` | Optional parent labels for hierarchy |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `palette` | `list[int] \| None` | `None` | Custom color palette |
| `background` | `str \| None` | `None` | Background color |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### Flat treemap (market cap)

```python
import seraplot as sp

chart = sp.build_treemap(
    "S&P 500 Market Cap (Sample)",
    labels=["AAPL", "MSFT", "AMZN", "NVDA", "GOOGL", "META", "TSLA"],
    values=[2900, 2800, 1700, 1600, 1500, 1100, 750],
)
```

### Hierarchical treemap

```python
import seraplot as sp

labels  = ["Electronics", "Phones", "Laptops", "Clothing", "Shirts", "Pants"]
parents = ["",            "Electronics","Electronics","","Clothing","Clothing"]
values  = [1, 400, 350, 1, 200, 150]

chart = sp.build_treemap(
    "Revenue by Category",
    labels=labels,
    values=values,
    parents=parents,
)
```

---

## See also

- [Sunburst](sunburst.md)
- [Bar Chart](bar.md)
