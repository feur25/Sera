# Clustering

<div class="lang-en">

SeraPlot provides two clustering algorithms — K-Means and DBSCAN — each available as both a one-call chart function and a reusable class API.

| Model | Type | Description |
|-------|------|-------------|
| [K-Means Chart](kmeans.md) | Chart | 2D K-Means clustering visualization with inertia display |
| [KMeans Class](kmeans-class.md) | Class API | N-dimensional K-Means with sklearn-compatible interface |
| [DBSCAN 2D](dbscan.md) | Chart | 2D DBSCAN density-based clustering chart |
| [DBSCAN 3D](dbscan3d.md) | Chart | 3D DBSCAN clustering visualization |
| [DBSCAN Class](dbscan-class.md) | Class API | DBSCAN class with `labels_`, `n_clusters_`, `n_noise_` |

### When to use each

- Use **chart functions** (`sp.kmeans()`, `sp.build_dbscan_chart()`) for quick one-liner visualizations.
- Use **class APIs** (`sp.KMeans()`, `sp.DBSCAN()`) when you need the cluster labels or centroids for downstream processing.

</div>

<div class="lang-fr">

SeraPlot propose deux algorithmes de clustering — K-Means et DBSCAN — disponibles en tant que fonction graphique et en tant qu'API classe.

| Modèle | Type | Description |
|-------|------|-------------|
| [Graphique K-Means](kmeans.md) | Graphique | Visualisation K-Means 2D avec affichage de l'inertie |
| [Classe KMeans](kmeans-class.md) | API Classe | K-Means N-dimensionnel compatible scikit-learn |
| [DBSCAN 2D](dbscan.md) | Graphique | Graphique de clustering DBSCAN par densité en 2D |
| [DBSCAN 3D](dbscan3d.md) | Graphique | Visualisation DBSCAN en 3D |
| [Classe DBSCAN](dbscan-class.md) | API Classe | Classe DBSCAN avec `labels_`, `n_clusters_`, `n_noise_` |

### Quand utiliser chaque variante

- Utilisez les **fonctions graphiques** (`sp.kmeans()`, `sp.build_dbscan_chart()`) pour des visualisations rapides en une ligne.
- Utilisez les **API classes** (`sp.KMeans()`, `sp.DBSCAN()`) quand vous avez besoin des labels de clusters ou des centroïdes pour un traitement ultérieur.

</div>
