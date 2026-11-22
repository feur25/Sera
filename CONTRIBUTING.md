# Contributing to SeraPlot

Thanks for considering a contribution. This document covers the practical workflow — branching,
commits, pull requests, code style, and testing. For how the codebase itself is laid out, read
[ARCHITECTURE.md](ARCHITECTURE.md) first.

## Before you start

- Open or find a [GitHub Issue](https://github.com/feur25/Sera/issues) describing the bug or
  feature before writing code, so the work isn't duplicated or built on a misunderstanding of
  scope.
- Check [ARCHITECTURE.md § Known inconsistencies](ARCHITECTURE.md#known-inconsistencies) first —
  some known gaps (the PyPI wheel coverage, the dual Cargo manifest) are already tracked and don't
  need rediscovering.

## Development setup

```bash
git clone https://github.com/feur25/Sera.git
cd Sera

# Rust
cargo build
cargo test --lib --features python   # see "Testing" below for why --features python matters

# Python binding (installs into a local .venv via maturin)
python -m venv .venv
.venv/bin/pip install maturin
.venv/bin/maturin develop

# JS/WASM binding
wasm-pack build --target web --out-dir pkg --features js
```

## Branching

Branch off `main`, one branch per logical change:

```
<type>/<short-description>
```

using the same `<type>` vocabulary as commits (below) — e.g. `fix/color-hex-data-idx-fallback`,
`feat/vector-field-streamlines`, `docs/architecture-overview`.

## Commit messages — Conventional Commits

This project follows [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/).
Every commit message is:

```
<type>[optional scope]: <short, imperative description>

[optional body]

[optional footer(s)]
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`.
A breaking change is flagged either with `!` after the type/scope (`fix!: …`) or a
`BREAKING CHANGE:` footer — either way, explain what breaks and how to migrate in the body.

**Scopes** should name the area touched, matching the module map in ARCHITECTURE.md — e.g.
`chart-methods`, `dispatch`, `map`, `pulse`, `ffi`, `wasm`, `vscode`, `docs`, `macros`, `release`.

```
fix(chart-methods): make color_hex fall back to data-idx when no data-series exists
feat(map): add isoline labels to contour_map
docs(architecture): document the three rendering architectures
fix(dispatch)!: apply_method_kwargs now runs before the native fast path, not after

BREAKING CHANGE: charts built through the native fast path (bar/line/scatter/... above the
render threshold) now apply constructor kwargs that were previously silently ignored.
```

Keep commits small and scoped to one change — prefer several small, clearly-scoped commits over
one large one, even for small fixes. This keeps `git log`/`git bisect` useful and makes review
easier.

## Pull requests

1. Push your branch and open a PR against `main`.
2. Describe *why*, not just *what* — link the issue it closes (`Closes #123`).
3. Make sure `cargo test --lib --features python` passes and, if you touched chart-method
   behaviour, that you manually verified it end to end (rebuild the Python extension with
   `maturin develop` and actually call the method against a real chart — a passing test suite
   verifies correctness, not that a visual/behavioural fix actually shows up).
4. **Only the repository owner ([@feur25](https://github.com/feur25)) merges pull requests.**
   Do not merge your own PR, even once approved and green — wait for it to be merged for you.
   This keeps a single person accountable for what lands on `main`.

## Code style

- **No comments that restate what the code does.** Identifiers should make that unnecessary. A
  comment is only justified when it captures a non-obvious *why* — a hidden constraint, a
  workaround for a specific bug, a convention a reader couldn't otherwise infer. This matches the
  existing codebase; new code should read the same way.
- **Reuse the existing patterns instead of inventing a new one.** Before writing dispatch logic,
  a new per-chart lookup, or a new struct, check whether `inventory`/the registries described in
  [ARCHITECTURE.md § 5](ARCHITECTURE.md#5-the-registrymacro-system) already cover it — most new
  chart-level behaviour is one `#[sera_doc]`/`#[sera_sig]`-annotated method, not a new mechanism.
- **One `#[sera_impl] impl Chart` block per feature file** under `bindings/chart_methods/` —
  don't scatter related methods across files, and don't add a second `impl Chart` block to a
  file that already has one.
- **Never break an existing registry.** Renaming or removing a chart-method's Rust name without
  keeping its documented aliases (`aliases(...)` in `#[sera_doc]`) breaks every binding at once,
  not just Python.
- **If a method's "already applied?" guard would skip an explicit call that's trying to change
  something** (a different position, a different color, ...), replace the existing effect instead
  of no-opping — see ARCHITECTURE.md §6's note on this recurring bug class.

## Testing

```bash
cargo test --lib --features python
```

The `--features python` flag is **not optional** for meaningful test results: the crate's default
features (`gui`, `sysmon`, `telemetry-http`) do not include `python`, and without it every
`inventory::submit!` registration from `#[sera_impl]` is compiled out — registry-based tests then
fail with empty lookups that look like real bugs but are just a missing feature flag.

For anything that changes rendered HTML/behaviour, also verify it against a live chart after
`maturin develop`:

```python
import seraplot as sp
sp.set_auto_display(False)
c = sp.bar("demo", labels=["a", "b"], values=[1, 2])
out = c.your_method(...)
assert out.html != c.html
```

A green test suite proves the code compiles and the units you tested behave as asserted; it does
not by itself prove a chart method does something meaningful on every chart family and variant.
When in doubt, check more than one chart family and variant, including at least one 3D family and
the `joint` family (see [ARCHITECTURE.md § 6](ARCHITECTURE.md#6-three-different-rendering-architectures-know-which-one-youre-editing)).

## Reporting bugs / proposing features

Use [GitHub Issues](https://github.com/feur25/Sera/issues). Existing labels: `bug`,
`enhancement`, `documentation`, `question`, `good first issue`, `help wanted`. Include:

- what you called (method, chart family, variant) and what you expected,
- what actually happened — ideally the rendered HTML diff or a screenshot,
- which binding (Python / WASM / C / C++ / C#) if it's not the Python one.
