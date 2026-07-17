# Multi-Line Chart

<div class="lang-en">


---

## Description

A multi-line chart overlays multiple line series on the same axes, enabling direct comparison of how several metrics evolve in parallel over the same time dimension. Each inner list in `series_values` must have the same length as `x_labels`. Series are distinguished by color from the default palette (or a custom one) and labeled in the legend. Unlike `build_line_chart`, this function is optimized for 2–10 simultaneous series; beyond that, consider a ridgeline or heatmap.

**Ideal for:**
- Comparing multiple KPIs or products over the same period
- Visualizing A/B test outcomes over time
- Overlaying actual vs forecast vs target lines

---

## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `color_hex` | basic, connected_scatter, dashed, filled, gapped, sparkline, spline, stepped |
| `dash_pattern` | dashed |
| `fill_opacity` | filled |
| `gap_threshold` | gapped |
| `gridlines` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `height` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `hover` | basic, multi |
| `labels` | basic, connected_scatter, dashed, filled, gapped, neon, spline, stepped |
| `legend_position` | connected_scatter, dashed, filled, gapped, multi, neon |
| `marker_size` | connected_scatter, gapped, spline, stepped |
| `palette` | connected_scatter, dashed, filled, gapped, multi, neon, sparkline |
| `series` | connected_scatter, dashed, filled, gapped, multi, neon, sparkline |
| `show_points` | basic, gapped, multi, neon, spline, stepped |
| `sort_order` | basic, multi |
| `spark_cell_h` | sparkline |
| `spark_cell_w` | sparkline |
| `spark_cols` | sparkline |
| `spline_tension` | spline |
| `stack_fill` | filled |
| `step_shape` | stepped |
| `stroke_width` | connected_scatter, dashed, filled, gapped, sparkline, spline, stepped |
| `title` | all |
| `values` | basic, connected_scatter, dashed, filled, gapped, neon, sparkline, spline, stepped |
| `width` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `x_label` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `x_labels` | connected_scatter, dashed, filled, gapped, multi, neon |
| `y_label` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |

---

## Returns

`Chart`

## Preview

<iframe src="../../previews/multiline.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

</div>

<div class="lang-fr">


---

<h2>Description</h2>

Un graphique multi-courbes superpose plusieurs séries de lignes sur les mêmes axes, permettant la comparaison directe de l'évolution de plusieurs métriques en parallèle sur la même dimension temporelle. Chaque liste interne de `series_values` doit avoir la même longueur que `x_labels`. Les séries sont distinguées par les couleurs de la palette par défaut (ou d'une palette personnalisée) et étiquetées dans la légende. Contrairement à `build_line_chart`, cette fonction est optimisée pour 2 à 10 séries simultanées ; au-delà, envisagez un ridgeline ou une carte thermique.

**Idéal pour :**
- Comparer plusieurs indicateurs ou produits sur la même période
- Visualiser les résultats de tests A/B dans le temps
- Superposer les lignes réel, prévision et objectif

---

<h2>Paramètres</h2>

| Paramètre | Utilisé par variantes |
|-----------|----------------------|
| `color_hex` | basic, connected_scatter, dashed, filled, gapped, sparkline, spline, stepped |
| `dash_pattern` | dashed |
| `fill_opacity` | filled |
| `gap_threshold` | gapped |
| `gridlines` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `height` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `hover` | basic, multi |
| `labels` | basic, connected_scatter, dashed, filled, gapped, neon, spline, stepped |
| `legend_position` | connected_scatter, dashed, filled, gapped, multi, neon |
| `marker_size` | connected_scatter, gapped, spline, stepped |
| `palette` | connected_scatter, dashed, filled, gapped, multi, neon, sparkline |
| `series` | connected_scatter, dashed, filled, gapped, multi, neon, sparkline |
| `show_points` | basic, gapped, multi, neon, spline, stepped |
| `sort_order` | basic, multi |
| `spark_cell_h` | sparkline |
| `spark_cell_w` | sparkline |
| `spark_cols` | sparkline |
| `spline_tension` | spline |
| `stack_fill` | filled |
| `step_shape` | stepped |
| `stroke_width` | connected_scatter, dashed, filled, gapped, sparkline, spline, stepped |
| `title` | toutes |
| `values` | basic, connected_scatter, dashed, filled, gapped, neon, sparkline, spline, stepped |
| `width` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `x_label` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `x_labels` | connected_scatter, dashed, filled, gapped, multi, neon |
| `y_label` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |

---

<h2>Retourne</h2>

`Chart`

<h2>Aperçu</h2>

<iframe src="../../previews/multiline.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

</div>
