# Radar — Spider / Star Chart

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

`sp.radar(title, axes, series, *, series_names=None, variant="basic", filled=True, fill_opacity=50, palette=None, **kwargs) -> Chart`


## Description

`sp.radar()` is the unified entry point for the entire radar / spider / star chart family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. Radar charts are ideal for multivariate comparison across 3+ axes — performance profiles, KPIs, skill maps, scoring systems. SeraPlot draws everything in pure Rust SVG with concentric grid rings, axis lines, automatic ring tick labels, optional legend and per-series palette colors. The polar-bar variant turns the chart into a categorical polar histogram, the stacked variant builds a cumulative composition view.
## Variants

<div data-sp-registry-table="variants" data-family="radar"></div>

## Parameters

<div data-sp-registry-table="options" data-family="radar"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<h2>Signature</h2>

`sp.radar(title, axes, series, *, series_names=None, variant="basic", filled=True, fill_opacity=50, palette=None, **kwargs) -> Chart`


<h2>Description</h2>

`sp.radar()` est le point d'entrée unifié pour toute la famille radar / spider / star. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments conservent le même nom d'une variante à l'autre. Le radar est idéal pour comparer plusieurs séries sur 3 axes ou plus — profils de performance, KPI, cartographie de compétences, systèmes de notation. SeraPlot dessine tout en SVG Rust natif avec anneaux de grille concentriques, axes, labels automatiques de graduation, légende optionnelle et couleurs de palette par série. La variante polar_bar transforme le radar en histogramme polaire catégoriel, la variante stacked construit une vue de composition cumulative.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="radar"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="radar"></div>

---

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

</div>

