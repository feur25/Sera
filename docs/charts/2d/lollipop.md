# Lollipop Chart

<div class="lang-en">

## Signature

```python
sp.build_lollipop_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    orientation: str = "v",
    show_text: bool = False,
    sort_order: str = "none",
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Lollipop chart — a cleaner alternative to bar charts using a thin stem and a circle at the end.
Reduces ink-to-data ratio compared to filled bars.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Values per category |
| `color_hex` | `int` | `0x6366F1` | Stem and dot color |
| `palette` | `list[int] \| None` | `None` | Per-category colors |
| `orientation` | `str` | `"v"` | `"v"` (vertical) or `"h"` (horizontal) |
| `show_text` | `bool` | `False` | Show value labels |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"`, or `"none"` |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Show gridlines |

---

## Returns

`Chart`

---

## Examples

### Sorted lollipop






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
<div class="sp-tabs" id="lollipop">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('lollipop','lollipop-py',this)">Python</button><button class="sp-tb" onclick="spTab('lollipop','lollipop-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('lollipop','lollipop-ts',this)">TypeScript</button></div>
<div id="lollipop-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_lollipop_chart(
    "Top Countries by GDP per Capita",
    labels=["Luxembourg", "Switzerland", "USA", "Germany", "France"],
    values=[135605, 92434, 76398, 50802, 42409],
    orientation="h",
    sort_order="desc",
    show_values=True,
    x_label="GDP per capita ($)",
)</code></pre></div>
<div id="lollipop-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.build_lollipop_chart("Top Countries by GDP per Capita",
["Luxembourg", "Switzerland", "USA", "Germany", "France"],
{
    values: [135605, 92434, 76398, 50802, 42409],
    orientation: "h",
    sort_order: "desc",
    show_values: true,
    x_label: "GDP per capita ($)"
})</code></pre></div>
<div id="lollipop-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.build_lollipop_chart("Top Countries by GDP per Capita",
["Luxembourg", "Switzerland", "USA", "Germany", "France"],
{
    values: [135605, 92434, 76398, 50802, 42409],
    orientation: "h",
    sort_order: "desc",
    show_values: true,
    x_label: "GDP per capita ($)"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/lollipop.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Bar Chart](bar.md)
- [Dumbbell](dumbbell.md)
- [Lollipop 3D](../3d/lollipop3d.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_lollipop_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    orientation: str = "v",
    show_text: bool = False,
    sort_order: str = "none",
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Graphique en sucette — alternative épurée aux barres avec une tige fine et un cercle au bout. Réduit le ratio encre/données par rapport aux barres remplies.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des catégories |
| `values` | `list[float]` | requis | Valeurs par catégorie |
| `color_hex` | `int` | `0x6366F1` | Couleur de la tige et du point |
| `palette` | `list[int] \| None` | `None` | Couleurs par catégorie |
| `orientation` | `str` | `"v"` | `"v"` (vertical) ou `"h"` (horizontal) |
| `show_text` | `bool` | `False` | Afficher les étiquettes de valeur |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"` ou `"none"` |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `480` | Hauteur du canvas |
| `x_label` | `str` | `""` | Étiquette de l'axe X |
| `y_label` | `str` | `""` | Étiquette de l'axe Y |
| `gridlines` | `bool` | `True` | Afficher les lignes de grille |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp

chart = sp.build_lollipop_chart(
    "Top pays par PIB par habitant",
    labels=["Luxembourg", "Suisse", "USA", "Allemagne", "France"],
    values=[135605, 92434, 76398, 50802, 42409],
    orientation="h",
    sort_order="desc",
    show_text=True,
    x_label="PIB par habitant ($)",
)
```

---

## Voir aussi

- [Graphique en barres](bar.md)
- [Haltère](dumbbell.md)
- [Sucette 3D](../3d/lollipop3d.md)

</div>
