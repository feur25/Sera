# Area Chart

<div class="lang-en">

## Signature

```python
sp.build_area_chart(
    title: str,
    x_labels: list[str],
    series_values: list[list[float]],
    *,
    stacked: bool = False,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    legend_position: str = "top",
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.area`, `sp.area_chart`

---

## Description

Filled area chart, optionally stacked. Ideal for showing cumulative part-to-whole over time.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x_labels` | `list[str]` | required | X-axis tick labels |
| `series_values` | `list[list[float]]` | required | One list per series |
| `stacked` | `bool` | `False` | Stack areas instead of overlapping |
| `series_names` | `list[str] \| None` | `None` | Legend names per series |
| `palette` | `list[int] \| None` | `None` | Custom colors |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Horizontal gridlines |
| `legend_position` | `str` | `"top"` | Legend position |
| `hover_json` | `str \| None` | `None` | Custom hover tooltip JSON |

---

## Returns

`Chart`

---

## Examples

### Overlapping areas


### Stacked area






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
<div class="sp-tabs" id="area">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('area','area-py',this)">Python</button><button class="sp-tb" onclick="spTab('area','area-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('area','area-ts',this)">TypeScript</button></div>
<div id="area-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.build_area_chart(
    "Revenue Stacked by Region",
    x_labels=["Q1","Q2","Q3","Q4"],
    series_values=[
        [12000, 14000, 13500, 16000],
        [8000,  9200,  10000, 11500],
        [4500,  5000,  5200,  6000],
    ],
    series_names=["North", "South", "East"],
    stacked=True,
    y_label="Revenue ($)",
)</code></pre></div>
<div id="area-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.build_area_chart("Revenue Stacked by Region",
["Q1", "Q2", "Q3", "Q4"],
{
    series_values: [[12000, 14000, 13500, 16000], [8000, 9200, 10000, 11500], [4500, 5000, 5200, 6000]],
    series_names: ["North", "South", "East"],
    stacked: true,
    y_label: "Revenue ($)"
})</code></pre></div>
<div id="area-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.build_area_chart("Revenue Stacked by Region",
["Q1", "Q2", "Q3", "Q4"],
{
    series_values: [[12000, 14000, 13500, 16000], [8000, 9200, 10000, 11500], [4500, 5000, 5200, 6000]],
    series_names: ["North", "South", "East"],
    stacked: true,
    y_label: "Revenue ($)"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/area.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Multi-Line Chart](multiline.md)
- [Line Chart](line.md)
- [Stacked Bar](stacked-bar.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_area_chart(
    title: str,
    x_labels: list[str],
    series_values: list[list[float]],
    *,
    stacked: bool = False,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    legend_position: str = "top",
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.area`, `sp.area_chart`

---

## Description

Graphique en aires remplies, optionnellement empilées. Idéal pour visualiser les évolutions cumulatives au fil du temps.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x_labels` | `list[str]` | requis | Étiquettes de graduation de l'axe X |
| `series_values` | `list[list[float]]` | requis | Une liste par série |
| `stacked` | `bool` | `False` | Empiler les aires au lieu de les superposer |
| `series_names` | `list[str] \| None` | `None` | Noms des séries pour la légende |
| `palette` | `list[int] \| None` | `None` | Couleurs personnalisées |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `480` | Hauteur du canvas |
| `x_label` | `str` | `""` | Étiquette de l'axe X |
| `y_label` | `str` | `""` | Étiquette de l'axe Y |
| `gridlines` | `bool` | `True` | Lignes de grille horizontales |
| `legend_position` | `str` | `"top"` | Position de la légende |
| `hover_json` | `str \| None` | `None` | JSON d'infobulle personnalisée |

---

## Retourne

`Chart`

---

## Exemples

### Aires empilées

```python
import seraplot as sp

chart = sp.build_area_chart(
    "Chiffre d'affaires empiлé par région",
    x_labels=["T1","T2","T3","T4"],
    series_values=[
        [12000, 14000, 13500, 16000],
        [8000,  9200,  10000, 11500],
        [4500,  5000,  5200,  6000],
    ],
    series_names=["Nord", "Sud", "Est"],
    stacked=True,
    y_label="Chiffre d'affaires (€)",
)
```

---

## Voir aussi

- [Graphique multi-courbes](multiline.md)
- [Graphique en courbe](line.md)
- [Barres empilées](stacked-bar.md)

</div>
