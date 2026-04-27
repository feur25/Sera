# Candlestick Chart

<div class="lang-en">

## Signature

```python
sp.build_candlestick(
    title: str,
    dates: list[str],
    opens: list[float],
    highs: list[float],
    lows: list[float],
    closes: list[float],
    *,
    width: int = 1000,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    color_up: int = 0x22c55e,
    color_down: int = 0xef4444,
    background: str | None = None,
    gridlines: bool = True,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.candlestick`

---

## Description

A candlestick chart is the standard visualization for OHLC (Open, High, Low, Close) financial price data. Each candle represents one time period: the body spans open to close, the upper wick reaches the session high, and the lower wick the session low. Green (bullish) candles indicate close > open; red (bearish) candles indicate close < open — colors are fully configurable via `color_up` and `color_down`. SeraPlot can render thousands of candles in milliseconds without any JavaScript charting library dependency.

**Ideal for:**
- Financial price chart analysis (stocks, crypto, forex, commodities)
- Identifying candlestick patterns (doji, engulfing, hammer)
- Comparing OHLC data across different instruments or time frames

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `dates` | `list[str]` | required | Date/time labels for each candle |
| `opens` | `list[float]` | required | Opening prices |
| `highs` | `list[float]` | required | Session high prices |
| `lows` | `list[float]` | required | Session low prices |
| `closes` | `list[float]` | required | Closing prices |
| `width` | `int` | `1000` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `color_up` | `int` | `0x22c55e` | Color for bullish candles (close > open) |
| `color_down` | `int` | `0xef4444` | Color for bearish candles (close < open) |
| `background` | `str \| None` | `None` | Background color or `None` = transparent |
| `gridlines` | `bool` | `True` | Show gridlines |
| `hover_json` | `str \| None` | `None` | Custom hover tooltip JSON |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="candlestick">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick','candlestick-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick','candlestick-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick','candlestick-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick','candlestick-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick','candlestick-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick','candlestick-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick','candlestick-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick','candlestick-cpp',this)">C++</button>
</div>
<div id="candlestick-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
dates  = ["Nov 1","Nov 4","Nov 5","Nov 6","Nov 7","Nov 8","Nov 11","Nov 12"]
opens  = [68200, 69800, 72500, 74000, 76800, 79200, 81000, 85000]
highs  = [70100, 73200, 75800, 77500, 80200, 82500, 87000, 89500]
lows   = [67800, 69200, 71500, 73200, 75900, 78000, 80500, 84000]
closes = [69800, 72500, 74000, 76800, 79200, 81000, 85000, 87200]
chart = sp.candlestick(
    title="BTC/USD — November 2024",
    dates=dates,
    opens=opens,
    highs=highs,
    lows=lows,
    closes=closes,
    y_label="Price (USD)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="candlestick-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const dates  = ["Nov 1","Nov 4","Nov 5","Nov 6","Nov 7","Nov 8","Nov 11","Nov 12"];
const opens  = [68200, 69800, 72500, 74000, 76800, 79200, 81000, 85000];
const highs  = [70100, 73200, 75800, 77500, 80200, 82500, 87000, 89500];
const lows   = [67800, 69200, 71500, 73200, 75900, 78000, 80500, 84000];
const closes = [69800, 72500, 74000, 76800, 79200, 81000, 85000, 87200];
const chart = sp.candlestick({
  title: "BTC/USD — November 2024",
  dates, opens, highs, lows, closes,
  yLabel: "Price (USD)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="candlestick-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const dates: string[]  = ["Nov 1","Nov 4","Nov 5","Nov 6","Nov 7","Nov 8","Nov 11","Nov 12"];
const opens: number[]  = [68200, 69800, 72500, 74000, 76800, 79200, 81000, 85000];
const highs: number[]  = [70100, 73200, 75800, 77500, 80200, 82500, 87000, 89500];
const lows: number[]   = [67800, 69200, 71500, 73200, 75900, 78000, 80500, 84000];
const closes: number[] = [69800, 72500, 74000, 76800, 79200, 81000, 85000, 87200];
const chart = sp.candlestick({
  title: "BTC/USD — November 2024",
  dates, opens, highs, lows, closes,
  yLabel: "Price (USD)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="candlestick-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
dates  <- c("Nov 1","Nov 4","Nov 5","Nov 6","Nov 7","Nov 8","Nov 11","Nov 12")
opens  <- c(68200, 69800, 72500, 74000, 76800, 79200, 81000, 85000)
highs  <- c(70100, 73200, 75800, 77500, 80200, 82500, 87000, 89500)
lows   <- c(67800, 69200, 71500, 73200, 75900, 78000, 80500, 84000)
closes <- c(69800, 72500, 74000, 76800, 79200, 81000, 85000, 87200)
chart <- sp$candlestick(
  title   = "BTC/USD — November 2024",
  dates   = dates,
  opens   = opens,
  highs   = highs,
  lows    = lows,
  closes  = closes,
  y_label = "Price (USD)",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="candlestick-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.candlestick()
    .title("BTC/USD — November 2024")
    .dates(List.of("Nov 1","Nov 4","Nov 5","Nov 6","Nov 7","Nov 8","Nov 11","Nov 12"))
    .opens(List.of(68200.0, 69800.0, 72500.0, 74000.0, 76800.0, 79200.0, 81000.0, 85000.0))
    .highs(List.of(70100.0, 73200.0, 75800.0, 77500.0, 80200.0, 82500.0, 87000.0, 89500.0))
    .lows(List.of(67800.0, 69200.0, 71500.0, 73200.0, 75900.0, 78000.0, 80500.0, 84000.0))
    .closes(List.of(69800.0, 72500.0, 74000.0, 76800.0, 79200.0, 81000.0, 85000.0, 87200.0))
    .yLabel("Price (USD)")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="candlestick-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.Candlestick(
    title:  "BTC/USD — November 2024",
    dates:  ["Nov 1","Nov 4","Nov 5","Nov 6","Nov 7","Nov 8","Nov 11","Nov 12"],
    opens:  [68200, 69800, 72500, 74000, 76800, 79200, 81000, 85000],
    highs:  [70100, 73200, 75800, 77500, 80200, 82500, 87000, 89500],
    lows:   [67800, 69200, 71500, 73200, 75900, 78000, 80500, 84000],
    closes: [69800, 72500, 74000, 76800, 79200, 81000, 85000, 87200],
    yLabel: "Price (USD)",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="candlestick-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp
val chart = sp.candlestick(
  title   = "BTC/USD — November 2024",
  dates   = List("Nov 1","Nov 4","Nov 5","Nov 6","Nov 7","Nov 8","Nov 11","Nov 12"),
  opens   = List(68200.0, 69800.0, 72500.0, 74000.0, 76800.0, 79200.0, 81000.0, 85000.0),
  highs   = List(70100.0, 73200.0, 75800.0, 77500.0, 80200.0, 82500.0, 87000.0, 89500.0),
  lows    = List(67800.0, 69200.0, 71500.0, 73200.0, 75900.0, 78000.0, 80500.0, 84000.0),
  closes  = List(69800.0, 72500.0, 74000.0, 76800.0, 79200.0, 81000.0, 85000.0, 87200.0),
  y_label   = "Price (USD)",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="candlestick-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::candlestick({
    .title    = "BTC/USD — November 2024",
    .dates    = {"Nov 1","Nov 4","Nov 5","Nov 6","Nov 7","Nov 8","Nov 11","Nov 12"},
    .opens    = {68200.0, 69800.0, 72500.0, 74000.0, 76800.0, 79200.0, 81000.0, 85000.0},
    .highs    = {70100.0, 73200.0, 75800.0, 77500.0, 80200.0, 82500.0, 87000.0, 89500.0},
    .lows     = {67800.0, 69200.0, 71500.0, 73200.0, 75900.0, 78000.0, 80500.0, 84000.0},
    .closes   = {69800.0, 72500.0, 74000.0, 76800.0, 79200.0, 81000.0, 85000.0, 87200.0},
    .y_label  = "Price (USD)",
    .gridlines = true,
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/candlestick.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Line Chart](line.md) — `sp.build_line_chart()`
- [Multiline Chart](multiline.md) — `sp.build_multiline()`
- [Scatter Chart](scatter.md) — `sp.build_scatter_chart()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_candlestick(
    title: str,
    dates: list[str],
    opens: list[float],
    highs: list[float],
    lows: list[float],
    closes: list[float],
    *,
    width: int = 1000,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    color_up: int = 0x22c55e,
    color_down: int = 0xef4444,
    background: str | None = None,
    gridlines: bool = True,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.candlestick`

---

## Description

Un graphique en chandeliers (candlestick) est la visualisation standard des données financières OHLC (Ouverture, Haut, Bas, Clôture). Chaque chandelier représente une période : le corps s'étend de l'ouverture à la clôture, la mèche supérieure atteint le plus haut de la session et la mèche inférieure le plus bas. Les chandeliers verts (haussiers) indiquent clôture > ouverture ; les chandeliers rouges (baissiers) indiquent clôture < ouverture — les couleurs sont entièrement configurables. SeraPlot peut rendre des milliers de chandeliers en quelques millisecondes sans dépendance à une bibliothèque graphique JavaScript.

**Idéal pour :**
- L'analyse de prix financiers (actions, crypto, forex, matières premières)
- L'identification de figures chartistes (doji, englobant, marteau)
- La comparaison de données OHLC entre différents instruments

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `dates` | `list[str]` | requis | Étiquettes de date/heure pour chaque chandelier |
| `opens` | `list[float]` | requis | Prix d'ouverture |
| `highs` | `list[float]` | requis | Plus hauts de session |
| `lows` | `list[float]` | requis | Plus bas de session |
| `closes` | `list[float]` | requis | Prix de clôture |
| `width` | `int` | `1000` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `x_label` | `str` | `""` | Étiquette de l'axe X |
| `y_label` | `str` | `""` | Étiquette de l'axe Y |
| `color_up` | `int` | `0x22c55e` | Couleur des chandeliers haussiers (clôture > ouverture) |
| `color_down` | `int` | `0xef4444` | Couleur des chandeliers baissiers (clôture < ouverture) |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |
| `gridlines` | `bool` | `True` | Afficher les lignes de grille |
| `hover_json` | `str \| None` | `None` | JSON d'infobulle personnalisée |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="candlestick-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-fr','candlestick-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-fr','candlestick-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr','candlestick-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr','candlestick-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-fr','candlestick-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-fr','candlestick-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-fr','candlestick-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-fr','candlestick-fr-cpp',this)">C++</button>
</div>
<div id="candlestick-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
dates  = ["1 nov","4 nov","5 nov","6 nov","7 nov","8 nov","11 nov","12 nov"]
opens  = [68200, 69800, 72500, 74000, 76800, 79200, 81000, 85000]
highs  = [70100, 73200, 75800, 77500, 80200, 82500, 87000, 89500]
lows   = [67800, 69200, 71500, 73200, 75900, 78000, 80500, 84000]
closes = [69800, 72500, 74000, 76800, 79200, 81000, 85000, 87200]
chart = sp.candlestick(
    title="BTC/USD — Novembre 2024",
    dates=dates,
    opens=opens,
    highs=highs,
    lows=lows,
    closes=closes,
    y_label="Prix (USD)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="candlestick-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const dates  = ["1 nov","4 nov","5 nov","6 nov","7 nov","8 nov","11 nov","12 nov"];
const opens  = [68200, 69800, 72500, 74000, 76800, 79200, 81000, 85000];
const highs  = [70100, 73200, 75800, 77500, 80200, 82500, 87000, 89500];
const lows   = [67800, 69200, 71500, 73200, 75900, 78000, 80500, 84000];
const closes = [69800, 72500, 74000, 76800, 79200, 81000, 85000, 87200];
const chart = sp.candlestick({
  title: "BTC/USD — Novembre 2024",
  dates, opens, highs, lows, closes,
  yLabel: "Prix (USD)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="candlestick-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const dates: string[]  = ["1 nov","4 nov","5 nov","6 nov","7 nov","8 nov","11 nov","12 nov"];
const opens: number[]  = [68200, 69800, 72500, 74000, 76800, 79200, 81000, 85000];
const highs: number[]  = [70100, 73200, 75800, 77500, 80200, 82500, 87000, 89500];
const lows: number[]   = [67800, 69200, 71500, 73200, 75900, 78000, 80500, 84000];
const closes: number[] = [69800, 72500, 74000, 76800, 79200, 81000, 85000, 87200];
const chart = sp.candlestick({
  title: "BTC/USD — Novembre 2024",
  dates, opens, highs, lows, closes,
  yLabel: "Prix (USD)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="candlestick-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
dates  <- c("1 nov","4 nov","5 nov","6 nov","7 nov","8 nov","11 nov","12 nov")
opens  <- c(68200, 69800, 72500, 74000, 76800, 79200, 81000, 85000)
highs  <- c(70100, 73200, 75800, 77500, 80200, 82500, 87000, 89500)
lows   <- c(67800, 69200, 71500, 73200, 75900, 78000, 80500, 84000)
closes <- c(69800, 72500, 74000, 76800, 79200, 81000, 85000, 87200)
chart <- sp$candlestick(
  title     = "BTC/USD — Novembre 2024",
  dates     = dates,
  opens     = opens,
  highs     = highs,
  lows      = lows,
  closes    = closes,
  y_label   = "Prix (USD)",
  gridlines = TRUE
)
chart$show()</code></pre></div>
<div id="candlestick-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.candlestick()
    .title("BTC/USD — Novembre 2024")
    .dates(List.of("1 nov","4 nov","5 nov","6 nov","7 nov","8 nov","11 nov","12 nov"))
    .opens(List.of(68200.0, 69800.0, 72500.0, 74000.0, 76800.0, 79200.0, 81000.0, 85000.0))
    .highs(List.of(70100.0, 73200.0, 75800.0, 77500.0, 80200.0, 82500.0, 87000.0, 89500.0))
    .lows(List.of(67800.0, 69200.0, 71500.0, 73200.0, 75900.0, 78000.0, 80500.0, 84000.0))
    .closes(List.of(69800.0, 72500.0, 74000.0, 76800.0, 79200.0, 81000.0, 85000.0, 87200.0))
    .yLabel("Prix (USD)")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="candlestick-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.Candlestick(
    title:  "BTC/USD — Novembre 2024",
    dates:  ["1 nov","4 nov","5 nov","6 nov","7 nov","8 nov","11 nov","12 nov"],
    opens:  [68200, 69800, 72500, 74000, 76800, 79200, 81000, 85000],
    highs:  [70100, 73200, 75800, 77500, 80200, 82500, 87000, 89500],
    lows:   [67800, 69200, 71500, 73200, 75900, 78000, 80500, 84000],
    closes: [69800, 72500, 74000, 76800, 79200, 81000, 85000, 87200],
    yLabel: "Prix (USD)",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="candlestick-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp
val chart = sp.candlestick(
  title   = "BTC/USD — Novembre 2024",
  dates   = List("1 nov","4 nov","5 nov","6 nov","7 nov","8 nov","11 nov","12 nov"),
  opens   = List(68200.0, 69800.0, 72500.0, 74000.0, 76800.0, 79200.0, 81000.0, 85000.0),
  highs   = List(70100.0, 73200.0, 75800.0, 77500.0, 80200.0, 82500.0, 87000.0, 89500.0),
  lows    = List(67800.0, 69200.0, 71500.0, 73200.0, 75900.0, 78000.0, 80500.0, 84000.0),
  closes  = List(69800.0, 72500.0, 74000.0, 76800.0, 79200.0, 81000.0, 85000.0, 87200.0),
  y_label   = "Prix (USD)",
  gridlines = true
)
chart.show()</code></pre></div>
<div id="candlestick-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::candlestick({
    .title    = "BTC/USD — Novembre 2024",
    .dates    = {"1 nov","4 nov","5 nov","6 nov","7 nov","8 nov","11 nov","12 nov"},
    .opens    = {68200.0, 69800.0, 72500.0, 74000.0, 76800.0, 79200.0, 81000.0, 85000.0},
    .highs    = {70100.0, 73200.0, 75800.0, 77500.0, 80200.0, 82500.0, 87000.0, 89500.0},
    .lows     = {67800.0, 69200.0, 71500.0, 73200.0, 75900.0, 78000.0, 80500.0, 84000.0},
    .closes   = {69800.0, 72500.0, 74000.0, 76800.0, 79200.0, 81000.0, 85000.0, 87200.0},
    .y_label  = "Prix (USD)",
    .gridlines = true,
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/candlestick.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [Graphique en ligne](line.md) — `sp.build_line_chart()`
- [Graphique multiligne](multiline.md) — `sp.build_multiline()`
- [Nuage de points](scatter.md) — `sp.build_scatter_chart()`

</div>
