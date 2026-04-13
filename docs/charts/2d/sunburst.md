# Sunburst Chart

## Signature

```python
sp.build_sunburst(
    title: str,
    labels: list[str],
    parents: list[str],
    values: list[float],
    *,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Hierarchical sunburst chart. Nodes are arranged in concentric rings
radiating outward from the root.

Graphique sunburst hiérarchique. Les nœuds sont disposés en anneaux
concentriques rayonnant à partir de la racine.

The `labels` and `parents` lists define a tree: each entry in `labels[i]` has
`parents[i]` as its parent (`""` for root nodes). `values` controls arc size.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Node labels |
| `parents` | `list[str]` | required | Parent label per node (`""` = root) |
| `values` | `list[float]` | required | Node size values |
| `width` | `int` | `700` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `palette` | `list[int] \| None` | `None` | Custom colors |
| `background` | `str \| None` | `None` | Background color |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns / Retour

`Chart`

---

## Examples / Exemples

### Company org chart

```python
import seraplot as sp

labels  = ["Corp", "Sales", "Tech", "HR", "B2B", "B2C", "Frontend", "Backend"]
parents = ["",       "Corp", "Corp","Corp","Sales","Sales","Tech",    "Tech"]
values  = [1,        40,     50,    10,    25,     15,     30,        20]

chart = sp.build_sunburst(
    "Headcount by Department",
    labels=labels,
    parents=parents,
    values=values,
)
```

---

## See also / Voir aussi

- [Treemap](treemap.md)
- [Pie Chart](pie.md)
- [Sunburst 3D](../3d/sunburst3d.md)
