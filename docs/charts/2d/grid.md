# Grid Layout - Compose Multiple Charts

<div class="lang-en">

<style>
.sp-tabs{border:1px solid var(--sp-border);border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:var(--sp-surface);border-bottom:1px solid var(--sp-border);flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:var(--sp-text-muted);cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:var(--sp-text)}
.sp-tb.sp-act{color:var(--sp-accent);border-bottom-color:var(--sp-accent)}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
</style>
## Signature

`sp.build_grid(charts, *, cols=2, gap=16, bg=None, cell_height=520) -> Chart`


## Description

`sp.build_grid()` packs N pre-built charts into a responsive CSS-grid layout, each chart hosted in its own iframe so styles never leak. It is the easiest way to assemble a multi-chart story or a quick dashboard tile - just stack any combination of bar / line / pie / scatter / 3D / map charts.

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `charts` | `list[Chart]` | required | Charts to arrange in the grid |
| `cols` | `int` | 2 | Number of columns |
| `gap` | `int` | 16 | Gap between cells in pixels |
| `bg` | `str` | None | Optional grid background |
| `cell_height` | `int` | 520 | Height per cell in pixels |

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/grid-basic.html"></iframe>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.build_grid(charts, *, cols=2, gap=16, bg=None, cell_height=520) -> Chart`


<h2>Description</h2>

`sp.build_grid()` empile N charts dans une grille CSS responsive, chaque chart isole dans son propre iframe pour eviter toute fuite de style. C est la facon la plus simple d assembler une narration multi-charts ou une tuile de dashboard - empile n importe quelle combinaison de bar / line / pie / scatter / 3D / map.

<h2>Parametres</h2>

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `charts` | `list[Chart]` | requis | Charts a disposer dans la grille |
| `cols` | `int` | 2 | Nombre de colonnes |
| `gap` | `int` | 16 | Espace entre cellules (px) |
| `bg` | `str` | None | Fond optionnel de la grille |
| `cell_height` | `int` | 520 | Hauteur par cellule (px) |

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

</div>
