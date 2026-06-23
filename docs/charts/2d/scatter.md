# Scatter Charts

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

`sp.scatter(title, x_values, y_values, *, variant="basic", categories=None, labels=None, color_values=None, **kwargs) -> Chart`


## Description

`sp.scatter()` is the unified entry point for the entire scatter family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. Scatter plots are the canonical way to display the joint distribution of two numeric variables; SeraPlot adds optional grouping, continuous color, distinct marker shapes, on-point labels and OLS regression — all in pure Rust SVG, thousands of times faster than Plotly.
## Variants

<div data-sp-registry-table="variants" data-family="scatter"></div>

## Parameters

<div data-sp-registry-table="options" data-family="scatter"></div>

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

`sp.scatter(title, x_values, y_values, *, variant="basic", categories=None, labels=None, color_values=None, **kwargs) -> Chart`


<h2>Description</h2>

`sp.scatter()` est le point d'entrée unifié de toute la famille scatter. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments gardent le même nom d'une variante à l'autre. Les nuages de points sont la façon canonique d'afficher la distribution conjointe de deux variables numériques ; SeraPlot ajoute groupement optionnel, couleur continue, formes de marqueurs distinctes, étiquettes sur les points et régression OLS — le tout en SVG Rust pur, des milliers de fois plus rapide que Plotly.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="scatter"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="scatter"></div>

---

<h2>Retour</h2>

`Chart` — objet exposant `.html` et `.show()`.

---

</div>
