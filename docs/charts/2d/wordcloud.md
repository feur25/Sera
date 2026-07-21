# Word Cloud - Six Rendering Architectures

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

`sp.build_wordcloud(title, words, frequencies, *, variant="basic", shape="rect", **kwargs) -> Chart`

## Description

`sp.build_wordcloud()` packs weighted tokens into six rendering architectures. **Basic** is the canonical spiral packer driven by a parametric `shape=` mask (rect, circle, heart, bird, glasses, diamond, star). **Bubble** gives each word its own color-filled disc sized by frequency - a packed-bubble layout. **Context** is an InfraNodus-style text-network cloud: words positioned by a force-directed layout driven by co-occurrence edges so semantically close words cluster spatially, colored by community. **Image** accepts any binary pixel mask (logo, icon, photo). **LabelMap** draws a datamapplot-style clustered scatter with leader-line labels. **Network** renders a keyword co-occurrence graph with bezier-curved edges.

### Shapes (for variant `basic`)

The `basic` variant accepts a `shape=` argument that selects the silhouette mask:

| Shape | Aliases | Description |
|---|---|---|
| `"rect"` | `rect / rectangle / box / default` | Rectangular Archimedean spiral - the textbook word cloud. |
| `"circle"` | `circle / round / disk / ball` | Words packed inside a perfect disc. |
| `"heart"` | `heart / love / valentine` | Cardioid heart silhouette. |
| `"bird"` | `bird / twitter / tweet / icon` | Composite-disk stylised bird silhouette. |
| `"glasses"` | `glasses / sunglasses / shades / specs` | Sunglasses silhouette (two ellipses + bridge). |
| `"diamond"` | `diamond / rhombus / lozenge` | Rotated square / rhombus silhouette. |
| `"star"` | `star / starburst / 5-point` | 5-pointed star silhouette. |

## Variants

<div data-sp-registry-table="variants" data-family="wordcloud"></div>

## Parameters

<div data-sp-registry-table="options" data-family="wordcloud"></div>

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="wordcloud-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('wordcloud-en')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('wordcloud-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-en','bubble',this)"><span class="sp-cic">U</span><span class="sp-clb">Bubble</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-en','context',this)"><span class="sp-cic">X</span><span class="sp-clb">Context</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-en','image',this)"><span class="sp-cic">I</span><span class="sp-clb">Image</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-en','labelmap',this)"><span class="sp-cic">L</span><span class="sp-clb">LabelMap</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-en','network',this)"><span class="sp-cic">N</span><span class="sp-clb">Network</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-en','neuron',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Neuron</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="wordcloud-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / spiral / rect / shape / shaped</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Spiral packing inside a parametric shape mask. Pick the silhouette via the `shape=` argument (rect, circle, heart, bird, glasses, diamond, star).</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-basic.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-en-bubble">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bubble"</code></span><span><strong>Aliases</strong> <code>bubble / bubbles / packed / circles / packing / pack</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Each word gets a colored disc sized by frequency - a packed-bubble word cloud. Words float in labeled circles, no overlap.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-bubble.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-en-context">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"context"</code></span><span><strong>Aliases</strong> <code>context / semantic / infranodus / text_network / textnetwork / force / force_directed</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Text-network word cloud (InfraNodus style): words positioned by force-directed layout based on co-occurrence edges, colored by semantic cluster - semantically close words appear spatially close.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-context.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-en-image">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"image"</code></span><span><strong>Aliases</strong> <code>image / img / mask / picture / photo / silhouette</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Words flow inside a custom binary image mask - upload any silhouette (logo, icon, photo) and the cloud takes its shape.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-image.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-en-labelmap">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"labelmap"</code></span><span><strong>Aliases</strong> <code>labelmap / label_map / datamap / datamapplot / topic_map / scatter_labels</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Datamapplot-style topic map - clustered scatter of points colored per category, with cluster labels positioned via leader lines.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-labelmap.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-en-network">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"network"</code></span><span><strong>Aliases</strong> <code>network / graph / keywords / co_occurrence / cooccurrence / knowledge_graph</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Keyword co-occurrence graph - golden-angle node layout, bezier-curved edges, frequency-sized circles. Editorial style of academic keyword maps.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-network.html"></iframe>
</div>

<div class="sp-variant" id="wordcloud-en-neuron">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"neuron"</code></span><span><strong>Aliases</strong> <code>neuron / neural / brain / synapse / network_glow / nodes</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Neural-network word cloud on a dark background. Words become glowing nodes; faint connecting edges link nearest neighbors, evoking a synaptic graph.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-neuron.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.build_wordcloud(title, words, frequencies, *, variant="basic", shape="rect", **kwargs) -> Chart`

<h2>Description</h2>

`sp.build_wordcloud()` propose six architectures de rendu. **Basic** est le packer spirale canonique pilote par un masque `shape=` (rect, circle, heart, bird, glasses, diamond, star). **Bubble** donne a chaque mot un disque colore dimensionne par frequence - un layout bubble-packed. **Context** est un nuage texte-reseau style InfraNodus : mots positionnes par layout force-dirige base sur les aretes de co-occurrence, colores par communaute. **Image** accepte n importe quel masque binaire de pixels. **LabelMap** dessine un scatter clusterise style datamapplot avec etiquettes en lignes de rappel. **Network** rend un graphe de co-occurrence de mots-cles avec aretes bezier.

<h3>Formes (pour la variante `basic`)</h3>

La variante `basic` accepte un argument `shape=` :

| Forme | Alias | Description |
|---|---|---|
| `"rect"` | `rect / rectangle / box / default` | Rectangular Archimedean spiral - the textbook word cloud. |
| `"circle"` | `circle / round / disk / ball` | Words packed inside a perfect disc. |
| `"heart"` | `heart / love / valentine` | Cardioid heart silhouette. |
| `"bird"` | `bird / twitter / tweet / icon` | Composite-disk stylised bird silhouette. |
| `"glasses"` | `glasses / sunglasses / shades / specs` | Sunglasses silhouette (two ellipses + bridge). |
| `"diamond"` | `diamond / rhombus / lozenge` | Rotated square / rhombus silhouette. |
| `"star"` | `star / starburst / 5-point` | 5-pointed star silhouette. |

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="wordcloud"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="wordcloud"></div>

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

<div class="sp-cls sp-open" id="wordcloud-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('wordcloud-fr')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('wordcloud-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-fr','bubble',this)"><span class="sp-cic">U</span><span class="sp-clb">Bubble</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-fr','context',this)"><span class="sp-cic">X</span><span class="sp-clb">Context</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-fr','image',this)"><span class="sp-cic">I</span><span class="sp-clb">Image</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-fr','labelmap',this)"><span class="sp-cic">L</span><span class="sp-clb">LabelMap</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-fr','network',this)"><span class="sp-cic">N</span><span class="sp-clb">Network</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-fr','neuron',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Neuron</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="wordcloud-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / spiral / rect / shape / shaped</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Packing en spirale dans un masque parametrique. Choisissez la silhouette via l argument `shape=` (rect, circle, heart, bird, glasses, diamond, star).</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-basic.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-fr-bubble">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bubble"</code></span><span><strong>Aliases</strong> <code>bubble / bubbles / packed / circles / packing / pack</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Chaque mot obtient un disque colore dimensionne par frequence - un nuage de bulles pack. Les mots flottent dans des cercles etiquetes, sans chevauchement.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-bubble.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-fr-context">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"context"</code></span><span><strong>Aliases</strong> <code>context / semantic / infranodus / text_network / textnetwork / force / force_directed</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Nuage de mots texte-reseau (style InfraNodus) : mots positionnes par layout force-dirige base sur les aretes de co-occurrence, colores par cluster semantique - les mots proches semantiquement sont proches spatialement.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-context.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-fr-image">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"image"</code></span><span><strong>Aliases</strong> <code>image / img / mask / picture / photo / silhouette</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Mots a l interieur d un masque binaire personnalise - uploadez n importe quelle silhouette et le nuage en prend la forme.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-image.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-fr-labelmap">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"labelmap"</code></span><span><strong>Aliases</strong> <code>labelmap / label_map / datamap / datamapplot / topic_map / scatter_labels</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Carte thematique style datamapplot - scatter de points clusterises colores par categorie, avec etiquettes de cluster positionnees via lignes de rappel.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-labelmap.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-fr-network">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"network"</code></span><span><strong>Aliases</strong> <code>network / graph / keywords / co_occurrence / cooccurrence / knowledge_graph</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Graphe de co-occurrence de mots-cles - layout en angle d or, aretes courbes bezier, cercles dimensionnes par frequence. Style editorial des cartes de mots-cles academiques.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-network.html"></iframe>
</div>

<div class="sp-variant" id="wordcloud-fr-neuron">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"neuron"</code></span><span><strong>Alias</strong> <code>neuron / neural / brain / synapse / network_glow / nodes</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Nuage de mots style reseau de neurones sur fond sombre. Les mots deviennent des noeuds lumineux relies par des aretes fines evoquant un graphe synaptique.</p>

<div class="sp-preview-label">Apercu</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-neuron.html"></iframe>
</div>
</div>
</div>

</div>
