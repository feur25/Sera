# Funnel — Conversion / Pipeline Chart

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:480px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.funnel(title, labels, values, *, variant="basic", palette=None, show_text=True, **kwargs) -> Chart`

## Description

`sp.funnel()` renders the entire funnel-chart family: a stacked sequence of stages where each step’s width encodes a value. The `variant` keyword switches the geometry without changing any other parameter. Funnels are the standard for conversion analytics (visitors → signups → paid), recruiting pipelines, sales pipelines, process drop-off and any descending-cohort analysis.
## Variants

<div data-sp-registry-table="variants" data-family="funnel"></div>

## Parameters

<div data-sp-registry-table="options" data-family="funnel"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

## Tips

- Sort stages descending before passing them in (or use `sort_order="desc"`).
- Use `"conversion"` when the audience cares about stage-to-stage retention rate.
- The `"pyramid"` variant works best when values follow a steep decay.
- For broad audiences, `"chevron"` reads as a sales pipeline more naturally than the trapezoid.

<div class="sp-cls sp-open" id="funnel-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('funnel-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('funnel-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','stepped',this)"><span class="sp-cic">S</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','rounded',this)"><span class="sp-cic">R</span><span class="sp-clb">Rounded</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','chevron',this)"><span class="sp-cic">V</span><span class="sp-clb">Chevron</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','pyramid',this)"><span class="sp-cic">P</span><span class="sp-clb">Pyramid</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','inverted',this)"><span class="sp-cic">I</span><span class="sp-clb">Inverted</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','conversion',this)"><span class="sp-cic">C</span><span class="sp-clb">Conversion</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','compare',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Compare</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','grouped',this)"><span class="sp-cic">▥</span><span class="sp-clb">Grouped</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="funnel-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / trapezoid / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic centered trapezoid pyramid — each step’s top width inherits the previous bottom width.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-basic.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-stepped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>stepped / bar / rect / rectangle</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Centered rectangles per stage (no diagonal slope), width ∝ value/max.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-stepped.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-rounded">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rounded"</code></span><span><strong>Aliases</strong> <code>rounded / round / pill / smooth</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trapezoids with rounded outer corners — softer, pill-like aesthetic.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-rounded.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-chevron">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"chevron"</code></span><span><strong>Aliases</strong> <code>chevron / arrow / pipeline / pointer</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Pentagon arrow shapes pointing right — sales-pipeline / process flow style.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-chevron.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-pyramid">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"pyramid"</code></span><span><strong>Aliases</strong> <code>pyramid / triangle / cone / point</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Continuous pyramid: each level narrows progressively to a point at the bottom.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-pyramid.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-inverted">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"inverted"</code></span><span><strong>Aliases</strong> <code>inverted / inverse / reverse / upside_down</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Vertically flipped funnel — widest stage at the bottom (growth pyramid).</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-inverted.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-conversion">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"conversion"</code></span><span><strong>Aliases</strong> <code>conversion / dropoff / rate / steps</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Basic trapezoid plus drop-off percentage between consecutive stages displayed in red.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-conversion.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-compare">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"compare"</code></span><span><strong>Aliases</strong> <code>compare / multi / side_by_side / funnels</code></span><span><strong>Required</strong> <code>series</code></span><span><strong>Optional</strong> <code>series_names</code>, <code>category_series</code>, <code>text_info</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Several independent funnels side by side, each with its own stage count and its own scale - the native equivalent of composing multiple <code>go.Funnel()</code> traces in Plotly, without ever building a trace by hand. Pass <code>series</code> (one value list per funnel), <code>series_names</code> (funnel names) and <code>category_series</code> (one stage-label list per funnel, since funnels can have different stage counts). Automatically selected whenever <code>series</code> has more than one entry and <code>category_series</code> is given - use <code>"grouped"</code> instead when every group shares the same stages. <code>text_info</code> combines <code>"value"</code>, <code>"percent_initial"</code>, <code>"percent_previous"</code> and <code>"percent_total"</code> with <code>+</code>.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-compare.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-grouped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"grouped"</code></span><span><strong>Aliases</strong> <code>grouped / colored / by_color / shared_stages</code></span><span><strong>Required</strong> <code>labels</code>, <code>series</code></span><span><strong>Optional</strong> <code>series_names</code>, <code>text_info</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">One funnel, several colored groups sharing the exact same stages - each stage's band splits into adjacent, touching segments (no gap) whose widths are proportional to each group's own value on one shared linear scale, so the combined shape tapers naturally like a single funnel. The native equivalent of <code>px.funnel(df, x="number", y="stage", color="office")</code> in Plotly - works with any number of groups, not just two. Auto-selected whenever <code>series</code> has more than one entry and <code>category_series</code> is not given.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-grouped.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.funnel(title, labels, values, *, variant="basic", palette=None, show_text=True, **kwargs) -> Chart`

<h2>Description</h2>

`sp.funnel()` produit toute la famille des entonnoirs : une séquence d’étapes empilées dont la largeur encode une valeur. Le mot-clé `variant` permute la géométrie sans changer aucun autre paramètre. Standard pour l’analyse de conversion (visiteurs → inscrits → payants), pipelines de recrutement, pipelines commerciaux, fuites de processus et toute analyse de cohorte décroissante.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="funnel"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="funnel"></div>

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

<h2>Astuces</h2>

- Triez les étapes décroissantes avant de les passer (ou utilisez `sort_order="desc"`).
- Utilisez `"conversion"` quand l’audience s’intéresse au taux de rétention entre étapes.
- La variante `"pyramid"` fonctionne mieux avec des valeurs en forte décroissance.
- Pour un public large, `"chevron"` se lit plus naturellement comme un pipeline commercial qu’un trapèze.

<div class="sp-cls sp-open" id="funnel-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('funnel-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('funnel-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','stepped',this)"><span class="sp-cic">S</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','rounded',this)"><span class="sp-cic">R</span><span class="sp-clb">Rounded</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','chevron',this)"><span class="sp-cic">V</span><span class="sp-clb">Chevron</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','pyramid',this)"><span class="sp-cic">P</span><span class="sp-clb">Pyramid</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','inverted',this)"><span class="sp-cic">I</span><span class="sp-clb">Inverted</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','conversion',this)"><span class="sp-cic">C</span><span class="sp-clb">Conversion</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','compare',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Comparer</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','grouped',this)"><span class="sp-cic">▥</span><span class="sp-clb">Groupé</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="funnel-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / trapezoid / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Pyramide trapézoïdale centrée classique — le haut de chaque étape hérite du bas de la précédente.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-basic.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-stepped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>stepped / bar / rect / rectangle</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Rectangles centrés par étape (sans pente diagonale), largeur ∝ valeur/max.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-stepped.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-rounded">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rounded"</code></span><span><strong>Aliases</strong> <code>rounded / round / pill / smooth</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trapèzes avec coins arrondis — esthétique douce de type pilule.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-rounded.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-chevron">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"chevron"</code></span><span><strong>Aliases</strong> <code>chevron / arrow / pipeline / pointer</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Pentagones en flèche pointant à droite — style pipeline commercial / processus.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-chevron.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-pyramid">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"pyramid"</code></span><span><strong>Aliases</strong> <code>pyramid / triangle / cone / point</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Pyramide continue : chaque niveau se rétrécit progressivement jusqu’à une pointe.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-pyramid.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-inverted">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"inverted"</code></span><span><strong>Aliases</strong> <code>inverted / inverse / reverse / upside_down</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Entonnoir inversé verticalement — étape la plus large en bas (pyramide de croissance).</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-inverted.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-conversion">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"conversion"</code></span><span><strong>Aliases</strong> <code>conversion / dropoff / rate / steps</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trapèze de base avec le pourcentage de chute entre étapes affiché en rouge.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-conversion.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-compare">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"compare"</code></span><span><strong>Alias</strong> <code>compare / multi / side_by_side / funnels</code></span><span><strong>Requis</strong> <code>series</code></span><span><strong>Optionnel</strong> <code>series_names</code>, <code>category_series</code>, <code>text_info</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Plusieurs entonnoirs indépendants côte à côte, chacun avec son propre nombre d'étapes et sa propre échelle - l'équivalent natif de composer plusieurs traces <code>go.Funnel()</code> en Plotly, sans jamais construire de trace à la main. Passez <code>series</code> (une liste de valeurs par entonnoir), <code>series_names</code> (noms des entonnoirs) et <code>category_series</code> (une liste d'étiquettes d'étapes par entonnoir, puisque les entonnoirs peuvent avoir des nombres d'étapes différents). Sélectionné automatiquement dès que <code>series</code> a plus d'une entrée et que <code>category_series</code> est fourni - utilisez <code>"grouped"</code> quand tous les groupes partagent les mêmes étapes. <code>text_info</code> combine <code>"value"</code>, <code>"percent_initial"</code>, <code>"percent_previous"</code> et <code>"percent_total"</code> avec <code>+</code>.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-compare.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-grouped">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"grouped"</code></span><span><strong>Alias</strong> <code>grouped / colored / by_color / shared_stages</code></span><span><strong>Requis</strong> <code>labels</code>, <code>series</code></span><span><strong>Optionnel</strong> <code>series_names</code>, <code>text_info</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Un seul entonnoir, plusieurs groupes colorés partageant exactement les mêmes étapes - la bande de chaque étape se divise en segments adjacents et jointifs (sans espace) dont les largeurs sont proportionnelles à la valeur de chaque groupe sur une même échelle linéaire partagée, si bien que la forme combinée se rétrécit naturellement comme un seul entonnoir. L'équivalent natif de <code>px.funnel(df, x="number", y="stage", color="office")</code> en Plotly - fonctionne avec n'importe quel nombre de groupes, pas seulement deux. Sélectionné automatiquement dès que <code>series</code> a plus d'une entrée et que <code>category_series</code> n'est pas fourni.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-grouped.html"></iframe>
</div>
</div>
</div>

</div>
