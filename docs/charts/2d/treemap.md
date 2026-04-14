# Treemap

## Signature

```python
sp.build_treemap(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    parents: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Treemap — square-based space-filling hierarchy visualization.
Tiles are sized proportionally to their value.

When `parents` is provided, the hierarchy is rendered as nested rectangles.
Without `parents`, a flat treemap is drawn.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Tile labels |
| `values` | `list[float]` | required | Tile sizes |
| `parents` | `list[str] \| None` | `None` | Optional parent labels for hierarchy |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `palette` | `list[int] \| None` | `None` | Custom color palette |
| `background` | `str \| None` | `None` | Background color |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### Flat treemap (market cap)


### Hierarchical treemap





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
<div class="sp-tabs" id="treemap">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('treemap','treemap-py',this)">Python</button><button class="sp-tb" onclick="spTab('treemap','treemap-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('treemap','treemap-ts',this)">TypeScript</button></div>
<div id="treemap-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

labels  = ["Electronics", "Phones", "Laptops", "Clothing", "Shirts", "Pants"]
parents = ["",            "Electronics","Electronics","","Clothing","Clothing"]
values  = [1, 400, 350, 1, 200, 150]

chart = sp.build_treemap(
    "Revenue by Category",
    labels=labels,
    values=values,
    parents=parents,
)</code></pre></div>
<div id="treemap-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const labels  = ["Electronics", "Phones", "Laptops", "Clothing", "Shirts", "Pants"]
const parents = ["",            "Electronics","Electronics","","Clothing","Clothing"]
const values  = [1, 400, 350, 1, 200, 150]

const chart = sp.buildTreemap("Revenue by Category",
labels,
{
    values: values,
    parents: parents
})</code></pre></div>
<div id="treemap-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const labels: string[] = ["Electronics", "Phones", "Laptops", "Clothing", "Shirts", "Pants"]
const parents: string[] = ["",            "Electronics","Electronics","","Clothing","Clothing"]
const values: number[] = [1, 400, 350, 1, 200, 150]

const chart = sp.buildTreemap("Revenue by Category",
labels,
{
    values: values,
    parents: parents
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/treemap.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Sunburst](sunburst.md)
- [Bar Chart](bar.md)
