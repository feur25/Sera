# Icicle — Hierarchical Banded Chart

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

`sp.icicle(title, labels, parents, values, *, variant="basic", palette=None, **kwargs) -> Chart`

Aliases: `sp.icicle`, `sp.icicles`, `sp.icicle_chart`, `sp.icicle_family`, `sp.build_icicle`

## Description

`sp.icicle()` renders a hierarchy as stacked bands: each depth level of the tree is one horizontal row, and the width of each node inside its row is proportional to its value. It is a rectangular alternative to [`sunburst()`](sunburst.md) and shares the exact same input schema (`labels` / `parents` / `values`), so any dataset already used with `sunburst()` or `treemap()` works unchanged.

> **Hierarchy encoding** — `labels` lists every node, `parents` gives the parent label of each node (`""` for a root). Leaf values come from `values`; internal-node values at `0` are auto-rolled-up from descendants.

## Variants

<div data-sp-registry-table="variants" data-family="icicle"></div>

Unknown variant strings fall back to the registered default. Variant keys may be prefixed with `en_`, `fr_`, `en-` or `fr-`.

## Data

`labels` (`list[str]`) — Node labels (one per row). `parents` (`list[str]`) — Parent label of each node (`""` for roots). `values` (`list[float]`) — Leaf values; internal zeros are auto-rolled-up.

## Parameters

<div data-sp-registry-table="options" data-family="icicle"></div>

## Themes

<div data-sp-registry-table="themes" data-family="icicle"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="icicle"></div>
</div>

<div class="sp-cls sp-open" id="icicle-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('icicle-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('icicle-en','basic',this)"><span class="sp-cic">▤</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('icicle-en','gapped',this)"><span class="sp-cic">▥</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('icicle-en','horizontal',this)"><span class="sp-cic">▦</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('icicle-en','radial',this)"><span class="sp-cic">◉</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('icicle-en','rank',this)"><span class="sp-cic">◉</span><span class="sp-clb">Rank</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="icicle-en-basic">
<p>Root row at the top, depth-based opacity, white separators between bands.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / layers</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-basic.html"></iframe>
</div>
<div class="sp-variant" id="icicle-en-gapped">
<p>Rounded corners with a small gap between every node, both across and between rows.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>gapped / spaced / isolated / padded</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-gapped.html"></iframe>
</div>
<div class="sp-variant" id="icicle-en-horizontal">
<p>Root column on the left, depth grows rightward instead of downward.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / h / sideways / left_to_right</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-horizontal.html"></iframe>
</div>
<div class="sp-variant" id="icicle-en-radial">
<p>The same hierarchy in polar coordinates instead of cartesian — depth becomes ring radius, horizontal span becomes angular span. The classic icicle/sunburst duality: identical data, identical layout math (`xspan`, `depth`), different coordinate system.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"radial"</code></span><span><strong>Aliases</strong> <code>radial / sunburst / polar / mandala</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-radial.html"></iframe>
</div>
<div class="sp-variant" id="icicle-en-rank">
<p>Colors every node by its value rank among its same-depth peers (indigo → red, lowest to highest) instead of by raw share of the grand total — makes cross-branch comparisons at a given level obvious even when one branch is much bigger than another.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rank"</code></span><span><strong>Aliases</strong> <code>rank / percentile / peer_rank / standing</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-rank.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.icicle(title, labels, parents, values, *, variant="basic", palette=None, **kwargs) -> Chart`

Alias : `sp.icicle`, `sp.icicles`, `sp.icicle_chart`, `sp.icicle_family`, `sp.build_icicle`

## Description

`sp.icicle()` représente une hiérarchie sous forme de bandes empilées : chaque niveau de profondeur de l'arbre est une rangée horizontale, et la largeur de chaque nœud dans sa rangée est proportionnelle à sa valeur. C'est une alternative rectangulaire à [`sunburst()`](sunburst.md), avec exactement le même schéma d'entrée (`labels` / `parents` / `values`) — tout jeu de données déjà utilisé avec `sunburst()` ou `treemap()` fonctionne sans modification.

> **Encodage de la hiérarchie** — `labels` liste tous les nœuds, `parents` donne le libellé du parent de chaque nœud (`""` pour une racine). Les valeurs des feuilles viennent de `values` ; les nœuds internes à `0` sont calculés automatiquement comme la somme de leurs descendants.

## Variantes

<div data-sp-registry-table="variants" data-family="icicle"></div>

Une variante inconnue retombe sur la valeur par défaut enregistrée. Les clés de variantes peuvent être préfixées par `en_`, `fr_`, `en-` ou `fr-`.

## Données

`labels` (`list[str]`) — Libellés des nœuds (un par ligne). `parents` (`list[str]`) — Parent de chaque nœud (`""` pour les racines). `values` (`list[float]`) — Valeurs feuilles ; zéros internes calculés auto.

## Paramètres

<div data-sp-registry-table="options" data-family="icicle"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="icicle"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="icicle"></div>
</div>

<div class="sp-cls sp-open" id="icicle-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('icicle-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('icicle-fr','basic',this)"><span class="sp-cic">▤</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('icicle-fr','gapped',this)"><span class="sp-cic">▥</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('icicle-fr','horizontal',this)"><span class="sp-cic">▦</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('icicle-fr','radial',this)"><span class="sp-cic">◉</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('icicle-fr','rank',this)"><span class="sp-cic">◉</span><span class="sp-clb">Rang</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="icicle-fr-basic">
<p>Rangée racine en haut, opacité dégressive selon la profondeur, séparateurs blancs entre bandes.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic / layers</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-basic.html"></iframe>
</div>
<div class="sp-variant" id="icicle-fr-gapped">
<p>Coins arrondis avec un petit espace entre chaque nœud, en largeur comme entre rangées.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gapped"</code></span><span><strong>Alias</strong> <code>gapped / spaced / isolated / padded</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-gapped.html"></iframe>
</div>
<div class="sp-variant" id="icicle-fr-horizontal">
<p>Colonne racine à gauche, la profondeur se développe vers la droite au lieu du bas.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"horizontal"</code></span><span><strong>Alias</strong> <code>horizontal / h / sideways / left_to_right</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-horizontal.html"></iframe>
</div>
<div class="sp-variant" id="icicle-fr-radial">
<p>La même hiérarchie en coordonnées polaires plutôt que cartésiennes — la profondeur devient un rayon d'anneau, l'empan horizontal devient un empan angulaire. La dualité classique icicle/sunburst : mêmes données, même calcul de mise en page (`xspan`, `depth`), système de coordonnées différent.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"radial"</code></span><span><strong>Alias</strong> <code>radial / sunburst / polar / mandala</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-radial.html"></iframe>
</div>
<div class="sp-variant" id="icicle-fr-rank">
<p>Colore chaque nœud selon son rang de valeur parmi ses pairs de même profondeur (indigo → rouge, du plus faible au plus élevé) plutôt que sa part brute du total général — rend évidentes les comparaisons entre branches à un niveau donné, même quand une branche est bien plus grande qu'une autre.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"rank"</code></span><span><strong>Alias</strong> <code>rank / percentile / peer_rank / standing</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/icicle-rank.html"></iframe>
</div>
</div>
</div>

</div>
