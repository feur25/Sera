# Radar Chart

<div class="lang-en">

## Signature

```python
sp.build_radar_chart(
    title: str,
    axes: list[str],
    series: list[list[float]],
    *,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    fill_opacity: float = 0.25,
    width: int = 600,
    height: int = 500,
    background: str | None = None,
    max_val: float | None = None,
) -> Chart
```

---

## Description

Spider / radar chart — polygon per series across radial axes.
Useful for profiling entities across multiple dimensions simultaneously.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `axes` | `list[str]` | required | Axis labels (dimension names) |
| `series` | `list[list[float]]` | required | One inner list per series (same length as `axes`) |
| `series_names` | `list[str] \| None` | `None` | Legend names per series |
| `palette` | `list[int] \| None` | `None` | Series fill colors |
| `fill_opacity` | `float` | `0.25` | Polygon fill opacity (0.0–1.0) |
| `width` | `int` | `600` | Canvas width |
| `height` | `int` | `500` | Canvas height |
| `background` | `str \| None` | `None` | Background color |
| `max_val` | `float \| None` | `None` | Common max scale value (auto if None) |

---

## Returns

`Chart`

---

## Examples

### Player comparison






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
<div class="sp-tabs" id="radar">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('radar','radar-py',this)">Python</button><button class="sp-tb" onclick="spTab('radar','radar-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('radar','radar-ts',this)">TypeScript</button></div>
<div id="radar-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_radar_chart(
    "Player Stats Comparison",
    axes=["Speed", "Strength", "Defense", "Dribbling", "Shooting", "Passing"],
    series_values=[[85, 70, 65, 90, 88, 82], [72, 88, 79, 68, 75, 85]],
    series_names=["Player A", "Player B"],
    palette=[0x6366f1, 0xf43f5e],
)</code></pre></div>
<div id="radar-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.build_radar_chart("Player Stats Comparison",
["Speed", "Strength", "Defense", "Dribbling", "Shooting", "Passing"],
{
    series_values: [[85, 70, 65, 90, 88, 82], [72, 88, 79, 68, 75, 85]],
    series_names: ["Player A", "Player B"],
    palette: [0x6366f1, 0xf43f5e]
})</code></pre></div>
<div id="radar-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.build_radar_chart("Player Stats Comparison",
["Speed", "Strength", "Defense", "Dribbling", "Shooting", "Passing"],
{
    series_values: [[85, 70, 65, 90, 88, 82], [72, 88, 79, 68, 75, 85]],
    series_names: ["Player A", "Player B"],
    palette: [0x6366f1, 0xf43f5e]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/radar.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Parallel Coordinates](parallel.md)
- [Radar 3D](../3d/radar3d.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_radar_chart(
    title: str,
    axes: list[str],
    series: list[list[float]],
    *,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    fill_opacity: float = 0.25,
    width: int = 600,
    height: int = 500,
    background: str | None = None,
    max_val: float | None = None,
) -> Chart
```

---

## Description

Graphique radar (toile d'araignée) — un polygone par série sur des axes radiaux. Idéal pour profiler des entités sur plusieurs dimensions simultanément.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `axes` | `list[str]` | requis | Étiquettes des axes (noms des dimensions) |
| `series` | `list[list[float]]` | requis | Une liste par série (même longueur que `axes`) |
| `series_names` | `list[str] \| None` | `None` | Noms des séries pour la légende |
| `palette` | `list[int] \| None` | `None` | Couleurs de remplissage par série |
| `fill_opacity` | `float` | `0.25` | Opacité du remplissage du polygone (0.0–1.0) |
| `width` | `int` | `600` | Largeur du canvas |
| `height` | `int` | `500` | Hauteur du canvas |
| `background` | `str \| None` | `None` | Couleur de fond |
| `max_val` | `float \| None` | `None` | Valeur maximale commune (auto si `None`) |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp

chart = sp.build_radar_chart(
    "Comparaison des statistiques joueurs",
    axes=["Vitesse", "Force", "Défense", "Dribble", "Tir", "Passe"],
    series=[[85, 70, 65, 90, 88, 82], [72, 88, 79, 68, 75, 85]],
    series_names=["Joueur A", "Joueur B"],
    palette=[0x6366f1, 0xf43f5e],
)
```

---

## Voir aussi

- [Coordonnées parallèles](parallel.md)
- [Radar 3D](../3d/radar3d.md)

</div>
