# Cross-Validation Splitters

<div class="lang-en">

Three splitter strategies, exposed via the `ml_kfold_split` JSON dispatcher in **all languages** (Python / WebAssembly / C FFI):

| `kind` | Inputs | When to use |
|--------|--------|-------------|
| `"kfold"` | `n`, `k`, `seed` | Default i.i.d. shuffle-then-split |
| `"stratified"` | `y`, `k`, `seed` | Classification — preserves class proportion in each fold |
| `"group"` | `groups`, `k` | Time series / patient-level / repeated-measure data — keeps the same group entirely in train OR test |

## API Reference

```python
import seraplot as sp, json

folds = json.loads(sp.ml_kfold_split(json.dumps({
    "kind": "kfold",
    "n":    1000,
    "k":    5,
    "seed": 42,
})))

strat = json.loads(sp.ml_kfold_split(json.dumps({
    "kind": "stratified",
    "y":    [0, 0, 1, 1, 2, 2, 0, 1, 2],
    "k":    3,
    "seed": 0,
})))

grp = json.loads(sp.ml_kfold_split(json.dumps({
    "kind":   "group",
    "groups": [1, 1, 2, 2, 3, 3, 4, 4],
    "k":      2,
})))
```

Each call returns `[ {"train": [...indices], "test": [...indices]}, ... ]` with `k` folds.

**Pure-Rust API**

```rust
use seraplot::ml::model_selection::split::{
    kfold_indices, stratified_kfold_indices, group_kfold_indices,
};

let folds_basic = kfold_indices(n, 5, seed);
let folds_strat = stratified_kfold_indices(&y, 5, seed);
let folds_grp   = group_kfold_indices(&groups, 5);
```

## Algorithmic Functioning

**KFold** — Fisher–Yates shuffle of $\{0, \dots, n-1\}$ then split into $k$ contiguous chunks.

**StratifiedKFold** — bucket indices by class, shuffle each bucket, then deal them round-robin across the $k$ folds. Guarantees each fold has $\lfloor n_c / k \rfloor$ or $\lceil n_c / k \rceil$ examples of class $c$.

**GroupKFold** — distinct groups are placed into folds via the **largest-first / least-loaded** heuristic (similar to scikit-learn): groups are sorted by size descending, each is assigned to the fold currently holding the fewest samples. No group ever appears in both `train` and `test` of the same fold.

</div>

<div class="lang-fr">

Trois stratégies de découpage, exposées via le dispatcher JSON `ml_kfold_split` dans **tous les langages** (Python / WebAssembly / C FFI) :

| `kind` | Entrées | Quand l'utiliser |
|--------|---------|------------------|
| `"kfold"` | `n`, `k`, `seed` | Découpage i.i.d. par mélange aléatoire |
| `"stratified"` | `y`, `k`, `seed` | Classification — conserve la proportion de classes par fold |
| `"group"` | `groups`, `k` | Séries temporelles / mesures répétées — un même groupe reste entièrement en train OU en test |

## Référence API

```python
import seraplot as sp, json

folds = json.loads(sp.ml_kfold_split(json.dumps({
    "kind": "kfold",
    "n":    1000,
    "k":    5,
    "seed": 42,
})))

strat = json.loads(sp.ml_kfold_split(json.dumps({
    "kind": "stratified",
    "y":    [0, 0, 1, 1, 2, 2, 0, 1, 2],
    "k":    3,
    "seed": 0,
})))

grp = json.loads(sp.ml_kfold_split(json.dumps({
    "kind":   "group",
    "groups": [1, 1, 2, 2, 3, 3, 4, 4],
    "k":      2,
})))
```

Chaque appel retourne `[ {"train": [...indices], "test": [...indices]}, ... ]` avec `k` folds.

**API Rust pur**

```rust
use seraplot::ml::model_selection::split::{
    kfold_indices, stratified_kfold_indices, group_kfold_indices,
};

let folds_basic = kfold_indices(n, 5, seed);
let folds_strat = stratified_kfold_indices(&y, 5, seed);
let folds_grp   = group_kfold_indices(&groups, 5);
```

## Fonctionnement algorithmique

**KFold** — mélange Fisher–Yates de $\{0, \dots, n-1\}$ puis découpage en $k$ morceaux contigus.

**StratifiedKFold** — on regroupe les indices par classe, on mélange chaque seau, puis on les distribue tour à tour entre les $k$ folds. Garantit que chaque fold contient $\lfloor n_c / k \rfloor$ ou $\lceil n_c / k \rceil$ exemples de la classe $c$.

**GroupKFold** — les groupes distincts sont placés via l'heuristique **plus gros d'abord / fold le moins chargé** (proche de scikit-learn) : groupes triés par taille décroissante, chacun affecté au fold actuellement le moins peuplé. Aucun groupe n'apparaît dans `train` et `test` du même fold.

</div>
