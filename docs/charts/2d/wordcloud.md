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

<div class="sp-tabs" id="wordcloud">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('wordcloud','wordcloud-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('wordcloud','wordcloud-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('wordcloud','wordcloud-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('wordcloud','wordcloud-r',this)">R</button>
<button class="sp-tb" onclick="spTab('wordcloud','wordcloud-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('wordcloud','wordcloud-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('wordcloud','wordcloud-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('wordcloud','wordcloud-cpp',this)">C++</button>
</div>
<div id="wordcloud-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.wordcloud(
    title="Tech Conference Topics 2024",
    words=["AI", "Rust", "WebAssembly", "Kubernetes", "LLM",
           "Python", "TypeScript", "GraphQL", "Edge Computing", "MLOps",
           "Observability", "Serverless", "DevOps", "OpenAPI", "Embeddings",
           "Vector DB", "RAG", "FineTuning", "ZeroTrust", "Streaming"],
    weights=[98, 84, 72, 68, 95, 77, 65, 52, 58, 88,
             45, 50, 61, 40, 82, 75, 79, 70, 43, 55],
    max_words=20,
)
chart.show()</code></pre></div>
<div id="wordcloud-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.wordcloud({
  title: "Tech Conference Topics 2024",
  words: ["AI","Rust","WebAssembly","Kubernetes","LLM",
          "Python","TypeScript","GraphQL","Edge Computing","MLOps",
          "Observability","Serverless","DevOps","OpenAPI","Embeddings",
          "Vector DB","RAG","FineTuning","ZeroTrust","Streaming"],
  weights: [98,84,72,68,95,77,65,52,58,88,45,50,61,40,82,75,79,70,43,55],
  maxWords: 20,
});
chart.show();</code></pre></div>
<div id="wordcloud-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.wordcloud({
  title: "Tech Conference Topics 2024",
  words: ["AI","Rust","WebAssembly","Kubernetes","LLM",
          "Python","TypeScript","GraphQL","Edge Computing","MLOps",
          "Observability","Serverless","DevOps","OpenAPI","Embeddings",
          "Vector DB","RAG","FineTuning","ZeroTrust","Streaming"],
  weights: [98,84,72,68,95,77,65,52,58,88,45,50,61,40,82,75,79,70,43,55],
  maxWords: 20,
});
chart.show();</code></pre></div>
<div id="wordcloud-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$wordcloud(
  title   = "Tech Conference Topics 2024",
  words   = c("AI","Rust","WebAssembly","Kubernetes","LLM",
              "Python","TypeScript","GraphQL","Edge Computing","MLOps",
              "Observability","Serverless","DevOps","OpenAPI","Embeddings",
              "Vector DB","RAG","FineTuning","ZeroTrust","Streaming"),
  weights = c(98,84,72,68,95,77,65,52,58,88,45,50,61,40,82,75,79,70,43,55),
  max_words = 20L
)
chart$show()</code></pre></div>
<div id="wordcloud-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.wordcloud()
    .title("Tech Conference Topics 2024")
    .words(List.of("AI","Rust","WebAssembly","Kubernetes","LLM",
                   "Python","TypeScript","GraphQL","Edge Computing","MLOps",
                   "Observability","Serverless","DevOps","OpenAPI","Embeddings",
                   "Vector DB","RAG","FineTuning","ZeroTrust","Streaming"))
    .weights(List.of(98.0,84.0,72.0,68.0,95.0,77.0,65.0,52.0,58.0,88.0,
                     45.0,50.0,61.0,40.0,82.0,75.0,79.0,70.0,43.0,55.0))
    .maxWords(20)
    .build();
chart.show();</code></pre></div>
<div id="wordcloud-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.Wordcloud(
    title:    "Tech Conference Topics 2024",
    words:    ["AI","Rust","WebAssembly","Kubernetes","LLM",
               "Python","TypeScript","GraphQL","Edge Computing","MLOps",
               "Observability","Serverless","DevOps","OpenAPI","Embeddings",
               "Vector DB","RAG","FineTuning","ZeroTrust","Streaming"],
    weights:  [98,84,72,68,95,77,65,52,58,88,45,50,61,40,82,75,79,70,43,55],
    maxWords: 20
);
chart.Show();</code></pre></div>
<div id="wordcloud-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp
val chart = sp.wordcloud(
  title     = "Tech Conference Topics 2024",
  words     = List("AI","Rust","WebAssembly","Kubernetes","LLM",
                   "Python","TypeScript","GraphQL","Edge Computing","MLOps",
                   "Observability","Serverless","DevOps","OpenAPI","Embeddings",
                   "Vector DB","RAG","FineTuning","ZeroTrust","Streaming"),
  weights   = List(98.0,84.0,72.0,68.0,95.0,77.0,65.0,52.0,58.0,88.0,
                   45.0,50.0,61.0,40.0,82.0,75.0,79.0,70.0,43.0,55.0),
  max_words = 20
)
chart.show()</code></pre></div>
<div id="wordcloud-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::wordcloud({
    .title     = "Tech Conference Topics 2024",
    .words     = {"AI","Rust","WebAssembly","Kubernetes","LLM",
                  "Python","TypeScript","GraphQL","Edge Computing","MLOps",
                  "Observability","Serverless","DevOps","OpenAPI","Embeddings",
                  "Vector DB","RAG","FineTuning","ZeroTrust","Streaming"},
    .weights   = {98.0,84.0,72.0,68.0,95.0,77.0,65.0,52.0,58.0,88.0,
                  45.0,50.0,61.0,40.0,82.0,75.0,79.0,70.0,43.0,55.0},
    .max_words = 20,
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/wordcloud.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Horizontal Bar](hbar.md) — `sp.build_hbar_chart()`
- [Treemap](treemap.md) — `sp.build_treemap()`

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

Aliases: `sp.wordcloud`

---

## Description

Un nuage de mots place les mots à des tailles de police proportionnelles à leurs `weights` en utilisant un algorithme de placement en spirale. Le moteur Rust de SeraPlot utilise une grille de détection de collisions pour un placement O(n log n), permettant de rendre des nuages de 200 mots en quelques millisecondes. Les caractères Unicode et les emoji sont entièrement supportés, permettant des nuages de mots dans n'importe quelle langue.

**Idéal pour :**
- Visualiser la fréquence des termes dans des corpus textuels (avis clients, réseaux sociaux, sujets de conférence)
- Communiquer rapidement des motifs de données qualitatives à des audiences non techniques
- Vue d'ensemble exploratoire rapide de l'importance relative des mots

---

## Paramètres

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

## Retourne

`Chart`

---

<div class="sp-tabs" id="wordcloud-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('wordcloud-fr','wordcloud-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('wordcloud-fr','wordcloud-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('wordcloud-fr','wordcloud-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('wordcloud-fr','wordcloud-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('wordcloud-fr','wordcloud-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('wordcloud-fr','wordcloud-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('wordcloud-fr','wordcloud-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('wordcloud-fr','wordcloud-fr-cpp',this)">C++</button>
</div>
<div id="wordcloud-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.wordcloud(
    title="Sujets de conférence tech 2024",
    words=["IA", "Rust", "WebAssembly", "Kubernetes", "LLM",
           "Python", "TypeScript", "GraphQL", "Edge Computing", "MLOps",
           "Observabilité", "Serverless", "DevOps", "OpenAPI", "Embeddings",
           "Vector DB", "RAG", "Affinage", "ZeroTrust", "Streaming"],
    weights=[98, 84, 72, 68, 95, 77, 65, 52, 58, 88,
             45, 50, 61, 40, 82, 75, 79, 70, 43, 55],
    max_words=20,
)
chart.show()</code></pre></div>
<div id="wordcloud-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.wordcloud({
  title: "Sujets de conférence tech 2024",
  words: ["IA","Rust","WebAssembly","Kubernetes","LLM",
          "Python","TypeScript","GraphQL","Edge Computing","MLOps",
          "Observabilité","Serverless","DevOps","OpenAPI","Embeddings",
          "Vector DB","RAG","Affinage","ZeroTrust","Streaming"],
  weights: [98,84,72,68,95,77,65,52,58,88,45,50,61,40,82,75,79,70,43,55],
  maxWords: 20,
});
chart.show();</code></pre></div>
<div id="wordcloud-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.wordcloud({
  title: "Sujets de conférence tech 2024",
  words: ["IA","Rust","WebAssembly","Kubernetes","LLM",
          "Python","TypeScript","GraphQL","Edge Computing","MLOps",
          "Observabilité","Serverless","DevOps","OpenAPI","Embeddings",
          "Vector DB","RAG","Affinage","ZeroTrust","Streaming"],
  weights: [98,84,72,68,95,77,65,52,58,88,45,50,61,40,82,75,79,70,43,55],
  maxWords: 20,
});
chart.show();</code></pre></div>
<div id="wordcloud-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$wordcloud(
  title     = "Sujets de conférence tech 2024",
  words     = c("IA","Rust","WebAssembly","Kubernetes","LLM",
                "Python","TypeScript","GraphQL","Edge Computing","MLOps",
                "Observabilité","Serverless","DevOps","OpenAPI","Embeddings",
                "Vector DB","RAG","Affinage","ZeroTrust","Streaming"),
  weights   = c(98,84,72,68,95,77,65,52,58,88,45,50,61,40,82,75,79,70,43,55),
  max_words = 20L
)
chart$show()</code></pre></div>
<div id="wordcloud-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.wordcloud()
    .title("Sujets de conférence tech 2024")
    .words(List.of("IA","Rust","WebAssembly","Kubernetes","LLM",
                   "Python","TypeScript","GraphQL","Edge Computing","MLOps",
                   "Observabilité","Serverless","DevOps","OpenAPI","Embeddings",
                   "Vector DB","RAG","Affinage","ZeroTrust","Streaming"))
    .weights(List.of(98.0,84.0,72.0,68.0,95.0,77.0,65.0,52.0,58.0,88.0,
                     45.0,50.0,61.0,40.0,82.0,75.0,79.0,70.0,43.0,55.0))
    .maxWords(20)
    .build();
chart.show();</code></pre></div>
<div id="wordcloud-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.Wordcloud(
    title:    "Sujets de conférence tech 2024",
    words:    ["IA","Rust","WebAssembly","Kubernetes","LLM",
               "Python","TypeScript","GraphQL","Edge Computing","MLOps",
               "Observabilité","Serverless","DevOps","OpenAPI","Embeddings",
               "Vector DB","RAG","Affinage","ZeroTrust","Streaming"],
    weights:  [98,84,72,68,95,77,65,52,58,88,45,50,61,40,82,75,79,70,43,55],
    maxWords: 20
);
chart.Show();</code></pre></div>
<div id="wordcloud-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp
val chart = sp.wordcloud(
  title     = "Sujets de conférence tech 2024",
  words     = List("IA","Rust","WebAssembly","Kubernetes","LLM",
                   "Python","TypeScript","GraphQL","Edge Computing","MLOps",
                   "Observabilité","Serverless","DevOps","OpenAPI","Embeddings",
                   "Vector DB","RAG","Affinage","ZeroTrust","Streaming"),
  weights   = List(98.0,84.0,72.0,68.0,95.0,77.0,65.0,52.0,58.0,88.0,
                   45.0,50.0,61.0,40.0,82.0,75.0,79.0,70.0,43.0,55.0),
  max_words = 20
)
chart.show()</code></pre></div>
<div id="wordcloud-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::wordcloud({
    .title     = "Sujets de conférence tech 2024",
    .words     = {"IA","Rust","WebAssembly","Kubernetes","LLM",
                  "Python","TypeScript","GraphQL","Edge Computing","MLOps",
                  "Observabilité","Serverless","DevOps","OpenAPI","Embeddings",
                  "Vector DB","RAG","Affinage","ZeroTrust","Streaming"},
    .weights   = {98.0,84.0,72.0,68.0,95.0,77.0,65.0,52.0,58.0,88.0,
                  45.0,50.0,61.0,40.0,82.0,75.0,79.0,70.0,43.0,55.0},
    .max_words = 20,
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/wordcloud.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [Graphique en barres](bar.md) — `sp.build_bar_chart()`
- [Barres horizontales](hbar.md) — `sp.build_hbar_chart()`
- [Treemap](treemap.md) — `sp.build_treemap()`

</div>
