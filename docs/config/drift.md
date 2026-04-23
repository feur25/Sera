# Drift Detection

<div class="lang-en">

Detect distribution shift between a reference dataset and a current one using the **Kolmogorov-Smirnov** two-sample test.

## Python

```python
import seraplot as sp
import json

result = json.loads(sp.drift(reference=ref_values, current=cur_values))
print(result)
```

Returns:

```json
{
  "ok": true,
  "ks_statistic": 0.18,
  "p_value": 0.003,
  "drift_detected": true,
  "n_reference": 1000,
  "n_current": 1000
}
```

`drift_detected` is `true` when `p_value < 0.05`.

## JavaScript

```javascript
import { driftKs } from "seraplot";
const r = JSON.parse(driftKs(JSON.stringify({ reference, current })));
```

</div>

<div class="lang-fr">

Détecte un drift de distribution entre un dataset de référence et un dataset actuel via le test à deux échantillons de **Kolmogorov-Smirnov**.

## Python

```python
import seraplot as sp
import json

result = json.loads(sp.drift(reference=ref_values, current=cur_values))
print(result)
```

Retourne :

```json
{
  "ok": true,
  "ks_statistic": 0.18,
  "p_value": 0.003,
  "drift_detected": true,
  "n_reference": 1000,
  "n_current": 1000
}
```

`drift_detected` est `true` quand `p_value < 0.05`.

## JavaScript

```javascript
import { driftKs } from "seraplot";
const r = JSON.parse(driftKs(JSON.stringify({ reference, current })));
```

</div>
