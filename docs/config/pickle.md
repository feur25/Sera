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

## ML models

Native `ml_save_model` / `ml_load_model` are registered in the Rust core but **not yet exposed as Python attributes** in 2.3.89. For now use stdlib `pickle` directly on fitted estimators:

```python
import pickle
from seraplot import KNeighborsClassifier

clf = KNeighborsClassifier(k=5).fit(X, y)
blob = pickle.dumps(clf)
restored = pickle.loads(blob)
```

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

## Modèles ML

Les fonctions natives `ml_save_model` / `ml_load_model` sont enregistrées dans le cœur Rust mais **pas encore exposées comme attributs Python** en 2.3.89. Utilisez `pickle` standard sur les estimateurs entraînés :

```python
import pickle
from seraplot import KNeighborsClassifier

clf = KNeighborsClassifier(k=5).fit(X, y)
blob = pickle.dumps(clf)
restored = pickle.loads(blob)
```

</div>
