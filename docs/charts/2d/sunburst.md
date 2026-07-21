# Sunburst — Hierarchical Ring Chart

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:520px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.sunburst(title, labels, parents, values, *, variant="basic", palette=None, **kwargs) -> Chart`

## Description

`sp.sunburst()` is the unified entry point for the entire sunburst-chart family. A sunburst represents a hierarchy as concentric rings: the innermost ring is the root, each outer ring is a deeper level, and angular size encodes value. The `variant` keyword selects the visual style without changing any other parameter. Sunbursts are the standard for visualizing nested taxonomies (org charts, file systems, market segmentation, expense categories, phylogenetic trees) and outperform classic pie charts as soon as a real hierarchy exists.

> **Hierarchy encoding** — `labels` lists every node, `parents` gives the parent label of each node ("" for a root). Leaf values are taken from `values`; internal node values are auto-rolled-up from descendants when set to 0.

## Variants

<div data-sp-registry-table="variants" data-family="sunburst"></div>

## Parameters

<div data-sp-registry-table="options" data-family="sunburst"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="sunburst-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('sunburst-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('sunburst-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-en','donut',this)"><span class="sp-cic">D</span><span class="sp-clb">Donut</span></button><button class="sp-cls-tab" onclick="spCls('sunburst-en','outlined',this)"><span class="sp-cic">O</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-en','gapped',this)"><span class="sp-cic">G</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-en','depth_fade',this)"><span class="sp-cic">X</span><span class="sp-clb">Depth fade</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-en','mono',this)"><span class="sp-cic">M</span><span class="sp-clb">Mono</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-en','zoomable',this)"><span class="sp-cic">Z</span><span class="sp-clb">Zoomable</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="sunburst-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / ring</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic concentric rings with depth-based opacity and white separators.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-basic.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-donut">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"donut"</code></span><span><strong>Aliases</strong> <code>donut / hole / ring_hole / donut_ring</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Larger central hole with the formatted grand total displayed at the center.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-donut.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-outlined">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / wireframe</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Wireframe wedges: white fill + colored stroke that thins on deeper rings.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-outlined.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-gapped">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>gapped / spaced / isolated / petals</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Angular and radial margins between every wedge for crisp petal-like separation.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-gapped.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-depth_fade">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"depth_fade"</code></span><span><strong>Aliases</strong> <code>depth_fade / fade / fading / depth</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Standard palette but opacity decreases with depth, focusing the eye on top levels.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-depth_fade.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-mono">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mono"</code></span><span><strong>Aliases</strong> <code>mono / monochrome / single / uniform</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Single-color rings differentiated only by depth-based opacity. Editorial look.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-mono.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-zoomable">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"zoomable"</code></span><span><strong>Aliases</strong> <code>zoomable / zoom / animated / interactive / drill</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Click a ring segment to zoom into that branch, animating every other segment to its new angle/radius. Click the center to zoom back out.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-zoomable.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.sunburst(title, labels, parents, values, *, variant="basic", palette=None, **kwargs) -> Chart`

<h2>Description</h2>

`sp.sunburst()` est le point d entree unifie pour toute la famille des graphiques sunburst. Un sunburst represente une hierarchie sous forme d anneaux concentriques : l anneau interieur est la racine, chaque anneau exterieur est un niveau plus profond, et l angle code la valeur. Le mot-cle `variant` change le style sans toucher aux autres parametres. Les sunbursts sont la reference pour visualiser des taxonomies imbriquees (organigrammes, systemes de fichiers, segmentation marche, categories de depenses, arbres phylogenetiques) et surpassent le camembert des qu une vraie hierarchie existe.

> **Encodage de la hierarchie** — `labels` liste tous les noeuds, `parents` donne le libelle du parent de chaque noeud ("" pour une racine). Les valeurs des feuilles viennent de `values` ; les noeuds internes a 0 sont calcules automatiquement comme la somme de leurs descendants.

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="sunburst"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="sunburst"></div>

<h2>Retour</h2>

`Chart` — objet avec une propriete `.html` et une methode `.show()`.

---

<div class="sp-cls sp-open" id="sunburst-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('sunburst-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('sunburst-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-fr','donut',this)"><span class="sp-cic">D</span><span class="sp-clb">Donut</span></button><button class="sp-cls-tab" onclick="spCls('sunburst-fr','outlined',this)"><span class="sp-cic">O</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-fr','gapped',this)"><span class="sp-cic">G</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-fr','depth_fade',this)"><span class="sp-cic">X</span><span class="sp-clb">Depth fade</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-fr','mono',this)"><span class="sp-cic">M</span><span class="sp-clb">Mono</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-fr','zoomable',this)"><span class="sp-cic">Z</span><span class="sp-clb">Zoomable</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="sunburst-fr-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / ring</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Anneaux concentriques classiques avec opacite degressive selon la profondeur.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-basic.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-donut">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"donut"</code></span><span><strong>Aliases</strong> <code>donut / hole / ring_hole / donut_ring</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Large trou central avec total general formate (k/M) au centre.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-donut.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-outlined">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / wireframe</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Quartiers en fil de fer : fond blanc + contour colore amincissant en profondeur.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-outlined.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-gapped">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>gapped / spaced / isolated / petals</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Marges angulaires et radiales entre quartiers pour une separation nette en petales.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-gapped.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-depth_fade">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"depth_fade"</code></span><span><strong>Aliases</strong> <code>depth_fade / fade / fading / depth</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Palette standard mais opacite decroissante en profondeur pour concentrer le regard.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-depth_fade.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-mono">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mono"</code></span><span><strong>Aliases</strong> <code>mono / monochrome / single / uniform</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Anneaux monochromes differencies uniquement par opacite. Rendu editorial.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-mono.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-zoomable">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"zoomable"</code></span><span><strong>Alias</strong> <code>zoomable / zoom / animated / interactive / drill</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Cliquer un segment zoome sur cette branche, en animant chaque autre segment vers son nouvel angle/rayon. Cliquer le centre pour dezoomer.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-zoomable.html"></iframe>
</div>

</div>
</div>

</div>
