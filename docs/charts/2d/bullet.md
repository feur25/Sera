# Bullet Chart

<div class="lang-en">

## Signature

```python
sp.build_bullet(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    targets: list[float] | None = None,
    max_vals: list[float] | None = None,
    ranges: list[list[float]] | None = None,
    show_text: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    background: str | None = None,
    palette: list[int] | None = None,
) -> Chart
```

---

## Description

Bullet chart — a compact bar that shows a value against a target and qualitative ranges (poor / acceptable / good).

Inspired by Tufte's bullet graph design.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Metric labels |
| `values` | `list[float]` | required | Actual measured values |
| `targets` | `list[float] \| None` | `None` | Target lines per metric |
| `max_vals` | `list[float] \| None` | `None` | Scale maximum per metric |
| `ranges` | `list[list[float]] \| None` | `None` | Qualitative ranges `[[poor, ok, good], ...]` |
| `show_text` | `bool` | `True` | Show value labels |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |

---

## Returns

`Chart`

---

## Examples

### KPI dashboard






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
<div class="sp-tabs" id="bullet">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('bullet','bullet-py',this)">Python</button><button class="sp-tb" onclick="spTab('bullet','bullet-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('bullet','bullet-ts',this)">TypeScript</button></div>
<div id="bullet-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_bullet(
    "KPI Dashboard",
    labels=["Revenue", "Satisfaction", "New Users"],
    values=[87500, 4.2, 1340],
    targets=[100000, 4.5, 1500],
    max_vals=[120000, 5.0, 2000],
)
</code></pre></div>
<div id="bullet-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.build_bullet("KPI Dashboard",
["Revenue", "Satisfaction", "New Users"],
{
    values: [87500, 4.2, 1340],
    targets: [100000, 4.5, 1500],
    max_vals: [120000, 5.0, 2000]
})</code></pre></div>
<div id="bullet-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.build_bullet("KPI Dashboard",
["Revenue", "Satisfaction", "New Users"],
{
    values: [87500, 4.2, 1340],
    targets: [100000, 4.5, 1500],
    max_vals: [120000, 5.0, 2000]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/bullet.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Bar Chart](bar.md)
- [Gauge](gauge.md)
- [Waterfall](waterfall.md)

</div>

<div class="lang-fr">

## Description

Graphique en barres compactes montrant une valeur face à une cible et des zones qualitatives (mauvais / acceptable / bon). Inspiré du bullet graph de Tufte.

</div>
