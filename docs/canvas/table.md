# Table — Data Engineering

<div class="lang-en">

`Table` is a small, columnar data-shaping utility: relational joins,
group-by/aggregate, pivots, rolling windows and filters, all in Rust, with no
pandas dependency. Its purpose is narrow and deliberate — reshape one source
of truth into the exact inputs each chart function expects, so several
panels built from the same data stay consistent, instead of hand-rolling
loops per chart.

```python
import seraplot as sp

t = sp.Table({
    "region": ["North", "South", "North", "South"],
    "product": ["Core", "Core", "Cloud", "Cloud"],
    "revenue": [24.0, 18.0, 12.0, 9.0],
})
```

Columns are built from Python `dict[str, list]` — each value can be `int`,
`float`, `str` or `bool`; mixed-type columns coerce to string on read.

---

## Reading

| Method | Effect |
|--------|--------|
| `columns() -> list[str]` | Column names, in original order. |
| `nrows` (getter) | Row count. |
| `column(name) -> list` | Raw values (native Python types). |
| `column_f64(name) -> list[float]` | Values coerced to float. |
| `column_str(name) -> list[str]` | Values coerced to string. |
| `to_records() -> list[dict]` | Row-oriented view, one dict per row. |
| `head(n) -> Table` | First `n` rows. |
| `select(names) -> Table` | A subset of columns. |

---

## Filtering & sorting

| Method | Effect |
|--------|--------|
| `filter_eq(col, value) -> Table` | Rows where `col == value`. |
| `filter_gt/lt/ge/le(col, value: float) -> Table` | Numeric comparison filters. |
| `filter_in(col, values: list[str]) -> Table` | Rows where `col` is one of `values`. |
| `sort_by(col, desc=False) -> Table` | Sorted copy. |
| `top_n(col, n, desc=True) -> Table` | Shortcut for `sort_by(col, desc).head(n)` — the "top 10" pattern. |

---

## Relational & ETL operations

| Method | Effect |
|--------|--------|
| `join(other, on, how="inner") -> Table` | Joins two tables on a key column. `how="left"` keeps unmatched left rows with zero-filled right columns. Colliding column names from `other` are prefixed `right_`. |
| `concat(other) -> Table` | Vertical union of two tables; missing columns on either side are zero/empty-filled. |
| `with_column(name, op, left, right) -> Table` | Adds a computed column. `op` is `"add"`/`"sub"`/`"mul"`/`"div"`. `right` is either another column's name or a constant. |
| `groupby_agg(group_col, value_col, agg="sum") -> Table` | Groups by `group_col`, aggregates `value_col`. `agg` is `"sum"`/`"mean"`/`"count"`/`"min"`/`"max"`/`"median"`. |
| `pivot(index_col, columns_col, values_col, agg="sum") -> Table` | Reshapes long data to wide: one row per `index_col` value, one column per unique `columns_col` value. |

---

## Time-series & stats prep

| Method | Effect |
|--------|--------|
| `rolling_mean(col, window) -> Table` | Adds `{col}_rolling{window}`, a trailing moving average. |
| `cumsum(col) -> Table` | Adds `{col}_cumsum`, the running total. |
| `pct_change(col) -> Table` | Adds `{col}_pct_change`, row-over-row percent change. |
| `rank(col, desc=False) -> Table` | Adds `{col}_rank`, 1-based rank. |
| `zscore(col) -> Table` | Adds `{col}_zscore`, `(x - mean) / std`. |
| `describe() -> Table` | One row per numeric column: `count`, `mean`, `min`, `max`, `std`. |

Every transform returns a **new** `Table` (chainable, no mutation):

```python
monthly = (
    sales.groupby_agg("month", "revenue", "sum")
         .sort_by("month")
         .rolling_mean("revenue", 3)
         .cumsum("revenue")
)
```

---

## Feeding charts directly

```python
gb = table.to_grouped_bar("month", "product", "revenue", "sum")
bar = sp.grouped_bar(
    "", labels=gb["category_labels"], values=gb["values"],
    series_names=gb["series_names"],
)
```

`to_grouped_bar(index_col, columns_col, values_col, agg="sum")` pivots and
flattens in one call, returning a dict shaped exactly for
`sp.grouped_bar(labels=, values=, series_names=)` — the most common
table-to-chart handoff, done in one line instead of manual pivoting.

For any other chart, `column_f64`/`column_str` after a `filter_*`/`sort_by`/
`groupby_agg` chain gets you there just as directly.

---

## Loading data

```python
t = sp.Table.from_csv("sales.csv")
```

Columns are auto-typed: numeric if every value in the column parses as a
float, string otherwise.

</div>

<div class="lang-fr">

`Table` est un petit outil de mise en forme de données en colonnes :
jointures relationnelles, group-by/agrégation, pivots, fenêtres glissantes
et filtres, le tout en Rust, sans dépendance à pandas. Son rôle est étroit
et délibéré — remodeler une source de vérité unique vers les entrées exactes
qu'attend chaque fonction de chart, pour que plusieurs panneaux construits
depuis les mêmes données restent cohérents, au lieu d'écrire des boucles à
la main pour chacun.

```python
import seraplot as sp

t = sp.Table({
    "region": ["North", "South", "North", "South"],
    "product": ["Core", "Core", "Cloud", "Cloud"],
    "revenue": [24.0, 18.0, 12.0, 9.0],
})
```

Les colonnes se construisent depuis un `dict[str, list]` Python — chaque
valeur peut être `int`, `float`, `str` ou `bool` ; les colonnes de type
mixte sont converties en chaîne à la lecture.

---

## Lecture

| Méthode | Effet |
|--------|--------|
| `columns() -> list[str]` | Noms des colonnes, dans l'ordre d'origine. |
| `nrows` (getter) | Nombre de lignes. |
| `column(name) -> list` | Valeurs brutes (types Python natifs). |
| `column_f64(name) -> list[float]` | Valeurs converties en float. |
| `column_str(name) -> list[str]` | Valeurs converties en chaîne. |
| `to_records() -> list[dict]` | Vue orientée ligne, un dict par ligne. |
| `head(n) -> Table` | Les `n` premières lignes. |
| `select(names) -> Table` | Un sous-ensemble de colonnes. |

---

## Filtrage & tri

| Méthode | Effet |
|--------|--------|
| `filter_eq(col, value) -> Table` | Lignes où `col == value`. |
| `filter_gt/lt/ge/le(col, value: float) -> Table` | Filtres de comparaison numérique. |
| `filter_in(col, values: list[str]) -> Table` | Lignes où `col` fait partie de `values`. |
| `sort_by(col, desc=False) -> Table` | Copie triée. |
| `top_n(col, n, desc=True) -> Table` | Raccourci pour `sort_by(col, desc).head(n)` — le motif "top 10". |

---

## Opérations relationnelles & ETL

| Méthode | Effet |
|--------|--------|
| `join(other, on, how="inner") -> Table` | Joint deux tables sur une colonne clé. `how="left"` garde les lignes gauches sans correspondance avec des colonnes droites à zéro. Les noms de colonnes de `other` en collision sont préfixés `right_`. |
| `concat(other) -> Table` | Union verticale de deux tables ; les colonnes manquantes d'un côté sont remplies à zéro/vide. |
| `with_column(name, op, left, right) -> Table` | Ajoute une colonne calculée. `op` vaut `"add"`/`"sub"`/`"mul"`/`"div"`. `right` est soit le nom d'une autre colonne, soit une constante. |
| `groupby_agg(group_col, value_col, agg="sum") -> Table` | Groupe par `group_col`, agrège `value_col`. `agg` vaut `"sum"`/`"mean"`/`"count"`/`"min"`/`"max"`/`"median"`. |
| `pivot(index_col, columns_col, values_col, agg="sum") -> Table` | Passe du format long au format large : une ligne par valeur de `index_col`, une colonne par valeur unique de `columns_col`. |

---

## Prépa séries temporelles & stats

| Méthode | Effet |
|--------|--------|
| `rolling_mean(col, window) -> Table` | Ajoute `{col}_rolling{window}`, une moyenne mobile arrière. |
| `cumsum(col) -> Table` | Ajoute `{col}_cumsum`, le total cumulé. |
| `pct_change(col) -> Table` | Ajoute `{col}_pct_change`, la variation en % ligne à ligne. |
| `rank(col, desc=False) -> Table` | Ajoute `{col}_rank`, le rang (base 1). |
| `zscore(col) -> Table` | Ajoute `{col}_zscore`, `(x - moyenne) / écart-type`. |
| `describe() -> Table` | Une ligne par colonne numérique : `count`, `mean`, `min`, `max`, `std`. |

Chaque transformation renvoie une **nouvelle** `Table` (chaînable, sans
mutation) :

```python
monthly = (
    sales.groupby_agg("month", "revenue", "sum")
         .sort_by("month")
         .rolling_mean("revenue", 3)
         .cumsum("revenue")
)
```

---

## Alimenter directement les charts

```python
gb = table.to_grouped_bar("month", "product", "revenue", "sum")
bar = sp.grouped_bar(
    "", labels=gb["category_labels"], values=gb["values"],
    series_names=gb["series_names"],
)
```

`to_grouped_bar(index_col, columns_col, values_col, agg="sum")` pivote et
aplatit en un seul appel, renvoyant un dict au format exact pour
`sp.grouped_bar(labels=, values=, series_names=)` — la passerelle
table-vers-chart la plus courante, faite en une ligne plutôt qu'un pivot
manuel.

Pour tout autre chart, `column_f64`/`column_str` après une chaîne
`filter_*`/`sort_by`/`groupby_agg` y mène tout aussi directement.

---

## Charger des données

```python
t = sp.Table.from_csv("sales.csv")
```

Les colonnes sont typées automatiquement : numérique si chaque valeur de la
colonne se parse en float, chaîne sinon.

</div>
