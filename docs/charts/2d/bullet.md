# Bullet - Compact KPI vs Target

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
.sp-preview-frame{width:100%;height:360px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.bullet(title, labels, values, *, targets=None, max_vals=None, ranges=None, comparisons=None, variant="basic", **kwargs) -> Chart`


## Description

`sp.bullet()` is the unified entry point for the bullet-chart family. Inspired by Edward Tufte, a bullet packs an actual value, a target, qualitative ranges and a scale into a single horizontal row - perfect for KPI dashboards where space is precious. The `variant` keyword switches the visual treatment (zones, traffic light, thermometer, progress pill, dot, ghost-bar comparison) without touching the data.

## Variants

<div data-sp-registry-table="variants" data-family="bullet"></div>

## Parameters

<div data-sp-registry-table="options" data-family="bullet"></div>

## Returns

`Chart` - object with `.html` property and `.show()` method.

---


<div class="sp-cls sp-open" id="bullet-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bullet-en')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bullet-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','thermo',this)"><span class="sp-cic">T</span><span class="sp-clb">Thermo</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','segmented',this)"><span class="sp-cic">Z</span><span class="sp-clb">Segmented</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','minimal',this)"><span class="sp-cic">M</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','dot',this)"><span class="sp-cic">D</span><span class="sp-clb">Dot</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','progress',this)"><span class="sp-cic">P</span><span class="sp-clb">Progress</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-en','compare',this)"><span class="sp-cic">C</span><span class="sp-clb">Compare</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="bullet-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / standard</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic Tufte bullet: track + qualitative range + value bar + target tick.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-basic.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-stacked">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stacked_ranges / zones / qualitative</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Three graduated qualitative bands (poor / satisfactory / good) drawn behind the value bar.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-stacked.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-thermo">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"thermo"</code></span><span><strong>Aliases</strong> <code>thermo / thermometer / vertical / column</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Vertical thermometer style with a bulb base - dramatic for KPIs in a row.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-thermo.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-segmented">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"segmented"</code></span><span><strong>Aliases</strong> <code>segmented / traffic / rag / zones_color</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Traffic-light segmented track (red / amber / green) for status dashboards.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-segmented.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-minimal">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"minimal"</code></span><span><strong>Aliases</strong> <code>minimal / sparkline / clean / naked</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Sparkline-thin pill bar with target tick only - ultra-clean inline indicator.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-minimal.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-dot">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dot"</code></span><span><strong>Aliases</strong> <code>dot / point / marker / pip</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Single dot on a track instead of a bar - dot-plot variant of the bullet.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-dot.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-progress">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"progress"</code></span><span><strong>Aliases</strong> <code>progress / pill / bar / percent</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Pill-shape gradient progress bar with a percentage label centered inside.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-progress.html"></iframe>
</div>
<div class="sp-variant" id="bullet-en-compare">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"compare"</code></span><span><strong>Aliases</strong> <code>compare / vs / ghost / prior</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Adds a ghost bar (e.g. previous period via comparisons) behind the current value.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-compare.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.bullet(title, labels, values, *, targets=None, max_vals=None, ranges=None, comparisons=None, variant="basic", **kwargs) -> Chart`


<h2>Description</h2>

`sp.bullet()` est le point d entree unique pour la famille bullet. Inspire par Edward Tufte, le bullet condense valeur, cible, zones qualitatives et echelle dans une seule ligne horizontale - parfait pour des dashboards KPIs serres. Le mot-cle `variant` change l aspect (zones, feu tricolore, thermometre, pillule de progression, point, comparaison par barre fantome) sans toucher aux donnees.

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="bullet"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="bullet"></div>

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

---


<div class="sp-cls sp-open" id="bullet-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bullet-fr')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bullet-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','thermo',this)"><span class="sp-cic">T</span><span class="sp-clb">Thermo</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','segmented',this)"><span class="sp-cic">Z</span><span class="sp-clb">Segmented</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','minimal',this)"><span class="sp-cic">M</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','dot',this)"><span class="sp-cic">D</span><span class="sp-clb">Dot</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','progress',this)"><span class="sp-cic">P</span><span class="sp-clb">Progress</span></button>
<button class="sp-cls-tab" onclick="spCls('bullet-fr','compare',this)"><span class="sp-cic">C</span><span class="sp-clb">Compare</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="bullet-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / standard</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bullet de Tufte classique : piste + zone qualitative + barre de valeur + tick de cible.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-basic.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-stacked">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stacked_ranges / zones / qualitative</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trois bandes qualitatives graduees (faible / correct / bon) derriere la barre de valeur.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-stacked.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-thermo">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"thermo"</code></span><span><strong>Aliases</strong> <code>thermo / thermometer / vertical / column</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Style thermometre vertical avec bulbe - tres parlant pour des KPIs alignes.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-thermo.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-segmented">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"segmented"</code></span><span><strong>Aliases</strong> <code>segmented / traffic / rag / zones_color</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Piste segmentee feu tricolore (rouge / orange / vert) pour tableaux de bord.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-segmented.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-minimal">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"minimal"</code></span><span><strong>Aliases</strong> <code>minimal / sparkline / clean / naked</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barre pillule fine type sparkline avec uniquement le tick cible - indicateur inline epure.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-minimal.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-dot">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dot"</code></span><span><strong>Aliases</strong> <code>dot / point / marker / pip</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Un seul point sur la piste au lieu d une barre - variante dot-plot du bullet.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-dot.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-progress">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"progress"</code></span><span><strong>Aliases</strong> <code>progress / pill / bar / percent</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barre de progression pillule en degrade avec pourcentage centre.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-progress.html"></iframe>
</div>
<div class="sp-variant" id="bullet-fr-compare">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"compare"</code></span><span><strong>Aliases</strong> <code>compare / vs / ghost / prior</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Ajoute une barre fantome (par ex. periode precedente via comparisons) derriere la valeur courante.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/bullet-compare.html"></iframe>
</div>
</div>
</div>

</div>
