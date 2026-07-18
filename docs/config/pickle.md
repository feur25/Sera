# Pickle / Serialization

<div class="lang-en">

`Chart` objects are picklable via `__getstate__` / `__setstate__` — works with `joblib`, `multiprocessing`, `Ray`, Streamlit reruns.

## Python

```python
import seraplot as sp
import pickle

chart = sp.bar("Revenue", labels=["a", "b", "c"], values=[1, 2, 3])
blob = pickle.dumps(chart)

restored = pickle.loads(blob)
restored.save("restored.html")
```

Internally, only the HTML string is serialized — minimal payload, no transient state.

## ML models

Trained models are a separate concern from `Chart` pickling — see [ML Model Persistence](ml-persistence.md) for `sp.ml_save_model()` / `sp.ml_load_model()`.

</div>

<div class="lang-fr">

Les objets `Chart` sont sérialisables via `__getstate__` / `__setstate__` — compatible `joblib`, `multiprocessing`, `Ray`, reruns Streamlit.

## Python

```python
import seraplot as sp
import pickle

chart = sp.bar("Revenu", labels=["a", "b", "c"], values=[1, 2, 3])
blob = pickle.dumps(chart)

restored = pickle.loads(blob)
restored.save("restored.html")
```

En interne, seule la chaîne HTML est sérialisée — payload minimal, aucun état transitoire.

## Modèles ML

Les modèles entraînés sont une préoccupation distincte du pickling de `Chart` — voir [ML Model Persistence](ml-persistence.md) pour `sp.ml_save_model()` / `sp.ml_load_model()`.

</div>
