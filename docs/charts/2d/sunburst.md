# Sunburst — Hierarchical Ring Chart

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

`sp.sunburst(title, labels, parents, values, *, variant="basic", palette=None, **kwargs) -> Chart`


## Description

`sp.sunburst()` is the unified entry point for the entire sunburst-chart family. A sunburst represents a hierarchy as concentric rings: the innermost ring is the root, each outer ring is a deeper level, and angular size encodes value. The `variant` keyword selects the visual style without changing any other parameter. Sunbursts are the standard for visualizing nested taxonomies (org charts, file systems, market segmentation, expense categories, phylogenetic trees) and outperform classic pie charts as soon as a real hierarchy exists.

> **Hierarchy encoding** — `labels` lists every node, `parents` gives the parent label of each node ("" for a root). Leaf values are taken from `values`; internal node values are auto-rolled-up from descendants when set to 0.

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title`    | `str`         | required  | Chart title |
| `labels`   | `list[str]`   | required  | Node labels (one per row) |
| `parents`  | `list[str]`   | required  | Parent label of each node ("" for roots) |
| `values`   | `list[float]` | required  | Leaf values; internal zeros are auto-rolled-up |
| `variant`  | `str`         | `"basic"` | Visual style (see table) |
| `palette`  | `list[int]`   | `None`    | Per-root color palette (rotates if shorter) |
| `width`    | `int`         | `700`     | Canvas width (px) |
| `height`   | `int`         | `700`     | Canvas height (px) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.sunburst(title, labels, parents, values, *, variant="basic", palette=None, **kwargs) -> Chart`


<h2>Description</h2>

`sp.sunburst()` est le point d entree unifie pour toute la famille des graphiques sunburst. Un sunburst represente une hierarchie sous forme d anneaux concentriques : l anneau interieur est la racine, chaque anneau exterieur est un niveau plus profond, et l angle code la valeur. Le mot-cle `variant` change le style sans toucher aux autres parametres. Les sunbursts sont la reference pour visualiser des taxonomies imbriquees (organigrammes, systemes de fichiers, segmentation marche, categories de depenses, arbres phylogenetiques) et surpassent le camembert des qu une vraie hierarchie existe.

> **Encodage de la hierarchie** — `labels` liste tous les noeuds, `parents` donne le libelle du parent de chaque noeud ("" pour une racine). Les valeurs des feuilles viennent de `values` ; les noeuds internes a 0 sont calcules automatiquement comme la somme de leurs descendants.

<h2>Parametres</h2>

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title`    | `str`         | requis    | Titre du graphique |
| `labels`   | `list[str]`   | requis    | Libelles des noeuds (un par ligne) |
| `parents`  | `list[str]`   | requis    | Parent de chaque noeud ("" pour les racines) |
| `values`   | `list[float]` | requis    | Valeurs feuilles ; zeros internes calcules auto |
| `variant`  | `str`         | `"basic"` | Style visuel (voir tableau) |
| `palette`  | `list[int]`   | `None`    | Palette couleurs par racine (rotation si trop courte) |
| `width`    | `int`         | `700`     | Largeur du canvas (px) |
| `height`   | `int`         | `700`     | Hauteur du canvas (px) |

<h2>Retour</h2>

`Chart` — objet avec une propriete `.html` et une methode `.show()`.

---

</div>
