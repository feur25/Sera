# ML Model Persistence

<div class="lang-en">

SeraPlot ships two universal primitives for serialising ML model state to a
language-agnostic JSON envelope. They are exposed through `for_each_fn!`, so
the **exact same wire format** is reachable from Python, JavaScript and the
C-FFI. A model trained in a Python notebook can be reloaded in a Node service
or a browser, and vice-versa.

The envelope shape is intentionally simple:

```json
{
  "seraplot_model_v": 1,
  "kind": "kmeans",
  "state": { "...": "model-specific JSON" }
}
```

This makes the persistence layer **trivial to inspect, diff, version-control,
encrypt or sign**. No pickle, no custom binary, no security risk.

---

## `sp.ml_save_model(payload: str) -> str`

Saves a JSON envelope. The input is a JSON string with three fields:

| Field | Type | Description |
|-------|------|-------------|
| `kind` | `str` | Free-form model identifier (e.g. `"kmeans"`, `"random_forest"`). |
| `state` | `any` | Model-specific JSON state (centroids, coefficients, tree splits...). |
| `path` | `str?` | Optional file path. If absent, the envelope is returned inline in the response. |

```python
import seraplot as sp
import json

centroids = [[0.1, 0.2], [4.5, 3.7]]
out = sp.ml_save_model(json.dumps({
    "kind": "kmeans",
    "state": {"k": 2, "centroids": centroids, "inertia": 12.3},
    "path": "kmeans.json",
}))

print(out)
```

```json
{"ok": true, "path": "kmeans.json", "size": 87}
```

If `path` is omitted, the envelope is returned in the `data` field:

```python
out = sp.ml_save_model(json.dumps({"kind": "kmeans", "state": {...}}))
parsed = json.loads(out)
serialised_blob = parsed["data"]   # ready to upload, encrypt, embed...
```

---

## `sp.ml_load_model(payload: str) -> str`

Loads a previously saved envelope from disk **or** from an inline JSON string.

```python
out = sp.ml_load_model(json.dumps({"path": "kmeans.json"}))
parsed = json.loads(out)
model = parsed["model"]   # {"seraplot_model_v":1,"kind":"kmeans","state":{...}}
```

```python
out = sp.ml_load_model(json.dumps({"data": serialised_blob}))
```

The response is always:

```json
{"ok": true, "model": { "seraplot_model_v": 1, "kind": "...", "state": {...} }}
```

or, on failure:

```json
{"ok": false, "error": "..."}
```

---

## End-to-end: train → save → reload → predict

```python
import seraplot as sp
import json, numpy as np

X = np.random.randn(1000, 2)
km = sp.KMeans(k=4)
km.fit(X)

state = {
    "k": km.k,
    "centroids": km.centroids_,
    "inertia": km.inertia_,
    "n_iter": km.n_iter_,
}
sp.ml_save_model(json.dumps({"kind": "kmeans", "state": state, "path": "km.json"}))

loaded = json.loads(sp.ml_load_model(json.dumps({"path": "km.json"})))["model"]
assert loaded["state"]["centroids"] == state["centroids"]
```

---

## JavaScript

```js
import * as sp from "seraplot";

const out = sp.mlSaveModel(JSON.stringify({
  kind: "kmeans",
  state: { centroids: [[0,0],[1,1]] },
  path: "km.json",
}));

const loaded = JSON.parse(sp.mlLoadModel(JSON.stringify({ path: "km.json" })));
console.log(loaded.model.state.centroids);
```

---

## Why JSON instead of pickle / bincode?

- **Cross-language** — the envelope works in Python, JS, C, Rust without any
  binary protocol negotiation.
- **Audit-friendly** — the file is human-readable, fits Git diffs and is
  trivial to redact PII from.
- **Safe** — no arbitrary code execution on load, unlike `pickle` or
  `joblib`.
- **Compressible** — gzipped JSON state is typically within 5–15 % of a
  binary serialisation for tabular ML models, with negligible parse cost.

</div>

<div class="lang-fr">

SeraPlot fournit deux primitives universelles pour sérialiser l'état d'un
modèle ML dans une enveloppe JSON indépendante du langage. Elles sont
exposées via `for_each_fn!`, donc le **format de fil exact** est accessible
depuis Python, JavaScript et le C-FFI. Un modèle entraîné dans un notebook
Python peut être rechargé dans un service Node ou dans le navigateur, et
inversement.

La forme de l'enveloppe est volontairement minimale :

```json
{
  "seraplot_model_v": 1,
  "kind": "kmeans",
  "state": { "...": "JSON spécifique au modèle" }
}
```

La couche de persistance est ainsi **triviale à inspecter, à diff-er, à
versionner, à chiffrer ou à signer**. Pas de pickle, pas de binaire custom,
pas de risque de sécurité.

---

## `sp.ml_save_model(payload: str) -> str`

Sauvegarde une enveloppe JSON. L'entrée est une chaîne JSON avec trois
champs :

| Champ | Type | Description |
|-------|------|-------------|
| `kind` | `str` | Identifiant libre du modèle (ex. `"kmeans"`, `"random_forest"`). |
| `state` | `any` | État JSON spécifique au modèle (centroïdes, coefficients, splits d'arbre...). |
| `path` | `str?` | Chemin de fichier optionnel. Si absent, l'enveloppe est renvoyée en ligne dans la réponse. |

```python
import seraplot as sp
import json

centroids = [[0.1, 0.2], [4.5, 3.7]]
out = sp.ml_save_model(json.dumps({
    "kind": "kmeans",
    "state": {"k": 2, "centroids": centroids, "inertia": 12.3},
    "path": "kmeans.json",
}))

print(out)
```

```json
{"ok": true, "path": "kmeans.json", "size": 87}
```

Si `path` est omis, l'enveloppe est renvoyée dans le champ `data` :

```python
out = sp.ml_save_model(json.dumps({"kind": "kmeans", "state": {...}}))
parsed = json.loads(out)
serialised_blob = parsed["data"]
```

---

## `sp.ml_load_model(payload: str) -> str`

Recharge une enveloppe précédemment sauvée depuis disque **ou** depuis une
chaîne JSON en mémoire.

```python
out = sp.ml_load_model(json.dumps({"path": "kmeans.json"}))
parsed = json.loads(out)
model = parsed["model"]
```

```python
out = sp.ml_load_model(json.dumps({"data": serialised_blob}))
```

La réponse est toujours :

```json
{"ok": true, "model": { "seraplot_model_v": 1, "kind": "...", "state": {...} }}
```

ou, en cas d'échec :

```json
{"ok": false, "error": "..."}
```

---

## Bout-en-bout : entraînement → sauvegarde → recharge → prédiction

```python
import seraplot as sp
import json, numpy as np

X = np.random.randn(1000, 2)
km = sp.KMeans(k=4)
km.fit(X)

state = {
    "k": km.k,
    "centroids": km.centroids_,
    "inertia": km.inertia_,
    "n_iter": km.n_iter_,
}
sp.ml_save_model(json.dumps({"kind": "kmeans", "state": state, "path": "km.json"}))

loaded = json.loads(sp.ml_load_model(json.dumps({"path": "km.json"})))["model"]
assert loaded["state"]["centroids"] == state["centroids"]
```

---

## JavaScript

```js
import * as sp from "seraplot";

const out = sp.mlSaveModel(JSON.stringify({
  kind: "kmeans",
  state: { centroids: [[0,0],[1,1]] },
  path: "km.json",
}));

const loaded = JSON.parse(sp.mlLoadModel(JSON.stringify({ path: "km.json" })));
console.log(loaded.model.state.centroids);
```

---

## Pourquoi JSON plutôt que pickle / bincode ?

- **Cross-language** — l'enveloppe fonctionne en Python, JS, C, Rust sans
  négociation de protocole binaire.
- **Auditable** — le fichier est lisible, s'intègre aux diffs Git et permet
  de retirer trivialement les PII.
- **Sûr** — aucune exécution de code arbitraire au chargement, contrairement
  à `pickle` ou `joblib`.
- **Compressible** — un état JSON gzippé pèse typiquement entre 5 et 15 % de
  plus qu'une sérialisation binaire pour des modèles ML tabulaires, avec un
  coût de parsing négligeable.

</div>
