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

Aliases: `sp.wordcloud`

---

## Description

A word cloud places words at font sizes proportional to their `weights` using an outward spiral placement algorithm. SeraPlot's Rust renderer uses a collision detection grid for O(n log n) placement so even clouds of 200 words render in milliseconds. Unicode characters and emoji are fully supported, enabling word clouds in any language. Words are colored by cycling through the active palette (or the default SeraPlot palette). Words that cannot fit after exhausting spiral iterations are silently dropped.

**Ideal for:**
- Visualizing term frequency in text corpora (customer feedback, social media, conference topics)
- Communicating qualitative data patterns quickly to non-technical audiences
- Quick exploratory overview of relative word importance

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `words` | `list[str]` | required | Word strings |
| `weights` | `list[float]` | required | Relative frequency or importance of each word |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `palette` | `list[int] \| None` | `None` | Colors cycled for words |
| `background` | `str \| None` | `None` | Background color or `None` = transparent |
| `max_words` | `int` | `200` | Maximum number of words to render |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>


</div>

<iframe src="../../previews/wordcloud.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Horizontal Bar](hbar.md) — `sp.build_hbar_chart()`
- [Treemap](treemap.md) — `sp.build_treemap()`

</div>

<div class="lang-fr">

<h2>Signature</h2>

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

Aliases: `sp.wordcloud`

---

<h2>Description</h2>

Un nuage de mots place les mots à des tailles de police proportionnelles à leurs `weights` en utilisant un algorithme de placement en spirale. Le moteur Rust de SeraPlot utilise une grille de détection de collisions pour un placement O(n log n), permettant de rendre des nuages de 200 mots en quelques millisecondes. Les caractères Unicode et les emoji sont entièrement supportés, permettant des nuages de mots dans n'importe quelle langue.

**Idéal pour :**
- Visualiser la fréquence des termes dans des corpus textuels (avis clients, réseaux sociaux, sujets de conférence)
- Communiquer rapidement des motifs de données qualitatives à des audiences non techniques
- Vue d'ensemble exploratoire rapide de l'importance relative des mots

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `words` | `list[str]` | requis | Chaînes de caractères des mots |
| `weights` | `list[float]` | requis | Fréquence ou importance relative de chaque mot |
| `width` | `int` | `900` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `palette` | `list[int] \| None` | `None` | Couleurs cyclées pour les mots |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |
| `max_words` | `int` | `200` | Nombre maximum de mots à afficher |

---

<h2>Retourne</h2>

`Chart`

---


</div>

<iframe src="../../previews/wordcloud.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

<h2>Voir aussi</h2>

- [Graphique en barres](bar.md) — `sp.build_bar_chart()`
- [Barres horizontales](hbar.md) — `sp.build_hbar_chart()`
- [Treemap](treemap.md) — `sp.build_treemap()`

</div>
