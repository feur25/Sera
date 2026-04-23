# Faceting / Small Multiples

<div class="lang-en">

Combine multiple charts into a grid layout — one call instead of manually composing HTML.

## Python

```python
import seraplot as sp

charts = [sp.bar([1,2,3], ["a","b","c"], title=region) for region in ["EU","US","APAC"]]
grid = sp.facet(charts, cols=3, gap=12, cell_height=400)
grid.save("regions.html")
```

## JavaScript

```javascript
import { buildGrid } from "seraplot";
const html = buildGrid(JSON.stringify({ charts: [chart1Html, chart2Html], cols: 2 }));
```

## Parameters

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `charts` | `list[Chart]` | required | Charts to lay out |
| `cols` | `int` | `2` | Columns per row |
| `gap` | `int` | `16` | Gap in pixels |
| `bg` | `str` | `None` | Background color |
| `cell_height` | `int` | `520` | Per-cell height |

</div>

<div class="lang-fr">

Combine plusieurs charts dans une grille — un seul appel au lieu de composer le HTML manuellement.

## Python

```python
import seraplot as sp

charts = [sp.bar([1,2,3], ["a","b","c"], title=region) for region in ["EU","US","APAC"]]
grid = sp.facet(charts, cols=3, gap=12, cell_height=400)
grid.save("regions.html")
```

## JavaScript

```javascript
import { buildGrid } from "seraplot";
const html = buildGrid(JSON.stringify({ charts: [chart1Html, chart2Html], cols: 2 }));
```

## Paramètres

| Nom | Type | Défaut | Description |
|-----|------|--------|-------------|
| `charts` | `list[Chart]` | requis | Charts à disposer |
| `cols` | `int` | `2` | Colonnes par ligne |
| `gap` | `int` | `16` | Espacement en pixels |
| `bg` | `str` | `None` | Couleur de fond |
| `cell_height` | `int` | `520` | Hauteur par cellule |

</div>
