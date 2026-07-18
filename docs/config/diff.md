# Chart Diff

<div class="lang-en">

Compare two charts structurally — useful for visual CI / regression testing. Implemented natively in Rust (`plot::utils::chart_diff`).

`chart_diff` is currently wired for the WASM/JS build only — it is not yet exposed as a Python method (`Chart.diff()` does not exist). Documented here as-is rather than papered over with a Python example that would not run.

## JavaScript / WASM

```javascript
import { chartDiff } from "seraplot";

const result = JSON.parse(chartDiff(JSON.stringify({ a: htmlA, b: htmlB })));
console.log(result);
```

Returns:

```json
{
  "ok": true,
  "identical": false,
  "size_a": 4521,
  "size_b": 4523,
  "common_prefix": 4400,
  "similarity": 0.97
}
```

`a`/`b` are compared on the `<svg>...</svg>` slice extracted from each chart's `.html`. `common_prefix` is the number of leading bytes the two SVGs share before the first difference; `similarity` is that shared length divided by the longer of the two.

</div>

<div class="lang-fr">

Compare deux charts structurellement — utile pour les CI visuelles / tests de régression. Implémenté nativement en Rust (`plot::utils::chart_diff`).

`chart_diff` n'est aujourd'hui câblé que pour le build WASM/JS — pas encore exposé comme méthode Python (`Chart.diff()` n'existe pas). Documenté tel quel plutôt que masqué derrière un exemple Python qui ne s'exécuterait pas.

## JavaScript / WASM

```javascript
import { chartDiff } from "seraplot";

const result = JSON.parse(chartDiff(JSON.stringify({ a: htmlA, b: htmlB })));
console.log(result);
```

Retourne :

```json
{
  "ok": true,
  "identical": false,
  "size_a": 4521,
  "size_b": 4523,
  "common_prefix": 4400,
  "similarity": 0.97
}
```

`a`/`b` sont comparés sur la tranche `<svg>...</svg>` extraite du `.html` de chaque chart. `common_prefix` est le nombre d'octets identiques au début des deux SVG avant la première différence ; `similarity` est cette longueur commune divisée par la plus longue des deux.

</div>
