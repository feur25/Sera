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

```python
import seraplot as sp

chart = sp.build_candlestick3d_chart(
    "BTC/USD 3D",
    dates=["Mon","Tue","Wed","Thu","Fri"],
    opens= [42000, 43500, 41800, 44000, 45200],
    highs= [44000, 44200, 43500, 46000, 47000],
    lows=  [41500, 43000, 40000, 43500, 44800],
    closes=[43500, 41800, 44000, 45200, 46500],
)
```

---

## See also

- [Candlestick 2D](../2d/candlestick.md)
