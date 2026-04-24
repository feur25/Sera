# Histogram Overlay

<div class="lang-en">

## Signature

```python
sp.build_histogram_overlay(
    title: str,
    values: list[float],
    overlay_values: list[float],
    *,
    color_hex: int = 0,
    overlay_color_hex: int = 0,
    bins: int = 20,
    series_names: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.histogram_overlay`

---

## Description

Overlaid histogram comparing two distributions side-by-side with transparency.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `values` | `list[float]` | required | First distribution data |
| `overlay_values` | `list[float]` | required | Second distribution data |
| `bins` | `int` | `20` | Number of bins |
| `series_names` | `list[str] \| None` | `None` | Names for legend `["Series A", "Series B"]` |
| `color_hex` | `int` | `0` | Color for first series |
| `overlay_color_hex` | `int` | `0` | Color for second series |

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
<div class="sp-tabs" id="histogram-overlay">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('histogram-overlay','histogram-overlay-py',this)">Python</button><button class="sp-tb" onclick="spTab('histogram-overlay','histogram-overlay-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('histogram-overlay','histogram-overlay-ts',this)">TypeScript</button></div>
<div id="histogram-overlay-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import numpy as np
rng = np.random.default_rng(42)
control = rng.normal(5.0, 1.0, 1000).tolist()
treatment = rng.normal(5.8, 1.2, 1000).tolist()
chart = sp.build_histogram_overlay(
    "Control vs Treatment",
    values=control,
    overlay_values=treatment,
    bins=30,
    series_names=["Control", "Treatment"],
    x_label="Measurement",
    y_label="Frequency",
    gridlines=True,
)</code></pre></div>
<div id="histogram-overlay-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
function randn() {
  const u = 1 - Math.random(), v = Math.random();
  return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * v);
}
const control  = Array.from({ length: 1000 }, () => 5.0 + 1.0 * randn());
const treatment = Array.from({ length: 1000 }, () => 5.8 + 1.2 * randn());
const chart = sp.build_histogram_overlay("Control vs Treatment", control, {
    overlay_values: treatment,
    bins: 30,
    series_names: ["Control", "Treatment"],
    x_label: "Measurement",
    y_label: "Frequency",
    gridlines: true,
});</code></pre></div>
<div id="histogram-overlay-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
function randn(): number {
  const u = 1 - Math.random(), v = Math.random();
  return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * v);
}
const control: number[]   = Array.from({ length: 1000 }, () => 5.0 + 1.0 * randn());
const treatment: number[] = Array.from({ length: 1000 }, () => 5.8 + 1.2 * randn());
const chart = sp.build_histogram_overlay("Control vs Treatment", control, {
    overlay_values: treatment,
    bins: 30,
    series_names: ["Control", "Treatment"],
    x_label: "Measurement",
    y_label: "Frequency",
    gridlines: true,
});</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/histogram-overlay.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Histogram](histogram.md)
- [KDE](kde.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_histogram_overlay(
    title: str,
    values: list[float],
    overlay_values: list[float],
    *,
    color_hex: int = 0,
    overlay_color_hex: int = 0,
    bins: int = 20,
    series_names: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.histogram_overlay`

---

## Description

Histogramme superposé pour comparer deux distributions avec transparence.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `values` | `list[float]` | requis | Données de la première distribution |
| `overlay_values` | `list[float]` | requis | Données de la deuxième distribution |
| `bins` | `int` | `20` | Nombre de classes |
| `series_names` | `list[str] \| None` | `None` | Noms pour la légende `["Série A", "Série B"]` |
| `color_hex` | `int` | `0` | Couleur de la première série |
| `overlay_color_hex` | `int` | `0` | Couleur de la deuxième série |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp
import numpy as np

rng = np.random.default_rng(42)
controle   = rng.normal(5.0, 1.0, 1000).tolist()
traitement = rng.normal(5.8, 1.2, 1000).tolist()

chart = sp.build_histogram_overlay(
    "Contrôle vs Traitement",
    values=controle,
    overlay_values=traitement,
    bins=30,
    series_names=["Contrôle", "Traitement"],
    x_label="Mesure",
    y_label="Fréquence",
    gridlines=True,
)
```

---

## Voir aussi

- [Histogramme](histogram.md)
- [KDE](kde.md)

</div>
