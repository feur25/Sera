# Parallel Coordinates - Multivariate Profile Lines

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

`sp.build_parallel(title, axes, series, *, variant="basic", **kwargs) -> Chart`


## Description

`sp.build_parallel()` renders a **parallel-coordinates** chart - one vertical axis per dimension, one polyline per row. Six variants cover the classical use cases: straight lines, smooth Bezier curves, categorical coloring, single-row highlight, density-blended overlay, and gradient coloring driven by any axis. Perfect for high-dimensional EDA, profile comparison, and class separability inspection.

## Variants

<div data-sp-registry-table="variants" data-family="parallel"></div>

## Parameters

<div data-sp-registry-table="options" data-family="parallel"></div>

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.build_parallel(title, axes, series, *, variant="basic", **kwargs) -> Chart`


<h2>Description</h2>

`sp.build_parallel()` rend un graphique **parallel-coordinates** - un axe vertical par dimension, une polyligne par ligne. Six variantes couvrent les cas classiques : lignes droites, courbes Bezier, couleur par categorie, mise en avant d une ligne, overlay de densite, et degrade pilote par un axe. Ideal pour l EDA haute-dimension, la comparaison de profils, et l inspection de separabilite de classes.

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="parallel"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="parallel"></div>

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

</div>
