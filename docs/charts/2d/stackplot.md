# Stackplot — Stacked Area / Streamgraph

<div class="lang-en">

<style>
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}
.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}
.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}
.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}
.sp-cls-tab .sp-clb{display:none}
.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}
.sp-variant.sp-von{display:block}
.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.stackplot(title, x_labels, series, *, variant="basic", series_names=None, **kwargs) -> Chart`

Aliases: `sp.stackplot`, `sp.stack_plot`, `sp.stacked_area`, `sp.build_stackplot`

## Description

`sp.stackplot()` draws multiple series as cumulatively stacked areas over a shared x-axis — reuses the exact same `x_labels`/`series`/`series_names` input shape as [`multiline()`](multiline.md), so any dataset already used with `multiline()` works unchanged. Negative values are clamped to 0 before stacking (a stack has no meaningful negative contribution).

## Variants

<div data-sp-registry-table="variants" data-family="stackplot"></div>

## Data

`x_labels` (`list[str]`) — Shared x-axis point labels. `series` (`list[list[float]]`) — One list of values per series, same length as `x_labels`. `series_names` (`list[str]`) — Legend label per series.

## Parameters

<div data-sp-registry-table="options" data-family="stackplot"></div>

## Themes

<div data-sp-registry-table="themes" data-family="stackplot"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="stackplot"></div>
</div>

<div class="sp-cls sp-open" id="stackplot-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('stackplot-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('stackplot-en','basic',this)"><span class="sp-cic">▮</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-en','streamgraph',this)"><span class="sp-cic">〜</span><span class="sp-clb">Streamgraph</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-en','normalized',this)"><span class="sp-cic">▤</span><span class="sp-clb">Normalized</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-en','radial',this)"><span class="sp-cic">◎</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-en','ribbon',this)"><span class="sp-cic">≈</span><span class="sp-clb">Ribbon</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="stackplot-en-basic">
<p>Traditional zero-baseline stacking, each series added on top of the previous one's cumulative total.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / stacked</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-basic.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-en-streamgraph">
<p>Centered ("silhouette") baseline — at every x-point the stack is centered around zero (`baseline = -total/2`) instead of starting at zero, giving the flowing ThemeRiver look.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"streamgraph"</code></span><span><strong>Aliases</strong> <code>streamgraph / stream / silhouette / themeriver</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-streamgraph.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-en-normalized">
<p>100%-stacked — every series is divided by the x-point's total before stacking, so the top of the stack is always 1.0. Shows share of total over time instead of absolute magnitude.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"normalized"</code></span><span><strong>Aliases</strong> <code>normalized / percent / hundred_percent / share</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-normalized.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-en-radial">
<p>Polar-wrapped stacking — each x-point becomes an angle around a circle instead of a position along an axis, and the cumulative bands grow outward as concentric rings from a small central hole. A genuinely different read on the same stacked data: total magnitude becomes an overall silhouette size, and each series' share becomes a colored band thickness at that angle.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"radial"</code></span><span><strong>Aliases</strong> <code>radial / polar / radar_stack / circular</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-radial.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-en-ribbon">
<p>Cartesian stacking rendered as smooth, glowing ribbons: quadratic-bezier-smoothed band edges, a top-to-bottom depth gradient per series, and a soft drop-shadow separating each layer — a more atmospheric, editorial look than the sharp-edged basic stack.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ribbon"</code></span><span><strong>Aliases</strong> <code>ribbon / glow / smooth / flow</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-ribbon.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.stackplot(title, x_labels, series, *, variant="basic", series_names=None, **kwargs) -> Chart`

Alias : `sp.stackplot`, `sp.stack_plot`, `sp.stacked_area`, `sp.build_stackplot`

## Description

`sp.stackplot()` trace plusieurs séries en aires empilées cumulativement sur un axe x partagé — réutilise exactement la même forme d'entrée `x_labels`/`series`/`series_names` que [`multiline()`](multiline.md), donc tout jeu de données déjà utilisé avec `multiline()` fonctionne sans modification. Les valeurs négatives sont ramenées à 0 avant l'empilement (un empilement n'a pas de contribution négative significative).

## Variantes

<div data-sp-registry-table="variants" data-family="stackplot"></div>

## Données

`x_labels` (`list[str]`) — Libellés des points sur l'axe x partagé. `series` (`list[list[float]]`) — Une liste de valeurs par série, même longueur que `x_labels`. `series_names` (`list[str]`) — Libellé de légende par série.

## Paramètres

<div data-sp-registry-table="options" data-family="stackplot"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="stackplot"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="stackplot"></div>
</div>

<div class="sp-cls sp-open" id="stackplot-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('stackplot-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('stackplot-fr','basic',this)"><span class="sp-cic">▮</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-fr','streamgraph',this)"><span class="sp-cic">〜</span><span class="sp-clb">Streamgraph</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-fr','normalized',this)"><span class="sp-cic">▤</span><span class="sp-clb">Normalized</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-fr','radial',this)"><span class="sp-cic">◎</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('stackplot-fr','ribbon',this)"><span class="sp-cic">≈</span><span class="sp-clb">Ribbon</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="stackplot-fr-basic">
<p>Empilement classique à ligne de base zéro, chaque série ajoutée au-dessus du total cumulé de la précédente.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic / stacked</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-basic.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-fr-streamgraph">
<p>Ligne de base centrée ("silhouette") — à chaque point x, l'empilement est centré autour de zéro (`baseline = -total/2`) au lieu de partir de zéro, donnant le rendu fluide façon ThemeRiver.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"streamgraph"</code></span><span><strong>Alias</strong> <code>streamgraph / stream / silhouette / themeriver</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-streamgraph.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-fr-normalized">
<p>Empilement à 100% — chaque série est divisée par le total du point x avant l'empilement, donc le sommet est toujours à 1.0. Montre la part du total dans le temps plutôt que la magnitude absolue.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"normalized"</code></span><span><strong>Alias</strong> <code>normalized / percent / hundred_percent / share</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-normalized.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-fr-radial">
<p>Empilement enroulé en polaire — chaque point x devient un angle autour d'un cercle au lieu d'une position sur un axe, et les bandes cumulées s'étendent vers l'extérieur en anneaux concentriques depuis un petit trou central. Une lecture vraiment différente des mêmes données empilées : la magnitude totale devient une silhouette globale, et la part de chaque série devient l'épaisseur d'une bande colorée à cet angle.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"radial"</code></span><span><strong>Alias</strong> <code>radial / polar / radar_stack / circular</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-radial.html"></iframe>
</div>
<div class="sp-variant" id="stackplot-fr-ribbon">
<p>Empilement cartésien rendu en rubans lisses et lumineux : bords de bandes lissés par courbes de Bézier quadratiques, dégradé de profondeur haut-bas par série, et une ombre portée douce séparant chaque couche — un rendu plus atmosphérique et éditorial que l'empilement basique aux arêtes nettes.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"ribbon"</code></span><span><strong>Alias</strong> <code>ribbon / glow / smooth / flow</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/stackplot-ribbon.html"></iframe>
</div>
</div>
</div>

</div>
