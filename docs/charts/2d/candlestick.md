# Candlestick Chart

## Signature

```python
sp.build_candlestick(
    title: str,
    dates: list[str],
    opens: list[float],
    highs: list[float],
    lows: list[float],
    closes: list[float],
    *,
    width: int = 1000,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    color_up: int = 0x22c55e,
    color_down: int = 0xef4444,
    background: str | None = None,
    gridlines: bool = True,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Financial candlestick chart for OHLC (Open / High / Low / Close) price data.

Green candles indicate a price rise (close > open), red candles a fall.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `dates` | `list[str]` | required | Date/time labels for X-axis |
| `opens` | `list[float]` | required | Opening prices |
| `highs` | `list[float]` | required | Session high prices |
| `lows` | `list[float]` | required | Session low prices |
| `closes` | `list[float]` | required | Closing prices |
| `color_up` | `int` | `0x22c55e` | Bullish candle fill color |
| `color_down` | `int` | `0xef4444` | Bearish candle fill color |
| `width` | `int` | `1000` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Horizontal gridlines |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### Stock price chart

```python
import seraplot as sp

chart = sp.build_candlestick(
    "AAPL - January 2024",
    ["Jan 2","Jan 3","Jan 4","Jan 5","Jan 8","Jan 9","Jan 10"],
    [185.0, 184.2, 182.5, 181.0, 183.5, 185.0, 186.0],
    [186.5, 185.0, 183.8, 183.5, 186.0, 187.2, 188.0],
    [183.5, 182.0, 180.5, 180.0, 183.0, 184.5, 185.5],
    [184.2, 182.5, 181.0, 183.5, 185.0, 186.0, 187.5],
    y_label="Price ($)",
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/candlestick.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Dumbbell](dumbbell.md)
- [Candlestick 3D](../3d/candlestick3d.md)
