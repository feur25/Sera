# Chart Object

<div class="lang-en">

Every SeraPlot function returns a `Chart` object — a thin wrapper around a
complete, standalone HTML string.

---

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `html` | `str` | Full self-contained HTML string |

```python
chart = sp.build_bar_chart("Title", labels=["A", "B"], values=[1.0, 2.0])
print(type(chart.html))   # <class 'str'>
print(len(chart.html))    # typically 20,000–90,000 bytes
```

---

## Auto-display in Jupyter

Charts display automatically when created inside a Jupyter cell. No `display()` call needed.

To disable:

```python
sp.set_auto_display(False)
chart = sp.build_bar_chart("My Chart", labels=["A"], values=[1.0])
```

---

## Manual display

```python
from IPython.display import HTML, display
display(HTML(chart.html))
```

---

## Export to file

```python
chart.save("output.html")
# or
with open("output.html", "w", encoding="utf-8") as f:
    f.write(chart.html)
```

---

## Method chaining

```python
chart = (
    sp.build_bar_chart("Sales", labels, values)
    .set_bg("#1e1e2e")
    .show_grid()
    .no_legend()
    .set_font_size(14)
)
```

See **[Chart Methods](chart-methods.md)** for the full reference.

</div>

<div class="lang-fr">

Chaque fonction SeraPlot retourne un objet `Chart` — un wrapper léger autour d'une chaîne HTML complète et autonome.

---

## Propriétés

| Propriété | Type | Description |
|-----------|------|-------------|
| `html` | `str` | Chaîne HTML complète et autonome |

```python
chart = sp.build_bar_chart("Titre", labels=["A", "B"], values=[1.0, 2.0])
print(len(chart.html))    # typiquement 20 000 à 90 000 octets
```

---

## Affichage automatique dans Jupyter

Les graphiques s'affichent automatiquement à la fin d'une cellule Jupyter. Aucun appel à `display()` n'est nécessaire.

Pour désactiver :

```python
sp.set_auto_display(False)
```

---

## Affichage manuel

```python
from IPython.display import HTML, display
display(HTML(chart.html))
```

---

## Export vers fichier

```python
chart.save("sortie.html")
# ou
with open("sortie.html", "w", encoding="utf-8") as f:
    f.write(chart.html)
```

---

## Chaînage de méthodes

```python
chart = (
    sp.build_bar_chart("Ventes", labels, values)
    .set_bg("#1e1e2e")
    .show_grid()
    .no_legend()
    .set_font_size(14)
)
```

Voir **[Méthodes du graphique](chart-methods.md)** pour la référence complète.

</div>


---

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `html` | `str` | Full self-contained HTML string |

```python
chart = sp.build_bar_chart("Title", labels=["A", "B"], values=[1.0, 2.0])
print(type(chart.html))   # <class 'str'>
print(len(chart.html))    # typically 20,000–90,000 bytes
```

---

## Auto-display in Jupyter

Charts display automatically when created inside a Jupyter cell. No `display()`
call needed — SeraPlot detects the Jupyter environment and renders the chart
inline.

To disable auto-display:

```python
sp.set_auto_display(False)
chart = sp.build_bar_chart("My Chart", labels=["A"], values=[1.0])
# chart is returned silently — display it manually when ready
```

---

## Manual display

```python
from IPython.display import HTML, display

chart = sp.build_bar_chart("Title", labels=["A"], values=[1.0])
display(HTML(chart.html))
```

---

## Export to file

```python
chart = sp.build_pie_chart("Distribution", labels=["X", "Y", "Z"], values=[30.0, 40.0, 30.0])
chart.save("output.html")
```

Or write the HTML string directly:

```python
with open("output.html", "w", encoding="utf-8") as f:
    f.write(chart.html)
```

---

## Method chaining

Every method on `Chart` returns a new `Chart`, so calls can be chained:

```python
chart = (
    sp.build_bar_chart("Sales", labels, values)
    .set_bg("#1e1e2e")
    .show_grid()
    .no_legend()
    .set_font_size(14)
)
```

See **[Chart Methods](chart-methods.md)** for the full reference.
