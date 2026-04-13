# Slideshow

## Signature

```python
sp.build_slideshow(
    charts: list[Chart],
    *,
    title: str = "",
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    autoplay: bool = False,
    interval_ms: int = 3000,
) -> Chart
```

---

## Description

Wraps multiple charts in an interactive slideshow with Previous / Next navigation controls.
All charts are pre-rendered; switching slides requires no server round-trip.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `charts` | `list[Chart]` | required | Chart objects to display |
| `title` | `str` | `""` | Slideshow title |
| `width` | `int` | `1000` | Container width in pixels |
| `height` | `int` | `600` | Container height in pixels |
| `background` | `str \| None` | `None` | Background color |
| `autoplay` | `bool` | `False` | Auto-advance slides |
| `interval_ms` | `int` | `3000` | Auto-advance interval in milliseconds |

---

## Returns

`Chart` (composite)

---

## Examples

### Quarterly report slideshow

```python
import seraplot as sp

slides = [
    sp.build_bar_chart("Q1 Revenue", labels=["A","B","C"], values=[120,80,95]),
    sp.build_line_chart("Growth Trend", labels=["Jan","Feb","Mar"], values=[10,14,18]),
    sp.build_pie_chart("Market Share", labels=["Us","Them"], values=[55,45]),
]

deck = sp.build_slideshow(slides, title="Q1 Board Deck")
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe srcdoc="<!DOCTYPE html><html><head><meta charset=&quot;utf-8&quot;><style>body{margin:0;padding:24px;background:#f0f2f5;display:flex;flex-direction:column;align-items:center;font-family:system-ui}.sp-frm{border-radius:12px;overflow:hidden;box-shadow:0 2px 12px rgba(0,0,0,.1);background:#fff}.sp-frm svg{width:900px;height:520px;display:block}.sp-ctrl{display:flex;gap:10px;margin-top:14px;align-items:center}.sp-btn{cursor:pointer;background:#6366f1;color:#fff;border:none;border-radius:8px;padding:7px 20px;font-size:14px;font-weight:600}.sp-btn:hover{background:#4f46e5}.sp-ctr{color:#64748b;font-size:13px;min-width:64px;text-align:center}.sp-prog{width:900px;height:4px;background:#e2e8f0;border-radius:2px;margin-top:10px;overflow:hidden}.sp-bar{height:100%;background:#6366f1;border-radius:2px;width:0%}</style><style>.sp-bg{fill:transparent!important}.sp-ttl{fill:#e2e8f0!important}svg text{fill:#cbd5e1!important}.sp-ax-x,.sp-ax-y{stroke:#475569!important}.sp-gl{stroke:#2d3748!important}.sp-xl,.sp-yl{fill:#94a3b8!important}[id^='spp']{box-shadow:none!important;border-radius:0!important}</style></head><body><div style=&quot;color:#1e293b;font-family:system-ui;font-size:22px;font-weight:700;text-align:center;margin-bottom:16px&quot;>Q1 Board Deck</div><div class=&quot;sp-frm&quot; id=&quot;sp-frm&quot;></div><div class=&quot;sp-ctrl&quot;><button class=&quot;sp-btn&quot; id=&quot;sp-p&quot;>&amp;#9664;</button><div class=&quot;sp-ctr&quot; id=&quot;sp-c&quot;>1 / 3</div><button class=&quot;sp-btn&quot; id=&quot;sp-n&quot;>&amp;#9654;</button></div><div class=&quot;sp-prog&quot;><div class=&quot;sp-bar&quot; id=&quot;sp-b&quot;></div></div><script type=&quot;application/json&quot; id=&quot;sp-d&quot;>[&quot;<svg xmlns=\&quot;http://www.w3.org/2000/svg\&quot; width=\&quot;900\&quot; height=\&quot;480\&quot; viewBox=\&quot;0 0 900 480\&quot; data-sp=\&quot;52,36,828,396\&quot;><rect class=\&quot;sp-bg\&quot; width=\&quot;100%\&quot; height=\&quot;100%\&quot;/><text x=\&quot;466\&quot; y=\&quot;24\&quot; text-anchor=\&quot;middle\&quot; font-family=\&quot;-apple-system,Arial,sans-serif\&quot; font-size=\&quot;15\&quot; font-weight=\&quot;700\&quot; fill=\&quot;#1a202c\&quot; class=\&quot;sp-ttl\&quot;>Q1 Revenue</text><text x=\&quot;48\&quot; y=\&quot;435\&quot; text-anchor=\&quot;end\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#9ca3af\&quot; class=\&quot;sp-yt\&quot;>0.00</text><text x=\&quot;48\&quot; y=\&quot;355\&quot; text-anchor=\&quot;end\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#9ca3af\&quot; class=\&quot;sp-yt\&quot;>24.00</text><text x=\&quot;48\&quot; y=\&quot;276\&quot; text-anchor=\&quot;end\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#9ca3af\&quot; class=\&quot;sp-yt\&quot;>48.00</text><text x=\&quot;48\&quot; y=\&quot;197\&quot; text-anchor=\&quot;end\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#9ca3af\&quot; class=\&quot;sp-yt\&quot;>72.00</text><text x=\&quot;48\&quot; y=\&quot;118\&quot; text-anchor=\&quot;end\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#9ca3af\&quot; class=\&quot;sp-yt\&quot;>96.00</text><text x=\&quot;48\&quot; y=\&quot;39\&quot; text-anchor=\&quot;end\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#9ca3af\&quot; class=\&quot;sp-yt\&quot;>120.00</text><line x1=\&quot;52\&quot; y1=\&quot;36\&quot; x2=\&quot;52\&quot; y2=\&quot;432\&quot; stroke=\&quot;#cbd5e1\&quot; stroke-width=\&quot;1\&quot; class=\&quot;sp-ax-y\&quot;/><line x1=\&quot;52\&quot; y1=\&quot;432\&quot; x2=\&quot;880\&quot; y2=\&quot;432\&quot; stroke=\&quot;#cbd5e1\&quot; stroke-width=\&quot;1\&quot; class=\&quot;sp-ax-x\&quot;/><rect data-idx=\&quot;0\&quot; data-series=\&quot;0\&quot; data-v=\&quot;120.00\&quot; data-lbl=\&quot;A\&quot; x=\&quot;52\&quot; y=\&quot;36\&quot; width=\&quot;275\&quot; height=\&quot;396\&quot; fill=\&quot;#6366f1\&quot; rx=\&quot;2\&quot;/><text x=\&quot;189\&quot; y=\&quot;446\&quot; text-anchor=\&quot;middle\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#6b7280\&quot;>A</text><rect data-idx=\&quot;1\&quot; data-series=\&quot;1\&quot; data-v=\&quot;80.00\&quot; data-lbl=\&quot;B\&quot; x=\&quot;328\&quot; y=\&quot;168\&quot; width=\&quot;275\&quot; height=\&quot;264\&quot; fill=\&quot;#f43f5e\&quot; rx=\&quot;2\&quot;/><text x=\&quot;465\&quot; y=\&quot;446\&quot; text-anchor=\&quot;middle\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#6b7280\&quot;>B</text><rect data-idx=\&quot;2\&quot; data-series=\&quot;2\&quot; data-v=\&quot;95.00\&quot; data-lbl=\&quot;C\&quot; x=\&quot;604\&quot; y=\&quot;119\&quot; width=\&quot;275\&quot; height=\&quot;313\&quot; fill=\&quot;#10b981\&quot; rx=\&quot;2\&quot;/><text x=\&quot;741\&quot; y=\&quot;446\&quot; text-anchor=\&quot;middle\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#6b7280\&quot;>C</text></svg>&quot;,&quot;<svg xmlns=\&quot;http://www.w3.org/2000/svg\&quot; width=\&quot;900\&quot; height=\&quot;480\&quot; viewBox=\&quot;0 0 900 480\&quot;><rect class=\&quot;sp-bg\&quot; width=\&quot;100%\&quot; height=\&quot;100%\&quot;/><text x=\&quot;450\&quot; y=\&quot;22\&quot; text-anchor=\&quot;middle\&quot; font-family=\&quot;-apple-system,Arial,sans-serif\&quot; font-size=\&quot;14\&quot; font-weight=\&quot;700\&quot; fill=\&quot;#1a202c\&quot;>Growth Trend</text><text x=\&quot;48\&quot; y=\&quot;435\&quot; text-anchor=\&quot;end\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#9ca3af\&quot; class=\&quot;sp-yt\&quot;>0.00</text><text x=\&quot;48\&quot; y=\&quot;355\&quot; text-anchor=\&quot;end\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#9ca3af\&quot; class=\&quot;sp-yt\&quot;>3.60</text><text x=\&quot;48\&quot; y=\&quot;276\&quot; text-anchor=\&quot;end\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#9ca3af\&quot; class=\&quot;sp-yt\&quot;>7.20</text><text x=\&quot;48\&quot; y=\&quot;197\&quot; text-anchor=\&quot;end\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#9ca3af\&quot; class=\&quot;sp-yt\&quot;>10.80</text><text x=\&quot;48\&quot; y=\&quot;118\&quot; text-anchor=\&quot;end\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#9ca3af\&quot; class=\&quot;sp-yt\&quot;>14.40</text><text x=\&quot;48\&quot; y=\&quot;39\&quot; text-anchor=\&quot;end\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#9ca3af\&quot; class=\&quot;sp-yt\&quot;>18.00</text><line x1=\&quot;52\&quot; y1=\&quot;36\&quot; x2=\&quot;52\&quot; y2=\&quot;432\&quot; stroke=\&quot;#cbd5e1\&quot; stroke-width=\&quot;1\&quot; class=\&quot;sp-ax-y\&quot;/><line x1=\&quot;52\&quot; y1=\&quot;432\&quot; x2=\&quot;880\&quot; y2=\&quot;432\&quot; stroke=\&quot;#cbd5e1\&quot; stroke-width=\&quot;1\&quot; class=\&quot;sp-ax-x\&quot;/><polyline fill=\&quot;none\&quot; stroke=\&quot;#6366f1\&quot; stroke-width=\&quot;2\&quot; stroke-linecap=\&quot;round\&quot; points=\&quot;52,212 466,124 880,36\&quot;/><circle data-idx=\&quot;0\&quot; data-y=\&quot;10.00\&quot; data-lbl=\&quot;Jan\&quot; cx=\&quot;52\&quot; cy=\&quot;212\&quot; r=\&quot;4\&quot; fill=\&quot;#6366f1\&quot; stroke=\&quot;#fff\&quot; stroke-width=\&quot;1.5\&quot;/><circle data-idx=\&quot;1\&quot; data-y=\&quot;14.00\&quot; data-lbl=\&quot;Feb\&quot; cx=\&quot;466\&quot; cy=\&quot;124\&quot; r=\&quot;4\&quot; fill=\&quot;#f43f5e\&quot; stroke=\&quot;#fff\&quot; stroke-width=\&quot;1.5\&quot;/><circle data-idx=\&quot;2\&quot; data-y=\&quot;18.00\&quot; data-lbl=\&quot;Mar\&quot; cx=\&quot;880\&quot; cy=\&quot;36\&quot; r=\&quot;4\&quot; fill=\&quot;#10b981\&quot; stroke=\&quot;#fff\&quot; stroke-width=\&quot;1.5\&quot;/><text x=\&quot;52\&quot; y=\&quot;446\&quot; text-anchor=\&quot;middle\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#6b7280\&quot;>Jan</text><text x=\&quot;466\&quot; y=\&quot;446\&quot; text-anchor=\&quot;middle\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#6b7280\&quot;>Feb</text><text x=\&quot;880\&quot; y=\&quot;446\&quot; text-anchor=\&quot;middle\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;9\&quot; fill=\&quot;#6b7280\&quot;>Mar</text></svg>&quot;,&quot;<svg xmlns=\&quot;http://www.w3.org/2000/svg\&quot; width=\&quot;720\&quot; height=\&quot;440\&quot; viewBox=\&quot;0 0 720 440\&quot;><rect class=\&quot;sp-bg\&quot; width=\&quot;100%\&quot; height=\&quot;100%\&quot;/><text x=\&quot;360\&quot; y=\&quot;22\&quot; text-anchor=\&quot;middle\&quot; font-family=\&quot;-apple-system,Arial,sans-serif\&quot; font-size=\&quot;15\&quot; font-weight=\&quot;700\&quot; fill=\&quot;#1a202c\&quot;>Market Share</text><path data-idx=\&quot;0\&quot; d=\&quot;M223.20,228.80 L223.20,55.83 A172.97,172.97 0 1,1 169.75,393.31 Z\&quot; data-lbl=\&quot;Us\&quot; data-v=\&quot;55.00\&quot; data-kv-Part=\&quot;55.00%\&quot; fill=\&quot;#6366f1\&quot; stroke=\&quot;#fff\&quot; stroke-width=\&quot;1.8\&quot;/><text x=\&quot;335.96\&quot; y=\&quot;246.66\&quot; text-anchor=\&quot;middle\&quot; dominant-baseline=\&quot;central\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;11\&quot; font-weight=\&quot;700\&quot; fill=\&quot;#fff\&quot;>55%</text><path data-idx=\&quot;1\&quot; d=\&quot;M223.20,228.80 L169.75,393.31 A172.97,172.97 0 0,1 223.20,55.83 Z\&quot; data-lbl=\&quot;Them\&quot; data-v=\&quot;45.00\&quot; data-kv-Part=\&quot;45.00%\&quot; fill=\&quot;#f43f5e\&quot; stroke=\&quot;#fff\&quot; stroke-width=\&quot;1.8\&quot;/><text x=\&quot;110.44\&quot; y=\&quot;210.94\&quot; text-anchor=\&quot;middle\&quot; dominant-baseline=\&quot;central\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;11\&quot; font-weight=\&quot;700\&quot; fill=\&quot;#fff\&quot;>45%</text><g data-legend=\&quot;1\&quot; data-series=\&quot;0\&quot;><rect x=\&quot;475\&quot; y=\&quot;198\&quot; width=\&quot;13\&quot; height=\&quot;13\&quot; rx=\&quot;3\&quot; fill=\&quot;#6366f1\&quot;/><text x=\&quot;492\&quot; y=\&quot;209\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;11\&quot; fill=\&quot;#374151\&quot;>Us (55.00%)</text></g><g data-legend=\&quot;1\&quot; data-series=\&quot;1\&quot;><rect x=\&quot;475\&quot; y=\&quot;220\&quot; width=\&quot;13\&quot; height=\&quot;13\&quot; rx=\&quot;3\&quot; fill=\&quot;#f43f5e\&quot;/><text x=\&quot;492\&quot; y=\&quot;231\&quot; font-family=\&quot;Arial,sans-serif\&quot; font-size=\&quot;11\&quot; fill=\&quot;#374151\&quot;>Them (45.00%)</text></g></svg>&quot;]</script><script>const frames=JSON.parse(document.getElementById('sp-d').textContent);let idx=0,timer;function show(i){idx=((i%frames.length)+frames.length)%frames.length;document.getElementById('sp-frm').innerHTML=frames[idx];document.getElementById('sp-c').textContent=(idx+1)+' / '+frames.length;const b=document.getElementById('sp-b');b.style.transition='none';b.style.width='0%';setTimeout(()=>{b.style.transition='width 2500ms linear';b.style.width='100%';},20);}function play(){clearInterval(timer);timer=setInterval(()=>{show(idx+1);},2500);}show(0);play();document.getElementById('sp-p').onclick=()=>{clearInterval(timer);show(idx-1);play();};document.getElementById('sp-n').onclick=()=>{clearInterval(timer);show(idx+1);play();};</script></body></html>" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117"></iframe>

</details>

---

## See also

- [Grid Layout](grid.md)
