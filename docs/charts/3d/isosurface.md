# Isosurface Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_isosurface_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    field: list[float],
    *,
    iso_level: float = 0.0,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    bg_color: str = "#1a1a2e",
    scene: str | None = None,
    width: int = 900,
    height: int = 560,
) -> Chart
```

Aliases: `sp.isosurface`, `sp.iso_surface`, `sp.isosurface_chart`, `sp.implicit_surface`

---

## Description

3D isosurface chart — extracts and renders the watertight surface where a scalar field `field` (sampled on the `x` &times; `y` &times; `z` grid, row-major, X fastest) equals `iso_level`, using marching cubes. Useful for visualizing implicit surfaces, level sets, and volumetric thresholds (e.g. medical scan boundaries, distance fields).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | Grid X coordinates |
| `y` | `list[float]` | required | Grid Y coordinates |
| `z` | `list[float]` | required | Grid Z coordinates |
| `field` | `list[float]` | required | Scalar field values, `len(x) * len(y) * len(z)` entries, X varies fastest |
| `iso_level` | `float` | `0.0` | Scalar value defining the extracted surface |
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
<div class="sp-tabs" id="isosurface">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('isosurface','isosurface-py',this)">Python</button><button class="sp-tb" onclick="spTab('isosurface','isosurface-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('isosurface','isosurface-ts',this)">TypeScript</button></div>
<div id="isosurface-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

n = 8
axis = [i / (n - 1) * 4 - 2 for i in range(n)]
field = []
for zc in axis:
    for yc in axis:
        for xc in axis:
            field.append(xc*xc + yc*yc + zc*zc - 3.0)  # sphere of radius sqrt(3)

chart = sp.isosurface("Isosurface Sphere", x=axis, y=axis, z=axis, field=field, iso_level=0.0)</code></pre></div>
<div id="isosurface-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const n = 8;
const axis = Array.from({length: n}, (_, i) => i / (n - 1) * 4 - 2);
const field = [];
for (const zc of axis)
  for (const yc of axis)
    for (const xc of axis)
      field.push(xc*xc + yc*yc + zc*zc - 3.0);

const chart = sp.isosurface("Isosurface Sphere", axis, axis, axis, field, { iso_level: 0.0 });</code></pre></div>
<div id="isosurface-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const n: number = 8;
const axis: number[] = Array.from({length: n}, (_, i) => i / (n - 1) * 4 - 2);
const field: number[] = [];
for (const zc of axis)
  for (const yc of axis)
    for (const xc of axis)
      field.push(xc*xc + yc*yc + zc*zc - 3.0);

const chart = sp.isosurface("Isosurface Sphere", axis, axis, axis, field, { iso_level: 0.0 });</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/isosurface.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_isosurface_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    field: list[float],
    *,
    iso_level: float = 0.0,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    bg_color: str = "#1a1a2e",
    scene: str | None = None,
    width: int = 900,
    height: int = 560,
) -> Chart
```

Alias : `sp.isosurface`, `sp.iso_surface`, `sp.isosurface_chart`, `sp.implicit_surface`

---

<h2>Description</h2>

Graphique d'isosurface 3D — extrait et affiche la surface étanche où un champ scalaire `field` (échantillonné sur la grille `x` &times; `y` &times; `z`, en ordre ligne par ligne, X variant le plus vite) est égal à `iso_level`, via l'algorithme du marching cubes. Utile pour visualiser des surfaces implicites, des lignes de niveau et des seuils volumétriques (par ex. contours d'imagerie médicale, champs de distance).

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x` | `list[float]` | requis | Coordonnées X de la grille |
| `y` | `list[float]` | requis | Coordonnées Y de la grille |
| `z` | `list[float]` | requis | Coordonnées Z de la grille |
| `field` | `list[float]` | requis | Valeurs du champ scalaire, `len(x) * len(y) * len(z)` entrées, X variant le plus vite |
| `iso_level` | `float` | `0.0` | Valeur scalaire définissant la surface extraite |
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

n = 8
axis = [i / (n - 1) * 4 - 2 for i in range(n)]
field = []
for zc in axis:
    for yc in axis:
        for xc in axis:
            field.append(xc*xc + yc*yc + zc*zc - 3.0)  # sphère de rayon sqrt(3)

chart = sp.isosurface("Isosurface sphérique", x=axis, y=axis, z=axis, field=field, iso_level=0.0)
```

---

</div>
