<div align="center">

# SeraPlot

**High-performance data visualization — Rust core, Python API**

[![PyPI](https://img.shields.io/pypi/v/seraplot)](https://pypi.org/project/seraplot/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/feur25/seraplot/blob/main/LICENSE)

</div>

SeraPlot is a Python visualization library with a Rust rendering engine. You call Python functions, SeraPlot returns a self-contained HTML file — 20 to 90 KB, no CDN, no external JavaScript, works offline.

The entire render pipeline runs in compiled Rust. There is no Python overhead on the hot path, no JavaScript runtime at build time, and no network call at display time. The result: render times measured in **microseconds**, not milliseconds.

---

## Speed

**Benchmark: Diabetes dataset (n=768, 40 runs). Render time includes HTML export.**

Speedup of SeraPlot vs Plotly exporting to HTML — the fair comparison for tools that produce embeddable output.

<div style="font-family:monospace;margin:1.2em 0">
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Pie</span>
  <div style="background:#6366f1;width:300px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">7,956×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Bar</span>
  <div style="background:#6366f1;width:245px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">6,488×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Lollipop</span>
  <div style="background:#6366f1;width:150px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">3,983×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Grouped Bar</span>
  <div style="background:#6366f1;width:136px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">3,596×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Candlestick</span>
  <div style="background:#6366f1;width:77px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">2,038×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Radar</span>
  <div style="background:#6366f1;width:56px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">1,498×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">KDE</span>
  <div style="background:#6366f1;width:28px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">753×</b>
</div>
</div>

Raw numbers (µs, log scale would make SeraPlot invisible):

| Chart | SeraPlot | Plotly figure | Plotly →HTML | Matplotlib |
|-------|----------|--------------|-------------|------------|
| Pie | **4.2** | 725 | 33,416 | 15,085 |
| Bar | **2.8** | 658 | 18,166 | 13,596 |
| Grouped Bar | **5.0** | 558 | 17,981 | 17,445 |
| Histogram | **12.4** | 2,496 | 32,762 | 37,973 |
| Scatter | **17.0** | 3,916 | 21,615 | 14,141 |
| Violin | **16.7** | 2,616 | 21,347 | 21,211 |
| Box Plot | **18.4** | 2,329 | 21,799 | 15,590 |
| KDE | **26.3** | 2,981 | 19,807 | 40,108 |
| Radar | **11.8** | 962 | 17,679 | 20,942 |
| Lollipop | **6.3** | 8,382 | 25,096 | 9,072 |
| Candlestick | **8.8** | 1,478 | 17,934 | N/A |
| Ridgeline | **88.8** | N/A | N/A | N/A |

Average speedup vs Plotly →HTML: **~3,500×**. The KDE worst case is still 753×. Ridgeline does not exist in Plotly or Matplotlib.

---

## Output file size

Plotly embeds its entire JavaScript bundle in every HTML export. A single Pie chart from Plotly is **4.7 MB**. The same chart from SeraPlot is **19 KB** — because SeraPlot only includes the minimal JavaScript needed for that specific chart type, not a general-purpose charting framework.

<div style="font-family:monospace;margin:1.2em 0">
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Pie</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px" title="SeraPlot 19 KB"></div>
    <div style="background:#ef4444;width:300px;height:16px;border-radius:2px" title="Plotly 4,733 KB"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">19 KB vs 4,733 KB &mdash; Plotly <b style="color:#ef4444">246×</b> larger</span>
</div>
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Bar</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:274px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">21 KB vs 4,733 KB &mdash; Plotly <b style="color:#ef4444">225×</b> larger</span>
</div>
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Scatter</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:148px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">39 KB vs 4,740 KB &mdash; Plotly <b style="color:#ef4444">121×</b> larger</span>
</div>
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Radar</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:250px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">23 KB vs 4,733 KB &mdash; Plotly <b style="color:#ef4444">205×</b> larger</span>
</div>
</div>

Matplotlib outputs PNG/SVG/PDF (50–500 KB) — not self-contained HTML.

---

## What SeraPlot actually is

SeraPlot is **not a wrapper** around Plotly, Chart.js, D3, or any JavaScript library. It is a standalone renderer written in Rust that produces HTML with its own minimal embedded JavaScript.

Each chart type has its own focused JS implementation. A Pie chart gets Pie JS. A Bar chart gets Bar JS. Nothing else is bundled. This is why the output is 20 KB instead of 4.7 MB.

The Python API is a thin binding layer — it validates inputs, calls into the Rust library, and returns an object whose `.to_html()` method gives you a complete standalone HTML string.

**Concrete consequences of this architecture:**

- No internet connection at render time — no CDN calls, no external fonts, no remote scripts
- Render latency in microseconds — you can generate thousands of charts per second in a pipeline
- Output files small enough to embed in emails, check into Git, or serve from a static file host
- Zero Python dependency conflicts — SeraPlot has no required Python dependencies at all
- 55+ chart types including 17 WebGL 3D charts and a Rust-native DBSCAN that runs 600× faster than scikit-learn

---
## Pourquoi SeraPlot

### Contrôle total du rendu — sans exception

Chaque graphique SeraPlot est un document HTML autonome. Cela signifie que chaque propriété visuelle est contrôlable via CSS appliqué directement au SVG rendu. SeraPlot expose cela en première classe via une chaîne de méthodes sur l'objet `Chart` :

```python
chart = (
    sp.build_bar_chart("Revenus mensuels", labels, values)
    .set_bg("#0f172a")                        # fond sombre
    .show_grid()                               # grille visible
    .no_axes()                                 # supprimer X et Y
    .show_labels(position="top")               # labels au-dessus des barres
    .set_font_size(13)                         # taille de police globale
    .inject_css("""
        .sp-gl { stroke: #334155 !important; }
        svg text { fill: #e2e8f0 !important; }
    """)
    .inject_js("document.querySelector('svg').style.cursor = 'crosshair';")
)
```

Surface de contrôle complète :

| Méthode | Effet |
|---------|-------|
| `set_bg(color)` | Couleur de fond du HTML entier |
| `set_global_background(color)` | Applique à tous les graphiques de la session |
| `set_frame(color)` | Fond du canvas SVG indépendamment du wrapper HTML |
| `show_grid()` / `hide_grid()` | Activer ou désactiver les lignes de grille |
| `no_x_axis()` / `no_y_axis()` / `no_axes()` | Supprimer les axes sélectivement |
| `show_labels(position, labels, colors)` | Faire apparaître des labels sur chaque élément (top, bottom, left, right) |
| `no_legend()` | Supprimer la légende |
| `no_title()` | Supprimer le titre |
| `set_font_size(px)` | Remplacer toutes les tailles de texte dans le SVG |
| `scale(factor)` | Mettre à l'échelle le graphique entier |
| `inject_css(css)` | Injecter du CSS arbitraire dans le `<head>` — accès complet au DOM |
| `inject_js(js)` | Injecter du JavaScript arbitraire avant `</body>` — comportement illimité |

`inject_css` et `inject_js` ne sont pas des issues de secours. Ce sont des API de premier rang. Ils donnent un accès direct au SVG DOM rendu : remplacer n'importe quelle classe interne, attacher des écouteurs d'événements, animer des éléments, intégrer des systèmes externes — tout ce qu'un navigateur peut faire.

---

### Tooltips enrichis : images, vidéos, HTML

Le survol SeraPlot n'est pas un attribut `title`. C'est un moteur de tooltip structuré. Chaque point de données possède son propre tooltip indépendant :

```python
import seraplot as sp

hover = sp.build_hover_json({
    "Produit":  ["Widget A",              "Widget B",    "Widget C"   ],
    "Revenu":   ["€142 000",              "€98 500",     "€210 000"   ],
    "Unités":   ["1 420",                 "985",          "2 100"     ],
    "image":    ["https://cdn.acme.com/a.png", "...",     "..."       ],
})

chart = sp.build_bar_chart("Revenus produit", labels, values, hover_json=hover)
```

Contenu disponible par point de données :

- **Lignes clé/valeur** — nombre illimité de champs étiquetés
- **Image inline** — champ `image` : photo affichée dans le tooltip
- **Vidéo inline** — champ `video` : lecteur vidéo intégré dans le tooltip
- **HTML arbitraire** — champ `html` : contenu HTML brut injecté dans le tooltip

Le tooltip est entièrement rendu dans le HTML autonome — aucune requête réseau supplémentaire si les assets sont locaux.

---

### Architecture cross-langage : un moteur Rust, toutes les surfaces

Le cœur Rust de SeraPlot expose un ABI C stable (fonctions cdecl `#[no_mangle]`). Le même `.dll`/`.so`/`.dylib` compilé est directement appelable depuis :

- **Python** — via des wheels PyO3 (zéro overhead, zéro marshaling)
- **C / C++** — FFI directe
- **Node.js** — via `ffi-napi` ou `node-addon-api`
- **Julia** — via `ccall`
- **Go** — via `cgo`
- **R** — via `.Call`
- **Tout langage disposant d'un FFI C**

Ce n'est pas une bibliothèque Python avec des internals Rust. C'est une bibliothèque Rust avec une surface Python — et une surface C — et toute autre surface que vous souhaitez lier. Vous obtenez la même latence de rendu en microsecondes dans n'importe quel langage.

---

### Débit qui rend de nouveaux produits possibles

L'avantage en vitesse n'est pas académique. À 2–90 µs par graphique, SeraPlot rend réalisables des catégories entières de produits qui ne sont pas faisables avec d'autres bibliothèques Python :

| Cas d'usage | À 18 µs par scatter chart |
|-------------|---------------------------|
| 1 000 graphiques personnalisés par requête HTTP | **18 ms** — inline dans la réponse |
| 100 000 graphiques par run CI | **1,8 seconde** — faisable à chaque commit |
| 1 000 000 variantes A/B test | **18 secondes** — une ligne de commande |
| Graphique par ligne, export DataFrame 10 000 lignes | **180 ms** — zéro infrastructure supplémentaire |

Ce n'est pas être plus rapide pour la même charge de travail. C'est que la charge de travail devient le produit.

---

### Fichiers conçus pour le déploiement réel

Un graphique qui pèse 19 Ko au lieu de 4,7 Mo n'est pas une amélioration cosmétique. Cela change ce que vous pouvez en faire :

- **Pièces jointes email** : les serveurs rejettent généralement les PJ au-delà de 10–25 Mo. Un lot de 500 graphiques SeraPlot (~10 Mo) serait un export Plotly de 2 350 Mo.
- **Contrôle de version** : des fichiers HTML de 20 Ko sont lisibles dans `git diff`. Des blobs binaires de 4,7 Mo ne le sont pas.
- **CDN statique** : 100 000 graphiques à 20 Ko = 2 Go. À 4,7 Mo = 470 Go. La différence est une ligne de facture AWS.
- **Déploiement hors-ligne** : le HTML s'ouvre dans n'importe quel navigateur sans connexion internet — aucun CDN, aucune police distante, aucun script externe, jamais.
- **Notebooks Jupyter** : 50 graphiques Plotly inline gonflent le `.ipynb` à plus de 235 Mo. Avec SeraPlot, il reste sous 5 Mo.

---

### Types de graphiques exclusifs

SeraPlot implémente des types de graphiques absents de toute autre bibliothèque Python :

| Type de graphique | Pourquoi il n'existe pas ailleurs |
|-------------------|-----------------------------------|
| **Ridgeline** | Courbes KDE superposées pour comparer plusieurs distributions simultanément — aucun équivalent Plotly, pas de natif Matplotlib |
| **Dumbbell** | Delta avant/après par catégorie — uniquement disponible comme contournement manuel ailleurs |
| **Slope** | Changement de rang entre deux instants — pas un type de graphique standard |
| **Bullet** | Jauge KPI avec zones de performance et ligne cible — absent de Plotly et Matplotlib |
| **Globe 3D** | Sphère WebGL 3D avec données géospatiales — impossible en Python sans D3/Deck.gl |
| **Slideshow** | Carrousel multi-graphiques dans un seul fichier HTML — unique à SeraPlot |
| **GPU Scatter 3D** | Nuage de points WebGL pour des millions de points sans sous-échantillonnage |

---

### Machine learning natif dans le même pipeline de rendu

SeraPlot embarque un DBSCAN écrit en Rust avec indexation spatiale KD-tree et accélération SIMD. Ce n'est pas un wrapper autour de scikit-learn :

| Points | scikit-learn | SeraPlot DBSCAN | Facteur |
|--------|-------------|-----------------|---------|
| 1 000 | 3,2 ms | 0,18 ms | **18×** |
| 10 000 | 54 ms | 1,1 ms | **49×** |
| 100 000 | 1 340 ms | 8,4 ms | **160×** |
| 500 000 | 21 000 ms | 38 ms | **553×** |

Vous clusterisez et rendez dans la même bibliothèque — sans installer scikit-learn, sans format de données intermédiaire, sans étape de conversion.

---

### Zéro dépendances

SeraPlot n'a aucune dépendance Python requise. Pas de numpy, pas de pandas, pas de scipy, pas de requests, pas de PIL. Un `pip install seraplot` — c'est tout.

Dans les environnements où la gestion des dépendances est une contrainte — réseaux d'entreprise, serveurs isolés, images Docker minimales, environnements conda avec conflits de versions — SeraPlot s'installe sans toucher à quoi que ce soit d'autre.

Le wheel fait 2 Mo. Plotly fait 15 Mo avec ses propres dépendances.

---

## Navigation

- **[Installation](getting-started/installation.md)** — `pip install seraplot`
- **[Quick Start](getting-started/quickstart.md)** — premier graphique en 3 lignes
- **[Graphiques 2D](charts/2d/bar.md)** — 33 types de graphiques
- **[Graphiques 3D](charts/3d/scatter3d.md)** — 17 types, moteur WebGL GPU
- **[Machine Learning](ml/dbscan.md)** — DBSCAN jusqu'à 600× plus rapide que scikit-learn
- **[Référence API](api/index.md)** — index complet des fonctions
