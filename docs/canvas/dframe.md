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

## Construction

| Constructor | Effect |
|---|---|
| `sp.SeraDFrame({"col": [...], ...})` | From a `dict[str, list]`. |
| `sp.SeraDFrame.from_csv(path)` | Reads a CSV; per-column type inference (numeric/bool/string) runs in parallel across columns via `rayon`. |
| `sp.SeraDFrame.from_pandas(df)` | From an existing `pandas.DataFrame`. |
| `sp.DFrameBuilder().column(name, values)....build()` | Incremental, chainable builder — add columns one at a time, `build()` when done. |

## Reading

`shape`, `columns()`, `dtypes()`, `head(n)`, `tail(n)`, `len(df)`,
`df["col"]` / `df[["a", "b"]]`, `column_f64`/`column_str`, `to_records()`.

## Shaping

`filter_eq/gt/lt/ge/le/in`, `sort_values(col, ascending=True)`,
`drop_duplicates(subset=None)`, `dropna(subset=None)`, `fillna(value)`,
`select(names)`, `rename(mapping)`, `assign(name, values)`,
`apply(col, python_callable)` — calls back into Python per value, so it
accepts **any** function, not just the built-in numeric ops.

## Relational & aggregate

`merge(other, on, how="inner"|"left")`, `concat(other)`,
`groupby(col)` returns a `SeraDFrameGroupBy` with `.agg({"col": "sum"|...})`,
`.mean()`/`.sum()`/`.min()`/`.max()`/`.count()`/`.size()` (auto-selecting
every numeric column, mirroring `pandas`' `groupby(col).mean()`).

## Stats

`describe()`, `corr(a, b)`, `value_counts(col)`, `unique(col)`,
`nunique(col)`.

## Interop

`to_pandas() -> pandas.DataFrame`, `to_csv(path)`, `to_table() -> Table` —
bridges into SeraPlot's `Table` so its chart-export helpers
(`to_grouped_bar`, `pivot`, ...) are one call away.

## Honest performance notes

Benchmarked against pandas 2.x, both on a 250k-row sample and on the same
data replicated to **3,000,000 rows × 34 columns**
(`v2/examples/GptDoc/seradframe_vs_pandas.ipynb`). Three deliberate design
choices behind the numbers:

- String columns are `Vec<Arc<str>>`, not `Vec<String>` — row/group
  operations clone a refcount bump instead of a heap-allocated string copy.
- `groupby().agg()` never gathers rows into per-group `Vec<f64>` buffers (a
  cache-hostile random-access pattern). It precomputes one `group_id: Vec<u32>`
  and does a single cache-friendly linear scan per column, accumulating
  sum/min/max/count directly — the same data-oriented technique as a
  histogram pass, not a sort-then-segment.
- The **global allocator is `mimalloc`**, not the system default. Profiling
  showed the real bottleneck at multi-million-row scale wasn't the parsing
  or aggregation logic — it was lock contention in the default allocator
  under many concurrent small allocations from rayon threads. Swapping the
  allocator turned `from_csv` from a *regression* (0.4× pandas) into a
  **1.1–1.8× win**, and pushed `describe()` past 16×, without touching a
  single line of parsing or aggregation code.

At 3,000,000 rows, SeraDFrame beats pandas on every benchmarked operation:
`describe()` **~16×**, `corr()` **~6–7×**, `sort_values` **~1.4×**,
`value_counts` **~1.1–2.4×**, `read_csv` **~1.1–1.8×**, `filter_gt` and
`groupby().mean()` **on par, sometimes ahead** (0.8–1.4× depending on machine
load — the one place still close enough to call a tie rather than a win).

An earlier attempt at partitioning `groupby`'s row-bucketing step across
threads was tested at this same 3M-row scale and made things **14× slower**
(0.07×) instead of faster — it was reverted rather than kept on the theory
that it "should" help. Every number above is the result of measuring, not
predicting.

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

## Construction

| Constructeur | Effet |
|---|---|
| `sp.SeraDFrame({"col": [...], ...})` | Depuis un `dict[str, list]`. |
| `sp.SeraDFrame.from_csv(path)` | Lit un CSV ; l'inférence de type par colonne (numérique/booléen/chaîne) tourne en parallèle sur les colonnes via `rayon`. |
| `sp.SeraDFrame.from_pandas(df)` | Depuis un `pandas.DataFrame` existant. |
| `sp.DFrameBuilder().column(nom, valeurs)....build()` | Builder incrémental et chaînable — ajoutez les colonnes une à une, `build()` à la fin. |

## Lecture

`shape`, `columns()`, `dtypes()`, `head(n)`, `tail(n)`, `len(df)`,
`df["col"]` / `df[["a", "b"]]`, `column_f64`/`column_str`, `to_records()`.

## Mise en forme

`filter_eq/gt/lt/ge/le/in`, `sort_values(col, ascending=True)`,
`drop_duplicates(subset=None)`, `dropna(subset=None)`, `fillna(value)`,
`select(names)`, `rename(mapping)`, `assign(name, values)`,
`apply(col, fonction_python)` — rappelle Python pour chaque valeur, accepte
donc **n'importe quelle** fonction, pas seulement les opérations numériques
intégrées.

## Relationnel & agrégation

`merge(other, on, how="inner"|"left")`, `concat(other)`,
`groupby(col)` renvoie un `SeraDFrameGroupBy` avec `.agg({"col": "sum"|...})`,
`.mean()`/`.sum()`/`.min()`/`.max()`/`.count()`/`.size()` (sélectionne
automatiquement toutes les colonnes numériques, comme `groupby(col).mean()`
sous `pandas`).

## Stats

`describe()`, `corr(a, b)`, `value_counts(col)`, `unique(col)`,
`nunique(col)`.

## Interopérabilité

`to_pandas() -> pandas.DataFrame`, `to_csv(path)`, `to_table() -> Table` —
passerelle vers le `Table` de SeraPlot pour profiter de ses aides
d'export vers les charts (`to_grouped_bar`, `pivot`, ...) en un appel.

## Notes de performance honnêtes

Benchmarké contre pandas 2.x, sur un échantillon de 250k lignes et sur les
mêmes données répliquées à **3 000 000 lignes × 34 colonnes**
(`v2/examples/GptDoc/seradframe_vs_pandas.ipynb`). Trois choix de
conception délibérés derrière ces chiffres :

- Les colonnes de chaînes sont des `Vec<Arc<str>>`, pas des `Vec<String>` —
  les opérations de ligne/groupe clonent un compteur de références, pas une
  copie de chaîne sur le tas.
- `groupby().agg()` ne rassemble jamais les lignes dans des buffers
  `Vec<f64>` par groupe (un accès mémoire aléatoire hostile au cache). Un
  `group_id: Vec<u32>` est précalculé une fois, puis un seul passage linéaire
  par colonne accumule somme/min/max/count directement — la même technique
  data-oriented qu'un histogramme, pas un tri-puis-segmentation.
- **L'allocateur global est `mimalloc`**, pas celui du système. Le
  profilage a montré que le vrai goulot d'étranglement à l'échelle du
  million de lignes n'était ni le parsing ni l'agrégation — c'était la
  contention de verrou dans l'allocateur par défaut sous de nombreuses
  petites allocations concurrentes venant des threads rayon. Changer
  l'allocateur a fait passer `from_csv` d'une *régression* (0.4× pandas) à
  un **gain de 1.1 à 1.8×**, et a poussé `describe()` au-delà de 16×, sans
  toucher une seule ligne de code de parsing ou d'agrégation.

À 3 000 000 lignes, SeraDFrame bat pandas sur chaque opération benchmarkée :
`describe()` **~16×**, `corr()` **~6-7×**, `sort_values` **~1.4×**,
`value_counts` **~1.1 à 2.4×**, `read_csv` **~1.1 à 1.8×**, `filter_gt` et
`groupby().mean()` **à égalité, parfois devant** (0.8 à 1.4× selon la
charge machine — le seul cas encore assez proche pour parler d'égalité
plutôt que de victoire).

Une tentative antérieure de paralléliser le regroupement en buckets du
`groupby` par partition a été testée à cette même échelle de 3M lignes et a
rendu les choses **14× plus lentes** (0.07×) au lieu de mieux — elle a été
annulée plutôt que gardée sur la théorie qu'elle "devrait" aider. Chaque
chiffre ci-dessus est le résultat d'une mesure, pas d'une prédiction.

</div>
