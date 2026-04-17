# Palette & Couleurs

<div class="lang-en">

## Overview

SeraPlot represents colors as 24-bit RGB integers (`int`) or CSS strings (`str`).

| Format | Example | Usage |
|--------|---------|-------|
| Hex integer | `0x6366f1` | `color_hex`, `palette` lists |
| CSS hex string | `"#6366f1"` | `background`, `bg_color` |
| CSS named color | `"navy"` | `background`, `bg_color` |

---

## Built-in Palettes

```python
import seraplot as sp

sp.PALETTE_DEFAULT   # Indigo-based multi-color sequence
sp.PALETTE_COOL      # Blues and purples
sp.PALETTE_WARM      # Reds, oranges, yellows
sp.PALETTE_EARTH     # Browns and greens
sp.PALETTE_MONO      # Greyscale
```

---

## Color Utility Reference

| Parameter name | Accepts | Description |
|---------------|---------|-------------|
| `color_hex` | `int` | Single element color |
| `palette` | `list[int]` | Multi-element color list |
| `background` | `str` | Chart canvas background |
| `color_low` / `color_high` | `int` | Min/max heatmap colors |
| `color_up` / `color_down` | `int` | Rising/falling candle colors |

</div>

<div class="lang-fr">

## Aperçu

SeraPlot représente les couleurs sous forme d'entiers RGB 24 bits (`int`) ou de chaînes CSS (`str`).

| Format | Exemple | Utilisation |
|--------|---------|-------------|
| Entier hex | `0x6366f1` | `color_hex`, listes `palette` |
| Chaîne CSS hex | `"#6366f1"` | `background`, `bg_color` |
| Nom CSS | `"navy"` | `background`, `bg_color` |

---

## Palettes intégrées

```python
import seraplot as sp

sp.PALETTE_DEFAULT   # Séquence multicolore à base d'indigo
sp.PALETTE_COOL      # Bleus et violets
sp.PALETTE_WARM      # Rouges, oranges, jaunes
sp.PALETTE_EARTH     # Bruns et verts
sp.PALETTE_MONO      # Niveaux de gris
```

Passez n'importe quelle liste d'entiers hex comme paramètre `palette` :

```python
chart = sp.build_bar_chart(
    "Revenus",
    labels=["A","B","C"],
    values=[100,200,150],
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e],
)
```

---

## Référence des paramètres de couleur

| Nom du paramètre | Accepte | Description |
|-----------------|---------|-------------|
| `color_hex` | `int` | Couleur d'un élément unique |
| `palette` | `list[int]` | Liste de couleurs multi-éléments |
| `background` | `str` | Fond du canevas HTML |
| `bg_color` | `str` | Fond du canevas 3D |
| `color_low` | `int` | Couleur valeur min (heatmaps, choroplèthes) |
| `color_mid` | `int` | Couleur valeur médiane |
| `color_high` | `int` | Couleur valeur max |
| `color_up` | `int` | Couleur bougie montante |
| `color_down` | `int` | Couleur bougie descendante |
| `color_pos` | `int` | Barre positive (cascade) |
| `color_neg` | `int` | Barre négative (cascade) |
| `color_total` | `int` | Barre totale (cascade) |

---

## Exemples

```python
import seraplot as sp

chart = sp.build_heatmap(
    "Corrélation",
    labels=["A","B","C"],
    flat_matrix=[1, 0.8, 0.2, 0.8, 1, 0.5, 0.2, 0.5, 1],
    color_low=0xfaf5ff,
    color_mid=0xa78bfa,
    color_high=0x4c1d95,
)
```

---

## Voir aussi

- [Configuration de fond](background.md)
- [Infobulles de survol](hover.md)

</div>


| Format | Example | Usage |
|--------|---------|-------|
| Hex integer | `0x6366f1` | `color_hex`, `palette` lists |
| CSS hex string | `"#6366f1"` | `background`, `bg_color` |
| CSS named color | `"navy"` | `background`, `bg_color` |

---

## Built-in Palettes

SeraPlot ships with several built-in color sequences, accessible as module constants:

```python
import seraplot as sp

sp.PALETTE_DEFAULT   # Indigo-based multi-color sequence
sp.PALETTE_COOL      # Blues and purples
sp.PALETTE_WARM      # Reds, oranges, yellows
sp.PALETTE_EARTH     # Browns and greens
sp.PALETTE_MONO      # Greyscale
```

Pass any list of hex ints as a `palette` parameter:

```python
chart = sp.build_bar_chart(
    "Revenue",
    labels=["A","B","C"],
    values=[100,200,150],
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e],
)
```

---

## Color Utility Reference

| Parameter name | Accepts | Description |
|---------------|---------|-------------|
| `color_hex` | `int` | Single element color |
| `palette` | `list[int]` | Multi-element color list |
| `background` | `str` | Chart canvas background |
| `bg_color` | `str` | 3D canvas background |
| `color_low` | `int` | Min value color (heatmaps, choropleths) |
| `color_mid` | `int` | Mid value color |
| `color_high` | `int` | Max value color |
| `color_up` | `int` | Rising candle color |
| `color_down` | `int` | Falling candle color |
| `color_pos` | `int` | Positive waterfall bar |
| `color_neg` | `int` | Negative waterfall bar |
| `color_total` | `int` | Total waterfall bar |

---

## Examples

### Purple gradient heatmap

```python
import seraplot as sp

chart = sp.build_heatmap(
    "Correlation",
    labels=["A","B","C"],
    flat_matrix=[1, 0.8, 0.2, 0.8, 1, 0.5, 0.2, 0.5, 1],
    color_low=0xfaf5ff,
    color_mid=0xa78bfa,
    color_high=0x4c1d95,
)
```

---

## See also

- [Background Config](background.md)
- [Hover Tooltips](hover.md)
