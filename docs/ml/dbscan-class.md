# DBSCAN Class

## Signature

```python
model = sp.DBSCAN(eps: float = 0.5, min_samples: int = 5)

model.fit(x: list[float], y: list[float]) -> None
model.fit_predict(x: list[float], y: list[float]) -> list[int]

model.labels_      -> list[int]
model.n_clusters_  -> int
model.n_noise_     -> int
```

---

## Description

Low-level DBSCAN class for programmatic access to cluster labels.
`-1` labels indicate noise points (not part of any cluster).

This class runs the same Rust-native DBSCAN as `build_dbscan_chart` but returns raw labels
instead of producing a chart.

---

## Constructor Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `eps` | `float` | `0.5` | Neighborhood distance threshold |
| `min_samples` | `int` | `5` | Minimum points to form a cluster core |

---

## Methods

### `fit(x, y)`

Runs DBSCAN on the 2D data. Populates `labels_`, `n_clusters_`, and `n_noise_`.

| Argument | Type | Description |
|----------|------|-------------|
| `x` | `list[float]` | X coordinates |
| `y` | `list[float]` | Y coordinates |

### `fit_predict(x, y) -> list[int]`

Equivalent to calling `fit(x, y)` then returning `labels_`.

---

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `labels_` | `list[int]` | Cluster label per point (`-1` = noise) |
| `n_clusters_` | `int` | Number of identified clusters |
| `n_noise_` | `int` | Number of noise points |

---

## Examples

### Accessing labels

```python
import seraplot as sp

x = [1.0, 1.1, 1.2, 10.0, 10.1, 99.0]
y = [1.0, 0.9, 1.1, 10.2, 10.0, 99.0]

model = sp.DBSCAN(eps=0.5, min_samples=2)
labels = model.fit_predict(x, y)

print(labels)
print(f"Clusters: {model.n_clusters_}")
print(f"Noise:    {model.n_noise_}")
```

### Pipeline: cluster then visualize

```python
import seraplot as sp

model = sp.DBSCAN(eps=1.0, min_samples=5)
model.fit(x_data, y_data)

color_groups = [str(lbl) for lbl in model.labels_]

chart = sp.build_scatter_chart(
    f"DBSCAN ({model.n_clusters_} clusters)",
    x_values=x_data,
    y_values=y_data,
    color_groups=color_groups,
)
```

---

## See also

- [DBSCAN Chart](dbscan.md)
- [DBSCAN 3D](dbscan3d.md)
- [Scatter Chart](../charts/2d/scatter.md)
