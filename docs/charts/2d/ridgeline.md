# Ridgeline — Joyplot / Stacked KDE

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

`sp.ridgeline(title, categories, values, *, variant="basic", overlap=0.5, bandwidth=0.0, n_points=60, fill_opacity=56, palette=None, **kwargs) -> Chart`

Aliases: `sp.ridgeline`, `sp.build_ridgeline_chart`

## Description

`sp.ridgeline()` is the unified entry point for the entire ridgeline family — also known as joyplot. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. A ridgeline plot stacks one KDE curve per category along a shared X axis with a controllable vertical overlap, making it ideal to compare distributions across many groups (years, regions, segments…). SeraPlot renders everything in pure Rust SVG, with quartile/mean overlays, rug ticks, gradient fills and a built-in viridis colormap.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Stacked filled ridges with white underlay | `palette`, `overlap` |
| `"gradient"` | Vertical gradient fill per ridge | `palette` |
| `"lines"` | Stroke-only outline ridges | `palette` |
| `"quartiles"` | Q1 / median / Q3 vertical markers | `palette` |
| `"mean"` | Mean dot + dashed line on each ridge | `palette` |
| `"rug"` | Rug ticks at sample positions | `palette` |
| `"heatmap"` | Auto viridis palette across ridges | — |
| `"spaced"` | Forced low overlap, separated ridges | `palette` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | required | all | Chart title |
| `categories` | `list[str]` | required | all | Per-value group label |
| `values` | `list[float]` | required | all | Numeric samples |
| `variant` | `str` | `"basic"` | — | Rendering variant (see table) |
| `overlap` | `float` | `0.5` | all (except spaced) | Vertical ridge overlap factor (0..2) |
| `bandwidth` | `float` | `0.0` | all | KDE bandwidth. 0 = Scott's rule (auto) |
| `n_points` | `int` | `60` | all | KDE evaluation points along X |
| `fill_opacity` | `int` | `56` | filled variants | Fill alpha 0..255 |
| `palette` | `list[int]` | `None` | all | Custom per-ridge palette |
| `width` | `int` | `900` | all | Canvas width (px) |
| `height` | `int` | `520` | all | Canvas height (px) |
| `x_label` | `str` | `""` | all | X-axis label |
| `y_label` | `str` | `""` | all | Y-axis label |
| `gridlines` | `bool` | `False` | all | Show vertical gridlines |
| `sort_order` | `str` | `"none"` | all | `none` / `asc` / `desc` (sort ridges by mean) |
| `background` | `str` | `None` | all | Background CSS color |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="ridgeline-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('ridgeline-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('ridgeline-en','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','gradient',this)"><span class="sp-cic">G</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','lines',this)"><span class="sp-cic">L</span><span class="sp-clb">Lines</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','quartiles',this)"><span class="sp-cic">Q</span><span class="sp-clb">Quartiles</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','mean',this)"><span class="sp-cic">M</span><span class="sp-clb">Mean</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','rug',this)"><span class="sp-cic">R</span><span class="sp-clb">Rug</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','heatmap',this)"><span class="sp-cic">H</span><span class="sp-clb">Heatmap</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-en','spaced',this)"><span class="sp-cic">S</span><span class="sp-clb">Spaced</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="ridgeline-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / filled / default / single / multi</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stacked filled ridges (one per category) with white underlay.</p>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-basic.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / shade / fade / vgrad</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Vertical gradient fill per ridge (opaque top → transparent bottom).</p>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-gradient.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-lines">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"lines"</code></span><span><strong>Aliases</strong> <code>lines / outline / stroke / no_fill</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stroke-only ridges, no fill — clean outline view.</p>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-lines.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-quartiles">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"quartiles"</code></span><span><strong>Aliases</strong> <code>quartiles / q / qrt / iqr</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Marks Q1, median (solid), and Q3 vertical lines on each ridge.</p>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-quartiles.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-mean">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mean"</code></span><span><strong>Aliases</strong> <code>mean / average / avg / mean_dot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Dashed line + dot at the mean of each distribution.</p>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-mean.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-rug">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rug"</code></span><span><strong>Aliases</strong> <code>rug / ticks / carpet / rugplot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Filled ridge with rug ticks below the baseline at sample positions.</p>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-rug.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-heatmap">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heatmap"</code></span><span><strong>Aliases</strong> <code>heatmap / heat / rainbow / colored</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Auto viridis palette across ridges (or custom palette).</p>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-heatmap.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-en-spaced">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"spaced"</code></span><span><strong>Aliases</strong> <code>spaced / separated / no_overlap / split</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Forced low overlap — ridges are separated for clarity.</p>
</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-spaced.html"></iframe>
</div>
</div></div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.ridgeline(title, categories, values, *, variant="basic", overlap=0.5, bandwidth=0.0, n_points=60, fill_opacity=56, palette=None, **kwargs) -> Chart`

Alias : `sp.ridgeline`, `sp.build_ridgeline_chart`

<h2>Description</h2>

`sp.ridgeline()` est le point d'entrée unifié pour toute la famille ridgeline — aussi appelé joyplot. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments conservent le même nom d'une variante à l'autre. Un ridgeline empile une courbe KDE par catégorie sur un axe X partagé avec un recouvrement vertical réglable, idéal pour comparer des distributions à travers plusieurs groupes (années, régions, segments…). SeraPlot rend tout en SVG Rust natif, avec marqueurs quartiles/moyenne, ticks rug, dégradés et palette viridis intégrée.

| Variante | Usage | Args clés |
|----------|-------|-----------|
| `"basic"` | Crêtes empilées remplies, fond blanc | `palette`, `overlap` |
| `"gradient"` | Dégradé vertical par crête | `palette` |
| `"lines"` | Crêtes en trait seul | `palette` |
| `"quartiles"` | Marqueurs verticaux Q1 / médiane / Q3 | `palette` |
| `"mean"` | Point + trait pointillé à la moyenne | `palette` |
| `"rug"` | Ticks rug aux positions des points | `palette` |
| `"heatmap"` | Palette viridis automatique | — |
| `"spaced"` | Recouvrement forcé bas, crêtes séparées | `palette` |

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `categories` | `list[str]` | requis | toutes | Étiquette de groupe par valeur |
| `values` | `list[float]` | requis | toutes | Échantillons numériques |
| `variant` | `str` | `"basic"` | — | Variante de rendu |
| `overlap` | `float` | `0.5` | toutes (sauf spaced) | Recouvrement vertical (0..2) |
| `bandwidth` | `float` | `0.0` | toutes | Bande passante KDE. 0 = règle de Scott |
| `n_points` | `int` | `60` | toutes | Points d'évaluation KDE |
| `fill_opacity` | `int` | `56` | variantes remplies | Alpha de remplissage 0..255 |
| `palette` | `list[int]` | `None` | toutes | Palette personnalisée |
| `width` | `int` | `900` | toutes | Largeur (px) |
| `height` | `int` | `520` | toutes | Hauteur (px) |
| `x_label` | `str` | `""` | toutes | Libellé axe X |
| `y_label` | `str` | `""` | toutes | Libellé axe Y |
| `gridlines` | `bool` | `False` | toutes | Afficher la grille verticale |
| `sort_order` | `str` | `"none"` | toutes | `none` / `asc` / `desc` (tri des crêtes par moyenne) |
| `background` | `str` | `None` | toutes | Couleur de fond CSS |

---

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

<div class="sp-cls sp-open" id="ridgeline-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('ridgeline-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('ridgeline-fr','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','gradient',this)"><span class="sp-cic">G</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','lines',this)"><span class="sp-cic">L</span><span class="sp-clb">Lines</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','quartiles',this)"><span class="sp-cic">Q</span><span class="sp-clb">Quartiles</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','mean',this)"><span class="sp-cic">M</span><span class="sp-clb">Mean</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','rug',this)"><span class="sp-cic">R</span><span class="sp-clb">Rug</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','heatmap',this)"><span class="sp-cic">H</span><span class="sp-clb">Heatmap</span></button>
<button class="sp-cls-tab" onclick="spCls('ridgeline-fr','spaced',this)"><span class="sp-cic">S</span><span class="sp-clb">Spaced</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="ridgeline-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / filled / default / single / multi</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Crêtes empilées remplies (une par catégorie) avec fond blanc.</p>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-basic.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-gradient">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code></span><span><strong>Alias</strong> <code>gradient / shade / fade / vgrad</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Remplissage en dégradé vertical par crête (opaque haut → transparent bas).</p>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-gradient.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-lines">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"lines"</code></span><span><strong>Alias</strong> <code>lines / outline / stroke / no_fill</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Crêtes en trait seul, sans remplissage — vue épurée.</p>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-lines.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-quartiles">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"quartiles"</code></span><span><strong>Alias</strong> <code>quartiles / q / qrt / iqr</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trace les verticales Q1, médiane (pleine) et Q3 sur chaque crête.</p>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-quartiles.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-mean">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"mean"</code></span><span><strong>Alias</strong> <code>mean / average / avg / mean_dot</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trait pointillé + point à la moyenne de chaque distribution.</p>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-mean.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-rug">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"rug"</code></span><span><strong>Alias</strong> <code>rug / ticks / carpet / rugplot</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Crête remplie avec ticks rug sous la ligne de base aux positions des points.</p>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-rug.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-heatmap">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"heatmap"</code></span><span><strong>Alias</strong> <code>heatmap / heat / rainbow / colored</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Palette viridis automatique sur les crêtes (ou palette personnalisée).</p>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-heatmap.html"></iframe>
</div>
<div class="sp-variant" id="ridgeline-fr-spaced">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"spaced"</code></span><span><strong>Alias</strong> <code>spaced / separated / no_overlap / split</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Recouvrement forcé bas — crêtes séparées pour la lisibilité.</p>
</div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/ridgeline-spaced.html"></iframe>
</div>
</div></div>

</div>

