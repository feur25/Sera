# Pickle / Serialization

<div class="lang-en">

`Chart` objects are picklable via `__getstate__` / `__setstate__` — works with `joblib`, `multiprocessing`, `Ray`, Streamlit reruns.

## Python

```python
import seraplot as sp
import pickle

chart = sp.bar([1,2,3], ["a","b","c"])
blob = pickle.dumps(chart)

restored = pickle.loads(blob)
restored.save("restored.html")
```

Internally, only the HTML string is serialized — minimal payload, no transient state.

</div>

<div class="lang-fr">

Les objets `Chart` sont sérialisables via `__getstate__` / `__setstate__` — compatible `joblib`, `multiprocessing`, `Ray`, reruns Streamlit.

## Python

```python
import seraplot as sp
import pickle

chart = sp.bar([1,2,3], ["a","b","c"])
blob = pickle.dumps(chart)

restored = pickle.loads(blob)
restored.save("restored.html")
```

En interne, seule la chaîne HTML est sérialisée — payload minimal, aucun état transitoire.

</div>
