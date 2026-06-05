# Dumbbell - Before / After Two-Point Comparison

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

`sp.dumbbell(title, labels, start, end, *, variant="basic", series_name_start="Start", series_name_end="End", **kwargs) -> Chart`


## Description

`sp.dumbbell()` is the unified entry point for the dumbbell-chart family. Each row plots two values - typically a before and an after - linked by a connector, making it the chart of choice for change, gap or comparison-over-time analyses (salary equity, turnaround KPIs, A/B uplifts, etc.). The `variant` keyword switches the visual treatment without touching the data.

## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `series_names` | all |

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.dumbbell(title, labels, start, end, *, variant="basic", series_name_start="Start", series_name_end="End", **kwargs) -> Chart`


<h2>Description</h2>

`sp.dumbbell()` est le point d entree unique pour la famille dumbbell. Chaque ligne montre deux valeurs - typiquement avant/apres - reliees par un connecteur, ce qui en fait le choix naturel pour visualiser un changement, un ecart ou une evolution (equite salariale, KPIs de redressement, uplifts A/B, etc.). Le mot-cle `variant` change le style visuel sans toucher aux donnees.

<h2>Parametres</h2>

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Libelles de ligne |
| `start` | `list[float]` | requis | Valeurs de depart |
| `end` | `list[float]` | requis | Valeurs d arrivee |
| `variant` | `str` | `"basic"` | Style visuel (voir tableau) |
| `series_name_start` | `str` | `"Start"` | Label legende serie depart |
| `series_name_end` | `str` | `"End"` | Label legende serie arrivee |
| `palette` | `list[int]` | `None` | Palette personnalisee: `[c_start, c_end, ...]` |
| `sort_order` | `str` | `"none"` | `"none"`, `"asc"` or `"desc"` |
| `width` | `int` | `1000` | Largeur (px) |
| `height` | `int` | `500` | Hauteur (px) |

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

---

</div>
