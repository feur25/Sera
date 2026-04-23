# VS Code Extension

> EN — Official **SeraPlot** extension for Visual Studio Code: live preview, theme studio, snippets and a chart gallery.
>
> FR — Extension officielle **SeraPlot** pour Visual Studio Code : aperçu en direct, theme studio, snippets et galerie de graphiques.

## Install — Installation

### Marketplace

```
ext install feur25.seraplot-vscode
```

Or from the Extensions view: search for **SeraPlot**.

### From a `.vsix`

```bash
code --install-extension seraplot-vscode-0.1.1.vsix
```

## Commands — Commandes

| ID | Title | EN | FR |
|----|-------|----|----|
| `seraplot.preview`     | SeraPlot: Live Preview      | Render every `sp.Chart` of the current Python file in a side panel and refresh on save | Affiche tous les `sp.Chart` du fichier Python courant dans un panneau et rafraîchit à la sauvegarde |
| `seraplot.themeStudio` | SeraPlot: Open Theme Studio | Pick palette + background, copy the generated `sp.set_global_background(...)` snippet | Choisir palette + fond, copier le snippet `sp.set_global_background(...)` généré |
| `seraplot.gallery`     | SeraPlot: Open Gallery      | Browse all chart families with thumbnails and one-click code samples | Parcourir toutes les familles de charts avec aperçus et exemples de code en un clic |

The **Live Preview** button also appears in the editor title bar for any `.py` file.

## Snippets

| Prefix | Description |
|--------|-------------|
| `seraplot-import`    | `import seraplot as sp` |
| `seraplot-bar`       | Minimal bar chart |
| `seraplot-scatter`   | Scatter chart |
| `seraplot-dashboard` | 2x2 grid layout |
| `seraplot-automl`    | `sp.auto_classify(...)` skeleton |
| `seraplot-drift`     | `sp.drift_detect(...)` skeleton |

## Settings — Paramètres

| Key | Default | EN | FR |
|-----|---------|----|----|
| `seraplot.pythonPath` | `python` | Interpreter used to render previews | Interpréteur Python utilisé pour les aperçus |
| `seraplot.autoReload` | `true`   | Re-render on save                   | Re-rendu à chaque sauvegarde |

Set `seraplot.pythonPath` to your project venv, e.g. `${workspaceFolder}/.venv/Scripts/python.exe` on Windows or `${workspaceFolder}/.venv/bin/python` on macOS / Linux.

## How the preview works — Fonctionnement de l'aperçu

1. The active `.py` file is executed via `runpy.run_path` in a child Python process using `seraplot.pythonPath`.
2. Every `sp.Chart` instance found in the module globals is exported with `sp.export_html(chart)`.
3. The HTML is concatenated and rendered inside a VS Code Webview panel.
4. With `seraplot.autoReload = true` the panel re-runs automatically when the file is saved.

> The preview is sandboxed in a Webview — no network access, no `eval`. Charts are CSP-safe (see [`config/csp.md`](../config/csp.md)).

## Source — Code source

Repository: <https://github.com/feur25/seraplot> — folder `seraplot-vscode/`.

License: MIT.
