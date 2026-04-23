# Downsampling (LTTB)

<div class="lang-en">

Reduce massive datasets while preserving visual shape using the **Largest-Triangle-Three-Buckets** algorithm. A 10M-point scatter chart becomes 5K points indistinguishable to the eye.

## Python

```python
import seraplot as sp

chart = sp.scatter(big_x, big_y).downsample(n=5000)

points = sp.lttb(list(zip(big_x, big_y)), threshold=5000)
```

## JavaScript

```javascript
import { downsampleLttb } from "seraplot";
const reduced = JSON.parse(downsampleLttb(JSON.stringify({ x, y, threshold: 5000 })));
```

## Why LTTB?

| Method | Preserves peaks | Speed | Visual fidelity |
|--------|----------------|-------|-----------------|
| Random sample | No | Fast | Poor |
| Every-Nth | Maybe | Fast | OK |
| **LTTB** | Yes | Fast | Excellent |

</div>

<div class="lang-fr">

Réduit les datasets massifs en préservant la forme visuelle avec l'algorithme **Largest-Triangle-Three-Buckets**. Un scatter de 10M points devient 5K points indistinguables à l'œil.

## Python

```python
import seraplot as sp

chart = sp.scatter(big_x, big_y).downsample(n=5000)

points = sp.lttb(list(zip(big_x, big_y)), threshold=5000)
```

## JavaScript

```javascript
import { downsampleLttb } from "seraplot";
const reduced = JSON.parse(downsampleLttb(JSON.stringify({ x, y, threshold: 5000 })));
```

## Pourquoi LTTB ?

| Méthode | Préserve les pics | Vitesse | Fidélité visuelle |
|---------|-------------------|---------|-------------------|
| Échantillon aléatoire | Non | Rapide | Mauvaise |
| Tous les N | Peut-être | Rapide | OK |
| **LTTB** | Oui | Rapide | Excellente |

</div>
