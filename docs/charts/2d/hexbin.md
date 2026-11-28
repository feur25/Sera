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
<button class="sp-cls-tab" onclick="spCls('hexbin-en','mincnt',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Mincnt</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-en','nested',this)"><span class="sp-cic">◎</span><span class="sp-clb">Nested</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-en','log_counts',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Log counts</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-en','weighted',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Weighted</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-en','dotted',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Dotted</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-en','marginals',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Marginals</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-en','voronoi',this)"><span class="sp-cic">◈</span><span class="sp-clb">Voronoi</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-en','neural',this)"><span class="sp-cic">✦</span><span class="sp-clb">Neural</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-en','bloom',this)"><span class="sp-cic">❀</span><span class="sp-clb">Bloom</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="hexbin-en-basic">
<p>Filled hexagons only, compact grid, right-side density legend.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / filled</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-basic.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-outlined">
<p>White cell borders; count printed inside each hexagon once cells are large enough to fit text.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / labeled</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-outlined.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-spaced">
<p>Hexagons drawn at 72% size with a visible gap between neighbors — a "confetti" look instead of a solid tiled surface.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"spaced"</code></span><span><strong>Aliases</strong> <code>spaced / gapped / confetti</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-spaced.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-highlight">
<p>Dims every cell except the densest ~15% (full opacity, white outline, count label) — draws the eye straight to the hotspots instead of the full density gradient.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"highlight"</code></span><span><strong>Aliases</strong> <code>highlight / top / hotspot / peak</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-highlight.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-mincnt">
<p>Bins below <code>min_count</code> are skipped entirely (left transparent) instead of drawn faint - a hard threshold rather than a dimmed gradient, matching R's <code>hexbin(mincnt=)</code>.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mincnt"</code></span><span><strong>Aliases</strong> <code>mincnt / threshold / sparse</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-mincnt.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-nested">
<p>Each cell's count is classed into an order-of-magnitude band (ones/tens/hundreds/thousands/10 thousands), colored and sized by band, with a smaller nested hexagon inside in the previous band's color - matching R hexbin's nested/centroid styles - plus a size+color legend.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"nested"</code></span><span><strong>Aliases</strong> <code>nested / magnitude / rings / centroids</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-nested.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-log_counts">
<p>Cell color is mapped on <code>log(count + 1)</code> instead of the raw count, matching matplotlib's <code>hexbin(bins="log")</code> — compresses the huge dynamic range that skewed point clouds produce so low-density cells stay visually distinguishable instead of collapsing near zero.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"log_counts"</code></span><span><strong>Aliases</strong> <code>log_counts / log / log_scale / logarithmic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-log_counts.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-weighted">
<p>Cell color encodes the average of a third variable (<code>values=</code>) inside each bin instead of the point count — the native equivalent of matplotlib's <code>hexbin(C=..., reduce_C_function=numpy.mean)</code>, for when the quantity of interest isn't density itself.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"weighted"</code></span><span><strong>Aliases</strong> <code>weighted / mean / aggregate / reduce_mean</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-weighted.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-dotted">
<p>White dashed cell borders over a full continuous colorscale (defaults to <code>magma</code>) with no plot border — matches matplotlib's <code>hexbin(edgecolor="white", linestyle="dotted", linewidth=1.5)</code> styling exactly.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dotted"</code></span><span><strong>Aliases</strong> <code>dotted / dashed / styled / magma</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-dotted.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-marginals">
<p>Adds 1D density strips above and to the right of the hexbin grid — a joint-plot style combination, showing the marginal distribution of each axis alongside the 2D density.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"marginals"</code></span><span><strong>Aliases</strong> <code>marginals / joint / with_histograms / density_marginals</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-marginals.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-voronoi">
<p>Tessellates the scattered points into a bounded Voronoi diagram instead of a regular hex grid — each cell's fill encodes local density (<code>values=</code> if supplied, otherwise the inverse of the cell's own area), so density is read from irregular organic cell sizes rather than a uniform lattice. Site points are drawn as small dots on top, echoing the "particle tracking" framing of density-Voronoi science posters.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"voronoi"</code></span><span><strong>Aliases</strong> <code>voronoi / density_voronoi / tessellation / particle_density / cells</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-voronoi.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-neural">
<p>Draws the Voronoi tessellation as stroke-only outlines — no fill — overlaid across several independently-jittered site positions, so shared edges between frames build up into a dense tangled mesh in the high-density core while the sparse periphery stays a single clean web of long lines. A gray-to-red-to-orange-to-yellow ramp and soft glow pick out the densest region; short red streaks mark low-density outlier points.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"neural"</code></span><span><strong>Aliases</strong> <code>neural / mesh / turbidity / particle_mesh / neural_mesh</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-neural.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-en-bloom">
<p>Estimates a smooth 2D density field from the scattered points (a Gaussian-kernel distance field, bandwidth auto-derived from point count and plot area) and bands it into discrete iso-level cells — every cell is colored by the highest density threshold it clears, so overlapping seeds fuse into organic merged "islands" instead of staying as separate circles. Cell edges are lightly jittered per-cell for a hand-cut, faceted look, and the outermost frame borrows architectural-drawing conventions: a diagonal-hatch margin, circled grid-reference bubbles, and a data-derived "NODES" tally (seed count, band count, peak-band cell count, coverage, bandwidth).</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bloom"</code></span><span><strong>Aliases</strong> <code>bloom / contour / density_bloom / iso_contour / organic_contour</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-bloom.html"></iframe>
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
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','mincnt',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Mincnt</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','nested',this)"><span class="sp-cic">◎</span><span class="sp-clb">Imbriqué</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','log_counts',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Log</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','weighted',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Pondéré</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','dotted',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Pointillé</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','marginals',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Marges</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','voronoi',this)"><span class="sp-cic">◈</span><span class="sp-clb">Voronoï</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','neural',this)"><span class="sp-cic">✦</span><span class="sp-clb">Neural</span></button>
<button class="sp-cls-tab" onclick="spCls('hexbin-fr','bloom',this)"><span class="sp-cic">❀</span><span class="sp-clb">Bloom</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="hexbin-fr-basic">
<p>Hexagones pleins uniquement, grille compacte, légende de densité à droite.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic / filled</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-basic.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-outlined">
<p>Contours blancs ; le comptage est affiché dans chaque hexagone assez grand pour contenir le texte.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"outlined"</code></span><span><strong>Alias</strong> <code>outlined / outline / stroke / labeled</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-outlined.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-spaced">
<p>Hexagones dessinés à 72% de leur taille avec un espace visible entre voisins — un rendu confetti plutôt qu'une surface pavée pleine.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"spaced"</code></span><span><strong>Alias</strong> <code>spaced / gapped / confetti</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-spaced.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-highlight">
<p>Estompe toutes les cellules sauf les ~15% les plus denses (pleine opacité, contour blanc, effectif affiché) — attire l'œil directement sur les zones chaudes plutôt que sur le dégradé complet.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"highlight"</code></span><span><strong>Alias</strong> <code>highlight / top / hotspot / peak</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-highlight.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-mincnt">
<p>Les cellules sous <code>min_count</code> sont totalement ignorées (laissées transparentes) plutôt que dessinées en estompé - un seuil dur plutôt qu'un dégradé atténué, comme <code>hexbin(mincnt=)</code> en R.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"mincnt"</code></span><span><strong>Alias</strong> <code>mincnt / threshold / sparse</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-mincnt.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-nested">
<p>L'effectif de chaque cellule est classé dans une bande d'ordre de grandeur (unités/dizaines/centaines/milliers/dizaines de milliers), colorée et dimensionnée selon la bande, avec un hexagone imbriqué plus petit à l'intérieur dans la couleur de la bande précédente - comme les styles imbriqués/centroïdes du package R hexbin - plus une légende taille+couleur.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"nested"</code></span><span><strong>Alias</strong> <code>nested / magnitude / rings / centroids</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-nested.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-log_counts">
<p>La couleur des cellules est basée sur <code>log(effectif + 1)</code> plutôt que sur l'effectif brut, comme <code>hexbin(bins="log")</code> en matplotlib — comprime la large plage dynamique des nuages de points asymétriques pour que les cellules peu denses restent visuellement distinguables au lieu de s'écraser près de zéro.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"log_counts"</code></span><span><strong>Alias</strong> <code>log_counts / log / log_scale / logarithmic</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-log_counts.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-weighted">
<p>La couleur des cellules encode la moyenne d'une troisième variable (<code>values=</code>) dans chaque cellule plutôt que l'effectif — l'équivalent natif de <code>hexbin(C=..., reduce_C_function=numpy.mean)</code> en matplotlib, quand la grandeur d'intérêt n'est pas la densité elle-même.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"weighted"</code></span><span><strong>Alias</strong> <code>weighted / mean / aggregate / reduce_mean</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-weighted.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-dotted">
<p>Contours pointillés blancs sur un dégradé continu complet (par défaut <code>magma</code>) sans bordure de graphique — reproduit exactement le style <code>hexbin(edgecolor="white", linestyle="dotted", linewidth=1.5)</code> de matplotlib.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"dotted"</code></span><span><strong>Alias</strong> <code>dotted / dashed / styled / magma</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-dotted.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-marginals">
<p>Ajoute des bandes de densité 1D au-dessus et à droite de la grille hexbin — une combinaison façon joint-plot, montrant la distribution marginale de chaque axe en plus de la densité 2D.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"marginals"</code></span><span><strong>Alias</strong> <code>marginals / joint / with_histograms / density_marginals</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-marginals.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-voronoi">
<p>Découpe le nuage de points en diagramme de Voronoï borné au lieu d'une grille hexagonale régulière — le remplissage de chaque cellule encode la densité locale (<code>values=</code> si fourni, sinon l'inverse de l'aire de la cellule elle-même), donc la densité se lit dans des tailles de cellules organiques irrégulières plutôt que dans une grille uniforme. Les points sources sont dessinés en petits points par-dessus, en écho aux posters scientifiques de type "particle tracking".</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"voronoi"</code></span><span><strong>Alias</strong> <code>voronoi / density_voronoi / tessellation / particle_density / cells</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-voronoi.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-neural">
<p>Trace la tessellation de Voronoï en contours seuls — sans remplissage — superposés sur plusieurs positions de sites légèrement décalées (jitter), si bien que les arêtes partagées entre ces passes s'accumulent en un maillage dense et enchevêtré dans le cœur de haute densité, tandis que la périphérie éparse reste un simple réseau propre de longues lignes. Un dégradé gris → rouge → orange → jaune et un léger halo mettent en évidence la zone la plus dense ; de courts traits rouges signalent les points isolés de faible densité.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"neural"</code></span><span><strong>Alias</strong> <code>neural / mesh / turbidity / particle_mesh / neural_mesh</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-neural.html"></iframe>
</div>
<div class="sp-variant" id="hexbin-fr-bloom">
<p>Estime un champ de densité 2D lisse à partir des points épars (champ de distance à noyau gaussien, largeur de bande dérivée automatiquement du nombre de points et de la surface du graphique) puis le découpe en cellules à niveaux iso discrets — chaque cellule est colorée selon le seuil de densité le plus élevé qu'elle franchit, si bien que des points sources qui se chevauchent fusionnent en "îlots" organiques au lieu de rester des cercles séparés. Les bords des cellules sont légèrement décalés individuellement pour un rendu façon découpe à la main, et le cadre extérieur emprunte les conventions du dessin d'architecture : marge à hachures diagonales, bulles de référence de grille cerclées, et un décompte "NODES" dérivé des données (nombre de sources, de bandes, de cellules au niveau maximal, couverture, largeur de bande).</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"bloom"</code></span><span><strong>Alias</strong> <code>bloom / contour / density_bloom / iso_contour / organic_contour</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/hexbin-bloom.html"></iframe>
</div>
</div>
</div>

</div>
