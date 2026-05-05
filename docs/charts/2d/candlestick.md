# Candlestick — OHLC Time Series

<div class="lang-en">

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}
.sp-cls-toggle:hover{background:#312e81;color:#e0e7ff;transform:translateY(-1px)}
.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540 0%,#141d33 70%,#0f172a 100%);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;box-shadow:-6px 4px 14px -4px rgba(0,0,0,.55),inset 0 1px 0 rgba(255,255,255,.04),inset 1px 0 0 rgba(255,255,255,.05);transition:all .25s cubic-bezier(.5,0,.2,1);clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}
.sp-cls-tab:hover{background:linear-gradient(90deg,#23304d,#1a2540 70%,#141d33);color:#e0e7ff;margin-left:-40px;box-shadow:-8px 6px 18px -4px rgba(0,0,0,.6),inset 0 1px 0 rgba(255,255,255,.06)}
.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3 0%,#1e1b4b 50%,#0f172a 100%);color:#f5f3ff;margin-left:-46px;box-shadow:-10px 8px 22px -4px rgba(99,102,241,.35),-3px 0 0 0 #818cf8 inset,inset 0 1px 0 rgba(165,180,252,.2);font-weight:700;z-index:3}
.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;font-weight:900;letter-spacing:-1px;width:16px;text-align:center;text-shadow:0 0 6px rgba(165,180,252,.4)}
.sp-cls-tab.sp-cact .sp-cic{color:#e0e7ff;text-shadow:0 0 10px rgba(165,180,252,.7)}
.sp-cls-tab .sp-clb{display:none;font-weight:inherit;letter-spacing:.01em}
.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}
.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;position:relative;z-index:1;border-radius:0 14px 14px 0;overflow:hidden}
.sp-variant{display:none}
.sp-variant.sp-von{display:block;animation:spFade .25s ease}
@keyframes spFade{from{opacity:0;transform:translateX(8px)}to{opacity:1;transform:translateX(0)}}
.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
.sp-preview-frame{width:100%;height:520px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.candlestick(title, labels, open, high, low, close, *, variant="basic", palette=None, **kwargs) -> Chart`

Aliases: `sp.candlestick`, `sp.build_candlestick`

## Description

`sp.candlestick()` is the unified entry point for the entire candlestick-chart family. A candlestick chart shows OHLC (Open, High, Low, Close) bars over time and is the de facto standard for financial markets, crypto, commodities, energy spot prices and any time-series with intra-period spread. The `variant` keyword switches the visual style without touching the data — including derived views like Heikin-Ashi smoothing, close-only line, mountain area and high-low range bars.

> **Color convention** — by default green = up (`close >= open`) and red = down. Override with `palette=[up_color, down_color]`. Bars are rendered left-to-right in input order; use `sort_order="asc"` to sort by close price.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / filled` | Classic OHLC candles: solid body, thin wick, green for up bars and red for down bars. |
| `"hollow"` | `hollow / empty / japanese / white_up` | Japanese-style hollow up candles (white fill + colored stroke) and filled down candles. |
| `"ohlc"` | `ohlc / western / bar / tick` | Western OHLC bars: vertical wick with left tick = open, right tick = close, no body. |
| `"heikin"` | `heikin / heikin_ashi / ha / smoothed` | Heikin-Ashi smoothed candles: filters market noise to highlight trends and reversals clearly. |
| `"outlined"` | `outlined / outline / stroke / wireframe` | Wireframe candles: translucent body with bold colored stroke; lighter visual footprint. |
| `"line"` | `line / close / lineplot / trend` | Close-price line chart with markers — same data, smoother trend reading without OHLC noise. |
| `"mountain"` | `mountain / area / filled_area / shade` | Close-price area chart with vertical gradient under the line; great for hero / cover charts. |
| `"range"` | `range / hl / highlow / spread` | High-low range bars only (no open/close), single color — pure volatility visualization. |

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title`    | `str`         | required  | Chart title |
| `labels`   | `list[str]`   | required  | Period labels (e.g. dates) |
| `open`     | `list[float]` | required  | Opening price per period |
| `high`     | `list[float]` | required  | Highest price per period |
| `low`      | `list[float]` | required  | Lowest price per period |
| `close`    | `list[float]` | required  | Closing price per period |
| `variant`  | `str`         | `"basic"` | Visual style (see table) |
| `palette`  | `list[int]`   | `None`    | `[up_color, down_color]` (defaults: green/red) |
| `gridlines`| `bool`        | `False`   | Horizontal price gridlines |
| `x_label`  | `str`         | `""`      | X-axis label |
| `y_label`  | `str`         | `""`      | Y-axis label |
| `sort_order` | `str`       | `"none"` | `"none"`, `"asc"`, `"desc"` (by close) |
| `width`    | `int`         | `1100`    | Canvas width (px) |
| `height`   | `int`         | `500`     | Canvas height (px) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="candlestick-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('candlestick-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('candlestick-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','hollow',this)"><span class="sp-cic">H</span><span class="sp-clb">Hollow</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','ohlc',this)"><span class="sp-cic">O</span><span class="sp-clb">OHLC</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','heikin',this)"><span class="sp-cic">K</span><span class="sp-clb">Heikin-Ashi</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','outlined',this)"><span class="sp-cic">L</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','line',this)"><span class="sp-cic">I</span><span class="sp-clb">Line</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','mountain',this)"><span class="sp-cic">M</span><span class="sp-clb">Mountain</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','range',this)"><span class="sp-cic">R</span><span class="sp-clb">Range</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="candlestick-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / filled</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic OHLC candles: solid body, thin wick, green for up bars and red for down bars.</p>
<div class="sp-tabs" id="candlestick-en-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-en-basic','candlestick-en-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-en-basic','candlestick-en-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-basic','candlestick-en-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-basic','candlestick-en-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-en-basic','candlestick-en-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-en-basic','candlestick-en-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-en-basic','candlestick-en-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-en-basic','candlestick-en-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-en-basic','candlestick-en-basic-cpp',this)">C++</button>
</div><div id="candlestick-en-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="basic",
)
chart.show()</code></pre></div><div id="candlestick-en-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="candlestick-en-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="candlestick-en-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="candlestick-en-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-en-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="candlestick-en-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="candlestick-en-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="candlestick-en-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-basic.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-hollow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"hollow"</code></span><span><strong>Aliases</strong> <code>hollow / empty / japanese / white_up</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Japanese-style hollow up candles (white fill + colored stroke) and filled down candles.</p>
<div class="sp-tabs" id="candlestick-en-hollow">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-en-hollow','candlestick-en-hollow-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-en-hollow','candlestick-en-hollow-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-hollow','candlestick-en-hollow-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-hollow','candlestick-en-hollow-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-en-hollow','candlestick-en-hollow-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-en-hollow','candlestick-en-hollow-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-en-hollow','candlestick-en-hollow-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-en-hollow','candlestick-en-hollow-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-en-hollow','candlestick-en-hollow-cpp',this)">C++</button>
</div><div id="candlestick-en-hollow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="hollow",
)
chart.show()</code></pre></div><div id="candlestick-en-hollow-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "hollow",
});
chart.show();</code></pre></div><div id="candlestick-en-hollow-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "hollow",
});
chart.show();</code></pre></div><div id="candlestick-en-hollow-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "hollow"
)
chart$show()</code></pre></div><div id="candlestick-en-hollow-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("hollow")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-en-hollow-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("hollow")
    .build();
chart.show();</code></pre></div><div id="candlestick-en-hollow-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "hollow"
);
chart.Show();</code></pre></div><div id="candlestick-en-hollow-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "hollow"
)
chart.show()</code></pre></div><div id="candlestick-en-hollow-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "hollow",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-hollow.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-ohlc">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ohlc"</code></span><span><strong>Aliases</strong> <code>ohlc / western / bar / tick</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Western OHLC bars: vertical wick with left tick = open, right tick = close, no body.</p>
<div class="sp-tabs" id="candlestick-en-ohlc">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-en-ohlc','candlestick-en-ohlc-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-en-ohlc','candlestick-en-ohlc-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-ohlc','candlestick-en-ohlc-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-ohlc','candlestick-en-ohlc-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-en-ohlc','candlestick-en-ohlc-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-en-ohlc','candlestick-en-ohlc-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-en-ohlc','candlestick-en-ohlc-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-en-ohlc','candlestick-en-ohlc-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-en-ohlc','candlestick-en-ohlc-cpp',this)">C++</button>
</div><div id="candlestick-en-ohlc-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="ohlc",
)
chart.show()</code></pre></div><div id="candlestick-en-ohlc-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "ohlc",
});
chart.show();</code></pre></div><div id="candlestick-en-ohlc-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "ohlc",
});
chart.show();</code></pre></div><div id="candlestick-en-ohlc-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "ohlc"
)
chart$show()</code></pre></div><div id="candlestick-en-ohlc-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("ohlc")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-en-ohlc-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("ohlc")
    .build();
chart.show();</code></pre></div><div id="candlestick-en-ohlc-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "ohlc"
);
chart.Show();</code></pre></div><div id="candlestick-en-ohlc-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "ohlc"
)
chart.show()</code></pre></div><div id="candlestick-en-ohlc-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "ohlc",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-ohlc.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-heikin">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heikin"</code></span><span><strong>Aliases</strong> <code>heikin / heikin_ashi / ha / smoothed</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Heikin-Ashi smoothed candles: filters market noise to highlight trends and reversals clearly.</p>
<div class="sp-tabs" id="candlestick-en-heikin">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-en-heikin','candlestick-en-heikin-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-en-heikin','candlestick-en-heikin-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-heikin','candlestick-en-heikin-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-heikin','candlestick-en-heikin-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-en-heikin','candlestick-en-heikin-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-en-heikin','candlestick-en-heikin-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-en-heikin','candlestick-en-heikin-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-en-heikin','candlestick-en-heikin-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-en-heikin','candlestick-en-heikin-cpp',this)">C++</button>
</div><div id="candlestick-en-heikin-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="heikin",
)
chart.show()</code></pre></div><div id="candlestick-en-heikin-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "heikin",
});
chart.show();</code></pre></div><div id="candlestick-en-heikin-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "heikin",
});
chart.show();</code></pre></div><div id="candlestick-en-heikin-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "heikin"
)
chart$show()</code></pre></div><div id="candlestick-en-heikin-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("heikin")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-en-heikin-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("heikin")
    .build();
chart.show();</code></pre></div><div id="candlestick-en-heikin-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "heikin"
);
chart.Show();</code></pre></div><div id="candlestick-en-heikin-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "heikin"
)
chart.show()</code></pre></div><div id="candlestick-en-heikin-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "heikin",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-heikin.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-outlined">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / wireframe</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Wireframe candles: translucent body with bold colored stroke; lighter visual footprint.</p>
<div class="sp-tabs" id="candlestick-en-outlined">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-en-outlined','candlestick-en-outlined-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-en-outlined','candlestick-en-outlined-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-outlined','candlestick-en-outlined-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-outlined','candlestick-en-outlined-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-en-outlined','candlestick-en-outlined-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-en-outlined','candlestick-en-outlined-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-en-outlined','candlestick-en-outlined-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-en-outlined','candlestick-en-outlined-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-en-outlined','candlestick-en-outlined-cpp',this)">C++</button>
</div><div id="candlestick-en-outlined-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="outlined",
)
chart.show()</code></pre></div><div id="candlestick-en-outlined-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "outlined",
});
chart.show();</code></pre></div><div id="candlestick-en-outlined-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "outlined",
});
chart.show();</code></pre></div><div id="candlestick-en-outlined-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "outlined"
)
chart$show()</code></pre></div><div id="candlestick-en-outlined-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("outlined")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-en-outlined-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("outlined")
    .build();
chart.show();</code></pre></div><div id="candlestick-en-outlined-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "outlined"
);
chart.Show();</code></pre></div><div id="candlestick-en-outlined-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "outlined"
)
chart.show()</code></pre></div><div id="candlestick-en-outlined-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "outlined",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-outlined.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-line">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"line"</code></span><span><strong>Aliases</strong> <code>line / close / lineplot / trend</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Close-price line chart with markers — same data, smoother trend reading without OHLC noise.</p>
<div class="sp-tabs" id="candlestick-en-line">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-en-line','candlestick-en-line-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-en-line','candlestick-en-line-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-line','candlestick-en-line-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-line','candlestick-en-line-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-en-line','candlestick-en-line-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-en-line','candlestick-en-line-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-en-line','candlestick-en-line-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-en-line','candlestick-en-line-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-en-line','candlestick-en-line-cpp',this)">C++</button>
</div><div id="candlestick-en-line-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="line",
)
chart.show()</code></pre></div><div id="candlestick-en-line-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "line",
});
chart.show();</code></pre></div><div id="candlestick-en-line-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "line",
});
chart.show();</code></pre></div><div id="candlestick-en-line-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "line"
)
chart$show()</code></pre></div><div id="candlestick-en-line-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("line")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-en-line-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("line")
    .build();
chart.show();</code></pre></div><div id="candlestick-en-line-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "line"
);
chart.Show();</code></pre></div><div id="candlestick-en-line-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "line"
)
chart.show()</code></pre></div><div id="candlestick-en-line-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "line",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-line.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-mountain">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mountain"</code></span><span><strong>Aliases</strong> <code>mountain / area / filled_area / shade</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Close-price area chart with vertical gradient under the line; great for hero / cover charts.</p>
<div class="sp-tabs" id="candlestick-en-mountain">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-en-mountain','candlestick-en-mountain-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-en-mountain','candlestick-en-mountain-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-mountain','candlestick-en-mountain-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-mountain','candlestick-en-mountain-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-en-mountain','candlestick-en-mountain-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-en-mountain','candlestick-en-mountain-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-en-mountain','candlestick-en-mountain-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-en-mountain','candlestick-en-mountain-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-en-mountain','candlestick-en-mountain-cpp',this)">C++</button>
</div><div id="candlestick-en-mountain-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="mountain",
)
chart.show()</code></pre></div><div id="candlestick-en-mountain-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "mountain",
});
chart.show();</code></pre></div><div id="candlestick-en-mountain-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "mountain",
});
chart.show();</code></pre></div><div id="candlestick-en-mountain-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "mountain"
)
chart$show()</code></pre></div><div id="candlestick-en-mountain-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("mountain")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-en-mountain-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("mountain")
    .build();
chart.show();</code></pre></div><div id="candlestick-en-mountain-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "mountain"
);
chart.Show();</code></pre></div><div id="candlestick-en-mountain-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "mountain"
)
chart.show()</code></pre></div><div id="candlestick-en-mountain-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "mountain",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-mountain.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-range">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"range"</code></span><span><strong>Aliases</strong> <code>range / hl / highlow / spread</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">High-low range bars only (no open/close), single color — pure volatility visualization.</p>
<div class="sp-tabs" id="candlestick-en-range">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-en-range','candlestick-en-range-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-en-range','candlestick-en-range-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-range','candlestick-en-range-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-en-range','candlestick-en-range-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-en-range','candlestick-en-range-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-en-range','candlestick-en-range-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-en-range','candlestick-en-range-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-en-range','candlestick-en-range-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-en-range','candlestick-en-range-cpp',this)">C++</button>
</div><div id="candlestick-en-range-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="range",
)
chart.show()</code></pre></div><div id="candlestick-en-range-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "range",
});
chart.show();</code></pre></div><div id="candlestick-en-range-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "range",
});
chart.show();</code></pre></div><div id="candlestick-en-range-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "range"
)
chart$show()</code></pre></div><div id="candlestick-en-range-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("range")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-en-range-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("range")
    .build();
chart.show();</code></pre></div><div id="candlestick-en-range-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "range"
);
chart.Show();</code></pre></div><div id="candlestick-en-range-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "range"
)
chart.show()</code></pre></div><div id="candlestick-en-range-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "range",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-range.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr">

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}
.sp-cls-toggle:hover{background:#312e81;color:#e0e7ff;transform:translateY(-1px)}
.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540 0%,#141d33 70%,#0f172a 100%);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;box-shadow:-6px 4px 14px -4px rgba(0,0,0,.55),inset 0 1px 0 rgba(255,255,255,.04),inset 1px 0 0 rgba(255,255,255,.05);transition:all .25s cubic-bezier(.5,0,.2,1);clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}
.sp-cls-tab:hover{background:linear-gradient(90deg,#23304d,#1a2540 70%,#141d33);color:#e0e7ff;margin-left:-40px;box-shadow:-8px 6px 18px -4px rgba(0,0,0,.6),inset 0 1px 0 rgba(255,255,255,.06)}
.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3 0%,#1e1b4b 50%,#0f172a 100%);color:#f5f3ff;margin-left:-46px;box-shadow:-10px 8px 22px -4px rgba(99,102,241,.35),-3px 0 0 0 #818cf8 inset,inset 0 1px 0 rgba(165,180,252,.2);font-weight:700;z-index:3}
.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;font-weight:900;letter-spacing:-1px;width:16px;text-align:center;text-shadow:0 0 6px rgba(165,180,252,.4)}
.sp-cls-tab.sp-cact .sp-cic{color:#e0e7ff;text-shadow:0 0 10px rgba(165,180,252,.7)}
.sp-cls-tab .sp-clb{display:none;font-weight:inherit;letter-spacing:.01em}
.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}
.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;position:relative;z-index:1;border-radius:0 14px 14px 0;overflow:hidden}
.sp-variant{display:none}
.sp-variant.sp-von{display:block;animation:spFade .25s ease}
@keyframes spFade{from{opacity:0;transform:translateX(8px)}to{opacity:1;transform:translateX(0)}}
.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
.sp-preview-frame{width:100%;height:520px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.candlestick(title, labels, open, high, low, close, *, variant="basic", palette=None, **kwargs) -> Chart`

Alias : `sp.candlestick`, `sp.build_candlestick`

## Description

`sp.candlestick()` est le point d entree unifie pour toute la famille des chandeliers. Un graphique en chandeliers affiche des barres OHLC (Ouverture, Haut, Bas, Cloture) dans le temps et constitue le standard de fait pour les marches financiers, la crypto, les matieres premieres, le spot energie et toute serie temporelle avec spread intra-periode. Le mot-cle `variant` change le style sans toucher aux donnees — y compris des vues derivees comme le lissage Heikin-Ashi, la ligne de cloture, l aire mountain et les barres haut-bas.

> **Convention de couleur** — par defaut vert = hausse (`close >= open`) et rouge = baisse. Surchargez avec `palette=[couleur_hausse, couleur_baisse]`. Les barres sont rendues de gauche a droite dans l ordre d entree ; `sort_order="asc"` pour trier par prix de cloture.

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / filled` | Bougies OHLC classiques : corps plein, meche fine, vert pour hausse et rouge pour baisse. |
| `"hollow"` | `hollow / empty / japanese / white_up` | Style japonais : bougies haussieres creuses (fond blanc + contour colore) et baissieres pleines. |
| `"ohlc"` | `ohlc / western / bar / tick` | Barres OHLC americaines : meche verticale, tick gauche = ouverture, droit = cloture, sans corps. |
| `"heikin"` | `heikin / heikin_ashi / ha / smoothed` | Bougies Heikin-Ashi lissees : filtre le bruit pour mettre en evidence tendances et retournements. |
| `"outlined"` | `outlined / outline / stroke / wireframe` | Bougies en fil de fer : corps translucide avec contour colore epais ; rendu plus aere. |
| `"line"` | `line / close / lineplot / trend` | Ligne des prix de cloture avec marqueurs — meme donnee, lecture de tendance plus lisse. |
| `"mountain"` | `mountain / area / filled_area / shade` | Aire des prix de cloture avec degrade vertical sous la courbe ; ideal pour visuel de couverture. |
| `"range"` | `range / hl / highlow / spread` | Barres haut-bas uniquement (sans ouverture/cloture), une seule couleur — visualisation de la volatilite. |

## Parametres

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title`    | `str`         | requis    | Titre du graphique |
| `labels`   | `list[str]`   | requis    | Libelles de periode (ex. dates) |
| `open`     | `list[float]` | requis    | Prix d ouverture par periode |
| `high`     | `list[float]` | requis    | Prix le plus haut par periode |
| `low`      | `list[float]` | requis    | Prix le plus bas par periode |
| `close`    | `list[float]` | requis    | Prix de cloture par periode |
| `variant`  | `str`         | `"basic"` | Style visuel (voir tableau) |
| `palette`  | `list[int]`   | `None`    | `[hausse, baisse]` (defaut : vert/rouge) |
| `gridlines`| `bool`        | `False`   | Lignes de grille horizontales |
| `x_label`  | `str`         | `""`      | Libelle axe X |
| `y_label`  | `str`         | `""`      | Libelle axe Y |
| `sort_order` | `str`       | `"none"` | `"none"`, `"asc"`, `"desc"` (par cloture) |
| `width`    | `int`         | `1100`    | Largeur du canvas (px) |
| `height`   | `int`         | `500`     | Hauteur du canvas (px) |

## Retour

`Chart` — objet avec une propriete `.html` et une methode `.show()`.

---

<div class="sp-cls sp-open" id="candlestick-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('candlestick-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('candlestick-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','hollow',this)"><span class="sp-cic">H</span><span class="sp-clb">Hollow</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','ohlc',this)"><span class="sp-cic">O</span><span class="sp-clb">OHLC</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','heikin',this)"><span class="sp-cic">K</span><span class="sp-clb">Heikin-Ashi</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','outlined',this)"><span class="sp-cic">L</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','line',this)"><span class="sp-cic">I</span><span class="sp-clb">Line</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','mountain',this)"><span class="sp-cic">M</span><span class="sp-clb">Mountain</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','range',this)"><span class="sp-cic">R</span><span class="sp-clb">Range</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="candlestick-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / filled</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bougies OHLC classiques : corps plein, meche fine, vert pour hausse et rouge pour baisse.</p>
<div class="sp-tabs" id="candlestick-fr-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-fr-basic','candlestick-fr-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-basic','candlestick-fr-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-basic','candlestick-fr-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-basic','candlestick-fr-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-basic','candlestick-fr-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-basic','candlestick-fr-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-basic','candlestick-fr-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-basic','candlestick-fr-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-basic','candlestick-fr-basic-cpp',this)">C++</button>
</div><div id="candlestick-fr-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="basic",
)
chart.show()</code></pre></div><div id="candlestick-fr-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="candlestick-fr-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="candlestick-fr-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="candlestick-fr-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-fr-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="candlestick-fr-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="candlestick-fr-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="candlestick-fr-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-basic.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-hollow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"hollow"</code></span><span><strong>Aliases</strong> <code>hollow / empty / japanese / white_up</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Style japonais : bougies haussieres creuses (fond blanc + contour colore) et baissieres pleines.</p>
<div class="sp-tabs" id="candlestick-fr-hollow">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-fr-hollow','candlestick-fr-hollow-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-hollow','candlestick-fr-hollow-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-hollow','candlestick-fr-hollow-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-hollow','candlestick-fr-hollow-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-hollow','candlestick-fr-hollow-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-hollow','candlestick-fr-hollow-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-hollow','candlestick-fr-hollow-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-hollow','candlestick-fr-hollow-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-hollow','candlestick-fr-hollow-cpp',this)">C++</button>
</div><div id="candlestick-fr-hollow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="hollow",
)
chart.show()</code></pre></div><div id="candlestick-fr-hollow-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "hollow",
});
chart.show();</code></pre></div><div id="candlestick-fr-hollow-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "hollow",
});
chart.show();</code></pre></div><div id="candlestick-fr-hollow-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "hollow"
)
chart$show()</code></pre></div><div id="candlestick-fr-hollow-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("hollow")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-fr-hollow-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("hollow")
    .build();
chart.show();</code></pre></div><div id="candlestick-fr-hollow-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "hollow"
);
chart.Show();</code></pre></div><div id="candlestick-fr-hollow-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "hollow"
)
chart.show()</code></pre></div><div id="candlestick-fr-hollow-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "hollow",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-hollow.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-ohlc">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ohlc"</code></span><span><strong>Aliases</strong> <code>ohlc / western / bar / tick</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres OHLC americaines : meche verticale, tick gauche = ouverture, droit = cloture, sans corps.</p>
<div class="sp-tabs" id="candlestick-fr-ohlc">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-fr-ohlc','candlestick-fr-ohlc-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-ohlc','candlestick-fr-ohlc-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-ohlc','candlestick-fr-ohlc-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-ohlc','candlestick-fr-ohlc-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-ohlc','candlestick-fr-ohlc-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-ohlc','candlestick-fr-ohlc-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-ohlc','candlestick-fr-ohlc-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-ohlc','candlestick-fr-ohlc-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-ohlc','candlestick-fr-ohlc-cpp',this)">C++</button>
</div><div id="candlestick-fr-ohlc-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="ohlc",
)
chart.show()</code></pre></div><div id="candlestick-fr-ohlc-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "ohlc",
});
chart.show();</code></pre></div><div id="candlestick-fr-ohlc-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "ohlc",
});
chart.show();</code></pre></div><div id="candlestick-fr-ohlc-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "ohlc"
)
chart$show()</code></pre></div><div id="candlestick-fr-ohlc-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("ohlc")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-fr-ohlc-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("ohlc")
    .build();
chart.show();</code></pre></div><div id="candlestick-fr-ohlc-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "ohlc"
);
chart.Show();</code></pre></div><div id="candlestick-fr-ohlc-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "ohlc"
)
chart.show()</code></pre></div><div id="candlestick-fr-ohlc-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "ohlc",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-ohlc.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-heikin">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heikin"</code></span><span><strong>Aliases</strong> <code>heikin / heikin_ashi / ha / smoothed</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bougies Heikin-Ashi lissees : filtre le bruit pour mettre en evidence tendances et retournements.</p>
<div class="sp-tabs" id="candlestick-fr-heikin">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-fr-heikin','candlestick-fr-heikin-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-heikin','candlestick-fr-heikin-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-heikin','candlestick-fr-heikin-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-heikin','candlestick-fr-heikin-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-heikin','candlestick-fr-heikin-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-heikin','candlestick-fr-heikin-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-heikin','candlestick-fr-heikin-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-heikin','candlestick-fr-heikin-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-heikin','candlestick-fr-heikin-cpp',this)">C++</button>
</div><div id="candlestick-fr-heikin-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="heikin",
)
chart.show()</code></pre></div><div id="candlestick-fr-heikin-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "heikin",
});
chart.show();</code></pre></div><div id="candlestick-fr-heikin-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "heikin",
});
chart.show();</code></pre></div><div id="candlestick-fr-heikin-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "heikin"
)
chart$show()</code></pre></div><div id="candlestick-fr-heikin-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("heikin")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-fr-heikin-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("heikin")
    .build();
chart.show();</code></pre></div><div id="candlestick-fr-heikin-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "heikin"
);
chart.Show();</code></pre></div><div id="candlestick-fr-heikin-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "heikin"
)
chart.show()</code></pre></div><div id="candlestick-fr-heikin-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "heikin",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-heikin.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-outlined">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / wireframe</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bougies en fil de fer : corps translucide avec contour colore epais ; rendu plus aere.</p>
<div class="sp-tabs" id="candlestick-fr-outlined">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-fr-outlined','candlestick-fr-outlined-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-outlined','candlestick-fr-outlined-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-outlined','candlestick-fr-outlined-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-outlined','candlestick-fr-outlined-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-outlined','candlestick-fr-outlined-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-outlined','candlestick-fr-outlined-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-outlined','candlestick-fr-outlined-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-outlined','candlestick-fr-outlined-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-outlined','candlestick-fr-outlined-cpp',this)">C++</button>
</div><div id="candlestick-fr-outlined-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="outlined",
)
chart.show()</code></pre></div><div id="candlestick-fr-outlined-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "outlined",
});
chart.show();</code></pre></div><div id="candlestick-fr-outlined-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "outlined",
});
chart.show();</code></pre></div><div id="candlestick-fr-outlined-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "outlined"
)
chart$show()</code></pre></div><div id="candlestick-fr-outlined-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("outlined")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-fr-outlined-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("outlined")
    .build();
chart.show();</code></pre></div><div id="candlestick-fr-outlined-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "outlined"
);
chart.Show();</code></pre></div><div id="candlestick-fr-outlined-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "outlined"
)
chart.show()</code></pre></div><div id="candlestick-fr-outlined-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "outlined",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-outlined.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-line">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"line"</code></span><span><strong>Aliases</strong> <code>line / close / lineplot / trend</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Ligne des prix de cloture avec marqueurs — meme donnee, lecture de tendance plus lisse.</p>
<div class="sp-tabs" id="candlestick-fr-line">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-fr-line','candlestick-fr-line-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-line','candlestick-fr-line-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-line','candlestick-fr-line-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-line','candlestick-fr-line-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-line','candlestick-fr-line-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-line','candlestick-fr-line-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-line','candlestick-fr-line-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-line','candlestick-fr-line-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-line','candlestick-fr-line-cpp',this)">C++</button>
</div><div id="candlestick-fr-line-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="line",
)
chart.show()</code></pre></div><div id="candlestick-fr-line-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "line",
});
chart.show();</code></pre></div><div id="candlestick-fr-line-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "line",
});
chart.show();</code></pre></div><div id="candlestick-fr-line-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "line"
)
chart$show()</code></pre></div><div id="candlestick-fr-line-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("line")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-fr-line-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("line")
    .build();
chart.show();</code></pre></div><div id="candlestick-fr-line-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "line"
);
chart.Show();</code></pre></div><div id="candlestick-fr-line-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "line"
)
chart.show()</code></pre></div><div id="candlestick-fr-line-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "line",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-line.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-mountain">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mountain"</code></span><span><strong>Aliases</strong> <code>mountain / area / filled_area / shade</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Aire des prix de cloture avec degrade vertical sous la courbe ; ideal pour visuel de couverture.</p>
<div class="sp-tabs" id="candlestick-fr-mountain">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-fr-mountain','candlestick-fr-mountain-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-mountain','candlestick-fr-mountain-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-mountain','candlestick-fr-mountain-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-mountain','candlestick-fr-mountain-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-mountain','candlestick-fr-mountain-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-mountain','candlestick-fr-mountain-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-mountain','candlestick-fr-mountain-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-mountain','candlestick-fr-mountain-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-mountain','candlestick-fr-mountain-cpp',this)">C++</button>
</div><div id="candlestick-fr-mountain-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="mountain",
)
chart.show()</code></pre></div><div id="candlestick-fr-mountain-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "mountain",
});
chart.show();</code></pre></div><div id="candlestick-fr-mountain-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "mountain",
});
chart.show();</code></pre></div><div id="candlestick-fr-mountain-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "mountain"
)
chart$show()</code></pre></div><div id="candlestick-fr-mountain-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("mountain")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-fr-mountain-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("mountain")
    .build();
chart.show();</code></pre></div><div id="candlestick-fr-mountain-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "mountain"
);
chart.Show();</code></pre></div><div id="candlestick-fr-mountain-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "mountain"
)
chart.show()</code></pre></div><div id="candlestick-fr-mountain-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "mountain",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-mountain.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-range">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"range"</code></span><span><strong>Aliases</strong> <code>range / hl / highlow / spread</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres haut-bas uniquement (sans ouverture/cloture), une seule couleur — visualisation de la volatilite.</p>
<div class="sp-tabs" id="candlestick-fr-range">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('candlestick-fr-range','candlestick-fr-range-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-range','candlestick-fr-range-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-range','candlestick-fr-range-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-range','candlestick-fr-range-r',this)">R</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-range','candlestick-fr-range-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-range','candlestick-fr-range-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-range','candlestick-fr-range-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-range','candlestick-fr-range-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('candlestick-fr-range','candlestick-fr-range-cpp',this)">C++</button>
</div><div id="candlestick-fr-range-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels = ["D01","D02","D03","D04","D05","D06","D07","D08","D09","D10"]
open_  = [100, 102, 101, 104, 103, 105, 107, 106, 108, 110]
high   = [103, 104, 105, 106, 106, 108, 109, 109, 111, 113]
low    = [ 99, 100, 100, 102, 101, 103, 105, 105, 107, 109]
close  = [102, 101, 104, 103, 105, 107, 106, 108, 110, 112]

chart = sp.candlestick(
    title="ACME Corp",
    labels=labels,
    open=open_, high=high, low=low, close=close,
    palette=[0x22C55E, 0xEF4444],
    gridlines=True,
    x_label="Day", y_label="Price ($)",
    variant="range",
)
chart.show()</code></pre></div><div id="candlestick-fr-range-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "range",
});
chart.show();</code></pre></div><div id="candlestick-fr-range-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.candlestick({
  title: "ACME Corp",
  labels: [/* ... */],
  open: [/* ... */], high: [/* ... */], low: [/* ... */], close: [/* ... */],
  variant: "range",
});
chart.show();</code></pre></div><div id="candlestick-fr-range-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$candlestick(
  title = "ACME Corp",
  labels = c(),
  open = c(), high = c(), low = c(), close = c(),
  variant = "range"
)
chart$show()</code></pre></div><div id="candlestick-fr-range-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::candlestick()
        .title("ACME Corp")
        .labels(vec![])
        .open(vec![]).high(vec![]).low(vec![]).close(vec![])
        .variant("range")
        .build();
    chart.show();
}</code></pre></div><div id="candlestick-fr-range-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.candlestick()
    .title("ACME Corp")
    .labels(List.of())
    .open(List.of()).high(List.of()).low(List.of()).close(List.of())
    .variant("range")
    .build();
chart.show();</code></pre></div><div id="candlestick-fr-range-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Candlestick(
    title: "ACME Corp",
    labels: new string[]{},
    open: new double[]{}, high: new double[]{}, low: new double[]{}, close: new double[]{},
    variant: "range"
);
chart.Show();</code></pre></div><div id="candlestick-fr-range-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.candlestick(
  title   = "ACME Corp",
  labels  = List(),
  open    = List(), high = List(), low = List(), close = List(),
  variant = "range"
)
chart.show()</code></pre></div><div id="candlestick-fr-range-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::candlestick({
    .title   = "ACME Corp",
    .labels  = {},
    .open    = {}, .high = {}, .low = {}, .close = {},
    .variant = "range",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-range.html"></iframe>
</div>

</div>
</div>

</div>
