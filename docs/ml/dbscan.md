# DBSCAN Chart

## Signature

```python
sp.build_dbscan_chart(
    title: str,
    x_values: list[float],
    y_values: list[float],
    *,
    eps: float = 0.5,
    min_samples: int = 5,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    palette: list[int] | None = None,
    background: str | None = None,
    normalize: bool = False,
) -> Chart
```

---

## Description

2D DBSCAN clustering chart. Runs the DBSCAN algorithm (implemented in Rust) and plots each point colored by cluster membership. Noise points are shown in grey.

Graphique de clustering DBSCAN 2D. Exécute l'algorithme DBSCAN (implémenté en Rust) et trace chaque point coloré selon son appartenance à un cluster. Les points de bruit sont affichés en gris.

SeraPlot's DBSCAN runs up to **600× faster** than scikit-learn on large datasets.

Le DBSCAN de SeraPlot s'exécute jusqu'à **600× plus vite** que scikit-learn sur de grands jeux de données.

---

## Parameters / Paramètres

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `x_values` | `list[float]` | required | X coordinates of data points |
| `y_values` | `list[float]` | required | Y coordinates of data points |
| `eps` | `float` | `0.5` | Maximum neighborhood distance (epsilon) |
| `min_samples` | `int` | `5` | Minimum points to form a dense region |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Show gridlines |
| `palette` | `list[int] \| None` | `None` | Custom cluster colors |
| `background` | `str \| None` | `None` | Chart background color |
| `normalize` | `bool` | `False` | Normalize features to [0, 1] before clustering |

---

## Returns / Retour

`Chart`

---

## Choosing eps and min_samples

- **`eps`**: Start with a k-distance graph. A good `eps` is where the sorted k-nearest-neighbor distances show a "knee". Too small → everything is noise. Too large → everything is one cluster.
- **`min_samples`**: Typically set to `dim × 2` where `dim` is the number of features. Larger values produce more robust clusters but may mark more points as noise.

---

## Examples / Exemples

### Synthetic blobs

```python
import seraplot as sp
import random

def make_blob(cx, cy, n=150, s=0.5):
    return [(cx + random.gauss(0, s), cy + random.gauss(0, s)) for _ in range(n)]

pts  = make_blob(0, 0) + make_blob(5, 5) + make_blob(10, 0)
x, y = zip(*pts)

chart = sp.build_dbscan_chart(
    "DBSCAN Clustering",
    x_values=list(x),
    y_values=list(y),
    eps=1.0,
    min_samples=5,
    x_label="Feature 1",
    y_label="Feature 2",
)
```

### With normalization

```python
import seraplot as sp

chart = sp.build_dbscan_chart(
    "DBSCAN — Normalized",
    x_values=x,
    y_values=y,
    eps=0.1,
    min_samples=5,
    normalize=True,
)
```

---

## See also / Voir aussi

- [DBSCAN Class](dbscan-class.md) — for accessing labels and cluster metadata
- [DBSCAN 3D](dbscan3d.md)
- [Scatter Chart](../charts/2d/scatter.md)
