# Cookbook

<div class="lang-en">

Short, task-oriented recipes for common situations. Each one is copy-pasteable.

## Load a CSV and chart a column

```python
import seraplot as sp

df = sp.SeraDFrame.from_csv("sales.csv")
by_region = df.groupby("region").agg({"revenue": "sum"})
chart = sp.bar(labels=by_region.column_str("region"), values=by_region.column_f64("revenue"))
chart.save("sales_by_region.html")
```

## Handle missing/invalid data before charting

`NaN`/`Infinity` in a Python list are sanitized automatically (replaced with `0`) rather than silently producing an empty chart — but for a meaningful result, clean the data first:

```python
df = sp.SeraDFrame.from_csv("sensor.csv")
df = df.dropna(subset=["reading"])
chart = sp.line(x_labels=df.column_str("timestamp"), values=df.column_f64("reading"))
```

## Compare distributions across categories

```python
chart = sp.violin(labels=["A", "B", "C"], values=all_values, categories=group_labels)
```

## Build a multi-chart dashboard

```python
charts = [sp.bar(labels=l, values=v).html for l, v in datasets]
dashboard = sp.build_grid(charts=charts, cols=2, title="Q3 Overview")
dashboard.save("dashboard.html")
```

## Large datasets — decimate or use the Canvas2D engine

```python
chart = sp.line(x_labels=dates, values=huge_series, max_points=2000)  # lossy but bounded output size
chart = sp.line(x_labels=dates, values=huge_series)  # >3000 points auto-switches to full-resolution Canvas2D pan/zoom
```

## Build a live-updating local dashboard

```python
app = sp.App("Live Metrics")
app.dropdown("period", ["7d", "30d"])
app.chart("out", sp.line("Sales", labels=["A", "B"], values=[1, 2]).html)
app.add_callback(inputs=["period"], output="out", handler=lambda period: sp.line("Sales", labels=["A", "B"], values=fetch(period)))
app.serve(port=8787)
```

## Export for print / embed without a browser

```python
chart.save("report.html")           # self-contained HTML, no server needed
```

## Migration notes

- **From `matplotlib`**: SeraPlot charts are HTML/SVG, not raster by default — `chart.save("x.html")` instead of `plt.savefig("x.png")`. Data arguments are keyword-only lists (`labels=`, `values=`) rather than positional array-likes.
- **From `plotly`**: `fig.show()` → `chart` (auto-displays in Jupyter) or `chart.save(path)`. Trace-based composition (`fig.add_trace(...)`) has no direct equivalent — build one chart per call, compose with `sp.build_grid()` for multi-chart layouts.
- **From `pandas` to `SeraDFrame`**: `SeraDFrame.from_pandas(df)` / `.to_pandas()` round-trip losslessly — most method names match pandas (`groupby`, `sort_values`, `dropna`, `merge`) so existing pandas code is often a near-drop-in swap.

</div>

<div class="lang-fr">

Recettes courtes et orientées tâche pour des situations courantes. Chacune est copiable-collable telle quelle.

## Charger un CSV et tracer une colonne

```python
import seraplot as sp

df = sp.SeraDFrame.from_csv("ventes.csv")
par_region = df.groupby("region").agg({"revenu": "sum"})
chart = sp.bar(labels=par_region.column_str("region"), values=par_region.column_f64("revenu"))
chart.save("ventes_par_region.html")
```

## Gérer les données manquantes/invalides avant de tracer

Les `NaN`/`Infinity` dans une liste Python sont assainis automatiquement (remplacés par `0`) plutôt que de produire silencieusement un chart vide — mais pour un résultat significatif, nettoyez d'abord les données :

```python
df = sp.SeraDFrame.from_csv("capteur.csv")
df = df.dropna(subset=["mesure"])
chart = sp.line(x_labels=df.column_str("horodatage"), values=df.column_f64("mesure"))
```

## Comparer des distributions entre catégories

```python
chart = sp.violin(labels=["A", "B", "C"], values=toutes_valeurs, categories=labels_groupe)
```

## Construire un dashboard multi-graphiques

```python
charts = [sp.bar(labels=l, values=v).html for l, v in jeux_de_donnees]
dashboard = sp.build_grid(charts=charts, cols=2, title="Vue d'ensemble T3")
dashboard.save("dashboard.html")
```

## Gros volumes de données — décimation ou moteur Canvas2D

```python
chart = sp.line(x_labels=dates, values=grosse_serie, max_points=2000)  # avec perte, mais taille de sortie bornée
chart = sp.line(x_labels=dates, values=grosse_serie)  # >3000 points bascule automatiquement sur le Canvas2D plein-résolution avec zoom/pan
```

## Construire un dashboard local avec mise à jour en direct

```python
app = sp.App("Métriques en direct")
app.dropdown("period", ["7j", "30j"])
app.chart("out", sp.line("Ventes", labels=["A", "B"], values=[1, 2]).html)
app.add_callback(inputs=["period"], output="out", handler=lambda period: sp.line("Ventes", labels=["A", "B"], values=recuperer(period)))
app.serve(port=8787)
```

## Export pour impression / intégration sans navigateur

```python
chart.save("rapport.html")           # HTML autonome, aucun serveur requis
```

## Notes de migration

- **Depuis `matplotlib`** : les charts SeraPlot sont du HTML/SVG, pas du raster par défaut — `chart.save("x.html")` au lieu de `plt.savefig("x.png")`. Les arguments de données sont des listes en kwargs (`labels=`, `values=`) plutôt que des tableaux positionnels.
- **Depuis `plotly`** : `fig.show()` → `chart` (auto-affiché dans Jupyter) ou `chart.save(chemin)`. La composition par traces (`fig.add_trace(...)`) n'a pas d'équivalent direct — un chart par appel, composé avec `sp.build_grid()` pour les mises en page multi-graphiques.
- **Depuis `pandas` vers `SeraDFrame`** : `SeraDFrame.from_pandas(df)` / `.to_pandas()` font l'aller-retour sans perte — la plupart des noms de méthodes correspondent à pandas (`groupby`, `sort_values`, `dropna`, `merge`), donc le code pandas existant est souvent quasi directement réutilisable.

</div>
