# Box Plot

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

`sp.boxplot(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`


## Description

`sp.boxplot()` is the unified entry point for the entire box-plot family. The `variant` keyword selects the rendering strategy — every other argument stays consistent across variants. Quartiles, 1.5×IQR whiskers and outliers are computed in pure Rust without NumPy or pandas.
## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `boxen_depth` | letter_value |
| `category_labels` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |
| `fill_opacity` | basic, grouped, horizontal, rainbow, violin |
| `gridlines` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |
| `height` | horizontal |
| `hover` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |
| `jitter` | basic, strip |
| `notch` | basic, grouped, outliers, points, rainbow, violin |
| `palette` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |
| `series` | grouped |
| `series_names` | grouped |
| `show_points` | basic, notched |
| `sort_order` | basic, horizontal, letter_value, rainbow, strip, violin |
| `stroke_width` | basic, grouped, horizontal, rainbow, violin |
| `title` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |
| `values` | basic, horizontal, letter_value, rainbow, strip, violin |
| `width` | horizontal |
| `x_label` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |
| `y_label` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.boxplot(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`


<h2>Description</h2>

`sp.boxplot()` est le point d'entrée unique pour toute la famille des boîtes à moustaches. Le paramètre `variant` sélectionne la stratégie de rendu — tous les autres arguments restent identiques entre les variantes. Quartiles, moustaches 1,5×IQR et valeurs aberrantes sont calculés en pur Rust, sans NumPy ni pandas.
<h2>Paramètres</h2>

| Paramètre | Utilisé par variantes |
|-----------|----------------------|
| `boxen_depth` | letter_value |
| `category_labels` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |
| `fill_opacity` | basic, grouped, horizontal, rainbow, violin |
| `gridlines` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |
| `height` | horizontal |
| `hover` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |
| `jitter` | basic, strip |
| `notch` | basic, grouped, outliers, points, rainbow, violin |
| `palette` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |
| `series` | grouped |
| `series_names` | grouped |
| `show_points` | basic, notched |
| `sort_order` | basic, horizontal, letter_value, rainbow, strip, violin |
| `stroke_width` | basic, grouped, horizontal, rainbow, violin |
| `title` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |
| `values` | basic, horizontal, letter_value, rainbow, strip, violin |
| `width` | horizontal |
| `x_label` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |
| `y_label` | basic, grouped, horizontal, letter_value, rainbow, strip, violin |

---

<h2>Retourne</h2>

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

</div><!-- /lang-fr -->
