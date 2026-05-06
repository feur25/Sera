# Grouped Bar Chart

<div class="lang-en">

## Signature

```python
sp.build_grouped_bar(
    title: str,
    category_labels: list[str],
    series_values: list[float],
    *,
    show_values: bool = False,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    legend_position: str = "right",
    gridlines: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.grouped_bar`

---

## Description

A grouped bar chart places bars for multiple series side by side within each category, enabling direct visual comparison of values across both categories and series simultaneously. `series_values` is a flat list in row-major order: `[cat0_s0, cat0_s1, cat1_s0, cat1_s1, ...]` — the total length must equal `len(category_labels) × n_series`. SeraPlot infers the number of series from `series_names` if provided, otherwise from `len(series_values) / len(category_labels)`. Grouped bar charts are the counterpart to stacked bar charts; use grouped when absolute values matter more than composition.

**Ideal for:**
- Comparing multiple product lines, regions, or periods side by side per category
- A/B testing results across user segments or multiple metrics
- Quarterly comparisons across several KPIs in a single compact view

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | Category labels (X axis) |
| `series_values` | `list[float]` | required | Flat row-major values: `[cat0_s0, cat0_s1, cat1_s0, ...]` |
| `show_values` | `bool` | `False` | Show value labels on bars |
| `series_names` | `list[str] \| None` | `None` | Series names for the legend |
| `palette` | `list[int] \| None` | `None` | Custom color palette as list of hex ints |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `legend_position` | `str` | `"right"` | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `gridlines` | `bool` | `False` | Show gridlines |
| `no_y_axis` | `bool` | `False` | Hide Y axis |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>


</div>

<iframe src="../../previews/grouped-bar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Stacked Bar](stacked-bar.md) — `sp.build_stacked_bar()`
- [Multiline Chart](multiline.md) — `sp.build_multiline()`

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_grouped_bar(
    title: str,
    category_labels: list[str],
    series_values: list[float],
    *,
    show_values: bool = False,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    legend_position: str = "right",
    gridlines: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.grouped_bar`

---

<h2>Description</h2>

Un graphique en barres groupées place les barres de plusieurs séries côte à côte au sein de chaque catégorie, permettant une comparaison visuelle directe des valeurs entre catégories et séries simultanément. `series_values` est une liste plate en ordre ligne-majeur : `[cat0_s0, cat0_s1, cat1_s0, cat1_s1, ...]` — la longueur totale doit être égale à `len(category_labels) × n_séries`. SeraPlot déduit le nombre de séries depuis `series_names` si fourni, sinon depuis `len(series_values) / len(category_labels)`. Les barres groupées sont le pendant des barres empilées ; utilisez le groupé quand les valeurs absolues comptent plus que la composition.

**Idéal pour :**
- Comparer plusieurs lignes de produits, régions ou périodes côte à côte par catégorie
- Résultats de tests A/B sur plusieurs segments d'utilisateurs
- Comparaisons trimestrielles de plusieurs indicateurs clés en une seule vue

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `category_labels` | `list[str]` | requis | Étiquettes des catégories (axe X) |
| `series_values` | `list[float]` | requis | Valeurs en ordre ligne-majeur : `[cat0_s0, cat0_s1, cat1_s0, ...]` |
| `show_values` | `bool` | `False` | Afficher les valeurs sur les barres |
| `series_names` | `list[str] \| None` | `None` | Noms des séries pour la légende |
| `palette` | `list[int] \| None` | `None` | Palette de couleurs personnalisée |
| `width` | `int` | `900` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `legend_position` | `str` | `"right"` | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `gridlines` | `bool` | `False` | Afficher les lignes de grille |
| `no_y_axis` | `bool` | `False` | Masquer l'axe Y |

---

<h2>Retourne</h2>

`Chart`

---


</div>

<iframe src="../../previews/grouped-bar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

<h2>Voir aussi</h2>

- [Graphique en barres](bar.md) — `sp.build_bar_chart()`
- [Barres empilées](stacked-bar.md) — `sp.build_stacked_bar()`
- [Graphique multiligne](multiline.md) — `sp.build_multiline()`

</div>
