# Evaluation

<div class="lang-en">

Evaluation tools measure model performance and provide reproducible data splits for unbiased assessment.

| Tool | Description |
|------|-------------|
| [Metrics](metrics.md) | Accuracy, precision, recall, F1, R², MSE, MAE, confusion matrix |
| [train_test_split / StratifiedKFold](train-test-split.md) | Reproducible train/test splits with optional stratification |

### Workflow

```python
import seraplot as sp
import numpy as np

# 1 — Split data
X_train, X_test, y_train, y_test = sp.train_test_split(
    X, y, test_size=0.2, random_state=42, stratify=True
)

# 2 — Train model
model = sp.RandomForestClassifier(n_estimators=100)
model.fit(X_train, y_train)

# 3 — Evaluate
y_pred = model.predict(X_test)
print(sp.accuracy_score(y_test, y_pred))
print(sp.classification_report(y_test, y_pred))
```

### Metrics reference

| Task | Metrics available |
|------|-----------------|
| Classification | `accuracy_score`, `precision_score`, `recall_score`, `f1_score`, `classification_report`, `confusion_matrix` |
| Regression | `r2_score`, `mean_squared_error`, `mean_absolute_error` |

</div>

<div class="lang-fr">

Les outils d'évaluation mesurent les performances des modèles et fournissent des découpages reproductibles pour une évaluation non biaisée.

| Outil | Description |
|------|-------------|
| [Métriques](metrics.md) | Précision, recall, F1, R², MSE, MAE, matrice de confusion |
| [train_test_split / StratifiedKFold](train-test-split.md) | Découpage train/test reproductible avec stratification optionnelle |

### Workflow

```python
import seraplot as sp
import numpy as np

# 1 — Découper les données
X_train, X_test, y_train, y_test = sp.train_test_split(
    X, y, test_size=0.2, random_state=42, stratify=True
)

# 2 — Entraîner le modèle
model = sp.RandomForestClassifier(n_estimators=100)
model.fit(X_train, y_train)

# 3 — Évaluer
y_pred = model.predict(X_test)
print(sp.accuracy_score(y_test, y_pred))
print(sp.classification_report(y_test, y_pred))
```

### Référence des métriques

| Tâche | Métriques disponibles |
|------|-----------------|
| Classification | `accuracy_score`, `precision_score`, `recall_score`, `f1_score`, `classification_report`, `confusion_matrix` |
| Régression | `r2_score`, `mean_squared_error`, `mean_absolute_error` |

</div>
