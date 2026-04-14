# Chart Methods

These methods are available on every `Chart` object returned by any SeraPlot
function. They all return a new `Chart`, so they can be chained freely.

---

## Background and frame

### `set_bg(color=None)`

Sets the background color of the full HTML wrapper.

```python
chart = sp.build_bar_chart("Sales", labels, values).set_bg("#0f172a")
chart = sp.build_scatter_chart("Data", x, y).set_bg("white")
chart = sp.build_pie_chart("Share", labels, values).set_bg(None)  # transparent
```

Accepts any CSS color string: `"#rrggbb"`, `"rgb(r,g,b)"`, named colors, or
`None`/`"transparent"`.

---

### `set_frame(color=None)`

Sets the SVG canvas background independently of the HTML wrapper. Useful when
embedding the chart inside a page with its own background.

```python
chart = sp.build_bar_chart("Title", labels, values).set_frame("transparent")
```

---

### `set_global_background(color)`

Module-level functions that apply a background to **all** charts created after
the call — useful for notebook sessions with a consistent theme.

```python
sp.set_global_background("#0f172a")   # all subsequent charts use this bg
chart1 = sp.build_bar_chart(...)
chart2 = sp.build_scatter_chart(...)

sp.reset_global_background()          # restore per-chart default
```

---

## Grid and axes

### `show_grid()`

Force gridlines on regardless of the `gridlines` parameter used at chart creation.

```python
chart = sp.build_histogram("Ages", values).show_grid()
```

---

### `hide_grid()`

Force gridlines off regardless of the `gridlines` parameter used at chart creation.

```python
chart = sp.build_line_chart("Trend", labels, values, gridlines=True).hide_grid()
```

---

### `no_x_axis()`

Removes the X-axis lines, ticks, and labels.

```python
chart = sp.build_bar_chart("", labels, values).no_x_axis()   # keep Y only
```

---

### `no_y_axis()`

Removes the Y-axis lines, ticks, and labels.

```python
chart = sp.build_bar_chart("", labels, values).no_y_axis()   # keep X only
```

---

### `no_axes()`

Removes both axes at once.

```python
chart = sp.build_scatter_chart("", x, y).no_axes()           # remove both axes
```

---

## Labels and text

### `show_labels(position="bottom", labels=None, colors=None)`

Renders a text label on each chart element. `position` can be `"top"`,
`"bottom"`, `"left"`, or `"right"`.

```python
# Automatic labels derived from the chart data
chart = sp.build_bar_chart("Revenue", labels, values).show_labels(position="top")

# Custom label text per element
chart = sp.build_bar_chart("Revenue", labels, values).show_labels(
    position="top",
    labels=["$142k", "$98k", "$210k"],
    colors=["#22c55e", "#ef4444", "#22c55e"],
)
```

---

### `no_title()`

Removes the chart title from the rendered output.

```python
chart = sp.build_pie_chart("Internal label", labels, values).no_title()
```

---

### `no_legend()`

Removes the legend from the rendered output.

```python
chart = sp.build_grouped_bar("Q1", cats, series).no_legend()
```

---

### `set_font_size(px)`

Override all text sizes in the SVG with a single pixel value.

```python
chart = sp.build_radar_chart("Skills", labels, values).set_font_size(11)
```

---

## Size and scale

### `scale(factor)`

Scales the entire chart (SVG and canvas). Useful to produce thumbnail or
high-DPI variants.

```python
small = sp.build_bar_chart("Sales", labels, values).scale(0.5)
large = sp.build_bar_chart("Sales", labels, values).scale(2.0)
```

---

## Hover

### `no_hover()`

Disables the tooltip engine. All `data-idx` pointer events are removed. Useful
for static embeds where hover interaction is unwanted.

```python
chart = sp.build_scatter_chart("Distribution", x, y).no_hover()
```

---

## CSS and JavaScript injection

### `inject_css(css)`

Injects a `<style>` block into the HTML `<head>`. Gives complete access to the
SVG DOM — override any internal class, change colors, animations, fonts.

SeraPlot's internal CSS classes:

| Class| Target |
|-----------------|--------|
| `svg text` | All text in the chart |
| `.sp-gl` | Gridlines |
| `.sp-ax-x`, `.sp-ax-y` | Axis lines |
| `.sp-xt`, `.sp-yt` | Axis tick labels |
| `.sp-xl`, `.sp-yl` | Axis title labels |
| `g[data-legend]` | Legend group |
| `.sp-ttl` | Chart title |
| `[data-idx]` | Interactive data elements (hover targets) |
| `rect.sp-bg` | SVG background rect |

```python
chart = sp.build_bar_chart("Dark theme", labels, values).inject_css("""
    rect.sp-bg  { fill: #0f172a !important; }
    svg text    { fill: #e2e8f0 !important; }
    .sp-gl      { stroke: #1e293b !important; }
    [data-idx]  { opacity: 0.85; }
    [data-idx]:hover { filter: brightness(1.3); }
""")
```

---

### `inject_js(js)`

Injects a `<script>` block before `</body>`. The entire rendered SVG DOM is
accessible. Use it to add event listeners, animations, or third-party integrations.

```python
# Highlight every bar on load
chart = sp.build_bar_chart("Sales", labels, values).inject_js("""
    document.querySelectorAll('[data-idx]').forEach((el, i) => {
        setTimeout(() => el.style.opacity = '1', i * 50);
    });
""")

# Export SVG on button click
chart = sp.build_scatter_chart("Data", x, y).inject_js("""
    const btn = document.createElement('button');
    btn.textContent = 'Download SVG';
    btn.onclick = () => {
        const svg  = document.querySelector('svg').outerHTML;
        const a    = document.createElement('a');
        a.href     = 'data:image/svg+xml;charset=utf-8,' + encodeURIComponent(svg);
        a.download = 'chart.svg';
        a.click();
    };
    document.body.appendChild(btn);
""")
```

---

## Export

### `save(path)`

Writes the HTML to a file.

```python
chart = sp.build_pie_chart("Share", labels, values)
chart.save("report/pie.html")
```

---

### `to_svg()`

Extracts the raw SVG string from the HTML.

```python
svg_string = chart.to_svg()
```

---

### `export_svg(path)`

Writes only the SVG to a file (no HTML wrapper).

```python
chart.export_svg("chart.svg")
```

---

### `export_png(path, scale=2.0)`

Exports the chart as a PNG via [cairosvg](https://cairosvg.org/).
Requires `pip install cairosvg`.

```python
chart.export_png("chart.png", scale=3.0)  # 3× resolution
```

---

## Hover tooltips

### `build_hover_json(labels, images=None, descriptions=None)`

Builds a rich hover tooltip JSON string to pass as `hover_json=` to any chart function.

| Parameter | Type | Description |
|---|---|---|
| `labels` | `list[str]` | Tooltip title for each data point |
| `images` | `list[str \| None]` | Optional image URL per point (shown in tooltip) |
| `descriptions` | `list[str \| None]` | Reserved for future use |

```python
import seraplot as sp

countries = ["France", "Germany", "Spain", "Italy"]
gdp = [2.9, 4.2, 1.4, 2.1]

logo = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png"
hover = sp.build_hover_json(countries, images=[logo] * len(countries))

chart = sp.build_bar_chart(
    "GDP by Country (trillion $)",
    labels=countries,
    values=gdp,
    hover_json=hover,
)
```

---

## Complete chaining example

```python
import seraplot as sp

labels = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]
values = [41.2, 38.5, 55.1, 62.0, 58.7, 71.4]

logo = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png"
hover = sp.build_hover_json(labels, images=[logo] * len(labels))

chart = (
    sp.build_bar_chart("Monthly Revenue", labels, values,
                       hover_json=hover, gridlines=True)
    .set_bg(None)
    .show_grid()
    .show_labels(position="top")
    .set_font_size(13)
    .inject_css("""
        .sp-bg  { fill: #0f172a !important; }
        svg text    { fill: #e2e8f0 !important; }
        .sp-gl      { stroke: #1e293b !important; }
    """)
)

chart.save("revenue.html")
```

