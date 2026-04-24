# Donut Chart

<div class="lang-en">

## Signature

```python
sp.build_donut_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_pct: bool = True,
    inner_radius_ratio: float = 0.55,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
    legend_position: str = "right",
) -> Chart
```

Aliases: `sp.donut`, `sp.donut_chart`

---

## Description

Donut chart — identical to a pie chart with a circular hole at the center.

Diagramme en anneau — comme un camembert mais avec un trou circulaire au centre.

The `inner_radius_ratio` controls what fraction of the radius is the hole (0.0 = solid pie, 1.0 = invisible ring).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Slice labels |
| `values` | `list[float]` | required | Slice values |
| `show_pct` | `bool` | `True` | Show percentage labels |
| `inner_radius_ratio` | `float` | `0.55` | Hole size (0.0–0.9) |
| `width` | `int` | `700` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `palette` | `list[int] \| None` | `None` | Custom hex color palette |
| `background` | `str \| None` | `None` | Chart background |
| `hover_json` | `str \| None` | `None` | Custom hover tooltip JSON |
| `legend_position` | `str` | `"right"` | Legend position |

---

## Returns

`Chart`

---

## Examples

### Basic donut






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
<div class="sp-tabs" id="donut">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('donut','donut-py',this)">Python</button><button class="sp-tb" onclick="spTab('donut','donut-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('donut','donut-ts',this)">TypeScript</button></div>
<div id="donut-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.build_donut_chart(
    "Budget Allocation",
    labels=["R&amp;D", "Marketing", "Operations", "Support"],
    values=[35, 25, 30, 10],
    inner_radius_ratio=0.55,
    show_pct=True,
)</code></pre></div>
<div id="donut-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.build_donut_chart("Budget Allocation",
["R&amp;D", "Marketing", "Operations", "Support"],
{
    values: [35, 25, 30, 10],
    inner_radius_ratio: 0.55,
    show_pct: true
})</code></pre></div>
<div id="donut-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.build_donut_chart("Budget Allocation",
["R&amp;D", "Marketing", "Operations", "Support"],
{
    values: [35, 25, 30, 10],
    inner_radius_ratio: 0.55,
    show_pct: true
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/donut.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Pie Chart](pie.md)
- [Sunburst](sunburst.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_donut_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_pct: bool = True,
    inner_radius_ratio: float = 0.55,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
    legend_position: str = "right",
) -> Chart
```

Aliases: `sp.donut`, `sp.donut_chart`

---

## Description

Diagramme en anneau — identique à un camembert avec un trou circulaire central. `inner_radius_ratio` contrôle la taille du trou (0.0 = camembert plein, 1.0 = anneau invisible).

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des tranches |
| `values` | `list[float]` | requis | Valeurs des tranches |
| `show_pct` | `bool` | `True` | Afficher les pourcentages |
| `inner_radius_ratio` | `float` | `0.55` | Taille du trou (0.0–0.9) |
| `width` | `int` | `700` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `palette` | `list[int] \| None` | `None` | Palette de couleurs hex |
| `background` | `str \| None` | `None` | Fond du graphique |
| `hover_json` | `str \| None` | `None` | JSON d'infobulle personnalisée |
| `legend_position` | `str` | `"right"` | Position de la légende |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp

chart = sp.build_donut_chart(
    "Répartition du budget",
    labels=["R&D", "Marketing", "Opérations", "Support"],
    values=[35, 25, 30, 10],
    inner_radius_ratio=0.55,
    show_pct=True,
)
```

---

## Voir aussi

- [Camembert](pie.md)
- [Sunburst](sunburst.md)

</div>
