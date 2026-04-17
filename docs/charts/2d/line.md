# Line Chart

<div class="lang-en">

## Signature

```python
sp.build_line_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    show_points: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    hover_json: str = "",
    legend_position: str = "right",
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Single-series line chart with optional data points.

For **multiple series**, use [`build_multiline_chart`](multiline.md).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | X-axis labels |
| `values` | `list[float]` | required | Y values |
| `color_hex` | `int` | `0x6366F1` | Line color as hex int (indigo by default) |
| `show_points` | `bool` | `True` | Draw circles at data points |
| `gridlines` | `bool` | `False` | Horizontal gridlines |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"`, `"none"` |
| `width` | `int` | `900` | Width in pixels |
| `height` | `int` | `480` | Height in pixels |

---

## Returns

`Chart`

---

## Examples

### Time series






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
<div class="sp-tabs" id="line">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('line','line-py',this)">Python</button><button class="sp-tb" onclick="spTab('line','line-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('line','line-ts',this)">TypeScript</button></div>
<div id="line-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
          "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
revenue = [1200.0, 1350.0, 1100.0, 1600.0, 1800.0, 2100.0,
           1950.0, 2300.0, 2000.0, 2500.0, 2200.0, 2800.0]

logo = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png"
hover = sp.build_hover_json(months, images=[logo] * len(months))

chart = (
    sp.build_line_chart(
        "Annual Revenue",
        labels=months,
        values=revenue,
        x_label="Month",
        y_label="Revenue (€)",
        gridlines=True,
        color_hex=0x22d3ee,
        hover_json=hover,
    )
    .set_bg(None)
    .show_grid()
)</code></pre></div>
<div id="line-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
          "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
const revenue = [1200.0, 1350.0, 1100.0, 1600.0, 1800.0, 2100.0,
           1950.0, 2300.0, 2000.0, 2500.0, 2200.0, 2800.0]

const logo = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png"
const hover = sp.build_hover_json(months,
[logo])

const chart = (
    sp.build_line_chart("Annual Revenue",
months,
{
    values: revenue,
    x_label: "Month",
    y_label: "Revenue (€)",
    gridlines: true,
    color_hex: 0x22d3ee,
    hover_json: hover
})
    .set_bg(null)
    .show_grid()
)</code></pre></div>
<div id="line-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const months: string[] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
          "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
const revenue: number[] = [1200.0, 1350.0, 1100.0, 1600.0, 1800.0, 2100.0,
           1950.0, 2300.0, 2000.0, 2500.0, 2200.0, 2800.0]

const logo: string = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png"
const hover = sp.build_hover_json(months,
[logo])

const chart = (
    sp.build_line_chart("Annual Revenue",
months,
{
    values: revenue,
    x_label: "Month",
    y_label: "Revenue (€)",
    gridlines: true,
    color_hex: 0x22d3ee,
    hover_json: hover
})
    .set_bg(null)
    .show_grid()
)</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/line.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Multi-line](multiline.md) — `sp.build_multiline_chart()` for multiple series
- [Area Chart](area.md) — `sp.build_area_chart()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_line_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    show_points: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    hover_json: str = "",
    legend_position: str = "right",
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Graphique en courbe simple avec points de données optionnels.

Pour **plusieurs séries**, utilisez [`build_multiline_chart`](multiline.md).

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes de l'axe X |
| `values` | `list[float]` | requis | Valeurs Y |
| `color_hex` | `int` | `0x6366F1` | Couleur de la courbe (hex int) |
| `show_points` | `bool` | `True` | Dessiner des cercles aux points de données |
| `gridlines` | `bool` | `False` | Lignes de grille horizontales |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"` ou `"none"` |
| `width` | `int` | `900` | Largeur en pixels |
| `height` | `int` | `480` | Hauteur en pixels |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp

mois = ["Jan", "Fév", "Mar", "Avr", "Mai", "Jun",
        "Jul", "Aoû", "Sep", "Oct", "Nov", "Déc"]
revenu = [1200.0, 1350.0, 1100.0, 1600.0, 1800.0, 2100.0,
          1950.0, 2300.0, 2000.0, 2500.0, 2200.0, 2800.0]

chart = (
    sp.build_line_chart(
        "Chiffre d'affaires annuel",
        labels=mois,
        values=revenu,
        x_label="Mois",
        y_label="Chiffre d'affaires (€)",
        gridlines=True,
        color_hex=0x22d3ee,
    )
    .set_bg(None)
)
```

---

## Voir aussi

- [Multi-courbes](multiline.md) — `sp.build_multiline_chart()` pour plusieurs séries
- [Graphique en aires](area.md) — `sp.build_area_chart()`

</div>
