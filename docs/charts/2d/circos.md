# Circos Plot

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

`sp.circos(title, labels, categories, axes, matrix, series, series_names, edges_i, edges_j, *, variant="basic", **kwargs) -> Chart`

Aliases: `sp.circos`, `sp.circos_plot`, `sp.multitrack_circle`, `sp.genome_browser`, `sp.circular_tracks`

## Description

A multi-track circular plot in the spirit of Circos genome browsers: items sit on a shared ring, grouped into colored cluster arcs, and every other dataset dimension becomes its own concentric track — bar tracks, a sequential heatmap, a normalized composition ring, single-value tracks (ratio, rank, degree) on their own color ramp, and a network of co-occurrence chords through the center. Every ring is independently hoverable.

## Variants

<div data-sp-registry-table="variants" data-family="circos"></div>

## Data

`labels` (`list[str]`) — Item names around the ring. `categories` (`list[str]`) — Cluster assignment per item, drawn as colored boundary arcs. `axes` (`list[str]`) — Column names for the heatmap/composition matrix (e.g. age groups). `matrix` (`list[list[float]]`) — One row per item, one value per `axes` column; also drives the derived average/spread/composition tracks. `series` (`list[list[float]]`) — One or more bar tracks, one value per item. `series_names` (`list[str]`) — Name per bar track. `edges_i` / `edges_j` (`list[int]`) — Source/target item indices for the center co-occurrence chords. `width` / `height` (`int`) — Chart dimensions.

## Parameters

<div data-sp-registry-table="options" data-family="circos"></div>

## Themes

<div data-sp-registry-table="themes" data-family="circos"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="circos"></div>
</div>

<div class="sp-cls sp-open" id="circos-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('circos-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('circos-en','basic',this)"><span class="sp-cic">✦</span><span class="sp-clb">Basic</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="circos-en-basic">
<p>Full multi-track layout: item labels, co-occurrence degree, average value + spread, cluster boundaries, bar tracks, a ratio and a rank track, a sequential heatmap, a normalized composition ring, and center co-occurrence chords.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / multitrack / genome_browser / circular_tracks</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/circos-basic.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.circos(title, labels, categories, axes, matrix, series, series_names, edges_i, edges_j, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.circos`, `sp.circos_plot`, `sp.multitrack_circle`, `sp.genome_browser`, `sp.circular_tracks`

## Description

Un graphique circulaire multi-pistes dans l'esprit des navigateurs génomiques Circos : les items sont placés sur un anneau partagé, groupés en arcs de cluster colorés, et chaque autre dimension du jeu de données devient sa propre piste concentrique — pistes en barres, heatmap séquentielle, anneau de composition normalisée, pistes à valeur unique (ratio, rang, degré) sur leur propre rampe de couleur, et un réseau d'accords de co-occurrence au centre. Chaque anneau est survolable indépendamment.

## Variantes

<div data-sp-registry-table="variants" data-family="circos"></div>

## Données

`labels` (`list[str]`) — Noms des items autour de l'anneau. `categories` (`list[str]`) — Cluster assigné à chaque item, tracé en arcs de frontière colorés. `axes` (`list[str]`) — Noms des colonnes de la matrice heatmap/composition (ex. tranches d'âge). `matrix` (`list[list[float]]`) — Une ligne par item, une valeur par colonne de `axes` ; alimente aussi les pistes dérivées (moyenne, dispersion, composition). `series` (`list[list[float]]`) — Une ou plusieurs pistes en barres, une valeur par item. `series_names` (`list[str]`) — Nom de chaque piste en barres. `edges_i` / `edges_j` (`list[int]`) — Indices des items source/cible pour les accords de co-occurrence au centre. `width` / `height` (`int`) — Dimensions du graphique.

## Paramètres

<div data-sp-registry-table="options" data-family="circos"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="circos"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="circos"></div>
</div>

<div class="sp-cls sp-open" id="circos-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('circos-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('circos-fr','basic',this)"><span class="sp-cic">✦</span><span class="sp-clb">Basique</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="circos-fr-basic">
<p>Disposition multi-pistes complète : noms des items, degré de co-occurrence, valeur moyenne + dispersion, frontières de cluster, pistes en barres, une piste de ratio et une de rang, une heatmap séquentielle, un anneau de composition normalisée, et des accords de co-occurrence au centre.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic / multitrack / genome_browser / circular_tracks</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/circos-basic.html"></iframe>
</div>
</div>
</div>

</div>
