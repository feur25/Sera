# Map Charts

<div class="lang-en">

SeraPlot provides geographic chart types for visualizing spatial data on any registered region set — from whole-world country data down to a single country's states or provinces. Every chart type below reads its variants, parameters and region sets live from the same register/inventory system the 2D charts use.

| Chart | Function |
|---|---|
| [Bubble Map](bubble-map.md) | `bubble_map()` |
| [Choropleth](choropleth.md) | `choropleth()` |
| [Flow Map](flow-map.md) | `flow_map()` |

Choropleth and Bubble Map both include a real orthographic ("globe from space") and polar (azimuthal, pole-centered) projection, computed from an actual pixel-to-latitude/longitude inversion of the world map — not a cosmetic re-skin.

## Any country, any region

`map` selects which registered geographic region set to draw (`"world"` for individual countries, `"usa_states"` for US states, and any future region set added the same way); `region` restricts drawing to one named group inside that set (a continent or political/economic bloc for `"world"`, a Census region for `"usa_states"`).

## Bridging to 2D charts

`sp.region_labels(map=..., group=...)` returns the same ordered label list a map for that region draws — build a map and a matching bar/line chart from the exact same named entities:

```python
import seraplot as sp

labels = sp.region_labels(map="usa_states", group="West")
values = [my_data[l] for l in labels]

geo = sp.bubble_map("Western Sales", labels=labels, values=values, map="usa_states", region="West", variant="proportional")
bars = sp.bar("Western Sales", labels=labels, values=values)
```

</div>

<div class="lang-fr">

SeraPlot propose des types de graphiques géographiques pour visualiser des données spatiales sur n'importe quel ensemble de régions enregistré — des données par pays à l'échelle mondiale jusqu'aux états ou provinces d'un seul pays. Chaque type de graphique ci-dessous lit ses variantes, paramètres et ensembles de régions en direct depuis le même système de register/inventory que les graphiques 2D.

| Graphique | Fonction |
|-----------|----------|
| [Carte à bulles](bubble-map.md) | `bubble_map()` |
| [Choropleth](choropleth.md) | `choropleth()` |
| [Carte de flux](flow-map.md) | `flow_map()` |

Choropleth et Bubble Map incluent tous deux une vraie projection orthographique (« globe vu de l'espace ») et polaire (azimutale, centrée sur un pôle), calculée à partir d'une véritable inversion pixel vers latitude/longitude de la carte du monde — pas un simple habillage cosmétique.

## N'importe quel pays, n'importe quelle région

`map` sélectionne quel ensemble de régions géographiques enregistré dessiner (`"world"` pour les pays individuels, `"usa_states"` pour les états américains, et tout futur ensemble de régions ajouté de la même façon) ; `region` restreint le dessin à un seul groupe nommé de cet ensemble (un continent ou un bloc politique/économique pour `"world"`, une région de recensement pour `"usa_states"`).

## Passerelle vers les graphiques 2D

`sp.region_labels(map=..., group=...)` retourne la même liste de labels ordonnée qu'une carte dessine pour cette région — construisez une carte et un graphique à barres/lignes assorti depuis exactement les mêmes entités nommées :

```python
import seraplot as sp

labels = sp.region_labels(map="usa_states", group="West")
values = [my_data[l] for l in labels]

geo = sp.bubble_map("Ventes de l'Ouest", labels=labels, values=values, map="usa_states", region="West", variant="proportional")
bars = sp.bar("Ventes de l'Ouest", labels=labels, values=values)
```

</div>
