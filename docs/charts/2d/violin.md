# Violin Chart

<div class="lang-en">

## Signature

```python
sp.build_violin(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    background: str | None = None,
    gridlines: bool = True,
    bandwidth: float = 1.0,
) -> Chart
```

---

## Description

Violin chart combining KDE density estimation with box-plot summary.
The mirrored shape shows the full probability distribution of each group.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `categories` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Flat list of all category samples |
| `color_hex` | `int` | `0x6366F1` | Single fill color |
| `palette` | `list[int] \| None` | `None` | Per-category colors |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Horizontal gridlines |
| `bandwidth` | `float` | `1.0` | KDE smoothing bandwidth multiplier |

---

## Returns

`Chart`

---

## Examples

### Comparing salary distributions





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
<div class="sp-tabs" id="violin">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('violin','violin-py',this)">Python</button><button class="sp-tb" onclick="spTab('violin','violin-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('violin','violin-ts',this)">TypeScript</button></div>
<div id="violin-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random

roles = {
    "Engineer": [random.gauss(95000, 15000) for _ in range(60)],
    "Manager":  [random.gauss(110000, 20000) for _ in range(60)],
    "Analyst":  [random.gauss(75000, 12000) for _ in range(60)],
}

chart = sp.build_violin(
    "Salary Distribution by Role",
    categories=list(roles.keys()),
    values=[v for g in roles.values() for v in g],
    y_label="Salary ($)",
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e],
)</code></pre></div>
<div id="violin-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import random

const roles = {
    "Engineer": [random.gauss(95000, 15000) for _ in range(60)],
    "Manager":  [random.gauss(110000, 20000) for _ in range(60)],
    "Analyst":  [random.gauss(75000, 12000) for _ in range(60)],
}

const chart = sp.build_violin("Salary Distribution by Role",
list(roles.keys()),
{
    values: [v for g in roles.values() for v in g],
    y_label: "Salary ($)",
    palette: [0x6366f1, 0x22d3ee, 0xf43f5e]
})</code></pre></div>
<div id="violin-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import random

const roles = {
    "Engineer": [random.gauss(95000, 15000) for _ in range(60)],
    "Manager":  [random.gauss(110000, 20000) for _ in range(60)],
    "Analyst":  [random.gauss(75000, 12000) for _ in range(60)],
}

const chart = sp.build_violin("Salary Distribution by Role",
list(roles.keys()),
{
    values: [v for g in roles.values() for v in g],
    y_label: "Salary ($)",
    palette: [0x6366f1, 0x22d3ee, 0xf43f5e]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/violin.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Box Plot](boxplot.md)
- [KDE](kde.md)
- [Ridgeline](ridgeline.md)
- [Violin 3D](../3d/violin3d.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_violin(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    background: str | None = None,
    gridlines: bool = True,
    bandwidth: float = 1.0,
) -> Chart
```

---

## Description

Graphique en violon combinant estimation KDE et résumé de boîte à moustaches. La forme en miroir montre la distribution complète de chaque groupe.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `categories` | `list[str]` | requis | Étiquettes des catégories |
| `values` | `list[float]` | requis | Liste plate de tous les échantillons par catégorie |
| `color_hex` | `int` | `0x6366F1` | Couleur de remplissage unique |
| `palette` | `list[int] \| None` | `None` | Couleurs par catégorie |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `480` | Hauteur du canvas |
| `x_label` | `str` | `""` | Étiquette de l'axe X |
| `y_label` | `str` | `""` | Étiquette de l'axe Y |
| `gridlines` | `bool` | `True` | Lignes de grille horizontales |
| `bandwidth` | `float` | `1.0` | Multiplicateur de bande passante KDE |

---

## Retourne

`Chart`

---

## Exemples

### Comparaison des distributions de salaires

```python
import seraplot as sp
import random

postes = {
    "Ingénieur":  [random.gauss(95000, 15000) for _ in range(60)],
    "Manager":    [random.gauss(110000, 20000) for _ in range(60)],
    "Analyste":   [random.gauss(75000, 12000) for _ in range(60)],
}

chart = sp.build_violin(
    "Distribution des salaires par poste",
    categories=list(postes.keys()),
    values=[v for g in postes.values() for v in g],
    y_label="Salaire (€)",
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e],
)
```

---

## Voir aussi

- [Boîte à moustaches](boxplot.md)
- [KDE](kde.md)
- [Ridgeline](ridgeline.md)
- [Violon 3D](../3d/violin3d.md)

</div>
