# Violin Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_violin3d_chart(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    bandwidth: float = 1.0,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "Density",
) -> Chart
```

Aliases: `sp.violin3d`

---

## Description

3D violin chart — KDE-based distribution surfaces per category rendered in WebGL.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `categories` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Flat sample data (equal count per category) |
| `bandwidth` | `float` | `1.0` | KDE bandwidth |
| `palette` | `list[int] \| None` | `None` | Per-category colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |

---

## Returns

`Chart`

---

## Examples





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
<div class="sp-tabs" id="violin3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('violin3d','violin3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('violin3d','violin3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('violin3d','violin3d-ts',this)">TypeScript</button></div>
<div id="violin3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
groups = ["Control", "Treatment A", "Treatment B"]
means  = [50, 65, 72]
values = [v for m in means for v in [random.gauss(m, 8) for _ in range(80)]]
chart = sp.build_violin3d_chart(
    "Trial Results",
    categories=groups,
    values=values,
)</code></pre></div>
<div id="violin3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import random
const groups = ["Control", "Treatment A", "Treatment B"]
const means  = [50, 65, 72]
const values = [v for m in means for v in [random.gauss(m, 8) for _ in range(80)]]
const chart = sp.build_violin3d_chart("Trial Results",
{
    categories: groups,
    values: values
})</code></pre></div>
<div id="violin3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import random
const groups: string[] = ["Control", "Treatment A", "Treatment B"]
const means: number[] = [50, 65, 72]
const values: number[] = [v for m in means for v in [random.gauss(m, 8) for _ in range(80)]]
const chart = sp.build_violin3d_chart("Trial Results",
{
    categories: groups,
    values: values
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/violin3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Violin 2D](../2d/violin.md)
- [KDE 3D](kde3d.md)

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_violin3d_chart(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    bandwidth: float = 1.0,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "Density",
) -> Chart
```

Aliases: `sp.violin3d`

---

<h2>Description</h2>

Graphique en violon 3D — surfaces de distribution basées sur KDE par catégorie rendues en WebGL.

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `categories` | `list[str]` | requis | Étiquettes des catégories |
| `values` | `list[float]` | requis | Données échantillon plates (nombre égal par catégorie) |
| `bandwidth` | `float` | `1.0` | Bande passante KDE |
| `palette` | `list[int] \| None` | `None` | Couleurs par catégorie |
| `bg_color` | `str` | `"#1a1a2e"` | Couleur de fond |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `600` | Hauteur du canvas |

---

<h2>Retourne</h2>

`Chart`

---

<h2>Exemples</h2>

```python
import seraplot as sp
import random

groupes = ["Contrôle", "Traitement A", "Traitement B"]
means  = [50, 65, 72]
values = [v for m in means for v in [random.gauss(m, 8) for _ in range(80)]]

chart = sp.build_violin3d_chart(
    "Résultats de l'essai",
    categories=groupes,
    values=values,
)
```

---

<h2>Voir aussi</h2>

- [Violon 2D](../2d/violin.md)
- [KDE 3D](kde3d.md)

</div>
