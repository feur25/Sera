# Installation

<style>
/* ── Install method switch ─────────────────────────────── */
.sp-inst-sw { border: 1px solid #30363d; border-radius: 10px; overflow: hidden; margin: 1.5em 0; }
.sp-inst-tabs {
  display: flex; background: #0d1117;
  border-bottom: 1px solid #1e293b; padding: 0 6px;
}
.sp-inst-tab {
  padding: 11px 20px; border: none; background: none; color: #64748b;
  font-size: 13px; font-weight: 600; cursor: pointer;
  border-bottom: 2px solid transparent; margin-bottom: -1px;
  transition: color .15s, border-color .15s; white-space: nowrap;
}
.sp-inst-tab:hover { color: #e2e8f0; }
.sp-inst-tab.on { color: #6366f1; border-bottom-color: #6366f1; }
.sp-inst-body { padding: 20px 22px; background: #0a0e1a; }
.sp-inst-pane { display: none; }
.sp-inst-pane.on { display: block; }
.sp-inst-desc { font-size: 13px; color: #94a3b8; margin: 0 0 10px; line-height: 1.6; }
.sp-inst-pane pre { margin: 0 0 10px; border-radius: 7px !important; background: #161b27 !important; }
.sp-inst-pane pre:last-of-type { margin-bottom: 0; }
.sp-inst-pane code { font-size: 12.5px !important; }
.sp-inst-tag {
  display: inline-flex; align-items: center; gap: 5px;
  background: #0f172a; border: 1px solid #1e293b; border-radius: 5px;
  padding: 3px 9px; font-size: 11px; font-weight: 600; color: #6366f1;
  margin-bottom: 10px;
}
.sp-inst-note {
  display: flex; align-items: flex-start; gap: 8px; margin-top: 12px;
  background: #0f172a; border: 1px solid #1e293b; border-radius: 7px;
  padding: 10px 13px; font-size: 12px; color: #64748b; line-height: 1.55;
}
.sp-inst-cmds { display: flex; flex-direction: column; gap: 8px; }

/* Why block */
.sp-why-card {
  display: grid; grid-template-columns: repeat(auto-fit, minmax(200px,1fr));
  gap: 12px; margin: 1.5em 0;
}
.sp-why-item {
  background: #0d1117; border: 1px solid #1e293b; border-radius: 10px;
  padding: 16px 18px;
}
.sp-why-item strong { display: block; font-size: 13px; color: #e2e8f0; margin-bottom: 5px; }
.sp-why-item span { font-size: 12px; color: #64748b; line-height: 1.55; }
.sp-size-cmp {
  display: flex; align-items: center; gap: 14px; flex-wrap: wrap;
  margin-top: 1.2em;
}
.sp-size-bar-wrap { flex: 1; min-width: 160px; }
.sp-size-bar-wrap label { font-size: 12px; color: #64748b; display: flex; justify-content: space-between; margin-bottom: 5px; }
.sp-size-bar { height: 8px; border-radius: 4px; }
</style>
<div class="lang-en">

## Requirements

- Python **3.8+**
- pip 21+

SeraPlot ships as a compiled Rust extension (`.pyd` / `.so`) bundled in the wheel. There is **no compiler required** on the user side — the binary is pre-built for each platform.

---
## Install

<div class="sp-inst-sw">
<div class="sp-inst-tabs">
<button class="sp-inst-tab on" onclick="spInst('en','pip',this)">📦 pip</button>
<button class="sp-inst-tab" onclick="spInst('en','uv',this)">⚡ uv</button>
<button class="sp-inst-tab" onclick="spInst('en','conda',this)">🐍 conda</button>
</div>
<div class="sp-inst-body">

<div id="sp-inst-en-pip" class="sp-inst-pane on">
<p class="sp-inst-desc">Standard Python package installer — works in any environment:</p>
<div class="sp-inst-cmds">
<pre><code class="language-bash">pip install seraplot</code></pre>
<pre><code class="language-bash">pip install seraplot==2.3.61</code></pre>
<pre><code class="language-bash">pip install --upgrade seraplot</code></pre>
</div>
<div class="sp-inst-note"><span>💡</span><span>For project isolation, always install inside a virtual environment: <code>python -m venv .venv && .venv\Scripts\activate</code></span></div>
</div>

<div id="sp-inst-en-uv" class="sp-inst-pane">
<div class="sp-inst-tag">⚡ Recommended — 10–100× faster than pip</div>
<p class="sp-inst-desc"><a href="https://github.com/astral-sh/uv" target="_blank" rel="noopener">uv</a> is a next-generation Python package manager written in Rust — resolves and installs packages in milliseconds.</p>
<pre><code class="language-bash">uv add seraplot</code></pre>
<div class="sp-inst-note"><span>💡</span><span>Run <code>pip install uv</code> once to install uv, then use <code>uv add</code> in any project.</span></div>
</div>

<div id="sp-inst-en-conda" class="sp-inst-pane">
<p class="sp-inst-desc">Install from the conda-forge channel, or declare it in your environment file:</p>
<pre><code class="language-bash">conda install -c conda-forge seraplot</code></pre>
<p class="sp-inst-desc" style="margin-top:12px">Or add to <code>environment.yml</code>:</p>
<pre><code class="language-yaml">dependencies:
  - pip:
    - seraplot</code></pre>
</div>

</div>
</div>

---

## Why the install is this simple

SeraPlot has **zero required Python dependencies**. The Rust extension is entirely self-contained — the HTML output embeds its own JavaScript inline and does not load anything from a CDN.

<div class="sp-why-card">
<div class="sp-why-item">
<strong>🌐 Works offline</strong>
<span>Charts render in air-gapped environments, emails, PDF exports via browser print — no CDN, no internet.</span>
</div>
<div class="sp-why-item">
<strong>🔒 No conflicts</strong>
<span>Zero dependency on <code>numpy</code>, <code>pandas</code>, or <code>scipy</code> — nothing to conflict with your existing stack.</span>
</div>
<div class="sp-why-item">
<strong>🚀 All platforms</strong>
<span>Pre-built wheels for Windows, Linux, and macOS. No compiler, no Rust toolchain needed.</span>
</div>
<div class="sp-why-item">
<strong>🪶 Tiny footprint</strong>
<span><code>pip install plotly</code> downloads ~15 MB. <code>pip install seraplot</code> downloads ~2 MB.</span>
</div>
</div>

</div>
<div class="lang-fr">

## Prérequis

- Python **3.8+**
- pip 21+

SeraPlot se distribue sous forme d'extension Rust compilée (`.pyd` / `.so`) incluse dans le wheel. **Aucun compilateur n'est requis** côté utilisateur — le binaire est pré-compilé pour chaque plateforme.

---
## Installer

<div class="sp-inst-sw">
<div class="sp-inst-tabs">
<button class="sp-inst-tab on" onclick="spInst('fr','pip',this)">📦 pip</button>
<button class="sp-inst-tab" onclick="spInst('fr','uv',this)">⚡ uv</button>
<button class="sp-inst-tab" onclick="spInst('fr','conda',this)">🐍 conda</button>
</div>
<div class="sp-inst-body">

<div id="sp-inst-fr-pip" class="sp-inst-pane on">
<p class="sp-inst-desc">Gestionnaire de paquets Python standard — fonctionne dans tous les environnements :</p>
<div class="sp-inst-cmds">
<pre><code class="language-bash">pip install seraplot</code></pre>
<pre><code class="language-bash">pip install seraplot==2.3.61</code></pre>
<pre><code class="language-bash">pip install --upgrade seraplot</code></pre>
</div>
<div class="sp-inst-note"><span>💡</span><span>Pour isoler votre projet, installez toujours dans un environnement virtuel : <code>python -m venv .venv && .venv\Scripts\activate</code></span></div>
</div>

<div id="sp-inst-fr-uv" class="sp-inst-pane">
<div class="sp-inst-tag">⚡ Recommandé — 10 à 100× plus rapide que pip</div>
<p class="sp-inst-desc"><a href="https://github.com/astral-sh/uv" target="_blank" rel="noopener">uv</a> est un gestionnaire de paquets Python nouvelle génération écrit en Rust — résout et installe les paquets en quelques millisecondes.</p>
<pre><code class="language-bash">uv add seraplot</code></pre>
<div class="sp-inst-note"><span>💡</span><span>Exécutez <code>pip install uv</code> une seule fois pour installer uv, puis utilisez <code>uv add</code> dans chaque projet.</span></div>
</div>

<div id="sp-inst-fr-conda" class="sp-inst-pane">
<p class="sp-inst-desc">Installez depuis le canal conda-forge, ou déclarez-le dans votre fichier d'environnement :</p>
<pre><code class="language-bash">conda install -c conda-forge seraplot</code></pre>
<p class="sp-inst-desc" style="margin-top:12px">Ou ajoutez dans <code>environment.yml</code> :</p>
<pre><code class="language-yaml">dependencies:
  - pip:
    - seraplot</code></pre>
</div>

</div>
</div>

---

## Pourquoi l'installation est aussi simple

SeraPlot n'a **aucune dépendance Python requise**. L'extension Rust est entièrement autonome — le HTML embarque son propre JavaScript sans rien charger depuis un CDN.

<div class="sp-why-card">
<div class="sp-why-item">
<strong>🌐 Fonctionne hors ligne</strong>
<span>Les graphiques se génèrent en environnement isolé, dans les e-mails, en impression PDF — sans CDN ni internet.</span>
</div>
<div class="sp-why-item">
<strong>🔒 Zéro conflit</strong>
<span>Aucune dépendance sur <code>numpy</code>, <code>pandas</code> ou <code>scipy</code> — rien qui puisse entrer en conflit avec votre stack.</span>
</div>
<div class="sp-why-item">
<strong>🚀 Toutes plateformes</strong>
<span>Wheels pré-compilés pour Windows, Linux et macOS. Aucun compilateur, aucune toolchain Rust requise.</span>
</div>
<div class="sp-why-item">
<strong>🪶 Empreinte minimale</strong>
<span><code>pip install plotly</code> télécharge ~15 Mo. <code>pip install seraplot</code> télécharge ~2 Mo.</span>
</div>
</div>

</div>

<script>
function spInst(lang, pane, btn) {
  var wrap = btn.closest('.sp-inst-sw');
  wrap.querySelectorAll('.sp-inst-pane').forEach(function(p) { p.classList.remove('on'); });
  wrap.querySelectorAll('.sp-inst-tab').forEach(function(b) { b.classList.remove('on'); });
  document.getElementById('sp-inst-' + lang + '-' + pane).classList.add('on');
  btn.classList.add('on');
}
</script>

</div>
