# Funnel — Conversion / Pipeline Chart

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

`sp.funnel(title, labels, values, *, variant="basic", palette=None, show_text=True, **kwargs) -> Chart`


## Description

`sp.funnel()` renders the entire funnel-chart family: a stacked sequence of stages where each step’s width encodes a value. The `variant` keyword switches the geometry without changing any other parameter. Funnels are the standard for conversion analytics (visitors → signups → paid), recruiting pipelines, sales pipelines, process drop-off and any descending-cohort analysis.
## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `palette` | all |
| `show_text` | all |
| `width` | all |

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

## Tips

- Sort stages descending before passing them in (or use `sort_order="desc"`).
- Use `"conversion"` when the audience cares about stage-to-stage retention rate.
- The `"pyramid"` variant works best when values follow a steep decay.
- For broad audiences, `"chevron"` reads as a sales pipeline more naturally than the trapezoid.

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.funnel(title, labels, values, *, variant="basic", palette=None, show_text=True, **kwargs) -> Chart`


<h2>Description</h2>

`sp.funnel()` produit toute la famille des entonnoirs : une séquence d’étapes empilées dont la largeur encode une valeur. Le mot-clé `variant` permute la géométrie sans changer aucun autre paramètre. Standard pour l’analyse de conversion (visiteurs → inscrits → payants), pipelines de recrutement, pipelines commerciaux, fuites de processus et toute analyse de cohorte décroissante.
<h2>Paramètres</h2>

| Paramètre | Utilisé par variantes |
|-----------|----------------------|
| `palette` | toutes |
| `show_text` | toutes |
| `width` | toutes |

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

<h2>Astuces</h2>

- Triez les étapes décroissantes avant de les passer (ou utilisez `sort_order="desc"`).
- Utilisez `"conversion"` quand l’audience s’intéresse au taux de rétention entre étapes.
- La variante `"pyramid"` fonctionne mieux avec des valeurs en forte décroissance.
- Pour un public large, `"chevron"` se lit plus naturellement comme un pipeline commercial qu’un trapèze.

</div>
