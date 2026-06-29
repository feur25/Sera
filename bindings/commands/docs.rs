pub const DOC_BUILD_HTML_CHART: &str =
    "build_html_chart(title, labels, values, width, height) -> Chart

Simple bar chart from raw values.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Category labels.
values : list[float]
    Bar values.
width : int
    Chart width in pixels.
height : int
    Chart height in pixels.";

pub const DOC_BUILD_BAR_CHART: &str = "build_bar_chart(title, labels, values, *, orientation='v', color_hex=0, show_text=False, color_groups=None, width=900, height=480, x_label='', y_label='', gridlines=False, sort_order='none', hover_json='', legend_position='right', palette=None, series_names=None, background=None, no_x_axis=False, no_y_axis=False) -> Chart

Vertical or horizontal bar chart with categorical data.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Category labels.
values : list[float]
    Bar values.
orientation : str, optional
    'v' (vertical) or 'h' (horizontal). Default 'v'.
color_hex : int, optional
    Single bar color as hex integer. Default 0 (uses palette).
show_text : bool, optional
    Show value labels on bars. Default False.
color_groups : list[str], optional
    Group names for color-coding bars.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 480.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
sort_order : str, optional
    Sort bars: 'none', 'asc', or 'desc'. Default 'none'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.
legend_position : str, optional
    Legend position: 'right', 'left', 'top', or 'bottom'. Default 'right'.
palette : list[int], optional
    Custom color palette as list of hex integers.
background : str or None, optional
    Background color string or None for transparent. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_HBAR: &str = "build_hbar(title, labels, values, *, color_hex=0, show_text=True, color_groups=None, width=900, height=500, x_label='', y_label='', gridlines=False, sort_order='none', hover_json='', legend_position='right', palette=None, series_names=None, background=None, no_x_axis=False, no_y_axis=False) -> Chart

Horizontal bar chart.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Category labels.
values : list[float]
    Bar values.
color_hex : int, optional
    Single bar color as hex integer. Default 0 (uses palette).
show_text : bool, optional
    Show value labels on bars. Default True.
color_groups : list[str], optional
    Group names for color-coding bars.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 500.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
sort_order : str, optional
    Sort bars: 'none', 'asc', or 'desc'. Default 'none'.
legend_position : str, optional
    Legend position: 'right', 'left', 'top', or 'bottom'. Default 'right'.
palette : list[int], optional
    Custom color palette as list of hex integers.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_LINE_CHART: &str = "build_line_chart(title, labels, values, *, color_hex=0x636EFA, show_points=True, width=900, height=480, x_label='', y_label='', gridlines=False, sort_order='none', hover_json='', legend_position='right', palette=None, series_names=None, background=None, no_x_axis=False, no_y_axis=False) -> Chart

Line chart with connected data points.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    X-axis labels.
values : list[float]
    Line values.
color_hex : int, optional
    Line color as hex integer. Default 0x636EFA.
show_points : bool, optional
    Show data point markers. Default True.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 480.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
sort_order : str, optional
    Sort data: 'none', 'asc', or 'desc'. Default 'none'.
legend_position : str, optional
    Legend position. Default 'right'.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_DBSCAN_CHART: &str = "build_dbscan_chart(title, x_values, y_values, *, eps=0.02, min_samples=10, width=1000, height=580, x_label='', y_label='', gridlines=False, palette=None, background=None, normalize=True) -> Chart

DBSCAN clustering scatter chart (2D).

Parameters
----------
title : str
    Chart title.
x_values : list[float]
    X coordinates.
y_values : list[float]
    Y coordinates.
eps : float, optional
    DBSCAN neighborhood radius. Default 0.02.
min_samples : int, optional
    Minimum points per cluster. Default 10.
width : int, optional
    Chart width in pixels. Default 1000.
height : int, optional
    Chart height in pixels. Default 580.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
palette : list[int], optional
    Custom cluster color palette.
background : str or None, optional
    Background color. Default None.
normalize : bool, optional
    Normalize input data before clustering. Default True.";

pub const DOC_BUILD_DBSCAN_CHART_3D: &str = "build_dbscan_chart_3d(title, x_values, y_values, z_values, *, eps=0.5, min_samples=5, width=900, height=560, x_label='X', y_label='Y', z_label='Z', bg_color='', normalize=False, palette=None) -> Chart

DBSCAN clustering scatter chart (3D).

Parameters
----------
title : str
    Chart title.
x_values : list[float]
    X coordinates.
y_values : list[float]
    Y coordinates.
z_values : list[float]
    Z coordinates.
eps : float, optional
    DBSCAN neighborhood radius. Default 0.5.
min_samples : int, optional
    Minimum points per cluster. Default 5.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default 'X'.
y_label : str, optional
    Y-axis label. Default 'Y'.
z_label : str, optional
    Z-axis label. Default 'Z'.
bg_color : str, optional
    Background color string. Default ''.
normalize : bool, optional
    Normalize input data. Default False.
palette : list[int], optional
    Custom cluster color palette.";

pub const DOC_BUILD_KMEANS_CHART: &str = "build_kmeans_chart(title, x_values, y_values, *, k=3, max_iter=300, tol=1e-4, mini_batch=False, batch_size=1000, width=1000, height=580, x_label='', y_label='', gridlines=False, normalize=True, show_centroids=True, palette=None, background=None) -> Chart

K-Means clustering scatter chart.

Parameters
----------
title : str
    Chart title.
x_values : list[float]
    X coordinates.
y_values : list[float]
    Y coordinates.
k : int, optional
    Number of clusters. Default 3.
max_iter : int, optional
    Maximum iterations. Default 300.
tol : float, optional
    Convergence tolerance. Default 1e-4.
mini_batch : bool, optional
    Use Mini-Batch K-Means. Default False.
batch_size : int, optional
    Batch size for mini-batch mode. Default 1000.
width : int, optional
    Chart width in pixels. Default 1000.
height : int, optional
    Chart height in pixels. Default 580.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
normalize : bool, optional
    Normalize input data. Default True.
show_centroids : bool, optional
    Show cluster centroids. Default True.
palette : list[int], optional
    Custom cluster color palette.
background : str or None, optional
    Background color. Default None.";

pub const DOC_BUILD_SCATTER_CHART: &str = "build_scatter_chart(title, x_values, y_values, *, variant=None, categories=None, labels=None, color_values=None, color_low=0x636EFA, color_high=0xF43F5E, color_hex=0, point_size=5.0, stroke_width=1.0, symbol='circle', symbols=None, regression_type='linear', show_regression=False, show_text=False, sizes=None, color_groups=None, width=900, height=540, x_label='', y_label='', gridlines=False, legend_position='right', palette=None, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

Scatter plot of numeric X/Y data.

Parameters
----------
title : str
    Chart title.
x_values : list[float]
    X coordinates.
y_values : list[float]
    Y coordinates.
color_hex : int, optional
    Point color as hex integer. Default 0 (uses palette).
show_text : bool, optional
    Show point labels. Default False.
labels : list[str], optional
    Per-point label text.
sizes : list[float], optional
    Per-point size values.
color_groups : list[str], optional
    Group names for color-coding points.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 540.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
legend_position : str, optional
    Legend position. Default 'right'.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.
show_regression : bool, optional
    Overlay a regression line. Default False.
regression_type : str, optional
    Regression type: 'linear', 'polynomial', etc. Default 'linear'.";

pub const DOC_BUILD_HISTOGRAM: &str = "build_histogram(title, values, *, variant='basic', orientation='v', bins=0, gap=2, color_hex=0x636EFA, overlay_values=None, overlay_color_hex=0xF43F5E, color_groups=None, palette=None, series_names=None, show_counts=False, stroke_width=1.0, width=860, height=380, x_label='', y_label='Count', gridlines=False, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

Unified histogram entry point. Dispatches by `variant`:
'basic' | 'horizontal' | 'normalized' | 'cumulative' | 'stacked' | 'overlay' | 'step'.
Set `orientation='h'` to render any variant horizontally (framework-native).

Parameters
----------
title : str
    Chart title.
values : list[float]
    Numeric values to bin.
variant : str, optional
    Histogram variant. Default 'basic'.
orientation : str, optional
    'v' (vertical) or 'h' (horizontal). 'h' routes any variant through the
    horizontal renderer. Default 'v'.
bins : int, optional
    Number of bins. 0 = auto (Sturges). Default 0.
gap : int, optional
    Pixel gap between adjacent bars. Default 2.
color_hex : int, optional
    Primary bar color. 0 falls back to `palette[0]` then 0x636EFA. Default 0x636EFA.
overlay_values : list[float] or None, optional
    Second distribution (used by 'overlay'). Default None.
overlay_color_hex : int, optional
    Overlay color. Default 0xF43F5E.
color_groups : list[str] or None, optional
    Per-value category labels. Used by 'stacked' and by 'overlay' to render
    N series via the `palette`. Default None.
palette : list[int] or None, optional
    Custom color palette (used by 'stacked' and N-series 'overlay').
series_names : list[str] or None, optional
    Two names for legend (used by classic 2-series 'overlay').
show_counts : bool, optional
    Show count labels on bars. Default False.
stroke_width : float, optional
    Stroke width (used by 'step'). Default 1.0.
width : int, optional
    Chart width in pixels. Default 860.
height : int, optional
    Chart height in pixels. Default 380.
x_label : str, optional
    X-axis label.
y_label : str, optional
    Y-axis label. Default 'Count'.
gridlines : bool, optional
    Show gridlines. Default False.
hover_json : str, optional
    Hover slot JSON.
background : str or None, optional
    Background color.
no_x_axis : bool, optional
    Hide the X axis.
no_y_axis : bool, optional
    Hide the Y axis.";

pub const DOC_BUILD_HISTOGRAM_OVERLAY: &str = "build_histogram_overlay(title, values, overlay_values, *, color_hex=0x636EFA, overlay_color_hex=0xF43F5E, bins=0, width=860, height=380, x_label='', y_label='Count', gridlines=False, background=None, no_x_axis=False, no_y_axis=False) -> Chart

Two overlapping histograms for distribution comparison.

Parameters
----------
title : str
    Chart title.
values : list[float]
    Primary distribution values.
overlay_values : list[float]
    Secondary distribution values.
color_hex : int, optional
    Primary histogram color. Default 0x636EFA.
overlay_color_hex : int, optional
    Overlay histogram color. Default 0xF43F5E.
bins : int, optional
    Number of bins. 0 = auto. Default 0.
width : int, optional
    Chart width in pixels. Default 860.
height : int, optional
    Chart height in pixels. Default 380.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default 'Count'.
gridlines : bool, optional
    Show gridlines. Default False.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_GROUPED_BAR: &str = "build_grouped_bar(title, category_labels, series_values, *, show_values=False, series_names=None, width=1100, height=480, x_label='', y_label='', gridlines=False, legend_position='right', palette=None, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

Grouped bar chart with multiple series side by side.

Parameters
----------
title : str
    Chart title.
category_labels : list[str]
    Category labels on the X axis.
series_values : list[list[float]]
    One inner list per series with a value per category.
show_values : bool, optional
    Show value labels on bars. Default False.
series_names : list[str], optional
    Names for each series.
width : int, optional
    Chart width in pixels. Default 1100.
height : int, optional
    Chart height in pixels. Default 480.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
legend_position : str, optional
    Legend position. Default 'right'.
palette : list[int], optional
    Custom color palette.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_STACKED_BAR: &str = "build_stacked_bar(title, category_labels, series_values, *, show_values=False, series_names=None, width=1100, height=480, x_label='', y_label='', gridlines=False, legend_position='right', palette=None, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

Stacked bar chart with multiple series.

Parameters
----------
title : str
    Chart title.
category_labels : list[str]
    Category labels on the X axis.
series_values : list[list[float]]
    One inner list per series with a value per category.
show_values : bool, optional
    Show value labels on bars. Default False.
series_names : list[str], optional
    Names for each series.
width : int, optional
    Chart width in pixels. Default 1100.
height : int, optional
    Chart height in pixels. Default 480.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
legend_position : str, optional
    Legend position. Default 'right'.
palette : list[int], optional
    Custom color palette.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_HEATMAP: &str = "build_heatmap(title, labels, flat_matrix, *, variant='basic', show_values=True, color_low=0x636EFA, color_mid=0xfafbfc, color_high=0xF43F5E, palette=None, bins=0, widths=None, ranges=None, col_labels=None, width=720, height=440, x_label='', y_label='', gridlines=False, background=None) -> Chart

Color-coded matrix heatmap (7 variants).

Variants
--------
basic        Smooth 3-stop gradient on a uniform grid (default).
annotated    Forces value labels in every cell with adaptive contrast.
categorical  Distinct palette color per discrete value (no gradient).
unequal      Cell widths/heights driven by `widths` and `ranges` arrays.
log          Logarithmic color mapping (log10) for skewed data.
discrete     Quantizes values into N color bands (set with `bins`).
correlation  Diverging palette centered at 0 (red\u{2194}blue), ideal for [-1,1] matrices.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Row labels.
flat_matrix : list[float]
    Flattened matrix values (row-major order, length = n_rows * n_cols).
variant : str, optional
    Heatmap variant. Default 'basic'.
show_values : bool, optional
    Show cell values when cell is large enough. Default True.
color_low / color_mid / color_high : int, optional
    Gradient stops (hex RGB).
palette : list[int] or None, optional
    Used by `categorical` and `discrete` variants for per-band colors.
bins : int, optional
    Number of discrete color bands (used by `discrete` variant).
widths : list[float] or None, optional
    Relative column widths (used by `unequal`).
ranges : list[float] or None, optional
    Relative row heights (used by `unequal`).
col_labels : list[str], optional
    Column labels. Defaults to row labels.
width / height : int, optional
    Canvas size in pixels.
x_label / y_label : str, optional
    Axis labels.
gridlines : bool, optional
    Show gridlines. Default False.
background : str or None, optional
    Background color. Default None.";

pub const DOC_BUILD_PIE_CHART: &str = "build_pie_chart(title, labels, values, *, show_pct=True, width=720, height=440, legend_position='right', palette=None, hover_json='', background=None) -> Chart

Pie chart.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Slice labels.
values : list[float]
    Slice values.
show_pct : bool, optional
    Show percentage labels. Default True.
width : int, optional
    Chart width in pixels. Default 720.
height : int, optional
    Chart height in pixels. Default 440.
legend_position : str, optional
    Legend position. Default 'right'.
palette : list[int], optional
    Custom color palette.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.
background : str or None, optional
    Background color. Default None.";

pub const DOC_BUILD_DONUT_CHART: &str = "build_donut_chart(title, labels, values, *, show_pct=True, inner_radius_ratio=0.55, width=720, height=440, legend_position='right', palette=None, hover_json='', background=None) -> Chart

Donut chart (pie with a hollow center).

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Slice labels.
values : list[float]
    Slice values.
show_pct : bool, optional
    Show percentage labels. Default True.
inner_radius_ratio : float, optional
    Ratio of inner to outer radius (0.0-1.0). Default 0.55.
width : int, optional
    Chart width in pixels. Default 720.
height : int, optional
    Chart height in pixels. Default 440.
legend_position : str, optional
    Legend position. Default 'right'.
palette : list[int], optional
    Custom color palette.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.
background : str or None, optional
    Background color. Default None.";

pub const DOC_BUILD_BOXPLOT: &str = "build_boxplot(title, category_labels, values, *, width=900, height=500, x_label='', y_label='', gridlines=False, palette=None, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

Box-and-whisker plot.

Parameters
----------
title : str
    Chart title.
category_labels : list[str]
    Category labels.
values : list[list[float]]
    One inner list of raw values per category.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 500.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_VIOLIN: &str = "build_violin(title, categories, values, *, width=900, height=500, x_label='', y_label='', gridlines=False, palette=None, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

Violin plot showing distribution density.

Parameters
----------
title : str
    Chart title.
categories : list[str]
    Category labels.
values : list[list[float]]
    One inner list of raw values per category.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 500.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_SLOPE: &str = "build_slope(title, labels, values_left, values_right, *, left_label='Before', right_label='After', show_text=True, width=700, height=500, palette=None, hover_json='', background=None) -> Chart

Slope chart comparing two states.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Item labels.
values_left : list[float]
    Left (before) values.
values_right : list[float]
    Right (after) values.
left_label : str, optional
    Label for the left axis. Default 'Before'.
right_label : str, optional
    Label for the right axis. Default 'After'.
show_text : bool, optional
    Show value labels. Default True.
width : int, optional
    Chart width in pixels. Default 700.
height : int, optional
    Chart height in pixels. Default 500.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.";

pub const DOC_BUILD_SUNBURST: &str = "build_sunburst(title, labels, parents, values, *, width=700, height=700, palette=None, hover_json='', background=None) -> Chart

Sunburst chart for hierarchical data.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Node labels.
parents : list[str]
    Parent label for each node ('' for root).
values : list[float]
    Node values.
width : int, optional
    Chart width in pixels. Default 700.
height : int, optional
    Chart height in pixels. Default 700.
palette : list[int], optional
    Custom color palette.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.
background : str or None, optional
    Background color. Default None.";

pub const DOC_BUILD_FUNNEL: &str = "build_funnel(title, labels, values, *, show_text=True, width=800, height=480, palette=None, hover_json='', background=None) -> Chart

Funnel chart for sequential stage data.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Stage labels.
values : list[float]
    Stage values (decreasing).
show_text : bool, optional
    Show value labels. Default True.
width : int, optional
    Chart width in pixels. Default 800.
height : int, optional
    Chart height in pixels. Default 480.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.";

pub const DOC_BUILD_TREEMAP: &str = "build_treemap(title, labels, values, *, parents=None, width=1100, height=520, palette=None, hover_json='', background=None) -> Chart

Treemap chart for nested proportional data.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Node labels.
values : list[float]
    Node values (determines rectangle size).
parents : list[str], optional
    Parent label for each node. None = flat layout.
width : int, optional
    Chart width in pixels. Default 1100.
height : int, optional
    Chart height in pixels. Default 520.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.";

pub const DOC_BUILD_MULTILINE_CHART: &str = "build_multiline_chart(title, x_labels, series_values, *, show_points=True, series_names=None, width=1100, height=480, x_label='', y_label='', gridlines=False, legend_position='right', palette=None, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

Multi-series line chart.

Parameters
----------
title : str
    Chart title.
x_labels : list[str]
    X-axis labels.
series_values : list[list[float]]
    One inner list per series.
show_points : bool, optional
    Show data point markers. Default True.
series_names : list[str], optional
    Name for each series.
width : int, optional
    Chart width in pixels. Default 1100.
height : int, optional
    Chart height in pixels. Default 480.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
legend_position : str, optional
    Legend position. Default 'right'.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_AREA_CHART: &str = "build_area_chart(title, x_labels, series_values, *, stacked=False, series_names=None, width=1100, height=480, x_label='', y_label='', gridlines=False, legend_position='right', palette=None, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

Area chart (filled line chart).

Parameters
----------
title : str
    Chart title.
x_labels : list[str]
    X-axis labels.
series_values : list[list[float]]
    One inner list per series.
stacked : bool, optional
    Stack the area series. Default False.
series_names : list[str], optional
    Name for each series.
width : int, optional
    Chart width in pixels. Default 1100.
height : int, optional
    Chart height in pixels. Default 480.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
legend_position : str, optional
    Legend position. Default 'right'.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_WATERFALL: &str = "build_waterfall(title, labels, values, *, show_text=True, width=900, height=480, x_label='', y_label='', gridlines=False, palette=None, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

Waterfall chart showing cumulative value changes.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Step labels.
values : list[float]
    Step changes (positive or negative).
show_text : bool, optional
    Show value labels. Default True.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 480.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_BULLET: &str = "build_bullet(title, labels, values, *, targets=None, max_vals=None, ranges=None, width=800, height=300, palette=None, hover_json='', background=None) -> Chart

Bullet chart for performance vs. target.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Metric labels.
values : list[float]
    Actual values.
targets : list[float], optional
    Target values.
max_vals : list[float], optional
    Maximum scale values.
ranges : list[list[float]], optional
    Background range bands per metric.
width : int, optional
    Chart width in pixels. Default 800.
height : int, optional
    Chart height in pixels. Default 300.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.";

pub const DOC_BUILD_BUBBLE_MAP: &str = "build_bubble_map(title, labels, values, *, width=1200, height=600, hover_json='', series_names=None, background=None, no_x_axis=False, no_y_axis=False) -> Chart

Bubble map with proportional circles.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Location or category labels.
values : list[float]
    Bubble size values.
width : int, optional
    Chart width in pixels. Default 1200.
height : int, optional
    Chart height in pixels. Default 600.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.
series_names : list[str], optional
    Series names.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_CHOROPLETH: &str = "build_choropleth(title, labels, values, *, width=1200, height=600, hover_json='', series_names=None, background=None, no_x_axis=False, no_y_axis=False) -> Chart

Choropleth map with filled regions.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Region labels.
values : list[float]
    Region values.
width : int, optional
    Chart width in pixels. Default 1200.
height : int, optional
    Chart height in pixels. Default 600.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.
series_names : list[str], optional
    Series names.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_SCATTER3D_CHART: &str = "build_scatter3d_chart(title, x_values, y_values, z_values, *, color_values=None, color_labels=None, series_names=None, bg_color='', width=900, height=560, x_label='', y_label='', z_label='Z', hover_json='') -> Chart

3D scatter plot.

Parameters
----------
title : str
    Chart title.
x_values : list[float]
    X coordinates.
y_values : list[float]
    Y coordinates.
z_values : list[float]
    Z coordinates.
color_values : list[float], optional
    Values used for color mapping.
color_labels : list[str], optional
    Color group labels.
series_names : list[str], optional
    Series names.
bg_color : str, optional
    Background color string. Default ''.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
z_label : str, optional
    Z-axis label. Default 'Z'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_BAR3D_CHART: &str = "build_bar3d_chart(title, x_values, y_values, z_values, *, color_values=None, color_labels=None, series_names=None, bg_color='', width=900, height=560, x_label='', y_label='', z_label='Z', hover_json='') -> Chart

3D bar chart.

Parameters
----------
title : str
    Chart title.
x_values : list[float]
    X positions.
y_values : list[float]
    Y positions.
z_values : list[float]
    Bar heights.
color_values : list[float], optional
    Values used for color mapping.
color_labels : list[str], optional
    Color group labels.
series_names : list[str], optional
    Series names.
bg_color : str, optional
    Background color string. Default ''.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
z_label : str, optional
    Z-axis label. Default 'Z'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_LINE3D_CHART: &str = "build_line3d_chart(title, x_values, y_values, z_values, *, color_values=None, color_labels=None, series_names=None, bg_color='', width=900, height=560, x_label='', y_label='', z_label='Z', hover_json='') -> Chart

3D line chart.

Parameters
----------
title : str
    Chart title.
x_values : list[float]
    X coordinates.
y_values : list[float]
    Y coordinates.
z_values : list[float]
    Z coordinates.
color_values : list[float], optional
    Values used for color mapping.
color_labels : list[str], optional
    Color group labels.
series_names : list[str], optional
    Series names.
bg_color : str, optional
    Background color string. Default ''.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
z_label : str, optional
    Z-axis label. Default 'Z'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_RADAR_CHART: &str = "build_radar_chart(title, axes, series_values, *, series_names=None, filled=True, fill_opacity=50, width=700, height=560, palette=None, hover_json='', background=None) -> Chart

2D radar (spider) chart.

Parameters
----------
title : str
    Chart title.
axes : list[str]
    Axis labels (dimensions).
series_values : list[list[float]]
    One inner list per series with a value per axis.
series_names : list[str], optional
    Name for each series.
filled : bool, optional
    Fill the radar polygon. Default True.
fill_opacity : int, optional
    Fill opacity 0-100. Default 50.
width : int, optional
    Chart width in pixels. Default 700.
height : int, optional
    Chart height in pixels. Default 560.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.";

pub const DOC_BUILD_RADAR3D_CHART: &str = "build_radar3d_chart(title, axes, series_values, *, series_names=None, width=700, height=560, x_label='', y_label='', z_label='Z', hover_json='') -> Chart

3D radar chart.

Parameters
----------
title : str
    Chart title.
axes : list[str]
    Axis labels.
series_values : list[list[float]]
    One inner list per series.
series_names : list[str], optional
    Series names.
width : int, optional
    Chart width in pixels. Default 700.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
z_label : str, optional
    Z-axis label. Default 'Z'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_LOLLIPOP_CHART: &str = "build_lollipop_chart(title, labels, values, *, color_hex=0, orientation='v', show_values=False, width=900, height=480, x_label='', y_label='', gridlines=False, palette=None, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

Lollipop chart (line + circle markers).

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Category labels.
values : list[float]
    Values.
color_hex : int, optional
    Color as hex integer. Default 0 (palette).
orientation : str, optional
    'v' (vertical) or 'h' (horizontal). Default 'v'.
show_values : bool, optional
    Show value labels. Default False.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 480.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_LOLLIPOP3D_CHART: &str = "build_lollipop3d_chart(title, x_values, y_values, z_values, *, color_labels=None, series_names=None, width=900, height=560, x_label='', y_label='', z_label='Z', hover_json='') -> Chart

3D lollipop chart.

Parameters
----------
title : str
    Chart title.
x_values : list[float]
    X positions.
y_values : list[float]
    Y positions.
z_values : list[float]
    Z values (heights).
color_labels : list[str], optional
    Color group labels.
series_names : list[str], optional
    Series names.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
z_label : str, optional
    Z-axis label. Default 'Z'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_KDE_CHART: &str = "build_kde_chart(title, values, *, categories=None, filled=True, fill_opacity=50, bandwidth=0.0, width=900, height=420, x_label='', y_label='Density', gridlines=False, palette=None, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

Kernel density estimate (KDE) chart.

Parameters
----------
title : str
    Chart title.
values : list[list[float]]
    One inner list per category/series.
categories : list[str], optional
    Category names for each series.
filled : bool, optional
    Fill under the density curve. Default True.
fill_opacity : int, optional
    Fill opacity 0-100. Default 50.
bandwidth : float, optional
    KDE bandwidth. 0.0 = auto. Default 0.0.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 420.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default 'Density'.
gridlines : bool, optional
    Show gridlines. Default False.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_KDE3D_CHART: &str = "build_kde3d_chart(title, values, *, categories=None, series_names=None, width=900, height=560, x_label='', y_label='Category', z_label='Density', hover_json='') -> Chart

3D kernel density estimate chart.

Parameters
----------
title : str
    Chart title.
values : list[list[float]]
    One inner list per category/series.
categories : list[str], optional
    Category names.
series_names : list[str], optional
    Series names.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default 'Category'.
z_label : str, optional
    Z-axis label. Default 'Density'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_RIDGELINE_CHART: &str = "build_ridgeline_chart(title, values, categories, *, overlap=0.5, bandwidth=0.0, width=900, height=520, x_label='', y_label='', gridlines=False, palette=None, hover_json='', background=None) -> Chart

Ridgeline (joy plot) chart for comparing distributions.

Parameters
----------
title : str
    Chart title.
values : list[list[float]]
    One inner list of values per category.
categories : list[str]
    Category names (one per row).
overlap : float, optional
    Row overlap factor. Default 0.5.
bandwidth : float, optional
    KDE bandwidth. 0.0 = auto. Default 0.0.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 520.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.";

pub const DOC_BUILD_RIDGELINE3D_CHART: &str = "build_ridgeline3d_chart(title, values, categories, *, series_names=None, width=900, height=560, x_label='', y_label='Category', z_label='Density', hover_json='') -> Chart

3D ridgeline chart.

Parameters
----------
title : str
    Chart title.
values : list[list[float]]
    One inner list of values per category.
categories : list[str]
    Category names.
series_names : list[str], optional
    Series names.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default 'Category'.
z_label : str, optional
    Z-axis label. Default 'Density'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_BUBBLE3D_CHART: &str = "build_bubble3d_chart(title, x_values, y_values, z_values, size_values, *, color_values=None, color_labels=None, series_names=None, width=900, height=560, x_label='', y_label='', z_label='Z', hover_json='') -> Chart

3D bubble chart (scatter with variable bubble size).

Parameters
----------
title : str
    Chart title.
x_values : list[float]
    X coordinates.
y_values : list[float]
    Y coordinates.
z_values : list[float]
    Z coordinates.
size_values : list[float]
    Bubble sizes.
color_values : list[float], optional
    Values used for color mapping.
color_labels : list[str], optional
    Color group labels.
series_names : list[str], optional
    Series names.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
z_label : str, optional
    Z-axis label. Default 'Z'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_PIE3D_CHART: &str = "build_pie3d_chart(title, labels, values, *, series_names=None, bg_color='', sort_order='none', width=700, height=560, palette=None, hover_json='') -> Chart

3D pie chart.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Slice labels.
values : list[float]
    Slice values.
series_names : list[str], optional
    Series names.
bg_color : str, optional
    Background color string. Default ''.
sort_order : str, optional
    Sort slices: 'none', 'asc', or 'desc'. Default 'none'.
width : int, optional
    Chart width in pixels. Default 700.
height : int, optional
    Chart height in pixels. Default 560.
palette : list[int], optional
    Custom color palette.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_VIOLIN3D_CHART: &str = "build_violin3d_chart(title, values, *, categories=None, series_names=None, width=900, height=560, x_label='Value', y_label='Category', z_label='Density', hover_json='') -> Chart

3D violin plot.

Parameters
----------
title : str
    Chart title.
values : list[list[float]]
    One inner list of raw values per category.
categories : list[str], optional
    Category names.
series_names : list[str], optional
    Series names.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default 'Value'.
y_label : str, optional
    Y-axis label. Default 'Category'.
z_label : str, optional
    Z-axis label. Default 'Density'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_HEATMAP3D_CHART: &str = "build_heatmap3d_chart(title, x_labels, y_labels, values, *, series_names=None, bg_color='', width=900, height=560, x_label='', y_label='', z_label='Z', hover_json='') -> Chart

3D heatmap.

Parameters
----------
title : str
    Chart title.
x_labels : list[str]
    Column labels.
y_labels : list[str]
    Row labels.
values : list[float]
    Flattened matrix values (row-major order).
series_names : list[str], optional
    Series names.
bg_color : str, optional
    Background color string. Default ''.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
z_label : str, optional
    Z-axis label. Default 'Z'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_CANDLESTICK3D_CHART: &str = "build_candlestick3d_chart(title, labels, open, high, low, close, *, series_names=None, bg_color='', width=900, height=560, x_label='Price', y_label='Bar', z_label='', hover_json='') -> Chart

3D candlestick chart for OHLC financial data.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Time period labels.
open : list[float]
    Open prices.
high : list[float]
    High prices.
low : list[float]
    Low prices.
close : list[float]
    Close prices.
series_names : list[str], optional
    Series names.
bg_color : str, optional
    Background color. Default ''.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default 'Price'.
y_label : str, optional
    Y-axis label. Default 'Bar'.
z_label : str, optional
    Z-axis label. Default ''.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_DUMBBELL3D_CHART: &str = "build_dumbbell3d_chart(title, labels, values_start, values_end, *, series_names=None, bg_color='', width=900, height=560, x_label='Start', y_label='Item', z_label='End', hover_json='') -> Chart

3D dumbbell chart comparing two values per item.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Item labels.
values_start : list[float]
    Start values.
values_end : list[float]
    End values.
series_names : list[str], optional
    Series names.
bg_color : str, optional
    Background color. Default ''.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default 'Start'.
y_label : str, optional
    Y-axis label. Default 'Item'.
z_label : str, optional
    Z-axis label. Default 'End'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_FUNNEL3D_CHART: &str = "build_funnel3d_chart(title, labels, values, *, series_names=None, bg_color='', sort_order='none', width=700, height=560, hover_json='') -> Chart

3D funnel chart.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Stage labels.
values : list[float]
    Stage values.
series_names : list[str], optional
    Series names.
bg_color : str, optional
    Background color. Default ''.
sort_order : str, optional
    Sort: 'none', 'asc', or 'desc'. Default 'none'.
width : int, optional
    Chart width in pixels. Default 700.
height : int, optional
    Chart height in pixels. Default 560.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_SUNBURST3D_CHART: &str = "build_sunburst3d_chart(title, labels, parents, values, *, series_names=None, bg_color='', width=700, height=560, hover_json='') -> Chart

3D sunburst chart.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Node labels.
parents : list[str]
    Parent label for each node ('' for root).
values : list[float]
    Node values.
series_names : list[str], optional
    Series names.
bg_color : str, optional
    Background color. Default ''.
width : int, optional
    Chart width in pixels. Default 700.
height : int, optional
    Chart height in pixels. Default 560.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_STACKED_BAR3D_CHART: &str = "build_stacked_bar3d_chart(title, category_labels, series_values, *, series_names=None, bg_color='', width=900, height=560, x_label='Category', y_label='Series', z_label='Value', hover_json='') -> Chart

3D stacked bar chart.

Parameters
----------
title : str
    Chart title.
category_labels : list[str]
    Category labels.
series_values : list[list[float]]
    One inner list per series.
series_names : list[str], optional
    Series names.
bg_color : str, optional
    Background color. Default ''.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 560.
x_label : str, optional
    X-axis label. Default 'Category'.
y_label : str, optional
    Y-axis label. Default 'Series'.
z_label : str, optional
    Z-axis label. Default 'Value'.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_GLOBE3D_CHART: &str = "build_globe3d_chart(title, latitudes, longitudes, values, *, labels=None, series_names=None, bg_color='', width=800, height=600, hover_json='') -> Chart

3D globe map with bubble markers.

Parameters
----------
title : str
    Chart title.
latitudes : list[float]
    Latitude coordinates in degrees.
longitudes : list[float]
    Longitude coordinates in degrees.
values : list[float]
    Marker size values.
labels : list[str], optional
    Per-point labels.
series_names : list[str], optional
    Series names.
bg_color : str, optional
    Background color. Default ''.
width : int, optional
    Chart width in pixels. Default 800.
height : int, optional
    Chart height in pixels. Default 600.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.";

pub const DOC_BUILD_WORDCLOUD: &str = "build_wordcloud(title, words, frequencies, *, min_font=12.0, max_font=72.0, background=None, width=900, height=500, sort_order='none', hover_json='', palette=None, series_names=None) -> Chart

Word cloud chart.

Parameters
----------
title : str
    Chart title.
words : list[str]
    Word labels.
frequencies : list[float]
    Word frequency/weight values.
min_font : float, optional
    Minimum font size in pixels. Default 12.0.
max_font : float, optional
    Maximum font size in pixels. Default 72.0.
background : str or None, optional
    Background color. Default None.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 500.
sort_order : str, optional
    Sort words: 'none', 'asc', or 'desc'. Default 'none'.
palette : list[int], optional
    Custom color palette.";

pub const DOC_BUILD_CANDLESTICK: &str = "build_candlestick(title, labels, open, high, low, close, *, width=1100, height=500, x_label='', y_label='Price', gridlines=False, background=None, no_x_axis=False, no_y_axis=False) -> Chart

Candlestick chart for OHLC financial data.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Time period labels.
open : list[float]
    Open prices.
high : list[float]
    High prices.
low : list[float]
    Low prices.
close : list[float]
    Close prices.
width : int, optional
    Chart width in pixels. Default 1100.
height : int, optional
    Chart height in pixels. Default 500.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default 'Price'.
gridlines : bool, optional
    Show gridlines. Default False.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_DUMBBELL: &str = "build_dumbbell(title, labels, values_start, values_end, *, series_names=None, width=1000, height=500, x_label='', y_label='', gridlines=False, palette=None, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

Dumbbell chart comparing two values per item.

Parameters
----------
title : str
    Chart title.
labels : list[str]
    Item labels.
values_start : list[float]
    Start values.
values_end : list[float]
    End values.
series_names : list[str], optional
    Series names.
width : int, optional
    Chart width in pixels. Default 1000.
height : int, optional
    Chart height in pixels. Default 500.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_BUBBLE: &str = "build_bubble(title, x_values, y_values, sizes, *, variant='basic', categories=None, labels=None, color_values=None, color_hex=0, color_low=0x636EFA, color_high=0xF43F5E, min_size=4.0, max_size=40.0, show_text=False, stroke_width=1.5, width=900, height=500, x_label='', y_label='', gridlines=False, sort_order='none', legend_position='right', palette=None, hover_json='', background=None, no_x_axis=False, no_y_axis=False) -> Chart

2D bubble chart family. The `variant` keyword selects the rendering strategy.

Variants: 'basic' | 'categorical'/'grouped' | 'gradient'/'colorscale' |
'labeled'/'text' | 'outlined'/'hollow' | 'negative'/'signed'.

Parameters
----------
title : str
    Chart title.
x_values : list[float]
    X coordinates.
y_values : list[float]
    Y coordinates.
sizes : list[float]
    Bubble sizes (mapped to bubble area; signed in 'negative' variant).
variant : str, optional
    Rendering variant. Default 'basic'.
categories : list[str], optional
    Per-point group name for color partitioning ('categorical' / 'outlined').
labels : list[str], optional
    Per-point text label, shown in tooltip and rendered in 'labeled' variant.
color_values : list[float], optional
    Per-point continuous value driving 'gradient' colorscale.
color_hex : int, optional
    Single fill color when no group/gradient applies. Default 0 (auto).
color_low : int, optional
    Gradient/diverging low color. Default 0x636EFA.
color_high : int, optional
    Gradient/diverging high color. Default 0xF43F5E.
min_size : float, optional
    Minimum bubble radius in pixels. Default 4.0.
max_size : float, optional
    Maximum bubble radius in pixels. Default 40.0.
show_text : bool, optional
    Force always-on text labels. Default False.
stroke_width : float, optional
    Bubble stroke width in pixels. Default 1.5.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 500.
x_label : str, optional
    X-axis label. Default ''.
y_label : str, optional
    Y-axis label. Default ''.
gridlines : bool, optional
    Show gridlines. Default False.
sort_order : str, optional
    'asc' | 'desc' | 'none' — controls draw order by absolute size. Default 'none'.
legend_position : str, optional
    'right' | 'left' | 'top' | 'bottom'. Default 'right'.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_GAUGE: &str = "build_gauge(title, value, *, min_val=0.0, max_val=100.0, label='', width=400, height=300, hover_json='', palette=None, series_names=None, background=None, no_x_axis=False, no_y_axis=False) -> Chart

Gauge chart.

Parameters
----------
title : str
    Chart title.
value : float
    Current gauge value.
min_val : float, optional
    Minimum scale value. Default 0.0.
max_val : float, optional
    Maximum scale value. Default 100.0.
label : str, optional
    Value label. Default ''.
width : int, optional
    Chart width in pixels. Default 400.
height : int, optional
    Chart height in pixels. Default 300.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Ignored for gauge charts. Default False.
no_y_axis : bool, optional
    Ignored for gauge charts. Default False.";

pub const DOC_BUILD_PARALLEL: &str = "build_parallel(title, axes, series_values, *, series_names=None, width=1000, height=500, hover_json='', legend_position='right', palette=None, background=None, no_x_axis=False, no_y_axis=False) -> Chart

Parallel coordinates chart for multivariate data.

Parameters
----------
title : str
    Chart title.
axes : list[str]
    Axis labels (dimensions).
series_values : list[list[float]]
    One inner list per observation with a value per axis.
series_names : list[str], optional
    Series names.
width : int, optional
    Chart width in pixels. Default 1000.
height : int, optional
    Chart height in pixels. Default 500.
hover_json : str, optional
    Custom hover tooltip JSON. Default ''.
legend_position : str, optional
    Legend position. Default 'right'.
palette : list[int], optional
    Custom color palette.
background : str or None, optional
    Background color. Default None.
no_x_axis : bool, optional
    Hide the X axis. Default False.
no_y_axis : bool, optional
    Hide the Y axis. Default False.";

pub const DOC_BUILD_GRID: &str =
    "build_grid(charts, cols=2, gap=16, bg=None, cell_height=520) -> Chart

Arrange multiple charts in a responsive grid layout.

Parameters
----------
charts : list[Chart]
    Chart objects to arrange.
cols : int, optional
    Number of grid columns. Default 2.
gap : int, optional
    Gap between cells in pixels. Default 16.
bg : str or None, optional
    Grid background color. Default None.
cell_height : int, optional
    Height of each cell in pixels. Default 520.";

pub const DOC_BUILD_SLIDESHOW: &str =
    "build_slideshow(charts, interval_ms=2500, title='', width=900, height=520) -> Chart

Slideshow that cycles through multiple charts.

Parameters
----------
charts : list[Chart]
    Chart objects to cycle through.
interval_ms : int, optional
    Milliseconds between slides. Default 2500.
title : str, optional
    Slideshow title. Default ''.
width : int, optional
    Chart width in pixels. Default 900.
height : int, optional
    Chart height in pixels. Default 520.";

pub const DOC_DBSCAN_MODEL: &str = "DBSCAN(eps=0.5, min_samples=5)

Density-Based Spatial Clustering of Applications with Noise.
Supports N-dimensional data. Uses a fast grid CSR engine for 2D data.

Parameters
----------
eps : float, optional
    Neighborhood radius. Default 0.5.
min_samples : int, optional
    Minimum points to form a dense region. Default 5.

Methods
-------
fit(X)
    Fit the model. X is list[list[float]] or ndarray.
fit_predict(X) -> list[int]
    Fit and return cluster labels. -1 = noise.

Attributes
----------
labels_ : list[int]
    Cluster assignment per sample. -1 = noise.
n_clusters_ : int
    Number of discovered clusters.
n_noise_ : int
    Number of noise points.";

pub const DOC_KMEANS_MODEL: &str =
    "KMeans(k=3, max_iter=300, tol=1e-4, mini_batch=False, batch_size=1000, n_init=10)

K-Means clustering algorithm with optional Mini-Batch variant.

Parameters
----------
k : int, optional
    Number of clusters. Default 3.
max_iter : int, optional
    Maximum iterations. Default 300.
tol : float, optional
    Convergence tolerance. Default 1e-4.
mini_batch : bool, optional
    Use Mini-Batch K-Means. Default False.
batch_size : int, optional
    Batch size for mini-batch mode. Default 1000.
n_init : int, optional
    Number of initializations. Default 10.

Methods
-------
fit(X)
    Fit the model.
predict(X) -> list[int]
    Predict cluster labels.
fit_predict(X) -> list[int]
    Fit and return cluster labels.
transform(X) -> ndarray
    Transform X to cluster distance space.

Attributes
----------
labels_ : list[int]
    Cluster assignment per training sample.
cluster_centers_ : ndarray
    Coordinates of cluster centroids.
inertia_ : float
    Sum of squared distances to nearest centroid.
n_iter_ : int
    Number of iterations run.";

pub const DOC_LINEAR_REGRESSION: &str = "LinearRegression(fit_intercept=True)

Ordinary least squares (OLS) linear regression.

Parameters
----------
fit_intercept : bool, optional
    Fit an intercept term. Default True.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[float]
    Predict target values.
score(X, y) -> float
    Return R^2 coefficient of determination.

Attributes
----------
coef_ : list[float]
    Estimated coefficients.
intercept_ : float
    Intercept term.
n_features_in_ : int
    Number of input features.";

pub const DOC_RIDGE: &str = "Ridge(alpha=1.0, fit_intercept=True)

L2-regularized linear regression.

Parameters
----------
alpha : float, optional
    Regularization strength. Default 1.0.
fit_intercept : bool, optional
    Fit an intercept term. Default True.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[float]
    Predict target values.
score(X, y) -> float
    Return R^2.

Attributes
----------
coef_ : list[float]
    Estimated coefficients.
intercept_ : float
    Intercept term.
alpha_ : float
    Regularization parameter.";

pub const DOC_LASSO: &str = "Lasso(alpha=1.0, max_iter=1000, tol=1e-4, fit_intercept=True)

L1-regularized linear regression (Lasso).

Parameters
----------
alpha : float, optional
    Regularization strength. Default 1.0.
max_iter : int, optional
    Maximum iterations. Default 1000.
tol : float, optional
    Convergence tolerance. Default 1e-4.
fit_intercept : bool, optional
    Fit an intercept term. Default True.

Methods
-------
fit(X, y, checkpoint_id=None)
    Fit the model.
predict(X) -> list[float]
    Predict target values.
score(X, y) -> float
    Return R^2.

Attributes
----------
coef_ : list[float]
    Estimated coefficients.
intercept_ : float
    Intercept term.
n_iter_ : int
    Number of iterations run.";

pub const DOC_ELASTIC_NET: &str =
    "ElasticNet(alpha=1.0, l1_ratio=0.5, max_iter=1000, tol=1e-4, fit_intercept=True)

Combined L1/L2 regularized linear regression.

Parameters
----------
alpha : float, optional
    Regularization strength. Default 1.0.
l1_ratio : float, optional
    L1/L2 mix ratio (0=Ridge, 1=Lasso). Default 0.5.
max_iter : int, optional
    Maximum iterations. Default 1000.
tol : float, optional
    Convergence tolerance. Default 1e-4.
fit_intercept : bool, optional
    Fit an intercept term. Default True.

Methods
-------
fit(X, y, checkpoint_id=None)
    Fit the model.
predict(X) -> list[float]
    Predict target values.
score(X, y) -> float
    Return R^2.

Attributes
----------
coef_ : list[float]
    Estimated coefficients.
intercept_ : float
    Intercept term.
n_iter_ : int
    Number of iterations run.";

pub const DOC_LOGISTIC_REGRESSION: &str =
    "LogisticRegression(c=1.0, max_iter=1000, tol=1e-4, fit_intercept=True)

Logistic regression for binary and multi-class classification.

Parameters
----------
c : float, optional
    Inverse regularization strength (higher = less regularization). Default 1.0.
max_iter : int, optional
    Maximum iterations. Default 1000.
tol : float, optional
    Convergence tolerance. Default 1e-4.
fit_intercept : bool, optional
    Fit an intercept term. Default True.

Methods
-------
fit(X, y, checkpoint_id=None)
    Fit the model.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.

Attributes
----------
coef_ : list[float]
    Estimated coefficients.
intercept_ : float
    Intercept term.";

pub const DOC_SGD_CLASSIFIER: &str =
    "SGDClassifier(loss='hinge', alpha=0.0001, max_iter=1000, tol=1e-3, fit_intercept=True)

Stochastic gradient descent classifier.

Parameters
----------
loss : str, optional
    Loss function: 'hinge', 'log', 'modified_huber', etc. Default 'hinge'.
alpha : float, optional
    Regularization term. Default 0.0001.
max_iter : int, optional
    Maximum passes over the training data. Default 1000.
tol : float, optional
    Stopping criterion. Default 1e-3.
fit_intercept : bool, optional
    Fit an intercept term. Default True.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.";

pub const DOC_SGD_REGRESSOR: &str =
    "SGDRegressor(loss='squared_error', alpha=0.0001, max_iter=1000, tol=1e-3, fit_intercept=True)

Stochastic gradient descent regressor.

Parameters
----------
loss : str, optional
    Loss function: 'squared_error', 'huber', etc. Default 'squared_error'.
alpha : float, optional
    Regularization term. Default 0.0001.
max_iter : int, optional
    Maximum passes over the training data. Default 1000.
tol : float, optional
    Stopping criterion. Default 1e-3.
fit_intercept : bool, optional
    Fit an intercept term. Default True.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[float]
    Predict target values.
score(X, y) -> float
    Return R^2.";

pub const DOC_DECISION_TREE_CLASSIFIER: &str = "DecisionTreeClassifier(max_depth=None, min_samples_split=2, min_samples_leaf=1, criterion='gini')

Decision tree classifier.

Parameters
----------
max_depth : int or None, optional
    Maximum tree depth. None = unlimited. Default None.
min_samples_split : int, optional
    Minimum samples required to split a node. Default 2.
min_samples_leaf : int, optional
    Minimum samples required at a leaf. Default 1.
criterion : str, optional
    Split quality: 'gini' or 'entropy'. Default 'gini'.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.

Attributes
----------
feature_importances_ : list[float]
    Normalized feature importance values.
n_classes_ : int
    Number of classes.";

pub const DOC_DECISION_TREE_REGRESSOR: &str = "DecisionTreeRegressor(max_depth=None, min_samples_split=2, min_samples_leaf=1, criterion='mse')

Decision tree regressor.

Parameters
----------
max_depth : int or None, optional
    Maximum tree depth. None = unlimited. Default None.
min_samples_split : int, optional
    Minimum samples required to split a node. Default 2.
min_samples_leaf : int, optional
    Minimum samples required at a leaf. Default 1.
criterion : str, optional
    Split quality: 'mse' or 'mae'. Default 'mse'.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[float]
    Predict target values.
score(X, y) -> float
    Return R^2.

Attributes
----------
feature_importances_ : list[float]
    Normalized feature importance values.";

pub const DOC_RANDOM_FOREST_CLASSIFIER: &str = "RandomForestClassifier(n_estimators=100, max_depth=None, min_samples_split=2, min_samples_leaf=1, max_features='sqrt', n_jobs=-1)

Random forest classifier (ensemble of decision trees).

Parameters
----------
n_estimators : int, optional
    Number of trees. Default 100.
max_depth : int or None, optional
    Maximum tree depth. None = unlimited. Default None.
min_samples_split : int, optional
    Minimum samples to split a node. Default 2.
min_samples_leaf : int, optional
    Minimum samples at a leaf. Default 1.
max_features : str or float, optional
    Features per split: 'sqrt', 'log2', or a float. Default 'sqrt'.
n_jobs : int, optional
    Number of parallel jobs. -1 = all CPUs. Default -1.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.

Attributes
----------
feature_importances_ : list[float]
    Mean feature importance across trees.";

pub const DOC_RANDOM_FOREST_REGRESSOR: &str = "RandomForestRegressor(n_estimators=100, max_depth=None, min_samples_split=2, min_samples_leaf=1, max_features='sqrt', n_jobs=-1)

Random forest regressor (ensemble of decision trees).

Parameters
----------
n_estimators : int, optional
    Number of trees. Default 100.
max_depth : int or None, optional
    Maximum tree depth. None = unlimited. Default None.
min_samples_split : int, optional
    Minimum samples to split a node. Default 2.
min_samples_leaf : int, optional
    Minimum samples at a leaf. Default 1.
max_features : str or float, optional
    Features per split. Default 'sqrt'.
n_jobs : int, optional
    Number of parallel jobs. Default -1.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[float]
    Predict target values.
score(X, y) -> float
    Return R^2.

Attributes
----------
feature_importances_ : list[float]
    Mean feature importance across trees.";

pub const DOC_GRADIENT_BOOSTING_CLASSIFIER: &str = "GradientBoostingClassifier(n_estimators=100, learning_rate=0.1, max_depth=3, subsample=1.0, min_samples_split=2, min_samples_leaf=1)

Gradient boosting classifier.

Parameters
----------
n_estimators : int, optional
    Number of boosting stages. Default 100.
learning_rate : float, optional
    Shrinkage factor per stage. Default 0.1.
max_depth : int, optional
    Maximum tree depth. Default 3.
subsample : float, optional
    Fraction of samples per stage. Default 1.0.
min_samples_split : int, optional
    Minimum samples to split a node. Default 2.
min_samples_leaf : int, optional
    Minimum samples at a leaf. Default 1.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.";

pub const DOC_GRADIENT_BOOSTING_REGRESSOR: &str = "GradientBoostingRegressor(n_estimators=100, learning_rate=0.1, max_depth=3, subsample=1.0, min_samples_split=2, min_samples_leaf=1)

Gradient boosting regressor.

Parameters
----------
n_estimators : int, optional
    Number of boosting stages. Default 100.
learning_rate : float, optional
    Shrinkage factor per stage. Default 0.1.
max_depth : int, optional
    Maximum tree depth. Default 3.
subsample : float, optional
    Fraction of samples per stage. Default 1.0.
min_samples_split : int, optional
    Minimum samples to split a node. Default 2.
min_samples_leaf : int, optional
    Minimum samples at a leaf. Default 1.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[float]
    Predict target values.
score(X, y) -> float
    Return R^2.";

pub const DOC_ADABOOST_CLASSIFIER: &str =
    "AdaBoostClassifier(n_estimators=50, learning_rate=1.0, max_depth=1)

AdaBoost classifier.

Parameters
----------
n_estimators : int, optional
    Number of weak learners. Default 50.
learning_rate : float, optional
    Contribution of each weak learner. Default 1.0.
max_depth : int, optional
    Depth of each decision tree base learner. Default 1.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.";

pub const DOC_ADABOOST_REGRESSOR: &str =
    "AdaBoostRegressor(n_estimators=50, learning_rate=1.0, max_depth=3, loss='linear')

AdaBoost regressor.

Parameters
----------
n_estimators : int, optional
    Number of weak learners. Default 50.
learning_rate : float, optional
    Contribution of each weak learner. Default 1.0.
max_depth : int, optional
    Depth of each decision tree base learner. Default 3.
loss : str, optional
    Loss function: 'linear', 'square', or 'exponential'. Default 'linear'.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[float]
    Predict target values.
score(X, y) -> float
    Return R^2.";

pub const DOC_KNEIGHBORS_CLASSIFIER: &str = "KNeighborsClassifier(n_neighbors=5, weights='uniform', algorithm='auto', leaf_size=30, n_jobs=1)

K-nearest neighbors classifier.

Parameters
----------
n_neighbors : int, optional
    Number of neighbors. Default 5.
weights : str, optional
    Weight function: 'uniform' or 'distance'. Default 'uniform'.
algorithm : str, optional
    Algorithm: 'auto', 'ball_tree', 'kd_tree', or 'brute'. Default 'auto'.
leaf_size : int, optional
    Leaf size for BallTree/KDTree. Default 30.
n_jobs : int, optional
    Parallel jobs. Default 1.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.";

pub const DOC_KNEIGHBORS_REGRESSOR: &str = "KNeighborsRegressor(n_neighbors=5, weights='uniform', algorithm='auto', leaf_size=30, n_jobs=1)

K-nearest neighbors regressor.

Parameters
----------
n_neighbors : int, optional
    Number of neighbors. Default 5.
weights : str, optional
    Weight function: 'uniform' or 'distance'. Default 'uniform'.
algorithm : str, optional
    Algorithm: 'auto', 'ball_tree', 'kd_tree', or 'brute'. Default 'auto'.
leaf_size : int, optional
    Leaf size for BallTree/KDTree. Default 30.
n_jobs : int, optional
    Parallel jobs. Default 1.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[float]
    Predict target values.
score(X, y) -> float
    Return R^2.";

pub const DOC_NEAREST_CENTROID: &str = "NearestCentroid(metric='euclidean')

Nearest centroid classifier.

Parameters
----------
metric : str, optional
    Distance metric: 'euclidean' or 'manhattan'. Default 'euclidean'.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.

Attributes
----------
centroids_ : ndarray
    Class centroid coordinates.
classes_ : list[int]
    Unique class labels seen during fit.";

pub const DOC_GAUSSIAN_NB: &str = "GaussianNB(var_smoothing=1e-9)

Naive Bayes classifier with Gaussian likelihood.

Parameters
----------
var_smoothing : float, optional
    Variance smoothing added to variance for numerical stability. Default 1e-9.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.";

pub const DOC_MULTINOMIAL_NB: &str = "MultinomialNB(alpha=1.0)

Multinomial Naive Bayes for discrete count features.

Parameters
----------
alpha : float, optional
    Laplace/Lidstone smoothing. Default 1.0.

Methods
-------
fit(X, y)
    Fit the model. X must contain non-negative values.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.";

pub const DOC_BERNOULLI_NB: &str = "BernoulliNB(alpha=1.0, binarize=0.0)

Bernoulli Naive Bayes for binary/boolean features.

Parameters
----------
alpha : float, optional
    Laplace/Lidstone smoothing. Default 1.0.
binarize : float, optional
    Threshold for binarizing features. Default 0.0.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.";

pub const DOC_LINEAR_SVC: &str = "LinearSVC(c=1.0, max_iter=1000, tol=1e-4, fit_intercept=True)

Linear support vector classifier.

Parameters
----------
c : float, optional
    Regularization parameter. Default 1.0.
max_iter : int, optional
    Maximum iterations. Default 1000.
tol : float, optional
    Convergence tolerance. Default 1e-4.
fit_intercept : bool, optional
    Fit an intercept term. Default True.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.

Attributes
----------
coef_ : list[float]
    Weight vector.
intercept_ : float
    Intercept term.";

pub const DOC_LINEAR_SVR: &str =
    "LinearSVR(c=1.0, epsilon=0.0, max_iter=1000, tol=1e-4, fit_intercept=True)

Linear support vector regressor.

Parameters
----------
c : float, optional
    Regularization parameter. Default 1.0.
epsilon : float, optional
    Epsilon in the epsilon-insensitive loss. Default 0.0.
max_iter : int, optional
    Maximum iterations. Default 1000.
tol : float, optional
    Convergence tolerance. Default 1e-4.
fit_intercept : bool, optional
    Fit an intercept term. Default True.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[float]
    Predict target values.
score(X, y) -> float
    Return R^2.

Attributes
----------
coef_ : list[float]
    Weight vector.
intercept_ : float
    Intercept term.";

pub const DOC_RIDGE_CLASSIFIER: &str = "RidgeClassifier(alpha=1.0, fit_intercept=True)

Ridge regression adapted for classification.

Parameters
----------
alpha : float, optional
    Regularization strength. Default 1.0.
fit_intercept : bool, optional
    Fit an intercept term. Default True.

Methods
-------
fit(X, y)
    Fit the model.
predict(X) -> list[int]
    Predict class labels.
score(X, y) -> float
    Return accuracy.

Attributes
----------
coef_ : list[float]
    Estimated coefficients.
intercept_ : float
    Intercept term.";
