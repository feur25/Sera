# Palette & Colors

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

There is no standalone `sp.PALETTE_*` constant — the built-in color sets ship
bundled inside a **theme** (background + palette + gridlines together). Call
`sp.theme(name)` to apply one, or read [Themes](themes.md) for the full list
and every palette's exact hex values.

```python
import seraplot as sp

print(sp.themes())
# ['dark', 'light', 'scientific', 'apple', 'notion', 'minimal', 'neon']

sp.theme("dark")
chart = sp.build_bar_chart("Revenue", labels=["A", "B", "C"], values=[100, 200, 150])
sp.reset_theme()
```

Pass any list of hex ints as the `palette` parameter to override the palette
for a single chart, independent of the active theme:

```python
chart = sp.build_bar_chart(
    "Revenue",
    labels=["A", "B", "C"],
    values=[100, 200, 150],
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
| `color_low` / `color_mid` / `color_high` | `int` | Min/mid/max heatmap colors |
| `color_up` / `color_down` | `int` | Rising/falling candle colors |
| `color_pos` / `color_neg` / `color_total` | `int` | Positive/negative/total bar (waterfall) |

---

## Example

```python
import seraplot as sp

chart = sp.build_heatmap(
    "Correlation",
    labels=["A", "B", "C"],
    col_labels=["A", "B", "C"],
    values=[1, 0.8, 0.2, 0.8, 1, 0.5, 0.2, 0.5, 1],
    color_low=0xfaf5ff,
    color_mid=0xa78bfa,
    color_high=0x4c1d95,
)
```

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

Il n'existe pas de constante `sp.PALETTE_*` autonome — les jeux de couleurs
intégrés sont livrés à l'intérieur d'un **thème** (fond + palette + quadrillage
ensemble). Appelez `sp.theme(name)` pour en appliquer un, ou consultez
[Thèmes](themes.md) pour la liste complète et les valeurs hex exactes de
chaque palette.

```python
import seraplot as sp

print(sp.themes())
# ['dark', 'light', 'scientific', 'apple', 'notion', 'minimal', 'neon']

sp.theme("dark")
chart = sp.build_bar_chart("Revenus", labels=["A", "B", "C"], values=[100, 200, 150])
sp.reset_theme()
```

Passez n'importe quelle liste d'entiers hex comme paramètre `palette` pour
surcharger la palette d'un seul chart, indépendamment du thème actif :

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

## Exemple

```python
import seraplot as sp

chart = sp.build_heatmap(
    "Corrélation",
    labels=["A","B","C"],
    col_labels=["A","B","C"],
    values=[1, 0.8, 0.2, 0.8, 1, 0.5, 0.2, 0.5, 1],
    color_low=0xfaf5ff,
    color_mid=0xa78bfa,
    color_high=0x4c1d95,
)
```

---

</div>
