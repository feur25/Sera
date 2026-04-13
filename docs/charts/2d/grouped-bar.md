
## Signature

```python
sp.build_grouped_bar(
    title: str,
    category_labels: list[str],
    series_values: list[float],
    *,
    show_values: bool = False,
    series_names: list[str] | None = None,
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

Grouped bar chart for comparing multiple series across categories.

`series_values` must be a **flat list** of length `n_categories × n_series`, row-major (category-first).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | Category names on X axis |
| `series_values` | `list[float]` | required | Flat values: `[cat0_s0, cat0_s1, cat1_s0, cat1_s1, ...]` |
| `show_values` | `bool` | `False` | Show value labels |
| `series_names` | `list[str] \| None` | `None` | Series names for legend |
| `palette` | `list[int] \| None` | `None` | Custom color palette |

---

## Returns

`Chart`

---

## Examples

```python
import seraplot as sp

categories = ["Q1", "Q2", "Q3", "Q4"]
values = [
    120.0, 90.0, 150.0,
    130.0, 110.0, 140.0,
    100.0, 95.0,  160.0,
    140.0, 120.0, 175.0,
]

logo = "https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png"
hover = sp.build_hover_json(categories * 3, images=[logo] * len(categories * 3))

chart = (
    sp.build_grouped_bar(
        "Quarterly Sales by Product",
        category_labels=categories,
        series_values=values,
        series_names=["Product A", "Product B", "Product C"],
        show_values=True,
        gridlines=True,
        legend_position="bottom",
        hover_json=hover,
    )
    .set_bg(None)
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe srcdoc="<!DOCTYPE html><html><head><meta charset=&quot;utf-8&quot;><title>Quarterly Sales by Product</title><style>#sp-tip{position:absolute;z-index:999999;pointer-events:none;opacity:0;transition:opacity .15s,transform .15s;transform:translateY(6px) scale(.97);background:#0b0e18;color:#f1f5f9;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:13px;border-radius:10px;min-width:160px;max-width:340px;box-shadow:0 4px 20px rgba(0,0,0,.45),0 0 0 1px rgba(255,255,255,.08);overflow:hidden}#sp-tip.sp-vis{opacity:1;transform:translateY(0) scale(1);pointer-events:auto}.sp-nav{display:flex;align-items:center;justify-content:space-between;padding:5px 10px;border-top:1px solid rgba(255,255,255,.08)}.sp-nav-btn{cursor:pointer;padding:0 10px;border-radius:5px;height:22px;line-height:22px;font-size:18px;background:rgba(255,255,255,.10);color:#e2e8f0;user-select:none;flex-shrink:0}.sp-nav-btn:hover{background:rgba(255,255,255,.22)}.sp-nav-dis{opacity:.25;pointer-events:none}.sp-nav-ctr{flex:1;text-align:center;font-size:11px;color:#94a3b8}.sp-head{padding:10px 14px 6px;font-weight:700;font-size:14px;color:#e2e8f0;border-bottom:1px solid rgba(255,255,255,.08)}.sp-body{padding:8px 14px 12px}.sp-row{display:flex;justify-content:space-between;align-items:baseline;gap:14px;padding:3px 0;border-bottom:1px solid rgba(255,255,255,.04)}.sp-row:last-child{border-bottom:none}.sp-key{color:#94a3b8;font-size:12px;white-space:nowrap}.sp-val{font-weight:600;font-size:12px;color:#f8fafc;text-align:right;word-break:break-all}#sp-tip img{display:block;width:100%;max-height:210px;object-fit:contain;border-top:1px solid rgba(255,255,255,.07)}#sp-tip video{display:block;width:100%;border-top:1px solid rgba(255,255,255,.07)}.sp-html{padding:8px 14px;font-size:12px;border-top:1px solid rgba(255,255,255,.07)}[data-idx]{cursor:pointer;transition:opacity .25s,filter .2s,transform .25s}[data-idx]:hover{filter:brightness(1.12) saturate(1.08)}.sp-cpanel{position:absolute;bottom:10px;left:50%;transform:translateX(-50%);background:#0b0e18;color:#f1f5f9;border-radius:10px;padding:8px 16px;font-size:12px;font-family:-apple-system,Arial,sans-serif;box-shadow:0 8px 24px rgba(0,0,0,.4);z-index:20;white-space:nowrap;display:none}.sp-cls-x{cursor:pointer;color:#94a3b8;margin-left:6px;font-size:13px}.sp-cls-x:hover{color:#f87171}</style><style>body{margin:0;background:#0d1117;display:flex;justify-content:center;padding:0}@keyframes sp-i{from{opacity:0;transform:translateY(8px) scale(.94)}}@keyframes sp-bar{from{opacity:0;transform:scaleY(0)}}@keyframes sp-pop{0%{opacity:0;transform:scale(0)}70%{transform:scale(1.08)}100%{opacity:1;transform:scale(1)}}@keyframes sp-arc{from{opacity:0;transform:scale(.82) rotate(-6deg)}}@keyframes sp-fn{from{opacity:0;transform:scaleX(.7) translateY(8px)}}svg rect[data-idx]{transform-box:fill-box;transform-origin:bottom center;animation:sp-bar .5s cubic-bezier(.22,.61,.36,1) both}svg circle[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-pop .42s cubic-bezier(.34,1.56,.64,1) both}svg path[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-arc .48s cubic-bezier(.22,.61,.36,1) both}svg polygon[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-fn .48s cubic-bezier(.22,.61,.36,1) both}svg line[data-idx]{animation:sp-i .45s cubic-bezier(.22,.61,.36,1) both}svg rect[data-idx]:hover{transform:scaleY(1.03);filter:brightness(1.12) saturate(1.1)}svg circle[data-idx]:hover{transform:scale(1.3);filter:brightness(1.15)}svg path[data-idx]:hover{filter:brightness(1.1) drop-shadow(0 2px 8px rgba(0,0,0,.2))}.sp-bg{fill:#fff}</style><style>.sp-bg{fill:transparent!important}.sp-ttl{fill:#e2e8f0!important}svg text{fill:#cbd5e1!important}.sp-ax-x,.sp-ax-y{stroke:#475569!important}.sp-gl{stroke:#2d3748!important}.sp-xl,.sp-yl{fill:#94a3b8!important}[id^='spp']{box-shadow:none!important;border-radius:0!important}</style></head><body><div id=&quot;spp1&quot; style=&quot;position:relative;display:inline-block;border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)&quot;><svg xmlns=&quot;http://www.w3.org/2000/svg&quot; width=&quot;1100&quot; height=&quot;480&quot; viewBox=&quot;0 0 1100 480&quot; data-sp=&quot;56,42,884,386&quot;><rect class=&quot;sp-bg&quot; width=&quot;100%&quot; height=&quot;100%&quot;/><text x=&quot;498&quot; y=&quot;26&quot; text-anchor=&quot;middle&quot; font-family=&quot;-apple-system,Arial,sans-serif&quot; font-size=&quot;15&quot; font-weight=&quot;700&quot; fill=&quot;#1a202c&quot; class=&quot;sp-ttl&quot;>Quarterly Sales by Product</text><text x=&quot;52&quot; y=&quot;432&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>0.00</text><line x1=&quot;56&quot; y1=&quot;363&quot; x2=&quot;940&quot; y2=&quot;363&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;52&quot; y=&quot;367&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>29.17</text><line x1=&quot;56&quot; y1=&quot;299&quot; x2=&quot;940&quot; y2=&quot;299&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;52&quot; y=&quot;303&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>58.33</text><line x1=&quot;56&quot; y1=&quot;235&quot; x2=&quot;940&quot; y2=&quot;235&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;52&quot; y=&quot;239&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>87.50</text><line x1=&quot;56&quot; y1=&quot;170&quot; x2=&quot;940&quot; y2=&quot;170&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;52&quot; y=&quot;174&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>116.67</text><line x1=&quot;56&quot; y1=&quot;106&quot; x2=&quot;940&quot; y2=&quot;106&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;52&quot; y=&quot;110&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>145.83</text><line x1=&quot;56&quot; y1=&quot;42&quot; x2=&quot;940&quot; y2=&quot;42&quot; stroke=&quot;#e2e8f0&quot; stroke-width=&quot;0.5&quot; class=&quot;sp-gl&quot;/><text x=&quot;52&quot; y=&quot;46&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>175.00</text><line x1=&quot;56&quot; y1=&quot;42&quot; x2=&quot;56&quot; y2=&quot;428&quot; stroke=&quot;#cbd5e1&quot; stroke-width=&quot;1&quot; class=&quot;sp-ax-y&quot;/><line x1=&quot;56&quot; y1=&quot;428&quot; x2=&quot;940&quot; y2=&quot;428&quot; stroke=&quot;#cbd5e1&quot; stroke-width=&quot;1&quot; class=&quot;sp-ax-x&quot;/><text x=&quot;166.50&quot; y=&quot;444&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#6b7280&quot;>Q1</text><rect data-idx=&quot;0&quot; data-series=&quot;0&quot; data-v=&quot;120.00&quot; data-lbl=&quot;Product A — Q1&quot; x=&quot;89.73&quot; y=&quot;163.31&quot; width=&quot;51.18&quot; height=&quot;264.69&quot; fill=&quot;#6366f1&quot; rx=&quot;3&quot; fill-opacity=&quot;0.88&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;115.32&quot; y=&quot;161.31&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;8&quot; fill=&quot;#1a202c&quot;>120.00</text><rect data-idx=&quot;1&quot; data-series=&quot;1&quot; data-v=&quot;110.00&quot; data-lbl=&quot;Product B — Q1&quot; x=&quot;140.91&quot; y=&quot;185.37&quot; width=&quot;51.18&quot; height=&quot;242.63&quot; fill=&quot;#f43f5e&quot; rx=&quot;3&quot; fill-opacity=&quot;0.88&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;166.50&quot; y=&quot;183.37&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;8&quot; fill=&quot;#1a202c&quot;>110.00</text><rect data-idx=&quot;2&quot; data-series=&quot;2&quot; data-v=&quot;160.00&quot; data-lbl=&quot;Product C — Q1&quot; x=&quot;192.09&quot; y=&quot;75.09&quot; width=&quot;51.18&quot; height=&quot;352.91&quot; fill=&quot;#10b981&quot; rx=&quot;3&quot; fill-opacity=&quot;0.88&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;217.68&quot; y=&quot;73.09&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;8&quot; fill=&quot;#1a202c&quot;>160.00</text><text x=&quot;387.50&quot; y=&quot;444&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#6b7280&quot;>Q2</text><rect data-idx=&quot;3&quot; data-series=&quot;0&quot; data-v=&quot;90.00&quot; data-lbl=&quot;Product A — Q2&quot; x=&quot;310.73&quot; y=&quot;229.49&quot; width=&quot;51.18&quot; height=&quot;198.51&quot; fill=&quot;#6366f1&quot; rx=&quot;3&quot; fill-opacity=&quot;0.88&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;336.32&quot; y=&quot;227.49&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;8&quot; fill=&quot;#1a202c&quot;>90.00</text><rect data-idx=&quot;4&quot; data-series=&quot;1&quot; data-v=&quot;140.00&quot; data-lbl=&quot;Product B — Q2&quot; x=&quot;361.91&quot; y=&quot;119.20&quot; width=&quot;51.18&quot; height=&quot;308.80&quot; fill=&quot;#f43f5e&quot; rx=&quot;3&quot; fill-opacity=&quot;0.88&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;387.50&quot; y=&quot;117.20&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;8&quot; fill=&quot;#1a202c&quot;>140.00</text><rect data-idx=&quot;5&quot; data-series=&quot;2&quot; data-v=&quot;140.00&quot; data-lbl=&quot;Product C — Q2&quot; x=&quot;413.09&quot; y=&quot;119.20&quot; width=&quot;51.18&quot; height=&quot;308.80&quot; fill=&quot;#10b981&quot; rx=&quot;3&quot; fill-opacity=&quot;0.88&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;438.68&quot; y=&quot;117.20&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;8&quot; fill=&quot;#1a202c&quot;>140.00</text><text x=&quot;608.50&quot; y=&quot;444&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#6b7280&quot;>Q3</text><rect data-idx=&quot;6&quot; data-series=&quot;0&quot; data-v=&quot;150.00&quot; data-lbl=&quot;Product A — Q3&quot; x=&quot;531.73&quot; y=&quot;97.14&quot; width=&quot;51.18&quot; height=&quot;330.86&quot; fill=&quot;#6366f1&quot; rx=&quot;3&quot; fill-opacity=&quot;0.88&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;557.32&quot; y=&quot;95.14&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;8&quot; fill=&quot;#1a202c&quot;>150.00</text><rect data-idx=&quot;7&quot; data-series=&quot;1&quot; data-v=&quot;100.00&quot; data-lbl=&quot;Product B — Q3&quot; x=&quot;582.91&quot; y=&quot;207.43&quot; width=&quot;51.18&quot; height=&quot;220.57&quot; fill=&quot;#f43f5e&quot; rx=&quot;3&quot; fill-opacity=&quot;0.88&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;608.50&quot; y=&quot;205.43&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;8&quot; fill=&quot;#1a202c&quot;>100.00</text><rect data-idx=&quot;8&quot; data-series=&quot;2&quot; data-v=&quot;120.00&quot; data-lbl=&quot;Product C — Q3&quot; x=&quot;634.09&quot; y=&quot;163.31&quot; width=&quot;51.18&quot; height=&quot;264.69&quot; fill=&quot;#10b981&quot; rx=&quot;3&quot; fill-opacity=&quot;0.88&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;659.68&quot; y=&quot;161.31&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;8&quot; fill=&quot;#1a202c&quot;>120.00</text><text x=&quot;829.50&quot; y=&quot;444&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#6b7280&quot;>Q4</text><rect data-idx=&quot;9&quot; data-series=&quot;0&quot; data-v=&quot;130.00&quot; data-lbl=&quot;Product A — Q4&quot; x=&quot;752.73&quot; y=&quot;141.26&quot; width=&quot;51.18&quot; height=&quot;286.74&quot; fill=&quot;#6366f1&quot; rx=&quot;3&quot; fill-opacity=&quot;0.88&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;778.32&quot; y=&quot;139.26&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;8&quot; fill=&quot;#1a202c&quot;>130.00</text><rect data-idx=&quot;10&quot; data-series=&quot;1&quot; data-v=&quot;95.00&quot; data-lbl=&quot;Product B — Q4&quot; x=&quot;803.91&quot; y=&quot;218.46&quot; width=&quot;51.18&quot; height=&quot;209.54&quot; fill=&quot;#f43f5e&quot; rx=&quot;3&quot; fill-opacity=&quot;0.88&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;829.50&quot; y=&quot;216.46&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;8&quot; fill=&quot;#1a202c&quot;>95.00</text><rect data-idx=&quot;11&quot; data-series=&quot;2&quot; data-v=&quot;175.00&quot; data-lbl=&quot;Product C — Q4&quot; x=&quot;855.09&quot; y=&quot;42.00&quot; width=&quot;51.18&quot; height=&quot;386.00&quot; fill=&quot;#10b981&quot; rx=&quot;3&quot; fill-opacity=&quot;0.88&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;880.68&quot; y=&quot;40.00&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;8&quot; fill=&quot;#1a202c&quot;>175.00</text><g data-legend=&quot;1&quot; data-series=&quot;0&quot;><rect x=&quot;952&quot; y=&quot;50&quot; width=&quot;12&quot; height=&quot;12&quot; rx=&quot;2&quot; fill=&quot;#6366f1&quot;/><text x=&quot;968&quot; y=&quot;60&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;10&quot; fill=&quot;#374151&quot;>Product A</text></g><g data-legend=&quot;1&quot; data-series=&quot;1&quot;><rect x=&quot;952&quot; y=&quot;72&quot; width=&quot;12&quot; height=&quot;12&quot; rx=&quot;2&quot; fill=&quot;#f43f5e&quot;/><text x=&quot;968&quot; y=&quot;82&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;10&quot; fill=&quot;#374151&quot;>Product B</text></g><g data-legend=&quot;1&quot; data-series=&quot;2&quot;><rect x=&quot;952&quot; y=&quot;94&quot; width=&quot;12&quot; height=&quot;12&quot; rx=&quot;2&quot; fill=&quot;#10b981&quot;/><text x=&quot;968&quot; y=&quot;104&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;10&quot; fill=&quot;#374151&quot;>Product C</text></g></svg><div class=&quot;sp-sel-ov&quot; style=&quot;display:none&quot;></div><div class=&quot;sp-cpanel&quot;></div><script>(function(){
var wrap=document.getElementById('spp1');if(!wrap)return;wrap.removeAttribute('id');
var svg=wrap.querySelector('svg');var data=[{&quot;title&quot;:&quot;Q1&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;Q2&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;Q3&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;Q4&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;Q1&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;Q2&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;Q3&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;Q4&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;Q1&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;Q2&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;Q3&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;Q4&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;}];

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

- [Stacked Bar](stacked-bar.md) — `sp.build_stacked_bar()`
- [Bar Chart](bar.md)
