# AutoML & Pipeline

<div class="lang-en">

## auto_classify

Try several classifiers, return the leaderboard sorted by score.

```python
import seraplot as sp

result = sp.auto_classify(X_train, y_train)
print(result["best_model"], result["best_score"])
for row in result["leaderboard"]:
    print(row)
```

Default models (2.3.89+): `knn`, `decision_tree`, `gradient_boosting`. The previous defaults `logistic` and `random_forest` are still available via `models=["logistic","random_forest"]` but were dropped from defaults pending stability fixes. Customize freely with e.g. `models=["knn","gradient_boosting"]`.

Failed/panicking models are caught: their leaderboard entry has `score = NaN` and is sorted last.

## Pipeline

Chain transformers + estimator (sklearn-compatible API).

```python
from seraplot import StandardScaler, RandomForestClassifier, Pipeline

pipe = Pipeline([
    ("scaler", StandardScaler()),
    ("model",  RandomForestClassifier(n_estimators=200)),
])
pipe.fit(X_train, y_train)
preds = pipe.predict(X_test)
```

</div>

<div class="lang-fr">

## auto_classify

Essaie plusieurs classifieurs, retourne le leaderboard trié par score.

```python
import seraplot as sp

result = sp.auto_classify(X_train, y_train)
print(result["best_model"], result["best_score"])
for row in result["leaderboard"]:
    print(row)
```

Modèles par défaut (2.3.89+) : `knn`, `decision_tree`, `gradient_boosting`. Les anciens défauts `logistic` et `random_forest` restent disponibles via `models=["logistic","random_forest"]` mais ont été retirés en attendant un correctif de stabilité. Personnalisable librement, ex. `models=["knn","gradient_boosting"]`.

Les modèles qui échouent/paniquent sont capturés : leur entrée du leaderboard a `score = NaN` et passe en dernier.

## Pipeline

Enchaîne transformers + estimateur (API compatible sklearn).

```python
from seraplot import StandardScaler, RandomForestClassifier, Pipeline

pipe = Pipeline([
    ("scaler", StandardScaler()),
    ("model",  RandomForestClassifier(n_estimators=200)),
])
pipe.fit(X_train, y_train)
preds = pipe.predict(X_test)
```

</div>
