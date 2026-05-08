# Radar — Spider / Star Chart

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
.sp-preview-frame{width:100%;height:560px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.radar(title, axes, series, *, series_names=None, variant="basic", filled=True, fill_opacity=50, palette=None, **kwargs) -> Chart`

Aliases: `sp.radar`, `sp.build_radar_chart`

## Description

`sp.radar()` is the unified entry point for the entire radar / spider / star chart family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. Radar charts are ideal for multivariate comparison across 3+ axes — performance profiles, KPIs, skill maps, scoring systems. SeraPlot draws everything in pure Rust SVG with concentric grid rings, axis lines, automatic ring tick labels, optional legend and per-series palette colors. The polar-bar variant turns the chart into a categorical polar histogram, the stacked variant builds a cumulative composition view.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Filled polygons + dots — standard radar | `palette`, `fill_opacity` |
| `"lines"` | Stroke-only outlines for many series | `palette` |
| `"filled"` | Strong fills, sorted back-to-front | `palette`, `fill_opacity` |
| `"markers"` | Light line + outlined dots | `palette` |
| `"dashed"` | Dashed outline polygons | `palette` |
| `"stacked"` | Cumulative stacked polygons | `palette`, `fill_opacity` |
| `"polar_bar"` | Radial bars per axis (grouped) | `palette`, `fill_opacity` |
| `"gradient"` | Radial center→edge gradient | `palette` |
| `"deluxe"` / `"premium"` / `"neon"` | Dark background, neon radial gradient fills, glow effect | `axes`, `series` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | required | all | Chart title |
| `axes` | `list[str]` | required | all | Axis labels (≥3) |
| `series` | `list[list[float]]` | required | all | One row per series, length must match `axes` |
| `series_names` | `list[str]` | `None` | all | Legend labels (one per series) |
| `variant` | `str` | `"basic"` | — | Rendering variant (see table) |
| `filled` | `bool` | `True` | basic | Fill polygons (basic only) |
| `fill_opacity` | `int` | `50` | filled variants | Fill alpha 0..255 |
| `palette` | `list[int]` | `None` | all | Custom per-series palette |
| `width` | `int` | `700` | all | Canvas width (px) |
| `height` | `int` | `560` | all | Canvas height (px) |
| `background` | `str` | `None` | all | Background CSS color |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="radar-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('radar-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('radar-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','lines',this)"><span class="sp-cic">L</span><span class="sp-clb">Lines</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','filled',this)"><span class="sp-cic">F</span><span class="sp-clb">Filled</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','markers',this)"><span class="sp-cic">M</span><span class="sp-clb">Markers</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','dashed',this)"><span class="sp-cic">D</span><span class="sp-clb">Dashed</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','polar_bar',this)"><span class="sp-cic">P</span><span class="sp-clb">PolarBar</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','gradient',this)"><span class="sp-cic">G</span><span class="sp-clb">Gradient</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="radar-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Filled polygon per series with stroke and dot markers — the standard radar.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-basic.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-lines">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"lines"</code></span><span><strong>Aliases</strong> <code>lines / outline / stroke / no_fill</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stroke-only polygons, no fill — clean overlay for many series.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-lines.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-filled">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"filled"</code></span><span><strong>Aliases</strong> <code>filled / fill / solid / area</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Strong fill, no stroke, sorted back-to-front by total area for clarity.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-filled.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-markers">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"markers"</code></span><span><strong>Aliases</strong> <code>markers / dots / points / marker</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Light stroke + bold outlined markers — emphasis on data points.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-markers.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-dashed">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dashed"</code></span><span><strong>Aliases</strong> <code>dashed / dash / dotted</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Dashed outline polygons — useful for projections, targets, baselines.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-dashed.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-stacked">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stack / cumulative</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Cumulative stacking on each axis — visualizes part-of-whole composition.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-stacked.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-polar_bar">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"polar_bar"</code></span><span><strong>Aliases</strong> <code>polar_bar / polar / bar / radial_bar</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Radial bars per axis grouped by series — categorical polar histogram.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-polar_bar.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / radial_gradient / shade / fade</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Radial gradient fill from center (opaque) to edge (transparent).</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-gradient.html"></iframe>
</div>
</div></div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.radar(title, axes, series, *, series_names=None, variant="basic", filled=True, fill_opacity=50, palette=None, **kwargs) -> Chart`

Alias : `sp.radar`, `sp.build_radar_chart`

<h2>Description</h2>

`sp.radar()` est le point d'entrée unifié pour toute la famille radar / spider / star. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments conservent le même nom d'une variante à l'autre. Le radar est idéal pour comparer plusieurs séries sur 3 axes ou plus — profils de performance, KPI, cartographie de compétences, systèmes de notation. SeraPlot dessine tout en SVG Rust natif avec anneaux de grille concentriques, axes, labels automatiques de graduation, légende optionnelle et couleurs de palette par série. La variante polar_bar transforme le radar en histogramme polaire catégoriel, la variante stacked construit une vue de composition cumulative.

| Variante | Usage | Args clés |
|----------|-------|-----------|
| `"basic"` | Polygones remplis + points — radar standard | `palette`, `fill_opacity` |
| `"lines"` | Contours seuls pour multi-séries | `palette` |
| `"filled"` | Remplissages forts, triés arrière→avant | `palette`, `fill_opacity` |
| `"markers"` | Trait léger + points détourés | `palette` |
| `"dashed"` | Polygones à contour pointillé | `palette` |
| `"stacked"` | Polygones empilés cumulés | `palette`, `fill_opacity` |
| `"polar_bar"` | Barres radiales par axe (groupées) | `palette`, `fill_opacity` |
| `"gradient"` | Dégradé radial centre→bord | `palette` |

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `axes` | `list[str]` | requis | toutes | Libellés d'axes (≥3) |
| `series` | `list[list[float]]` | requis | toutes | Une ligne par série, longueur = `axes` |
| `series_names` | `list[str]` | `None` | toutes | Libellés de légende |
| `variant` | `str` | `"basic"` | — | Variante de rendu |
| `filled` | `bool` | `True` | basic | Remplir les polygones (basic seul) |
| `fill_opacity` | `int` | `50` | variantes remplies | Alpha 0..255 |
| `palette` | `list[int]` | `None` | toutes | Palette personnalisée |
| `width` | `int` | `700` | toutes | Largeur (px) |
| `height` | `int` | `560` | toutes | Hauteur (px) |
| `background` | `str` | `None` | toutes | Couleur de fond CSS |

---

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

<div class="sp-cls sp-open" id="radar-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('radar-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('radar-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','lines',this)"><span class="sp-cic">L</span><span class="sp-clb">Lines</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','filled',this)"><span class="sp-cic">F</span><span class="sp-clb">Filled</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','markers',this)"><span class="sp-cic">M</span><span class="sp-clb">Markers</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','dashed',this)"><span class="sp-cic">D</span><span class="sp-clb">Dashed</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','polar_bar',this)"><span class="sp-cic">P</span><span class="sp-clb">PolarBar</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','gradient',this)"><span class="sp-cic">G</span><span class="sp-clb">Gradient</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="radar-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Polygone rempli par série avec contour et points — le radar standard.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-basic.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-lines">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"lines"</code></span><span><strong>Alias</strong> <code>lines / outline / stroke / no_fill</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Polygones en trait seul, sans remplissage — overlay net pour plusieurs séries.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-lines.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-filled">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"filled"</code></span><span><strong>Alias</strong> <code>filled / fill / solid / area</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Remplissage fort sans contour, trié de l'arrière vers l'avant par aire totale.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-filled.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-markers">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"markers"</code></span><span><strong>Alias</strong> <code>markers / dots / points / marker</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trait léger + marqueurs détourés — accent sur les points de données.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-markers.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-dashed">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"dashed"</code></span><span><strong>Alias</strong> <code>dashed / dash / dotted</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Polygones à contour pointillé — utile pour projections, cibles, références.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-dashed.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-stacked">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"stacked"</code></span><span><strong>Alias</strong> <code>stacked / stack / cumulative</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Empilement cumulatif sur chaque axe — visualise une composition part/tout.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-stacked.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-polar_bar">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"polar_bar"</code></span><span><strong>Alias</strong> <code>polar_bar / polar / bar / radial_bar</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres radiales par axe groupées par série — histogramme polaire catégoriel.</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-polar_bar.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-gradient">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code></span><span><strong>Alias</strong> <code>gradient / radial_gradient / shade / fade</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Remplissage en dégradé radial du centre (opaque) vers le bord (transparent).</p>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-gradient.html"></iframe>
</div>
</div></div>

</div>

