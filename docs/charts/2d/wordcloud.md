# Word Cloud

<div class="lang-en">

## Signature

```python
sp.build_wordcloud(
    title: str,
    words: list[str],
    weights: list[float],
    *,
    width: int = 900,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    max_words: int = 200,
) -> Chart
```

---

## Description

Word cloud (tag cloud) where font size reflects the weight of each word.

Words with higher `weights` are displayed larger. Layout is computed via a spiral placement algorithm.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `words` | `list[str]` | required | Word|
| `weights` | `list[float]` | required | Weight per word (higher = larger) |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `palette` | `list[int] \| None` | `None` | Custom color palette |
| `background` | `str \| None` | `None` | Background color |
| `max_words` | `int` | `200` | Maximum number of words rendered |

---

## Returns

`Chart`

---

## Examples

### Technology popularity





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
<div class="sp-tabs" id="wordcloud">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('wordcloud','wordcloud-py',this)">Python</button><button class="sp-tb" onclick="spTab('wordcloud','wordcloud-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('wordcloud','wordcloud-ts',this)">TypeScript</button></div>
<div id="wordcloud-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
from collections import Counter

text = "python python rust rust rust go go java javascript python data ml deep learning neural"
counts = Counter(text.split())

chart = sp.build_wordcloud(
    "Tech Mentions",
    words=list(counts.keys()),
    frequencies=list(counts.values()),
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e, 0xf59e0b, 0x10b981],
)</code></pre></div>
<div id="wordcloud-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
from collections import Counter

const text = "python python rust rust rust go go java javascript python data ml deep learning neural"
const counts = Counter(text.split())

const chart = sp.build_wordcloud("Tech Mentions",
list(counts.keys()),
{
    frequencies: list(counts.values()),
    palette: [0x6366f1, 0x22d3ee, 0xf43f5e, 0xf59e0b, 0x10b981]
})</code></pre></div>
<div id="wordcloud-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
from collections import Counter

const text: string = "python python rust rust rust go go java javascript python data ml deep learning neural"
const counts = Counter(text.split())

const chart = sp.build_wordcloud("Tech Mentions",
list(counts.keys()),
{
    frequencies: list(counts.values()),
    palette: [0x6366f1, 0x22d3ee, 0xf43f5e, 0xf59e0b, 0x10b981]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/wordcloud.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Bar Chart](bar.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_wordcloud(
    title: str,
    words: list[str],
    weights: list[float],
    *,
    width: int = 900,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    max_words: int = 200,
) -> Chart
```

---

## Description

Nuage de mots où la taille de la police reflète le poids de chaque mot. Les mots avec un `weights` plus élevé sont affichés en plus grand. La disposition est calculée via un algorithme de placement en spirale.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `words` | `list[str]` | requis | Liste des mots |
| `weights` | `list[float]` | requis | Poids par mot (plus élevé = plus grand) |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `480` | Hauteur du canvas |
| `palette` | `list[int] \| None` | `None` | Palette de couleurs |
| `background` | `str \| None` | `None` | Couleur de fond |
| `max_words` | `int` | `200` | Nombre maximum de mots affichés |

---

## Retourne

`Chart`

---

## Exemples

### Popularité des technologies

```python
import seraplot as sp
from collections import Counter

texte = "python python rust rust rust go go java javascript python data ml deep learning neural"
counts = Counter(texte.split())

chart = sp.build_wordcloud(
    "Mentions technologiques",
    words=list(counts.keys()),
    weights=list(counts.values()),
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e, 0xf59e0b, 0x10b981],
)
```

---

## Voir aussi

- [Graphique en barres](bar.md)

</div>
