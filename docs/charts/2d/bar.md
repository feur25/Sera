# Bar Charts

<div class="lang-en">

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}

</style>
## Signature

`sp.bar(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`


## Description

`sp.bar()` is the unified entry point for the entire bar-chart family. The `variant` keyword selects the rendering strategy — all other arguments remain consistent across variants.
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

`Chart` — object with `.html` property and `.show()` method.

---

---

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.bar(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`


<h2>Description</h2>

`sp.bar()` est le point d'entrée unique pour toute la famille de graphiques en barres. Le paramètre `variant` sélectionne la stratégie de rendu.
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

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

---

</div><!-- /lang-fr -->
