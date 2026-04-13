# Ridgeline Chart

## Signature

```python
sp.build_ridgeline_chart(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    bandwidth: float = 1.0,
    overlap: float = 0.5,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    background: str | None = None,
    gridlines: bool = False,
) -> Chart
```

---

## Description

Ridgeline (joy) chart — stacked KDE curves per category.
Excellent for comparing distributional shapes across many groups.

Excellent pour comparer les formes de distribution entre de nombreux groupes.

`values` is a flat list. The number of values must be divisible by `len(categories)` (equal samples per group).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `categories` | `list[str]` | required | Category labels (one ridge each) |
| `values` | `list[float]` | required | Flat concatenated sample data |
| `bandwidth` | `float` | `1.0` | KDE bandwidth factor |
| `overlap` | `float` | `0.5` | Ridge overlap (0 = no overlap, 1 = full overlap) |
| `color_hex` | `int` | `0x6366F1` | Single fill color |
| `palette` | `list[int] \| None` | `None` | Per-ridge colors |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `x_label` | `str` | `""` | X-axis label |
| `gridlines` | `bool` | `False` | Vertical gridlines |

---

## Returns

`Chart`

---

## Examples

### Daily temperature ridgeline

```python
import seraplot as sp
import random

months = ["Jan", "Apr", "Jul", "Oct"]
means  = [5, 15, 28, 16]

values = [v for m in means for v in [random.gauss(m, 4) for _ in range(100)]]

chart = sp.build_ridgeline_chart(
    "Monthly Temperature Distribution",
    categories=months,
    values=values,
    overlap=0.6,
    x_label="°C",
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<div style="width:100%;overflow:auto;border-radius:8px;margin:12px 0;background:#0d1117">
<!DOCTYPE html><html><head><meta charset="utf-8"><title>Monthly Temperature Distribution</title><style>#sp-tip{position:absolute;z-index:999999;pointer-events:none;opacity:0;transition:opacity .15s,transform .15s;transform:translateY(6px) scale(.97);background:#0b0e18;color:#f1f5f9;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:13px;border-radius:10px;min-width:160px;max-width:340px;box-shadow:0 4px 20px rgba(0,0,0,.45),0 0 0 1px rgba(255,255,255,.08);overflow:hidden}#sp-tip.sp-vis{opacity:1;transform:translateY(0) scale(1);pointer-events:auto}.sp-nav{display:flex;align-items:center;justify-content:space-between;padding:5px 10px;border-top:1px solid rgba(255,255,255,.08)}.sp-nav-btn{cursor:pointer;padding:0 10px;border-radius:5px;height:22px;line-height:22px;font-size:18px;background:rgba(255,255,255,.10);color:#e2e8f0;user-select:none;flex-shrink:0}.sp-nav-btn:hover{background:rgba(255,255,255,.22)}.sp-nav-dis{opacity:.25;pointer-events:none}.sp-nav-ctr{flex:1;text-align:center;font-size:11px;color:#94a3b8}.sp-head{padding:10px 14px 6px;font-weight:700;font-size:14px;color:#e2e8f0;border-bottom:1px solid rgba(255,255,255,.08)}.sp-body{padding:8px 14px 12px}.sp-row{display:flex;justify-content:space-between;align-items:baseline;gap:14px;padding:3px 0;border-bottom:1px solid rgba(255,255,255,.04)}.sp-row:last-child{border-bottom:none}.sp-key{color:#94a3b8;font-size:12px;white-space:nowrap}.sp-val{font-weight:600;font-size:12px;color:#f8fafc;text-align:right;word-break:break-all}#sp-tip img{display:block;width:100%;max-height:210px;object-fit:contain;border-top:1px solid rgba(255,255,255,.07)}#sp-tip video{display:block;width:100%;border-top:1px solid rgba(255,255,255,.07)}.sp-html{padding:8px 14px;font-size:12px;border-top:1px solid rgba(255,255,255,.07)}[data-idx]{cursor:pointer;transition:opacity .25s,filter .2s,transform .25s}[data-idx]:hover{filter:brightness(1.12) saturate(1.08)}.sp-cpanel{position:absolute;bottom:10px;left:50%;transform:translateX(-50%);background:#0b0e18;color:#f1f5f9;border-radius:10px;padding:8px 16px;font-size:12px;font-family:-apple-system,Arial,sans-serif;box-shadow:0 8px 24px rgba(0,0,0,.4);z-index:20;white-space:nowrap;display:none}.sp-cls-x{cursor:pointer;color:#94a3b8;margin-left:6px;font-size:13px}.sp-cls-x:hover{color:#f87171}</style><style>body{margin:0;background:transparent;padding:0}@keyframes sp-i{from{opacity:0;transform:translateY(8px) scale(.94)}}@keyframes sp-bar{from{opacity:0;transform:scaleY(0)}}@keyframes sp-pop{0%{opacity:0;transform:scale(0)}70%{transform:scale(1.08)}100%{opacity:1;transform:scale(1)}}@keyframes sp-arc{from{opacity:0;transform:scale(.82) rotate(-6deg)}}@keyframes sp-fn{from{opacity:0;transform:scaleX(.7) translateY(8px)}}svg rect[data-idx]{transform-box:fill-box;transform-origin:bottom center;animation:sp-bar .5s cubic-bezier(.22,.61,.36,1) both}svg circle[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-pop .42s cubic-bezier(.34,1.56,.64,1) both}svg path[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-arc .48s cubic-bezier(.22,.61,.36,1) both}svg polygon[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-fn .48s cubic-bezier(.22,.61,.36,1) both}svg line[data-idx]{animation:sp-i .45s cubic-bezier(.22,.61,.36,1) both}svg rect[data-idx]:hover{transform:scaleY(1.03);filter:brightness(1.12) saturate(1.1)}svg circle[data-idx]:hover{transform:scale(1.3);filter:brightness(1.15)}svg path[data-idx]:hover{filter:brightness(1.1) drop-shadow(0 2px 8px rgba(0,0,0,.2))}.sp-bg{fill:#fff}</style><style>.sp-bg{{fill:transparent!important}}.sp-ttl{{fill:#e2e8f0!important}}svg text{{fill:#cbd5e1!important}}.sp-ax-x,.sp-ax-y{{stroke:#475569!important}}.sp-gl{{stroke:#2d3748!important}}.sp-xl,.sp-yl{{fill:#94a3b8!important}}[id^='spp']{{box-shadow:none!important;border-radius:0!important}}</style></head><body><div id="spp1" style="position:relative;display:inline-block;border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)"><svg xmlns="http://www.w3.org/2000/svg" width="900" height="520" viewBox="0 0 900 520"><rect class="sp-bg" width="100%" height="100%"/><text x="450" y="24" text-anchor="middle" font-family="-apple-system,Arial,sans-serif" font-size="15" font-weight="700" fill="#1a202c" class="sp-ttl">Monthly Temperature Distribution</text><line x1="130" y1="476" x2="770" y2="476" stroke="#cbd5e1" stroke-width="1" class="sp-ax-x"/><text x="130" y="490" text-anchor="middle" font-family="Arial,sans-serif" font-size="10" fill="#9ca3af" class="sp-xt">-1.03</text><text x="236" y="490" text-anchor="middle" font-family="Arial,sans-serif" font-size="10" fill="#9ca3af" class="sp-xt">0.14</text><text x="343" y="490" text-anchor="middle" font-family="Arial,sans-serif" font-size="10" fill="#9ca3af" class="sp-xt">1.32</text><text x="450" y="490" text-anchor="middle" font-family="Arial,sans-serif" font-size="10" fill="#9ca3af" class="sp-xt">2.50</text><text x="556" y="490" text-anchor="middle" font-family="Arial,sans-serif" font-size="10" fill="#9ca3af" class="sp-xt">3.68</text><text x="663" y="490" text-anchor="middle" font-family="Arial,sans-serif" font-size="10" fill="#9ca3af" class="sp-xt">4.85</text><text x="770" y="490" text-anchor="middle" font-family="Arial,sans-serif" font-size="10" fill="#9ca3af" class="sp-xt">6.03</text><g data-series="3"><path d="M130.00,476.00 L130.00,476.00 L140.85,476.00 L151.69,476.00 L162.54,476.00 L173.39,476.00 L184.24,476.00 L195.08,476.00 L205.93,476.00 L216.78,476.00 L227.63,476.00 L238.47,476.00 L249.32,476.00 L260.17,476.00 L271.02,476.00 L281.86,476.00 L292.71,476.00 L303.56,476.00 L314.41,476.00 L325.25,476.00 L336.10,476.00 L346.95,475.94 L357.80,475.90 L368.64,475.84 L379.49,475.76 L390.34,475.62 L401.19,475.43 L412.03,475.16 L422.88,474.76 L433.73,474.20 L444.58,473.44 L455.42,472.40 L466.27,471.00 L477.12,469.17 L487.97,466.80 L498.81,463.78 L509.66,460.00 L520.51,455.36 L531.36,449.74 L542.20,443.07 L553.05,435.29 L563.90,426.39 L574.75,416.40 L585.59,405.42 L596.44,393.62 L607.29,381.20 L618.14,368.47 L628.98,355.76 L639.83,343.47 L650.68,331.99 L661.53,321.76 L672.37,313.14 L683.22,306.49 L694.07,302.09 L704.92,300.11 L715.76,300.63 L726.61,303.65 L737.46,309.02 L748.31,316.53 L759.15,325.87 L770.00,336.68 L770.00,476.00 Z" fill="#ffffff"/><path d="M130.00,476.00 L130.00,476.00 L140.85,476.00 L151.69,476.00 L162.54,476.00 L173.39,476.00 L184.24,476.00 L195.08,476.00 L205.93,476.00 L216.78,476.00 L227.63,476.00 L238.47,476.00 L249.32,476.00 L260.17,476.00 L271.02,476.00 L281.86,476.00 L292.71,476.00 L303.56,476.00 L314.41,476.00 L325.25,476.00 L336.10,476.00 L346.95,475.94 L357.80,475.90 L368.64,475.84 L379.49,475.76 L390.34,475.62 L401.19,475.43 L412.03,475.16 L422.88,474.76 L433.73,474.20 L444.58,473.44 L455.42,472.40 L466.27,471.00 L477.12,469.17 L487.97,466.80 L498.81,463.78 L509.66,460.00 L520.51,455.36 L531.36,449.74 L542.20,443.07 L553.05,435.29 L563.90,426.39 L574.75,416.40 L585.59,405.42 L596.44,393.62 L607.29,381.20 L618.14,368.47 L628.98,355.76 L639.83,343.47 L650.68,331.99 L661.53,321.76 L672.37,313.14 L683.22,306.49 L694.07,302.09 L704.92,300.11 L715.76,300.63 L726.61,303.65 L737.46,309.02 L748.31,316.53 L759.15,325.87 L770.00,336.68 L770.00,476.00 Z" fill="#f59e0b" fill-opacity="0.22"/><polyline points="130.00,476.00 140.85,476.00 151.69,476.00 162.54,476.00 173.39,476.00 184.24,476.00 195.08,476.00 205.93,476.00 216.78,476.00 227.63,476.00 238.47,476.00 249.32,476.00 260.17,476.00 271.02,476.00 281.86,476.00 292.71,476.00 303.56,476.00 314.41,476.00 325.25,476.00 336.10,476.00 346.95,475.94 357.80,475.90 368.64,475.84 379.49,475.76 390.34,475.62 401.19,475.43 412.03,475.16 422.88,474.76 433.73,474.20 444.58,473.44 455.42,472.40 466.27,471.00 477.12,469.17 487.97,466.80 498.81,463.78 509.66,460.00 520.51,455.36 531.36,449.74 542.20,443.07 553.05,435.29 563.90,426.39 574.75,416.40 585.59,405.42 596.44,393.62 607.29,381.20 618.14,368.47 628.98,355.76 639.83,343.47 650.68,331.99 661.53,321.76 672.37,313.14 683.22,306.49 694.07,302.09 704.92,300.11 715.76,300.63 726.61,303.65 737.46,309.02 748.31,316.53 759.15,325.87 770.00,336.68" fill="none" stroke="#f59e0b" stroke-width="2" stroke-linejoin="round"/><line x1="126" y1="476" x2="770" y2="476" stroke="#e2e8f0" stroke-width="0.6"/><text x="122" y="425" text-anchor="end" font-family="-apple-system,Arial,sans-serif" font-size="11" font-weight="600" fill="#374151">Oct</text></g><g data-series="2"><path d="M130.00,366.00 L130.00,361.99 L140.85,360.47 L151.69,358.47 L162.54,355.90 L173.39,352.65 L184.24,348.61 L195.08,343.66 L205.93,337.71 L216.78,330.68 L227.63,322.54 L238.47,313.28 L249.32,302.96 L260.17,291.70 L271.02,279.67 L281.86,267.12 L292.71,254.35 L303.56,241.73 L314.41,229.66 L325.25,218.54 L336.10,208.78 L346.95,200.77 L357.80,194.82 L368.64,191.18 L379.49,190.00 L390.34,191.34 L401.19,195.13 L412.03,201.22 L422.88,209.36 L433.73,219.21 L444.58,230.41 L455.42,242.53 L466.27,255.17 L477.12,267.93 L487.97,280.46 L498.81,292.45 L509.66,303.65 L520.51,313.91 L531.36,323.09 L542.20,331.16 L553.05,338.12 L563.90,344.00 L574.75,348.89 L585.59,352.88 L596.44,356.09 L607.29,358.61 L618.14,360.58 L628.98,362.07 L639.83,363.20 L650.68,364.03 L661.53,364.63 L672.37,365.07 L683.22,365.37 L694.07,365.58 L704.92,365.73 L715.76,365.82 L726.61,365.89 L737.46,365.93 L748.31,366.00 L759.15,366.00 L770.00,366.00 L770.00,366.00 Z" fill="#ffffff"/><path d="M130.00,366.00 L130.00,361.99 L140.85,360.47 L151.69,358.47 L162.54,355.90 L173.39,352.65 L184.24,348.61 L195.08,343.66 L205.93,337.71 L216.78,330.68 L227.63,322.54 L238.47,313.28 L249.32,302.96 L260.17,291.70 L271.02,279.67 L281.86,267.12 L292.71,254.35 L303.56,241.73 L314.41,229.66 L325.25,218.54 L336.10,208.78 L346.95,200.77 L357.80,194.82 L368.64,191.18 L379.49,190.00 L390.34,191.34 L401.19,195.13 L412.03,201.22 L422.88,209.36 L433.73,219.21 L444.58,230.41 L455.42,242.53 L466.27,255.17 L477.12,267.93 L487.97,280.46 L498.81,292.45 L509.66,303.65 L520.51,313.91 L531.36,323.09 L542.20,331.16 L553.05,338.12 L563.90,344.00 L574.75,348.89 L585.59,352.88 L596.44,356.09 L607.29,358.61 L618.14,360.58 L628.98,362.07 L639.83,363.20 L650.68,364.03 L661.53,364.63 L672.37,365.07 L683.22,365.37 L694.07,365.58 L704.92,365.73 L715.76,365.82 L726.61,365.89 L737.46,365.93 L748.31,366.00 L759.15,366.00 L770.00,366.00 L770.00,366.00 Z" fill="#10b981" fill-opacity="0.22"/><polyline points="130.00,361.99 140.85,360.47 151.69,358.47 162.54,355.90 173.39,352.65 184.24,348.61 195.08,343.66 205.93,337.71 216.78,330.68 227.63,322.54 238.47,313.28 249.32,302.96 260.17,291.70 271.02,279.67 281.86,267.12 292.71,254.35 303.56,241.73 314.41,229.66 325.25,218.54 336.10,208.78 346.95,200.77 357.80,194.82 368.64,191.18 379.49,190.00 390.34,191.34 401.19,195.13 412.03,201.22 422.88,209.36 433.73,219.21 444.58,230.41 455.42,242.53 466.27,255.17 477.12,267.93 487.97,280.46 498.81,292.45 509.66,303.65 520.51,313.91 531.36,323.09 542.20,331.16 553.05,338.12 563.90,344.00 574.75,348.89 585.59,352.88 596.44,356.09 607.29,358.61 618.14,360.58 628.98,362.07 639.83,363.20 650.68,364.03 661.53,364.63 672.37,365.07 683.22,365.37 694.07,365.58 704.92,365.73 715.76,365.82 726.61,365.89 737.46,365.93 748.31,366.00 759.15,366.00 770.00,366.00" fill="none" stroke="#10b981" stroke-width="2" stroke-linejoin="round"/><line x1="126" y1="366" x2="770" y2="366" stroke="#e2e8f0" stroke-width="0.6"/><text x="122" y="315" text-anchor="end" font-family="-apple-system,Arial,sans-serif" font-size="11" font-weight="600" fill="#374151">Jul</text></g><g data-series="1"><path d="M130.00,256.00 L130.00,255.02 L140.85,254.58 L151.69,253.95 L162.54,253.09 L173.39,251.93 L184.24,250.38 L195.08,248.36 L205.93,245.76 L216.78,242.48 L227.63,238.39 L238.47,233.40 L249.32,227.40 L260.17,220.32 L271.02,212.12 L281.86,202.81 L292.71,192.45 L303.56,181.14 L314.41,169.08 L325.25,156.51 L336.10,143.74 L346.95,131.14 L357.80,119.10 L368.64,108.04 L379.49,98.36 L390.34,90.44 L401.19,84.59 L412.03,81.06 L422.88,80.01 L433.73,81.46 L444.58,85.37 L455.42,91.57 L466.27,99.79 L477.12,109.72 L487.97,120.97 L498.81,133.12 L509.66,145.78 L520.51,158.53 L531.36,171.04 L542.20,183.00 L553.05,194.17 L563.90,204.37 L574.75,213.50 L585.59,221.52 L596.44,228.42 L607.29,234.26 L618.14,239.10 L628.98,243.05 L639.83,246.22 L650.68,248.72 L661.53,250.66 L672.37,252.14 L683.22,253.24 L694.07,254.06 L704.92,254.66 L715.76,255.08 L726.61,255.38 L737.46,255.59 L748.31,255.73 L759.15,255.83 L770.00,255.89 L770.00,256.00 Z" fill="#ffffff"/><path d="M130.00,256.00 L130.00,255.02 L140.85,254.58 L151.69,253.95 L162.54,253.09 L173.39,251.93 L184.24,250.38 L195.08,248.36 L205.93,245.76 L216.78,242.48 L227.63,238.39 L238.47,233.40 L249.32,227.40 L260.17,220.32 L271.02,212.12 L281.86,202.81 L292.71,192.45 L303.56,181.14 L314.41,169.08 L325.25,156.51 L336.10,143.74 L346.95,131.14 L357.80,119.10 L368.64,108.04 L379.49,98.36 L390.34,90.44 L401.19,84.59 L412.03,81.06 L422.88,80.01 L433.73,81.46 L444.58,85.37 L455.42,91.57 L466.27,99.79 L477.12,109.72 L487.97,120.97 L498.81,133.12 L509.66,145.78 L520.51,158.53 L531.36,171.04 L542.20,183.00 L553.05,194.17 L563.90,204.37 L574.75,213.50 L585.59,221.52 L596.44,228.42 L607.29,234.26 L618.14,239.10 L628.98,243.05 L639.83,246.22 L650.68,248.72 L661.53,250.66 L672.37,252.14 L683.22,253.24 L694.07,254.06 L704.92,254.66 L715.76,255.08 L726.61,255.38 L737.46,255.59 L748.31,255.73 L759.15,255.83 L770.00,255.89 L770.00,256.00 Z" fill="#f43f5e" fill-opacity="0.22"/><polyline points="130.00,255.02 140.85,254.58 151.69,253.95 162.54,253.09 173.39,251.93 184.24,250.38 195.08,248.36 205.93,245.76 216.78,242.48 227.63,238.39 238.47,233.40 249.32,227.40 260.17,220.32 271.02,212.12 281.86,202.81 292.71,192.45 303.56,181.14 314.41,169.08 325.25,156.51 336.10,143.74 346.95,131.14 357.80,119.10 368.64,108.04 379.49,98.36 390.34,90.44 401.19,84.59 412.03,81.06 422.88,80.01 433.73,81.46 444.58,85.37 455.42,91.57 466.27,99.79 477.12,109.72 487.97,120.97 498.81,133.12 509.66,145.78 520.51,158.53 531.36,171.04 542.20,183.00 553.05,194.17 563.90,204.37 574.75,213.50 585.59,221.52 596.44,228.42 607.29,234.26 618.14,239.10 628.98,243.05 639.83,246.22 650.68,248.72 661.53,250.66 672.37,252.14 683.22,253.24 694.07,254.06 704.92,254.66 715.76,255.08 726.61,255.38 737.46,255.59 748.31,255.73 759.15,255.83 770.00,255.89" fill="none" stroke="#f43f5e" stroke-width="2" stroke-linejoin="round"/><line x1="126" y1="256" x2="770" y2="256" stroke="#e2e8f0" stroke-width="0.6"/><text x="122" y="205" text-anchor="end" font-family="-apple-system,Arial,sans-serif" font-size="11" font-weight="600" fill="#374151">Apr</text></g><g data-series="0"><path d="M130.00,146.00 L130.00,6.68 L140.85,-4.13 L151.69,-13.47 L162.54,-20.98 L173.39,-26.35 L184.24,-29.37 L195.08,-29.89 L205.93,-27.91 L216.78,-23.51 L227.63,-16.86 L238.47,-8.24 L249.32,1.99 L260.17,13.47 L271.02,25.76 L281.86,38.47 L292.71,51.20 L303.56,63.62 L314.41,75.42 L325.25,86.40 L336.10,96.39 L346.95,105.29 L357.80,113.07 L368.64,119.74 L379.49,125.36 L390.34,130.00 L401.19,133.78 L412.03,136.80 L422.88,139.17 L433.73,141.00 L444.58,142.40 L455.42,143.44 L466.27,144.20 L477.12,144.76 L487.97,145.16 L498.81,145.43 L509.66,145.62 L520.51,145.76 L531.36,145.84 L542.20,145.90 L553.05,145.94 L563.90,146.00 L574.75,146.00 L585.59,146.00 L596.44,146.00 L607.29,146.00 L618.14,146.00 L628.98,146.00 L639.83,146.00 L650.68,146.00 L661.53,146.00 L672.37,146.00 L683.22,146.00 L694.07,146.00 L704.92,146.00 L715.76,146.00 L726.61,146.00 L737.46,146.00 L748.31,146.00 L759.15,146.00 L770.00,146.00 L770.00,146.00 Z" fill="#ffffff"/><path d="M130.00,146.00 L130.00,6.68 L140.85,-4.13 L151.69,-13.47 L162.54,-20.98 L173.39,-26.35 L184.24,-29.37 L195.08,-29.89 L205.93,-27.91 L216.78,-23.51 L227.63,-16.86 L238.47,-8.24 L249.32,1.99 L260.17,13.47 L271.02,25.76 L281.86,38.47 L292.71,51.20 L303.56,63.62 L314.41,75.42 L325.25,86.40 L336.10,96.39 L346.95,105.29 L357.80,113.07 L368.64,119.74 L379.49,125.36 L390.34,130.00 L401.19,133.78 L412.03,136.80 L422.88,139.17 L433.73,141.00 L444.58,142.40 L455.42,143.44 L466.27,144.20 L477.12,144.76 L487.97,145.16 L498.81,145.43 L509.66,145.62 L520.51,145.76 L531.36,145.84 L542.20,145.90 L553.05,145.94 L563.90,146.00 L574.75,146.00 L585.59,146.00 L596.44,146.00 L607.29,146.00 L618.14,146.00 L628.98,146.00 L639.83,146.00 L650.68,146.00 L661.53,146.00 L672.37,146.00 L683.22,146.00 L694.07,146.00 L704.92,146.00 L715.76,146.00 L726.61,146.00 L737.46,146.00 L748.31,146.00 L759.15,146.00 L770.00,146.00 L770.00,146.00 Z" fill="#6366f1" fill-opacity="0.22"/><polyline points="130.00,6.68 140.85,-4.13 151.69,-13.47 162.54,-20.98 173.39,-26.35 184.24,-29.37 195.08,-29.89 205.93,-27.91 216.78,-23.51 227.63,-16.86 238.47,-8.24 249.32,1.99 260.17,13.47 271.02,25.76 281.86,38.47 292.71,51.20 303.56,63.62 314.41,75.42 325.25,86.40 336.10,96.39 346.95,105.29 357.80,113.07 368.64,119.74 379.49,125.36 390.34,130.00 401.19,133.78 412.03,136.80 422.88,139.17 433.73,141.00 444.58,142.40 455.42,143.44 466.27,144.20 477.12,144.76 487.97,145.16 498.81,145.43 509.66,145.62 520.51,145.76 531.36,145.84 542.20,145.90 553.05,145.94 563.90,146.00 574.75,146.00 585.59,146.00 596.44,146.00 607.29,146.00 618.14,146.00 628.98,146.00 639.83,146.00 650.68,146.00 661.53,146.00 672.37,146.00 683.22,146.00 694.07,146.00 704.92,146.00 715.76,146.00 726.61,146.00 737.46,146.00 748.31,146.00 759.15,146.00 770.00,146.00" fill="none" stroke="#6366f1" stroke-width="2" stroke-linejoin="round"/><line x1="126" y1="146" x2="770" y2="146" stroke="#e2e8f0" stroke-width="0.6"/><text x="122" y="95" text-anchor="end" font-family="-apple-system,Arial,sans-serif" font-size="11" font-weight="600" fill="#374151">Jan</text></g><g data-legend="1" data-series="0"><rect x="778" y="46" width="12" height="12" rx="2" fill="#6366f1"/><text x="794" y="56" font-family="Arial,sans-serif" font-size="10" fill="#374151">Jan</text></g><g data-legend="1" data-series="1"><rect x="778" y="68" width="12" height="12" rx="2" fill="#f43f5e"/><text x="794" y="78" font-family="Arial,sans-serif" font-size="10" fill="#374151">Apr</text></g><g data-legend="1" data-series="2"><rect x="778" y="90" width="12" height="12" rx="2" fill="#10b981"/><text x="794" y="100" font-family="Arial,sans-serif" font-size="10" fill="#374151">Jul</text></g><g data-legend="1" data-series="3"><rect x="778" y="112" width="12" height="12" rx="2" fill="#f59e0b"/><text x="794" y="122" font-family="Arial,sans-serif" font-size="10" fill="#374151">Oct</text></g><text x="450" y="516" text-anchor="middle" font-family="Arial,sans-serif" font-size="10" fill="#6b7280" class="sp-xl">°C</text></svg><div class="sp-sel-ov" style="display:none"></div><div class="sp-cpanel"></div><script>(function(){
var wrap=document.getElementById('spp1');if(!wrap)return;wrap.removeAttribute('id');
var svg=wrap.querySelector('svg');var data=[];

var tip=document.getElementById('sp-tip');
if(!tip){tip=document.createElement('div');tip.id='sp-tip';document.body.appendChild(tip);}
var tipIdxs=[],tipPos=0,lastE=null;
function esc(s){return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;');}
function cardHTML(d){
 var h='';if(d.title)h+='<div class="sp-head">'+esc(d.title)+'</div>';
 var rows='';(d.kv||[]).forEach(function(p){
  rows+='<div class="sp-row"><span class="sp-key">'+esc(p[0])+'</span><span class="sp-val">'+esc(p[1])+'</span></div>';});
 if(rows)h+='<div class="sp-body">'+rows+'</div>';
 if(d.image)h+='<img src="'+esc(d.image)+'" alt="" loading="lazy">';
 if(d.video)h+='<video src="'+esc(d.video)+'" controls muted playsinline></video>';
 if(d.html)h+='<div class="sp-html">'+d.html+'</div>';
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
 var el=svg.querySelector('[data-idx="'+idx+'"]');if(!el)return null;
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
 var title=lbl||(el.getAttribute('data-v')!=null&&el.getAttribute('data-r')!=null?el.getAttribute('data-r')+' \u00d7 '+el.getAttribute('data-c'):'Point '+(idx+1));
 return{title:title,kv:kv};}
function renderTip(){
 if(!tipIdxs.length){tip.classList.remove('sp-vis');return;}
 var d=getSlot(tipIdxs[tipPos]);if(!d){tip.classList.remove('sp-vis');return;}
 var h=cardHTML(d);
 if(tipIdxs.length>1){
  var p=tipPos,n=tipIdxs.length;
  h+='<div class="sp-nav">'
   +'<span class="sp-nav-btn'+(p>0?'':' sp-nav-dis')+'" data-d="-1">&#8249;</span>'
   +'<span class="sp-nav-ctr">'+(p+1)+' / '+n+'</span>'
   +'<span class="sp-nav-btn'+(p<n-1?'':' sp-nav-dis')+'" data-d="1">&#8250;</span>'
   +'</div>';}
 tip.innerHTML=h;tip.classList.add('sp-vis');
 if(lastE)placeTip(lastE);
 tip.querySelectorAll('.sp-nav-btn:not(.sp-nav-dis)').forEach(function(btn){
  btn.onclick=function(ev){ev.stopPropagation();
   var nd=tipPos+parseInt(btn.getAttribute('data-d'),10);
   if(nd>=0&&nd<tipIdxs.length){tipPos=nd;renderTip();}};});}
var dragging=false,dsx=0,dsy=0,moved=false,pinned=false;

wrap.addEventListener('mouseleave',function(e){
 if(pinned)return;
 var rt=e.relatedTarget;
 if(rt&&(rt===tip||tip.contains(rt)))return;
 tip.classList.remove('sp-vis');tipIdxs=[];});
wrap.addEventListener('mousemove',function(e){
 if(dragging||pinned)return;
 lastE=e;

 var hits=[];
 document.elementsFromPoint(e.clientX,e.clientY).forEach(function(el){
  if(el===tip||tip.contains(el))return;
  var found=null;
  for(var n=el;n&&n!==document.body;n=n.parentElement){
   if(found===null&&n.getAttribute&&n.getAttribute('data-idx')!==null)found=n;
   if(n===wrap){
    if(found!==null){var idx=parseInt(found.getAttribute('data-idx'),10);
     if(hits.indexOf(idx)===-1)hits.push(idx);}
    return;}
  }
 });
 if(hits.length){
  var same=hits.length===tipIdxs.length&&hits.every(function(v,i){return v===tipIdxs[i];});
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
 if(!moved&&Math.abs(cx-dsx)<8&&Math.abs(cy-dsy)<8)return;
 if(!moved){pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}
 moved=true;ov.style.display='block';
 ov.style.left=Math.min(dsx,cx)+'px';ov.style.top=Math.min(dsy,cy)+'px';
 ov.style.width=Math.abs(cx-dsx)+'px';ov.style.height=Math.abs(cy-dsy)+'px';});
document.addEventListener('mouseup',function(e){if(!dragging)return;dragging=false;
 ov.style.display='none';if(!moved){
 var ch=[];document.elementsFromPoint(e.clientX,e.clientY).forEach(function(el){if(el===tip||tip.contains(el))return;var fd=null;for(var n2=el;n2&&n2!==document.body;n2=n2.parentElement){if(fd===null&&n2.getAttribute&&n2.getAttribute('data-idx')!==null)fd=n2;if(n2===wrap){if(fd!==null){var idx=parseInt(fd.getAttribute('data-idx'),10);if(ch.indexOf(idx)===-1)ch.push(idx);}return;}}});
 if(ch.length){pinned=true;tipIdxs=ch;tipPos=0;lastE=e;renderTip();}else{pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}return;}
 var r=wrap.getBoundingClientRect();var cx=e.clientX-r.left,cy=e.clientY-r.top;
 var rx1=Math.min(dsx,cx),ry1=Math.min(dsy,cy),rx2=Math.max(dsx,cx),ry2=Math.max(dsy,cy);
 if(rx2-rx1<8&&ry2-ry1<8)return;
 var ctm=svg.getScreenCTM();if(!ctm)return;var inv=ctm.inverse();
 function toS(px,py){var pt=svg.createSVGPoint();pt.x=px+r.left;pt.y=py+r.top;return pt.matrixTransform(inv);}
 var p1=toS(rx1,ry1),p2=toS(rx2,ry2);
 var bx1=Math.min(p1.x,p2.x),by1=Math.min(p1.y,p2.y),bx2=Math.max(p1.x,p2.x),by2=Math.max(p1.y,p2.y);
 var pts=svg.querySelectorAll('[data-idx]');var sel=[],unsel=[];
 pts.forEach(function(el){try{var bb=el.getBBox();var ecx=bb.x+bb.width/2,ecy=bb.y+bb.height/2;
  if(ecx>=bx1&&ecx<=bx2&&ecy>=by1&&ecy<=by2)sel.push(el);else unsel.push(el);}catch(ex){unsel.push(el);}});
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
 var s='<span style="color:#6366F1;font-weight:700">'+sel.length+' pts</span>';
 if(xs.length&&ys.length){
  var mx=xs.reduce(function(a,b){return a+b;})/xs.length;
  var my=ys.reduce(function(a,b){return a+b;})/ys.length;
  var vX=xs.map(function(v){return(v-mx)*(v-mx);}).reduce(function(a,b){return a+b;})/xs.length;
  var vY=ys.map(function(v){return(v-my)*(v-my);}).reduce(function(a,b){return a+b;})/ys.length;
  s+=' &middot; X&#772; <b>'+mx.toFixed(2)+'</b> &plusmn;'+Math.sqrt(vX).toFixed(2);
  s+=' &middot; Y&#772; <b>'+my.toFixed(2)+'</b> &plusmn;'+Math.sqrt(vY).toFixed(2);}
 s+=' <span class="sp-cls-x">&#10005;</span>';
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
 for(var nd=e.target;nd&&nd!==svg;nd=nd.parentElement){if(nd.getAttribute&&nd.getAttribute('data-idx')!==null){found=nd;break;}}
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
 var els=svg.querySelectorAll('[data-series="'+s+'"]:not([data-legend])');
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
 var gls=svg.querySelectorAll('.sp-gl');if(gls.length&&nT>1)for(var j=0;j<gls.length;j++){var f=(j+1)/(nT-1);var gy=pT+Math.round((1-f)*pH);gls[j].setAttribute('y1',gy);gls[j].setAttribute('y2',gy);}
}
svg.querySelectorAll('[data-idx]').forEach(function(el,i){el.style.animationDelay=(i*18)+'ms';});
})();</script></div></body></html>
</div>

</details>

---

## See also

- [KDE](kde.md)
- [Violin](violin.md)
- [Ridgeline 3D](../3d/ridgeline3d.md)
