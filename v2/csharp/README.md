# SeraPlot.NET

P/Invoke bindings over the same Rust core that ships to PyPI, generated from the same function registry -- no hand-maintained method list, no feature gap with the Python package.

## Layout

- `SeraPlot/SeraPlot.g.cs` -- generated on every `cargo build`/`cargo check` in `v2/src` (see `docs/gen/bindings.rs`, `gen_csharp`). One `SeraPlot.Api.<PascalName>(string input)` per registered function, all JSON in, JSON or HTML string out, matching the Python API's own kwargs shape.
- `SeraPlot/Chart.cs` -- hand-written convenience wrapper (`.Html`, `.Save(path)`) around the raw HTML string a chart builder returns.
- `SeraPlot/runtimes/win-x64/native/seraplot.dll` -- the compiled Rust `cdylib`, built with `--features ffi`. Packed into `runtimes/win-x64/native/` inside the `.nupkg` per the standard NuGet native-asset convention, and copied to this project's own build output for local testing via `ProjectReference`.

## Building the native library

```
cd v2/src
cargo build --release --features ffi
copy target\release\seraplot.dll ..\csharp\SeraPlot\runtimes\win-x64\native\seraplot.dll
```

(`v2/src` is its own Cargo workspace, so the build lands in `v2/src/target/release/`, not `v2/target/`.)

## Build, pack, test

```
cd v2/csharp/SeraPlot
dotnet build -c Release
dotnet pack -c Release
```

The `.nupkg` lands in `SeraPlot/bin/Release/`. Upload that file at https://www.nuget.org/packages/manage/upload.
