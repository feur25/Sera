# Histogram

## Signature

```python
sp.build_histogram(
    title: str,
    values: list[float],
    *,
    color_hex: int = 0,
    bins: int = 20,
    show_counts: bool = False,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    hover_json: str = "",
    legend_position: str = "right",
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Distribution histogram with configurable bin count.

Histogramme de distribution avec nombre de bins configurable.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `values` | `list[float]` | required | Raw numerical data |
| `bins` | `int` | `20` | Number of histogram bins |
| `show_counts` | `bool` | `False` | Show count labels above each bar |
| `color_hex` | `int` | `0` | Bar color as hex int |
| `width` | `int` | `900` | Width in pixels |
| `height` | `int` | `480` | Height in pixels |

---

## Returns

`Chart`

---

## Examples

### Normal distribution

```python
import seraplot as sp
import numpy as np

data = np.random.normal(0, 1, 5000).tolist()

chart = sp.build_histogram(
    "Normal Distribution — N(0,1)",
    values=data,
    bins=40,
    x_label="Value",
    y_label="Count",
    gridlines=True,
    show_counts=False,
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe srcdoc="<!DOCTYPE html><html><head><meta charset=&quot;utf-8&quot;><title>Normal Distribution — N(0,1)</title><style>#sp-tip{position:absolute;z-index:999999;pointer-events:none;opacity:0;transition:opacity .15s,transform .15s;transform:translateY(6px) scale(.97);background:#0b0e18;color:#f1f5f9;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:13px;border-radius:10px;min-width:160px;max-width:340px;box-shadow:0 4px 20px rgba(0,0,0,.45),0 0 0 1px rgba(255,255,255,.08);overflow:hidden}#sp-tip.sp-vis{opacity:1;transform:translateY(0) scale(1);pointer-events:auto}.sp-nav{display:flex;align-items:center;justify-content:space-between;padding:5px 10px;border-top:1px solid rgba(255,255,255,.08)}.sp-nav-btn{cursor:pointer;padding:0 10px;border-radius:5px;height:22px;line-height:22px;font-size:18px;background:rgba(255,255,255,.10);color:#e2e8f0;user-select:none;flex-shrink:0}.sp-nav-btn:hover{background:rgba(255,255,255,.22)}.sp-nav-dis{opacity:.25;pointer-events:none}.sp-nav-ctr{flex:1;text-align:center;font-size:11px;color:#94a3b8}.sp-head{padding:10px 14px 6px;font-weight:700;font-size:14px;color:#e2e8f0;border-bottom:1px solid rgba(255,255,255,.08)}.sp-body{padding:8px 14px 12px}.sp-row{display:flex;justify-content:space-between;align-items:baseline;gap:14px;padding:3px 0;border-bottom:1px solid rgba(255,255,255,.04)}.sp-row:last-child{border-bottom:none}.sp-key{color:#94a3b8;font-size:12px;white-space:nowrap}.sp-val{font-weight:600;font-size:12px;color:#f8fafc;text-align:right;word-break:break-all}#sp-tip img{display:block;width:100%;max-height:210px;object-fit:contain;border-top:1px solid rgba(255,255,255,.07)}#sp-tip video{display:block;width:100%;border-top:1px solid rgba(255,255,255,.07)}.sp-html{padding:8px 14px;font-size:12px;border-top:1px solid rgba(255,255,255,.07)}[data-idx]{cursor:pointer;transition:opacity .25s,filter .2s,transform .25s}[data-idx]:hover{filter:brightness(1.12) saturate(1.08)}.sp-cpanel{position:absolute;bottom:10px;left:50%;transform:translateX(-50%);background:#0b0e18;color:#f1f5f9;border-radius:10px;padding:8px 16px;font-size:12px;font-family:-apple-system,Arial,sans-serif;box-shadow:0 8px 24px rgba(0,0,0,.4);z-index:20;white-space:nowrap;display:none}.sp-cls-x{cursor:pointer;color:#94a3b8;margin-left:6px;font-size:13px}.sp-cls-x:hover{color:#f87171}</style><style>body{margin:0;background:#0d1117;display:flex;justify-content:center;padding:0}@keyframes sp-i{from{opacity:0;transform:translateY(8px) scale(.94)}}@keyframes sp-bar{from{opacity:0;transform:scaleY(0)}}@keyframes sp-pop{0%{opacity:0;transform:scale(0)}70%{transform:scale(1.08)}100%{opacity:1;transform:scale(1)}}@keyframes sp-arc{from{opacity:0;transform:scale(.82) rotate(-6deg)}}@keyframes sp-fn{from{opacity:0;transform:scaleX(.7) translateY(8px)}}svg rect[data-idx]{transform-box:fill-box;transform-origin:bottom center;animation:sp-bar .5s cubic-bezier(.22,.61,.36,1) both}svg circle[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-pop .42s cubic-bezier(.34,1.56,.64,1) both}svg path[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-arc .48s cubic-bezier(.22,.61,.36,1) both}svg polygon[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-fn .48s cubic-bezier(.22,.61,.36,1) both}svg line[data-idx]{animation:sp-i .45s cubic-bezier(.22,.61,.36,1) both}svg rect[data-idx]:hover{transform:scaleY(1.03);filter:brightness(1.12) saturate(1.1)}svg circle[data-idx]:hover{transform:scale(1.3);filter:brightness(1.15)}svg path[data-idx]:hover{filter:brightness(1.1) drop-shadow(0 2px 8px rgba(0,0,0,.2))}.sp-bg{fill:#fff}</style><style>.sp-bg{fill:transparent!important}.sp-ttl{fill:#e2e8f0!important}svg text{fill:#cbd5e1!important}.sp-ax-x,.sp-ax-y{stroke:#475569!important}.sp-gl{stroke:#2d3748!important}.sp-xl,.sp-yl{fill:#94a3b8!important}[id^='spp']{box-shadow:none!important;border-radius:0!important}</style></head><body><div id=&quot;spp1&quot; style=&quot;position:relative;display:inline-block;border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)&quot;><svg xmlns=&quot;http://www.w3.org/2000/svg&quot; width=&quot;860&quot; height=&quot;380&quot; viewBox=&quot;0 0 860 380&quot;><rect class=&quot;sp-bg&quot; width=&quot;100%&quot; height=&quot;100%&quot;/><text x=&quot;446&quot; y=&quot;26&quot; text-anchor=&quot;middle&quot; font-family=&quot;-apple-system,Arial,sans-serif&quot; font-size=&quot;15&quot; font-weight=&quot;700&quot; fill=&quot;#1a202c&quot; class=&quot;sp-ttl&quot;>Normal Distribution — N(0,1)</text><text x=&quot;48&quot; y=&quot;337&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>0</text><line x1=&quot;52&quot; y1=&quot;274&quot; x2=&quot;840&quot; y2=&quot;274&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;48&quot; y=&quot;277&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>62</text><line x1=&quot;52&quot; y1=&quot;214&quot; x2=&quot;840&quot; y2=&quot;214&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;48&quot; y=&quot;217&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>124</text><line x1=&quot;52&quot; y1=&quot;155&quot; x2=&quot;840&quot; y2=&quot;155&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;48&quot; y=&quot;158&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>186</text><line x1=&quot;52&quot; y1=&quot;95&quot; x2=&quot;840&quot; y2=&quot;95&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;48&quot; y=&quot;98&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>248</text><line x1=&quot;52&quot; y1=&quot;36&quot; x2=&quot;840&quot; y2=&quot;36&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;48&quot; y=&quot;39&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>310</text><line x1=&quot;52&quot; y1=&quot;36&quot; x2=&quot;52&quot; y2=&quot;334&quot; stroke=&quot;#cbd5e1&quot; stroke-width=&quot;1&quot; class=&quot;sp-ax-y&quot;/><line x1=&quot;52&quot; y1=&quot;334&quot; x2=&quot;840&quot; y2=&quot;334&quot; stroke=&quot;#cbd5e1&quot; stroke-width=&quot;1&quot; class=&quot;sp-ax-x&quot;/><text x=&quot;446&quot; y=&quot;376&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;10&quot; fill=&quot;#6b7280&quot; class=&quot;sp-xl&quot;>Value</text><text x=&quot;14&quot; y=&quot;185&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;10&quot; fill=&quot;#6b7280&quot; transform=&quot;rotate(-90,14,185)&quot; class=&quot;sp-yl&quot;>Count</text><rect data-idx=&quot;0&quot; data-series=&quot;0&quot; data-lbl=&quot;-3.03–-2.89&quot; data-kv-Count=&quot;5&quot; data-kv-Min=&quot;-3.03&quot; data-kv-Max=&quot;-2.89&quot; x=&quot;52&quot; y=&quot;330&quot; width=&quot;18&quot; height=&quot;4&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;1&quot; data-series=&quot;0&quot; data-lbl=&quot;-2.89–-2.74&quot; data-kv-Count=&quot;5&quot; data-kv-Min=&quot;-2.89&quot; data-kv-Max=&quot;-2.74&quot; x=&quot;71&quot; y=&quot;330&quot; width=&quot;18&quot; height=&quot;4&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;2&quot; data-series=&quot;0&quot; data-lbl=&quot;-2.74–-2.59&quot; data-kv-Count=&quot;10&quot; data-kv-Min=&quot;-2.74&quot; data-kv-Max=&quot;-2.59&quot; x=&quot;91&quot; y=&quot;325&quot; width=&quot;18&quot; height=&quot;9&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;3&quot; data-series=&quot;0&quot; data-lbl=&quot;-2.59–-2.44&quot; data-kv-Count=&quot;0&quot; data-kv-Min=&quot;-2.59&quot; data-kv-Max=&quot;-2.44&quot; x=&quot;111&quot; y=&quot;334&quot; width=&quot;18&quot; height=&quot;0&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;4&quot; data-series=&quot;0&quot; data-lbl=&quot;-2.44–-2.29&quot; data-kv-Count=&quot;30&quot; data-kv-Min=&quot;-2.44&quot; data-kv-Max=&quot;-2.29&quot; x=&quot;130&quot; y=&quot;306&quot; width=&quot;18&quot; height=&quot;28&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;5&quot; data-series=&quot;0&quot; data-lbl=&quot;-2.29–-2.14&quot; data-kv-Count=&quot;35&quot; data-kv-Min=&quot;-2.29&quot; data-kv-Max=&quot;-2.14&quot; x=&quot;150&quot; y=&quot;301&quot; width=&quot;18&quot; height=&quot;33&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;6&quot; data-series=&quot;0&quot; data-lbl=&quot;-2.14–-2.00&quot; data-kv-Count=&quot;25&quot; data-kv-Min=&quot;-2.14&quot; data-kv-Max=&quot;-2.00&quot; x=&quot;170&quot; y=&quot;310&quot; width=&quot;18&quot; height=&quot;24&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;7&quot; data-series=&quot;0&quot; data-lbl=&quot;-2.00–-1.85&quot; data-kv-Count=&quot;35&quot; data-kv-Min=&quot;-2.00&quot; data-kv-Max=&quot;-1.85&quot; x=&quot;189&quot; y=&quot;301&quot; width=&quot;18&quot; height=&quot;33&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;8&quot; data-series=&quot;0&quot; data-lbl=&quot;-1.85–-1.70&quot; data-kv-Count=&quot;55&quot; data-kv-Min=&quot;-1.85&quot; data-kv-Max=&quot;-1.70&quot; x=&quot;209&quot; y=&quot;282&quot; width=&quot;18&quot; height=&quot;52&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;9&quot; data-series=&quot;0&quot; data-lbl=&quot;-1.70–-1.55&quot; data-kv-Count=&quot;85&quot; data-kv-Min=&quot;-1.70&quot; data-kv-Max=&quot;-1.55&quot; x=&quot;229&quot; y=&quot;253&quot; width=&quot;18&quot; height=&quot;81&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;10&quot; data-series=&quot;0&quot; data-lbl=&quot;-1.55–-1.40&quot; data-kv-Count=&quot;115&quot; data-kv-Min=&quot;-1.55&quot; data-kv-Max=&quot;-1.40&quot; x=&quot;249&quot; y=&quot;224&quot; width=&quot;18&quot; height=&quot;110&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;11&quot; data-series=&quot;0&quot; data-lbl=&quot;-1.40–-1.25&quot; data-kv-Count=&quot;125&quot; data-kv-Min=&quot;-1.40&quot; data-kv-Max=&quot;-1.25&quot; x=&quot;268&quot; y=&quot;214&quot; width=&quot;18&quot; height=&quot;120&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;12&quot; data-series=&quot;0&quot; data-lbl=&quot;-1.25–-1.11&quot; data-kv-Count=&quot;150&quot; data-kv-Min=&quot;-1.25&quot; data-kv-Max=&quot;-1.11&quot; x=&quot;288&quot; y=&quot;190&quot; width=&quot;18&quot; height=&quot;144&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;13&quot; data-series=&quot;0&quot; data-lbl=&quot;-1.11–-0.96&quot; data-kv-Count=&quot;170&quot; data-kv-Min=&quot;-1.11&quot; data-kv-Max=&quot;-0.96&quot; x=&quot;308&quot; y=&quot;171&quot; width=&quot;18&quot; height=&quot;163&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;14&quot; data-series=&quot;0&quot; data-lbl=&quot;-0.96–-0.81&quot; data-kv-Count=&quot;265&quot; data-kv-Min=&quot;-0.96&quot; data-kv-Max=&quot;-0.81&quot; x=&quot;327&quot; y=&quot;80&quot; width=&quot;18&quot; height=&quot;254&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;15&quot; data-series=&quot;0&quot; data-lbl=&quot;-0.81–-0.66&quot; data-kv-Count=&quot;285&quot; data-kv-Min=&quot;-0.81&quot; data-kv-Max=&quot;-0.66&quot; x=&quot;347&quot; y=&quot;61&quot; width=&quot;18&quot; height=&quot;273&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;16&quot; data-series=&quot;0&quot; data-lbl=&quot;-0.66–-0.51&quot; data-kv-Count=&quot;240&quot; data-kv-Min=&quot;-0.66&quot; data-kv-Max=&quot;-0.51&quot; x=&quot;367&quot; y=&quot;104&quot; width=&quot;18&quot; height=&quot;230&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;17&quot; data-series=&quot;0&quot; data-lbl=&quot;-0.51–-0.36&quot; data-kv-Count=&quot;235&quot; data-kv-Min=&quot;-0.51&quot; data-kv-Max=&quot;-0.36&quot; x=&quot;386&quot; y=&quot;109&quot; width=&quot;18&quot; height=&quot;225&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;18&quot; data-series=&quot;0&quot; data-lbl=&quot;-0.36–-0.22&quot; data-kv-Count=&quot;285&quot; data-kv-Min=&quot;-0.36&quot; data-kv-Max=&quot;-0.22&quot; x=&quot;406&quot; y=&quot;61&quot; width=&quot;18&quot; height=&quot;273&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;19&quot; data-series=&quot;0&quot; data-lbl=&quot;-0.22–-0.07&quot; data-kv-Count=&quot;310&quot; data-kv-Min=&quot;-0.22&quot; data-kv-Max=&quot;-0.07&quot; x=&quot;426&quot; y=&quot;36&quot; width=&quot;18&quot; height=&quot;298&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;20&quot; data-series=&quot;0&quot; data-lbl=&quot;-0.07–0.08&quot; data-kv-Count=&quot;250&quot; data-kv-Min=&quot;-0.07&quot; data-kv-Max=&quot;0.08&quot; x=&quot;446&quot; y=&quot;94&quot; width=&quot;18&quot; height=&quot;240&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;21&quot; data-series=&quot;0&quot; data-lbl=&quot;0.08–0.23&quot; data-kv-Count=&quot;255&quot; data-kv-Min=&quot;0.08&quot; data-kv-Max=&quot;0.23&quot; x=&quot;465&quot; y=&quot;89&quot; width=&quot;18&quot; height=&quot;245&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;22&quot; data-series=&quot;0&quot; data-lbl=&quot;0.23–0.38&quot; data-kv-Count=&quot;290&quot; data-kv-Min=&quot;0.23&quot; data-kv-Max=&quot;0.38&quot; x=&quot;485&quot; y=&quot;56&quot; width=&quot;18&quot; height=&quot;278&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;23&quot; data-series=&quot;0&quot; data-lbl=&quot;0.38–0.53&quot; data-kv-Count=&quot;265&quot; data-kv-Min=&quot;0.38&quot; data-kv-Max=&quot;0.53&quot; x=&quot;505&quot; y=&quot;80&quot; width=&quot;18&quot; height=&quot;254&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;24&quot; data-series=&quot;0&quot; data-lbl=&quot;0.53–0.67&quot; data-kv-Count=&quot;205&quot; data-kv-Min=&quot;0.53&quot; data-kv-Max=&quot;0.67&quot; x=&quot;524&quot; y=&quot;137&quot; width=&quot;18&quot; height=&quot;197&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;25&quot; data-series=&quot;0&quot; data-lbl=&quot;0.67–0.82&quot; data-kv-Count=&quot;185&quot; data-kv-Min=&quot;0.67&quot; data-kv-Max=&quot;0.82&quot; x=&quot;544&quot; y=&quot;157&quot; width=&quot;18&quot; height=&quot;177&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;26&quot; data-series=&quot;0&quot; data-lbl=&quot;0.82–0.97&quot; data-kv-Count=&quot;200&quot; data-kv-Min=&quot;0.82&quot; data-kv-Max=&quot;0.97&quot; x=&quot;564&quot; y=&quot;142&quot; width=&quot;18&quot; height=&quot;192&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;27&quot; data-series=&quot;0&quot; data-lbl=&quot;0.97–1.12&quot; data-kv-Count=&quot;240&quot; data-kv-Min=&quot;0.97&quot; data-kv-Max=&quot;1.12&quot; x=&quot;583&quot; y=&quot;104&quot; width=&quot;18&quot; height=&quot;230&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;28&quot; data-series=&quot;0&quot; data-lbl=&quot;1.12–1.27&quot; data-kv-Count=&quot;155&quot; data-kv-Min=&quot;1.12&quot; data-kv-Max=&quot;1.27&quot; x=&quot;603&quot; y=&quot;185&quot; width=&quot;18&quot; height=&quot;149&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;29&quot; data-series=&quot;0&quot; data-lbl=&quot;1.27–1.42&quot; data-kv-Count=&quot;110&quot; data-kv-Min=&quot;1.27&quot; data-kv-Max=&quot;1.42&quot; x=&quot;623&quot; y=&quot;229&quot; width=&quot;18&quot; height=&quot;105&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;30&quot; data-series=&quot;0&quot; data-lbl=&quot;1.42–1.56&quot; data-kv-Count=&quot;75&quot; data-kv-Min=&quot;1.42&quot; data-kv-Max=&quot;1.56&quot; x=&quot;643&quot; y=&quot;262&quot; width=&quot;18&quot; height=&quot;72&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;31&quot; data-series=&quot;0&quot; data-lbl=&quot;1.56–1.71&quot; data-kv-Count=&quot;75&quot; data-kv-Min=&quot;1.56&quot; data-kv-Max=&quot;1.71&quot; x=&quot;662&quot; y=&quot;262&quot; width=&quot;18&quot; height=&quot;72&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;32&quot; data-series=&quot;0&quot; data-lbl=&quot;1.71–1.86&quot; data-kv-Count=&quot;65&quot; data-kv-Min=&quot;1.71&quot; data-kv-Max=&quot;1.86&quot; x=&quot;682&quot; y=&quot;272&quot; width=&quot;18&quot; height=&quot;62&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;33&quot; data-series=&quot;0&quot; data-lbl=&quot;1.86–2.01&quot; data-kv-Count=&quot;55&quot; data-kv-Min=&quot;1.86&quot; data-kv-Max=&quot;2.01&quot; x=&quot;702&quot; y=&quot;282&quot; width=&quot;18&quot; height=&quot;52&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;34&quot; data-series=&quot;0&quot; data-lbl=&quot;2.01–2.16&quot; data-kv-Count=&quot;30&quot; data-kv-Min=&quot;2.01&quot; data-kv-Max=&quot;2.16&quot; x=&quot;721&quot; y=&quot;306&quot; width=&quot;18&quot; height=&quot;28&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;35&quot; data-series=&quot;0&quot; data-lbl=&quot;2.16–2.30&quot; data-kv-Count=&quot;25&quot; data-kv-Min=&quot;2.16&quot; data-kv-Max=&quot;2.30&quot; x=&quot;741&quot; y=&quot;310&quot; width=&quot;18&quot; height=&quot;24&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;36&quot; data-series=&quot;0&quot; data-lbl=&quot;2.30–2.45&quot; data-kv-Count=&quot;20&quot; data-kv-Min=&quot;2.30&quot; data-kv-Max=&quot;2.45&quot; x=&quot;761&quot; y=&quot;315&quot; width=&quot;18&quot; height=&quot;19&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;37&quot; data-series=&quot;0&quot; data-lbl=&quot;2.45–2.60&quot; data-kv-Count=&quot;5&quot; data-kv-Min=&quot;2.45&quot; data-kv-Max=&quot;2.60&quot; x=&quot;780&quot; y=&quot;330&quot; width=&quot;18&quot; height=&quot;4&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;38&quot; data-series=&quot;0&quot; data-lbl=&quot;2.60–2.75&quot; data-kv-Count=&quot;25&quot; data-kv-Min=&quot;2.60&quot; data-kv-Max=&quot;2.75&quot; x=&quot;800&quot; y=&quot;310&quot; width=&quot;18&quot; height=&quot;24&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;39&quot; data-series=&quot;0&quot; data-lbl=&quot;2.75–2.90&quot; data-kv-Count=&quot;5&quot; data-kv-Min=&quot;2.75&quot; data-kv-Max=&quot;2.90&quot; x=&quot;820&quot; y=&quot;330&quot; width=&quot;18&quot; height=&quot;4&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;52&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>-3.03</text><text x=&quot;150&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>-2.29</text><text x=&quot;249&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>-1.55</text><text x=&quot;347&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>-0.81</text><text x=&quot;446&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>-0.07</text><text x=&quot;544&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>0.67</text><text x=&quot;643&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>1.42</text><text x=&quot;741&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>2.16</text><text x=&quot;840&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>2.90</text></svg><div class=&quot;sp-sel-ov&quot; style=&quot;display:none&quot;></div><div class=&quot;sp-cpanel&quot;></div><script>(function(){
var wrap=document.getElementById('spp1');if(!wrap)return;wrap.removeAttribute('id');
var svg=wrap.querySelector('svg');var data=[{&quot;title&quot;:&quot;-3.03–-2.89&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-2.89–-2.74&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-2.74–-2.59&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-2.59–-2.44&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-2.44–-2.29&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-2.29–-2.14&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-2.14–-2.00&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-2.00–-1.85&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-1.85–-1.70&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-1.70–-1.55&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-1.55–-1.40&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-1.40–-1.25&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-1.25–-1.11&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-1.11–-0.96&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-0.96–-0.81&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-0.81–-0.66&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-0.66–-0.51&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-0.51–-0.36&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-0.36–-0.22&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-0.22–-0.07&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;-0.07–0.08&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;0.08–0.23&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;0.23–0.38&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;0.38–0.53&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;0.53–0.67&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;0.67–0.82&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;0.82–0.97&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;0.97–1.12&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;1.12–1.27&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;1.27–1.42&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;1.42–1.56&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;1.56–1.71&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;1.71–1.86&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;1.86–2.01&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;2.01–2.16&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;2.16–2.30&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;2.30–2.45&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;2.45–2.60&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;2.60–2.75&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;2.75–2.90&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;}];

var tip=document.getElementById('sp-tip');
if(!tip){tip=document.createElement('div');tip.id='sp-tip';document.body.appendChild(tip);}
var tipIdxs=[],tipPos=0,lastE=null;
function esc(s){return String(s).replace(/&amp;/g,'&amp;amp;').replace(/</g,'&amp;lt;');}
function cardHTML(d){
 var h='';if(d.title)h+='<div class=&quot;sp-head&quot;>'+esc(d.title)+'</div>';
 var rows='';(d.kv||[]).forEach(function(p){
  rows+='<div class=&quot;sp-row&quot;><span class=&quot;sp-key&quot;>'+esc(p[0])+'</span><span class=&quot;sp-val&quot;>'+esc(p[1])+'</span></div>';});
 if(rows)h+='<div class=&quot;sp-body&quot;>'+rows+'</div>';
 if(d.image)h+='<img src=&quot;'+esc(d.image)+'&quot; alt=&quot;&quot; loading=&quot;eager&quot;>';
 if(d.video)h+='<video src=&quot;'+esc(d.video)+'&quot; controls muted playsinline></video>';
 if(d.html)h+='<div class=&quot;sp-html&quot;>'+d.html+'</div>';
 return h;}
function placeTip(e){
 var sx=window.scrollX||0,sy=window.scrollY||0;
 var x=e.pageX+16,y=e.pageY-14;
 var tw=tip.offsetWidth||220,th=tip.offsetHeight||60;
 if(x+tw>sx+window.innerWidth-6)x=e.pageX-tw-16;
 if(x<sx+4)x=sx+4;
 if(y<sy+6)y=e.pageY+20;
 if(y+th>sy+window.innerHeight-6)y=sy+window.innerHeight-th-6;
 tip.style.left=x+'px';tip.style.top=y+'px';}

function getSlot(idx){
 var d=data[idx];if(d)return d;
 var el=svg.querySelector('[data-idx=&quot;'+idx+'&quot;]');if(!el)return null;
 var kv=[];
 var x=el.getAttribute('data-x'),y=el.getAttribute('data-y');
 if(x!=null){var xf=parseFloat(x);kv.push(['X',xf===xf?xf.toFixed(2):x]);}
 if(y!=null){kv.push(['Valeur',parseFloat(y).toFixed(2)]);}
 else{var v=el.getAttribute('data-v');
 if(v!=null){
  var r=el.getAttribute('data-r'),c=el.getAttribute('data-c');
  kv.push(['Valeur',parseFloat(v).toFixed(3)]);
  if(r)kv.push(['Ligne',r]);if(c)kv.push(['Colonne',c]);}}
 var z=el.getAttribute('data-z');
 if(z!=null){kv.push(['Z',parseFloat(z).toFixed(2)]);}
 var lbl=el.getAttribute('data-lbl');
 var an=el.attributes;if(an){for(var ai=0;ai<an.length;ai++){var a=an[ai];if(a.name.substring(0,8)==='data-kv-'){kv.push([a.name.substring(8),a.value]);}}}
 var title=lbl||(el.getAttribute('data-v')!=null&amp;&amp;el.getAttribute('data-r')!=null?el.getAttribute('data-r')+' \u00d7 '+el.getAttribute('data-c'):'Point '+(idx+1));
 return{title:title,kv:kv};}
function renderTip(){
 if(!tipIdxs.length){tip.classList.remove('sp-vis');return;}
 var d=getSlot(tipIdxs[tipPos]);if(!d){tip.classList.remove('sp-vis');return;}
 var h=cardHTML(d);
 if(tipIdxs.length>1){
  var p=tipPos,n=tipIdxs.length;
  h+='<div class=&quot;sp-nav&quot;>'
   +'<span class=&quot;sp-nav-btn'+(p>0?'':' sp-nav-dis')+'&quot; data-d=&quot;-1&quot;>&amp;#8249;</span>'
   +'<span class=&quot;sp-nav-ctr&quot;>'+(p+1)+' / '+n+'</span>'
   +'<span class=&quot;sp-nav-btn'+(p<n-1?'':' sp-nav-dis')+'&quot; data-d=&quot;1&quot;>&amp;#8250;</span>'
   +'</div>';}
 tip.innerHTML=h;tip.classList.add('sp-vis');
 if(lastE)placeTip(lastE);
 tip.querySelectorAll('.sp-nav-btn:not(.sp-nav-dis)').forEach(function(btn){
  btn.onclick=function(ev){ev.stopPropagation();
   var nd=tipPos+parseInt(btn.getAttribute('data-d'),10);
   if(nd>=0&amp;&amp;nd<tipIdxs.length){tipPos=nd;renderTip();}};});}
var dragging=false,dsx=0,dsy=0,moved=false,pinned=false;

wrap.addEventListener('mouseleave',function(e){
 if(pinned)return;
 var rt=e.relatedTarget;
 if(rt&amp;&amp;(rt===tip||tip.contains(rt)))return;
 tip.classList.remove('sp-vis');tipIdxs=[];});
wrap.addEventListener('mousemove',function(e){
 if(dragging||pinned)return;
 lastE=e;

 var hits=[];
 document.elementsFromPoint(e.clientX,e.clientY).forEach(function(el){
  if(el===tip||tip.contains(el))return;
  var found=null;
  for(var n=el;n&amp;&amp;n!==document.body;n=n.parentElement){
   if(found===null&amp;&amp;n.getAttribute&amp;&amp;n.getAttribute('data-idx')!==null)found=n;
   if(n===wrap){
    if(found!==null){var idx=parseInt(found.getAttribute('data-idx'),10);
     if(hits.indexOf(idx)===-1)hits.push(idx);}
    return;}
  }
 });
 if(hits.length){
  var same=hits.length===tipIdxs.length&amp;&amp;hits.every(function(v,i){return v===tipIdxs[i];});
  if(!same){tipIdxs=hits;tipPos=0;renderTip();}else placeTip(e);
 }else{tip.classList.remove('sp-vis');tipIdxs=[];}});

var origVB=svg.getAttribute('viewBox')||'';
function parseVB(s){return s.split(/[\s,]+/).map(Number);}
function animateVB(x,y,w,h){
 svg.style.transition='all 0.5s cubic-bezier(.4,0,.2,1)';
 svg.setAttribute('viewBox',x+' '+y+' '+w+' '+h);}
function resetVB(){
 svg.style.transition='all 0.45s cubic-bezier(.4,0,.2,1)';
 if(origVB)svg.setAttribute('viewBox',origVB);
 setTimeout(function(){svg.style.transition='';},500);}
function reAnim(){var els=svg.querySelectorAll('[data-idx]');els.forEach(function(el){el.style.animation='none';el.style.filter='';});void svg.offsetHeight;els.forEach(function(el,i){el.style.animation='';el.style.animationDelay=(i*14)+'ms';});}
var ov=wrap.querySelector('.sp-sel-ov');var panel=wrap.querySelector('.sp-cpanel');
wrap.addEventListener('mousedown',function(e){
 if(e.button!==0)return;
 var t=e.target;if(t===panel||panel.contains(t))return;if(t===tip||tip.contains(t))return;
 dragging=true;moved=false;
 var r=wrap.getBoundingClientRect();dsx=e.clientX-r.left;dsy=e.clientY-r.top;
 ov.style.cssText='display:none;position:absolute;pointer-events:none;border:2px solid #6366F1;background:rgba(99,102,241,.12);box-sizing:border-box;border-radius:3px;z-index:10';
 e.preventDefault();});
document.addEventListener('mousemove',function(e){if(!dragging)return;
 var r=wrap.getBoundingClientRect();var cx=e.clientX-r.left,cy=e.clientY-r.top;
 if(!moved&amp;&amp;Math.abs(cx-dsx)<8&amp;&amp;Math.abs(cy-dsy)<8)return;
 if(!moved){pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}
 moved=true;ov.style.display='block';
 ov.style.left=Math.min(dsx,cx)+'px';ov.style.top=Math.min(dsy,cy)+'px';
 ov.style.width=Math.abs(cx-dsx)+'px';ov.style.height=Math.abs(cy-dsy)+'px';});
document.addEventListener('mouseup',function(e){if(!dragging)return;dragging=false;
 ov.style.display='none';if(!moved){
 var ch=[];document.elementsFromPoint(e.clientX,e.clientY).forEach(function(el){if(el===tip||tip.contains(el))return;var fd=null;for(var n2=el;n2&amp;&amp;n2!==document.body;n2=n2.parentElement){if(fd===null&amp;&amp;n2.getAttribute&amp;&amp;n2.getAttribute('data-idx')!==null)fd=n2;if(n2===wrap){if(fd!==null){var idx=parseInt(fd.getAttribute('data-idx'),10);if(ch.indexOf(idx)===-1)ch.push(idx);}return;}}});
 if(ch.length){pinned=true;tipIdxs=ch;tipPos=0;lastE=e;renderTip();}else{pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}return;}
 var r=wrap.getBoundingClientRect();var cx=e.clientX-r.left,cy=e.clientY-r.top;
 var rx1=Math.min(dsx,cx),ry1=Math.min(dsy,cy),rx2=Math.max(dsx,cx),ry2=Math.max(dsy,cy);
 if(rx2-rx1<8&amp;&amp;ry2-ry1<8)return;
 var ctm=svg.getScreenCTM();if(!ctm)return;var inv=ctm.inverse();
 function toS(px,py){var pt=svg.createSVGPoint();pt.x=px+r.left;pt.y=py+r.top;return pt.matrixTransform(inv);}
 var p1=toS(rx1,ry1),p2=toS(rx2,ry2);
 var bx1=Math.min(p1.x,p2.x),by1=Math.min(p1.y,p2.y),bx2=Math.max(p1.x,p2.x),by2=Math.max(p1.y,p2.y);
 var pts=svg.querySelectorAll('[data-idx]');var sel=[],unsel=[];
 pts.forEach(function(el){try{var bb=el.getBBox();var ecx=bb.x+bb.width/2,ecy=bb.y+bb.height/2;
  if(ecx>=bx1&amp;&amp;ecx<=bx2&amp;&amp;ecy>=by1&amp;&amp;ecy<=by2)sel.push(el);else unsel.push(el);}catch(ex){unsel.push(el);}});
 if(!sel.length)return;
 sel.forEach(function(el){el.style.stroke='#6366F1';el.style.strokeWidth='2.5';el.style.opacity='';el.style.transform='';});
 unsel.forEach(function(el){
  el.style.transformBox='fill-box';el.style.transformOrigin='center';
  el.style.transition='transform 0.28s cubic-bezier(.4,0,.2,1),opacity 0.28s';
  el.style.transform='scale(0,0)';el.style.opacity='0';
  setTimeout(function(){el.style.display='none';el.style.transition='';},300);});

 var mnx=Infinity,mny=Infinity,mxx=-Infinity,mxy=-Infinity;
 sel.forEach(function(el){try{var bb=el.getBBox();
  mnx=Math.min(mnx,bb.x);mny=Math.min(mny,bb.y);
  mxx=Math.max(mxx,bb.x+bb.width);mxy=Math.max(mxy,bb.y+bb.height);}catch(ex){}});
 var vb=parseVB(origVB.length?origVB:(svg.getAttribute('viewBox')||'0 0 800 500'));
 var axL=mnx-vb[0],axT=mny-vb[1],axR=(vb[0]+vb[2])-mxx,axB=(vb[1]+vb[3])-mxy;
 var sw=mxx-mnx||1,sh=mxy-mny||1;
 var pL=Math.max(sw*0.12,axL>0?axL*0.7:sw*0.12);
 var pR=Math.max(sw*0.08,axR>0?axR*0.5:sw*0.08);
 var pT=Math.max(sh*0.12,axT>0?axT*0.7:sh*0.12);
 var pB=Math.max(sh*0.20,axB>0?axB*0.8:sh*0.20);
 animateVB(mnx-pL,mny-pT,(mxx+pR)-(mnx-pL),(mxy+pB)-(mny-pT));

 var xs=sel.map(function(el){return parseFloat(el.getAttribute('data-x'));}).filter(Number.isFinite);
 var ys=sel.map(function(el){return parseFloat(el.getAttribute('data-y'));}).filter(Number.isFinite);
 var s='<span style=&quot;color:#6366F1;font-weight:700&quot;>'+sel.length+' pts</span>';
 if(xs.length&amp;&amp;ys.length){
  var mx=xs.reduce(function(a,b){return a+b;})/xs.length;
  var my=ys.reduce(function(a,b){return a+b;})/ys.length;
  var vX=xs.map(function(v){return(v-mx)*(v-mx);}).reduce(function(a,b){return a+b;})/xs.length;
  var vY=ys.map(function(v){return(v-my)*(v-my);}).reduce(function(a,b){return a+b;})/ys.length;
  s+=' &amp;middot; X&amp;#772; <b>'+mx.toFixed(2)+'</b> &amp;plusmn;'+Math.sqrt(vX).toFixed(2);
  s+=' &amp;middot; Y&amp;#772; <b>'+my.toFixed(2)+'</b> &amp;plusmn;'+Math.sqrt(vY).toFixed(2);}
 s+=' <span class=&quot;sp-cls-x&quot;>&amp;#10005;</span>';
 panel.innerHTML=s;panel.style.display='block';
 panel.querySelector('.sp-cls-x').addEventListener('click',clearSel);});
function clearSel(){panel.style.display='none';resetVB();
 svg.querySelectorAll('[data-idx]').forEach(function(el){
  el.style.display='';el.style.opacity='';el.style.stroke='';el.style.strokeWidth='';el.style.filter='';
  el.style.transform='';el.style.transition='';el.style.transformBox='';el.style.transformOrigin='';});
 reAnim();}
document.addEventListener('keydown',function(e){if(e.key==='Escape'){if(dblZoomed){dblZoomed=false;resetVB();svg.querySelectorAll('[data-idx]').forEach(function(el){el.style.opacity='';el.style.display='';el.style.transform='';el.style.transition='';});reAnim();}if(pinned){pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}}});
var dblZoomed=false;
svg.addEventListener('dblclick',function(e){
 if(dblZoomed){dblZoomed=false;resetVB();svg.querySelectorAll('[data-idx]').forEach(function(el){el.style.opacity='';el.style.display='';el.style.transform='';el.style.transition='';el.style.filter='';});reAnim();return;}
 var found=null;
 for(var nd=e.target;nd&amp;&amp;nd!==svg;nd=nd.parentElement){if(nd.getAttribute&amp;&amp;nd.getAttribute('data-idx')!==null){found=nd;break;}}
 if(!found)return;
 e.stopPropagation();
 var idx=parseInt(found.getAttribute('data-idx'),10);
 var bb;try{bb=found.getBBox();}catch(ex){return;}
 var pad=Math.max(bb.width,bb.height)*1.5+30;
 svg.querySelectorAll('[data-idx]').forEach(function(el){
  var eli=parseInt(el.getAttribute('data-idx'),10);
  if(eli===idx){el.style.opacity='1';el.style.filter='drop-shadow(0 0 6px rgba(99,102,241,.45))';}else{el.style.display='none';}});
 var vb=parseVB(origVB.length?origVB:(svg.getAttribute('viewBox')||'0 0 800 500'));
 var nx=Math.max(vb[0],bb.x-pad),ny=Math.max(vb[1],bb.y-pad);
 var nw=Math.min(vb[2],bb.width+pad*2),nh=Math.min(vb[3],bb.height+pad*2);
 animateVB(nx,ny,nw,nh);
 dblZoomed=true;
},false);
var legs=svg.querySelectorAll('[data-legend]');
legs.forEach(function(lg){lg.style.cursor='pointer';
lg.addEventListener('click',function(){
 var s=lg.getAttribute('data-series');if(!s)return;
 var h=lg.getAttribute('data-hidden')==='1';
 var els=svg.querySelectorAll('[data-series=&quot;'+s+'&quot;]:not([data-legend])');
 if(h){els.forEach(function(el){el.style.display='';el.style.opacity='';el.style.transform='';el.style.transformBox='';el.style.transformOrigin='';el.style.transition='';});lg.style.opacity='';lg.removeAttribute('data-hidden');}
 else{els.forEach(function(el){
  el.style.transformBox='fill-box';el.style.transformOrigin='center';
  el.style.transition='transform 0.3s cubic-bezier(.4,0,.2,1),opacity 0.3s';
  el.style.transform='scale(0,0)';el.style.opacity='0';
  setTimeout(function(){el.style.display='none';el.style.transition='';},320);});
  lg.style.opacity='0.3';lg.setAttribute('data-hidden','1');}spRescale();});});
function spRescale(){
 var m=svg.getAttribute('data-sp');if(!m)return;
 var p=m.split(',').map(Number),pL=p[0],pT=p[1],pW=p[2],pH=p[3];
 var vals=[];
 svg.querySelectorAll('[data-pts]').forEach(function(el){if(el.style.display==='none')return;el.getAttribute('data-pts').split(',').forEach(function(v){var f=parseFloat(v);if(isFinite(f))vals.push(f);});});
 svg.querySelectorAll('circle[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=parseFloat(el.getAttribute('data-y'));if(isFinite(f))vals.push(f);});
 svg.querySelectorAll('rect[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=parseFloat(el.getAttribute('data-y'));if(isFinite(f))vals.push(f);});
 svg.querySelectorAll('rect[data-v]').forEach(function(el){if(el.style.display==='none')return;if(el.hasAttribute('data-y'))return;var f=parseFloat(el.getAttribute('data-v'));if(isFinite(f))vals.push(f);});
 if(!vals.length)return;
 var mn=Math.min.apply(null,vals),mx=Math.max.apply(null,vals);if(mn>0)mn=0;var rg=mx-mn;if(rg<1e-12)rg=1;
 svg.querySelectorAll('polyline[data-pts]').forEach(function(el){if(el.style.display==='none')return;var vs=el.getAttribute('data-pts').split(','),n=vs.length,sx=pW/Math.max(n-1,1),r='';for(var i=0;i<n;i++){var f=(parseFloat(vs[i])-mn)/rg;if(i>0)r+=' ';r+=(pL+Math.round(i*sx))+','+(pT+Math.round((1-f)*pH));}el.setAttribute('points',r);});
 svg.querySelectorAll('circle[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=(parseFloat(el.getAttribute('data-y'))-mn)/rg;el.setAttribute('cy',pT+Math.round((1-f)*pH));});
 svg.querySelectorAll('rect[data-y]').forEach(function(el){if(el.style.display==='none')return;var v=parseFloat(el.getAttribute('data-y'));if(!isFinite(v))return;var f=(v-mn)/rg;var ny=pT+Math.round((1-f)*pH);var base=pT+pH;if(mn<0){base=pT+Math.round((0-mn)/rg*pH);}if(v>=0){el.setAttribute('y',ny);el.setAttribute('height',Math.max(1,base-ny));}else{el.setAttribute('y',base);el.setAttribute('height',Math.max(1,ny-base));}});
 svg.querySelectorAll('rect[data-v]').forEach(function(el){if(el.style.display==='none')return;if(el.hasAttribute('data-y'))return;var v=parseFloat(el.getAttribute('data-v'));if(!isFinite(v))return;var f=(v-mn)/rg;var ny=pT+Math.round((1-f)*pH);var base=pT+pH;if(mn<0){base=pT+Math.round((0-mn)/rg*pH);}if(v>=0){el.setAttribute('y',ny);el.setAttribute('height',Math.max(1,base-ny));}else{el.setAttribute('y',base);el.setAttribute('height',Math.max(1,ny-base));}});
 var tks=svg.querySelectorAll('.sp-yt'),nT=tks.length;if(nT>1)for(var i=0;i<nT;i++){var f=i/(nT-1);var v=mn+f*rg;tks[i].textContent=v>=1000?Math.round(v)+'':v.toFixed(2);tks[i].setAttribute('y',pT+Math.round((1-f)*pH)+3);}
 var gls=svg.querySelectorAll('.sp-gl');if(gls.length&amp;&amp;nT>1)for(var j=0;j<gls.length;j++){var f=(j+1)/(nT-1);var gy=pT+Math.round((1-f)*pH);gls[j].setAttribute('y1',gy);gls[j].setAttribute('y2',gy);}
}
svg.querySelectorAll('[data-idx]').forEach(function(el,i){el.style.animationDelay=(i*18)+'ms';});
})();</script></div></body></html>" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117"></iframe>

</details>

---

## See also

- [Histogram Overlay](histogram-overlay.md) — compare two distributions
- [KDE](kde.md) — smooth density estimate
- [Violin](violin.md) — distribution by category
