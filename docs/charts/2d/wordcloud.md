# Word Cloud - Six Rendering Architectures

<div class="lang-en">

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}
.sp-cls-toggle:hover{background:#312e81;color:#e0e7ff;transform:translateY(-1px)}
.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540 0%,#141d33 70%,#0f172a 100%);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;box-shadow:-6px 4px 14px -4px rgba(0,0,0,.55),inset 0 1px 0 rgba(255,255,255,.04),inset 1px 0 0 rgba(255,255,255,.05);transition:all .25s cubic-bezier(.5,0,.2,1);clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}
.sp-cls-tab:hover{background:linear-gradient(90deg,#23304d,#1a2540 70%,#141d33);color:#e0e7ff;margin-left:-40px;box-shadow:-8px 6px 18px -4px rgba(0,0,0,.6),inset 0 1px 0 rgba(255,255,255,.06)}
.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3 0%,#1e1b4b 50%,#0f172a 100%);color:#f5f3ff;margin-left:-46px;box-shadow:-10px 8px 22px -4px rgba(99,102,241,.35),-3px 0 0 0 #818cf8 inset,inset 0 1px 0 rgba(165,180,252,.2);font-weight:700;z-index:3}
.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;font-weight:900;letter-spacing:-1px;width:16px;text-align:center;text-shadow:0 0 6px rgba(165,180,252,.4)}
.sp-cls-tab.sp-cact .sp-cic{color:#e0e7ff;text-shadow:0 0 10px rgba(165,180,252,.7)}
.sp-cls-tab .sp-clb{display:none;font-weight:inherit;letter-spacing:.01em}
.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}
.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;position:relative;z-index:1;border-radius:0 14px 14px 0;overflow:hidden}
.sp-variant{display:none}
.sp-variant.sp-von{display:block;animation:spFade .25s ease}
@keyframes spFade{from{opacity:0;transform:translateX(8px)}to{opacity:1;transform:translateX(0)}}
.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
.sp-preview-frame{width:100%;height:520px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.build_wordcloud(title, words, frequencies, *, variant="basic", shape="rect", **kwargs) -> Chart`

Aliases: `sp.build_wordcloud` &middot; `sp.wordcloud` &middot; `sp.word_cloud` &middot; `sp.tag_cloud` &middot; `sp.cloud` &middot; `sp.text_cloud` &middot; `sp.token_cloud`

## Description

`sp.build_wordcloud()` packs weighted tokens into six rendering architectures. **Basic** is the canonical spiral packer driven by a parametric `shape=` mask (rect, circle, heart, bird, glasses, diamond, star). **Bubble** gives each word its own color-filled disc sized by frequency - a packed-bubble layout. **Context** is an InfraNodus-style text-network cloud: words positioned by a force-directed layout driven by co-occurrence edges so semantically close words cluster spatially, colored by community. **Image** accepts any binary pixel mask (logo, icon, photo). **LabelMap** draws a datamapplot-style clustered scatter with leader-line labels. **Network** renders a keyword co-occurrence graph with bezier-curved edges.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / spiral / rect / shape / shaped` | Spiral packing inside a parametric shape mask. Pick the silhouette via the `shape=` argument (rect, circle, heart, bird, glasses, diamond, star). |
| `"bubble"` | `bubble / bubbles / packed / circles / packing / pack` | Each word gets a colored disc sized by frequency - a packed-bubble word cloud. Words float in labeled circles, no overlap. |
| `"context"` | `context / semantic / infranodus / text_network / textnetwork / force / force_directed` | Text-network word cloud (InfraNodus style): words positioned by force-directed layout based on co-occurrence edges, colored by semantic cluster - semantically close words appear spatially close. |
| `"image"` | `image / img / mask / picture / photo / silhouette` | Words flow inside a custom binary image mask - upload any silhouette (logo, icon, photo) and the cloud takes its shape. |
| `"labelmap"` | `labelmap / label_map / datamap / datamapplot / topic_map / scatter_labels` | Datamapplot-style topic map - clustered scatter of points colored per category, with cluster labels positioned via leader lines. |
| `"network"` | `network / graph / keywords / co_occurrence / cooccurrence / knowledge_graph` | Keyword co-occurrence graph - golden-angle node layout, bezier-curved edges, frequency-sized circles. Editorial style of academic keyword maps. |

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

## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `height` | all |
| `hover` | all |
| `palette` | all |
| `title` | all |
| `width` | all |

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

Aliases: `sp.build_wordcloud` &middot; `sp.wordcloud` &middot; `sp.word_cloud` &middot; `sp.tag_cloud` &middot; `sp.cloud` &middot; `sp.text_cloud` &middot; `sp.token_cloud`

<h2>Description</h2>

`sp.build_wordcloud()` propose six architectures de rendu. **Basic** est le packer spirale canonique pilote par un masque `shape=` (rect, circle, heart, bird, glasses, diamond, star). **Bubble** donne a chaque mot un disque colore dimensionne par frequence - un layout bubble-packed. **Context** est un nuage texte-reseau style InfraNodus : mots positionnes par layout force-dirige base sur les aretes de co-occurrence, colores par communaute. **Image** accepte n importe quel masque binaire de pixels. **LabelMap** dessine un scatter clusterise style datamapplot avec etiquettes en lignes de rappel. **Network** rend un graphe de co-occurrence de mots-cles avec aretes bezier.

<h2>Variantes</h2>

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / spiral / rect / shape / shaped` | Packing en spirale dans un masque parametrique. Choisissez la silhouette via l argument `shape=` (rect, circle, heart, bird, glasses, diamond, star). |
| `"bubble"` | `bubble / bubbles / packed / circles / packing / pack` | Chaque mot obtient un disque colore dimensionne par frequence - un nuage de bulles pack. Les mots flottent dans des cercles etiquetes, sans chevauchement. |
| `"context"` | `context / semantic / infranodus / text_network / textnetwork / force / force_directed` | Nuage de mots texte-reseau (style InfraNodus) : mots positionnes par layout force-dirige base sur les aretes de co-occurrence, colores par cluster semantique - les mots proches semantiquement sont proches spatialement. |
| `"image"` | `image / img / mask / picture / photo / silhouette` | Mots a l interieur d un masque binaire personnalise - uploadez n importe quelle silhouette et le nuage en prend la forme. |
| `"labelmap"` | `labelmap / label_map / datamap / datamapplot / topic_map / scatter_labels` | Carte thematique style datamapplot - scatter de points clusterises colores par categorie, avec etiquettes de cluster positionnees via lignes de rappel. |
| `"network"` | `network / graph / keywords / co_occurrence / cooccurrence / knowledge_graph` | Graphe de co-occurrence de mots-cles - layout en angle d or, aretes courbes bezier, cercles dimensionnes par frequence. Style editorial des cartes de mots-cles academiques. |

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

<h2>Parametres</h2>

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title` | `str` | requis | Titre du graphique |
| `words` | `list[str]` | requis | Tokens (toutes variantes sauf labelmap) |
| `frequencies` | `list[float]` | requis | Poids par mot (controle la taille) |
| `variant` | `str` | "basic" | Mode de rendu (voir Variantes) |
| `shape` | `str` | "rect" | Sous-forme pour `basic` (voir Formes) |
| `mask` | `list[int]` | None | Masque binaire (variante `image`) - row-major, 1=interieur |
| `mask_width` | `int` | 0 | Largeur du masque (variante `image`) |
| `mask_height` | `int` | 0 | Hauteur du masque (variante `image`) |
| `points_x` | `list[float]` | None | Coords x du scatter (variante `labelmap`) |
| `points_y` | `list[float]` | None | Coords y du scatter (variante `labelmap`) |
| `category_indices` | `list[int]` | None | Id de cluster par point/mot (`labelmap`, `context`) |
| `cluster_labels` | `list[str]` | None | Etiquette par cluster (variante `labelmap`) |
| `edges_i` | `list[int]` | None | Indices source des aretes (variantes `network`, `context`) |
| `edges_j` | `list[int]` | None | Indices cible des aretes (variantes `network`, `context`) |
| `edges_w` | `list[float]` | None | Poids des aretes (variantes `network`, `context`) |
| `min_font` | `float` | 12.0 | Plus petite taille de police rendue |
| `max_font` | `float` | 72.0 | Plus grande taille de police rendue |
| `palette` | `list[int]` | None | Palette personnalisee |
| `bg_color` | `str` | auto | Couleur de fond |
| `width` | `int` | 900 | Largeur du canvas (px) |
| `height` | `int` | 500 | Hauteur du canvas (px) |

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
</div>
