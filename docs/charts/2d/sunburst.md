# Sunburst — Hierarchical Ring Chart

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

`sp.sunburst(title, labels, parents, values, *, variant="basic", palette=None, **kwargs) -> Chart`

Aliases: `sp.sunburst`, `sp.build_sunburst`

## Description

`sp.sunburst()` is the unified entry point for the entire sunburst-chart family. A sunburst represents a hierarchy as concentric rings: the innermost ring is the root, each outer ring is a deeper level, and angular size encodes value. The `variant` keyword selects the visual style without changing any other parameter. Sunbursts are the standard for visualizing nested taxonomies (org charts, file systems, market segmentation, expense categories, phylogenetic trees) and outperform classic pie charts as soon as a real hierarchy exists.

> **Hierarchy encoding** — `labels` lists every node, `parents` gives the parent label of each node ("" for a root). Leaf values are taken from `values`; internal node values are auto-rolled-up from descendants when set to 0.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / ring` | Classic concentric rings with depth-based opacity and white separators. |
| `"donut"` | `donut / hole / ring_hole / donut_ring` | Larger central hole with the formatted grand total displayed at the center. |
| `"flame"` | `flame / warm / heat / fire` | Warm gradient (red -> amber -> cream) per ring, evokes a heatmap or fire. |
| `"rainbow"` | `rainbow / spectrum / hue / prism` | HSL palette: hue derived from mid-angle, saturation/lightness softened with depth. |
| `"outlined"` | `outlined / outline / stroke / wireframe` | Wireframe wedges: white fill + colored stroke that thins on deeper rings. |
| `"gapped"` | `gapped / spaced / isolated / petals` | Angular and radial margins between every wedge for crisp petal-like separation. |
| `"depth_fade"` | `depth_fade / fade / fading / depth` | Standard palette but opacity decreases with depth, focusing the eye on top levels. |
| `"mono"` | `mono / monochrome / single / uniform` | Single-color rings differentiated only by depth-based opacity. Editorial look. |

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title`    | `str`         | required  | Chart title |
| `labels`   | `list[str]`   | required  | Node labels (one per row) |
| `parents`  | `list[str]`   | required  | Parent label of each node ("" for roots) |
| `values`   | `list[float]` | required  | Leaf values; internal zeros are auto-rolled-up |
| `variant`  | `str`         | `"basic"` | Visual style (see table) |
| `palette`  | `list[int]`   | `None`    | Per-root color palette (rotates if shorter) |
| `width`    | `int`         | `700`     | Canvas width (px) |
| `height`   | `int`         | `700`     | Canvas height (px) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="sunburst-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('sunburst-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('sunburst-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-en','donut',this)"><span class="sp-cic">D</span><span class="sp-clb">Donut</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-en','flame',this)"><span class="sp-cic">F</span><span class="sp-clb">Flame</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-en','rainbow',this)"><span class="sp-cic">R</span><span class="sp-clb">Rainbow</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-en','outlined',this)"><span class="sp-cic">O</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-en','gapped',this)"><span class="sp-cic">G</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-en','depth_fade',this)"><span class="sp-cic">X</span><span class="sp-clb">Depth fade</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-en','mono',this)"><span class="sp-cic">M</span><span class="sp-clb">Mono</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="sunburst-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / ring</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic concentric rings with depth-based opacity and white separators.</p>
<div class="sp-tabs" id="sunburst-en-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-en-basic','sunburst-en-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-en-basic','sunburst-en-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-basic','sunburst-en-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-basic','sunburst-en-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-en-basic','sunburst-en-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-en-basic','sunburst-en-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-en-basic','sunburst-en-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-en-basic','sunburst-en-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-en-basic','sunburst-en-basic-cpp',this)">C++</button>
</div><div id="sunburst-en-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="basic",
)
chart.show()</code></pre></div><div id="sunburst-en-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="sunburst-en-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="sunburst-en-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="sunburst-en-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-en-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="sunburst-en-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="sunburst-en-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="sunburst-en-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-basic.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-donut">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"donut"</code></span><span><strong>Aliases</strong> <code>donut / hole / ring_hole / donut_ring</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Larger central hole with the formatted grand total displayed at the center.</p>
<div class="sp-tabs" id="sunburst-en-donut">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-en-donut','sunburst-en-donut-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-en-donut','sunburst-en-donut-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-donut','sunburst-en-donut-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-donut','sunburst-en-donut-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-en-donut','sunburst-en-donut-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-en-donut','sunburst-en-donut-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-en-donut','sunburst-en-donut-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-en-donut','sunburst-en-donut-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-en-donut','sunburst-en-donut-cpp',this)">C++</button>
</div><div id="sunburst-en-donut-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="donut",
)
chart.show()</code></pre></div><div id="sunburst-en-donut-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "donut",
});
chart.show();</code></pre></div><div id="sunburst-en-donut-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "donut",
});
chart.show();</code></pre></div><div id="sunburst-en-donut-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "donut"
)
chart$show()</code></pre></div><div id="sunburst-en-donut-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("donut")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-en-donut-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("donut")
    .build();
chart.show();</code></pre></div><div id="sunburst-en-donut-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "donut"
);
chart.Show();</code></pre></div><div id="sunburst-en-donut-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "donut"
)
chart.show()</code></pre></div><div id="sunburst-en-donut-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "donut",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-donut.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-flame">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"flame"</code></span><span><strong>Aliases</strong> <code>flame / warm / heat / fire</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Warm gradient (red -> amber -> cream) per ring, evokes a heatmap or fire.</p>
<div class="sp-tabs" id="sunburst-en-flame">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-en-flame','sunburst-en-flame-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-en-flame','sunburst-en-flame-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-flame','sunburst-en-flame-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-flame','sunburst-en-flame-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-en-flame','sunburst-en-flame-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-en-flame','sunburst-en-flame-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-en-flame','sunburst-en-flame-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-en-flame','sunburst-en-flame-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-en-flame','sunburst-en-flame-cpp',this)">C++</button>
</div><div id="sunburst-en-flame-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="flame",
)
chart.show()</code></pre></div><div id="sunburst-en-flame-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "flame",
});
chart.show();</code></pre></div><div id="sunburst-en-flame-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "flame",
});
chart.show();</code></pre></div><div id="sunburst-en-flame-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "flame"
)
chart$show()</code></pre></div><div id="sunburst-en-flame-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("flame")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-en-flame-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("flame")
    .build();
chart.show();</code></pre></div><div id="sunburst-en-flame-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "flame"
);
chart.Show();</code></pre></div><div id="sunburst-en-flame-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "flame"
)
chart.show()</code></pre></div><div id="sunburst-en-flame-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "flame",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-flame.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-rainbow">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rainbow"</code></span><span><strong>Aliases</strong> <code>rainbow / spectrum / hue / prism</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">HSL palette: hue derived from mid-angle, saturation/lightness softened with depth.</p>
<div class="sp-tabs" id="sunburst-en-rainbow">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-en-rainbow','sunburst-en-rainbow-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-en-rainbow','sunburst-en-rainbow-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-rainbow','sunburst-en-rainbow-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-rainbow','sunburst-en-rainbow-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-en-rainbow','sunburst-en-rainbow-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-en-rainbow','sunburst-en-rainbow-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-en-rainbow','sunburst-en-rainbow-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-en-rainbow','sunburst-en-rainbow-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-en-rainbow','sunburst-en-rainbow-cpp',this)">C++</button>
</div><div id="sunburst-en-rainbow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="rainbow",
)
chart.show()</code></pre></div><div id="sunburst-en-rainbow-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "rainbow",
});
chart.show();</code></pre></div><div id="sunburst-en-rainbow-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "rainbow",
});
chart.show();</code></pre></div><div id="sunburst-en-rainbow-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "rainbow"
)
chart$show()</code></pre></div><div id="sunburst-en-rainbow-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("rainbow")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-en-rainbow-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("rainbow")
    .build();
chart.show();</code></pre></div><div id="sunburst-en-rainbow-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "rainbow"
);
chart.Show();</code></pre></div><div id="sunburst-en-rainbow-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "rainbow"
)
chart.show()</code></pre></div><div id="sunburst-en-rainbow-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "rainbow",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-rainbow.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-outlined">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / wireframe</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Wireframe wedges: white fill + colored stroke that thins on deeper rings.</p>
<div class="sp-tabs" id="sunburst-en-outlined">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-en-outlined','sunburst-en-outlined-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-en-outlined','sunburst-en-outlined-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-outlined','sunburst-en-outlined-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-outlined','sunburst-en-outlined-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-en-outlined','sunburst-en-outlined-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-en-outlined','sunburst-en-outlined-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-en-outlined','sunburst-en-outlined-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-en-outlined','sunburst-en-outlined-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-en-outlined','sunburst-en-outlined-cpp',this)">C++</button>
</div><div id="sunburst-en-outlined-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="outlined",
)
chart.show()</code></pre></div><div id="sunburst-en-outlined-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "outlined",
});
chart.show();</code></pre></div><div id="sunburst-en-outlined-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "outlined",
});
chart.show();</code></pre></div><div id="sunburst-en-outlined-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "outlined"
)
chart$show()</code></pre></div><div id="sunburst-en-outlined-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("outlined")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-en-outlined-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("outlined")
    .build();
chart.show();</code></pre></div><div id="sunburst-en-outlined-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "outlined"
);
chart.Show();</code></pre></div><div id="sunburst-en-outlined-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "outlined"
)
chart.show()</code></pre></div><div id="sunburst-en-outlined-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "outlined",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-outlined.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-gapped">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>gapped / spaced / isolated / petals</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Angular and radial margins between every wedge for crisp petal-like separation.</p>
<div class="sp-tabs" id="sunburst-en-gapped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-en-gapped','sunburst-en-gapped-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-en-gapped','sunburst-en-gapped-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-gapped','sunburst-en-gapped-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-gapped','sunburst-en-gapped-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-en-gapped','sunburst-en-gapped-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-en-gapped','sunburst-en-gapped-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-en-gapped','sunburst-en-gapped-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-en-gapped','sunburst-en-gapped-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-en-gapped','sunburst-en-gapped-cpp',this)">C++</button>
</div><div id="sunburst-en-gapped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="gapped",
)
chart.show()</code></pre></div><div id="sunburst-en-gapped-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "gapped",
});
chart.show();</code></pre></div><div id="sunburst-en-gapped-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "gapped",
});
chart.show();</code></pre></div><div id="sunburst-en-gapped-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "gapped"
)
chart$show()</code></pre></div><div id="sunburst-en-gapped-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("gapped")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-en-gapped-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("gapped")
    .build();
chart.show();</code></pre></div><div id="sunburst-en-gapped-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "gapped"
);
chart.Show();</code></pre></div><div id="sunburst-en-gapped-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "gapped"
)
chart.show()</code></pre></div><div id="sunburst-en-gapped-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "gapped",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-gapped.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-depth_fade">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"depth_fade"</code></span><span><strong>Aliases</strong> <code>depth_fade / fade / fading / depth</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Standard palette but opacity decreases with depth, focusing the eye on top levels.</p>
<div class="sp-tabs" id="sunburst-en-depth_fade">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-en-depth_fade','sunburst-en-depth_fade-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-en-depth_fade','sunburst-en-depth_fade-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-depth_fade','sunburst-en-depth_fade-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-depth_fade','sunburst-en-depth_fade-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-en-depth_fade','sunburst-en-depth_fade-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-en-depth_fade','sunburst-en-depth_fade-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-en-depth_fade','sunburst-en-depth_fade-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-en-depth_fade','sunburst-en-depth_fade-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-en-depth_fade','sunburst-en-depth_fade-cpp',this)">C++</button>
</div><div id="sunburst-en-depth_fade-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="depth_fade",
)
chart.show()</code></pre></div><div id="sunburst-en-depth_fade-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "depth_fade",
});
chart.show();</code></pre></div><div id="sunburst-en-depth_fade-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "depth_fade",
});
chart.show();</code></pre></div><div id="sunburst-en-depth_fade-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "depth_fade"
)
chart$show()</code></pre></div><div id="sunburst-en-depth_fade-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("depth_fade")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-en-depth_fade-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("depth_fade")
    .build();
chart.show();</code></pre></div><div id="sunburst-en-depth_fade-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "depth_fade"
);
chart.Show();</code></pre></div><div id="sunburst-en-depth_fade-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "depth_fade"
)
chart.show()</code></pre></div><div id="sunburst-en-depth_fade-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "depth_fade",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-depth_fade.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-en-mono">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mono"</code></span><span><strong>Aliases</strong> <code>mono / monochrome / single / uniform</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Single-color rings differentiated only by depth-based opacity. Editorial look.</p>
<div class="sp-tabs" id="sunburst-en-mono">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-en-mono','sunburst-en-mono-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-en-mono','sunburst-en-mono-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-mono','sunburst-en-mono-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-en-mono','sunburst-en-mono-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-en-mono','sunburst-en-mono-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-en-mono','sunburst-en-mono-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-en-mono','sunburst-en-mono-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-en-mono','sunburst-en-mono-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-en-mono','sunburst-en-mono-cpp',this)">C++</button>
</div><div id="sunburst-en-mono-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="mono",
)
chart.show()</code></pre></div><div id="sunburst-en-mono-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "mono",
});
chart.show();</code></pre></div><div id="sunburst-en-mono-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "mono",
});
chart.show();</code></pre></div><div id="sunburst-en-mono-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "mono"
)
chart$show()</code></pre></div><div id="sunburst-en-mono-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("mono")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-en-mono-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("mono")
    .build();
chart.show();</code></pre></div><div id="sunburst-en-mono-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "mono"
);
chart.Show();</code></pre></div><div id="sunburst-en-mono-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "mono"
)
chart.show()</code></pre></div><div id="sunburst-en-mono-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "mono",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-mono.html"></iframe>
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

`sp.sunburst(title, labels, parents, values, *, variant="basic", palette=None, **kwargs) -> Chart`

Alias : `sp.sunburst`, `sp.build_sunburst`

## Description

`sp.sunburst()` est le point d entree unifie pour toute la famille des graphiques sunburst. Un sunburst represente une hierarchie sous forme d anneaux concentriques : l anneau interieur est la racine, chaque anneau exterieur est un niveau plus profond, et l angle code la valeur. Le mot-cle `variant` change le style sans toucher aux autres parametres. Les sunbursts sont la reference pour visualiser des taxonomies imbriquees (organigrammes, systemes de fichiers, segmentation marche, categories de depenses, arbres phylogenetiques) et surpassent le camembert des qu une vraie hierarchie existe.

> **Encodage de la hierarchie** — `labels` liste tous les noeuds, `parents` donne le libelle du parent de chaque noeud ("" pour une racine). Les valeurs des feuilles viennent de `values` ; les noeuds internes a 0 sont calcules automatiquement comme la somme de leurs descendants.

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / ring` | Anneaux concentriques classiques avec opacite degressive selon la profondeur. |
| `"donut"` | `donut / hole / ring_hole / donut_ring` | Large trou central avec total general formate (k/M) au centre. |
| `"flame"` | `flame / warm / heat / fire` | Degrade chaud (rouge -> ambre -> creme) par anneau, evoque chaleur ou feu. |
| `"rainbow"` | `rainbow / spectrum / hue / prism` | Palette HSL : teinte derivee de l angle median, saturation/lumiere adoucies par profondeur. |
| `"outlined"` | `outlined / outline / stroke / wireframe` | Quartiers en fil de fer : fond blanc + contour colore amincissant en profondeur. |
| `"gapped"` | `gapped / spaced / isolated / petals` | Marges angulaires et radiales entre quartiers pour une separation nette en petales. |
| `"depth_fade"` | `depth_fade / fade / fading / depth` | Palette standard mais opacite decroissante en profondeur pour concentrer le regard. |
| `"mono"` | `mono / monochrome / single / uniform` | Anneaux monochromes differencies uniquement par opacite. Rendu editorial. |

## Parametres

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title`    | `str`         | requis    | Titre du graphique |
| `labels`   | `list[str]`   | requis    | Libelles des noeuds (un par ligne) |
| `parents`  | `list[str]`   | requis    | Parent de chaque noeud ("" pour les racines) |
| `values`   | `list[float]` | requis    | Valeurs feuilles ; zeros internes calcules auto |
| `variant`  | `str`         | `"basic"` | Style visuel (voir tableau) |
| `palette`  | `list[int]`   | `None`    | Palette couleurs par racine (rotation si trop courte) |
| `width`    | `int`         | `700`     | Largeur du canvas (px) |
| `height`   | `int`         | `700`     | Hauteur du canvas (px) |

## Retour

`Chart` — objet avec une propriete `.html` et une methode `.show()`.

---

<div class="sp-cls sp-open" id="sunburst-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('sunburst-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('sunburst-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-fr','donut',this)"><span class="sp-cic">D</span><span class="sp-clb">Donut</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-fr','flame',this)"><span class="sp-cic">F</span><span class="sp-clb">Flame</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-fr','rainbow',this)"><span class="sp-cic">R</span><span class="sp-clb">Rainbow</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-fr','outlined',this)"><span class="sp-cic">O</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-fr','gapped',this)"><span class="sp-cic">G</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-fr','depth_fade',this)"><span class="sp-cic">X</span><span class="sp-clb">Depth fade</span></button>
<button class="sp-cls-tab" onclick="spCls('sunburst-fr','mono',this)"><span class="sp-cic">M</span><span class="sp-clb">Mono</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="sunburst-fr-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / ring</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Anneaux concentriques classiques avec opacite degressive selon la profondeur.</p>
<div class="sp-tabs" id="sunburst-fr-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-fr-basic','sunburst-fr-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-basic','sunburst-fr-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-basic','sunburst-fr-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-basic','sunburst-fr-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-basic','sunburst-fr-basic-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-basic','sunburst-fr-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-basic','sunburst-fr-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-basic','sunburst-fr-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-basic','sunburst-fr-basic-cpp',this)">C++</button>
</div><div id="sunburst-fr-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="basic",
)
chart.show()</code></pre></div><div id="sunburst-fr-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="sunburst-fr-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "basic",
});
chart.show();</code></pre></div><div id="sunburst-fr-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "basic"
)
chart$show()</code></pre></div><div id="sunburst-fr-basic-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("basic")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-fr-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("basic")
    .build();
chart.show();</code></pre></div><div id="sunburst-fr-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "basic"
);
chart.Show();</code></pre></div><div id="sunburst-fr-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "basic"
)
chart.show()</code></pre></div><div id="sunburst-fr-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "basic",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-basic.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-donut">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"donut"</code></span><span><strong>Aliases</strong> <code>donut / hole / ring_hole / donut_ring</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Large trou central avec total general formate (k/M) au centre.</p>
<div class="sp-tabs" id="sunburst-fr-donut">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-fr-donut','sunburst-fr-donut-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-donut','sunburst-fr-donut-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-donut','sunburst-fr-donut-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-donut','sunburst-fr-donut-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-donut','sunburst-fr-donut-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-donut','sunburst-fr-donut-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-donut','sunburst-fr-donut-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-donut','sunburst-fr-donut-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-donut','sunburst-fr-donut-cpp',this)">C++</button>
</div><div id="sunburst-fr-donut-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="donut",
)
chart.show()</code></pre></div><div id="sunburst-fr-donut-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "donut",
});
chart.show();</code></pre></div><div id="sunburst-fr-donut-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "donut",
});
chart.show();</code></pre></div><div id="sunburst-fr-donut-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "donut"
)
chart$show()</code></pre></div><div id="sunburst-fr-donut-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("donut")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-fr-donut-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("donut")
    .build();
chart.show();</code></pre></div><div id="sunburst-fr-donut-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "donut"
);
chart.Show();</code></pre></div><div id="sunburst-fr-donut-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "donut"
)
chart.show()</code></pre></div><div id="sunburst-fr-donut-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "donut",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-donut.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-flame">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"flame"</code></span><span><strong>Aliases</strong> <code>flame / warm / heat / fire</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Degrade chaud (rouge -> ambre -> creme) par anneau, evoque chaleur ou feu.</p>
<div class="sp-tabs" id="sunburst-fr-flame">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-fr-flame','sunburst-fr-flame-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-flame','sunburst-fr-flame-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-flame','sunburst-fr-flame-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-flame','sunburst-fr-flame-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-flame','sunburst-fr-flame-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-flame','sunburst-fr-flame-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-flame','sunburst-fr-flame-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-flame','sunburst-fr-flame-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-flame','sunburst-fr-flame-cpp',this)">C++</button>
</div><div id="sunburst-fr-flame-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="flame",
)
chart.show()</code></pre></div><div id="sunburst-fr-flame-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "flame",
});
chart.show();</code></pre></div><div id="sunburst-fr-flame-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "flame",
});
chart.show();</code></pre></div><div id="sunburst-fr-flame-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "flame"
)
chart$show()</code></pre></div><div id="sunburst-fr-flame-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("flame")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-fr-flame-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("flame")
    .build();
chart.show();</code></pre></div><div id="sunburst-fr-flame-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "flame"
);
chart.Show();</code></pre></div><div id="sunburst-fr-flame-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "flame"
)
chart.show()</code></pre></div><div id="sunburst-fr-flame-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "flame",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-flame.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-rainbow">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rainbow"</code></span><span><strong>Aliases</strong> <code>rainbow / spectrum / hue / prism</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Palette HSL : teinte derivee de l angle median, saturation/lumiere adoucies par profondeur.</p>
<div class="sp-tabs" id="sunburst-fr-rainbow">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-fr-rainbow','sunburst-fr-rainbow-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-rainbow','sunburst-fr-rainbow-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-rainbow','sunburst-fr-rainbow-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-rainbow','sunburst-fr-rainbow-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-rainbow','sunburst-fr-rainbow-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-rainbow','sunburst-fr-rainbow-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-rainbow','sunburst-fr-rainbow-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-rainbow','sunburst-fr-rainbow-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-rainbow','sunburst-fr-rainbow-cpp',this)">C++</button>
</div><div id="sunburst-fr-rainbow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="rainbow",
)
chart.show()</code></pre></div><div id="sunburst-fr-rainbow-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "rainbow",
});
chart.show();</code></pre></div><div id="sunburst-fr-rainbow-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "rainbow",
});
chart.show();</code></pre></div><div id="sunburst-fr-rainbow-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "rainbow"
)
chart$show()</code></pre></div><div id="sunburst-fr-rainbow-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("rainbow")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-fr-rainbow-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("rainbow")
    .build();
chart.show();</code></pre></div><div id="sunburst-fr-rainbow-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "rainbow"
);
chart.Show();</code></pre></div><div id="sunburst-fr-rainbow-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "rainbow"
)
chart.show()</code></pre></div><div id="sunburst-fr-rainbow-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "rainbow",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-rainbow.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-outlined">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / wireframe</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Quartiers en fil de fer : fond blanc + contour colore amincissant en profondeur.</p>
<div class="sp-tabs" id="sunburst-fr-outlined">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-fr-outlined','sunburst-fr-outlined-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-outlined','sunburst-fr-outlined-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-outlined','sunburst-fr-outlined-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-outlined','sunburst-fr-outlined-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-outlined','sunburst-fr-outlined-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-outlined','sunburst-fr-outlined-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-outlined','sunburst-fr-outlined-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-outlined','sunburst-fr-outlined-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-outlined','sunburst-fr-outlined-cpp',this)">C++</button>
</div><div id="sunburst-fr-outlined-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="outlined",
)
chart.show()</code></pre></div><div id="sunburst-fr-outlined-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "outlined",
});
chart.show();</code></pre></div><div id="sunburst-fr-outlined-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "outlined",
});
chart.show();</code></pre></div><div id="sunburst-fr-outlined-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "outlined"
)
chart$show()</code></pre></div><div id="sunburst-fr-outlined-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("outlined")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-fr-outlined-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("outlined")
    .build();
chart.show();</code></pre></div><div id="sunburst-fr-outlined-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "outlined"
);
chart.Show();</code></pre></div><div id="sunburst-fr-outlined-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "outlined"
)
chart.show()</code></pre></div><div id="sunburst-fr-outlined-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "outlined",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-outlined.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-gapped">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>gapped / spaced / isolated / petals</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Marges angulaires et radiales entre quartiers pour une separation nette en petales.</p>
<div class="sp-tabs" id="sunburst-fr-gapped">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-fr-gapped','sunburst-fr-gapped-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-gapped','sunburst-fr-gapped-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-gapped','sunburst-fr-gapped-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-gapped','sunburst-fr-gapped-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-gapped','sunburst-fr-gapped-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-gapped','sunburst-fr-gapped-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-gapped','sunburst-fr-gapped-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-gapped','sunburst-fr-gapped-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-gapped','sunburst-fr-gapped-cpp',this)">C++</button>
</div><div id="sunburst-fr-gapped-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="gapped",
)
chart.show()</code></pre></div><div id="sunburst-fr-gapped-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "gapped",
});
chart.show();</code></pre></div><div id="sunburst-fr-gapped-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "gapped",
});
chart.show();</code></pre></div><div id="sunburst-fr-gapped-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "gapped"
)
chart$show()</code></pre></div><div id="sunburst-fr-gapped-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("gapped")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-fr-gapped-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("gapped")
    .build();
chart.show();</code></pre></div><div id="sunburst-fr-gapped-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "gapped"
);
chart.Show();</code></pre></div><div id="sunburst-fr-gapped-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "gapped"
)
chart.show()</code></pre></div><div id="sunburst-fr-gapped-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "gapped",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-gapped.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-depth_fade">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"depth_fade"</code></span><span><strong>Aliases</strong> <code>depth_fade / fade / fading / depth</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Palette standard mais opacite decroissante en profondeur pour concentrer le regard.</p>
<div class="sp-tabs" id="sunburst-fr-depth_fade">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-fr-depth_fade','sunburst-fr-depth_fade-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-depth_fade','sunburst-fr-depth_fade-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-depth_fade','sunburst-fr-depth_fade-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-depth_fade','sunburst-fr-depth_fade-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-depth_fade','sunburst-fr-depth_fade-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-depth_fade','sunburst-fr-depth_fade-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-depth_fade','sunburst-fr-depth_fade-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-depth_fade','sunburst-fr-depth_fade-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-depth_fade','sunburst-fr-depth_fade-cpp',this)">C++</button>
</div><div id="sunburst-fr-depth_fade-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="depth_fade",
)
chart.show()</code></pre></div><div id="sunburst-fr-depth_fade-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "depth_fade",
});
chart.show();</code></pre></div><div id="sunburst-fr-depth_fade-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "depth_fade",
});
chart.show();</code></pre></div><div id="sunburst-fr-depth_fade-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "depth_fade"
)
chart$show()</code></pre></div><div id="sunburst-fr-depth_fade-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("depth_fade")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-fr-depth_fade-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("depth_fade")
    .build();
chart.show();</code></pre></div><div id="sunburst-fr-depth_fade-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "depth_fade"
);
chart.Show();</code></pre></div><div id="sunburst-fr-depth_fade-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "depth_fade"
)
chart.show()</code></pre></div><div id="sunburst-fr-depth_fade-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "depth_fade",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-depth_fade.html"></iframe>
</div>

<div class="sp-variant" id="sunburst-fr-mono">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mono"</code></span><span><strong>Aliases</strong> <code>mono / monochrome / single / uniform</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Anneaux monochromes differencies uniquement par opacite. Rendu editorial.</p>
<div class="sp-tabs" id="sunburst-fr-mono">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-fr-mono','sunburst-fr-mono-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-mono','sunburst-fr-mono-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-mono','sunburst-fr-mono-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-mono','sunburst-fr-mono-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-mono','sunburst-fr-mono-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-mono','sunburst-fr-mono-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-mono','sunburst-fr-mono-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-mono','sunburst-fr-mono-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-fr-mono','sunburst-fr-mono-cpp',this)">C++</button>
</div><div id="sunburst-fr-mono-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Tech","Web","Mobile","Cloud","Sales","NA","EU","APAC","Ops","Support","R&amp;D"]
parents = ["",    "Tech","Tech","Tech","",     "Sales","Sales","Sales","",   "Ops",    "Ops"]
values  = [0,     45,    32,    28,    0,      40,    35,    25,    0,    18,        22]

chart = sp.sunburst(
    title="Org Hierarchy",
    labels=labels,
    parents=parents,
    values=values,
    palette=[0x6366F1, 0x10B981, 0xF59E0B, 0xEF4444, 0xA855F7, 0x06B6D4],
    variant="mono",
)
chart.show()</code></pre></div><div id="sunburst-fr-mono-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "mono",
});
chart.show();</code></pre></div><div id="sunburst-fr-mono-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.sunburst({
  title: "Chart",
  labels: [/* ... */],
  values: [/* ... */],
  variant: "mono",
});
chart.show();</code></pre></div><div id="sunburst-fr-mono-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart &lt;- sp$sunburst(
  title = "Chart",
  labels = c(),
  values = c(),
  variant = "mono"
)
chart$show()</code></pre></div><div id="sunburst-fr-mono-rust" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-rust">use seraplot::sp;

fn main() {
    let chart = sp::sunburst()
        .title("Chart")
        .labels(vec![])
        .values(vec![])
        .variant("mono")
        .build();
    chart.show();
}</code></pre></div><div id="sunburst-fr-mono-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Chart")
    .labels(List.of())
    .values(List.of())
    .variant("mono")
    .build();
chart.show();</code></pre></div><div id="sunburst-fr-mono-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title: "Chart",
    labels: new string[]{},
    values: new double[]{},
    variant: "mono"
);
chart.Show();</code></pre></div><div id="sunburst-fr-mono-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Chart",
  labels  = List(),
  values  = List(),
  variant = "mono"
)
chart.show()</code></pre></div><div id="sunburst-fr-mono-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Chart",
    .labels  = {},
    .values  = {},
    .variant = "mono",
});
chart.show();</code></pre></div></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sunburst-mono.html"></iframe>
</div>

</div>
</div>

</div>
