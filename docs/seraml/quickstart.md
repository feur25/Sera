<style>
.sml-qs-hero{padding:22px 28px;border-radius:14px;background:linear-gradient(135deg,#050c1a,#071620);border:1px solid rgba(34,211,238,.18);margin:1.2em 0 2em}
.sml-qs-hero h2{margin:0 0 5px;font-size:19px;color:#67e8f9;font-weight:800;border:none}
.sml-qs-hero p{margin:0;color:#94a3b8;font-size:13.5px;line-height:1.55}
.sml-step{margin:1.8em 0 1em;display:flex;align-items:center;gap:10px}
.sml-step-num{width:28px;height:28px;border-radius:50%;background:rgba(34,211,238,.12);border:1px solid rgba(34,211,238,.3);display:flex;align-items:center;justify-content:center;font-size:12px;font-weight:800;color:#22d3ee;flex-shrink:0}
.sml-step-label{font-size:15px;font-weight:700;color:#e2e8f0}
</style>

<div class="sml-qs-hero">
<h2>Quick Start</h2>
<p>Train, evaluate, and visualize a model in under 30 lines.</p>
</div>

<div class="sml-step"><div class="sml-step-num">1</div><div class="sml-step-label">Import</div></div>

```python
from seraml.linear_model import LinearRegression
from seraml.model_selection import train_test_split
from seraml.preprocessing import StandardScaler
from seraml import metrics
import numpy as np
```

<div class="sml-step"><div class="sml-step-num">2</div><div class="sml-step-label">Prepare data</div></div>

```python
rng = np.random.default_rng(42)
X = rng.standard_normal((1000, 8))
y = X @ rng.standard_normal(8) + rng.standard_normal(1000) * 0.5

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2)

scaler = StandardScaler()
X_train = scaler.fit_transform(X_train)
X_test = scaler.transform(X_test)
```

<div class="sml-step"><div class="sml-step-num">3</div><div class="sml-step-label">Train</div></div>

```python
model = LinearRegression()
model.fit(X_train, y_train)
```

<div class="sml-step"><div class="sml-step-num">4</div><div class="sml-step-label">Evaluate</div></div>

```python
preds = model.predict(X_test)

print("R²  ", metrics.r2_score(y_test, preds))
print("MAE ", metrics.mean_absolute_error(y_test, preds))
print("RMSE", metrics.root_mean_squared_error(y_test, preds))
```

<div class="sml-step"><div class="sml-step-num">5</div><div class="sml-step-label">Visualize with SeraPlot</div></div>

```python
import seraplot as sp

sp.scatter(
    "Predicted vs Actual",
    values=list(y_test),
    series=[list(preds)],
).show()
```

## Classification example

```python
from seraml.ensemble import RandomForestClassifier
from seraml.model_selection import train_test_split, GridSearchCV
from seraml import metrics

clf = RandomForestClassifier(n_estimators=100, max_depth=6)
clf.fit(X_train, y_train_cls)

preds = clf.predict(X_test)
print(metrics.classification_report(y_test_cls, preds))
```

## Cluster example

```python
from seraml.cluster import KMeans

km = KMeans(n_clusters=4)
km.fit(X)
labels = km.labels_
```

→ [Introduction →](introduction.html)  
→ [ML Reference →](../ml/index.html)
