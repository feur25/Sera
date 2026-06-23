# Bar Charts

<div class="lang-en">

<style>
.lang-en table,.lang-fr table{width:100%}
.sp-panel-source{display:none!important}
</style>

## Signature

`sp.bar(title, labels=None, values=None, *, variant="basic", series=None, series_names=None, theme="none", **kwargs) -> Chart`

Aliases: `sp.bar_chart()`, `sp.bars()`, `sp.bar_unified()`, `sp.bars_unified()`, `sp.bar_family()`.

## Description

`sp.bar()` is the unified entry point for the SeraPlot bar-chart family. It renders standalone Rust-generated HTML/SVG charts. The `variant` keyword selects the renderer, and shared chart options are applied by the common chart pipeline.

The default renderer is a vertical categorical bar chart. The same API also covers every bar variant registered in Rust.

## Variants

<div data-sp-registry-table="variants" data-family="bar"></div>

Unknown variant strings fall back to the registered default. Variant keys may be prefixed with `en_`, `fr_`, `en-` or `fr-`.

## Data

`labels` are category labels for bar variants. Single-series variants use `values`. Multi-series variants use `series`, where each inner list is one series, and `series_names` supplies legend names.

When `series` is missing but `series_names` is provided, `values` is interpreted as a flattened matrix split by `len(labels)`: the first category-length block is the first series, the next block is the second series, and so on.

## Parameters

<div data-sp-registry-table="options" data-family="bar"></div>

## Themes

<div data-sp-registry-table="themes" data-family="bar"></div>

## Returns

`Chart` object with an `.html` property and a `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="bar"></div>
</div>

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.bar(title, labels=None, values=None, *, variant="basic", series=None, series_names=None, theme="none", **kwargs) -> Chart`

Alias : `sp.bar_chart()`, `sp.bars()`, `sp.bar_unified()`, `sp.bars_unified()`, `sp.bar_family()`.

<h2>Description</h2>

`sp.bar()` est le point d'entrée unifié de la famille de graphiques en barres de SeraPlot. Il génère des graphiques HTML/SVG autonomes depuis Rust. Le mot-clé `variant` choisit le renderer, et les options communes passent par le pipeline commun.

Le rendu par défaut est un bar chart catégoriel vertical. La même API couvre toutes les variantes bar enregistrées côté Rust.

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="bar"></div>

Une variante inconnue retombe sur la valeur par défaut enregistrée. Les clés de variantes peuvent être préfixées par `en_`, `fr_`, `en-` ou `fr-`.

<h2>Données</h2>

`labels` sert de liste de catégories pour les variantes bar. Les variantes mono-série utilisent `values`. Les variantes multi-séries utilisent `series`, où chaque liste interne est une série, et `series_names` fournit les noms de légende.

Quand `series` manque mais que `series_names` est fourni, `values` est interprété comme une matrice aplatie découpée par `len(labels)` : le premier bloc appartient à la première série, le suivant à la deuxième, etc.

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="bar"></div>

<h2>Thèmes</h2>

<div data-sp-registry-table="themes" data-family="bar"></div>

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="bar"></div>
</div>

</div><!-- /lang-fr -->
