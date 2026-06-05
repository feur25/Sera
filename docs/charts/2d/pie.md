# Pie Charts

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

`sp.pie(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`


## Description

`sp.pie()` is the unified entry point for the entire pie-chart family. The `variant` keyword selects the rendering strategy — all other arguments remain consistent across variants.
## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `center_subtext` | kpi, nested, proportional |
| `center_text` | kpi, nested, proportional |
| `donut` | nested, proportional, subplots |
| `gridlines` | proportional |
| `height` | nested, proportional, subplots |
| `hover` | proportional |
| `labels` | nested, proportional, subplots |
| `legend_position` | proportional |
| `min_label_frac` | proportional |
| `palette` | proportional, subplots |
| `pattern` | proportional |
| `proportional` | proportional, subplots |
| `pull` | basic, donut, exploded, kpi, pattern, proportional, semi |
| `secondary_labels` | nested, proportional |
| `secondary_values` | nested, proportional |
| `series` | proportional, subplots |
| `show_pct` | proportional |
| `sort_order` | proportional |
| `subplot_cols` | proportional, subplots |
| `subplot_titles` | proportional, subplots |
| `title` | nested, proportional, subplots |
| `values` | exploded, kpi, nested, proportional |
| `variant` | proportional |
| `width` | nested, proportional, subplots |
| `x_label` | proportional |
| `y_label` | proportional |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.pie(title, labels=None, values=None, *, variant="basic", **kwargs) -> Chart`


<h2>Description</h2>

`sp.pie()` est le point d'entrée unique pour toute la famille des camemberts. Le paramètre `variant` sélectionne la stratégie de rendu — donut, éclaté, KPI, semi-cercle ou imbriqué — tout en conservant la même API simple.
<h2>Paramètres</h2>

| Paramètre | Utilisé par variantes |
|-----------|----------------------|
| `center_subtext` | kpi, nested, proportional |
| `center_text` | kpi, nested, proportional |
| `donut` | nested, proportional, subplots |
| `gridlines` | proportional |
| `height` | nested, proportional, subplots |
| `hover` | proportional |
| `labels` | nested, proportional, subplots |
| `legend_position` | proportional |
| `min_label_frac` | proportional |
| `palette` | proportional, subplots |
| `pattern` | proportional |
| `proportional` | proportional, subplots |
| `pull` | basic, donut, exploded, kpi, pattern, proportional, semi |
| `secondary_labels` | nested, proportional |
| `secondary_values` | nested, proportional |
| `series` | proportional, subplots |
| `show_pct` | proportional |
| `sort_order` | proportional |
| `subplot_cols` | proportional, subplots |
| `subplot_titles` | proportional, subplots |
| `title` | nested, proportional, subplots |
| `values` | exploded, kpi, nested, proportional |
| `variant` | proportional |
| `width` | nested, proportional, subplots |
| `x_label` | proportional |
| `y_label` | proportional |

---

<h2>Retourne</h2>

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

</div><!-- /lang-fr -->
