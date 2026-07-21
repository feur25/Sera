# Circle Packing

<div class="lang-en">

<style>
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}
.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}
.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}
.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}
.sp-cls-tab .sp-clb{display:none}
.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}
.sp-variant.sp-von{display:block}
.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.circle_pack(title, labels, parents, values, *, variant="basic", **kwargs) -> Chart`

Aliases: `sp.circle_pack`, `sp.circle_packing`, `sp.pack`, `sp.bubble_pack`

## Description

Circle packing represents hierarchical data as nested circles, where the area of each circle is proportional to its value. Parent–child relationships are defined by the `parents` list (empty string = root node).

## Variants

<div data-sp-registry-table="variants" data-family="circle_pack"></div>

Unknown variant strings fall back to the registered default. Variant keys may be prefixed with `en_`, `fr_`, `en-` or `fr-`.

## Data

`labels` (`list[str]`) — Node names. `parents` (`list[str]`) — Parent name for each node (`""` = root). `values` (`list[float]`) — Size of each leaf node. `width` / `height` (`int`) — Chart dimensions.

## Parameters

<div data-sp-registry-table="options" data-family="circle_pack"></div>

## Themes

<div data-sp-registry-table="themes" data-family="circle_pack"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="circle_pack"></div>
</div>

<div class="sp-cls sp-open" id="cp-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('cp-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('cp-en','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('cp-en','flat',this)"><span class="sp-cic">○</span><span class="sp-clb">Flat</span></button>
<button class="sp-cls-tab" onclick="spCls('cp-en','outlined',this)"><span class="sp-cic">◌</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('cp-en','bubble',this)"><span class="sp-cic">◎</span><span class="sp-clb">Bubble</span></button>
<button class="sp-cls-tab" onclick="spCls('cp-en','leaf_focus',this)"><span class="sp-cic">◍</span><span class="sp-clb">Leaf Focus</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="cp-en-basic">
<p>Filled nested circles with depth-based opacity</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / nested</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/circle_pack-basic.html"></iframe>
</div>
<div class="sp-variant" id="cp-en-flat">
<p>Single-level bubble layout (no nesting)</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"flat"</code></span><span><strong>Aliases</strong> <code>flat / single</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/circle_pack-flat.html"></iframe>
</div>
<div class="sp-variant" id="cp-en-outlined">
<p>Stroke-only circles</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / stroke / border</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/circle_pack-outlined.html"></iframe>
</div>
<div class="sp-variant" id="cp-en-bubble">
<p>Runs a real greedy circle-packing layout: each circle is placed tangent to the best pair of already-placed circles (falling back to a spiral search when no tangent placement fits), producing a genuinely nested, non-overlapping bubble cluster instead of the grid-like flat layout.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bubble"</code></span><span><strong>Aliases</strong> <code>bubble / packed</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/circle_pack-bubble.html"></iframe>
</div>
<div class="sp-variant" id="cp-en-leaf_focus">
<p>Fills only the leaf circles with color and shrinks every container circle to a faint dashed outline — declutters the hierarchy chrome so attention goes straight to the actual data points instead of the grouping structure.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"leaf_focus"</code></span><span><strong>Aliases</strong> <code>leaf_focus / leaves / leaves_only / focus</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/circle_pack-leaf_focus.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.circle_pack(title, labels, parents, values, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.circle_pack`, `sp.circle_packing`, `sp.pack`, `sp.bubble_pack`

## Description

Le circle packing représente des données hiérarchiques sous forme de cercles imbriqués, où l'aire de chaque cercle est proportionnelle à sa valeur. Les relations parent-enfant sont définies par la liste `parents` (chaîne vide = nœud racine).

## Variantes

<div data-sp-registry-table="variants" data-family="circle_pack"></div>

Une variante inconnue retombe sur la valeur par défaut enregistrée. Les clés de variantes peuvent être préfixées par `en_`, `fr_`, `en-` ou `fr-`.

## Données

`labels` (`list[str]`) — Noms des nœuds. `parents` (`list[str]`) — Nom du parent de chaque nœud (`""` = racine). `values` (`list[float]`) — Taille de chaque nœud feuille. `width` / `height` (`int`) — Dimensions du graphique.

## Paramètres

<div data-sp-registry-table="options" data-family="circle_pack"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="circle_pack"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="circle_pack"></div>
</div>

<div class="sp-cls sp-open" id="cp-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('cp-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('cp-fr','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('cp-fr','flat',this)"><span class="sp-cic">○</span><span class="sp-clb">Plat</span></button>
<button class="sp-cls-tab" onclick="spCls('cp-fr','outlined',this)"><span class="sp-cic">◌</span><span class="sp-clb">Contour</span></button>
<button class="sp-cls-tab" onclick="spCls('cp-fr','bubble',this)"><span class="sp-cic">◎</span><span class="sp-clb">Bulle</span></button>
<button class="sp-cls-tab" onclick="spCls('cp-fr','leaf_focus',this)"><span class="sp-cic">◍</span><span class="sp-clb">Focus feuilles</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="cp-fr-basic">
<p>Cercles imbriqués pleins, opacité selon la profondeur</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / nested</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/circle_pack-basic.html"></iframe>
</div>
<div class="sp-variant" id="cp-fr-flat">
<p>Disposition en bulles à un seul niveau (pas d'imbrication)</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"flat"</code></span><span><strong>Alias</strong> <code>flat / single</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/circle_pack-flat.html"></iframe>
</div>
<div class="sp-variant" id="cp-fr-outlined">
<p>Cercles en contour seul</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"outlined"</code></span><span><strong>Alias</strong> <code>outlined / stroke / border</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/circle_pack-outlined.html"></iframe>
</div>
<div class="sp-variant" id="cp-fr-bubble">
<p>Applique un véritable algorithme de tassement de cercles (« circle packing ») glouton : chaque cercle est placé tangent à la meilleure paire de cercles déjà posés (avec repli sur une recherche en spirale si aucun placement tangent ne convient), pour obtenir un amas de bulles réellement imbriqué et sans chevauchement, au lieu de la disposition en grille de la variante à plat.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"bubble"</code></span><span><strong>Alias</strong> <code>bubble / packed</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/circle_pack-bubble.html"></iframe>
</div>
<div class="sp-variant" id="cp-fr-leaf_focus">
<p>Ne remplit en couleur que les cercles feuilles et réduit chaque cercle conteneur à un fin contour pointillé — désencombre le chrome hiérarchique pour que l'attention aille directement aux points de données plutôt qu'à la structure de regroupement.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"leaf_focus"</code></span><span><strong>Alias</strong> <code>leaf_focus / leaves / leaves_only / focus</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/circle_pack-leaf_focus.html"></iframe>
</div>
</div>
</div>

</div>
