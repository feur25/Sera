# Stackplot — Stacked Area / Streamgraph

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.stackplot(title, x_labels, series, *, variant="basic", series_names=None, **kwargs) -> Chart`

Aliases: `sp.stackplot`, `sp.stack_plot`, `sp.stacked_area`, `sp.build_stackplot`

## Description

`sp.stackplot()` draws multiple series as cumulatively stacked areas over a shared x-axis — reuses the exact same `x_labels`/`series`/`series_names` input shape as [`multiline()`](multiline.md), so any dataset already used with `multiline()` works unchanged. Negative values are clamped to 0 before stacking (a stack has no meaningful negative contribution).

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / stacked` | Traditional zero-baseline stacking, each series added on top of the previous one's cumulative total. |
| `"streamgraph"` | `streamgraph / stream / silhouette / themeriver` | Centered ("silhouette") baseline — at every x-point the stack is centered around zero (`baseline = -total/2`) instead of starting at zero, giving the flowing ThemeRiver look. |
| `"normalized"` | `normalized / percent / hundred_percent / share` | 100%-stacked — every series is divided by the x-point's total before stacking, so the top of the stack is always 1.0. Shows share of total over time instead of absolute magnitude. |
| `"gradient"` | `gradient / fade / depth / shaded` | Each stacked band fills with a top-to-bottom gradient (solid at the top edge, fading toward the bottom) instead of a flat color, for a sense of depth. |

## Parameters

| Parameter      | Type                 | Default   | Description |
|---|---|---|---|
| `title`        | `str`                | required  | Chart title |
| `x_labels`     | `list[str]`          | required  | Shared x-axis point labels |
| `series`       | `list[list[float]]`  | required  | One list of values per series, same length as `x_labels` |
| `variant`      | `str`                | `"basic"` | Visual style (see table) |
| `series_names` | `list[str]`          | `None`    | Legend label per series |
| `palette`      | `list[int]`          | `None`    | Per-series color palette |
| `width`        | `int`                | `1100`    | Canvas width (px) |
| `height`       | `int`                | `480`     | Canvas height (px) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

## Example

```python
import seraplot as sp
chart = sp.stackplot(
    "Traffic Sources",
    x_labels=["Jan","Feb","Mar","Apr","May"],
    series=[[10,14,12,18,20],[8,9,11,10,13],[5,6,7,9,8]],
    series_names=["Organic","Paid","Referral"],
    variant="streamgraph",
)
chart.show()
```

<div class="sp-cls sp-open" id="stackplot-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('stackplot-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('stackplot-en','basic',this)"><span class="sp-cic">▮</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-en','streamgraph',this)"><span class="sp-cic">〜</span><span class="sp-clb">Streamgraph</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-en','normalized',this)"><span class="sp-cic">▤</span><span class="sp-clb">Normalized</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-en','gradient',this)"><span class="sp-cic">◈</span><span class="sp-clb">Gradient</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="stackplot-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / stacked</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-basic.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-en-streamgraph">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"streamgraph"</code></span><span><strong>Aliases</strong> <code>streamgraph / stream / silhouette / themeriver</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-streamgraph.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-en-normalized">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"normalized"</code></span><span><strong>Aliases</strong> <code>normalized / percent / hundred_percent / share</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-normalized.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-en-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / fade / depth / shaded</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-gradient.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

## Signature

`sp.stackplot(title, x_labels, series, *, variant="basic", series_names=None, **kwargs) -> Chart`

Alias : `sp.stackplot`, `sp.stack_plot`, `sp.stacked_area`, `sp.build_stackplot`

## Description

`sp.stackplot()` trace plusieurs séries en aires empilées cumulativement sur un axe x partagé — réutilise exactement la même forme d'entrée `x_labels`/`series`/`series_names` que [`multiline()`](multiline.md), donc tout jeu de données déjà utilisé avec `multiline()` fonctionne sans modification. Les valeurs négatives sont ramenées à 0 avant l'empilement (un empilement n'a pas de contribution négative significative).

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / stacked` | Empilement classique à ligne de base zéro, chaque série ajoutée au-dessus du total cumulé de la précédente. |
| `"streamgraph"` | `streamgraph / stream / silhouette / themeriver` | Ligne de base centrée ("silhouette") — à chaque point x, l'empilement est centré autour de zéro (`baseline = -total/2`) au lieu de partir de zéro, donnant le rendu fluide façon ThemeRiver. |
| `"normalized"` | `normalized / percent / hundred_percent / share` | Empilement à 100% — chaque série est divisée par le total du point x avant l'empilement, donc le sommet est toujours à 1.0. Montre la part du total dans le temps plutôt que la magnitude absolue. |
| `"gradient"` | `gradient / fade / depth / shaded` | Chaque bande empilée se remplit d'un dégradé haut-bas (plein en haut, s'estompant vers le bas) au lieu d'une couleur plate, pour un effet de profondeur. |

## Paramètres

| Paramètre      | Type                 | Défaut    | Description |
|---|---|---|---|
| `title`        | `str`                | requis    | Titre du graphique |
| `x_labels`     | `list[str]`          | requis    | Libellés des points sur l'axe x partagé |
| `series`       | `list[list[float]]`  | requis    | Une liste de valeurs par série, même longueur que `x_labels` |
| `variant`      | `str`                | `"basic"` | Style visuel (voir tableau) |
| `series_names` | `list[str]`          | `None`    | Libellé de légende par série |
| `palette`      | `list[int]`          | `None`    | Palette de couleurs par série |
| `width`        | `int`                | `1100`    | Largeur du canvas (px) |
| `height`       | `int`                | `480`     | Hauteur du canvas (px) |

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

## Exemple

```python
import seraplot as sp
chart = sp.stackplot(
    "Traffic Sources",
    x_labels=["Jan","Feb","Mar","Apr","May"],
    series=[[10,14,12,18,20],[8,9,11,10,13],[5,6,7,9,8]],
    series_names=["Organic","Paid","Referral"],
    variant="streamgraph",
)
chart.show()
```

<div class="sp-cls sp-open" id="stackplot-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('stackplot-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('stackplot-fr','basic',this)"><span class="sp-cic">▮</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-fr','streamgraph',this)"><span class="sp-cic">〜</span><span class="sp-clb">Streamgraph</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-fr','normalized',this)"><span class="sp-cic">▤</span><span class="sp-clb">Normalized</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-fr','gradient',this)"><span class="sp-cic">◈</span><span class="sp-clb">Gradient</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="stackplot-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic / stacked</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-basic.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-fr-streamgraph">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"streamgraph"</code></span><span><strong>Alias</strong> <code>streamgraph / stream / silhouette / themeriver</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-streamgraph.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-fr-normalized">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"normalized"</code></span><span><strong>Alias</strong> <code>normalized / percent / hundred_percent / share</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-normalized.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-fr-gradient">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code></span><span><strong>Alias</strong> <code>gradient / fade / depth / shaded</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-gradient.html"></iframe>
</div>
</div>
</div>

</div>
