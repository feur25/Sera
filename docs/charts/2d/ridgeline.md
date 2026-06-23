# Ridgeline — Joyplot / Stacked KDE

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

`sp.ridgeline(title, categories, values, *, variant="basic", overlap=0.5, bandwidth=0.0, n_points=60, fill_opacity=56, palette=None, **kwargs) -> Chart`


## Description

`sp.ridgeline()` is the unified entry point for the entire ridgeline family — also known as joyplot. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. A ridgeline plot stacks one KDE curve per category along a shared X axis with a controllable vertical overlap, making it ideal to compare distributions across many groups (years, regions, segments…). SeraPlot renders everything in pure Rust SVG, with quartile/mean overlays, rug ticks, gradient fills and a built-in viridis colormap.
## Variants

<div data-sp-registry-table="variants" data-family="ridgeline"></div>

## Parameters

<div data-sp-registry-table="options" data-family="ridgeline"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.ridgeline(title, categories, values, *, variant="basic", overlap=0.5, bandwidth=0.0, n_points=60, fill_opacity=56, palette=None, **kwargs) -> Chart`


<h2>Description</h2>

`sp.ridgeline()` est le point d'entrée unifié pour toute la famille ridgeline — aussi appelé joyplot. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments conservent le même nom d'une variante à l'autre. Un ridgeline empile une courbe KDE par catégorie sur un axe X partagé avec un recouvrement vertical réglable, idéal pour comparer des distributions à travers plusieurs groupes (années, régions, segments…). SeraPlot rend tout en SVG Rust natif, avec marqueurs quartiles/moyenne, ticks rug, dégradés et palette viridis intégrée.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="ridgeline"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="ridgeline"></div>

---

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

</div>

