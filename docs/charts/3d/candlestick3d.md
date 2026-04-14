# Candlestick Chart 3D

## Signature

```python
sp.build_candlestick3d_chart(
    title: str,
    dates: list[str],
    opens: list[float],
    highs: list[float],
    lows: list[float],
    closes: list[float],
    *,
    color_up: int = 0x22c55e,
    color_down: int = 0xef4444,
    bg_color: str = "#1a1a2e",
    width: int = 1000,
    height: int = 600,
    x_label: str = "",
    y_label: str = "",
    z_label: str = "",
    hover_json: str | None = None,
) -> Chart
```

---

## Description

OHLC candlestick chart rendered in 3D WebGL. Candles are extruded as 3D prisms.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `dates` | `list[str]` | required | Date labels |
| `opens` | `list[float]` | required | Open prices |
| `highs` | `list[float]` | required | High prices |
| `lows` | `list[float]` | required | Low prices |
| `closes` | `list[float]` | required | Close prices |
| `color_up` | `int` | `0x22c55e` | Bullish candle color |
| `color_down` | `int` | `0xef4444` | Bearish candle color |
| `bg_color` | `str` | `"#1a1a2e"` | Background |
| `width` | `int` | `1000` | Canvas width |
| `height` | `int` | `600` | Canvas height |

---

## Returns

`Chart`

---

## Examples




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
<div class="sp-tabs" id="candlestick3d">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('candlestick3d','candlestick3d-py',this)">Python</button><button class="sp-tb" onclick="spTab('candlestick3d','candlestick3d-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('candlestick3d','candlestick3d-ts',this)">TypeScript</button></div>
<div id="candlestick3d-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.build_candlestick3d_chart(
    "BTC/USD 3D",
    labels=["Mon", "Tue", "Wed", "Thu", "Fri"],
    open= [42000, 43500, 41800, 44000, 45200],
    high= [44000, 44200, 43500, 46000, 47000],
    low=  [41500, 43000, 40000, 43500, 44800],
    close=[43500, 41800, 44000, 45200, 46500],
)</code></pre></div>
<div id="candlestick3d-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.buildCandlestick3dChart("BTC/USD 3D",
["Mon", "Tue", "Wed", "Thu", "Fri"],
[42000, 43500, 41800, 44000, 45200],
[44000, 44200, 43500, 46000, 47000],
[41500, 43000, 40000, 43500, 44800],
{
    close: [43500, 41800, 44000, 45200, 46500]
})</code></pre></div>
<div id="candlestick3d-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.buildCandlestick3dChart("BTC/USD 3D",
["Mon", "Tue", "Wed", "Thu", "Fri"],
[42000, 43500, 41800, 44000, 45200],
[44000, 44200, 43500, 46000, 47000],
[41500, 43000, 40000, 43500, 44800],
{
    close: [43500, 41800, 44000, 45200, 46500]
})</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/candlestick3d.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Candlestick 2D](../2d/candlestick.md)
