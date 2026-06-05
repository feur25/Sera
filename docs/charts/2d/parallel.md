# Parallel Coordinates - Multivariate Profile Lines

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

`sp.build_parallel(title, axes, series, *, variant="basic", **kwargs) -> Chart`


## Description

`sp.build_parallel()` renders a **parallel-coordinates** chart - one vertical axis per dimension, one polyline per row. Six variants cover the classical use cases: straight lines, smooth Bezier curves, categorical coloring, single-row highlight, density-blended overlay, and gradient coloring driven by any axis. Perfect for high-dimensional EDA, profile comparison, and class separability inspection.

## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `axes` | arc, deluxe, gradient, ribbon |
| `categories` | categorical |
| `color_axis` | gradient |
| `height` | arc, deluxe, ribbon |
| `highlight_index` | highlight |
| `palette` | arc, basic, categorical, deluxe, density, highlight, ribbon, smooth |
| `series_names` | all |
| `series_values` | all |
| `title` | arc, ribbon |
| `width` | arc, categorical, deluxe, gradient, ribbon |

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.build_parallel(title, axes, series, *, variant="basic", **kwargs) -> Chart`


<h2>Description</h2>

`sp.build_parallel()` rend un graphique **parallel-coordinates** - un axe vertical par dimension, une polyligne par ligne. Six variantes couvrent les cas classiques : lignes droites, courbes Bezier, couleur par categorie, mise en avant d une ligne, overlay de densite, et degrade pilote par un axe. Ideal pour l EDA haute-dimension, la comparaison de profils, et l inspection de separabilite de classes.

<h2>Parametres</h2>

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title` | `str` | requis | Titre du graphique |
| `axes` | `list[str]` | requis | Etiquettes des axes (gauche vers droite) |
| `series` | `list[list[float]]` | requis | Profils - une liste interne par axe |
| `variant` | `str` | "basic" | Style de rendu (voir tableau) |
| `series_names` | `list[str]` | None | Noms optionnels par ligne (hover/legende) |
| `category_indices` | `list[int]` | None | Id de categorie par ligne (variant categorical) |
| `highlight_index` | `int` | -1 | Ligne a mettre en avant (variant highlight) |
| `color_axis` | `int` | -1 | Index de l axe pilotant la couleur (variant gradient) |
| `palette` | `list[int]` | None | Palette personnalisee |
| `width` | `int` | 1000 | Largeur (px) |
| `height` | `int` | 500 | Hauteur (px) |

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

</div>
