# Slideshow

<div class="lang-en">

## Signature

```python
sp.build_slideshow(
    charts: list[Chart],
    *,
    title: str = "",
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    autoplay: bool = False,
    interval_ms: int = 3000,
) -> Chart
```

---

## Description

Wraps multiple charts in an interactive slideshow with Previous / Next navigation controls.
All charts are pre-rendered; switching slides requires no server round-trip.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `charts` | `list[Chart]` | required | Chart objects to display |
| `title` | `str` | `""` | Slideshow title |
| `width` | `int` | `1000` | Container width in pixels |
| `height` | `int` | `600` | Container height in pixels |
| `background` | `str \| None` | `None` | Background color |
| `autoplay` | `bool` | `False` | Auto-advance slides |
| `interval_ms` | `int` | `3000` | Auto-advance interval in milliseconds |

---

## Returns

`Chart` (composite)

---

## Examples

### Quarterly report slideshow






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
<div class="sp-tabs" id="slideshow">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('slideshow','slideshow-py',this)">Python</button><button class="sp-tb" onclick="spTab('slideshow','slideshow-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('slideshow','slideshow-ts',this)">TypeScript</button></div>
<div id="slideshow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
slides = [
    sp.build_bar_chart("Q1 Revenue", labels=["A","B","C"], values=[120,80,95]),
    sp.build_line_chart("Growth Trend", labels=["Jan","Feb","Mar"], values=[10,14,18]),
    sp.build_pie_chart("Market Share", labels=["Us","Them"], values=[55,45]),
]
deck = sp.build_slideshow(slides, title="Q1 Board Deck")</code></pre></div>
<div id="slideshow-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const slides = [
    sp.build_bar_chart("Q1 Revenue",
["A", "B", "C"],
{
    values: [120, 80, 95]
}),
    sp.build_line_chart("Growth Trend",
["Jan", "Feb", "Mar"],
{
    values: [10, 14, 18]
}),
    sp.build_pie_chart("Market Share",
["Us", "Them"],
{
    values: [55, 45]
}),
]
const deck = sp.build_slideshow(slides,
"Q1 Board Deck")</code></pre></div>
<div id="slideshow-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const slides: number[] = [
    sp.build_bar_chart("Q1 Revenue",
["A", "B", "C"],
{
    values: [120, 80, 95]
}),
    sp.build_line_chart("Growth Trend",
["Jan", "Feb", "Mar"],
{
    values: [10, 14, 18]
}),
    sp.build_pie_chart("Market Share",
["Us", "Them"],
{
    values: [55, 45]
}),
]
const deck = sp.build_slideshow(slides,
"Q1 Board Deck")</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/slideshow.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Grid Layout](grid.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_slideshow(
    charts: list[Chart],
    *,
    title: str = "",
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    autoplay: bool = False,
    interval_ms: int = 3000,
) -> Chart
```

---

## Description

Emballe plusieurs graphiques dans un diaporama interactif avec navigation Précédent/Suivant. Tous les graphiques sont pré-rendus ; la navigation ne nécessite aucun aller-retour serveur.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `charts` | `list[Chart]` | requis | Objets Chart à afficher |
| `title` | `str` | `""` | Titre du diaporama |
| `width` | `int` | `1000` | Largeur du conteneur en pixels |
| `height` | `int` | `600` | Hauteur du conteneur en pixels |
| `background` | `str \| None` | `None` | Couleur de fond |
| `autoplay` | `bool` | `False` | Avance automatique des diapositives |
| `interval_ms` | `int` | `3000` | Intervalle d'avance automatique en millisecondes |

---

## Retourne

`Chart` (composite)

---

## Exemples

```python
import seraplot as sp

diapositives = [
    sp.build_bar_chart("CA T1", labels=["A","B","C"], values=[120,80,95]),
    sp.build_line_chart("Tendance", labels=["Jan","Fév","Mar"], values=[10,14,18]),
    sp.build_pie_chart("Parts de marché", labels=["Nous","Eux"], values=[55,45]),
]

presentation = sp.build_slideshow(diapositives, title="Rapport T1")
```

---

## Voir aussi

- [Grille](grid.md)

</div>
