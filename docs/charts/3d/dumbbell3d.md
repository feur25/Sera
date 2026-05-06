# Dumbbell Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_dumbbell3d_chart(
    title: str,
    labels: list[str],
    values_start: list[float],
    values_end: list[float],
    *,
    color_start: int = 0x6366f1,
    color_end: int = 0xf43f5e,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "",
    show_text: bool = False,
) -> Chart
```

Aliases: `sp.dumbbell3d`

---

## Description

Dumbbell chart in 3D — connects start and end spheres with a 3D tube.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels |
| `values_start` | `list[float]` | required | Start values |
| `values_end` | `list[float]` | required | End values |
| `color_start` | `int` | `0x6366f1` | Start sphere color |
| `color_end` | `int` | `0xf43f5e` | End sphere color |
| `bg_color` | `str` | `"#1a1a2e"` | Background |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `show_text` | `bool` | `False` | Show value labels |

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
<div class="sp-tabs" id="dumbbell3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('dumbbell3d','dumbbell3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('dumbbell3d','dumbbell3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('dumbbell3d','dumbbell3d-ts',this)">TypeScript</button></div>
<div id="dumbbell3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.build_dumbbell3d_chart(
    "Before vs After 3D",
    labels=["Group A", "Group B", "Group C"],
    values_start=[40, 55, 70],
    values_end=[60, 75, 85],
)</code></pre></div>
<div id="dumbbell3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.build_dumbbell3d_chart("Before vs After 3D",
["Group A", "Group B", "Group C"],
[40, 55, 70],
{
    values_end: [60, 75, 85]
})</code></pre></div>
<div id="dumbbell3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.build_dumbbell3d_chart("Before vs After 3D",
["Group A", "Group B", "Group C"],
[40, 55, 70],
{
    values_end: [60, 75, 85]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/dumbbell3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Dumbbell 2D](../2d/dumbbell.md)
- [Slope](../2d/slope.md)

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_dumbbell3d_chart(
    title: str,
    labels: list[str],
    values_start: list[float],
    values_end: list[float],
    *,
    color_start: int = 0x6366f1,
    color_end: int = 0xf43f5e,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "",
    show_text: bool = False,
) -> Chart
```

Aliases: `sp.dumbbell3d`

---

<h2>Description</h2>

Graphique haltère 3D — connecte les sphères de début et de fin avec un tube 3D.

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des catégories |
| `values_start` | `list[float]` | requis | Valeurs de départ |
| `values_end` | `list[float]` | requis | Valeurs d'arrivée |
| `color_start` | `int` | `0x6366f1` | Couleur de la sphère de départ |
| `color_end` | `int` | `0xf43f5e` | Couleur de la sphère d'arrivée |
| `bg_color` | `str` | `"#1a1a2e"` | Couleur de fond |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `600` | Hauteur du canvas |
| `show_text` | `bool` | `False` | Afficher les étiquettes de valeur |

---

<h2>Retourne</h2>

`Chart`

---

<h2>Exemples</h2>

```python
import seraplot as sp

chart = sp.build_dumbbell3d_chart(
    "Avant vs Après 3D",
    labels=["Groupe A", "Groupe B", "Groupe C"],
    values_start=[40, 55, 70],
    values_end=[60, 75, 85],
)
```

---

<h2>Voir aussi</h2>

- [Haltère 2D](../2d/dumbbell.md)
- [Pente](../2d/slope.md)

</div>
