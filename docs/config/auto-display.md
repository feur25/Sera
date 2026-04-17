# Affichage automatique (Jupyter)

<div class="lang-en">

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

```python
sp.set_auto_display(False)
charts = []
for name, values in datasets.items():
    charts.append(sp.build_bar_chart(name, labels=["A","B","C"], values=values))
for c in charts:
    c.show()
```

</div>

<div class="lang-fr">

## Description

Contrôle si les objets `Chart` sont automatiquement rendus dans les cellules Jupyter.

| État | Comportement |
|------|-------------|
| `True` (défaut) | Le graphique en fin de cellule est rendu immédiatement |
| `False` | Il faut appeler `display(chart)` ou `chart.show()` explicitement |

## Paramètres

| Paramètre | Type | Description |
|-----------|------|-------------|
| `enabled` | `bool` | `True` pour activer, `False` pour désactiver |

</div>

