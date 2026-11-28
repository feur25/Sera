# SeraPlot for C++

A single generated header over the same Rust core that ships to PyPI and NuGet -- no hand-maintained method list, no feature gap with the Python package.

## Layout

- `include/seraplot.g.h` -- generated on every `cargo build`/`cargo check` in `v2/src` (see `docs/gen/bindings.rs`, `gen_cpp_header`). `extern "C"` declarations for every `sera_*` FFI export, plus a `namespace seraplot { inline std::string <camelName>(const std::string& input); ... }` wrapper that owns the free/copy dance around the raw `char*` Rust hands back.

Alongside the per-chart wrappers, `seraplot::` also exposes the same introspection and alias
layer Python gets: `list()`, `callByName(name, json)` (alias-resolving, dynamic dispatch --
the same path `sp._sera_call` uses), `aliasAdd`/`aliasRemove`/`aliasList`/`aliasResolve`/
`aliasSave`/`aliasLoad`, `chartVariants()`, `params(chart, variant)`, `chartThemes()`,
`scenes3d()`, `docs()`, `themes()`, `demos()`, `version()`.
- `build.bat` -- copies the freshly built `seraplot.dll`/`seraplot.dll.lib` from `v2/target/release`, sets up the MSVC environment, compiles and links one `.cpp` file against them.

## Building the native library

```
cd v2/src
cargo build --release --features ffi
```

(`v2/src` is its own Cargo workspace, so the build lands in `v2/src/target/release/`.)

## Compiling something against it

```
cd v2/cpp
build.bat path\to\file.cpp
build\file.exe
```

`seraplot.dll` is copied next to the produced `.exe` so it runs without extra `PATH` setup.
