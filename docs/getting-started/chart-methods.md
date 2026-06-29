# Chart methods

<style>
.cm-hero{margin:1.6em 0 2em;padding:28px 30px;border-radius:14px;background:linear-gradient(135deg,#0f172a 0%,#1e1b4b 60%,#312e81 100%);border:1px solid rgba(99,102,241,.35);box-shadow:0 18px 50px -12px rgba(99,102,241,.25),inset 0 1px 0 rgba(255,255,255,.05);position:relative;overflow:hidden}
.cm-hero::before{content:"";position:absolute;top:-40%;right:-10%;width:60%;height:180%;background:radial-gradient(ellipse at center,rgba(129,140,248,.18) 0%,transparent 65%);pointer-events:none}
.cm-hero h2{margin:0 0 8px;font-size:22px;color:#f5f3ff;font-weight:700;letter-spacing:-.01em;border:none}
.cm-hero p{margin:0;color:#cbd5e1;font-size:14.5px;line-height:1.6;max-width:62ch}
.cm-pills{display:flex;gap:8px;flex-wrap:wrap;margin-top:14px;position:relative;z-index:1}
.cm-pill{padding:4px 11px;background:rgba(99,102,241,.14);border:1px solid rgba(165,180,252,.3);border-radius:999px;font-size:11px;font-weight:600;color:#c7d2fe;letter-spacing:.04em}

.cm-toc{display:grid;grid-template-columns:repeat(auto-fit,minmax(220px,1fr));gap:10px;margin:1.4em 0 2em}
.cm-toc a{display:flex;align-items:center;gap:10px;padding:12px 14px;background:#0a0f1c;border:1px solid #1e293b;border-radius:10px;color:#cbd5e1;text-decoration:none;font-size:13px;font-weight:600;transition:border-color .15s,transform .15s,color .15s}
.cm-toc a:hover{border-color:#3730a3;transform:translateY(-2px);color:#e0e7ff}
.cm-toc .cm-ic{flex-shrink:0;width:30px;height:30px;border-radius:8px;background:linear-gradient(135deg,#3730a3,#1e1b4b);display:flex;align-items:center;justify-content:center;color:#a5b4fc;font-weight:800;font-size:13px;letter-spacing:-.5px}

.cm-section{margin:2.4em 0 .6em;padding:14px 18px;border-radius:10px 10px 0 0;background:linear-gradient(90deg,rgba(99,102,241,.12),rgba(99,102,241,0));border-left:3px solid #6366f1;display:flex;align-items:center;gap:12px}
.cm-section .cm-sn{display:flex;align-items:center;justify-content:center;width:30px;height:30px;border-radius:8px;background:#6366f1;color:#fff;font-weight:800;font-size:14px;letter-spacing:-.5px;box-shadow:0 4px 14px -4px rgba(99,102,241,.55)}
.cm-section h3{margin:0;font-size:18px;color:#e0e7ff;font-weight:700;border:none;padding:0}
.cm-section p{margin:0 0 0 auto;color:#94a3b8;font-size:12.5px;font-style:italic}

.cm-card{margin:1em 0 1.4em;padding:18px 20px;background:#0a0f1c;border:1px solid #1e293b;border-left:3px solid #475569;border-radius:0 10px 10px 0;box-shadow:0 6px 18px -8px rgba(0,0,0,.5)}
.cm-card.cm-new{border-left-color:#22c55e}
.cm-card .cm-name{display:flex;align-items:center;gap:10px;margin:0 0 6px;flex-wrap:wrap}
.cm-card code.cm-fn{font-family:"JetBrains Mono",Consolas,monospace;font-size:14px;color:#a5b4fc;background:rgba(99,102,241,.10);padding:3px 9px;border-radius:6px;font-weight:600}
.cm-tag{display:inline-block;padding:2px 8px;border-radius:999px;font-size:10.5px;font-weight:700;letter-spacing:.04em;text-transform:uppercase}
.cm-tag.cm-tag-new{background:rgba(34,197,94,.15);color:#86efac;border:1px solid rgba(34,197,94,.35)}
.cm-tag.cm-tag-chain{background:rgba(99,102,241,.15);color:#c7d2fe;border:1px solid rgba(99,102,241,.35)}
.cm-tag.cm-tag-global{background:rgba(251,191,36,.15);color:#fde68a;border:1px solid rgba(251,191,36,.35)}
.cm-tag.cm-tag-export{background:rgba(244,63,94,.15);color:#fda4af;border:1px solid rgba(244,63,94,.35)}
.cm-tag.cm-tag-prop{background:rgba(168,85,247,.15);color:#d8b4fe;border:1px solid rgba(168,85,247,.35)}
.cm-tag.cm-tag-alias{background:rgba(20,184,166,.15);color:#5eead4;border:1px solid rgba(20,184,166,.35);text-transform:none;font-weight:600}
.cm-card .cm-desc{margin:6px 0 10px;color:#cbd5e1;font-size:13.5px;line-height:1.6}
.cm-card pre{margin:0;border-radius:8px;background:#06080f;border:1px solid #131a2a;padding:12px 14px;overflow-x:auto}
.cm-card pre code{font-size:12.5px;line-height:1.55;color:#e2e8f0;background:none;padding:0}
.cm-params{display:flex;flex-direction:column;gap:6px;margin-top:8px}
.cm-param{display:flex;align-items:baseline;gap:10px;flex-wrap:wrap;font-size:12.5px}
.cm-param code{background:#1e293b;padding:1px 7px;border-radius:5px;color:#e2e8f0;font-size:12px}
.cm-param-ty{color:#818cf8;font-family:"JetBrains Mono",Consolas,monospace;font-size:11.5px}
.cm-param-desc{color:#94a3b8}

.cm-table{width:100%;border-collapse:collapse;margin:1em 0;font-size:13px;background:#0a0f1c;border-radius:8px;overflow:hidden;border:1px solid #1e293b}
.cm-table th{background:#0f172a;color:#a5b4fc;padding:10px 14px;text-align:left;font-weight:700;font-size:12px;text-transform:uppercase;letter-spacing:.04em;border-bottom:1px solid #1e293b}
.cm-table td{padding:9px 14px;color:#cbd5e1;border-bottom:1px solid #131a2a;vertical-align:top}
.cm-table tr:last-child td{border-bottom:none}
.cm-table code{background:#1e293b;padding:1px 6px;border-radius:4px;font-size:12px;color:#e2e8f0}

.cm-tip{margin:1em 0;padding:12px 16px;background:rgba(34,197,94,.06);border-left:3px solid #22c55e;border-radius:0 6px 6px 0;color:#cbd5e1;font-size:13.5px;line-height:1.55}
.cm-tip strong{color:#86efac;font-weight:700}
.cm-warn{margin:1em 0;padding:12px 16px;background:rgba(251,146,60,.06);border-left:3px solid #fb923c;border-radius:0 6px 6px 0;color:#cbd5e1;font-size:13.5px;line-height:1.55}
.cm-warn strong{color:#fdba74;font-weight:700}
</style>
<div class="lang-en">

<div class="cm-hero">
<h2>Chart methods & global config</h2>
<p>Every <code>Chart</code> object returned by SeraPlot supports the same fluent API. Set defaults once with <code>sp.config()</code> and tweak per chart with chainable methods. All methods return a new <code>Chart</code>, so chaining is always safe.</p>
<div class="cm-pills"><span class="cm-pill">Fluent API</span><span class="cm-pill">Global + per-chart override</span><span class="cm-pill">60+ chart types</span><span class="cm-pill">Doc-as-code: every card below is generated from the Rust source</span></div>
</div>

<div class="cm-toc">
<a href="#global-config"><span class="cm-ic">1</span><span>Global config</span></a>
<a href="#themes"><span class="cm-ic">2</span><span>Themes</span></a>
<a href="#chart-methods"><span class="cm-ic">3</span><span>Chart methods</span></a>
<a href="#magic-properties"><span class="cm-ic">4</span><span>Magic properties</span></a>
<a href="#annotations-cross-chart"><span class="cm-ic">5</span><span>Annotations (cross-chart)</span></a>
<a href="#composers-grid-+-slideshow"><span class="cm-ic">6</span><span>Composers (Grid + Slideshow)</span></a>
</div>
<div class="cm-section"><span class="cm-sn">1</span><h3>Global config</h3></div>

Set defaults once. <strong>Every chart</strong> created afterwards inherits this configuration automatically.

<div data-sp-registry-table="methods" data-file="config/config.md,config/global.md"></div>
<div class="cm-section"><span class="cm-sn">2</span><h3>Themes</h3></div>

Curated presets that combine palette, background and gridlines.

<table class="cm-table"><thead><tr><th>Theme</th><th>Mood</th></tr></thead><tbody><tr><td><code>"dark"</code></td><td>High-contrast dark dashboard</td></tr><tr><td><code>"light"</code></td><td>Soft pastel light backdrop</td></tr><tr><td><code>"scientific"</code></td><td>Publication-style monochrome</td></tr><tr><td><code>"apple"</code></td><td>Glassy iOS-inspired palette</td></tr><tr><td><code>"notion"</code></td><td>Calm Notion-style neutrals</td></tr><tr><td><code>"minimal"</code></td><td>Bare lines, no chrome</td></tr><tr><td><code>"neon"</code></td><td>Vibrant cyberpunk neons</td></tr></tbody></table>
<div data-sp-registry-table="methods" data-file="theme/theme.md"></div>
<div class="cm-section"><span class="cm-sn">3</span><h3>Chart methods</h3><p>Generated from #[sera_doc] in the Rust source</p></div>

Every card below is generated straight from the <code>#[sera_doc(...)]</code> annotation on the matching Rust method — name, parameters and description always match the actual implementation.

<div data-sp-registry-table="methods" data-file="charts/chart.md,charts/export.md"></div>
<div class="cm-section"><span class="cm-sn">4</span><h3>Magic properties</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.html  # property</code><span class="cm-tag cm-tag-prop">property</span></div>
<div class="cm-desc">Read the full HTML payload as a string.</div>
<pre><code class="language-python">src = chart.html</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">len(chart) -&gt; int</code><span class="cm-tag cm-tag-prop">property</span></div>
<div class="cm-desc">Returns the size of the HTML payload in bytes.</div>
<pre><code class="language-python">print(len(chart))</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">bool(chart) -&gt; bool</code><span class="cm-tag cm-tag-prop">property</span></div>
<div class="cm-desc"><code>True</code> when the chart has rendered output.</div>
<pre><code class="language-python">if chart:
    chart.show()</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">5</span><h3>Annotations (cross-chart)</h3><p>SVG overlays inherited by every plot</p></div>

<div class="cm-card cm-new">
<div class="cm-name"><code class="cm-fn">annotations=[...] (kwarg)</code><span class="cm-tag cm-tag-new">new</span><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Pass a list of annotation dicts to <strong>any</strong> chart builder. Coordinates default to fractional canvas space (<code>0.0 - 1.0</code>); set <code>"frac": false</code> to use raw pixels. Supported <code>kind</code> values: <code>"hline"</code>, <code>"vline"</code>, <code>"line"</code>, <code>"arrow"</code>, <code>"rect"</code>, <code>"text"</code>.</div>
<pre><code class="language-python">chart = sp.build_line_chart(
    "Sales", labels=months, values=sales,
    annotations=[
        {"kind":"hline", "y":0.5, "color":"#22c55e", "dash":"6 4", "text":"Target"},
        {"kind":"vline", "x":0.62, "color":"#f59e0b", "text":"Launch"},
        {"kind":"rect",  "x":0.05, "y":0.65, "x2":0.40, "y2":0.92,
                         "color":"#6366f1", "fill":"#6366f1", "opacity":0.10},
        {"kind":"arrow", "x":0.45, "y":0.30, "x2":0.85, "y2":0.18, "color":"#ef4444"},
        {"kind":"text",  "x":0.46, "y":0.28, "text":"Outlier", "color":"#ef4444"},
    ],
)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">Annotation fields</code></div>
<div class="cm-desc">Every annotation accepts the same shape:</div>
<table class="cm-table">
<thead><tr><th>Field</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>kind</code></td><td>str</td><td>required</td><td><code>hline</code> / <code>vline</code> / <code>line</code> / <code>arrow</code> / <code>rect</code> / <code>text</code></td></tr>
<tr><td><code>x</code>, <code>y</code></td><td>float</td><td>0.5</td><td>Anchor point (frac of canvas if <code>frac=True</code>, pixels otherwise)</td></tr>
<tr><td><code>x2</code>, <code>y2</code></td><td>float</td><td>1.0</td><td>End point for line / arrow / rect</td></tr>
<tr><td><code>color</code></td><td>str</td><td><code>"#ef4444"</code></td><td>Stroke / text color (CSS)</td></tr>
<tr><td><code>fill</code></td><td>str</td><td><code>"none"</code></td><td>Fill color (rect only)</td></tr>
<tr><td><code>stroke_width</code></td><td>float</td><td>1.5</td><td>Stroke width</td></tr>
<tr><td><code>dash</code></td><td>str</td><td><code>""</code></td><td>SVG dash array, e.g. <code>"6 4"</code></td></tr>
<tr><td><code>opacity</code></td><td>float</td><td>1.0</td><td>0.0 - 1.0</td></tr>
<tr><td><code>text</code></td><td>str</td><td>None</td><td>Optional label rendered next to the primitive</td></tr>
<tr><td><code>font_size</code></td><td>float</td><td>11.0</td><td>Label font size</td></tr>
<tr><td><code>frac</code></td><td>bool</td><td>true</td><td>Coordinate space: fractional (0-1) or pixel</td></tr>
</tbody>
</table>
</div>
<div class="cm-tip"><strong>Tip:</strong> Annotations apply uniformly to every 2D and 3D chart through the <code>apply_annotations()</code> hook - no per-chart wiring required.</div>

<div class="cm-section"><span class="cm-sn">6</span><h3>Composers (Grid + Slideshow)</h3><p>Group charts into stories</p></div>

<div class="cm-card cm-new">
<div class="cm-name"><code class="cm-fn">sp.grid(charts, cols=3, gap=16, bg_color="#0a0f1c", title="", cell_height=None) -&gt; Chart</code><span class="cm-tag cm-tag-new">new</span></div>
<div class="cm-desc">Compose any number of pre-built charts into a responsive CSS-grid layout, each chart hosted in its own iframe. <code>sp.build_grid</code> is an identical alias. <code>cell_height</code> defaults to the chart's own <code>height</code> attribute.</div>
<pre><code class="language-python">bar  = sp.build_bar_chart("Q-Sales", labels=["Q1","Q2","Q3","Q4"], values=[120,180,150,210])
line = sp.build_line_chart("Trend", labels=months, values=sales)
pie  = sp.build_pie_chart("Mix", labels=["A","B","C"], values=[40,35,25])

grid = sp.grid([bar, line, pie], cols=3, gap=14, title="Dashboard")
grid.show()</code></pre>
</div>
<div class="cm-card cm-new">
<div class="cm-name"><code class="cm-fn">sp.build_slideshow(charts, interval_ms=2500, title='', width=900, height=520) -&gt; Chart</code><span class="cm-tag cm-tag-new">new</span></div>
<div class="cm-desc">Build a navigable HTML carousel with previous / next buttons and an auto-advance progress bar - perfect for telling a story without a slide deck.</div>
<pre><code class="language-python">slides = [sp.build_bar_chart(f"Slide {i+1}", labels=["A","B"], values=[i, i+1]) for i in range(4)]
show = sp.build_slideshow(slides, interval_ms=2200, title="Quarterly Story")
show.show()</code></pre>
</div>

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<div class="cm-hero">
<h2>Méthodes & configuration globale</h2>
<p>Tout objet <code>Chart</code> renvoyé par SeraPlot expose la même API fluide. Définis les valeurs par défaut une fois avec <code>sp.config()</code>, ajuste chart par chart avec des méthodes chaînées. Toutes les méthodes retournent un nouveau <code>Chart</code> : le chaînage est toujours sûr.</p>
<div class="cm-pills"><span class="cm-pill">API fluide</span><span class="cm-pill">Global + override par chart</span><span class="cm-pill">60+ types de graphiques</span><span class="cm-pill">Doc-as-code : chaque carte ci-dessous vient du code Rust</span></div>
</div>

<div class="cm-toc">
<a href="#configuration-globale"><span class="cm-ic">1</span><span>Configuration globale</span></a>
<a href="#themes"><span class="cm-ic">2</span><span>Thèmes</span></a>
<a href="#methodes-de-chart"><span class="cm-ic">3</span><span>Méthodes de Chart</span></a>
<a href="#proprietes-magiques"><span class="cm-ic">4</span><span>Propriétés magiques</span></a>
<a href="#annotations-transversales"><span class="cm-ic">5</span><span>Annotations (transversales)</span></a>
<a href="#composers-grid-+-slideshow"><span class="cm-ic">6</span><span>Composers (Grid + Slideshow)</span></a>
</div>
<div class="cm-section"><span class="cm-sn">1</span><h3>Configuration globale</h3></div>

Définis les défauts une fois. <strong>Tous les graphiques</strong> créés ensuite héritent automatiquement de cette configuration.

<div data-sp-registry-table="methods" data-file="config/config.md,config/global.md"></div>
<div class="cm-section"><span class="cm-sn">2</span><h3>Thèmes</h3></div>

Préréglages combinant palette, fond et grille.

<table class="cm-table"><thead><tr><th>Thème</th><th>Ambiance</th></tr></thead><tbody><tr><td><code>"dark"</code></td><td>Tableau de bord sombre haut contraste</td></tr><tr><td><code>"light"</code></td><td>Fond clair pastel doux</td></tr><tr><td><code>"scientific"</code></td><td>Style publication monochrome</td></tr><tr><td><code>"apple"</code></td><td>Palette inspirée iOS, glassy</td></tr><tr><td><code>"notion"</code></td><td>Neutres calmes type Notion</td></tr><tr><td><code>"minimal"</code></td><td>Lignes pures, sans décoration</td></tr><tr><td><code>"neon"</code></td><td>Néons cyberpunk vibrants</td></tr></tbody></table>
<div data-sp-registry-table="methods" data-file="theme/theme.md"></div>
<div class="cm-section"><span class="cm-sn">3</span><h3>Méthodes de Chart</h3><p>Générées depuis #[sera_doc] dans le code Rust</p></div>

Chaque carte ci-dessous est générée directement depuis l'annotation <code>#[sera_doc(...)]</code> de la méthode Rust correspondante — nom, paramètres et description correspondent toujours à l'implémentation réelle.

<div data-sp-registry-table="methods" data-file="charts/chart.md,charts/export.md"></div>
<div class="cm-section"><span class="cm-sn">4</span><h3>Propriétés magiques</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.html  # property</code><span class="cm-tag cm-tag-prop">property</span></div>
<div class="cm-desc">Lit le HTML complet sous forme de chaîne.</div>
<pre><code class="language-python">src = chart.html</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">len(chart) -&gt; int</code><span class="cm-tag cm-tag-prop">property</span></div>
<div class="cm-desc">Renvoie la taille du HTML en octets.</div>
<pre><code class="language-python">print(len(chart))</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">bool(chart) -&gt; bool</code><span class="cm-tag cm-tag-prop">property</span></div>
<div class="cm-desc"><code>True</code> si le chart a un rendu.</div>
<pre><code class="language-python">if chart:
    chart.show()</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">5</span><h3>Annotations (transversales)</h3><p>Surcouches SVG héritées par tous les plots</p></div>

<div class="cm-card cm-new">
<div class="cm-name"><code class="cm-fn">annotations=[...] (kwarg)</code><span class="cm-tag cm-tag-new">nouveau</span><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Passe une liste de dictionnaires d'annotations à <strong>n'importe quel</strong> builder. Coordonnées fractionnaires par défaut (<code>0.0 - 1.0</code>) ; mets <code>"frac": false</code> pour utiliser des pixels. Valeurs supportées pour <code>kind</code> : <code>"hline"</code>, <code>"vline"</code>, <code>"line"</code>, <code>"arrow"</code>, <code>"rect"</code>, <code>"text"</code>.</div>
<pre><code class="language-python">chart = sp.build_line_chart(
    "Ventes", labels=mois, values=ventes,
    annotations=[
        {"kind":"hline", "y":0.5, "color":"#22c55e", "dash":"6 4", "text":"Cible"},
        {"kind":"vline", "x":0.62, "color":"#f59e0b", "text":"Lancement"},
        {"kind":"rect",  "x":0.05, "y":0.65, "x2":0.40, "y2":0.92,
                         "color":"#6366f1", "fill":"#6366f1", "opacity":0.10},
        {"kind":"arrow", "x":0.45, "y":0.30, "x2":0.85, "y2":0.18, "color":"#ef4444"},
        {"kind":"text",  "x":0.46, "y":0.28, "text":"Outlier", "color":"#ef4444"},
    ],
)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">Champs d'annotation</code></div>
<div class="cm-desc">Toutes les annotations partagent la même structure :</div>
<table class="cm-table">
<thead><tr><th>Champ</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>kind</code></td><td>str</td><td>requis</td><td><code>hline</code> / <code>vline</code> / <code>line</code> / <code>arrow</code> / <code>rect</code> / <code>text</code></td></tr>
<tr><td><code>x</code>, <code>y</code></td><td>float</td><td>0.5</td><td>Point d'ancrage (frac du canvas si <code>frac=True</code>, pixels sinon)</td></tr>
<tr><td><code>x2</code>, <code>y2</code></td><td>float</td><td>1.0</td><td>Point final pour line / arrow / rect</td></tr>
<tr><td><code>color</code></td><td>str</td><td><code>"#ef4444"</code></td><td>Couleur de trait / texte (CSS)</td></tr>
<tr><td><code>fill</code></td><td>str</td><td><code>"none"</code></td><td>Couleur de remplissage (rect uniquement)</td></tr>
<tr><td><code>stroke_width</code></td><td>float</td><td>1.5</td><td>Épaisseur du trait</td></tr>
<tr><td><code>dash</code></td><td>str</td><td><code>""</code></td><td>SVG dash array, ex. <code>"6 4"</code></td></tr>
<tr><td><code>opacity</code></td><td>float</td><td>1.0</td><td>0.0 - 1.0</td></tr>
<tr><td><code>text</code></td><td>str</td><td>None</td><td>Étiquette optionnelle à côté du primitif</td></tr>
<tr><td><code>font_size</code></td><td>float</td><td>11.0</td><td>Taille de la police de l'étiquette</td></tr>
<tr><td><code>frac</code></td><td>bool</td><td>true</td><td>Espace de coordonnées : fractionnaire (0-1) ou pixel</td></tr>
</tbody>
</table>
</div>
<div class="cm-tip"><strong>Astuce :</strong> les annotations s'appliquent uniformément à tous les charts 2D et 3D via le hook <code>apply_annotations()</code> — aucun branchement par chart requis.</div>

<div class="cm-section"><span class="cm-sn">6</span><h3>Composers (Grid + Slideshow)</h3><p>Regroupe des charts en histoires</p></div>

<div class="cm-card cm-new">
<div class="cm-name"><code class="cm-fn">sp.grid(charts, cols=3, gap=16, bg_color="#0a0f1c", title="", cell_height=None) -&gt; Chart</code><span class="cm-tag cm-tag-new">nouveau</span></div>
<div class="cm-desc">Empile N charts déjà construits dans une grille CSS responsive, chaque chart isolé dans son iframe. <code>sp.build_grid</code> est un alias identique. <code>cell_height</code> utilise par défaut la hauteur détectée dans le chart.</div>
<pre><code class="language-python">bar  = sp.build_bar_chart("Ventes-Q", labels=["Q1","Q2","Q3","Q4"], values=[120,180,150,210])
line = sp.build_line_chart("Tendance", labels=mois, values=ventes)
pie  = sp.build_pie_chart("Mix", labels=["A","B","C"], values=[40,35,25])

grid = sp.grid([bar, line, pie], cols=3, gap=14, title="Tableau de bord")
grid.show()</code></pre>
</div>
<div class="cm-card cm-new">
<div class="cm-name"><code class="cm-fn">sp.build_slideshow(charts, interval_ms=2500, title='', width=900, height=520) -&gt; Chart</code><span class="cm-tag cm-tag-new">nouveau</span></div>
<div class="cm-desc">Construit un carrousel HTML navigable avec boutons précédent/suivant et barre de progression - parfait pour raconter une histoire sans présentation externe.</div>
<pre><code class="language-python">slides = [sp.build_bar_chart(f"Slide {i+1}", labels=["A","B"], values=[i, i+1]) for i in range(4)]
show = sp.build_slideshow(slides, interval_ms=2200, title="Histoire trimestrielle")
show.show()</code></pre>
</div>

</div><!-- /lang-fr -->
