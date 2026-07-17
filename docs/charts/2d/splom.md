# SPLOM — Scatter Plot Matrix

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:520px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.splom(title, axes, series, *, variant="basic", colorscale=None, **kwargs) -> Chart`

Aliases: `sp.splom`, `sp.scatter_matrix`, `sp.splom_chart`, `sp.pairplot`, `sp.scatterplot_matrix`, `sp.build_splom`

## Description

`sp.splom()` lays out every pairwise combination of a set of numeric dimensions as an M×M grid of small scatter plots in a single self-contained chart — the standard first look at a multivariate numeric dataset. It reuses exactly the same row-major data shape as [`parallel()`](parallel.md) (`axes` + `series`, one row per observation with one value per axis), so any dataset already wired up for parallel coordinates works unchanged. Diagonal cells show the axis name instead of a self-scatter.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / dots` | Off-diagonal cells are plain scatter plots in a single palette color. |
| `"correlation"` | `correlation / corr / heat / shaded` | Each off-diagonal cell's background is tinted by the Pearson correlation coefficient between its two dimensions (via the same continuous colorscale engine as [`heatmap()`](heatmap.md)), with points overlaid in a neutral color — a heatmap and a SPLOM in one chart. |
| `"density"` | `density / alpha / overplot / cloud` | Every point drawn at 14% opacity with no stroke — overlapping points accumulate into darker regions, revealing density in datasets too large for solid dots to stay readable. |

## Parameters

| Parameter    | Type            | Default   | Description |
|---|---|---|---|
| `title`      | `str`           | required  | Chart title |
| `axes`       | `list[str]`     | required  | Dimension names |
| `series`     | `list[list[float]]` | required | One row per observation, one value per dimension |
| `variant`    | `str`           | `"basic"` | Visual style (see table) |
| `colorscale` | `str`           | `"rdbu"`  | Continuous color scale for the `"correlation"` variant |
| `point_size` | `float`         | `2.2`     | Marker radius (px) |
| `width`      | `int`           | `900`     | Canvas width (px) |
| `height`     | `int`           | `900`     | Canvas height (px) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

## Example

```python
import seraplot as sp
chart = sp.splom(
    "Vehicle Attributes",
    axes=["Speed", "Power", "Range"],
    series=[[80,65,70],[60,80,55],[40,70,90],[90,40,60],[55,85,45],[70,55,80]],
    variant="correlation",
)
chart.show()
```

<div class="sp-cls sp-open" id="splom-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('splom-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('splom-en','basic',this)"><span class="sp-cic">⬤</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('splom-en','correlation',this)"><span class="sp-cic">◐</span><span class="sp-clb">Correlation</span></button>
<button class="sp-cls-tab" onclick="spCls('splom-en','density',this)"><span class="sp-cic">░</span><span class="sp-clb">Density</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="splom-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / dots</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/splom-basic.html"></iframe>
</div>
<div class="sp-variant" id="splom-en-correlation">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"correlation"</code></span><span><strong>Aliases</strong> <code>correlation / corr / heat / shaded</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/splom-correlation.html"></iframe>
</div>
<div class="sp-variant" id="splom-en-density">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"density"</code></span><span><strong>Aliases</strong> <code>density / alpha / overplot / cloud</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/splom-density.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

## Signature

`sp.splom(title, axes, series, *, variant="basic", colorscale=None, **kwargs) -> Chart`

Alias : `sp.splom`, `sp.scatter_matrix`, `sp.splom_chart`, `sp.pairplot`, `sp.scatterplot_matrix`, `sp.build_splom`

## Description

`sp.splom()` dispose chaque combinaison par paires d'un ensemble de dimensions numériques en une grille M×M de petits nuages de points dans un seul graphique autonome — le premier regard standard sur un jeu de données numérique multivarié. Il réutilise exactement la même forme de données ligne-par-ligne que [`parallel()`](parallel.md) (`axes` + `series`, une ligne par observation avec une valeur par axe), donc tout jeu de données déjà câblé pour les coordonnées parallèles fonctionne sans modification. Les cellules diagonales affichent le nom de l'axe plutôt qu'un nuage de points contre lui-même.

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / dots` | Cellules hors-diagonale en nuage de points simple, couleur de palette unique. |
| `"correlation"` | `correlation / corr / heat / shaded` | Le fond de chaque cellule hors-diagonale est teinté selon le coefficient de corrélation de Pearson entre ses deux dimensions (via le même moteur de dégradés continus que [`heatmap()`](heatmap.md)), avec les points superposés en couleur neutre — une heatmap et un SPLOM en un seul graphique. |
| `"density"` | `density / alpha / overplot / cloud` | Chaque point dessiné à 14% d'opacité sans contour — les points superposés s'accumulent en zones plus sombres, révélant la densité sur des jeux de données trop grands pour rester lisibles en points pleins. |

## Paramètres

| Paramètre    | Type            | Défaut    | Description |
|---|---|---|---|
| `title`      | `str`           | requis    | Titre du graphique |
| `axes`       | `list[str]`     | requis    | Noms des dimensions |
| `series`     | `list[list[float]]` | requis | Une ligne par observation, une valeur par dimension |
| `variant`    | `str`           | `"basic"` | Style visuel (voir tableau) |
| `colorscale` | `str`           | `"rdbu"`  | Dégradé continu pour la variante `"correlation"` |
| `point_size` | `float`         | `2.2`     | Rayon des marqueurs (px) |
| `width`      | `int`           | `900`     | Largeur du canvas (px) |
| `height`     | `int`           | `900`     | Hauteur du canvas (px) |

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

## Exemple

```python
import seraplot as sp
chart = sp.splom(
    "Vehicle Attributes",
    axes=["Speed", "Power", "Range"],
    series=[[80,65,70],[60,80,55],[40,70,90],[90,40,60],[55,85,45],[70,55,80]],
    variant="correlation",
)
chart.show()
```

<div class="sp-cls sp-open" id="splom-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('splom-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('splom-fr','basic',this)"><span class="sp-cic">⬤</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('splom-fr','correlation',this)"><span class="sp-cic">◐</span><span class="sp-clb">Correlation</span></button>
<button class="sp-cls-tab" onclick="spCls('splom-fr','density',this)"><span class="sp-cic">░</span><span class="sp-clb">Densité</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="splom-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic / dots</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/splom-basic.html"></iframe>
</div>
<div class="sp-variant" id="splom-fr-correlation">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"correlation"</code></span><span><strong>Alias</strong> <code>correlation / corr / heat / shaded</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/splom-correlation.html"></iframe>
</div>
<div class="sp-variant" id="splom-fr-density">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"density"</code></span><span><strong>Alias</strong> <code>density / alpha / overplot / cloud</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/splom-density.html"></iframe>
</div>
</div>
</div>

</div>
