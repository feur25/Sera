# Hive Plot

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

`sp.hive(title, axes, labels, categories, values, edges_i, edges_j, edges_w, *, variant="basic", **kwargs) -> Chart`

Aliases: `sp.hive`, `sp.hive_plot`, `sp.hive_chart`, `sp.hive_graph`, `sp.radial_network`

## Description

Hive plots organize network nodes on radial axes by category. Each axis corresponds to one node group (`axes`). Node position along the axis is determined by `values` (0–1). Edges between nodes are drawn as straight or curved lines through the center.

## Variants

<div data-sp-registry-table="variants" data-family="hive"></div>

## Data

`axes` (`list[str]`) — Axis (group) names. `labels` (`list[str]`) — Node names. `categories` (`list[str]`) — Group assignment per node. `values` (`list[float]`) — Node position along axis (0–1). `edges_i` (`list[int]`) — Source node indices. `edges_j` (`list[int]`) — Target node indices. `edges_w` (`list[float]`) — Edge weights. `width` / `height` (`int`) — Chart dimensions.

## Parameters

<div data-sp-registry-table="options" data-family="hive"></div>

## Themes

<div data-sp-registry-table="themes" data-family="hive"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="hive"></div>
</div>

<div class="sp-cls sp-open" id="hive-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('hive-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('hive-en','basic',this)"><span class="sp-cic">✦</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-en','curved',this)"><span class="sp-cic">∫</span><span class="sp-clb">Curved</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-en','weighted',this)"><span class="sp-cic">≈</span><span class="sp-clb">Weighted</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-en','minimal',this)"><span class="sp-cic">—</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-en','directed',this)"><span class="sp-cic">➤</span><span class="sp-clb">Directed</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="hive-en-basic">
<p>Straight edge lines</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hive-basic.html"></iframe>
</div>
<div class="sp-variant" id="hive-en-curved">
<p>Cubic bezier curves through center</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"curved"</code></span><span><strong>Aliases</strong> <code>curved / smooth / bezier</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hive-curved.html"></iframe>
</div>
<div class="sp-variant" id="hive-en-weighted">
<p>Stroke width proportional to edge weight</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"weighted"</code></span><span><strong>Aliases</strong> <code>weighted / width / value</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hive-weighted.html"></iframe>
</div>
<div class="sp-variant" id="hive-en-minimal">
<p>Straight (uncurved), thin, low-opacity edges with small strokeless nodes — a decluttered reading of dense hive plots that trades the curved-edge chrome for raw connectivity.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"minimal"</code></span><span><strong>Aliases</strong> <code>minimal / thin / clean</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hive-minimal.html"></iframe>
</div>
<div class="sp-variant" id="hive-en-directed">
<p>Draws a small arrowhead where each edge lands on its target node — hive plots are frequently used for directed graphs (network traffic, citations), and this makes the direction readable at a glance instead of implied.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"directed"</code></span><span><strong>Aliases</strong> <code>directed / arrows / flow / dependency</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hive-directed.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.hive(title, axes, labels, categories, values, edges_i, edges_j, edges_w, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.hive`, `sp.hive_plot`, `sp.hive_chart`, `sp.hive_graph`, `sp.radial_network`

## Description

Les hive plots organisent les nœuds d'un réseau sur des axes radiaux par catégorie. Chaque axe correspond à un groupe de nœuds (`axes`). La position d'un nœud le long de l'axe est déterminée par `values` (0–1). Les arêtes entre nœuds sont tracées en lignes droites ou courbes passant par le centre.

## Variantes

<div data-sp-registry-table="variants" data-family="hive"></div>

## Données

`axes` (`list[str]`) — Noms des axes (groupes). `labels` (`list[str]`) — Noms des nœuds. `categories` (`list[str]`) — Groupe assigné à chaque nœud. `values` (`list[float]`) — Position du nœud le long de l'axe (0–1). `edges_i` (`list[int]`) — Indices des nœuds source. `edges_j` (`list[int]`) — Indices des nœuds cible. `edges_w` (`list[float]`) — Poids des arêtes. `width` / `height` (`int`) — Dimensions du graphique.

## Paramètres

<div data-sp-registry-table="options" data-family="hive"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="hive"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="hive"></div>
</div>

<div class="sp-cls sp-open" id="hive-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('hive-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('hive-fr','basic',this)"><span class="sp-cic">✦</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-fr','curved',this)"><span class="sp-cic">∫</span><span class="sp-clb">Courbé</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-fr','weighted',this)"><span class="sp-cic">≈</span><span class="sp-clb">Pondéré</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-fr','minimal',this)"><span class="sp-cic">—</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-fr','directed',this)"><span class="sp-cic">➤</span><span class="sp-clb">Dirigé</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="hive-fr-basic">
<p>Arêtes en lignes droites</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hive-basic.html"></iframe>
</div>
<div class="sp-variant" id="hive-fr-curved">
<p>Courbes de bézier cubiques passant par le centre</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"curved"</code></span><span><strong>Alias</strong> <code>curved / smooth / bezier</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hive-curved.html"></iframe>
</div>
<div class="sp-variant" id="hive-fr-weighted">
<p>Épaisseur du trait proportionnelle au poids de l'arête</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"weighted"</code></span><span><strong>Alias</strong> <code>weighted / width / value</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hive-weighted.html"></iframe>
</div>
<div class="sp-variant" id="hive-fr-minimal">
<p>Arêtes droites (non courbées), fines et peu opaques, avec de petits nœuds sans contour — une lecture épurée des hive plots denses qui sacrifie le chrome des arêtes courbées au profit de la connectivité brute.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"minimal"</code></span><span><strong>Alias</strong> <code>minimal / thin / clean</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hive-minimal.html"></iframe>
</div>
<div class="sp-variant" id="hive-fr-directed">
<p>Trace une petite flèche là où chaque arête arrive sur son nœud cible — les hive plots servent souvent à représenter des graphes orientés (trafic réseau, citations), et ceci rend le sens lisible d'un coup d'œil plutôt qu'implicite.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"directed"</code></span><span><strong>Alias</strong> <code>directed / arrows / flow / dependency</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hive-directed.html"></iframe>
</div>
</div>
</div>

</div>
