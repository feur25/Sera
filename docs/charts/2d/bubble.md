# Bubble Charts

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

`sp.bubble(title, x_values, y_values, sizes, *, variant="basic", categories=None, labels=None, color_values=None, **kwargs) -> Chart`


## Description

`sp.bubble()` is the unified entry point for the entire bubble-chart family. The `variant` keyword selects the rendering strategy — all other arguments remain consistent across variants. A bubble chart extends a 2D scatter plot with a third numeric dimension represented as bubble **area** (not radius), following best practices for perceptual accuracy.
## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `categories` | categorical, labeled, outlined |
| `color_hex` | basic, labeled, negative, outlined |
| `color_high` | gradient, negative |
| `color_low` | gradient, negative |
| `color_values` | gradient |
| `gridlines` | all |
| `hover` | all |
| `labels` | basic, deluxe, gradient, labeled, negative, outlined, plasma |
| `palette` | categorical, labeled, outlined |
| `sizes` | gradient, negative |
| `stroke_width` | basic, categorical, gradient, labeled, negative, outlined |
| `title` | all |
| `width` | categorical, gradient, negative, outlined |
| `x_label` | all |
| `x_values` | all |
| `y_label` | all |
| `y_values` | all |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

---

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.bubble(title, x_values, y_values, sizes, *, variant="basic", categories=None, labels=None, color_values=None, **kwargs) -> Chart`


<h2>Description</h2>

`sp.bubble()` est le point d'entrée unique de toute la famille des graphiques à bulles. Le paramètre `variant` choisit la stratégie de rendu — tous les autres arguments restent cohérents entre variantes. Une bulle représente une troisième dimension via son **aire** (et non son rayon), pour une lecture perceptive correcte.
<h2>Paramètres</h2>

| Paramètre | Utilisé par variantes |
|-----------|----------------------|
| `categories` | categorical, labeled, outlined |
| `color_hex` | basic, labeled, negative, outlined |
| `color_high` | gradient, negative |
| `color_low` | gradient, negative |
| `color_values` | gradient |
| `gridlines` | toutes |
| `hover` | toutes |
| `labels` | basic, deluxe, gradient, labeled, negative, outlined, plasma |
| `palette` | categorical, labeled, outlined |
| `sizes` | gradient, negative |
| `stroke_width` | basic, categorical, gradient, labeled, negative, outlined |
| `title` | toutes |
| `width` | categorical, gradient, negative, outlined |
| `x_label` | toutes |
| `x_values` | toutes |
| `y_label` | toutes |
| `y_values` | toutes |

---

<h2>Retourne</h2>

`Chart` — objet exposant `.html` et `.show()`.

---

---

</div><!-- /lang-fr -->
