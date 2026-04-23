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

Default models: `logistic`, `knn`, `decision_tree`, `random_forest`, `gradient_boosting`. Customize with `models=["random_forest","knn"]`.

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

Modèles par défaut : `logistic`, `knn`, `decision_tree`, `random_forest`, `gradient_boosting`. Personnalisable avec `models=["random_forest","knn"]`.

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
