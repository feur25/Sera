
# Démarrage rapide

<div class="lang-en">

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
<div id="qs-save-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">chart.save(plot_name.html)</code></pre></div>
<div id="qs-save-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">chart.save(plot_name.html);</code></pre></div>
<div id="qs-save-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">chart.save(plot_name.html);</code></pre></div>
</div>

</div>

<div class="lang-fr">

## Votre premier graphique

<div class="sp-tabs" id="qs-bar-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-bar-fr','qs-bar-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-bar-fr','qs-bar-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-bar-fr','qs-bar-fr-ts',this)">TypeScript</button>
</div>
<div id="qs-bar-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.build_bar_chart(
    "Ventes par région",
    labels=["Nord", "Sud", "Est", "Ouest"],
    values=[120.0, 85.0, 200.0, 140.0],
)</code></pre></div>
<div id="qs-bar-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.build_bar_chart("Ventes par région", ["Nord", "Sud", "Est", "Ouest"], {
    values: [120.0, 85.0, 200.0, 140.0],
});</code></pre></div>
<div id="qs-bar-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.build_bar_chart("Ventes par région", ["Nord", "Sud", "Est", "Ouest"], {
    values: [120.0, 85.0, 200.0, 140.0],
});</code></pre></div>
</div>

> Dans Jupyter, le graphique s'affiche automatiquement.

---

## Enregistrer en HTML

<div class="sp-tabs" id="qs-save-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-save-fr','qs-save-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-save-fr','qs-save-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-save-fr','qs-save-fr-ts',this)">TypeScript</button>
</div>
<div id="qs-save-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">chart.save(nom_du_plot.html)</code></pre></div>
<div id="qs-save-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">chart.save(nom_du_plot.html);</code></pre></div>
<div id="qs-save-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">chart.save(nom_du_plot.html);</code></pre></div>
</div>
</div>
