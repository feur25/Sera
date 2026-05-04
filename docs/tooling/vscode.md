# VS Code Extension

<style>
/* ── Floating PiP GIF ──────────────────────────────────────────── */
#sp-pip-wrap {
  margin: 1.2em 0 2em;
  border-radius: 10px;
  overflow: hidden;
  box-shadow: 0 4px 28px rgba(0,0,0,.55);
  cursor: default;
}
#sp-pip-wrap img { width: 100%; display: block; }

#sp-pip-float {
  position: fixed;
  bottom: 24px; right: 24px;
  width: 270px;
  border-radius: 10px;
  background: #0d1117;
  box-shadow: 0 8px 36px rgba(0,0,0,.8);
  z-index: 8800;
  cursor: move;
  display: none;
  overflow: hidden;
  user-select: none;
}
#sp-pip-float.sp-pip-on {
  display: block;
  animation: spPipIn .38s cubic-bezier(.16,1,.3,1) both;
}
@keyframes spPipIn {
  from { opacity:0; transform:scale(.6) translate(-14px,18px); }
  to   { opacity:1; transform:scale(1)  translate(0,0); }
}
#sp-pip-float img { width:100%; display:block; border-radius:10px 10px 0 0; pointer-events:none; }
.sp-pip-bar {
  display: flex; align-items: center; justify-content: space-between;
  padding: 5px 9px; background: #161b27; border-top: 1px solid #1e293b;
}
.sp-pip-lbl { font-size:10px; color:#64748b; font-weight:600; letter-spacing:.05em; text-transform:uppercase; }
.sp-pip-x {
  background: none; border: none; color: #64748b;
  font-size: 18px; cursor: pointer; line-height: 1; padding: 1px 4px;
  border-radius: 4px; transition: color .15s, background .15s;
}
.sp-pip-x:hover { color: #f1f5f9; background: #1e293b; }

/* ── Install method switch ─────────────────────────────────────── */
.sp-isw { border: 1px solid #30363d; border-radius: 10px; overflow: hidden; margin: 1.5em 0; }
.sp-isw-hdr {
  display: flex; background: #0d1117;
  border-bottom: 1px solid #1e293b; padding: 0 6px;
}
.sp-isw-tab {
  padding: 11px 18px; border: none; background: none; color: #64748b;
  font-size: 13px; font-weight: 600; cursor: pointer;
  border-bottom: 2px solid transparent; margin-bottom: -1px;
  transition: color .15s, border-color .15s; white-space: nowrap;
}
.sp-isw-tab:hover { color: #e2e8f0; }
.sp-isw-tab.on { color: #6366f1; border-bottom-color: #6366f1; }
.sp-isw-body { padding: 20px 22px; background: #0a0e1a; }
.sp-isw-pane { display: none; }
.sp-isw-pane.on { display: block; }
.sp-isw-desc { font-size: 13px; color: #94a3b8; margin: 0 0 10px; }
.sp-isw-pane pre {
  margin: 0; border-radius: 7px !important;
  background: #161b27 !important;
}
.sp-isw-pane code { font-size: 12.5px !important; }
.sp-mkt-btn {
  display: inline-flex; align-items: center; gap: 7px; margin-top: 14px;
  background: linear-gradient(135deg,#6366f1,#8b5cf6);
  color: #fff !important; text-decoration: none !important;
  padding: 10px 20px; border-radius: 8px; font-size: 13px; font-weight: 700;
  transition: filter .15s;
}
.sp-mkt-btn:hover { filter: brightness(1.15); }
.sp-isw-note {
  display: flex; align-items: flex-start; gap: 8px; margin-top: 14px;
  background: #0f172a; border: 1px solid #1e293b; border-radius: 7px;
  padding: 10px 13px; font-size: 12px; color: #64748b; line-height: 1.5;
}
.sp-isw-note-ico { flex-shrink: 0; }
</style>

<!-- Hero GIF — shared between EN and FR (placed once, outside both lang divs) -->
<div id="sp-pip-wrap">
  <img id="sp-pip-img" src="https://raw.githubusercontent.com/feur25/seraplot-documentation/main/seraplot-demo.gif" alt="SeraPlot Live Preview" loading="lazy">
</div>

<!-- Floating mini GIF (PiP) -->
<div id="sp-pip-float">
  <img src="https://raw.githubusercontent.com/feur25/seraplot-documentation/main/seraplot-demo.gif" alt="">
  <div class="sp-pip-bar">
    <span class="sp-pip-lbl">Live Preview</span>
    <button class="sp-pip-x" id="sp-pip-close" title="Close">×</button>
  </div>
</div>

<div class="lang-en">

Official **SeraPlot** extension for Visual Studio Code — live preview, theme studio, snippets and a chart gallery.

**Marketplace**: <https://marketplace.visualstudio.com/items?itemName=feur25.seraplot-vscode>

---

## Install

<div class="sp-isw">
<div class="sp-isw-hdr">
<button class="sp-isw-tab on" onclick="spIsw('en','mp',this)">🛒 Marketplace</button>
<button class="sp-isw-tab" onclick="spIsw('en','vsix',this)">📦 .vsix file</button>
</div>
<div class="sp-isw-body">

<div id="sp-isw-en-mp" class="sp-isw-pane on">
<p class="sp-isw-desc">From the command palette or terminal — no browser needed:</p>
<pre><code>ext install feur25.seraplot-vscode</code></pre>
<p class="sp-isw-desc" style="margin-top:12px">Or from the Extensions view <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>X</kbd>, search <strong>SeraPlot</strong>:</p>
<a class="sp-mkt-btn" href="https://marketplace.visualstudio.com/items?itemName=feur25.seraplot-vscode" target="_blank" rel="noopener">Open in VS Code Marketplace →</a>
<div class="sp-isw-note">
<span class="sp-isw-note-ico">💡</span>
<span>The extension auto-detects your Python environment via <code>seraplot.pythonPath</code>. Works on Windows, macOS and Linux.</span>
</div>
</div>

<div id="sp-isw-en-vsix" class="sp-isw-pane">
<p class="sp-isw-desc">Download the <code>.vsix</code> from the GitHub releases page, then install via terminal:</p>
<pre><code class="language-bash">code --install-extension seraplot-vscode-0.6.1.vsix</code></pre>
<p class="sp-isw-desc" style="margin-top:12px">Or drag the <code>.vsix</code> file directly into the VS Code Extensions panel.</p>
<div class="sp-isw-note">
<span class="sp-isw-note-ico">📦</span>
<span>Ideal for air-gapped environments, corporate proxies, or pinning a specific version without going through the Marketplace.</span>
</div>
</div>

</div>
</div>

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

Extension officielle **SeraPlot** pour Visual Studio Code — aperçu en direct, theme studio, snippets et galerie de graphiques.

**Marketplace** : <https://marketplace.visualstudio.com/items?itemName=feur25.seraplot-vscode>

---

## Installation

<div class="sp-isw">
<div class="sp-isw-hdr">
<button class="sp-isw-tab on" onclick="spIsw('fr','mp',this)">🛒 Marketplace</button>
<button class="sp-isw-tab" onclick="spIsw('fr','vsix',this)">📦 Fichier .vsix</button>
</div>
<div class="sp-isw-body">

<div id="sp-isw-fr-mp" class="sp-isw-pane on">
<p class="sp-isw-desc">Depuis la palette de commandes ou un terminal — sans navigateur :</p>
<pre><code>ext install feur25.seraplot-vscode</code></pre>
<p class="sp-isw-desc" style="margin-top:12px">Ou depuis la vue Extensions <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>X</kbd>, cherchez <strong>SeraPlot</strong> :</p>
<a class="sp-mkt-btn" href="https://marketplace.visualstudio.com/items?itemName=feur25.seraplot-vscode" target="_blank" rel="noopener">Ouvrir dans le Marketplace VS Code →</a>
<div class="sp-isw-note">
<span class="sp-isw-note-ico">💡</span>
<span>L'extension détecte automatiquement votre environnement Python via <code>seraplot.pythonPath</code>. Fonctionne sur Windows, macOS et Linux.</span>
</div>
</div>

<div id="sp-isw-fr-vsix" class="sp-isw-pane">
<p class="sp-isw-desc">Téléchargez le <code>.vsix</code> depuis la page des releases GitHub, puis installez via le terminal :</p>
<pre><code class="language-bash">code --install-extension seraplot-vscode-0.6.1.vsix</code></pre>
<p class="sp-isw-desc" style="margin-top:12px">Ou glissez-déposez le fichier <code>.vsix</code> directement dans le panneau Extensions de VS Code.</p>
<div class="sp-isw-note">
<span class="sp-isw-note-ico">📦</span>
<span>Idéal pour les environnements hors ligne, les proxys d'entreprise, ou pour figer une version spécifique sans passer par le Marketplace.</span>
</div>
</div>

</div>
</div>

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

<script>
/* ── Install switch ──────────────────────────────────────────── */
function spIsw(lang, pane, btn) {
  var wrap = btn.closest('.sp-isw');
  wrap.querySelectorAll('.sp-isw-pane').forEach(function(p) { p.classList.remove('on'); });
  wrap.querySelectorAll('.sp-isw-tab').forEach(function(b) { b.classList.remove('on'); });
  document.getElementById('sp-isw-' + lang + '-' + pane).classList.add('on');
  btn.classList.add('on');
}

/* ── Floating PiP GIF ────────────────────────────────────────── */
(function () {
  var dismissed = false;
  var dragging  = false;
  var dragOX = 0, dragOY = 0;

  var wrap  = document.getElementById('sp-pip-wrap');
  var pip   = document.getElementById('sp-pip-float');
  var close = document.getElementById('sp-pip-close');
  if (!wrap || !pip || !close) return;

  /* close */
  close.addEventListener('click', function () {
    dismissed = true;
    pip.style.transition = 'opacity .3s, transform .3s';
    pip.style.opacity = '0';
    pip.style.transform = 'scale(.7) translate(-10px,10px)';
    setTimeout(function () {
      pip.style.display = 'none';
      pip.style.opacity = '';
      pip.style.transform = '';
      pip.style.transition = '';
      pip.classList.remove('sp-pip-on');
    }, 320);
  });

  /* mouse drag */
  pip.addEventListener('mousedown', function (e) {
    if (e.target === close) return;
    dragging = true;
    var r = pip.getBoundingClientRect();
    dragOX = e.clientX - r.left;
    dragOY = e.clientY - r.top;
    pip.style.transition = 'none';
    e.preventDefault();
  });
  document.addEventListener('mousemove', function (e) {
    if (!dragging) return;
    var x = Math.max(0, Math.min(window.innerWidth  - pip.offsetWidth,  e.clientX - dragOX));
    var y = Math.max(0, Math.min(window.innerHeight - pip.offsetHeight, e.clientY - dragOY));
    pip.style.left   = x + 'px';
    pip.style.right  = 'auto';
    pip.style.top    = y + 'px';
    pip.style.bottom = 'auto';
  });
  document.addEventListener('mouseup', function () { dragging = false; });

  /* touch drag */
  pip.addEventListener('touchstart', function (e) {
    if (e.target === close) return;
    var t = e.touches[0];
    var r = pip.getBoundingClientRect();
    dragOX = t.clientX - r.left;
    dragOY = t.clientY - r.top;
    pip.style.transition = 'none';
  }, { passive: true });
  document.addEventListener('touchmove', function (e) {
    if (!pip.classList.contains('sp-pip-on')) return;
    var t = e.touches[0];
    var x = Math.max(0, Math.min(window.innerWidth  - pip.offsetWidth,  t.clientX - dragOX));
    var y = Math.max(0, Math.min(window.innerHeight - pip.offsetHeight, t.clientY - dragOY));
    pip.style.left   = x + 'px';
    pip.style.right  = 'auto';
    pip.style.top    = y + 'px';
    pip.style.bottom = 'auto';
  }, { passive: true });

  /* IntersectionObserver: show mini when hero scrolls off-top */
  var io = new IntersectionObserver(function (entries) {
    var e = entries[0];
    if (!e.isIntersecting && e.boundingClientRect.top < 0 && !dismissed) {
      pip.style.display = 'block';
      pip.style.right  = '24px';
      pip.style.left   = 'auto';
      pip.style.bottom = '24px';
      pip.style.top    = 'auto';
      pip.style.transition = '';
      requestAnimationFrame(function () { pip.classList.add('sp-pip-on'); });
    } else if (e.isIntersecting) {
      pip.classList.remove('sp-pip-on');
      setTimeout(function () {
        if (!pip.classList.contains('sp-pip-on')) pip.style.display = 'none';
      }, 380);
    }
  }, { threshold: 0 });
  io.observe(wrap);
}());
</script>
