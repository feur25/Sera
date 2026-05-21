# Stacked Bar Chart

<div class="lang-en">

Aliases: `sp.stacked_bar`

---

## Description

A stacked bar chart divides each bar into colored segments representing sub-series contributions to a total. `series_values` is a flat list laid out in row-major order: all values for series 1 first, then all values for series 2, etc. If there are `n` categories and `k` series, `series_values` has length `n × k`. The chart is ideal for seeing both the total magnitude and the part-to-whole composition in a single view. Use `no_y_axis` with `show_values` for a cleaner self-labelled layout.

**Ideal for:**
- Tracking total and component breakdown simultaneously (revenue by product line over years)
- Part-to-whole composition across categories
- Budget allocation or resource distribution comparisons

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | X-axis category labels |
| `series_values` | `list[float]` | required | Flat row-major values: all series 1 values, then series 2, etc. |
| `show_values` | `bool` | `False` | Show numeric labels inside each segment |
| `series_names` | `list[str] \| None` | `None` | Legend labels per series |
| `palette` | `list[int] \| None` | `None` | Colors per series |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `legend_position` | `str` | `"right"` | Legend position: `"right"`, `"bottom"`, `"top"`, `"none"` |
| `gridlines` | `bool` | `False` | Show horizontal gridlines |
| `no_y_axis` | `bool` | `False` | Hide Y-axis ticks and labels |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>


</div>

<iframe src="../../previews/stacked-bar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Grouped Bar Chart](grouped-bar.md) — `sp.build_grouped_bar()`
- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Area Chart](area.md) — `sp.build_area_chart()`

</div>

<div class="lang-fr">

Aliases: `sp.stacked_bar`

---

<h2>Description</h2>

Un graphique en barres empilées divise chaque barre en segments colorés représentant les contributions de sous-séries à un total. `series_values` est une liste plate en ordre row-major : toutes les valeurs de la série 1 en premier, puis toutes les valeurs de la série 2, etc. Si on a `n` catégories et `k` séries, `series_values` a une longueur de `n × k`. Le graphique est idéal pour voir à la fois la magnitude totale et la composition partie-à-tout en une seule vue.

**Idéal pour :**
- Suivre simultanément le total et la décomposition par composant (revenus par ligne de produits sur plusieurs années)
- Comparaisons de composition partie-à-tout entre catégories
- Comparaisons d'allocation budgétaire ou de distribution de ressources

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `category_labels` | `list[str]` | requis | Étiquettes des catégories de l'axe X |
| `series_values` | `list[float]` | requis | Valeurs row-major : toutes les valeurs de la série 1, puis série 2, etc. |
| `show_values` | `bool` | `False` | Afficher les étiquettes numériques dans chaque segment |
| `series_names` | `list[str] \| None` | `None` | Étiquettes de légende par série |
| `palette` | `list[int] \| None` | `None` | Couleurs par série |
| `width` | `int` | `900` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `legend_position` | `str` | `"right"` | Position de la légende : `"right"`, `"bottom"`, `"top"`, `"none"` |
| `gridlines` | `bool` | `False` | Afficher les lignes de grille horizontales |
| `no_y_axis` | `bool` | `False` | Masquer les graduations et étiquettes de l'axe Y |

---

<h2>Retourne</h2>

`Chart`

---


</div>

<iframe src="../../previews/stacked-bar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

<h2>Voir aussi</h2>

- [Barres groupées](grouped-bar.md) — `sp.build_grouped_bar()`
- [Graphique en barres](bar.md) — `sp.build_bar_chart()`
- [Graphique en aires](area.md) — `sp.build_area_chart()`

</div>
