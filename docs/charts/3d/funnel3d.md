# Funnel Chart 3D

<div class="lang-en">

## Signature

```python
sp.build_funnel3d_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 700,
    height: int = 600,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.funnel3d`

---

## Description

3D funnel chart where each stage is a truncated cone (frustum) in a WebGL scene.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Stage labels |
| `values` | `list[float]` | required | Stage values |
| `show_text` | `bool` | `True` | Show value labels |
| `palette` | `list[int] \| None` | `None` | Per-stage colors |
| `bg_color` | `str` | `"#1a1a2e"` | Background |
| `width` | `int` | `700` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

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
<div class="sp-tabs" id="funnel3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('funnel3d','funnel3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('funnel3d','funnel3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('funnel3d','funnel3d-ts',this)">TypeScript</button></div>
<div id="funnel3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.build_funnel3d_chart(
    "Conversion Funnel 3D",
    labels=["Visitors", "Leads", "Opportunities", "Proposals", "Won"],
    values=[10000, 3200, 1100, 450, 120],
)</code></pre></div>
<div id="funnel3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.build_funnel3d_chart("Conversion Funnel 3D",
["Visitors", "Leads", "Opportunities", "Proposals", "Won"],
{
    values: [10000, 3200, 1100, 450, 120]
})</code></pre></div>
<div id="funnel3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.build_funnel3d_chart("Conversion Funnel 3D",
["Visitors", "Leads", "Opportunities", "Proposals", "Won"],
{
    values: [10000, 3200, 1100, 450, 120]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/funnel3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Funnel 2D](../2d/funnel.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_funnel3d_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    palette: list[int] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 700,
    height: int = 600,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.funnel3d`

---

## Description

Entonnoir 3D où chaque étape est un cône tronqué (frustum) dans une scène WebGL.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des étapes |
| `values` | `list[float]` | requis | Valeurs des étapes |
| `show_text` | `bool` | `True` | Afficher les étiquettes de valeur |
| `palette` | `list[int] \| None` | `None` | Couleurs par étape |
| `bg_color` | `str` | `"#1a1a2e"` | Couleur de fond |
| `width` | `int` | `700` | Largeur du canvas |
| `height` | `int` | `600` | Hauteur du canvas |
| `hover_json` | `str \| None` | `None` | JSON d'infobulle personnalisée |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp

chart = sp.build_funnel3d_chart(
    "Entonnoir de conversion 3D",
    labels=["Visiteurs", "Prospects", "Opportunités", "Propositions", "Conclus"],
    values=[10000, 3200, 1100, 450, 120],
)
```

---

## Voir aussi

- [Entonnoir 2D](../2d/funnel.md)

</div>
