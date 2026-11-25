<p align="center">
  <a href="https://feur25.github.io/Sera/getting-started/intro-seraplot.html"><img src="https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/icon.png" alt="SeraPlot" width="110"></a>
</p>

<h1 align="center">SeraPlot</h1>
<p align="center"><b>A Rust-native charting engine, exposed to Python as a single compiled extension.</b></p>
<p align="center">No JavaScript charting library. No Node toolchain. No external rendering service.</p>

<p align="center">
<a href="https://pypi.org/project/seraplot/"><img src="https://img.shields.io/pypi/v/seraplot" alt="PyPI"></a>
<a href="https://www.npmjs.com/package/seraplot"><img src="https://img.shields.io/npm/v/seraplot" alt="npm"></a>
<a href="https://github.com/feur25/Sera/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License"></a>
</p>

One `pip install` gets you 60+ chart types across 2D, 3D, and maps. Every chart renders to a single self-contained HTML file — no CDN, no backend, works offline or embedded anywhere a browser can open a file.

> **Full documentation:** https://feur25.github.io/Sera/getting-started/intro-seraplot.html

---

## See it move

Two real 3D renders, captured straight from the engine.

| | |
|:---:|:---:|
| ![bar_3d animated](https://raw.githubusercontent.com/feur25/Sera/main/docs/seraplot/videos/serastudio_smooth_bar3d_race_10s.gif) | ![line_3d animated](https://raw.githubusercontent.com/feur25/Sera/main/docs/seraplot/videos/serastudio_smooth_line3d_wave_10s.gif) |
| `bar_3d` — animated race | `line_3d` — animated wave |

---

## Not just a library — a desktop app too

SeraPlot ships a native viewer (C#/WPF): a live, browsable catalog of every chart family and variant, one-click switching between chart types, an interactive legend, export — a dedicated visualization desktop app bundled with the engine.

![SeraPlot native viewer](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/app_demo.gif)

---

## Install

| Manager | Command |
|---|---|
| **pip** | `pip install seraplot` |
| **uv** | `uv pip install seraplot` |
| **conda** | `conda install -c conda-forge seraplot` |
| **npm** (JS/WASM) | `npm install seraplot` |
| **NuGet** (C#) | `dotnet add package SeraPlot` |

2.8.8 ships wheels for Python 3.9–3.12 on Windows, Linux, and macOS, plus an sdist. C/C++ bindings build from source — see [CONTRIBUTING.md](https://github.com/feur25/Sera/blob/main/CONTRIBUTING.md).

## Two lines of code

```python
import seraplot as sp

chart = sp.bar("Revenue by Product", labels=["A", "B", "C"], values=[42, 68, 35])
chart.save("chart.html")
```

Open `chart.html` in any browser. No server, no build step.

---

## The same chart, three libraries

```python
# SeraPlot — 2 lines
import seraplot as sp
sp.bar("Revenue by Product", labels, values).save("chart.html")
```

```python
# Plotly — 4 lines
import plotly.express as px
fig = px.bar(x=labels, y=values, title="Revenue by Product")
fig.update_layout(template="plotly_white")
fig.write_html("chart.html")
```

```python
# Matplotlib — 7 lines
import matplotlib.pyplot as plt
fig, ax = plt.subplots(figsize=(9, 5))
ax.bar(labels, values, color="#6366f1")
ax.set_title("Revenue by Product")
ax.set_ylabel("Revenue")
plt.tight_layout()
plt.savefig("chart.png")
```

| | SeraPlot | Plotly | Matplotlib |
|---|:---:|:---:|:---:|
| Lines of code | **2** | 4 | 7 |
| Output | HTML | HTML | PNG |
| File size | **21 KB** | 4.7 MB | ~150 KB |
| Interactive | ✅ | ✅ | ❌ |
| Dependencies | **0** | 6+ | 3+ |

---

## Why not matplotlib or plotly

A non-exhaustive list — every point below is a real, shipped feature, not a marketing line.

- **A genuinely self-contained HTML file.** No external CDN the way Plotly's 4.7 MB output needs one — works offline, by email, in S3, in a PDF, in air-gapped CI.
- **Every chainable method is also a constructor keyword.** `.title()`, `.flip()`, `.color_hex()`, … all work as `sp.bar(..., flip=True)` too, not just as `.method()` chains.
- **Four languages, one engine.** Python, JavaScript/WebAssembly, C/C++, and C# share the exact same method registry — no per-language reimplementation to drift out of sync.
- **A native desktop app included.** A bundled C# viewer with a live chart catalog — neither matplotlib nor Plotly ships anything comparable in the package itself.
- **Live streaming and video export built in.** Sera Pulse: live updates, downsampling for large datasets, direct GIF/video export of a running chart.
- **Maps and 3D out of the box.** Choropleths, cartograms, a 3D globe, vector fields — not a separate plugin to install.
- **Zero runtime dependencies.** No 6+ bundled JS packages like Plotly, no backend to configure like Matplotlib — one compiled Rust binary.
- **Drop-in matplotlib migration.** `seraplot.matplotlib as plt` — swap the import, keep the rest of an existing matplotlib script.

---

## One engine, several jobs

| | |
|---|---|
| **Plot** | 60+ chart types — 2D, 3D (WebGL), and maps. Built-in themes, palettes, animation, zoom, crosshair, export. |
| **Stream & scale** | Live updates via Sera Pulse, downsampling for large datasets, video/GIF export of a running chart. |
| **Ship** | Self-contained HTML — no CDN, no backend, works offline, by email, in S3, in PDFs. |
| **Integrate** | Python, JavaScript/WebAssembly, C, C++, C#. Same engine, same method registry, everywhere. |
| **Compose** | Chainable methods (`.title()`, `.flip()`, `.color_hex()`, …) also work as constructor keyword arguments. |

---

## Gallery

34 real renders out of 60+ available chart families — 2D, 3D, maps, hierarchies, networks. Every image below is an actual SeraPlot chart, not a mockup.

| | | | |
|:---:|:---:|:---:|:---:|
| ![bar_3d](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/bar_3d.png) `bar_3d` | ![scatter_3d](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/scatter_3d.png) `scatter_3d` | ![pie3d](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/pie3d.png) `pie3d` | ![globe](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/globe.png) `globe` |
| ![choropleth](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/choropleth.png) `choropleth` | ![contour_map](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/contour_map.png) `contour_map` | ![vector_field_map](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/vector_field_map.png) `vector_field_map` | ![heatmap3d](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/heatmap3d.png) `heatmap3d` |
| ![bubble3d](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/bubble3d.png) `bubble3d` | ![radar](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/radar.png) `radar` | ![sunburst](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/sunburst.png) `sunburst` | ![icicle](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/icicle.png) `icicle` |
| ![circle_pack](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/circle_pack.png) `circle_pack` | ![chord](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/chord.png) `chord` | ![sankey](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/sankey.png) `sankey` | ![arc_diagram](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/arc_diagram.png) `arc_diagram` |
| ![parallel](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/parallel.png) `parallel` | ![venn](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/venn.png) `venn` | ![violin](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/violin.png) `violin` | ![boxplot](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/boxplot.png) `boxplot` |
| ![histogram](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/histogram.png) `histogram` | ![hexbin](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/hexbin.png) `hexbin` | ![heatmap](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/heatmap.png) `heatmap` | ![treemap](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/treemap.png) `treemap` |
| ![waterfall](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/waterfall.png) `waterfall` | ![candlestick](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/candlestick.png) `candlestick` | ![gantt](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/gantt.png) `gantt` | ![dumbbell](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/dumbbell.png) `dumbbell` |
| ![lollipop](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/lollipop.png) `lollipop` | ![funnel](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/funnel.png) `funnel` | ![gauge](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/gauge.png) `gauge` | ![bullet](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/bullet.png) `bullet` |
| ![wordcloud](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/wordcloud.png) `wordcloud` | ![bubble](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/bubble.png) `bubble` | ![area](https://raw.githubusercontent.com/feur25/Sera/main/docs/theme/images/readme/area.png) `area` | |

---

<p align="center"><a href="https://feur25.github.io/Sera/getting-started/intro-seraplot.html">Full documentation</a> · <a href="https://github.com/feur25/Sera">GitHub</a> · <a href="https://github.com/feur25/Sera/blob/main/CONTRIBUTING.md">Contributing</a></p>
