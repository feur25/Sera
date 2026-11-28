# Wireframe Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_wireframe3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    matrix: list[list[float]],
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

Aliases: `sp.wireframe3d`, `sp.wireframe_3d`, `sp.wireframe`, `sp.wireframe3d_chart`

---

## Description

3D wireframe chart — the edge-only counterpart of [Surface 3D](surface3d.md): renders a `z = f(x, y)` height grid as a mesh outline without shaded faces, useful when overlapping surfaces need to stay legible.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | Grid column coordinates |
| `y` | `list[float]` | required | Grid row coordinates |
| `matrix` | `list[list[float]]` | required | Height grid, one row per `y` value |
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
<div class="sp-tabs" id="wireframe3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('wireframe3d','wireframe3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('wireframe3d','wireframe3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('wireframe3d','wireframe3d-ts',this)">TypeScript</button></div>
<div id="wireframe3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import math
import seraplot as sp

n = 24
xs = [i / (n - 1) * 6 - 3 for i in range(n)]
ys = [i / (n - 1) * 6 - 3 for i in range(n)]
matrix = [[math.sin(math.sqrt(x*x + y*y)) * 2.0 for x in xs] for y in ys]

chart = sp.wireframe3d("Wireframe Wave", x=xs, y=ys, matrix=matrix)</code></pre></div>
<div id="wireframe3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const n = 24;
const xs = Array.from({length: n}, (_, i) => i / (n - 1) * 6 - 3);
const ys = Array.from({length: n}, (_, i) => i / (n - 1) * 6 - 3);
const matrix = ys.map(y => xs.map(x => Math.sin(Math.sqrt(x*x + y*y)) * 2.0));

const chart = sp.wireframe3d("Wireframe Wave", xs, ys, matrix);</code></pre></div>
<div id="wireframe3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const n: number = 24;
const xs: number[] = Array.from({length: n}, (_, i) => i / (n - 1) * 6 - 3);
const ys: number[] = Array.from({length: n}, (_, i) => i / (n - 1) * 6 - 3);
const matrix: number[][] = ys.map(y => xs.map(x => Math.sin(Math.sqrt(x*x + y*y)) * 2.0));

const chart = sp.wireframe3d("Wireframe Wave", xs, ys, matrix);</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/wireframe3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_wireframe3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    matrix: list[list[float]],
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

Alias : `sp.wireframe3d`, `sp.wireframe_3d`, `sp.wireframe`, `sp.wireframe3d_chart`

---

<h2>Description</h2>

Graphique en fil de fer 3D — la variante « arêtes seules » de [Surface 3D](surface3d.md) : affiche une grille de hauteurs `z = f(x, y)` sous forme de contour de maillage sans faces ombrées, utile quand des surfaces superposées doivent rester lisibles.

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x` | `list[float]` | requis | Coordonnées des colonnes de la grille |
| `y` | `list[float]` | requis | Coordonnées des lignes de la grille |
| `matrix` | `list[list[float]]` | requis | Grille de hauteurs, une ligne par valeur de `y` |
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
import math
import seraplot as sp

n = 24
xs = [i / (n - 1) * 6 - 3 for i in range(n)]
ys = [i / (n - 1) * 6 - 3 for i in range(n)]
matrix = [[math.sin(math.sqrt(x*x + y*y)) * 2.0 for x in xs] for y in ys]

chart = sp.wireframe3d("Vague en fil de fer", x=xs, y=ys, matrix=matrix)
```

---

</div>
