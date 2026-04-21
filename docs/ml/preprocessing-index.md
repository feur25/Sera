# Preprocessing

<div class="lang-en">

Preprocessing transformers scale and normalise features before feeding them to a model. Most linear models and distance-based models (KNN, SVM) require feature scaling to work correctly.

| Transformer | Description |
|-------------|-------------|
| [StandardScaler](preprocessing.md) | Zero mean, unit variance — $z = (x - \mu) / \sigma$ |
| [MinMaxScaler](preprocessing.md) | Scales to a fixed range $[0, 1]$ by default |
| [RobustScaler](preprocessing.md) | Uses median and IQR — robust to outliers |
| [MaxAbsScaler](preprocessing.md) | Divides by the maximum absolute value — preserves sparsity |
| [Normalizer](preprocessing.md) | Scales each sample to unit norm |

All transformers are documented on the same page: [Preprocessing](preprocessing.md).

### Typical pipeline

```python
import seraplot as sp
import numpy as np

X_train, X_test, y_train, y_test = sp.train_test_split(X, y, test_size=0.2)

scaler = sp.StandardScaler()
X_train_scaled = scaler.fit_transform(X_train)
X_test_scaled  = scaler.transform(X_test)   # use fit parameters from train set

model = sp.LogisticRegression()
model.fit(X_train_scaled, y_train)
```

> Always fit the scaler on the **training set only**, then apply the same transformation to the test set.

</div>

<div class="lang-fr">

Les transformateurs de prétraitement normalisent les variables avant de les fournir à un modèle. La plupart des modèles linéaires et des modèles basés sur la distance (KNN, SVM) nécessitent une normalisation pour fonctionner correctement.

| Transformateur | Description |
|-------------|-------------|
| [StandardScaler](preprocessing.md) | Moyenne nulle, variance unitaire — $z = (x - \mu) / \sigma$ |
| [MinMaxScaler](preprocessing.md) | Mise à l'échelle dans une plage fixe $[0, 1]$ par défaut |
| [RobustScaler](preprocessing.md) | Utilise la médiane et l'IQR — robuste aux valeurs aberrantes |
| [MaxAbsScaler](preprocessing.md) | Divise par la valeur absolue maximale — préserve la sparsité |
| [Normalizer](preprocessing.md) | Met chaque échantillon à norme unitaire |

Tous les transformateurs sont documentés sur la même page : [Preprocessing](preprocessing.md).

### Pipeline typique

```python
import seraplot as sp
import numpy as np

X_train, X_test, y_train, y_test = sp.train_test_split(X, y, test_size=0.2)

scaler = sp.StandardScaler()
X_train_scaled = scaler.fit_transform(X_train)
X_test_scaled  = scaler.transform(X_test)   # utiliser les paramètres du jeu d'entraînement

model = sp.LogisticRegression()
model.fit(X_train_scaled, y_train)
```

> Toujours ajuster le normaliseur sur le **jeu d'entraînement uniquement**, puis appliquer la même transformation au jeu de test.

</div>
