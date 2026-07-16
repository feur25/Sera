# Streamtube Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_streamtube_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    u: list[float],
    v: list[float],
    w: list[float],
    *,
    tube_radius: float = 0.3,
    n_steps: int | None = None,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    bg_color: str = "#1a1a2e",
    scene: str | None = None,
    width: int = 900,
    height: int = 560,
) -> Chart
```

Aliases: `sp.streamtube`, `sp.stream_tube`, `sp.streamtube_chart`, `sp.flow_tube`

---

## Description

3D streamtube chart — traces a tube along a sequence of seed points `(x, y, z)` following the local flow direction `(u, v, w)` at each point, visualizing fluid or vector-field flow paths as continuous ribbons rather than discrete arrows (see [Cone 3D](cone.md) for the arrow-based alternative).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | Seed point X positions |
| `y` | `list[float]` | required | Seed point Y positions |
| `z` | `list[float]` | required | Seed point Z positions |
| `u` | `list[float]` | required | X component of the flow direction at each point |
| `v` | `list[float]` | required | Y component of the flow direction at each point |
| `w` | `list[float]` | required | Z component of the flow direction at each point |
| `tube_radius` | `float` | `0.3` | Tube radius |
| `n_steps` | `int \| None` | `None` | Integration steps along the flow (auto if `None`) |
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
<div class="sp-tabs" id="streamtube">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('streamtube','streamtube-py',this)">Python</button><button class="sp-tb" onclick="spTab('streamtube','streamtube-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('streamtube','streamtube-ts',this)">TypeScript</button></div>
<div id="streamtube-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import math
import seraplot as sp

n = 12
x = [0.0] * n
y = [0.0] * n
z = [i * 0.5 for i in range(n)]
u = [math.cos(i * 0.5) for i in range(n)]
v = [math.sin(i * 0.5) for i in range(n)]
w = [1.0] * n

chart = sp.streamtube("Flow Streamtube", x=x, y=y, z=z, u=u, v=v, w=w, tube_radius=0.3)</code></pre></div>
<div id="streamtube-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const n = 12;
const x = Array(n).fill(0);
const y = Array(n).fill(0);
const z = Array.from({length: n}, (_, i) => i * 0.5);
const u = Array.from({length: n}, (_, i) => Math.cos(i * 0.5));
const v = Array.from({length: n}, (_, i) => Math.sin(i * 0.5));
const w = Array(n).fill(1);

const chart = sp.streamtube("Flow Streamtube", x, y, z, u, v, w, { tube_radius: 0.3 });</code></pre></div>
<div id="streamtube-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const n: number = 12;
const x: number[] = Array(n).fill(0);
const y: number[] = Array(n).fill(0);
const z: number[] = Array.from({length: n}, (_, i) => i * 0.5);
const u: number[] = Array.from({length: n}, (_, i) => Math.cos(i * 0.5));
const v: number[] = Array.from({length: n}, (_, i) => Math.sin(i * 0.5));
const w: number[] = Array(n).fill(1);

const chart = sp.streamtube("Flow Streamtube", x, y, z, u, v, w, { tube_radius: 0.3 });</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/streamtube.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_streamtube_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    u: list[float],
    v: list[float],
    w: list[float],
    *,
    tube_radius: float = 0.3,
    n_steps: int | None = None,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    bg_color: str = "#1a1a2e",
    scene: str | None = None,
    width: int = 900,
    height: int = 560,
) -> Chart
```

Alias : `sp.streamtube`, `sp.stream_tube`, `sp.streamtube_chart`, `sp.flow_tube`

---

<h2>Description</h2>

Graphique de tube de flux 3D — trace un tube le long d'une séquence de points de départ `(x, y, z)` en suivant la direction d'écoulement locale `(u, v, w)` à chaque point, pour visualiser les trajectoires d'un flux ou d'un champ vectoriel sous forme de rubans continus plutôt que de flèches discrètes (voir [Cône 3D](cone.md) pour l'alternative à base de flèches).

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `x` | `list[float]` | requis | Positions X des points de départ |
| `y` | `list[float]` | requis | Positions Y des points de départ |
| `z` | `list[float]` | requis | Positions Z des points de départ |
| `u` | `list[float]` | requis | Composante X de la direction d'écoulement à chaque point |
| `v` | `list[float]` | requis | Composante Y de la direction d'écoulement à chaque point |
| `w` | `list[float]` | requis | Composante Z de la direction d'écoulement à chaque point |
| `tube_radius` | `float` | `0.3` | Rayon du tube |
| `n_steps` | `int \| None` | `None` | Pas d'intégration le long du flux (auto si `None`) |
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

n = 12
x = [0.0] * n
y = [0.0] * n
z = [i * 0.5 for i in range(n)]
u = [math.cos(i * 0.5) for i in range(n)]
v = [math.sin(i * 0.5) for i in range(n)]
w = [1.0] * n

chart = sp.streamtube("Tube de flux", x=x, y=y, z=z, u=u, v=v, w=w, tube_radius=0.3)
```

---

</div>
