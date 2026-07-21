# Lollipop - Categorical Value Sticks

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

`sp.lollipop(title, labels, values, *, variant="basic", color_groups=None, highlight_index=-1, **kwargs) -> Chart`

## Description

`sp.lollipop()` is the unified entry point for the lollipop family. Each item becomes a thin stick capped by a dot - lighter ink than a bar chart for the same ranking, and the family includes circular, diverging, focused and grouped editorial layouts (the Office variant reproduces the season-rating panel pattern).

## Variants

<div data-sp-registry-table="variants" data-family="lollipop"></div>

## Parameters

<div data-sp-registry-table="options" data-family="lollipop"></div>

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="lollipop-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('lollipop-en')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('lollipop-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-en','cleveland',this)"><span class="sp-cic">C</span><span class="sp-clb">Cleveland</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-en','diverging',this)"><span class="sp-cic">D</span><span class="sp-clb">Diverging</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-en','circular',this)"><span class="sp-cic">O</span><span class="sp-clb">Circular</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-en','highlight',this)"><span class="sp-cic">H</span><span class="sp-clb">Highlight</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-en','office',this)"><span class="sp-cic">G</span><span class="sp-clb">Office</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="lollipop-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / vertical</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Vertical sticks topped with dots - the canonical lollipop for ranked categorical values.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-basic.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-en-cleveland">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cleveland"</code></span><span><strong>Aliases</strong> <code>cleveland / horizontal / h / row</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Horizontal Cleveland dot plot - long labels read naturally and dots align cleanly along value axis.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-cleveland.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-en-diverging">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"diverging"</code></span><span><strong>Aliases</strong> <code>diverging / div / signed / delta</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Sticks pivot around the mean: green points sit above, red points below - perfect for deviation analysis.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-diverging.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-en-circular">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"circular"</code></span><span><strong>Aliases</strong> <code>circular / polar / radial / round</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Polar layout where each category is an angular spoke - eye-catching for small alphabets and dashboard tiles.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-circular.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-en-highlight">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"highlight"</code></span><span><strong>Aliases</strong> <code>highlight / spotlight / focus / dim</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">All sticks dimmed except one accent (auto-max or `highlight_index`) - ideal for editorial focus.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-highlight.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-en-office">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"office"</code></span><span><strong>Aliases</strong> <code>office / grouped / season / panel</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Group-aware lollipops with per-group mean line and color band - inspired by The Office IMDb season chart.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-office.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.lollipop(title, labels, values, *, variant="basic", color_groups=None, highlight_index=-1, **kwargs) -> Chart`

<h2>Description</h2>

`sp.lollipop()` est le point d entree unique pour la famille lollipop. Chaque item devient un baton fin termine par un point - moins d encre qu un bar chart pour le meme classement, et la famille couvre des layouts circulaires, divergents, focalises et editoriaux groupes (la variante Office reproduit le motif des saisons IMDb de The Office).

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="lollipop"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="lollipop"></div>

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

---

<div class="sp-cls sp-open" id="lollipop-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('lollipop-fr')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('lollipop-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-fr','cleveland',this)"><span class="sp-cic">C</span><span class="sp-clb">Cleveland</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-fr','diverging',this)"><span class="sp-cic">D</span><span class="sp-clb">Diverging</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-fr','circular',this)"><span class="sp-cic">O</span><span class="sp-clb">Circular</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-fr','highlight',this)"><span class="sp-cic">H</span><span class="sp-clb">Highlight</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-fr','office',this)"><span class="sp-cic">G</span><span class="sp-clb">Office</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="lollipop-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / vertical</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Batons verticaux surmontes de points - le lollipop canonique pour valeurs categorielles classees.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-basic.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-fr-cleveland">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cleveland"</code></span><span><strong>Aliases</strong> <code>cleveland / horizontal / h / row</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Cleveland dot plot horizontal - les longs libelles se lisent naturellement, les points s alignent sur l axe des valeurs.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-cleveland.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-fr-diverging">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"diverging"</code></span><span><strong>Aliases</strong> <code>diverging / div / signed / delta</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Batons pivotent autour de la moyenne: vert au-dessus, rouge en-dessous - parfait pour les ecarts.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-diverging.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-fr-circular">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"circular"</code></span><span><strong>Aliases</strong> <code>circular / polar / radial / round</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Disposition polaire ou chaque categorie est un rayon - tres lisible pour petits jeux et dashboards.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-circular.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-fr-highlight">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"highlight"</code></span><span><strong>Aliases</strong> <code>highlight / spotlight / focus / dim</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Tous les batons grises sauf un accent (auto-max ou `highlight_index`) - ideal pour focus editorial.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-highlight.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-fr-office">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"office"</code></span><span><strong>Aliases</strong> <code>office / grouped / season / panel</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Lollipops groupes avec moyenne par groupe et bande de couleur - inspire du chart IMDb de The Office.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-office.html"></iframe>
</div>
</div>
</div>

</div>
