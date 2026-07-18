# Parallel Categories — Categorical Flow Diagram

<div class="lang-en">

<style>
.sp-panel-source{display:none!important}
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.parcats(title, axes, category_series, *, variant="basic", **kwargs) -> Chart`

Aliases: `sp.parcats`, `sp.parallel_categories`, `sp.parcats_chart`, `sp.parallel_categories_chart`, `sp.build_parcats`

## Description

`sp.parcats()` is the categorical counterpart to [`parallel()`](parallel.md): instead of numeric axes with one polyline per row, each axis holds a set of discrete category values, and ribbons flow between adjacent axes with width proportional to how many rows share that pair of categories — the standard chart for tracing how categorical attributes co-occur across a dataset (e.g. gender → survival → class). Internally it builds a node for every distinct `(axis, category)` pair and an edge for every consecutive-axis transition, then reuses the exact same layered layout engine and bezier ribbon renderer as [`sankey()`](sankey.md) (`sankey::common::compute_layout` / `sankey_link_path`) — the two charts share their positioning math, not just their visual family.

> **Data shape** — `category_series` is a list of rows, each row a list of category values with one entry per axis (`category_series[row][axis]`), mirroring exactly the row-major shape already used by `parallel(series=...)`.

## Variants

<div data-sp-registry-table="variants" data-family="parcats"></div>

Unknown variant strings fall back to the registered default. Variant keys may be prefixed with `en_`, `fr_`, `en-` or `fr-`.

## Data

`axes` (`list[str]`) — Axis names, left to right. `category_series` (`list[list[str]]`) — One row per observation, one category value per axis.


## Parameters

<div data-sp-registry-table="options" data-family="parcats"></div>

## Themes

<div data-sp-registry-table="themes" data-family="parcats"></div>


## Returns

`Chart` — object with `.html` property and `.show()` method.


<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="parcats"></div>
</div>

<div class="sp-cls sp-open" id="parcats-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('parcats-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('parcats-en','basic',this)"><span class="sp-cic">▤</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('parcats-en','highlight',this)"><span class="sp-cic">★</span><span class="sp-clb">Highlight</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="parcats-en-basic">
<p>Flat semi-transparent ribbons colored by their source node.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / flat</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parcats-basic.html"></iframe>
</div>
<div class="sp-variant" id="parcats-en-highlight">
<p>Dims every ribbon except each node's single heaviest outgoing flow, which is boosted to full opacity with a thin white outline — traces the dominant path through the categories.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"highlight"</code></span><span><strong>Aliases</strong> <code>highlight / dominant / spotlight / focus_flow</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/parcats-highlight.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.parcats(title, axes, category_series, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.parcats`, `sp.parallel_categories`, `sp.parcats_chart`, `sp.parallel_categories_chart`, `sp.build_parcats`

## Description

`sp.parcats()` est l'équivalent catégoriel de [`parallel()`](parallel.md) : au lieu d'axes numériques avec une polyligne par ligne, chaque axe contient un ensemble de valeurs catégorielles discrètes, et des rubans relient les axes adjacents avec une largeur proportionnelle au nombre de lignes partageant cette paire de catégories — le graphique de référence pour tracer comment des attributs catégoriels co-occurrent dans un jeu de données (ex : genre → survie → classe). En interne, un nœud est créé pour chaque paire distincte `(axe, catégorie)` et une arête pour chaque transition entre axes consécutifs, puis le moteur de layout en couches et le rendu de rubans en bézier de [`sankey()`](sankey.md) sont réutilisés tels quels (`sankey::common::compute_layout` / `sankey_link_path`) — les deux graphiques partagent leurs mathématiques de positionnement, pas seulement leur famille visuelle.

> **Forme des données** — `category_series` est une liste de lignes, chaque ligne une liste de valeurs catégorielles avec une entrée par axe (`category_series[ligne][axe]`), reprenant exactement la forme ligne-par-ligne déjà utilisée par `parallel(series=...)`.

## Variantes

<div data-sp-registry-table="variants" data-family="parcats"></div>

Une variante inconnue retombe sur la valeur par défaut enregistrée. Les clés de variantes peuvent être préfixées par `en_`, `fr_`, `en-` ou `fr-`.

## Données

`axes` (`list[str]`) — Noms des axes, de gauche à droite. `category_series` (`list[list[str]]`) — Une ligne par observation, une valeur catégorielle par axe.


## Paramètres

<div data-sp-registry-table="options" data-family="parcats"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="parcats"></div>


## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.


<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="parcats"></div>
</div>

<div class="sp-cls sp-open" id="parcats-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('parcats-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('parcats-fr','basic',this)"><span class="sp-cic">▤</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('parcats-fr','highlight',this)"><span class="sp-cic">★</span><span class="sp-clb">Highlight</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="parcats-fr-basic">
<p>Rubans plats semi-transparents colorés selon leur nœud source.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic / flat</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/parcats-basic.html"></iframe>
</div>
<div class="sp-variant" id="parcats-fr-highlight">
<p>Estompe tous les rubans sauf le flux sortant le plus lourd de chaque nœud, poussé à pleine opacité avec un fin contour blanc — trace le chemin dominant à travers les catégories.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"highlight"</code></span><span><strong>Alias</strong> <code>highlight / dominant / spotlight / focus_flow</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/parcats-highlight.html"></iframe>
</div>
</div>
</div>

</div>
