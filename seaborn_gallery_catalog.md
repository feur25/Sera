# Seaborn example gallery — catalogue de portage vers SeraPlot

Source : https://seaborn.pydata.org/examples/index.html (49 exemples)

Statut : `existe` (variante à ajouter sur une famille déjà présente), `joint` (nouvelle
famille bivariée : panneau principal + marges), `facet` (méta-mécanisme générique de
petits multiples, hérite de n'importe quelle famille existante), `nouveau` (famille
sans équivalent).

## Prioritaires (demandés explicitement)

| Exemple | URL | Statut | Cible |
|---|---|---|---|
| Hexbin plot with marginal distributions | hexbin_marginals.html | joint | `joint::hexbin_marginal` |
| Scatterplot heatmap | heat_scatter.html | joint | `joint::heat_scatter` |
| Different cubehelix palettes | layered_bivariate_plot.html | joint | `joint::layered_bivariate` |
| Facetted histogram | faceted_histogram.html | facet | `facet::grid(histogram, *)` |

## Bivarié / joint (nouvelle famille `joint`)

| Exemple | URL | Cible |
|---|---|---|
| Hexbin plot with marginal distributions | hexbin_marginals.html | `joint::hexbin_marginal` |
| Scatterplot heatmap | heat_scatter.html | `joint::heat_scatter` |
| Layered bivariate plot | layered_bivariate_plot.html | `joint::layered_bivariate` |
| Joint histogram | joint_histogram.html | `joint::histogram` |
| Joint kernel density estimate | joint_kde.html | `joint::kde` |
| Overlapping bivariate KDE plots | multiple_bivariate_kde.html | `joint::kde_multi` |
| Plotting on a large number of facets | marginal_ticks.html | `joint::marginal_ticks` |
| Linear regression with marginal distributions | regression_marginals.html | `joint::regression` |
| Smooth kernel density with marginal histograms | smooth_bivariate_kde.html | `joint::kde_smooth` |

## Facettage générique (méta-mécanisme `facet`, hérite de n'importe quelle famille)

| Exemple | URL | Cible |
|---|---|---|
| Facetted histogram | faceted_histogram.html | `facet::grid(histogram)` |
| Facetted lineplot | faceted_lineplot.html | `facet::grid(line)` |
| Many facets | many_facets.html | `facet::grid(*)` |
| Radial facets | radial_facets.html | `facet::grid(polar)` |
| Conditional small multiples of KDE | multiple_conditional_kde.html | `facet::grid(kde)` |
| Multiple ECDFs | multiple_ecdf.html | `facet::grid(ecdf)` |
| Facetted time series | timeseries_facets.html | `facet::grid(line)` |
| 3-variable histogram | three_variable_histogram.html | `facet::grid(histogram2d)` |
| Pair grid + KDE | pair_grid_with_kde.html | `facet::pairgrid(kde)` |
| Paired point plots | paired_pointplots.html | `facet::pairgrid(pointplot)` |
| Dot plot with several variables | pairgrid_dotplot.html | `facet::pairgrid(dot)` |

## Déjà couvert par une famille existante (nouvelle variante seulement)

| Exemple | URL | Famille existante | Variante à ajouter |
|---|---|---|---|
| Anscombe's quartet | anscombes_quartet.html | scatter | `regression_facet` |
| Different scatterplot variables | different_scatter_variables.html | scatter | `sized_hued` |
| Lineplot with error bands | errorband_lineplots.html | line | `error_band` |
| Grouped barplot | grouped_barplot.html | bar | déjà `grouped` |
| Grouped boxplot | grouped_boxplot.html | boxplot | déjà `grouped` |
| Grouped violinplots | grouped_violinplots.html | violin | déjà `split`/basic |
| Stacked histogram | histogram_stacked.html | histogram | déjà `stacked` |
| Horizontal boxplot | horizontal_boxplot.html | boxplot | déjà `horizontal` |
| Jittered stripplot | jitter_stripplot.html | scatter/violin | `strip` |
| Ridgeplot (KDE) | kde_ridgeplot.html | ridgeline | déjà couvert |
| Boxenplot (letter-value) | large_distributions.html | boxplot | déjà `letter_value` |
| Logistic regression | logistic_regression.html | scatter | `logistic_fit` |
| Pairwise correlation heatmap | many_pairwise_correlations.html | heatmap | déjà `correlation` |
| Multiple regression lines | multiple_regression.html | line | `regression_multi` |
| Palette choices (barplot) | palette_choices.html | bar | thème seul |
| Palette generation (KDE) | palette_generation.html | kde | thème seul |
| Part-whole bars | part_whole_bars.html | bar | `part_whole` |
| Conditional means with point plot | pointplot_anova.html | dumbbell/bullet | `pointplot` |
| Regression with marginal dists | regression_marginals.html | (joint) | voir joint |
| Residual plot | residplot.html | scatter | `residual` |
| Scatterplot with bubbles | scatter_bubbles.html | bubble | déjà couvert |
| Categorical scatterplot (swarm) | scatterplot_categorical.html | scatter | `swarm` |
| Scatterplot matrix | scatterplot_matrix.html | splom | déjà couvert |
| Scatterplot with varying size | scatterplot_sizes.html | bubble | déjà couvert |
| Simple violinplots | simple_violinplots.html | violin | déjà couvert |
| Structured heatmap (clustermap) | structured_heatmap.html | dendrogram/heatmap | `clustered_heatmap` |
| Facetted time series | timeseries_facets.html | line | voir facet |
| Wide-form data lineplot | wide_data_lineplot.html | line | déjà couvert |
| Wide-form violinplot | wide_form_violinplot.html | violin | déjà couvert |
| Multi-panel scatter with marginals | joint_kde.html | (joint) | voir joint |
| Discrete strip + regression | strip_regplot.html | scatter | `strip_regression` |

## Implémenté

- `joint::hexbin_marginal`, `joint::heat_scatter`, `joint::layered_bivariate` — famille `joint`
  (`services/plot/statistical/joint/`), aliases `joint`/`jointplot`/`joint_plot`/`bivariate`.
- `facet::grid(*)` — mécanisme générique (`services/plot/statistical/facet/`), aliases
  `facet`/`facet_grid`/`facetgrid`/`small_multiples`. Prend `family`, `facet_by` et n'importe
  quel argument de la famille cible ; hérite automatiquement de toute famille enregistrée dans
  `facet::dispatch` (aucun code par graphique).

## Notes d'architecture

- `joint` est une famille au sens classique du framework (config.rs + variant.rs +
  mod.rs), mais son rendu compose systématiquement un panneau central plus deux
  marges (haut, droite), en réutilisant les primitives de tracé déjà partagées dans
  `services/plot/statistical/common.rs` (aucune duplication de géométrie).
- `facet` est un mécanisme générique et non une famille de graphiques : il prend en
  entrée un nom de famille + variante + une clé de facettage, découpe les données par
  catégorie et appelle N fois le `render()` de la famille cible dans une grille. Tout
  nouveau type de graphique en hérite automatiquement, sans code supplémentaire —
  c'est l'implémentation demandée de « n'importe quel variant du parent ».
- Beaucoup d'exemples de la galerie sont déjà couverts par une variante existante
  (boxenplot ↔ `letter_value`, ridgeplot ↔ `ridgeline`, clustermap ↔ `dendrogram` +
  `heatmap`, pairplot ↔ `splom`) : le travail réel à faible effort est de vérifier ces
  correspondances plutôt que de les recréer.
