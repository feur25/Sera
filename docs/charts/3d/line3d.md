# Line Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_line3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    series_names: list[str] | None = None,
    show_points: bool = True,
) -> Chart
```

Aliases: `sp.line3d`

---

## Description

3D line chart connecting sequential points in 3D space.
Useful for trajectories, time-series in 3D space, and parametric curves.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | X coordinates |
| `y` | `list[float]` | required | Y coordinates |
| `z` | `list[float]` | required | Z coordinates |
| `color_hex` | `int` | `0x6366F1` | Line color |
| `palette` | `list[int] \| None` | `None` | Multi-series colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `x_label` | `str` | `"X"` | X-axis label |
| `y_label` | `str` | `"Y"` | Y-axis label |
| `z_label` | `str` | `"Z"` | Z-axis label |
| `series_names` | `list[str] \| None` | `None` | Series legend names |
| `show_points` | `bool` | `True` | Show point markers |

---

## Returns

`Chart`

---

## Examples

### Helix trajectory





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
<div class="sp-tabs" id="line3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('line3d','line3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('line3d','line3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('line3d','line3d-ts',this)">TypeScript</button></div>
<div id="line3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import math
t = [i * 0.1 for i in range(100)]
x = [math.cos(v) for v in t]
y = [math.sin(v) for v in t]
z = t
chart = sp.build_line3d_chart(
    "Helix",
    x_values=x, y_values=y, z_values=z,
    x_label="cos(t)", y_label="sin(t)", z_label="t",
)</code></pre></div>
<div id="line3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import math
const t = [i * 0.1 for i in range(100)]
const x = [math.cos(v) for v in t]
const y = [math.sin(v) for v in t]
const z = t
const chart = sp.build_line3d_chart("Helix",
x,
y,
{
    z_values: z,
    x_label: "cos(t)",
    y_label: "sin(t)",
    z_label: "t"
})</code></pre></div>
<div id="line3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import math
const t: number[] = [i * 0.1 for i in range(100)]
const x: number[] = [math.cos(v) for v in t]
const y: number[] = [math.sin(v) for v in t]
const z = t
const chart = sp.build_line3d_chart("Helix",
x,
y,
{
    z_values: z,
    x_label: "cos(t)",
    y_label: "sin(t)",
    z_label: "t"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/line3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Line Chart 2D](../2d/line.md)
- [Scatter 3D](scatter3d.md)

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_line3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    series_names: list[str] | None = None,
    show_points: bool = True,
) -> Chart
```

Aliases: `sp.line3d`

---

<h2>Description</h2>

Graphique en courbe 3D connectant des points séquentiels dans l'espace 3D. Utile pour les trajectoires, les séries temporelles 3D et les courbes paramétriques.

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x` | `list[float]` | requis | Coordonnées X |
| `y` | `list[float]` | requis | Coordonnées Y |
| `z` | `list[float]` | requis | Coordonnées Z |
| `color_hex` | `int` | `0x6366F1` | Couleur de la courbe |
| `palette` | `list[int] \| None` | `None` | Couleurs multi-séries |
| `bg_color` | `str` | `"#1a1a2e"` | Couleur de fond |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `600` | Hauteur du canvas |
| `x_label` | `str` | `"X"` | Étiquette de l'axe X |
| `y_label` | `str` | `"Y"` | Étiquette de l'axe Y |
| `z_label` | `str` | `"Z"` | Étiquette de l'axe Z |
| `series_names` | `list[str] \| None` | `None` | Noms des séries pour la légende |
| `show_points` | `bool` | `True` | Afficher les marqueurs de points |

---

<h2>Retourne</h2>

`Chart`

---

<h2>Exemples</h2>

```python
import seraplot as sp
import math

t = [i * 0.1 for i in range(100)]
x = [math.cos(v) for v in t]
y = [math.sin(v) for v in t]
z = t

chart = sp.build_line3d_chart(
    "Hélice",
    x_values=x, y_values=y, z_values=z,
    x_label="cos(t)", y_label="sin(t)", z_label="t",
)
```

---

<h2>Voir aussi</h2>

- [Courbe 2D](../2d/line.md)
- [Nuage de points 3D](scatter3d.md)

</div>
