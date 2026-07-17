# Icicle — Hierarchical Banded Chart

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.icicle(title, labels, parents, values, *, variant="basic", palette=None, **kwargs) -> Chart`

Aliases: `sp.icicle`, `sp.icicles`, `sp.icicle_chart`, `sp.icicle_family`, `sp.build_icicle`

## Description

`sp.icicle()` renders a hierarchy as stacked bands: each depth level of the tree is one horizontal row, and the width of each node inside its row is proportional to its value. It is a rectangular alternative to [`sunburst()`](sunburst.md) and shares the exact same input schema (`labels` / `parents` / `values`), so any dataset already used with `sunburst()` or `treemap()` works unchanged.

> **Hierarchy encoding** — `labels` lists every node, `parents` gives the parent label of each node (`""` for a root). Leaf values come from `values`; internal-node values at `0` are auto-rolled-up from descendants.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / layers` | Root row at the top, depth-based opacity, white separators between bands. |
| `"gapped"` | `gapped / spaced / isolated / padded` | Rounded corners with a small gap between every node, both across and between rows. |
| `"horizontal"` | `horizontal / h / sideways / left_to_right` | Root column on the left, depth grows rightward instead of downward. |

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title`    | `str`         | required  | Chart title |
| `labels`   | `list[str]`   | required  | Node labels (one per row) |
| `parents`  | `list[str]`   | required  | Parent label of each node (`""` for roots) |
| `values`   | `list[float]` | required  | Leaf values; internal zeros are auto-rolled-up |
| `variant`  | `str`         | `"basic"` | Visual style (see table) |
| `palette`  | `list[int]`   | `None`    | Per-root color palette (rotates if shorter) |
| `width`    | `int`         | `760`     | Canvas width (px) |
| `height`   | `int`         | `520`     | Canvas height (px) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

## Example

```python
import seraplot as sp
chart = sp.icicle(
    "Org Chart",
    labels=["Company", "Eng", "Sales", "Backend", "Frontend", "EMEA", "AMER"],
    parents=["", "Company", "Company", "Eng", "Eng", "Sales", "Sales"],
    values=[0, 0, 0, 40, 25, 20, 15],
    variant="basic",
)
chart.show()
```

<div class="sp-cls sp-open" id="icicle-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('icicle-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('icicle-en','basic',this)"><span class="sp-cic">▤</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('icicle-en','gapped',this)"><span class="sp-cic">▥</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('icicle-en','horizontal',this)"><span class="sp-cic">▦</span><span class="sp-clb">Horizontal</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="icicle-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / layers</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-basic.html"></iframe>
</div>
<div class="sp-variant" id="icicle-en-gapped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>gapped / spaced / isolated / padded</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-gapped.html"></iframe>
</div>
<div class="sp-variant" id="icicle-en-horizontal">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / h / sideways / left_to_right</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-horizontal.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

## Signature

`sp.icicle(title, labels, parents, values, *, variant="basic", palette=None, **kwargs) -> Chart`

Alias : `sp.icicle`, `sp.icicles`, `sp.icicle_chart`, `sp.icicle_family`, `sp.build_icicle`

## Description

`sp.icicle()` représente une hiérarchie sous forme de bandes empilées : chaque niveau de profondeur de l'arbre est une rangée horizontale, et la largeur de chaque nœud dans sa rangée est proportionnelle à sa valeur. C'est une alternative rectangulaire à [`sunburst()`](sunburst.md), avec exactement le même schéma d'entrée (`labels` / `parents` / `values`) — tout jeu de données déjà utilisé avec `sunburst()` ou `treemap()` fonctionne sans modification.

> **Encodage de la hiérarchie** — `labels` liste tous les nœuds, `parents` donne le libellé du parent de chaque nœud (`""` pour une racine). Les valeurs des feuilles viennent de `values` ; les nœuds internes à `0` sont calculés automatiquement comme la somme de leurs descendants.

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / layers` | Rangée racine en haut, opacité dégressive selon la profondeur, séparateurs blancs entre bandes. |
| `"gapped"` | `gapped / spaced / isolated / padded` | Coins arrondis avec un petit espace entre chaque nœud, en largeur comme entre rangées. |
| `"horizontal"` | `horizontal / h / sideways / left_to_right` | Colonne racine à gauche, la profondeur se développe vers la droite au lieu du bas. |

## Paramètres

| Paramètre | Type | Défaut | Description |
|---|---|---|---|
| `title`    | `str`         | requis    | Titre du graphique |
| `labels`   | `list[str]`   | requis    | Libellés des nœuds (un par ligne) |
| `parents`  | `list[str]`   | requis    | Parent de chaque nœud (`""` pour les racines) |
| `values`   | `list[float]` | requis    | Valeurs feuilles ; zéros internes calculés auto |
| `variant`  | `str`         | `"basic"` | Style visuel (voir tableau) |
| `palette`  | `list[int]`   | `None`    | Palette couleurs par racine (rotation si trop courte) |
| `width`    | `int`         | `760`     | Largeur du canvas (px) |
| `height`   | `int`         | `520`     | Hauteur du canvas (px) |

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-cls sp-open" id="icicle-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('icicle-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('icicle-fr','basic',this)"><span class="sp-cic">▤</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('icicle-fr','gapped',this)"><span class="sp-cic">▥</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('icicle-fr','horizontal',this)"><span class="sp-cic">▦</span><span class="sp-clb">Horizontal</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="icicle-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / layers</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-basic.html"></iframe>
</div>
<div class="sp-variant" id="icicle-fr-gapped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>gapped / spaced / isolated / padded</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-gapped.html"></iframe>
</div>
<div class="sp-variant" id="icicle-fr-horizontal">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / h / sideways / left_to_right</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-horizontal.html"></iframe>
</div>
</div>
</div>

</div>
