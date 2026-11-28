# API Reference — Data & Chart Methods

<div class="lang-en">

Auto-generated from the framework's own inline documentation macros (314 entries) — run `python v2/examples/gen_api_reference.py` to regenerate after adding or documenting a method. Chart builder functions (2D/3D/map) and ML estimators are covered separately in [API Reference](api.md).

## SeraDFrame / Table methods (168)

### `DFrameBuilder`

Chainable column-by-column dataframe builder.

### `SeraDFrame`

Columnar Rust-native dataframe built from a dict of columns.

### `SeraDFrame.abs`

Absolute value of every numeric column.

### `SeraDFrame.agg`

Whole-frame aggregation into a single-row SeraDFrame.

### `SeraDFrame.align`

Aligns two SeraDFrames onto the union of their columns, filling gaps.

### `SeraDFrame.all`

True if every value in a column is truthy.

### `SeraDFrame.any`

True if any value in a column is truthy.

### `SeraDFrame.apply`

Calls a Python function per value in a column.

### `SeraDFrame.applymap`

Calls a Python function on every cell of every column.

### `SeraDFrame.assign`

Adds or replaces a column.

### `SeraDFrame.astype`

Casts a column to float/str/bool.

### `SeraDFrame.between`

Keeps rows within an inclusive numeric range.

### `SeraDFrame.bfill`

Back-fills missing values with the next valid observation.

### `SeraDFrame.bin`

Adds {col}_bin, an equal-width bin index in 0..n_bins.

### `SeraDFrame.clip`

Clamps a numeric column between two bounds.

### `SeraDFrame.columns`

Lists column names in order.

### `SeraDFrame.combine_first`

Fills this frame's missing values from the other frame, row by row.

### `SeraDFrame.compare`

Boolean mask of shared columns showing where two SeraDFrames differ.

### `SeraDFrame.concat`

Vertical union with another SeraDFrame.

### `SeraDFrame.corr`

Pearson correlation between two columns.

### `SeraDFrame.corr_matrix`

Full pairwise Pearson correlation matrix across numeric columns.

### `SeraDFrame.corrwith`

Pearson correlation of each shared numeric column against another SeraDFrame.

### `SeraDFrame.cov`

Sample covariance between two columns.

### `SeraDFrame.crosstab`

Frequency cross-tabulation of two columns.

### `SeraDFrame.cummax`

Adds {col}_cummax, running maximum.

### `SeraDFrame.cummin`

Adds {col}_cummin, running minimum.

### `SeraDFrame.cumprod`

Adds {col}_cumprod, running product.

### `SeraDFrame.cumsum`

Adds {col}_cumsum, running total.

### `SeraDFrame.describe`

Per-column count/mean/min/max/std.

### `SeraDFrame.diff`

Adds {col}_diff, the row-over-row difference.

### `SeraDFrame.drop`

Drops the given columns.

### `SeraDFrame.drop_duplicates`

Drops duplicate rows.

Aliases: `dedupe`

### `SeraDFrame.dropna`

Drops rows with missing values.

### `SeraDFrame.dt_add_seconds`

Adds {col}_shifted, the epoch column offset by a duration in seconds.

### `SeraDFrame.dt_ceil`

Adds {col}_ceil_UNIT, epoch rounded up to a second/minute/hour/day boundary.

### `SeraDFrame.dt_day`

Adds {col}_day from an epoch-second column.

### `SeraDFrame.dt_dayofyear`

Adds {col}_dayofyear (1-366) from an epoch-second column.

### `SeraDFrame.dt_floor`

Adds {col}_floor_UNIT, epoch rounded down to a second/minute/hour/day boundary.

### `SeraDFrame.dt_hour`

Adds {col}_hour from an epoch-second column.

### `SeraDFrame.dt_minute`

Adds {col}_minute from an epoch-second column.

### `SeraDFrame.dt_month`

Adds {col}_month from an epoch-second column.

### `SeraDFrame.dt_quarter`

Adds {col}_quarter (1-4) from an epoch-second column.

### `SeraDFrame.dt_round`

Adds {col}_round_UNIT, epoch rounded to the nearest second/minute/hour/day boundary.

### `SeraDFrame.dt_second`

Adds {col}_second from an epoch-second column.

### `SeraDFrame.dt_weekday`

Adds {col}_weekday, 0=Monday..6=Sunday, from an epoch-second column.

### `SeraDFrame.dt_year`

Adds {col}_year from an epoch-second column.

### `SeraDFrame.dtypes`

Maps column name to inferred dtype.

### `SeraDFrame.duplicated`

Boolean mask of rows that repeat an earlier row.

Aliases: `is_duplicated`

### `SeraDFrame.empty`

True if the frame has no rows or no columns.

### `SeraDFrame.equals`

Structural equality with another SeraDFrame.

### `SeraDFrame.ewm_mean`

Adds {col}_ewm_mean, an exponentially weighted moving average (adjust=False semantics).

### `SeraDFrame.expanding_mean`

Adds {col}_expanding_mean, a running mean from row 0.

### `SeraDFrame.expanding_std`

Adds {col}_expanding_std, a running std (Welford) from row 0.

### `SeraDFrame.expanding_sum`

Adds {col}_expanding_sum, a running total from row 0.

### `SeraDFrame.factorize`

Encodes a column as (integer codes, unique values).

### `SeraDFrame.ffill`

Forward-fills missing values with the last valid observation.

### `SeraDFrame.fillna`

Fills missing values with a constant.

### `SeraDFrame.filter_eq`

Keeps rows where a column equals a value.

Aliases: `where_eq`

### `SeraDFrame.filter_gt`

Keeps rows greater than a value.

### `SeraDFrame.filter_in`

Keeps rows whose value is in a set.

### `SeraDFrame.filter_lt`

Keeps rows lesser than a value.

### `SeraDFrame.from_csv`

Reads a CSV into a SeraDFrame with parallel per-column type inference.

### `SeraDFrame.from_pandas`

Builds a SeraDFrame from an existing pandas.DataFrame.

### `SeraDFrame.get_dummies`

One-hot encodes a categorical column into {col}_{category} flags.

### `SeraDFrame.glimpse`

Prints row/column counts and a per-column dtype + value preview.

### `SeraDFrame.groupby`

Groups rows by one or more columns for aggregation.

Aliases: `group_by`

### `SeraDFrame.head`

First n rows.

### `SeraDFrame.idxmax`

Row index of the maximum value.

### `SeraDFrame.idxmin`

Row index of the minimum value.

### `SeraDFrame.interpolate`

Fills NaN gaps in a column by linear interpolation.

### `SeraDFrame.isin`

Boolean mask of rows whose value is in a set.

Aliases: `is_in`

### `SeraDFrame.isna`

Per-cell missing-value boolean mask.

Aliases: `is_null`

### `SeraDFrame.kurt`

Sample-adjusted excess kurtosis of a numeric column.

### `SeraDFrame.map_values`

Substitutes column values via a dict mapping.

### `SeraDFrame.mask_col`

Replaces values where a boolean column is true, keeps the rest.

### `SeraDFrame.max`

Maximum of a numeric column.

### `SeraDFrame.mean`

Mean of a numeric column.

### `SeraDFrame.median`

Median of a numeric column via O(n) selection.

### `SeraDFrame.melt`

Unpivots id/value columns into long variable/value rows.

Aliases: `unpivot`

### `SeraDFrame.memory_usage`

Estimated bytes held per column.

Aliases: `estimated_size`

### `SeraDFrame.merge`

Relational join with another SeraDFrame on one or more keys.

Aliases: `join`

### `SeraDFrame.min`

Minimum of a numeric column.

### `SeraDFrame.mode`

Most frequent value(s) of a column, ties included.

### `SeraDFrame.ndim`

Always 2 for a SeraDFrame.

### `SeraDFrame.nlargest`

Top-n rows by column value via O(n) partial selection, no full sort.

Aliases: `top_k`

### `SeraDFrame.normalize`

Adds {col}_norm, min-max scaled to 0..1.

### `SeraDFrame.notna`

Inverse of isna.

Aliases: `is_not_null`

### `SeraDFrame.nsmallest`

Bottom-n rows by column value via O(n) partial selection, no full sort.

Aliases: `bottom_k`

### `SeraDFrame.pct_change`

Adds {col}_pct_change, row-over-row percent change.

### `SeraDFrame.pipe`

Passes self into a Python function, chainable.

### `SeraDFrame.qcut`

Adds {col}_qbin, equal-frequency quantile bin index.

### `SeraDFrame.quantile`

Arbitrary quantile of a numeric column via O(n) selection and linear interpolation.

### `SeraDFrame.query`

Filters rows via a bounded expression string (single 'and'/'or' combinator, comparisons only).

### `SeraDFrame.rank`

Adds {col}_rank, average rank with ties.

### `SeraDFrame.rename`

Renames columns via a mapping.

### `SeraDFrame.replace`

Replaces a scalar value in a column.

### `SeraDFrame.rolling_max`

Adds {col}_rollN_max, a trailing rolling maximum of window N.

### `SeraDFrame.rolling_mean`

Adds {col}_rollN_mean, a trailing rolling mean of window N.

### `SeraDFrame.rolling_min`

Adds {col}_rollN_min, a trailing rolling minimum of window N.

### `SeraDFrame.rolling_std`

Adds {col}_rollN_std, a trailing rolling std of window N.

### `SeraDFrame.rolling_sum`

Adds {col}_rollN_sum, a trailing rolling sum of window N.

### `SeraDFrame.round`

Rounds every numeric column to n decimals.

### `SeraDFrame.sample`

Random row sample via reservoir sampling.

### `SeraDFrame.select`

Keeps only the given columns.

### `SeraDFrame.select_dtypes`

Keeps only columns matching a dtype.

### `SeraDFrame.sem`

Standard error of the mean of a numeric column.

### `SeraDFrame.shape`

Returns (rows, columns).

### `SeraDFrame.shift`

Adds {col}_shiftN, values shifted by N periods.

### `SeraDFrame.size`

Total cell count (rows times columns).

### `SeraDFrame.skew`

Sample-adjusted (Fisher-Pearson) skewness of a numeric column.

### `SeraDFrame.sort_values`

Sorts rows by one or more columns, lexicographic tie-break.

Aliases: `sort`

### `SeraDFrame.std`

Standard deviation of a numeric column.

### `SeraDFrame.str_contains`

Boolean mask of values containing a substring.

### `SeraDFrame.str_count`

Adds {col}_count, the number of non-overlapping regex matches.

### `SeraDFrame.str_endswith`

Boolean mask of values ending with a suffix.

### `SeraDFrame.str_extract`

Adds {col}_extract, the first regex capture group (or full match).

### `SeraDFrame.str_len`

Adds {col}_len, character count per value.

### `SeraDFrame.str_lower`

Lowercases a string column.

### `SeraDFrame.str_match`

Boolean mask of values matching a regex pattern.

### `SeraDFrame.str_replace`

Replaces a substring in every value of a column.

### `SeraDFrame.str_split_get`

Adds {col}_splitN, the Nth field after splitting by a separator.

### `SeraDFrame.str_startswith`

Boolean mask of values starting with a prefix.

### `SeraDFrame.str_strip`

Trims whitespace from a string column.

### `SeraDFrame.str_upper`

Uppercases a string column.

### `SeraDFrame.sum`

Sum of a numeric column.

### `SeraDFrame.tail`

Last n rows.

### `SeraDFrame.to_csv`

Writes to a CSV file.

### `SeraDFrame.to_datetime`

Parses an ISO-ish date/time string column into epoch-second floats.

### `SeraDFrame.to_pandas`

Converts to a pandas.DataFrame, losslessly.

### `SeraDFrame.to_records`

Row-oriented list of dicts.

Aliases: `to_dicts`

### `SeraDFrame.to_table`

Bridges into SeraPlot's Table so chart-export helpers apply.

### `SeraDFrame.unique`

Unique values of a column.

### `SeraDFrame.update`

Overwrites shared columns with the other frame's non-missing values, row by row.

### `SeraDFrame.value_counts`

Counts occurrences per unique value.

Aliases: `counts`

### `SeraDFrame.values`

Row-major list-of-lists view of every cell.

### `SeraDFrame.where_col`

Keeps values where a boolean column is true, replaces the rest with a constant.

### `SeraDFrame.with_columns`

Batch-adds or replaces several columns in one pass.

### `SeraDFrameGroupBy.agg`

Per-column aggregation spec.

### `SeraDFrameGroupBy.head`

First n rows of every group.

### `SeraDFrameGroupBy.mean`

Mean of every numeric column per group.

### `SeraDFrameGroupBy.nth`

The nth row of every group (negative indexes from the end).

### `SeraDFrameGroupBy.size`

Row count per group.

### `SeraDFrameGroupBy.sum`

Sum of every numeric column per group.

### `SeraDFrameGroupBy.tail`

Last n rows of every group.

### `SeraDFrameGroupBy.transform`

Broadcasts a per-group aggregate back onto every original row.

### `Table`

Small columnar data-shaping utility built from a dict of columns.

### `Table.columns`

Column names, in original order.

### `Table.concat`

Vertical union of two tables.

### `Table.cumsum`

Running total.

### `Table.describe`

count/mean/min/max/std per numeric column.

### `Table.filter_eq`

Rows where a column equals a value.

### `Table.filter_gt`

Rows above a numeric value.

### `Table.from_csv`

Reads a CSV into a Table.

### `Table.groupby_agg`

Groups by a column, aggregates another.

Aliases: `group_by_agg`

### `Table.head`

First n rows.

### `Table.join`

Relational join on a key column.

### `Table.pct_change`

Row-over-row percent change.

### `Table.pivot`

Reshapes long data to wide.

### `Table.rank`

1-based rank.

### `Table.rolling_mean`

Trailing moving average.

### `Table.select`

A subset of columns.

### `Table.sort_by`

Sorted copy by a column.

Aliases: `sort`

### `Table.to_grouped_bar`

Pivots and flattens into sp.grouped_bar() inputs.

### `Table.to_multiline`

Extracts columns into sp.multiline() inputs.

### `Table.to_scatter`

Extracts columns into sp.scatter() inputs.

### `Table.top_n`

Shortcut for sort_by(desc).head(n).

### `Table.with_column`

Adds a computed column (add/sub/mul/div).

### `Table.zscore`

(x - mean) / std.


## Chart methods (146)

Methods callable on the `Chart` object returned by any builder (e.g. `sp.bar(...).legend()`).

### `a11y`

Adds ARIA accessibility attributes (title and description) to the SVG element.

Aliases: `accessibility`, `aria`

  | Param | Type | Description |
  |---|---|---|
  | `title` | `str` | Accessible title for screen readers. |
  | `desc` | `str` | Accessible description for screen readers. |

### `animate`

Adds a staggered entry animation to data elements (bars, circles, areas).

Aliases: `animation`

  | Param | Type | Description |
  |---|---|---|
  | `duration` | `int` | Animation duration in milliseconds. Default: 300. |

### `annotate_max`

Highlights and labels the data element with the highest value, on value-based bar charts.

Aliases: `mark_max`, `ann_max`

  | Param | Type | Description |
  |---|---|---|
  | `label` | `str | None` | Optional custom label text. Defaults to the value itself. |
  | `color` | `str` | Highlight and label color. Default: #22c55e. |

### `annotate_min`

Highlights and labels the data element with the lowest value, on value-based bar charts.

Aliases: `mark_min`, `ann_min`

  | Param | Type | Description |
  |---|---|---|
  | `label` | `str | None` | Optional custom label text. Defaults to the value itself. |
  | `color` | `str` | Highlight and label color. Default: #ef4444. |

### `apply_color_bindings`

Applies the globally registered label→color bindings (set via sp.bind_colors()) to this chart.

Aliases: `apply_bindings`, `color_from_bindings`

### `axis_label_angle`

Rotates X and/or Y axis tick labels independently.

Aliases: `tick_angle`, `label_angle`

  | Param | Type | Description |
  |---|---|---|
  | `x_angle` | `int | None` | Rotation angle for X axis labels in degrees. |
  | `y_angle` | `int | None` | Rotation angle for Y axis labels in degrees. |

### `bar_gap`

Adjusts the gap ratio between bars. Higher values create thinner bars with more space.

Aliases: `gap`

  | Param | Type | Description |
  |---|---|---|
  | `ratio` | `float` | Gap ratio between 0.0 (no gap) and 0.95 (almost no bar). Default: 0.3. |

### `bar_scale`

Applies per-bar visual height scale factors without changing data values. A factor of 2.0 doubles the bar height; 0.5 halves it. Use dev() SCALE mode to find values interactively.

Aliases: `visual_scale`, `bar_heights`, `scale_bars`

  | Param | Type | Description |
  |---|---|---|
  | `factors` | `list[float]` | Scale factor per bar in data order. Length must match the number of bars. |

### `border_radius`

Applies a CSS border-radius to the chart container element.

Aliases: `radius`, `corners`

  | Param | Type | Description |
  |---|---|---|
  | `px` | `int` | Corner radius in pixels. |

### `box_annotate`

Draws a box-and-whisker summary (min, Q1, median, Q3, max) of the chart's own values as a small inset glyph, like a built-in boxplot companion.

Aliases: `boxplot`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Box, whiskers and median line color. Default: #6366f1. |

### `cadrage`

Draws a visual cluster frame around specified heatmap columns: a subtle fill, column separator grid lines, a border rectangle with corner L-brackets, and a looping zoom animation that pulses the viewBox in and out of the cluster region.

Aliases: `frame_cols`, `highlight_cols`, `cluster_frame`

  | Param | Type | Description |
  |---|---|---|
  | `cols` | `list[str]` | Column labels to frame (must match the heatmap col_labels). |
  | `rows` | `list[str] | None` | Row labels to restrict the frame to. Default: all rows. |
  | `color` | `str | None` | Frame and separator color. Default: #6366f1. |

### `caption`

Adds a small footnote caption centered below the chart.

Aliases: `footnote`

  | Param | Type | Description |
  |---|---|---|
  | `text` | `str` | Caption text. |

### `cluster`

Runs DBSCAN density clustering and recolors each element by its cluster, in-browser and dependency-free. Works on any chart: uses x/y for scatter points, or falls back to the single value for bars/lollipops/etc. Noise points (no dense neighborhood) are colored gray.

Aliases: `dbscan`

  | Param | Type | Description |
  |---|---|---|
  | `eps` | `float` | Maximum distance (in data units) between two points for them to be considered neighbors. |
  | `min_samples` | `int` | Minimum neighborhood size (including the point itself) to seed a cluster. Default: 3. |

### `color_density`

Colors each data element by its value on a density gradient: low values → blue/indigo, high values → yellow/red.

Aliases: `density_color`, `gradient_by_value`

### `colorbar`

Adds a heat colorbar legend showing the value range used to color the data points, in both 2D and 3D charts. Position can be 'right' (default), 'left', 'top' or 'bottom'.

Aliases: `color_scale`, `heat_legend`, `color_legend`

  | Param | Type | Description |
  |---|---|---|
  | `position` | `str` | 'right', 'left', 'top' or 'bottom'. |

### `compare_ticker`

Enables shift-click comparison ticker mode. Shift-click two or more charts to spawn a floating ticker showing which chart scores higher (>, >>, >>>). Shift-click a selected chart again to deselect it.

Aliases: `cmp_ticker`

### `compose`

Overlays another chart's SVG on top of this chart using an SVG blend mode. Both charts must have compatible dimensions for best results.

Aliases: `overlay`, `blend`, `layer_on`

  | Param | Type | Description |
  |---|---|---|
  | `other` | `Chart` | The chart to overlay. |
  | `blend` | `str | None` | CSS mix-blend-mode (e.g. 'multiply', 'screen', 'overlay'). Default 'screen'. |

### `corner_radius_bars`

Applies a corner radius to all bar rectangles in the chart.

Aliases: `bar_radius`

  | Param | Type | Description |
  |---|---|---|
  | `radius` | `str` | Radius in pixels as string or percentage (e.g. '8' or '50%'). |

### `correlation_badge`

Computes and displays the Pearson correlation coefficient (r) of a scatter chart's x/y values as a small badge.

Aliases: `corr`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Badge color. Default: #6366f1. |

### `crosshair`

Adds an interactive crosshair that follows the mouse cursor across the SVG.

Aliases: `xhair`

### `csp_safe`

Removes inline event handlers to make the chart compatible with strict Content-Security-Policy environments.

Aliases: `safe_csp`

### `cumulative_line`

Overlays a cumulative-sum line (running total over the running grand total), the classic Pareto-chart finishing touch.

Aliases: `cumline`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Line and marker color. Default: #6366f1. |

### `cut_bars`

Overlays white horizontal cuts on every bar at regular pixel intervals, creating a segmented / Tufte data-ink effect.

Aliases: `tufte_bars`, `unit_bars`, `bars_cut`

  | Param | Type | Description |
  |---|---|---|
  | `step` | `float | None` | Pixel height of each segment. None = auto (plot-height / 12). |
  | `gap` | `int` | Pixel height of the white gap between segments (default 2). |
  | `color` | `str | None` | Cut color (default '#ffffff'). Use the chart background color to blend seamlessly. |

### `delta_labels`

Labels each bar with its percentage change versus the previous bar, handy for tracking growth (QoQ, MoM, ...).

Aliases: `deltas`, `pct_change`

  | Param | Type | Description |
  |---|---|---|
  | `pos_color` | `str` | Color used for positive changes. Default: #22c55e. |
  | `neg_color` | `str` | Color used for negative changes. Default: #ef4444. |

### `desaturate`

Reduces the color saturation of specific data elements (by index) or all elements if no indices given. factor=0.0 is full grayscale, 1.0 is original color.

Aliases: `desat`, `grayscale_elements`

### `despine`

Removes the axis spine lines, keeping ticks and labels (seaborn-style despine).

Aliases: `no_spines`

### `dev`

Activates dev mode: makes the legend and title draggable in the browser, shows a floating panel with real-time SVG coordinates, and generates a Python snippet you can copy. When you find the right positions, replace .dev() with .legend_offset(dx, dy) and/or .title_offset(dx, dy).

Aliases: `devmode`, `inspector`, `layout_inspector`

### `diff`

Returns a textual diff between this chart's HTML and another chart's HTML.

Aliases: `compare`

  | Param | Type | Description |
  |---|---|---|
  | `other` | `Chart` | The other Chart instance to compare against. |

### `diff_from_mean`

Labels each bar with its absolute deviation from the dataset's mean, instead of percentage change versus the previous bar like delta_labels().

Aliases: `mean_diff`

  | Param | Type | Description |
  |---|---|---|
  | `pos_color` | `str` | Color used for above-average bars. Default: #22c55e. |
  | `neg_color` | `str` | Color used for below-average bars. Default: #ef4444. |

### `dim_above`

Dims elements whose value is strictly above the threshold — those at or below stay bright. Useful for emphasizing low-value regions or anomalies under a cap.

Aliases: `dim_over`, `fade_above`, `fade_over`, `highlight_below`, `focus_below`, `dim_high`, `above_threshold`

  | Param | Type | Description |
  |---|---|---|
  | `threshold` | `float` | Value threshold. Elements with value <= threshold stay at full opacity. |
  | `opacity` | `float` | Opacity applied to dimmed elements. Default 0.2. |

### `dim_below`

Dims elements whose value is strictly below the threshold — those at or above stay bright. Use for emphasizing top performers or highlighting bars that clear a target line.

Aliases: `dim_under`, `fade_below`, `fade_under`, `highlight_above`, `focus_above`, `dim_low`, `below_threshold`

  | Param | Type | Description |
  |---|---|---|
  | `threshold` | `float` | Value threshold. Elements with value >= threshold stay at full opacity. |
  | `opacity` | `float` | Opacity applied to dimmed elements. Default 0.2. |

### `dim_others`

Dims every element NOT in the given index list. Pass a list of indices to keep at full opacity — everything else fades to `opacity`. Chainable with dim_below / dim_above for combined effects.

Aliases: `focus`, `downscale`, `highlight_top`, `fade_others`, `keep_only`, `isolate`, `dim_rest`, `show_only`

  | Param | Type | Description |
  |---|---|---|
  | `keep` | `list[int]` | Indices of elements to keep at full opacity. |
  | `opacity` | `float` | Opacity applied to dimmed elements (0.0–1.0). Default 0.2. |

### `downsample`

Downsamples line chart data using the LTTB algorithm to reduce visual clutter.

Aliases: `decimate`, `lod`

  | Param | Type | Description |
  |---|---|---|
  | `n` | `int` | Target number of data points after downsampling. |
  | `method` | `str` | Downsampling method. Currently only 'lttb' is supported. |

### `draw_tool`

Enables a freehand drawing overlay on the chart: pen mode, text mode, color picker, clear and SVG download. Drawings are stored as SVG paths inside the chart and survive save().

Aliases: `whiteboard`, `sketch`, `paintbrush`, `annotate_tool`, `pen`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Initial stroke and text color as a CSS hex string (default '#ff0000'). |

### `error_bars`

Draws symmetric error-bar whiskers above and below each bar's top, like matplotlib's errorbar() or yerr.

Aliases: `errorbar`

  | Param | Type | Description |
  |---|---|---|
  | `margin` | `float` | Error margin, expressed in the same units as the chart's values. |
  | `color` | `str` | Whisker color. Default: #475569. |

### `export_button`

Adds a floating download button to the chart that saves the full HTML on click.

Aliases: `download_button`, `export_btn`, `with_export_button`

### `export_svg`

Saves the chart's SVG to a file.

Aliases: `save_svg`, `svg_export`, `write_svg`

  | Param | Type | Description |
  |---|---|---|
  | `path` | `str` | Destination .svg file path. |

### `fill_between`

Shades the area under a line chart's curve down to the baseline, like matplotlib's fill_between(x, y, 0).

Aliases: `fill_area`, `area_fill`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Fill color. Default: #6366f1. |
  | `opacity` | `float` | Fill opacity between 0.0 and 1.0. Default: 0.15. |

### `flip`

Flips a vertical bar chart into a horizontal bar chart by recalculating bar positions.

Aliases: `swap_axes`, `transpose`

### `flourish`

Adds a decorative particle texture layer over the chart — tiny scattered dots that evoke Visual Cinnamon's signature textural depth.

Aliases: `particles`, `texture`, `grain`, `dots`

  | Param | Type | Description |
  |---|---|---|
  | `density` | `float` | Particle density 0.0–1.0. Default 0.4. |
  | `seed` | `int` | Random seed for reproducibility. Default 42. |
  | `color` | `str | None` | Particle color. Default '#ffffff'. |

### `focus_mode`

On hover, dims all non-hovered plotly slices (sunburst, pie, treemap) to near-invisible and hides their text labels — only the hovered segment and its ancestors stay fully visible. Double-click resets. Ideal for exploring multi-level sunburst hierarchies.

Aliases: `sunburst_focus`, `layer_focus`, `slice_focus`, `hover_dim_slices`, `plotly_focus`

### `font`

Sets the font family for all SVG text and body text in the chart.

Aliases: `set_font`, `font_family`

  | Param | Type | Description |
  |---|---|---|
  | `name` | `str` | Font family name (e.g. 'Roboto', 'Inter'). |

### `forecast_line`

Extrapolates a linear-regression trend beyond the last data point, with a dashed line and ghost markers for each forecast step.

Aliases: `forecast`, `extrapolate`

  | Param | Type | Description |
  |---|---|---|
  | `periods` | `int` | Number of future periods to forecast. Default: 3. |
  | `color` | `str` | Forecast line and marker color. Default: #8b5cf6. |

### `frames`

Adds data-frame animation controls with play, pause, step and timeline slider controls. Each frame is a list of values that replaces the current bar heights with a smooth transition.

Aliases: `animate_frames`, `play`, `frame_animation`, `slider_animation`

  | Param | Type | Description |
  |---|---|---|
  | `frames` | `list[list[float]]` | List of frames; each frame is a list of values with the same length as the chart data. |
  | `speed_ms` | `int` | Duration between frames in milliseconds. Default: 600. |
  | `labels` | `list[str] | None` | Optional label shown next to the slider for each frame. |

### `glow`

Adds a neon drop-shadow glow behind every data element (bars, lines, points).

Aliases: `neon`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Glow color. Defaults to the chart's accent color (#6366f1). |

### `grid_at`

Draws a solid horizontal reference line at the given data value. Unlike hline(), this line is solid and rendered below the bars, acting as a threshold grid marker.

Aliases: `ref_line`, `baseline`, `threshold_line`, `value_line`

  | Param | Type | Description |
  |---|---|---|
  | `value` | `float` | Data value at which to draw the reference line. |
  | `color` | `str` | Line color as a CSS string (default '#64748b'). |
  | `label` | `str | None` | Optional text label shown at the right end of the line. |

### `grid_x`

Shows only vertical (X-axis category separator) grid lines, hiding horizontal value lines.

Aliases: `grid_vertical`, `x_grid`

### `grid_y`

Shows only horizontal (Y-axis value level) grid lines, hiding vertical category lines.

Aliases: `grid_horizontal`, `y_grid`

### `group`

Groups this chart with every other chart sharing the same group name into one navigable, clickable, interchangeable tabbed section. Works for any 2D or 3D chart, in a notebook or any HTML page.

Aliases: `tab_group`

  | Param | Type | Description |
  |---|---|---|
  | `name` | `str` | Group identifier shared by every chart that should appear together. |
  | `position` | `str` | Where the tab strip sits relative to the chart: 'top' (default), 'bottom', 'left' or 'right'. |

### `group_hover_opacity`

When hovering a group element, dims all elements outside that group. Accepts an optional opacity level (0.0–1.0, default 0.15).

Aliases: `group_hover`, `hover_dim`

### `growth_badge`

Displays a badge with the total % change between the first and last bar — a quick growth-rate summary.

Aliases: `growth`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Badge color. Default: #22c55e. |

### `hide_grid`

Hides the grid lines if they were previously enabled.

Aliases: `no_grid`

### `highlight`

Highlights a single data element by index with a distinct color and glow, dimming nothing else.

Aliases: `mark`, `spotlight`

  | Param | Type | Description |
  |---|---|---|
  | `index` | `int` | Zero-based index of the data element to highlight. |
  | `color` | `str | None` | Highlight color. Default: #f59e0b. |

### `highlight_group`

Statically highlights elements whose label matches one of the given labels, dimming all others to the specified opacity.

Aliases: `highlight_labels`, `focus_group`

### `highlight_range`

Shades a contiguous range of bars/points by index, distinct from reference_band which shades by Y value.

Aliases: `hl_range`, `idx_band`

  | Param | Type | Description |
  |---|---|---|
  | `low` | `int` | Start index of the range (inclusive). |
  | `high` | `int` | End index of the range (inclusive). |
  | `color` | `str` | Highlight fill color. Default: #6366f1. |
  | `opacity` | `float` | Fill opacity between 0.0 and 1.0. Default: 0.12. |

### `hline`

Draws a dashed horizontal reference/threshold line at the given data value, on value-based bar charts.

Aliases: `hrule`, `target_line`

  | Param | Type | Description |
  |---|---|---|
  | `value` | `float` | Data value where the line is drawn. |
  | `color` | `str` | Line color. Default: #ef4444. |
  | `label` | `str | None` | Optional label drawn next to the line. |

### `horizontal`

Alias for flip(). Renders the chart with horizontal bars.

Aliases: `hbar`, `flip_h`

### `hover_slots`

Replaces the hover tooltip data with a JSON array of slot objects. Each slot maps to the existing tooltip fields: title (header), kv (key/value rows), image (url or data-uri — supports GIFs), video (url), html (raw HTML block). Pass one slot per data point.

Aliases: `set_hover`, `hover_data`, `tooltip_slots`, `with_hover`, `hover_json`

  | Param | Type | Description |
  |---|---|---|
  | `slots_json` | `str` |  |

### `hstack`

Places two charts side-by-side in a single HTML container — the fundamental Visual Cinnamon multi-panel layout.

Aliases: `side_by_side`, `columns`, `two_col`

  | Param | Type | Description |
  |---|---|---|
  | `other` | `Chart` | The second chart to place on the right. |
  | `gap` | `int` | Gap between charts in pixels. Default 16. |

### `image_bg`

Injects a background image (URL or data URI) behind the chart SVG content.

Aliases: `bg_image`, `background_image`

  | Param | Type | Description |
  |---|---|---|
  | `url` | `str` | Image URL or base64 data URI. |
  | `opacity` | `float` | Opacity 0.0–1.0. Default 0.35. |

### `inject_css`

Injects a raw CSS string into the chart's <head> element.

Aliases: `css`, `custom_css`

  | Param | Type | Description |
  |---|---|---|
  | `css` | `str` | Raw CSS rules to inject. |

### `inject_js`

Injects a raw JavaScript string into the chart's <body> element.

Aliases: `js`, `custom_js`

  | Param | Type | Description |
  |---|---|---|
  | `js` | `str` | Raw JavaScript code to inject. |

### `iqr_band`

Shades a horizontal band across the interquartile range (Q1 to Q3) of the chart's own values, to see at a glance which bars fall inside the 'normal' middle 50%.

Aliases: `iqr`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Band fill color. Default: #6366f1. |
  | `opacity` | `float` | Fill opacity between 0.0 and 1.0. Default: 0.10. |

### `label_position`

Alias for show_labels(position). Adds an interactive legend overlay at the given position.

Aliases: `labels_at`

  | Param | Type | Description |
  |---|---|---|
  | `position` | `str` | Overlay position: 'top', 'bottom', 'left', 'right'. |

### `lasso`

Enables box selection on scatter charts. Shift+drag to select points inside a rectangle; unselected points are dimmed. Shift+click an empty area clears the selection.

Aliases: `box_select`, `rect_select`, `rubber_band`

### `legend`

Repositions the chart legend and enables interactive series toggling by click.

Aliases: `show_leg`

  | Param | Type | Description |
  |---|---|---|
  | `position` | `str` | Legend position: 'right' (default), 'left', 'top', 'bottom', 'none'. |

### `legend_offset`

Shifts the legend by (dx, dy) pixels in SVG coordinates relative to its default position. Use dev() to find the right values interactively.

Aliases: `shift_legend`, `move_legend`

  | Param | Type | Description |
  |---|---|---|
  | `dx` | `int` | Horizontal offset in SVG pixels. |
  | `dy` | `int` | Vertical offset in SVG pixels. |

### `legend_position`

Alias for legend(position). Repositions the chart legend.

Aliases: `leg_pos`

  | Param | Type | Description |
  |---|---|---|
  | `position` | `str` | Legend position: 'right', 'left', 'top', 'bottom', 'none'. |

### `LiveStream`

A bounded ring-buffer accumulator that turns a series of (x, y) samples arriving over time into a continuously redrawn chart. push()/extend()/clear() repaint the same Jupyter output cell in place via IPython's display_id, so the chart updates smoothly with no flicker or new cells.

  | Param | Type | Description |
  |---|---|---|
  | `kind` | `str` | Chart kind: 'line' or 'scatter'. Default: 'line'. |
  | `title` | `str` | Chart title. |
  | `max_points` | `int` | Ring buffer size; oldest samples are dropped past this. Default: 500. |
  | `color_hex` | `int` | Series color as a 24-bit RGB integer. |
  | `width` | `int` | Canvas width in pixels. |
  | `height` | `int` | Canvas height in pixels. |

### `LiveStream.clear`

Empties the ring buffer and redraws the (now empty) live chart in place.

Aliases: `reset`

### `LiveStream.extend`

Appends two lists of samples in lock-step, enforces max_points, and redraws the live chart in place.

Aliases: `add_points`, `append_many`

  | Param | Type | Description |
  |---|---|---|
  | `xs` | `list[float]` | X values. |
  | `ys` | `list[float]` | Y values. |

### `LiveStream.push`

Appends one (x, y) sample, enforces max_points, and redraws the live chart in place.

Aliases: `append`, `add_point`

  | Param | Type | Description |
  |---|---|---|
  | `x` | `float` | X value. |
  | `y` | `float` | Y value. |

### `LiveStream.render`

Renders the current buffer to a standalone Chart object, without touching the live display.

Aliases: `snapshot`, `to_chart`

### `log_scale`

Switches the value axis to a logarithmic scale, like matplotlib's plt.yscale('log') — great for data spanning several orders of magnitude.

Aliases: `logy`

### `mask_circle`

Masks the chart to a circle shape using an SVG clipPath.

Aliases: `circle_mask`, `round_mask`, `clip_circle`

  | Param | Type | Description |
  |---|---|---|
  | `cx` | `float | None` | Circle center X as fraction of width (0–1). Default 0.5. |
  | `cy` | `float | None` | Circle center Y as fraction of height (0–1). Default 0.5. |
  | `r` | `float | None` | Radius as fraction of min(width,height)/2. Default 0.48. |

### `mask_poly`

Masks the chart to a regular polygon (triangle, hexagon, star…) using an SVG clipPath.

Aliases: `polygon_mask`, `hex_mask`, `shape_mask`

  | Param | Type | Description |
  |---|---|---|
  | `sides` | `int` | Number of polygon sides (3=triangle, 6=hex, etc.). |
  | `rotation` | `float` | Rotation in degrees. Default 0. |

### `mean`

Draws a dashed horizontal mean line across the plot area, computed from the chart's data values (works on bar, scatter, and other value-based charts).

Aliases: `mean_line`, `avg_line`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Line color. Default: #ef4444. |

### `median`

Draws a dashed horizontal median line across the plot area, computed from the chart's data values (works on bar, scatter, and other value-based charts).

Aliases: `median_line`, `med_line`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Line color. Default: #6366f1. |

### `morph_bg`

Animates the chart background through a gradient transition — cinematic color morphing that brings data to life.

Aliases: `gradient_bg`, `animated_bg`, `morph_gradient`

  | Param | Type | Description |
  |---|---|---|
  | `from_color` | `str` | Start color (CSS). Default '#0f172a'. |
  | `to_color` | `str` | End color (CSS). Default '#1e1b4b'. |
  | `duration` | `float` | Animation duration in seconds. Default 4.0. |

### `moving_average`

Overlays a moving-average line (trailing window), like pandas' .rolling(window).mean() — smooths out noisy series to reveal the underlying trend.

Aliases: `rolling_mean`

  | Param | Type | Description |
  |---|---|---|
  | `window` | `int` | Number of trailing points to average over. |
  | `color` | `str` | Line color. Default: #f59e0b. |

### `neon_bloom`

Adds a multi-layer neon bloom effect to SVG elements — 3 stacked feGaussianBlur passes create a deep luminous glow around bars, lines and circles.

Aliases: `bloom`, `neon_glow`, `plasma`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Bloom glow color as a CSS hex string. Default: #6366f1. |

### `nice_x_axis`

Reformats x-axis tick labels to rounded values. If step is given, ticks become 0, step, 2*step… (uniform); otherwise each tick is rounded to the nearest auto-computed nice step.

Aliases: `round_x_ticks`, `x_ticks_nice`, `format_x_axis`

  | Param | Type | Description |
  |---|---|---|
  | `step` | `float | None` | Tick interval for uniform spacing. None = auto. |

### `no_axes`

Hides both X and Y axes along with their ticks and labels.

Aliases: `hide_axes`, `no_axis`, `hide_axis`, `remove_axes`, `drop_axes`

### `no_background`

Removes every chart background layer and keeps the output transparent.

Aliases: `transparent_bg`, `no_bg`

### `no_hover`

Disables the hover tooltip and removes hover highlighting on data elements.

Aliases: `no_tooltip`

### `no_legend`

Hides the chart legend.

Aliases: `hide_legend`

### `no_select`

Disables the chart's drag-to-select overlay and browser text selection — combine with draw_tool() so dragging only draws, never activates the selection rectangle.

Aliases: `disable_select`, `prevent_select`, `unselectable`, `no_text_select`

### `no_title`

Hides the chart title.

Aliases: `hide_title`

### `no_x_axis`

Hides the X axis, its ticks, and its label.

Aliases: `hide_x`, `no_x`, `remove_x`, `hide_x_axis`, `drop_x`

### `no_y_axis`

Hides the Y axis, its ticks, and its label.

Aliases: `hide_y`, `no_y`, `remove_y`, `hide_y_axis`, `drop_y`

### `orient3d`

Sets the initial camera orientation of a 3D chart: 'iso' (default), 'horizontal' (side-on), 'vertical' (top-down) or 'front'.

Aliases: `orientation3d`, `tilt3d`, `rotate3d`

  | Param | Type | Description |
  |---|---|---|
  | `mode` | `str` | 'iso', 'horizontal', 'vertical' or 'front'. |

### `outliers`

Flags statistical outliers with a warning marker and a highlighted color. Works on any chart: bars/lollipops/etc are flagged by z-score on their value, scatter points by residual distance from their best-fit line.

Aliases: `flag_outliers`, `anomalies`, `scatter_outliers`

  | Param | Type | Description |
  |---|---|---|
  | `threshold_std` | `float` | Number of standard deviations beyond which a value is flagged. Default: 2.0. |
  | `color` | `str` | Highlight color for flagged bars. Default: #ef4444. |

### `outline`

Draws a solid colored outline around every data element, without blurring like glow().

Aliases: `border`, `stroke`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Outline color. Defaults to the chart's accent color (#6366f1). |
  | `width` | `float` | Outline width in pixels. Default: 2.0. |

### `pareto_marker`

Marks the bar at which the cumulative total crosses a threshold percentage (80% by default), the classic Pareto '80/20' cutoff point. Pairs well with cumulative_line().

Aliases: `pareto`, `eighty_twenty`

  | Param | Type | Description |
  |---|---|---|
  | `threshold_pct` | `float` | Cumulative percentage threshold to mark. Default: 80.0. |
  | `color` | `str` | Marker line and label color. Default: #ef4444. |

### `pct_of_total`

Labels each bar with its share of the total (%) instead of its raw value.

Aliases: `pct`

  | Param | Type | Description |
  |---|---|---|
  | `decimals` | `int` | Number of decimal places to show. Default: 1. |
  | `color` | `str` | Label text color. Default: #475569. |

### `percentile_band`

Shades a horizontal band between two arbitrary percentiles of the chart's own values, a generalization of iqr_band for any custom range (e.g. 5th-95th).

Aliases: `pct_band`

  | Param | Type | Description |
  |---|---|---|
  | `low_pct` | `float` | Lower percentile (0-100). |
  | `high_pct` | `float` | Upper percentile (0-100). |
  | `color` | `str` | Band fill color. Default: #6366f1. |
  | `opacity` | `float` | Fill opacity between 0.0 and 1.0. Default: 0.10. |

### `pick`

Enables intra-chart shift-click element comparison. Shift-click any two or more bars or points to select them — unselected elements dim to 18% opacity, and a badge above the plot shows the selected values ranked with >, >>, >>> arrows. Shift-click a selected element again to deselect it.

Aliases: `select_compare`, `element_compare`

### `png_button`

Adds a PNG download button to the chart. Clicking it rasterizes the SVG to a canvas, then triggers a PNG download without a server.

Aliases: `export_png_button`, `download_png`, `png_download`

### `pulse`

Makes data elements gently pulse, drawing attention to the chart (e.g. for live dashboards). By default pulses every element; target specific ones with index, or every element above a value with above.

Aliases: `blink`, `beacon`

  | Param | Type | Description |
  |---|---|---|
  | `duration` | `float` | Pulse cycle duration in seconds. Default: 2.0. |
  | `index` | `list[int] | None` | Zero-based indices of the data elements to pulse (e.g. [0] for the first). Defaults to every element. |
  | `above` | `float | None` | Only pulse data elements whose value exceeds this threshold. |
  | `color` | `str | None` | Force the pulsing elements to flash this color. Defaults to None, which keeps each element's normal color and only pulses its opacity. |

### `range_buttons`

Appends 1M / 6M / YTD / 1Y / All preset range buttons below the chart. If data-point labels parse as dates (native Date.parse, no external dependency), buttons filter by real calendar range from the last point; otherwise they fall back to an equivalent proportional window over the index range.

Aliases: `time_range_buttons`, `date_buttons`, `range_selector`, `quick_range`

### `range_slider`

Appends a dual-handle range slider below the chart. Dragging the handles filters visible data elements by index.

Aliases: `time_slider`, `slider`, `filter_range`, `date_range`

### `rank_badges`

Pins numbered rank badges (1, 2, 3, ...) on the top N highest-value bars, for quick leaderboard-style reads.

Aliases: `ranks`

  | Param | Type | Description |
  |---|---|---|
  | `top_n` | `int` | How many of the highest-value bars to badge. Default: 3. |
  | `color` | `str` | Badge fill color. Default: #6366f1. |

### `reference_band`

Draws a shaded reference band between two data values, on value-based bar charts (e.g. a target range).

Aliases: `ref_band`, `target_zone`

  | Param | Type | Description |
  |---|---|---|
  | `low` | `float` | Lower bound of the band. |
  | `high` | `float` | Upper bound of the band. |
  | `color` | `str` | Band fill color. Default: #f59e0b. |
  | `opacity` | `float` | Band opacity between 0.0 and 1.0. Default: 0.12. |

### `responsive`

Makes the SVG width 100% of its container while keeping proportional height.

Aliases: `fluid`, `auto_size`

### `rolling_std_band`

Overlays a Bollinger-band-style shaded region (rolling mean ± rolling standard deviation) over a trailing window, the classic volatility envelope from financial charting.

Aliases: `bollinger_band`, `volatility_band`

  | Param | Type | Description |
  |---|---|---|
  | `window` | `int` | Number of trailing points used for the rolling mean and standard deviation. |
  | `color` | `str` | Band fill color. Default: #6366f1. |
  | `opacity` | `float` | Fill opacity between 0.0 and 1.0. Default: 0.15. |

### `rotate`

Rotates the entire chart by a snapped angle (0, 90, 180 or 270 degrees).

Aliases: `spin`

  | Param | Type | Description |
  |---|---|---|
  | `deg` | `int` | Rotation in degrees, snapped to nearest 90°. Default: 90. |

### `rotate_labels`

Rotates X axis tick labels by the specified angle in degrees.

Aliases: `tilt_labels`

  | Param | Type | Description |
  |---|---|---|
  | `angle` | `int` | Rotation angle in degrees (e.g. -45 for diagonal labels). |

### `save`

Saves the chart HTML to a file at the given path.

Aliases: `save_html`, `write`, `export_html`

  | Param | Type | Description |
  |---|---|---|
  | `path` | `str` | Destination file path (e.g. 'chart.html'). |

### `scale`

Scales the entire SVG by a given factor from the top-left origin.

Aliases: `resize`

  | Param | Type | Description |
  |---|---|---|
  | `factor` | `float` | Scale multiplier (e.g. 1.5 for 150%). |

### `scatter_regression`

Fits and draws a linear regression line through a scatter chart's points, like seaborn's regplot() or numpy.polyfit(x, y, 1).

Aliases: `regression_line`, `lmplot`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Line color. Default: #ef4444. |
  | `width` | `float` | Line stroke width. Default: 2.0. |

### `secondary_y`

Overlays a second line series mapped to an independent right-side Y axis. Useful for comparing series with different scales on the same chart.

Aliases: `y2`, `right_axis`, `dual_axis`, `twin_axis`, `second_axis`

  | Param | Type | Description |
  |---|---|---|
  | `values` | `list[float]` | Data values for the secondary series. Must match the primary series length. |
  | `color` | `str | None` | Line and axis color. Default: #f59e0b. |
  | `label` | `str | None` | Optional right-axis label. |

### `segment_bars`

Replaces grid lines with white stripes drawn directly inside each bar.

### `set_bg`

Sets the background color of the chart. Pass None to remove the background.

Aliases: `bg`, `background`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | CSS color string (hex, rgb, named). None removes the background. |

### `set_font_size`

Overrides all SVG text elements to the specified font size in pixels.

Aliases: `font_size`

  | Param | Type | Description |
  |---|---|---|
  | `px` | `int` | Font size in pixels. |

### `set_frame`

Sets the background color of the SVG/canvas frame. Use 'transparent' or None to remove it.

Aliases: `frame`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | CSS color for the frame background. |

### `set_margin`

Adds internal padding to the chart and adjusts data element positions accordingly.

Aliases: `margin`

  | Param | Type | Description |
  |---|---|---|
  | `px` | `int` | Margin in pixels applied to all four sides. |

### `set_opacity`

Sets the opacity of all data elements (bars, circles, areas) in the chart.

Aliases: `opacity`, `alpha`

  | Param | Type | Description |
  |---|---|---|
  | `value` | `float` | Opacity between 0.0 (invisible) and 1.0 (fully opaque). |

### `set_padding`

Applies CSS padding to the chart container element.

Aliases: `padding`

  | Param | Type | Description |
  |---|---|---|
  | `px` | `int` | Padding in pixels applied to all four sides. |

### `shadow`

Adds a soft drop-shadow behind the whole chart container.

Aliases: `drop_shadow`

  | Param | Type | Description |
  |---|---|---|
  | `blur` | `int` | Shadow blur radius in pixels. Default: 24. |
  | `color` | `str | None` | Shadow color. Defaults to a neutral dark shadow. |

### `show_grid`

Shows horizontal and vertical grid lines on the chart background.

Aliases: `grid`

### `show_labels`

Adds an interactive series filter overlay with clickable pill-shaped labels.

Aliases: `labels`, `legend_labels`

  | Param | Type | Description |
  |---|---|---|
  | `position` | `str` | Position of the overlay: 'top', 'bottom', 'left', 'right'. |
  | `labels` | `list[str] | None` | Custom label names. Auto-detected if None. |
  | `colors` | `list[str] | None` | Custom color hex strings matching labels. |

### `show_legend`

Forces the chart legend to be visible even if it was hidden.

Aliases: `leg`

### `show_title`

Forces the chart title to be visible with a contrast stroke for readability.

Aliases: `title_on`

### `sort_by`

Sorts chart bars by value or label using a client-side JavaScript re-render.

Aliases: `sort`, `order_by`

  | Param | Type | Description |
  |---|---|---|
  | `order` | `str` | Sort order: 'desc' (default), 'asc', 'alpha', 'alpha_desc', 'none'. |

### `sparse_grid`

Converts solid grid lines to a dashed pattern, reducing visual ink weight (Tufte data-ink principle).

Aliases: `dashed_grid`, `dotted_grid`

### `stamp`

Stamps large artistic text over the chart — watermark, label, or Visual Cinnamon–style annotation.

Aliases: `watermark_text`, `label_stamp`, `big_text`

  | Param | Type | Description |
  |---|---|---|
  | `text` | `str` | Text content to stamp. |
  | `x` | `float` | X position as percent of SVG width. Default 50. |
  | `y` | `float` | Y position as percent of SVG height. Default 50. |
  | `size` | `float` | Font size in px. Default 72. |
  | `color` | `str` | Text color. Default '#ffffff'. |
  | `opacity` | `float` | Opacity 0.0–1.0. Default 0.08. |

### `subtitle`

Adds a small subtitle directly under the chart title.

Aliases: `sub`

  | Param | Type | Description |
  |---|---|---|
  | `text` | `str` | Subtitle text. |

### `text_angle`

Sets the rotation angle for data value labels.

Aliases: `rotate_text`

  | Param | Type | Description |
  |---|---|---|
  | `degrees` | `int` | Rotation angle in degrees. |

### `text_auto`

Overlays data value labels on all chart elements. Supports format strings and positioning.

Aliases: `auto_text`, `data_labels`

  | Param | Type | Description |
  |---|---|---|
  | `format` | `str | None` | Format string (e.g. '.2f', '.0%') or empty string for auto. None disables. |
  | `position` | `str | None` | Label position: 'auto', 'inside', 'outside'. |
  | `angle` | `int | None` | Label rotation angle in degrees. |
  | `font_size` | `int | None` | Font size of the data labels in pixels. |
  | `color` | `str | None` | Color of the data labels. |

### `text_font`

Sets font family, size, and color for data value labels.

Aliases: `font_text`

  | Param | Type | Description |
  |---|---|---|
  | `family` | `str | None` | Font family name. |
  | `size` | `int | None` | Font size in pixels. |
  | `color` | `str | None` | Label text color. |

### `text_position`

Sets the position for data value labels on chart elements.

Aliases: `text_pos`

  | Param | Type | Description |
  |---|---|---|
  | `position` | `str` | Position: 'auto', 'inside', 'outside'. |

### `title_color`

Sets the title color of a 3D chart (works on 3D canvas charts only). Default: white.

Aliases: `ttl_color`, `tc`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | CSS color for the title text. |

### `title_offset`

Shifts the chart title by (dx, dy) pixels in SVG coordinates relative to its default position. Use dev() to find the right values interactively.

Aliases: `shift_title`, `move_title`

  | Param | Type | Description |
  |---|---|---|
  | `dx` | `int` | Horizontal offset in SVG pixels. |
  | `dy` | `int` | Vertical offset in SVG pixels. |

### `title_size`

Sets the font size of the chart title in pixels.

Aliases: `ttl_size`

  | Param | Type | Description |
  |---|---|---|
  | `px` | `int` | Title font size in pixels. |

### `to_svg`

Extracts and returns the raw SVG string from the chart HTML, or None if not present.

Aliases: `svg`, `as_svg`, `get_svg`

### `trace`

Draws a line on the chart. Fractional plot-area coordinates (0.0 = bottom/left, 1.0 = top/right) for straight lines; pass `values` for a data-driven polyline overlay; `vertical=True` draws a vertical rule at x1. Chainable.

Aliases: `draw_line`, `draw`, `overlay_line`, `rule`, `add_line`, `annotate_line`, `line`, `guideline`, `marker_line`

  | Param | Type | Description |
  |---|---|---|
  | `x1` | `float` | Start X fraction (0.0–1.0). Used as the vertical rule position when vertical=True. |
  | `y1` | `float` | Start Y fraction (0.0 = bottom, 1.0 = top). |
  | `x2` | `float` | End X fraction (0.0–1.0). |
  | `y2` | `float` | End Y fraction (0.0 = bottom, 1.0 = top). |
  | `color` | `str | None` | Line color. Default: #ef4444. |
  | `dash` | `str | None` | Stroke style: 'solid' (default), 'dashed', or 'dotted'. |
  | `width` | `float` | Stroke width in pixels. Default: 2.0. |
  | `vertical` | `bool` | When True, draws a vertical rule at x1 spanning y1→y2 (defaults to full height). |
  | `values` | `list[float] | None` | Data values to draw as a polyline overlay, mapped to the chart's Y-axis scale and distributed evenly across the X range. |

### `trendline`

Overlays a linear regression trend line computed from the chart's own values, on value-based bar charts.

Aliases: `trend`, `regline`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Trend line color. Default: #10b981. |
  | `width` | `float` | Trend line width in pixels. Default: 2.0. |

### `uniform_text`

Enforces a minimum font size for data labels; hides or shows labels that don't fit.

Aliases: `uniform`

  | Param | Type | Description |
  |---|---|---|
  | `min_size` | `int` | Minimum font size in pixels. |
  | `mode` | `str` | Behaviour when label doesn't fit: 'hide' or 'show'. |

### `value_labels`

Prints the numeric value above each bar, like matplotlib's bar_label() or Plotly's textposition='auto'.

Aliases: `vlabels`, `bar_labels`

  | Param | Type | Description |
  |---|---|---|
  | `decimals` | `int` | Number of decimal places to show. Default: 0. |
  | `color` | `str` | Label text color. Default: #475569. |

### `void_bg`

Applies a dark void background with animated star particles injected into the SVG — turns any chart into a cinematic space scene.

Aliases: `dark_bg`, `space_bg`, `cosmos_bg`, `night_bg`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Base background color. Default: #07090f. |

### `vstack`

Stacks two charts vertically in a single HTML container.

Aliases: `stack`, `rows`, `two_row`

  | Param | Type | Description |
  |---|---|---|
  | `other` | `Chart` | The second chart to place below. |
  | `gap` | `int` | Gap between charts in pixels. Default 16. |

### `watermark`

Overlays a large diagonal watermark text across the whole chart, for branding or draft marks.

Aliases: `wm`

  | Param | Type | Description |
  |---|---|---|
  | `text` | `str` | Watermark text. |
  | `opacity` | `float` | Watermark opacity between 0.0 and 1.0. Default: 0.08. |

### `x_labels_rotation`

Rotates the X-axis tick labels by the given angle in degrees. Positive = clockwise. Use dev() to find the right value interactively.

Aliases: `rotate_x_labels`, `x_label_angle`, `tilt_x`

  | Param | Type | Description |
  |---|---|---|
  | `degrees` | `int` | Rotation angle in degrees (-90 to 90). |

### `y_labels_rotation`

Rotates the Y-axis tick labels by the given angle in degrees. Use dev() to find the right value interactively.

Aliases: `rotate_y_labels`, `y_label_angle`, `tilt_y`

  | Param | Type | Description |
  |---|---|---|
  | `degrees` | `int` | Rotation angle in degrees (-90 to 90). |

### `zoom`

Enables mouse-wheel zoom and click-drag panning on the chart. Double-click to reset.

Aliases: `magnify`

### `zscore_heat`

Recolors every bar on a diverging blue-white-red scale based on its z-score, turning the chart into a heatmap-as-bars view of which values are unusually high or low.

Aliases: `zheat`, `heatbars`


</div>

<div class="lang-fr">

Généré automatiquement à partir des macros de documentation inline du framework (314 entrées) — lancer `python v2/examples/gen_api_reference.py` pour régénérer après ajout ou documentation d'une méthode. Les fonctions de construction de charts (2D/3D/carte) et les estimateurs ML sont couverts séparément dans [Référence API](api.md).

## Méthodes SeraDFrame / Table (168)

### `DFrameBuilder`

Constructeur de dataframe chainable colonne par colonne.

### `SeraDFrame`

Dataframe colonnaire natif Rust construit depuis un dict de colonnes.

### `SeraDFrame.abs`

Valeur absolue de chaque colonne numerique.

### `SeraDFrame.agg`

Agregation de tout le frame en un SeraDFrame d'une ligne.

### `SeraDFrame.align`

Aligne deux SeraDFrames sur l'union de leurs colonnes, comble les trous.

### `SeraDFrame.all`

Vrai si toutes les valeurs de la colonne sont vraies.

### `SeraDFrame.any`

Vrai si une valeur de la colonne est vraie.

### `SeraDFrame.apply`

Appelle une fonction Python par valeur d'une colonne.

### `SeraDFrame.applymap`

Appelle une fonction Python sur chaque cellule de chaque colonne.

### `SeraDFrame.assign`

Ajoute ou remplace une colonne.

### `SeraDFrame.astype`

Convertit une colonne en float/str/bool.

### `SeraDFrame.between`

Garde les lignes dans un intervalle numerique inclusif.

### `SeraDFrame.bfill`

Propage vers l'arriere la prochaine valeur valide pour combler les manquants.

### `SeraDFrame.bin`

Ajoute {col}_bin, un index de bin a largeur egale entre 0 et n_bins.

### `SeraDFrame.clip`

Borne une colonne numerique entre deux limites.

### `SeraDFrame.columns`

Liste les noms de colonnes dans l'ordre.

### `SeraDFrame.combine_first`

Comble les valeurs manquantes de ce frame avec l'autre frame, ligne par ligne.

### `SeraDFrame.compare`

Masque booleen des colonnes partagees indiquant ou deux SeraDFrames different.

### `SeraDFrame.concat`

Union verticale avec un autre SeraDFrame.

### `SeraDFrame.corr`

Correlation de Pearson entre deux colonnes.

### `SeraDFrame.corr_matrix`

Matrice complete de correlation de Pearson entre colonnes numeriques.

### `SeraDFrame.corrwith`

Correlation de Pearson de chaque colonne numerique partagee avec un autre SeraDFrame.

### `SeraDFrame.cov`

Covariance d'echantillon entre deux colonnes.

### `SeraDFrame.crosstab`

Tableau croise de frequences de deux colonnes.

### `SeraDFrame.cummax`

Ajoute {col}_cummax, maximum cumule.

### `SeraDFrame.cummin`

Ajoute {col}_cummin, minimum cumule.

### `SeraDFrame.cumprod`

Ajoute {col}_cumprod, produit cumule.

### `SeraDFrame.cumsum`

Ajoute {col}_cumsum, somme cumulee.

### `SeraDFrame.describe`

Count/mean/min/max/std par colonne.

### `SeraDFrame.diff`

Ajoute {col}_diff, la difference ligne a ligne.

### `SeraDFrame.drop`

Supprime les colonnes donnees.

### `SeraDFrame.drop_duplicates`

Supprime les lignes dupliquees.

Alias: `dedupe`

### `SeraDFrame.dropna`

Supprime les lignes avec valeurs manquantes.

### `SeraDFrame.dt_add_seconds`

Ajoute {col}_shifted, la colonne epoch decalee d'une duree en secondes.

### `SeraDFrame.dt_ceil`

Ajoute {col}_ceil_UNIT, secondes-epoch arrondies vers le haut a la frontiere seconde/minute/heure/jour.

### `SeraDFrame.dt_day`

Ajoute {col}_day depuis une colonne secondes-epoch.

### `SeraDFrame.dt_dayofyear`

Ajoute {col}_dayofyear (1-366) depuis une colonne secondes-epoch.

### `SeraDFrame.dt_floor`

Ajoute {col}_floor_UNIT, secondes-epoch arrondies vers le bas a la frontiere seconde/minute/heure/jour.

### `SeraDFrame.dt_hour`

Ajoute {col}_hour depuis une colonne secondes-epoch.

### `SeraDFrame.dt_minute`

Ajoute {col}_minute depuis une colonne secondes-epoch.

### `SeraDFrame.dt_month`

Ajoute {col}_month depuis une colonne secondes-epoch.

### `SeraDFrame.dt_quarter`

Ajoute {col}_quarter (1-4) depuis une colonne secondes-epoch.

### `SeraDFrame.dt_round`

Ajoute {col}_round_UNIT, secondes-epoch arrondies a la frontiere la plus proche seconde/minute/heure/jour.

### `SeraDFrame.dt_second`

Ajoute {col}_second depuis une colonne secondes-epoch.

### `SeraDFrame.dt_weekday`

Ajoute {col}_weekday, 0=lundi..6=dimanche, depuis une colonne secondes-epoch.

### `SeraDFrame.dt_year`

Ajoute {col}_year depuis une colonne secondes-epoch.

### `SeraDFrame.dtypes`

Associe chaque colonne a son dtype inferre.

### `SeraDFrame.duplicated`

Masque booleen des lignes qui repetent une ligne precedente.

Alias: `is_duplicated`

### `SeraDFrame.empty`

Vrai si le frame n'a aucune ligne ou aucune colonne.

### `SeraDFrame.equals`

Egalite structurelle avec un autre SeraDFrame.

### `SeraDFrame.ewm_mean`

Ajoute {col}_ewm_mean, une moyenne mobile ponderee exponentiellement (semantique adjust=False).

### `SeraDFrame.expanding_mean`

Ajoute {col}_expanding_mean, une moyenne cumulee depuis la ligne 0.

### `SeraDFrame.expanding_std`

Ajoute {col}_expanding_std, un ecart-type cumule (Welford) depuis la ligne 0.

### `SeraDFrame.expanding_sum`

Ajoute {col}_expanding_sum, une somme cumulee depuis la ligne 0.

### `SeraDFrame.factorize`

Encode une colonne en (codes entiers, valeurs uniques).

### `SeraDFrame.ffill`

Propage vers l'avant la derniere valeur valide pour combler les manquants.

### `SeraDFrame.fillna`

Remplit les valeurs manquantes par une constante.

### `SeraDFrame.filter_eq`

Garde les lignes ou une colonne vaut une valeur.

Alias: `where_eq`

### `SeraDFrame.filter_gt`

Garde les lignes superieures a une valeur.

### `SeraDFrame.filter_in`

Garde les lignes dont la valeur est dans un ensemble.

### `SeraDFrame.filter_lt`

Garde les lignes inferieures a une valeur.

### `SeraDFrame.from_csv`

Lit un CSV en SeraDFrame avec inference de type parallele par colonne.

### `SeraDFrame.from_pandas`

Construit un SeraDFrame depuis un pandas.DataFrame existant.

### `SeraDFrame.get_dummies`

Encode one-hot une colonne categorielle en indicateurs {col}_{categorie}.

### `SeraDFrame.glimpse`

Affiche le nombre de lignes/colonnes et un apercu dtype + valeurs par colonne.

### `SeraDFrame.groupby`

Groupe les lignes par une ou plusieurs colonnes pour agregation.

Alias: `group_by`

### `SeraDFrame.head`

Les n premieres lignes.

### `SeraDFrame.idxmax`

Index de ligne de la valeur maximale.

### `SeraDFrame.idxmin`

Index de ligne de la valeur minimale.

### `SeraDFrame.interpolate`

Comble les trous NaN d'une colonne par interpolation lineaire.

### `SeraDFrame.isin`

Masque booleen des lignes dont la valeur est dans un ensemble.

Alias: `is_in`

### `SeraDFrame.isna`

Masque booleen par cellule des valeurs manquantes.

Alias: `is_null`

### `SeraDFrame.kurt`

Kurtosis en exces (ajustee echantillon) d'une colonne numerique.

### `SeraDFrame.map_values`

Substitue les valeurs d'une colonne via un mapping dict.

### `SeraDFrame.mask_col`

Remplace les valeurs ou une colonne booleenne est vraie, garde le reste.

### `SeraDFrame.max`

Maximum d'une colonne numerique.

### `SeraDFrame.mean`

Moyenne d'une colonne numerique.

### `SeraDFrame.median`

Mediane d'une colonne numerique via selection O(n).

### `SeraDFrame.melt`

Depivote des colonnes id/value en lignes longues variable/value.

Alias: `unpivot`

### `SeraDFrame.memory_usage`

Estimation des octets occupes par colonne.

Alias: `estimated_size`

### `SeraDFrame.merge`

Jointure relationnelle avec un autre SeraDFrame sur une ou plusieurs cles.

Alias: `join`

### `SeraDFrame.min`

Minimum d'une colonne numerique.

### `SeraDFrame.mode`

Valeur(s) la/les plus frequente(s) d'une colonne, egalites incluses.

### `SeraDFrame.ndim`

Toujours 2 pour un SeraDFrame.

### `SeraDFrame.nlargest`

Les n plus grandes lignes selon une colonne via selection partielle O(n), sans tri complet.

Alias: `top_k`

### `SeraDFrame.normalize`

Ajoute {col}_norm, mis a l'echelle min-max entre 0 et 1.

### `SeraDFrame.notna`

Inverse de isna.

Alias: `is_not_null`

### `SeraDFrame.nsmallest`

Les n plus petites lignes selon une colonne via selection partielle O(n), sans tri complet.

Alias: `bottom_k`

### `SeraDFrame.pct_change`

Ajoute {col}_pct_change, variation en pourcentage ligne a ligne.

### `SeraDFrame.pipe`

Passe self dans une fonction Python, chainable.

### `SeraDFrame.qcut`

Ajoute {col}_qbin, index de bin par quantile a frequence egale.

### `SeraDFrame.quantile`

Quantile arbitraire d'une colonne numerique via selection O(n) et interpolation lineaire.

### `SeraDFrame.query`

Filtre les lignes via une expression textuelle bornee (un seul combinateur 'and'/'or', comparaisons uniquement).

### `SeraDFrame.rank`

Ajoute {col}_rank, rang moyen en cas d'egalite.

### `SeraDFrame.rename`

Renomme des colonnes via un mapping.

### `SeraDFrame.replace`

Remplace une valeur scalaire dans une colonne.

### `SeraDFrame.rolling_max`

Ajoute {col}_rollN_max, un maximum glissant de fenetre N.

### `SeraDFrame.rolling_mean`

Ajoute {col}_rollN_mean, une moyenne glissante de fenetre N.

### `SeraDFrame.rolling_min`

Ajoute {col}_rollN_min, un minimum glissant de fenetre N.

### `SeraDFrame.rolling_std`

Ajoute {col}_rollN_std, un ecart-type glissant de fenetre N.

### `SeraDFrame.rolling_sum`

Ajoute {col}_rollN_sum, une somme glissante de fenetre N.

### `SeraDFrame.round`

Arrondit chaque colonne numerique a n decimales.

### `SeraDFrame.sample`

Echantillon aleatoire de lignes par reservoir sampling.

### `SeraDFrame.select`

Ne garde que les colonnes donnees.

### `SeraDFrame.select_dtypes`

Ne garde que les colonnes correspondant a un dtype.

### `SeraDFrame.sem`

Erreur standard de la moyenne d'une colonne numerique.

### `SeraDFrame.shape`

Renvoie (lignes, colonnes).

### `SeraDFrame.shift`

Ajoute {col}_shiftN, valeurs decalees de N periodes.

### `SeraDFrame.size`

Nombre total de cellules (lignes fois colonnes).

### `SeraDFrame.skew`

Asymetrie (Fisher-Pearson, ajustee echantillon) d'une colonne numerique.

### `SeraDFrame.sort_values`

Trie les lignes selon une ou plusieurs colonnes, egalites departagees lexicographiquement.

Alias: `sort`

### `SeraDFrame.std`

Ecart-type d'une colonne numerique.

### `SeraDFrame.str_contains`

Masque booleen des valeurs contenant une sous-chaine.

### `SeraDFrame.str_count`

Ajoute {col}_count, le nombre de correspondances regex non chevauchantes.

### `SeraDFrame.str_endswith`

Masque booleen des valeurs finissant par un suffixe.

### `SeraDFrame.str_extract`

Ajoute {col}_extract, le premier groupe capture regex (ou la correspondance complete).

### `SeraDFrame.str_len`

Ajoute {col}_len, nombre de caracteres par valeur.

### `SeraDFrame.str_lower`

Met une colonne texte en minuscules.

### `SeraDFrame.str_match`

Masque booleen des valeurs correspondant a un motif regex.

### `SeraDFrame.str_replace`

Remplace une sous-chaine dans chaque valeur d'une colonne.

### `SeraDFrame.str_split_get`

Ajoute {col}_splitN, le Nieme champ apres decoupage par separateur.

### `SeraDFrame.str_startswith`

Masque booleen des valeurs commencant par un prefixe.

### `SeraDFrame.str_strip`

Retire les espaces d'une colonne texte.

### `SeraDFrame.str_upper`

Met une colonne texte en majuscules.

### `SeraDFrame.sum`

Somme d'une colonne numerique.

### `SeraDFrame.tail`

Les n dernieres lignes.

### `SeraDFrame.to_csv`

Ecrit dans un fichier CSV.

### `SeraDFrame.to_datetime`

Parse une colonne texte de date/heure ISO en flottants secondes-epoch.

### `SeraDFrame.to_pandas`

Convertit vers un pandas.DataFrame, sans perte.

### `SeraDFrame.to_records`

Vue orientee ligne, liste de dicts.

Alias: `to_dicts`

### `SeraDFrame.to_table`

Passerelle vers le Table de SeraPlot pour les aides d'export chart.

### `SeraDFrame.unique`

Valeurs uniques d'une colonne.

### `SeraDFrame.update`

Remplace les colonnes partagees par les valeurs non manquantes de l'autre frame, ligne par ligne.

### `SeraDFrame.value_counts`

Compte les occurrences par valeur unique.

Alias: `counts`

### `SeraDFrame.values`

Vue ligne par ligne, liste de listes, de chaque cellule.

### `SeraDFrame.where_col`

Garde les valeurs ou une colonne booleenne est vraie, remplace le reste par une constante.

### `SeraDFrame.with_columns`

Ajoute ou remplace plusieurs colonnes en un seul passage.

### `SeraDFrameGroupBy.agg`

Specification d'agregation par colonne.

### `SeraDFrameGroupBy.head`

Les n premieres lignes de chaque groupe.

### `SeraDFrameGroupBy.mean`

Moyenne de chaque colonne numerique par groupe.

### `SeraDFrameGroupBy.nth`

La n-ieme ligne de chaque groupe (index negatif depuis la fin).

### `SeraDFrameGroupBy.size`

Nombre de lignes par groupe.

### `SeraDFrameGroupBy.sum`

Somme de chaque colonne numerique par groupe.

### `SeraDFrameGroupBy.tail`

Les n dernieres lignes de chaque groupe.

### `SeraDFrameGroupBy.transform`

Diffuse une agregation par groupe sur chaque ligne d'origine.

### `Table`

Petit outil de mise en forme de donnees en colonnes construit depuis un dict.

### `Table.columns`

Noms des colonnes, dans l'ordre d'origine.

### `Table.concat`

Union verticale de deux tables.

### `Table.cumsum`

Total cumule.

### `Table.describe`

count/mean/min/max/std par colonne numerique.

### `Table.filter_eq`

Lignes ou une colonne vaut une valeur.

### `Table.filter_gt`

Lignes au-dessus d'une valeur numerique.

### `Table.from_csv`

Lit un CSV en Table.

### `Table.groupby_agg`

Groupe par une colonne, agrege une autre.

Alias: `group_by_agg`

### `Table.head`

Les n premieres lignes.

### `Table.join`

Jointure relationnelle sur une colonne cle.

### `Table.pct_change`

Variation en pourcentage ligne a ligne.

### `Table.pivot`

Passe du format long au format large.

### `Table.rank`

Rang (base 1).

### `Table.rolling_mean`

Moyenne mobile arriere.

### `Table.select`

Un sous-ensemble de colonnes.

### `Table.sort_by`

Copie triee selon une colonne.

Alias: `sort`

### `Table.to_grouped_bar`

Pivote et aplatit vers les entrees de sp.grouped_bar().

### `Table.to_multiline`

Extrait des colonnes vers les entrees de sp.multiline().

### `Table.to_scatter`

Extrait des colonnes vers les entrees de sp.scatter().

### `Table.top_n`

Raccourci pour sort_by(desc).head(n).

### `Table.with_column`

Ajoute une colonne calculee (add/sub/mul/div).

### `Table.zscore`

(x - moyenne) / ecart-type.


## Méthodes Chart (146)

Méthodes appelables sur l'objet `Chart` retourné par n'importe quel builder (ex. `sp.bar(...).legend()`).

### `a11y`

Ajoute des attributs d'accessibilité ARIA (titre et description) à l'élément SVG.

Alias: `accessibility`, `aria`

  | Param | Type | Description |
  |---|---|---|
  | `title` | `str` | Titre accessible pour les lecteurs d'écran. |
  | `desc` | `str` | Description accessible pour les lecteurs d'écran. |

### `animate`

Ajoute une animation d'entrée décalée aux éléments de données (barres, cercles, zones).

Alias: `animation`

  | Param | Type | Description |
  |---|---|---|
  | `duration` | `int` | Durée de l'animation en millisecondes. Défaut: 300. |

### `annotate_max`

Met en évidence et étiquette l'élément de données ayant la valeur la plus élevée, sur les graphiques en barres basés sur des valeurs.

Alias: `mark_max`, `ann_max`

  | Param | Type | Description |
  |---|---|---|
  | `label` | `str | None` | Texte d'étiquette personnalisé optionnel. Par défaut, la valeur elle-même. |
  | `color` | `str` | Couleur de mise en évidence et de l'étiquette. Défaut : #22c55e. |

### `annotate_min`

Met en évidence et étiquette l'élément de données ayant la valeur la plus basse, sur les graphiques en barres basés sur des valeurs.

Alias: `mark_min`, `ann_min`

  | Param | Type | Description |
  |---|---|---|
  | `label` | `str | None` | Texte d'étiquette personnalisé optionnel. Par défaut, la valeur elle-même. |
  | `color` | `str` | Couleur de mise en évidence et de l'étiquette. Défaut : #ef4444. |

### `apply_color_bindings`

Applique les correspondances label→couleur globales (définies via sp.bind_colors()) à ce graphique.

Alias: `apply_bindings`, `color_from_bindings`

### `axis_label_angle`

Fait pivoter indépendamment les étiquettes de graduation des axes X et/ou Y.

Alias: `tick_angle`, `label_angle`

  | Param | Type | Description |
  |---|---|---|
  | `x_angle` | `int | None` | Angle de rotation des étiquettes de l'axe X en degrés. |
  | `y_angle` | `int | None` | Angle de rotation des étiquettes de l'axe Y en degrés. |

### `bar_gap`

Ajuste le ratio d'espacement entre les barres. Des valeurs plus élevées créent des barres plus fines.

Alias: `gap`

  | Param | Type | Description |
  |---|---|---|
  | `ratio` | `float` | Ratio d'espacement entre 0.0 (sans espacement) et 0.95 (presque sans barre). Défaut: 0.3. |

### `bar_scale`

Applique des facteurs d'echelle visuels par barre sans modifier les valeurs de donnees. Un facteur de 2.0 double la hauteur; 0.5 la reduit de moitie. Utilisez le mode SCALE de dev() pour trouver les valeurs.

Alias: `visual_scale`, `bar_heights`, `scale_bars`

  | Param | Type | Description |
  |---|---|---|
  | `factors` | `list[float]` | Facteur d'echelle par barre dans l'ordre des donnees. La longueur doit correspondre au nombre de barres. |

### `border_radius`

Applique un border-radius CSS à l'élément conteneur du graphique.

Alias: `radius`, `corners`

  | Param | Type | Description |
  |---|---|---|
  | `px` | `int` | Rayon des coins en pixels. |

### `box_annotate`

Dessine un résumé boîte-à-moustaches (min, Q1, médiane, Q3, max) des valeurs du graphique sous forme de petit glyphe en marge, comme un boxplot d'accompagnement intégré.

Alias: `boxplot`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Couleur de la boîte, des moustaches et de la médiane. Défaut : #6366f1. |

### `cadrage`

Dessine un cadrage visuel autour de colonnes de heatmap spécifiées : un fond subtil, des séparateurs de colonnes, un rectangle de bordure avec des coins en L, et une animation de zoom en boucle qui pulse le viewBox sur la zone du cluster.

Alias: `frame_cols`, `highlight_cols`, `cluster_frame`

  | Param | Type | Description |
  |---|---|---|
  | `cols` | `list[str]` | Étiquettes de colonnes à encadrer (doivent correspondre aux col_labels de la heatmap). |
  | `rows` | `list[str] | None` | Étiquettes de lignes pour restreindre le cadrage. Défaut : toutes les lignes. |
  | `color` | `str | None` | Couleur du cadre et des séparateurs. Défaut : #6366f1. |

### `caption`

Ajoute une petite légende de bas de page centrée sous le graphique.

Alias: `footnote`

  | Param | Type | Description |
  |---|---|---|
  | `text` | `str` | Texte de la légende. |

### `cluster`

Exécute un clustering par densité DBSCAN et recolore chaque élément selon son cluster, directement dans le navigateur sans dépendance. Fonctionne sur tout type de graphique : utilise x/y pour un scatter, ou se rabat sur la valeur seule pour les barres/lollipops/etc. Les points de bruit (sans voisinage dense) sont colorés en gris.

Alias: `dbscan`

  | Param | Type | Description |
  |---|---|---|
  | `eps` | `float` | Distance maximale (en unités de données) entre deux points pour qu'ils soient considérés voisins. |
  | `min_samples` | `int` | Taille minimale de voisinage (point inclus) pour amorcer un cluster. Défaut : 3. |

### `color_density`

Colore chaque élément de données selon sa valeur sur un gradient de densité : faibles valeurs → bleu/indigo, hautes valeurs → jaune/rouge.

Alias: `density_color`, `gradient_by_value`

### `colorbar`

Ajoute une jauge de couleur (colorbar) montrant la plage de valeurs utilisée pour colorer les points, en 2D comme en 3D. La position peut être 'right' (défaut), 'left', 'top' ou 'bottom'.

Alias: `color_scale`, `heat_legend`, `color_legend`

  | Param | Type | Description |
  |---|---|---|
  | `position` | `str` | 'right', 'left', 'top' ou 'bottom'. |

### `compare_ticker`

Active le mode ticker de comparaison par shift-clic. Shift-cliquez deux graphiques ou plus pour faire apparaître un ticker flottant. Shift-cliquez à nouveau pour désélectionner.

Alias: `cmp_ticker`

### `compose`

Superpose le SVG d'un autre graphique sur celui-ci avec un mode de fusion SVG.

Alias: `overlay`, `blend`, `layer_on`

  | Param | Type | Description |
  |---|---|---|
  | `other` | `Chart` | Le graphique à superposer. |
  | `blend` | `str | None` | Mode de fusion CSS. Défaut 'screen'. |

### `corner_radius_bars`

Applique un rayon de coin à tous les rectangles de barres du graphique.

Alias: `bar_radius`

  | Param | Type | Description |
  |---|---|---|
  | `radius` | `str` | Rayon en pixels sous forme de chaîne ou pourcentage (ex: '8' ou '50%'). |

### `correlation_badge`

Calcule et affiche le coefficient de corrélation de Pearson (r) des valeurs x/y d'un scatter sous forme de petit badge.

Alias: `corr`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Couleur du badge. Défaut : #6366f1. |

### `crosshair`

Ajoute un réticule interactif qui suit le curseur de la souris sur le SVG.

Alias: `xhair`

### `csp_safe`

Supprime les gestionnaires d'événements inline pour rendre le graphique compatible avec les environnements à politique de sécurité de contenu stricte.

Alias: `safe_csp`

### `cumulative_line`

Superpose une courbe de cumul (somme courante sur le total général), la touche finale classique d'un diagramme de Pareto.

Alias: `cumline`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Couleur de la courbe et des marqueurs. Défaut : #6366f1. |

### `cut_bars`

Superpose des coupures blanches horizontales sur chaque barre à intervalles réguliers, créant un effet de barres segmentées / Tufte data-ink.

Alias: `tufte_bars`, `unit_bars`, `bars_cut`

  | Param | Type | Description |
  |---|---|---|
  | `step` | `float | None` | Hauteur en pixels de chaque segment. None = auto (hauteur-graphique / 12). |
  | `gap` | `int` | Hauteur en pixels de la coupure blanche entre les segments (défaut 2). |
  | `color` | `str | None` | Couleur de la coupure (défaut '#ffffff'). Utiliser la couleur de fond du graphique pour un rendu homogène. |

### `delta_labels`

Annote chaque barre avec son évolution en pourcentage par rapport à la barre précédente, pratique pour suivre une croissance (trimestre, mois, ...).

Alias: `deltas`, `pct_change`

  | Param | Type | Description |
  |---|---|---|
  | `pos_color` | `str` | Couleur utilisée pour les évolutions positives. Défaut : #22c55e. |
  | `neg_color` | `str` | Couleur utilisée pour les évolutions négatives. Défaut : #ef4444. |

### `desaturate`

Réduit la saturation de certains éléments de données (par indice) ou de tous si aucun indice fourni. factor=0.0 = niveaux de gris, 1.0 = couleur originale.

Alias: `desat`, `grayscale_elements`

### `despine`

Retire les traits d'axe (spines) tout en gardant les graduations et les libellés (despine façon seaborn).

Alias: `no_spines`

### `dev`

Active le mode dev : rend la legende et le titre depla?ables dans le navigateur, affiche un panneau flottant avec les coordonnees SVG en temps reel, et genere un extrait Python a copier. Une fois les bonnes positions trouvees, remplacez .dev() par .legend_offset(dx, dy) et/ou .title_offset(dx, dy).

Alias: `devmode`, `inspector`, `layout_inspector`

### `diff`

Retourne un diff textuel entre le HTML de ce graphique et celui d'un autre graphique.

Alias: `compare`

  | Param | Type | Description |
  |---|---|---|
  | `other` | `Chart` | L'autre instance Chart à comparer. |

### `diff_from_mean`

Annote chaque barre avec son écart absolu par rapport à la moyenne du jeu de données, à la différence de delta_labels() qui compare au pourcentage de variation par rapport à la barre précédente.

Alias: `mean_diff`

  | Param | Type | Description |
  |---|---|---|
  | `pos_color` | `str` | Couleur utilisée pour les barres au-dessus de la moyenne. Défaut : #22c55e. |
  | `neg_color` | `str` | Couleur utilisée pour les barres en-dessous de la moyenne. Défaut : #ef4444. |

### `dim_above`

Estompe les éléments dont la valeur est strictement supérieure au seuil — ceux en dessous ou égaux restent lumineux. Utile pour mettre en avant les régions de faibles valeurs.

Alias: `dim_over`, `fade_above`, `fade_over`, `highlight_below`, `focus_below`, `dim_high`, `above_threshold`

  | Param | Type | Description |
  |---|---|---|
  | `threshold` | `float` | Seuil de valeur. Les éléments avec valeur <= seuil restent en pleine opacité. |
  | `opacity` | `float` | Opacité des éléments estompés. Défaut 0.2. |

### `dim_below`

Estompe les éléments dont la valeur est strictement inférieure au seuil — ceux au-dessus ou égaux restent lumineux. Pratique pour mettre en avant les meilleures performances.

Alias: `dim_under`, `fade_below`, `fade_under`, `highlight_above`, `focus_above`, `dim_low`, `below_threshold`

  | Param | Type | Description |
  |---|---|---|
  | `threshold` | `float` | Seuil de valeur. Les éléments avec valeur >= seuil restent en pleine opacité. |
  | `opacity` | `float` | Opacité des éléments estompés. Défaut 0.2. |

### `dim_others`

Estompe tous les éléments qui ne sont PAS dans la liste d'indices. Passez une liste d'indices à garder en pleine opacité — tout le reste s'estompe à `opacity`. Chaînable avec dim_below / dim_above.

Alias: `focus`, `downscale`, `highlight_top`, `fade_others`, `keep_only`, `isolate`, `dim_rest`, `show_only`

  | Param | Type | Description |
  |---|---|---|
  | `keep` | `list[int]` | Indices des éléments à garder en pleine opacité. |
  | `opacity` | `float` | Opacité des éléments estompés (0.0–1.0). Défaut 0.2. |

### `downsample`

Réduit les données du graphique en courbes via l'algorithme LTTB pour diminuer l'encombrement visuel.

Alias: `decimate`, `lod`

  | Param | Type | Description |
  |---|---|---|
  | `n` | `int` | Nombre cible de points de données après réduction. |
  | `method` | `str` | Méthode de réduction. Seul 'lttb' est actuellement supporté. |

### `draw_tool`

Active un calque de dessin libre sur le graphique : mode crayon, mode texte, sélecteur de couleur, effacer et téléchargement SVG. Les tracés sont des chemins SVG intégrés dans le graphique et survivent à save().

Alias: `whiteboard`, `sketch`, `paintbrush`, `annotate_tool`, `pen`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Couleur initiale du trait et du texte en CSS hex (défaut '#ff0000'). |

### `error_bars`

Dessine des moustaches symétriques (barres d'erreur) au-dessus et en-dessous du sommet de chaque barre, comme errorbar() ou yerr de matplotlib.

Alias: `errorbar`

  | Param | Type | Description |
  |---|---|---|
  | `margin` | `float` | Marge d'erreur, exprimée dans la même unité que les valeurs du graphique. |
  | `color` | `str` | Couleur des moustaches. Défaut : #475569. |

### `export_button`

Ajoute un bouton de téléchargement flottant au graphique qui sauvegarde le HTML complet au clic.

Alias: `download_button`, `export_btn`, `with_export_button`

### `export_svg`

Enregistre le SVG du graphique dans un fichier.

Alias: `save_svg`, `svg_export`, `write_svg`

  | Param | Type | Description |
  |---|---|---|
  | `path` | `str` | Chemin du fichier .svg de destination. |

### `fill_between`

Colore la zone sous la courbe d'un line chart jusqu'à la ligne de base, comme fill_between(x, y, 0) de matplotlib.

Alias: `fill_area`, `area_fill`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Couleur de remplissage. Défaut : #6366f1. |
  | `opacity` | `float` | Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.15. |

### `flip`

Transforme un graphique à barres verticales en graphique à barres horizontales en recalculant les positions.

Alias: `swap_axes`, `transpose`

### `flourish`

Ajoute une couche de particules décoratives sur le graphique — petits points épars pour une profondeur visuelle signature.

Alias: `particles`, `texture`, `grain`, `dots`

  | Param | Type | Description |
  |---|---|---|
  | `density` | `float` | Densité de particules 0.0–1.0. Défaut 0.4. |
  | `seed` | `int` | Graine aléatoire. Défaut 42. |
  | `color` | `str | None` | Couleur des particules. Défaut '#ffffff'. |

### `focus_mode`

Au survol, estompe toutes les tranches Plotly non survolées (sunburst, pie, treemap) et masque leurs labels — seul le segment survolé et ses ancêtres restent visibles. Double-clic pour réinitialiser. Idéal pour explorer les hiérarchies sunburst multi-niveaux.

Alias: `sunburst_focus`, `layer_focus`, `slice_focus`, `hover_dim_slices`, `plotly_focus`

### `font`

Définit la famille de polices pour tous les textes SVG et les textes du corps du graphique.

Alias: `set_font`, `font_family`

  | Param | Type | Description |
  |---|---|---|
  | `name` | `str` | Nom de la famille de polices (ex: 'Roboto', 'Inter'). |

### `forecast_line`

Extrapole une tendance de régression linéaire au-delà du dernier point de données, avec un trait pointillé et des marqueurs fantômes pour chaque pas de prévision.

Alias: `forecast`, `extrapolate`

  | Param | Type | Description |
  |---|---|---|
  | `periods` | `int` | Nombre de périodes futures à prévoir. Défaut : 3. |
  | `color` | `str` | Couleur du trait et des marqueurs de prévision. Défaut : #8b5cf6. |

### `frames`

Ajoute des controles d'animation par frames avec lecture, pause, pas et slider temporel. Chaque frame est une liste de valeurs qui remplace les hauteurs de barres avec une transition fluide.

Alias: `animate_frames`, `play`, `frame_animation`, `slider_animation`

  | Param | Type | Description |
  |---|---|---|
  | `frames` | `list[list[float]]` | Liste de frames ; chaque frame contient des valeurs de meme longueur que les donnees du graphique. |
  | `speed_ms` | `int` | Duree entre frames en millisecondes. Defaut : 600. |
  | `labels` | `list[str] | None` | Etiquette optionnelle affichee pres du slider pour chaque frame. |

### `glow`

Ajoute une lueur néon (drop-shadow) derrière chaque élément de données (barres, lignes, points).

Alias: `neon`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Couleur de la lueur. Par défaut, la couleur d'accent du graphique (#6366f1). |

### `grid_at`

Trace une ligne de référence horizontale solide à la valeur de données indiquée. Contrairement à hline(), cette ligne est pleine et rendue sous les barres, servant de marqueur de seuil.

Alias: `ref_line`, `baseline`, `threshold_line`, `value_line`

  | Param | Type | Description |
  |---|---|---|
  | `value` | `float` | Valeur de données à laquelle tracer la ligne de référence. |
  | `color` | `str` | Couleur de la ligne en CSS (défaut '#64748b'). |
  | `label` | `str | None` | Étiquette optionnelle affichée à l'extrémité droite de la ligne. |

### `grid_x`

Affiche uniquement les lignes de grille verticales (séparatrices de catégories X), masquant les lignes horizontales.

Alias: `grid_vertical`, `x_grid`

### `grid_y`

Affiche uniquement les lignes de grille horizontales (niveaux de valeur Y), masquant les lignes verticales.

Alias: `grid_horizontal`, `y_grid`

### `group`

Regroupe ce graphique avec tous les autres graphiques partageant le même nom de groupe en une section à onglets navigable, cliquable et interchangeable. Fonctionne pour tout graphique 2D ou 3D, dans un notebook ou n'importe quelle page HTML.

Alias: `tab_group`

  | Param | Type | Description |
  |---|---|---|
  | `name` | `str` | Identifiant de groupe partagé par tous les graphiques qui doivent apparaître ensemble. |
  | `position` | `str` | Où se place la barre d'onglets par rapport au graphique : 'top' (défaut), 'bottom', 'left' ou 'right'. |

### `group_hover_opacity`

Au survol d'un élément de groupe, assombrit tous les éléments hors groupe. Accepte un niveau d'opacité optionnel (0.0–1.0, défaut 0.15).

Alias: `group_hover`, `hover_dim`

### `growth_badge`

Affiche un badge avec la variation totale en % entre la première et la dernière barre — un résumé rapide du taux de croissance.

Alias: `growth`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Couleur du badge. Défaut : #22c55e. |

### `hide_grid`

Masque les lignes de grille si elles étaient précédemment activées.

Alias: `no_grid`

### `highlight`

Met en évidence un seul élément de données par son index avec une couleur distincte et une lueur, sans assombrir le reste.

Alias: `mark`, `spotlight`

  | Param | Type | Description |
  |---|---|---|
  | `index` | `int` | Index (à partir de 0) de l'élément de données à mettre en évidence. |
  | `color` | `str | None` | Couleur de mise en évidence. Défaut : #f59e0b. |

### `highlight_group`

Met en avant de façon statique les éléments dont le label correspond à la liste donnée, en assombrissant les autres au niveau d'opacité spécifié.

Alias: `highlight_labels`, `focus_group`

### `highlight_range`

Colore une plage contiguë de barres/points par index, à la différence de reference_band qui colore par valeur Y.

Alias: `hl_range`, `idx_band`

  | Param | Type | Description |
  |---|---|---|
  | `low` | `int` | Index de début de la plage (inclus). |
  | `high` | `int` | Index de fin de la plage (inclus). |
  | `color` | `str` | Couleur de remplissage de la mise en évidence. Défaut : #6366f1. |
  | `opacity` | `float` | Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.12. |

### `hline`

Trace une ligne de référence/seuil en pointillés à la valeur donnée, sur les graphiques en barres basés sur des valeurs.

Alias: `hrule`, `target_line`

  | Param | Type | Description |
  |---|---|---|
  | `value` | `float` | Valeur de donnée où la ligne est tracée. |
  | `color` | `str` | Couleur de la ligne. Défaut : #ef4444. |
  | `label` | `str | None` | Étiquette optionnelle affichée près de la ligne. |

### `horizontal`

Alias de flip(). Affiche le graphique avec des barres horizontales.

Alias: `hbar`, `flip_h`

### `hover_slots`

Remplace les données de l'infobulle par un tableau JSON de slots. Chaque slot correspond aux champs existants : title (en-tête), kv (lignes clé/valeur), image (url ou data-uri — GIFs supportés), video (url), html (bloc HTML brut). Un slot par point de données.

Alias: `set_hover`, `hover_data`, `tooltip_slots`, `with_hover`, `hover_json`

  | Param | Type | Description |
  |---|---|---|
  | `slots_json` | `str` |  |

### `hstack`

Place deux graphiques côte à côte dans un seul conteneur HTML — le layout multi-panneau fondamental.

Alias: `side_by_side`, `columns`, `two_col`

  | Param | Type | Description |
  |---|---|---|
  | `other` | `Chart` | Le second graphique à placer à droite. |
  | `gap` | `int` | Écart entre graphiques en pixels. Défaut 16. |

### `image_bg`

Injecte une image de fond (URL ou data URI) derrière le contenu SVG du graphique.

Alias: `bg_image`, `background_image`

  | Param | Type | Description |
  |---|---|---|
  | `url` | `str` | URL ou data URI base64. |
  | `opacity` | `float` | Opacité 0.0–1.0. Défaut 0.35. |

### `inject_css`

Injecte une chaîne CSS brute dans l'élément <head> du graphique.

Alias: `css`, `custom_css`

  | Param | Type | Description |
  |---|---|---|
  | `css` | `str` | Règles CSS brutes à injecter. |

### `inject_js`

Injecte une chaîne JavaScript brute dans l'élément <body> du graphique.

Alias: `js`, `custom_js`

  | Param | Type | Description |
  |---|---|---|
  | `js` | `str` | Code JavaScript brut à injecter. |

### `iqr_band`

Colore une bande horizontale couvrant l'écart interquartile (Q1 à Q3) des valeurs du graphique, pour voir d'un coup d'œil quelles barres se trouvent dans les 50% centraux 'normaux'.

Alias: `iqr`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Couleur de remplissage de la bande. Défaut : #6366f1. |
  | `opacity` | `float` | Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.10. |

### `label_position`

Alias de show_labels(position). Ajoute une superposition de légende interactive à la position donnée.

Alias: `labels_at`

  | Param | Type | Description |
  |---|---|---|
  | `position` | `str` | Position de la superposition: 'top', 'bottom', 'left', 'right'. |

### `lasso`

Active la selection rectangulaire sur les nuages de points. Shift+drag selectionne les points dans un rectangle ; les autres points sont attenues. Shift+clic dans une zone vide efface la selection.

Alias: `box_select`, `rect_select`, `rubber_band`

### `legend`

Repositionne la légende du graphique et active la bascule interactive des séries au clic.

Alias: `show_leg`

  | Param | Type | Description |
  |---|---|---|
  | `position` | `str` | Position de la légende: 'right' (défaut), 'left', 'top', 'bottom', 'none'. |

### `legend_offset`

Deplace la legende de (dx, dy) pixels en coordonnees SVG par rapport a sa position par defaut. Utilisez dev() pour trouver les bonnes valeurs interactivement.

Alias: `shift_legend`, `move_legend`

  | Param | Type | Description |
  |---|---|---|
  | `dx` | `int` | Decalage horizontal en pixels SVG. |
  | `dy` | `int` | Decalage vertical en pixels SVG. |

### `legend_position`

Alias de legend(position). Repositionne la légende du graphique.

Alias: `leg_pos`

  | Param | Type | Description |
  |---|---|---|
  | `position` | `str` | Position de la légende: 'right', 'left', 'top', 'bottom', 'none'. |

### `LiveStream`

Un accumulateur à buffer circulaire borné qui transforme une série d'échantillons (x, y) arrivant dans le temps en un graphique redessiné en continu. push()/extend()/clear() repeignent la même cellule de sortie Jupyter sur place via le display_id d'IPython, donc le graphique se met à jour sans scintillement ni nouvelle cellule.

  | Param | Type | Description |
  |---|---|---|
  | `kind` | `str` | Type de graphique : 'line' ou 'scatter'. Défaut : 'line'. |
  | `title` | `str` | Titre du graphique. |
  | `max_points` | `int` | Taille du buffer circulaire ; les échantillons les plus anciens sont supprimés au-delà. Défaut : 500. |
  | `color_hex` | `int` | Couleur de la série en entier RGB 24 bits. |
  | `width` | `int` | Largeur du canvas en pixels. |
  | `height` | `int` | Hauteur du canvas en pixels. |

### `LiveStream.clear`

Vide le buffer circulaire et redessine le graphique live (désormais vide) sur place.

Alias: `reset`

### `LiveStream.extend`

Ajoute deux listes d'échantillons en parallèle, applique max_points, et redessine le graphique live sur place.

Alias: `add_points`, `append_many`

  | Param | Type | Description |
  |---|---|---|
  | `xs` | `list[float]` | Valeurs X. |
  | `ys` | `list[float]` | Valeurs Y. |

### `LiveStream.push`

Ajoute un échantillon (x, y), applique max_points, et redessine le graphique live sur place.

Alias: `append`, `add_point`

  | Param | Type | Description |
  |---|---|---|
  | `x` | `float` | Valeur X. |
  | `y` | `float` | Valeur Y. |

### `LiveStream.render`

Rend le buffer courant sous forme d'objet Chart autonome, sans toucher à l'affichage live.

Alias: `snapshot`, `to_chart`

### `log_scale`

Passe l'axe des valeurs en échelle logarithmique, comme plt.yscale('log') de matplotlib — idéal pour des données s'étalant sur plusieurs ordres de grandeur.

Alias: `logy`

### `mask_circle`

Découpe le graphique en cercle via un clipPath SVG.

Alias: `circle_mask`, `round_mask`, `clip_circle`

  | Param | Type | Description |
  |---|---|---|
  | `cx` | `float | None` | Centre X en fraction de la largeur. Défaut 0.5. |
  | `cy` | `float | None` | Centre Y en fraction de la hauteur. Défaut 0.5. |
  | `r` | `float | None` | Rayon en fraction de min(l,h)/2. Défaut 0.48. |

### `mask_poly`

Découpe le graphique selon un polygone régulier via clipPath SVG.

Alias: `polygon_mask`, `hex_mask`, `shape_mask`

  | Param | Type | Description |
  |---|---|---|
  | `sides` | `int` | Nombre de côtés (3=triangle, 6=hexagone…). |
  | `rotation` | `float` | Rotation en degrés. Défaut 0. |

### `mean`

Trace une ligne horizontale en pointillés au niveau de la moyenne, calculée à partir des données du graphique (fonctionne sur les barres, scatter et autres graphiques à valeurs).

Alias: `mean_line`, `avg_line`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Couleur de la ligne. Défaut : #ef4444. |

### `median`

Trace une ligne horizontale en pointillés au niveau de la médiane, calculée à partir des données du graphique (fonctionne sur les barres, scatter et autres graphiques à valeurs).

Alias: `median_line`, `med_line`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Couleur de la ligne. Défaut : #6366f1. |

### `morph_bg`

Anime le fond du graphique via une transition de dégradé — morphing cinématique qui donne vie aux données.

Alias: `gradient_bg`, `animated_bg`, `morph_gradient`

  | Param | Type | Description |
  |---|---|---|
  | `from_color` | `str` | Couleur de départ (CSS). Défaut '#0f172a'. |
  | `to_color` | `str` | Couleur d'arrivée (CSS). Défaut '#1e1b4b'. |
  | `duration` | `float` | Durée de l'animation en secondes. Défaut 4.0. |

### `moving_average`

Superpose une courbe de moyenne mobile (fenêtre glissante), comme .rolling(window).mean() de pandas — lisse une série bruitée pour révéler la tendance sous-jacente.

Alias: `rolling_mean`

  | Param | Type | Description |
  |---|---|---|
  | `window` | `int` | Nombre de points (précédents) sur lesquels moyenner. |
  | `color` | `str` | Couleur de la courbe. Défaut : #f59e0b. |

### `neon_bloom`

Ajoute un effet neon bloom multicouche aux elements SVG — 3 passes feGaussianBlur creent une lueur lumineuse profonde autour des barres, lignes et cercles.

Alias: `bloom`, `neon_glow`, `plasma`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Couleur de lueur en hexadecimal. Defaut : #6366f1. |

### `nice_x_axis`

Reformate les étiquettes de l'axe x en valeurs arrondies. Si step est fourni, les ticks deviennent 0, step, 2*step… ; sinon chaque tick est arrondi au step automatique.

Alias: `round_x_ticks`, `x_ticks_nice`, `format_x_axis`

  | Param | Type | Description |
  |---|---|---|
  | `step` | `float | None` | Intervalle uniforme ou None pour auto. |

### `no_axes`

Masque les axes X et Y ainsi que leurs graduations et étiquettes.

Alias: `hide_axes`, `no_axis`, `hide_axis`, `remove_axes`, `drop_axes`

### `no_background`

Supprime toutes les couches d'arrière-plan du graphique et conserve une sortie transparente.

Alias: `transparent_bg`, `no_bg`

### `no_hover`

Désactive l'infobulle au survol et supprime le surlignage des éléments au survol.

Alias: `no_tooltip`

### `no_legend`

Masque la légende du graphique.

Alias: `hide_legend`

### `no_select`

Désactive le rectangle de sélection par glisser du graphique et la sélection de texte — à combiner avec draw_tool() pour que le glisser ne dessine que, sans activer la sélection.

Alias: `disable_select`, `prevent_select`, `unselectable`, `no_text_select`

### `no_title`

Masque le titre du graphique.

Alias: `hide_title`

### `no_x_axis`

Masque l'axe X, ses graduations et son étiquette.

Alias: `hide_x`, `no_x`, `remove_x`, `hide_x_axis`, `drop_x`

### `no_y_axis`

Masque l'axe Y, ses graduations et son étiquette.

Alias: `hide_y`, `no_y`, `remove_y`, `hide_y_axis`, `drop_y`

### `orient3d`

Définit l'orientation initiale de la caméra d'un graphique 3D : 'iso' (défaut), 'horizontal' (vue de côté), 'vertical' (vue de dessus) ou 'front'.

Alias: `orientation3d`, `tilt3d`, `rotate3d`

  | Param | Type | Description |
  |---|---|---|
  | `mode` | `str` | 'iso', 'horizontal', 'vertical' ou 'front'. |

### `outliers`

Signale les valeurs aberrantes statistiques avec un marqueur d'avertissement et une couleur de mise en évidence. Fonctionne sur tout type de graphique : barres/lollipops/etc sont signalés par z-score sur leur valeur, les points de scatter par distance résiduelle à leur droite de régression.

Alias: `flag_outliers`, `anomalies`, `scatter_outliers`

  | Param | Type | Description |
  |---|---|---|
  | `threshold_std` | `float` | Nombre d'écarts-types au-delà duquel une valeur est signalée. Défaut : 2.0. |
  | `color` | `str` | Couleur de mise en évidence des barres signalées. Défaut : #ef4444. |

### `outline`

Trace un contour coloré et net autour de chaque élément de données, sans flou comme glow().

Alias: `border`, `stroke`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Couleur du contour. Par défaut, la couleur d'accent du graphique (#6366f1). |
  | `width` | `float` | Épaisseur du contour en pixels. Défaut : 2.0. |

### `pareto_marker`

Marque la barre à partir de laquelle le total cumulé dépasse un pourcentage seuil (80% par défaut), le point de coupure Pareto '80/20' classique. Se combine bien avec cumulative_line().

Alias: `pareto`, `eighty_twenty`

  | Param | Type | Description |
  |---|---|---|
  | `threshold_pct` | `float` | Pourcentage cumulé seuil à marquer. Défaut : 80.0. |
  | `color` | `str` | Couleur du trait et du label du marqueur. Défaut : #ef4444. |

### `pct_of_total`

Annote chaque barre avec sa part du total (%) plutôt que sa valeur brute.

Alias: `pct`

  | Param | Type | Description |
  |---|---|---|
  | `decimals` | `int` | Nombre de décimales à afficher. Défaut : 1. |
  | `color` | `str` | Couleur du texte du label. Défaut : #475569. |

### `percentile_band`

Colore une bande horizontale entre deux percentiles arbitraires des valeurs du graphique, une généralisation de iqr_band pour toute plage personnalisée (ex : 5e-95e).

Alias: `pct_band`

  | Param | Type | Description |
  |---|---|---|
  | `low_pct` | `float` | Percentile bas (0-100). |
  | `high_pct` | `float` | Percentile haut (0-100). |
  | `color` | `str` | Couleur de remplissage de la bande. Défaut : #6366f1. |
  | `opacity` | `float` | Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.10. |

### `pick`

Active la comparaison d'éléments dans le graphique par shift-clic. Shift-cliquez deux barres ou points ou plus — les éléments non sélectionnés s'estompent à 18% d'opacité, et un badge au-dessus affiche les valeurs sélectionnées classées avec >, >>, >>>. Shift-cliquez à nouveau pour désélectionner.

Alias: `select_compare`, `element_compare`

### `png_button`

Ajoute un bouton de téléchargement PNG au graphique. Au clic, le SVG est rasterise dans un canvas puis telecharge en PNG sans serveur.

Alias: `export_png_button`, `download_png`, `png_download`

### `pulse`

Fait pulser des éléments de données, pour attirer l'attention (ex : tableaux de bord en direct). Par défaut, pulse tous les éléments ; cible des éléments précis avec index, ou tout élément au-dessus d'une valeur avec above.

Alias: `blink`, `beacon`

  | Param | Type | Description |
  |---|---|---|
  | `duration` | `float` | Durée du cycle de pulsation en secondes. Défaut : 2.0. |
  | `index` | `list[int] | None` | Index (à partir de 0) des éléments à faire pulser (ex : [0] pour le premier). Par défaut, tous les éléments. |
  | `above` | `float | None` | Ne fait pulser que les éléments de données dont la valeur dépasse ce seuil. |
  | `color` | `str | None` | Force les éléments à clignoter dans cette couleur. Par défaut None, ce qui conserve la couleur normale de chaque élément et ne fait pulser que son opacité. |

### `range_buttons`

Ajoute des boutons de plage predefinis 1M / 6M / YTD / 1A / Tout sous le graphique. Si les etiquettes des points se parsent comme des dates (Date.parse natif, sans dependance externe), les boutons filtrent par vraie plage calendaire depuis le dernier point ; sinon ils retombent sur une fenetre proportionnelle equivalente sur la plage d'index.

Alias: `time_range_buttons`, `date_buttons`, `range_selector`, `quick_range`

### `range_slider`

Ajoute un curseur de plage a deux poignees sous le graphique. Les poignees filtrent les elements visibles par indice.

Alias: `time_slider`, `slider`, `filter_range`, `date_range`

### `rank_badges`

Épingle des badges de classement numérotés (1, 2, 3, ...) sur les N barres aux valeurs les plus élevées, pour une lecture rapide façon classement.

Alias: `ranks`

  | Param | Type | Description |
  |---|---|---|
  | `top_n` | `int` | Nombre de barres (les plus élevées) à marquer d'un badge. Défaut : 3. |
  | `color` | `str` | Couleur de remplissage du badge. Défaut : #6366f1. |

### `reference_band`

Trace une bande de référence ombrée entre deux valeurs, sur les graphiques en barres basés sur des valeurs (ex : une plage cible).

Alias: `ref_band`, `target_zone`

  | Param | Type | Description |
  |---|---|---|
  | `low` | `float` | Borne basse de la bande. |
  | `high` | `float` | Borne haute de la bande. |
  | `color` | `str` | Couleur de remplissage de la bande. Défaut : #f59e0b. |
  | `opacity` | `float` | Opacité de la bande entre 0.0 et 1.0. Défaut : 0.12. |

### `responsive`

Rend la largeur du SVG égale à 100% de son conteneur tout en conservant une hauteur proportionnelle.

Alias: `fluid`, `auto_size`

### `rolling_std_band`

Superpose une zone ombrée façon bandes de Bollinger (moyenne mobile ± écart-type mobile) sur une fenêtre glissante, l'enveloppe de volatilité classique des graphiques financiers.

Alias: `bollinger_band`, `volatility_band`

  | Param | Type | Description |
  |---|---|---|
  | `window` | `int` | Nombre de points (précédents) utilisés pour la moyenne et l'écart-type mobiles. |
  | `color` | `str` | Couleur de remplissage de la bande. Défaut : #6366f1. |
  | `opacity` | `float` | Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.15. |

### `rotate`

Fait pivoter l'intégralité du graphique selon un angle arrondi (0, 90, 180 ou 270 degrés).

Alias: `spin`

  | Param | Type | Description |
  |---|---|---|
  | `deg` | `int` | Rotation en degrés, arrondie au 90° le plus proche. Défaut: 90. |

### `rotate_labels`

Fait pivoter les étiquettes de graduation de l'axe X de l'angle spécifié en degrés.

Alias: `tilt_labels`

  | Param | Type | Description |
  |---|---|---|
  | `angle` | `int` | Angle de rotation en degrés (ex: -45 pour des étiquettes diagonales). |

### `save`

Enregistre le HTML du graphique dans un fichier au chemin indiqué.

Alias: `save_html`, `write`, `export_html`

  | Param | Type | Description |
  |---|---|---|
  | `path` | `str` | Chemin du fichier de destination (ex: 'chart.html'). |

### `scale`

Met à l'échelle l'intégralité du SVG par un facteur donné depuis le coin supérieur gauche.

Alias: `resize`

  | Param | Type | Description |
  |---|---|---|
  | `factor` | `float` | Multiplicateur d'échelle (ex: 1.5 pour 150%). |

### `scatter_regression`

Calcule et trace une droite de régression linéaire à travers les points d'un scatter, comme regplot() de seaborn ou numpy.polyfit(x, y, 1).

Alias: `regression_line`, `lmplot`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Couleur de la droite. Défaut : #ef4444. |
  | `width` | `float` | Épaisseur de la droite. Défaut : 2.0. |

### `secondary_y`

Superpose une deuxieme serie en ligne sur un axe Y droit independant. Utile pour comparer des series avec des echelles differentes sur le meme graphique.

Alias: `y2`, `right_axis`, `dual_axis`, `twin_axis`, `second_axis`

  | Param | Type | Description |
  |---|---|---|
  | `values` | `list[float]` | Valeurs de la serie secondaire. Doit correspondre a la longueur de la serie principale. |
  | `color` | `str | None` | Couleur de la ligne et de l'axe. Defaut : #f59e0b. |
  | `label` | `str | None` | Etiquette optionnelle pour l'axe droit. |

### `segment_bars`

Remplace la grille par des segments blancs.

### `set_bg`

Définit la couleur d'arrière-plan du graphique. Passez None pour supprimer l'arrière-plan.

Alias: `bg`, `background`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Couleur CSS (hex, rgb, nommée). None supprime l'arrière-plan. |

### `set_font_size`

Remplace la taille de police de tous les éléments texte SVG par la valeur spécifiée en pixels.

Alias: `font_size`

  | Param | Type | Description |
  |---|---|---|
  | `px` | `int` | Taille de police en pixels. |

### `set_frame`

Définit la couleur d'arrière-plan du cadre SVG/canvas. Utilisez 'transparent' ou None pour le supprimer.

Alias: `frame`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Couleur CSS pour l'arrière-plan du cadre. |

### `set_margin`

Ajoute un espacement interne au graphique et ajuste en conséquence les positions des éléments de données.

Alias: `margin`

  | Param | Type | Description |
  |---|---|---|
  | `px` | `int` | Marge en pixels appliquée aux quatre côtés. |

### `set_opacity`

Définit l'opacité de tous les éléments de données (barres, cercles, zones) du graphique.

Alias: `opacity`, `alpha`

  | Param | Type | Description |
  |---|---|---|
  | `value` | `float` | Opacité entre 0.0 (invisible) et 1.0 (totalement opaque). |

### `set_padding`

Applique un padding CSS à l'élément conteneur du graphique.

Alias: `padding`

  | Param | Type | Description |
  |---|---|---|
  | `px` | `int` | Padding en pixels appliqué aux quatre côtés. |

### `shadow`

Ajoute une ombre douce derrière tout le conteneur du graphique.

Alias: `drop_shadow`

  | Param | Type | Description |
  |---|---|---|
  | `blur` | `int` | Rayon de flou de l'ombre en pixels. Défaut : 24. |
  | `color` | `str | None` | Couleur de l'ombre. Par défaut, une ombre neutre sombre. |

### `show_grid`

Affiche les lignes de grille horizontales et verticales en arrière-plan du graphique.

Alias: `grid`

### `show_labels`

Ajoute une superposition de filtre de séries interactif avec des étiquettes en forme de pilule cliquables.

Alias: `labels`, `legend_labels`

  | Param | Type | Description |
  |---|---|---|
  | `position` | `str` | Position de la superposition: 'top', 'bottom', 'left', 'right'. |
  | `labels` | `list[str] | None` | Noms d'étiquettes personnalisés. Détection automatique si None. |
  | `colors` | `list[str] | None` | Couleurs hex personnalisées correspondant aux étiquettes. |

### `show_legend`

Force la légende du graphique à être visible même si elle était masquée.

Alias: `leg`

### `show_title`

Force le titre du graphique à être visible avec un contour de contraste pour la lisibilité.

Alias: `title_on`

### `sort_by`

Trie les barres du graphique par valeur ou étiquette via un rendu JavaScript côté client.

Alias: `sort`, `order_by`

  | Param | Type | Description |
  |---|---|---|
  | `order` | `str` | Ordre de tri: 'desc' (défaut), 'asc', 'alpha', 'alpha_desc', 'none'. |

### `sparse_grid`

Convertit les lignes de grille pleines en pointillés, réduisant le poids visuel (principe data-ink de Tufte).

Alias: `dashed_grid`, `dotted_grid`

### `stamp`

Tampon de texte artistique grand format sur le graphique — filigrane, label ou annotation style Visual Cinnamon.

Alias: `watermark_text`, `label_stamp`, `big_text`

  | Param | Type | Description |
  |---|---|---|
  | `text` | `str` | Texte à tamponner. |
  | `x` | `float` | Position X en % de la largeur SVG. Défaut 50. |
  | `y` | `float` | Position Y en % de la hauteur SVG. Défaut 50. |
  | `size` | `float` | Taille de police en px. Défaut 72. |
  | `color` | `str` | Couleur du texte. Défaut '#ffffff'. |
  | `opacity` | `float` | Opacité 0.0–1.0. Défaut 0.08. |

### `subtitle`

Ajoute un petit sous-titre directement sous le titre du graphique.

Alias: `sub`

  | Param | Type | Description |
  |---|---|---|
  | `text` | `str` | Texte du sous-titre. |

### `text_angle`

Définit l'angle de rotation des étiquettes de valeurs de données.

Alias: `rotate_text`

  | Param | Type | Description |
  |---|---|---|
  | `degrees` | `int` | Angle de rotation en degrés. |

### `text_auto`

Superpose des étiquettes de valeurs de données sur tous les éléments du graphique. Supporte les chaînes de format et le positionnement.

Alias: `auto_text`, `data_labels`

  | Param | Type | Description |
  |---|---|---|
  | `format` | `str | None` | Chaîne de format (ex: '.2f', '.0%') ou chaîne vide pour automatique. None désactive. |
  | `position` | `str | None` | Position de l'étiquette: 'auto', 'inside', 'outside'. |
  | `angle` | `int | None` | Angle de rotation des étiquettes en degrés. |
  | `font_size` | `int | None` | Taille de police des étiquettes de données en pixels. |
  | `color` | `str | None` | Couleur des étiquettes de données. |

### `text_font`

Définit la famille de polices, la taille et la couleur des étiquettes de valeurs de données.

Alias: `font_text`

  | Param | Type | Description |
  |---|---|---|
  | `family` | `str | None` | Nom de la famille de polices. |
  | `size` | `int | None` | Taille de police en pixels. |
  | `color` | `str | None` | Couleur du texte des étiquettes. |

### `text_position`

Définit la position des étiquettes de valeurs de données sur les éléments du graphique.

Alias: `text_pos`

  | Param | Type | Description |
  |---|---|---|
  | `position` | `str` | Position: 'auto', 'inside', 'outside'. |

### `title_color`

Définit la couleur du titre d'un graphique 3D (fonctionne uniquement sur les graphiques canvas 3D). Défaut : blanc.

Alias: `ttl_color`, `tc`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Couleur CSS pour le texte du titre. |

### `title_offset`

Deplace le titre du graphique de (dx, dy) pixels en coordonnees SVG par rapport a sa position par defaut. Utilisez dev() pour trouver les bonnes valeurs interactivement.

Alias: `shift_title`, `move_title`

  | Param | Type | Description |
  |---|---|---|
  | `dx` | `int` | Decalage horizontal en pixels SVG. |
  | `dy` | `int` | Decalage vertical en pixels SVG. |

### `title_size`

Définit la taille de police du titre du graphique en pixels.

Alias: `ttl_size`

  | Param | Type | Description |
  |---|---|---|
  | `px` | `int` | Taille de police du titre en pixels. |

### `to_svg`

Extrait et retourne la chaîne SVG brute depuis le HTML du graphique, ou None si absente.

Alias: `svg`, `as_svg`, `get_svg`

### `trace`

Trace une ligne sur le graphique. Coordonnées fractionnelles pour lignes droites ; `values` pour une superposition de polyline ; `vertical=True` pour une règle verticale. Chaînable.

Alias: `draw_line`, `draw`, `overlay_line`, `rule`, `add_line`, `annotate_line`, `line`, `guideline`, `marker_line`

  | Param | Type | Description |
  |---|---|---|
  | `x1` | `float` | X de départ (0.0–1.0). Position de la règle verticale si vertical=True. |
  | `y1` | `float` | Y de départ (0.0 = bas, 1.0 = haut). |
  | `x2` | `float` | X de fin (0.0–1.0). |
  | `y2` | `float` | Y de fin (0.0 = bas, 1.0 = haut). |
  | `color` | `str | None` | Couleur de la ligne. Défaut : #ef4444. |
  | `dash` | `str | None` | Style du trait : 'solid' (défaut), 'dashed' ou 'dotted'. |
  | `width` | `float` | Épaisseur du trait en pixels. Défaut : 2.0. |
  | `vertical` | `bool` | Si True, trace une règle verticale en x1 de y1 à y2 (hauteur totale par défaut). |
  | `values` | `list[float] | None` | Valeurs à tracer comme polyline superposée, mappées sur l'axe Y du graphique et distribuées sur la plage X. |

### `trendline`

Superpose une droite de régression linéaire calculée à partir des valeurs du graphique, sur les graphiques en barres basés sur des valeurs.

Alias: `trend`, `regline`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str` | Couleur de la droite de tendance. Défaut : #10b981. |
  | `width` | `float` | Épaisseur de la droite de tendance en pixels. Défaut : 2.0. |

### `uniform_text`

Impose une taille de police minimale pour les étiquettes de données; masque ou affiche les étiquettes qui ne tiennent pas.

Alias: `uniform`

  | Param | Type | Description |
  |---|---|---|
  | `min_size` | `int` | Taille de police minimale en pixels. |
  | `mode` | `str` | Comportement quand l'étiquette ne tient pas: 'hide' ou 'show'. |

### `value_labels`

Affiche la valeur numérique au-dessus de chaque barre, comme bar_label() de matplotlib ou textposition='auto' de Plotly.

Alias: `vlabels`, `bar_labels`

  | Param | Type | Description |
  |---|---|---|
  | `decimals` | `int` | Nombre de décimales à afficher. Défaut : 0. |
  | `color` | `str` | Couleur du texte du label. Défaut : #475569. |

### `void_bg`

Applique un fond sombre avec des particules stellaires animees injectees dans le SVG — transforme tout graphique en scene spatiale cinematique.

Alias: `dark_bg`, `space_bg`, `cosmos_bg`, `night_bg`

  | Param | Type | Description |
  |---|---|---|
  | `color` | `str | None` | Couleur de fond de base. Defaut : #07090f. |

### `vstack`

Empile deux graphiques verticalement dans un seul conteneur HTML.

Alias: `stack`, `rows`, `two_row`

  | Param | Type | Description |
  |---|---|---|
  | `other` | `Chart` | Le second graphique à placer en dessous. |
  | `gap` | `int` | Écart entre graphiques en pixels. Défaut 16. |

### `watermark`

Superpose un grand texte en filigrane diagonal sur tout le graphique, pour le branding ou les brouillons.

Alias: `wm`

  | Param | Type | Description |
  |---|---|---|
  | `text` | `str` | Texte du filigrane. |
  | `opacity` | `float` | Opacité du filigrane entre 0.0 et 1.0. Défaut : 0.08. |

### `x_labels_rotation`

Fait pivoter les etiquettes de l'axe X de l'angle donne en degres. Positif = sens horaire. Utilisez dev() pour trouver la bonne valeur.

Alias: `rotate_x_labels`, `x_label_angle`, `tilt_x`

  | Param | Type | Description |
  |---|---|---|
  | `degrees` | `int` | Angle de rotation en degres (-90 a 90). |

### `y_labels_rotation`

Fait pivoter les etiquettes de l'axe Y de l'angle donne en degres. Utilisez dev() pour trouver la bonne valeur.

Alias: `rotate_y_labels`, `y_label_angle`, `tilt_y`

  | Param | Type | Description |
  |---|---|---|
  | `degrees` | `int` | Angle de rotation en degres (-90 a 90). |

### `zoom`

Active le zoom à la molette et le déplacement par glisser-cliquer. Double-clic pour réinitialiser.

Alias: `magnify`

### `zscore_heat`

Recolore chaque barre sur une échelle divergente bleu-blanc-rouge selon son z-score, transformant le graphique en vue façon heatmap des valeurs anormalement hautes ou basses.

Alias: `zheat`, `heatbars`


</div>
