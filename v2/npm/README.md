# SeraPlot

Rust-native data visualization, compiled to WebAssembly and called directly --
no server, no interpreter, no bridge beyond the WASM boundary itself.

<a href="https://ibb.co/5hq2v6vH"><img src="https://i.ibb.co/r2nvMmMr/image.png" alt="A mosaic of SeraPlot chart types -- sunburst, radar, box plot, violin, sankey, heatmap, dendrogram, candlestick, treemap, network graph, chord diagram, pie, area, bar, bubble, and more" border="0"></a>

60+ chart families, 400+ named variants, discovered live from the engine's own
registry -- never a hand-maintained list drifting out of sync with what the
engine can actually do.

## Install

```
npm install seraplot
```

## Build a chart

```js
const seraplot = require('seraplot');

const chart = seraplot.buildBar(JSON.stringify({
  title: 'Q1-Q4',
  labels: ['Q1', 'Q2', 'Q3', 'Q4'],
  values: [24, 38, 17, 42],
}));

require('fs').writeFileSync('bar.html', chart);
```

Every chart family is exported directly, camelCase, straight off the engine's
registry (`buildBar`, `buildSankey`, `buildHeatmap`, ...) -- pass a JSON string,
get an HTML/SVG string back.

## Call by name, chain styling

```js
const chart = seraplot.call('bar', json)
  .call('grid_x')
  .call('no_hover')
  .call('set_bg', JSON.stringify({ color: '#111' }));

chart.save('bar.html');
```

`seraplot.call(name, json)` resolves through the same alias table the engine
ships with (family names, short names, anything you register yourself) and
returns a `Chart` -- chainable, since every call to `.call(...)` re-wraps the
result. `Chart` also has `.toSvg()`, `.save(path)`, and `.toString()`.

## Find out what's available

Nothing here is hand-typed into this package -- every name below comes straight
from the engine at runtime:

```js
JSON.parse(seraplot.list());          // every chart/utility function name
JSON.parse(seraplot.chartMethods());  // every chainable styling method name
JSON.parse(seraplot.chartVariants()); // every family and its named variants
JSON.parse(seraplot.chartAliases());  // built-in short names per family
seraplot.params('bar');               // accepted keyword arguments for a family
seraplot.chartThemes();
seraplot.scenes3d();
seraplot.docs();
seraplot.version();
```

## Aliases

```js
seraplot.aliasAdd('build_bar', 'b');
seraplot.call('b', json);             // resolves to build_bar
seraplot.aliasList();
```

## Why native

The engine behind these charts is the same one benchmarked at two to three
orders of magnitude faster than typical interpreted or browser-rendered
charting stacks on large series -- because it never leaves compiled code to
do it. This package doesn't add a layer on top of that; it's the same
compiled engine, called directly through WebAssembly.

- Source and issue tracker: https://github.com/feur25/Sera
- Docs and chart catalog: https://feur25.github.io/Sera/
