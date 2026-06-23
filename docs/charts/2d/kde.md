# KDE — Kernel Density Estimate

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

`sp.kde(title, values, *, variant="basic", categories=None, bandwidth=0.0, filled=True, fill_opacity=50, bins=30, n_points=80, palette=None, **kwargs) -> Chart`


## Description

`sp.kde()` is the unified entry point for the entire Kernel Density Estimate family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. KDE produces a smooth, continuous density estimate from a sample of points using a Gaussian kernel with Scott's rule for automatic bandwidth selection. SeraPlot renders the curves as pure Rust SVG, with native multi-series, normalization, CDF, rug, histogram overlay and gradient fills.
## Variants

<div data-sp-registry-table="variants" data-family="kde"></div>

## Parameters

<div data-sp-registry-table="options" data-family="kde"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.kde(title, values, *, variant="basic", categories=None, bandwidth=0.0, filled=True, fill_opacity=50, bins=30, n_points=80, palette=None, **kwargs) -> Chart`


<h2>Description</h2>

`sp.kde()` est le point d'entrée unifié pour toute la famille KDE (Kernel Density Estimate). Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments conservent le même nom d'une variante à l'autre. La KDE produit une estimation de densité continue lissée à partir d'un échantillon de points avec un noyau gaussien et la règle de Scott pour le choix automatique de la bande passante. SeraPlot rend les courbes en SVG Rust natif, avec multi-séries, normalisation, CDF, rug, histogramme superposé et remplissage en dégradé.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="kde"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="kde"></div>

---

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

</div>

