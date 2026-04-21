# Naive Bayes

<div class="lang-en">

Naive Bayes classifiers apply Bayes' theorem with the assumption that features are conditionally independent given the class. Despite this strong assumption, they are fast, interpretable, and surprisingly effective for text and categorical data.

| Model | Best for | Description |
|-------|----------|-------------|
| [GaussianNB](naive-bayes.md) | Continuous features | Assumes Gaussian (normal) distribution per feature per class |
| [MultinomialNB](naive-bayes.md) | Count / frequency data | Models feature counts (e.g., word frequencies in text) |
| [BernoulliNB](naive-bayes.md) | Binary features | Models binary presence/absence of features |

All three are documented on the same page: [GaussianNB / MultinomialNB / BernoulliNB](naive-bayes.md).

### Key properties

- Extremely fast to train — $O(nd)$ time.
- Works well with small datasets.
- Naturally handles multi-class classification.
- Requires feature scaling only for GaussianNB with very different variance ranges.

</div>

<div class="lang-fr">

Les classifieurs Naive Bayes appliquent le théorème de Bayes en supposant que les variables sont conditionnellement indépendantes sachant la classe. Malgré cette hypothèse forte, ils sont rapides, interprétables et étonnamment efficaces pour les données textuelles et catégorielles.

| Modèle | Idéal pour | Description |
|-------|----------|-------------|
| [GaussianNB](naive-bayes.md) | Variables continues | Suppose une distribution gaussienne par variable par classe |
| [MultinomialNB](naive-bayes.md) | Données de comptage / fréquence | Modélise les comptages de variables (ex. : fréquences de mots) |
| [BernoulliNB](naive-bayes.md) | Variables binaires | Modélise la présence/absence binaire de variables |

Les trois sont documentés sur la même page : [GaussianNB / MultinomialNB / BernoulliNB](naive-bayes.md).

### Propriétés clés

- Très rapide à entraîner — temps $O(nd)$.
- Fonctionne bien avec de petits ensembles de données.
- Gère nativement la classification multi-classe.
- La normalisation des variables n'est nécessaire que pour GaussianNB avec des plages de variance très différentes.

</div>
