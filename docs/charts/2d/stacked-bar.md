# Stacked Bar Chart

<div class="lang-en">


---

## Description

A stacked bar chart divides each bar into colored segments representing sub-series contributions to a total. `series_values` is a flat list laid out in row-major order: all values for series 1 first, then all values for series 2, etc. If there are `n` categories and `k` series, `series_values` has length `n × k`. The chart is ideal for seeing both the total magnitude and the part-to-whole composition in a single view. Use `no_y_axis` with `show_values` for a cleaner self-labelled layout.

**Ideal for:**
- Tracking total and component breakdown simultaneously (revenue by product line over years)
- Part-to-whole composition across categories
- Budget allocation or resource distribution comparisons

---

## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `bar_gap` | grouped_stacked, relative |
| `bargroup_gap` | grouped_stacked |
| `category_labels` | grouped, grouped_stacked, marimekko, multicategory, relative |
| `color_groups` | basic |
| `color_hex` | basic, deluxe, pictogram, prism |
| `corner_radius` | grouped_stacked, relative |
| `gridlines` | basic, deluxe, grouped, grouped_stacked, marimekko, multicategory, prism, relative |
| `height` | all |
| `hover` | basic, deluxe, grouped, prism |
| `icon_size` | pictogram |
| `labels` | basic, deluxe, pictogram, prism |
| `legend_position` | grouped, grouped_stacked, marimekko, multicategory, relative |
| `max_icons_per_column` | pictogram |
| `offset_groups` | grouped_stacked |
| `orientation` | grouped |
| `palette` | all |
| `series` | grouped, grouped_stacked, marimekko, multicategory, relative |
| `show_text` | basic, deluxe, grouped, marimekko, prism |
| `sort_order` | basic, grouped |
| `super_categories` | multicategory |
| `title` | all |
| `unit_description` | pictogram |
| `units_per_icon` | pictogram |
| `values` | basic, deluxe, multicategory, pictogram, prism |
| `width` | all |
| `widths` | marimekko |
| `x_label` | basic, grouped, grouped_stacked, marimekko, multicategory, relative |
| `y_label` | basic, grouped, grouped_stacked, marimekko, multicategory, relative |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
</div>

<iframe src="../../previews/stacked-bar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

</div>

<div class="lang-fr">


---

<h2>Description</h2>

Un graphique en barres empilées divise chaque barre en segments colorés représentant les contributions de sous-séries à un total. `series_values` est une liste plate en ordre row-major : toutes les valeurs de la série 1 en premier, puis toutes les valeurs de la série 2, etc. Si on a `n` catégories et `k` séries, `series_values` a une longueur de `n × k`. Le graphique est idéal pour voir à la fois la magnitude totale et la composition partie-à-tout en une seule vue.

**Idéal pour :**
- Suivre simultanément le total et la décomposition par composant (revenus par ligne de produits sur plusieurs années)
- Comparaisons de composition partie-à-tout entre catégories
- Comparaisons d'allocation budgétaire ou de distribution de ressources

---

<h2>Paramètres</h2>

| Paramètre | Utilisé par variantes |
|-----------|----------------------|
| `bar_gap` | grouped_stacked, relative |
| `bargroup_gap` | grouped_stacked |
| `category_labels` | grouped, grouped_stacked, marimekko, multicategory, relative |
| `color_groups` | basic |
| `color_hex` | basic, deluxe, pictogram, prism |
| `corner_radius` | grouped_stacked, relative |
| `gridlines` | basic, deluxe, grouped, grouped_stacked, marimekko, multicategory, prism, relative |
| `height` | toutes |
| `hover` | basic, deluxe, grouped, prism |
| `icon_size` | pictogram |
| `labels` | basic, deluxe, pictogram, prism |
| `legend_position` | grouped, grouped_stacked, marimekko, multicategory, relative |
| `max_icons_per_column` | pictogram |
| `offset_groups` | grouped_stacked |
| `orientation` | grouped |
| `palette` | toutes |
| `series` | grouped, grouped_stacked, marimekko, multicategory, relative |
| `show_text` | basic, deluxe, grouped, marimekko, prism |
| `sort_order` | basic, grouped |
| `super_categories` | multicategory |
| `title` | toutes |
| `unit_description` | pictogram |
| `units_per_icon` | pictogram |
| `values` | basic, deluxe, multicategory, pictogram, prism |
| `width` | toutes |
| `widths` | marimekko |
| `x_label` | basic, grouped, grouped_stacked, marimekko, multicategory, relative |
| `y_label` | basic, grouped, grouped_stacked, marimekko, multicategory, relative |

---

<h2>Retourne</h2>

`Chart`

---

</div>

<iframe src="../../previews/stacked-bar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

</div>
