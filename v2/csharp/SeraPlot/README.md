# SeraPlot

Rust-native data visualization for .NET. Every chart family reachable from the
Python package (`pip install seraplot`) is reachable here too, generated from
the same function registry -- JSON in, SVG/HTML out, no hand-maintained
binding list.

```csharp
using SeraPlot;

var html = Api.BuildBar("""{"title":"Q1-Q4","labels":["Q1","Q2","Q3","Q4"],"values":[24,38,17,42]}""");
new Chart(html).Save("bar.html");
```

Every registered chart builder is exposed the same way: `Api.<PascalName>(string jsonInput)`
returns an HTML/SVG string. `Chart` is a thin convenience wrapper (`.Html`, `.Save(path)`,
implicit conversion to `string`).

Everything reachable from Python is reachable here too, through the same underlying
registry and the same alias-resolving dispatcher `sp._sera_call` uses:

```csharp
Api.List();                            // every registered function name, as JSON
Api.CallByName("build_bar", json);      // call any of them dynamically, by name
Api.AliasAdd("build_bar", "b");         // your own short names, shared with Python's sp.config.add(...)
Api.CallByName("b", json);              // -- resolves through the alias, same as sp.b(...)
Api.ChartVariants();                    // every family -> variant list, as JSON
Api.Params("build_bar", null);          // accepted keyword arguments for a family
Api.ChartThemes(); Api.Scenes3d(); Api.Docs();
```

Ships a `win-x64` native asset (`seraplot.dll`) alongside the managed
`SeraPlot.Interop.dll` -- no separate native install step.

- Source and issue tracker: https://github.com/feur25/Sera
- Docs and chart catalog: https://feur25.github.io/Sera/
- Python package: https://pypi.org/project/seraplot/
