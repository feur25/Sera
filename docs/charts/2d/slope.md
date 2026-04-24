# Slope Chart

<div class="lang-en">

## Signature

```python
sp.build_slope(
    title: str,
    labels: list[str],
    values_left: list[float],
    values_right: list[float],
    left_label: str,
    right_label: str,
    *,
    show_text: bool = True,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 600,
    height: int = 480,
    background: str | None = None,
) -> Chart
```

Aliases: `sp.slope`

---

## Description

Slope chart comparing two values per entity (before/after, period A vs B).
Parallel vertical axes are connected by slope lines — rising or falling.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Entity names (one per line) |
| `values_left` | `list[float]` | required | Values on the left axis |
| `values_right` | `list[float]` | required | Values on the right axis |
| `left_label` | `str` | required | Left axis label (e.g. `"2020"`) |
| `right_label` | `str` | required | Right axis label (e.g. `"2024"`) |
| `show_text` | `bool` | `True` | Show values next to endpoints |
| `color_hex` | `int` | `0x6366F1` | Line color (single) |
| `palette` | `list[int] \| None` | `None` | Per-entity line colors |
| `width` | `int` | `600` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `background` | `str \| None` | `None` | Chart background |

---

## Returns

`Chart`

---

## Examples

### Country ranking change






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
<div class="sp-tabs" id="slope">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('slope','slope-py',this)">Python</button><button class="sp-tb" onclick="spTab('slope','slope-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('slope','slope-ts',this)">TypeScript</button></div>
<div id="slope-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.build_slope(
    "HDI Change 2000 to 2023",
    labels=["Germany", "Japan", "Brazil", "India", "Nigeria"],
    values_left=[0.926, 0.909, 0.694, 0.493, 0.452],
    values_right=[0.950, 0.920, 0.760, 0.644, 0.548],
    left_label="2000",
    right_label="2023",
    show_text=True,
)</code></pre></div>
<div id="slope-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.build_slope("HDI Change 2000 to 2023",
["Germany", "Japan", "Brazil", "India", "Nigeria"],
[0.926, 0.909, 0.694, 0.493, 0.452],
{
    values_right: [0.950, 0.920, 0.760, 0.644, 0.548],
    left_label: "2000",
    right_label: "2023",
    show_text: true
})</code></pre></div>
<div id="slope-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.build_slope("HDI Change 2000 to 2023",
["Germany", "Japan", "Brazil", "India", "Nigeria"],
[0.926, 0.909, 0.694, 0.493, 0.452],
{
    values_right: [0.950, 0.920, 0.760, 0.644, 0.548],
    left_label: "2000",
    right_label: "2023",
    show_text: true
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/slope.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Line Chart](line.md)
- [Dumbbell](dumbbell.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_slope(
    title: str,
    labels: list[str],
    values_left: list[float],
    values_right: list[float],
    left_label: str,
    right_label: str,
    *,
    show_text: bool = True,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 600,
    height: int = 480,
    background: str | None = None,
) -> Chart
```

Aliases: `sp.slope`

---

## Description

Graphique de pente comparant deux valeurs par entité (avant/après, période A vs B). Les axes verticaux parallèles sont reliés par des lignes de pente montantes ou descendantes.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Noms des entités (une par ligne) |
| `values_left` | `list[float]` | requis | Valeurs sur l'axe gauche |
| `values_right` | `list[float]` | requis | Valeurs sur l'axe droit |
| `left_label` | `str` | requis | Titre de l'axe gauche (ex. `"2020"`) |
| `right_label` | `str` | requis | Titre de l'axe droit (ex. `"2024"`) |
| `show_text` | `bool` | `True` | Afficher les valeurs aux extrémités |
| `color_hex` | `int` | `0x6366F1` | Couleur des lignes (unique) |
| `palette` | `list[int] \| None` | `None` | Couleurs par entité |
| `width` | `int` | `600` | Largeur du canvas |
| `height` | `int` | `480` | Hauteur du canvas |
| `background` | `str \| None` | `None` | Couleur de fond |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp

chart = sp.build_slope(
    "IDH : Évolution 2000–2023",
    labels=["Allemagne", "Japon", "Brésil", "Inde", "Nigéria"],
    values_left=[0.926, 0.909, 0.694, 0.493, 0.452],
    values_right=[0.950, 0.920, 0.760, 0.644, 0.548],
    left_label="2000",
    right_label="2023",
    show_text=True,
)
```

---

## Voir aussi

- [Graphique en courbe](line.md)
- [Haltère](dumbbell.md)

</div>
