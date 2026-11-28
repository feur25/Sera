# Chart Diff

<div class="lang-en">

Compare two charts structurally — useful for visual CI / regression testing.

## Python

```python
import seraplot as sp

a = sp.bar("", labels=["x","y","z"], values=[1,2,3])
b = sp.bar("", labels=["x","y","z"], values=[1,2,4])
result = a.diff(b)
print(result)
```

Returns JSON: `{ "ok": true, "identical": false, "size_a": 4521, "size_b": 4523, "common_prefix": 4400, "similarity": 0.97 }`.

## JavaScript

```javascript
import { chartDiff } from "seraplot";
const diff = JSON.parse(chartDiff(JSON.stringify({ a: htmlA, b: htmlB })));
```

</div>

<div class="lang-fr">

Compare deux charts structurellement — utile pour les CI visuelles / tests de régression.

## Python

```python
import seraplot as sp

a = sp.bar("", labels=["x","y","z"], values=[1,2,3])
b = sp.bar("", labels=["x","y","z"], values=[1,2,4])
result = a.diff(b)
print(result)
```

Retourne du JSON : `{ "ok": true, "identical": false, "size_a": 4521, "size_b": 4523, "common_prefix": 4400, "similarity": 0.97 }`.

## JavaScript

```javascript
import { chartDiff } from "seraplot";
const diff = JSON.parse(chartDiff(JSON.stringify({ a: htmlA, b: htmlB })));
```

</div>
