
## Your first chart

```python
import seraplot as sp

chart = sp.build_bar_chart(
    "Sales by Region",
    labels=["North", "South", "East", "West"],
    values=[120.0, 85.0, 200.0, 140.0],
)
```

> In Jupyter, the chart displays automatically. / Dans Jupyter, le graphique s'affiche automatiquement.

---

## Save to HTML

```python
with open("chart.html", "w", encoding="utf-8") as f:
    f.write(chart.html)
```

---

## Common patterns

### Scatter plot with colored groups

```python
import numpy as np

x = np.random.randn(500).tolist()
y = np.random.randn(500).tolist()
groups = (["A"] * 250) + (["B"] * 250)

chart = sp.build_scatter_chart(
    "Two Clusters",
    x_values=x,
    y_values=y,
    color_groups=groups,
)
```

---

### DBSCAN clustering in one call

```python
import numpy as np

x = np.random.randn(10_000).tolist()
y = np.random.randn(10_000).tolist()

chart = sp.build_dbscan_chart(
    "DBSCAN",
    x_values=x,
    y_values=y,
    eps=0.3,
    min_samples=10,
)
```

---

### Global dark background for all charts

```python
sp.set_global_background("#0f172a")

chart1 = sp.build_bar_chart("Chart 1", labels=["A", "B"], values=[10.0, 20.0])
chart2 = sp.build_line_chart("Chart 2", labels=["x1", "x2"], values=[5.0, 15.0])

sp.reset_global_background()
```

---

### 3D scatter

```python
import numpy as np

x = np.random.randn(1000).tolist()
y = np.random.randn(1000).tolist()
z = np.random.randn(1000).tolist()

chart = sp.build_scatter3d_chart(
    "3D Scatter",
    x_values=x,
    y_values=y,
    z_values=z,
    x_label="X", y_label="Y", z_label="Z",
)
```
