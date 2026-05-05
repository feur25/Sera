# KDE — Kernel Density Estimate

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
.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.kde(title, values, *, variant="basic", categories=None, bandwidth=0.0, filled=True, fill_opacity=50, bins=30, n_points=80, palette=None, **kwargs) -> Chart`

Aliases: `sp.kde`, `sp.build_kde_chart`

## Description

`sp.kde()` is the unified entry point for the entire Kernel Density Estimate family. The `variant` keyword selects the rendering strategy — every other argument keeps the same name across variants. KDE produces a smooth, continuous density estimate from a sample of points using a Gaussian kernel with Scott's rule for automatic bandwidth selection. SeraPlot renders the curves as pure Rust SVG, with native multi-series, normalization, CDF, rug, histogram overlay and gradient fills.

| Variant | Use case | Key extra args |
|---------|----------|----------------|
| `"basic"` | One or more filled density curves | `categories`, `palette` |
| `"outline"` | Stroke-only curves for clean overlays | `palette` |
| `"stepped"` | Step-shaped density (rectangular) | `fill_opacity` |
| `"rug"` | KDE + rug ticks at sample positions | `categories` |
| `"histogram"` | KDE on top of normalized histogram | `bins` |
| `"normalized"` | Each series rescaled to area = 1 (true PDF) | `categories` |
| `"cumulative"` | Cumulative density (CDF) curves in [0, 1] | `categories` |
| `"gradient"` | Vertical gradient fill (opaque → transparent) | `palette` |

---

## Parameters

| Parameter | Type | Default | Variants | Description |
|-----------|------|---------|----------|-------------|
| `title` | `str` | required | all | Chart title |
| `values` | `list[float]` | required | all | Raw numeric samples |
| `variant` | `str` | `"basic"` | — | Rendering variant (see table) |
| `categories` | `list[str]` | `None` | all | Per-value group label for multi-series |
| `bandwidth` | `float` | `0.0` | all | KDE bandwidth. 0 = Scott's rule (auto) |
| `filled` | `bool` | `True` | basic, stepped, normalized | Fill area under curve |
| `fill_opacity` | `int` | `50` | all (filled) | Fill alpha in 0..255 |
| `bins` | `int` | `30` | histogram | Histogram bin count |
| `n_points` | `int` | `80` | all | KDE evaluation points along X |
| `palette` | `list[int]` | `None` | all | Custom per-series palette |
| `width` | `int` | `900` | all | Canvas width (px) |
| `height` | `int` | `420` | all | Canvas height (px) |
| `x_label` | `str` | `""` | all | X-axis label |
| `y_label` | `str` | `"Density"` | all | Y-axis label |
| `gridlines` | `bool` | `False` | all | Show gridlines |
| `sort_order` | `str` | `"none"` | all | `none` / `asc` / `desc` (sort series by mean) |
| `background` | `str` | `None` | all | Background CSS color |

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="kde-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('kde-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('kde-en','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','outline',this)"><span class="sp-cic">O</span><span class="sp-clb">Outline</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','stepped',this)"><span class="sp-cic">T</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','rug',this)"><span class="sp-cic">R</span><span class="sp-clb">Rug</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','histogram',this)"><span class="sp-cic">H</span><span class="sp-clb">Histogram</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','normalized',this)"><span class="sp-cic">N</span><span class="sp-clb">Normalized</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','cumulative',this)"><span class="sp-cic">C</span><span class="sp-clb">Cumulative</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-en','gradient',this)"><span class="sp-cic">G</span><span class="sp-clb">Gradient</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="kde-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / filled / default / single / multi</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Filled curve, single or multi-series.</p>
<div class="sp-tabs" id="kgen-kde-en-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-en-basic','kgen-kde-en-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-basic','kgen-kde-en-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-basic','kgen-kde-en-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-basic','kgen-kde-en-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-basic','kgen-kde-en-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-basic','kgen-kde-en-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-basic','kgen-kde-en-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-basic','kgen-kde-en-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-basic','kgen-kde-en-basic-cpp',this)">C++</button>
</div><div id="kgen-kde-en-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
a = [random.gauss(-1.5, 0.7) for _ in range(500)]
b = [random.gauss(0.5, 1.0) for _ in range(500)]
c = [random.gauss(2.5, 0.6) for _ in range(500)]
chart = sp.kde(
    title="Density",
    values=a + b + c,
    categories=["A"]*500 + ["B"]*500 + ["C"]*500,
    variant="basic",
    palette=[0x6366F1, 0xF59E0B, 0xEF4444],
    x_label="value", y_label="density",
    gridlines=True,
)
chart.show()</code></pre></div><div id="kgen-kde-en-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="kgen-kde-en-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="kgen-kde-en-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="kgen-kde-en-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-en-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-en-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="kgen-kde-en-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="kgen-kde-en-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-basic.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-outline">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outline"</code></span><span><strong>Aliases</strong> <code>outline / line / stroke / compare / no_fill</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stroke-only curves for clean overlays.</p>
<div class="sp-tabs" id="kgen-kde-en-outline">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-en-outline','kgen-kde-en-outline-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-outline','kgen-kde-en-outline-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-outline','kgen-kde-en-outline-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-outline','kgen-kde-en-outline-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-outline','kgen-kde-en-outline-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-outline','kgen-kde-en-outline-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-outline','kgen-kde-en-outline-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-outline','kgen-kde-en-outline-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-outline','kgen-kde-en-outline-cpp',this)">C++</button>
</div><div id="kgen-kde-en-outline-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="Density (Outline)",
    values=values,
    categories=groups,
    variant="outline",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    x_label="value", y_label="density",
)
chart.show()</code></pre></div><div id="kgen-kde-en-outline-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "outline",
});
chart.show();</code></pre></div><div id="kgen-kde-en-outline-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "outline",
});
chart.show();</code></pre></div><div id="kgen-kde-en-outline-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "outline"
)
chart$show()</code></pre></div><div id="kgen-kde-en-outline-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("outline")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-en-outline-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("outline")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-en-outline-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "outline"
);
chart.Show();</code></pre></div><div id="kgen-kde-en-outline-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "outline"
)
chart.show()</code></pre></div><div id="kgen-kde-en-outline-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "outline",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-outline.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-stepped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>stepped / step / stair / stairs</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stair-stepped density (rectangular look).</p>
<div class="sp-tabs" id="kgen-kde-en-stepped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-en-stepped','kgen-kde-en-stepped-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-stepped','kgen-kde-en-stepped-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-stepped','kgen-kde-en-stepped-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-stepped','kgen-kde-en-stepped-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-stepped','kgen-kde-en-stepped-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-stepped','kgen-kde-en-stepped-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-stepped','kgen-kde-en-stepped-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-stepped','kgen-kde-en-stepped-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-stepped','kgen-kde-en-stepped-cpp',this)">C++</button>
</div><div id="kgen-kde-en-stepped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="Density (Stepped)",
    values=values,
    categories=groups,
    variant="stepped",
    fill_opacity=80,
    x_label="value", y_label="density",
)
chart.show()</code></pre></div><div id="kgen-kde-en-stepped-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "stepped",
});
chart.show();</code></pre></div><div id="kgen-kde-en-stepped-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "stepped",
});
chart.show();</code></pre></div><div id="kgen-kde-en-stepped-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "stepped"
)
chart$show()</code></pre></div><div id="kgen-kde-en-stepped-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("stepped")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-en-stepped-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("stepped")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-en-stepped-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "stepped"
);
chart.Show();</code></pre></div><div id="kgen-kde-en-stepped-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "stepped"
)
chart.show()</code></pre></div><div id="kgen-kde-en-stepped-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "stepped",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-stepped.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-rug">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rug"</code></span><span><strong>Aliases</strong> <code>rug / carpet / ticks / rugplot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">KDE curve with rug ticks at sample positions.</p>
<div class="sp-tabs" id="kgen-kde-en-rug">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-en-rug','kgen-kde-en-rug-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-rug','kgen-kde-en-rug-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-rug','kgen-kde-en-rug-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-rug','kgen-kde-en-rug-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-rug','kgen-kde-en-rug-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-rug','kgen-kde-en-rug-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-rug','kgen-kde-en-rug-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-rug','kgen-kde-en-rug-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-rug','kgen-kde-en-rug-cpp',this)">C++</button>
</div><div id="kgen-kde-en-rug-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="Density (Rug)",
    values=values,
    categories=groups,
    variant="rug",
    x_label="value", y_label="density",
    gridlines=True,
)
chart.show()</code></pre></div><div id="kgen-kde-en-rug-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "rug",
});
chart.show();</code></pre></div><div id="kgen-kde-en-rug-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "rug",
});
chart.show();</code></pre></div><div id="kgen-kde-en-rug-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "rug"
)
chart$show()</code></pre></div><div id="kgen-kde-en-rug-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("rug")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-en-rug-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("rug")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-en-rug-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "rug"
);
chart.Show();</code></pre></div><div id="kgen-kde-en-rug-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "rug"
)
chart.show()</code></pre></div><div id="kgen-kde-en-rug-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "rug",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-rug.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-histogram">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"histogram"</code></span><span><strong>Aliases</strong> <code>histogram / hist / with_hist / kdehist / distplot</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">KDE curve overlaid on a normalized histogram.</p>
<div class="sp-tabs" id="kgen-kde-en-histogram">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-en-histogram','kgen-kde-en-histogram-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-histogram','kgen-kde-en-histogram-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-histogram','kgen-kde-en-histogram-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-histogram','kgen-kde-en-histogram-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-histogram','kgen-kde-en-histogram-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-histogram','kgen-kde-en-histogram-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-histogram','kgen-kde-en-histogram-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-histogram','kgen-kde-en-histogram-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-histogram','kgen-kde-en-histogram-cpp',this)">C++</button>
</div><div id="kgen-kde-en-histogram-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="Distribution + KDE",
    values=values,
    variant="histogram",
    bins=30,
    palette=[0x6366F1],
    x_label="value", y_label="density",
)
chart.show()</code></pre></div><div id="kgen-kde-en-histogram-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "histogram",
});
chart.show();</code></pre></div><div id="kgen-kde-en-histogram-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "histogram",
});
chart.show();</code></pre></div><div id="kgen-kde-en-histogram-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "histogram"
)
chart$show()</code></pre></div><div id="kgen-kde-en-histogram-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("histogram")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-en-histogram-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("histogram")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-en-histogram-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "histogram"
);
chart.Show();</code></pre></div><div id="kgen-kde-en-histogram-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "histogram"
)
chart.show()</code></pre></div><div id="kgen-kde-en-histogram-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "histogram",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-histogram.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-normalized">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"normalized"</code></span><span><strong>Aliases</strong> <code>normalized / pdf / norm / density</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Each series normalized so its area integrates to 1.</p>
<div class="sp-tabs" id="kgen-kde-en-normalized">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-en-normalized','kgen-kde-en-normalized-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-normalized','kgen-kde-en-normalized-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-normalized','kgen-kde-en-normalized-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-normalized','kgen-kde-en-normalized-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-normalized','kgen-kde-en-normalized-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-normalized','kgen-kde-en-normalized-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-normalized','kgen-kde-en-normalized-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-normalized','kgen-kde-en-normalized-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-normalized','kgen-kde-en-normalized-cpp',this)">C++</button>
</div><div id="kgen-kde-en-normalized-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="PDF",
    values=values,
    categories=groups,
    variant="normalized",
    fill_opacity=70,
    x_label="value", y_label="pdf",
)
chart.show()</code></pre></div><div id="kgen-kde-en-normalized-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "normalized",
});
chart.show();</code></pre></div><div id="kgen-kde-en-normalized-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "normalized",
});
chart.show();</code></pre></div><div id="kgen-kde-en-normalized-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "normalized"
)
chart$show()</code></pre></div><div id="kgen-kde-en-normalized-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("normalized")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-en-normalized-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("normalized")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-en-normalized-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "normalized"
);
chart.Show();</code></pre></div><div id="kgen-kde-en-normalized-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "normalized"
)
chart.show()</code></pre></div><div id="kgen-kde-en-normalized-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "normalized",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-normalized.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-cumulative">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cumulative"</code></span><span><strong>Aliases</strong> <code>cumulative / cdf / cum</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Cumulative density (CDF) curve in [0, 1].</p>
<div class="sp-tabs" id="kgen-kde-en-cumulative">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-en-cumulative','kgen-kde-en-cumulative-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-cumulative','kgen-kde-en-cumulative-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-cumulative','kgen-kde-en-cumulative-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-cumulative','kgen-kde-en-cumulative-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-cumulative','kgen-kde-en-cumulative-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-cumulative','kgen-kde-en-cumulative-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-cumulative','kgen-kde-en-cumulative-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-cumulative','kgen-kde-en-cumulative-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-cumulative','kgen-kde-en-cumulative-cpp',this)">C++</button>
</div><div id="kgen-kde-en-cumulative-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="CDF",
    values=values,
    categories=groups,
    variant="cumulative",
    x_label="value", y_label="cdf",
)
chart.show()</code></pre></div><div id="kgen-kde-en-cumulative-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "cumulative",
});
chart.show();</code></pre></div><div id="kgen-kde-en-cumulative-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "cumulative",
});
chart.show();</code></pre></div><div id="kgen-kde-en-cumulative-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "cumulative"
)
chart$show()</code></pre></div><div id="kgen-kde-en-cumulative-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("cumulative")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-en-cumulative-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("cumulative")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-en-cumulative-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "cumulative"
);
chart.Show();</code></pre></div><div id="kgen-kde-en-cumulative-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "cumulative"
)
chart.show()</code></pre></div><div id="kgen-kde-en-cumulative-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "cumulative",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-cumulative.html"></iframe>
</div>
<div class="sp-variant" id="kde-en-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / shade / fade / ridge</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Vertical gradient fill (opaque top → transparent bottom).</p>
<div class="sp-tabs" id="kgen-kde-en-gradient">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-en-gradient','kgen-kde-en-gradient-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-gradient','kgen-kde-en-gradient-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-gradient','kgen-kde-en-gradient-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-gradient','kgen-kde-en-gradient-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-gradient','kgen-kde-en-gradient-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-gradient','kgen-kde-en-gradient-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-gradient','kgen-kde-en-gradient-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-gradient','kgen-kde-en-gradient-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-en-gradient','kgen-kde-en-gradient-cpp',this)">C++</button>
</div><div id="kgen-kde-en-gradient-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="Density (Gradient)",
    values=values,
    variant="gradient",
    palette=[0x8B5CF6],
    x_label="value", y_label="density",
)
chart.show()</code></pre></div><div id="kgen-kde-en-gradient-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "gradient",
});
chart.show();</code></pre></div><div id="kgen-kde-en-gradient-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "gradient",
});
chart.show();</code></pre></div><div id="kgen-kde-en-gradient-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "gradient"
)
chart$show()</code></pre></div><div id="kgen-kde-en-gradient-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("gradient")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-en-gradient-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("gradient")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-en-gradient-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "gradient"
);
chart.Show();</code></pre></div><div id="kgen-kde-en-gradient-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "gradient"
)
chart.show()</code></pre></div><div id="kgen-kde-en-gradient-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "gradient",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/kde-gradient.html"></iframe>
</div>
</div></div>

</div>

<div class="lang-fr">

## Signature

`sp.kde(title, values, *, variant="basic", categories=None, bandwidth=0.0, filled=True, fill_opacity=50, bins=30, n_points=80, palette=None, **kwargs) -> Chart`

Alias : `sp.kde`, `sp.build_kde_chart`

## Description

`sp.kde()` est le point d'entrée unifié pour toute la famille KDE (Kernel Density Estimate). Le mot-clé `variant` sélectionne la stratégie de rendu — tous les autres arguments conservent le même nom d'une variante à l'autre. La KDE produit une estimation de densité continue lissée à partir d'un échantillon de points avec un noyau gaussien et la règle de Scott pour le choix automatique de la bande passante. SeraPlot rend les courbes en SVG Rust natif, avec multi-séries, normalisation, CDF, rug, histogramme superposé et remplissage en dégradé.

| Variante | Usage | Args clés |
|----------|-------|-----------|
| `"basic"` | Une ou plusieurs courbes pleines | `categories`, `palette` |
| `"outline"` | Trait seul pour superpositions | `palette` |
| `"stepped"` | Densité en escalier | `fill_opacity` |
| `"rug"` | KDE + ticks rug aux positions | `categories` |
| `"histogram"` | KDE sur histogramme normalisé | `bins` |
| `"normalized"` | Chaque série remise à aire = 1 (vraie PDF) | `categories` |
| `"cumulative"` | Densité cumulée (CDF) dans [0, 1] | `categories` |
| `"gradient"` | Remplissage en dégradé vertical | `palette` |

---

## Paramètres

| Paramètre | Type | Défaut | Variantes | Description |
|-----------|------|--------|-----------|-------------|
| `title` | `str` | requis | toutes | Titre du graphique |
| `values` | `list[float]` | requis | toutes | Échantillons numériques bruts |
| `variant` | `str` | `"basic"` | — | Variante de rendu |
| `categories` | `list[str]` | `None` | toutes | Étiquette de groupe par valeur |
| `bandwidth` | `float` | `0.0` | toutes | Bande passante KDE. 0 = règle de Scott |
| `filled` | `bool` | `True` | basic, stepped, normalized | Remplir l'aire sous la courbe |
| `fill_opacity` | `int` | `50` | toutes (remplies) | Alpha de remplissage 0..255 |
| `bins` | `int` | `30` | histogram | Nombre de classes de l'histogramme |
| `n_points` | `int` | `80` | toutes | Points d'évaluation de la KDE |
| `palette` | `list[int]` | `None` | toutes | Palette personnalisée |
| `width` | `int` | `900` | toutes | Largeur (px) |
| `height` | `int` | `420` | toutes | Hauteur (px) |
| `x_label` | `str` | `""` | toutes | Libellé axe X |
| `y_label` | `str` | `"Density"` | toutes | Libellé axe Y |
| `gridlines` | `bool` | `False` | toutes | Afficher la grille |
| `sort_order` | `str` | `"none"` | toutes | `none` / `asc` / `desc` (tri par moyenne) |
| `background` | `str` | `None` | toutes | Couleur de fond CSS |

---

## Retour

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

<div class="sp-cls sp-open" id="kde-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('kde-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('kde-fr','basic',this)"><span class="sp-cic">I</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','outline',this)"><span class="sp-cic">O</span><span class="sp-clb">Outline</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','stepped',this)"><span class="sp-cic">T</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','rug',this)"><span class="sp-cic">R</span><span class="sp-clb">Rug</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','histogram',this)"><span class="sp-cic">H</span><span class="sp-clb">Histogram</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','normalized',this)"><span class="sp-cic">N</span><span class="sp-clb">Normalized</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','cumulative',this)"><span class="sp-cic">C</span><span class="sp-clb">Cumulative</span></button>
<button class="sp-cls-tab" onclick="spCls('kde-fr','gradient',this)"><span class="sp-cic">G</span><span class="sp-clb">Gradient</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="kde-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / filled / default / single / multi</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Courbe pleine, mono ou multi-séries.</p>
<div class="sp-tabs" id="kgen-kde-fr-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-fr-basic','kgen-kde-fr-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-basic','kgen-kde-fr-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-basic','kgen-kde-fr-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-basic','kgen-kde-fr-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-basic','kgen-kde-fr-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-basic','kgen-kde-fr-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-basic','kgen-kde-fr-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-basic','kgen-kde-fr-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-basic','kgen-kde-fr-basic-cpp',this)">C++</button>
</div><div id="kgen-kde-fr-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(0)
a = [random.gauss(-1.5, 0.7) for _ in range(500)]
b = [random.gauss(0.5, 1.0) for _ in range(500)]
c = [random.gauss(2.5, 0.6) for _ in range(500)]
chart = sp.kde(
    title="Density",
    values=a + b + c,
    categories=["A"]*500 + ["B"]*500 + ["C"]*500,
    variant="basic",
    palette=[0x6366F1, 0xF59E0B, 0xEF4444],
    x_label="value", y_label="density",
    gridlines=True,
)
chart.show()</code></pre></div><div id="kgen-kde-fr-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="kgen-kde-fr-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-fr-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-fr-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="kgen-kde-fr-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="kgen-kde-fr-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-basic.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-outline">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"outline"</code></span><span><strong>Alias</strong> <code>outline / line / stroke / compare / no_fill</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Courbes en trait seul pour des superpositions épurées.</p>
<div class="sp-tabs" id="kgen-kde-fr-outline">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-fr-outline','kgen-kde-fr-outline-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-outline','kgen-kde-fr-outline-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-outline','kgen-kde-fr-outline-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-outline','kgen-kde-fr-outline-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-outline','kgen-kde-fr-outline-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-outline','kgen-kde-fr-outline-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-outline','kgen-kde-fr-outline-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-outline','kgen-kde-fr-outline-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-outline','kgen-kde-fr-outline-cpp',this)">C++</button>
</div><div id="kgen-kde-fr-outline-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="Density (Outline)",
    values=values,
    categories=groups,
    variant="outline",
    palette=[0x6366F1, 0x10B981, 0xF59E0B],
    x_label="value", y_label="density",
)
chart.show()</code></pre></div><div id="kgen-kde-fr-outline-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "outline",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-outline-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "outline",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-outline-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "outline"
)
chart$show()</code></pre></div><div id="kgen-kde-fr-outline-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("outline")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-fr-outline-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("outline")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-fr-outline-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "outline"
);
chart.Show();</code></pre></div><div id="kgen-kde-fr-outline-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "outline"
)
chart.show()</code></pre></div><div id="kgen-kde-fr-outline-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "outline",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-outline.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-stepped">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"stepped"</code></span><span><strong>Alias</strong> <code>stepped / step / stair / stairs</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Densité en escalier (rendu rectangulaire).</p>
<div class="sp-tabs" id="kgen-kde-fr-stepped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-fr-stepped','kgen-kde-fr-stepped-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-stepped','kgen-kde-fr-stepped-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-stepped','kgen-kde-fr-stepped-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-stepped','kgen-kde-fr-stepped-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-stepped','kgen-kde-fr-stepped-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-stepped','kgen-kde-fr-stepped-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-stepped','kgen-kde-fr-stepped-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-stepped','kgen-kde-fr-stepped-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-stepped','kgen-kde-fr-stepped-cpp',this)">C++</button>
</div><div id="kgen-kde-fr-stepped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="Density (Stepped)",
    values=values,
    categories=groups,
    variant="stepped",
    fill_opacity=80,
    x_label="value", y_label="density",
)
chart.show()</code></pre></div><div id="kgen-kde-fr-stepped-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "stepped",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-stepped-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "stepped",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-stepped-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "stepped"
)
chart$show()</code></pre></div><div id="kgen-kde-fr-stepped-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("stepped")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-fr-stepped-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("stepped")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-fr-stepped-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "stepped"
);
chart.Show();</code></pre></div><div id="kgen-kde-fr-stepped-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "stepped"
)
chart.show()</code></pre></div><div id="kgen-kde-fr-stepped-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "stepped",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-stepped.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-rug">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"rug"</code></span><span><strong>Alias</strong> <code>rug / carpet / ticks / rugplot</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Courbe KDE avec ticks rug aux positions des points.</p>
<div class="sp-tabs" id="kgen-kde-fr-rug">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-fr-rug','kgen-kde-fr-rug-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-rug','kgen-kde-fr-rug-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-rug','kgen-kde-fr-rug-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-rug','kgen-kde-fr-rug-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-rug','kgen-kde-fr-rug-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-rug','kgen-kde-fr-rug-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-rug','kgen-kde-fr-rug-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-rug','kgen-kde-fr-rug-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-rug','kgen-kde-fr-rug-cpp',this)">C++</button>
</div><div id="kgen-kde-fr-rug-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="Density (Rug)",
    values=values,
    categories=groups,
    variant="rug",
    x_label="value", y_label="density",
    gridlines=True,
)
chart.show()</code></pre></div><div id="kgen-kde-fr-rug-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "rug",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-rug-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "rug",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-rug-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "rug"
)
chart$show()</code></pre></div><div id="kgen-kde-fr-rug-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("rug")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-fr-rug-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("rug")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-fr-rug-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "rug"
);
chart.Show();</code></pre></div><div id="kgen-kde-fr-rug-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "rug"
)
chart.show()</code></pre></div><div id="kgen-kde-fr-rug-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "rug",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-rug.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-histogram">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"histogram"</code></span><span><strong>Alias</strong> <code>histogram / hist / with_hist / kdehist / distplot</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Courbe KDE par-dessus un histogramme normalisé.</p>
<div class="sp-tabs" id="kgen-kde-fr-histogram">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-fr-histogram','kgen-kde-fr-histogram-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-histogram','kgen-kde-fr-histogram-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-histogram','kgen-kde-fr-histogram-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-histogram','kgen-kde-fr-histogram-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-histogram','kgen-kde-fr-histogram-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-histogram','kgen-kde-fr-histogram-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-histogram','kgen-kde-fr-histogram-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-histogram','kgen-kde-fr-histogram-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-histogram','kgen-kde-fr-histogram-cpp',this)">C++</button>
</div><div id="kgen-kde-fr-histogram-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="Distribution + KDE",
    values=values,
    variant="histogram",
    bins=30,
    palette=[0x6366F1],
    x_label="value", y_label="density",
)
chart.show()</code></pre></div><div id="kgen-kde-fr-histogram-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "histogram",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-histogram-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "histogram",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-histogram-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "histogram"
)
chart$show()</code></pre></div><div id="kgen-kde-fr-histogram-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("histogram")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-fr-histogram-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("histogram")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-fr-histogram-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "histogram"
);
chart.Show();</code></pre></div><div id="kgen-kde-fr-histogram-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "histogram"
)
chart.show()</code></pre></div><div id="kgen-kde-fr-histogram-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "histogram",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-histogram.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-normalized">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"normalized"</code></span><span><strong>Alias</strong> <code>normalized / pdf / norm / density</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Chaque série normalisée pour que son aire vaille 1.</p>
<div class="sp-tabs" id="kgen-kde-fr-normalized">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-fr-normalized','kgen-kde-fr-normalized-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-normalized','kgen-kde-fr-normalized-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-normalized','kgen-kde-fr-normalized-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-normalized','kgen-kde-fr-normalized-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-normalized','kgen-kde-fr-normalized-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-normalized','kgen-kde-fr-normalized-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-normalized','kgen-kde-fr-normalized-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-normalized','kgen-kde-fr-normalized-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-normalized','kgen-kde-fr-normalized-cpp',this)">C++</button>
</div><div id="kgen-kde-fr-normalized-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="PDF",
    values=values,
    categories=groups,
    variant="normalized",
    fill_opacity=70,
    x_label="value", y_label="pdf",
)
chart.show()</code></pre></div><div id="kgen-kde-fr-normalized-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "normalized",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-normalized-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "normalized",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-normalized-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "normalized"
)
chart$show()</code></pre></div><div id="kgen-kde-fr-normalized-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("normalized")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-fr-normalized-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("normalized")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-fr-normalized-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "normalized"
);
chart.Show();</code></pre></div><div id="kgen-kde-fr-normalized-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "normalized"
)
chart.show()</code></pre></div><div id="kgen-kde-fr-normalized-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "normalized",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-normalized.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-cumulative">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"cumulative"</code></span><span><strong>Alias</strong> <code>cumulative / cdf / cum</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Densité cumulée (CDF) dans [0, 1].</p>
<div class="sp-tabs" id="kgen-kde-fr-cumulative">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-fr-cumulative','kgen-kde-fr-cumulative-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-cumulative','kgen-kde-fr-cumulative-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-cumulative','kgen-kde-fr-cumulative-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-cumulative','kgen-kde-fr-cumulative-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-cumulative','kgen-kde-fr-cumulative-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-cumulative','kgen-kde-fr-cumulative-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-cumulative','kgen-kde-fr-cumulative-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-cumulative','kgen-kde-fr-cumulative-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-cumulative','kgen-kde-fr-cumulative-cpp',this)">C++</button>
</div><div id="kgen-kde-fr-cumulative-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="CDF",
    values=values,
    categories=groups,
    variant="cumulative",
    x_label="value", y_label="cdf",
)
chart.show()</code></pre></div><div id="kgen-kde-fr-cumulative-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "cumulative",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-cumulative-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "cumulative",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-cumulative-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "cumulative"
)
chart$show()</code></pre></div><div id="kgen-kde-fr-cumulative-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("cumulative")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-fr-cumulative-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("cumulative")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-fr-cumulative-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "cumulative"
);
chart.Show();</code></pre></div><div id="kgen-kde-fr-cumulative-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "cumulative"
)
chart.show()</code></pre></div><div id="kgen-kde-fr-cumulative-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "cumulative",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-cumulative.html"></iframe>
</div>
<div class="sp-variant" id="kde-fr-gradient">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code></span><span><strong>Alias</strong> <code>gradient / shade / fade / ridge</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Remplissage en dégradé vertical (opaque haut → transparent bas).</p>
<div class="sp-tabs" id="kgen-kde-fr-gradient">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kgen-kde-fr-gradient','kgen-kde-fr-gradient-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-gradient','kgen-kde-fr-gradient-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-gradient','kgen-kde-fr-gradient-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-gradient','kgen-kde-fr-gradient-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-gradient','kgen-kde-fr-gradient-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-gradient','kgen-kde-fr-gradient-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-gradient','kgen-kde-fr-gradient-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-gradient','kgen-kde-fr-gradient-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kgen-kde-fr-gradient','kgen-kde-fr-gradient-cpp',this)">C++</button>
</div><div id="kgen-kde-fr-gradient-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.kde(
    title="Density (Gradient)",
    values=values,
    variant="gradient",
    palette=[0x8B5CF6],
    x_label="value", y_label="density",
)
chart.show()</code></pre></div><div id="kgen-kde-fr-gradient-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "gradient",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-gradient-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.kde({
  title: "Density",
  values: [/* numbers */],
  variant: "gradient",
});
chart.show();</code></pre></div><div id="kgen-kde-fr-gradient-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$kde(
  title = "Density",
  values = c(),
  variant = "gradient"
)
chart$show()</code></pre></div><div id="kgen-kde-fr-gradient-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::kde()
        .title("Density")
        .values(vec![])
        .variant("gradient")
        .build();
    chart.show();
}</code></pre></div><div id="kgen-kde-fr-gradient-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.kde()
    .title("Density")
    .values(List.of())
    .variant("gradient")
    .build();
chart.show();</code></pre></div><div id="kgen-kde-fr-gradient-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title: "Density",
    values: new double[]{},
    variant: "gradient"
);
chart.Show();</code></pre></div><div id="kgen-kde-fr-gradient-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title   = "Density",
  values  = List(),
  variant = "gradient"
)
chart.show()</code></pre></div><div id="kgen-kde-fr-gradient-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include <seraplot/seraplot.hpp>

auto chart = sp::kde({
    .title   = "Density",
    .values  = {},
    .variant = "gradient",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/kde-gradient.html"></iframe>
</div>
</div></div>

</div>

