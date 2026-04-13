# Auto Display (Jupyter)

## Signature

```python
sp.set_auto_display(enabled: bool) -> None
```

---

## Description

Controls whether `Chart` objects are automatically rendered inline in Jupyter notebooks
when they are the last expression of a cell.

| State | Behavior |
|-------|----------|
| `True` (default) | `chart` at end of cell → rendered immediately |
| `False` | Must call `display(chart)` or `chart.show()` explicitly |

---

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `enabled` | `bool` | `True` to enable auto-display, `False` to disable |

---

## Examples

### Disable auto-display to batch charts

```python
import seraplot as sp

sp.set_auto_display(False)

charts = []
for name, values in datasets.items():
    charts.append(sp.build_bar_chart(name, labels=["A","B","C"], values=values))

for c in charts:
    c.show()
```

### Re-enable

```python
sp.set_auto_display(True)
```

---

## See also

- [Chart Object](../getting-started/chart-object.md)
- [Background](background.md)
