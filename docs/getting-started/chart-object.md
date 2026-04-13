# Chart Object / Objet Chart

Every SeraPlot function returns a `Chart` object.

Chaque fonction SeraPlot retourne un objet `Chart`.

---

## Properties / Propriétés

| Property | Type | Description |
|----------|------|-------------|
| `html` | `str` | Full self-contained HTML string / Chaîne HTML complète autonome |

```python
chart = sp.build_bar_chart("Title", labels=["A"], values=[1.0])

print(type(chart.html))
print(len(chart.html), "bytes")
```

---

## Auto-display in Jupyter / Affichage automatique dans Jupyter

Charts display automatically when created inside a Jupyter cell.

Les graphiques s'affichent automatiquement quand ils sont créés dans une cellule Jupyter.

To disable auto-display / Pour désactiver l'affichage automatique :

```python
sp.set_auto_display(False)

chart = sp.build_bar_chart("My Chart", labels=["A"], values=[1.0])
```

---

## Manual display / Affichage manuel

```python
from IPython.display import HTML, display

chart = sp.build_bar_chart("Title", labels=["A"], values=[1.0])
display(HTML(chart.html))
```

---

## Export to file / Export vers un fichier

```python
chart = sp.build_pie_chart("Distribution", labels=["X", "Y", "Z"], values=[30.0, 40.0, 30.0])

with open("output.html", "w", encoding="utf-8") as f:
    f.write(chart.html)
```
