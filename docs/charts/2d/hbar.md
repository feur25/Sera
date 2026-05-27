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

| Parameter | Used by variants |
|-----------|------------------|
| `bar_gap` | grouped_stacked, relative |
| `bargroup_gap` | grouped_stacked |
| `category_labels` | grouped, grouped_stacked, marimekko, multicategory, relative |
| `color_groups` | basic |
| `color_hex` | basic, deluxe, pictogram, prism |
| `corner_radius` | grouped_stacked, relative |
| `gridlines` | basic, deluxe, grouped, grouped_stacked, marimekko, multicategory, prism, relative |
| `height` | all |
| `hover` | basic, deluxe, grouped, prism |
| `icon_size` | pictogram |
| `labels` | basic, deluxe, pictogram, prism |
| `legend_position` | grouped, grouped_stacked, marimekko, multicategory, relative |
| `max_icons_per_column` | pictogram |
| `offset_groups` | grouped_stacked |
| `orientation` | grouped |
| `palette` | all |
| `series` | grouped, grouped_stacked, marimekko, multicategory, relative |
| `show_text` | basic, deluxe, grouped, marimekko, prism |
| `sort_order` | basic, grouped |
| `super_categories` | multicategory |
| `title` | all |
| `unit_description` | pictogram |
| `units_per_icon` | pictogram |
| `values` | basic, deluxe, multicategory, pictogram, prism |
| `width` | all |
| `widths` | marimekko |
| `x_label` | basic, grouped, grouped_stacked, marimekko, multicategory, relative |
| `y_label` | basic, grouped, grouped_stacked, marimekko, multicategory, relative |

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

| Paramètre | Utilisé par variantes |
|-----------|----------------------|
| `bar_gap` | grouped_stacked, relative |
| `bargroup_gap` | grouped_stacked |
| `category_labels` | grouped, grouped_stacked, marimekko, multicategory, relative |
| `color_groups` | basic |
| `color_hex` | basic, deluxe, pictogram, prism |
| `corner_radius` | grouped_stacked, relative |
| `gridlines` | basic, deluxe, grouped, grouped_stacked, marimekko, multicategory, prism, relative |
| `height` | toutes |
| `hover` | basic, deluxe, grouped, prism |
| `icon_size` | pictogram |
| `labels` | basic, deluxe, pictogram, prism |
| `legend_position` | grouped, grouped_stacked, marimekko, multicategory, relative |
| `max_icons_per_column` | pictogram |
| `offset_groups` | grouped_stacked |
| `orientation` | grouped |
| `palette` | toutes |
| `series` | grouped, grouped_stacked, marimekko, multicategory, relative |
| `show_text` | basic, deluxe, grouped, marimekko, prism |
| `sort_order` | basic, grouped |
| `super_categories` | multicategory |
| `title` | toutes |
| `unit_description` | pictogram |
| `units_per_icon` | pictogram |
| `values` | basic, deluxe, multicategory, pictogram, prism |
| `width` | toutes |
| `widths` | marimekko |
| `x_label` | basic, grouped, grouped_stacked, marimekko, multicategory, relative |
| `y_label` | basic, grouped, grouped_stacked, marimekko, multicategory, relative |

---

<h2>Retourne</h2>

`Chart`

---


</div>

<iframe src="../../previews/hbar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

</div>
