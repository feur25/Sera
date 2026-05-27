# Area Chart

<div class="lang-en">

Aliases: `sp.area`, `sp.area_chart`

---

## Description

An area chart is a line chart where the region between the line and the horizontal axis is filled with a semi-transparent color, emphasizing the total magnitude of values over time. When `stacked=True`, multiple series are layered on top of each other so their cumulative total fills the canvas — ideal for part-to-whole analysis. Each inner list in `series_values` represents one series; all inner lists must have the same length as `x_labels`. SeraPlot automatically assigns colors from the default palette or from a custom one.

**Ideal for:**
- Showing total volume changing over time
- Visualizing part-to-whole composition across time with stacked mode
- Comparing two or three series with overlapping fills

---

## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `color_hex` | basic, connected_scatter, dashed, filled, gapped, sparkline, spline, stepped |
| `dash_pattern` | dashed |
| `fill_opacity` | filled |
| `gap_threshold` | gapped |
| `gridlines` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `height` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `hover` | basic, multi |
| `labels` | basic, connected_scatter, dashed, filled, gapped, neon, spline, stepped |
| `legend_position` | connected_scatter, dashed, filled, gapped, multi, neon |
| `marker_size` | connected_scatter, gapped, spline, stepped |
| `palette` | connected_scatter, dashed, filled, gapped, multi, neon, sparkline |
| `series` | connected_scatter, dashed, filled, gapped, multi, neon, sparkline |
| `show_points` | basic, gapped, multi, neon, spline, stepped |
| `sort_order` | basic, multi |
| `spark_cell_h` | sparkline |
| `spark_cell_w` | sparkline |
| `spark_cols` | sparkline |
| `spline_tension` | spline |
| `stack_fill` | filled |
| `step_shape` | stepped |
| `stroke_width` | connected_scatter, dashed, filled, gapped, sparkline, spline, stepped |
| `title` | all |
| `values` | basic, connected_scatter, dashed, filled, gapped, neon, sparkline, spline, stepped |
| `width` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `x_label` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `x_labels` | connected_scatter, dashed, filled, gapped, multi, neon |
| `y_label` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>


</div>

<iframe src="../../previews/area.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

</div>

<div class="lang-fr">

Aliases: `sp.area`, `sp.area_chart`

---

<h2>Description</h2>

Un graphique de zone est un graphique en ligne dont la région entre la courbe et l'axe horizontal est remplie d'une couleur semi-transparente, mettant en valeur la magnitude totale des valeurs dans le temps. Lorsque `stacked=True`, plusieurs séries sont superposées de sorte que leur total cumulatif remplit le canevas — idéal pour l'analyse parties-tout. Chaque liste interne de `series_values` représente une série ; toutes les listes internes doivent avoir la même longueur que `x_labels`. SeraPlot assigne automatiquement les couleurs de la palette par défaut ou d'une palette personnalisée.

**Idéal pour :**
- Montrer l'évolution du volume total dans le temps
- Visualiser la composition parties-tout dans le temps avec le mode empilé
- Comparer deux ou trois séries avec des zones superposées

---

<h2>Paramètres</h2>

| Paramètre | Utilisé par variantes |
|-----------|----------------------|
| `color_hex` | basic, connected_scatter, dashed, filled, gapped, sparkline, spline, stepped |
| `dash_pattern` | dashed |
| `fill_opacity` | filled |
| `gap_threshold` | gapped |
| `gridlines` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `height` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `hover` | basic, multi |
| `labels` | basic, connected_scatter, dashed, filled, gapped, neon, spline, stepped |
| `legend_position` | connected_scatter, dashed, filled, gapped, multi, neon |
| `marker_size` | connected_scatter, gapped, spline, stepped |
| `palette` | connected_scatter, dashed, filled, gapped, multi, neon, sparkline |
| `series` | connected_scatter, dashed, filled, gapped, multi, neon, sparkline |
| `show_points` | basic, gapped, multi, neon, spline, stepped |
| `sort_order` | basic, multi |
| `spark_cell_h` | sparkline |
| `spark_cell_w` | sparkline |
| `spark_cols` | sparkline |
| `spline_tension` | spline |
| `stack_fill` | filled |
| `step_shape` | stepped |
| `stroke_width` | connected_scatter, dashed, filled, gapped, sparkline, spline, stepped |
| `title` | toutes |
| `values` | basic, connected_scatter, dashed, filled, gapped, neon, sparkline, spline, stepped |
| `width` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `x_label` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |
| `x_labels` | connected_scatter, dashed, filled, gapped, multi, neon |
| `y_label` | basic, connected_scatter, dashed, filled, gapped, multi, neon, spline, stepped |

---

<h2>Retourne</h2>

`Chart`

---


</div>

<iframe src="../../previews/area.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

</div>
