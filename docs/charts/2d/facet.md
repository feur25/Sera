# Facet Grid — Small Multiples for Any Chart

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
.sp-preview-frame{border:none;border-radius:10px;display:block;width:100%;height:460px;background:transparent}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.facet(family, *, variant=None, facet_by, title="", cols=3, cell_width=320, cell_height=280, **kwargs) -> Chart`

Aliases: `sp.facet`, `sp.facet_grid`, `sp.facetgrid`, `sp.small_multiples`, `sp.build_facet`

## Description

`sp.facet()` is not a chart family of its own — it is the framework's generic small-multiples
mechanism, the SeraPlot equivalent of seaborn's `FacetGrid`. It takes the name of any existing
2D chart family (`family=`), splits every data array in the call by a `facet_by` group key, and
calls that family's own builder once per group, laying the results out in a grid. Every current
and future chart family gets faceting for free — there is no per-chart code, no separate variant
enum, and no duplicated geometry: `services/plot/statistical/facet/mod.rs` dispatches straight to
the target family's real `build_*` function (`histogram::build_histogram`,
`line::build_line`, `joint::build_joint`, …), so any `**kwargs` valid for that family
(`colorscale=`, `bins=`, `variant=`, …) is simply forwarded through unfiltered.

Only arguments whose array length matches `facet_by` get split; everything else (scalars like
`bins=`, `variant=`, `colorscale=`) is copied to every cell unchanged. `cols=` sets the grid's
column count, `cell_width=` / `cell_height=` size each panel.

Currently wired families: `area`, `bar`, `boxplot`, `bubble`, `bullet`, `candlestick`, `dumbbell`,
`eventplot`, `funnel`, `gantt`, `gauge`, `heatmap`, `hexbin`, `histogram`, `icicle`, `joint`,
`kde`, `line`, `lollipop`, `parallel`, `pie`, `radar`, `ridgeline`, `scatter`, `slope`, `splom`,
`stackplot`, `sunburst`, `treemap`, `violin`, `waterfall`, `wordcloud` — see
`facet::dispatch()` for the exact match table.

## Data

Any array field accepted by the target `family=` (`values`, `labels`, `x`, `y`, …), plus
`facet_by` (`list[str]`) — the group label for each row, same length as the arrays being split.

## Recipes

Not every case needs its own SeraPlot demo — some seaborn gallery examples are already one call
away from an existing chart page:

- **Multiple ECDFs** (overlaid, not facetted) — [`histogram(variant="cumulative", color_groups=...)`](histogram.md) or [`kde(variant="cumulative", categories=...)`](kde.md): seaborn overlays several ECDF lines in one panel via category grouping, which is the same `color_groups=` / `categories=` convention already used for stacking and multi-series density.
- **Pair grid** (KDE diagonal, paired point plots, dot-plot matrix) — [`splom(variant="density")`](splom.md) or [`splom(variant="basic")`](splom.md): SPLOM is SeraPlot's scatterplot-matrix family and already covers the pairwise-comparison layout these seaborn examples show.
- **Radial facets** — `facet(family="bar", facet_by=...)`: SeraPlot doesn't have a native polar/rose histogram variant yet, so this reproduces the *faceting technique* with a regular bar chart per facet rather than a pixel-perfect polar match.
- **Facetted time series** — same mechanism as the Faceted Lineplot recipe below; swap in a time-ordered `labels=` array.

<div class="sp-panel-source">
<h2>Recipes</h2>
</div>

<div class="sp-cls sp-open" id="facet-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('facet-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('facet-en','faceted_histogram',this)"><span class="sp-cic">▥</span><span class="sp-clb">Faceted histogram</span></button>
<button class="sp-cls-tab" onclick="spCls('facet-en','faceted_lineplot',this)"><span class="sp-cic">∿</span><span class="sp-clb">Faceted lineplot</span></button>
<button class="sp-cls-tab" onclick="spCls('facet-en','many_facets',this)"><span class="sp-cic">▦</span><span class="sp-clb">Many facets</span></button>
<button class="sp-cls-tab" onclick="spCls('facet-en','multiple_conditional_kde',this)"><span class="sp-cic">◐</span><span class="sp-clb">Conditional KDE</span></button>
<button class="sp-cls-tab" onclick="spCls('facet-en','three_variable_histogram',this)"><span class="sp-cic">⬡</span><span class="sp-clb">3-variable histogram</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="facet-en-faceted_histogram">
<p>One histogram panel per <code>facet_by</code> group — matches seaborn's <code>faceted_histogram</code> example.</p>
<div class="sp-vmeta"><span><strong>Call</strong> <code>facet(family="histogram", values=[...], facet_by=[...], cols=2)</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/facet-faceted_histogram.html"></iframe>
</div>
<div class="sp-variant" id="facet-en-faceted_lineplot">
<p>One line panel per group — matches seaborn's <code>faceted_lineplot</code> / <code>timeseries_facets</code> examples (use a time-ordered <code>labels=</code> for the latter).</p>
<div class="sp-vmeta"><span><strong>Call</strong> <code>facet(family="line", labels=[...], values=[...], facet_by=[...], cols=3)</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/facet-faceted_lineplot.html"></iframe>
</div>
<div class="sp-variant" id="facet-en-many_facets">
<p>Faceting scales to any number of groups — matches seaborn's <code>many_facets</code> example.</p>
<div class="sp-vmeta"><span><strong>Call</strong> <code>facet(family="bar", labels=[...], values=[...], facet_by=[...], cols=3)</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/facet-many_facets.html"></iframe>
</div>
<div class="sp-variant" id="facet-en-multiple_conditional_kde">
<p>One KDE curve per condition — matches seaborn's <code>multiple_conditional_kde</code> example.</p>
<div class="sp-vmeta"><span><strong>Call</strong> <code>facet(family="kde", values=[...], facet_by=[...], cols=3)</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/facet-multiple_conditional_kde.html"></iframe>
</div>
<div class="sp-variant" id="facet-en-three_variable_histogram">
<p>Facets a bivariate <code>joint(variant="histogram2d")</code> panel by a third variable — matches seaborn's <code>three_variable_histogram</code> example, combining the facet mechanism with the [`joint`](joint.md) family's `histogram2d` alias.</p>
<div class="sp-vmeta"><span><strong>Call</strong> <code>facet(family="joint", variant="histogram2d", x=[...], y=[...], facet_by=[...], cols=2)</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/facet-three_variable_histogram.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.facet(family, *, variant=None, facet_by, title="", cols=3, cell_width=320, cell_height=280, **kwargs) -> Chart`

Alias : `sp.facet`, `sp.facet_grid`, `sp.facetgrid`, `sp.small_multiples`, `sp.build_facet`

## Description

`sp.facet()` n'est pas une famille de graphique en soi — c'est le mécanisme générique de petits
multiples du framework, l'équivalent SeraPlot de `FacetGrid` de seaborn. Il prend le nom de
n'importe quelle famille 2D existante (`family=`), découpe chaque tableau de données de l'appel
selon une clé de groupe `facet_by`, et appelle le builder de cette famille une fois par groupe, en
disposant les résultats dans une grille. Chaque famille actuelle et future obtient le facettage
gratuitement — aucun code par graphique, aucune énumération de variante séparée, aucune géométrie
dupliquée : `services/plot/statistical/facet/mod.rs` appelle directement le vrai `build_*` de la
famille cible (`histogram::build_histogram`, `line::build_line`, `joint::build_joint`, …), donc
tout `**kwargs` valide pour cette famille (`colorscale=`, `bins=`, `variant=`, …) est simplement
transmis tel quel.

Seuls les arguments dont la longueur de tableau correspond à `facet_by` sont découpés ; le reste
(scalaires comme `bins=`, `variant=`, `colorscale=`) est copié tel quel dans chaque cellule.
`cols=` fixe le nombre de colonnes de la grille, `cell_width=` / `cell_height=` dimensionnent
chaque panneau.

Familles déjà câblées : `area`, `bar`, `boxplot`, `bubble`, `bullet`, `candlestick`, `dumbbell`,
`eventplot`, `funnel`, `gantt`, `gauge`, `heatmap`, `hexbin`, `histogram`, `icicle`, `joint`,
`kde`, `line`, `lollipop`, `parallel`, `pie`, `radar`, `ridgeline`, `scatter`, `slope`, `splom`,
`stackplot`, `sunburst`, `treemap`, `violin`, `waterfall`, `wordcloud` — voir `facet::dispatch()`
pour la table de correspondance exacte.

## Données

Tout champ tableau accepté par la famille `family=` cible (`values`, `labels`, `x`, `y`, …), plus
`facet_by` (`list[str]`) — l'étiquette de groupe de chaque ligne, de même longueur que les
tableaux à découper.

## Recettes

Certains exemples de la galerie seaborn ne nécessitent pas de nouvelle démo SeraPlot — ils sont
déjà à un appel d'une page de graphique existante :

- **ECDF multiples** (superposées, pas facettées) — [`histogram(variant="cumulative", color_groups=...)`](histogram.md) ou [`kde(variant="cumulative", categories=...)`](kde.md) : seaborn superpose plusieurs courbes ECDF dans un seul panneau via le regroupement par catégorie, la même convention `color_groups=` / `categories=` déjà utilisée pour l'empilement et la densité multi-séries.
- **Pair grid** (diagonale KDE, point plots appairés, matrice de dot-plots) — [`splom(variant="density")`](splom.md) ou [`splom(variant="basic")`](splom.md) : SPLOM est la famille matrice de nuages de points de SeraPlot et couvre déjà la disposition en comparaison par paires de ces exemples.
- **Facettes radiales** — `facet(family="bar", facet_by=...)` : SeraPlot n'a pas encore de variante native d'histogramme polaire/rose des vents, donc ceci reproduit la *technique de facettage* avec un bar chart classique par facette plutôt qu'une correspondance polaire exacte.
- **Séries temporelles facettées** — même mécanisme que la recette Faceted Lineplot ci-dessous ; remplacez `labels=` par un tableau ordonné dans le temps.

<div class="sp-panel-source">
<h2>Recettes</h2>
</div>

<div class="sp-cls sp-open" id="facet-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('facet-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('facet-fr','faceted_histogram',this)"><span class="sp-cic">▥</span><span class="sp-clb">Histogramme facetté</span></button>
<button class="sp-cls-tab" onclick="spCls('facet-fr','faceted_lineplot',this)"><span class="sp-cic">∿</span><span class="sp-clb">Courbes facettées</span></button>
<button class="sp-cls-tab" onclick="spCls('facet-fr','many_facets',this)"><span class="sp-cic">▦</span><span class="sp-clb">Beaucoup de facettes</span></button>
<button class="sp-cls-tab" onclick="spCls('facet-fr','multiple_conditional_kde',this)"><span class="sp-cic">◐</span><span class="sp-clb">KDE conditionnelle</span></button>
<button class="sp-cls-tab" onclick="spCls('facet-fr','three_variable_histogram',this)"><span class="sp-cic">⬡</span><span class="sp-clb">Histogramme à 3 variables</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="facet-fr-faceted_histogram">
<p>Un panneau d'histogramme par groupe <code>facet_by</code> — correspond à l'exemple <code>faceted_histogram</code> de seaborn.</p>
<div class="sp-vmeta"><span><strong>Appel</strong> <code>facet(family="histogram", values=[...], facet_by=[...], cols=2)</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/facet-faceted_histogram.html"></iframe>
</div>
<div class="sp-variant" id="facet-fr-faceted_lineplot">
<p>Un panneau de courbe par groupe — correspond aux exemples <code>faceted_lineplot</code> / <code>timeseries_facets</code> de seaborn (utilisez un <code>labels=</code> ordonné dans le temps pour le second).</p>
<div class="sp-vmeta"><span><strong>Appel</strong> <code>facet(family="line", labels=[...], values=[...], facet_by=[...], cols=3)</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/facet-faceted_lineplot.html"></iframe>
</div>
<div class="sp-variant" id="facet-fr-many_facets">
<p>Le facettage passe à l'échelle pour n'importe quel nombre de groupes — correspond à l'exemple <code>many_facets</code> de seaborn.</p>
<div class="sp-vmeta"><span><strong>Appel</strong> <code>facet(family="bar", labels=[...], values=[...], facet_by=[...], cols=3)</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/facet-many_facets.html"></iframe>
</div>
<div class="sp-variant" id="facet-fr-multiple_conditional_kde">
<p>Une courbe de KDE par condition — correspond à l'exemple <code>multiple_conditional_kde</code> de seaborn.</p>
<div class="sp-vmeta"><span><strong>Appel</strong> <code>facet(family="kde", values=[...], facet_by=[...], cols=3)</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/facet-multiple_conditional_kde.html"></iframe>
</div>
<div class="sp-variant" id="facet-fr-three_variable_histogram">
<p>Facette un panneau bivarié <code>joint(variant="histogram2d")</code> par une troisième variable — correspond à l'exemple <code>three_variable_histogram</code> de seaborn, combinant le mécanisme de facettage avec l'alias <code>histogram2d</code> de la famille [`joint`](joint.md).</p>
<div class="sp-vmeta"><span><strong>Appel</strong> <code>facet(family="joint", variant="histogram2d", x=[...], y=[...], facet_by=[...], cols=2)</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/facet-three_variable_histogram.html"></iframe>
</div>
</div>
</div>

</div>
