
## Your first chart

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

<div class="sp-tabs" id="qs-bar">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-bar','qs-bar-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-bar','qs-bar-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-bar','qs-bar-ts',this)">TypeScript</button>
</div>
<div id="qs-bar-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.build_bar_chart(
    "Sales by Region",
    labels=["North", "South", "East", "West"],
    values=[120.0, 85.0, 200.0, 140.0],
)</code></pre></div>
<div id="qs-bar-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.build_bar_chart("Sales by Region", ["North", "South", "East", "West"], {
    values: [120.0, 85.0, 200.0, 140.0],
});</code></pre></div>
<div id="qs-bar-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.build_bar_chart("Sales by Region", ["North", "South", "East", "West"], {
    values: [120.0, 85.0, 200.0, 140.0],
});</code></pre></div>
</div>

> In Jupyter, the chart displays automatically.

---

## Save to HTML

<div class="sp-tabs" id="qs-save">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-save','qs-save-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-save','qs-save-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-save','qs-save-ts',this)">TypeScript</button>
</div>
<div id="qs-save-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">with open("chart.html", "w", encoding="utf-8") as f:
    f.write(chart.html)</code></pre></div>
<div id="qs-save-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const fs = require('fs');
fs.writeFileSync("chart.html", chart.html);</code></pre></div>
<div id="qs-save-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as fs from 'fs';
fs.writeFileSync("chart.html", chart.html);</code></pre></div>
</div>

---

## Common patterns

### Scatter plot with colored groups

<div class="sp-tabs" id="qs-scatter">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-scatter','qs-scatter-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-scatter','qs-scatter-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-scatter','qs-scatter-ts',this)">TypeScript</button>
</div>
<div id="qs-scatter-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import numpy as np
x = np.random.randn(500).tolist()
y = np.random.randn(500).tolist()
groups = (["A"] * 250) + (["B"] * 250)
chart = sp.build_scatter_chart(
    "Two Clusters",
    x_values=x,
    y_values=y,
    color_groups=groups,
)</code></pre></div>
<div id="qs-scatter-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const x = Array.from({ length: 500 }, () => (Math.random() * 6) - 3);
const y = Array.from({ length: 500 }, () => (Math.random() * 6) - 3);
const groups = [...Array(250).fill("A"), ...Array(250).fill("B")];
const chart = sp.build_scatter_chart("Two Clusters", x, {
    y_values: y,
    color_groups: groups,
});</code></pre></div>
<div id="qs-scatter-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const x: number[] = Array.from({ length: 500 }, () => (Math.random() * 6) - 3);
const y: number[] = Array.from({ length: 500 }, () => (Math.random() * 6) - 3);
const groups: string[] = [...Array(250).fill("A"), ...Array(250).fill("B")];
const chart = sp.build_scatter_chart("Two Clusters", x, {
    y_values: y,
    color_groups: groups,
});</code></pre></div>
</div>

---

### DBSCAN clustering in one call

<div class="sp-tabs" id="qs-dbscan">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-dbscan','qs-dbscan-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-dbscan','qs-dbscan-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-dbscan','qs-dbscan-ts',this)">TypeScript</button>
</div>
<div id="qs-dbscan-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import numpy as np
x = np.random.randn(10_000).tolist()
y = np.random.randn(10_000).tolist()
chart = sp.build_dbscan_chart(
    "DBSCAN",
    x_values=x,
    y_values=y,
    eps=0.3,
    min_samples=10,
)</code></pre></div>
<div id="qs-dbscan-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const x = Array.from({ length: 10000 }, () => (Math.random() * 8) - 4);
const y = Array.from({ length: 10000 }, () => (Math.random() * 8) - 4);
const chart = sp.build_dbscan_chart("DBSCAN", x, {
    y_values: y,
    eps: 0.3,
    min_samples: 10,
});</code></pre></div>
<div id="qs-dbscan-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const x: number[] = Array.from({ length: 10000 }, () => (Math.random() * 8) - 4);
const y: number[] = Array.from({ length: 10000 }, () => (Math.random() * 8) - 4);
const chart = sp.build_dbscan_chart("DBSCAN", x, {
    y_values: y,
    eps: 0.3,
    min_samples: 10,
});</code></pre></div>
</div>

---

### Global dark background for all charts

<div class="sp-tabs" id="qs-bg">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-bg','qs-bg-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-bg','qs-bg-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-bg','qs-bg-ts',this)">TypeScript</button>
</div>
<div id="qs-bg-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">sp.set_global_background("#0f172a")
chart1 = sp.build_bar_chart("Chart 1", labels=["A", "B"], values=[10.0, 20.0])
chart2 = sp.build_line_chart("Chart 2", labels=["x1", "x2"], values=[5.0, 15.0])
sp.reset_global_background()</code></pre></div>
<div id="qs-bg-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

sp.set_global_background("#0f172a");

const chart1 = sp.build_bar_chart("Chart 1", ["A", "B"], { values: [10.0, 20.0] });
const chart2 = sp.build_line_chart("Chart 2", ["x1", "x2"], { values: [5.0, 15.0] });

sp.reset_global_background();</code></pre></div>
<div id="qs-bg-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

sp.set_global_background("#0f172a");

const chart1 = sp.build_bar_chart("Chart 1", ["A", "B"], { values: [10.0, 20.0] });
const chart2 = sp.build_line_chart("Chart 2", ["x1", "x2"], { values: [5.0, 15.0] });

sp.reset_global_background();</code></pre></div>
</div>

---

### 3D scatter

<div class="sp-tabs" id="qs-scatter3d">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-scatter3d','qs-scatter3d-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-scatter3d','qs-scatter3d-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-scatter3d','qs-scatter3d-ts',this)">TypeScript</button>
</div>
<div id="qs-scatter3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import numpy as np
x = np.random.randn(1000).tolist()
y = np.random.randn(1000).tolist()
z = np.random.randn(1000).tolist()
chart = sp.build_scatter3d_chart(
    "3D Scatter",
    x_values=x,
    y_values=y,
    z_values=z,
    x_label="X", y_label="Y", z_label="Z",
)</code></pre></div>
<div id="qs-scatter3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const x = Array.from({ length: 1000 }, () => (Math.random() * 8) - 4);
const y = Array.from({ length: 1000 }, () => (Math.random() * 8) - 4);
const z = Array.from({ length: 1000 }, () => (Math.random() * 8) - 4);
const chart = sp.build_scatter3d_chart("3D Scatter", x, {
    y_values: y,
    z_values: z,
    x_label: "X",
    y_label: "Y",
    z_label: "Z",
});</code></pre></div>
<div id="qs-scatter3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const x: number[] = Array.from({ length: 1000 }, () => (Math.random() * 8) - 4);
const y: number[] = Array.from({ length: 1000 }, () => (Math.random() * 8) - 4);
const z: number[] = Array.from({ length: 1000 }, () => (Math.random() * 8) - 4);
const chart = sp.build_scatter3d_chart("3D Scatter", x, {
    y_values: y,
    z_values: z,
    x_label: "X",
    y_label: "Y",
    z_label: "Z",
});</code></pre></div>
</div>
```
