# Infobulles personnalisées

<div class="lang-en">

## Signature

```python
chart.hover_json(slots_json: str) -> Chart
```

`hover_json` is a chainable `Chart` method — not a standalone `sp.` function.
It takes a single JSON string and returns a new `Chart` with the tooltip
override applied, the same chaining shape as every other `Chart` method
(`chart.title(...).hover_json(...).show()`).

---

## Description

Overrides the tooltip content shown per data point, independent of the
values actually plotted — useful when the label/value combination on the
axes isn't the whole story (currency formatting, a delta vs. last period, a
category not otherwise on the chart).

---

## Example

```python
import json
import seraplot as sp

chart = sp.bar(title="Revenus par produit", labels=["Alpha", "Beta", "Gamma"], values=[420, 380, 290])

hover_payload = json.dumps({
    "fields": ["Produit", "Revenu (€)", "Croissance"],
    "values": [
        ["Alpha", 420, "+12%"],
        ["Beta",  380, "+5%"],
        ["Gamma", 290, "-3%"],
    ],
})

chart = chart.hover_json(hover_payload)
```

</div>

<div class="lang-fr">

## Signature

```python
chart.hover_json(slots_json: str) -> Chart
```

`hover_json` est une méthode chaînable sur `Chart` — pas une fonction `sp.`
autonome. Elle prend une seule chaîne JSON et retourne un nouveau `Chart`
avec l'override de tooltip appliqué, la même forme chaînable que toute autre
méthode `Chart` (`chart.title(...).hover_json(...).show()`).

---

## Description

Remplace le contenu du tooltip affiché par point de donnée, indépendamment
des valeurs réellement tracées — utile quand la combinaison label/valeur des
axes ne raconte pas toute l'histoire (formatage monétaire, un delta vs la
période précédente, une catégorie absente du graphique lui-même).

---

## Exemple

```python
import json
import seraplot as sp

chart = sp.bar(title="Revenus par produit", labels=["Alpha", "Beta", "Gamma"], values=[420, 380, 290])

hover_payload = json.dumps({
    "fields": ["Produit", "Revenu (€)", "Croissance"],
    "values": [
        ["Alpha", 420, "+12%"],
        ["Beta",  380, "+5%"],
        ["Gamma", 290, "-3%"],
    ],
})

chart = chart.hover_json(hover_payload)
```

</div>
