
## Signature

```python
sp.build_histogram_overlay(
    title: str,
    values: list[float],
    overlay_values: list[float],
    *,
    color_hex: int = 0,
    overlay_color_hex: int = 0,
    bins: int = 20,
    series_names: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Overlaid histogram comparing two distributions side-by-side with transparency.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `values` | `list[float]` | required | First distribution data |
| `overlay_values` | `list[float]` | required | Second distribution data |
| `bins` | `int` | `20` | Number of bins |
| `series_names` | `list[str] \| None` | `None` | Names for legend `["Series A", "Series B"]` |
| `color_hex` | `int` | `0` | Color for first series |
| `overlay_color_hex` | `int` | `0` | Color for second series |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp
import numpy as np

rng = np.random.default_rng(42)
control = rng.normal(5.0, 1.0, 1000).tolist()
treatment = rng.normal(5.8, 1.2, 1000).tolist()

chart = sp.build_histogram_overlay(
    "Control vs Treatment",
    values=control,
    overlay_values=treatment,
    bins=30,
    series_names=["Control", "Treatment"],
    x_label="Measurement",
    y_label="Frequency",
    gridlines=True,
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe srcdoc="<!DOCTYPE html><html><head><meta charset=&quot;utf-8&quot;><title>Control vs Treatment</title><style>#sp-tip{position:absolute;z-index:999999;pointer-events:none;opacity:0;transition:opacity .15s,transform .15s;transform:translateY(6px) scale(.97);background:#0b0e18;color:#f1f5f9;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:13px;border-radius:10px;min-width:160px;max-width:340px;box-shadow:0 4px 20px rgba(0,0,0,.45),0 0 0 1px rgba(255,255,255,.08);overflow:hidden}#sp-tip.sp-vis{opacity:1;transform:translateY(0) scale(1);pointer-events:auto}.sp-nav{display:flex;align-items:center;justify-content:space-between;padding:5px 10px;border-top:1px solid rgba(255,255,255,.08)}.sp-nav-btn{cursor:pointer;padding:0 10px;border-radius:5px;height:22px;line-height:22px;font-size:18px;background:rgba(255,255,255,.10);color:#e2e8f0;user-select:none;flex-shrink:0}.sp-nav-btn:hover{background:rgba(255,255,255,.22)}.sp-nav-dis{opacity:.25;pointer-events:none}.sp-nav-ctr{flex:1;text-align:center;font-size:11px;color:#94a3b8}.sp-head{padding:10px 14px 6px;font-weight:700;font-size:14px;color:#e2e8f0;border-bottom:1px solid rgba(255,255,255,.08)}.sp-body{padding:8px 14px 12px}.sp-row{display:flex;justify-content:space-between;align-items:baseline;gap:14px;padding:3px 0;border-bottom:1px solid rgba(255,255,255,.04)}.sp-row:last-child{border-bottom:none}.sp-key{color:#94a3b8;font-size:12px;white-space:nowrap}.sp-val{font-weight:600;font-size:12px;color:#f8fafc;text-align:right;word-break:break-all}#sp-tip img{display:block;width:100%;max-height:210px;object-fit:contain;border-top:1px solid rgba(255,255,255,.07)}#sp-tip video{display:block;width:100%;border-top:1px solid rgba(255,255,255,.07)}.sp-html{padding:8px 14px;font-size:12px;border-top:1px solid rgba(255,255,255,.07)}[data-idx]{cursor:pointer;transition:opacity .25s,filter .2s,transform .25s}[data-idx]:hover{filter:brightness(1.12) saturate(1.08)}.sp-cpanel{position:absolute;bottom:10px;left:50%;transform:translateX(-50%);background:#0b0e18;color:#f1f5f9;border-radius:10px;padding:8px 16px;font-size:12px;font-family:-apple-system,Arial,sans-serif;box-shadow:0 8px 24px rgba(0,0,0,.4);z-index:20;white-space:nowrap;display:none}.sp-cls-x{cursor:pointer;color:#94a3b8;margin-left:6px;font-size:13px}.sp-cls-x:hover{color:#f87171}</style><style>body{margin:0;background:#0d1117;display:flex;justify-content:center;padding:0}@keyframes sp-i{from{opacity:0;transform:translateY(8px) scale(.94)}}@keyframes sp-bar{from{opacity:0;transform:scaleY(0)}}@keyframes sp-pop{0%{opacity:0;transform:scale(0)}70%{transform:scale(1.08)}100%{opacity:1;transform:scale(1)}}@keyframes sp-arc{from{opacity:0;transform:scale(.82) rotate(-6deg)}}@keyframes sp-fn{from{opacity:0;transform:scaleX(.7) translateY(8px)}}svg rect[data-idx]{transform-box:fill-box;transform-origin:bottom center;animation:sp-bar .5s cubic-bezier(.22,.61,.36,1) both}svg circle[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-pop .42s cubic-bezier(.34,1.56,.64,1) both}svg path[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-arc .48s cubic-bezier(.22,.61,.36,1) both}svg polygon[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-fn .48s cubic-bezier(.22,.61,.36,1) both}svg line[data-idx]{animation:sp-i .45s cubic-bezier(.22,.61,.36,1) both}svg rect[data-idx]:hover{transform:scaleY(1.03);filter:brightness(1.12) saturate(1.1)}svg circle[data-idx]:hover{transform:scale(1.3);filter:brightness(1.15)}svg path[data-idx]:hover{filter:brightness(1.1) drop-shadow(0 2px 8px rgba(0,0,0,.2))}.sp-bg{fill:#fff}</style><style>.sp-bg{fill:transparent!important}.sp-ttl{fill:#e2e8f0!important}svg text{fill:#cbd5e1!important}.sp-ax-x,.sp-ax-y{stroke:#475569!important}.sp-gl{stroke:#2d3748!important}.sp-xl,.sp-yl{fill:#94a3b8!important}[id^='spp']{box-shadow:none!important;border-radius:0!important}</style></head><body><div id=&quot;spp1&quot; style=&quot;position:relative;display:inline-block;border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)&quot;><svg xmlns=&quot;http://www.w3.org/2000/svg&quot; width=&quot;860&quot; height=&quot;380&quot; viewBox=&quot;0 0 860 380&quot;><rect class=&quot;sp-bg&quot; width=&quot;100%&quot; height=&quot;100%&quot;/><text x=&quot;446&quot; y=&quot;26&quot; text-anchor=&quot;middle&quot; font-family=&quot;-apple-system,Arial,sans-serif&quot; font-size=&quot;15&quot; font-weight=&quot;700&quot; fill=&quot;#1a202c&quot; class=&quot;sp-ttl&quot;>Control vs Treatment</text><text x=&quot;48&quot; y=&quot;337&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>0</text><line x1=&quot;52&quot; y1=&quot;274&quot; x2=&quot;840&quot; y2=&quot;274&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;48&quot; y=&quot;277&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>24</text><line x1=&quot;52&quot; y1=&quot;214&quot; x2=&quot;840&quot; y2=&quot;214&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;48&quot; y=&quot;217&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>48</text><line x1=&quot;52&quot; y1=&quot;155&quot; x2=&quot;840&quot; y2=&quot;155&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;48&quot; y=&quot;158&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>71</text><line x1=&quot;52&quot; y1=&quot;95&quot; x2=&quot;840&quot; y2=&quot;95&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;48&quot; y=&quot;98&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>95</text><line x1=&quot;52&quot; y1=&quot;36&quot; x2=&quot;840&quot; y2=&quot;36&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;48&quot; y=&quot;39&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>119</text><line x1=&quot;52&quot; y1=&quot;36&quot; x2=&quot;52&quot; y2=&quot;334&quot; stroke=&quot;#cbd5e1&quot; stroke-width=&quot;1&quot; class=&quot;sp-ax-y&quot;/><line x1=&quot;52&quot; y1=&quot;334&quot; x2=&quot;840&quot; y2=&quot;334&quot; stroke=&quot;#cbd5e1&quot; stroke-width=&quot;1&quot; class=&quot;sp-ax-x&quot;/><text x=&quot;446&quot; y=&quot;376&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;10&quot; fill=&quot;#6b7280&quot; class=&quot;sp-xl&quot;>Measurement</text><text x=&quot;14&quot; y=&quot;185&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;10&quot; fill=&quot;#6b7280&quot; transform=&quot;rotate(-90,14,185)&quot; class=&quot;sp-yl&quot;>Frequency</text><rect data-idx=&quot;0&quot; data-series=&quot;0&quot; data-lbl=&quot;1.35–1.58&quot; data-kv-Count=&quot;1&quot; data-kv-Min=&quot;1.35&quot; data-kv-Max=&quot;1.58&quot; x=&quot;52&quot; y=&quot;332&quot; width=&quot;25&quot; height=&quot;2&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;1&quot; data-series=&quot;0&quot; data-lbl=&quot;1.58–1.81&quot; data-kv-Count=&quot;0&quot; data-kv-Min=&quot;1.58&quot; data-kv-Max=&quot;1.81&quot; x=&quot;78&quot; y=&quot;334&quot; width=&quot;25&quot; height=&quot;0&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;2&quot; data-series=&quot;0&quot; data-lbl=&quot;1.81–2.03&quot; data-kv-Count=&quot;0&quot; data-kv-Min=&quot;1.81&quot; data-kv-Max=&quot;2.03&quot; x=&quot;104&quot; y=&quot;334&quot; width=&quot;25&quot; height=&quot;0&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;3&quot; data-series=&quot;0&quot; data-lbl=&quot;2.03–2.26&quot; data-kv-Count=&quot;2&quot; data-kv-Min=&quot;2.03&quot; data-kv-Max=&quot;2.26&quot; x=&quot;130&quot; y=&quot;329&quot; width=&quot;25&quot; height=&quot;5&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;4&quot; data-series=&quot;0&quot; data-lbl=&quot;2.26–2.49&quot; data-kv-Count=&quot;3&quot; data-kv-Min=&quot;2.26&quot; data-kv-Max=&quot;2.49&quot; x=&quot;157&quot; y=&quot;327&quot; width=&quot;25&quot; height=&quot;7&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;5&quot; data-series=&quot;0&quot; data-lbl=&quot;2.49–2.72&quot; data-kv-Count=&quot;8&quot; data-kv-Min=&quot;2.49&quot; data-kv-Max=&quot;2.72&quot; x=&quot;183&quot; y=&quot;314&quot; width=&quot;25&quot; height=&quot;20&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;6&quot; data-series=&quot;0&quot; data-lbl=&quot;2.72–2.94&quot; data-kv-Count=&quot;8&quot; data-kv-Min=&quot;2.72&quot; data-kv-Max=&quot;2.94&quot; x=&quot;209&quot; y=&quot;314&quot; width=&quot;25&quot; height=&quot;20&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;7&quot; data-series=&quot;0&quot; data-lbl=&quot;2.94–3.17&quot; data-kv-Count=&quot;14&quot; data-kv-Min=&quot;2.94&quot; data-kv-Max=&quot;3.17&quot; x=&quot;235&quot; y=&quot;299&quot; width=&quot;25&quot; height=&quot;35&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;8&quot; data-series=&quot;0&quot; data-lbl=&quot;3.17–3.40&quot; data-kv-Count=&quot;22&quot; data-kv-Min=&quot;3.17&quot; data-kv-Max=&quot;3.40&quot; x=&quot;262&quot; y=&quot;279&quot; width=&quot;25&quot; height=&quot;55&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;9&quot; data-series=&quot;0&quot; data-lbl=&quot;3.40–3.63&quot; data-kv-Count=&quot;32&quot; data-kv-Min=&quot;3.40&quot; data-kv-Max=&quot;3.63&quot; x=&quot;288&quot; y=&quot;254&quot; width=&quot;25&quot; height=&quot;80&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;10&quot; data-series=&quot;0&quot; data-lbl=&quot;3.63–3.85&quot; data-kv-Count=&quot;38&quot; data-kv-Min=&quot;3.63&quot; data-kv-Max=&quot;3.85&quot; x=&quot;314&quot; y=&quot;239&quot; width=&quot;25&quot; height=&quot;95&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;11&quot; data-series=&quot;0&quot; data-lbl=&quot;3.85–4.08&quot; data-kv-Count=&quot;61&quot; data-kv-Min=&quot;3.85&quot; data-kv-Max=&quot;4.08&quot; x=&quot;340&quot; y=&quot;182&quot; width=&quot;25&quot; height=&quot;152&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;12&quot; data-series=&quot;0&quot; data-lbl=&quot;4.08–4.31&quot; data-kv-Count=&quot;63&quot; data-kv-Min=&quot;4.08&quot; data-kv-Max=&quot;4.31&quot; x=&quot;367&quot; y=&quot;177&quot; width=&quot;25&quot; height=&quot;157&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;13&quot; data-series=&quot;0&quot; data-lbl=&quot;4.31–4.54&quot; data-kv-Count=&quot;59&quot; data-kv-Min=&quot;4.31&quot; data-kv-Max=&quot;4.54&quot; x=&quot;393&quot; y=&quot;187&quot; width=&quot;25&quot; height=&quot;147&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;14&quot; data-series=&quot;0&quot; data-lbl=&quot;4.54–4.77&quot; data-kv-Count=&quot;96&quot; data-kv-Min=&quot;4.54&quot; data-kv-Max=&quot;4.77&quot; x=&quot;419&quot; y=&quot;94&quot; width=&quot;25&quot; height=&quot;240&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;15&quot; data-series=&quot;0&quot; data-lbl=&quot;4.77–4.99&quot; data-kv-Count=&quot;89&quot; data-kv-Min=&quot;4.77&quot; data-kv-Max=&quot;4.99&quot; x=&quot;446&quot; y=&quot;112&quot; width=&quot;25&quot; height=&quot;222&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;16&quot; data-series=&quot;0&quot; data-lbl=&quot;4.99–5.22&quot; data-kv-Count=&quot;88&quot; data-kv-Min=&quot;4.99&quot; data-kv-Max=&quot;5.22&quot; x=&quot;472&quot; y=&quot;114&quot; width=&quot;25&quot; height=&quot;220&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;17&quot; data-series=&quot;0&quot; data-lbl=&quot;5.22–5.45&quot; data-kv-Count=&quot;119&quot; data-kv-Min=&quot;5.22&quot; data-kv-Max=&quot;5.45&quot; x=&quot;498&quot; y=&quot;36&quot; width=&quot;25&quot; height=&quot;298&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;18&quot; data-series=&quot;0&quot; data-lbl=&quot;5.45–5.68&quot; data-kv-Count=&quot;72&quot; data-kv-Min=&quot;5.45&quot; data-kv-Max=&quot;5.68&quot; x=&quot;524&quot; y=&quot;154&quot; width=&quot;25&quot; height=&quot;180&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;19&quot; data-series=&quot;0&quot; data-lbl=&quot;5.68–5.90&quot; data-kv-Count=&quot;65&quot; data-kv-Min=&quot;5.68&quot; data-kv-Max=&quot;5.90&quot; x=&quot;551&quot; y=&quot;172&quot; width=&quot;25&quot; height=&quot;162&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;20&quot; data-series=&quot;0&quot; data-lbl=&quot;5.90–6.13&quot; data-kv-Count=&quot;42&quot; data-kv-Min=&quot;5.90&quot; data-kv-Max=&quot;6.13&quot; x=&quot;577&quot; y=&quot;229&quot; width=&quot;25&quot; height=&quot;105&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;21&quot; data-series=&quot;0&quot; data-lbl=&quot;6.13–6.36&quot; data-kv-Count=&quot;33&quot; data-kv-Min=&quot;6.13&quot; data-kv-Max=&quot;6.36&quot; x=&quot;603&quot; y=&quot;252&quot; width=&quot;25&quot; height=&quot;82&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;22&quot; data-series=&quot;0&quot; data-lbl=&quot;6.36–6.59&quot; data-kv-Count=&quot;29&quot; data-kv-Min=&quot;6.36&quot; data-kv-Max=&quot;6.59&quot; x=&quot;629&quot; y=&quot;262&quot; width=&quot;25&quot; height=&quot;72&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;23&quot; data-series=&quot;0&quot; data-lbl=&quot;6.59–6.81&quot; data-kv-Count=&quot;25&quot; data-kv-Min=&quot;6.59&quot; data-kv-Max=&quot;6.81&quot; x=&quot;656&quot; y=&quot;272&quot; width=&quot;25&quot; height=&quot;62&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;24&quot; data-series=&quot;0&quot; data-lbl=&quot;6.81–7.04&quot; data-kv-Count=&quot;13&quot; data-kv-Min=&quot;6.81&quot; data-kv-Max=&quot;7.04&quot; x=&quot;682&quot; y=&quot;302&quot; width=&quot;25&quot; height=&quot;32&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;25&quot; data-series=&quot;0&quot; data-lbl=&quot;7.04–7.27&quot; data-kv-Count=&quot;9&quot; data-kv-Min=&quot;7.04&quot; data-kv-Max=&quot;7.27&quot; x=&quot;708&quot; y=&quot;312&quot; width=&quot;25&quot; height=&quot;22&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;26&quot; data-series=&quot;0&quot; data-lbl=&quot;7.27–7.50&quot; data-kv-Count=&quot;3&quot; data-kv-Min=&quot;7.27&quot; data-kv-Max=&quot;7.50&quot; x=&quot;734&quot; y=&quot;327&quot; width=&quot;25&quot; height=&quot;7&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;27&quot; data-series=&quot;0&quot; data-lbl=&quot;7.50–7.72&quot; data-kv-Count=&quot;3&quot; data-kv-Min=&quot;7.50&quot; data-kv-Max=&quot;7.72&quot; x=&quot;761&quot; y=&quot;327&quot; width=&quot;25&quot; height=&quot;7&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;28&quot; data-series=&quot;0&quot; data-lbl=&quot;7.72–7.95&quot; data-kv-Count=&quot;2&quot; data-kv-Min=&quot;7.72&quot; data-kv-Max=&quot;7.95&quot; x=&quot;787&quot; y=&quot;329&quot; width=&quot;25&quot; height=&quot;5&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;29&quot; data-series=&quot;0&quot; data-lbl=&quot;7.95–8.18&quot; data-kv-Count=&quot;1&quot; data-kv-Min=&quot;7.95&quot; data-kv-Max=&quot;8.18&quot; x=&quot;813&quot; y=&quot;332&quot; width=&quot;25&quot; height=&quot;2&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;33&quot; data-series=&quot;1&quot; x=&quot;130&quot; y=&quot;329&quot; width=&quot;25&quot; height=&quot;5&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;34&quot; data-series=&quot;1&quot; x=&quot;157&quot; y=&quot;327&quot; width=&quot;25&quot; height=&quot;7&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;35&quot; data-series=&quot;1&quot; x=&quot;183&quot; y=&quot;329&quot; width=&quot;25&quot; height=&quot;5&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;36&quot; data-series=&quot;1&quot; x=&quot;209&quot; y=&quot;324&quot; width=&quot;25&quot; height=&quot;10&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;37&quot; data-series=&quot;1&quot; x=&quot;235&quot; y=&quot;314&quot; width=&quot;25&quot; height=&quot;20&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;38&quot; data-series=&quot;1&quot; x=&quot;262&quot; y=&quot;297&quot; width=&quot;25&quot; height=&quot;37&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;39&quot; data-series=&quot;1&quot; x=&quot;288&quot; y=&quot;302&quot; width=&quot;25&quot; height=&quot;32&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;40&quot; data-series=&quot;1&quot; x=&quot;314&quot; y=&quot;287&quot; width=&quot;25&quot; height=&quot;47&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;41&quot; data-series=&quot;1&quot; x=&quot;340&quot; y=&quot;262&quot; width=&quot;25&quot; height=&quot;72&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;42&quot; data-series=&quot;1&quot; x=&quot;367&quot; y=&quot;239&quot; width=&quot;25&quot; height=&quot;95&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;43&quot; data-series=&quot;1&quot; x=&quot;393&quot; y=&quot;219&quot; width=&quot;25&quot; height=&quot;115&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;44&quot; data-series=&quot;1&quot; x=&quot;419&quot; y=&quot;237&quot; width=&quot;25&quot; height=&quot;97&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;45&quot; data-series=&quot;1&quot; x=&quot;446&quot; y=&quot;204&quot; width=&quot;25&quot; height=&quot;130&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;46&quot; data-series=&quot;1&quot; x=&quot;472&quot; y=&quot;159&quot; width=&quot;25&quot; height=&quot;175&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;47&quot; data-series=&quot;1&quot; x=&quot;498&quot; y=&quot;179&quot; width=&quot;25&quot; height=&quot;155&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;48&quot; data-series=&quot;1&quot; x=&quot;524&quot; y=&quot;132&quot; width=&quot;25&quot; height=&quot;202&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;49&quot; data-series=&quot;1&quot; x=&quot;551&quot; y=&quot;147&quot; width=&quot;25&quot; height=&quot;187&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;50&quot; data-series=&quot;1&quot; x=&quot;577&quot; y=&quot;144&quot; width=&quot;25&quot; height=&quot;190&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;51&quot; data-series=&quot;1&quot; x=&quot;603&quot; y=&quot;149&quot; width=&quot;25&quot; height=&quot;185&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;52&quot; data-series=&quot;1&quot; x=&quot;629&quot; y=&quot;177&quot; width=&quot;25&quot; height=&quot;157&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;53&quot; data-series=&quot;1&quot; x=&quot;656&quot; y=&quot;204&quot; width=&quot;25&quot; height=&quot;130&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;54&quot; data-series=&quot;1&quot; x=&quot;682&quot; y=&quot;217&quot; width=&quot;25&quot; height=&quot;117&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;55&quot; data-series=&quot;1&quot; x=&quot;708&quot; y=&quot;254&quot; width=&quot;25&quot; height=&quot;80&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;56&quot; data-series=&quot;1&quot; x=&quot;734&quot; y=&quot;269&quot; width=&quot;25&quot; height=&quot;65&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;57&quot; data-series=&quot;1&quot; x=&quot;761&quot; y=&quot;262&quot; width=&quot;25&quot; height=&quot;72&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;58&quot; data-series=&quot;1&quot; x=&quot;787&quot; y=&quot;307&quot; width=&quot;25&quot; height=&quot;27&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;59&quot; data-series=&quot;1&quot; x=&quot;813&quot; y=&quot;254&quot; width=&quot;25&quot; height=&quot;80&quot; fill=&quot;#f43f5e&quot; fill-opacity=&quot;0.60&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;52&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>1.35</text><text x=&quot;157&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>2.26</text><text x=&quot;262&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>3.17</text><text x=&quot;367&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>4.08</text><text x=&quot;472&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>4.99</text><text x=&quot;577&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>5.90</text><text x=&quot;682&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>6.81</text><text x=&quot;787&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>7.72</text><g data-legend=&quot;1&quot; data-series=&quot;0&quot;><rect x=&quot;730&quot; y=&quot;41&quot; width=&quot;12&quot; height=&quot;12&quot; rx=&quot;2&quot; fill=&quot;#6366f1&quot;/><text x=&quot;746&quot; y=&quot;51&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;10&quot; fill=&quot;#374151&quot;>Control</text></g><g data-legend=&quot;1&quot; data-series=&quot;1&quot;><rect x=&quot;730&quot; y=&quot;58&quot; width=&quot;12&quot; height=&quot;12&quot; rx=&quot;2&quot; fill=&quot;#f43f5e&quot;/><text x=&quot;746&quot; y=&quot;68&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;10&quot; fill=&quot;#374151&quot;>Treatment</text></g></svg><div class=&quot;sp-sel-ov&quot; style=&quot;display:none&quot;></div><div class=&quot;sp-cpanel&quot;></div><script>(function(){
var wrap=document.getElementById('spp1');if(!wrap)return;wrap.removeAttribute('id');
var svg=wrap.querySelector('svg');var data=[{&quot;title&quot;:&quot;1.35–1.58&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;1.58–1.81&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;1.81–2.03&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;2.03–2.26&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;2.26–2.49&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;2.49–2.72&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;2.72–2.94&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;2.94–3.17&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;3.17–3.40&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;3.40–3.63&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;3.63–3.85&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;3.85–4.08&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;4.08–4.31&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;4.31–4.54&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;4.54–4.77&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;4.77–4.99&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;4.99–5.22&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;5.22–5.45&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;5.45–5.68&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;5.68–5.90&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;5.90–6.13&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;6.13–6.36&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;6.36–6.59&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;6.59–6.81&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;6.81–7.04&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;7.04–7.27&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;7.27–7.50&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;7.50–7.72&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;7.72–7.95&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;7.95–8.18&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;31&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;32&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;33&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;34&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;35&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;36&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;37&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;38&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;39&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;40&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;41&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;42&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;43&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;44&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;45&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;46&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;47&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;48&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;49&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;50&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;51&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;52&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;53&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;54&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;55&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;56&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;57&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;58&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;59&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;60&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;}];

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

- [Histogram](histogram.md)
- [KDE](kde.md)
