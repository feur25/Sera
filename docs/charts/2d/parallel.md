# Parallel Coordinates - Multivariate Profile Lines

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

`sp.build_parallel(title, axes, series, *, variant="basic", **kwargs) -> Chart`


## Description

`sp.build_parallel()` renders a **parallel-coordinates** chart - one vertical axis per dimension, one polyline per row. Six variants cover the classical use cases: straight lines, smooth Bezier curves, categorical coloring, single-row highlight, density-blended overlay, and gradient coloring driven by any axis. Perfect for high-dimensional EDA, profile comparison, and class separability inspection.

## Variants

<div data-sp-registry-table="variants" data-family="parallel"></div>

## Parameters

<div data-sp-registry-table="options" data-family="parallel"></div>

## Returns

`Chart` - object with `.html` property and `.show()` method.

---


<div class="sp-cls sp-open" id="parallel-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('parallel-en')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('parallel-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('parallel-en','smooth',this)"><span class="sp-cic">S</span><span class="sp-clb">Smooth</span></button>
<button class="sp-cls-tab" onclick="spCls('parallel-en','categorical',this)"><span class="sp-cic">C</span><span class="sp-clb">Categorical</span></button>
<button class="sp-cls-tab" onclick="spCls('parallel-en','highlight',this)"><span class="sp-cic">H</span><span class="sp-clb">Highlight</span></button>
<button class="sp-cls-tab" onclick="spCls('parallel-en','density',this)"><span class="sp-cic">D</span><span class="sp-clb">Density</span></button>
<button class="sp-cls-tab" onclick="spCls('parallel-en','arc',this)"><span class="sp-cic">∿</span><span class="sp-clb">Arc</span></button>
<button class="sp-cls-tab" onclick="spCls('parallel-en','ribbon',this)"><span class="sp-cic">≈</span><span class="sp-clb">Ribbon</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="parallel-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / lines</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Straight polylines through every axis - the textbook parallel-coordinates chart.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-basic.html"></iframe>
</div>
<div class="sp-variant" id="parallel-en-smooth">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"smooth"</code></span><span><strong>Aliases</strong> <code>smooth / curved / bezier / spline</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bezier-smoothed lines reduce visual clutter for dense datasets.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-smooth.html"></iframe>
</div>
<div class="sp-variant" id="parallel-en-categorical">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code></span><span><strong>Aliases</strong> <code>categorical / category / groups / colored</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">One color per category - perfect for comparing classes side by side.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-categorical.html"></iframe>
</div>
<div class="sp-variant" id="parallel-en-highlight">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"highlight"</code></span><span><strong>Aliases</strong> <code>highlight / spotlight / focus / dim</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Spotlights one series and dims the others - great for storytelling.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-highlight.html"></iframe>
</div>
<div class="sp-variant" id="parallel-en-density">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"density"</code></span><span><strong>Aliases</strong> <code>density / fade / translucent / alpha</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Translucent lines reveal density bands inside thousands of profiles.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-density.html"></iframe>
</div>

<div class="sp-variant" id="parallel-en-arc">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arc"</code></span><span><strong>Aliases</strong> <code>arc / bezier_color / smooth_color / colored_bezier</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Smooth cubic bezier curves with per-series gradient coloring. Reduces visual clutter compared to straight lines while preserving individual series identity.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-arc.html"></iframe>
</div>

<div class="sp-variant" id="parallel-en-ribbon">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ribbon"</code></span><span><strong>Aliases</strong> <code>ribbon / flow / band / filled_bezier</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Filled bezier bands between adjacent axes. Each series is rendered as a translucent ribbon + thin solid stroke, creating a flowing Sankey-lite effect.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-ribbon.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.build_parallel(title, axes, series, *, variant="basic", **kwargs) -> Chart`


<h2>Description</h2>

`sp.build_parallel()` rend un graphique **parallel-coordinates** - un axe vertical par dimension, une polyligne par ligne. Six variantes couvrent les cas classiques : lignes droites, courbes Bezier, couleur par categorie, mise en avant d une ligne, overlay de densite, et degrade pilote par un axe. Ideal pour l EDA haute-dimension, la comparaison de profils, et l inspection de separabilite de classes.

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="parallel"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="parallel"></div>

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.


<div class="sp-cls sp-open" id="parallel-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('parallel-fr')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('parallel-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('parallel-fr','smooth',this)"><span class="sp-cic">S</span><span class="sp-clb">Smooth</span></button>
<button class="sp-cls-tab" onclick="spCls('parallel-fr','categorical',this)"><span class="sp-cic">C</span><span class="sp-clb">Categorical</span></button>
<button class="sp-cls-tab" onclick="spCls('parallel-fr','highlight',this)"><span class="sp-cic">H</span><span class="sp-clb">Highlight</span></button>
<button class="sp-cls-tab" onclick="spCls('parallel-fr','density',this)"><span class="sp-cic">D</span><span class="sp-clb">Density</span></button>
<button class="sp-cls-tab" onclick="spCls('parallel-fr','arc',this)"><span class="sp-cic">∿</span><span class="sp-clb">Arc</span></button>
<button class="sp-cls-tab" onclick="spCls('parallel-fr','ribbon',this)"><span class="sp-cic">≈</span><span class="sp-clb">Ribbon</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="parallel-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / lines</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Polylignes droites sur tous les axes - le parallel-coordinates canonique.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-basic.html"></iframe>
</div>
<div class="sp-variant" id="parallel-fr-smooth">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"smooth"</code></span><span><strong>Aliases</strong> <code>smooth / curved / bezier / spline</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Lignes Bezier qui reduisent l encombrement visuel sur grands jeux.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-smooth.html"></iframe>
</div>
<div class="sp-variant" id="parallel-fr-categorical">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"categorical"</code></span><span><strong>Aliases</strong> <code>categorical / category / groups / colored</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Une couleur par categorie - parfait pour comparer des classes.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-categorical.html"></iframe>
</div>
<div class="sp-variant" id="parallel-fr-highlight">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"highlight"</code></span><span><strong>Aliases</strong> <code>highlight / spotlight / focus / dim</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Met en avant une serie et estompe les autres - ideal pour storytelling.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-highlight.html"></iframe>
</div>
<div class="sp-variant" id="parallel-fr-density">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"density"</code></span><span><strong>Aliases</strong> <code>density / fade / translucent / alpha</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Lignes translucides revelent les bandes de densite sur des milliers de profils.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-density.html"></iframe>
</div>

<div class="sp-variant" id="parallel-fr-arc">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"arc"</code></span><span><strong>Alias</strong> <code>arc / bezier_color / smooth_color / colored_bezier</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Courbes de Bezier cubiques avec degrade de couleur par serie. Reduit l encombrement visuel par rapport aux lignes droites tout en preservant l identite de chaque serie.</p>

<div class="sp-preview-label">Apercu</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-arc.html"></iframe>
</div>

<div class="sp-variant" id="parallel-fr-ribbon">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"ribbon"</code></span><span><strong>Alias</strong> <code>ribbon / flow / band / filled_bezier</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bandes de Bezier remplies entre les axes adjacents. Chaque serie est rendue comme un ruban translucide + trait fin, creant un effet de flux style Sankey simplifie.</p>

<div class="sp-preview-label">Apercu</div>
<iframe class="sp-preview-frame" src="../../previews/parallel-ribbon.html"></iframe>
</div>
</div>
</div>

</div>
