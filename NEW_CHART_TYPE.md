# Adding a new chart type

Checklist for adding a brand-new chart family (not a new variant of an existing one — see the
note at the end for that smaller case). Verified against how existing families are actually wired
up, not written from a template.

## 1. Renderer

- Add `services/plot/statistical/<family>/` with one file per variant (e.g. `basic.rs`,
  `stacked.rs`, ...) plus a `mod.rs`/`common.rs`/`config.rs` as needed, following the shape of an
  existing family of similar complexity.
- Each variant's render function takes a `&<Family>Config` and returns the rendered `String`
  (SVG for 2D, the `js_3d.rs`-driven HTML for 3D — see
  [ARCHITECTURE.md §6](ARCHITECTURE.md#6-three-different-rendering-architectures-know-which-one-youre-editing)
  for which one applies and what conventions its output needs to follow).
- **Tag every colored data element with `data-series="N"` or `data-idx="N"`** as you write the
  renderer, not after the fact. This is what lets `color_hex`/`palette`/the legend overlay work on
  the new family automatically instead of silently no-opping — see issue #12 for what happens when
  a renderer skips this.

## 2. Wire it into construction

- Register the builder function in `services/plot/statistical/chart_registry.rs::dispatch()`'s
  match statement — this one is **hand-maintained, not inventory-based**; a new family needs its
  own match arm.
- Register the family name + variant keys/aliases so `sp.chart_variants()` lists it. Most 2D
  families use the `plot_family!` macro (`services/plot/family_macro.rs`), which registers a
  `ChartFamilyEntry` via `inventory` for you — follow an existing family's `plot_family!` call for
  the exact shape. 3D families that reuse the generic scene system (terrain/tower/radial/podium)
  or that are a "geometry twin" of an existing 2D family are wired up differently in
  `lib.rs::chart_variants()` — look at how an existing 3D/twin family is registered there before
  inventing a third pattern.

## 3. Demo data

- Add `#[crate::chart_demo("kwargs=... as a literal arg string")]` above each variant's render
  entry point. The attribute reads the file's own path (`file!()`) to derive the family/variant
  name, so the file needs to already be in the right place from step 1.
- Confirm it actually works: `cargo test --lib --features python
  chart_variants_demo_coverage_tests` fails the build if a family registered in
  `chart_variants()` has no working demo for its default variant — this exact gap (a family,
  `plot_3d_types`, registered with zero demo data for any of its 5 variants) is what this checklist
  and that test exist to catch going forward.

## 4. Docs

- Add a page under `docs/charts/2d/`, `docs/charts/3d/`, or `docs/charts/map/` matching the
  family's `kind`. Most existing pages use `<div data-sp-registry-table="methods"
  data-file="...">` to pull the applicable chart_methods live from the registry rather than
  hand-listing them — follow that pattern so the page can't go stale against the Rust source.
- Add the page to `docs/SUMMARY.md`'s table of contents.

## 5. Verify it end to end

- `maturin develop`, then from Python: build the chart via its new constructor, call a handful of
  chart_methods against it (at least `title`, `color_hex`, and anything else that seems relevant
  to the family), and confirm the output HTML actually changes — a green test suite does not prove
  a chart_method does something meaningful on a new family, only that nothing crashed. See
  [CONTRIBUTING.md § Testing](CONTRIBUTING.md#testing).
- If the family has multiple variants, check more than one — a method working on the default
  variant doesn't guarantee it works on every variant (see issue #11 for a case where it didn't).

## Adding a new variant of an *existing* family

Smaller version of the same checklist: one new file under the family's own
`services/plot/statistical/<family>/` directory with its own `#[chart_demo(...)]`-annotated render
function, a new entry in that family's `plot_family!`/registration call, and the same end-to-end
verification in step 5 — steps 2 (new dispatch arm) and 4 (new docs page) aren't needed since the
family already has both.
