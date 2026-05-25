# Horizontal Bar Chart

<div class="lang-en">

Aliases: `sp.hbar`, `sp.barh`, `sp.horizontal_bar`

---

## Description

A horizontal bar chart draws bars along the horizontal axis instead of the vertical axis, which is advantageous when category labels are long text strings that would otherwise overlap or require rotation. It is the canonical chart for ranked lists, leaderboards, and survey responses. By default SeraPlot shows value labels at the end of each bar (`show_text=True`), which eliminates the need for a separate axis scale when the labels themselves communicate magnitude. Sorting with `sort_order="desc"` produces a clean ranked view with minimal extra code.

**Ideal for:**
- Ranked lists and leaderboards with 5–30 items
- Comparing categories with long text labels
- Survey responses and NPS breakdowns

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `labels` | `list[str]` | — | Category label for each bar |
| `values` | `list[float]` | — | Numeric value for each bar |
| `show_text` | `bool` | `True` | Display the numeric value at the end of each bar |
| `sort_order` | `str` | `"none"` | Sort bars before rendering: `"asc"`, `"desc"`, or `"none"` |
| `color_groups` | `list[str] \| None` | `None` | Categorical group name per bar for auto-coloring |
| `palette` | `list[int] \| None` | `None` | Custom bar colors as hex integers |
| `background` | `str \| None` | `None` | CSS background color override |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `500` | Canvas height in pixels |
| `x_label` | `str` | `""` | Label for the X axis |
| `y_label` | `str` | `""` | Label for the Y axis |
| `no_y_axis` | `bool` | `False` | Hide the Y axis labels |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>


</div>

<iframe src="../../previews/hbar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

</div>

<div class="lang-fr">

Aliases: `sp.hbar`, `sp.barh`, `sp.horizontal_bar`

---

<h2>Description</h2>

Un graphique à barres horizontales trace les barres le long de l'axe horizontal plutôt que vertical, ce qui est avantageux lorsque les labels de catégories sont de longues chaînes de texte qui se chevaucheraient ou nécessiteraient une rotation. C'est le graphique canonique pour les classements, les tableaux de bord et les réponses à des sondages. Par défaut, SeraPlot affiche les valeurs numériques à l'extrémité de chaque barre (`show_text=True`), ce qui élimine le besoin d'une échelle d'axe séparée. Le tri avec `sort_order="desc"` produit une vue classée claire avec un minimum de code.

**Idéal pour :**
- Les classements et palmarès de 5 à 30 éléments
- La comparaison de catégories avec de longs libellés
- Les réponses à des sondages et les analyses NPS

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `labels` | `list[str]` | — | Label de catégorie pour chaque barre |
| `values` | `list[float]` | — | Valeur numérique pour chaque barre |
| `show_text` | `bool` | `True` | Afficher la valeur numérique à l'extrémité de chaque barre |
| `sort_order` | `str` | `"none"` | Trier les barres avant le rendu : `"asc"`, `"desc"` ou `"none"` |
| `color_groups` | `list[str] \| None` | `None` | Nom de groupe catégoriel par barre pour la coloration automatique |
| `palette` | `list[int] \| None` | `None` | Couleurs de barres personnalisées en entiers hexadécimaux |
| `background` | `str \| None` | `None` | Couleur de fond CSS |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `500` | Hauteur du canevas en pixels |
| `x_label` | `str` | `""` | Label de l'axe X |
| `y_label` | `str` | `""` | Label de l'axe Y |
| `no_y_axis` | `bool` | `False` | Masquer les labels de l'axe Y |

---

<h2>Retourne</h2>

`Chart`

---


</div>

<iframe src="../../previews/hbar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

</div>
