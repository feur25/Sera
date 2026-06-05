# Histogram Charts

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

`sp.histogram(title, values, *, variant="basic", bins=0, overlay_values=None, color_groups=None, series_names=None, **kwargs) -> Chart`


## Description

`sp.histogram()` is the unified entry point for the entire histogram family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. Histograms are the canonical way to visualize the distribution of a single numeric variable; SeraPlot adds horizontal layout, density normalization, cumulative distribution, stacked groups, A/B overlay and step outline — all in pure Rust SVG, thousands of times faster than Plotly.
## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `bins` | all |
| `categories` | overlay, stacked |
| `color` | basic, cumulative, horizontal, normalized, overlay, step |
| `count_scale` | basic |
| `gap` | basic, deluxe, horizontal, normalized, overlay, stacked |
| `gridlines` | all |
| `height` | all |
| `hover` | all |
| `opacity` | basic, cumulative, horizontal, normalized |
| `overlay_color` | basic, overlay |
| `overlay_values` | basic, overlay |
| `palette` | overlay, stacked |
| `series_names` | basic, overlay |
| `show_counts` | basic, deluxe, horizontal |
| `sort_order` | basic |
| `stroke_width` | step |
| `title` | all |
| `values` | all |
| `width` | all |
| `x_label` | basic, cumulative, horizontal, normalized, overlay, stacked, step |
| `y_label` | basic, cumulative, horizontal, normalized, overlay, stacked, step |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
</style>
<h2>Signature</h2>

`sp.histogram(title, values, *, variant="basic", bins=0, overlay_values=None, color_groups=None, series_names=None, **kwargs) -> Chart`


<h2>Description</h2>

`sp.histogram()` est le point d'entrée unifié de toute la famille histogramme. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments gardent le même nom d'une variante à l'autre. Les histogrammes sont la façon canonique de visualiser la distribution d'une variable numérique ; SeraPlot ajoute layout horizontal, normalisation densité, distribution cumulative, groupes empilés, superposition A/B et contour en escalier — le tout en SVG Rust pur, des milliers de fois plus rapide que Plotly.
<h2>Paramètres</h2>

| Paramètre | Utilisé par variantes |
|-----------|----------------------|
| `bins` | toutes |
| `categories` | overlay, stacked |
| `color` | basic, cumulative, horizontal, normalized, overlay, step |
| `count_scale` | basic |
| `gap` | basic, deluxe, horizontal, normalized, overlay, stacked |
| `gridlines` | toutes |
| `height` | toutes |
| `hover` | toutes |
| `opacity` | basic, cumulative, horizontal, normalized |
| `overlay_color` | basic, overlay |
| `overlay_values` | basic, overlay |
| `palette` | overlay, stacked |
| `series_names` | basic, overlay |
| `show_counts` | basic, deluxe, horizontal |
| `sort_order` | basic |
| `stroke_width` | step |
| `title` | toutes |
| `values` | toutes |
| `width` | toutes |
| `x_label` | basic, cumulative, horizontal, normalized, overlay, stacked, step |
| `y_label` | basic, cumulative, horizontal, normalized, overlay, stacked, step |

---

<h2>Retour</h2>

`Chart` — objet exposant `.html` et `.show()`.

---

</div>
