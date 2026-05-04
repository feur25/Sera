# Neighbors

<div class="lang-en">

Instance-based models make predictions by looking at the closest training examples in feature space. No explicit model is fitted — the training data itself is the model.

| Model | Task | Description |
|-------|------|-------------|
| [KNeighborsClassifier / KNeighborsRegressor](knn.md) | Both | k-nearest neighbours using Euclidean distance |
| NearestCentroid | Classification | Assigns each sample to the class whose centroid is nearest |

### Key properties

- **Non-parametric** — no assumptions about the data distribution.
- **Lazy learning** — training is $O(1)$; all computation is deferred to prediction time.
- **Effect of k** — small $k$ = high variance (overfitting); large $k$ = high bias (underfitting). Tune via cross-validation.
- Works best on low-to-medium dimensional datasets. Performance degrades in high dimensions (curse of dimensionality).

</div>

<div class="lang-fr">

Les modèles à base d'instances font des prédictions en cherchant les exemples d'entraînement les plus proches dans l'espace des variables. Aucun modèle explicite n'est ajusté — les données d'entraînement constituent le modèle.

| Modèle | Tâche | Description |
|-------|------|-------------|
| [KNeighborsClassifier / KNeighborsRegressor](knn.md) | Les deux | k plus proches voisins avec distance euclidienne |
| NearestCentroid | Classification | Affecte chaque échantillon à la classe dont le centroïde est le plus proche |

### Propriétés clés

- **Non-paramétrique** — aucune hypothèse sur la distribution des données.
- **Apprentissage paresseux** — l'entraînement est $O(1)$ ; tout le calcul est reporté à la prédiction.
- **Effet de k** — petit $k$ = forte variance (surapprentissage) ; grand $k$ = fort biais (sous-apprentissage). À régler par validation croisée.
- Fonctionne mieux sur des données de dimension faible à moyenne. Les performances se dégradent en haute dimension (malédiction de la dimensionnalité).

</div>
