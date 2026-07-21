# Gauge - Single-Value Arc Indicator

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:360px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.gauge(title, *, value, min_val=0.0, max_val=100.0, label="", variant="basic", comparison=0.0, **kwargs) -> Chart`

## Description

`sp.gauge()` is the unified entry point for the gauge family. A gauge maps a single scalar to a colored arc with optional thresholds - perfect for status / health / utilization KPIs. The `variant` keyword switches the geometry (half, three-quarter, full ring), the embellishments (needle, ticks, glow) and the layering (single arc vs. concentric arcs for value-vs-target).

## Variants

<div data-sp-registry-table="variants" data-family="gauge"></div>

## Parameters

<div data-sp-registry-table="options" data-family="gauge"></div>

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="gauge-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('gauge-en')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('gauge-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','radial',this)"><span class="sp-cic">R</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','arc270',this)"><span class="sp-cic">A</span><span class="sp-clb">Arc 270</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','sleek',this)"><span class="sp-cic">S</span><span class="sp-clb">Sleek</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','tick',this)"><span class="sp-cic">T</span><span class="sp-clb">Tick</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','segmented',this)"><span class="sp-cic">Z</span><span class="sp-clb">Segmented</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','glow',this)"><span class="sp-cic">G</span><span class="sp-clb">Glow</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-en','concentric',this)"><span class="sp-cic">C</span><span class="sp-clb">Concentric</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="gauge-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / half / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Half-circle gauge with needle and color thresholds - the speedometer everyone knows.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-basic.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-radial">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"radial"</code></span><span><strong>Aliases</strong> <code>radial / donut / ring / full</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Full-circle donut progress arc - elegant ring KPI for dashboards.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-radial.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-arc270">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arc270"</code></span><span><strong>Aliases</strong> <code>arc270 / three_quarter / arc / wide</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">270-degree arc - more arc length for finer reading than a half-circle.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-arc270.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-sleek">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"sleek"</code></span><span><strong>Aliases</strong> <code>sleek / minimal / clean / flat</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">No needle, no ticks - oversized value text on a clean colored arc.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-sleek.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-tick">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"tick"</code></span><span><strong>Aliases</strong> <code>tick / tickmarks / scaled / ruler</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Half-arc with ruler tick marks every 5% and major labels every 25%.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-tick.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-segmented">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"segmented"</code></span><span><strong>Aliases</strong> <code>segmented / battery / signal / chunked</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Battery / signal-bar style with discrete chunks lighting up by threshold.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-segmented.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-glow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"glow"</code></span><span><strong>Aliases</strong> <code>glow / neon / halo / luminous</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Neon glow effect on the active arc - dramatic dark dashboard look.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-glow.html"></iframe>
</div>
<div class="sp-variant" id="gauge-en-concentric">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"concentric"</code></span><span><strong>Aliases</strong> <code>concentric / rings / target / dual</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Two concentric arcs: outer = current, inner = comparison or target.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-concentric.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.gauge(title, *, value, min_val=0.0, max_val=100.0, label="", variant="basic", comparison=0.0, **kwargs) -> Chart`

<h2>Description</h2>

`sp.gauge()` est le point d entree unique pour la famille jauge. Une jauge associe un scalaire unique a un arc colore avec des seuils optionnels - parfait pour des KPIs de statut / sante / utilisation. Le mot-cle `variant` change la geometrie (demi, trois-quart, anneau complet), les ornements (aiguille, ticks, glow) et la composition (arc simple ou arcs concentriques pour valeur-vs-cible).

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="gauge"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="gauge"></div>

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

---

<div class="sp-cls sp-open" id="gauge-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('gauge-fr')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('gauge-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','radial',this)"><span class="sp-cic">R</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','arc270',this)"><span class="sp-cic">A</span><span class="sp-clb">Arc 270</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','sleek',this)"><span class="sp-cic">S</span><span class="sp-clb">Sleek</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','tick',this)"><span class="sp-cic">T</span><span class="sp-clb">Tick</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','segmented',this)"><span class="sp-cic">Z</span><span class="sp-clb">Segmented</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','glow',this)"><span class="sp-cic">G</span><span class="sp-clb">Glow</span></button>
<button class="sp-cls-tab" onclick="spCls('gauge-fr','concentric',this)"><span class="sp-cic">C</span><span class="sp-clb">Concentric</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="gauge-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / half / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Jauge demi-cercle avec aiguille et seuils colores - le compteur que tout le monde connait.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-basic.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-radial">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"radial"</code></span><span><strong>Aliases</strong> <code>radial / donut / ring / full</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Arc de progression circulaire complet - KPI elegant en anneau pour tableaux de bord.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-radial.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-arc270">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arc270"</code></span><span><strong>Aliases</strong> <code>arc270 / three_quarter / arc / wide</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Arc de 270 degres - plus de longueur pour une lecture plus fine qu un demi-cercle.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-arc270.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-sleek">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"sleek"</code></span><span><strong>Aliases</strong> <code>sleek / minimal / clean / flat</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Sans aiguille ni ticks - valeur en grand sur un arc colore epure.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-sleek.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-tick">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"tick"</code></span><span><strong>Aliases</strong> <code>tick / tickmarks / scaled / ruler</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Demi-arc avec graduations regle tous les 5% et labels majeurs tous les 25%.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-tick.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-segmented">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"segmented"</code></span><span><strong>Aliases</strong> <code>segmented / battery / signal / chunked</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Style batterie / barre de reseau avec segments discrets s allumant par seuil.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-segmented.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-glow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"glow"</code></span><span><strong>Aliases</strong> <code>glow / neon / halo / luminous</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Effet neon sur l arc actif - look dashboard sombre tres marquant.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-glow.html"></iframe>
</div>
<div class="sp-variant" id="gauge-fr-concentric">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"concentric"</code></span><span><strong>Aliases</strong> <code>concentric / rings / target / dual</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Deux arcs concentriques : externe = courant, interne = comparaison ou cible.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gauge-concentric.html"></iframe>
</div>
</div>
</div>

</div>
