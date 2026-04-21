# SVM — Support Vector Machines

<div class="lang-en">

Support Vector Machines find the optimal hyperplane that maximises the margin between classes (classification) or fits within an epsilon-tube around the data (regression).

| Model | Task | Description |
|-------|------|-------------|
| [LinearSVC / LinearSVR](svm.md) | Both | Linear kernel SVM for large-scale problems |

### Key properties

- **LinearSVC** — classification with a linear kernel. Scales well to large datasets via coordinate descent.
- **LinearSVR** — regression with an epsilon-insensitive loss function.
- Both use L2 regularisation controlled by `C` (smaller = stronger regularisation).
- Best suited for linearly separable problems or high-dimensional sparse data (e.g., text classification).

### When to use SVM

| Situation | Advice |
|-----------|--------|
| Large dataset, linear problem | `LinearSVC` / `LinearSVR` |
| Non-linear boundaries | Use `GradientBoosting` or `RandomForest` instead |
| Text / NLP classification | `LinearSVC` is a strong baseline |

</div>

<div class="lang-fr">

Les machines à vecteurs de support trouvent l'hyperplan optimal qui maximise la marge entre les classes (classification) ou s'adapte dans un tube epsilon autour des données (régression).

| Modèle | Tâche | Description |
|-------|------|-------------|
| [LinearSVC / LinearSVR](svm.md) | Les deux | SVM à noyau linéaire pour les problèmes à grande échelle |

### Propriétés clés

- **LinearSVC** — classification avec un noyau linéaire. Passe à l'échelle grâce à la descente de coordonnées.
- **LinearSVR** — régression avec une fonction de perte insensible à epsilon.
- Les deux utilisent une régularisation L2 contrôlée par `C` (plus petit = régularisation plus forte).
- Idéal pour les problèmes linéairement séparables ou les données sparses en haute dimension (ex. : classification de texte).

### Quand utiliser SVM

| Situation | Conseil |
|-----------|--------|
| Grand ensemble de données, problème linéaire | `LinearSVC` / `LinearSVR` |
| Frontières non linéaires | Utiliser `GradientBoosting` ou `RandomForest` |
| Classification texte / NLP | `LinearSVC` est une bonne ligne de base |

</div>
