# Treemap — Hierarchical Proportional Tiles

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

`sp.treemap(title, labels, values, *, parents=None, variant="basic", palette=None, **kwargs) -> Chart`


## Description

`sp.treemap()` is the unified entry point for the entire treemap-chart family. A treemap divides a rectangle into proportional sub-rectangles whose area encodes value; when a `parents` list is given the layout becomes hierarchical (each parent gets its own block, leaves are squarified within). The `variant` keyword switches the visual style without touching the data. Treemaps are the standard for visualizing budgets, market cap, disk usage, portfolio weights, file systems and any 'whole = sum of parts' breakdown.

> **Hierarchical mode** — pass `parents` (one parent label per leaf, can be empty string `""` for a flat treemap). Internal totals are auto-computed from leaves. Sort leaves with the `sort_order` parameter (`"desc"` recommended).

## Variants

<div data-sp-registry-table="variants" data-family="treemap"></div>

## Parameters

<div data-sp-registry-table="options" data-family="treemap"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.treemap(title, labels, values, *, parents=None, variant="basic", palette=None, **kwargs) -> Chart`


<h2>Description</h2>

`sp.treemap()` est le point d entree unifie pour toute la famille treemap. Un treemap decoupe un rectangle en sous-rectangles proportionnels dont l aire code la valeur ; lorsqu une liste `parents` est fournie le rendu devient hierarchique (chaque parent recoit son propre bloc, les feuilles y sont squarifiees). Le mot-cle `variant` change le style sans toucher aux donnees. Les treemaps sont la reference pour visualiser budgets, capitalisations boursieres, occupation disque, poids de portefeuille, systemes de fichiers et toute decomposition 'tout = somme des parties'.

> **Mode hierarchique** — passez `parents` (un libelle parent par feuille, chaine vide `""` pour un treemap plat). Les totaux internes sont auto-calcules. Triez les feuilles avec `sort_order` (`"desc"` recommande).

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="treemap"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="treemap"></div>

<h2>Retour</h2>

`Chart` — objet avec une propriete `.html` et une methode `.show()`.

---

</div>
