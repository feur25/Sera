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

## The controller: one config call, every chart obeys it

Every pillar reads from the same global controller instead of repeating options per call. Set it once — font, palette, animation, locale — and every `Chart` built afterward inherits it automatically, with any single call still free to override a value locally.

```python
import seraplot as sp

sp.config(
    font="Inter", font_size=13,
    palette="viridis", background="#0d1117",
    animation=True, animation_duration=420,
    crosshair=True, zoom=True, tooltip=True,
    locale="en-US", thousands_sep=",",
    responsive=True, export_button=True,
)

sp.bar("Revenue", labels, values)          # inherits every setting above
sp.bar("Revenue", labels, values, font_size=16)  # overrides just this one
```

`sp.config()` exposes 30+ knobs this way — typography, palette/background, gridlines and despine, text auto-formatting and rotation, bar corner radius, watermark, drop shadow, crosshair/zoom/tooltip behavior, locale-aware number formatting, responsive layout, export button visibility. `sp.reset_config()` reverts to defaults, `sp.theme()` swaps a whole preset in one call. See [Chart Methods](getting-started/chart-methods.md) for the full controller reference and every chainable per-chart method, and [Configuration](config/index.md) for the config surface in depth.

## Accessibility and browser support, actually checked

Not a claim — verified this way: `axe-core` runs against real Chromium for every chart family with zero serious/critical violations; text and non-text contrast is measured (relative luminance, not eyeballed) against WCAG AA; keyboard navigation (roving tabindex, arrow/Home/End) works on data points and legends; the whole catalog loads with zero console errors on real Chromium, Firefox, and WebKit engines, not just one.

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

## Le contrôleur : un appel de config, tous les graphiques obéissent

Chaque pilier lit depuis le même contrôleur global plutôt que de répéter les options à chaque appel. Configurez une fois — police, palette, animation, locale — et chaque `Chart` construit ensuite hérite automatiquement, tout en restant libre de surcharger une valeur localement.

```python
import seraplot as sp

sp.config(
    font="Inter", font_size=13,
    palette="viridis", background="#0d1117",
    animation=True, animation_duration=420,
    crosshair=True, zoom=True, tooltip=True,
    locale="fr-FR", thousands_sep=" ",
    responsive=True, export_button=True,
)

sp.bar("Revenu", labels, values)                  # hérite de tous les réglages ci-dessus
sp.bar("Revenu", labels, values, font_size=16)     # surcharge juste celui-ci
```

`sp.config()` expose 30+ réglages de cette façon — typographie, palette/fond, gridlines et despine, formatage/rotation automatique du texte, rayon des coins de barre, watermark, ombre portée, comportement crosshair/zoom/tooltip, formatage numérique localisé, mise en page responsive, visibilité du bouton d'export. `sp.reset_config()` revient aux défauts, `sp.theme()` change tout un preset en un appel. Voir [Méthodes des graphiques](getting-started/chart-methods.md) pour la référence complète du contrôleur et chaque méthode chainable par graphique, et [Configuration](config/index.md) pour la surface de config en détail.

## Accessibilité et compatibilité navigateur, réellement vérifiées

Pas une affirmation — vérifié ainsi : `axe-core` s'exécute contre un vrai Chromium pour chaque famille de graphique, zéro violation serious/critical ; le contraste texte et non-texte est mesuré (luminance relative, pas à l'œil) contre WCAG AA ; la navigation clavier (tabindex flottant, flèches/Home/End) fonctionne sur les points de données et les légendes ; tout le catalogue charge sans erreur console sur de vrais moteurs Chromium, Firefox et WebKit, pas un seul.

## Par où commencer

- **[Installation](getting-started/installation.md)** — `pip install seraplot`
- **[Démarrage rapide](getting-started/quickstart.md)** — premier graphique en 3 lignes
- **[Intro SeraPlot](getting-started/intro-seraplot.md)** — le moteur de graphiques en détail, avec benchmarks face à face
- **[Vitrine](showcase.md)** — toutes les familles de graphiques et variantes, avec un aperçu image
- **[Machine Learning](ml/index.md)** — le catalogue d'estimateurs SeraML
- **[SeraDFrame](canvas/dframe.md)** — le dataframe natif
- **[Référence API](api/index.md)** — index complet des fonctions

</div>
