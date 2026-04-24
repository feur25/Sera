# Getting Started

> **« Plot anything. Train anything. Ship anywhere. »**

<div class="lang-en">

SeraPlot is a Rust-native engine that brings together everything you usually wire up by hand: **interactive charts**, **scikit-learn-compatible machine learning**, **streaming dashboards**, **export pipelines**, and a **VS Code authoring experience** — all in a single `pip install`, with zero runtime dependencies.

---

## What you can build in minutes

- A self-contained 21 KB HTML chart you can email, drop in S3, or return from a FastAPI endpoint
- A 1-million-point scatter that stays interactive in Jupyter, with no backend
- A trained `RandomForestClassifier` saved to disk and re-loaded later, with the same API as scikit-learn
- A live dashboard that streams new points in real time, with downsampling and drift detection
- A theme-customized PDF or PNG report, exported from a CI pipeline at 6 µs/chart

---

## Why teams switch

| Need | SeraPlot answer |
|---|---|
| **Replace matplotlib in one line** | `import seraplot.matplotlib as plt` — same API, instantly interactive |
| **Stop hosting a Plotly server** | Every chart is a standalone HTML file; no Dash, no Streamlit |
| **Render thousands of charts in CI** | 6 µs per chart, fully parallel, no GIL bottleneck |
| **Train ML faster than scikit-learn** | Native Rust models, 1.3× to 686× speedups |
| **Ship to air-gapped customers** | Zero CDN, zero JS framework, zero internet at render time |
| **Author charts inside VS Code** | Live preview, gallery, theme studio, snippets, auto-detected variables |

---

## Three things that make SeraPlot different

### 1. The output is the deliverable
Every chart is a complete HTML document. No backend, no CDN, no JavaScript framework. You can email it, attach it, version it in Git, drop it into Notion, or print it to PDF. It still works five years from now, even offline.

### 2. ML is part of the box
SeraPlot ships a scikit-learn-compatible ML stack written in Rust — DBSCAN, K-Means, RandomForest, GradientBoosting, SVM, PCA, GridSearchCV, train/test split, metrics. You don't need a second library to fit a model and visualize its output.

### 3. The same engine works everywhere
Python, JavaScript, TypeScript, Rust. Pandas, NumPy. CPython 3.8+. Windows, Linux, macOS. A wheel is a wheel. The HTML is the HTML. Nothing platform-specific leaks into your output.

---

## Read this first

1. **[Installation](installation.md)** — `pip install seraplot`, then a 30-second sanity check
2. **[Quick Start](quickstart.md)** — your first chart in 3 lines (Python, JS, TS)
3. **[Chart Object](chart-object.md)** — what every function returns and how to display, save, embed
4. **[Chart Methods](chart-methods.md)** — the universal API: theme, animate, zoom, crosshair, export

After that, jump to **[Charts](../charts/index.md)** for the 57 chart types, **[Machine Learning](../ml/index.md)** for the ML stack, or **[Configuration](../config/index.md)** for global theming and runtime options.

</div>

<div class="lang-fr">

SeraPlot est un moteur écrit en Rust qui réunit tout ce que vous assembleriez d'habitude à la main : **graphiques interactifs**, **machine learning compatible scikit-learn**, **dashboards en streaming**, **pipelines d'export**, et une **expérience d'édition VS Code** — le tout dans un seul `pip install`, avec zéro dépendance d'exécution.

---

## Ce que vous construisez en quelques minutes

- Un graphique HTML autonome de 21 Ko, envoyable par e-mail, déposable sur S3, ou retournable depuis un endpoint FastAPI
- Un nuage de points d'1 million de points qui reste interactif dans Jupyter, sans backend
- Un `RandomForestClassifier` entraîné, sauvegardé sur disque puis rechargé, avec la même API que scikit-learn
- Un tableau de bord live qui ingère de nouveaux points en temps réel, avec downsampling et détection de drift
- Un rapport PDF ou PNG personnalisé, exporté depuis une pipeline CI à 6 µs/graphique

---

## Pourquoi les équipes basculent

| Besoin | Réponse SeraPlot |
|---|---|
| **Remplacer matplotlib en une ligne** | `import seraplot.matplotlib as plt` — même API, interactif immédiatement |
| **Arrêter d'héberger un serveur Plotly** | Chaque graphique est un fichier HTML autonome ; pas de Dash, pas de Streamlit |
| **Rendre des milliers de graphiques en CI** | 6 µs par graphique, entièrement parallèle, sans goulot GIL |
| **Entraîner du ML plus vite que scikit-learn** | Modèles natifs Rust, accélérations de 1,3× à 686× |
| **Livrer à des clients en environnement isolé** | Zéro CDN, zéro framework JS, zéro internet au moment du rendu |
| **Coder vos graphiques dans VS Code** | Aperçu en direct, galerie, studio de thèmes, snippets, variables détectées automatiquement |

---

## Trois choses qui rendent SeraPlot différent

### 1. La sortie *est* le livrable
Chaque graphique est un document HTML complet. Pas de backend, pas de CDN, pas de framework JavaScript. Vous l'envoyez par e-mail, vous le versionnez avec Git, vous le déposez dans Notion, vous l'imprimez en PDF. Il fonctionne encore dans cinq ans, même hors-ligne.

### 2. Le ML est inclus
SeraPlot livre une stack ML compatible scikit-learn écrite en Rust — DBSCAN, K-Means, RandomForest, GradientBoosting, SVM, PCA, GridSearchCV, train/test split, métriques. Pas besoin d'une seconde librairie pour entraîner un modèle et visualiser sa sortie.

### 3. Le même moteur partout
Python, JavaScript, TypeScript, Rust. Pandas, NumPy. CPython 3.8+. Windows, Linux, macOS. Un wheel reste un wheel. Le HTML reste le HTML. Rien de spécifique à la plateforme ne fuit dans votre sortie.

---

## À lire en premier

1. **[Installation](installation.md)** — `pip install seraplot`, puis un test de 30 secondes
2. **[Démarrage rapide](quickstart.md)** — votre premier graphique en 3 lignes (Python, JS, TS)
3. **[Objet Chart](chart-object.md)** — ce que retourne chaque fonction et comment l'afficher, le sauvegarder, l'embarquer
4. **[Méthodes du graphique](chart-methods.md)** — l'API universelle : thème, animation, zoom, crosshair, export

Ensuite, dirigez-vous vers **[Graphiques](../charts/index.md)** pour les 57 types, **[Machine Learning](../ml/index.md)** pour la stack ML, ou **[Configuration](../config/index.md)** pour le thème global et les options d'exécution.

</div>
