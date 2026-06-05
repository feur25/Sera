# Lollipop - Categorical Value Sticks

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

`sp.lollipop(title, labels, values, *, variant="basic", color_groups=None, highlight_index=-1, **kwargs) -> Chart`


## Description

`sp.lollipop()` is the unified entry point for the lollipop family. Each item becomes a thin stick capped by a dot - lighter ink than a bar chart for the same ranking, and the family includes circular, diverging, focused and grouped editorial layouts (the Office variant reproduces the season-rating panel pattern).

## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `color_hex` | highlight |
| `gridlines` | basic, cleveland, highlight, office |
| `height` | circular |
| `highlight_index` | highlight |
| `palette` | office |
| `show_values` | basic, cleveland, diverging |
| `title` | circular |
| `width` | circular |
| `y_label` | basic, diverging, highlight, office |

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.lollipop(title, labels, values, *, variant="basic", color_groups=None, highlight_index=-1, **kwargs) -> Chart`


<h2>Description</h2>

`sp.lollipop()` est le point d entree unique pour la famille lollipop. Chaque item devient un baton fin termine par un point - moins d encre qu un bar chart pour le meme classement, et la famille couvre des layouts circulaires, divergents, focalises et editoriaux groupes (la variante Office reproduit le motif des saisons IMDb de The Office).

<h2>Parametres</h2>

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Libelles categoriels (un par baton) |
| `values` | `list[float]` | requis | Valeur par libelle |
| `variant` | `str` | "basic" | Style visuel (voir tableau) |
| `color_groups` | `list[str]` | None | Groupe par item - active le groupage Office |
| `highlight_index` | `int` | -1 | Index a mettre en avant (Highlight); -1 = auto-max |
| `color_hex` | `int` | 0x6366F1 | Couleur par defaut |
| `palette` | `list[int]` | None | Palette personnalisee |
| `show_values` | `bool` | False | Afficher la valeur a cote de chaque point |
| `gridlines` | `bool` | False | Activer la grille de fond |
| `sort_order` | `str` | "none" | "none" / "asc" / "desc" / "alpha" |
| `width` | `int` | 900 | Largeur (px) |
| `height` | `int` | 480 | Hauteur (px) |

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

---

</div>
