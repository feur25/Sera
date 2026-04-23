# Chart Methods

<div class="lang-en">

These methods are available on every `Chart` object returned by any SeraPlot
function. They all return a new `Chart`, so they can be chained freely.

---

## Global config (automatic inheritance)

### `sp.config(**kwargs)`

Set once, **every chart** created after inherits the configuration automatically. No per-chart override needed.

| Parameter | Type | Effect |
|-----------|------|--------|
| `font` | str | Font family for all text (e.g., `"Inter"`, `"Roboto"`) |
| `font_size` | int | Base font size in px |
| `title_size` | int | Title font size in px |
| `crosshair` | bool | Crosshair lines follow mouse hover |
| `zoom` | bool | Mouse wheel zoom + pan (double-click resets) |
| `animation` | bool | Fade-in animation on chart elements |
| `animation_duration` | int | Duration (ms), default 300 |
| `export_button` | bool | Download button on each chart |
| `responsive` | bool | Auto-resize to container width |
| `border_radius` | int | Container border radius (px) |
| `margin` | int | Container padding (px) |
| `opacity` | float | Element opacity `0.0`–`1.0` |
| `background` | str | Background color (any CSS color) |
| `gridlines` | bool | Show grid lines in chart |
| `palette` | list[int] | Color palette as hex ints (e.g., `[0x6366F1, 0xFB7185]`) |
| `locale` | str | Number formatting locale |
| `thousands_sep` | str | Thousands separator char |
| `tooltip` | str | Tooltip mode |

```python
import seraplot as sp

sp.config(
    font="Inter",
    font_size=14,
    title_size=22,
    crosshair=True,
    zoom=True,
    animation=True,
    animation_duration=500,
    export_button=True,
    responsive=True,
    border_radius=12,
    margin=16,
    opacity=0.85,
    background="#0f172a",
    gridlines=True,
    palette=[0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24],
)

chart1 = sp.bar("Revenue", ["Q1", "Q2", "Q3"], [120, 180, 140])     # inherits ALL config
chart2 = sp.line("Trend", ["Jan", "Feb", "Mar"], [100, 110, 105])   # same config
chart3 = sp.scatter("Correlation", [1, 2, 3], [10, 20, 30])         # same config
```

---

### `sp.reset_config()`

Reset all global config to defaults (no background, no crosshair, no zoom, etc.).

```python
sp.reset_config()
chart = sp.bar("Clean", labels, values)  # back to defaults
```

---

## Per-chart override (method chaining)

Override global config for individual charts:

| Method | Effect |
|--------|--------|
| `.font("Inter")` | Override font family |
| `.title_size(22)` | Override title size |
| `.set_font_size(14)` | Override base font size |
| `.crosshair()` | Enable/add crosshair on this chart |
| `.zoom()` | Enable/add zoom on this chart |
| `.animate(300)` | Add animation (ms) |
| `.export_button()` | Add download button |
| `.responsive()` | Make responsive |
| `.border_radius(12)` | Set border radius |
| `.set_opacity(0.85)` | Set element opacity |
| `.set_margin(16)` | Set padding |
| `.set_bg("#color")` | Override background |

```python
import seraplot as sp

sp.config(font="Inter", background="#0f172a", gridlines=True)

chart1 = sp.bar("Default", labels, values)           # uses config
chart2 = sp.bar("Override", labels, values).font("Roboto").border_radius(20)  # different font + radius
chart3 = sp.line("Clean", dates, values).zoom()       # adds zoom on top of config
```

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155}
.sp-tb{padding:9px 22px;border:none;background:none;color:#64748b;cursor:pointer;font-size:13px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){hljs.highlightElement(c)})}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc code').forEach(function(c){hljs.highlightElement(c)})});
</script>

---

## Background and frame

### `set_bg(color=None)`

Sets the background color of the full HTML wrapper.

```python
chart = sp.build_bar_chart("Sales", labels, values).set_bg("#0f172a")
chart = sp.build_scatter_chart("Data", x, y).set_bg("white")
chart = sp.build_pie_chart("Share", labels, values).set_bg(None)  # transparent
```

Accepts any CSS color string: `"#rrggbb"`, `"rgb(r,g,b)"`, named colors, or
`None`/`"transparent"`.

---

### `set_frame(color=None)`

Sets the SVG canvas background independently of the HTML wrapper. Useful when
embedding the chart inside a page with its own background.

```python
chart = sp.build_bar_chart("Title", labels, values).set_frame("transparent")
```

---

### `set_global_background(color)`

Module-level functions that apply a background to **all** charts created after
the call — useful for notebook sessions with a consistent theme.

<div class="sp-tabs" id="cm-gbg">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('cm-gbg','cm-gbg-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('cm-gbg','cm-gbg-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('cm-gbg','cm-gbg-ts',this)">TypeScript</button>
</div>
<div id="cm-gbg-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">sp.set_global_background("#0f172a")   # all subsequent charts use this bg
chart1 = sp.build_bar_chart(...)
chart2 = sp.build_scatter_chart(...)
sp.reset_global_background()          # restore per-chart default</code></pre></div>
<div id="cm-gbg-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">sp.set_global_background("#0f172a");  // all subsequent charts use this bg
const chart1 = sp.build_bar_chart(...);
const chart2 = sp.build_scatter_chart(...);
sp.reset_global_background();         // restore per-chart default</code></pre></div>
<div id="cm-gbg-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">sp.set_global_background("#0f172a");  // all subsequent charts use this bg
const chart1 = sp.build_bar_chart(...);
const chart2 = sp.build_scatter_chart(...);
sp.reset_global_background();         // restore per-chart default</code></pre></div>
</div>

---

## Grid and axes

### `show_grid()`

Force gridlines on regardless of the `gridlines` parameter used at chart creation.

```python
chart = sp.build_histogram("Ages", values).show_grid()
```

---

### `hide_grid()`

Force gridlines off regardless of the `gridlines` parameter used at chart creation.

```python
chart = sp.build_line_chart("Trend", labels, values, gridlines=True).hide_grid()
```

---

### `no_x_axis()`

Removes the X-axis lines, ticks, and labels.

```python
chart = sp.build_bar_chart("", labels, values).no_x_axis()   # keep Y only
```

---

### `no_y_axis()`

Removes the Y-axis lines, ticks, and labels.

```python
chart = sp.build_bar_chart("", labels, values).no_y_axis()   # keep X only
```

---

### `no_axes()`

Removes both axes at once.

```python
chart = sp.build_scatter_chart("", x, y).no_axes()           # remove both axes
```

---

## Labels and text

### `show_labels(position="bottom", labels=None, colors=None)`

Renders a text label on each chart element. `position` can be `"top"`,
`"bottom"`, `"left"`, or `"right"`.

```python
# Automatic labels derived from the chart data
chart = sp.build_bar_chart("Revenue", labels, values).show_labels(position="top")

# Custom label text per element
chart = sp.build_bar_chart("Revenue", labels, values).show_labels(
    position="top",
    labels=["$142k", "$98k", "$210k"],
    colors=["#22c55e", "#ef4444", "#22c55e"],
)
```

---

### `no_title()`

Removes the chart title from the rendered output.

```python
chart = sp.build_pie_chart("Internal label", labels, values).no_title()
```

---

### `no_legend()`

Removes the legend from the rendered output.

```python
chart = sp.build_grouped_bar("Q1", cats, series).no_legend()
```

---

### `set_font_size(px)`

Override all text sizes in the SVG with a single pixel value.

```python
chart = sp.build_radar_chart("Skills", labels, values).set_font_size(11)
```

---

## Size and scale

### `scale(factor)`

Scales the entire chart (SVG and canvas). Useful to produce thumbnail or
high-DPI variants.

```python
small = sp.build_bar_chart("Sales", labels, values).scale(0.5)
large = sp.build_bar_chart("Sales", labels, values).scale(2.0)
```

---

## Hover

### `no_hover()`

Disables the tooltip engine. All `data-idx` pointer events are removed. Useful
for static embeds where hover interaction is unwanted.

```python
chart = sp.build_scatter_chart("Distribution", x, y).no_hover()
```

---

## CSS and JavaScript injection

### `inject_css(css)`

Injects a `<style>` block into the HTML `<head>`. Gives complete access to the
SVG DOM — override any internal class, change colors, animations, fonts.

SeraPlot's internal CSS classes:

| Class| Target |
|-----------------|--------|
| `svg text` | All text in the chart |
| `.sp-gl` | Gridlines |
| `.sp-ax-x`, `.sp-ax-y` | Axis lines |
| `.sp-xt`, `.sp-yt` | Axis tick labels |
| `.sp-xl`, `.sp-yl` | Axis title labels |
| `g[data-legend]` | Legend group |
| `.sp-ttl` | Chart title |
| `[data-idx]` | Interactive data elements (hover targets) |
| `rect.sp-bg` | SVG background rect |

```python
chart = sp.build_bar_chart("Dark theme", labels, values).inject_css("""
    rect.sp-bg  { fill: #0f172a !important; }
    svg text    { fill: #e2e8f0 !important; }
    .sp-gl      { stroke: #1e293b !important; }
    [data-idx]  { opacity: 0.85; }
    [data-idx]:hover { filter: brightness(1.3); }
""")
```

---

### `inject_js(js)`

Injects a `<script>` block before `</body>`. The entire rendered SVG DOM is
accessible. Use it to add event listeners, animations, or third-party integrations.

```python
# Highlight every bar on load
chart = sp.build_bar_chart("Sales", labels, values).inject_js("""
    document.querySelectorAll('[data-idx]').forEach((el, i) => {
        setTimeout(() => el.style.opacity = '1', i * 50);
    });
""")

# Export SVG on button click
chart = sp.build_scatter_chart("Data", x, y).inject_js("""
    const btn = document.createElement('button');
    btn.textContent = 'Download SVG';
    btn.onclick = () => {
        const svg  = document.querySelector('svg').outerHTML;
        const a    = document.createElement('a');
        a.href     = 'data:image/svg+xml;charset=utf-8,' + encodeURIComponent(svg);
        a.download = 'chart.svg';
        a.click();
    };
    document.body.appendChild(btn);
""")
```

---

## Export

### `save(path)`

Writes the HTML to a file.

```python
chart = sp.build_pie_chart("Share", labels, values)
chart.save("report/pie.html")
```

---

### `to_svg()`

Extracts the raw SVG string from the HTML.

```python
svg_string = chart.to_svg()
```

---

### `export_svg(path)`

Writes only the SVG to a file (no HTML wrapper).

```python
chart.export_svg("chart.svg")
```

---

</div>

<div class="lang-fr">

Ces méthodes sont disponibles sur chaque objet `Chart` retourné par n'importe quelle
fonction SeraPlot. Elles retournent toutes un nouveau `Chart` — elles peuvent donc être
chaînées librement.

---

## Configuration globale (héritage automatique)

### `sp.config(**kwargs)`

Définie une fois, **chaque graphique** créé ensuite hérite automatiquement de la configuration. Aucune surcharge par graphique nécessaire.

| Paramètre | Type | Effet |
|-----------|------|-------|
| `font` | str | Police pour tout le texte (ex. `"Inter"`, `"Roboto"`) |
| `font_size` | int | Taille de police de base en px |
| `title_size` | int | Taille de police du titre en px |
| `crosshair` | bool | Réticule qui suit la souris au survol |
| `zoom` | bool | Zoom molette + déplacement (double-clic réinitialise) |
| `animation` | bool | Animation d'apparition sur les éléments |
| `animation_duration` | int | Durée (ms), défaut 300 |
| `export_button` | bool | Bouton de téléchargement sur chaque graphique |
| `responsive` | bool | Redimensionnement automatique à la largeur du conteneur |
| `border_radius` | int | Rayon des coins du conteneur (px) |
| `margin` | int | Marge intérieure du conteneur (px) |
| `opacity` | float | Opacité des éléments `0.0`–`1.0` |
| `background` | str | Couleur de fond (toute couleur CSS) |
| `gridlines` | bool | Afficher les lignes de grille |
| `palette` | list[int] | Palette de couleurs en entiers hex (ex. `[0x6366F1, 0xFB7185]`) |
| `locale` | str | Locale de formatage des nombres |
| `thousands_sep` | str | Caractère séparateur des milliers |
| `tooltip` | str | Mode des infobulles |

```python
import seraplot as sp

sp.config(
    font="Inter",
    font_size=14,
    title_size=22,
    crosshair=True,
    zoom=True,
    animation=True,
    animation_duration=500,
    export_button=True,
    responsive=True,
    border_radius=12,
    margin=16,
    opacity=0.85,
    background="#0f172a",
    gridlines=True,
    palette=[0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24],
)

chart1 = sp.bar("Chiffre d'affaires", ["T1", "T2", "T3"], [120, 180, 140])
chart2 = sp.line("Tendance", ["Jan", "Fév", "Mar"], [100, 110, 105])
chart3 = sp.scatter("Corrélation", [1, 2, 3], [10, 20, 30])
```

---

### `sp.reset_config()`

Remet toute la configuration globale aux valeurs par défaut (pas de fond, pas de réticule, pas de zoom, etc.).

```python
sp.reset_config()
chart = sp.bar("Propre", labels, values)
```

---

## Surcharge par graphique (chaînage de méthodes)

Surcharger la configuration globale pour des graphiques individuels :

| Méthode | Effet |
|---------|-------|
| `.font("Inter")` | Surcharger la police |
| `.title_size(22)` | Surcharger la taille du titre |
| `.set_font_size(14)` | Surcharger la taille de base |
| `.crosshair()` | Activer/ajouter le réticule sur ce graphique |
| `.zoom()` | Activer/ajouter le zoom sur ce graphique |
| `.animate(300)` | Ajouter une animation (ms) |
| `.export_button()` | Ajouter un bouton de téléchargement |
| `.responsive()` | Rendre responsive |
| `.border_radius(12)` | Définir le rayon des coins |
| `.set_opacity(0.85)` | Définir l'opacité des éléments |
| `.set_margin(16)` | Définir la marge |
| `.set_bg("#couleur")` | Surcharger le fond |

```python
import seraplot as sp

sp.config(font="Inter", background="#0f172a", gridlines=True)

chart1 = sp.bar("Défaut", labels, values)
chart2 = sp.bar("Surcharge", labels, values).font("Roboto").border_radius(20)
chart3 = sp.line("Propre", dates, values).zoom()
```

---

## Fond et cadre

### `set_bg(color=None)`

Définit la couleur de fond du wrapper HTML complet.

```python
chart = sp.build_bar_chart("Ventes", labels, values).set_bg("#0f172a")
chart = sp.build_scatter_chart("Données", x, y).set_bg("white")
chart = sp.build_pie_chart("Parts", labels, values).set_bg(None)  # transparent
```

Accepte toute chaîne de couleur CSS : `"#rrggbb"`, `"rgb(r,g,b)"`, couleurs nommées, ou
`None`/`"transparent"`.

---

### `set_frame(color=None)`

Définit le fond du canevas SVG indépendamment du wrapper HTML. Utile lors d'une
intégration dans une page avec son propre fond.

```python
chart = sp.build_bar_chart("Titre", labels, values).set_frame("transparent")
```

---

### `set_global_background(color)`

Fonctions au niveau du module qui appliquent un fond à **tous** les graphiques créés
après l'appel — utile pour les sessions notebook avec un thème cohérent.

<div class="sp-tabs" id="cm-gbg-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('cm-gbg-fr','cm-gbg-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('cm-gbg-fr','cm-gbg-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('cm-gbg-fr','cm-gbg-fr-ts',this)">TypeScript</button>
</div>
<div id="cm-gbg-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">sp.set_global_background("#0f172a")
chart1 = sp.build_bar_chart(...)
chart2 = sp.build_scatter_chart(...)
sp.reset_global_background()</code></pre></div>
<div id="cm-gbg-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">sp.set_global_background("#0f172a");
const chart1 = sp.build_bar_chart(...);
const chart2 = sp.build_scatter_chart(...);
sp.reset_global_background();</code></pre></div>
<div id="cm-gbg-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">sp.set_global_background("#0f172a");
const chart1 = sp.build_bar_chart(...);
const chart2 = sp.build_scatter_chart(...);
sp.reset_global_background();</code></pre></div>
</div>

---

## Grille et axes

### `show_grid()`

Force l'affichage des lignes de grille indépendamment du paramètre `gridlines` à la création.

```python
chart = sp.build_histogram("Âges", values).show_grid()
```

---

### `hide_grid()`

Force la désactivation des lignes de grille indépendamment du paramètre `gridlines` à la création.

```python
chart = sp.build_line_chart("Tendance", labels, values, gridlines=True).hide_grid()
```

---

### `no_x_axis()`

Supprime les lignes, ticks et étiquettes de l'axe X.

```python
chart = sp.build_bar_chart("", labels, values).no_x_axis()
```

---

### `no_y_axis()`

Supprime les lignes, ticks et étiquettes de l'axe Y.

```python
chart = sp.build_bar_chart("", labels, values).no_y_axis()
```

---

### `no_axes()`

Supprime les deux axes en une seule fois.

```python
chart = sp.build_scatter_chart("", x, y).no_axes()
```

---

## Étiquettes et texte

### `show_labels(position="bottom", labels=None, colors=None)`

Affiche une étiquette texte sur chaque élément du graphique. `position` peut être
`"top"`, `"bottom"`, `"left"` ou `"right"`.

```python
chart = sp.build_bar_chart("Chiffre d'affaires", labels, values).show_labels(position="top")

chart = sp.build_bar_chart("Chiffre d'affaires", labels, values).show_labels(
    position="top",
    labels=["142 k€", "98 k€", "210 k€"],
    colors=["#22c55e", "#ef4444", "#22c55e"],
)
```

---

### `no_title()`

Supprime le titre du rendu.

```python
chart = sp.build_pie_chart("Étiquette interne", labels, values).no_title()
```

---

### `no_legend()`

Supprime la légende du rendu.

```python
chart = sp.build_grouped_bar("T1", cats, series).no_legend()
```

---

### `set_font_size(px)`

Surcharge toutes les tailles de texte du SVG avec une seule valeur en pixels.

```python
chart = sp.build_radar_chart("Compétences", labels, values).set_font_size(11)
```

---

## Taille et échelle

### `scale(factor)`

Met à l'échelle tout le graphique (SVG et canevas). Utile pour produire des miniatures
ou des variantes haute définition.

```python
small = sp.build_bar_chart("Ventes", labels, values).scale(0.5)
large = sp.build_bar_chart("Ventes", labels, values).scale(2.0)
```

---

## Survol

### `no_hover()`

Désactive le moteur d'infobulles. Tous les événements pointeur `data-idx` sont retirés.
Utile pour des intégrations statiques sans interaction au survol.

```python
chart = sp.build_scatter_chart("Distribution", x, y).no_hover()
```

---

## Injection CSS et JavaScript

### `inject_css(css)`

Injecte un bloc `<style>` dans le `<head>` du HTML. Donne un accès complet au DOM SVG
— surcharger n'importe quelle classe interne, changer les couleurs, animations, polices.

Classes CSS internes de SeraPlot :

| Classe | Cible |
|--------|-------|
| `svg text` | Tout le texte du graphique |
| `.sp-gl` | Lignes de grille |
| `.sp-ax-x`, `.sp-ax-y` | Lignes des axes |
| `.sp-xt`, `.sp-yt` | Étiquettes des ticks |
| `.sp-xl`, `.sp-yl` | Titres des axes |
| `g[data-legend]` | Groupe légende |
| `.sp-ttl` | Titre du graphique |
| `[data-idx]` | Éléments interactifs (cibles de survol) |
| `rect.sp-bg` | Rectangle de fond du SVG |

```python
chart = sp.build_bar_chart("Thème sombre", labels, values).inject_css("""
    rect.sp-bg  { fill: #0f172a !important; }
    svg text    { fill: #e2e8f0 !important; }
    .sp-gl      { stroke: #1e293b !important; }
    [data-idx]  { opacity: 0.85; }
    [data-idx]:hover { filter: brightness(1.3); }
""")
```

---

### `inject_js(js)`

Injecte un bloc `<script>` avant `</body>`. Tout le DOM SVG rendu est accessible.
Utilisez-le pour ajouter des écouteurs d'événements, animations ou intégrations tierces.

```python
chart = sp.build_bar_chart("Ventes", labels, values).inject_js("""
    document.querySelectorAll('[data-idx]').forEach((el, i) => {
        setTimeout(() => el.style.opacity = '1', i * 50);
    });
""")

chart = sp.build_scatter_chart("Données", x, y).inject_js("""
    const btn = document.createElement('button');
    btn.textContent = 'Télécharger SVG';
    btn.onclick = () => {
        const svg  = document.querySelector('svg').outerHTML;
        const a    = document.createElement('a');
        a.href     = 'data:image/svg+xml;charset=utf-8,' + encodeURIComponent(svg);
        a.download = 'chart.svg';
        a.click();
    };
    document.body.appendChild(btn);
""")
```

---

## Export

### `save(path)`

Écrit le HTML dans un fichier.

```python
chart = sp.build_pie_chart("Parts", labels, values)
chart.save("rapport/pie.html")
```

---

### `to_svg()`

Extrait la chaîne SVG brute du HTML.

```python
svg_string = chart.to_svg()
```

---

### `export_svg(path)`

Écrit uniquement le SVG dans un fichier (sans le wrapper HTML).

```python
chart.export_svg("chart.svg")
```

</div>
