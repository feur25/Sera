# Scatter Ternary — Three-Component Composition Plot

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:520px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.scatterternary(title, x_values, y_values, z_values, *, variant="basic", x_label="A", y_label="B", z_label="C", **kwargs) -> Chart`

Aliases: `sp.scatterternary`, `sp.scatter_ternary`, `sp.ternary`, `sp.ternary_plot`, `sp.ternary_scatter`, `sp.build_scatter_ternary`

## Description

`sp.scatterternary()` plots three-component compositions (e.g. soil sand/silt/clay, alloy element fractions, poll shares) inside an equilateral triangle — the standard chart whenever three parts sum to a whole. Each point's three values are barycentric weights normalized internally (`a+b+c` need not equal 1 or 100, only their ratio matters), converted to Cartesian coordinates and rendered with a full gridline mesh (three families of lines at 20/40/60/80%, one parallel to each triangle side). Reuses the exact same `x_values`/`y_values`/`z_values` and `x_label`/`y_label`/`z_label` inputs already used by 3D scatter charts — no new parameter shape introduced.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / dots` | Single palette color, semi-transparent markers. |
| `"gradient"` | `gradient / color / colorscale / continuous` | Markers colored by a continuous value (`color_values`, defaults to the A component) via the same colorscale engine as [`heatmap()`](heatmap.md). |
| `"bubble"` | `bubble / sized / weighted / proportional` | Point radius scales with `color_values` (4.5× range between smallest and largest) instead of a fixed `point_size` — a fourth variable encoded as size on top of the three ternary axes. |
| `"labeled"` | `labeled / labelled / annotated / named` | Same layout as basic, with each point's label printed beside it — useful once you need to identify specific observations, not just see the overall distribution. |

## Parameters

| Parameter      | Type          | Default   | Description |
|---|---|---|---|
| `title`        | `str`         | required  | Chart title |
| `x_values`     | `list[float]` | required  | First component (A, top vertex) |
| `y_values`     | `list[float]` | required  | Second component (B, right vertex) |
| `z_values`     | `list[float]` | required  | Third component (C, left vertex) |
| `variant`      | `str`         | `"basic"` | Visual style (see table) |
| `x_label`      | `str`         | `"A"`     | Label for the top vertex |
| `y_label`      | `str`         | `"B"`     | Label for the right vertex |
| `z_label`      | `str`         | `"C"`     | Label for the left vertex |
| `color_values` | `list[float]` | `None`    | Continuous values for the `"gradient"` variant |
| `colorscale`   | `str`         | `"viridis"` | Color scale name for the `"gradient"` variant |
| `point_size`   | `float`       | `4.0`     | Marker radius (px) |
| `width`        | `int`         | `700`     | Canvas width (px) |
| `height`       | `int`         | `640`     | Canvas height (px) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

## Example

```python
import seraplot as sp
chart = sp.scatterternary(
    "Soil Composition",
    x_values=[0.7, 0.2, 0.1, 0.4, 0.33],
    y_values=[0.2, 0.6, 0.1, 0.3, 0.33],
    z_values=[0.1, 0.2, 0.8, 0.3, 0.34],
    x_label="Sand", y_label="Silt", z_label="Clay",
)
chart.show()
```

<div class="sp-cls sp-open" id="scatterternary-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('scatterternary-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('scatterternary-en','basic',this)"><span class="sp-cic">▲</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('scatterternary-en','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('scatterternary-en','bubble',this)"><span class="sp-cic">●</span><span class="sp-clb">Bubble</span></button>
<button class="sp-cls-tab" onclick="spCls('scatterternary-en','labeled',this)"><span class="sp-cic">●</span><span class="sp-clb">Labeled</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="scatterternary-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / dots</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatterternary-basic.html"></iframe>
</div>
<div class="sp-variant" id="scatterternary-en-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / color / colorscale / continuous</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatterternary-gradient.html"></iframe>
</div>
<div class="sp-variant" id="scatterternary-en-bubble">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bubble"</code></span><span><strong>Aliases</strong> <code>bubble / sized / weighted / proportional</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatterternary-bubble.html"></iframe>
</div>
<div class="sp-variant" id="scatterternary-en-labeled">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"labeled"</code></span><span><strong>Aliases</strong> <code>labeled / labelled / annotated / named</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/scatterternary-labeled.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

## Signature

`sp.scatterternary(title, x_values, y_values, z_values, *, variant="basic", x_label="A", y_label="B", z_label="C", **kwargs) -> Chart`

Alias : `sp.scatterternary`, `sp.scatter_ternary`, `sp.ternary`, `sp.ternary_plot`, `sp.ternary_scatter`, `sp.build_scatter_ternary`

## Description

`sp.scatterternary()` trace des compositions à trois composantes (ex : sable/limon/argile d'un sol, fractions d'éléments d'un alliage, parts de sondage) à l'intérieur d'un triangle équilatéral — le graphique de référence dès que trois parts forment un tout. Les trois valeurs de chaque point sont des poids barycentriques normalisés en interne (`a+b+c` n'a pas besoin de valoir 1 ou 100, seul leur ratio compte), converties en coordonnées cartésiennes et rendues avec une grille complète (trois familles de lignes à 20/40/60/80%, une parallèle à chaque côté du triangle). Réutilise exactement les mêmes entrées `x_values`/`y_values`/`z_values` et `x_label`/`y_label`/`z_label` déjà utilisées par les nuages de points 3D — aucune nouvelle forme de paramètre introduite.

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / dots` | Couleur de palette unique, marqueurs semi-transparents. |
| `"gradient"` | `gradient / color / colorscale / continuous` | Marqueurs colorés selon une valeur continue (`color_values`, par défaut la composante A) via le même moteur de dégradés que [`heatmap()`](heatmap.md). |
| `"bubble"` | `bubble / sized / weighted / proportional` | Le rayon des points varie selon `color_values` (facteur 4.5 entre le plus petit et le plus grand) au lieu d'un `point_size` fixe — une quatrième variable encodée en taille en plus des trois axes ternaires. |
| `"labeled"` | `labeled / labelled / annotated / named` | Même disposition que basic, avec le label de chaque point affiché à côté — utile dès qu'il faut identifier des observations précises, pas seulement voir la distribution globale. |

## Paramètres

| Paramètre      | Type          | Défaut    | Description |
|---|---|---|---|
| `title`        | `str`         | requis    | Titre du graphique |
| `x_values`     | `list[float]` | requis    | Première composante (A, sommet haut) |
| `y_values`     | `list[float]` | requis    | Deuxième composante (B, sommet droit) |
| `z_values`     | `list[float]` | requis    | Troisième composante (C, sommet gauche) |
| `variant`      | `str`         | `"basic"` | Style visuel (voir tableau) |
| `x_label`      | `str`         | `"A"`     | Libellé du sommet haut |
| `y_label`      | `str`         | `"B"`     | Libellé du sommet droit |
| `z_label`      | `str`         | `"C"`     | Libellé du sommet gauche |
| `color_values` | `list[float]` | `None`    | Valeurs continues pour la variante `"gradient"` |
| `colorscale`   | `str`         | `"viridis"` | Nom du dégradé pour la variante `"gradient"` |
| `point_size`   | `float`       | `4.0`     | Rayon des marqueurs (px) |
| `width`        | `int`         | `700`     | Largeur du canvas (px) |
| `height`       | `int`         | `640`     | Hauteur du canvas (px) |

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

## Exemple

```python
import seraplot as sp
chart = sp.scatterternary(
    "Soil Composition",
    x_values=[0.7, 0.2, 0.1, 0.4, 0.33],
    y_values=[0.2, 0.6, 0.1, 0.3, 0.33],
    z_values=[0.1, 0.2, 0.8, 0.3, 0.34],
    x_label="Sand", y_label="Silt", z_label="Clay",
)
chart.show()
```

<div class="sp-cls sp-open" id="scatterternary-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('scatterternary-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('scatterternary-fr','basic',this)"><span class="sp-cic">▲</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('scatterternary-fr','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('scatterternary-fr','bubble',this)"><span class="sp-cic">●</span><span class="sp-clb">Bulle</span></button>
<button class="sp-cls-tab" onclick="spCls('scatterternary-fr','labeled',this)"><span class="sp-cic">●</span><span class="sp-clb">Labeled</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="scatterternary-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic / dots</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/scatterternary-basic.html"></iframe>
</div>
<div class="sp-variant" id="scatterternary-fr-gradient">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code></span><span><strong>Alias</strong> <code>gradient / color / colorscale / continuous</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/scatterternary-gradient.html"></iframe>
</div>
<div class="sp-variant" id="scatterternary-fr-bubble">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"bubble"</code></span><span><strong>Alias</strong> <code>bubble / sized / weighted / proportional</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/scatterternary-bubble.html"></iframe>
</div>
<div class="sp-variant" id="scatterternary-fr-labeled">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"labeled"</code></span><span><strong>Alias</strong> <code>labeled / labelled / annotated / named</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/scatterternary-labeled.html"></iframe>
</div>
</div>
</div>

</div>
