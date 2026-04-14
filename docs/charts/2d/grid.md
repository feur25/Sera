# Grid Layout

## Signature

```python
sp.build_grid(
    charts: list[Chart],
    *,
    cols: int = 2,
    width: int = 1200,
    height: int = 800,
    background: str | None = None,
    gap: int = 12,
    title: str = "",
) -> Chart
```

---

## Description

Arranges multiple charts in a responsive grid layout within a single HTML output.

Arrange plusieurs graphiques dans une mise en page grille responsive dans un seul fichier HTML.

Charts are placed left-to-right, top-to-bottom. When `len(charts)` is not divisible by `cols`, the last row is left-aligned.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `charts` | `list[Chart]` | required | Chart objects to embed |
| `cols` | `int` | `2` | Number of columns |
| `width` | `int` | `1200` | Total grid container width in pixels |
| `height` | `int` | `800` | Total grid container height in pixels |
| `background` | `str \| None` | `None` | Grid background color |
| `gap` | `int` | `12` | Gap in pixels between cells |
| `title` | `str` | `""` | Optional header above the grid |

---

## Returns

`Chart` (composite)

---

## Examples

### Dashboard with 4 charts





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
<div class="sp-tabs" id="grid">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('grid','grid-py',this)">Python</button><button class="sp-tb" onclick="spTab('grid','grid-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('grid','grid-ts',this)">TypeScript</button></div>
<div id="grid-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

bar   = sp.build_bar_chart("Revenue", labels=["A","B","C"], values=[100,200,150])
pie   = sp.build_pie_chart("Share",   labels=["A","B"],     values=[60,40])
line  = sp.build_line_chart("Trend",  labels=["Jan","Feb","Mar"], values=[10,20,15])
hist  = sp.build_histogram("Dist",    values=[1,2,2,3,3,3,4,4,5])

dashboard = sp.build_grid(
    [bar, pie, line, hist],
    cols=2,
)</code></pre></div>
<div id="grid-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const bar   = sp.buildBarChart("Revenue",
["A", "B", "C"],
{
    values: [100, 200, 150]
})
const pie   = sp.buildPieChart("Share",
["A", "B"],
{
    values: [60, 40]
})
const line  = sp.buildLineChart("Trend",
["Jan", "Feb", "Mar"],
{
    values: [10, 20, 15]
})
const hist  = sp.buildHistogram("Dist",
{
    values: [1, 2, 2, 3, 3, 3, 4, 4, 5]
})

const dashboard = sp.buildGrid([bar, pie, line, hist],
2)</code></pre></div>
<div id="grid-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const bar   = sp.buildBarChart("Revenue",
["A", "B", "C"],
{
    values: [100, 200, 150]
})
const pie   = sp.buildPieChart("Share",
["A", "B"],
{
    values: [60, 40]
})
const line  = sp.buildLineChart("Trend",
["Jan", "Feb", "Mar"],
{
    values: [10, 20, 15]
})
const hist  = sp.buildHistogram("Dist",
{
    values: [1, 2, 2, 3, 3, 3, 4, 4, 5]
})

const dashboard = sp.buildGrid([bar, pie, line, hist],
2)</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/grid.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Slideshow](slideshow.md)
