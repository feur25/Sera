# Background Configuration

## Functions

### `set_global_background(color: str) -> None`

Sets a global background color applied to all subsequently created charts.

```python
sp.set_global_background("#1a1a2e")
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `color` | `str` | CSS color string (hex `"#rrggbb"`, `"rgb(…)"`, named color) |

---

### `reset_global_background() -> None`

Clears the global background, reverting to the per-chart default.

```python
sp.reset_global_background()
```

---

### `set_bg_fn(html: str, color: str) -> str`

Applies a background color to an existing HTML chart string.
Returns the modified HTML string.

```python
html_str = chart.to_html()
html_with_bg = sp.set_bg_fn(html_str, "#0f172a")
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `str` | HTML string from `Chart.to_html()` |
| `color` | `str` | CSS background color |

**Returns**: `str` — Modified HTML string.

---

## Examples

### Dark theme dashboard

```python
import seraplot as sp

sp.set_global_background("#0f172a")

bar   = sp.build_bar_chart("Revenue", labels=["A","B"], values=[300, 200])
pie   = sp.build_pie_chart("Share",   labels=["A","B"], values=[60, 40])

sp.reset_global_background()
```

---

## See also

- [Palette & Colors](palette.md)
- [Auto Display](auto-display.md)
