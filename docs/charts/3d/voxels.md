# Voxels Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_voxels_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    *,
    voxel_size: float = 0.8,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    bg_color: str = "#1a1a2e",
    scene: str | None = None,
    width: int = 900,
    height: int = 560,
) -> Chart
```

Aliases: `sp.voxels`, `sp.voxel`, `sp.voxels_chart`, `sp.cubes3d`

---

## Description

3D voxel chart — renders a cube at each `(x, y, z)` coordinate, useful for occupancy grids, volumetric data, or discretized point clouds.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | Voxel center X coordinates |
| `y` | `list[float]` | required | Voxel center Y coordinates |
| `z` | `list[float]` | required | Voxel center Z coordinates |
| `voxel_size` | `float` | `0.8` | Edge length of each cube |
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
<div class="sp-tabs" id="voxels">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('voxels','voxels-py',this)">Python</button><button class="sp-tb" onclick="spTab('voxels','voxels-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('voxels','voxels-ts',this)">TypeScript</button></div>
<div id="voxels-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

x, y, z = [], [], []
for i in range(3):
    for j in range(3):
        for k in range(2):
            if (i + j + k) % 2 == 0:
                x.append(i * 1.2)
                y.append(j * 1.2)
                z.append(k * 1.2)

chart = sp.voxels("Voxel Grid", x=x, y=y, z=z, voxel_size=1.0)</code></pre></div>
<div id="voxels-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const x = [], y = [], z = [];
for (let i = 0; i < 3; i++)
  for (let j = 0; j < 3; j++)
    for (let k = 0; k < 2; k++)
      if ((i + j + k) % 2 === 0) { x.push(i*1.2); y.push(j*1.2); z.push(k*1.2); }

const chart = sp.voxels("Voxel Grid", x, y, z, { voxel_size: 1.0 });</code></pre></div>
<div id="voxels-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const x: number[] = [], y: number[] = [], z: number[] = [];
for (let i = 0; i < 3; i++)
  for (let j = 0; j < 3; j++)
    for (let k = 0; k < 2; k++)
      if ((i + j + k) % 2 === 0) { x.push(i*1.2); y.push(j*1.2); z.push(k*1.2); }

const chart = sp.voxels("Voxel Grid", x, y, z, { voxel_size: 1.0 });</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/voxels.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_voxels_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    *,
    voxel_size: float = 0.8,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    bg_color: str = "#1a1a2e",
    scene: str | None = None,
    width: int = 900,
    height: int = 560,
) -> Chart
```

Alias : `sp.voxels`, `sp.voxel`, `sp.voxels_chart`, `sp.cubes3d`

---

<h2>Description</h2>

Graphique de voxels 3D — affiche un cube à chaque coordonnée `(x, y, z)`, utile pour les grilles d'occupation, les données volumétriques ou les nuages de points discrétisés.

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x` | `list[float]` | requis | Coordonnées X des centres de voxels |
| `y` | `list[float]` | requis | Coordonnées Y des centres de voxels |
| `z` | `list[float]` | requis | Coordonnées Z des centres de voxels |
| `voxel_size` | `float` | `0.8` | Longueur d'arête de chaque cube |
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

x, y, z = [], [], []
for i in range(3):
    for j in range(3):
        for k in range(2):
            if (i + j + k) % 2 == 0:
                x.append(i * 1.2)
                y.append(j * 1.2)
                z.append(k * 1.2)

chart = sp.voxels("Grille de voxels", x=x, y=y, z=z, voxel_size=1.0)
```

---

</div>
