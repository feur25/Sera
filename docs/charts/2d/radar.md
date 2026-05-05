# Radar — Spider / Star Chart

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
.sp-preview-frame{width:100%;height:560px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.radar(title, axes, series, *, series_names=None, variant="basic", filled=True, fill_opacity=50, palette=None, **kwargs) -> Chart`

Aliases: `sp.radar`, `sp.build_radar_chart`

## Description

`sp.radar()` is the unified entry point for the entire radar / spider / star chart family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. Radar charts are ideal for multivariate comparison across 3+ axes — performance profiles, KPIs, skill maps, scoring systems. SeraPlot draws everything in pure Rust SVG with concentric grid rings, axis lines, automatic ring tick labels, optional legend and per-series palette colors. The polar-bar variant turns the chart into a categorical polar histogram, the stacked variant builds a cumulative composition view.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | Filled polygons + dots — standard radar | `palette`, `fill_opacity` |
| `"lines"` | Stroke-only outlines for many series | `palette` |
| `"filled"` | Strong fills, sorted back-to-front | `palette`, `fill_opacity` |
| `"markers"` | Light line + outlined dots | `palette` |
| `"dashed"` | Dashed outline polygons | `palette` |
| `"stacked"` | Cumulative stacked polygons | `palette`, `fill_opacity` |
| `"polar_bar"` | Radial bars per axis (grouped) | `palette`, `fill_opacity` |
| `"gradient"` | Radial center→edge gradient | `palette` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | required | all | Chart title |
| `axes` | `list[str]` | required | all | Axis labels (≥3) |
| `series` | `list[list[float]]` | required | all | One row per series, length must match `axes` |
| `series_names` | `list[str]` | `None` | all | Legend labels (one per series) |
| `variant` | `str` | `"basic"` | — | Rendering variant (see table) |
| `filled` | `bool` | `True` | basic | Fill polygons (basic only) |
| `fill_opacity` | `int` | `50` | filled variants | Fill alpha 0..255 |
| `palette` | `list[int]` | `None` | all | Custom per-series palette |
| `width` | `int` | `700` | all | Canvas width (px) |
| `height` | `int` | `560` | all | Canvas height (px) |
| `background` | `str` | `None` | all | Background CSS color |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="radar-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('radar-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('radar-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','lines',this)"><span class="sp-cic">L</span><span class="sp-clb">Lines</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','filled',this)"><span class="sp-cic">F</span><span class="sp-clb">Filled</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','markers',this)"><span class="sp-cic">M</span><span class="sp-clb">Markers</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','dashed',this)"><span class="sp-cic">D</span><span class="sp-clb">Dashed</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','polar_bar',this)"><span class="sp-cic">P</span><span class="sp-clb">PolarBar</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-en','gradient',this)"><span class="sp-cic">G</span><span class="sp-clb">Gradient</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="radar-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Filled polygon per series with stroke and dot markers — the standard radar.</p>
<div class="sp-tabs" id="rdgen-radar-en-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-en-basic','rdgen-radar-en-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-basic','rdgen-radar-en-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-basic','rdgen-radar-en-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-basic','rdgen-radar-en-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-basic','rdgen-radar-en-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-basic','rdgen-radar-en-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-basic','rdgen-radar-en-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-basic','rdgen-radar-en-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-basic','rdgen-radar-en-basic-cpp',this)">C++</button>
</div><div id="rdgen-radar-en-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="basic",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-en-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "basic",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "basic",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "basic"
)
chart$show()</code></pre></div><div id="rdgen-radar-en-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-en-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-en-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "basic"
);
chart.Show();</code></pre></div><div id="rdgen-radar-en-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "basic"
)
chart.show()</code></pre></div><div id="rdgen-radar-en-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-basic.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-lines">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"lines"</code></span><span><strong>Aliases</strong> <code>lines / outline / stroke / no_fill</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stroke-only polygons, no fill — clean overlay for many series.</p>
<div class="sp-tabs" id="rdgen-radar-en-lines">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-en-lines','rdgen-radar-en-lines-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-lines','rdgen-radar-en-lines-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-lines','rdgen-radar-en-lines-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-lines','rdgen-radar-en-lines-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-lines','rdgen-radar-en-lines-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-lines','rdgen-radar-en-lines-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-lines','rdgen-radar-en-lines-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-lines','rdgen-radar-en-lines-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-lines','rdgen-radar-en-lines-cpp',this)">C++</button>
</div><div id="rdgen-radar-en-lines-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="lines",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-en-lines-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "lines",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-lines-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "lines",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-lines-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "lines"
)
chart$show()</code></pre></div><div id="rdgen-radar-en-lines-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("lines")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-en-lines-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("lines")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-en-lines-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "lines"
);
chart.Show();</code></pre></div><div id="rdgen-radar-en-lines-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "lines"
)
chart.show()</code></pre></div><div id="rdgen-radar-en-lines-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "lines",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-lines.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-filled">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"filled"</code></span><span><strong>Aliases</strong> <code>filled / fill / solid / area</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Strong fill, no stroke, sorted back-to-front by total area for clarity.</p>
<div class="sp-tabs" id="rdgen-radar-en-filled">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-en-filled','rdgen-radar-en-filled-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-filled','rdgen-radar-en-filled-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-filled','rdgen-radar-en-filled-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-filled','rdgen-radar-en-filled-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-filled','rdgen-radar-en-filled-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-filled','rdgen-radar-en-filled-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-filled','rdgen-radar-en-filled-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-filled','rdgen-radar-en-filled-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-filled','rdgen-radar-en-filled-cpp',this)">C++</button>
</div><div id="rdgen-radar-en-filled-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="filled",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-en-filled-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "filled",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-filled-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "filled",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-filled-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "filled"
)
chart$show()</code></pre></div><div id="rdgen-radar-en-filled-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("filled")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-en-filled-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("filled")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-en-filled-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "filled"
);
chart.Show();</code></pre></div><div id="rdgen-radar-en-filled-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "filled"
)
chart.show()</code></pre></div><div id="rdgen-radar-en-filled-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "filled",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-filled.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-markers">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"markers"</code></span><span><strong>Aliases</strong> <code>markers / dots / points / marker</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Light stroke + bold outlined markers — emphasis on data points.</p>
<div class="sp-tabs" id="rdgen-radar-en-markers">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-en-markers','rdgen-radar-en-markers-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-markers','rdgen-radar-en-markers-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-markers','rdgen-radar-en-markers-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-markers','rdgen-radar-en-markers-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-markers','rdgen-radar-en-markers-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-markers','rdgen-radar-en-markers-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-markers','rdgen-radar-en-markers-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-markers','rdgen-radar-en-markers-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-markers','rdgen-radar-en-markers-cpp',this)">C++</button>
</div><div id="rdgen-radar-en-markers-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="markers",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-en-markers-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "markers",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-markers-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "markers",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-markers-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "markers"
)
chart$show()</code></pre></div><div id="rdgen-radar-en-markers-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("markers")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-en-markers-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("markers")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-en-markers-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "markers"
);
chart.Show();</code></pre></div><div id="rdgen-radar-en-markers-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "markers"
)
chart.show()</code></pre></div><div id="rdgen-radar-en-markers-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "markers",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-markers.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-dashed">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dashed"</code></span><span><strong>Aliases</strong> <code>dashed / dash / dotted</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Dashed outline polygons — useful for projections, targets, baselines.</p>
<div class="sp-tabs" id="rdgen-radar-en-dashed">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-en-dashed','rdgen-radar-en-dashed-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-dashed','rdgen-radar-en-dashed-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-dashed','rdgen-radar-en-dashed-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-dashed','rdgen-radar-en-dashed-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-dashed','rdgen-radar-en-dashed-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-dashed','rdgen-radar-en-dashed-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-dashed','rdgen-radar-en-dashed-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-dashed','rdgen-radar-en-dashed-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-dashed','rdgen-radar-en-dashed-cpp',this)">C++</button>
</div><div id="rdgen-radar-en-dashed-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="dashed",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-en-dashed-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "dashed",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-dashed-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "dashed",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-dashed-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "dashed"
)
chart$show()</code></pre></div><div id="rdgen-radar-en-dashed-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("dashed")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-en-dashed-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("dashed")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-en-dashed-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "dashed"
);
chart.Show();</code></pre></div><div id="rdgen-radar-en-dashed-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "dashed"
)
chart.show()</code></pre></div><div id="rdgen-radar-en-dashed-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "dashed",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-dashed.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-stacked">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stacked"</code></span><span><strong>Aliases</strong> <code>stacked / stack / cumulative</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Cumulative stacking on each axis — visualizes part-of-whole composition.</p>
<div class="sp-tabs" id="rdgen-radar-en-stacked">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-en-stacked','rdgen-radar-en-stacked-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-stacked','rdgen-radar-en-stacked-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-stacked','rdgen-radar-en-stacked-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-stacked','rdgen-radar-en-stacked-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-stacked','rdgen-radar-en-stacked-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-stacked','rdgen-radar-en-stacked-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-stacked','rdgen-radar-en-stacked-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-stacked','rdgen-radar-en-stacked-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-stacked','rdgen-radar-en-stacked-cpp',this)">C++</button>
</div><div id="rdgen-radar-en-stacked-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="stacked",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-en-stacked-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "stacked",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-stacked-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "stacked",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-stacked-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "stacked"
)
chart$show()</code></pre></div><div id="rdgen-radar-en-stacked-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("stacked")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-en-stacked-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("stacked")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-en-stacked-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "stacked"
);
chart.Show();</code></pre></div><div id="rdgen-radar-en-stacked-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "stacked"
)
chart.show()</code></pre></div><div id="rdgen-radar-en-stacked-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "stacked",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-stacked.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-polar_bar">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"polar_bar"</code></span><span><strong>Aliases</strong> <code>polar_bar / polar / bar / radial_bar</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Radial bars per axis grouped by series — categorical polar histogram.</p>
<div class="sp-tabs" id="rdgen-radar-en-polar_bar">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-en-polar_bar','rdgen-radar-en-polar_bar-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-polar_bar','rdgen-radar-en-polar_bar-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-polar_bar','rdgen-radar-en-polar_bar-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-polar_bar','rdgen-radar-en-polar_bar-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-polar_bar','rdgen-radar-en-polar_bar-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-polar_bar','rdgen-radar-en-polar_bar-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-polar_bar','rdgen-radar-en-polar_bar-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-polar_bar','rdgen-radar-en-polar_bar-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-polar_bar','rdgen-radar-en-polar_bar-cpp',this)">C++</button>
</div><div id="rdgen-radar-en-polar_bar-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="polar_bar",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-en-polar_bar-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "polar_bar",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-polar_bar-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "polar_bar",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-polar_bar-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "polar_bar"
)
chart$show()</code></pre></div><div id="rdgen-radar-en-polar_bar-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("polar_bar")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-en-polar_bar-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("polar_bar")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-en-polar_bar-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "polar_bar"
);
chart.Show();</code></pre></div><div id="rdgen-radar-en-polar_bar-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "polar_bar"
)
chart.show()</code></pre></div><div id="rdgen-radar-en-polar_bar-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "polar_bar",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-polar_bar.html"></iframe>
</div>
<div class="sp-variant" id="radar-en-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / radial_gradient / shade / fade</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Radial gradient fill from center (opaque) to edge (transparent).</p>
<div class="sp-tabs" id="rdgen-radar-en-gradient">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-en-gradient','rdgen-radar-en-gradient-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-gradient','rdgen-radar-en-gradient-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-gradient','rdgen-radar-en-gradient-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-gradient','rdgen-radar-en-gradient-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-gradient','rdgen-radar-en-gradient-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-gradient','rdgen-radar-en-gradient-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-gradient','rdgen-radar-en-gradient-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-gradient','rdgen-radar-en-gradient-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-en-gradient','rdgen-radar-en-gradient-cpp',this)">C++</button>
</div><div id="rdgen-radar-en-gradient-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="gradient",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-en-gradient-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "gradient",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-gradient-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "gradient",
});
chart.show();</code></pre></div><div id="rdgen-radar-en-gradient-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "gradient"
)
chart$show()</code></pre></div><div id="rdgen-radar-en-gradient-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("gradient")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-en-gradient-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("gradient")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-en-gradient-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "gradient"
);
chart.Show();</code></pre></div><div id="rdgen-radar-en-gradient-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "gradient"
)
chart.show()</code></pre></div><div id="rdgen-radar-en-gradient-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "gradient",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/radar-gradient.html"></iframe>
</div>
</div></div>

</div>

<div class="lang-fr">

## Signature

`sp.radar(title, axes, series, *, series_names=None, variant="basic", filled=True, fill_opacity=50, palette=None, **kwargs) -> Chart`

Alias : `sp.radar`, `sp.build_radar_chart`

## Description

`sp.radar()` est le point d'entrée unifié pour toute la famille radar / spider / star. Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments conservent le même nom d'une variante à l'autre. Le radar est idéal pour comparer plusieurs séries sur 3 axes ou plus — profils de performance, KPI, cartographie de compétences, systèmes de notation. SeraPlot dessine tout en SVG Rust natif avec anneaux de grille concentriques, axes, labels automatiques de graduation, légende optionnelle et couleurs de palette par série. La variante polar_bar transforme le radar en histogramme polaire catégoriel, la variante stacked construit une vue de composition cumulative.

| Variante | Usage | Args clés |
|----------|-------|-----------|
| `"basic"` | Polygones remplis + points — radar standard | `palette`, `fill_opacity` |
| `"lines"` | Contours seuls pour multi-séries | `palette` |
| `"filled"` | Remplissages forts, triés arrière→avant | `palette`, `fill_opacity` |
| `"markers"` | Trait léger + points détourés | `palette` |
| `"dashed"` | Polygones à contour pointillé | `palette` |
| `"stacked"` | Polygones empilés cumulés | `palette`, `fill_opacity` |
| `"polar_bar"` | Barres radiales par axe (groupées) | `palette`, `fill_opacity` |
| `"gradient"` | Dégradé radial centre→bord | `palette` |

---

## Paramètres

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `axes` | `list[str]` | requis | toutes | Libellés d'axes (≥3) |
| `series` | `list[list[float]]` | requis | toutes | Une ligne par série, longueur = `axes` |
| `series_names` | `list[str]` | `None` | toutes | Libellés de légende |
| `variant` | `str` | `"basic"` | — | Variante de rendu |
| `filled` | `bool` | `True` | basic | Remplir les polygones (basic seul) |
| `fill_opacity` | `int` | `50` | variantes remplies | Alpha 0..255 |
| `palette` | `list[int]` | `None` | toutes | Palette personnalisée |
| `width` | `int` | `700` | toutes | Largeur (px) |
| `height` | `int` | `560` | toutes | Hauteur (px) |
| `background` | `str` | `None` | toutes | Couleur de fond CSS |

---

## Retour

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

<div class="sp-cls sp-open" id="radar-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('radar-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('radar-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','lines',this)"><span class="sp-cic">L</span><span class="sp-clb">Lines</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','filled',this)"><span class="sp-cic">F</span><span class="sp-clb">Filled</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','markers',this)"><span class="sp-cic">M</span><span class="sp-clb">Markers</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','dashed',this)"><span class="sp-cic">D</span><span class="sp-clb">Dashed</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','stacked',this)"><span class="sp-cic">S</span><span class="sp-clb">Stacked</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','polar_bar',this)"><span class="sp-cic">P</span><span class="sp-clb">PolarBar</span></button>
<button class="sp-cls-tab" onclick="spCls('radar-fr','gradient',this)"><span class="sp-cic">G</span><span class="sp-clb">Gradient</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="radar-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Polygone rempli par série avec contour et points — le radar standard.</p>
<div class="sp-tabs" id="rdgen-radar-fr-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-fr-basic','rdgen-radar-fr-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-basic','rdgen-radar-fr-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-basic','rdgen-radar-fr-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-basic','rdgen-radar-fr-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-basic','rdgen-radar-fr-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-basic','rdgen-radar-fr-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-basic','rdgen-radar-fr-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-basic','rdgen-radar-fr-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-basic','rdgen-radar-fr-basic-cpp',this)">C++</button>
</div><div id="rdgen-radar-fr-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="basic",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "basic",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "basic",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "basic"
)
chart$show()</code></pre></div><div id="rdgen-radar-fr-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-fr-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-fr-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "basic"
);
chart.Show();</code></pre></div><div id="rdgen-radar-fr-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "basic"
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-basic.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-lines">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"lines"</code></span><span><strong>Alias</strong> <code>lines / outline / stroke / no_fill</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Polygones en trait seul, sans remplissage — overlay net pour plusieurs séries.</p>
<div class="sp-tabs" id="rdgen-radar-fr-lines">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-fr-lines','rdgen-radar-fr-lines-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-lines','rdgen-radar-fr-lines-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-lines','rdgen-radar-fr-lines-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-lines','rdgen-radar-fr-lines-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-lines','rdgen-radar-fr-lines-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-lines','rdgen-radar-fr-lines-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-lines','rdgen-radar-fr-lines-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-lines','rdgen-radar-fr-lines-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-lines','rdgen-radar-fr-lines-cpp',this)">C++</button>
</div><div id="rdgen-radar-fr-lines-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="lines",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-lines-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "lines",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-lines-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "lines",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-lines-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "lines"
)
chart$show()</code></pre></div><div id="rdgen-radar-fr-lines-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("lines")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-fr-lines-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("lines")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-fr-lines-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "lines"
);
chart.Show();</code></pre></div><div id="rdgen-radar-fr-lines-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "lines"
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-lines-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "lines",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-lines.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-filled">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"filled"</code></span><span><strong>Alias</strong> <code>filled / fill / solid / area</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Remplissage fort sans contour, trié de l'arrière vers l'avant par aire totale.</p>
<div class="sp-tabs" id="rdgen-radar-fr-filled">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-fr-filled','rdgen-radar-fr-filled-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-filled','rdgen-radar-fr-filled-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-filled','rdgen-radar-fr-filled-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-filled','rdgen-radar-fr-filled-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-filled','rdgen-radar-fr-filled-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-filled','rdgen-radar-fr-filled-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-filled','rdgen-radar-fr-filled-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-filled','rdgen-radar-fr-filled-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-filled','rdgen-radar-fr-filled-cpp',this)">C++</button>
</div><div id="rdgen-radar-fr-filled-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="filled",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-filled-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "filled",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-filled-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "filled",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-filled-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "filled"
)
chart$show()</code></pre></div><div id="rdgen-radar-fr-filled-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("filled")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-fr-filled-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("filled")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-fr-filled-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "filled"
);
chart.Show();</code></pre></div><div id="rdgen-radar-fr-filled-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "filled"
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-filled-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "filled",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-filled.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-markers">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"markers"</code></span><span><strong>Alias</strong> <code>markers / dots / points / marker</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trait léger + marqueurs détourés — accent sur les points de données.</p>
<div class="sp-tabs" id="rdgen-radar-fr-markers">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-fr-markers','rdgen-radar-fr-markers-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-markers','rdgen-radar-fr-markers-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-markers','rdgen-radar-fr-markers-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-markers','rdgen-radar-fr-markers-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-markers','rdgen-radar-fr-markers-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-markers','rdgen-radar-fr-markers-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-markers','rdgen-radar-fr-markers-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-markers','rdgen-radar-fr-markers-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-markers','rdgen-radar-fr-markers-cpp',this)">C++</button>
</div><div id="rdgen-radar-fr-markers-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="markers",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-markers-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "markers",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-markers-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "markers",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-markers-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "markers"
)
chart$show()</code></pre></div><div id="rdgen-radar-fr-markers-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("markers")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-fr-markers-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("markers")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-fr-markers-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "markers"
);
chart.Show();</code></pre></div><div id="rdgen-radar-fr-markers-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "markers"
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-markers-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "markers",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-markers.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-dashed">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"dashed"</code></span><span><strong>Alias</strong> <code>dashed / dash / dotted</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Polygones à contour pointillé — utile pour projections, cibles, références.</p>
<div class="sp-tabs" id="rdgen-radar-fr-dashed">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-fr-dashed','rdgen-radar-fr-dashed-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-dashed','rdgen-radar-fr-dashed-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-dashed','rdgen-radar-fr-dashed-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-dashed','rdgen-radar-fr-dashed-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-dashed','rdgen-radar-fr-dashed-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-dashed','rdgen-radar-fr-dashed-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-dashed','rdgen-radar-fr-dashed-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-dashed','rdgen-radar-fr-dashed-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-dashed','rdgen-radar-fr-dashed-cpp',this)">C++</button>
</div><div id="rdgen-radar-fr-dashed-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="dashed",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-dashed-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "dashed",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-dashed-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "dashed",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-dashed-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "dashed"
)
chart$show()</code></pre></div><div id="rdgen-radar-fr-dashed-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("dashed")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-fr-dashed-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("dashed")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-fr-dashed-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "dashed"
);
chart.Show();</code></pre></div><div id="rdgen-radar-fr-dashed-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "dashed"
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-dashed-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "dashed",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-dashed.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-stacked">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"stacked"</code></span><span><strong>Alias</strong> <code>stacked / stack / cumulative</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Empilement cumulatif sur chaque axe — visualise une composition part/tout.</p>
<div class="sp-tabs" id="rdgen-radar-fr-stacked">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-fr-stacked','rdgen-radar-fr-stacked-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-stacked','rdgen-radar-fr-stacked-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-stacked','rdgen-radar-fr-stacked-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-stacked','rdgen-radar-fr-stacked-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-stacked','rdgen-radar-fr-stacked-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-stacked','rdgen-radar-fr-stacked-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-stacked','rdgen-radar-fr-stacked-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-stacked','rdgen-radar-fr-stacked-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-stacked','rdgen-radar-fr-stacked-cpp',this)">C++</button>
</div><div id="rdgen-radar-fr-stacked-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="stacked",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-stacked-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "stacked",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-stacked-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "stacked",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-stacked-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "stacked"
)
chart$show()</code></pre></div><div id="rdgen-radar-fr-stacked-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("stacked")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-fr-stacked-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("stacked")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-fr-stacked-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "stacked"
);
chart.Show();</code></pre></div><div id="rdgen-radar-fr-stacked-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "stacked"
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-stacked-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "stacked",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-stacked.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-polar_bar">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"polar_bar"</code></span><span><strong>Alias</strong> <code>polar_bar / polar / bar / radial_bar</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres radiales par axe groupées par série — histogramme polaire catégoriel.</p>
<div class="sp-tabs" id="rdgen-radar-fr-polar_bar">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-fr-polar_bar','rdgen-radar-fr-polar_bar-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-polar_bar','rdgen-radar-fr-polar_bar-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-polar_bar','rdgen-radar-fr-polar_bar-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-polar_bar','rdgen-radar-fr-polar_bar-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-polar_bar','rdgen-radar-fr-polar_bar-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-polar_bar','rdgen-radar-fr-polar_bar-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-polar_bar','rdgen-radar-fr-polar_bar-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-polar_bar','rdgen-radar-fr-polar_bar-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-polar_bar','rdgen-radar-fr-polar_bar-cpp',this)">C++</button>
</div><div id="rdgen-radar-fr-polar_bar-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="polar_bar",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-polar_bar-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "polar_bar",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-polar_bar-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "polar_bar",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-polar_bar-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "polar_bar"
)
chart$show()</code></pre></div><div id="rdgen-radar-fr-polar_bar-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("polar_bar")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-fr-polar_bar-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("polar_bar")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-fr-polar_bar-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "polar_bar"
);
chart.Show();</code></pre></div><div id="rdgen-radar-fr-polar_bar-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "polar_bar"
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-polar_bar-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "polar_bar",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-polar_bar.html"></iframe>
</div>
<div class="sp-variant" id="radar-fr-gradient">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code></span><span><strong>Alias</strong> <code>gradient / radial_gradient / shade / fade</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Remplissage en dégradé radial du centre (opaque) vers le bord (transparent).</p>
<div class="sp-tabs" id="rdgen-radar-fr-gradient">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('rdgen-radar-fr-gradient','rdgen-radar-fr-gradient-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-gradient','rdgen-radar-fr-gradient-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-gradient','rdgen-radar-fr-gradient-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-gradient','rdgen-radar-fr-gradient-r',this)">R</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-gradient','rdgen-radar-fr-gradient-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-gradient','rdgen-radar-fr-gradient-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-gradient','rdgen-radar-fr-gradient-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-gradient','rdgen-radar-fr-gradient-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('rdgen-radar-fr-gradient','rdgen-radar-fr-gradient-cpp',this)">C++</button>
</div><div id="rdgen-radar-fr-gradient-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

axes = ["Speed", "Power", "Range", "Comfort", "Price", "Style", "Tech"]
series = [
    [82, 78, 88, 62, 58, 72, 80],
    [68, 86, 72, 82, 74, 68, 90],
    [72, 64, 82, 76, 86, 84, 70],
]
chart = sp.radar(
    title="Vehicle Comparison",
    axes=axes,
    series=series,
    series_names=["Alpha", "Beta", "Gamma"],
    variant="gradient",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    fill_opacity=80,
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-gradient-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "gradient",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-gradient-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Radar",
  axes: ["A", "B", "C", "D", "E"],
  series: [[/* row 1 */], [/* row 2 */]],
  seriesNames: ["S1", "S2"],
  variant: "gradient",
});
chart.show();</code></pre></div><div id="rdgen-radar-fr-gradient-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title = "Radar",
  axes = c("A", "B", "C", "D", "E"),
  series = list(c(), c()),
  series_names = c("S1", "S2"),
  variant = "gradient"
)
chart$show()</code></pre></div><div id="rdgen-radar-fr-gradient-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::radar()
        .title("Radar")
        .axes(vec!["A", "B", "C", "D", "E"])
        .series(vec![vec![], vec![]])
        .series_names(vec!["S1", "S2"])
        .variant("gradient")
        .build();
    chart.show();
}</code></pre></div><div id="rdgen-radar-fr-gradient-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.radar()
    .title("Radar")
    .axes(List.of("A", "B", "C", "D", "E"))
    .series(List.of(List.of(), List.of()))
    .seriesNames(List.of("S1", "S2"))
    .variant("gradient")
    .build();
chart.show();</code></pre></div><div id="rdgen-radar-fr-gradient-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Radar(
    title: "Radar",
    axes: new[]{ "A", "B", "C", "D", "E" },
    series: new[]{ new double[]{}, new double[]{} },
    seriesNames: new[]{ "S1", "S2" },
    variant: "gradient"
);
chart.Show();</code></pre></div><div id="rdgen-radar-fr-gradient-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.radar(
  title        = "Radar",
  axes         = List("A", "B", "C", "D", "E"),
  series       = List(List(), List()),
  seriesNames  = List("S1", "S2"),
  variant      = "gradient"
)
chart.show()</code></pre></div><div id="rdgen-radar-fr-gradient-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::radar({
    .title        = "Radar",
    .axes         = {"A", "B", "C", "D", "E"},
    .series       = { {}, {} },
    .series_names = {"S1", "S2"},
    .variant      = "gradient",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/radar-gradient.html"></iframe>
</div>
</div></div>

</div>

