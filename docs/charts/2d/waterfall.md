# Waterfall — Running-Total Bridge Chart

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

`sp.waterfall(title, labels, values, *, variant="basic", show_text=True, **kwargs) -> Chart`


## Description

`sp.waterfall()` renders the entire waterfall-chart family: a sequence of bars where each step adds (positive) or subtracts (negative) from a running total. The `variant` keyword selects the geometry without touching any other parameter. Waterfalls are the standard for P&L bridges, variance analysis, cohort decomposition, fee/tax breakdowns and any "from A to B, what changed?" narrative.

> **Totals** — set a value to `0` and use a label containing `total`, `net`, `final`, `gross` or `ebitda` to mark a subtotal bar; it is rendered with the totals color and anchored on the running sum.

## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `gridlines` | horizontal |
| `height` | horizontal |
| `show_text` | all |
| `title` | horizontal |
| `width` | horizontal |

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.waterfall(title, labels, values, *, variant="basic", show_text=True, **kwargs) -> Chart`


<h2>Description</h2>

`sp.waterfall()` rassemble toute la famille des graphiques waterfall : une suite de barres ou chaque etape ajoute (positif) ou retranche (negatif) au cumul courant. Le mot-cle `variant` change la geometrie sans toucher aux autres parametres. Les waterfalls sont la reference pour les ponts de P&L, l analyse d ecarts, la decomposition de cohortes, le detail des frais/taxes et tout recit du type "de A vers B, qu est-ce qui a change ?".

> **Totaux** — mettez la valeur a `0` et utilisez un libelle contenant `total`, `net`, `final`, `gross` ou `ebitda` pour marquer une barre de sous-total ; elle est rendue avec la couleur des totaux et ancree sur le cumul courant.

<h2>Parametres</h2>

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title`     | `str`         | requis    | Titre du graphique |
| `labels`    | `list[str]`   | requis    | Libelles des etapes (gauche -> droite) |
| `values`    | `list[float]` | requis    | Deltas par etape (mettez 0 + libelle "total" pour les sous-totaux) |
| `variant`   | `str`         | `"basic"` | Variante geometrique (voir tableau) |
| `show_text` | `bool`        | `True`    | Afficher les annotations de valeur au-dessus des barres |
| `width`     | `int`         | `900`     | Largeur du canvas (px) |
| `height`    | `int`         | `480`     | Hauteur du canvas (px) |

<h2>Retour</h2>

`Chart` — objet avec une propriete `.html` et une methode `.show()`.

---

</div>
