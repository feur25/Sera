# Dumbbell - Before / After Two-Point Comparison

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

`sp.dumbbell(title, labels, start, end, *, variant="basic", series_name_start="Start", series_name_end="End", **kwargs) -> Chart`

## Description

`sp.dumbbell()` is the unified entry point for the dumbbell-chart family. Each row plots two values - typically a before and an after - linked by a connector, making it the chart of choice for change, gap or comparison-over-time analyses (salary equity, turnaround KPIs, A/B uplifts, etc.). The `variant` keyword switches the visual treatment without touching the data.

## Variants

<div data-sp-registry-table="variants" data-family="dumbbell"></div>

## Parameters

<div data-sp-registry-table="options" data-family="dumbbell"></div>

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="dumbbell-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('dumbbell-en')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('dumbbell-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','arrow',this)"><span class="sp-cic">A</span><span class="sp-clb">Arrow</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','delta',this)"><span class="sp-cic">D</span><span class="sp-clb">Delta</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','barbell',this)"><span class="sp-cic">W</span><span class="sp-clb">Barbell</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','glow',this)"><span class="sp-cic">G</span><span class="sp-clb">Glow</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','dotted',this)"><span class="sp-cic">O</span><span class="sp-clb">Dotted</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-en','ranked',this)"><span class="sp-cic">R</span><span class="sp-clb">Ranked</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="dumbbell-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / dot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic two-dot dumbbell with a gray connecting bar; the workhorse of before/after comparisons.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-basic.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-arrow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arrow"</code></span><span><strong>Aliases</strong> <code>arrow / directional / delta_arrow / flow</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Arrowhead points from start to end so direction of change is immediate.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-arrow.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-delta">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"delta"</code></span><span><strong>Aliases</strong> <code>delta / change / diff / signed</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bar between dots is colored by sign (green up, red down) to encode direction and magnitude.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-delta.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-barbell">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"barbell"</code></span><span><strong>Aliases</strong> <code>barbell / thick / weighted / editorial</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Square weighted endpoints on a thick gray axis - editorial barbell look for slides.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-barbell.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-glow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"glow"</code></span><span><strong>Aliases</strong> <code>glow / halo / neon / soft</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Soft halo around endpoints with thin connector for a luminous, modern feel.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-glow.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-dotted">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dotted"</code></span><span><strong>Aliases</strong> <code>dotted / dashed / minimal / thin</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Dashed connector with hollow ring markers - lightweight and airy.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-dotted.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-en-ranked">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ranked"</code></span><span><strong>Aliases</strong> <code>ranked / ranking / ordered / numbered</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Adds a numeric rank in front of every label - perfect for top-N comparisons.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-ranked.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.dumbbell(title, labels, start, end, *, variant="basic", series_name_start="Start", series_name_end="End", **kwargs) -> Chart`

<h2>Description</h2>

`sp.dumbbell()` est le point d entree unique pour la famille dumbbell. Chaque ligne montre deux valeurs - typiquement avant/apres - reliees par un connecteur, ce qui en fait le choix naturel pour visualiser un changement, un ecart ou une evolution (equite salariale, KPIs de redressement, uplifts A/B, etc.). Le mot-cle `variant` change le style visuel sans toucher aux donnees.

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="dumbbell"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="dumbbell"></div>

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

---

<div class="sp-cls sp-open" id="dumbbell-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('dumbbell-fr')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('dumbbell-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','arrow',this)"><span class="sp-cic">A</span><span class="sp-clb">Arrow</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','delta',this)"><span class="sp-cic">D</span><span class="sp-clb">Delta</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','barbell',this)"><span class="sp-cic">W</span><span class="sp-clb">Barbell</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','glow',this)"><span class="sp-cic">G</span><span class="sp-clb">Glow</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','dotted',this)"><span class="sp-cic">O</span><span class="sp-clb">Dotted</span></button>
<button class="sp-cls-tab" onclick="spCls('dumbbell-fr','ranked',this)"><span class="sp-cic">R</span><span class="sp-clb">Ranked</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="dumbbell-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / dot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Dumbbell classique a deux points et barre grise - la base des comparaisons avant/apres.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-basic.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-arrow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arrow"</code></span><span><strong>Aliases</strong> <code>arrow / directional / delta_arrow / flow</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Une fleche pointe du depart vers l arrivee, la direction du changement est immediate.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-arrow.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-delta">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"delta"</code></span><span><strong>Aliases</strong> <code>delta / change / diff / signed</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">La barre entre les points prend la couleur du signe (vert hausse, rouge baisse).</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-delta.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-barbell">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"barbell"</code></span><span><strong>Aliases</strong> <code>barbell / thick / weighted / editorial</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Halteres carres sur un axe epais - look editorial pour presentations.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-barbell.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-glow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"glow"</code></span><span><strong>Aliases</strong> <code>glow / halo / neon / soft</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Halo doux autour des extremites avec connecteur fin - style lumineux et moderne.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-glow.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-dotted">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dotted"</code></span><span><strong>Aliases</strong> <code>dotted / dashed / minimal / thin</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Connecteur en pointilles et marqueurs en anneaux - leger et aere.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-dotted.html"></iframe>
</div>
<div class="sp-variant" id="dumbbell-fr-ranked">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ranked"</code></span><span><strong>Aliases</strong> <code>ranked / ranking / ordered / numbered</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Ajoute un rang numerique devant chaque label - ideal pour comparer un top-N.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dumbbell-ranked.html"></iframe>
</div>
</div>
</div>

</div>
