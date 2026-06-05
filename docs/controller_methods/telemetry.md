# Telemetry

<style>
.badge { display: inline-block; font-size: 10px; font-weight: 700; padding: 1px 6px; border-radius: 4px; vertical-align: middle; margin-left: 4px; letter-spacing: .04em; }
.badge-core { background: rgba(100,116,139,.12); border: 1px solid rgba(100,116,139,.3); color: #64748b; }
.badge-opt  { background: rgba(180,140,80,.1); border: 1px solid rgba(180,140,80,.3); color: #92731f; }
.note { border-left: 3px solid rgba(100,116,139,.4); padding: .5em 0 .5em 1em; margin: 1.2em 0; color: inherit; font-size: .93em; line-height: 1.65; }
.note code { font-size: .9em; }
.status-off { display: inline-flex; align-items: center; gap: 7px; font-size: .88em; font-weight: 600; color: #64748b; margin: .2em 0 1.6em; }
.status-off::before { content: ""; width: 8px; height: 8px; border-radius: 50%; background: #94a3b8; flex-shrink: 0; }
.steps { list-style: none; padding: 0; margin: .8em 0 1.4em; counter-reset: step; }
.steps li { counter-increment: step; display: flex; gap: .9em; align-items: flex-start; padding: .6em 0; border-bottom: 1px solid rgba(100,116,139,.1); font-size: .93em; line-height: 1.6; }
.steps li:last-child { border-bottom: none; }
.steps li::before { content: counter(step); min-width: 22px; height: 22px; border-radius: 50%; border: 1px solid rgba(100,116,139,.3); color: #64748b; font-size: .8em; font-weight: 700; display: flex; align-items: center; justify-content: center; flex-shrink: 0; margin-top: 1px; }
.steps li strong { font-weight: 700; margin-right: .3em; }
</style>

SeraPlot can optionally collect anonymized performance metrics to help identify bottlenecks and understand usage across environments. No data leaves your machine without explicit consent.

<div class="status-off">Disabled by default — requires explicit activation</div>

---

## What is collected

| Field | Type | Description | Example |
|---|---|---|---|
| `ts` <span class="badge badge-core">CORE</span> | timestamp | Unix timestamp of the event | `1746615600` |
| `method` <span class="badge badge-core">CORE</span> | string | Name of the function called | `"scatter"` |
| `duration_ms` <span class="badge badge-core">CORE</span> | float | Execution time in milliseconds | `42.7` |
| `version` <span class="badge badge-core">CORE</span> | string | SeraPlot version | `"2.7.0"` |
| `os` <span class="badge badge-core">CORE</span> | string | Operating system | `"linux"` |
| `arch` <span class="badge badge-core">CORE</span> | string | CPU architecture | `"x86_64"` |
| `cpu_count` <span class="badge badge-core">CORE</span> | integer | Logical CPU cores | `8` |
| `ram_gb` <span class="badge badge-core">CORE</span> | float | Total system RAM in GB | `16.0` |
| `available_ram_gb` <span class="badge badge-core">CORE</span> | float | Available RAM at call time | `8.5` |
| `cpu_brand` <span class="badge badge-core">CORE</span> | string | CPU model name | `"Intel Core i7-9700"` |
| `python_version` <span class="badge badge-core">CORE</span> | string | Python interpreter version | `"3.11.0"` |
| `data_count` <span class="badge badge-opt">OPT</span> | integer | Number of data points processed | `1000000` |
| `data_size_mb` <span class="badge badge-opt">OPT</span> | float | Approximate input size in MB | `12.4` |
| `input_shape` <span class="badge badge-opt">OPT</span> | string | Input data dimensions | `"1000x256"` |
| `output_shape` <span class="badge badge-opt">OPT</span> | string | Output data dimensions | `"1000x64"` |
| `algorithm` <span class="badge badge-opt">OPT</span> | string | Algorithm variant used | `"KMeans"` |

CORE fields are always present. OPT fields are attached by the caller when relevant context is available.

---

## Privacy

SeraPlot telemetry **never** collects:

- User identity or credentials
- File names or file contents
- Model weights or parameters
- Individual data values or samples
- IP addresses or geolocation
- Any personally identifiable information

<div class="note">Events are dispatched in a background thread with a 5-second timeout. On failure they fall back to <code>~/.seraplot/telemetry.jsonl</code> and are retried on the next flush. No data is held longer than necessary.</div>

---

## How it works

<ol class="steps">
<li><strong>Capture.</strong> At the end of each instrumented call, the method name, execution time, and optional context (data size, shapes, algorithm) are collected.</li>
<li><strong>Serialize.</strong> The event is serialized as compact JSON, enriched with system info cached from the first event of the session.</li>
<li><strong>Persist locally.</strong> The JSON line is appended to <code>~/.seraplot/telemetry.jsonl</code> before any network call.</li>
<li><strong>Dispatch.</strong> A background thread sends the payload to a GitHub Actions dispatcher endpoint (<code>POST repos/feur25/seraplot/dispatches</code>).</li>
<li><strong>Flush.</strong> Successfully transmitted events are cleared from local storage.</li>
</ol>

---

## Enable / Disable

```python
import seraplot as sp

sp.telemetry_consent(enabled=True)   # opt in
sp.telemetry_consent(enabled=False)  # opt out
```

This writes `~/.seraplot/consent.json`. Until this is called, telemetry is always off.

---

## Metrics summary

```python
metrics = sp.get_metrics_summary()
print(metrics)
```

Returns a JSON object with per-method statistics:

```json
{
  "event_count": 42,
  "methods": {
    "scatter":    { "count": 12, "avg_ms": 8.3,   "min_ms": 2.1, "max_ms": 31.0, "p50_ms": 7.1, "p95_ms": 18.4, "p99_ms": 31.0 },
    "KMeans.fit": { "count": 5,  "avg_ms": 142.0,  "min_ms": 98.0, "max_ms": 220.0, "p50_ms": 138.5, "p95_ms": 210.3 }
  }
}
```

---

## Source

The telemetry module is open-source at [`v2/src/telemetry.rs`](https://github.com/feur25/seraplot/blob/main/v2/src/telemetry.rs). Read it to verify exactly what is sent.

---

---

# Télémétrie

SeraPlot peut collecter optionnellement des métriques de performance anonymisées pour identifier les goulots d'étranglement et comprendre les usages. Aucune donnée ne quitte votre machine sans consentement explicite.

<div class="status-off">Désactivée par défaut — nécessite une activation explicite</div>

---

## Données collectées

| Champ | Type | Description | Exemple |
|---|---|---|---|
| `ts` <span class="badge badge-core">CORE</span> | timestamp | Horodatage Unix de l'événement | `1746615600` |
| `method` <span class="badge badge-core">CORE</span> | string | Nom de la fonction appelée | `"scatter"` |
| `duration_ms` <span class="badge badge-core">CORE</span> | float | Temps d'exécution en millisecondes | `42.7` |
| `version` <span class="badge badge-core">CORE</span> | string | Version de SeraPlot | `"2.7.0"` |
| `os` <span class="badge badge-core">CORE</span> | string | Système d'exploitation | `"linux"` |
| `arch` <span class="badge badge-core">CORE</span> | string | Architecture CPU | `"x86_64"` |
| `cpu_count` <span class="badge badge-core">CORE</span> | integer | Nombre de cœurs logiques | `8` |
| `ram_gb` <span class="badge badge-core">CORE</span> | float | RAM totale en Go | `16.0` |
| `available_ram_gb` <span class="badge badge-core">CORE</span> | float | RAM disponible au moment de l'appel | `8.5` |
| `cpu_brand` <span class="badge badge-core">CORE</span> | string | Modèle du processeur | `"Intel Core i7-9700"` |
| `python_version` <span class="badge badge-core">CORE</span> | string | Version de l'interpréteur Python | `"3.11.0"` |
| `data_count` <span class="badge badge-opt">OPT</span> | integer | Nombre de points de données traités | `1000000` |
| `data_size_mb` <span class="badge badge-opt">OPT</span> | float | Taille approximative des données en Mo | `12.4` |
| `input_shape` <span class="badge badge-opt">OPT</span> | string | Dimensions des données d'entrée | `"1000x256"` |
| `output_shape` <span class="badge badge-opt">OPT</span> | string | Dimensions des données de sortie | `"1000x64"` |
| `algorithm` <span class="badge badge-opt">OPT</span> | string | Algorithme spécifique utilisé | `"KMeans"` |

Les champs CORE sont toujours présents. Les champs OPT sont ajoutés par l'appelant quand le contexte est disponible.

---

## Confidentialité

La télémétrie SeraPlot ne collecte **jamais** :

- Identité ou credentials utilisateur
- Noms ou contenus de fichiers
- Poids ou paramètres de modèles
- Valeurs individuelles ou échantillons
- Adresses IP ou géolocalisation
- Toute information personnelle identifiable

<div class="note">Les événements sont envoyés dans un thread en arrière-plan avec un timeout de 5 secondes. En cas d'échec, ils sont écrits dans <code>~/.seraplot/telemetry.jsonl</code> et renvoyés au prochain flush. Aucune donnée n'est conservée plus longtemps que nécessaire.</div>

---

## Fonctionnement

<ol class="steps">
<li><strong>Capture.</strong> À la fin de chaque appel instrumenté, le nom de la méthode, le temps d'exécution et le contexte optionnel (taille des données, formes, algorithme) sont collectés.</li>
<li><strong>Sérialisation.</strong> L'événement est sérialisé en JSON compact, enrichi des informations système mises en cache dès le premier événement de la session.</li>
<li><strong>Persistance locale.</strong> La ligne JSON est ajoutée à <code>~/.seraplot/telemetry.jsonl</code> avant tout appel réseau.</li>
<li><strong>Envoi.</strong> Un thread en arrière-plan envoie le payload vers un endpoint GitHub Actions dispatcher (<code>POST repos/feur25/seraplot/dispatches</code>).</li>
<li><strong>Nettoyage.</strong> Les événements transmis avec succès sont supprimés du stockage local.</li>
</ol>

---

## Activer / Désactiver

```python
import seraplot as sp

sp.telemetry_consent(enabled=True)   # activer
sp.telemetry_consent(enabled=False)  # désactiver
```

Cela écrit `~/.seraplot/consent.json`. Tant que cette fonction n'est pas appelée, la télémétrie reste désactivée.

---

## Résumé des métriques

```python
metrics = sp.get_metrics_summary()
print(metrics)
```

Retourne un objet JSON avec les statistiques par méthode :

```json
{
  "event_count": 42,
  "methods": {
    "scatter":    { "count": 12, "avg_ms": 8.3,  "min_ms": 2.1,  "max_ms": 31.0,  "p50_ms": 7.1,  "p95_ms": 18.4, "p99_ms": 31.0 },
    "KMeans.fit": { "count": 5,  "avg_ms": 142.0, "min_ms": 98.0, "max_ms": 220.0, "p50_ms": 138.5, "p95_ms": 210.3 }
  }
}
```

---

## Code source

Le module de télémétrie est open-source dans [`v2/src/telemetry.rs`](https://github.com/feur25/seraplot/blob/main/v2/src/telemetry.rs). Lisez-le pour vérifier exactement ce qui est envoyé.
