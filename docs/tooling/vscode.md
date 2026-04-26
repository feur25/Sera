# VS Code Extension

<div class="lang-en">

Official **SeraPlot** extension for Visual Studio Code: live preview, theme studio,
snippets and a chart gallery.

![SeraPlot Live Preview](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/seraplot-demo.gif)

**Marketplace**: https://marketplace.visualstudio.com/items?itemName=feur25.seraplot-vscode

---

## Install

### Marketplace

```
ext install feur25.seraplot-vscode
```

Or from the Extensions view: search for **SeraPlot**.

### From a `.vsix`

```bash
code --install-extension seraplot-vscode-0.3.9.vsix
```

---

## Commands

| ID | Title | Description |
|----|-------|-------------|
| `seraplot.preview`     | SeraPlot: Live Preview      | Render every `sp.Chart` of the current Python file in a side panel and refresh on save |
| `seraplot.themeStudio` | SeraPlot: Open Theme Studio | Pick a palette + background, copy the generated `sp.set_global_background(...)` snippet |
| `seraplot.gallery`     | SeraPlot: Open Gallery      | Browse all chart families with thumbnails and one-click code samples |

The **Live Preview** button also appears in the editor title bar for any `.py` file.

---

## Snippets

| Prefix | Description |
|--------|-------------|
| `seraplot-import`    | `import seraplot as sp` |
| `seraplot-bar`       | Minimal bar chart |
| `seraplot-scatter`   | Scatter chart |
| `seraplot-dashboard` | 2x2 grid layout |
| `seraplot-automl`    | `sp.auto_classify(...)` skeleton |
| `seraplot-drift`     | `sp.drift_detect(...)` skeleton |

---

## Settings

| Key | Default | Description |
|-----|---------|-------------|
| `seraplot.pythonPath` | `python` | Python interpreter used to render previews |
| `seraplot.autoReload` | `true`   | Re-render on save |

Set `seraplot.pythonPath` to your project venv, e.g.
`${workspaceFolder}/.venv/Scripts/python.exe` on Windows or
`${workspaceFolder}/.venv/bin/python` on macOS / Linux.

---

## How the preview works

1. The active `.py` file is executed via `runpy.run_path` in a child Python process using `seraplot.pythonPath`.
2. Every `sp.Chart` instance found in the module globals is exported with `sp.export_html(chart)`.
3. The HTML is concatenated and rendered inside a VS Code Webview panel.
4. With `seraplot.autoReload = true` the panel re-runs automatically when the file is saved.

> The preview is sandboxed in a Webview — no network access, no `eval`. Charts are CSP-safe (see [`config/csp.md`](../config/csp.md)).

---

## Source

Repository: <https://github.com/feur25/seraplot> — folder `seraplot-vscode/`.

License: MIT.

</div>

<div class="lang-fr">

Extension officielle **SeraPlot** pour Visual Studio Code : aperçu en direct, theme
studio, snippets et galerie de graphiques.

![SeraPlot Live Preview](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/seraplot-demo.gif)

**Marketplace** : https://marketplace.visualstudio.com/items?itemName=feur25.seraplot-vscode

---

## Installation

### Marketplace

```
ext install feur25.seraplot-vscode
```

Ou depuis la vue Extensions : rechercher **SeraPlot**.

### Depuis un `.vsix`

```bash
code --install-extension seraplot-vscode-0.3.9.vsix
```

---

## Commandes

| ID | Titre | Description |
|----|-------|-------------|
| `seraplot.preview`     | SeraPlot: Live Preview      | Affiche tous les `sp.Chart` du fichier Python courant dans un panneau et rafraîchit à la sauvegarde |
| `seraplot.themeStudio` | SeraPlot: Open Theme Studio | Choisir une palette + un fond, copier le snippet `sp.set_global_background(...)` généré |
| `seraplot.gallery`     | SeraPlot: Open Gallery      | Parcourir toutes les familles de graphiques avec aperçus et exemples de code en un clic |

Le bouton **Live Preview** apparaît également dans la barre de titre de l'éditeur pour tout fichier `.py`.

---

## Snippets

| Préfixe | Description |
|---------|-------------|
| `seraplot-import`    | `import seraplot as sp` |
| `seraplot-bar`       | Graphique en barres minimal |
| `seraplot-scatter`   | Nuage de points |
| `seraplot-dashboard` | Grille 2×2 |
| `seraplot-automl`    | Squelette `sp.auto_classify(...)` |
| `seraplot-drift`     | Squelette `sp.drift_detect(...)` |

---

## Paramètres

| Clé | Défaut | Description |
|-----|--------|-------------|
| `seraplot.pythonPath` | `python` | Interpréteur Python utilisé pour les aperçus |
| `seraplot.autoReload` | `true`   | Re-rendu à chaque sauvegarde |

Pointer `seraplot.pythonPath` vers le venv du projet, par exemple
`${workspaceFolder}/.venv/Scripts/python.exe` sous Windows ou
`${workspaceFolder}/.venv/bin/python` sous macOS / Linux.

---

## Fonctionnement de l'aperçu

1. Le fichier `.py` actif est exécuté via `runpy.run_path` dans un processus Python enfant utilisant `seraplot.pythonPath`.
2. Chaque instance `sp.Chart` trouvée dans les globales du module est exportée avec `sp.export_html(chart)`.
3. Le HTML est concaténé et rendu dans un panneau Webview de VS Code.
4. Avec `seraplot.autoReload = true`, le panneau se relance automatiquement à chaque sauvegarde du fichier.

> L'aperçu est sandboxé dans un Webview — pas d'accès réseau, pas d'`eval`. Les graphiques sont CSP-safe (voir [`config/csp.md`](../config/csp.md)).

---

## Code source

Dépôt : <https://github.com/feur25/seraplot> — dossier `seraplot-vscode/`.

Licence : MIT.

</div>
