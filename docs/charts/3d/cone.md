# Cone Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_cone_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    u: list[float],
    v: list[float],
    w: list[float],
    *,
    cone_size: float = 1.0,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    bg_color: str = "#1a1a2e",
    scene: str | None = None,
    width: int = 900,
    height: int = 560,
) -> Chart
```

Aliases: `sp.cone`, `sp.cone3d`, `sp.cone_3d`, `sp.cone_chart`

---

## Description

3D vector field chart — draws a cone at each `(x, y, z)` position, oriented along the `(u, v, w)` direction vector, for visualizing gradients, flow fields, and force vectors.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | Cone base X positions |
| `y` | `list[float]` | required | Cone base Y positions |
| `z` | `list[float]` | required | Cone base Z positions |
| `u` | `list[float]` | required | X component of the direction vector |
| `v` | `list[float]` | required | Y component of the direction vector |
| `w` | `list[float]` | required | Z component of the direction vector |
| `cone_size` | `float` | `1.0` | Overall cone length scale |
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
<div class="sp-tabs" id="cone">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('cone','cone-py',this)">Python</button><button class="sp-tb" onclick="spTab('cone','cone-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('cone','cone-ts',this)">TypeScript</button></div>
<div id="cone-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

x = [i * 1.5 - 3 for i in range(5)]
y = [0.0] * 5
z = [0.0] * 5
u = [0.0] * 5
v = [0.0] * 5
w = [1.0] * 5

chart = sp.cone("Vector Field Cones", x=x, y=y, z=z, u=u, v=v, w=w, cone_size=1.0)</code></pre></div>
<div id="cone-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const x = [-3, -1.5, 0, 1.5, 3];
const y = [0, 0, 0, 0, 0];
const z = [0, 0, 0, 0, 0];
const u = [0, 0, 0, 0, 0];
const v = [0, 0, 0, 0, 0];
const w = [1, 1, 1, 1, 1];

const chart = sp.cone("Vector Field Cones", x, y, z, u, v, w, { cone_size: 1.0 });</code></pre></div>
<div id="cone-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const x: number[] = [-3, -1.5, 0, 1.5, 3];
const y: number[] = [0, 0, 0, 0, 0];
const z: number[] = [0, 0, 0, 0, 0];
const u: number[] = [0, 0, 0, 0, 0];
const v: number[] = [0, 0, 0, 0, 0];
const w: number[] = [1, 1, 1, 1, 1];

const chart = sp.cone("Vector Field Cones", x, y, z, u, v, w, { cone_size: 1.0 });</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/cone.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_cone_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    u: list[float],
    v: list[float],
    w: list[float],
    *,
    cone_size: float = 1.0,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    bg_color: str = "#1a1a2e",
    scene: str | None = None,
    width: int = 900,
    height: int = 560,
) -> Chart
```

Alias : `sp.cone`, `sp.cone3d`, `sp.cone_3d`, `sp.cone_chart`

---

<h2>Description</h2>

Graphique de champ vectoriel 3D — dessine un cône à chaque position `(x, y, z)`, orienté selon le vecteur direction `(u, v, w)`, pour visualiser des gradients, des champs d'écoulement et des vecteurs de force.

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x` | `list[float]` | requis | Positions X de la base des cônes |
| `y` | `list[float]` | requis | Positions Y de la base des cônes |
| `z` | `list[float]` | requis | Positions Z de la base des cônes |
| `u` | `list[float]` | requis | Composante X du vecteur direction |
| `v` | `list[float]` | requis | Composante Y du vecteur direction |
| `w` | `list[float]` | requis | Composante Z du vecteur direction |
| `cone_size` | `float` | `1.0` | Échelle globale de longueur des cônes |
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

x = [i * 1.5 - 3 for i in range(5)]
y = [0.0] * 5
z = [0.0] * 5
u = [0.0] * 5
v = [0.0] * 5
w = [1.0] * 5

chart = sp.cone("Cônes de champ vectoriel", x=x, y=y, z=z, u=u, v=v, w=w, cone_size=1.0)
```

---

</div>
