# Scatter 3D

## Signature

```python
sp.build_scatter3d_chart(
    title: str,
    x: list[float],
    y: list[float],
    z: list[float],
    *,
    color_values: list[float] | None = None,
    color_labels: list[str] | None = None,
    series_names: list[str] | None = None,
    bg_color: str = "#1a1a2e",
    width: int = 900,
    height: int = 600,
    x_label: str = "X",
    y_label: str = "Y",
    z_label: str = "Z",
    hover_json: str | None = None,
    palette: list[int] | None = None,
) -> Chart
```

---

## Description

GPU-accelerated 3D scatter plot rendered via WebGL.
Handles millions of points at interactive frame rates.

Use `color_values` for a continuous color scale, or `color_labels` for categorical coloring.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x` | `list[float]` | required | X coordinates |
| `y` | `list[float]` | required | Y coordinates |
| `z` | `list[float]` | required | Z coordinates |
| `color_values` | `list[float] \| None` | `None` | Continuous colormap values |
| `color_labels` | `list[str] \| None` | `None` | Categorical color group labels |
| `series_names` | `list[str] \| None` | `None` | Series legend names |
| `bg_color` | `str` | `"#1a1a2e"` | Canvas background color |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `x_label` | `str` | `"X"` | X-axis label |
| `y_label` | `str` | `"Y"` | Y-axis label |
| `z_label` | `str` | `"Z"` | Z-axis label |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |
| `palette` | `list[int] \| None` | `None` | Custom color palette |

---

## Returns

`Chart`

---

## Performance

The renderer uses a single `gl.drawArrays(POINTS, …)` call per frame.
Tested at 10 million points at 60 fps on a mid-range GPU.

---

## Examples

### 3D scatter with categorical colors

```python
import seraplot as sp
import random

n = 1000
x = [random.gauss(0, 1) for _ in range(n)]
y = [random.gauss(0, 1) for _ in range(n)]
z = [random.gauss(0, 1) for _ in range(n)]
groups = [random.choice(["A", "B", "C"]) for _ in range(n)]

chart = sp.build_scatter3d_chart(
    "3D Point Cloud",
    x_values=x, y_values=y, z_values=z,
    color_labels=groups,
    x_label="X", y_label="Y", z_label="Z",
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<div style="width:100%;overflow:auto;border-radius:8px;margin:12px 0;background:#0d1117">
<!DOCTYPE html><html><head><meta charset="utf-8"><style>body{margin:0;background:#0e1117;padding:16px 0}.c3w{position:relative;display:inline-block;user-select:none;cursor:grab;border-radius:12px;overflow:hidden;box-shadow:0 8px 32px rgba(0,0,0,.5),0 0 0 1px rgba(255,255,255,.06)}.c3w:active{cursor:grabbing}.c3t{position:absolute;z-index:99;pointer-events:none;opacity:0;transition:opacity .15s,transform .15s;transform:translateY(4px) scale(.97);background:rgba(11,14,24,.92);color:#f1f5f9;backdrop-filter:blur(8px);font:12px -apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;border-radius:10px;padding:10px 14px;min-width:140px;box-shadow:0 8px 24px rgba(0,0,0,.5),0 0 0 1px rgba(255,255,255,.08)}.c3t.v{opacity:1;transform:translateY(0) scale(1)}.c3t.p{pointer-events:auto;cursor:default}.c3t b{font-size:13px;display:block;margin-bottom:6px;color:#e2e8f0}.c3t span{color:#64748b;margin-right:6px;font-size:11px}.c3t .tv{color:#f8fafc;font-weight:600}</style><style>.sp-bg{{fill:transparent!important}}.sp-ttl{{fill:#e2e8f0!important}}svg text{{fill:#cbd5e1!important}}.sp-ax-x,.sp-ax-y{{stroke:#475569!important}}.sp-gl{{stroke:#2d3748!important}}.sp-xl,.sp-yl{{fill:#94a3b8!important}}[id^='spp']{{box-shadow:none!important;border-radius:0!important}}</style></head><body><div id="c3d0" class="c3w" style="width:900px;height:560px"><canvas id="c3d0c" style="width:900px;height:560px"></canvas><div id="c3d0t" class="c3t"></div></div><script>(function(){var W=900,H=560,cid='c3d0',M=0,BG='#0e1117';var X=[0.54,-0.32,-1.68,0.23,0.04,-0.95,-0.99,-0.84,0.74,0.87,-1.14,-1.10,1.26,-0.98,-1.03,0.55,0.58,0.97,-0.77,-1.41,-0.30,0.80,0.25,-0.49,-0.86,-0.54,0.04,-0.03,-0.54,1.16,0.29,2.57,0.41,-0.61,0.09,-1.43,0.95,-0.47,0.11,1.64,-0.17,-0.69,0.20,-1.85,-0.71,0.14,0.55,1.26,-1.51,0.60,0.61,0.90,-2.09,0.91,-2.26,-0.35,-0.77,-0.23,-1.19,0.89,0.77,0.87,0.65,-0.29,0.82,-0.23,-0.01,1.53,-0.53,0.17,1.49,0.55,-0.81,1.42,-0.09,-0.34,-0.48,-0.41,0.63,1.01,-1.83,0.80,-0.12,-0.77,-1.28,-0.13,2.24,-0.35,0.60,1.95,0.23,0.24,-0.02,-0.28,-0.74,-0.24,0.27,-0.31,0.21,-1.15,1.06,0.88,-0.44,-0.48,-0.99,1.12,-0.16,-0.40,0.44,0.80,0.96,0.86,-0.45,1.38,0.53,-0.66,0.90,0.10,0.27,-0.36,1.35,-0.79,1.24,-0.30,0.94,-1.28,0.72,0.92,0.20,-0.54,0.60,0.49,-1.39,-1.81,0.04,-1.17,0.29,0.73,0.27,1.47,-1.30,0.53,0.51,0.60,-0.86,1.29,-0.16,-0.38,-0.48,-0.60,0.16,-0.58,-0.94,-0.04,-1.28,1.09,0.60,-1.14,-0.75,-0.87,1.38,-1.93,-1.09,-2.39,-0.28,-0.48,0.96,-0.31,-0.68,-0.81,0.73,0.16,-0.10,-1.70,0.22,0.98,1.58,-1.23,-0.30,-1.03,-0.97,0.74,1.31,-0.88,-0.10,-0.67,-1.50,0.46,-2.24,-0.47,0.65,-1.07,-0.07,-0.31,3.18,-1.91,0.55,-1.47,1.53,0.18,2.12,0.09,-0.69,-2.07,0.18,-0.80,0.77,0.24,0.04,-0.79,0.30,-0.04,0.53,-1.06,-0.56,-0.51,-1.07,0.75,1.08,0.33,-0.92,0.05,-0.66,-0.26,1.03,1.19,1.01,0.61,1.12,-0.64,-1.09,0.31,-1.85,0.11,-1.66,1.12,0.53,-1.35,0.45,-1.01,-0.04,0.45,-1.36,-0.55,-0.48,-1.28,-0.31,0.07,-0.50,0.01,-0.77,-2.33,0.49,2.04,-0.17,-1.27,0.87,1.57,-0.14,-1.24,-0.04,0.16,1.78,1.38,0.37,1.66,-0.04,0.53,-0.62,-0.80,0.19,-0.52,0.21,0.21,0.99,0.47,0.87,-0.73,-1.68,-1.79,-0.19,-0.46,1.10,0.38,0.98,-0.30,-0.55,-0.01,-1.64,-0.81,0.94,0.67,-0.50,-0.15,-0.00,0.90,-1.63,0.69,1.20,-0.20,0.19,-1.47,0.15,1.37,-0.35,-1.28,-0.52,0.89,0.73,1.17,0.74,1.35,-0.38,0.01,1.74,0.17,0.54,-0.26,-1.24,-1.81,2.28,-0.21,-0.36,1.46,-0.18,-1.87,2.84,0.33,1.18,-0.52,2.39,0.71,-0.73,1.54,-0.55,0.18,0.76,0.48,0.20,-0.37,1.05,0.03,0.80,-0.12,0.23,-0.60,0.48,0.55,-0.77,0.46,-0.23,-0.22,-0.90,1.93,-0.37,-0.03,0.10,0.02,0.68,0.74,0.31,0.42,0.76,0.18,1.98,-0.45,-0.95,-1.49,-0.03,-0.79,0.25,-0.03,-0.98,1.02,1.17,-1.34,-0.38,-0.27,-1.22,0.50,0.79,-0.75,0.08,-0.91,0.77,-1.04,1.17,-0.14,0.21,0.69,-0.36,-2.08,0.96,-1.13,0.53,-0.78,0.13,0.70,-1.06,-0.80,-0.86,0.42,-2.38,2.01,1.21,0.60,-0.16,-0.22,-0.45,-0.14,-1.88,-1.46,0.12,-0.40,-1.04,1.42,1.96,0.81,-0.25,1.83,1.37,0.62,0.60,1.10,1.03,-0.36,0.03,-0.08,-0.08,-0.07,-2.08,1.02,-0.89,-0.13,0.16,-0.40,-0.27,1.04,-0.83,3.01,0.50,0.54,0.97,1.33,1.42,1.75,0.76,-0.23,-0.64,1.06,0.46,0.01,-0.94,-1.22,-0.90,0.59,-0.24,-0.59,0.68,0.37,-0.76,0.00,0.58,-1.09,2.22,-0.17,0.60,-0.13,0.57,0.63,-0.02,1.27,1.14,0.08,1.11,-0.38,0.49,0.96,-0.88,0.06,-1.12,1.72,1.72,1.13,0.59,0.55,0.84,0.19,-0.52,0.19,0.59,0.57,-0.08,1.16,0.24,-1.44,0.53,-0.59,-0.60,-0.68,-1.18,-1.01,-0.83,0.79,-0.85,-0.06,-2.71,0.17,0.27,-1.01,-0.98,1.69,-0.99,1.68,-1.93,1.59,0.36,0.30,-1.14,0.35,-0.36,-1.56,-1.78,1.12,-1.88,-1.29,0.36,-0.35,1.41,-0.06,-0.21,0.28,-0.76,-0.52,-0.47,1.20,0.30,1.22,-0.10,-0.01,-0.86,-0.18,1.28,1.51,0.28,0.62,0.94,-1.23,1.03,0.67,-0.09,0.03,-0.98,-0.15,-0.67,-0.75,1.54,-0.66,0.41,-1.33,0.00,0.80,-1.03,0.18,-0.58,-0.50,0.89,-2.39,1.62,1.00,-0.34,-0.01,1.18,-1.97,-0.11,0.70,1.32,0.18,0.62,1.56,-0.13,-1.81,-0.53,0.77,0.80,-0.12,0.71,-0.83,0.68,-0.13,-0.84,-0.37,-0.83,-2.09,1.81,1.77,0.22,-0.23,-0.50,0.41,0.38,0.87,0.44,0.16,-0.65,-0.99,-1.36,-0.82,-0.13,0.05,0.11,1.36,-1.06,-1.34,0.25,1.07,1.73,0.08,1.67,-0.88,0.18,0.95,-0.39,-0.94,-0.74,1.21,-0.17,-0.05,0.06,-0.94,0.19,-1.02,-0.12,1.53,0.20,0.97,0.65,0.65,0.57,0.11,0.01,-0.66,0.14,0.31,0.73,0.16,-1.13,0.59,-0.77,-0.05,0.49,-0.89,0.17,0.16,0.09,1.11,-0.09,0.11,0.52,-1.24,-0.88,-1.45,0.44,-2.00,1.43,0.06,-1.75,-0.47,1.17,-0.17,-0.42,-0.46,1.35,-0.98,0.04,-1.65,-0.53,-0.26,-1.57,1.03,-0.23,2.05,-2.71,0.13,-0.78,0.51,-0.20,1.92,1.37,2.46,2.32,0.68,-0.65,0.18,-0.12,0.20,-0.98,-0.59,1.03,1.09,0.13,0.18,-0.62,-0.63,-1.76,-1.08,-1.24,0.10,-0.90,0.84,0.27,-0.10,1.35,-1.37,0.06,-0.72,-0.80,-0.43,0.78,1.09,-0.56,-1.76,-0.89,-0.05,1.02,-0.57,-0.17,0.59,0.94,2.91,-0.59,-0.19,-0.14,0.49,-0.25,1.95,-0.71,-0.87,-1.11,1.35,-1.13,-0.07,-1.02,1.61,-1.26,-1.40,1.40,0.49,-0.04,-0.74,-1.39,0.20,0.51,1.14,0.27,0.87,-1.18,0.57,-0.50,-0.19,0.53,1.29,-1.14,-0.06,1.26,0.17,-1.89,-2.41,0.46,0.55,1.14,2.22,0.59,0.49,-0.04,0.32,-0.25,-1.09,-1.68,-0.97,0.35,0.83,-0.81,3.20,1.51,-1.94,0.34,0.21,1.50,0.59,0.11,-0.93,2.12,-0.87,-0.43,0.55,-0.77,0.04,1.07,-0.61,1.11,0.92,-1.02,-0.07,-0.80,0.02,-0.47,1.38,0.16,0.12,0.21,-0.48,2.20,0.48,0.39,-2.04,0.34,1.21,-0.48,-0.47,-1.01,-0.01,-0.74,0.32,-0.91,0.35,1.13,-0.62,-0.94,-0.52,-0.11,-1.07,-0.42,-1.25,-0.01,-0.72,-1.04,0.20,0.25,0.70,0.17,0.37,-1.10,0.39,-1.51,-0.74,0.14,-0.82,-0.38,0.21,0.96,1.09,-0.54,0.50,-1.34,-0.26,-0.47,2.31,0.40,-0.59,-0.15,0.11,-0.41,0.85,-1.08,-0.38,0.49,0.37,1.49,-0.21,0.44,0.10,0.16,1.31,-1.03,-0.48,-0.55,-0.21,-0.14,-1.73,0.42,-0.61,1.38,0.50,-0.15,0.69,-0.33,2.17,-0.53,0.29,-1.28,0.25,-1.61,-0.20,-1.67,0.08,-0.34,-1.16,-1.06,1.03,0.78,-0.95,-0.28,-0.10,0.54,0.86,0.36,-0.25,-1.48,1.49,1.37,-0.94,-0.11,1.18,-1.16,2.35,1.88,-0.85,0.11,-0.58,-0.79,-0.19,-0.23,-1.70,-0.95,1.21,-2.08,-0.56,0.73,-0.06,-1.37,-1.29,0.51,-0.85,1.09,3.12,1.12,0.36,-1.42,0.68,0.52,0.21,0.89,2.08,-0.77,-0.30,-0.15,-0.31,0.64,0.38,-0.53,0.62,-0.16,-0.58,0.48,-1.12,0.55,-1.84,0.37,1.48,0.05,-0.92,0.95,-0.74,1.66,1.06,-0.66,-0.28,-0.77,-1.12,1.38,1.52,0.18,1.56,-0.85,-1.47,-1.27,-0.06,0.56,0.10,-0.20,-0.55,-0.96,-1.18,-0.91,-1.54,1.11,-0.19,0.22,-1.19,0.54,-0.01,0.12,1.59,0.67,1.04,-0.82,1.05,0.37,-0.61],Y=[0.31,1.88,2.09,0.05,-0.69,0.87,-1.97,0.19,0.66,0.66,-1.27,-1.52,-0.34,0.60,-0.39,-0.68,-0.56,-1.15,-0.15,0.60,0.12,-0.39,0.83,-0.05,1.03,3.44,-3.00,1.26,0.61,-0.84,-0.78,0.80,-0.39,0.12,-0.78,-1.37,-0.29,-0.30,-0.69,1.19,-1.56,-0.59,0.18,-0.21,-1.84,-1.13,-0.83,1.30,0.02,-0.23,0.43,-0.24,-1.97,-0.24,-1.22,-0.39,-1.78,-1.14,2.01,0.90,1.49,-0.90,-0.78,0.51,0.80,-0.51,0.73,-1.83,-1.06,-0.19,-0.84,0.04,0.58,1.08,0.57,0.22,1.45,-1.25,2.19,1.23,0.81,0.28,0.04,0.35,0.52,-1.74,0.14,-0.84,-0.60,0.17,-1.26,0.56,1.45,-0.05,1.66,0.04,1.29,0.37,-1.35,-0.93,1.84,0.91,-0.73,-0.99,0.04,0.26,0.72,-0.43,1.28,-0.48,0.29,-1.11,0.87,-0.20,0.55,0.84,-1.59,0.90,0.10,0.53,0.25,0.69,-0.55,-0.44,-0.62,2.57,-1.84,-0.12,-0.59,0.13,-0.50,1.51,-0.53,-0.31,0.33,0.07,-0.15,0.48,-1.95,-1.68,0.02,0.41,-1.24,-1.85,-0.13,-0.24,-0.33,-0.62,-0.60,-0.17,0.18,0.42,0.88,-2.67,-0.51,1.12,0.70,-1.45,1.53,-1.33,0.25,-0.18,0.88,-1.05,-1.72,0.69,-1.59,1.62,1.39,2.13,1.20,-0.27,0.28,-1.97,0.31,-0.63,-0.52,-0.20,1.50,-0.27,-0.33,0.08,-0.04,0.19,1.18,0.17,0.67,-0.05,-0.16,1.11,0.01,-0.83,0.35,-0.14,0.74,0.27,-0.05,1.25,0.79,1.42,-1.75,1.16,-0.64,-2.03,-0.26,0.82,-0.54,-1.72,-0.13,-0.87,1.58,-0.03,-0.66,1.00,0.84,0.54,-0.24,1.06,-0.46,0.18,0.69,-2.10,0.17,0.23,1.67,-1.11,-1.58,-1.63,-1.40,-0.93,-1.12,-1.18,-2.41,-0.89,1.41,0.30,1.51,0.34,-0.33,-0.05,1.28,1.88,0.46,1.33,0.26,0.79,-0.84,-0.50,-0.53,0.28,1.50,0.64,1.70,-0.41,2.23,0.61,-0.17,0.03,0.79,0.78,-0.45,-0.13,-0.70,-0.91,-0.26,0.79,-0.72,-0.49,1.37,-0.94,1.29,-0.97,-0.87,1.23,-0.51,-0.10,-0.40,0.70,2.27,-0.06,0.53,0.17,-0.15,-1.28,-0.02,-1.03,-0.38,-1.52,-0.09,-0.44,0.29,1.14,1.87,0.11,1.22,-0.04,-0.39,0.82,2.77,-0.65,-0.39,0.16,-0.50,-0.45,0.62,0.07,0.45,0.20,-0.99,-1.61,1.46,-0.15,-0.26,1.10,-0.65,-0.04,-0.63,0.84,-0.64,0.13,-1.51,0.01,0.13,1.73,1.25,2.13,-1.02,-0.91,-0.70,0.50,1.19,2.54,2.68,-0.07,1.26,0.72,0.67,0.43,1.64,-1.22,0.98,0.26,-0.43,-1.17,-0.05,0.50,0.93,-0.29,-0.40,-0.54,-0.80,-0.80,-0.36,-1.73,2.12,1.08,-0.14,-0.25,0.53,1.60,-1.27,0.69,1.08,1.29,-0.60,-0.86,-1.61,-0.32,2.34,0.63,-0.38,-0.49,-3.00,1.34,-0.02,-0.51,-0.00,-1.40,-0.33,-0.20,0.74,0.04,-0.55,0.51,-0.33,-1.47,2.30,-2.09,-1.26,0.32,-0.08,-0.35,1.14,1.72,0.51,-0.69,-2.28,0.35,-1.68,0.23,1.07,1.65,2.37,0.36,0.90,-1.06,-0.53,-0.32,0.21,-0.51,1.88,-0.44,-1.31,0.93,2.31,-0.45,-0.67,0.69,2.45,0.29,-0.31,0.78,-0.63,1.63,0.54,-1.24,0.07,-1.87,0.19,-0.81,-1.30,-0.65,0.19,1.32,-0.53,0.55,-0.53,0.62,-0.79,-1.45,-0.28,0.96,1.03,-1.23,-0.01,0.27,-1.09,-1.87,-0.11,-0.26,0.60,0.43,0.74,-1.38,-1.51,0.03,-0.60,1.08,0.02,-0.04,-1.44,0.42,1.27,-0.36,0.38,-1.67,-0.60,-1.73,-2.14,-1.10,-0.86,0.22,-1.26,-1.40,1.90,0.26,0.73,-0.90,0.43,1.69,-0.52,0.28,0.11,-0.24,-0.24,-1.78,-1.20,-0.74,2.96,0.45,1.10,0.34,0.25,0.45,0.13,-0.47,-2.41,0.84,-0.29,0.32,0.61,0.73,0.02,-1.25,0.32,-0.24,-0.39,0.29,1.69,0.33,-0.12,0.66,1.67,0.15,1.71,-0.37,0.53,-0.82,-0.10,1.46,2.51,-0.14,-0.09,1.39,1.10,0.45,-0.33,-0.41,-0.61,1.12,1.34,1.00,1.53,-1.28,-0.14,0.33,0.38,-0.25,-0.41,-1.29,-0.11,-0.57,-1.66,1.33,1.09,0.45,0.92,-0.53,0.32,1.55,-0.50,-0.69,0.93,0.04,-0.41,-0.86,-0.56,-1.75,0.00,-0.71,-1.20,0.23,0.68,0.08,-2.53,-0.23,1.69,0.35,1.63,0.07,0.28,0.45,0.28,-0.44,0.23,0.37,-1.52,0.34,-2.14,2.20,-0.22,-0.50,0.14,1.66,1.69,-1.19,0.05,-1.39,2.10,-0.28,1.14,-0.44,0.19,-0.87,-1.38,-1.38,0.70,2.69,-2.39,0.84,0.74,-0.36,1.19,-0.82,-0.74,1.96,0.19,2.38,-0.53,-0.97,-1.04,-1.06,-0.38,0.75,-1.16,0.75,0.76,-0.04,0.02,-0.31,1.09,0.18,1.08,-0.69,-2.08,1.29,1.30,0.10,0.52,0.47,1.49,-1.30,-0.34,0.46,-1.57,0.03,-1.97,-1.08,-0.39,0.03,-0.30,-0.71,-1.50,0.78,0.10,-2.66,-0.45,1.86,-0.15,0.27,0.31,-0.77,0.81,-0.12,1.26,-0.95,-0.21,0.74,-0.67,-0.24,-0.45,-0.74,-0.43,-0.66,0.64,-1.34,-0.42,-1.72,-0.96,-0.47,1.44,-1.22,-1.22,-0.31,1.08,0.34,1.17,-0.35,2.18,1.28,-0.02,0.28,-0.98,-0.58,0.87,0.44,-0.15,-1.00,0.83,0.86,-0.09,-1.20,0.97,0.07,1.27,0.19,0.50,0.19,-0.99,0.32,-0.83,-1.03,2.38,-0.55,-1.12,0.40,1.85,0.22,1.44,-0.20,-0.82,-2.24,0.52,-0.08,0.25,-0.55,1.17,-0.51,-0.65,-0.07,1.20,-2.04,-1.00,0.53,0.46,-1.32,-0.92,0.69,-0.75,-1.26,-0.44,-0.04,-1.51,1.37,-0.59,2.22,-0.56,-0.52,0.79,1.98,-1.40,-1.32,-0.68,0.33,0.31,-1.12,0.42,0.61,0.19,-0.23,0.34,0.76,0.36,1.04,0.55,1.02,-0.84,-0.57,-1.06,-1.29,-0.09,0.33,1.51,1.18,-0.54,-0.86,-0.23,-1.16,-0.52,1.11,-1.73,-0.13,0.36,1.34,-0.31,0.70,0.67,-0.55,0.66,1.84,1.20,-1.78,0.85,0.64,0.43,1.07,-0.28,0.59,1.64,-0.81,2.28,0.54,0.51,-0.86,1.25,0.83,0.38,0.21,1.63,1.61,0.30,0.23,-0.88,-0.91,0.33,-0.34,-0.74,1.07,0.48,0.57,-0.01,-0.38,-0.61,2.43,-0.98,1.38,0.18,-1.12,0.16,1.36,-1.02,-2.40,-0.08,-0.64,-1.35,-1.83,-0.41,0.82,-0.45,-1.61,-1.18,-2.02,-1.39,0.23,1.00,0.74,-1.45,-0.02,-1.36,0.75,-0.95,1.70,1.20,-2.34,-0.31,0.36,0.54,-0.39,0.90,-0.87,-0.61,-0.27,-0.04,-0.86,1.87,0.03,-0.81,2.73,-0.26,1.46,-1.04,0.06,-1.04,0.16,0.13,-0.68,0.31,-1.82,0.06,-0.43,-0.05,-0.89,0.60,-1.09,-2.01,0.51,-0.47,-0.41,-1.08,1.43,-0.80,1.00,-1.59,-0.07,1.62,1.94,-0.56,0.56,-0.61,-1.50,1.13,-0.89,1.56,-0.87,-1.59,1.44,-0.57,0.92,-0.75,0.15,0.71,-1.86,0.00,-0.45,-0.29,1.19,0.75,0.50,-0.33,-1.99,0.55,-0.84,1.70,-0.70,0.08,1.00,0.52,-0.47,-0.72,1.06,-1.26,0.10,-0.74,-0.22,-0.53,-0.73,0.16,2.38,0.25,1.12,-1.62,-0.39,0.98,-1.73,-0.21,-0.28,1.01,1.27,-1.78,-0.74,-1.35,0.20,0.49,-0.21,-0.73,-0.34,-0.03,-1.46,-0.70,0.84,0.05,0.10,-1.41,0.29,0.41,-0.33,0.03,-0.42,0.85,0.22,-0.39,-0.75,-1.37,-0.45,1.10,-0.02,-0.05,0.37,0.96,-0.29,-0.83,0.72,1.36,0.82,0.16,0.85,2.09,1.54,2.32,-0.62,2.12,-0.46,1.64,0.11,0.54,1.46,-1.26,0.73,-0.15,1.00,-1.15,0.74,0.65,-1.18,-0.53,2.06,-0.09,0.47,-1.16,0.60],Z=[-0.69,0.95,1.21,0.09,0.77,-0.48,0.89,-1.68,0.40,0.16,2.25,0.18,-0.80,-0.80,1.79,1.85,0.64,0.24,1.62,0.28,-0.97,-0.57,0.73,-0.28,1.08,-2.09,-0.49,1.85,1.79,0.07,0.08,-0.78,-0.68,0.28,-1.15,1.03,-0.11,0.69,1.27,-1.68,0.59,1.01,-0.43,-1.07,-0.01,1.45,-0.81,1.77,0.82,2.20,1.27,2.17,-0.01,-0.26,0.51,-0.08,-0.31,0.18,-1.02,3.21,0.68,0.38,2.78,-1.47,-0.80,0.19,0.78,0.74,-0.80,0.80,0.76,-1.17,-0.53,0.03,0.45,-0.64,-0.97,1.03,0.55,-3.35,0.82,-1.09,-0.39,-0.38,-0.25,1.51,2.29,0.08,-1.32,1.23,-0.35,-0.61,0.95,1.25,1.17,-1.11,-0.39,0.44,1.30,-0.59,-1.34,1.12,-0.66,0.04,-0.35,0.28,-0.32,0.03,1.88,-0.18,-0.48,0.40,0.75,-0.06,0.69,2.17,-0.94,-1.41,-1.33,-1.41,-0.84,-0.17,-0.20,1.68,0.26,1.21,-0.81,-0.99,0.52,1.53,0.11,-0.20,0.92,2.69,-0.83,-0.61,0.62,-0.31,2.08,0.81,-1.28,0.09,0.19,0.29,-1.22,-0.08,1.65,0.99,-0.96,-0.01,-0.78,-1.29,0.09,0.75,-0.14,-0.20,0.53,0.17,-0.83,0.32,-1.11,-0.71,-0.82,-0.50,0.00,-1.12,1.47,0.96,0.40,0.46,-0.92,0.83,-0.01,-0.56,-1.36,0.64,-0.29,-0.42,1.50,0.18,-0.59,-1.35,0.78,-1.23,1.51,-1.86,0.08,0.30,0.60,1.71,-0.63,-0.72,0.16,-0.51,-2.15,0.00,0.00,0.56,1.11,-0.72,-0.43,-0.89,-0.29,0.11,-0.45,-0.19,-0.48,-2.95,-1.14,0.90,0.80,-0.49,0.70,1.50,0.86,0.44,0.50,-0.63,0.78,0.50,0.49,1.39,0.40,-0.51,0.84,-0.62,-0.73,0.50,-0.29,-1.20,-1.77,1.58,-0.10,-0.16,-0.39,-1.21,0.62,-1.95,-0.48,0.71,0.82,1.94,2.65,-1.05,-0.01,0.80,0.89,1.06,-2.07,-0.02,0.96,0.85,1.65,-1.50,0.66,-1.21,-0.09,0.45,0.57,0.23,0.83,-0.09,-2.27,-2.42,0.28,-1.07,0.83,1.45,-0.72,-1.78,0.37,0.77,-0.58,-1.85,-0.07,0.82,-1.12,-1.29,-1.16,0.39,0.03,-0.04,-0.17,-0.47,0.29,1.67,-0.03,0.49,1.05,-0.68,0.11,-0.63,0.15,-1.61,-1.01,0.56,0.01,2.10,0.12,1.85,-1.19,0.24,0.53,-1.71,-0.47,0.60,-0.55,0.14,-0.15,-0.35,-0.89,1.66,-0.78,0.31,-1.05,0.49,0.06,0.26,0.35,-1.83,0.68,1.66,0.35,-1.39,0.83,1.58,1.17,-0.86,-0.50,-0.22,-0.17,0.71,-0.35,0.49,1.69,-1.27,0.33,0.29,-0.17,-1.57,-0.91,-0.91,0.52,-0.10,-1.92,0.24,-1.47,-0.77,0.49,-1.13,1.45,-0.13,0.96,0.22,-0.62,0.30,0.28,-2.50,1.10,-0.05,1.00,0.73,-0.67,1.05,0.04,1.03,-0.86,-1.85,-1.20,1.32,0.47,-0.40,0.06,-0.17,1.32,-0.76,-0.84,0.24,2.29,-0.57,-1.08,0.53,0.06,0.10,0.47,-1.67,-0.24,1.47,1.03,0.70,1.11,2.58,1.15,0.17,-2.58,-0.94,-0.11,-0.46,-0.64,0.08,1.22,0.41,-0.44,-1.48,-0.06,0.78,-0.75,0.30,-0.39,-1.41,-1.59,1.26,0.93,-1.64,0.83,0.54,1.17,0.79,0.10,-0.79,0.17,1.22,0.76,1.21,-0.82,-0.08,0.67,-0.37,0.08,0.48,0.84,1.85,0.85,1.33,-0.54,-0.47,0.34,-0.94,0.36,-1.11,1.44,1.26,-0.10,-0.10,0.11,0.63,-0.08,-1.37,0.59,0.62,-0.45,-2.57,-0.44,-0.69,-1.27,-2.95,0.17,-0.30,-0.27,2.05,-0.86,1.09,-0.69,-0.16,-0.34,-1.07,-0.00,1.15,0.15,-0.02,0.06,-2.06,-0.29,-1.12,-1.84,-0.44,0.71,0.49,-0.34,0.82,-0.72,-3.03,0.40,0.33,-1.42,-0.54,-0.78,-1.40,-0.87,1.24,0.07,1.02,-0.33,0.64,-1.00,-0.45,0.39,-0.24,0.74,-0.55,0.62,-0.89,-0.77,1.02,-0.45,-0.96,0.45,-0.42,0.50,0.37,-0.24,1.11,-0.03,0.15,0.08,-0.49,-3.52,0.61,0.14,0.94,-0.70,-1.27,0.30,-0.74,-0.04,0.30,-0.85,0.02,-0.78,-0.42,0.31,-0.42,0.09,-1.18,-1.20,1.71,-1.24,-0.83,-0.12,0.29,-0.02,0.32,1.09,-0.24,-1.16,1.86,0.10,-1.19,-0.11,-0.28,-0.78,1.31,1.19,-0.76,-1.54,-1.18,-0.48,-0.86,0.68,0.63,1.53,0.40,0.61,0.00,-0.57,-1.04,-0.26,-0.12,-0.30,1.00,0.49,-0.20,-0.92,-1.12,0.43,-0.42,-0.77,1.06,0.13,1.25,0.27,0.24,0.05,-0.32,1.27,-0.59,0.89,0.50,-0.05,1.22,-1.04,0.58,0.72,0.50,0.18,0.92,0.44,0.51,0.62,0.04,-1.33,-0.72,-1.40,1.66,0.83,-1.02,-1.10,-0.19,2.37,1.81,1.91,-0.53,-0.83,-0.43,1.30,0.20,-1.31,0.21,0.28,-0.07,-0.40,-1.24,-0.09,-0.23,-1.50,0.83,-1.09,-0.51,0.47,0.75,-0.02,0.01,1.39,0.75,-1.21,-1.34,-0.35,1.64,0.52,0.13,0.03,-0.60,-1.09,0.87,0.25,0.01,-1.35,-0.21,-0.44,0.00,-0.67,0.62,0.48,-0.49,1.42,0.15,-0.52,1.32,-1.66,2.05,1.92,0.27,0.72,1.53,-0.24,1.31,1.84,0.46,-1.27,-0.92,-0.27,1.25,-0.60,-0.02,-0.66,-0.64,0.66,0.32,0.38,0.15,0.95,0.03,-0.41,2.04,0.16,-0.03,-0.03,-1.00,0.86,-0.23,0.08,0.34,-0.26,1.29,0.15,2.07,-0.63,1.71,-0.03,0.19,1.50,0.74,-1.34,1.52,2.36,-1.30,-0.58,1.21,-0.11,0.95,-0.61,0.97,-0.33,-1.09,-0.39,-0.09,-0.41,0.29,-1.67,-1.19,0.29,0.91,-0.52,0.75,-0.94,-0.88,-0.23,-0.62,-0.25,-0.42,-0.64,1.06,-0.04,-0.95,1.31,1.88,-0.44,0.53,1.90,-1.25,0.37,-1.68,-1.84,1.03,1.01,1.03,0.82,-0.88,-1.41,-1.09,1.05,0.39,-1.07,-0.19,1.37,0.08,0.41,1.34,1.30,0.57,1.40,1.93,-1.47,-1.91,0.33,2.37,1.21,-1.62,-1.38,-1.47,0.00,0.46,0.49,0.69,2.49,0.93,-0.23,-0.27,-2.00,0.31,-0.01,-0.21,-0.90,-0.14,0.29,-0.19,0.28,-1.97,0.80,-1.83,0.00,0.21,-0.55,0.54,-0.45,0.80,-0.32,-0.91,0.67,-1.54,0.36,-0.59,0.26,0.03,-0.25,0.31,0.15,-0.85,0.51,-0.15,-2.22,-0.15,-0.38,1.14,-2.13,-0.66,-1.21,0.86,-0.09,0.11,1.09,0.85,0.84,-0.62,0.56,1.08,-0.95,0.76,0.80,0.03,0.73,0.94,-1.18,1.99,-0.57,0.67,-0.76,0.11,1.02,0.88,1.26,-0.11,0.37,0.64,0.45,0.47,0.89,0.42,0.32,1.39,0.72,0.81,0.40,-0.74,0.73,1.86,0.33,0.38,0.24,0.54,-0.63,-0.18,-1.21,-1.15,0.45,-0.87,0.72,0.43,0.08,1.25,-0.67,-0.73,-0.46,-0.18,0.01,0.67,0.52,-0.47,0.60,-2.43,-1.01,-0.72,-1.30,2.13,0.59,-0.90,0.49,0.31,-0.59,0.98,-0.54,0.29,-0.06,-0.01,-0.22,-0.84,2.45,-0.21,-1.26,1.40,-0.32,-0.50,0.65,0.40,0.75,-0.45,0.99,0.87,-0.50,1.19,0.35,-1.64,-0.28,1.21,0.45,0.40,1.59,0.93,-0.34,0.40,-1.70,0.03,-0.06,0.89,0.36,0.62,0.81,0.79,-0.04,1.83,0.53,-0.44,-0.19,1.96,0.97,0.32,-0.68,0.84,-0.05,0.30,-0.23,0.99,0.25,0.74,0.19,-0.11,0.83,0.45,0.39,0.27,0.89,-0.57,-0.22,-1.31,-0.05,-1.01,-0.13,-0.56,-2.07,-0.92,0.97,-0.76,2.33,0.26,-1.33,-0.56,-0.37,-0.01,-0.83,-0.66,0.27,0.23,-1.82,0.02,-1.38,2.04,-0.04,0.42,0.64,-1.52,0.92,1.31,-1.83,1.21,-1.45,-0.56,1.45,-0.30,1.10,0.38,0.87,0.70,0.70,-1.38,0.25,1.33,-0.69,-0.48,-1.09,-1.77],C=[];var PAL=['#6366f1','#f43f5e','#10b981','#f59e0b','#8b5cf6','#06b6d4','#ec4899','#84cc16','#ef4444','#14b8a6'];var CL=['A','C','A','C','B','C','C','B','A','A','A','C','B','B','A','C','B','A','A','A','B','C','A','B','B','B','A','C','B','C','C','C','C','B','C','B','A','C','A','C','A','C','A','B','B','C','B','B','C','B','C','C','B','B','C','B','B','B','C','A','A','B','B','C','C','A','B','B','A','A','B','B','C','A','A','A','B','B','C','B','B','B','B','C','C','C','B','B','A','A','B','B','B','C','C','A','A','C','C','A','A','C','C','A','A','C','A','A','B','A','A','B','C','A','A','C','A','C','A','A','B','B','A','B','A','C','A','A','A','A','A','B','C','B','A','A','B','B','C','B','A','C','A','C','A','A','C','C','C','A','B','A','B','B','A','C','B','C','A','A','A','A','A','C','A','B','A','A','C','A','A','B','A','B','A','B','B','A','A','C','A','A','A','A','C','A','A','B','B','B','B','C','C','B','A','A','B','A','B','A','A','A','C','A','C','C','B','A','B','B','B','A','C','A','C','C','B','B','A','B','B','A','B','A','C','C','B','C','B','C','A','C','B','B','A','A','C','B','C','C','C','C','B','B','B','B','B','A','C','A','C','C','A','A','C','B','C','C','C','B','A','B','A','C','C','C','A','B','C','C','B','A','A','A','B','A','C','B','B','C','A','B','A','A','A','C','B','A','A','B','C','A','C','A','C','A','B','B','C','A','C','C','C','B','B','A','C','C','C','C','A','B','C','B','A','C','B','B','A','C','C','C','B','C','C','C','A','C','C','B','A','B','C','A','B','C','C','B','C','B','C','B','B','C','A','B','A','B','C','B','A','C','A','C','A','C','A','C','B','C','C','B','C','C','C','B','B','B','B','B','C','A','B','B','B','B','C','C','C','A','C','B','A','A','B','A','C','A','A','A','B','C','B','B','C','C','C','C','A','B','B','B','A','B','A','B','C','A','B','C','C','C','C','B','B','C','B','A','C','A','C','C','B','A','A','C','C','B','A','B','A','A','B','B','B','A','A','A','A','C','B','C','C','B','B','A','B','C','C','B','A','A','B','B','A','B','C','C','A','C','A','C','C','C','B','B','A','B','C','C','A','C','C','C','A','A','B','A','A','C','B','B','C','A','C','A','B','A','C','B','C','C','A','A','A','C','A','B','B','C','C','A','C','C','A','C','A','A','B','B','B','C','C','A','B','B','A','B','C','C','C','B','B','B','A','C','B','C','B','C','A','C','C','C','A','A','C','C','C','A','A','B','B','C','A','B','A','C','A','C','C','B','C','A','B','B','B','B','A','A','A','C','B','B','B','C','B','B','A','C','C','C','C','C','C','B','B','B','A','A','C','C','A','A','C','A','C','B','C','C','C','A','C','C','C','B','A','A','B','C','C','A','A','C','B','B','B','C','C','B','C','C','B','A','B','B','C','A','A','C','B','C','B','B','B','C','B','C','B','C','A','A','A','A','C','B','C','B','B','A','A','C','A','A','A','B','B','C','A','B','C','B','A','A','B','A','B','A','C','B','A','B','A','C','B','B','B','C','C','B','B','B','B','A','C','A','C','C','A','C','A','B','A','C','A','C','B','B','A','B','B','A','A','A','B','B','A','B','A','A','C','A','A','C','B','B','C','B','C','B','C','A','A','B','B','A','B','A','C','C','B','B','B','A','A','B','A','C','B','C','C','B','B','A','C','B','A','C','B','C','B','A','A','C','B','C','C','A','C','C','C','B','C','B','B','A','B','B','B','B','B','C','C','B','B','C','A','A','A','C','B','C','C','B','C','B','A','B','A','A','C','B','A','A','B','A','B','C','A','A','C','C','B','B','A','A','B','C','B','A','A','A','A','B','C','C','B','B','A','C','C','B','A','B','B','C','C','B','B','A','C','A','C','A','B','C','A','B','C','A','C','A','A','C','B','B','C','C','C','B','B','C','B','A','C','A','B','C','B','B','B','A','A','C','C','A','C','B','B','C','A','B','C','B','B','C','C','B','C','A','A','B','A','A','B','A','C','B','B','C','C','C','B','A','C','C','B','B','C','C','C','C','A','B','C','C','A','B','B','A','B','B','C','A','B','A','A','A','A','A','B','A','B','A','A','C','C','B','B','A','A','A','B','B','C','B','B','C','C','C','A','B','C','C','A','C','C','B','B','A','C','C','B','A','C','C','B','A','C','A','B','C','C','B','C','B','B','B','A','B','B','C','B','C','C','C','A','B','A','C','A','C','B','A','A','A','C','C','A','A','C','A','B','A','A','C','A','A','A','B','C','B','A','A','A','B','C','A','A','A'];var xl='X',yl='Y',zl='Z',ttl='3D Point Cloud';
var N=X.length,uc=C.length>=N;
var xmn=1e18,xmx=-1e18,ymn=1e18,ymx=-1e18,zmn=1e18,zmx=-1e18;
for(var i=0;i<N;i++){if(X[i]<xmn)xmn=X[i];if(X[i]>xmx)xmx=X[i];if(Y[i]<ymn)ymn=Y[i];if(Y[i]>ymx)ymx=Y[i];if(Z[i]<zmn)zmn=Z[i];if(Z[i]>zmx)zmx=Z[i];}
var xr=xmx-xmn||1,yr=ymx-ymn||1,zr=zmx-zmn||1;
var yaw=0.785,pitch=0.6,zoom=1.0,TAU=6.2832,fov=0.8;
var dg=false,lx=0,ly=0,mv=false,dwX=0,dwY=0,raf=0;
var cv=document.getElementById(cid+'c'),g=cv.getContext('2d'),wrap=document.getElementById(cid),tip=document.getElementById(cid+'t');
var dpr=window.devicePixelRatio||1;cv.width=W*dpr;cv.height=H*dpr;g.scale(dpr,dpr);
var pin=false,piI=-1,pp=[];
var AX='#f472b6',AY='#22d3ee',AZ='#fbbf24';
var autoR=false,velY=0,velP=0,panX=0,panY=0,keys={};
var fric=0.95,kSpd=0.03;
var _rc={};
var _glcv=null,_gl=null,_glP=null,_glB=null;
function hx2rgb(h){if(_rc[h])return _rc[h];var r=[parseInt(h.slice(1,3),16),parseInt(h.slice(3,5),16),parseInt(h.slice(5,7),16)];_rc[h]=r;return r;}
function pj(px,py,pz){var ex=zoom*Math.cos(yaw)*Math.cos(pitch),ey=zoom*Math.sin(yaw)*Math.cos(pitch),ez=zoom*Math.sin(pitch);var fx=-ex,fy=-ey,fz=-ez,fl=Math.sqrt(fx*fx+fy*fy+fz*fz);fx/=fl;fy/=fl;fz/=fl;var rx=-fy,ry=fx,rz=0,rl=Math.sqrt(rx*rx+ry*ry)||1e-6;rx/=rl;ry/=rl;var u2x=fy*rz-fz*ry,u2y=fz*rx-fx*rz,u2z=fx*ry-fy*rx;var dx=px-ex,dy=py-ey,dz=pz-ez;var dp=dx*fx+dy*fy+dz*fz;if(dp<0.001)return null;var cx2=dx*rx+dy*ry+dz*rz,cy2=dx*u2x+dy*u2y+dz*u2z;var th=Math.tan(fov/2),asp=W/H;return{x:cx2/(dp*th*asp)+panX/sc,y:cy2/(dp*th)+panY/sc,d:dp};}
var cV=[[-0.5,-0.5,-0.5],[0.5,-0.5,-0.5],[0.5,0.5,-0.5],[-0.5,0.5,-0.5],[-0.5,-0.5,0.5],[0.5,-0.5,0.5],[0.5,0.5,0.5],[-0.5,0.5,0.5]];
var cE=[[0,1],[1,2],[2,3],[3,0],[4,5],[5,6],[6,7],[7,4],[0,4],[1,5],[2,6],[3,7]];
var isDark=BG==='transparent'||BG==='#0e1117'||(BG.charAt(0)==='#'&&parseInt(BG.slice(1,3),16)<60);
function drawBG(){
  g.clearRect(0,0,W,H);
  if(BG==='transparent')return;
  if(isDark){
    var gr=g.createRadialGradient(W*0.5,H*0.42,0,W*0.5,H*0.5,Math.max(W,H)*0.75);
    gr.addColorStop(0,'#131c2e');gr.addColorStop(0.55,'#0d1117');gr.addColorStop(1,'#060810');
    g.fillStyle=gr;g.fillRect(0,0,W,H);
    var n1=g.createRadialGradient(W*0.1,H*0.88,0,W*0.1,H*0.88,W*0.48);
    n1.addColorStop(0,'rgba(99,102,241,0.04)');n1.addColorStop(1,'rgba(0,0,0,0)');
    g.fillStyle=n1;g.fillRect(0,0,W,H);
    var n2=g.createRadialGradient(W*0.9,H*0.1,0,W*0.9,H*0.1,W*0.4);
    n2.addColorStop(0,'rgba(20,184,166,0.03)');n2.addColorStop(1,'rgba(0,0,0,0)');
    g.fillStyle=n2;g.fillRect(0,0,W,H);
  } else {
    g.fillStyle=BG;g.fillRect(0,0,W,H);
  }
}
function drawFloor(mx,my,sc){
  for(var q=0;q<=10;q++){var f=q/10-0.5;
    var p0=pj(f,-0.5,-0.5),p1=pj(f,0.5,-0.5);
    if(p0&&p1){var al=(q%5===0)?0.09:0.04;g.strokeStyle=isDark?'rgba(99,102,241,'+al+')':'rgba(0,0,0,'+(al*0.7)+')';g.lineWidth=q%5===0?0.8:0.4;g.beginPath();g.moveTo(mx+p0.x*sc,my-p0.y*sc);g.lineTo(mx+p1.x*sc,my-p1.y*sc);g.stroke();}
    var q0=pj(-0.5,f,-0.5),q1=pj(0.5,f,-0.5);
    if(q0&&q1){var al2=(q%5===0)?0.09:0.04;g.strokeStyle=isDark?'rgba(99,102,241,'+al2+')':'rgba(0,0,0,'+(al2*0.7)+')';g.lineWidth=q%5===0?0.8:0.4;g.beginPath();g.moveTo(mx+q0.x*sc,my-q0.y*sc);g.lineTo(mx+q1.x*sc,my-q1.y*sc);g.stroke();}
  }
  var c0=pj(-0.5,-0.5,-0.5),c1=pj(0.5,-0.5,-0.5),c2=pj(0.5,0.5,-0.5),c3=pj(-0.5,0.5,-0.5);
  if(c0&&c1&&c2&&c3){
    g.beginPath();g.moveTo(mx+c0.x*sc,my-c0.y*sc);g.lineTo(mx+c1.x*sc,my-c1.y*sc);g.lineTo(mx+c2.x*sc,my-c2.y*sc);g.lineTo(mx+c3.x*sc,my-c3.y*sc);g.closePath();
    var fg=g.createRadialGradient(mx,my,0,mx,my,sc*0.6);
    fg.addColorStop(0,isDark?'rgba(99,102,241,0.04)':'rgba(0,0,0,0.02)');
    fg.addColorStop(1,'rgba(0,0,0,0)');
    g.fillStyle=fg;g.fill();
  }
}
function arw(p0,p1,col){
  if(!p0||!p1)return;
  var sx0=mx+p0.x*sc,sy0=my-p0.y*sc,sx1=mx+p1.x*sc,sy1=my-p1.y*sc;
  g.strokeStyle=col;g.lineWidth=1.1;g.globalAlpha=0.55;
  g.beginPath();g.moveTo(sx0,sy0);g.lineTo(sx1,sy1);g.stroke();
  var an=Math.atan2(sy1-sy0,sx1-sx0),al=7;
  g.fillStyle=col;g.globalAlpha=0.7;g.beginPath();
  g.moveTo(sx1,sy1);g.lineTo(sx1-al*Math.cos(an-0.42),sy1-al*Math.sin(an-0.42));
  g.lineTo(sx1-al*Math.cos(an+0.42),sy1-al*Math.sin(an+0.42));
  g.closePath();g.fill();g.globalAlpha=1;
}
var mx,my,sc;
function rBb(mx,my,sc){
  var su=typeof S!=='undefined';
  var pts=[];
  for(var i=0;i<N;i++){
    var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;
    var p=pj(nx,ny,nz);if(!p)continue;
    var sz=su&&S[i]!==undefined?0.3+S[i]*0.7:0.5;
    pts.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d,ci:uc?C[i]%PAL.length:i%PAL.length,i:i,sz:sz});
  }
  pts.sort(function(a,b){return a.d-b.d;});
  var dlo=pts.length?pts[0].d:1,dhi=pts.length?pts[pts.length-1].d:2,dr=dhi-dlo||1;
  var selSx=0,selSy=0,selR=0,selCol='';
  for(var j=0;j<pts.length;j++){
    var p=pts[j],dn=(p.d-dlo)/dr;
    var r=Math.max(6,Math.min(32,p.sz*28*(1-dn*0.3)));
    var col=PAL[p.ci],rgb=hx2rgb(col);
    var lr2=Math.min(255,rgb[0]+80),lg2=Math.min(255,rgb[1]+80),lb2=Math.min(255,rgb[2]+80);
    g.globalAlpha=isDark?0.15:0.1;
    g.fillStyle='rgba(0,0,0,0.4)';g.beginPath();g.ellipse(p.sx+2,p.sy+r*0.5,r*1.3,r*0.35,0,0,TAU);g.fill();
    g.globalAlpha=1;
    var cg=g.createRadialGradient(p.sx-r*0.35,p.sy-r*0.35,r*0.05,p.sx,p.sy,r);
    cg.addColorStop(0,'rgba(255,255,255,0.92)');
    cg.addColorStop(0.15,'rgb('+lr2+','+lg2+','+lb2+')');
    cg.addColorStop(0.5,col);
    cg.addColorStop(0.85,'rgba('+Math.max(0,rgb[0]-40)+','+Math.max(0,rgb[1]-40)+','+Math.max(0,rgb[2]-40)+',0.9)');
    cg.addColorStop(1,'rgba('+Math.max(0,rgb[0]-70)+','+Math.max(0,rgb[1]-70)+','+Math.max(0,rgb[2]-70)+',0.6)');
    g.fillStyle=cg;g.beginPath();g.arc(p.sx,p.sy,r,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.6)';g.beginPath();g.arc(p.sx-r*0.3,p.sy-r*0.35,r*0.2,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.25)';g.beginPath();g.arc(p.sx-r*0.15,p.sy-r*0.2,r*0.35,0,TAU);g.fill();
    pp.push({sx:p.sx,sy:p.sy,i:p.i,r:r});
    if(p.i===piI){selSx=p.sx;selSy=p.sy;selR=r;selCol=col;}
  }
  if(piI>=0&&selCol)drawHalo(selSx,selSy,selR,selCol);
}
function R(){
  drawBG();
  mx=W/2;my=H/2;sc=Math.min(W,H)*0.34;
  if(ttl){
    g.textAlign='center';g.textBaseline='top';
    g.font='600 15px -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif';
    if(isDark){
      var tg=g.createLinearGradient(W/2-120,0,W/2+120,0);
      tg.addColorStop(0,'#818cf8');tg.addColorStop(0.5,'#e2e8f0');tg.addColorStop(1,'#38bdf8');
      g.fillStyle=tg;
    } else { g.fillStyle='#1a202c'; }
    g.fillText(ttl,W/2,10);
  }
  drawFloor(mx,my,sc);
  var noCube=M===7||M===12||M===13||M===15;
  if(!noCube){
  g.save();
  for(var e=0;e<12;e++){
    var a=cV[cE[e][0]],b=cV[cE[e][1]];
    var pa=pj(a[0],a[1],a[2]),pb=pj(b[0],b[1],b[2]);
    if(!pa||!pb)continue;
    var avgD=(pa.d+pb.d)/2;
    var al2=isDark?Math.max(0.05,0.26-avgD*0.07):Math.max(0.04,0.13-avgD*0.02);
    g.strokeStyle=isDark?'rgba(148,163,184,'+al2+')':'rgba(0,0,0,'+al2+')';
    g.lineWidth=0.7;g.beginPath();g.moveTo(mx+pa.x*sc,my-pa.y*sc);g.lineTo(mx+pb.x*sc,my-pb.y*sc);g.stroke();
  }
  g.restore();
  var or=pj(-0.5,-0.5,-0.5);
  arw(or,pj(0.56,-0.5,-0.5),AX);arw(or,pj(-0.5,0.56,-0.5),AY);arw(or,pj(-0.5,-0.5,0.56),AZ);
  g.font='8.5px -apple-system,sans-serif';
  for(var k=0;k<=4;k++){var f=k/4;
    var ax0=pj(f-0.5,-0.5,-0.5);if(ax0){g.textAlign='center';g.textBaseline='top';g.fillStyle=isDark?'rgba(244,114,182,0.55)':'#9ca3af';g.fillText((xmn+xr*f).toFixed(1),mx+ax0.x*sc,my-ax0.y*sc+5);}
    var ay0=pj(0.5,f-0.5,-0.5);if(ay0){g.textAlign='center';g.textBaseline='top';g.fillStyle=isDark?'rgba(34,211,238,0.55)':'#9ca3af';g.fillText((ymn+yr*f).toFixed(1),mx+ay0.x*sc,my-ay0.y*sc+5);}
    var az0=pj(0.5,-0.5,f-0.5);if(az0){g.textAlign='left';g.textBaseline='middle';g.fillStyle=isDark?'rgba(251,191,36,0.55)':'#9ca3af';g.fillText((zmn+zr*f).toFixed(1),mx+az0.x*sc+6,my-az0.y*sc);}
  }
  g.font='700 10.5px -apple-system,sans-serif';
  var lp=pj(0,-0.5,-0.5);if(lp){g.textAlign='center';g.textBaseline='top';g.fillStyle=AX;g.fillText(xl,mx+lp.x*sc,my-lp.y*sc+17);}
  lp=pj(0.5,0,-0.5);if(lp){g.textAlign='center';g.textBaseline='top';g.fillStyle=AY;g.fillText(yl,mx+lp.x*sc,my-lp.y*sc+17);}
  lp=pj(-0.5,-0.5,0);if(lp){g.textAlign='right';g.textBaseline='middle';g.fillStyle=AZ;g.fillText(zl,mx+lp.x*sc-10,my-lp.y*sc);}
  }
  pp=[];
  if(M===0)rSgl(mx,my,sc);else if(M===1)rB(mx,my,sc);else if(M===2)rL(mx,my,sc);else if(M===3)rRdr(mx,my,sc);else if(M===4)rLol(mx,my,sc);else if(M===5)rKde(mx,my,sc);else if(M===6)rRdg(mx,my,sc);else if(M===7)rPie(mx,my,sc);else if(M===8)rVio(mx,my,sc);else if(M===9)rHm(mx,my,sc);else if(M===10)rCd(mx,my,sc);else if(M===11)rDu(mx,my,sc);else if(M===12)rFn(mx,my,sc);else if(M===13)rSb(mx,my,sc);else if(M===14)rStk(mx,my,sc);else if(M===15)rGlb(mx,my,sc);else if(M===16)rBb(mx,my,sc);
  drawLgd();
  g.font='9.5px -apple-system,sans-serif';g.fillStyle=isDark?'rgba(100,116,139,0.4)':'rgba(0,0,0,0.18)';
  g.textAlign='center';g.textBaseline='bottom';
  g.fillText('drag \xb7 scroll \xb7 dblclick reset \xb7 WASD/arrows \xb7 Q/E zoom \xb7 Space=auto \xb7 shift+drag=pan',W/2,H-5);
  if(autoR){g.fillStyle='#6366F1';g.globalAlpha=0.55;g.fillText('\u27f3 auto',W-45,H-5);g.globalAlpha=1;}
}
function drawLgd(){
  if(CL.length===0)return;
  var lx2=W-150,ly2=36,lh=CL.length*22+16,lw=140;
  g.save();
  g.fillStyle=isDark?'rgba(14,18,30,0.88)':'rgba(249,250,251,0.94)';
  var lr=8;g.beginPath();
  g.moveTo(lx2-6+lr,ly2-6);g.arcTo(lx2-6+lw,ly2-6,lx2-6+lw,ly2-6+lh,lr);
  g.arcTo(lx2-6+lw,ly2-6+lh,lx2-6,ly2-6+lh,lr);g.arcTo(lx2-6,ly2-6+lh,lx2-6,ly2-6,lr);
  g.arcTo(lx2-6,ly2-6,lx2-6+lw,ly2-6,lr);g.closePath();g.fill();
  g.strokeStyle=isDark?'rgba(255,255,255,0.07)':'rgba(0,0,0,0.06)';g.lineWidth=1;g.stroke();
  g.restore();
  g.font='11px -apple-system,sans-serif';g.textBaseline='middle';g.textAlign='left';
  for(var li=0;li<CL.length;li++){
    var cy2=ly2+7+li*22,col=PAL[li%PAL.length];
    g.fillStyle=col;g.beginPath();g.arc(lx2+9,cy2,5,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.5)';g.beginPath();g.arc(lx2+7.5,cy2-1.5,1.8,0,TAU);g.fill();
    g.fillStyle=isDark?'#e2e8f0':'#374151';g.fillText(CL[li],lx2+20,cy2);
  }
}
function drawHalo(sx,sy,r,col){
  var rgb=hx2rgb(col);
  var og=g.createRadialGradient(sx,sy,r*0.8,sx,sy,r*3.2);
  og.addColorStop(0,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.35)');
  og.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0)');
  g.fillStyle=og;g.beginPath();g.arc(sx,sy,r*3.2,0,TAU);g.fill();
  g.strokeStyle='rgba(255,255,255,0.75)';g.lineWidth=1.5;
  g.beginPath();g.arc(sx,sy,r+2.5,0,TAU);g.stroke();
}
function rS(mx,my,sc){
  var pts=[];
  for(var i=0;i<N;i++){
    var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;
    var p=pj(nx,ny,nz);if(!p)continue;
    pts.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d,ci:uc?C[i]%PAL.length:i%PAL.length,i:i,nz:nz});
  }
  pts.sort(function(a,b){return a.d-b.d;});
  var dlo=pts.length?pts[0].d:1,dhi=pts.length?pts[pts.length-1].d:2,dr=dhi-dlo||1;
  var selSx=0,selSy=0,selR=0,selCol='';
  for(var j=0;j<pts.length;j++){
    var p=pts[j],dn=(p.d-dlo)/dr,r=Math.max(4,Math.min(12,10-dn*6));
    var col=PAL[p.ci],rgb=hx2rgb(col);
    var lr2=Math.min(255,rgb[0]+70),lg2=Math.min(255,rgb[1]+70),lb2=Math.min(255,rgb[2]+70);
    g.globalAlpha=isDark?0.12:0.08;
    g.fillStyle='rgba(0,0,0,0.5)';g.beginPath();g.ellipse(p.sx+1.5,p.sy+r*0.4,r*1.2,r*0.3,0,0,TAU);g.fill();
    g.globalAlpha=1;
    var cg=g.createRadialGradient(p.sx-r*0.3,p.sy-r*0.3,0,p.sx,p.sy,r);
    cg.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');
    cg.addColorStop(0.5,col);
    cg.addColorStop(1,'rgba('+Math.max(0,rgb[0]-30)+','+Math.max(0,rgb[1]-30)+','+Math.max(0,rgb[2]-30)+',0.8)');
    g.fillStyle=cg;g.beginPath();g.arc(p.sx,p.sy,r,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.55)';g.beginPath();g.arc(p.sx-r*0.28,p.sy-r*0.28,r*0.25,0,TAU);g.fill();
    g.strokeStyle='rgba(255,255,255,0.1)';g.lineWidth=0.5;g.beginPath();g.arc(p.sx,p.sy,r,0,TAU);g.stroke();
    pp.push({sx:p.sx,sy:p.sy,i:p.i,r:r});
    if(p.i===piI){selSx=p.sx;selSy=p.sy;selR=r;selCol=col;}
  }
  if(piI>=0&&selCol)drawHalo(selSx,selSy,selR,selCol);
}
function _iGL(){
  if(_glcv)return _gl!=null;
  _glcv=document.createElement('canvas');
  _glcv.width=W*dpr;_glcv.height=H*dpr;
  var gl=_glcv.getContext('webgl',{alpha:true,antialias:false,premultipliedAlpha:false})||_glcv.getContext('experimental-webgl',{alpha:true,premultipliedAlpha:false});
  if(!gl)return false;
  var vs=gl.createShader(gl.VERTEX_SHADER);
  gl.shaderSource(vs,'attribute vec3 aP,aC;uniform vec3 uE,uF,uR,uU;uniform float uT,uA,uSX,uSY,uPS;uniform vec2 uPN;varying vec3 vC;void main(){vec3 d=aP-uE;float dp=dot(d,uF);if(dp<0.001){gl_Position=vec4(0,0,9,1);gl_PointSize=1.;vC=aC;return;}float cx=dot(d,uR);float cy=dot(d,uU);float nx=(cx/(dp*uT*uA)+uPN.x)*uSX;float ny=(cy/(dp*uT)+uPN.y)*uSY;gl_Position=vec4(nx,ny,0.,1.);gl_PointSize=clamp(uPS/dp,1.5,20.);vC=aC;}');
  gl.compileShader(vs);
  var fs=gl.createShader(gl.FRAGMENT_SHADER);
  gl.shaderSource(fs,'precision mediump float;varying vec3 vC;void main(){vec2 c=gl_PointCoord-0.5;float d=length(c);float edge=smoothstep(0.5,0.42,d);if(edge<0.001)discard;float h=max(0.,1.-d*2.);gl_FragColor=vec4(vC+vec3(0.22*h*h),edge*(0.88+0.12*h));}');
  gl.compileShader(fs);
  var prog=gl.createProgram();gl.attachShader(prog,vs);gl.attachShader(prog,fs);gl.linkProgram(prog);
  if(!gl.getProgramParameter(prog,gl.LINK_STATUS))return false;
  _gl=gl;_glP=prog;
  var pos=new Float32Array(N*3),col=new Float32Array(N*3);
  for(var i=0;i<N;i++){
    pos[i*3]=(X[i]-xmn)/xr-0.5;pos[i*3+1]=(Y[i]-ymn)/yr-0.5;pos[i*3+2]=(Z[i]-zmn)/zr-0.5;
    var ci=(uc?C[i]:i)%PAL.length,rgb=hx2rgb(PAL[ci]);
    col[i*3]=rgb[0]/255;col[i*3+1]=rgb[1]/255;col[i*3+2]=rgb[2]/255;
  }
  var pb=gl.createBuffer();gl.bindBuffer(gl.ARRAY_BUFFER,pb);gl.bufferData(gl.ARRAY_BUFFER,pos,gl.STATIC_DRAW);
  var cb=gl.createBuffer();gl.bindBuffer(gl.ARRAY_BUFFER,cb);gl.bufferData(gl.ARRAY_BUFFER,col,gl.STATIC_DRAW);
  _glB={pb:pb,cb:cb,aP:gl.getAttribLocation(prog,'aP'),aC:gl.getAttribLocation(prog,'aC'),
    uE:gl.getUniformLocation(prog,'uE'),uF:gl.getUniformLocation(prog,'uF'),
    uR:gl.getUniformLocation(prog,'uR'),uU:gl.getUniformLocation(prog,'uU'),
    uT:gl.getUniformLocation(prog,'uT'),uA:gl.getUniformLocation(prog,'uA'),
    uSX:gl.getUniformLocation(prog,'uSX'),uSY:gl.getUniformLocation(prog,'uSY'),
    uPS:gl.getUniformLocation(prog,'uPS'),uPN:gl.getUniformLocation(prog,'uPN')};
  return true;
}
function rSgl(mx,my,sc){
  if(!_iGL()){rS(mx,my,sc);return;}
  var gl=_gl,b=_glB;
  gl.viewport(0,0,W*dpr,H*dpr);gl.clearColor(0,0,0,0);gl.clear(gl.COLOR_BUFFER_BIT);
  gl.enable(gl.BLEND);gl.blendFunc(gl.SRC_ALPHA,gl.ONE_MINUS_SRC_ALPHA);
  gl.useProgram(_glP);
  var ex=zoom*Math.cos(yaw)*Math.cos(pitch),ey=zoom*Math.sin(yaw)*Math.cos(pitch),ez=zoom*Math.sin(pitch);
  var fl=Math.sqrt(ex*ex+ey*ey+ez*ez)||1;
  var fx=-ex/fl,fy=-ey/fl,fz=-ez/fl;
  var rx=-fy,ry=fx,rz=0,rl=Math.sqrt(rx*rx+ry*ry)||1e-6;rx/=rl;ry/=rl;
  var ux=fy*rz-fz*ry,uy=fz*rx-fx*rz,uz=fx*ry-fy*rx;
  gl.uniform3f(b.uE,ex,ey,ez);gl.uniform3f(b.uF,fx,fy,fz);
  gl.uniform3f(b.uR,rx,ry,0);gl.uniform3f(b.uU,ux,uy,uz);
  gl.uniform1f(b.uT,Math.tan(fov/2));gl.uniform1f(b.uA,W/H);
  gl.uniform1f(b.uSX,2*sc/W);gl.uniform1f(b.uSY,2*sc/H);
  gl.uniform1f(b.uPS,6*dpr);gl.uniform2f(b.uPN,panX/sc,panY/sc);
  gl.bindBuffer(gl.ARRAY_BUFFER,b.pb);gl.enableVertexAttribArray(b.aP);gl.vertexAttribPointer(b.aP,3,gl.FLOAT,false,0,0);
  gl.bindBuffer(gl.ARRAY_BUFFER,b.cb);gl.enableVertexAttribArray(b.aC);gl.vertexAttribPointer(b.aC,3,gl.FLOAT,false,0,0);
  gl.drawArrays(gl.POINTS,0,N);
  g.save();g.globalCompositeOperation='source-over';g.drawImage(_glcv,0,0,W,H);g.restore();
}
function rB(mx,my,sc){
  var bars=[];
  for(var i=0;i<N;i++){
    var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;
    var pb=pj(nx,ny,-0.5),pt=pj(nx,ny,nz);
    if(!pb||!pt)continue;
    bars.push({bx:mx+pb.x*sc,by:my-pb.y*sc,tx:mx+pt.x*sc,ty:my-pt.y*sc,d:pt.d,ci:uc?C[i]%PAL.length:i%PAL.length,i:i,nz:nz});
  }
  bars.sort(function(a,b){return a.d-b.d;});
  var dlo=bars.length?bars[0].d:1,dhi=bars.length?bars[bars.length-1].d:2,dr=dhi-dlo||1;
  var selB=null;
  for(var j=0;j<bars.length;j++){
    var b=bars[j],col=PAL[b.ci],rgb=hx2rgb(col);
    var dn=(b.d-dlo)/dr,bw=Math.max(4,Math.min(12,10-dn*4));
    g.globalAlpha=isDark?0.18:0.1;
    g.fillStyle=col;g.beginPath();g.ellipse(b.bx,b.by,bw*2,bw*0.6,0,0,TAU);g.fill();
    g.globalAlpha=1;
    var lr2=Math.min(255,rgb[0]+50),lg2=Math.min(255,rgb[1]+50),lb2=Math.min(255,rgb[2]+50);
    var dr2=Math.max(0,rgb[0]-30),dg2=Math.max(0,rgb[1]-30),db2=Math.max(0,rgb[2]-30);
    var offX=bw*0.3,offY=0;
    g.strokeStyle='rgba('+dr2+','+dg2+','+db2+',0.5)';g.lineWidth=bw+2;g.lineCap='round';
    g.beginPath();g.moveTo(b.bx+offX,b.by+offY);g.lineTo(b.tx+offX,b.ty+offY);g.stroke();
    var sg=g.createLinearGradient(b.bx-bw,b.by,b.tx+bw,b.ty);
    sg.addColorStop(0,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.4)');
    sg.addColorStop(0.4,col);
    sg.addColorStop(0.8,'rgb('+lr2+','+lg2+','+lb2+')');
    sg.addColorStop(1,'rgb('+Math.min(255,rgb[0]+80)+','+Math.min(255,rgb[1]+80)+','+Math.min(255,rgb[2]+80)+')');
    g.strokeStyle=sg;g.lineWidth=bw;g.lineCap='round';
    g.beginPath();g.moveTo(b.bx,b.by);g.lineTo(b.tx,b.ty);g.stroke();
    g.fillStyle='rgb('+lr2+','+lg2+','+lb2+')';g.beginPath();g.arc(b.tx,b.ty,bw*0.6,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.65)';g.beginPath();g.arc(b.tx-bw*0.2,b.ty-bw*0.2,bw*0.3,0,TAU);g.fill();
    pp.push({sx:b.tx,sy:b.ty,i:b.i,r:bw+2});
    if(b.i===piI)selB=b;
  }
  if(piI>=0&&selB)drawHalo(selB.tx,selB.ty,4,PAL[selB.ci]);
}
function rL(mx,my,sc){
  var pts=[];
  for(var i=0;i<N;i++){
    var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;
    var p=pj(nx,ny,nz);if(!p)continue;
    pts.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d,ci:uc?C[i]%PAL.length:0,i:i});
  }
  if(pts.length<2){
    if(pts.length===1)pp.push({sx:pts[0].sx,sy:pts[0].sy,i:pts[0].i,r:5});
    return;
  }
  for(var pass=0;pass<2;pass++){
    var lw2=pass===0?5:2,al=pass===0?0.18:1;
    g.save();g.globalAlpha=al;g.lineJoin='round';g.lineCap='round';g.lineWidth=lw2;
    for(var j=1;j<pts.length;j++){
      var p0=pts[j-1],p1=pts[j];
      var r0=hx2rgb(PAL[p0.ci]),r1=hx2rgb(PAL[p1.ci]);
      var bo=pass===1?45:0;
      var lg2=g.createLinearGradient(p0.sx,p0.sy,p1.sx,p1.sy);
      lg2.addColorStop(0,'rgba('+Math.min(255,r0[0]+bo)+','+Math.min(255,r0[1]+bo)+','+Math.min(255,r0[2]+bo)+',1)');
      lg2.addColorStop(1,'rgba('+Math.min(255,r1[0]+bo)+','+Math.min(255,r1[1]+bo)+','+Math.min(255,r1[2]+bo)+',1)');
      g.strokeStyle=lg2;g.beginPath();g.moveTo(p0.sx,p0.sy);g.lineTo(p1.sx,p1.sy);g.stroke();
    }
    g.restore();
  }
  var selP=null;
  for(var j=0;j<pts.length;j++){
    var p=pts[j],col=PAL[p.ci],rgb=hx2rgb(col),r=4.5;
    var lr2=Math.min(255,rgb[0]+70),lg3=Math.min(255,rgb[1]+70),lb2=Math.min(255,rgb[2]+70);
    g.fillStyle='rgb('+lr2+','+lg3+','+lb2+')';g.beginPath();g.arc(p.sx,p.sy,r,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.52)';g.beginPath();g.arc(p.sx-r*0.28,p.sy-r*0.28,r*0.28,0,TAU);g.fill();
    pp.push({sx:p.sx,sy:p.sy,i:p.i,r:r});
    if(p.i===piI){selP=p;}
  }
  if(piI>=0&&selP)drawHalo(selP.sx,selP.sy,4.5,PAL[selP.ci]);
}
function rRdr(mx,my,sc){
  var groups={};
  for(var i=0;i<N;i++){var ci=uc?C[i]%PAL.length:0;if(!groups[ci])groups[ci]=[];groups[ci].push(i);}
  var gkeys=Object.keys(groups).sort(function(a,b){return parseInt(a)-parseInt(b);});
  for(var gi=0;gi<gkeys.length;gi++){
    var ci=parseInt(gkeys[gi]),idxs=groups[ci];
    idxs.sort(function(a,b){return Math.atan2(Z[a],X[a])-Math.atan2(Z[b],X[b]);});
    var col=PAL[ci%PAL.length],rgb=hx2rgb(col);
    var pts2d=[],bpts=[];
    for(var k=0;k<idxs.length;k++){
      var ii=idxs[k];
      var nx=(X[ii]-xmn)/xr-0.5,ny=(Y[ii]-ymn)/yr-0.5,nz=(Z[ii]-zmn)/zr-0.5;
      var p=pj(nx,ny,nz);if(p)pts2d.push({sx:mx+p.x*sc,sy:my-p.y*sc,d:p.d,ii:ii});
      var pb=pj(nx,-0.5,nz);if(pb)bpts.push({sx:mx+pb.x*sc,sy:my-pb.y*sc});
    }
    if(pts2d.length<3)continue;
    if(bpts.length===pts2d.length){
      g.globalAlpha=0.06;g.beginPath();g.moveTo(bpts[0].sx,bpts[0].sy);
      for(var k=1;k<bpts.length;k++)g.lineTo(bpts[k].sx,bpts[k].sy);
      g.closePath();g.fillStyle=col;g.fill();g.globalAlpha=1;
      for(var k=0;k<pts2d.length;k++){
        g.strokeStyle='rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.18)';g.lineWidth=1;
        g.beginPath();g.moveTo(bpts[k].sx,bpts[k].sy);g.lineTo(pts2d[k].sx,pts2d[k].sy);g.stroke();
      }
    }
    var lr2=Math.min(255,rgb[0]+40),lg2=Math.min(255,rgb[1]+40),lb2=Math.min(255,rgb[2]+40);
    g.beginPath();g.moveTo(pts2d[0].sx,pts2d[0].sy);
    for(var k=1;k<pts2d.length;k++)g.lineTo(pts2d[k].sx,pts2d[k].sy);
    g.closePath();
    var minY2=1e9,maxY2=-1e9;for(var k=0;k<pts2d.length;k++){if(pts2d[k].sy<minY2)minY2=pts2d[k].sy;if(pts2d[k].sy>maxY2)maxY2=pts2d[k].sy;}
    var fg=g.createLinearGradient(0,minY2,0,maxY2);
    fg.addColorStop(0,'rgba('+lr2+','+lg2+','+lb2+',0.38)');
    fg.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.12)');
    g.fillStyle=fg;g.fill();
    g.save();g.globalAlpha=0.15;g.strokeStyle=col;g.lineWidth=8;g.stroke();g.restore();
    g.strokeStyle=col;g.lineWidth=2.5;g.stroke();
    g.strokeStyle='rgba('+Math.min(255,rgb[0]+80)+','+Math.min(255,rgb[1]+80)+','+Math.min(255,rgb[2]+80)+',0.5)';
    g.lineWidth=1;g.stroke();
    for(var k=0;k<pts2d.length;k++){
      var sr=5;
      var cg=g.createRadialGradient(pts2d[k].sx-sr*0.25,pts2d[k].sy-sr*0.25,0,pts2d[k].sx,pts2d[k].sy,sr);
      cg.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');cg.addColorStop(0.7,col);
      cg.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.6)');
      g.fillStyle=cg;g.beginPath();g.arc(pts2d[k].sx,pts2d[k].sy,sr,0,TAU);g.fill();
      g.fillStyle='rgba(255,255,255,0.5)';g.beginPath();g.arc(pts2d[k].sx-sr*0.25,pts2d[k].sy-sr*0.25,sr*0.25,0,TAU);g.fill();
      pp.push({sx:pts2d[k].sx,sy:pts2d[k].sy,i:idxs[k],r:sr+2});
    }
  }
}
function rLol(mx,my,sc){
  var lols=[];
  for(var i=0;i<N;i++){
    var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;
    var pb=pj(nx,ny,-0.5),pt=pj(nx,ny,nz);
    if(!pb||!pt)continue;
    lols.push({bx:mx+pb.x*sc,by:my-pb.y*sc,tx:mx+pt.x*sc,ty:my-pt.y*sc,d:pt.d,ci:uc?C[i]%PAL.length:i%PAL.length,i:i});
  }
  lols.sort(function(a,b){return a.d-b.d;});
  var selL=null;
  for(var j=0;j<lols.length;j++){
    var l=lols[j],col=PAL[l.ci],rgb=hx2rgb(col);
    var lr2=Math.min(255,rgb[0]+60),lg2=Math.min(255,rgb[1]+60),lb2=Math.min(255,rgb[2]+60);
    g.strokeStyle='rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.2)';g.lineWidth=4;g.lineCap='round';
    g.beginPath();g.moveTo(l.bx,l.by);g.lineTo(l.tx,l.ty);g.stroke();
    g.strokeStyle=col;g.lineWidth=1.8;g.lineCap='round';
    g.beginPath();g.moveTo(l.bx,l.by);g.lineTo(l.tx,l.ty);g.stroke();
    var sr=10;
    var cg=g.createRadialGradient(l.tx-sr*0.3,l.ty-sr*0.3,0,l.tx,l.ty,sr);
    cg.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');cg.addColorStop(0.65,col);cg.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.5)');
    g.fillStyle=cg;g.beginPath();g.arc(l.tx,l.ty,sr,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.55)';g.beginPath();g.arc(l.tx-sr*0.3,l.ty-sr*0.3,sr*0.3,0,TAU);g.fill();
    pp.push({sx:l.tx,sy:l.ty,i:l.i,r:sr+2});
    if(l.i===piI)selL=l;
  }
  if(piI>=0&&selL)drawHalo(selL.tx,selL.ty,8,PAL[selL.ci]);
}
function rKde(mx,my,sc){
  var ymap={};
  for(var i=0;i<N;i++){var yk=Math.round(Y[i]*1000)/1000;if(!ymap[yk])ymap[yk]=[];ymap[yk].push(i);}
  var ykeys=Object.keys(ymap).sort(function(a,b){return parseFloat(a)-parseFloat(b);});
  var dThick=Math.min(0.28,0.75/Math.max(ykeys.length-1,1));
  var strips=[];
  for(var gi=0;gi<ykeys.length;gi++){
    var idxs=ymap[ykeys[gi]];idxs.sort(function(a,b){return X[a]-X[b];});
    var col=PAL[gi%PAL.length],rgb=hx2rgb(col);
    var rawPts=[];
    for(var k=0;k<idxs.length;k++){
      var ii=idxs[k];
      var nx=(X[ii]-xmn)/xr-0.5,ny=(Y[ii]-ymn)/yr-0.5,nz=(Z[ii]-zmn)/zr-0.5;
      rawPts.push({nx:nx,ny:ny,nz:nz});
    }
    if(rawPts.length<2)continue;
    var mp=pj(rawPts[Math.floor(rawPts.length/2)].nx,rawPts[Math.floor(rawPts.length/2)].ny,0);
    strips.push({raw:rawPts,col:col,rgb:rgb,gi:gi,avgD:mp?mp.d:gi});
  }
  strips.sort(function(a,b){return a.avgD-b.avgD;});
  var nLay=20;
  for(var si=0;si<strips.length;si++){
    var s=strips[si],raw=s.raw,col=s.col,rgb=s.rgb;
    var dr2=Math.max(0,rgb[0]-75),dg2=Math.max(0,rgb[1]-75),db2=Math.max(0,rgb[2]-75);
    var lr2=Math.min(255,rgb[0]+90),lg2=Math.min(255,rgb[1]+90),lb2=Math.min(255,rgb[2]+90);
    for(var li=0;li<nLay;li++){
      var frac=li/(nLay-1);
      var nyOff=dThick*(frac-0.5);
      var pts=[],bpts=[];
      for(var k=0;k<raw.length;k++){
        var p=pj(raw[k].nx,raw[k].ny+nyOff,raw[k].nz);
        var pb=pj(raw[k].nx,raw[k].ny+nyOff,-0.5);
        if(p)pts.push({sx:mx+p.x*sc,sy:my-p.y*sc});
        if(pb)bpts.push({sx:mx+pb.x*sc,sy:my-pb.y*sc});
      }
      if(pts.length<2||bpts.length<2)continue;
      g.beginPath();g.moveTo(bpts[0].sx,bpts[0].sy);
      for(var k=1;k<bpts.length;k++)g.lineTo(bpts[k].sx,bpts[k].sy);
      for(var k=pts.length-1;k>=0;k--)g.lineTo(pts[k].sx,pts[k].sy);
      g.closePath();
      var sA=0.18+0.82*frac;
      var cr=Math.round(dr2+(rgb[0]-dr2)*sA),cg=Math.round(dg2+(rgb[1]-dg2)*sA),cb=Math.round(db2+(rgb[2]-db2)*sA);
      g.fillStyle='rgba('+cr+','+cg+','+cb+','+(0.48+0.45*frac)+')';g.fill();
    }
    var topFront=[],topBack=[];
    for(var k=0;k<raw.length;k++){
      var pF=pj(raw[k].nx,raw[k].ny+dThick*0.5,raw[k].nz);
      var pB=pj(raw[k].nx,raw[k].ny-dThick*0.5,raw[k].nz);
      if(pF)topFront.push({sx:mx+pF.x*sc,sy:my-pF.y*sc});
      if(pB)topBack.push({sx:mx+pB.x*sc,sy:my-pB.y*sc});
    }
    if(topFront.length>=2&&topBack.length>=2){
      g.beginPath();g.moveTo(topFront[0].sx,topFront[0].sy);
      for(var k=1;k<topFront.length;k++)g.lineTo(topFront[k].sx,topFront[k].sy);
      for(var k=topBack.length-1;k>=0;k--)g.lineTo(topBack[k].sx,topBack[k].sy);
      g.closePath();
      var sg=g.createLinearGradient(topFront[0].sx,topFront[0].sy,topBack[0].sx,topBack[0].sy);
      sg.addColorStop(0,'rgba('+lr2+','+lg2+','+lb2+',0.95)');
      sg.addColorStop(1,'rgba('+Math.round(rgb[0]*0.6)+','+Math.round(rgb[1]*0.6)+','+Math.round(rgb[2]*0.6)+',0.55)');
      g.fillStyle=sg;g.fill();
    }
    var frontPts=[];
    for(var k=0;k<raw.length;k++){
      var p=pj(raw[k].nx,raw[k].ny+dThick*0.5,raw[k].nz);
      if(p)frontPts.push({sx:mx+p.x*sc,sy:my-p.y*sc});
    }
    if(frontPts.length>=2){
      g.save();g.globalAlpha=0.12;g.strokeStyle=col;g.lineWidth=10;g.lineCap='round';g.lineJoin='round';
      g.beginPath();g.moveTo(frontPts[0].sx,frontPts[0].sy);
      for(var k=1;k<frontPts.length;k++)g.lineTo(frontPts[k].sx,frontPts[k].sy);g.stroke();g.restore();
      g.strokeStyle=col;g.lineWidth=2.5;g.lineCap='round';g.lineJoin='round';
      g.beginPath();g.moveTo(frontPts[0].sx,frontPts[0].sy);
      for(var k=1;k<frontPts.length;k++)g.lineTo(frontPts[k].sx,frontPts[k].sy);g.stroke();
      g.strokeStyle='rgba('+lr2+','+lg2+','+lb2+',0.68)';g.lineWidth=1;
      g.beginPath();g.moveTo(frontPts[0].sx,frontPts[0].sy);
      for(var k=1;k<frontPts.length;k++)g.lineTo(frontPts[k].sx,frontPts[k].sy);g.stroke();
      var mi=Math.floor(frontPts.length/2);
      pp.push({sx:frontPts[mi].sx,sy:frontPts[mi].sy,i:s.gi,r:12});
    }
  }
}
function rRdg(mx,my,sc){
  var ymap={};
  for(var i=0;i<N;i++){var yk=Math.round(Y[i]*1000)/1000;if(!ymap[yk])ymap[yk]=[];ymap[yk].push(i);}
  var ykeys=Object.keys(ymap).sort(function(a,b){return parseFloat(a)-parseFloat(b);});
  var dThick=Math.min(0.32,0.88/Math.max(ykeys.length-1,1));
  var strips=[];
  for(var gi=0;gi<ykeys.length;gi++){
    var idxs=ymap[ykeys[gi]];idxs.sort(function(a,b){return X[a]-X[b];});
    var col=PAL[gi%PAL.length],rgb=hx2rgb(col);
    var rawPts=[];
    for(var k=0;k<idxs.length;k++){
      var ii=idxs[k];
      var nx=(X[ii]-xmn)/xr-0.5,ny=(Y[ii]-ymn)/yr-0.5,nz=(Z[ii]-zmn)/zr-0.5;
      rawPts.push({nx:nx,ny:ny,nz:nz});
    }
    if(rawPts.length<2)continue;
    var mp=pj(rawPts[Math.floor(rawPts.length/2)].nx,rawPts[Math.floor(rawPts.length/2)].ny-dThick*0.4,0);
    strips.push({raw:rawPts,col:col,rgb:rgb,gi:gi,avgD:mp?mp.d:gi});
  }
  strips.sort(function(a,b){return a.avgD-b.avgD;});
  var nLay=22;
  for(var si=0;si<strips.length;si++){
    var s=strips[si],raw=s.raw,col=s.col,rgb=s.rgb;
    var dr2=Math.max(0,rgb[0]-80),dg2=Math.max(0,rgb[1]-80),db2=Math.max(0,rgb[2]-80);
    var lr2=Math.min(255,rgb[0]+90),lg2=Math.min(255,rgb[1]+90),lb2=Math.min(255,rgb[2]+90);
    for(var li=0;li<nLay;li++){
      var frac=li/(nLay-1);
      var nyOff=dThick*(frac-0.5);
      var pts=[],bpts=[];
      for(var k=0;k<raw.length;k++){
        var p=pj(raw[k].nx,raw[k].ny+nyOff,raw[k].nz);
        var pb=pj(raw[k].nx,raw[k].ny+nyOff,-0.5);
        if(p)pts.push({sx:mx+p.x*sc,sy:my-p.y*sc});
        if(pb)bpts.push({sx:mx+pb.x*sc,sy:my-pb.y*sc});
      }
      if(pts.length<2||bpts.length<2)continue;
      g.beginPath();g.moveTo(bpts[0].sx,bpts[0].sy);
      for(var k=1;k<bpts.length;k++)g.lineTo(bpts[k].sx,bpts[k].sy);
      for(var k=pts.length-1;k>=0;k--)g.lineTo(pts[k].sx,pts[k].sy);
      g.closePath();
      var sA=0.12+0.88*frac;
      var cr=Math.round(dr2+(rgb[0]-dr2)*sA),cg=Math.round(dg2+(rgb[1]-dg2)*sA),cb=Math.round(db2+(rgb[2]-db2)*sA);
      g.fillStyle='rgba('+cr+','+cg+','+cb+',0.93)';g.fill();
    }
    var topFront=[],topBack=[];
    for(var k=0;k<raw.length;k++){
      var pF=pj(raw[k].nx,raw[k].ny+dThick*0.5,raw[k].nz);
      var pB=pj(raw[k].nx,raw[k].ny-dThick*0.5,raw[k].nz);
      if(pF)topFront.push({sx:mx+pF.x*sc,sy:my-pF.y*sc});
      if(pB)topBack.push({sx:mx+pB.x*sc,sy:my-pB.y*sc});
    }
    if(topFront.length>=2&&topBack.length>=2){
      g.beginPath();g.moveTo(topFront[0].sx,topFront[0].sy);
      for(var k=1;k<topFront.length;k++)g.lineTo(topFront[k].sx,topFront[k].sy);
      for(var k=topBack.length-1;k>=0;k--)g.lineTo(topBack[k].sx,topBack[k].sy);
      g.closePath();
      var sg=g.createLinearGradient(topFront[0].sx,topFront[0].sy,topBack[0].sx,topBack[0].sy);
      sg.addColorStop(0,'rgba('+lr2+','+lg2+','+lb2+',0.97)');
      sg.addColorStop(0.55,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.82)');
      sg.addColorStop(1,'rgba('+Math.round(rgb[0]*0.5)+','+Math.round(rgb[1]*0.5)+','+Math.round(rgb[2]*0.5)+',0.5)');
      g.fillStyle=sg;g.fill();
    }
    var lA=pj(raw[0].nx,raw[0].ny+dThick*0.5,raw[0].nz),lB=pj(raw[0].nx,raw[0].ny-dThick*0.5,raw[0].nz);
    var lC=pj(raw[0].nx,raw[0].ny-dThick*0.5,-0.5),lD=pj(raw[0].nx,raw[0].ny+dThick*0.5,-0.5);
    if(lA&&lB&&lC&&lD){
      g.beginPath();g.moveTo(mx+lA.x*sc,my-lA.y*sc);g.lineTo(mx+lB.x*sc,my-lB.y*sc);g.lineTo(mx+lC.x*sc,my-lC.y*sc);g.lineTo(mx+lD.x*sc,my-lD.y*sc);g.closePath();
      g.fillStyle='rgba('+dr2+','+dg2+','+db2+',0.68)';g.fill();
    }
    var rA=pj(raw[raw.length-1].nx,raw[raw.length-1].ny+dThick*0.5,raw[raw.length-1].nz);
    var rB=pj(raw[raw.length-1].nx,raw[raw.length-1].ny-dThick*0.5,raw[raw.length-1].nz);
    var rC=pj(raw[raw.length-1].nx,raw[raw.length-1].ny-dThick*0.5,-0.5);
    var rD=pj(raw[raw.length-1].nx,raw[raw.length-1].ny+dThick*0.5,-0.5);
    if(rA&&rB&&rC&&rD){
      g.beginPath();g.moveTo(mx+rA.x*sc,my-rA.y*sc);g.lineTo(mx+rB.x*sc,my-rB.y*sc);g.lineTo(mx+rC.x*sc,my-rC.y*sc);g.lineTo(mx+rD.x*sc,my-rD.y*sc);g.closePath();
      g.fillStyle='rgba('+Math.round((dr2+rgb[0])/2)+','+Math.round((dg2+rgb[1])/2)+','+Math.round((db2+rgb[2])/2)+',0.55)';g.fill();
    }
    var frontPts=[];
    for(var k=0;k<raw.length;k++){
      var p=pj(raw[k].nx,raw[k].ny+dThick*0.5,raw[k].nz);
      if(p)frontPts.push({sx:mx+p.x*sc,sy:my-p.y*sc});
    }
    if(frontPts.length>=2){
      g.save();g.globalAlpha=0.14;g.strokeStyle=col;g.lineWidth=14;g.lineCap='round';g.lineJoin='round';
      g.beginPath();g.moveTo(frontPts[0].sx,frontPts[0].sy);
      for(var k=1;k<frontPts.length;k++)g.lineTo(frontPts[k].sx,frontPts[k].sy);g.stroke();g.restore();
      g.strokeStyle=col;g.lineWidth=3;g.lineCap='round';g.lineJoin='round';
      g.beginPath();g.moveTo(frontPts[0].sx,frontPts[0].sy);
      for(var k=1;k<frontPts.length;k++)g.lineTo(frontPts[k].sx,frontPts[k].sy);g.stroke();
      g.strokeStyle='rgba('+lr2+','+lg2+','+lb2+',0.72)';g.lineWidth=1.2;
      g.beginPath();g.moveTo(frontPts[0].sx,frontPts[0].sy);
      for(var k=1;k<frontPts.length;k++)g.lineTo(frontPts[k].sx,frontPts[k].sy);g.stroke();
      var mi=Math.floor(frontPts.length/2);
      pp.push({sx:frontPts[mi].sx,sy:frontPts[mi].sy,i:s.gi,r:12});
    }
  }
}
function rPie(mx,my,sc){
  var total=0;for(var i=0;i<N;i++)total+=Z[i];if(total<=0)return;
  var slices=[],si=0;
  for(var i=0;i<N;i++){var ci=uc?C[i]%PAL.length:i%PAL.length;slices.push({v:Z[i],ci:ci,i:i,a0:0,a1:0});}
  var ca=0;for(var i=0;i<slices.length;i++){slices[i].a0=ca;ca+=slices[i].v/total*TAU;slices[i].a1=ca;}
  var th=0.35,rd=Math.min(W,H)*0.28;
  var tiltF=Math.cos(pitch)*0.85+0.15;
  var sinP=Math.sin(pitch);
  var layers=20,depthPx=th*sc*0.6;
  var extr=[];
  for(var i=0;i<slices.length;i++){
    var s=slices[i],mid=(s.a0+s.a1)/2;
    var frontness=-Math.sin(mid+yaw)*sinP+Math.cos(mid)*0.001;
    extr.push({s:s,mid:mid,front:frontness});
  }
  extr.sort(function(a,b){return a.front-b.front;});
  for(var ei=0;ei<extr.length;ei++){
    var e=extr[ei],s=e.s,col=PAL[s.ci],rgb=hx2rgb(col);
    var dr2=Math.max(0,rgb[0]-55),dg2=Math.max(0,rgb[1]-55),db2=Math.max(0,rgb[2]-55);
    var lr2=Math.min(255,rgb[0]+70),lg2=Math.min(255,rgb[1]+70),lb2=Math.min(255,rgb[2]+70);
    var steps=28;
    for(var li=0;li<layers;li++){
      var f0=li/layers,f1=(li+1)/layers;
      var y0=depthPx*(0.5-f0),y1=depthPx*(0.5-f1);
      g.beginPath();
      for(var st=0;st<=steps;st++){var a=s.a0+(s.a1-s.a0)*st/steps;g.lineTo(mx+Math.cos(a)*rd,my+y0+Math.sin(a)*rd*tiltF);}
      for(var st=steps;st>=0;st--){var a=s.a0+(s.a1-s.a0)*st/steps;g.lineTo(mx+Math.cos(a)*rd,my+y1+Math.sin(a)*rd*tiltF);}
      g.closePath();
      var sA=0.45+0.45*(1-f0);
      g.fillStyle='rgba('+Math.round(dr2+(rgb[0]-dr2)*sA)+','+Math.round(dg2+(rgb[1]-dg2)*sA)+','+Math.round(db2+(rgb[2]-db2)*sA)+',0.92)';g.fill();
    }
    var aS=s.a0,aE=s.a1;
    var topY=-depthPx*0.5,botY=depthPx*0.5;
    g.beginPath();
    g.moveTo(mx+Math.cos(aS)*rd,my+topY+Math.sin(aS)*rd*tiltF);
    g.lineTo(mx+Math.cos(aS)*rd,my+botY+Math.sin(aS)*rd*tiltF);
    g.lineTo(mx,my+botY);g.lineTo(mx,my+topY);g.closePath();
    g.fillStyle='rgba('+dr2+','+dg2+','+db2+',0.7)';g.fill();
    g.beginPath();
    g.moveTo(mx+Math.cos(aE)*rd,my+topY+Math.sin(aE)*rd*tiltF);
    g.lineTo(mx+Math.cos(aE)*rd,my+botY+Math.sin(aE)*rd*tiltF);
    g.lineTo(mx,my+botY);g.lineTo(mx,my+topY);g.closePath();
    g.fillStyle='rgba('+dr2+','+dg2+','+db2+',0.6)';g.fill();
    g.beginPath();g.ellipse(mx,my+topY,rd,rd*tiltF,0,s.a0,s.a1);g.lineTo(mx,my+topY);g.closePath();
    var tg=g.createRadialGradient(mx-rd*0.2,my+topY-rd*0.15,0,mx,my+topY,rd);
    tg.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');tg.addColorStop(0.5,col);tg.addColorStop(1,'rgb('+dr2+','+dg2+','+db2+')');
    g.fillStyle=tg;g.fill();
    g.strokeStyle='rgba(255,255,255,0.15)';g.lineWidth=1;g.stroke();
    var hp=(s.a0+s.a1)/2;var hx2=mx+Math.cos(hp)*rd*0.6,hy2=my+topY+Math.sin(hp)*rd*0.6*tiltF;
    pp.push({sx:hx2,sy:hy2,i:s.i,r:rd*0.25});
  }
}
function rVio(mx,my,sc){
  var ymap={};
  for(var i=0;i<N;i++){var yk=Math.round(Y[i]*1000)/1000;if(!ymap[yk])ymap[yk]=[];ymap[yk].push(i);}
  var ykeys=Object.keys(ymap).sort(function(a,b){return parseFloat(a)-parseFloat(b);});
  var strips=[];
  for(var gi=0;gi<ykeys.length;gi++){
    var idxs=ymap[ykeys[gi]];idxs.sort(function(a,b){return X[a]-X[b];});
    var col=PAL[gi%PAL.length],rgb=hx2rgb(col);
    var rawPts=[];
    for(var k=0;k<idxs.length;k++){
      var ii=idxs[k];
      var nx=(X[ii]-xmn)/xr-0.5,ny=(Y[ii]-ymn)/yr-0.5,nz=(Z[ii]-zmn)/(zr||1);
      rawPts.push({nx:nx,ny:ny,r:Math.max(0.006,nz*0.21)});
    }
    if(rawPts.length<2)continue;
    var mp=pj(rawPts[Math.floor(rawPts.length/2)].nx,rawPts[Math.floor(rawPts.length/2)].ny,0);
    strips.push({raw:rawPts,col:col,rgb:rgb,gi:gi,avgD:mp?mp.d:gi});
  }
  strips.sort(function(a,b){return a.avgD-b.avgD;});
  var nSeg=14;
  for(var si=0;si<strips.length;si++){
    var s=strips[si],raw=s.raw,col=s.col,rgb=s.rgb;
    var dr2=Math.max(0,rgb[0]-80),dg2=Math.max(0,rgb[1]-80),db2=Math.max(0,rgb[2]-80);
    var lr2=Math.min(255,rgb[0]+90),lg2=Math.min(255,rgb[1]+90),lb2=Math.min(255,rgb[2]+90);
    var rings=[];
    for(var k=0;k<raw.length;k++){
      var ring=[];
      for(var ai=0;ai<nSeg;ai++){
        var ang=TAU*ai/nSeg;
        var p=pj(raw[k].nx,raw[k].ny+raw[k].r*Math.sin(ang),raw[k].r*Math.cos(ang));
        ring.push(p?{sx:mx+p.x*sc,sy:my-p.y*sc}:null);
      }
      rings.push(ring);
    }
    var sliceOrd=[];
    for(var ai=0;ai<nSeg;ai++){
      var ang=TAU*(ai+0.5)/nSeg;
      sliceOrd.push({ai:ai,d:Math.cos(yaw-ang)});
    }
    sliceOrd.sort(function(a,b){return a.d-b.d;});
    for(var oi=0;oi<sliceOrd.length;oi++){
      var ai=sliceOrd[oi].ai,ai2=(ai+1)%nSeg;
      var ang=TAU*(ai+0.5)/nSeg;
      var nDotL=Math.max(0,(Math.cos(yaw-ang)*0.75+Math.sin(ang)*Math.sin(pitch)*0.25+1)*0.5);
      var sA=0.12+0.82*nDotL;
      var cr=Math.round(dr2+(rgb[0]-dr2)*sA),cg=Math.round(dg2+(rgb[1]-dg2)*sA),cb=Math.round(db2+(rgb[2]-db2)*sA);
      if(nDotL>0.82){var bl=Math.round((nDotL-0.82)/0.18*65);cr=Math.min(255,cr+bl);cg=Math.min(255,cg+bl);cb=Math.min(255,cb+bl);}
      g.fillStyle='rgba('+cr+','+cg+','+cb+',0.95)';
      for(var k=0;k<raw.length-1;k++){
        var p00=rings[k][ai],p01=rings[k][ai2],p10=rings[k+1][ai],p11=rings[k+1][ai2];
        if(!p00||!p01||!p10||!p11)continue;
        g.beginPath();g.moveTo(p00.sx,p00.sy);g.lineTo(p01.sx,p01.sy);g.lineTo(p11.sx,p11.sy);g.lineTo(p10.sx,p10.sy);g.closePath();g.fill();
      }
    }
    var kE=raw.length-1;
    for(var ai=0;ai<nSeg;ai++){
      var ai2=(ai+1)%nSeg;
      var c0=pj(raw[0].nx,raw[0].ny,0);
      if(c0&&rings[0][ai]&&rings[0][ai2]){
        g.fillStyle='rgba('+dr2+','+dg2+','+db2+',0.82)';
        g.beginPath();g.moveTo(mx+c0.x*sc,my-c0.y*sc);g.lineTo(rings[0][ai].sx,rings[0][ai].sy);g.lineTo(rings[0][ai2].sx,rings[0][ai2].sy);g.closePath();g.fill();
      }
      var c1=pj(raw[kE].nx,raw[kE].ny,0);
      if(c1&&rings[kE][ai]&&rings[kE][ai2]){
        g.fillStyle='rgba('+lr2+','+lg2+','+lb2+',0.72)';
        g.beginPath();g.moveTo(mx+c1.x*sc,my-c1.y*sc);g.lineTo(rings[kE][ai].sx,rings[kE][ai].sy);g.lineTo(rings[kE][ai2].sx,rings[kE][ai2].sy);g.closePath();g.fill();
      }
    }
    var mi=Math.floor(raw.length/2);
    var pm=pj(raw[mi].nx,raw[mi].ny,raw[mi].r);
    if(pm)pp.push({sx:mx+pm.x*sc,sy:my-pm.y*sc,i:s.gi,r:15});
  }
}
function rHm(mx,my,sc){
  var nr=Math.round(ymx-ymn+1)||1,nc=Math.round(xmx-xmn+1)||1;
  var cells=[];
  for(var i=0;i<N;i++){
    var nx=(X[i]-xmn)/xr-0.5,ny=(Y[i]-ymn)/yr-0.5,nz=(Z[i]-zmn)/zr-0.5;
    var bw=0.85/nc,bh=0.85/nr;
    var c0=pj(nx-bw/2,ny-bh/2,-0.5),c1=pj(nx+bw/2,ny-bh/2,-0.5),c2=pj(nx+bw/2,ny+bh/2,-0.5),c3=pj(nx-bw/2,ny+bh/2,-0.5);
    var t0=pj(nx-bw/2,ny-bh/2,-0.5+Math.max(0.04,nz*0.8+0.04)),t1=pj(nx+bw/2,ny-bh/2,-0.5+Math.max(0.04,nz*0.8+0.04)),t2=pj(nx+bw/2,ny+bh/2,-0.5+Math.max(0.04,nz*0.8+0.04)),t3=pj(nx-bw/2,ny+bh/2,-0.5+Math.max(0.04,nz*0.8+0.04));
    if(!c0||!c1||!c2||!c3||!t0||!t1||!t2||!t3)continue;
    var nv=Math.max(0,Math.min(1,(Z[i]-zmn)/zr));
    var r2,g2,b2;
    if(nv<0.25){r2=Math.round(15+nv*4*70);g2=Math.round(20+nv*4*100);b2=Math.round(80+nv*4*140);}
    else if(nv<0.5){var t=(nv-0.25)*4;r2=Math.round(85+t*80);g2=Math.round(120+t*60);b2=Math.round(220-t*40);}
    else if(nv<0.75){var t=(nv-0.5)*4;r2=Math.round(165+t*70);g2=Math.round(180-t*50);b2=Math.round(180-t*100);}
    else{var t=(nv-0.75)*4;r2=Math.round(235+t*20);g2=Math.round(130-t*60);b2=Math.round(80-t*50);}
    var d2=(t0.d+t1.d+t2.d+t3.d)/4;
    cells.push({c:[c0,c1,c2,c3],t:[t0,t1,t2,t3],r:r2,g:g2,b:b2,d:d2,i:i,nz:nz});
  }
  cells.sort(function(a,b){return a.d-b.d;});
  for(var j=0;j<cells.length;j++){
    var c=cells[j],r2=c.r,g2=c.g,b2=c.b;
    var lr=Math.min(255,r2+50),lg=Math.min(255,g2+50),lb=Math.min(255,b2+50);
    var dr=Math.max(0,r2-30),dg=Math.max(0,g2-30),db=Math.max(0,b2-30);
    g.beginPath();g.moveTo(mx+c.c[0].x*sc,my-c.c[0].y*sc);g.lineTo(mx+c.t[0].x*sc,my-c.t[0].y*sc);g.lineTo(mx+c.t[3].x*sc,my-c.t[3].y*sc);g.lineTo(mx+c.c[3].x*sc,my-c.c[3].y*sc);g.closePath();
    g.fillStyle='rgb('+dr+','+dg+','+db+')';g.fill();
    g.beginPath();g.moveTo(mx+c.c[1].x*sc,my-c.c[1].y*sc);g.lineTo(mx+c.t[1].x*sc,my-c.t[1].y*sc);g.lineTo(mx+c.t[2].x*sc,my-c.t[2].y*sc);g.lineTo(mx+c.c[2].x*sc,my-c.c[2].y*sc);g.closePath();
    g.fillStyle='rgb('+Math.round((r2+dr)/2)+','+Math.round((g2+dg)/2)+','+Math.round((b2+db)/2)+')';g.fill();
    g.beginPath();g.moveTo(mx+c.c[0].x*sc,my-c.c[0].y*sc);g.lineTo(mx+c.t[0].x*sc,my-c.t[0].y*sc);g.lineTo(mx+c.t[1].x*sc,my-c.t[1].y*sc);g.lineTo(mx+c.c[1].x*sc,my-c.c[1].y*sc);g.closePath();
    g.fillStyle='rgb('+Math.round((r2+lr)/2)+','+Math.round((g2+lg)/2)+','+Math.round((b2+lb)/2)+')';g.fill();
    g.beginPath();g.moveTo(mx+c.t[0].x*sc,my-c.t[0].y*sc);g.lineTo(mx+c.t[1].x*sc,my-c.t[1].y*sc);g.lineTo(mx+c.t[2].x*sc,my-c.t[2].y*sc);g.lineTo(mx+c.t[3].x*sc,my-c.t[3].y*sc);g.closePath();
    var tg=g.createLinearGradient(mx+c.t[0].x*sc,my-c.t[0].y*sc,mx+c.t[2].x*sc,my-c.t[2].y*sc);
    tg.addColorStop(0,'rgb('+lr+','+lg+','+lb+')');tg.addColorStop(1,'rgb('+r2+','+g2+','+b2+')');
    g.fillStyle=tg;g.fill();
    g.strokeStyle='rgba(255,255,255,0.08)';g.lineWidth=0.5;g.stroke();
    var cx2=(c.t[0].x+c.t[2].x)/2,cy2=(c.t[0].y+c.t[2].y)/2;
    pp.push({sx:mx+cx2*sc,sy:my-cy2*sc,i:c.i,r:8});
  }
}
function rCd(mx,my,sc){
  var bars=[];
  var n2=Math.floor(N/4);
  for(var i=0;i<n2;i++){
    var o=X[i*4],h=X[i*4+1]||o,l=X[i*4+2]||o,cl=X[i*4+3]||o;
    var nx=(i/(n2-1||1))-0.5;
    var bull=cl>=o;
    var ci=bull?2:0;
    var bw2=0.3/(n2||1),bd=bw2*0.8;
    var oLo=Math.min(o,cl),oHi=Math.max(o,cl);
    var zLo=-0.5+(oLo-xmn)/xr*0.7,zHi=-0.5+(oHi-xmn)/xr*0.7;
    var zWk=-0.5+(l-xmn)/xr*0.7,zWkH=-0.5+(h-xmn)/xr*0.7;
    var c0=pj(nx-bw2,-bd,zLo),c1=pj(nx+bw2,-bd,zLo),c2=pj(nx+bw2,bd,zLo),c3=pj(nx-bw2,bd,zLo);
    var t0=pj(nx-bw2,-bd,zHi),t1=pj(nx+bw2,-bd,zHi),t2=pj(nx+bw2,bd,zHi),t3=pj(nx-bw2,bd,zHi);
    var wp=pj(nx,0,zWk),wph=pj(nx,0,zWkH);
    if(!c0||!c1||!c2||!c3||!t0||!t1||!t2||!t3)continue;
    bars.push({c:[c0,c1,c2,c3],t:[t0,t1,t2,t3],wp:wp,wph:wph,o:o,h:h,l:l,cl:cl,ci:ci,d:(c0.d+t2.d)/2,i:i,bull:bull,nx:nx,bw2:bw2});
  }
  bars.sort(function(a,b){return a.d-b.d;});
  for(var j=0;j<bars.length;j++){
    var b=bars[j],col=b.bull?'#22c55e':'#ef4444',rgb=hx2rgb(col);
    var lr2=Math.min(255,rgb[0]+55),lg2=Math.min(255,rgb[1]+55),lb2=Math.min(255,rgb[2]+55);
    var dr2=Math.max(0,rgb[0]-50),dg2=Math.max(0,rgb[1]-50),db2=Math.max(0,rgb[2]-50);
    if(b.wp&&b.wph){g.strokeStyle=col;g.lineWidth=2;g.beginPath();
      g.moveTo(mx+b.wp.x*sc,my-b.wp.y*sc);g.lineTo(mx+b.wph.x*sc,my-b.wph.y*sc);g.stroke();}
    g.beginPath();g.moveTo(mx+b.c[0].x*sc,my-b.c[0].y*sc);g.lineTo(mx+b.t[0].x*sc,my-b.t[0].y*sc);g.lineTo(mx+b.t[3].x*sc,my-b.t[3].y*sc);g.lineTo(mx+b.c[3].x*sc,my-b.c[3].y*sc);g.closePath();
    g.fillStyle='rgb('+dr2+','+dg2+','+db2+')';g.fill();
    g.beginPath();g.moveTo(mx+b.c[1].x*sc,my-b.c[1].y*sc);g.lineTo(mx+b.t[1].x*sc,my-b.t[1].y*sc);g.lineTo(mx+b.t[2].x*sc,my-b.t[2].y*sc);g.lineTo(mx+b.c[2].x*sc,my-b.c[2].y*sc);g.closePath();
    g.fillStyle='rgb('+Math.round((rgb[0]+dr2)/2)+','+Math.round((rgb[1]+dg2)/2)+','+Math.round((rgb[2]+db2)/2)+')';g.fill();
    g.beginPath();g.moveTo(mx+b.c[0].x*sc,my-b.c[0].y*sc);g.lineTo(mx+b.t[0].x*sc,my-b.t[0].y*sc);g.lineTo(mx+b.t[1].x*sc,my-b.t[1].y*sc);g.lineTo(mx+b.c[1].x*sc,my-b.c[1].y*sc);g.closePath();
    g.fillStyle='rgb('+Math.round((rgb[0]+lr2)/2)+','+Math.round((rgb[1]+lg2)/2)+','+Math.round((rgb[2]+lb2)/2)+')';g.fill();
    g.beginPath();g.moveTo(mx+b.c[2].x*sc,my-b.c[2].y*sc);g.lineTo(mx+b.t[2].x*sc,my-b.t[2].y*sc);g.lineTo(mx+b.t[3].x*sc,my-b.t[3].y*sc);g.lineTo(mx+b.c[3].x*sc,my-b.c[3].y*sc);g.closePath();
    g.fillStyle='rgb('+Math.round((rgb[0]+dr2*0.7)/1.7)+','+Math.round((rgb[1]+dg2*0.7)/1.7)+','+Math.round((rgb[2]+db2*0.7)/1.7)+')';g.fill();
    g.beginPath();g.moveTo(mx+b.t[0].x*sc,my-b.t[0].y*sc);g.lineTo(mx+b.t[1].x*sc,my-b.t[1].y*sc);g.lineTo(mx+b.t[2].x*sc,my-b.t[2].y*sc);g.lineTo(mx+b.t[3].x*sc,my-b.t[3].y*sc);g.closePath();
    var tg=g.createLinearGradient(mx+b.t[0].x*sc,my-b.t[0].y*sc,mx+b.t[2].x*sc,my-b.t[2].y*sc);
    tg.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');tg.addColorStop(1,'rgb('+rgb[0]+','+rgb[1]+','+rgb[2]+')');
    g.fillStyle=tg;g.fill();
    g.strokeStyle='rgba(255,255,255,0.1)';g.lineWidth=0.6;g.stroke();
    pp.push({sx:mx+(b.t[0].x+b.t[2].x)/2*sc,sy:my-(b.t[0].y+b.t[2].y)/2*sc,i:b.i,r:10});
  }
}
function rDu(mx,my,sc){
  var items=[];
  for(var i=0;i<N;i++){
    var nx=(Y[i]-ymn)/yr-0.5;
    var zS=(X[i]-xmn)/xr-0.5,zE=(Z[i]-zmn)/zr-0.5;
    var ci=uc?C[i]%PAL.length:i%PAL.length;
    var pS=pj(zS,nx,nx*0.12),pE=pj(zE,nx,nx*0.12);
    if(!pS||!pE)continue;
    items.push({sx0:mx+pS.x*sc,sy0:my-pS.y*sc,sx1:mx+pE.x*sc,sy1:my-pE.y*sc,d:(pS.d+pE.d)/2,ci:ci,i:i});
  }
  items.sort(function(a,b){return a.d-b.d;});
  for(var j=0;j<items.length;j++){
    var it=items[j],col=PAL[it.ci],rgb=hx2rgb(col);
    var lr2=Math.min(255,rgb[0]+60),lg2=Math.min(255,rgb[1]+60),lb2=Math.min(255,rgb[2]+60);
    g.strokeStyle='rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.25)';g.lineWidth=5;g.lineCap='round';
    g.beginPath();g.moveTo(it.sx0,it.sy0);g.lineTo(it.sx1,it.sy1);g.stroke();
    g.strokeStyle=col;g.lineWidth=2.5;
    g.beginPath();g.moveTo(it.sx0,it.sy0);g.lineTo(it.sx1,it.sy1);g.stroke();
    var sr=9;
    var cg1=g.createRadialGradient(it.sx0-sr*0.3,it.sy0-sr*0.3,0,it.sx0,it.sy0,sr);
    cg1.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');cg1.addColorStop(0.65,'#60a5fa');cg1.addColorStop(1,'rgba(96,165,250,0.5)');
    g.fillStyle=cg1;g.beginPath();g.arc(it.sx0,it.sy0,sr,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.5)';g.beginPath();g.arc(it.sx0-sr*0.25,it.sy0-sr*0.25,sr*0.25,0,TAU);g.fill();
    var cg2=g.createRadialGradient(it.sx1-sr*0.3,it.sy1-sr*0.3,0,it.sx1,it.sy1,sr);
    cg2.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');cg2.addColorStop(0.65,'#f472b6');cg2.addColorStop(1,'rgba(244,114,182,0.5)');
    g.fillStyle=cg2;g.beginPath();g.arc(it.sx1,it.sy1,sr,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.5)';g.beginPath();g.arc(it.sx1-sr*0.25,it.sy1-sr*0.25,sr*0.25,0,TAU);g.fill();
    pp.push({sx:(it.sx0+it.sx1)/2,sy:(it.sy0+it.sy1)/2,i:it.i,r:12});
  }
}
function rFn(mx,my,sc){
  var total=Z[0]||1;for(var i=0;i<N;i++)if(Z[i]>total)total=Z[i];
  var items=[];
  for(var i=0;i<N;i++)items.push({v:Z[i],ci:uc?C[i]%PAL.length:i%PAL.length,i:i});
  var rd=Math.min(W,H)*0.26,tiltF=Math.cos(pitch)*0.85+0.15;
  var segH=0.85/N;
  items.sort(function(a,b){return b.v-a.v;});
  for(var i=0;i<items.length;i++){
    var it=items[i],fr=it.v/total,col=PAL[it.ci],rgb=hx2rgb(col);
    var lr2=Math.min(255,rgb[0]+60),lg2=Math.min(255,rgb[1]+60),lb2=Math.min(255,rgb[2]+60);
    var dr2=Math.max(0,rgb[0]-40),dg2=Math.max(0,rgb[1]-40),db2=Math.max(0,rgb[2]-40);
    var yTop=-0.4+i*segH,yBot=yTop+segH*0.9;
    var rTop=rd*fr,rBot=rd*(i+1<items.length?items[i+1].v/total:fr*0.4);
    var pTop=pj(0,yTop,0),pBot=pj(0,yBot,0);
    if(!pTop||!pBot)continue;
    var cyT=my-pTop.y*sc,cyB=my-pBot.y*sc;
    var layers=10;
    for(var li=0;li<layers;li++){
      var f2=li/layers,depth=(f2-0.5)*0.15*sc;
      var curRt=rTop+depth*0.1,curRb=rBot+depth*0.1;
      var offX=Math.cos(yaw)*depth*0.08;
      g.beginPath();g.ellipse(mx+offX,cyT-depth*0.3,curRt,curRt*tiltF*0.5,0,0,TAU);
      g.lineTo(mx+offX+curRb,cyB-depth*0.3);g.ellipse(mx+offX,cyB-depth*0.3,curRb,curRb*tiltF*0.5,0,0,Math.PI);
      g.closePath();
      var sA=0.6+0.3*(1-f2);
      g.fillStyle='rgba('+Math.round(rgb[0]*sA)+','+Math.round(rgb[1]*sA)+','+Math.round(rgb[2]*sA)+',0.5)';g.fill();
    }
    g.beginPath();g.ellipse(mx,cyT,rTop,rTop*tiltF*0.5,0,0,TAU);g.closePath();
    var tg=g.createRadialGradient(mx-rTop*0.2,cyT-rTop*0.15,0,mx,cyT,rTop);
    tg.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');tg.addColorStop(0.6,col);tg.addColorStop(1,'rgb('+dr2+','+dg2+','+db2+')');
    g.fillStyle=tg;g.fill();g.strokeStyle='rgba(255,255,255,0.12)';g.lineWidth=0.8;g.stroke();
    pp.push({sx:mx,sy:(cyT+cyB)/2,i:it.i,r:rTop*0.5});
  }
}
function rSb(mx,my,sc){
  var rd=Math.min(W,H)*0.3,tiltF=Math.cos(pitch)*0.85+0.15;
  var rings={};var maxRing=0;
  for(var i=0;i<N;i++){var rn=Math.round(Y[i]);if(rn>maxRing)maxRing=rn;if(!rings[rn])rings[rn]=[];rings[rn].push(i);}
  for(var rn=0;rn<=maxRing;rn++){
    if(!rings[rn])continue;
    var idxs=rings[rn];
    var total=0;for(var k=0;k<idxs.length;k++)total+=Z[idxs[k]];if(total<=0)continue;
    var innerR=rd*rn/(maxRing+1)*0.88+rd*0.09;
    var outerR=rd*(rn+1)/(maxRing+1)*0.88+rd*0.09;
    var depth=(maxRing-rn+1)*0.19*sc;
    var nLay=18,ca=0;
    for(var k=0;k<idxs.length;k++){
      var ii=idxs[k],ci=uc?C[ii]%PAL.length:ii%PAL.length;
      var a0=ca,a1=ca+Z[ii]/total*TAU;ca=a1;
      var col=PAL[ci],rgb=hx2rgb(col);
      var lr2=Math.min(255,rgb[0]+65),lg2=Math.min(255,rgb[1]+65),lb2=Math.min(255,rgb[2]+65);
      var dr2=Math.max(0,rgb[0]-65),dg2=Math.max(0,rgb[1]-65),db2=Math.max(0,rgb[2]-65);
      for(var li=0;li<nLay;li++){
        var frac=li/(nLay-1);
        var offY2=-depth*frac;
        var steps=24;
        g.beginPath();
        for(var st=0;st<=steps;st++){var a=a0+(a1-a0)*st/steps;g.lineTo(mx+Math.cos(a)*outerR,my+offY2+Math.sin(a)*outerR*tiltF);}
        for(var st=steps;st>=0;st--){var a=a0+(a1-a0)*st/steps;g.lineTo(mx+Math.cos(a)*innerR,my+offY2+Math.sin(a)*innerR*tiltF);}
        g.closePath();
        var sA=0.1+0.9*frac;
        g.fillStyle='rgba('+Math.round(dr2+(rgb[0]-dr2)*sA)+','+Math.round(dg2+(rgb[1]-dg2)*sA)+','+Math.round(db2+(rgb[2]-db2)*sA)+',0.9)';g.fill();
      }
      var topOff=-depth,botOff=0,stW=26;
      g.beginPath();
      for(var st=0;st<=stW;st++){var a=a0+(a1-a0)*st/stW;g.lineTo(mx+Math.cos(a)*outerR,my+topOff+Math.sin(a)*outerR*tiltF);}
      for(var st=stW;st>=0;st--){var a=a0+(a1-a0)*st/stW;g.lineTo(mx+Math.cos(a)*outerR,my+botOff+Math.sin(a)*outerR*tiltF);}
      g.closePath();g.fillStyle='rgba('+dr2+','+dg2+','+db2+',0.65)';g.fill();
      g.beginPath();
      for(var st=0;st<=stW;st++){var a=a0+(a1-a0)*st/stW;g.lineTo(mx+Math.cos(a)*innerR,my+topOff+Math.sin(a)*innerR*tiltF);}
      for(var st=stW;st>=0;st--){var a=a0+(a1-a0)*st/stW;g.lineTo(mx+Math.cos(a)*innerR,my+botOff+Math.sin(a)*innerR*tiltF);}
      g.closePath();g.fillStyle='rgba('+Math.round((dr2+rgb[0])/2)+','+Math.round((dg2+rgb[1])/2)+','+Math.round((db2+rgb[2])/2)+',0.45)';g.fill();
      var mid=(a0+a1)/2,mr=(innerR+outerR)/2;
      g.beginPath();
      for(var st=0;st<=stW;st++){var a=a0+(a1-a0)*st/stW;g.lineTo(mx+Math.cos(a)*outerR,my+topOff+Math.sin(a)*outerR*tiltF);}
      for(var st=stW;st>=0;st--){var a=a0+(a1-a0)*st/stW;g.lineTo(mx+Math.cos(a)*innerR,my+topOff+Math.sin(a)*innerR*tiltF);}
      g.closePath();
      var cxs=mx+Math.cos(mid)*mr,cys=my+topOff+Math.sin(mid)*mr*tiltF;
      var sg2=g.createRadialGradient(cxs-mr*0.22,cys-10,0,cxs,cys,mr*0.65);
      sg2.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');sg2.addColorStop(0.45,col);sg2.addColorStop(1,'rgb('+dr2+','+dg2+','+db2+')');
      g.fillStyle=sg2;g.fill();g.strokeStyle='rgba(255,255,255,0.18)';g.lineWidth=0.8;g.stroke();
      pp.push({sx:cxs,sy:cys,i:ii,r:10});
    }
  }
}
function rStk(mx,my,sc){
  var cats={};var catOrd=[];
  for(var i=0;i<N;i++){
    var ck=Math.round(X[i]*1000)/1000;
    if(!cats[ck]){cats[ck]=[];catOrd.push(ck);}
    cats[ck].push(i);
  }
  catOrd.sort(function(a,b){return a-b;});
  var bars=[];
  for(var ci2=0;ci2<catOrd.length;ci2++){
    var ck=catOrd[ci2],idxs=cats[ck];
    var cum=0;
    var nx=(ci2/(catOrd.length-1||1))-0.5;
    for(var k=0;k<idxs.length;k++){
      var ii=idxs[k],v=Z[ii];
      var z0=cum/(zmx||1)-0.5,z1=(cum+v)/(zmx||1)-0.5;cum+=v;
      var ci=uc?C[ii]%PAL.length:k%PAL.length;
      var bw2=0.42/(catOrd.length||1);
      var c0=pj(nx-bw2,-bw2,z0),c1=pj(nx+bw2,-bw2,z0),c2=pj(nx+bw2,bw2,z0),c3=pj(nx-bw2,bw2,z0);
      var t0=pj(nx-bw2,-bw2,z1),t1=pj(nx+bw2,-bw2,z1),t2=pj(nx+bw2,bw2,z1),t3=pj(nx-bw2,bw2,z1);
      if(!c0||!c1||!c2||!c3||!t0||!t1||!t2||!t3)continue;
      bars.push({c:[c0,c1,c2,c3],t:[t0,t1,t2,t3],ci:ci,d:(t0.d+t2.d)/2,i:ii});
    }
  }
  bars.sort(function(a,b){return a.d-b.d;});
  for(var j=0;j<bars.length;j++){
    var b=bars[j],col=PAL[b.ci],rgb=hx2rgb(col);
    var lr2=Math.min(255,rgb[0]+50),lg2=Math.min(255,rgb[1]+50),lb2=Math.min(255,rgb[2]+50);
    var dr2=Math.max(0,rgb[0]-35),dg2=Math.max(0,rgb[1]-35),db2=Math.max(0,rgb[2]-35);
    g.beginPath();g.moveTo(mx+b.c[0].x*sc,my-b.c[0].y*sc);g.lineTo(mx+b.t[0].x*sc,my-b.t[0].y*sc);g.lineTo(mx+b.t[3].x*sc,my-b.t[3].y*sc);g.lineTo(mx+b.c[3].x*sc,my-b.c[3].y*sc);g.closePath();
    g.fillStyle='rgb('+dr2+','+dg2+','+db2+')';g.fill();
    g.beginPath();g.moveTo(mx+b.c[1].x*sc,my-b.c[1].y*sc);g.lineTo(mx+b.t[1].x*sc,my-b.t[1].y*sc);g.lineTo(mx+b.t[2].x*sc,my-b.t[2].y*sc);g.lineTo(mx+b.c[2].x*sc,my-b.c[2].y*sc);g.closePath();
    g.fillStyle='rgb('+Math.round((rgb[0]+dr2)/2)+','+Math.round((rgb[1]+dg2)/2)+','+Math.round((rgb[2]+db2)/2)+')';g.fill();
    g.beginPath();g.moveTo(mx+b.t[0].x*sc,my-b.t[0].y*sc);g.lineTo(mx+b.t[1].x*sc,my-b.t[1].y*sc);g.lineTo(mx+b.t[2].x*sc,my-b.t[2].y*sc);g.lineTo(mx+b.t[3].x*sc,my-b.t[3].y*sc);g.closePath();
    var tg=g.createLinearGradient(mx+b.t[0].x*sc,my-b.t[0].y*sc,mx+b.t[2].x*sc,my-b.t[2].y*sc);
    tg.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');tg.addColorStop(1,col);
    g.fillStyle=tg;g.fill();g.strokeStyle='rgba(255,255,255,0.08)';g.lineWidth=0.5;g.stroke();
    var cx2=(b.t[0].x+b.t[2].x)/2,cy2=(b.t[0].y+b.t[2].y)/2;
    pp.push({sx:mx+cx2*sc,sy:my-cy2*sc,i:b.i,r:8});
  }
}
function rGlb(mx,my,sc){
  var gr2=Math.min(W,H)*0.35;
  var D2R=Math.PI/180,sp=Math.sin(pitch);
  var atm=g.createRadialGradient(mx,my,gr2*0.92,mx,my,gr2*1.18);
  atm.addColorStop(0,'rgba(56,189,248,0.12)');atm.addColorStop(0.5,'rgba(99,102,241,0.06)');atm.addColorStop(1,'rgba(0,0,0,0)');
  g.fillStyle=atm;g.beginPath();g.arc(mx,my,gr2*1.18,0,TAU);g.fill();
  var ocean=g.createRadialGradient(mx-gr2*0.3,my-gr2*0.3,gr2*0.1,mx,my,gr2);
  ocean.addColorStop(0,isDark?'#1e3a5f':'#bfdbfe');ocean.addColorStop(0.5,isDark?'#0f2847':'#93c5fd');ocean.addColorStop(1,isDark?'#0a1628':'#60a5fa');
  g.fillStyle=ocean;g.beginPath();g.arc(mx,my,gr2,0,TAU);g.fill();
  g.save();g.beginPath();g.arc(mx,my,gr2,0,TAU);g.clip();
  if(typeof MAP!=='undefined'){
    for(var pi=0;pi<MAP.length;pi++){
      var poly=MAP[pi],pts2=[],ccx=0,ccy=0,cn=0;
      for(var k=0;k<poly.length;k+=2){
        var nx2=poly[k]*0.0001,ny2=poly[k+1]*0.0001;
        var lon2=(-169.11+nx2*359.6)*D2R,lat2=(83.6-ny2*142.11)*D2R;
        var cx2=Math.cos(lat2)*Math.cos(lon2+yaw),cy2=Math.cos(lat2)*Math.sin(lon2+yaw),cz2=Math.sin(lat2);
        ccx+=cx2;ccy+=cy2;cn++;
        pts2.push(mx+cx2*gr2,my-cz2*gr2*0.95-cy2*gr2*sp*0.3);
      }
      if(cn>0)ccy/=cn;
      if(ccy<-0.15||pts2.length<6)continue;
      g.beginPath();g.moveTo(pts2[0],pts2[1]);
      for(var k2=2;k2<pts2.length;k2+=2)g.lineTo(pts2[k2],pts2[k2+1]);
      g.closePath();
      var sh=Math.max(0,Math.min(1,(ccy+0.15)/1.15));
      if(isDark){g.fillStyle='rgb('+Math.round(40+sh*30)+','+Math.round(68+sh*35)+','+Math.round(55+sh*25)+')';}
      else{g.fillStyle='rgb('+Math.round(100+sh*45)+','+Math.round(140+sh*45)+','+Math.round(108+sh*35)+')';}
      g.fill();
      g.strokeStyle=isDark?'rgba(100,200,150,0.18)':'rgba(0,80,40,0.12)';g.lineWidth=0.4;g.stroke();
    }
  }
  g.restore();
  g.strokeStyle=isDark?'rgba(56,189,248,0.35)':'rgba(0,0,0,0.15)';g.lineWidth=1.5;g.beginPath();g.arc(mx,my,gr2,0,TAU);g.stroke();
  g.save();g.beginPath();g.arc(mx,my,gr2,0,TAU);g.clip();
  for(var lat3=-60;lat3<=60;lat3+=30){
    var r3=gr2*Math.cos(lat3*D2R),y3=my-gr2*Math.sin(lat3*D2R)*0.95;
    g.strokeStyle=isDark?'rgba(56,189,248,0.07)':'rgba(0,0,0,0.04)';g.lineWidth=0.4;
    g.beginPath();g.ellipse(mx,y3,r3,r3*0.15,0,0,TAU);g.stroke();
  }
  for(var ln=0;ln<360;ln+=30){
    var lnR=ln*D2R+yaw;g.strokeStyle=isDark?'rgba(56,189,248,0.05)':'rgba(0,0,0,0.03)';g.lineWidth=0.3;
    g.beginPath();var started=false;
    for(var lt=-90;lt<=90;lt+=5){
      var ltR=lt*D2R,cx3=Math.cos(ltR)*Math.cos(lnR),cy3=Math.cos(ltR)*Math.sin(lnR),cz3=Math.sin(ltR);
      if(cy3<-0.05){started=false;continue;}
      var sx3=mx+cx3*gr2,sy3=my-cz3*gr2*0.95-cy3*gr2*sp*0.3;
      if(!started){g.moveTo(sx3,sy3);started=true;}else g.lineTo(sx3,sy3);
    }
    g.stroke();
  }
  g.restore();
  var pts=[];
  for(var i=0;i<N;i++){
    var lat4=Y[i]*D2R,lon4=X[i]*D2R;
    var cx4=Math.cos(lat4)*Math.cos(lon4+yaw),cy4=Math.cos(lat4)*Math.sin(lon4+yaw),cz4=Math.sin(lat4);
    if(cy4<-0.1)continue;
    var sx4=mx+cx4*gr2,sy4=my-cz4*gr2*0.95-cy4*gr2*sp*0.3;
    var nv=Math.max(0,Math.min(1,(Z[i]-zmn)/zr));
    var ci=uc?C[i]%PAL.length:i%PAL.length;
    pts.push({sx:sx4,sy:sy4,d:cy4,ci:ci,i:i,nv:nv});
  }
  pts.sort(function(a,b){return b.d-a.d;});
  for(var j=0;j<pts.length;j++){
    var p=pts[j],col=PAL[p.ci],rgb=hx2rgb(col);
    var lr2=Math.min(255,rgb[0]+60),lg2=Math.min(255,rgb[1]+60),lb2=Math.min(255,rgb[2]+60);
    var barH=Math.max(4,p.nv*gr2*0.35);
    g.strokeStyle='rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.3)';g.lineWidth=4;g.lineCap='round';
    g.beginPath();g.moveTo(p.sx,p.sy);g.lineTo(p.sx,p.sy-barH);g.stroke();
    g.strokeStyle=col;g.lineWidth=2;
    g.beginPath();g.moveTo(p.sx,p.sy);g.lineTo(p.sx,p.sy-barH);g.stroke();
    var sr=Math.max(4,3+p.nv*5);
    var cg=g.createRadialGradient(p.sx-sr*0.3,p.sy-barH-sr*0.3,0,p.sx,p.sy-barH,sr);
    cg.addColorStop(0,'rgb('+lr2+','+lg2+','+lb2+')');cg.addColorStop(0.6,col);cg.addColorStop(1,'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+',0.5)');
    g.fillStyle=cg;g.beginPath();g.arc(p.sx,p.sy-barH,sr,0,TAU);g.fill();
    g.fillStyle='rgba(255,255,255,0.5)';g.beginPath();g.arc(p.sx-sr*0.25,p.sy-barH-sr*0.25,sr*0.25,0,TAU);g.fill();
    pp.push({sx:p.sx,sy:p.sy-barH,i:p.i,r:sr+3});
  }
}
function ht(ex,ey){var bi=-1,bd=900;for(var i=pp.length-1;i>=0;i--){var dx=pp[i].sx-ex,dy=pp[i].sy-ey,d2=dx*dx+dy*dy,hr=(pp[i].r+10)*(pp[i].r+10);if(d2<bd&&d2<hr){bd=d2;bi=pp[i].i;}}return bi;}
function sT(idx,ex,ey){
  var lbl=CL.length>0&&uc?CL[C[idx]%CL.length]:'Point '+(idx+1);
  var h='<b>'+lbl+'</b>';
  h+='<span>'+xl+':</span> <span class="tv">'+X[idx].toFixed(3)+'</span><br>';
  h+='<span>'+yl+':</span> <span class="tv">'+Y[idx].toFixed(3)+'</span><br>';
  h+='<span>'+zl+':</span> <span class="tv">'+Z[idx].toFixed(3)+'</span>';
  tip.innerHTML=h;tip.className='c3t v'+(pin?' p':'');
  var bx=wrap.getBoundingClientRect();
  var tx=ex-bx.left+16,ty=ey-bx.top-14;
  if(tx+175>W)tx=ex-bx.left-182;if(ty<0)ty=ey-bx.top+22;
  tip.style.left=tx+'px';tip.style.top=ty+'px';
}
function hT(){if(!pin){tip.className='c3t';}}
var shiftDrag=false;
function tick(){
  var dirty=false;
  if(autoR&&!dg){yaw+=0.004;dirty=true;}
  if(Math.abs(velY)>0.0003||Math.abs(velP)>0.0003){
    yaw+=velY;pitch=Math.max(-1.47,Math.min(1.47,pitch+velP));
    velY*=fric;velP*=fric;dirty=true;
  }
  if(keys.ArrowLeft||keys.a||keys.A){yaw-=kSpd;dirty=true;}
  if(keys.ArrowRight||keys.d||keys.D){yaw+=kSpd;dirty=true;}
  if(keys.ArrowUp||keys.w||keys.W){pitch=Math.min(1.47,pitch+kSpd);dirty=true;}
  if(keys.ArrowDown||keys.s||keys.S){pitch=Math.max(-1.47,pitch-kSpd);dirty=true;}
  if(keys.q||keys.Q){zoom=Math.min(5,zoom*1.015);dirty=true;}
  if(keys.e||keys.E){zoom=Math.max(0.3,zoom*0.985);dirty=true;}
  if(dirty)R();
  requestAnimationFrame(tick);
}
R();requestAnimationFrame(tick);
wrap.addEventListener('mousedown',function(e){
  if(e.shiftKey||e.button===2||e.button===1){shiftDrag=true;dg=true;mv=false;lx=e.clientX;ly=e.clientY;e.preventDefault();return;}
  dg=true;mv=false;lx=e.clientX;ly=e.clientY;dwX=e.clientX;dwY=e.clientY;velY=0;velP=0;e.preventDefault();
});
window.addEventListener('mousemove',function(e){
  if(dg&&shiftDrag){
    panX+=(e.clientX-lx);panY+=(e.clientY-ly);lx=e.clientX;ly=e.clientY;R();return;
  }
  if(dg){
    var dx=e.clientX-lx,dy=e.clientY-ly;
    if(Math.abs(e.clientX-dwX)>3||Math.abs(e.clientY-dwY)>3)mv=true;
    velY=dx*0.008;velP=dy*0.008;
    yaw+=velY;pitch=Math.max(-1.47,Math.min(1.47,pitch+velP));
    lx=e.clientX;ly=e.clientY;
    R();return;
  }
  if(pin)return;
  var bx=wrap.getBoundingClientRect(),ex=e.clientX-bx.left,ey=e.clientY-bx.top;
  if(ex<0||ey<0||ex>W||ey>H){hT();return;}
  var idx=ht(ex,ey);if(idx>=0)sT(idx,e.clientX,e.clientY);else hT();
});
window.addEventListener('mouseup',function(e){
  if(!dg)return;var wasSh=shiftDrag;dg=false;shiftDrag=false;
  if(wasSh)return;
  if(!mv){var bx=wrap.getBoundingClientRect(),ex=e.clientX-bx.left,ey=e.clientY-bx.top;var idx=ht(ex,ey);if(idx>=0){pin=true;piI=idx;sT(idx,e.clientX,e.clientY);R();}else{pin=false;piI=-1;tip.className='c3t';R();}}
});
wrap.addEventListener('wheel',function(e){zoom=Math.max(0.3,Math.min(5,zoom*(e.deltaY>0?1.08:0.93)));R();e.preventDefault();},{passive:false});
wrap.addEventListener('dblclick',function(){yaw=0.785;pitch=0.6;zoom=1.0;panX=0;panY=0;pin=false;piI=-1;tip.className='c3t';velY=0;velP=0;R();});
wrap.addEventListener('mouseleave',function(){if(!pin)hT();});
wrap.addEventListener('contextmenu',function(e){e.preventDefault();});
document.addEventListener('keydown',function(e){
  keys[e.key]=true;
  if(e.key==='Escape'){if(pin){pin=false;piI=-1;tip.className='c3t';R();}}
  if(e.key===' '){autoR=!autoR;R();e.preventDefault();}
  if(e.key==='='||e.key==='+'){zoom=Math.min(5,zoom*1.1);R();}
  if(e.key==='-'){zoom=Math.max(0.3,zoom*0.9);R();}
  if(e.key==='r'||e.key==='R'){yaw=0.785;pitch=0.6;zoom=1.0;panX=0;panY=0;velY=0;velP=0;R();}
});
document.addEventListener('keyup',function(e){keys[e.key]=false;});
var t0d=0;
wrap.addEventListener('touchstart',function(e){
  if(e.touches.length===2){t0d=Math.hypot(e.touches[1].clientX-e.touches[0].clientX,e.touches[1].clientY-e.touches[0].clientY);e.preventDefault();return;}
  if(e.touches.length===1){dg=true;mv=false;lx=e.touches[0].clientX;ly=e.touches[0].clientY;dwX=lx;dwY=ly;velY=0;velP=0;e.preventDefault();}
},{passive:false});
wrap.addEventListener('touchmove',function(e){
  if(e.touches.length===2){var nd=Math.hypot(e.touches[1].clientX-e.touches[0].clientX,e.touches[1].clientY-e.touches[0].clientY);if(t0d>0){zoom=Math.max(0.3,Math.min(5,zoom*(nd/t0d)));t0d=nd;R();}e.preventDefault();return;}
  if(!dg||e.touches.length!==1)return;var dx=e.touches[0].clientX-lx,dy=e.touches[0].clientY-ly;if(Math.abs(e.touches[0].clientX-dwX)>3||Math.abs(e.touches[0].clientY-dwY)>3)mv=true;velY=dx*0.008;velP=dy*0.008;yaw+=velY;pitch=Math.max(-1.47,Math.min(1.47,pitch+velP));lx=e.touches[0].clientX;ly=e.touches[0].clientY;R();e.preventDefault();
},{passive:false});
wrap.addEventListener('touchend',function(e){if(e.touches.length===0){dg=false;t0d=0;if(!mv){var bx=wrap.getBoundingClientRect(),ex=dwX-bx.left,ey=dwY-bx.top;var idx=ht(ex,ey);if(idx>=0){pin=true;piI=idx;sT(idx,dwX,dwY);R();}else{pin=false;piI=-1;tip.className='c3t';}}}});
})();</script></body></html>
</div>

</details>

---

## See also

- [Scatter 2D](../2d/scatter.md)
- [DBSCAN 3D](../../ml/dbscan.md)
- [Bubble 3D](bubble3d.md)
