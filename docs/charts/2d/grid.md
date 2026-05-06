# Grid Layout

<div class="lang-en">

## Signature

```python
sp.build_grid(
    charts: list[Chart],
    *,
    cols: int = 2,
    width: int = 1200,
    height: int = 800,
    background: str | None = None,
    gap: int = 12,
    title: str = "",
) -> Chart
```

Aliases: `sp.grid`

---

## Description

A grid layout places multiple `Chart` objects side by side in a responsive column grid. Charts fill left-to-right, top-to-bottom in the order they appear in `charts`. The `cols` parameter sets the number of columns; if `len(charts)` is not a multiple of `cols`, the last row is left-aligned with empty space on the right. Each cell gets an equal share of the total `width` and `height`, minus `gap` pixels between cells. The grid is ideal for dashboards and reports where multiple equal-importance charts need to be seen simultaneously.

**Ideal for:**
- Dashboards displaying several KPI charts at once
- Side-by-side comparisons of related charts (e.g., before/after, different regions)
- Report layouts combining diverse chart types in a structured grid

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `charts` | `list[Chart]` | required | Charts to place in the grid |
| `cols` | `int` | `2` | Number of columns |
| `width` | `int` | `1200` | Total canvas width in pixels |
| `height` | `int` | `800` | Total canvas height in pixels |
| `background` | `str \| None` | `None` | Background color or `None` = transparent |
| `gap` | `int` | `12` | Gap between cells in pixels |
| `title` | `str` | `""` | Optional grid header title |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>


</div>

<iframe src="../../previews/grid.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Slideshow](slideshow.md) — `sp.build_slideshow()`
- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Line Chart](line.md) — `sp.build_line_chart()`

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_grid(
    charts: list[Chart],
    *,
    cols: int = 2,
    width: int = 1200,
    height: int = 800,
    background: str | None = None,
    gap: int = 12,
    title: str = "",
) -> Chart
```

Aliases: `sp.grid`

---

<h2>Description</h2>

Une grille place plusieurs objets `Chart` côte à côte dans une grille en colonnes. Les graphiques remplissent de gauche à droite, de haut en bas, dans l'ordre d'apparition dans `charts`. Le paramètre `cols` définit le nombre de colonnes. Si `len(charts)` n'est pas un multiple de `cols`, la dernière ligne est alignée à gauche avec de l'espace vide à droite. Chaque cellule reçoit une part égale de la `width` et `height` totales, moins `gap` pixels entre les cellules.

**Idéal pour :**
- Tableaux de bord affichant plusieurs graphiques KPI simultanément
- Comparaisons côte à côte de graphiques liés (avant/après, différentes régions)
- Mises en page de rapport combinant des types de graphiques variés dans une grille structurée

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `charts` | `list[Chart]` | requis | Graphiques à placer dans la grille |
| `cols` | `int` | `2` | Nombre de colonnes |
| `width` | `int` | `1200` | Largeur totale du canvas en pixels |
| `height` | `int` | `800` | Hauteur totale du canvas en pixels |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |
| `gap` | `int` | `12` | Espace entre les cellules en pixels |
| `title` | `str` | `""` | Titre d'en-tête optionnel de la grille |

---

<h2>Retourne</h2>

`Chart`

---


</div>

<iframe src="../../previews/grid.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

<h2>Voir aussi</h2>

- [Slideshow](slideshow.md) — `sp.build_slideshow()`
- [Graphique en barres](bar.md) — `sp.build_bar_chart()`
- [Courbe](line.md) — `sp.build_line_chart()`

</div>
