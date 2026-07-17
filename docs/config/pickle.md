# Pickle / Serialization

<div class="lang-en">

`Chart` objects are picklable via `__getstate__` / `__setstate__` — works with `joblib`, `multiprocessing`, `Ray`, Streamlit reruns.

## Python

```python
import seraplot as sp
import pickle

chart = sp.bar("Revenue", ["a", "b", "c"], [1, 2, 3])
blob = pickle.dumps(chart)

restored = pickle.loads(blob)
restored.save("restored.html")
```

Internally, only the HTML string is serialized — minimal payload, no transient state.

## ML models

`sp.ml_save_model()` / `sp.ml_load_model()` are real, working functions backed by an in-process model registry (name + optional version, arbitrary JSON payload, params/metrics/tags). Save whatever a training call returned, reload it later by name:

```python
import json
import seraplot as sp

result = sp.knn_classifier(data=X_train, target=y_train, k=5)
sp.ml_save_model(
    name="knn_v1", kind="knn_classifier",
    payload=json.dumps(result), params={"k": "5"}, metrics={}, tags=[],
)

loaded = sp.ml_load_model(name="knn_v1")
restored_result = json.loads(loaded["payload"])
```

</div>

<div class="lang-fr">

Les objets `Chart` sont sérialisables via `__getstate__` / `__setstate__` — compatible `joblib`, `multiprocessing`, `Ray`, reruns Streamlit.

## Python

```python
import seraplot as sp
import pickle

chart = sp.bar("Revenu", ["a", "b", "c"], [1, 2, 3])
blob = pickle.dumps(chart)

restored = pickle.loads(blob)
restored.save("restored.html")
```

En interne, seule la chaîne HTML est sérialisée — payload minimal, aucun état transitoire.

## Modèles ML

`sp.ml_save_model()` / `sp.ml_load_model()` sont des fonctions réelles et fonctionnelles, adossées à un registre de modèles en mémoire (nom + version optionnelle, payload JSON arbitraire, params/métriques/tags). Sauvegardez ce qu'un appel d'entraînement a retourné, rechargez-le plus tard par son nom :

```python
import json
import seraplot as sp

result = sp.knn_classifier(data=X_train, target=y_train, k=5)
sp.ml_save_model(
    name="knn_v1", kind="knn_classifier",
    payload=json.dumps(result), params={"k": "5"}, metrics={}, tags=[],
)

loaded = sp.ml_load_model(name="knn_v1")
restored_result = json.loads(loaded["payload"])
```

</div>
