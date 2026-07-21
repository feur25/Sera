# Sankey Diagram

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:380px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.sankey(title, labels, edges_i, edges_j, edges_w, *, variant="basic", **kwargs) -> Chart`

Aliases: `sp.sankey`, `sp.sankeys`, `sp.sankey_chart`, `sp.sankey_diagram`, `sp.flow_chart`

## Description

Sankey diagrams visualize flows between nodes. Node widths and link widths are proportional to flow volumes. Edges are defined by source indices (`edges_i`), target indices (`edges_j`), and weights (`edges_w`). Nodes are laid out in columns by BFS depth.

## Variants

<div data-sp-registry-table="variants" data-family="sankey"></div>

## Data

`labels` (`list[str]`) — Node names. `edges_i` (`list[int]`) — Source node indices. `edges_j` (`list[int]`) — Target node indices. `edges_w` (`list[float]`) — Flow weights. `width` / `height` (`int`) — Chart dimensions.

## Parameters

<div data-sp-registry-table="options" data-family="sankey"></div>

## Themes

<div data-sp-registry-table="themes" data-family="sankey"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="sankey"></div>
</div>

<div class="sp-cls sp-open" id="sankey-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('sankey-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('sankey-en','basic',this)"><span class="sp-cic">⇉</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-en','gapped',this)"><span class="sp-cic">⇥</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-en','ribbon',this)"><span class="sp-cic">▬</span><span class="sp-clb">Ribbon</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-en','minimal',this)"><span class="sp-cic">—</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-en','sorted',this)"><span class="sp-cic">⇅</span><span class="sp-clb">Sorted</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="sankey-en-basic">
<p>Standard bezier ribbon links</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-basic.html"></iframe>
</div>
<div class="sp-variant" id="sankey-en-gapped">
<p>Increased node spacing</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>gapped / spaced / separated</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-gapped.html"></iframe>
</div>
<div class="sp-variant" id="sankey-en-ribbon">
<p>Wider nodes and ribbons</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ribbon"</code></span><span><strong>Aliases</strong> <code>ribbon / wide / thick</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-ribbon.html"></iframe>
</div>
<div class="sp-variant" id="sankey-en-minimal">
<p>Thin outline style</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"minimal"</code></span><span><strong>Aliases</strong> <code>minimal / thin / outline</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-minimal.html"></iframe>
</div>
<div class="sp-variant" id="sankey-en-sorted">
<p>Reorders nodes within each depth column by descending total throughput, so the dominant flows cluster together instead of sitting in input order — makes it easy to spot which nodes carry the most volume.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"sorted"</code></span><span><strong>Aliases</strong> <code>sorted / reordered / by_flow / ranked</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-sorted.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.sankey(title, labels, edges_i, edges_j, edges_w, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.sankey`, `sp.sankeys`, `sp.sankey_chart`, `sp.sankey_diagram`, `sp.flow_chart`

## Description

Les diagrammes de Sankey visualisent des flux entre nœuds. La largeur des nœuds et des liens est proportionnelle au volume du flux. Les arêtes sont définies par des indices source (`edges_i`), des indices cible (`edges_j`), et des poids (`edges_w`). Les nœuds sont disposés en colonnes par profondeur BFS.

## Variantes

<div data-sp-registry-table="variants" data-family="sankey"></div>

## Données

`labels` (`list[str]`) — Noms des nœuds. `edges_i` (`list[int]`) — Indices des nœuds source. `edges_j` (`list[int]`) — Indices des nœuds cible. `edges_w` (`list[float]`) — Poids des flux. `width` / `height` (`int`) — Dimensions du graphique.

## Paramètres

<div data-sp-registry-table="options" data-family="sankey"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="sankey"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="sankey"></div>
</div>

<div class="sp-cls sp-open" id="sankey-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('sankey-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('sankey-fr','basic',this)"><span class="sp-cic">⇉</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-fr','gapped',this)"><span class="sp-cic">⇥</span><span class="sp-clb">Espacé</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-fr','ribbon',this)"><span class="sp-cic">▬</span><span class="sp-clb">Ruban</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-fr','minimal',this)"><span class="sp-cic">—</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-fr','sorted',this)"><span class="sp-cic">⇅</span><span class="sp-clb">Trié</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="sankey-fr-basic">
<p>Liens en rubans bézier standards</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-basic.html"></iframe>
</div>
<div class="sp-variant" id="sankey-fr-gapped">
<p>Espacement des nœuds augmenté</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gapped"</code></span><span><strong>Alias</strong> <code>gapped / spaced / separated</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-gapped.html"></iframe>
</div>
<div class="sp-variant" id="sankey-fr-ribbon">
<p>Nœuds et rubans plus larges</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"ribbon"</code></span><span><strong>Alias</strong> <code>ribbon / wide / thick</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-ribbon.html"></iframe>
</div>
<div class="sp-variant" id="sankey-fr-minimal">
<p>Style filaire fin</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"minimal"</code></span><span><strong>Alias</strong> <code>minimal / thin / outline</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-minimal.html"></iframe>
</div>
<div class="sp-variant" id="sankey-fr-sorted">
<p>Réordonne les nœuds de chaque colonne de profondeur par débit total décroissant, pour que les flux dominants se regroupent au lieu de rester dans l'ordre d'entrée — facilite le repérage des nœuds qui transportent le plus de volume.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"sorted"</code></span><span><strong>Alias</strong> <code>sorted / reordered / by_flow / ranked</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-sorted.html"></iframe>
</div>
</div>
</div>

</div>
