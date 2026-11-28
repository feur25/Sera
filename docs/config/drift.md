# Drift Detection

<div class="lang-en">

Detect distribution shift between a reference dataset and a current one using the **Kolmogorov-Smirnov** two-sample test — implemented natively in Rust (`plot::utils::drift_ks`).

`drift_ks` is currently wired for the WASM/JS build only — it is not yet exposed as a Python function (`sp.drift` does not exist). Documented here as-is rather than papered over with a Python example that would not run.

## JavaScript / WASM

```javascript
import { driftKs } from "seraplot";

const r = JSON.parse(driftKs(JSON.stringify({ reference, current })));
console.log(r);
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

</div>

<div class="lang-fr">

Détecte un drift de distribution entre un dataset de référence et un dataset actuel via le test à deux échantillons de **Kolmogorov-Smirnov** — implémenté nativement en Rust (`plot::utils::drift_ks`).

`drift_ks` n'est aujourd'hui câblé que pour le build WASM/JS — pas encore exposé comme fonction Python (`sp.drift` n'existe pas). Documenté tel quel plutôt que masqué derrière un exemple Python qui ne s'exécuterait pas.

## JavaScript / WASM

```javascript
import { driftKs } from "seraplot";

const r = JSON.parse(driftKs(JSON.stringify({ reference, current })));
console.log(r);
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

</div>
