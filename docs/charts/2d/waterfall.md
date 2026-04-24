# Waterfall Chart

<div class="lang-en">

## Signature

```python
sp.build_waterfall(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    color_pos: int = 0x22c55e,
    color_neg: int = 0xef4444,
    color_total: int = 0x6366f1,
    palette: list[int] | None = None,
    background: str | None = None,
    gridlines: bool = True,
) -> Chart
```

Aliases: `sp.waterfall`

---

## Description

Waterfall chart showing sequential positive and negative contributions to a running total.
The last bar can act as the cumulative total.

Positive values rise, negative values fall. The last bar typically represents the final total.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Step labels |
| `values` | `list[float]` | required | Step values (positive or negative) |
| `show_text` | `bool` | `True` | Show value labels on bars |
| `color_pos` | `int` | `0x22c55e` | Color for positive bars |
| `color_neg` | `int` | `0xef4444` | Color for negative bars |
| `color_total` | `int` | `0x6366f1` | Color for total bar |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Horizontal gridlines |

---

## Returns

`Chart`

---

## Examples

### P&L breakdown





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
<div class="sp-tabs" id="waterfall">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('waterfall','waterfall-py',this)">Python</button><button class="sp-tb" onclick="spTab('waterfall','waterfall-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('waterfall','waterfall-ts',this)">TypeScript</button></div>
<div id="waterfall-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.build_waterfall(
    "Annual P&amp;L Waterfall",
    labels=["Revenue", "COGS", "Gross Profit", "OpEx", "EBITDA", "D&amp;A", "Net Income"],
    values=[100000, -45000, 0, -30000, 0, -5000, 0],
    show_text=True,
    y_label="$",
)</code></pre></div>
<div id="waterfall-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.build_waterfall("Annual P&amp;L Waterfall",
["Revenue", "COGS", "Gross Profit", "OpEx", "EBITDA", "D&amp;A", "Net Income"],
{
    values: [100000, -45000, 0, -30000, 0, -5000, 0],
    show_text: true,
    y_label: "$"
})</code></pre></div>
<div id="waterfall-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.build_waterfall("Annual P&amp;L Waterfall",
["Revenue", "COGS", "Gross Profit", "OpEx", "EBITDA", "D&amp;A", "Net Income"],
{
    values: [100000, -45000, 0, -30000, 0, -5000, 0],
    show_text: true,
    y_label: "$"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/waterfall.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Bar Chart](bar.md)
- [Funnel](funnel.md)
- [Bullet](bullet.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_waterfall(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    color_pos: int = 0x22c55e,
    color_neg: int = 0xef4444,
    color_total: int = 0x6366f1,
    palette: list[int] | None = None,
    background: str | None = None,
    gridlines: bool = True,
) -> Chart
```

Aliases: `sp.waterfall`

---

## Description

Graphique en cascade montrant les contributions positives et négatives séquentielles à un total cumulatif. Les valeurs positives montent, les négatives descendent. La dernière barre représente généralement le total final.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des étapes |
| `values` | `list[float]` | requis | Valeurs des étapes (positives ou négatives) |
| `show_text` | `bool` | `True` | Afficher les valeurs sur les barres |
| `color_pos` | `int` | `0x22c55e` | Couleur des barres positives |
| `color_neg` | `int` | `0xef4444` | Couleur des barres négatives |
| `color_total` | `int` | `0x6366f1` | Couleur des barres total |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `480` | Hauteur du canvas |
| `y_label` | `str` | `""` | Étiquette de l'axe Y |
| `gridlines` | `bool` | `True` | Lignes de grille horizontales |

---

## Retourne

`Chart`

---

## Exemples

### Compte de résultat

```python
import seraplot as sp

chart = sp.build_waterfall(
    "Cascade Résultat annuel",
    labels=["Chiffre d'affaires", "Coût des ventes", "Marge brute", "Charges", "EBITDA", "D&A", "Résultat net"],
    values=[100000, -45000, 0, -30000, 0, -5000, 0],
    show_text=True,
    y_label="€",
)
```

---

## Voir aussi

- [Graphique en barres](bar.md)
- [Entonnoir](funnel.md)
- [Bullet](bullet.md)

</div>
