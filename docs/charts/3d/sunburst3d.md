# Sunburst Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_sunburst3d_chart(
    title: str,
    labels: list[str],
    parents: list[str],
    values: list[float],
    *,
    extrusion: float = 0.15,
    bg_color: str = "#1a1a2e",
    palette: list[int] | None = None,
    width: int = 700,
    height: int = 600,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

3D sunburst chart — concentric extruded arc rings in a WebGL scene.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Node labels |
| `parents` | `list[str]` | required | Parent labels |
| `values` | `list[float]` | required | Node sizes |
| `extrusion` | `float` | `0.15` | Depth of arc extrusion |
| `bg_color` | `str` | `"#1a1a2e"` | Background |
| `palette` | `list[int] \| None` | `None` | Custom palette |
| `width` | `int` | `700` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

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
<div class="sp-tabs" id="sunburst3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('sunburst3d','sunburst3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('sunburst3d','sunburst3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('sunburst3d','sunburst3d-ts',this)">TypeScript</button></div>
<div id="sunburst3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
labels  = ["Root", "A", "B", "A1", "A2", "B1"]
parents = ["", "Root", "Root", "A", "A", "B"]
values  = [1, 40, 60, 25, 15, 60]
chart = sp.build_sunburst3d_chart(
    "Org Chart 3D",
    labels=labels, parents=parents, values=values,
)</code></pre></div>
<div id="sunburst3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const labels  = ["Root", "A", "B", "A1", "A2", "B1"]
const parents = ["", "Root", "Root", "A", "A", "B"]
const values  = [1, 40, 60, 25, 15, 60]
const chart = sp.build_sunburst3d_chart("Org Chart 3D",
labels,
parents,
{
    values: values
})</code></pre></div>
<div id="sunburst3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const labels: string[] = ["Root", "A", "B", "A1", "A2", "B1"]
const parents: string[] = ["", "Root", "Root", "A", "A", "B"]
const values: number[] = [1, 40, 60, 25, 15, 60]
const chart = sp.build_sunburst3d_chart("Org Chart 3D",
labels,
parents,
{
    values: values
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/sunburst3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Sunburst 2D](../2d/sunburst.md)
- [Pie 3D](pie3d.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_sunburst3d_chart(
    title: str,
    labels: list[str],
    parents: list[str],
    values: list[float],
    *,
    extrusion: float = 0.15,
    bg_color: str = "#1a1a2e",
    palette: list[int] | None = None,
    width: int = 700,
    height: int = 600,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Sunburst 3D — anneaux d'arc extrudés concentriques dans une scène WebGL.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des nœuds |
| `parents` | `list[str]` | requis | Étiquettes des parents |
| `values` | `list[float]` | requis | Tailles des nœuds |
| `extrusion` | `float` | `0.15` | Profondeur d'extrusion des arcs |
| `bg_color` | `str` | `"#1a1a2e"` | Couleur de fond |
| `palette` | `list[int] \| None` | `None` | Palette personnalisée |
| `width` | `int` | `700` | Largeur du canvas |
| `height` | `int` | `600` | Hauteur du canvas |
| `hover_json` | `str \| None` | `None` | JSON d'infobulle personnalisée |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp

labels  = ["Racine", "A", "B", "A1", "A2", "B1"]
parents = ["", "Racine", "Racine", "A", "A", "B"]
values  = [1, 40, 60, 25, 15, 60]

chart = sp.build_sunburst3d_chart(
    "Organigramme 3D",
    labels=labels, parents=parents, values=values,
)
```

---

## Voir aussi

- [Sunburst 2D](../2d/sunburst.md)
- [Camembert 3D](pie3d.md)

</div>
