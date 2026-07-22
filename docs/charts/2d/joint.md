# Joint — Bivariate Panel + Marginals

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

`sp.joint(title, x_values, y_values, *, variant="hexbin", marginal="histogram", panel_variant="basic", show_points=False, show_regression=False, bins=24, colorscale=None, categories=None, **kwargs) -> Chart`

Aliases: `sp.joint`, `sp.jointplot`, `sp.joint_plot`, `sp.bivariate`, `sp.build_joint`

## Description

`sp.joint()` composes a bivariate main panel with top and right marginal distributions — the SeraPlot equivalent of seaborn's `jointplot()` / `JointGrid`. The main panel and the marginals are two **independent** choices, not a fixed pairing: `variant=` picks the main-panel technique (`hexbin`, `heat_grid`, `kde_heat`, `kde_contour`, `scatter`) and `marginal=` picks the top/right technique (`histogram`, `kde`, `rug`, `none`) — any combination works, e.g. `sp.joint(x, y, variant="hexbin", marginal="kde")` puts KDE curves in the margins of a hexbin panel instead of the default histograms. When `variant="hexbin"`, `panel_variant=` additionally selects among [`hexbin()`](hexbin.md)'s own cell styles (`basic` / `outlined` / `spaced` / `highlight`) by calling straight into hexbin's own per-cell drawing code — no reimplementation. `show_points=True` overlays raw scatter points on any panel, `show_regression=True` overlays a least-squares fit line (reusing [`scatter()`](scatter.md)'s regression fit).

Every panel reuses existing framework primitives instead of duplicating geometry: hexagonal binning from [`hexbin()`](hexbin.md), 1D bin edges from [`histogram()`](histogram.md), the Gaussian KDE engine from [`kde()`](kde.md), and the continuous colorscale engine from [`heatmap()`](heatmap.md). Pass `categories=` alongside `x_values` / `y_values` to compare density surfaces across groups with `variant="kde_contour"` — the same `categories=` convention used by [`kde()`](kde.md) and [`histogram()`](histogram.md) for grouped series.

The 8 names from earlier SeraPlot releases (`hexbin_marginal`, `heat_scatter`, `layered_bivariate`, `joint_kde`, `kde_smooth`, `multiple_bivariate_kde`, `marginal_ticks`, `regression_marginals`, plus `joint_histogram` / `histogram2d`) still work unchanged as `variant=` values — each just resolves to a `(panel, marginal, show_points, show_regression)` preset under the hood, so nothing breaks.

## Variants

<div data-sp-registry-table="variants" data-family="joint"></div>

## Data

`x_values` (`list[float]`) — X coordinates. `y_values` (`list[float]`) — Y coordinates. `categories` (`list[str]`, optional) — group label per point, required for `variant="kde_contour"`.

## Parameters

<div data-sp-registry-table="options" data-family="joint"></div>

## Themes

<div data-sp-registry-table="themes" data-family="joint"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="joint"></div>
</div>

<div class="sp-cls sp-open" id="joint-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('joint-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('joint-en','hexbin_marginal',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Hexbin marginal</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-en','heat_scatter',this)"><span class="sp-cic">▦</span><span class="sp-clb">Heat scatter</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-en','layered_bivariate',this)"><span class="sp-cic">∿</span><span class="sp-clb">Layered bivariate</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-en','joint_kde',this)"><span class="sp-cic">◐</span><span class="sp-clb">Joint KDE</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-en','kde_smooth',this)"><span class="sp-cic">▤</span><span class="sp-clb">KDE smooth</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-en','multiple_bivariate_kde',this)"><span class="sp-cic">⊛</span><span class="sp-clb">Multiple bivariate KDE</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-en','marginal_ticks',this)"><span class="sp-cic">┆</span><span class="sp-clb">Marginal ticks</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-en','regression_marginals',this)"><span class="sp-cic">↗</span><span class="sp-clb">Regression marginals</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-en','custom_combo',this)"><span class="sp-cic">✦</span><span class="sp-clb">Custom composition</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="joint-en-hexbin_marginal">
<p>Hexagonal density binning in the main panel, 1D histogram marginals along both axes — the closest match to seaborn's <code>hexbin_marginals</code> example.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"hexbin_marginal"</code></span><span><strong>Aliases</strong> <code>hexbin_marginal / hexbin / hexbin_marginals</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-hexbin_marginal.html"></iframe>
</div>
<div class="sp-variant" id="joint-en-heat_scatter">
<p>Square-grid 2D histogram heatmap with histogram marginals — matches seaborn's <code>heat_scatter</code> / "Scatterplot heatmap" example. Same technique also reachable via the <code>joint_histogram</code> / <code>histogram2d</code> aliases.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heat_scatter"</code></span><span><strong>Aliases</strong> <code>heat_scatter / heatscatter / density_heat / joint_histogram / histogram2d / hist2d</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-heat_scatter.html"></iframe>
</div>
<div class="sp-variant" id="joint-en-layered_bivariate">
<p>Smooth 2D Gaussian KDE surface with a raw scatter overlay and KDE marginal curves — matches seaborn's <code>layered_bivariate_plot</code> example.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"layered_bivariate"</code></span><span><strong>Aliases</strong> <code>layered_bivariate / kde_scatter / layered</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-layered_bivariate.html"></iframe>
</div>
<div class="sp-variant" id="joint-en-joint_kde">
<p>The same smooth 2D KDE surface as <code>layered_bivariate</code> with KDE marginal curves, but without the scatter overlay — matches seaborn's <code>joint_kde</code> example.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"joint_kde"</code></span><span><strong>Aliases</strong> <code>joint_kde / kde_joint / bivariate_kde</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-joint_kde.html"></iframe>
</div>
<div class="sp-variant" id="joint-en-kde_smooth">
<p>A finer 2D KDE surface paired with histogram marginals instead of KDE marginals — matches seaborn's <code>smooth_bivariate_kde</code> example.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"kde_smooth"</code></span><span><strong>Aliases</strong> <code>kde_smooth / smooth_bivariate_kde / smooth_kde</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-kde_smooth.html"></iframe>
</div>
<div class="sp-variant" id="joint-en-multiple_bivariate_kde">
<p>Overlapping translucent 2D KDE density islands, one per <code>categories=</code> group, no marginals — matches seaborn's "Overlapping bivariate KDE plots" example.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"multiple_bivariate_kde"</code></span><span><strong>Aliases</strong> <code>multiple_bivariate_kde / kde_multi / overlapping_kde</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-multiple_bivariate_kde.html"></iframe>
</div>
<div class="sp-variant" id="joint-en-marginal_ticks">
<p>Scatter main panel with rug tick marks in the margins instead of full marginal distributions — matches seaborn's <code>marginal_ticks</code> example.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"marginal_ticks"</code></span><span><strong>Aliases</strong> <code>marginal_ticks / rug_marginal / rug</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-marginal_ticks.html"></iframe>
</div>
<div class="sp-variant" id="joint-en-regression_marginals">
<p>Scatter plus a least-squares linear fit line, with histogram marginals — matches seaborn's <code>regression_marginals</code> example.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"regression_marginals"</code></span><span><strong>Aliases</strong> <code>regression_marginals / regression_joint / joint_regression</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-regression_marginals.html"></iframe>
</div>
<div class="sp-variant" id="joint-en-custom_combo">
<p>None of the 8 presets above is a fixed pairing — this is the same <code>hexbin</code> panel as the first tab, but with <code>panel_variant="outlined"</code> (hexbin's own outlined cell style) and <code>marginal="kde"</code> swapped in for the default histograms: <code>sp.joint(x, y, variant="hexbin", panel_variant="outlined", marginal="kde")</code>. Any panel × any marginal × (for hexbin) any panel_variant works.</p>
<div class="sp-vmeta"><span><strong>variant</strong> <code>"hexbin"</code></span><span><strong>panel_variant</strong> <code>"outlined"</code></span><span><strong>marginal</strong> <code>"kde"</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-custom_combo.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.joint(title, x_values, y_values, *, variant="hexbin_marginal", bins=24, colorscale=None, categories=None, **kwargs) -> Chart`

Alias : `sp.joint`, `sp.jointplot`, `sp.joint_plot`, `sp.bivariate`, `sp.build_joint`

## Description

`sp.joint()` compose un panneau bivarié principal avec des distributions marginales en haut et à droite — l'équivalent SeraPlot de `jointplot()` / `JointGrid` de seaborn. Chaque variante réutilise les primitives existantes du framework plutôt que de dupliquer la géométrie : le binning hexagonal de [`hexbin()`](hexbin.md), les bornes de classes de [`histogram()`](histogram.md), le moteur de KDE gaussien de [`kde()`](kde.md), et le moteur de dégradés continus de [`heatmap()`](heatmap.md). Passez `categories=` en plus de `x_values` / `y_values` pour comparer des surfaces de densité entre groupes avec `variant="multiple_bivariate_kde"` — la même convention `categories=` que [`kde()`](kde.md) et [`histogram()`](histogram.md) pour les séries groupées.

`heat_scatter` répond aussi à `joint_histogram` / `histogram2d` / `hist2d` — un histogramme 2D avec marges est la même technique sous un autre nom, donc aucune variante séparée ne la duplique.

## Variantes

<div data-sp-registry-table="variants" data-family="joint"></div>

## Données

`x_values` (`list[float]`) — Coordonnées X. `y_values` (`list[float]`) — Coordonnées Y. `categories` (`list[str]`, optionnel) — étiquette de groupe par point, requis pour `multiple_bivariate_kde`.

## Paramètres

<div data-sp-registry-table="options" data-family="joint"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="joint"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="joint"></div>
</div>

<div class="sp-cls sp-open" id="joint-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('joint-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('joint-fr','hexbin_marginal',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Hexbin marginal</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-fr','heat_scatter',this)"><span class="sp-cic">▦</span><span class="sp-clb">Heat scatter</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-fr','layered_bivariate',this)"><span class="sp-cic">∿</span><span class="sp-clb">Layered bivariate</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-fr','joint_kde',this)"><span class="sp-cic">◐</span><span class="sp-clb">Joint KDE</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-fr','kde_smooth',this)"><span class="sp-cic">▤</span><span class="sp-clb">KDE smooth</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-fr','multiple_bivariate_kde',this)"><span class="sp-cic">⊛</span><span class="sp-clb">KDE bivariées multiples</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-fr','marginal_ticks',this)"><span class="sp-cic">┆</span><span class="sp-clb">Ticks marginaux</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-fr','regression_marginals',this)"><span class="sp-cic">↗</span><span class="sp-clb">Régression + marges</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-fr','custom_combo',this)"><span class="sp-cic">✦</span><span class="sp-clb">Composition libre</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="joint-fr-hexbin_marginal">
<p>Binning hexagonal en densité dans le panneau principal, marges en histogrammes 1D sur les deux axes — l'équivalent le plus proche de l'exemple <code>hexbin_marginals</code> de seaborn.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"hexbin_marginal"</code></span><span><strong>Alias</strong> <code>hexbin_marginal / hexbin / hexbin_marginals</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-hexbin_marginal.html"></iframe>
</div>
<div class="sp-variant" id="joint-fr-heat_scatter">
<p>Carte de chaleur d'histogramme 2D en grille carrée avec marges en histogrammes — correspond à l'exemple <code>heat_scatter</code> / « Scatterplot heatmap » de seaborn. Même technique accessible aussi via les alias <code>joint_histogram</code> / <code>histogram2d</code>.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"heat_scatter"</code></span><span><strong>Alias</strong> <code>heat_scatter / heatscatter / density_heat / joint_histogram / histogram2d / hist2d</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-heat_scatter.html"></iframe>
</div>
<div class="sp-variant" id="joint-fr-layered_bivariate">
<p>Surface de KDE gaussienne 2D lisse avec un nuage de points brut superposé et des courbes de KDE en marge — correspond à l'exemple <code>layered_bivariate_plot</code> de seaborn.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"layered_bivariate"</code></span><span><strong>Alias</strong> <code>layered_bivariate / kde_scatter / layered</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-layered_bivariate.html"></iframe>
</div>
<div class="sp-variant" id="joint-fr-joint_kde">
<p>La même surface de KDE 2D lisse que <code>layered_bivariate</code> avec des courbes de KDE en marge, mais sans le nuage de points superposé — correspond à l'exemple <code>joint_kde</code> de seaborn.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"joint_kde"</code></span><span><strong>Alias</strong> <code>joint_kde / kde_joint / bivariate_kde</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-joint_kde.html"></iframe>
</div>
<div class="sp-variant" id="joint-fr-kde_smooth">
<p>Une surface de KDE 2D plus fine associée à des marges en histogrammes plutôt qu'en KDE — correspond à l'exemple <code>smooth_bivariate_kde</code> de seaborn.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"kde_smooth"</code></span><span><strong>Alias</strong> <code>kde_smooth / smooth_bivariate_kde / smooth_kde</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-kde_smooth.html"></iframe>
</div>
<div class="sp-variant" id="joint-fr-multiple_bivariate_kde">
<p>Îlots de densité KDE 2D translucides et superposés, un par groupe <code>categories=</code>, sans marges — correspond à l'exemple « Overlapping bivariate KDE plots » de seaborn.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"multiple_bivariate_kde"</code></span><span><strong>Alias</strong> <code>multiple_bivariate_kde / kde_multi / overlapping_kde</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-multiple_bivariate_kde.html"></iframe>
</div>
<div class="sp-variant" id="joint-fr-marginal_ticks">
<p>Nuage de points dans le panneau principal avec des ticks en marge au lieu de distributions complètes — correspond à l'exemple <code>marginal_ticks</code> de seaborn.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"marginal_ticks"</code></span><span><strong>Alias</strong> <code>marginal_ticks / rug_marginal / rug</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-marginal_ticks.html"></iframe>
</div>
<div class="sp-variant" id="joint-fr-regression_marginals">
<p>Nuage de points avec une droite de régression par moindres carrés, marges en histogrammes — correspond à l'exemple <code>regression_marginals</code> de seaborn.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"regression_marginals"</code></span><span><strong>Alias</strong> <code>regression_marginals / regression_joint / joint_regression</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-regression_marginals.html"></iframe>
</div>
<div class="sp-variant" id="joint-fr-custom_combo">
<p>Aucun des 8 préréglages ci-dessus n'est une association figée — ceci reprend le même panneau <code>hexbin</code> que le premier onglet, mais avec <code>panel_variant="outlined"</code> (le style de cellule outlined propre à hexbin) et <code>marginal="kde"</code> à la place des histogrammes par défaut : <code>sp.joint(x, y, variant="hexbin", panel_variant="outlined", marginal="kde")</code>. N'importe quel panneau × n'importe quelle marge × (pour hexbin) n'importe quel panel_variant fonctionne.</p>
<div class="sp-vmeta"><span><strong>variant</strong> <code>"hexbin"</code></span><span><strong>panel_variant</strong> <code>"outlined"</code></span><span><strong>marginal</strong> <code>"kde"</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-custom_combo.html"></iframe>
</div>
</div>
</div>

</div>
