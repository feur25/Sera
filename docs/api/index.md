# API Reference

Complete alphabetical index of every public symbol exported by `seraplot`.

Index alphabétique complet de tous les symboles publics exportés par `seraplot`.

---

## Module: `seraplot`

```python
import seraplot as sp
```

---

## Chart Functions — 2D

| Function | Description |
|----------|-------------|
| [`build_area_chart`](../charts/2d/area.md) | Filled area / stacked area |
| [`build_bar_chart`](../charts/2d/bar.md) | Vertical or horizontal bar chart |
| [`build_boxplot`](../charts/2d/boxplot.md) | Box-and-whisker plot |
| [`build_bubble`](../charts/2d/bubble.md) | Bubble (scatter + size dimension) |
| [`build_bullet`](../charts/2d/bullet.md) | Bullet / progress bar chart |
| [`build_candlestick`](../charts/2d/candlestick.md) | OHLC candlestick financial chart |
| [`build_donut_chart`](../charts/2d/donut.md) | Donut (pie with hole) |
| [`build_dumbbell`](../charts/2d/dumbbell.md) | Dumbbell comparison chart |
| [`build_funnel`](../charts/2d/funnel.md) | Funnel / conversion chart |
| [`build_gauge`](../charts/2d/gauge.md) | Semicircular gauge |
| [`build_grid`](../charts/2d/grid.md) | Multi-chart grid layout |
| [`build_grouped_bar`](../charts/2d/grouped-bar.md) | Side-by-side grouped bars |
| [`build_hbar`](../charts/2d/hbar.md) | Horizontal bar chart |
| [`build_heatmap`](../charts/2d/heatmap.md) | Color-matrix heatmap |
| [`build_histogram`](../charts/2d/histogram.md) | Frequency histogram |
| [`build_histogram_overlay`](../charts/2d/histogram-overlay.md) | Overlaid histograms |
| [`build_kde_chart`](../charts/2d/kde.md) | Kernel density estimation curve |
| [`build_line_chart`](../charts/2d/line.md) | Line chart |
| [`build_lollipop_chart`](../charts/2d/lollipop.md) | Lollipop chart |
| [`build_multiline_chart`](../charts/2d/multiline.md) | Multiple line series |
| [`build_parallel`](../charts/2d/parallel.md) | Parallel coordinates |
| [`build_pie_chart`](../charts/2d/pie.md) | Pie chart |
| [`build_radar_chart`](../charts/2d/radar.md) | Radar / spider chart |
| [`build_ridgeline_chart`](../charts/2d/ridgeline.md) | Ridgeline (joy) chart |
| [`build_scatter_chart`](../charts/2d/scatter.md) | Scatter plot |
| [`build_slideshow`](../charts/2d/slideshow.md) | Interactive slideshow |
| [`build_slope`](../charts/2d/slope.md) | Slope / before-after chart |
| [`build_stacked_bar`](../charts/2d/stacked-bar.md) | Stacked bar chart |
| [`build_sunburst`](../charts/2d/sunburst.md) | Sunburst hierarchy chart |
| [`build_treemap`](../charts/2d/treemap.md) | Treemap hierarchy chart |
| [`build_violin`](../charts/2d/violin.md) | Violin distribution chart |
| [`build_waterfall`](../charts/2d/waterfall.md) | Waterfall / bridge chart |
| [`build_wordcloud`](../charts/2d/wordcloud.md) | Word cloud |

---

## Chart Functions — 3D

| Function | Description |
|----------|-------------|
| [`build_bar3d_chart`](../charts/3d/bar3d.md) | 3D bar chart |
| [`build_bubble3d_chart`](../charts/3d/bubble3d.md) | 3D bubble chart |
| [`build_candlestick3d_chart`](../charts/3d/candlestick3d.md) | 3D OHLC candlestick |
| [`build_dumbbell3d_chart`](../charts/3d/dumbbell3d.md) | 3D dumbbell chart |
| [`build_funnel3d_chart`](../charts/3d/funnel3d.md) | 3D funnel chart |
| [`build_globe3d_chart`](../charts/3d/globe3d.md) | Interactive 3D globe |
| [`build_heatmap3d_chart`](../charts/3d/heatmap3d.md) | 3D extruded heatmap |
| [`build_kde3d_chart`](../charts/3d/kde3d.md) | 3D KDE surface |
| [`build_line3d_chart`](../charts/3d/line3d.md) | 3D line chart |
| [`build_lollipop3d_chart`](../charts/3d/lollipop3d.md) | 3D lollipop chart |
| [`build_pie3d_chart`](../charts/3d/pie3d.md) | 3D pie chart |
| [`build_radar3d_chart`](../charts/3d/radar3d.md) | 3D radar chart |
| [`build_ridgeline3d_chart`](../charts/3d/ridgeline3d.md) | 3D ridgeline chart |
| [`build_scatter3d_chart`](../charts/3d/scatter3d.md) | 3D scatter (GPU WebGL) |
| [`build_stacked_bar3d_chart`](../charts/3d/stacked-bar3d.md) | 3D stacked bar chart |
| [`build_sunburst3d_chart`](../charts/3d/sunburst3d.md) | 3D sunburst chart |
| [`build_violin3d_chart`](../charts/3d/violin3d.md) | 3D violin chart |

---

## Chart Functions — ML

| Function | Description |
|----------|-------------|
| [`build_dbscan_chart`](../ml/dbscan.md) | 2D DBSCAN cluster visualization |
| [`build_dbscan_chart_3d`](../ml/dbscan3d.md) | 3D DBSCAN cluster visualization |

---

## Chart Functions — Map

| Function | Description |
|----------|-------------|
| [`build_bubble_map`](../charts/map/bubble-map.md) | Proportional symbol world map |
| [`build_choropleth`](../charts/map/choropleth.md) | Filled choropleth world map |

---

## Classes

| Class | Description |
|-------|-------------|
| [`DBSCAN`](../ml/dbscan-class.md) | Programmatic DBSCAN model |

---

## Configuration Functions

| Function | Description |
|----------|-------------|
| [`set_global_background`](../config/background.md) | Apply background to all charts |
| [`reset_global_background`](../config/background.md) | Clear global background |
| [`set_auto_display`](../config/auto-display.md) | Toggle Jupyter auto-display |
| [`set_bg_fn`](../config/background.md) | Apply background to existing HTML |

---

## Utility Functions

| Function | Description |
|----------|-------------|
| [`build_hover_json`](../config/hover.md) | Build custom hover tooltip JSON |
| `bench_chart_value` | Benchmark chart render time |
| `bench_pure_rust` | Low-level Rust benchmark |
| `show_chart_value` | Force-display a chart in Jupyter |
