# Pulse Chart

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.pulse(title, labels, values, *, variant="radial", **kwargs) -> Chart`

Aliases: `sp.pulse`, `sp.pulse_chart`, `sp.radial_bar`, `sp.clock_chart`, `sp.rhythm`, `sp.radial_rhythm`

## Description

The Pulse chart is an original SeraPlot chart type that maps temporal or cyclic data onto a radial clock-face layout. Each slice is a time period (hour, day, month…) and the bar height encodes the intensity value. The `"wave"` variant connects data points into a smooth radial polygon.

| Variant | Description |
|---------|-------------|
| `"radial"` | Clock-face arc bars per period |
| `"wave"` | Smooth closed radial polygon |
| `"dot"` | Radial scatter with connecting spokes |
| `"filled"` | Fully opaque arc bars |
| `"gradient"` | Radial gradient arcs |

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `labels` | `list[str]` | Period labels (e.g. days, hours) |
| `values` | `list[float]` | Intensity per period (any scale) |
| `variant` | `str` | Rendering style |
| `palette` | `list[int]` | Custom colors (one per segment) |
| `inner_r` | `float` | Inner ring radius (default 60) |
| `show_labels` | `bool` | Show period labels |
| `width` / `height` | `int` | Chart dimensions (default 560×560) |

## Example

```python
import seraplot as sp
chart = sp.pulse(
    "Weekly Activity",
    labels=["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
    values=[0.6, 0.9, 0.75, 0.5, 0.85, 0.3, 0.4],
)
chart.show()
```

<div class="sp-cls sp-open" id="pulse-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('pulse-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('pulse-en','radial',this)"><span class="sp-cic">◉</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('pulse-en','wave',this)"><span class="sp-cic">∿</span><span class="sp-clb">Wave</span></button>
<button class="sp-cls-tab" onclick="spCls('pulse-en','dot',this)"><span class="sp-cic">•</span><span class="sp-clb">Dot</span></button>
<button class="sp-cls-tab" onclick="spCls('pulse-en','filled',this)"><span class="sp-cic">●</span><span class="sp-clb">Filled</span></button>
<button class="sp-cls-tab" onclick="spCls('pulse-en','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Gradient</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="pulse-en-radial">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"radial"</code></span><span><strong>Aliases</strong> <code>radial / default / classic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pulse-radial.html"></iframe>
</div>
<div class="sp-variant" id="pulse-en-wave">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"wave"</code></span><span><strong>Aliases</strong> <code>wave / sine / smooth</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pulse-wave.html"></iframe>
</div>
<div class="sp-variant" id="pulse-en-dot">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"dot"</code></span><span><strong>Aliases</strong> <code>dot / scatter / bubble</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pulse-dot.html"></iframe>
</div>
<div class="sp-variant" id="pulse-en-filled">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"filled"</code></span><span><strong>Aliases</strong> <code>filled / area / solid</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pulse-filled.html"></iframe>
</div>
<div class="sp-variant" id="pulse-en-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / color / heatring</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/pulse-gradient.html"></iframe>
</div>
</div>
</div>

</div>
