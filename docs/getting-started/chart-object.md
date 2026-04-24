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
you don't need anything, you just do make your chart, seraplot has a native display inside

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
Comme dit au précédemment, vous n'avez aucunement besoin de rien, juste de crée votre chart, Seraplot à une fonction native d'affichage

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
