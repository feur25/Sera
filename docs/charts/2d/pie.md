# Pie Charts

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

.sp-variant > p:first-of-type{margin:0;padding:14px 18px 8px;background:linear-gradient(180deg,rgba(99,102,241,.08),rgba(99,102,241,.03));border:1px solid rgba(99,102,241,.18);border-bottom:none;border-radius:10px 10px 0 0;color:#e2e8f0;font-size:14px;line-height:1.55;font-weight:500}
.sp-variant > p:first-of-type + pre{margin:0 0 18px;padding:14px 18px 16px;background:linear-gradient(180deg,#0d1326,#080d1a);border:1px solid rgba(99,102,241,.18);border-top:none;border-radius:0 0 10px 10px;box-shadow:0 6px 18px -8px rgba(0,0,0,.6);overflow-x:auto}
.sp-variant > p:first-of-type + pre code{background:none;padding:0;font-size:12.5px;line-height:1.55;color:#cbd5e1}

.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
.sp-preview-frame{width:100%;height:380px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.pie(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

Aliases: `sp.pie`, `sp.pie_chart`, `sp.pie_unified`, `sp.pie_family`, `sp.pies`

## Description

`sp.pie()` is the unified entry point for the entire pie-chart family. The `variant` keyword selects the rendering strategy — all other arguments remain consistent across variants.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Standard pie | `labels`, `values` |
| `"donut"` / `"ring"` | Pie with a hole in the center | `inner_radius_ratio` |
| `"exploded"` / `"pulled"` | Slice(s) pulled out from the center for emphasis | `pull` |
| `"subplots"` / `"grid"` / `"facet"` | Multiple pies in a grid sharing labels | `series`, `subplot_titles`, `subplot_cols` |
| `"proportional"` / `"scalegroup"` | Subplots sized proportionally to their totals | `series`, `subplot_titles` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | — | all | Chart title shown at the top |
| `labels` | `list[str]` | `None` | all | Slice labels (shared across subplots) |
| `values` | `list[float]` | `None` | basic, donut, exploded | Single-series numeric values |
| `series` | `list[list[float]]` | `None` | subplots, proportional | One values list per pie in the grid |
| `subplot_titles` | `list[str]` | `None` | subplots, proportional | Caption shown below each pie |
| `subplot_cols` | `int` | `0` (auto) | subplots, proportional | Number of columns in the grid |
| `pull` | `list[float]` | `None` | exploded | Per-slice radial offset as a fraction of radius (0–0.6) |
| `inner_radius_ratio` | `float` | `0.0` | donut | Hole radius as a fraction of the outer radius |
| `show_pct` | `bool` | `True` | all | Render percentage labels inside slices |
| `min_label_frac` | `float` | `0.04` | all | Hide percentage label below this fraction |
| `palette` | `list[int]` | `None` | all | Custom slice colors as hex integers |
| `width` | `int` | `720` | all | Canvas width in pixels |
| `height` | `int` | `440` | all | Canvas height in pixels |
| `sort_order` | `str` | `"none"` | basic, donut, exploded | `"asc"`, `"desc"`, `"alpha"`, `"alpha_desc"`, or `"none"` |
| `legend_position` | `str` | `"right"` | all | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `background` | `str` | `None` | all | Background CSS color; `None` = transparent |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="pie-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('pie-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('pie-en','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','donut',this)"><span class="sp-cic">◍</span><span class="sp-clb">Donut</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','exploded',this)"><span class="sp-cic">◐</span><span class="sp-clb">Exploded</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','subplots',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Subplots</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','proportional',this)"><span class="sp-cic">◔</span><span class="sp-clb">Proportional</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','semi',this)"><span class="sp-cic">◗</span><span class="sp-clb">Semi</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','kpi',this)"><span class="sp-cic">◉</span><span class="sp-clb">KPI</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','nested',this)"><span class="sp-cic">◎</span><span class="sp-clb">Nested</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','pattern',this)"><span class="sp-cic">▦</span><span class="sp-clb">Pattern</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="pie-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code> / <code>"pie"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pie-basic.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-donut">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"donut"</code> / <code>"ring"</code> / <code>"hole"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pie-donut.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-exploded">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"exploded"</code> / <code>"pulled"</code> / <code>"explode"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pie-exploded.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-subplots">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"subplots"</code> / <code>"grid"</code> / <code>"facet"</code> / <code>"multi"</code></span><span><strong>Required</strong> <code>labels</code>, <code>series</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pie-subplots.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-proportional">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"proportional"</code> / <code>"scaled"</code> / <code>"scalegroup"</code></span><span><strong>Required</strong> <code>labels</code>, <code>series</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pie-proportional.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-semi">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"semi"</code> / <code>"half_pie"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pie-semi.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-kpi">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"kpi"</code> / <code>"indicator"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pie-kpi.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-nested">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"nested"</code> / <code>"concentric"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code>, <code>secondary_values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pie-nested.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-pattern">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"pattern"</code> / <code>"hatched"</code></span><span><strong>Patterns</strong> <code>"stripes"</code>, <code>"dots"</code>, <code>"diagonal"</code>, <code>"cross"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>


</div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pie-pattern.html"></iframe>
</div>

</div>
</div>

</div><!-- /lang-en -->


<div class="lang-fr" style="display:none">

## Signature

`sp.pie(title, labels=None, values=None, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.pie`, `sp.pie_chart`, `sp.donut`

## Description

`sp.pie()` est le point d'entrée unique pour toute la famille des camemberts. Le paramètre `variant` sélectionne la stratégie de rendu — donut, éclaté, KPI, semi-cercle ou imbriqué — tout en conservant la même API simple.

| Variante | Cas d'usage | Arguments clés |
|----------|-------------|----------------|
| `"basic"` | Camembert classique | `labels`, `values` |
| `"donut"` / `"ring"` | Anneau avec trou central | `labels`, `values`, `inner_radius` |
| `"exploded"` / `"explode"` | Tranches détachées de l'anneau | `labels`, `values`, `pull` |
| `"subplots"` / `"grid"` | Plusieurs camemberts côte à côte | `panels` |
| `"proportional"` / `"prop"` | Tailles proportionnelles entre panneaux | `panels`, `total_basis` |
| `"semi"` / `"half"` | Demi-camembert (jauge) | `labels`, `values` |
| `"kpi"` / `"indicator"` | Donut avec valeur au centre | `labels`, `values`, `center_text` |
| `"nested"` / `"sunburst"` | Anneaux concentriques hiérarchiques | `rings` |
| `"pattern"` / `"hatch"` | Tranches avec motifs (accessibilité) | `labels`, `values`, `patterns` |

---

## Paramètres

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `labels` | `list[str]` | `None` | toutes sauf subplots, nested | Labels de tranche |
| `values` | `list[float]` | `None` | toutes sauf subplots, nested | Valeurs (normalisées en %) |
| `variant` | `str` | `"basic"` | — | Variante de rendu |
| `inner_radius` | `float` | `0.0` | basic, donut, exploded, kpi | Rayon du trou (0–0.95) |
| `pull` | `list[float]` | `None` | exploded | Distance d'extraction par tranche |
| `panels` | `list[dict]` | `None` | subplots, proportional | Liste de panneaux `{title, labels, values}` |
| `total_basis` | `str` | `"max"` | proportional | `"max"`, `"sum"` ou valeur de référence |
| `center_text` | `str` | `""` | kpi | Texte central du KPI |
| `rings` | `list[dict]` | `None` | nested | Anneaux concentriques |
| `patterns` | `list[str]` | `None` | pattern | Motifs (`"dots"`, `"lines"`, `"cross"`, …) |
| `palette` | `list[int]` | `None` | toutes | Couleurs personnalisées |
| `width` | `int` | `700` | toutes | Largeur du canevas en px |
| `height` | `int` | `500` | toutes | Hauteur du canevas en px |
| `legend_position` | `str` | `"right"` | toutes | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `show_text` | `bool` | `True` | toutes | Afficher pourcentages sur les tranches |
| `background` | `str` | `None` | toutes | Couleur de fond CSS |

---

## Retourne

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

<div class="sp-cls sp-open" id="pie-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('pie-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('pie-fr','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','donut',this)"><span class="sp-cic">◯</span><span class="sp-clb">Donut</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','exploded',this)"><span class="sp-cic">✦</span><span class="sp-clb">Éclaté</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','subplots',this)"><span class="sp-cic">▦</span><span class="sp-clb">Sous-graphiques</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','proportional',this)"><span class="sp-cic">◐</span><span class="sp-clb">Proportionnel</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','semi',this)"><span class="sp-cic">◗</span><span class="sp-clb">Semi</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','kpi',this)"><span class="sp-cic">⊙</span><span class="sp-clb">KPI</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','nested',this)"><span class="sp-cic">◎</span><span class="sp-clb">Imbriqué</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','pattern',this)"><span class="sp-cic">▩</span><span class="sp-clb">Motifs</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="pie-fr-basic"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/pie-basic.html"></iframe></div>

<div class="sp-variant" id="pie-fr-donut"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"donut"</code> / <code>"ring"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code>, <code>inner_radius</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/pie-donut.html"></iframe></div>

<div class="sp-variant" id="pie-fr-exploded"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"exploded"</code> / <code>"explode"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code>, <code>pull</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/pie-exploded.html"></iframe></div>

<div class="sp-variant" id="pie-fr-subplots"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"subplots"</code> / <code>"grid"</code></span><span><strong>Requis</strong> <code>panels</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/pie-subplots.html"></iframe></div>

<div class="sp-variant" id="pie-fr-proportional"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"proportional"</code> / <code>"prop"</code></span><span><strong>Requis</strong> <code>panels</code>, <code>total_basis</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/pie-proportional.html"></iframe></div>

<div class="sp-variant" id="pie-fr-semi"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"semi"</code> / <code>"half"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/pie-semi.html"></iframe></div>

<div class="sp-variant" id="pie-fr-kpi"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"kpi"</code> / <code>"indicator"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code>, <code>center_text</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/pie-kpi.html"></iframe></div>

<div class="sp-variant" id="pie-fr-nested"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"nested"</code> / <code>"sunburst"</code></span><span><strong>Requis</strong> <code>rings</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/pie-nested.html"></iframe></div>

<div class="sp-variant" id="pie-fr-pattern"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"pattern"</code> / <code>"hatch"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code>, <code>patterns</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" src="../../previews/pie-pattern.html"></iframe></div>

</div>
</div>

</div><!-- /lang-fr -->
