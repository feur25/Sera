# Donut Chart

<div class="lang-en">

Aliases: `sp.donut`, `sp.donut_chart`

---

## Description

A donut chart is a pie chart with a circular cutout at the center, reducing the visual weight and leaving space to display a summary metric (total, KPI name, or label) in the hole. The `inner_radius_ratio` parameter controls the size of the cutout: `0.0` produces a solid pie, `0.9` produces a thin ring. Donut charts carry the same part-to-whole semantics as pie charts while being more modern and less prone to area-distortion perception issues. They work best with 3–7 slices.

**Ideal for:**
- Part-to-whole breakdowns where you also want to highlight a key total in the center
- KPI dashboards and executive summaries
- A slightly more modern alternative to standard pie charts

---

## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `center_subtext` | kpi, nested, proportional |
| `center_text` | kpi, nested, proportional |
| `donut` | nested, proportional, subplots |
| `gridlines` | proportional |
| `height` | nested, proportional, subplots |
| `hover` | proportional |
| `labels` | nested, proportional, subplots |
| `legend_position` | proportional |
| `min_label_frac` | proportional |
| `palette` | proportional, subplots |
| `pattern` | proportional |
| `proportional` | proportional, subplots |
| `pull` | basic, donut, exploded, kpi, pattern, proportional, semi |
| `secondary_labels` | nested, proportional |
| `secondary_values` | nested, proportional |
| `series` | proportional, subplots |
| `show_pct` | proportional |
| `sort_order` | proportional |
| `subplot_cols` | proportional, subplots |
| `subplot_titles` | proportional, subplots |
| `title` | nested, proportional, subplots |
| `values` | exploded, kpi, nested, proportional |
| `variant` | proportional |
| `width` | nested, proportional, subplots |
| `x_label` | proportional |
| `y_label` | proportional |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>


</div>

<iframe src="../../previews/donut.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

</div>

<div class="lang-fr">

Aliases: `sp.donut`, `sp.donut_chart`

---

<h2>Description</h2>

Un graphique en anneau est un graphique en secteurs avec un trou circulaire au centre, réduisant le poids visuel et laissant de la place pour afficher une métrique récapitulative (total, nom d'indicateur ou label) dans le vide. Le paramètre `inner_radius_ratio` contrôle la taille du trou : `0.0` produit un secteur plein, `0.9` produit un anneau fin. Les graphiques en anneau ont la même sémantique parties-tout que les graphiques en secteurs, tout en étant plus modernes et moins sujets aux problèmes de perception liés à la distorsion des surfaces. Ils fonctionnent mieux avec 3 à 7 tranches.

**Idéal pour :**
- Les décompositions parties-tout lorsque vous souhaitez également mettre en évidence un total clé au centre
- Les tableaux de bord d'indicateurs et les rapports exécutifs
- Une alternative légèrement plus moderne aux graphiques en secteurs standard

---

<h2>Paramètres</h2>

| Paramètre | Utilisé par variantes |
|-----------|----------------------|
| `center_subtext` | kpi, nested, proportional |
| `center_text` | kpi, nested, proportional |
| `donut` | nested, proportional, subplots |
| `gridlines` | proportional |
| `height` | nested, proportional, subplots |
| `hover` | proportional |
| `labels` | nested, proportional, subplots |
| `legend_position` | proportional |
| `min_label_frac` | proportional |
| `palette` | proportional, subplots |
| `pattern` | proportional |
| `proportional` | proportional, subplots |
| `pull` | basic, donut, exploded, kpi, pattern, proportional, semi |
| `secondary_labels` | nested, proportional |
| `secondary_values` | nested, proportional |
| `series` | proportional, subplots |
| `show_pct` | proportional |
| `sort_order` | proportional |
| `subplot_cols` | proportional, subplots |
| `subplot_titles` | proportional, subplots |
| `title` | nested, proportional, subplots |
| `values` | exploded, kpi, nested, proportional |
| `variant` | proportional |
| `width` | nested, proportional, subplots |
| `x_label` | proportional |
| `y_label` | proportional |

---

<h2>Retourne</h2>

`Chart`

---


</div>

<iframe src="../../previews/donut.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

</div>
