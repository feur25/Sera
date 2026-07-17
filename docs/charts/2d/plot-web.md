# Plot Web — Force-Directed Bubble Network

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.plot_web(title, x_values, y_values, *, variant="scatter", sizes=None, labels=None, groups=None, **kwargs) -> Chart`

Aliases: `sp.plot_web`, `sp.web_plot`, `sp.plotweb`, `sp.carbon_web`, `sp.web_chart`, `sp.flow_web`

## Description

`sp.plot_web()` places each data point as a sized bubble node and lets the two positional axes (`x_values`/`y_values`) drive layout instead of a fixed cartesian grid — the `scatter` variant reads them as literal coordinates on a light-trail canvas, the `radial` variant re-projects them onto concentric rings around a center point. Bubble radius scales between `min_r` and `max_r` from the `sizes` array (or a constant radius if omitted), and `groups` assigns a categorical color from `palette` per node.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"scatter"` | `scatter / web / connected / default / flow` | Nodes positioned directly from `x_values`/`y_values`, connecting light-trail background. |
| `"radial"` | `radial / solar / stellar / mandala / spider` | Nodes re-projected onto concentric rings, angle and radius derived from the same input coordinates. |

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title`      | `str`         | required  | Chart title |
| `x_values`   | `list[float]` | required  | Horizontal position (scatter) or angle-driving value (radial) |
| `y_values`   | `list[float]` | required  | Vertical position (scatter) or radius-driving value (radial) |
| `variant`    | `str`         | `"scatter"` | Layout style (see table) |
| `sizes`      | `list[float]` | `[]`      | Per-node value driving bubble radius between `min_r` and `max_r` |
| `labels`     | `list[str]`   | `[]`      | Per-node hover label |
| `groups`     | `list[str]`   | `[]`      | Per-node category, colored from `palette` |
| `size_label` | `str`         | `""`      | Legend label for the size scale |
| `x_log`      | `bool`        | `False`   | Logarithmic scale on the x-axis |
| `min_r`      | `float`       | `6.0`     | Minimum bubble radius (px) |
| `max_r`      | `float`       | `38.0`    | Maximum bubble radius (px) |
| `width`      | `int`         | `1440`    | Canvas width (px) |
| `height`     | `int`         | `580`     | Canvas height (px) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

## Example

```python
import seraplot as sp
chart = sp.plot_web(
    "Node network",
    x_values=[1, 2, 3, 4, 5],
    y_values=[3, 1, 4, 1, 5],
    sizes=[10, 20, 15, 25, 18],
    labels=["A", "B", "C", "D", "E"],
)
chart.show()
```

<div class="sp-cls sp-open" id="plot-web-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('plot-web-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('plot-web-en','scatter',this)"><span class="sp-cic">●</span><span class="sp-clb">Scatter</span></button>
<button class="sp-cls-tab" onclick="spCls('plot-web-en','radial',this)"><span class="sp-cic">◎</span><span class="sp-clb">Radial</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="plot-web-en-scatter">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"scatter"</code></span><span><strong>Aliases</strong> <code>scatter / web / connected / default / flow</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/plot-web.html"></iframe>
</div>
<div class="sp-variant" id="plot-web-en-radial">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"radial"</code></span><span><strong>Aliases</strong> <code>radial / solar / stellar / mandala / spider</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/plot-web-radial.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

## Signature

`sp.plot_web(title, x_values, y_values, *, variant="scatter", sizes=None, labels=None, groups=None, **kwargs) -> Chart`

Alias : `sp.plot_web`, `sp.web_plot`, `sp.plotweb`, `sp.carbon_web`, `sp.web_chart`, `sp.flow_web`

## Description

`sp.plot_web()` place chaque point de donnée comme un nœud-bulle et laisse les deux axes positionnels (`x_values`/`y_values`) piloter la disposition plutôt qu'une grille cartésienne fixe — la variante `scatter` les lit comme des coordonnées littérales sur un canvas à traînées lumineuses, la variante `radial` les reprojette sur des anneaux concentriques autour d'un point central. Le rayon des bulles est mis à l'échelle entre `min_r` et `max_r` à partir du tableau `sizes` (ou un rayon constant si omis), et `groups` assigne une couleur catégorielle depuis `palette` par nœud.

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"scatter"` | `scatter / web / connected / default / flow` | Nœuds positionnés directement depuis `x_values`/`y_values`, fond à traînées lumineuses. |
| `"radial"` | `radial / solar / stellar / mandala / spider` | Nœuds reprojetés sur des anneaux concentriques, angle et rayon dérivés des mêmes coordonnées d'entrée. |

## Paramètres

| Paramètre | Type | Défaut | Description |
|---|---|---|---|
| `title`      | `str`         | requis    | Titre du graphique |
| `x_values`   | `list[float]` | requis    | Position horizontale (scatter) ou valeur pilotant l'angle (radial) |
| `y_values`   | `list[float]` | requis    | Position verticale (scatter) ou valeur pilotant le rayon (radial) |
| `variant`    | `str`         | `"scatter"` | Style de disposition (voir tableau) |
| `sizes`      | `list[float]` | `[]`      | Valeur par nœud pilotant le rayon de bulle entre `min_r` et `max_r` |
| `labels`     | `list[str]`   | `[]`      | Étiquette de survol par nœud |
| `groups`     | `list[str]`   | `[]`      | Catégorie par nœud, colorée depuis `palette` |
| `size_label` | `str`         | `""`      | Étiquette de légende pour l'échelle de taille |
| `x_log`      | `bool`        | `False`   | Échelle logarithmique sur l'axe X |
| `min_r`      | `float`       | `6.0`     | Rayon de bulle minimum (px) |
| `max_r`      | `float`       | `38.0`    | Rayon de bulle maximum (px) |
| `width`      | `int`         | `1440`    | Largeur du canvas (px) |
| `height`     | `int`         | `580`     | Hauteur du canvas (px) |

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

## Exemple

```python
import seraplot as sp
chart = sp.plot_web(
    "Node network",
    x_values=[1, 2, 3, 4, 5],
    y_values=[3, 1, 4, 1, 5],
    sizes=[10, 20, 15, 25, 18],
    labels=["A", "B", "C", "D", "E"],
)
chart.show()
```

<div class="sp-cls sp-open" id="plot-web-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('plot-web-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('plot-web-fr','scatter',this)"><span class="sp-cic">●</span><span class="sp-clb">Scatter</span></button>
<button class="sp-cls-tab" onclick="spCls('plot-web-fr','radial',this)"><span class="sp-cic">◎</span><span class="sp-clb">Radial</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="plot-web-fr-scatter">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"scatter"</code></span><span><strong>Alias</strong> <code>scatter / web / connected / default / flow</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/plot-web.html"></iframe>
</div>
<div class="sp-variant" id="plot-web-fr-radial">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"radial"</code></span><span><strong>Alias</strong> <code>radial / solar / stellar / mandala / spider</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/plot-web-radial.html"></iframe>
</div>
</div>
</div>

</div>
