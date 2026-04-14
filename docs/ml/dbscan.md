# DBSCAN Chart

## Signature

```python
sp.build_dbscan_chart(
    title: str,
    x_values: list[float],
    y_values: list[float],
    *,
    eps: float = 0.5,
    min_samples: int = 5,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    palette: list[int] | None = None,
    background: str | None = None,
    normalize: bool = False,
) -> Chart
```

---

## Description

2D DBSCAN clustering chart. Runs the DBSCAN algorithm (implemented in Rust) and plots each point colored by cluster membership. Noise points are shown in grey.

SeraPlot's DBSCAN runs up to **600× faster** than scikit-learn on large datasets.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x_values` | `list[float]` | required | X coordinates of data points |
| `y_values` | `list[float]` | required | Y coordinates of data points |
| `eps` | `float` | `0.5` | Maximum neighborhood distance (epsilon) |
| `min_samples` | `int` | `5` | Minimum points to form a dense region |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Show gridlines |
| `palette` | `list[int] \| None` | `None` | Custom cluster colors |
| `background` | `str \| None` | `None` | Chart background color |
| `normalize` | `bool` | `False` | Normalize features to [0, 1] before clustering |

---

## Returns

`Chart`

---

## Performance vs scikit-learn

SeraPlot's DBSCAN is implemented entirely in Rust with spatial indexing. On the same hardware and dataset it runs **up to 600× faster** than scikit-learn's implementation.

| Dataset size | SeraPlot | scikit-learn | Speedup |
|-------------|----------|-------------|---------|
| 1,000 pts | ~0.2 ms | ~5 ms | ~25× |
| 10,000 pts | ~1.5 ms | ~200 ms | ~130× |
| 100,000 pts | ~50 ms | ~30,000 ms | ~600× |
| 500,000 pts | ~280 ms | timeout | — |

The gap widens with dataset size because SeraPlot uses a KD-tree with SIMD acceleration internally, while scikit-learn's pure Python overhead dominates at high point counts.

`build_dbscan_chart` runs the algorithm and renders the chart in a single call. If you only need the cluster labels (no chart), use the [`DBSCAN` class](dbscan-class.md) which is sklearn-compatible (`fit`, `labels_`, `n_clusters_`, `n_noise_`).

---

## Choosing eps and min_samples

- **`eps`**: Start with a k-distance graph. A good `eps` is where the sorted k-nearest-neighbor distances show a "knee". Too small → everything is noise. Too large → everything is one cluster.
- **`min_samples`**: Typically set to `dim × 2` where `dim` is the number of features. Larger values produce more robust clusters but may mark more points as noise.

---

## Examples

### Synthetic blobs




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
<div class="sp-tabs" id="dbscan">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('dbscan','dbscan-py',this)">Python</button><button class="sp-tb" onclick="spTab('dbscan','dbscan-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('dbscan','dbscan-ts',this)">TypeScript</button></div>
<div id="dbscan-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random

def make_blob(cx, cy, n=150, s=0.5):
    return [(cx + random.gauss(0, s), cy + random.gauss(0, s)) for _ in range(n)]

pts  = make_blob(0, 0) + make_blob(5, 5) + make_blob(10, 0)
x, y = zip(*pts)

chart = sp.build_dbscan_chart(
    "DBSCAN Clustering",
    x_values=list(x),
    y_values=list(y),
    eps=1.0,
    min_samples=5,
    x_label="Feature 1",
    y_label="Feature 2",
)</code></pre></div>
<div id="dbscan-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
import random

def make_blob(cx, cy, {n: 150, s: 0.5}):
    return [(cx + random.gauss(0, s), cy + random.gauss(0, s)) for _ in range(n)]

const pts  = make_blob(0, 0) + make_blob(5, 5) + make_blob(10, 0)
x, y = zip(*pts)

const chart = sp.buildDbscanChart("DBSCAN Clustering",
list(x),
{
    y_values: list(y),
    eps: 1.0,
    min_samples: 5,
    x_label: "Feature 1",
    y_label: "Feature 2"
})</code></pre></div>
<div id="dbscan-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
import random

def make_blob(cx, cy, {n: 150, s: 0.5}):
    return [(cx + random.gauss(0, s), cy + random.gauss(0, s)) for _ in range(n)]

const pts  = make_blob(0, 0) + make_blob(5, 5) + make_blob(10, 0)
x, y = zip(*pts)

const chart = sp.buildDbscanChart("DBSCAN Clustering",
list(x),
{
    y_values: list(y),
    eps: 1.0,
    min_samples: 5,
    x_label: "Feature 1",
    y_label: "Feature 2"
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../previews/dbscan.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

### With normalization

```python
import seraplot as sp

chart = sp.build_dbscan_chart(
    "DBSCAN — Normalized",
    x_values=x,
    y_values=y,
    eps=0.1,
    min_samples=5,
    normalize=True,
)
```

---

## See also

- [DBSCAN Class](dbscan-class.md) — for accessing labels and cluster metadata
- [DBSCAN 3D](dbscan3d.md)
- [Scatter Chart](../charts/2d/scatter.md)
