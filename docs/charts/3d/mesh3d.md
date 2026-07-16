# Mesh Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_mesh3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    mesh_i: list[int],
    mesh_j: list[int],
    mesh_k: list[int],
    *,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    bg_color: str = "#1a1a2e",
    scene: str | None = None,
    width: int = 900,
    height: int = 560,
) -> Chart
```

Aliases: `sp.mesh3d`, `sp.mesh_3d`, `sp.trimesh3d`, `sp.mesh3d_chart`

---

## Description

Arbitrary triangle mesh chart — renders any 3D shape from an explicit vertex list (`x`, `y`, `z`) and triangle index triplets (`mesh_i`, `mesh_j`, `mesh_k`), one triangle per aligned entry. Use this when the geometry isn't a regular grid (unlike [Surface 3D](surface3d.md)).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | Vertex X coordinates |
| `y` | `list[float]` | required | Vertex Y coordinates |
| `z` | `list[float]` | required | Vertex Z coordinates |
| `mesh_i` | `list[int]` | required | First vertex index of each triangle |
| `mesh_j` | `list[int]` | required | Second vertex index of each triangle |
| `mesh_k` | `list[int]` | required | Third vertex index of each triangle |
| `x_label` | `str` | `"X"` | X axis label |
| `y_label` | `str` | `"Y"` | Y axis label |
| `z_label` | `str` | `"Z"` | Z axis label |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `560` | Canvas height |

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
<div class="sp-tabs" id="mesh3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('mesh3d','mesh3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('mesh3d','mesh3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('mesh3d','mesh3d-ts',this)">TypeScript</button></div>
<div id="mesh3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

x = [0.0, 1.0, 0.0, 1.0, 0.5]
y = [0.0, 0.0, 1.0, 1.0, 0.5]
z = [0.0, 0.0, 0.0, 0.0, 1.2]
mesh_i = [0, 0, 1, 2]
mesh_j = [1, 2, 3, 3]
mesh_k = [4, 4, 4, 0]

chart = sp.mesh3d("Pyramid Mesh", x=x, y=y, z=z, mesh_i=mesh_i, mesh_j=mesh_j, mesh_k=mesh_k)</code></pre></div>
<div id="mesh3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const x = [0.0, 1.0, 0.0, 1.0, 0.5];
const y = [0.0, 0.0, 1.0, 1.0, 0.5];
const z = [0.0, 0.0, 0.0, 0.0, 1.2];
const meshI = [0, 0, 1, 2];
const meshJ = [1, 2, 3, 3];
const meshK = [4, 4, 4, 0];

const chart = sp.mesh3d("Pyramid Mesh", x, y, z, meshI, meshJ, meshK);</code></pre></div>
<div id="mesh3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const x: number[] = [0.0, 1.0, 0.0, 1.0, 0.5];
const y: number[] = [0.0, 0.0, 1.0, 1.0, 0.5];
const z: number[] = [0.0, 0.0, 0.0, 0.0, 1.2];
const meshI: number[] = [0, 0, 1, 2];
const meshJ: number[] = [1, 2, 3, 3];
const meshK: number[] = [4, 4, 4, 0];

const chart = sp.mesh3d("Pyramid Mesh", x, y, z, meshI, meshJ, meshK);</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/mesh3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_mesh3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    mesh_i: list[int],
    mesh_j: list[int],
    mesh_k: list[int],
    *,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    bg_color: str = "#1a1a2e",
    scene: str | None = None,
    width: int = 900,
    height: int = 560,
) -> Chart
```

Alias : `sp.mesh3d`, `sp.mesh_3d`, `sp.trimesh3d`, `sp.mesh3d_chart`

---

<h2>Description</h2>

Graphique de maillage triangulaire arbitraire — affiche n'importe quelle forme 3D à partir d'une liste explicite de sommets (`x`, `y`, `z`) et de triplets d'indices de triangles (`mesh_i`, `mesh_j`, `mesh_k`), un triangle par entrée alignée. À utiliser quand la géométrie n'est pas une grille régulière (contrairement à [Surface 3D](surface3d.md)).

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x` | `list[float]` | requis | Coordonnées X des sommets |
| `y` | `list[float]` | requis | Coordonnées Y des sommets |
| `z` | `list[float]` | requis | Coordonnées Z des sommets |
| `mesh_i` | `list[int]` | requis | Premier indice de sommet de chaque triangle |
| `mesh_j` | `list[int]` | requis | Deuxième indice de sommet de chaque triangle |
| `mesh_k` | `list[int]` | requis | Troisième indice de sommet de chaque triangle |
| `x_label` | `str` | `"X"` | Étiquette de l'axe X |
| `y_label` | `str` | `"Y"` | Étiquette de l'axe Y |
| `z_label` | `str` | `"Z"` | Étiquette de l'axe Z |
| `bg_color` | `str` | `"#1a1a2e"` | Couleur de fond |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `560` | Hauteur du canvas |

---

<h2>Retourne</h2>

`Chart`

---

<h2>Exemples</h2>

```python
import seraplot as sp

x = [0.0, 1.0, 0.0, 1.0, 0.5]
y = [0.0, 0.0, 1.0, 1.0, 0.5]
z = [0.0, 0.0, 0.0, 0.0, 1.2]
mesh_i = [0, 0, 1, 2]
mesh_j = [1, 2, 3, 3]
mesh_k = [4, 4, 4, 0]

chart = sp.mesh3d("Maillage pyramide", x=x, y=y, z=z, mesh_i=mesh_i, mesh_j=mesh_j, mesh_k=mesh_k)
```

---

</div>
