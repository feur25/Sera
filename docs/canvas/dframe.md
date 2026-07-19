# SeraDFrame

<div class="lang-en">

`SeraDFrame` is a columnar, Rust-native dataframe — `Vec<f64>` / `Vec<String>`
/ `Vec<bool>` per column, no per-cell object boxing — covering the common
pandas surface: relational joins, group-by/aggregate, sorting, filtering,
dedup, `describe`/`corr`, and a builder pattern, plus **native, lossless
conversion to and from `pandas.DataFrame`** so it drops into an existing
pandas pipeline anywhere.

```python
import seraplot as sp

df = sp.SeraDFrame.from_csv("events.csv")
by_region = df.groupby("region").agg({"cost": "sum", "latency_ms": "mean"})
top5 = by_region.sort_values("cost", ascending=False).head(5)
```

Every table below is generated at page load straight from the `#[sera_doc(...)]`
attributes on each method in `v2/src/data/dframe/` — not hand-maintained, so
it cannot drift from what is actually implemented. `SeraDFrame` methods do
not currently carry aliases the way chart functions do — one canonical name
per method, matching the underlying pandas-shaped surface directly.

## Construction & Interop

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="construct"></div>

## Reading & Attributes

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="attrs"></div>

## Filtering & Masking

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="filter,mask,query"></div>

## Shaping & Transform

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="derive,elementwise,reshape,tools"></div>

## Relational & Combine

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="relational,combine"></div>

## GroupBy

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="groupby"></div>

## Rolling & Expanding

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="rolling"></div>

## Datetime

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="datetime"></div>

## Stats & Reductions

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="stats,reduce"></div>

## String Methods

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="strings"></div>

</div>

<div class="lang-fr">

`SeraDFrame` est un dataframe colonnaire, natif Rust — `Vec<f64>` /
`Vec<String>` / `Vec<bool>` par colonne, sans boxing objet par cellule —
couvrant la surface pandas courante : jointures relationnelles,
group-by/agrégation, tri, filtrage, dédoublonnage, `describe`/`corr`, un
patron builder, plus une **conversion native et sans perte vers et depuis
`pandas.DataFrame`**, pour s'insérer n'importe où dans un pipeline pandas
existant.

```python
import seraplot as sp

df = sp.SeraDFrame.from_csv("events.csv")
by_region = df.groupby("region").agg({"cost": "sum", "latency_ms": "mean"})
top5 = by_region.sort_values("cost", ascending=False).head(5)
```

Chaque tableau ci-dessous est généré au chargement de la page directement
depuis les attributs `#[sera_doc(...)]` de chaque méthode dans
`v2/src/data/dframe/` — pas maintenu à la main, donc impossible de dériver
de ce qui est réellement implémenté. Les méthodes `SeraDFrame` n'ont
actuellement pas d'alias comme les fonctions de graphique — un seul nom
canonique par méthode, reflétant directement la surface pandas sous-jacente.

<h2>Construction & Interopérabilité</h2>

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="construct"></div>

<h2>Lecture & attributs</h2>

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="attrs"></div>

<h2>Filtrage & masques</h2>

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="filter,mask,query"></div>

<h2>Mise en forme & transformation</h2>

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="derive,elementwise,reshape,tools"></div>

<h2>Relationnel & combinaison</h2>

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="relational,combine"></div>

<h2>GroupBy</h2>

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="groupby"></div>

<h2>Fenêtres glissantes</h2>

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="rolling"></div>

<h2>Dates & heures</h2>

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="datetime"></div>

<h2>Stats & réductions</h2>

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="stats,reduce"></div>

<h2>Méthodes de chaînes</h2>

<div data-sp-data-table data-file="canvas/dframe.md" data-modules="strings"></div>

</div>
