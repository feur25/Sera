# Permutation Importance

<div class="lang-en">

Model-agnostic feature importance: shuffle one feature column at a time and measure how much the score drops. Works with any fitted estimator that exposes `predict`.

## API Reference

**Cross-language JSON dispatcher** — `ml_permutation_importance`

```python
import seraplot as sp, json, numpy as np

reg = sp.RandomForestRegressor(n_estimators=200).fit(X, y)
baseline_pred = reg.predict(X).tolist()

perm_preds = []
rng = np.random.default_rng(0)
for j in range(X.shape[1]):
    reps = []
    for r in range(5):
        Xp = X.copy()
        rng.shuffle(Xp[:, j])
        reps.append(reg.predict(Xp).tolist())
    perm_preds.append(reps)

out = json.loads(sp.ml_permutation_importance(json.dumps({
    "data":          X.tolist(),
    "y":             y.tolist(),
    "baseline_pred": baseline_pred,
    "perm_preds":    perm_preds,
    "task":          "regression",
})))
```

Returns `{ "importances_mean": [...], "importances_std": [...], "baseline": <score> }`.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `data` | `list[list[float]]` | — | Feature matrix $X$ ($n \times p$) |
| `y` | `list[float]` | — | Targets (cast to int for classification) |
| `baseline_pred` | `list[float]` | — | $\hat{y}$ on un-shuffled data |
| `perm_preds` | `list[list[list[float]]]` | — | `[p][n_repeats][n]` predictions with column $j$ shuffled |
| `task` | `str` | `"regression"` | `"regression"` (R²) or `"classification"` (accuracy) |

The dispatcher does not own the model; you compute the predictions in your language of choice and pass them in. This keeps WASM / FFI / Python parity.

**Pure-Rust API** (closures, in-process)

```rust
use seraplot::ml::model_selection::{permutation_importance_reg, permutation_importance_cls};

let (mean, std) = permutation_importance_reg(
    &x, n, p, &y,
    /* n_repeats */ 5,
    /* seed */      42,
    |xp, n, p| model.predict(xp, n, p),
);
```

## Algorithmic Functioning

Let $s_0$ be the baseline score (R² for regression, accuracy for classification) and $s_j^{(r)}$ the score after permuting column $j$ on repeat $r$:

<div>$$\Delta_j^{(r)} = s_0 - s_j^{(r)}\qquad\bar{\Delta}_j = \frac{1}{R}\sum_{r=1}^{R}\Delta_j^{(r)},\qquad \sigma_j = \mathrm{std}(\Delta_j^{(\cdot)})$$</div>

Large $\bar{\Delta}_j$ → feature $j$ matters; $\bar{\Delta}_j \approx 0$ → noise feature; negative values can occur with very small datasets and indicate the feature is *uncorrelated* with the target.

</div>

<div class="lang-fr">

Importance de feature *agnostique au modèle* : on permute une colonne à la fois et l'on mesure la chute de score. Fonctionne avec tout estimateur ajusté exposant `predict`.

## Référence API

**Dispatcher JSON inter-langage** — `ml_permutation_importance`

```python
import seraplot as sp, json, numpy as np

reg = sp.RandomForestRegressor(n_estimators=200).fit(X, y)
baseline_pred = reg.predict(X).tolist()

perm_preds = []
rng = np.random.default_rng(0)
for j in range(X.shape[1]):
    reps = []
    for r in range(5):
        Xp = X.copy()
        rng.shuffle(Xp[:, j])
        reps.append(reg.predict(Xp).tolist())
    perm_preds.append(reps)

out = json.loads(sp.ml_permutation_importance(json.dumps({
    "data":          X.tolist(),
    "y":             y.tolist(),
    "baseline_pred": baseline_pred,
    "perm_preds":    perm_preds,
    "task":          "regression",
})))
```

Retourne `{ "importances_mean": [...], "importances_std": [...], "baseline": <score> }`.

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `data` | `list[list[float]]` | — | Matrice $X$ ($n \times p$) |
| `y` | `list[float]` | — | Cible (convertie en int en classification) |
| `baseline_pred` | `list[float]` | — | $\hat{y}$ sur données non permutées |
| `perm_preds` | `list[list[list[float]]]` | — | `[p][n_repeats][n]` prédictions avec colonne $j$ permutée |
| `task` | `str` | `"regression"` | `"regression"` (R²) ou `"classification"` (accuracy) |

Le dispatcher ne possède pas le modèle ; vous calculez les prédictions dans le langage de votre choix puis vous les passez. Cela garantit la parité WASM / FFI / Python.

**API Rust pur** (closures, in-process)

```rust
use seraplot::ml::model_selection::{permutation_importance_reg, permutation_importance_cls};

let (mean, std) = permutation_importance_reg(
    &x, n, p, &y,
    /* n_repeats */ 5,
    /* seed */      42,
    |xp, n, p| model.predict(xp, n, p),
);
```

## Fonctionnement algorithmique

Soit $s_0$ le score de référence (R² en régression, accuracy en classification) et $s_j^{(r)}$ le score après permutation de la colonne $j$ à la répétition $r$ :

<div>$$\Delta_j^{(r)} = s_0 - s_j^{(r)}\qquad\bar{\Delta}_j = \frac{1}{R}\sum_{r=1}^{R}\Delta_j^{(r)},\qquad \sigma_j = \mathrm{std}(\Delta_j^{(\cdot)})$$</div>

$\bar{\Delta}_j$ grand → la feature $j$ est importante ; $\bar{\Delta}_j \approx 0$ → bruit ; valeurs négatives possibles sur petits jeux et indiquent une feature *non corrélée* à la cible.

</div>
