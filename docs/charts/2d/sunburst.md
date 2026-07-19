# Sunburst — Hierarchical Ring Chart

<div class="lang-en">

<style>

.sp-tabs{border:1px solid var(--sp-border);border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:var(--sp-surface);border-bottom:1px solid var(--sp-border);flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:var(--sp-text-muted);cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:var(--sp-text)}
.sp-tb.sp-act{color:var(--sp-accent);border-bottom-color:var(--sp-accent)}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}
.sp-cls-toggle:hover{background:#312e81;color:#e0e7ff;transform:translateY(-1px)}
.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540 0%,#141d33 70%,var(--sp-surface) 100%);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;box-shadow:-6px 4px 14px -4px rgba(0,0,0,.55),inset 0 1px 0 rgba(255,255,255,.04),inset 1px 0 0 rgba(255,255,255,.05);transition:all .25s cubic-bezier(.5,0,.2,1);clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}
.sp-cls-tab:hover{background:linear-gradient(90deg,#23304d,#1a2540 70%,#141d33);color:#e0e7ff;margin-left:-40px;box-shadow:-8px 6px 18px -4px rgba(0,0,0,.6),inset 0 1px 0 rgba(255,255,255,.06)}
.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3 0%,#1e1b4b 50%,var(--sp-surface) 100%);color:#f5f3ff;margin-left:-46px;box-shadow:-10px 8px 22px -4px rgba(99,102,241,.35),-3px 0 0 0 #818cf8 inset,inset 0 1px 0 rgba(165,180,252,.2);font-weight:700;z-index:3}
.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;font-weight:900;letter-spacing:-1px;width:16px;text-align:center;text-shadow:0 0 6px rgba(165,180,252,.4)}
.sp-cls-tab.sp-cact .sp-cic{color:#e0e7ff;text-shadow:0 0 10px rgba(165,180,252,.7)}
.sp-cls-tab .sp-clb{display:none;font-weight:inherit;letter-spacing:.01em}
.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}
.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;position:relative;z-index:1;border-radius:0 14px 14px 0;overflow:hidden}
.sp-variant{display:none}
.sp-variant.sp-von{display:block;animation:spFade .25s ease}
@keyframes spFade{from{opacity:0;transform:translateX(8px)}to{opacity:1;transform:translateX(0)}}
.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid var(--sp-accent);border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:var(--sp-text);font-size:12px}
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

</div>
</div>

</div>
