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

<div style="width:100%;overflow:auto;border-radius:8px;margin:12px 0;background:#0d1117">
<!DOCTYPE html><html><head><meta charset="utf-8"><title>Normal Distribution — N(0,1)</title><style>#sp-tip{position:absolute;z-index:999999;pointer-events:none;opacity:0;transition:opacity .15s,transform .15s;transform:translateY(6px) scale(.97);background:#0b0e18;color:#f1f5f9;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:13px;border-radius:10px;min-width:160px;max-width:340px;box-shadow:0 4px 20px rgba(0,0,0,.45),0 0 0 1px rgba(255,255,255,.08);overflow:hidden}#sp-tip.sp-vis{opacity:1;transform:translateY(0) scale(1);pointer-events:auto}.sp-nav{display:flex;align-items:center;justify-content:space-between;padding:5px 10px;border-top:1px solid rgba(255,255,255,.08)}.sp-nav-btn{cursor:pointer;padding:0 10px;border-radius:5px;height:22px;line-height:22px;font-size:18px;background:rgba(255,255,255,.10);color:#e2e8f0;user-select:none;flex-shrink:0}.sp-nav-btn:hover{background:rgba(255,255,255,.22)}.sp-nav-dis{opacity:.25;pointer-events:none}.sp-nav-ctr{flex:1;text-align:center;font-size:11px;color:#94a3b8}.sp-head{padding:10px 14px 6px;font-weight:700;font-size:14px;color:#e2e8f0;border-bottom:1px solid rgba(255,255,255,.08)}.sp-body{padding:8px 14px 12px}.sp-row{display:flex;justify-content:space-between;align-items:baseline;gap:14px;padding:3px 0;border-bottom:1px solid rgba(255,255,255,.04)}.sp-row:last-child{border-bottom:none}.sp-key{color:#94a3b8;font-size:12px;white-space:nowrap}.sp-val{font-weight:600;font-size:12px;color:#f8fafc;text-align:right;word-break:break-all}#sp-tip img{display:block;width:100%;max-height:210px;object-fit:contain;border-top:1px solid rgba(255,255,255,.07)}#sp-tip video{display:block;width:100%;border-top:1px solid rgba(255,255,255,.07)}.sp-html{padding:8px 14px;font-size:12px;border-top:1px solid rgba(255,255,255,.07)}[data-idx]{cursor:pointer;transition:opacity .25s,filter .2s,transform .25s}[data-idx]:hover{filter:brightness(1.12) saturate(1.08)}.sp-cpanel{position:absolute;bottom:10px;left:50%;transform:translateX(-50%);background:#0b0e18;color:#f1f5f9;border-radius:10px;padding:8px 16px;font-size:12px;font-family:-apple-system,Arial,sans-serif;box-shadow:0 8px 24px rgba(0,0,0,.4);z-index:20;white-space:nowrap;display:none}.sp-cls-x{cursor:pointer;color:#94a3b8;margin-left:6px;font-size:13px}.sp-cls-x:hover{color:#f87171}</style><style>body{margin:0;background:#fff;display:flex;justify-content:center;padding:0}@keyframes sp-i{from{opacity:0;transform:translateY(8px) scale(.94)}}@keyframes sp-bar{from{opacity:0;transform:scaleY(0)}}@keyframes sp-pop{0%{opacity:0;transform:scale(0)}70%{transform:scale(1.08)}100%{opacity:1;transform:scale(1)}}@keyframes sp-arc{from{opacity:0;transform:scale(.82) rotate(-6deg)}}@keyframes sp-fn{from{opacity:0;transform:scaleX(.7) translateY(8px)}}svg rect[data-idx]{transform-box:fill-box;transform-origin:bottom center;animation:sp-bar .5s cubic-bezier(.22,.61,.36,1) both}svg circle[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-pop .42s cubic-bezier(.34,1.56,.64,1) both}svg path[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-arc .48s cubic-bezier(.22,.61,.36,1) both}svg polygon[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-fn .48s cubic-bezier(.22,.61,.36,1) both}svg line[data-idx]{animation:sp-i .45s cubic-bezier(.22,.61,.36,1) both}svg rect[data-idx]:hover{transform:scaleY(1.03);filter:brightness(1.12) saturate(1.1)}svg circle[data-idx]:hover{transform:scale(1.3);filter:brightness(1.15)}svg path[data-idx]:hover{filter:brightness(1.1) drop-shadow(0 2px 8px rgba(0,0,0,.2))}.sp-bg{fill:#fff}</style><style>.sp-bg{{fill:transparent!important}}.sp-ttl{{fill:#e2e8f0!important}}svg text{{fill:#cbd5e1!important}}.sp-ax-x,.sp-ax-y{{stroke:#475569!important}}.sp-gl{{stroke:#2d3748!important}}.sp-xl,.sp-yl{{fill:#94a3b8!important}}[id^='spp']{{box-shadow:none!important;border-radius:0!important}}</style></head><body><div id="spp1" style="position:relative;display:inline-block;border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)"><svg xmlns="http://www.w3.org/2000/svg" width="860" height="380" viewBox="0 0 860 380"><rect class="sp-bg" width="100%" height="100%"/><text x="446" y="26" text-anchor="middle" font-family="-apple-system,Arial,sans-serif" font-size="15" font-weight="700" fill="#1a202c" class="sp-ttl">Normal Distribution — N(0,1)</text><text x="48" y="337" text-anchor="end" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-yt">0</text><line x1="52" y1="274" x2="840" y2="274" stroke="#e2e8f0" stroke-width="0.5" class="sp-gl"/><text x="48" y="277" text-anchor="end" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-yt">80</text><line x1="52" y1="214" x2="840" y2="214" stroke="#e2e8f0" stroke-width="0.5" class="sp-gl"/><text x="48" y="217" text-anchor="end" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-yt">160</text><line x1="52" y1="155" x2="840" y2="155" stroke="#e2e8f0" stroke-width="0.5" class="sp-gl"/><text x="48" y="158" text-anchor="end" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-yt">240</text><line x1="52" y1="95" x2="840" y2="95" stroke="#e2e8f0" stroke-width="0.5" class="sp-gl"/><text x="48" y="98" text-anchor="end" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-yt">320</text><line x1="52" y1="36" x2="840" y2="36" stroke="#e2e8f0" stroke-width="0.5" class="sp-gl"/><text x="48" y="39" text-anchor="end" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-yt">400</text><line x1="52" y1="36" x2="52" y2="334" stroke="#cbd5e1" stroke-width="1" class="sp-ax-y"/><line x1="52" y1="334" x2="840" y2="334" stroke="#cbd5e1" stroke-width="1" class="sp-ax-x"/><text x="446" y="376" text-anchor="middle" font-family="Arial,sans-serif" font-size="10" fill="#6b7280" class="sp-xl">Value</text><text x="14" y="185" text-anchor="middle" font-family="Arial,sans-serif" font-size="10" fill="#6b7280" transform="rotate(-90,14,185)" class="sp-yl">Count</text><rect data-idx="0" data-series="0" data-lbl="-3.04–-2.87" data-kv-Count="10" data-kv-Min="-3.04" data-kv-Max="-2.87" x="52" y="327" width="18" height="7" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="1" data-series="0" data-lbl="-2.87–-2.70" data-kv-Count="5" data-kv-Min="-2.87" data-kv-Max="-2.70" x="71" y="331" width="18" height="3" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="2" data-series="0" data-lbl="-2.70–-2.53" data-kv-Count="0" data-kv-Min="-2.70" data-kv-Max="-2.53" x="91" y="334" width="18" height="0" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="3" data-series="0" data-lbl="-2.53–-2.36" data-kv-Count="15" data-kv-Min="-2.53" data-kv-Max="-2.36" x="111" y="323" width="18" height="11" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="4" data-series="0" data-lbl="-2.36–-2.19" data-kv-Count="15" data-kv-Min="-2.36" data-kv-Max="-2.19" x="130" y="323" width="18" height="11" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="5" data-series="0" data-lbl="-2.19–-2.02" data-kv-Count="55" data-kv-Min="-2.19" data-kv-Max="-2.02" x="150" y="294" width="18" height="40" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="6" data-series="0" data-lbl="-2.02–-1.86" data-kv-Count="50" data-kv-Min="-2.02" data-kv-Max="-1.86" x="170" y="297" width="18" height="37" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="7" data-series="0" data-lbl="-1.86–-1.69" data-kv-Count="60" data-kv-Min="-1.86" data-kv-Max="-1.69" x="189" y="290" width="18" height="44" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="8" data-series="0" data-lbl="-1.69–-1.52" data-kv-Count="120" data-kv-Min="-1.69" data-kv-Max="-1.52" x="209" y="245" width="18" height="89" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="9" data-series="0" data-lbl="-1.52–-1.35" data-kv-Count="110" data-kv-Min="-1.52" data-kv-Max="-1.35" x="229" y="253" width="18" height="81" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="10" data-series="0" data-lbl="-1.35–-1.18" data-kv-Count="120" data-kv-Min="-1.35" data-kv-Max="-1.18" x="249" y="245" width="18" height="89" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="11" data-series="0" data-lbl="-1.18–-1.01" data-kv-Count="230" data-kv-Min="-1.18" data-kv-Max="-1.01" x="268" y="163" width="18" height="171" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="12" data-series="0" data-lbl="-1.01–-0.84" data-kv-Count="190" data-kv-Min="-1.01" data-kv-Max="-0.84" x="288" y="193" width="18" height="141" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="13" data-series="0" data-lbl="-0.84–-0.67" data-kv-Count="300" data-kv-Min="-0.84" data-kv-Max="-0.67" x="308" y="111" width="18" height="223" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="14" data-series="0" data-lbl="-0.67–-0.50" data-kv-Count="245" data-kv-Min="-0.67" data-kv-Max="-0.50" x="327" y="152" width="18" height="182" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="15" data-series="0" data-lbl="-0.50–-0.33" data-kv-Count="335" data-kv-Min="-0.50" data-kv-Max="-0.33" x="347" y="85" width="18" height="249" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="16" data-series="0" data-lbl="-0.33–-0.16" data-kv-Count="315" data-kv-Min="-0.33" data-kv-Max="-0.16" x="367" y="100" width="18" height="234" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="17" data-series="0" data-lbl="-0.16–0.01" data-kv-Count="400" data-kv-Min="-0.16" data-kv-Max="0.01" x="386" y="36" width="18" height="298" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="18" data-series="0" data-lbl="0.01–0.18" data-kv-Count="355" data-kv-Min="0.01" data-kv-Max="0.18" x="406" y="70" width="18" height="264" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="19" data-series="0" data-lbl="0.18–0.35" data-kv-Count="280" data-kv-Min="0.18" data-kv-Max="0.35" x="426" y="126" width="18" height="208" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="20" data-series="0" data-lbl="0.35–0.51" data-kv-Count="285" data-kv-Min="0.35" data-kv-Max="0.51" x="446" y="122" width="18" height="212" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="21" data-series="0" data-lbl="0.51–0.68" data-kv-Count="260" data-kv-Min="0.51" data-kv-Max="0.68" x="465" y="141" width="18" height="193" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="22" data-series="0" data-lbl="0.68–0.85" data-kv-Count="250" data-kv-Min="0.68" data-kv-Max="0.85" x="485" y="148" width="18" height="186" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="23" data-series="0" data-lbl="0.85–1.02" data-kv-Count="260" data-kv-Min="0.85" data-kv-Max="1.02" x="505" y="141" width="18" height="193" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="24" data-series="0" data-lbl="1.02–1.19" data-kv-Count="130" data-kv-Min="1.02" data-kv-Max="1.19" x="524" y="238" width="18" height="96" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="25" data-series="0" data-lbl="1.19–1.36" data-kv-Count="140" data-kv-Min="1.19" data-kv-Max="1.36" x="544" y="230" width="18" height="104" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="26" data-series="0" data-lbl="1.36–1.53" data-kv-Count="155" data-kv-Min="1.36" data-kv-Max="1.53" x="564" y="219" width="18" height="115" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="27" data-series="0" data-lbl="1.53–1.70" data-kv-Count="90" data-kv-Min="1.53" data-kv-Max="1.70" x="583" y="267" width="18" height="67" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="28" data-series="0" data-lbl="1.70–1.87" data-kv-Count="75" data-kv-Min="1.70" data-kv-Max="1.87" x="603" y="279" width="18" height="55" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="29" data-series="0" data-lbl="1.87–2.04" data-kv-Count="50" data-kv-Min="1.87" data-kv-Max="2.04" x="623" y="297" width="18" height="37" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="30" data-series="0" data-lbl="2.04–2.21" data-kv-Count="30" data-kv-Min="2.04" data-kv-Max="2.21" x="643" y="312" width="18" height="22" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="31" data-series="0" data-lbl="2.21–2.38" data-kv-Count="30" data-kv-Min="2.21" data-kv-Max="2.38" x="662" y="312" width="18" height="22" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="32" data-series="0" data-lbl="2.38–2.55" data-kv-Count="10" data-kv-Min="2.38" data-kv-Max="2.55" x="682" y="327" width="18" height="7" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="33" data-series="0" data-lbl="2.55–2.72" data-kv-Count="0" data-kv-Min="2.55" data-kv-Max="2.72" x="702" y="334" width="18" height="0" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="34" data-series="0" data-lbl="2.72–2.89" data-kv-Count="0" data-kv-Min="2.72" data-kv-Max="2.89" x="721" y="334" width="18" height="0" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="35" data-series="0" data-lbl="2.89–3.05" data-kv-Count="15" data-kv-Min="2.89" data-kv-Max="3.05" x="741" y="323" width="18" height="11" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="36" data-series="0" data-lbl="3.05–3.22" data-kv-Count="0" data-kv-Min="3.05" data-kv-Max="3.22" x="761" y="334" width="18" height="0" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="37" data-series="0" data-lbl="3.22–3.39" data-kv-Count="5" data-kv-Min="3.22" data-kv-Max="3.39" x="780" y="331" width="18" height="3" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="38" data-series="0" data-lbl="3.39–3.56" data-kv-Count="0" data-kv-Min="3.39" data-kv-Max="3.56" x="800" y="334" width="18" height="0" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><rect data-idx="39" data-series="0" data-lbl="3.56–3.73" data-kv-Count="5" data-kv-Min="3.56" data-kv-Max="3.73" x="820" y="331" width="18" height="3" fill="#6366f1" fill-opacity="0.80" rx="2" stroke="#fff" stroke-width="0.4"/><text x="52" y="348" text-anchor="middle" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-xt">-3.04</text><text x="150" y="348" text-anchor="middle" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-xt">-2.19</text><text x="249" y="348" text-anchor="middle" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-xt">-1.35</text><text x="347" y="348" text-anchor="middle" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-xt">-0.50</text><text x="446" y="348" text-anchor="middle" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-xt">0.35</text><text x="544" y="348" text-anchor="middle" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-xt">1.19</text><text x="643" y="348" text-anchor="middle" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-xt">2.04</text><text x="741" y="348" text-anchor="middle" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-xt">2.89</text><text x="840" y="348" text-anchor="middle" font-family="Arial,sans-serif" font-size="9" fill="#9ca3af" class="sp-xt">3.73</text></svg><div class="sp-sel-ov" style="display:none"></div><div class="sp-cpanel"></div><script>(function(){
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

- [Histogram Overlay](histogram-overlay.md) — compare two distributions
- [KDE](kde.md) — smooth density estimate
- [Violin](violin.md) — distribution by category
