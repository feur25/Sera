# Hexbin — Hexagonal Density Binning

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.hexbin(title, x_values, y_values, *, variant="basic", gridsize=20, colorscale=None, **kwargs) -> Chart`

Aliases: `sp.hexbin`, `sp.hexbins`, `sp.hexbin_chart`, `sp.hexagonal_binning`, `sp.build_hexbin`

## Description

`sp.hexbin()` bins a 2D scatter cloud into a regular hexagonal grid and colors each hexagon by point density (count), the standard alternative to a scatter plot once point overlap makes individual markers unreadable. Points are assigned to hexagon cells directly in pixel space using the true nearest-center rule (two candidate offset grids, closest wins), so cells tile without gaps or overlap regardless of the data's aspect ratio. Cell color reuses the same continuous colorscale engine as [`heatmap()`](heatmap.md) and `bubble(variant="gradient")` — any of `viridis` / `plasma` / `inferno` / `magma` / `cividis` / `turbo` / `rdbu` / `blues` / `reds` / `greens` works via `colorscale=`.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / filled` | Filled hexagons only, compact grid, right-side density legend. |
| `"outlined"` | `outlined / outline / stroke / labeled` | White cell borders; count printed inside each hexagon once cells are large enough to fit text. |

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title`      | `str`         | required  | Chart title |
| `x_values`   | `list[float]` | required  | X coordinates |
| `y_values`   | `list[float]` | required  | Y coordinates |
| `variant`    | `str`         | `"basic"` | Visual style (see table) |
| `gridsize`   | `int`         | `20`      | Number of hexagon columns across the plot width |
| `colorscale` | `str`         | `"viridis"` | Continuous color scale name for density |
| `width`      | `int`         | `900`     | Canvas width (px) |
| `height`     | `int`         | `520`     | Canvas height (px) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

## Example

```python
import seraplot as sp
import random
random.seed(0)
x = [random.gauss(0, 1) for _ in range(5000)]
y = [random.gauss(0, 1) for _ in range(5000)]
chart = sp.hexbin("Density", x_values=x, y_values=y, gridsize=25, colorscale="plasma")
chart.show()
```

<div class="sp-cls sp-open" id="hexbin-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('hexbin-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('hexbin-en','basic',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-en','outlined',this)"><span class="sp-cic">⬢</span><span class="sp-clb">Outlined</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="hexbin-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / filled</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hexbin-basic.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-outlined">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / labeled</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hexbin-outlined.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

## Signature

`sp.hexbin(title, x_values, y_values, *, variant="basic", gridsize=20, colorscale=None, **kwargs) -> Chart`

Alias : `sp.hexbin`, `sp.hexbins`, `sp.hexbin_chart`, `sp.hexagonal_binning`, `sp.build_hexbin`

## Description

`sp.hexbin()` regroupe un nuage de points 2D dans une grille hexagonale régulière et colore chaque hexagone selon la densité de points (comptage) — l'alternative standard au nuage de points classique dès que le chevauchement des marqueurs le rend illisible. Les points sont assignés directement en espace pixel via la règle du centre le plus proche (deux grilles candidates décalées, la plus proche l'emporte), donc les cellules pavent sans trou ni recouvrement quel que soit le ratio d'aspect des données. La couleur des cellules réutilise le même moteur de dégradés continus que [`heatmap()`](heatmap.md) et `bubble(variant="gradient")` — `viridis` / `plasma` / `inferno` / `magma` / `cividis` / `turbo` / `rdbu` / `blues` / `reds` / `greens` fonctionnent via `colorscale=`.

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / filled` | Hexagones pleins uniquement, grille compacte, légende de densité à droite. |
| `"outlined"` | `outlined / outline / stroke / labeled` | Contours blancs ; le comptage est affiché dans chaque hexagone assez grand pour contenir le texte. |

## Paramètres

| Paramètre | Type | Défaut | Description |
|---|---|---|---|
| `title`      | `str`         | requis    | Titre du graphique |
| `x_values`   | `list[float]` | requis    | Coordonnées X |
| `y_values`   | `list[float]` | requis    | Coordonnées Y |
| `variant`    | `str`         | `"basic"` | Style visuel (voir tableau) |
| `gridsize`   | `int`         | `20`      | Nombre de colonnes d'hexagones sur la largeur du graphique |
| `colorscale` | `str`         | `"viridis"` | Nom du dégradé continu pour la densité |
| `width`      | `int`         | `900`     | Largeur du canvas (px) |
| `height`     | `int`         | `520`     | Hauteur du canvas (px) |

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-cls sp-open" id="hexbin-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('hexbin-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('hexbin-fr','basic',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','outlined',this)"><span class="sp-cic">⬢</span><span class="sp-clb">Outlined</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="hexbin-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / filled</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hexbin-basic.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-outlined">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / labeled</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hexbin-outlined.html"></iframe>
</div>
</div>
</div>

</div>
