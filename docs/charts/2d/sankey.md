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
<button class="sp-cls-tab" onclick="spCls('sankey-en','hourglass',this)"><span class="sp-cic">⧗</span><span class="sp-clb">Hourglass</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-en','matrix',this)"><span class="sp-cic">▦</span><span class="sp-clb">Matrix</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-en','beacon',this)"><span class="sp-cic">◎</span><span class="sp-clb">Beacon</span></button>
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
<div class="sp-variant" id="sankey-en-hourglass">
<p>Every source starts as its own thin row on the left and every ribbon is pulled through a single fixed pinch point before fanning back out into a radial arc of target wedges on the right — each wedge's angular width is proportional to the square root of its total incoming flow (a mild compression so one outlier target can't visually swallow the rest of the arc). Built for bipartite many-sources-to-few-categories data, e.g. foods flowing into the nutrients they're rich in.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"hourglass"</code></span><span><strong>Aliases</strong> <code>hourglass / radiant_flow / nutrient_flow / braided / flow_bloom</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-hourglass.html"></iframe>
</div>
<div class="sp-variant" id="sankey-en-matrix">
<p>Sources are laid out as a dot-matrix grid instead of a node list — one marker per record, sized by its own weight and colored by the category it ultimately flows to — with thin low-opacity ribbons converging on a handful of large target circles sized by total share and labeled with their percentage of the whole. Built for record-level "big data" flows: many individual rows collapsing into a few outcome buckets.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"matrix"</code></span><span><strong>Aliases</strong> <code>matrix / mosaic / dot_matrix / grid_flow / big_data</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-matrix.html"></iframe>
</div>
<div class="sp-variant" id="sankey-en-beacon">
<p>A single source radiates to every target around a full circle instead of two flat columns — each target sits at its own angle with a short pill sized and colored by weight (a continuous colorscale, not a fixed palette), connected back to the hub by a thin bowed spoke. Built for one-to-many schedules where each target also carries its own label, e.g. a departure board: one hub airport, every outbound flight arranged around the dial with its destination, time and duration.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"beacon"</code></span><span><strong>Aliases</strong> <code>beacon / flight_radar / route_wheel / departure_board / hub_wheel</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-beacon.html"></iframe>
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
<button class="sp-cls-tab" onclick="spCls('sankey-fr','hourglass',this)"><span class="sp-cic">⧗</span><span class="sp-clb">Sablier</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-fr','matrix',this)"><span class="sp-cic">▦</span><span class="sp-clb">Matrice</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-fr','beacon',this)"><span class="sp-cic">◎</span><span class="sp-clb">Balise</span></button>
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
<div class="sp-variant" id="sankey-fr-hourglass">
<p>Chaque source démarre comme une fine ligne à gauche, et chaque ruban est tiré à travers un unique point d'étranglement fixe avant de se déployer en éventail radial de secteurs cibles à droite — la largeur angulaire de chaque secteur est proportionnelle à la racine carrée de son flux entrant total (une légère compression pour qu'une cible atypique n'écrase pas visuellement tout l'arc). Conçu pour des données bipartites "beaucoup de sources vers peu de catégories", par exemple des aliments qui alimentent les nutriments dans lesquels ils sont riches.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"hourglass"</code></span><span><strong>Alias</strong> <code>hourglass / radiant_flow / nutrient_flow / braided / flow_bloom</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-hourglass.html"></iframe>
</div>
<div class="sp-variant" id="sankey-fr-matrix">
<p>Les sources sont disposées en grille de points plutôt qu'en liste de nœuds — un marqueur par enregistrement, dimensionné selon son propre poids et coloré selon la catégorie vers laquelle il finit par s'écouler — avec de fins rubans à faible opacité convergeant vers quelques grands cercles cibles dimensionnés par leur part totale et étiquetés avec leur pourcentage de l'ensemble. Conçu pour des flux "big data" au niveau enregistrement : de nombreuses lignes individuelles qui se regroupent en quelques catégories de résultat.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"matrix"</code></span><span><strong>Alias</strong> <code>matrix / mosaic / dot_matrix / grid_flow / big_data</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-matrix.html"></iframe>
</div>
<div class="sp-variant" id="sankey-fr-beacon">
<p>Une seule source rayonne vers chaque cible sur un cercle complet plutôt que sur deux colonnes plates — chaque cible occupe son propre angle avec une pastille dimensionnée et colorée selon son poids (une échelle de couleur continue, pas une palette fixe), reliée au centre par un fin arc courbe. Conçu pour des horaires un-vers-plusieurs où chaque cible porte aussi sa propre étiquette, par exemple un tableau des départs : un aéroport central, chaque vol au départ disposé autour du cadran avec sa destination, son heure et sa durée.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"beacon"</code></span><span><strong>Alias</strong> <code>beacon / flight_radar / route_wheel / departure_board / hub_wheel</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-beacon.html"></iframe>
</div>
</div>
</div>

</div>
