# Slope — Before / After Comparison Chart

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

`sp.slope(title, labels, left, right, *, variant="basic", left_label="Before", right_label="After", palette=None, show_text=True, **kwargs) -> Chart`


## Description

`sp.slope()` renders the entire slope-chart family: two parallel value axes (left / right) with one connector per row. The `variant` keyword swaps the connector style without changing any other parameter. Slope charts excel at before/after comparisons, A/B test outcomes, ranking shifts, KPI changes between periods, and any pair-wise change across many entities.
## Variants

<div data-sp-registry-table="variants" data-family="slope"></div>

## Parameters

<div data-sp-registry-table="options" data-family="slope"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

## Tips

- Use `sort_order="asc"` / `"desc"` to reorder rows by `left` value before drawing.
- The `"diverging"` and `"thick"` variants encode magnitude visually — perfect for executive summaries.
- For rank shifts (positions in a league), prefer `"bumps"` rather than `"basic"`.
- Combine `palette=` with `"monochrome"` to match brand colours per category.

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.slope(title, labels, left, right, *, variant="basic", left_label="Before", right_label="After", palette=None, show_text=True, **kwargs) -> Chart`


<h2>Description</h2>

`sp.slope()` produit toute la famille des slope charts : deux axes de valeurs parallèles (gauche / droite) avec un connecteur par ligne. Le mot-clé `variant` permute le style du connecteur sans changer aucun autre paramètre. Idéal pour comparer avant/après, résultats A/B, changements de classement, KPI entre périodes, et toute évolution par paire sur de nombreuses entités.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="slope"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="slope"></div>

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

<h2>Astuces</h2>

- Utilisez `sort_order="asc"` / `"desc"` pour réordonner les lignes selon `left` avant le rendu.
- Les variantes `"diverging"` et `"thick"` encodent visuellement la magnitude — parfaites pour un résumé exécutif.
- Pour les changements de rang (positions dans un classement), préférez `"bumps"` à `"basic"`.
- Combinez `palette=` avec `"monochrome"` pour aligner les couleurs sur les catégories de marque.

</div>
