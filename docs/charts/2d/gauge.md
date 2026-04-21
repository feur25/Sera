# Gauge Chart

<div class="lang-en">

## Signature

```python
sp.build_gauge(
    title: str,
    value: float,
    *,
    min_val: float = 0.0,
    max_val: float = 100.0,
    thresholds: list[float] | None = None,
    threshold_colors: list[int] | None = None,
    color_hex: int = 0x6366F1,
    width: int = 500,
    height: int = 350,
    show_value: bool = True,
    background: str | None = None,
    label: str = "",
) -> Chart
```

---

## Description

Semicircular gauge (speedometer) chart. Colored arcs can indicate thresholds (danger / warning / ok).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `value` | `float` | required | Current reading |
| `min_val` | `float` | `0.0` | Scale minimum |
| `max_val` | `float` | `100.0` | Scale maximum |
| `thresholds` | `list[float] \| None` | `None` | Zone boundaries within [min,max] |
| `threshold_colors` | `list[int] \| None` | `None` | Colors for each threshold zone |
| `color_hex` | `int` | `0x6366F1` | Needle color |
| `width` | `int` | `500` | Canvas width |
| `height` | `int` | `350` | Canvas height |
| `show_value` | `bool` | `True` | Display numeric value below needle |
| `label` | `str` | `""` | Unit label under the value |

---

## Returns

`Chart`

---

## Examples

### CPU usage gauge






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
<div class="sp-tabs" id="gauge">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('gauge','gauge-py',this)">Python</button><button class="sp-tb" onclick="spTab('gauge','gauge-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('gauge','gauge-ts',this)">TypeScript</button></div>
<div id="gauge-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.build_gauge(
    "CPU Usage",
    value=73.5,
    min_val=0,
    max_val=100,
    label="%",
)</code></pre></div>
<div id="gauge-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.build_gauge("CPU Usage",
{
    value: 73.5,
    min_val: 0,
    max_val: 100,
    label: "%"
})</code></pre></div>
<div id="gauge-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.build_gauge("CPU Usage",
{
    value: 73.5,
    min_val: 0,
    max_val: 100,
    label: "%"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/gauge.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Bullet](bullet.md)
- [Bar Chart](bar.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_gauge(
    title: str,
    value: float,
    *,
    min_val: float = 0.0,
    max_val: float = 100.0,
    thresholds: list[float] | None = None,
    threshold_colors: list[int] | None = None,
    color_hex: int = 0x6366F1,
    width: int = 500,
    height: int = 350,
    show_value: bool = True,
    background: str | None = None,
    label: str = "",
) -> Chart
```

---

## Description

Jauge semicirculaire (tachymètre). Les arcs colorés indiquent les seuils (danger / avertissement / ok).

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `value` | `float` | requis | Valeur actuelle |
| `min_val` | `float` | `0.0` | Minimum de l'échelle |
| `max_val` | `float` | `100.0` | Maximum de l'échelle |
| `thresholds` | `list[float] \| None` | `None` | Limites de zones dans [min,max] |
| `threshold_colors` | `list[int] \| None` | `None` | Couleurs par zone de seuil |
| `color_hex` | `int` | `0x6366F1` | Couleur de l'aiguille |
| `width` | `int` | `500` | Largeur du canvas |
| `height` | `int` | `350` | Hauteur du canvas |
| `show_value` | `bool` | `True` | Afficher la valeur numérique sous l'aiguille |
| `label` | `str` | `""` | Étiquette d'unité sous la valeur |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp

chart = sp.build_gauge(
    "Utilisation CPU",
    value=73.5,
    min_val=0,
    max_val=100,
    label="%",
)
```

---

## Voir aussi

- [Bullet](bullet.md)
- [Graphique en barres](bar.md)

</div>
