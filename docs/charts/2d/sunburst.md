# Sunburst Chart

<div class="lang-en">

## Signature

```python
sp.build_sunburst(
    title: str,
    labels: list[str],
    parents: list[str],
    values: list[float],
    *,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.sunburst`

---

## Description

Hierarchical sunburst chart. Nodes are arranged in concentric rings
radiating outward from the root.

The `labels` and `parents` lists define a tree: each entry in `labels[i]` has
`parents[i]` as its parent (`""` for root nodes). `values` controls arc size.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Node labels |
| `parents` | `list[str]` | required | Parent label per node (`""` = root) |
| `values` | `list[float]` | required | Node size values |
| `width` | `int` | `700` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `palette` | `list[int] \| None` | `None` | Custom colors |
| `background` | `str \| None` | `None` | Background color |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### Company org chart






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
<div class="sp-tabs" id="sunburst">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('sunburst','sunburst-py',this)">Python</button><button class="sp-tb" onclick="spTab('sunburst','sunburst-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('sunburst','sunburst-ts',this)">TypeScript</button></div>
<div id="sunburst-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
labels  = ["Corp", "Sales", "Tech", "HR", "B2B", "B2C", "Frontend", "Backend"]
parents = ["",       "Corp", "Corp","Corp","Sales","Sales","Tech",    "Tech"]
values  = [1,        40,     50,    10,    25,     15,     30,        20]
chart = sp.build_sunburst(
    "Headcount by Department",
    labels=labels,
    parents=parents,
    values=values,
)</code></pre></div>
<div id="sunburst-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const labels  = ["Corp", "Sales", "Tech", "HR", "B2B", "B2C", "Frontend", "Backend"]
const parents = ["",       "Corp", "Corp","Corp","Sales","Sales","Tech",    "Tech"]
const values  = [1,        40,     50,    10,    25,     15,     30,        20]
const chart = sp.build_sunburst("Headcount by Department",
labels,
parents,
{
    values: values
})</code></pre></div>
<div id="sunburst-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const labels: string[] = ["Corp", "Sales", "Tech", "HR", "B2B", "B2C", "Frontend", "Backend"]
const parents: string[] = ["",       "Corp", "Corp","Corp","Sales","Sales","Tech",    "Tech"]
const values: number[] = [1,        40,     50,    10,    25,     15,     30,        20]
const chart = sp.build_sunburst("Headcount by Department",
labels,
parents,
{
    values: values
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/sunburst.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Treemap](treemap.md)
- [Pie Chart](pie.md)
- [Sunburst 3D](../3d/sunburst3d.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_sunburst(
    title: str,
    labels: list[str],
    parents: list[str],
    values: list[float],
    *,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.sunburst`

---

## Description

Graphique sunburst hiérarchique. Les nœuds sont disposés en anneaux concentriques rayonnant depuis la racine.

`labels[i]` a `parents[i]` comme parent (`""` pour les nœuds racine). `values` contrôle la taille des arcs.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des nœuds |
| `parents` | `list[str]` | requis | Parent de chaque nœud (`""` = racine) |
| `values` | `list[float]` | requis | Valeurs de taille des nœuds |
| `width` | `int` | `700` | Largeur du canvas |
| `height` | `int` | `480` | Hauteur du canvas |
| `palette` | `list[int] \| None` | `None` | Couleurs personnalisées |
| `background` | `str \| None` | `None` | Couleur de fond |
| `hover_json` | `str \| None` | `None` | JSON d'infobulle personnalisée |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp

labels  = ["Société", "Ventes", "Tech", "RH", "B2B", "B2C", "Frontend", "Backend"]
parents = ["",         "Société", "Société", "Société", "Ventes", "Ventes", "Tech", "Tech"]
values  = [1,           40,       50,     10,   25,     15,     30,        20]

chart = sp.build_sunburst(
    "Effectifs par département",
    labels=labels,
    parents=parents,
    values=values,
)
```

---

## Voir aussi

- [Treemap](treemap.md)
- [Camembert](pie.md)
- [Sunburst 3D](../3d/sunburst3d.md)

</div>
