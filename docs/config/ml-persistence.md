# ML Model Persistence

<div class="lang-en">

`sp.ml_save_model()` / `sp.ml_load_model()` are real, verified functions backed
by an in-process, name-versioned model registry — not a file-path envelope.
Every save under the same `name` gets the next `version` automatically; load
without a `version` returns the latest.

---

## `sp.ml_save_model(name, kind, payload, params, metrics, tags) -> dict`

| Argument | Type | Description |
|----------|------|-------------|
| `name` | `str` | Registry key. Repeated saves under the same name auto-increment `version`. |
| `kind` | `str` | Free-form model identifier (e.g. `"knn_classifier"`, `"kmeans"`). |
| `payload` | `str` | Arbitrary JSON-serialized state — typically the dict a training call returned. |
| `params` | `dict[str, str]` | Hyperparameters, stored alongside for reference. |
| `metrics` | `dict[str, float]` | Training/eval metrics, stored alongside. |
| `tags` | `list[str]` | Free-form labels for later filtering. |

Returns `{"name": ..., "version": ...}`.

```python
import json
import seraplot as sp

result = sp.knn_classifier(data=X_train, target=y_train, k=5)
saved = sp.ml_save_model(
    name="knn_v1", kind="knn_classifier",
    payload=json.dumps(result), params={"k": "5"}, metrics={}, tags=[],
)
print(saved)  # {'name': 'knn_v1', 'version': 1}
```

---

## `sp.ml_load_model(name, version=None) -> dict`

Loads the latest save for `name`, or a specific `version` if given.

```python
loaded = sp.ml_load_model(name="knn_v1")
# {'found': True, 'name': 'knn_v1', 'version': 1, 'kind': 'knn_classifier', 'payload': '...'}

restored = json.loads(loaded["payload"])
```

Missing name/version returns `{"found": False}` rather than raising.

---

## End-to-end: train → save → reload

```python
import json
import seraplot as sp

result = sp.knn_classifier(data=X_train, target=y_train, k=5)
sp.ml_save_model(
    name="knn_v1", kind="knn_classifier",
    payload=json.dumps(result), params={"k": "5"}, metrics={}, tags=[],
)

loaded = sp.ml_load_model(name="knn_v1")
restored = json.loads(loaded["payload"])
assert restored == result
```

Saving again under the same `name` bumps the version rather than overwriting:

```python
sp.ml_save_model(name="knn_v1", kind="knn_classifier", payload=json.dumps(result), params={}, metrics={}, tags=[])
# {'name': 'knn_v1', 'version': 2}

sp.ml_load_model(name="knn_v1", version=1)  # the original save is still there
```

</div>

<div class="lang-fr">

`sp.ml_save_model()` / `sp.ml_load_model()` sont des fonctions réelles et
vérifiées, adossées à un registre de modèles en mémoire, indexé par nom et
versionné — pas une enveloppe fichier. Chaque sauvegarde sous le même `name`
obtient automatiquement la `version` suivante ; charger sans `version`
retourne la dernière.

---

## `sp.ml_save_model(name, kind, payload, params, metrics, tags) -> dict`

| Argument | Type | Description |
|----------|------|-------------|
| `name` | `str` | Clé du registre. Des sauvegardes répétées sous le même nom incrémentent `version` automatiquement. |
| `kind` | `str` | Identifiant libre du modèle (ex. `"knn_classifier"`, `"kmeans"`). |
| `payload` | `str` | État sérialisé en JSON arbitraire — typiquement le dict retourné par un appel d'entraînement. |
| `params` | `dict[str, str]` | Hyperparamètres, stockés pour référence. |
| `metrics` | `dict[str, float]` | Métriques d'entraînement/évaluation, stockées pour référence. |
| `tags` | `list[str]` | Labels libres pour un filtrage ultérieur. |

Retourne `{"name": ..., "version": ...}`.

```python
import json
import seraplot as sp

result = sp.knn_classifier(data=X_train, target=y_train, k=5)
saved = sp.ml_save_model(
    name="knn_v1", kind="knn_classifier",
    payload=json.dumps(result), params={"k": "5"}, metrics={}, tags=[],
)
print(saved)  # {'name': 'knn_v1', 'version': 1}
```

---

## `sp.ml_load_model(name, version=None) -> dict`

Charge la dernière sauvegarde pour `name`, ou une `version` précise si fournie.

```python
loaded = sp.ml_load_model(name="knn_v1")
# {'found': True, 'name': 'knn_v1', 'version': 1, 'kind': 'knn_classifier', 'payload': '...'}

restored = json.loads(loaded["payload"])
```

Un nom/version absent retourne `{"found": False}` plutôt que de lever une exception.

---

## Bout-en-bout : entraînement → sauvegarde → recharge

```python
import json
import seraplot as sp

result = sp.knn_classifier(data=X_train, target=y_train, k=5)
sp.ml_save_model(
    name="knn_v1", kind="knn_classifier",
    payload=json.dumps(result), params={"k": "5"}, metrics={}, tags=[],
)

loaded = sp.ml_load_model(name="knn_v1")
restored = json.loads(loaded["payload"])
assert restored == result
```

Sauvegarder à nouveau sous le même `name` incrémente la version plutôt que d'écraser :

```python
sp.ml_save_model(name="knn_v1", kind="knn_classifier", payload=json.dumps(result), params={}, metrics={}, tags=[])
# {'name': 'knn_v1', 'version': 2}

sp.ml_load_model(name="knn_v1", version=1)  # la sauvegarde originale existe toujours
```

</div>
