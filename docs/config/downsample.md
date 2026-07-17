# Downsampling (LTTB)

<div class="lang-en">

Reduce massive datasets while preserving visual shape using the **Largest-Triangle-Three-Buckets** algorithm. A 10M-point scatter chart becomes 5K points indistinguishable to the eye.

## Python

```python
import seraplot as sp

chart = sp.scatter(title="Big scatter", x_values=big_x, y_values=big_y).downsample()
```

`downsample` is a chainable `Chart` method, not a standalone `sp.` function —
there is no Python `sp.lttb()`; the algorithm is only registered on the
JS/WASM side today (`downsampleLttb`, below).

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

chart = sp.scatter(title="Grand scatter", x_values=big_x, y_values=big_y).downsample()
```

`downsample` est une méthode chaînable sur `Chart`, pas une fonction `sp.`
autonome — il n'y a pas de `sp.lttb()` en Python ; l'algorithme n'est
enregistré que côté JS/WASM aujourd'hui (`downsampleLttb`, ci-dessous).

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
