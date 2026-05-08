# Violin Plot

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
.sp-cls.sp-open .sp-cls-rail{min-width:180px;padding:18px 8px}
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
.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.violin(title, labels=None, values=None, *, variant="box", **kwargs) -> Chart`

Aliases: `sp.violin`, `sp.violins`, `sp.violin_chart`, `sp.violin_family`, `sp.violin_unified`

## Description

`sp.violin()` is the unified entry point for the entire violin-plot family. The `variant` keyword selects the rendering strategy — every other argument stays consistent across variants. The kernel-density estimation, quartiles and statistics are computed in pure Rust, no NumPy or pandas required.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Clean symmetric KDE silhouette, no inner stats | `labels`, `values` |
| `"box"` | KDE wrapping a dark IQR box and white median dot | `bandwidth` |
| `"quartile"` | KDE with three dashed quartile lines (Q1/median/Q3) | `bandwidth` |
| `"mean"` | KDE with mean dashed line and median dot | `bandwidth` |
| `"points"` | KDE silhouette with every individual sample jittered | `jitter` |
| `"strip"` | Pure jittered scatter with no KDE silhouette | `jitter` |
| `"horizontal"` | Rotated layout, ideal for many categories or long names | `bandwidth` |
| `"split"` | Pairs of categories rendered back-to-back on shared axis | `bandwidth` |
| `"half"` | Single-sided violin (right half only) with median + mean | `bandwidth` |
| `"rainbow"` | Spectral hue rotation across categories with inner box | `bandwidth` |
| `"deluxe"` / `"premium"` / `"neon"` | Dark neon gradient fills with glow | `labels`, `values` |
| `"aurora"` | Warm/cool gradient pair fills, increased overlap | `labels`, `values` |
| `"crystal"` | Translucent glass violin with horizontal stripe refraction | `labels`, `values` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | — | all | Chart title shown at the top |
| `labels` | `list[str]` | `None` | all | One category label per sample (flat list) |
| `values` | `list[float]` | `None` | all | Flat list of numeric samples aligned with `labels` |
| `variant` | `str` | `"box"` | all | Selects the violin variant |
| `bandwidth` | `float` | `1.0` | all | KDE bandwidth multiplier (Silverman rule scaled) |
| `fill_opacity` | `float` | `0.55` | all | Violin body fill opacity (0–1) |
| `stroke_width` | `float` | `1.4` | all | Violin outline stroke width in pixels |
| `jitter` | `float` | `0.35` | points, strip | Horizontal scatter spread (0–1) |
| `show_points` | `bool` | `False` | basic, box | Overlay individual samples on top |
| `show_box` | `bool` | `False` | basic, quartile | Force inner IQR box on non-box variants |
| `show_mean` | `bool` | `False` | basic, box, quartile | Force mean dashed line |
| `palette` | `list[int]` | `None` | all except rainbow | Per-category fill colors as hex integers |
| `width` | `int` | `900` | all | Canvas width in pixels |
| `height` | `int` | `500` | all | Canvas height in pixels |
| `x_label` | `str` | `""` | all | X-axis label |
| `y_label` | `str` | `""` | all | Y-axis label |
| `gridlines` | `bool` | `False` | all | Show gridlines on the value axis |
| `sort_order` | `str` | `"none"` | all | `"asc"`, `"desc"`, `"alpha"`, `"alpha_desc"`, or `"none"` |
| `legend_position` | `str` | `"right"` | all | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | all | Background CSS color; `None` = transparent |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="vl-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('vl-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab" onclick="spCls('vl-en','basic',this)"><span class="sp-cic">◇</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab sp-cact" onclick="spCls('vl-en','box',this)"><span class="sp-cic">▭</span><span class="sp-clb">Box</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','quartile',this)"><span class="sp-cic">≣</span><span class="sp-clb">Quartile</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','mean',this)"><span class="sp-cic">μ</span><span class="sp-clb">Mean</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','points',this)"><span class="sp-cic">⁝</span><span class="sp-clb">Points</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','strip',this)"><span class="sp-cic">⋮</span><span class="sp-clb">Strip</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','horizontal',this)"><span class="sp-cic">⇆</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','split',this)"><span class="sp-cic">◐</span><span class="sp-clb">Split</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','half',this)"><span class="sp-cic">◗</span><span class="sp-clb">Half</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','rainbow',this)"><span class="sp-cic">◑</span><span class="sp-clb">Rainbow</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','deluxe',this)"><span class="sp-cic">✦</span><span class="sp-clb">Deluxe</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','aurora',this)"><span class="sp-cic">☄</span><span class="sp-clb">Aurora</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-en','crystal',this)"><span class="sp-cic">◇</span><span class="sp-clb">Crystal</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant" id="vl-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-basic.html"></iframe>

</div>

<div class="sp-variant sp-von" id="vl-en-box">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"box"</code> (default)</span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-box.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-quartile">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"quartile"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-quartile.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-mean">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mean"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-mean.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-points">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"points"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-points.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-strip">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"strip"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-strip.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-horizontal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-horizontal.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-split">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"split"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-split.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-half">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"half"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-half.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-rainbow">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rainbow"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-rainbow.html"></iframe>

</div>

<div class="sp-variant" id="vl-en-deluxe">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"deluxe"</code></span><span><strong>Aliases</strong> <code>deluxe / premium / neon</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-deluxe.html"></iframe>
</div>

<div class="sp-variant" id="vl-en-aurora">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"aurora"</code></span><span><strong>Aliases</strong> <code>aurora / warm-cool / bicolor</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-aurora.html"></iframe>
</div>

<div class="sp-variant" id="vl-en-crystal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"crystal"</code></span><span><strong>Aliases</strong> <code>crystal / glass / prism / transparent</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/violin-crystal.html"></iframe>
</div>

</div>
</div>

</div><!-- /lang-en -->


<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.violin(title, labels=None, values=None, *, variant="box", **kwargs) -> Chart`

Alias : `sp.violin`, `sp.violins`, `sp.violin_chart`, `sp.violin_family`, `sp.violin_unified`

<h2>Description</h2>

`sp.violin()` est le point d'entrée unique pour toute la famille des violons. Le paramètre `variant` sélectionne la stratégie de rendu — tous les autres arguments restent identiques entre les variantes. L'estimation de densité par noyau (KDE), les quartiles et les statistiques sont calculés en pur Rust, sans NumPy ni pandas.

| Variante | Cas d'usage | Arguments clés |
|----------|-------------|----------------|
| `"basic"` | Silhouette KDE symétrique épurée, sans statistiques internes | `labels`, `values` |
| `"box"` | KDE enveloppant une boîte IQR sombre et un point médian blanc | `bandwidth` |
| `"quartile"` | KDE avec trois lignes pointillées (Q1 / médiane / Q3) | `bandwidth` |
| `"mean"` | KDE avec ligne moyenne pointillée et point médian | `bandwidth` |
| `"points"` | Silhouette KDE + tous les échantillons individuels jittered | `jitter` |
| `"strip"` | Nuage jittered pur, sans silhouette KDE | `jitter` |
| `"horizontal"` | Disposition pivotée, idéale pour longs noms ou nombreux groupes | `bandwidth` |
| `"split"` | Paires de catégories rendues dos à dos sur axe partagé | `bandwidth` |
| `"half"` | Violon mono-côté (moitié droite uniquement) avec médiane + moyenne | `bandwidth` |
| `"rainbow"` | Rotation spectrale des teintes entre catégories avec boîte interne | `bandwidth` |

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | — | toutes | Titre du graphique |
| `labels` | `list[str]` | `None` | toutes | Une étiquette de catégorie par échantillon (liste plate) |
| `values` | `list[float]` | `None` | toutes | Liste plate des valeurs alignées sur `labels` |
| `variant` | `str` | `"box"` | toutes | Sélectionne la variante |
| `bandwidth` | `float` | `1.0` | toutes | Multiplicateur de largeur de bande KDE (règle de Silverman) |
| `fill_opacity` | `float` | `0.55` | toutes | Opacité de remplissage du violon (0–1) |
| `stroke_width` | `float` | `1.4` | toutes | Épaisseur du contour en pixels |
| `jitter` | `float` | `0.35` | points, strip | Étalement horizontal du nuage (0–1) |
| `show_points` | `bool` | `False` | basic, box | Superpose les échantillons individuels |
| `show_box` | `bool` | `False` | basic, quartile | Force la boîte IQR interne sur les variantes sans boîte |
| `show_mean` | `bool` | `False` | basic, box, quartile | Force l'affichage de la ligne moyenne |
| `palette` | `list[int]` | `None` | toutes sauf rainbow | Couleurs par catégorie (entiers hex) |
| `width` | `int` | `900` | toutes | Largeur du canevas en px |
| `height` | `int` | `500` | toutes | Hauteur du canevas en px |
| `x_label` | `str` | `""` | toutes | Label axe X |
| `y_label` | `str` | `""` | toutes | Label axe Y |
| `gridlines` | `bool` | `False` | toutes | Lignes de grille sur l'axe des valeurs |
| `sort_order` | `str` | `"none"` | toutes | `"asc"`, `"desc"`, `"alpha"`, `"alpha_desc"`, `"none"` |
| `legend_position` | `str` | `"right"` | toutes | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | toutes | Couleur de fond CSS ; `None` = transparent |

---

<h2>Retourne</h2>

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

<div class="sp-cls sp-open" id="vl-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('vl-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','basic',this)"><span class="sp-cic">◇</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab sp-cact" onclick="spCls('vl-fr','box',this)"><span class="sp-cic">▭</span><span class="sp-clb">Boîte</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','quartile',this)"><span class="sp-cic">≣</span><span class="sp-clb">Quartile</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','mean',this)"><span class="sp-cic">μ</span><span class="sp-clb">Moyenne</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','points',this)"><span class="sp-cic">⁝</span><span class="sp-clb">Points</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','strip',this)"><span class="sp-cic">⋮</span><span class="sp-clb">Bande</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','horizontal',this)"><span class="sp-cic">⇆</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','split',this)"><span class="sp-cic">◐</span><span class="sp-clb">Scindé</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','half',this)"><span class="sp-cic">◗</span><span class="sp-clb">Demi</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','rainbow',this)"><span class="sp-cic">◑</span><span class="sp-clb">Arc-en-ciel</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','deluxe',this)"><span class="sp-cic">✦</span><span class="sp-clb">Deluxe</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','aurora',this)"><span class="sp-cic">☄</span><span class="sp-clb">Aurore</span></button>
<button class="sp-cls-tab" onclick="spCls('vl-fr','crystal',this)"><span class="sp-cic">◇</span><span class="sp-clb">Cristal</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant" id="vl-fr-basic"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-basic.html"></iframe></div>

<div class="sp-variant sp-von" id="vl-fr-box"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"box"</code> (par défaut)</span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-box.html"></iframe></div>

<div class="sp-variant" id="vl-fr-quartile"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"quartile"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-quartile.html"></iframe></div>

<div class="sp-variant" id="vl-fr-mean"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"mean"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-mean.html"></iframe></div>

<div class="sp-variant" id="vl-fr-points"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"points"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-points.html"></iframe></div>

<div class="sp-variant" id="vl-fr-strip"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"strip"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-strip.html"></iframe></div>

<div class="sp-variant" id="vl-fr-horizontal"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"horizontal"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-horizontal.html"></iframe></div>

<div class="sp-variant" id="vl-fr-split"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"split"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-split.html"></iframe></div>

<div class="sp-variant" id="vl-fr-half"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"half"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-half.html"></iframe></div>

<div class="sp-variant" id="vl-fr-rainbow"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"rainbow"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-rainbow.html"></iframe></div>

<div class="sp-variant" id="vl-fr-deluxe"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"deluxe"</code></span><span><strong>Alias</strong> <code>deluxe / premium / neon</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-deluxe.html"></iframe></div>

<div class="sp-variant" id="vl-fr-aurora"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"aurora"</code></span><span><strong>Alias</strong> <code>aurora / warm-cool / bicolor</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-aurora.html"></iframe></div>

<div class="sp-variant" id="vl-fr-crystal"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"crystal"</code></span><span><strong>Alias</strong> <code>crystal / glass / prism / transparent</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/violin-crystal.html"></iframe></div>

</div>
</div>

</div><!-- /lang-fr -->
