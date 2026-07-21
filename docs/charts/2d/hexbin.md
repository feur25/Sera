# Hexbin — Hexagonal Density Binning

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

`sp.hexbin(title, x_values, y_values, *, variant="basic", gridsize=20, colorscale=None, **kwargs) -> Chart`

Aliases: `sp.hexbin`, `sp.hexbins`, `sp.hexbin_chart`, `sp.hexagonal_binning`, `sp.build_hexbin`

## Description

`sp.hexbin()` bins a 2D scatter cloud into a regular hexagonal grid and colors each hexagon by point density (count), the standard alternative to a scatter plot once point overlap makes individual markers unreadable. Points are assigned to hexagon cells directly in pixel space using the true nearest-center rule (two candidate offset grids, closest wins), so cells tile without gaps or overlap regardless of the data's aspect ratio. Cell color reuses the same continuous colorscale engine as [`heatmap()`](heatmap.md) and `bubble(variant="gradient")` — any of `viridis` / `plasma` / `inferno` / `magma` / `cividis` / `turbo` / `rdbu` / `blues` / `reds` / `greens` works via `colorscale=`.

## Variants

<div data-sp-registry-table="variants" data-family="hexbin"></div>

Unknown variant strings fall back to the registered default. Variant keys may be prefixed with `en_`, `fr_`, `en-` or `fr-`.

## Data

`x_values` (`list[float]`) — X coordinates. `y_values` (`list[float]`) — Y coordinates.

## Parameters

<div data-sp-registry-table="options" data-family="hexbin"></div>

## Themes

<div data-sp-registry-table="themes" data-family="hexbin"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="hexbin"></div>
</div>

<div class="sp-cls sp-open" id="hexbin-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('hexbin-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('hexbin-en','basic',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-en','outlined',this)"><span class="sp-cic">⬢</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-en','spaced',this)"><span class="sp-cic">⬣</span><span class="sp-clb">Spaced</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-en','highlight',this)"><span class="sp-cic">⬣</span><span class="sp-clb">Highlight</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="hexbin-en-basic">
<p>Filled hexagons only, compact grid, right-side density legend.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / filled</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hexbin-basic.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-outlined">
<p>White cell borders; count printed inside each hexagon once cells are large enough to fit text.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / labeled</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hexbin-outlined.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-spaced">
<p>Hexagons drawn at 72% size with a visible gap between neighbors — a "confetti" look instead of a solid tiled surface.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"spaced"</code></span><span><strong>Aliases</strong> <code>spaced / gapped / confetti / dotted</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hexbin-spaced.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-highlight">
<p>Dims every cell except the densest ~15% (full opacity, white outline, count label) — draws the eye straight to the hotspots instead of the full density gradient.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"highlight"</code></span><span><strong>Aliases</strong> <code>highlight / top / hotspot / peak</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hexbin-highlight.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.hexbin(title, x_values, y_values, *, variant="basic", gridsize=20, colorscale=None, **kwargs) -> Chart`

Alias : `sp.hexbin`, `sp.hexbins`, `sp.hexbin_chart`, `sp.hexagonal_binning`, `sp.build_hexbin`

## Description

`sp.hexbin()` regroupe un nuage de points 2D dans une grille hexagonale régulière et colore chaque hexagone selon la densité de points (comptage) — l'alternative standard au nuage de points classique dès que le chevauchement des marqueurs le rend illisible. Les points sont assignés directement en espace pixel via la règle du centre le plus proche (deux grilles candidates décalées, la plus proche l'emporte), donc les cellules pavent sans trou ni recouvrement quel que soit le ratio d'aspect des données. La couleur des cellules réutilise le même moteur de dégradés continus que [`heatmap()`](heatmap.md) et `bubble(variant="gradient")` — `viridis` / `plasma` / `inferno` / `magma` / `cividis` / `turbo` / `rdbu` / `blues` / `reds` / `greens` fonctionnent via `colorscale=`.

## Variantes

<div data-sp-registry-table="variants" data-family="hexbin"></div>

Une variante inconnue retombe sur la valeur par défaut enregistrée. Les clés de variantes peuvent être préfixées par `en_`, `fr_`, `en-` ou `fr-`.

## Données

`x_values` (`list[float]`) — Coordonnées X. `y_values` (`list[float]`) — Coordonnées Y.

## Paramètres

<div data-sp-registry-table="options" data-family="hexbin"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="hexbin"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="hexbin"></div>
</div>

<div class="sp-cls sp-open" id="hexbin-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('hexbin-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('hexbin-fr','basic',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','outlined',this)"><span class="sp-cic">⬢</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','spaced',this)"><span class="sp-cic">⬣</span><span class="sp-clb">Espacé</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','highlight',this)"><span class="sp-cic">⬣</span><span class="sp-clb">Highlight</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="hexbin-fr-basic">
<p>Hexagones pleins uniquement, grille compacte, légende de densité à droite.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic / filled</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hexbin-basic.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-outlined">
<p>Contours blancs ; le comptage est affiché dans chaque hexagone assez grand pour contenir le texte.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"outlined"</code></span><span><strong>Alias</strong> <code>outlined / outline / stroke / labeled</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hexbin-outlined.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-spaced">
<p>Hexagones dessinés à 72% de leur taille avec un espace visible entre voisins — un rendu confetti plutôt qu'une surface pavée pleine.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"spaced"</code></span><span><strong>Alias</strong> <code>spaced / gapped / confetti / dotted</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hexbin-spaced.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-highlight">
<p>Estompe toutes les cellules sauf les ~15% les plus denses (pleine opacité, contour blanc, effectif affiché) — attire l'œil directement sur les zones chaudes plutôt que sur le dégradé complet.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"highlight"</code></span><span><strong>Alias</strong> <code>highlight / top / hotspot / peak</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hexbin-highlight.html"></iframe>
</div>
</div>
</div>

</div>
