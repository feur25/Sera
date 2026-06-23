# Line Charts

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

`sp.line(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`


## Description

`sp.line()` is the unified entry point for the entire line-chart family. The `variant` keyword selects the rendering strategy — every other argument is shared across variants.
## Variants

<div data-sp-registry-table="variants" data-family="line"></div>

## Parameters

<div data-sp-registry-table="options" data-family="line"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.line(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`


<h2>Description</h2>

`sp.line()` est le point d'entrée unifié pour toute la famille de graphiques en ligne. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments sont partagés entre les variantes.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="line"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="line"></div>

---

<h2>Retourne</h2>

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

---

</div>
