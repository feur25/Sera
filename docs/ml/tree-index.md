# Tree-Based Models

<div class="lang-en">

Tree-based models partition the feature space using a hierarchy of decision rules. They handle non-linear relationships natively and require minimal preprocessing.

| Model | Task | Description |
|-------|------|-------------|
| [DecisionTree](decision-tree.md) | Both | Single decision tree (classifier and regressor) |
| [RandomForest](random-forest.md) | Both | Ensemble of decorrelated decision trees via bagging |
| [GradientBoosting](gradient-boosting.md) | Both | Sequential boosting with gradient descent on residuals |
| [AdaBoost](adaboost.md) | Both | Adaptive boosting with sample reweighting |

### Choosing the right model

| Situation | Recommended model |
|-----------|------------------|
| Interpretability required | `DecisionTreeClassifier` |
| High accuracy, parallel training | `RandomForestClassifier` |
| Best accuracy, careful tuning | `GradientBoostingClassifier` |
| Simple boosting, fast training | `AdaBoostClassifier` |

### Key properties

- All models expose `feature_importances_` for feature selection.
- All support `fit`, `predict`, `score`, `get_params`, `set_params`.
- Ensemble models (RandomForest, GradientBoosting, AdaBoost) are significantly more robust to overfitting than a single decision tree.

</div>

<div class="lang-fr">

Les modèles basés sur les arbres partitionnent l'espace des variables à l'aide d'une hiérarchie de règles de décision. Ils gèrent nativement les relations non linéaires et nécessitent peu de prétraitement.

| Modèle | Tâche | Description |
|-------|------|-------------|
| [DecisionTree](decision-tree.md) | Les deux | Arbre de décision unique (classifieur et régresseur) |
| [RandomForest](random-forest.md) | Les deux | Ensemble d'arbres décorrélés via le bagging |
| [GradientBoosting](gradient-boosting.md) | Les deux | Boosting séquentiel avec descente de gradient sur les résidus |
| [AdaBoost](adaboost.md) | Les deux | Boosting adaptatif avec rééchantillonnage |

### Choisir le bon modèle

| Situation | Modèle recommandé |
|-----------|------------------|
| Interprétabilité requise | `DecisionTreeClassifier` |
| Haute précision, entraînement parallèle | `RandomForestClassifier` |
| Meilleure précision, réglage fin | `GradientBoostingClassifier` |
| Boosting simple, entraînement rapide | `AdaBoostClassifier` |

### Propriétés clés

- Tous les modèles exposent `feature_importances_` pour la sélection de variables.
- Tous supportent `fit`, `predict`, `score`, `get_params`, `set_params`.
- Les modèles ensemblistes (RandomForest, GradientBoosting, AdaBoost) sont bien plus robustes au surapprentissage qu'un arbre de décision unique.

</div>
