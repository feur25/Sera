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
.cm-card .cm-desc{margin:6px 0 10px;color:#cbd5e1;font-size:13.5px;line-height:1.6}
.cm-card pre{margin:0;border-radius:8px;background:#06080f;border:1px solid #131a2a;padding:12px 14px;overflow-x:auto}
.cm-card pre code{font-size:12.5px;line-height:1.55;color:#e2e8f0;background:none;padding:0}

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
<div class="cm-pills"><span class="cm-pill">Fluent API</span><span class="cm-pill">Global + per-chart override</span><span class="cm-pill">60+ chart types</span><span class="cm-pill">Plotly-compatible value labels</span></div>
</div>

<div class="cm-toc">
<a href="#global-config"><span class="cm-ic">1</span><span>Global config</span></a>
<a href="#themes"><span class="cm-ic">2</span><span>Themes</span></a>
<a href="#background-frame"><span class="cm-ic">3</span><span>Background & frame</span></a>
<a href="#grid-axes"><span class="cm-ic">4</span><span>Grid & axes</span></a>
<a href="#title-legend"><span class="cm-ic">5</span><span>Title & legend</span></a>
<a href="#typography"><span class="cm-ic">6</span><span>Typography</span></a>
<a href="#value-labels"><span class="cm-ic">7</span><span>Value labels</span></a>
<a href="#bar-styling"><span class="cm-ic">8</span><span>Bar styling</span></a>
<a href="#animation-interactivity"><span class="cm-ic">9</span><span>Animation & interactivity</span></a>
<a href="#responsive-layout"><span class="cm-ic">10</span><span>Responsive & layout</span></a>
<a href="#export-download"><span class="cm-ic">11</span><span>Export & download</span></a>
<a href="#custom-css-/-js"><span class="cm-ic">12</span><span>Custom CSS / JS</span></a>
<a href="#accessibility-csp"><span class="cm-ic">13</span><span>Accessibility & CSP</span></a>
<a href="#diff-introspection"><span class="cm-ic">14</span><span>Diff & introspection</span></a>
<a href="#magic-properties"><span class="cm-ic">15</span><span>Magic properties</span></a>
</div>
<div class="cm-section"><span class="cm-sn">1</span><h3>Global config</h3></div>

Set defaults once. <strong>Every chart</strong> created afterwards inherits this configuration automatically.

<table class="cm-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Effect</th></tr></thead>
<tbody>
<tr><td><code>font</code></td><td>str</td><td>system</td><td>Font family for every text element</td></tr>
<tr><td><code>font_size</code></td><td>int</td><td>12</td><td>Base font size in px</td></tr>
<tr><td><code>title_size</code></td><td>int</td><td>16</td><td>Title font size in px</td></tr>
<tr><td><code>border_radius</code></td><td>int</td><td>0</td><td>Container border radius (px)</td></tr>
<tr><td><code>opacity</code></td><td>float</td><td>1.0</td><td>Element opacity 0.0–1.0</td></tr>
<tr><td><code>responsive</code></td><td>bool</td><td>False</td><td>Auto-resize to container width</td></tr>
<tr><td><code>animation</code></td><td>bool</td><td>False</td><td>Fade-in animation on chart load</td></tr>
<tr><td><code>animation_duration</code></td><td>int</td><td>300</td><td>Animation duration in ms</td></tr>
<tr><td><code>crosshair</code></td><td>bool</td><td>False</td><td>Crosshair lines follow mouse hover</td></tr>
<tr><td><code>zoom</code></td><td>bool</td><td>False</td><td>Mouse wheel zoom + pan (double-click resets)</td></tr>
<tr><td><code>tooltip</code></td><td>str</td><td>—</td><td>Tooltip mode</td></tr>
<tr><td><code>locale</code></td><td>str</td><td>—</td><td>Number formatting locale</td></tr>
<tr><td><code>thousands_sep</code></td><td>str</td><td>—</td><td>Thousands separator character</td></tr>
<tr><td><code>margin</code></td><td>int</td><td>0</td><td>Container padding (px)</td></tr>
<tr><td><code>export_button</code></td><td>bool</td><td>False</td><td>Show download button on each chart</td></tr>
<tr><td><code>palette</code></td><td>list[int]</td><td>—</td><td>Color palette as hex ints (e.g. <code>[0x6366F1,0xFB7185]</code>)</td></tr>
<tr><td><code>background</code></td><td>str</td><td>—</td><td>Background color (any CSS color)</td></tr>
<tr><td><code>gridlines</code></td><td>bool</td><td>False</td><td>Show grid lines in chart</td></tr>
<tr><td><code>text_auto</code></td><td>bool / str</td><td>False</td><td>Display values on bars/markers (<code>True</code> raw, or d3 format like <code>".2s"</code>)</td></tr>
<tr><td><code>text_position</code></td><td>str</td><td>"auto"</td><td><code>"auto"</code> / <code>"inside"</code> / <code>"outside"</code></td></tr>
<tr><td><code>text_angle</code></td><td>int</td><td>0</td><td>Value label rotation (deg)</td></tr>
<tr><td><code>text_font_size</code></td><td>int</td><td>12</td><td>Max font size for value labels (px)</td></tr>
<tr><td><code>text_font_color</code></td><td>str</td><td>—</td><td>Value label color</td></tr>
<tr><td><code>uniform_text_min_size</code></td><td>int</td><td>0</td><td>Minimum px before label is hidden</td></tr>
<tr><td><code>uniform_text_mode</code></td><td>str</td><td>"hide"</td><td><code>"hide"</code> small labels or <code>"show"</code> overflow</td></tr>
<tr><td><code>bar_corner_radius</code></td><td>int / str</td><td>0</td><td>Bar corner radius in px or <code>"20%"</code> of bar width</td></tr>
</tbody>
</table>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">sp.config(**kwargs)</code><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Apply persistent defaults to all subsequent charts.</div>
<pre><code class="language-python">import seraplot as sp

sp.config(
    font=&quot;Inter&quot;, font_size=12, title_size=16,
    crosshair=True, zoom=True, animation=True,
    palette=[0x6366F1, 0xFB7185, 0x10B981],
    background=&quot;#fafafa&quot;, border_radius=8, margin=16,
)
chart = sp.bar(&quot;Sales&quot;, [&quot;Q1&quot;,&quot;Q2&quot;,&quot;Q3&quot;], [100,150,120])</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">sp.reset_config()</code><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Clear every global setting back to factory defaults.</div>
<pre><code class="language-python">sp.reset_config()
chart = sp.bar(&quot;Clean&quot;, labels, values)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">2</span><h3>Themes</h3></div>

Curated presets that combine palette, background and gridlines.

<table class="cm-table"><thead><tr><th>Theme</th><th>Mood</th></tr></thead><tbody><tr><td><code>"dark"</code></td><td>High-contrast dark dashboard</td></tr><tr><td><code>"light"</code></td><td>Soft pastel light backdrop</td></tr><tr><td><code>"scientific"</code></td><td>Publication-style monochrome</td></tr><tr><td><code>"apple"</code></td><td>Glassy iOS-inspired palette</td></tr><tr><td><code>"notion"</code></td><td>Calm Notion-style neutrals</td></tr><tr><td><code>"minimal"</code></td><td>Bare lines, no chrome</td></tr><tr><td><code>"neon"</code></td><td>Vibrant cyberpunk neons</td></tr></tbody></table>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">sp.theme(name)</code><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Apply a preset theme. Combine with <code>sp.config()</code> overrides.</div>
<pre><code class="language-python">sp.theme(&quot;dark&quot;)
chart = sp.bar(&quot;Modern&quot;, labels, values)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">sp.themes() -&gt; list[str]</code><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">List every available theme name.</div>
<pre><code class="language-python">available = sp.themes()
print(available)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">sp.reset_theme()</code><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Reset back to no theme (default palette, no background, no gridlines).</div>
<pre><code class="language-python">sp.reset_theme()</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">3</span><h3>Background & frame</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.set_bg(color=None) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Set the HTML wrapper background. Accepts any CSS color or <code>None</code> for transparent.</div>
<pre><code class="language-python">chart = sp.line(&quot;Trend&quot;, x, y).set_bg(&quot;#0f172a&quot;)
chart = sp.pie(&quot;Share&quot;, labels, values).set_bg(None)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.set_frame(color=None) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Set the SVG canvas background, independent from the HTML wrapper.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Title&quot;, labels, values).set_frame(&quot;transparent&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">sp.set_global_background(color)</code><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Apply a background to <strong>every</strong> chart created afterwards. Useful for notebooks.</div>
<pre><code class="language-python">sp.set_global_background(&quot;#0f172a&quot;)
chart1 = sp.bar(...)
chart2 = sp.scatter(...)
sp.reset_global_background()</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">4</span><h3>Grid & axes</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.show_grid() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Force gridlines on (overrides config).</div>
<pre><code class="language-python">chart = sp.bar(&quot;Data&quot;, labels, values).show_grid()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.hide_grid() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Force gridlines off.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Data&quot;, labels, values, gridlines=True).hide_grid()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.no_x_axis() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Drop X-axis line, ticks and labels.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Y only&quot;, labels, values).no_x_axis()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.no_y_axis() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Drop Y-axis line, ticks and labels.</div>
<pre><code class="language-python">chart = sp.bar(&quot;X only&quot;, labels, values).no_y_axis()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.no_axes() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Drop both axes at once.</div>
<pre><code class="language-python">chart = sp.scatter(&quot;Clean&quot;, x, y).no_axes()</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">5</span><h3>Title & legend</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.no_title() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Hide the chart title.</div>
<pre><code class="language-python">chart = sp.pie(&quot;Internal&quot;, labels, values).no_title()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.no_legend() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Hide the legend (multi-series charts).</div>
<pre><code class="language-python">chart = sp.grouped_bar(&quot;Q1&quot;, cats, series).no_legend()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.title_size(px: int) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Override title font size for this chart.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Big&quot;, labels, values).title_size(24)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">6</span><h3>Typography</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.font(name: str) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Override the font family for this chart.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Roboto&quot;, labels, values).font(&quot;Roboto&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.set_font_size(px: int) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Override base text size for all axis/legend labels.</div>
<pre><code class="language-python">chart = sp.radar(&quot;Skills&quot;, labels, values).set_font_size(11)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.text_font(family=None, size=None, color=None) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Style the value labels (text_auto family).</div>
<pre><code class="language-python">chart = sp.bar(&quot;Styled&quot;, labels, values).text_font(family=&quot;Courier&quot;, size=11, color=&quot;#333&quot;)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">7</span><h3>Value labels</h3></div>

<div class="cm-tip"><strong>Tip:</strong> works on <strong>every</strong> chart that emits <code>data-v</code> attributes — bars, lollipops, scatter, pie, treemap, line markers… Format strings follow d3 / Plotly conventions: <code>".2s"</code>, <code>".0f"</code>, <code>".1%"</code>.</div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.text_auto(format=True, position=None, angle=None, font_size=None, color=None) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Show value labels on every data point. <code>format=True</code> uses raw values; pass a d3 format string for fancy formatting.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Raw&quot;, labels, values).text_auto(True)
chart = sp.pie(&quot;Pct&quot;, labels, values).text_auto(&quot;.1%&quot;, position=&quot;outside&quot;)
chart = sp.bar(&quot;SI&quot;, labels, values).text_auto(&quot;.2s&quot;, angle=45, font_size=13)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.text_position(position: str) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Override label placement: <code>"auto"</code>, <code>"inside"</code>, <code>"outside"</code>.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Out&quot;, labels, values).text_position(&quot;outside&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.text_angle(degrees: int) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Rotate value labels by N degrees.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Rot&quot;, labels, values).text_angle(90)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.uniform_text(min_size=8, mode=&quot;hide&quot;) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Force consistent label sizing. <code>mode="hide"</code> drops labels smaller than <code>min_size</code>; <code>"show"</code> allows overflow.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Uniform&quot;, labels, values).uniform_text(min_size=10, mode=&quot;hide&quot;)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">8</span><h3>Bar styling</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.corner_radius_bars(radius) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Round bar corners. Accepts an integer (pixels) or a string like <code>"20%"</code> of the bar width.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Modern&quot;, labels, values).corner_radius_bars(8)
chart = sp.bar(&quot;Pct&quot;, labels, values).corner_radius_bars(&quot;15%&quot;)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">9</span><h3>Animation & interactivity</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.animate(duration=300) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Fade-in animation. Pass duration in ms.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Fade&quot;, labels, values).animate(500)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.crosshair() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Crosshair lines follow the mouse.</div>
<pre><code class="language-python">chart = sp.scatter(&quot;XY&quot;, x, y).crosshair()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.zoom() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Mouse-wheel zoom and pan; double-click resets.</div>
<pre><code class="language-python">chart = sp.line(&quot;Big&quot;, x, y).zoom()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.no_hover() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Disable all hover effects and tooltips.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Static&quot;, labels, values).no_hover()</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">10</span><h3>Responsive & layout</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.responsive() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Auto-resize to the parent container width.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Fluid&quot;, labels, values).responsive()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.border_radius(px: int) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Set the container border radius.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Round&quot;, labels, values).border_radius(12)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.set_margin(px: int) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Set the container padding.</div>
<pre><code class="language-python">chart = sp.scatter(&quot;Pad&quot;, x, y).set_margin(20)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.set_opacity(value: float) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Set element opacity (0.0–1.0).</div>
<pre><code class="language-python">chart = sp.bar(&quot;Faded&quot;, labels, values).set_opacity(0.7)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.scale(factor: float) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Scale chart by multiplier (1.0 = normal).</div>
<pre><code class="language-python">chart = sp.pie(&quot;Big&quot;, labels, values).scale(1.5)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">11</span><h3>Export & download</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.save(path: str)</code><span class="cm-tag cm-tag-export">export</span></div>
<div class="cm-desc">Write chart HTML to disk.</div>
<pre><code class="language-python">chart.save(&quot;chart.html&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.show()</code><span class="cm-tag cm-tag-export">export</span></div>
<div class="cm-desc">Display chart in Jupyter via IPython.display.</div>
<pre><code class="language-python">sp.scatter(&quot;Data&quot;, x, y).show()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.to_svg() -&gt; str</code><span class="cm-tag cm-tag-export">export</span></div>
<div class="cm-desc">Return the inner <code>&lt;svg&gt;</code> element as a string.</div>
<pre><code class="language-python">svg = chart.to_svg()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.export_svg(path: str)</code><span class="cm-tag cm-tag-export">export</span></div>
<div class="cm-desc">Save the SVG portion to disk.</div>
<pre><code class="language-python">chart.export_svg(&quot;chart.svg&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.export_png(path: str, scale=2.0)</code><span class="cm-tag cm-tag-export">export</span></div>
<div class="cm-desc">Export PNG (requires <code>cairosvg</code>).</div>
<pre><code class="language-python">chart.export_png(&quot;chart.png&quot;, scale=1.5)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.export_button() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Add a download button on the chart.</div>
<pre><code class="language-python">chart = sp.pie(&quot;Share&quot;, labels, values).export_button()</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">12</span><h3>Custom CSS / JS</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.inject_css(css: str) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Inject extra CSS into the chart's <code>&lt;head&gt;</code>.</div>
<pre><code class="language-python">chart = sp.bar(&quot;X&quot;, labels, values).inject_css(&quot;.sp-ttl{color:#22c55e}&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.inject_js(js: str) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Inject extra JavaScript at the end of the body.</div>
<pre><code class="language-python">chart = sp.bar(&quot;X&quot;, labels, values).inject_js(&quot;console.log(&#x27;loaded&#x27;)&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.show_labels(position=&quot;bottom&quot;, labels=None, colors=None) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Render a clickable text label overlay (alternative legend).</div>
<pre><code class="language-python">chart = sp.bar(&quot;Q&quot;, labels, values).show_labels(
    &quot;top&quot;, labels=[&quot;Series A&quot;, &quot;Series B&quot;], colors=[&quot;#6366F1&quot;, &quot;#FB7185&quot;]
)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">13</span><h3>Accessibility & CSP</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.a11y(title=&quot;&quot;, desc=&quot;&quot;) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Inject ARIA <code>title</code> and <code>desc</code> for screen readers.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Sales&quot;, labels, values).a11y(title=&quot;Q1–Q4 revenue&quot;, desc=&quot;Bar chart showing quarterly revenue trends&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.csp_safe() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Move inline scripts to <code>data-*</code> attributes for strict Content-Security-Policy environments.</div>
<pre><code class="language-python">chart = sp.bar(&quot;CSP&quot;, labels, values).csp_safe()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.downsample(n=2000, method=&quot;lttb&quot;) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Reduce data points using LTTB (Largest-Triangle-Three-Buckets) for very large series.</div>
<pre><code class="language-python">chart = sp.line(&quot;Big&quot;, xs, ys).downsample(n=1000)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">14</span><h3>Diff & introspection</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.diff(other: Chart) -&gt; str</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Compare two charts and return a JSON diff describing structural and stylistic differences.</div>
<pre><code class="language-python">a = sp.bar(&quot;v1&quot;, labels, vals1)
b = sp.bar(&quot;v2&quot;, labels, vals2)
print(a.diff(b))</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.doc() -&gt; str</code><span class="cm-tag cm-tag-prop">property</span></div>
<div class="cm-desc">Return the full inline documentation for the chart type.</div>
<pre><code class="language-python">print(chart.doc())</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">15</span><h3>Magic properties</h3></div>

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
<div class="cm-section"><span class="cm-sn">16</span><h3>Annotations (cross-chart)</h3><p>SVG overlays inherited by every plot</p></div>

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

<div class="cm-section"><span class="cm-sn">17</span><h3>Composers (Grid + Slideshow)</h3><p>Group charts into stories</p></div>

<div class="cm-card cm-new">
<div class="cm-name"><code class="cm-fn">sp.build_grid(charts, *, cols=2, gap=16, bg=None, cell_height=520) -&gt; Chart</code><span class="cm-tag cm-tag-new">new</span></div>
<div class="cm-desc">Compose any number of pre-built charts into a responsive CSS-grid layout, each chart hosted in its own iframe.</div>
<pre><code class="language-python">bar = sp.build_bar_chart("Q-Sales", labels=["Q1","Q2","Q3","Q4"], values=[120,180,150,210])
line = sp.build_line_chart("Trend", labels=months, values=sales)
pie = sp.build_pie_chart("Mix", labels=["A","B","C"], values=[40,35,25])
grid = sp.build_grid([bar, line, pie], cols=2, gap=14, cell_height=320)
grid.show()</code></pre>
</div>
<div class="cm-card cm-new">
<div class="cm-name"><code class="cm-fn">sp.build_slideshow(charts, interval_ms=2500, title='', width=900, height=520) -&gt; Chart</code><span class="cm-tag cm-tag-new">new</span></div>
<div class="cm-desc">Build a navigable HTML carousel with previous / next buttons and an auto-advance progress bar - perfect for telling a story without a slide deck.</div>
<pre><code class="language-python">slides = [sp.build_bar_chart(f"Slide {i+1}", labels=["A","B"], values=[i, i+1]) for i in range(4)]
show = sp.build_slideshow(slides, interval_ms=2200, title="Quarterly Story")
show.show()</code></pre>
</div>
</div>
<div class="lang-fr">

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
.cm-card .cm-desc{margin:6px 0 10px;color:#cbd5e1;font-size:13.5px;line-height:1.6}
.cm-card pre{margin:0;border-radius:8px;background:#06080f;border:1px solid #131a2a;padding:12px 14px;overflow-x:auto}
.cm-card pre code{font-size:12.5px;line-height:1.55;color:#e2e8f0;background:none;padding:0}

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
<div class="cm-hero">
<h2>Méthodes & configuration globale</h2>
<p>Tout objet <code>Chart</code> renvoyé par SeraPlot expose la même API fluide. Définis les valeurs par défaut une fois avec <code>sp.config()</code>, ajuste chart par chart avec des méthodes chaînées. Toutes les méthodes retournent un nouveau <code>Chart</code> : le chaînage est toujours sûr.</p>
<div class="cm-pills"><span class="cm-pill">API fluide</span><span class="cm-pill">Global + override par chart</span><span class="cm-pill">60+ types de graphiques</span><span class="cm-pill">Étiquettes Plotly</span></div>
</div>

<div class="cm-toc">
<a href="#configuration-globale"><span class="cm-ic">1</span><span>Configuration globale</span></a>
<a href="#themes"><span class="cm-ic">2</span><span>Thèmes</span></a>
<a href="#arriere-plan-cadre"><span class="cm-ic">3</span><span>Arrière-plan & cadre</span></a>
<a href="#grille-axes"><span class="cm-ic">4</span><span>Grille & axes</span></a>
<a href="#titre-legende"><span class="cm-ic">5</span><span>Titre & légende</span></a>
<a href="#typographie"><span class="cm-ic">6</span><span>Typographie</span></a>
<a href="#etiquettes-de-valeur"><span class="cm-ic">7</span><span>Étiquettes de valeur</span></a>
<a href="#style-des-barres"><span class="cm-ic">8</span><span>Style des barres</span></a>
<a href="#animation-interactivite"><span class="cm-ic">9</span><span>Animation & interactivité</span></a>
<a href="#reactif-mise-en-page"><span class="cm-ic">10</span><span>Réactif & mise en page</span></a>
<a href="#export-telechargement"><span class="cm-ic">11</span><span>Export & téléchargement</span></a>
<a href="#css-/-js-personnalises"><span class="cm-ic">12</span><span>CSS / JS personnalisés</span></a>
<a href="#accessibilite-csp"><span class="cm-ic">13</span><span>Accessibilité & CSP</span></a>
<a href="#diff-introspection"><span class="cm-ic">14</span><span>Diff & introspection</span></a>
<a href="#proprietes-magiques"><span class="cm-ic">15</span><span>Propriétés magiques</span></a>
</div>
<div class="cm-section"><span class="cm-sn">1</span><h3>Configuration globale</h3></div>

Définis les défauts une fois. <strong>Tous les graphiques</strong> créés ensuite héritent automatiquement de cette configuration.

<table class="cm-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Effet</th></tr></thead>
<tbody>
<tr><td><code>font</code></td><td>str</td><td>system</td><td>Font family for every text element</td></tr>
<tr><td><code>font_size</code></td><td>int</td><td>12</td><td>Base font size in px</td></tr>
<tr><td><code>title_size</code></td><td>int</td><td>16</td><td>Title font size in px</td></tr>
<tr><td><code>border_radius</code></td><td>int</td><td>0</td><td>Container border radius (px)</td></tr>
<tr><td><code>opacity</code></td><td>float</td><td>1.0</td><td>Element opacity 0.0–1.0</td></tr>
<tr><td><code>responsive</code></td><td>bool</td><td>False</td><td>Auto-resize to container width</td></tr>
<tr><td><code>animation</code></td><td>bool</td><td>False</td><td>Fade-in animation on chart load</td></tr>
<tr><td><code>animation_duration</code></td><td>int</td><td>300</td><td>Animation duration in ms</td></tr>
<tr><td><code>crosshair</code></td><td>bool</td><td>False</td><td>Crosshair lines follow mouse hover</td></tr>
<tr><td><code>zoom</code></td><td>bool</td><td>False</td><td>Mouse wheel zoom + pan (double-click resets)</td></tr>
<tr><td><code>tooltip</code></td><td>str</td><td>—</td><td>Tooltip mode</td></tr>
<tr><td><code>locale</code></td><td>str</td><td>—</td><td>Number formatting locale</td></tr>
<tr><td><code>thousands_sep</code></td><td>str</td><td>—</td><td>Thousands separator character</td></tr>
<tr><td><code>margin</code></td><td>int</td><td>0</td><td>Container padding (px)</td></tr>
<tr><td><code>export_button</code></td><td>bool</td><td>False</td><td>Show download button on each chart</td></tr>
<tr><td><code>palette</code></td><td>list[int]</td><td>—</td><td>Color palette as hex ints (e.g. <code>[0x6366F1,0xFB7185]</code>)</td></tr>
<tr><td><code>background</code></td><td>str</td><td>—</td><td>Background color (any CSS color)</td></tr>
<tr><td><code>gridlines</code></td><td>bool</td><td>False</td><td>Show grid lines in chart</td></tr>
<tr><td><code>text_auto</code></td><td>bool / str</td><td>False</td><td>Display values on bars/markers (<code>True</code> raw, or d3 format like <code>".2s"</code>)</td></tr>
<tr><td><code>text_position</code></td><td>str</td><td>"auto"</td><td><code>"auto"</code> / <code>"inside"</code> / <code>"outside"</code></td></tr>
<tr><td><code>text_angle</code></td><td>int</td><td>0</td><td>Value label rotation (deg)</td></tr>
<tr><td><code>text_font_size</code></td><td>int</td><td>12</td><td>Max font size for value labels (px)</td></tr>
<tr><td><code>text_font_color</code></td><td>str</td><td>—</td><td>Value label color</td></tr>
<tr><td><code>uniform_text_min_size</code></td><td>int</td><td>0</td><td>Minimum px before label is hidden</td></tr>
<tr><td><code>uniform_text_mode</code></td><td>str</td><td>"hide"</td><td><code>"hide"</code> small labels or <code>"show"</code> overflow</td></tr>
<tr><td><code>bar_corner_radius</code></td><td>int / str</td><td>0</td><td>Bar corner radius in px or <code>"20%"</code> of bar width</td></tr>
</tbody>
</table>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">sp.config(**kwargs)</code><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Applique des défauts persistants à tous les charts suivants.</div>
<pre><code class="language-python">import seraplot as sp

sp.config(
    font=&quot;Inter&quot;, font_size=12, title_size=16,
    crosshair=True, zoom=True, animation=True,
    palette=[0x6366F1, 0xFB7185, 0x10B981],
    background=&quot;#fafafa&quot;, border_radius=8, margin=16,
)
chart = sp.bar(&quot;Sales&quot;, [&quot;Q1&quot;,&quot;Q2&quot;,&quot;Q3&quot;], [100,150,120])</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">sp.reset_config()</code><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Remet chaque paramètre global aux valeurs d'usine.</div>
<pre><code class="language-python">sp.reset_config()
chart = sp.bar(&quot;Clean&quot;, labels, values)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">2</span><h3>Thèmes</h3></div>

Préréglages combinant palette, fond et grille.

<table class="cm-table"><thead><tr><th>Theme</th><th>Mood</th></tr></thead><tbody><tr><td><code>"dark"</code></td><td>Tableau de bord sombre haut contraste</td></tr><tr><td><code>"light"</code></td><td>Fond clair pastel doux</td></tr><tr><td><code>"scientific"</code></td><td>Style publication monochrome</td></tr><tr><td><code>"apple"</code></td><td>Palette inspirée iOS, glassy</td></tr><tr><td><code>"notion"</code></td><td>Neutres calmes type Notion</td></tr><tr><td><code>"minimal"</code></td><td>Lignes pures, sans décoration</td></tr><tr><td><code>"neon"</code></td><td>Néons cyberpunk vibrants</td></tr></tbody></table>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">sp.theme(name)</code><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Applique un thème prédéfini. Cumulable avec <code>sp.config()</code>.</div>
<pre><code class="language-python">sp.theme(&quot;dark&quot;)
chart = sp.bar(&quot;Modern&quot;, labels, values)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">sp.themes() -&gt; list[str]</code><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Liste tous les thèmes disponibles.</div>
<pre><code class="language-python">available = sp.themes()
print(available)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">sp.reset_theme()</code><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Revient à aucun thème (palette par défaut, sans fond, sans grille).</div>
<pre><code class="language-python">sp.reset_theme()</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">3</span><h3>Arrière-plan & cadre</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.set_bg(color=None) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Définit le fond du conteneur HTML. Toute couleur CSS ou <code>None</code> pour transparent.</div>
<pre><code class="language-python">chart = sp.line(&quot;Trend&quot;, x, y).set_bg(&quot;#0f172a&quot;)
chart = sp.pie(&quot;Share&quot;, labels, values).set_bg(None)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.set_frame(color=None) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Définit le fond du canvas SVG, indépendant du conteneur HTML.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Title&quot;, labels, values).set_frame(&quot;transparent&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">sp.set_global_background(color)</code><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Applique un fond à <strong>tous</strong> les charts créés ensuite. Utile pour les notebooks.</div>
<pre><code class="language-python">sp.set_global_background(&quot;#0f172a&quot;)
chart1 = sp.bar(...)
chart2 = sp.scatter(...)
sp.reset_global_background()</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">4</span><h3>Grille & axes</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.show_grid() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Force la grille active (override config).</div>
<pre><code class="language-python">chart = sp.bar(&quot;Data&quot;, labels, values).show_grid()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.hide_grid() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Force la grille à OFF.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Data&quot;, labels, values, gridlines=True).hide_grid()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.no_x_axis() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Supprime l'axe X (ligne, ticks, labels).</div>
<pre><code class="language-python">chart = sp.bar(&quot;Y only&quot;, labels, values).no_x_axis()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.no_y_axis() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Supprime l'axe Y (ligne, ticks, labels).</div>
<pre><code class="language-python">chart = sp.bar(&quot;X only&quot;, labels, values).no_y_axis()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.no_axes() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Supprime les deux axes en même temps.</div>
<pre><code class="language-python">chart = sp.scatter(&quot;Clean&quot;, x, y).no_axes()</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">5</span><h3>Titre & légende</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.no_title() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Masque le titre.</div>
<pre><code class="language-python">chart = sp.pie(&quot;Internal&quot;, labels, values).no_title()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.no_legend() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Masque la légende (charts multi-séries).</div>
<pre><code class="language-python">chart = sp.grouped_bar(&quot;Q1&quot;, cats, series).no_legend()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.title_size(px: int) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Override la taille du titre pour ce chart.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Big&quot;, labels, values).title_size(24)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">6</span><h3>Typographie</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.font(name: str) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Override la police pour ce chart.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Roboto&quot;, labels, values).font(&quot;Roboto&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.set_font_size(px: int) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Override la taille de base de tous les labels.</div>
<pre><code class="language-python">chart = sp.radar(&quot;Skills&quot;, labels, values).set_font_size(11)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.text_font(family=None, size=None, color=None) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Style les étiquettes de valeur (famille text_auto).</div>
<pre><code class="language-python">chart = sp.bar(&quot;Styled&quot;, labels, values).text_font(family=&quot;Courier&quot;, size=11, color=&quot;#333&quot;)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">7</span><h3>Étiquettes de valeur</h3></div>

<div class="cm-tip"><strong>Astuce :</strong> fonctionne sur <strong>tous</strong> les charts qui exposent <code>data-v</code> — barres, lollipops, scatter, pie, treemap, marqueurs de ligne… Les formats suivent d3 / Plotly : <code>".2s"</code>, <code>".0f"</code>, <code>".1%"</code>.</div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.text_auto(format=True, position=None, angle=None, font_size=None, color=None) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Affiche les valeurs sur chaque point. <code>format=True</code> = brut ; passe un format d3 pour personnaliser.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Raw&quot;, labels, values).text_auto(True)
chart = sp.pie(&quot;Pct&quot;, labels, values).text_auto(&quot;.1%&quot;, position=&quot;outside&quot;)
chart = sp.bar(&quot;SI&quot;, labels, values).text_auto(&quot;.2s&quot;, angle=45, font_size=13)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.text_position(position: str) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Override la position : <code>"auto"</code>, <code>"inside"</code>, <code>"outside"</code>.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Out&quot;, labels, values).text_position(&quot;outside&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.text_angle(degrees: int) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Tourne les étiquettes de N degrés.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Rot&quot;, labels, values).text_angle(90)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.uniform_text(min_size=8, mode=&quot;hide&quot;) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Force une taille uniforme. <code>mode="hide"</code> masque les labels &lt; <code>min_size</code> ; <code>"show"</code> autorise l'overflow.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Uniform&quot;, labels, values).uniform_text(min_size=10, mode=&quot;hide&quot;)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">8</span><h3>Style des barres</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.corner_radius_bars(radius) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Arrondit les coins. Entier (pixels) ou chaîne type <code>"20%"</code> de la largeur de barre.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Modern&quot;, labels, values).corner_radius_bars(8)
chart = sp.bar(&quot;Pct&quot;, labels, values).corner_radius_bars(&quot;15%&quot;)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">9</span><h3>Animation & interactivité</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.animate(duration=300) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Animation fade-in. Durée en ms.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Fade&quot;, labels, values).animate(500)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.crosshair() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Lignes crosshair qui suivent la souris.</div>
<pre><code class="language-python">chart = sp.scatter(&quot;XY&quot;, x, y).crosshair()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.zoom() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Zoom molette + pan ; double-clic = reset.</div>
<pre><code class="language-python">chart = sp.line(&quot;Big&quot;, x, y).zoom()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.no_hover() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Désactive les effets hover et les tooltips.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Static&quot;, labels, values).no_hover()</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">10</span><h3>Réactif & mise en page</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.responsive() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Redimensionnement auto sur la largeur parent.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Fluid&quot;, labels, values).responsive()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.border_radius(px: int) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Définit le rayon des coins du conteneur.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Round&quot;, labels, values).border_radius(12)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.set_margin(px: int) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Définit le padding du conteneur.</div>
<pre><code class="language-python">chart = sp.scatter(&quot;Pad&quot;, x, y).set_margin(20)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.set_opacity(value: float) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Opacité des éléments (0.0–1.0).</div>
<pre><code class="language-python">chart = sp.bar(&quot;Faded&quot;, labels, values).set_opacity(0.7)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.scale(factor: float) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Mise à l'échelle (1.0 = normal).</div>
<pre><code class="language-python">chart = sp.pie(&quot;Big&quot;, labels, values).scale(1.5)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">11</span><h3>Export & téléchargement</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.save(path: str)</code><span class="cm-tag cm-tag-export">export</span></div>
<div class="cm-desc">Écrit le HTML du chart sur disque.</div>
<pre><code class="language-python">chart.save(&quot;chart.html&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.show()</code><span class="cm-tag cm-tag-export">export</span></div>
<div class="cm-desc">Affiche dans Jupyter via IPython.display.</div>
<pre><code class="language-python">sp.scatter(&quot;Data&quot;, x, y).show()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.to_svg() -&gt; str</code><span class="cm-tag cm-tag-export">export</span></div>
<div class="cm-desc">Renvoie l'élément <code>&lt;svg&gt;</code> sous forme de chaîne.</div>
<pre><code class="language-python">svg = chart.to_svg()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.export_svg(path: str)</code><span class="cm-tag cm-tag-export">export</span></div>
<div class="cm-desc">Sauve la partie SVG sur disque.</div>
<pre><code class="language-python">chart.export_svg(&quot;chart.svg&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.export_png(path: str, scale=2.0)</code><span class="cm-tag cm-tag-export">export</span></div>
<div class="cm-desc">Export PNG (nécessite <code>cairosvg</code>).</div>
<pre><code class="language-python">chart.export_png(&quot;chart.png&quot;, scale=1.5)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.export_button() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Ajoute un bouton de téléchargement.</div>
<pre><code class="language-python">chart = sp.pie(&quot;Share&quot;, labels, values).export_button()</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">12</span><h3>CSS / JS personnalisés</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.inject_css(css: str) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Injecte du CSS dans le <code>&lt;head&gt;</code> du chart.</div>
<pre><code class="language-python">chart = sp.bar(&quot;X&quot;, labels, values).inject_css(&quot;.sp-ttl{color:#22c55e}&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.inject_js(js: str) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Injecte du JavaScript en fin de body.</div>
<pre><code class="language-python">chart = sp.bar(&quot;X&quot;, labels, values).inject_js(&quot;console.log(&#x27;loaded&#x27;)&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.show_labels(position=&quot;bottom&quot;, labels=None, colors=None) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Ajoute des étiquettes textuelles cliquables (légende alternative).</div>
<pre><code class="language-python">chart = sp.bar(&quot;Q&quot;, labels, values).show_labels(
    &quot;top&quot;, labels=[&quot;Series A&quot;, &quot;Series B&quot;], colors=[&quot;#6366F1&quot;, &quot;#FB7185&quot;]
)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">13</span><h3>Accessibilité & CSP</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.a11y(title=&quot;&quot;, desc=&quot;&quot;) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Ajoute <code>title</code> et <code>desc</code> ARIA pour les lecteurs d'écran.</div>
<pre><code class="language-python">chart = sp.bar(&quot;Sales&quot;, labels, values).a11y(title=&quot;Q1–Q4 revenue&quot;, desc=&quot;Bar chart showing quarterly revenue trends&quot;)</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.csp_safe() -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Déplace les scripts inline en attributs <code>data-*</code> pour les environnements CSP strict.</div>
<pre><code class="language-python">chart = sp.bar(&quot;CSP&quot;, labels, values).csp_safe()</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.downsample(n=2000, method=&quot;lttb&quot;) -&gt; Chart</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Réduit le nombre de points avec LTTB (Largest-Triangle-Three-Buckets) pour les grosses séries.</div>
<pre><code class="language-python">chart = sp.line(&quot;Big&quot;, xs, ys).downsample(n=1000)</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">14</span><h3>Diff & introspection</h3></div>

<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.diff(other: Chart) -&gt; str</code><span class="cm-tag cm-tag-chain">chainable</span></div>
<div class="cm-desc">Compare deux charts et renvoie un diff JSON décrivant les différences.</div>
<pre><code class="language-python">a = sp.bar(&quot;v1&quot;, labels, vals1)
b = sp.bar(&quot;v2&quot;, labels, vals2)
print(a.diff(b))</code></pre>
</div>
<div class="cm-card">
<div class="cm-name"><code class="cm-fn">.doc() -&gt; str</code><span class="cm-tag cm-tag-prop">property</span></div>
<div class="cm-desc">Renvoie la documentation inline complète du type de chart.</div>
<pre><code class="language-python">print(chart.doc())</code></pre>
</div>
<div class="cm-section"><span class="cm-sn">15</span><h3>Propriétés magiques</h3></div>

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

<div class="cm-section"><span class="cm-sn">16</span><h3>Annotations (transversales)</h3><p>Surcouches SVG heritees par tous les plots</p></div>

<div class="cm-card cm-new">
<div class="cm-name"><code class="cm-fn">annotations=[...] (kwarg)</code><span class="cm-tag cm-tag-new">nouveau</span><span class="cm-tag cm-tag-global">global</span></div>
<div class="cm-desc">Passe une liste de dictionnaires d annotations a <strong>n importe quel</strong> builder. Coordonnees fractionnaires par defaut (<code>0.0 - 1.0</code>) ; mets <code>"frac": false</code> pour utiliser des pixels. Valeurs supportees pour <code>kind</code> : <code>"hline"</code>, <code>"vline"</code>, <code>"line"</code>, <code>"arrow"</code>, <code>"rect"</code>, <code>"text"</code>.</div>
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
<div class="cm-name"><code class="cm-fn">Champs d annotation</code></div>
<div class="cm-desc">Toutes les annotations partagent la meme structure :</div>
<table class="cm-table">
<thead><tr><th>Champ</th><th>Type</th><th>Defaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>kind</code></td><td>str</td><td>requis</td><td><code>hline</code> / <code>vline</code> / <code>line</code> / <code>arrow</code> / <code>rect</code> / <code>text</code></td></tr>
<tr><td><code>x</code>, <code>y</code></td><td>float</td><td>0.5</td><td>Point d ancrage (frac canvas si <code>frac=True</code>, pixels sinon)</td></tr>
<tr><td><code>x2</code>, <code>y2</code></td><td>float</td><td>1.0</td><td>Point final pour line / arrow / rect</td></tr>
<tr><td><code>color</code></td><td>str</td><td><code>"#ef4444"</code></td><td>Couleur de trait / texte (CSS)</td></tr>
<tr><td><code>fill</code></td><td>str</td><td><code>"none"</code></td><td>Couleur de remplissage (rect uniquement)</td></tr>
<tr><td><code>stroke_width</code></td><td>float</td><td>1.5</td><td>Epaisseur du trait</td></tr>
<tr><td><code>dash</code></td><td>str</td><td><code>""</code></td><td>SVG dash array, ex. <code>"6 4"</code></td></tr>
<tr><td><code>opacity</code></td><td>float</td><td>1.0</td><td>0.0 - 1.0</td></tr>
<tr><td><code>text</code></td><td>str</td><td>None</td><td>Etiquette optionnelle a cote du primitif</td></tr>
<tr><td><code>font_size</code></td><td>float</td><td>11.0</td><td>Taille de la police de l etiquette</td></tr>
<tr><td><code>frac</code></td><td>bool</td><td>true</td><td>Espace de coordonnees : fractionnaire (0-1) ou pixel</td></tr>
</tbody>
</table>
</div>
<div class="cm-tip"><strong>Astuce :</strong> les annotations s appliquent uniformement a tous les charts 2D et 3D via le hook <code>apply_annotations()</code> - aucun branchement par chart requis.</div>

<div class="cm-section"><span class="cm-sn">17</span><h3>Composers (Grid + Slideshow)</h3><p>Regroupe des charts en histoires</p></div>

<div class="cm-card cm-new">
<div class="cm-name"><code class="cm-fn">sp.build_grid(charts, *, cols=2, gap=16, bg=None, cell_height=520) -&gt; Chart</code><span class="cm-tag cm-tag-new">nouveau</span></div>
<div class="cm-desc">Empile N charts deja construits dans une grille CSS responsive, chaque chart isole dans son iframe.</div>
<pre><code class="language-python">bar = sp.build_bar_chart("Ventes-Q", labels=["Q1","Q2","Q3","Q4"], values=[120,180,150,210])
line = sp.build_line_chart("Tendance", labels=mois, values=ventes)
pie = sp.build_pie_chart("Mix", labels=["A","B","C"], values=[40,35,25])
grid = sp.build_grid([bar, line, pie], cols=2, gap=14, cell_height=320)
grid.show()</code></pre>
</div>
<div class="cm-card cm-new">
<div class="cm-name"><code class="cm-fn">sp.build_slideshow(charts, interval_ms=2500, title='', width=900, height=520) -&gt; Chart</code><span class="cm-tag cm-tag-new">nouveau</span></div>
<div class="cm-desc">Construit un carrousel HTML navigable avec boutons precedent/suivant et barre de progression - parfait pour raconter une histoire sans presentation externe.</div>
<pre><code class="language-python">slides = [sp.build_bar_chart(f"Slide {i+1}", labels=["A","B"], values=[i, i+1]) for i in range(4)]
show = sp.build_slideshow(slides, interval_ms=2200, title="Histoire trimestrielle")
show.show()</code></pre>
</div>

</div>
