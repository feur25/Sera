<div align="center">

# Sera

**One Rust engine. Three pillars. Zero glue code.**

*SeraPlot for visualization, SeraML for machine learning, SeraDFrame for dataframes — compiled into a single native binary, calling each other directly instead of serializing across library boundaries.*

[![PyPI](https://img.shields.io/pypi/v/seraplot)](https://pypi.org/project/seraplot/)
[![npm](https://img.shields.io/npm/v/seraplot)](https://www.npmjs.com/package/seraplot)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/feur25/seraplot/blob/main/LICENSE)

</div>

<div class="lang-en">

## Why one engine instead of three libraries

Plotly, scikit-learn, and pandas are each excellent at what they do — and each is a separate library with its own object model. Connecting them means converting DataFrames to NumPy arrays to lists and back, at every boundary. Sera exists because that conversion cost is real, and because a chart, a trained model, and the table that fed both of them don't need to live in different worlds.

`sp.kmeans()` computes the clustering **and** renders the chart in the same Rust call — no round-trip through scikit-learn, no intermediate serialization. That is the whole design principle behind Sera: build once, natively, and let every pillar call the others directly.

| Pillar | What it is | Where to go deeper |
|---|---|---|
| **SeraPlot** | The rendering engine — 61 2D chart families, 24 3D types, 2 map types, each with several variants. Self-contained HTML/SVG output, no JS bundle. | [Intro SeraPlot](getting-started/intro-seraplot.md) · [Showcase](showcase.md) |
| **SeraML** | A scikit-learn-shaped machine learning layer written in Rust — linear models, trees, ensembles (random forest, gradient boosting, AdaBoost), SVM, k-NN, Naive Bayes, PCA, clustering, anomaly detection, model selection, metrics, preprocessing. Plus a model registry, PowerBI/Tableau export, and GPU/distributed backends that scikit-learn itself doesn't ship. | [Machine Learning](ml/index.md) |
| **SeraDFrame** | A native columnar dataframe, built for the same pipeline rather than adapted from one — no pandas dependency, no copy at the boundary when it feeds a chart or a model. | [SeraDFrame](canvas/dframe.md) |

## How to develop well with Sera

The three pillars share the same conventions on purpose, so that switching between them costs nothing:

- **One call, one result.** `sp.bar(...)`, `sp.linear_regression(...)`, `df.groupby(...)` all follow the same flat-function shape — no builder object to assemble first, no separate `.fit()` then `.transform()` unless the algorithm genuinely needs state across calls.
- **Aliases are free.** Every function is reachable under several names (`sp.bar` / `sp.bars` / `sp.bar_chart` / `sp.bar_unified`) resolved through a shared alias registry — pick the name that reads best in your code, they compile to the same call.
- **Native output, always.** A chart is real HTML/SVG, a model result is a real Python dict, a DataFrame column is real Rust-backed memory — never an opaque wrapper object you have to unwrap to get to the actual data.
- **The fastest path is the default path.** Because computation happens in Rust before anything crosses into Python, the ergonomic way to write code and the fast way to write code are the same code.

## Real metrics, measured on this codebase

Numbers below come from this project's own source and test suite — not marketing copy.

| Metric | Value |
|---|---|
| SeraPlot rendering engine | ≈ 72,200 lines of Rust (`plot/` + `html/` + `canvas/` + `bindings/`) |
| SeraML machine learning layer | ≈ 17,251 lines of Rust — 41 estimator functions, numerically verified against toy inputs (exact regression coefficients, correct PCA variance direction, coherent feature importances) |
| SeraDFrame + Table | ≈ 5,783 lines of Rust |
| Total native codebase | ≈ 105,000 lines of Rust across `v2/src/` |
| 2D chart families / 3D types / map types | 61 / 24 / 2, each with several rendering variants |
| Output size (bar chart, HTML) | ≈ 21 KB — no bundled JS runtime, vs. several MB for a Plotly-equivalent export |
| Accessibility | WCAG AA contrast verified, `axe-core` automated audit passing (0 serious/critical violations) across sample charts |
| Cross-browser | Chromium, Firefox, WebKit — real engines, 0 console errors across the chart catalog |

See [Intro SeraPlot](getting-started/intro-seraplot.md) for chart-generation speed benchmarks against Plotly and Matplotlib specifically.

## Where to start

- **[Installation](getting-started/installation.md)** — `pip install seraplot`
- **[Quick Start](getting-started/quickstart.md)** — first chart in 3 lines
- **[Intro SeraPlot](getting-started/intro-seraplot.md)** — the charting engine in depth, with head-to-head benchmarks
- **[Showcase](showcase.md)** — every chart family and variant, with a preview image
- **[Machine Learning](ml/index.md)** — SeraML's estimator catalog
- **[SeraDFrame](canvas/dframe.md)** — the native dataframe
- **[API Reference](api/index.md)** — complete function index

</div>

<div class="lang-fr">

## Pourquoi un seul moteur plutôt que trois bibliothèques

Plotly, scikit-learn et pandas sont chacun excellents dans leur domaine — et chacun est une bibliothèque séparée avec son propre modèle d'objets. Les connecter implique de convertir des DataFrames en tableaux NumPy puis en listes et inversement, à chaque frontière. Sera existe parce que ce coût de conversion est réel, et parce qu'un graphique, un modèle entraîné et la table qui a nourri les deux n'ont pas besoin de vivre dans des mondes différents.

`sp.kmeans()` calcule le clustering **et** rend le graphique dans le même appel Rust — aucun aller-retour vers scikit-learn, aucune sérialisation intermédiaire. C'est tout le principe de conception derrière Sera : construire une seule fois, nativement, et laisser chaque pilier appeler les autres directement.

| Pilier | Ce que c'est | Pour aller plus loin |
|---|---|---|
| **SeraPlot** | Le moteur de rendu — 61 familles de graphiques 2D, 24 types 3D, 2 types de cartes, chacun avec plusieurs variantes. Sortie HTML/SVG autonome, sans bundle JS. | [Intro SeraPlot](getting-started/intro-seraplot.md) · [Vitrine](showcase.md) |
| **SeraML** | Une couche de machine learning façon scikit-learn écrite en Rust — modèles linéaires, arbres, ensembles (random forest, gradient boosting, AdaBoost), SVM, k-NN, Naive Bayes, PCA, clustering, détection d'anomalies, sélection de modèle, métriques, preprocessing. Plus un registre de modèles, un export PowerBI/Tableau, et des backends GPU/distribués que scikit-learn lui-même n'a pas. | [Machine Learning](ml/index.md) |
| **SeraDFrame** | Un dataframe colonnaire natif, conçu pour le même pipeline plutôt qu'adapté depuis un autre — pas de dépendance pandas, pas de copie à la frontière quand il alimente un graphique ou un modèle. | [SeraDFrame](canvas/dframe.md) |

## Comment bien développer avec Sera

Les trois piliers partagent les mêmes conventions volontairement, pour que passer de l'un à l'autre ne coûte rien :

- **Un appel, un résultat.** `sp.bar(...)`, `sp.linear_regression(...)`, `df.groupby(...)` suivent tous la même forme de fonction à plat — pas d'objet builder à assembler d'abord, pas de `.fit()` puis `.transform()` séparés sauf si l'algorithme a réellement besoin d'état entre les appels.
- **Les alias sont gratuits.** Chaque fonction est accessible sous plusieurs noms (`sp.bar` / `sp.bars` / `sp.bar_chart` / `sp.bar_unified`) résolus via un registre d'alias partagé — choisissez le nom qui se lit le mieux dans votre code, ils compilent vers le même appel.
- **Sortie native, toujours.** Un graphique est du vrai HTML/SVG, un résultat de modèle est un vrai dict Python, une colonne de DataFrame est de la vraie mémoire portée par Rust — jamais un objet wrapper opaque qu'il faut déballer pour accéder à la donnée réelle.
- **Le chemin le plus ergonomique est le chemin le plus rapide.** Parce que le calcul se fait en Rust avant de traverser vers Python, la façon la plus naturelle d'écrire du code et la façon la plus rapide sont le même code.

## Métriques réelles, mesurées sur ce code

Les chiffres ci-dessous viennent du code source et de la suite de tests de ce projet — pas d'argument marketing.

| Métrique | Valeur |
|---|---|
| Moteur de rendu SeraPlot | ≈ 72 200 lignes Rust (`plot/` + `html/` + `canvas/` + `bindings/`) |
| Couche machine learning SeraML | ≈ 17 251 lignes Rust — 41 fonctions d'estimateurs, vérifiées numériquement sur des cas jouets (coefficients de régression exacts, direction de variance PCA correcte, importances de variables cohérentes) |
| SeraDFrame + Table | ≈ 5 783 lignes Rust |
| Total du code natif | ≈ 105 000 lignes Rust dans `v2/src/` |
| Familles de charts 2D / types 3D / types carte | 61 / 24 / 2, chacun avec plusieurs variantes de rendu |
| Taille de sortie (bar chart, HTML) | ≈ 21 Ko — aucun runtime JS embarqué, contre plusieurs Mo pour un export Plotly équivalent |
| Accessibilité | Contraste WCAG AA vérifié, audit automatisé `axe-core` au vert (0 violation serious/critical) sur l'échantillon de charts |
| Compatibilité navigateur | Chromium, Firefox, WebKit — moteurs réels, 0 erreur console sur le catalogue de charts |

Voir [Intro SeraPlot](getting-started/intro-seraplot.md) pour les benchmarks de vitesse de génération face à Plotly et Matplotlib.

## Par où commencer

- **[Installation](getting-started/installation.md)** — `pip install seraplot`
- **[Démarrage rapide](getting-started/quickstart.md)** — premier graphique en 3 lignes
- **[Intro SeraPlot](getting-started/intro-seraplot.md)** — le moteur de graphiques en détail, avec benchmarks face à face
- **[Vitrine](showcase.md)** — toutes les familles de graphiques et variantes, avec un aperçu image
- **[Machine Learning](ml/index.md)** — le catalogue d'estimateurs SeraML
- **[SeraDFrame](canvas/dframe.md)** — le dataframe natif
- **[Référence API](api/index.md)** — index complet des fonctions

</div>
