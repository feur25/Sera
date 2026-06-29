# Radar Chart 3D

<div class="lang-en">

<style>
.lang-en table,.lang-fr table{width:100%}
</style>

## Signature

`sp.radar3d(title, axes=None, series=None, *, series_names=None, palette=None, bg_color="#1a1a2e", width=700, height=600, max_val=None, fill_opacity=0.25, ring_gap=1.0, **kwargs) -> Chart`

Aliases: `sp.build_radar3d_chart()`, `sp.radar_3d()`, `sp.radar3d_chart()`, `sp.radar3d_family()`.

## Description

`sp.radar3d()` renders a 3D radar (spider) chart in a WebGL-like canvas scene. Each series becomes one ring of points around the shared `axes`, stacked along the depth axis instead of overlaid flat like the 2D radar.

## Parameters

<div data-sp-registry-table="options" data-family="radar3d"></div>

`ring_gap` controls the distance between series rings along the depth axis: `1.0` (default) keeps the original one-unit spacing per series, while lower values pull the rings closer together, down to `0.0` where every ring collapses onto the same plane.

## Returns

`Chart` object with an `.html` property and a `.show()` method.

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.radar3d(title, axes=None, series=None, *, series_names=None, palette=None, bg_color="#1a1a2e", width=700, height=600, max_val=None, fill_opacity=0.25, ring_gap=1.0, **kwargs) -> Chart`

Alias : `sp.build_radar3d_chart()`, `sp.radar_3d()`, `sp.radar3d_chart()`, `sp.radar3d_family()`.

<h2>Description</h2>

`sp.radar3d()` génère un graphique radar (toile d'araignée) en 3D dans une scène de type WebGL. Chaque série forme un anneau de points autour des `axes` partagés, empilés le long de l'axe de profondeur plutôt que superposés à plat comme le radar 2D.

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="radar3d"></div>

`ring_gap` contrôle la distance entre les anneaux de séries le long de l'axe de profondeur : `1.0` (défaut) conserve l'espacement d'origine d'une unité par série, tandis que des valeurs plus basses rapprochent les anneaux, jusqu'à `0.0` où tous les anneaux se superposent sur le même plan.

<h2>Retour</h2>

Objet `Chart` avec une propriété `.html` et une méthode `.show()`.

</div><!-- /lang-fr -->
