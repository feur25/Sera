# Background Configuration

<div class="lang-en">

## Functions

### `set_global_background(color)`

Sets a global background color applied to all subsequently created charts.

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

<div class="sp-tabs" id="bg-set">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bg-set','bg-set-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bg-set','bg-set-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bg-set','bg-set-ts',this)">TypeScript</button>
</div>
<div id="bg-set-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">sp.set_global_background("#1a1a2e")</code></pre></div>
<div id="bg-set-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">sp.set_global_background("#1a1a2e");</code></pre></div>
<div id="bg-set-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">sp.set_global_background("#1a1a2e");</code></pre></div>
</div>

| Parameter | Type | Description |
|-----------|------|-------------|
| `color` | `str` / `string` | CSS color string (hex `"#rrggbb"`, `"rgb(…)"`, named color) |

---

### `reset_global_background()`

Clears the global background, reverting to the per-chart default.

<div class="sp-tabs" id="bg-reset">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bg-reset','bg-reset-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bg-reset','bg-reset-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bg-reset','bg-reset-ts',this)">TypeScript</button>
</div>
<div id="bg-reset-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">sp.reset_global_background()</code></pre></div>
<div id="bg-reset-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">sp.reset_global_background();</code></pre></div>
<div id="bg-reset-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">sp.reset_global_background();</code></pre></div>
</div>

---

### `set_bg_fn(html, color)`

Applies a background color to an existing HTML chart string. Returns the modified HTML string.

<div class="sp-tabs" id="bg-fn">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bg-fn','bg-fn-py',this)">Python</button>
</div>
<div id="bg-fn-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">html_str = chart.to_html()
html_with_bg = sp.set_bg_fn(html_str, "#0f172a")</code></pre></div>
</div>

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `str` | HTML string from `Chart.to_html()` |
| `color` | `str` | CSS background color |

**Returns**: `str` — Modified HTML string. (Python only)

---

## Examples

### Dark theme dashboard

<div class="sp-tabs" id="bg-ex">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bg-ex','bg-ex-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bg-ex','bg-ex-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bg-ex','bg-ex-ts',this)">TypeScript</button>
</div>
<div id="bg-ex-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
sp.set_global_background("#0f172a")
bar = sp.build_bar_chart("Revenue", labels=["A", "B"], values=[300, 200])
pie = sp.build_pie_chart("Share",   labels=["A", "B"], values=[60, 40])
sp.reset_global_background()</code></pre></div>
<div id="bg-ex-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
sp.set_global_background("#0f172a");
const bar = sp.build_bar_chart("Revenue", ["A", "B"], { values: [300, 200] });
const pie = sp.build_pie_chart("Share",   ["A", "B"], { values: [60, 40] });
sp.reset_global_background();</code></pre></div>
<div id="bg-ex-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
sp.set_global_background("#0f172a");
const bar = sp.build_bar_chart("Revenue", ["A", "B"], { values: [300, 200] });
const pie = sp.build_pie_chart("Share",   ["A", "B"], { values: [60, 40] });
sp.reset_global_background();</code></pre></div>
</div>

---

## See also

- [Palette & Colors](palette.md)
- [Auto Display](auto-display.md)

</div>

<div class="lang-fr">

## Fonctions

### `set_global_background(color)`

Définit une couleur de fond globale appliquée à tous les graphiques créés après l'appel.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `color` | `str` | Couleur CSS (hex `"#rrggbb"`, `"rgb(\u2026)"`, nom de couleur) |

### `reset_global_background()`

Efface le fond global, revenant à la valeur par défaut de chaque graphique.

### `set_bg_fn(html, color)`

Applique une couleur de fond à une chaîne HTML existante. Retourne la chaîne HTML modifiée.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `html` | `str` | Chaîne HTML de `Chart.to_html()` |
| `color` | `str` | Couleur de fond CSS |

**Retourne** : `str` — Chaîne HTML modifiée.

---

## Exemples

```python
import seraplot as sp

sp.set_global_background("#0f172a")

barre = sp.build_bar_chart("Revenus", labels=["A", "B"], values=[300, 200])
camembert = sp.build_pie_chart("Parts",   labels=["A", "B"], values=[60, 40])

sp.reset_global_background()
```

---

## Voir aussi

- [Palette & Couleurs](palette.md)
- [Affichage automatique](auto-display.md)

</div>

