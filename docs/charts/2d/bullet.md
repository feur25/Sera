# Bullet - Compact KPI vs Target

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

`sp.bullet(title, labels, values, *, targets=None, max_vals=None, ranges=None, comparisons=None, variant="basic", **kwargs) -> Chart`


## Description

`sp.bullet()` is the unified entry point for the bullet-chart family. Inspired by Edward Tufte, a bullet packs an actual value, a target, qualitative ranges and a scale into a single horizontal row - perfect for KPI dashboards where space is precious. The `variant` keyword switches the visual treatment (zones, traffic light, thermometer, progress pill, dot, ghost-bar comparison) without touching the data.

## Variants

<div data-sp-registry-table="variants" data-family="bullet"></div>

## Parameters

<div data-sp-registry-table="options" data-family="bullet"></div>

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.bullet(title, labels, values, *, targets=None, max_vals=None, ranges=None, comparisons=None, variant="basic", **kwargs) -> Chart`


<h2>Description</h2>

`sp.bullet()` est le point d entree unique pour la famille bullet. Inspire par Edward Tufte, le bullet condense valeur, cible, zones qualitatives et echelle dans une seule ligne horizontale - parfait pour des dashboards KPIs serres. Le mot-cle `variant` change l aspect (zones, feu tricolore, thermometre, pillule de progression, point, comparaison par barre fantome) sans toucher aux donnees.

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="bullet"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="bullet"></div>

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

---

</div>
