# Heatmap

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

`sp.heatmap(title, labels=None, values=None, *, variant="basic", col_labels=None, **kwargs) -> Chart`


## Description

`sp.heatmap()` is the unified entry point for the entire heatmap family. The `variant` keyword selects the rendering strategy — every other argument stays consistent across variants. Cell colors are computed in pure Rust, no NumPy required. The matrix is passed as a flat list of length `len(labels) * len(col_labels)` (row-major).
## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `col_labels` | bubble, cluster, marginal, pivot, unequal |
| `colorscale` | confusion |
| `contour_levels` | contour |
| `discrete_steps` | discrete |
| `diverging` | bubble, marginal, pivot |
| `flat_matrix` | bubble, cluster, marginal, pivot |
| `height` | bubble, marginal, pivot |
| `hover` | bubble, marginal, pivot |
| `palette` | categorical |
| `row_labels` | bubble, cluster, marginal, pivot, unequal |
| `show_values` | categorical |
| `title` | bubble, marginal, pivot |
| `width` | bubble, marginal, pivot |
| `x_widths` | unequal |
| `y_heights` | unequal |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.heatmap(title, matrix, *, variant="basic", x_labels=None, y_labels=None, **kwargs) -> Chart`


<h2>Description</h2>

`sp.heatmap()` est le point d'entrée unique pour toute la famille des cartes de chaleur. Le paramètre `variant` sélectionne la stratégie de rendu — annotée, catégorielle, log, contour, clustering hiérarchique, etc. — tout en partageant la même API de base.
<h2>Paramètres</h2>

| Paramètre | Utilisé par variantes |
|-----------|----------------------|
| `col_labels` | bubble, cluster, marginal, pivot, unequal |
| `colorscale` | confusion |
| `contour_levels` | contour |
| `discrete_steps` | discrete |
| `diverging` | bubble, marginal, pivot |
| `flat_matrix` | bubble, cluster, marginal, pivot |
| `height` | bubble, marginal, pivot |
| `hover` | bubble, marginal, pivot |
| `palette` | categorical |
| `row_labels` | bubble, cluster, marginal, pivot, unequal |
| `show_values` | categorical |
| `title` | bubble, marginal, pivot |
| `width` | bubble, marginal, pivot |
| `x_widths` | unequal |
| `y_heights` | unequal |

---

<h2>Retourne</h2>

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

</div><!-- /lang-fr -->
